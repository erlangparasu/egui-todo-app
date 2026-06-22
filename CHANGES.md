# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

### Added

#### Core Features
- **Create Todo Items** - Create new todo items with title and description
- **Edit Todo Items** - Modify existing todo title and description (disabled when readonly)
- **List Todo Items** - View all active todo items in a scrollable list
- **Show Detail View** - View complete details of a todo item
- **Delete Todo Items** - Soft delete functionality (moves to trash)
- **Mark as Completed** - Toggle completion status with visual checkmark indicator
- **Mark as Readonly** - Lock todo items from editing (Toggle RO button)

#### Priority System
- **Priority Levels (P1-P5)** - Each todo item has a priority level
  - P1 = Highest priority
  - P5 = Lowest priority
  - Default priority is P3
- Priority is displayed as a badge in the list view (e.g., `#P2`)
- Priority can be set when creating a new todo item

#### Reordering
- **Move Up/Down Buttons** - Reorder todo items using ^ and v buttons
- Buttons appear in the task list next to each item
- Reordering persists to database via order_index field

#### Tags System
- **Add Tags** - Attach multiple tags to todo items
- **Remove Tags** - Remove tags from todo items (planned)
- **Tag Display** - Tags shown as chips with x button for removal
- **Tag Persistence** - Tags stored in separate `tags` and `todo_tags` tables
- **Tag Autocomplete** - Tags are reused across todos

#### Search
- **Search by Title** - Filter todo items by title substring
- **Search by Description** - Filter todo items by description substring
- **Real-time Filtering** - Results update as you type
- **Result Counter** - Shows "Showing X/Y tasks" indicator

#### Trash System
- **Soft Delete** - Deleted items go to trash (deletion_date set)
- **Trash View** - Dedicated view showing all trashed items
- **Restore** - Restore items from trash back to active list
- **Permanent Delete** - Completely remove item from database
- **Empty Trash** - Clear all items in trash at once
- **Trash Counter** - Badge shows number of items in trash

#### Export Database
- **Native File Dialog** - Cross-platform save dialog using rfd crate
- **Full Database Export** - Exports complete SQLite database file
- Works on Windows, macOS, and Linux

#### Database Schema
- **todos table** - Main todo items storage
  - id (PRIMARY KEY)
  - title (TEXT NOT NULL)
  - description (TEXT)
  - completed (INTEGER DEFAULT 0)
  - readonly (INTEGER DEFAULT 0)
  - priority (INTEGER DEFAULT 3)
  - order_index (INTEGER DEFAULT 0)
  - creation_date (INTEGER NOT NULL)
  - changed_date (INTEGER NOT NULL)
  - deletion_date (INTEGER)

- **tags table** - Tag definitions
  - id (PRIMARY KEY)
  - name (TEXT NOT NULL UNIQUE)

- **todo_tags table** - Many-to-many relationship
  - todo_id (FOREIGN KEY)
  - tag_id (FOREIGN KEY)
  - PRIMARY KEY (todo_id, tag_id)

#### Date Tracking
- **Creation Date** - Automatically set when item is created
- **Changed Date** - Updated on any modification
- **Deletion Date** - Set when item is soft-deleted
- **Date Formatting** - Displayed as "YYYY-MM-DD HH:MM"

#### UI Features
- **Task List View** - Main view showing all active todos
- **Detail View** - Full information display for selected todo
- **Edit View** - Form for modifying todo properties
- **Trash View** - Separate view for deleted items
- **Navigation Buttons** - Tasks and Trash buttons in header
- **Button Sizing** - Consistent button sizes for usability

### Changed

#### UI Improvements
- Reordered detail view buttons: Back, Toggle Done, Delete, Toggle RO, Edit
- Edit button only visible when item is not readonly
- Priority displayed as `#P{N}` format in list
- Readonly status shown as "Yes 🔒" or "No" in detail view
- Completion status shown as "✓" checkmark in list
- Search input field added to list view

#### Database Changes
- Added `priority` column (INTEGER DEFAULT 3)
- Added `order_index` column (INTEGER DEFAULT 0)
- Created `tags` table for tag management
- Created `todo_tags` junction table for many-to-many relationship

### Fixed

- Borrow checker issues in move_task function
- Closure capture issues in list view rendering
- Unused imports and dead code warnings

### Infrastructure

#### GitHub Actions CI/CD
- **Cross-platform builds** - Compiles for Windows, macOS, and Linux
- **Multiple targets**:
  - Linux: x86_64-unknown-linux-gnu
  - macOS: x86_64-apple-darwin, aarch64-apple-darwin
  - Windows: x86_64-pc-windows-msvc
- **Fail-fast disabled** - Each platform builds independently
- **Artifact upload** - Each build produces downloadable artifact
- **Release creation** - Tags trigger automatic GitHub releases

#### Project Structure
```
egui-todo-app/
├── src/
│   ├── main.rs          # Main application code
│   └── database.rs      # SQLite database operations
├── migrations/          # SQL migration files
│   ├── 001_init.sql
│   ├── 002_add_readonly.sql
│   ├── 003_export_feature.sql
│   └── 004_add_tags.sql
├── .github/
│   └── workflows/
│       └── build.yml    # GitHub Actions workflow
├── Cargo.toml           # Rust dependencies
├── README.md           # Project documentation
└── todo.db             # SQLite database (gitignored)
```

### Dependencies

- **eframe 0.31** - egui framework for GUI
- **egui 0.31** - Immediate mode GUI library
- **chrono 0.4** - Date/time handling
- **rusqlite 0.32** - SQLite bindings with bundled feature
- **rfd 0.15** - Native file dialogs

## Version History

### v0.1.0 (Initial Release)
- Basic CRUD operations
- SQLite persistence
- Soft delete with trash
- Export database feature