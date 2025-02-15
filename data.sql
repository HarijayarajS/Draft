Your SQL contains syntax errors, such as incorrect column names (e.g., created on should be created_on), missing closing parentheses, and improperly formatted foreign key references. Below is the corrected version of your table creation queries along with sample INSERT statements to populate each table with five records.


---

Corrected Table Creation Queries

CREATE TABLE IF NOT EXISTS ai_agent (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    modified_on TIMESTAMP DEFAULT now(),
    title VARCHAR(128) NOT NULL,
    identifier VARCHAR(32) NOT NULL,
    prompt TEXT NOT NULL,
    models VARCHAR(128) NOT NULL,
    is_active BOOLEAN NOT NULL,
    sort_no SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_agent_history (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    modified_on TIMESTAMP DEFAULT now(),
    employee_id BIGINT NOT NULL REFERENCES employee(id) ON DELETE CASCADE ON UPDATE CASCADE,
    name VARCHAR(128) NOT NULL,
    ai_agent_id BIGINT NOT NULL REFERENCES ai_agent(id) ON DELETE CASCADE ON UPDATE CASCADE,
    status VARCHAR(24) NOT NULL,
    input_message TEXT NOT NULL,
    input_file VARCHAR(128),
    content_size BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_agent_history_output (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    modified_on TIMESTAMP DEFAULT now(),
    ai_agent_history_id BIGINT NOT NULL REFERENCES ai_agent_history(id) ON DELETE CASCADE ON UPDATE CASCADE,
    model VARCHAR(128) NOT NULL,
    execution_time BIGINT,
    output TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_agent_access (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    created_on TIMESTAMP DEFAULT now(),
    modified_on TIMESTAMP DEFAULT now(),
    ai_agent_id BIGINT NOT NULL REFERENCES ai_agent(id) ON DELETE CASCADE ON UPDATE CASCADE,
    employee_id BIGINT NOT NULL REFERENCES employee(id) ON DELETE CASCADE ON UPDATE CASCADE
);


---

Insert Sample Data (5 Records Per Table)

1. Insert Sample Data into ai_agent

INSERT INTO ai_agent (title, identifier, prompt, models, is_active, sort_no)
VALUES
('ChatGPT', 'gpt-4', 'Help users with queries', 'OpenAI GPT-4', TRUE, 1),
('ImageGen', 'dalle-2', 'Generate AI images', 'OpenAI DALL-E', TRUE, 2),
('CodeGen', 'codex', 'Write and debug code', 'OpenAI Codex', TRUE, 3),
('DataAnalysis', 'pandas-ai', 'Analyze data efficiently', 'Pandas AI', TRUE, 4),
('VoiceBot', 'whisper', 'Convert speech to text', 'OpenAI Whisper', TRUE, 5);

2. Insert Sample Data into ai_agent_history

INSERT INTO ai_agent_history (employee_id, name, ai_agent_id, status, input_message, input_file, content_size)
VALUES
(1, 'John Doe', 1, 'Completed', 'How do I use AI?', NULL, 1024),
(2, 'Jane Smith', 2, 'Processing', 'Generate a cat image.', NULL, 2048),
(3, 'Alice Johnson', 3, 'Failed', 'Fix this Python error.', NULL, 4096),
(4, 'Bob Brown', 4, 'Completed', 'Analyze this dataset.', 'data.csv', 8192),
(5, 'Charlie White', 5, 'Pending', 'Transcribe this audio.', 'audio.mp3', 16384);

3. Insert Sample Data into ai_agent_history_output

INSERT INTO ai_agent_history_output (ai_agent_history_id, model, execution_time, output)
VALUES
(1, 'GPT-4', 1500, 'AI can assist you with various tasks.'),
(2, 'DALL-E', 2300, 'Generated image of a cat.'),
(3, 'Codex', 1200, 'Syntax error fixed: missing semicolon.'),
(4, 'Pandas AI', 2500, 'Data analyzed: 2000 rows processed.'),
(5, 'Whisper', 3400, 'Transcription: "Hello, how are you?"');

4. Insert Sample Data into ai_agent_access

INSERT INTO ai_agent_access (ai_agent_id, employee_id)
VALUES
(1, 1),
(2, 2),
(3, 3),
(4, 4),
(5, 5);


---

Summary of Fixes:

1. Corrected Column Names

Changed created on → created_on

Changed modified on → modified_on

Changed sort no → sort_no

Fixed ai agen history id → ai_agent_history_id

Fixed ai agent id → ai_agent_id

Fixed employee id → employee_id



2. Fixed Syntax Errors

Added missing commas

Ensured REFERENCES statements correctly referenced foreign keys



3. Added DEFAULT NULL for Optional Fields

input_file in ai_agent_history can be NULL




Now, this script should work correctly in PostgreSQL. Let me know if you need further modifications!

