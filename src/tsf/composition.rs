use std::ffi::OsString;
use log::{trace, warn};
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::UI::TextServices::{ITfComposition, ITfCompositionSink_Impl};
use windows::core::Result;
use crate::{extend::OsStrExt2, engine::engine};
use super::{edit_session, TextService, TextServiceInner};

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
        self.composition = Some(composition); 
        if let Some(pos) = self.get_pos() {
            self.candidate_list()?.locate(pos.0, pos.1)?;
        }
        Ok(())
    }

    pub fn end_composition(&mut self) -> Result<()> {
        // clean up the shit as clean as possbile instead of question-markin' all the way thru
        if let (Some(context), Some(composition)) = (self.context.as_ref(), self.composition.as_ref()) {
            let _ = edit_session::end_composition(self.tid, context, composition);
        }
        self.composition = None;
        self.spelling.clear();
        self.suggestions.clear();
        self.candidate_list()?.hide();
        Ok(())
    }

    fn set_text(&self, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition()?.GetRange()? };
        edit_session::set_text(self.tid, self.context()?, range, &text)
    }

    fn get_pos(&self) -> Option<(i32, i32)> {
        let range = unsafe{ self.composition().ok()?.GetRange().ok()? };
        let pos = edit_session::get_pos(self.tid, self.context().ok()?, &range).ok()?;
        if pos.0 <= 0 && pos.1 <= 0 {
            warn!("Abnormal position: ({}, {})", pos.0, pos.1);
            None
        } else {
            Some(pos)
        }
    }

    fn composition(&self) -> Result<&ITfComposition> {
        self.composition.as_ref().ok_or(E_FAIL.into())
    }

    fn udpate_text(&mut self) -> Result<()> {
        if self.suggestions.is_empty() {
            self.set_text(&self.spelling)
        } else {
            self.groupped_spelling.clear();
            let mut from = 0;
            for to in &self.suggestions[0].groupping {
                self.groupped_spelling.push_str(&self.spelling[from..*to]);
                self.groupped_spelling.push('\'');
                from = *to;
            }
            if from != self.spelling.len() {
                self.groupped_spelling.push_str(&self.spelling[from..])
            } else {
                self.groupped_spelling.pop();
            }
            self.set_text(&self.groupped_spelling)
        }
    }

    fn update_candidate_list(&mut self) -> Result<()> {
        // cannot borrow `self.output` as immutable because it is also borrowed as mutable
        // ok guess i have to clone it first
        let pos = self.get_pos();
        self.try_create_candiate_list()?;
        let candidate_list = self.candidate_list()?;
        if self.suggestions.is_empty() {
            candidate_list.hide();
        } else {
            candidate_list.show(&self.suggestions)?;
            if let Some(pos) = pos {
                candidate_list.locate(pos.0, pos.1)?;
            }
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
        self.suggestions = engine().suggest(&self.spelling);
        self.udpate_text()?;
        self.update_candidate_list()?;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<()>{
        trace!("pop");
        self.spelling.pop();
        if self.spelling.is_empty() {
            return self.abort();
        }
        self.suggestions = engine().suggest(&self.spelling);
        self.udpate_text()?;
        self.update_candidate_list()?;
        Ok(())
    }

    /// Commit the suggestion, keeping the unrecognizable trailing characters
    pub fn commit(&mut self) -> Result<()>{
        if self.suggestions.is_empty() {
            self.spelling.push(' ');
            self.set_text(&self.spelling)?;
            self.end_composition()
        } else {
            self.select(0)         
        }
    }

    /// Commit the suggestion and release the unrecognizable trailing characters.
    pub fn force_commit(&mut self, ch: char) -> Result<()>{
        trace!("force_commit");
        if self.suggestions.is_empty() {
            self.spelling.push(ch);
            self.set_text(&self.spelling)?;
        } else {
            let sugg = self.suggestions.get(0).unwrap();
            // todo fix clone
            let mut output = String::from(&sugg.output);
            let last = *sugg.groupping.last().unwrap();
            if last < self.spelling.len() {
                output.push(' ');
                output.push_str(&self.spelling[last..])
            }
            output.push(ch);
            self.set_text(&sugg.output)?;
        }
        self.end_composition()
    }

    /// Select the desired suggestion by pressing num keys (or maybe tab, enter or any thing else)
    #[allow(dead_code)]
    pub fn select(&mut self, index: usize) -> Result<()> {
        if index >= self.suggestions.len() {
            return Ok(());
        }
        let sugg = self.suggestions.get(index).unwrap();
        let last = *sugg.groupping.last().unwrap();
        if last == self.spelling.len() {
            self.set_text(&sugg.output)?;
            self.end_composition()
        } else {
            let trailing = &self.spelling[last..].to_string();
            self.set_text(&sugg.output)?;
            self.end_composition()

            // FIXME keeping the trailing chars won't work
            // self.start_composition()?;
            // self.spelling.push_str(&trailing);
            // self.set_text(&self.spelling);
        }
    }

    // Release the raw ascii chars
    pub fn release(&mut self) -> Result<()> {
        self.set_text(&self.spelling)?;
        self.end_composition()
    }

    // Interupted. Abort everything.
    pub fn abort(&mut self) -> Result<()> {
        let _ = self.set_text(&self.spelling);
        self.end_composition()
    }
}



#[allow(non_snake_case)]
impl ITfCompositionSink_Impl for TextService {
    fn OnCompositionTerminated(&self, _ecwrite:u32, _composition: Option<&ITfComposition>) -> Result<()> {
        trace!("OnCompositionTerminated");
        // popping out the last letter will trigger this method.
        // `self.write()` causes deadlock(?) in such circumstances
        // because `pop` waits for the completion of this method
        // and this method waits for the releasing of the lock held by `pop`.
        // `self.try_lock()` avoids such issue
        self.try_write()?.abort()
    }
}