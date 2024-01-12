use std::cell::Cell;
use std::ffi::OsStr;

use log::{trace, debug, error};
use windows::Win32::Foundation::S_OK;
use windows::core::{implement, Result, ComInterface, AsImpl, Error};
use windows::Win32::UI::TextServices::{ITfEditSession, ITfEditSession_Impl, ITfContextComposition, ITfCompositionSink, ITfComposition, ITfContext, TF_ES_READWRITE, ITfInsertAtSelection, TF_IAS_QUERYONLY, ITfRange, TF_ST_CORRECTION};

//----------------------------------------------------------------------------
//
//  Edit of any kind must be operated in edit sessions.
//  It's for safety reasons I guess.
//  But it's a pain in the ass to use such sessions so let' hide them under functions.
//
//----------------------------------------------------------------------------

pub fn start_composition(tid:u32, context: &ITfContext, composition_sink: &ITfCompositionSink) -> Result<ITfComposition> {
    trace!("start_composition");
    #[implement(ITfEditSession)]
    struct Session<'a> {
        context: &'a ITfContext,
        composition_sink: &'a ITfCompositionSink,
        composition: Cell<Option<ITfComposition>>,   // out
    }
    
    impl ITfEditSession_Impl for Session<'_> {
        #[allow(non_snake_case)]
        fn DoEditSession(&self, ec: u32) -> Result<()> {
            // to get the current range (namely the selected text or simply the cursor) you insert "nothing"
            // which genius came up with these APIs?
            let range = unsafe {
                self.context.cast::<ITfInsertAtSelection>()?
                    .InsertTextAtSelection(ec, TF_IAS_QUERYONLY, &[])?
            };
            debug!("Fetched the range successfully.");
            let context_composition = self.context.cast::<ITfContextComposition>()?;
            let composition = unsafe {
                let res = context_composition.StartComposition(
                    ec, &range, self.composition_sink);
                if let Err(e) = res {
                    error!("Can not start composition.");
                    error!("{}", e);
                    return Err(e);
                }
                res.unwrap()
            };
            debug!("Started composition.");
            self.composition.set(Some(composition));
            Ok(())
        }
    }

    let session = ITfEditSession::from(Session {
        context: context, 
        composition_sink: composition_sink, 
        composition: Cell::new(None)
    });

    unsafe {
        // todo dwflags
        // https://learn.microsoft.com/en-us/windows/win32/api/msctf/nf-msctf-itfcontext-requesteditsession
        let result = context.RequestEditSession(tid, &session, TF_ES_READWRITE)?;
        debug!("Requested a edit session to start composition");
        if result != S_OK {
            Err(Error::from(result))
        } else {
            let session: &Session = session.as_impl();
            Ok(session.composition.take().expect("Composition is None."))
        }
    }
}


pub fn end_composition(tid:u32, context: &ITfContext, composition: &ITfComposition) -> Result<()>{
    trace!("end_composition");
    #[implement(ITfEditSession)]
    struct Session<'a> (&'a ITfComposition);
    impl ITfEditSession_Impl for Session<'_> {
        #[allow(non_snake_case)]
        fn DoEditSession(&self, ec:u32) -> Result<()> {
            unsafe {self.0.EndComposition(ec)}
        }
    }
    let session = ITfEditSession::from(Session(composition));
    unsafe {
        let result = context.RequestEditSession(tid, &session, TF_ES_READWRITE)?;
        if result != S_OK {
            Err(Error::from(result))
        } else {
            Ok(())
        }
    }
}

pub fn set_text(tid:u32, context: &ITfContext, range: &ITfRange, text: &[u16]) -> Result<()> {
    #[implement(ITfEditSession)]
    struct Session<'a> {
        range: &'a ITfRange,
        text: &'a [u16],
    };

    impl ITfEditSession_Impl for Session<'_> {
        #[allow(non_snake_case)]
        fn DoEditSession(&self, ec:u32) -> Result<()> {
            unsafe {
                self.range.SetText(ec, TF_ST_CORRECTION, self.text)
            }
        }
    }

    let session = ITfEditSession::from(Session{range: range, text: text});
    unsafe {
        let result = context.RequestEditSession(tid, &session, TF_ES_READWRITE)?;
        if result != S_OK {
            Err(Error::from(result))
        } else {
            Ok(())
        }
    }
}