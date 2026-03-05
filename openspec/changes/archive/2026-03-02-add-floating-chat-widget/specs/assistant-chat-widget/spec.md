## ADDED Requirements
### Requirement: Global Floating Assistant Entry
The system SHALL display a global floating assistant entry across all admin pages.

#### Scenario: Entry visible on all pages
- **WHEN** a user visits any admin route
- **THEN** the floating ball is visible in the lower right and stays above page content

### Requirement: Demo Chat Window
The system SHALL open a demo chat window when the floating entry is activated.

#### Scenario: Open and close the window
- **WHEN** a user clicks the floating ball
- **THEN** the chat window opens with mock messages and a privacy notice
- **WHEN** the user clicks the floating ball again or the overlay
- **THEN** the chat window closes

### Requirement: Demo-Only Messaging
The system SHALL keep demo chat messaging local without outbound network requests.

#### Scenario: Send a message
- **WHEN** the user submits a message
- **THEN** the message is appended locally and a mock reply is shown
