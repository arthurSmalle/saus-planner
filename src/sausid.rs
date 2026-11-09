// saus versie van een uuid
// tijdelijke implementatie (hopelijk geen 2 command die snel genoeg zijn zodat ze zelfde id
// krijgen...)
use chrono::{prelude::*};

pub fn get_id() -> String{
    Utc::now().timestamp_nanos_opt().expect("out of bound enkel in onhaalbare tijd (tegen dan zijn we al dood...)").to_string()
}
