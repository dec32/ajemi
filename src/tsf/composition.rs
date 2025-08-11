use std::ffi::OsString;

use log::{debug, trace};
use windows::{
    Win32::{
        Foundation::E_FAIL,
        UI::TextServices::{ITfComposition, ITfCompositionSink_Impl},
    },
    core::Result,
};

use super::{TextService, TextServiceInner, edit_session};
use crate::{PREEDIT_DELIMITER, extend::OsStrExt2};

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
        let composition =
            edit_session::start_composition(self.tid, self.context()?, &self.interface()?)?;
        self.composition = Some(composition);
        if let Some((x, y)) = self.get_pos() {
            self.candidate_list()?.locate(x, y)?;
        }
        Ok(())
    }

    pub fn end_composition(&mut self) -> Result<()> {
        // clean up the shit as clean as possbile instead of question-markin' all the way thru
        if let (Some(context), Some(composition)) =
            (self.context.as_ref(), self.composition.as_ref())
        {
            let _ = edit_session::end_composition(self.tid, context, composition);
        }
        self.composition = None;
        self.spelling.clear();
        self.selected.clear();
        self.suggestions.clear();
        self.candidate_list()?.hide();
        Ok(())
    }

    fn udpate_preedit(&mut self) -> Result<()> {
        self.preedit.clear();
        self.preedit.push_str(&self.selected);
        if self.suggestions.is_empty() {
            self.preedit.push_str(&self.spelling);
        } else {
            let mut from = 0;
            for to in &self.suggestions[0].groupping {
                self.preedit.push_str(&self.spelling[from..*to]);
                self.preedit.push_str(PREEDIT_DELIMITER);
                from = *to;
            }
            if from != self.spelling.len() {
                self.preedit.push_str(&self.spelling[from..])
            } else {
                self.preedit.pop();
            }
        }
        let range = unsafe { self.composition()?.GetRange()? };
        let text = OsString::from(&self.preedit).wchars();
        edit_session::set_text(
            self.tid,
            self.context()?,
            range,
            &text,
            self.display_attribute.as_ref(),
        )
    }

    fn update_candidate_list(&mut self) -> Result<()> {
        self.assure_candidate_list()?;
        let candidate_list = self.candidate_list()?;
        if self.suggestions.is_empty() {
            candidate_list.hide();
        } else {
            candidate_list.show(&self.suggestions)?;
            if let Some((x, y)) = self.get_pos() {
                candidate_list.locate(x, y)?;
            }
        }
        Ok(())
    }

    fn set_text(&self, text: &str) -> Result<()> {
        let text = OsString::from(text).wchars();
        let range = unsafe { self.composition()?.GetRange()? };
        edit_session::set_text(self.tid, self.context()?, range, &text, None)
    }

    fn get_pos(&self) -> Option<(i32, i32)> {
        let range = unsafe { self.composition().ok()?.GetRange().ok()? };
        let pos = edit_session::get_pos(self.tid, self.context().ok()?, &range).ok()?;
        if pos.0 <= 0 && pos.1 <= 0 {
            debug!("Abnormal position: ({}, {})", pos.0, pos.1);
            None
        } else {
            Some(pos)
        }
    }

    fn composition(&self) -> Result<&ITfComposition> {
        self.composition.as_ref().ok_or(E_FAIL.into())
    }
}

// handle input and transit state
// calling these function while not composing would cause the program to crash
impl TextServiceInner {
    pub fn push(&mut self, ch: char) -> Result<()> {
        self.spelling.push(ch);
        self.suggestions = self.engine.suggest(&self.spelling);
        self.udpate_preedit()?;
        self.update_candidate_list()?;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<()> {
        // todo pop can be used to revert selection
        self.spelling.pop();
        if self.spelling.is_empty() {
            return self.abort();
        }
        self.suggestions = self.engine.suggest(&self.spelling);
        self.udpate_preedit()?;
        self.update_candidate_list()?;
        Ok(())
    }

    /// Commit the 1st suggestion, keeping the unrecognizable trailing characters
    pub fn commit(&mut self) -> Result<()> {
        if self.suggestions.is_empty() {
            self.force_release(' ')
        } else {
            self.select(0)
        }
    }

    /// Commit the 1st suggestion and release the unrecognizable trailing characters.
    pub fn force_commit(&mut self, ch: char) -> Result<()> {
        if self.suggestions.is_empty() {
            self.force_release(ch)
        } else {
            let sugg = self.suggestions.first().unwrap();
            self.selected.push_str(&sugg.output);
            let last = *sugg.groupping.last().unwrap();
            if last != self.spelling.len() {
                self.selected.push(' ');
                self.selected.push_str(&self.spelling[last..])
            }
            self.selected.push(ch);
            self.set_text(&self.selected)?;
            self.end_composition()
        }
    }

    /// Select the desired suggestion by pressing numbers.
    pub fn select(&mut self, index: usize) -> Result<()> {
        if index >= self.suggestions.len() {
            return Ok(());
        }
        let sugg = self.suggestions.get(index).unwrap();
        let last = *sugg.groupping.last().unwrap();
        if last == self.spelling.len() {
            if self.selected.is_empty() {
                self.set_text(&sugg.output)?;
            } else {
                self.selected.push_str(&sugg.output);
                self.set_text(&self.selected)?;
            };
            self.end_composition()
        } else {
            self.selected.push_str(&sugg.output);
            // TODO strip off the begining instead of re allocate
            self.spelling = self.spelling[last..].to_string();
            self.suggestions = self.engine.suggest(&self.spelling);
            self.udpate_preedit()?;
            self.update_candidate_list()
        }
    }

    // Release the raw ascii chars
    pub fn release(&mut self) -> Result<()> {
        if self.selected.is_empty() {
            self.set_text(&self.spelling)?;
        } else {
            self.selected.push(' ');
            self.selected.push_str(&self.spelling);
            self.set_text(&self.selected)?;
        }
        self.end_composition()
    }

    fn force_release(&mut self, ch: char) -> Result<()> {
        if self.selected.is_empty() {
            self.spelling.push(ch);
            self.set_text(&self.spelling)?;
        } else {
            self.selected.push(' ');
            self.selected.push_str(&self.spelling);
            self.selected.push(ch);
            self.set_text(&self.selected)?;
        }
        self.end_composition()
    }

    // Interupted. Abort everything.
    pub fn abort(&mut self) -> Result<()> {
        if self.selected.is_empty() {
            let _ = self.set_text(&self.spelling);
        } else {
            if !self.spelling.is_empty() {
                self.selected.push(' ');
                self.selected.push_str(&self.spelling);
            }
            self.set_text(&self.selected)?;
        }
        self.end_composition()
    }
}

#[allow(non_snake_case)]
impl ITfCompositionSink_Impl for TextService {
    fn OnCompositionTerminated(
        &self,
        _ecwrite: u32,
        _composition: Option<&ITfComposition>,
    ) -> Result<()> {
        trace!("OnCompositionTerminated");
        // popping out the last letter will trigger this method.
        // `self.write()` causes deadlock(?) in such circumstances
        // because `pop` waits for the completion of this method
        // and this method waits for the releasing of the lock held by `pop`.
        // `self.try_lock()` avoids such issue
        self.try_write()?.abort()
    }
}
