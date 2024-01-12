use windows::{core::{implement,Result}, Win32::UI::TextServices::{ITfCompositionSink,ITfCompositionSink_Impl, ITfComposition}};

#[implement(ITfCompositionSink)]
pub struct CompositionSink;

impl ITfCompositionSink_Impl for CompositionSink {
    #[allow(non_snake_case)]
    fn OnCompositionTerminated(&self, _ecwrite:u32, _composition: Option<&ITfComposition>) -> Result<()> {
        Ok(())
    }
}