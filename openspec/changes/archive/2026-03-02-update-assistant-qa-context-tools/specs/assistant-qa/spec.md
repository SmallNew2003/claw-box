## ADDED Requirements
### Requirement: Context Entry Controls
The system SHALL provide entry controls to add file attachments and web links as context in the QA composer area.

#### Scenario: Add context entry
- **WHEN** a user clicks the attachment or link control
- **THEN** a new context item is added to the context list

### Requirement: Context List
The system SHALL display a context list that includes attachments and links, and allow removing items.

#### Scenario: Remove context item
- **WHEN** a user removes a context item
- **THEN** the item is removed from the list

### Requirement: Prompt Optimization
The system SHALL provide a prompt optimization control that suggests an improved question based on the current input.

#### Scenario: Optimize prompt
- **WHEN** a user triggers prompt optimization
- **THEN** the system shows an optimized prompt text for the user to use

### Requirement: Demo-Only Context
The system SHALL keep context actions local without outbound requests.

#### Scenario: Demo only behavior
- **WHEN** a user adds or removes context items
- **THEN** the changes occur locally without network requests
