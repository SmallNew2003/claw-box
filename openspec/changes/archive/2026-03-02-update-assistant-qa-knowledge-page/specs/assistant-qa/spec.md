## ADDED Requirements
### Requirement: Knowledge Management View
The system SHALL provide a knowledge management view within the QA page that matches the reference layout and excludes model embedding controls.

#### Scenario: Switch to knowledge management
- **WHEN** a user selects the knowledge management entry
- **THEN** the knowledge management view is shown without model embedding controls

### Requirement: Mini Sidebar Selection
The system SHALL show a selected state for the active mini sidebar entry.

#### Scenario: Sidebar entry selected
- **WHEN** a user switches between "对话" and "知识库管理"
- **THEN** the active entry is highlighted
