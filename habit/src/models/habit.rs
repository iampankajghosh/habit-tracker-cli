use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completions: Vec<DateTime<Utc>>,
    pub target_frequency: Option<u32>,
    pub is_active: bool,
}

impl Habit {
    pub fn new(name: String, description: Option<String>, target_frequency: Option<u32>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: Utc::now(),
            completions: Vec::new(),
            target_frequency,
            is_active: true,
        }
    }

    pub fn name_ref(&self) -> &str {
        &self.name
    }

    pub fn mark_complete(&mut self, date: DateTime<Utc>) -> bool {
        let exists = self.completions.iter().any(|d| d.date_naive() == date.date_naive());
        if exists {
            return false;
        }
        self.completions.push(date);
        self.completions.sort();
        true
    }

    pub fn recent_completions(&self) -> &[DateTime<Utc>] {
        &self.completions
    }
}

