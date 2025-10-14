# 📋 Habit Tracker CLI - Product Requirements Document (PRD)

## 🎯 Product Overview

**Product Name**: Habit Tracker CLI  
**Version**: 0.1.0  
**Description**: A Rust-powered command-line habit tracking application that leverages core Rust ownership, borrowing, and memory safety principles to provide a robust, type-safe habit management system with JSON persistence.

**Core Value Proposition**: Simple, reliable habit tracking with automatic JSON storage, leveraging Rust's memory safety guarantees to prevent data corruption and ensure consistent state management.

## 🎪 Key Features

### Core Functionality

1. **Habit Creation** (`add` command)

   - Create habits with name, optional description, and target frequency
   - Automatic UUID generation for unique identification
   - Timestamped creation tracking

2. **Habit Completion Tracking** (`complete` command)

   - Mark habits complete with duplicate date prevention
   - Automatic sorting of completion history
   - Lookup by ID or name

3. **Habit Listing** (`list` command)

   - Display all habits with completion status
   - Filter by active status
   - Show completion percentages and statistics

4. **Habit Management** (`remove`, `edit` commands)
   - Remove habits by ID or name
   - Edit habit details (future enhancement)

### Technical Features

- **JSON Persistence**: Automatic serialization/deserialization to `habits.json`
- **CLI Interface**: Subcommand-based parsing with `clap`
- **Error Handling**: Comprehensive custom error types with `thiserror`
- **Type Safety**: Full ownership and borrowing pattern implementation

## 🏗️ Technical Architecture

### Module Structure

```
src/
├── main.rs          # CLI entry point & command routing
├── lib.rs           # Public API re-exports
├── models/          # Habit data structures & business logic
│   └── habit.rs     # Habit struct, CompletionStatus enum
├── storage/         # Persistence layer
│   └── json_storage.rs # JSON file I/O with serde
├── cli/            # Command parsing & execution
│   └── commands.rs # Clap structs & CommandHandler trait
├── error.rs        # Custom error hierarchy
└── utils.rs        # Shared utilities & advanced patterns
```

### Data Model

```rust
struct Habit {
    id: Uuid,                    // Unique identifier
    name: String,               // Habit name (owned)
    description: Option<String>, // Optional details
    created_at: DateTime<Utc>,   // Creation timestamp
    completions: Vec<DateTime>,  // Completion history
    target_frequency: Option<u32>, // Weekly goal
    is_active: bool,            // Active/inactive status
}
```

### Storage Layer

- **File**: `habits.json` (configurable via `HABIT_STORAGE` env var)
- **Operations**: Load/Save entire habit collection
- **Search**: By ID or name with borrowing patterns
- **Error Handling**: IO and serialization errors wrapped in custom types

## 🔧 Implementation Details

### Ownership & Borrowing Patterns

- **Factory Methods**: `Habit::new()` takes ownership of input strings
- **Immutable Access**: `habit.name()` returns `&str` references
- **Mutable Operations**: `mark_complete(&mut self, date)` with validation
- **Slice Borrowing**: `recent_completions()` returns lifetime-bound slices

### Error Handling Strategy

```rust
enum HabitError {
    NotFound(String),           // Habit lookup failures
    InvalidName(String),        // Validation errors
    AlreadyCompleted(String),   // Duplicate completion prevention
    Storage(std::io::Error),    // File I/O failures
    Serialization(serde_json::Error), // JSON processing errors
}
```

### CLI Command Structure

```
habit <COMMAND>
Commands:
  add              Add new habit
    name <NAME>    Required habit name
    --description  Optional description
    --frequency    Target days per week

  list             List habits
    --active       Show only active habits (default: true)

  complete         Mark habit complete
    <IDENTIFIER>   Habit ID or name

  remove           Remove habit
    <IDENTIFIER>   Habit ID or name
```

## 🎨 User Experience

### Command Examples

```bash
# Create daily reading habit
habit add "Read 30 minutes" --description "Daily reading goal" --frequency 7

# List active habits with progress
habit list

# Complete today's reading
habit complete "Read 30 minutes"

# View all habits including inactive
habit list --active false
```

### Output Format

```
ID: 550e8400-e29b-41d4-a716-446655440000 | Read 30 minutes
  Status: 71.4% (5/7 completed)
  Description: Daily reading goal
  Created: 2025-01-15
  Completions: 5 times
```

## 📊 Success Metrics

### Technical KPIs

- **Compile Success**: 100% type-safe implementation
- **Memory Safety**: Zero runtime panics from ownership violations
- **Persistence Reliability**: 100% data integrity on save/load cycles
- **CLI Usability**: Sub-2-second command execution

### User KPIs

- **Habit Creation Rate**: Track successful `add` commands
- **Completion Consistency**: Monitor duplicate prevention effectiveness
- **Data Recovery**: Ensure no data loss on crashes

## 🔒 Constraints & Assumptions

### Technical Constraints

- **Rust Edition**: 2024 (latest stable edition)
- **Dependencies**: Minimal external crates (serde, clap, chrono, uuid)
- **Storage**: Single JSON file (no database)
- **Concurrency**: Single-threaded (no async for Week 1 scope)

### User Assumptions

- **CLI Comfort**: Users familiar with terminal commands
- **JSON Access**: Write permissions to working directory
- **UTC Timezone**: All timestamps in UTC

## 🚧 Future Enhancements (Out of Scope)

- **Async Operations**: Parallel habit processing with `tokio`
- **Database Backend**: SQLite or PostgreSQL integration
- **Web Dashboard**: REST API with `axum`
- **Advanced Analytics**: Streak calculations, habit chains
- **Sync**: Cloud synchronization across devices

## 📈 MVP Definition

### Minimum Viable Product Checklist

- [ ] `cargo build` compiles without warnings
- [ ] `habit add` creates and persists habits
- [ ] `habit list` displays formatted habit status
- [ ] `habit complete` prevents duplicate completions
- [ ] JSON persistence with automatic backup
- [ ] Comprehensive error messages for all failure modes
- [ ] Ownership/borrowing patterns correctly implemented

### Launch Criteria

1. All core commands functional end-to-end
2. Unit tests passing for model and storage layers
3. Documentation covering installation and usage
4. Example `habits.json` with seed data

## 🎯 Business Justification

**Problem Solved**: Manual habit tracking via notes/spreadsheets leads to data loss and inconsistent tracking. Existing apps often require internet connectivity and subscriptions.

**Solution**: Offline-first, type-safe CLI tool that "just works" with automatic persistence and Rust's memory safety guarantees.

**Target Users**:

- Developers who prefer CLI tools
- Privacy-conscious users avoiding cloud services
- Rust enthusiasts learning ownership patterns

**Differentiation**:

- Zero-cost, offline operation
- Compile-time memory safety guarantees
- Educational value for Rust learning
- Extensible architecture for future features

---

**Status**: MVP Ready for Development  
**Estimated Effort**: 1-2 weeks for core implementation  
**Tech Stack**: Rust 2024, Cargo, serde, clap ecosystem
