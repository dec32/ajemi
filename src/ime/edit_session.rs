use std::cell::Cell;

use windows::core::{implement, Result, ComInterface, AsImpl};
use windows::Win32::UI::TextServices::{ITfEditSession, ITfEditSession_Impl, ITfContextComposition, ITfCompositionSink, ITfComposition, ITfContext, TF_ES_READWRITE, ITfInsertAtSelection, TF_IAS_QUERYONLY};

//----------------------------------------------------------------------------
//
//  Edit of any kind must be operated in edit sessions.
//  It's for safety reasons I guess.
//  But it's a pain in the ass to use such sessions so let' hide them under functions.
//
//----------------------------------------------------------------------------

pub fn start_composition(tid:u32, context: &ITfContext, composition_sink: &ITfCompositionSink) -> Result<ITfComposition> {

    #[implement(ITfEditSession)]
    struct Session<'a> {
        context: &'a ITfContext,
        composition_sink: &'a ITfCompositionSink,
        composition: Cell<Option<ITfComposition>>,   // out
    }
    
    impl ITfEditSession_Impl for Session<'_> {
        #[allow(non_snake_case)]
        fn DoEditSession(&self, ec: u32) -> Result<()> {
            // to get the current range (namely where the cursor is) you insert "nothing"
            // which genius came up with these APIs?
            let range = unsafe {
                self.context.cast::<ITfInsertAtSelection>()?
                    .InsertTextAtSelection(ec, TF_IAS_QUERYONLY, &[])?
            };

            let context_composition = self.context.cast::<ITfContextComposition>()?;
            let composition = unsafe {
                context_composition.StartComposition(
                    ec, &range, self.composition_sink)?
            };
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
        context.RequestEditSession(tid, &session, TF_ES_READWRITE)?;
        let session: &Session = session.as_impl();
        Ok(session.composition.take().expect("Composition is None."))
    }
}


pub fn end_composition(tid:u32, context: &ITfContext, composition: &ITfComposition) -> Result<()>{
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
        context.RequestEditSession(tid, &session, TF_ES_READWRITE)?;
        Ok(())
    }
}