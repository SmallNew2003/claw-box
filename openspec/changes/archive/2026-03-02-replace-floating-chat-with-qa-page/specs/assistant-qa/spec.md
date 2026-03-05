## ADDED Requirements
### Requirement: General QA Page Entry
The system SHALL provide a primary navigation entry named "通用智能问答".

#### Scenario: Entry available in main menu
- **WHEN** a user views the admin primary navigation
- **THEN** the "通用智能问答" entry is visible and opens the QA page

### Requirement: QA Page Layout
The system SHALL display a two-column QA layout with a session/prompt list on the left and a chat panel on the right.

#### Scenario: Open QA page
- **WHEN** a user opens the "通用智能问答" page
- **THEN** the left column shows sessions/prompts and the right column shows the chat panel

### Requirement: Demo-Only Messaging
The system SHALL keep QA demo messages local without outbound network requests.

#### Scenario: Send a message
- **WHEN** a user sends a message in the QA page
- **THEN** the message is appended locally and a mock reply is shown

### Requirement: No Floating Assistant Entry
The system SHALL NOT display the global floating assistant entry on admin pages.

#### Scenario: Admin page view
- **WHEN** a user visits any admin page
- **THEN** no floating assistant ball is displayed
