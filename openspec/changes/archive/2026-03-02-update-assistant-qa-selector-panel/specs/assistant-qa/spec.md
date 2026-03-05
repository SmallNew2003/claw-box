## ADDED Requirements
### Requirement: Selector Placement
The system SHALL display model, knowledge base, and agent selector pills directly below the message input area.

#### Scenario: Composer toolbar visible
- **WHEN** a user views the QA composer area
- **THEN** selector pills for model, knowledge base, and agent are visible under the input

### Requirement: Selector Panel Interaction
The system SHALL open a lightweight selection panel when a selector pill is clicked and update the current selection when an option is chosen.

#### Scenario: Open panel and select option
- **WHEN** a user clicks a selector pill and chooses an option
- **THEN** the panel closes and the pill label updates to the selected value

### Requirement: Panel Close Behavior
The system SHALL close the selector panel when the user clicks outside the panel or presses ESC.

#### Scenario: Dismiss panel
- **WHEN** a panel is open and the user clicks outside or presses ESC
- **THEN** the panel closes without changing the current selection

### Requirement: Knowledge Base Panel Actions
The system SHALL show "清除" and "添加知识库..." action rows in the knowledge base panel for demo purposes.

#### Scenario: Open knowledge base panel
- **WHEN** a user opens the knowledge base selector panel
- **THEN** the panel lists action rows for clearing and adding a knowledge base
