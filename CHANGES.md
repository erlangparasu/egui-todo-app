# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

## [v1.0.4] - 2026-06-22

### Added

#### Tag Filter System
- **Tag Checkboxes** - Display used tags as checkboxes below Search bar
- **Multi-tag Filtering** - Select multiple tags to filter tasks (OR logic)
- **Used Tags Only** - Only tags assigned to at least one active todo are shown
- **Dynamic Tag List** - Tag filter list refreshes when tags are added/removed
- **Combined Filtering** - Tag filters work together with title/description search

#### UI Improvements
- **Dynamic Button Labels** - "Mark Done"/"Mark Pending" based on completion status
- **Dynamic Lock Button** - "Lock"/"Unlock" based on readonly status
- **Priority Dropdown in Detail View** - Change priority directly from detail view (disabled when readonly)
- **Create Button Placement** - Create button placed immediately after Priority input
- **ScrollArea ID Fix** - Added `ui.push_id` to prevent widget ID clashes

### Changed

#### Tag System
- **Used Tags Only** - Tag filter now only shows tags that are assigned to at least one active todo
- **get_used_tags Function** - New database function to fetch only used tags

### Fixed

- **Impl Block Structure** - Fixed nested impl block issue with Default trait
- **ScrollArea ID Clash** - Fixed duplicate widget ID error for ScrollArea

## [v1.0.3] - 2026-06-22

### Added
- CHANGES.md changelog file documenting all features

### Infrastructure
- GitHub Actions workflow for cross-platform builds
  - Linux (x86_64-unknown-linux-gnu)
  - macOS (x86_64-apple-darwin, aarch64-apple-darwin)
  - Windows (x86_64-pc-windows-msvc)
- fail-fast: false for independent platform builds

## [v1.0.2] - 2026-06-22

### Infrastructure
- Fixed GitHub Actions rust action name (dtolnay/rust-action → dtolnay/rust-toolchain)

## [v1.0.1] - 2026-06-22

### Infrastructure
- Initial GitHub Actions workflow setup

## [v1.0.0] - 2026-06-22

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
- **Remove Tags** - Remove tags from todo items with x button
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
- Reordered detail view buttons: Back, Mark Done/Pending, Delete, Lock/Unlock, Edit
- Edit button only visible when item is not readonly
- Priority displayed as `#P{N}` format in list
- Readonly status shown as "Yes 🔒" or "No" in detail view
- Completion status shown as "✓" checkmark in list
- Search input field added to list view

### Database Schema

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    completed INTEGER DEFAULT 0,
    readonly INTEGER DEFAULT 0,
    priority INTEGER DEFAULT 3,
    order_index INTEGER DEFAULT 0,
    creation_date INTEGER NOT NULL,
    changed_date INTEGER NOT NULL,
    deletion_date INTEGER
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE todo_tags (
    todo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    FOREIGN KEY (todo_id) REFERENCES todos(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
```

### Dependencies

- **eframe 0.31** - egui framework for GUI
- **egui 0.31** - Immediate mode GUI library
- **chrono 0.4** - Date/time handling
- **rusqlite 0.32** - SQLite bindings with bundled feature
- **rfd 0.15** - Native file dialogs

### Project Structure

```
egui-todo-app/
├── src/
│   ├── main.rs          # Main application code
│   └── database.rs      # SQLite database operations
├── migrations/          # SQL migration files
├── .github/
│   └── workflows/
│       └── build.yml    # GitHub Actions workflow
├── Cargo.toml           # Rust dependencies
├── README.md           # Project documentation
└── CHANGES.md          # Changelog
```