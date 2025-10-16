use crate::error::{HabitError, Result};
use crate::models::habit::Habit;
use crate::storage::json_storage::HabitStore;
use chrono::Utc;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(name = "habit", version, about = "Habit Tracker CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add new habit
    Add {
        name: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        frequency: Option<u32>,
    },
    /// List habits
    List {
        #[arg(long, default_value_t = true)]
        active: bool,
    },
    /// Mark habit complete for today
    Complete { identifier: String },
    /// Remove habit
    Remove { identifier: String },
    /// Edit habit details
    Edit {
        identifier: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        frequency: Option<String>,
        #[arg(long)]
        active: Option<bool>,
    },
}

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Add {
            name,
            description,
            frequency,
        } => {
            if name.trim().is_empty() {
                return Err(HabitError::InvalidName(name));
            }
            let mut store = HabitStore::load()?;
            let habit = Habit::new(name, description, frequency);
            let id = habit.id;
            let title = habit.name.clone();
            store.habits.push(habit);
            store.save()?;
            println!("  Added habit: '{}' (ID: {})", title, id);
            Ok(())
        }
        Commands::List { active } => {
            let store = HabitStore::load()?;
            let mut any = false;
            for h in store.habits.iter().filter(|h| !active || h.is_active) {
                any = true;
                println!("ID: {} | {}", h.id, h.name);
                if let Some(desc) = &h.description {
                    println!("  Description: {}", desc);
                }
                println!("  Created: {}", h.created_at.date_naive());
                println!("  Completions: {}/{} days", h.completions.len(), h.target_frequency.unwrap_or(0));
                if let Some(freq) = h.target_frequency {
                    println!("  Target: {} days", freq);
                }
                println!("  Active: {}", h.is_active);
            }
            if !any {
                println!("  No habits to display (active = {})", active);
            }
            Ok(())
        }
        Commands::Complete { identifier } => {
            let mut store = HabitStore::load()?;
            let Some(habit) = store.find_by_ident_mut(&identifier) else {
                return Err(HabitError::NotFound(identifier));
            };
            let ok = habit.mark_complete(Utc::now());
            if !ok {
                return Err(HabitError::AlreadyCompleted(habit.name.clone()));
            }
            let name = habit.name.clone();
            store.save()?;
            println!("✅ Marked complete: '{}' (today)", name);
            Ok(())
        }
        Commands::Remove { identifier } => {
            let mut store = HabitStore::load()?;
            let before = store.habits.len();
            store.habits.retain(|h| {
                if let Ok(id) = identifier.parse::<Uuid>() {
                    h.id != id
                } else {
                    !h.name.eq_ignore_ascii_case(&identifier)
                }
            });
            if store.habits.len() == before {
                return Err(HabitError::NotFound(identifier));
            }
            store.save()?;
            println!("🗑️  Removed habit: {}", identifier);
            Ok(())
        }
        Commands::Edit {
            identifier,
            name,
            description,
            frequency,
            active,
        } => {
            let mut store = HabitStore::load()?;
            let Some(habit) = store.find_by_ident_mut(&identifier) else {
                return Err(HabitError::NotFound(identifier));
            };
            if let Some(new_name) = name {
                if new_name.trim().is_empty() {
                    return Err(HabitError::InvalidName(new_name));
                }
                habit.name = new_name;
            }
            if let Some(desc) = description {
                if desc.eq_ignore_ascii_case("null") {
                    habit.description = None;
                } else {
                    habit.description = Some(desc);
                }
            }
            if let Some(freq_str) = frequency {
                if freq_str.eq_ignore_ascii_case("null") {
                    habit.target_frequency = None;
                } else {
                    let parsed: u32 = freq_str
                        .parse()
                        .map_err(|_| HabitError::InvalidName("frequency".into()))?;
                    habit.target_frequency = Some(parsed);
                }
            }
            if let Some(is_active) = active {
                habit.is_active = is_active;
            }
            let name_out = habit.name.clone();
            store.save()?;
            println!("✏️  Updated habit: '{}'", name_out);
            Ok(())
        }
    }
}
