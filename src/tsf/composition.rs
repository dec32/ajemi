use std::ffi::OsString;
use log::{trace, error};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::{ITfComposition, ITfCompositionSink_Impl};
use windows::core::{Result, Error};
use crate::{extend::OsStrExt2, engine::engine};
use super::{edit_session, TextServiceInner, TextService};

//----------------------------------------------------------------------------
//
//  Composition is the texts held by the input method waiting to be "composed"
//  into proper output, or more straightforwardly, those quirky underscored
//  text.
//
//----------------------------------------------------------------------------

impl TextServiceInner {
    // there are only two states: composing or not
    pub fn start_composition(&mut self) -> Result<()> {
        let composition = edit_session::start_composition(
            self.tid, self.context()?, &self.interface()?)?;
        // todo use (0, 0) if failed. 
        let pos = edit_session::get_pos(
            self.tid, self.context()?, unsafe{ &composition.GetRange()? })?;
        self.candidate_list()?.locate(pos.0, pos.1)?;
        self.composition = Some(composition);
        self.spelling.clear();
        self.output.clear();
        self.groupping.clear();
        Ok(())
    }

    pub fn end_composition(&mut self) -> Result<()> {
        // OnCompositionTerminated will set composition to None at any moment.
        edit_session::end_composition(self.tid, self.context()?, self.composition()?)?;
        self.composition = None;
        self.candidate_list()?.hide();
        Ok(())
    }

    // make things easier
    fn set_text(&self, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition()?.GetRange()? };
        edit_session::set_text(self.tid, self.context()?, range, &text)
    }

    fn composition(&self) -> Result<&ITfComposition> {
        self.composition.as_ref().ok_or_else(||{
            error!("Composition is None.");
            Error::from(E_FAIL)
        })
    }

    // FIXME this function is slow-ass
    fn set_text_as_suggestions(&self) -> Result<()> {
        if self.output.is_empty() {
            self.set_text(&self.spelling)
        } else {
            let mut buf = String::with_capacity(32);
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
            self.set_text(&buf)
        }
    }

    fn update_candidate_list(&mut self) -> Result<()> {
        if self.output.is_empty() {
            self.candidate_list()?.hide();
        } else {
            self.candidate_list()?.show(&self.output)?;
        }
        Ok(())
    }
}

// handle input and transit state
// calling these function while not composing would cause the program to crash
impl TextServiceInner {
    pub fn push(&mut self, ch: char) -> Result<()>{
        trace!("push({ch})"); 
        // todo auto-commit
        self.spelling.push(ch);
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions()?;
        self.update_candidate_list()?;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<()>{
        trace!("pop");
        self.spelling.pop();
        if self.spelling.is_empty() {
            return self.abort();
        }
        engine().suggest(&self.spelling, &mut self.groupping, &mut self.output);
        self.set_text_as_suggestions()?;
        self.update_candidate_list()?;
        Ok(())
    }

    /// Commit the suggestion, keeping the unrecognizable trailing characters
    pub fn commit(&mut self) -> Result<()>{
        if self.output.is_empty() {
            self.spelling.push(' ');
            self.set_text(&self.spelling)?;
            self.end_composition()
        } else {
            let last = *self.groupping.last().unwrap();
            if last == self.spelling.len() {
                self.set_text(&self.output)?;
                self.end_composition()
            } else {
                // // FIXME it will eat the already composed part
                // let trailing = &self.spelling[last..].to_string();
                // self.set_text(context, &self.output)?;
                // self.end(context)?;
                // self.start(context)?;
                // self.spelling.push_str(&trailing);
                // self.set_text(context, &self.spelling)
                self.force_commit(' ')
            }            
        }
    }

    /// Commit the suggestion and release the unrecognizable trailing characters.
    pub fn force_commit(&mut self, ch: char) -> Result<()>{
        trace!("force_commit");
        if self.output.is_empty() {
            self.spelling.push(ch);
            self.set_text(&self.spelling)?;
        } else {
            let last = *self.groupping.last().unwrap();
            if last < self.spelling.len() {
                self.output.push(' ');
                self.output.push_str(&self.spelling[last..])
            }
            self.output.push(ch);
            self.set_text(&self.output)?;
        }
        self.end_composition()
    }

    /// Select the desired suggestion by pressing num keys (or maybe tab, enter or any thing else)
    #[allow(dead_code)]
    pub fn select(&mut self) -> Result<()> {
        todo!("for v0.1 there's not multiple candidates to select from")
    }

    // Release the raw ascii chars
    pub fn release(&mut self) -> Result<()> {
        self.set_text(&self.spelling)?;
        self.end_composition()
    }

    // Interupted. Abort everything.
    pub fn abort(&mut self) -> Result<()> {
        self.set_text(&"")?;
        self.end_composition()
    }
}



#[allow(non_snake_case)]
impl ITfCompositionSink_Impl for TextService {
    fn OnCompositionTerminated(&self, _ecwrite:u32, _composition: Option<&ITfComposition>) -> Result<()> {
        trace!("OnCompositionTerminated");
        // FIXME the lock can never be obtained
        let mut inner = self.write()?;
        inner.set_text(&"")?;
        inner.composition = None;
        inner.candidate_list()?.hide();
        Ok(())
    }
}