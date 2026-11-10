// saus versie van een uuid
// tijdelijke implementatie (hopelijk geen 2 command die snel genoeg zijn zodat ze zelfde id
// krijgen...)
use chrono::prelude::*;

pub struct SausidGen {
    last_id: i64,
}

impl SausidGen {
    pub fn new() -> Self {
        SausidGen { last_id: 0 }
    }

    pub fn get_saudid(&mut self) -> i64 {
        let mut id = Utc::now()
            .timestamp_nanos_opt()
            .expect("out of bound enkel in onhaalbare tijd (tegen dan zijn we al dood...)");
        if id == self.last_id {
            id += 1;
        }
        self.last_id = id;
        id
    }
}
