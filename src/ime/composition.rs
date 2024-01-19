use std::ffi::OsString;
use std::sync::atomic::AtomicBool;
use log::{trace, debug};
use windows::Win32::UI::TextServices::{ITfContext, ITfComposition, ITfCompositionSink, ITfCompositionSink_Impl};
use windows::core::{Result, implement, AsImpl};
use crate::{ime::edit_session, extend::OsStrExt2, engine::engine};

use super::candidate_list::CandidateList;

//----------------------------------------------------------------------------
//
//  Composition is the texts held by the input method waiting to be "composed"
//  into proper output, or more straightforwardly, those quirky underscored
//  text.
//
//----------------------------------------------------------------------------

pub struct Composition {
    tid: u32,
    spelling: String,
    // suggestion from engine
    output: String,
    groupping: Vec<usize>,
    // TSF related
    composition: Option<ITfComposition>,
    composition_sink: ITfCompositionSink,
    // UI
    candidate_list: CandidateList,
}

impl Composition {
    pub fn new (tid: u32) -> Composition {
        Composition {
            tid, 
            spelling: String::new(),
            output: String::new(),
            groupping: Vec::new(),
            composition: None,
            composition_sink: ITfCompositionSink::from(CompositionSink::default()),
            candidate_list: CandidateList::new(),
        }
    }

    // there are only two states: composing or not
    pub fn start(&mut self, context: &ITfContext) -> Result<()> {
        let composition = edit_session::start_composition(
            self.tid, context, &self.composition_sink)?;
        // todo use (0, 0) if failed. 
        let pos = edit_session::get_pos(
            self.tid, context, unsafe{ &composition.GetRange()? })?; 
        self.candidate_list.locate(pos);
        self.composition = Some(composition);
        self.spelling.clear();
        self.output.clear();
        self.groupping.clear();
        self.composition_sink().reuse();
        Ok(())
    }

    pub fn end(&mut self, context: &ITfContext) -> Result<()> {
        edit_session::end_composition(
            self.tid, context, self.composition.as_ref().as_ref().unwrap())?;
        self.composition = None;
        self.candidate_list.hide();
        Ok(())
    }

    /// To check the current state. 
    /// Mutations are allowed only when this method returns `true`.
    pub fn composing(&self) -> bool {
        if self.composition.is_none() {
            return false;
        }
        if self.composition_sink().terminated() {
            debug!("Composition was terminated by force a while ago.");
            return false;
        }
        return true;
    }

    fn composition_sink(&self) -> &CompositionSink {
        unsafe { self.composition_sink.as_impl() }
    }

    // make things easier
    fn set_text(&self, context: &ITfContext, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition.as_ref().unwrap().GetRange()? };
        edit_session::set_text(self.tid, context, range, &text)
    }

    // FIXME this function is slow-ass
    fn set_text_as_suggestions(&self, context: &ITfContext) -> Result<()> {
        if self.output.is_empty() {
            self.set_text(context, &self.spelling)
        } else {
            let mut buf = String::with_capacity(16);
            buf.push('[');
            buf += &self.output;
            buf.push(']');
            let mut from = 0;
            for to in &self.groupping {
                buf.push_str(&self.spelling[from..*to]);
                buf.push('\'');
                from = *to;
            }
            if from != self.spelling.len() {
                buf.push_str(&self.spelling[from..])
            } else {
                buf.pop();
            }
            self.set_text(context, &buf)
        }
    }

    fn update_candidate_list(&self) {
        if self.output.is_empty() {
            self.candidate_list.hide()
        } else {
            self.candidate_list.show(&self.output)
        }
    }
}

// handle input and transit state
// calling these function while not composing would cause the program to crash
impl Composition {
    pub fn push(&mut self, context: &ITfContext, ch: char) -> Result<()>{
        trace!("push({ch})"); 
        // todo auto-commit
        self.spelling.push(ch);
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions(context)?;
        self.update_candidate_list();
        Ok(())
    }

    pub fn pop(&mut self, context: &ITfContext) -> Result<()>{
        self.spelling.pop();
        if self.spelling.is_empty() {
            self.abort(context)?;
            return Ok(());
        }
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions(context)?;
        self.update_candidate_list();
        Ok(())
    }

    // commit the suggestion, keeping the unrecognizable trailing characters
    pub fn commit(&mut self, context: &ITfContext) -> Result<()>{
        if self.output.is_empty() {
            self.spelling.push(' ');
            self.set_text(context, &self.spelling)?;
            self.end(context)
        } else {
            let last = *self.groupping.last().unwrap();
            if last == self.spelling.len() {
                self.set_text(context, &self.output)?;
                self.end(context)
            } else {
                // // FIXME it will eat the already composed part
                // let trailing = &self.spelling[last..].to_string();
                // self.set_text(context, &self.output)?;
                // self.end(context)?;
                // self.start(context)?;
                // self.spelling.push_str(&trailing);
                // self.set_text(context, &self.spelling)
                self.force_commit(context, ' ')
            }            
        }
    }

    // commit the suggestion and release the unrecognizable trailing characters.
    pub fn force_commit(&mut self, context: &ITfContext, ch: char) -> Result<()>{
        trace!("force_commit");
        if self.output.is_empty() {
            self.spelling.push(ch);
            self.set_text(context, &self.spelling)?;
        } else {
            let last = *self.groupping.last().unwrap();
            if last < self.spelling.len() {
                self.output.push(' ');
                self.output.push_str(&self.spelling[last..])
            }
            self.output.push(ch);
            self.set_text(context, &self.output)?;
        }
        self.end(context)
    }

    // select the desired suggestion by pressing num keys (or maybe tab, enter or any thing else)
    #[allow(dead_code)]
    pub fn select(&mut self, _context: &ITfContext) -> Result<()> {
        todo!("for v0.1 there's not multiple candidates to select from")
    }

    // release the raw ascii chars
    pub fn release(&mut self, context: &ITfContext) -> Result<()> {
        self.set_text(context, &self.spelling)?;
        self.end(context)
    }

    // interupted. abort everything.
    pub fn abort(&mut self, context: &ITfContext) -> Result<()> {
        self.set_text(context, &"")?;
        self.end(context)
    }
}

#[derive(Default)]
#[implement(ITfCompositionSink)]
struct CompositionSink{
    terminated: AtomicBool
}
impl CompositionSink {
    fn reuse(&self) {
        self.terminated.store(false, std::sync::atomic::Ordering::Relaxed)
    }

    fn terminated(&self) -> bool {
        self.terminated.fetch_or(false, std::sync::atomic::Ordering::Relaxed)
    }
}

#[allow(non_snake_case)]
impl ITfCompositionSink_Impl for CompositionSink {
    fn OnCompositionTerminated(&self, _ecwrite:u32, _composition: Option<&ITfComposition>) -> Result<()> {
        trace!("OnCompositionTerminated");
        // FIXME this only prevents a terminated composition from swallowing letters into void.
        // However it does not make the composition abort the text it may hold.
        // To call Composition::abort from here can be very hard.
        // To re-rewrite Composition::abort here is impossible because this stupid method don't
        // have context: ITfContext passed in.
        // Thus this issue is ignored for now. It's not a fatal one anyway.
        self.terminated.store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }
}