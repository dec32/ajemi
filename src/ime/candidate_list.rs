use log::trace;

pub struct CandidateList {
    
}

impl CandidateList {
    pub fn new() -> CandidateList {
        CandidateList{}
    }

    pub fn locate(&mut self, pos: (i32, i32)) {
        trace!("locate({}, {})", pos.0, pos.1)
    }
    
    pub fn show(&self, text: &str) {
        // now all you need todo is find a UI framework that supports putting a floating panel 
        // at any given position
    }

    pub fn hide(&self) {
        
    }
}

