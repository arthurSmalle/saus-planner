pub mod taak;
pub mod sausid;

#[cfg(test)]
mod tests {
    use crate::sausid;

use super::taak::*;
    use chrono::{Days, TimeDelta};

    fn maak_taak() {
        let huidige_tijd = chrono::Utc::now();
        let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    }
    #[test]
    fn test_id_collision(){
        let mut id_gen = sausid::SausidGen::new();
        let first_id = id_gen.get_saudid();
        let second_id = id_gen.get_saudid();
        assert_ne!(first_id, second_id);
    }
}
