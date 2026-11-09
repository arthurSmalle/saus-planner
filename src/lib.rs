pub mod taak;
pub mod sausid;

#[cfg(test)]
mod tests {
    use super::taak::*;
    use chrono::{Days, TimeDelta};

    fn maak_taak() {
        let huidige_tijd = chrono::Utc::now();
        let binnen_dag = huidige_tijd.checked_add_days(Days::new(1)).unwrap();
    }
}
