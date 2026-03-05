## ADDED Requirements
### Requirement: Shared QA Entry
The system SHALL expose a "通用智能问答" entry in both admin and portal navigation that opens the same QA page.

#### Scenario: Entry available in both roles
- **WHEN** a user views the admin or portal primary navigation
- **THEN** the "通用智能问答" entry is visible and opens the shared QA page

### Requirement: Model Selector
The system SHALL provide a model selector on the QA page to switch between demo models.

#### Scenario: Switch model
- **WHEN** a user selects a model option
- **THEN** the current model label updates to the selected value

### Requirement: Knowledge Base Selector
The system SHALL provide a knowledge base selector on the QA page to scope demo conversations.

#### Scenario: Switch knowledge base
- **WHEN** a user selects a knowledge base option
- **THEN** the current knowledge base label updates to the selected value

### Requirement: Demo Selector Data
The system SHALL populate the model and knowledge base selectors using local mock data without outbound requests.

#### Scenario: Open QA page
- **WHEN** a user opens the QA page
- **THEN** the selectors display predefined local options
