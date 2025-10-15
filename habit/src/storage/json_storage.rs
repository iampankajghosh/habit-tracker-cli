use crate::models::habit::Habit;
use crate::error::{Result};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HabitStore {
    pub habits: Vec<Habit>,
}

impl HabitStore {
    pub fn load() -> Result<Self> {
        let path = storage_path();
        if !path.exists() {
            return Ok(Self { habits: Vec::new() });
        }
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let store: Self = serde_json::from_reader(reader)?;
        Ok(store)
    }

    pub fn save(&self) -> Result<()> {
        let path = storage_path();
        if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
        let tmp = path.with_extension("json.tmp");
        let mut f = File::create(&tmp)?;
        let data = serde_json::to_vec_pretty(self)?;
        f.write_all(&data)?;
        f.flush()?;
        fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn find_by_ident_mut(&mut self, ident: &str) -> Option<&mut Habit> {
        // match by UUID or name
        if let Ok(id) = ident.parse::<Uuid>() {
            self.habits.iter_mut().find(|h| h.id == id)
        } else {
            self.habits.iter_mut().find(|h| h.name.eq_ignore_ascii_case(ident))
        }
    }
}

fn storage_path() -> PathBuf {
    if let Ok(custom) = env::var("HABIT_STORAGE") {
        return PathBuf::from(custom);
    }
    PathBuf::from("habits.json")
}

