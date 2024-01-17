use std::ffi::OsString;
use log::trace;
use windows::Win32::UI::TextServices::{ITfContext, ITfComposition, ITfCompositionSink, ITfCompositionSink_Impl};
use windows::core::{Result, implement};
use crate::{ime::edit_session, extend::OsStrExt2, engine::engine};

//----------------------------------------------------------------------------
//
//  Composition is the texts held by the input method waiting to be "composed"
//  into proper output, or more straightforwardly, those quirky underscored
//  text.
//
//----------------------------------------------------------------------------

#[derive(Default)]
pub struct Composition {
    tid: u32,
    composition: Option<ITfComposition>,
    spelling: String,
    // suggestion from engine
    output: String,
    groupping: Vec<usize>
}

impl Composition {
    pub fn new (tid: u32) -> Composition {
        Composition {tid, ..Default::default()}
    }

    // there are only two states: composing or not
    pub fn start(&mut self, context: &ITfContext) -> Result<()> {
        self.composition = Some(edit_session::start_composition(self.tid, context, &CompositionSink{}.into())?);
        Ok(())
    }

    pub fn end(&mut self, context: &ITfContext) -> Result<()> {
        edit_session::end_composition(self.tid, context, self.composition.as_ref().unwrap())?;
        self.composition = None;
        self.spelling.clear();
        self.output.clear();
        self.groupping.clear();
        Ok(())
    }

    // to check the current state
    pub fn composing(&self) -> bool {
        self.composition.is_some()
    }

    // make things easier
    pub fn set_text(&self, context: &ITfContext, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition.as_ref().unwrap().GetRange()? };
        edit_session::set_text(self.tid, context, range, &text)
    }

    // FIXME this function is slow-ass
    pub fn set_text_as_suggestions(&self, context: &ITfContext) -> Result<()> {
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
}

// handle input and transit state
// calling these function while not composing would cause the program to crash
impl Composition {
    pub fn push(&mut self, context: &ITfContext, ch: char) -> Result<()>{
        trace!("push({ch})"); 
        // todo auto-commit
        self.spelling.push(ch);
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions(context)
    }

    pub fn pop(&mut self, context: &ITfContext) -> Result<()>{
        self.spelling.pop();
        if self.spelling.is_empty() {
            self.abort(context)?;
            return Ok(());
        }
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions(context)
    }

    // commit the suggestion, keeping the unrecognizable trailing characters
    pub fn commit(&mut self, context: &ITfContext) -> Result<()>{
        if self.output.is_empty() {
            self.spelling.push(' ');
            self.set_text(context, &self.spelling)?;
            self.end(context)
        } else {
            self.set_text(context, &self.output)?;
            let last = *self.groupping.last().unwrap();
            if last == self.spelling.len() {
                self.end(context)
            } else {
                let trailing = &self.spelling[last..].to_string();
                self.end(context)?;
                self.start(context)?;
                self.spelling.push_str(&trailing);
                self.set_text(context, &self.spelling)
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

#[implement(ITfCompositionSink)]
struct CompositionSink;
impl ITfCompositionSink_Impl for CompositionSink {
    #[allow(non_snake_case)]
    fn OnCompositionTerminated(&self, _ecwrite:u32, _composition: Option<&ITfComposition>) -> Result<()> {
        trace!("OnCompositionTerminated");
        Ok(())
    }
}