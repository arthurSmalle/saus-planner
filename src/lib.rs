use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::Path;

pub mod project;
pub mod sausid;
pub mod taak;

pub trait SaveToFile: Serialize {
    fn to_file(&self, path: &Path) -> Result<(), serde_json::Error> {
        let json = serde_json::to_string(&self)?;
        fs::write(path, json).unwrap();
        Ok(())
    }
}

pub trait GetList<T> {
    fn parse_all_files(loc: &str) -> Vec<T> {
        let mut lijst: Vec<T> = Vec::new();
        lijst
    }
}

#[cfg(test)]
mod tests {
    use crate::sausid;
    #[test]
    fn test_id_collision() {
        let mut id_gen = sausid::SausidGen::new();
        let first_id = id_gen.get_saudid();
        let second_id = id_gen.get_saudid();
        assert_ne!(first_id, second_id);
    }
}
