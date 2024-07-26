# Planning doc for grindset buddy

## Prompt for first cut of CLI

Write me a simple cli tool in rust, called `grindset`, which would allow me to more easily track my leetcode progress.
The tool should provide the following commands:

- when executed without any commands, should create boilerplate files for a new problem attempt, organized correctly in file structure which would be set up by the `init` command below.
  - It should take 2 arguments:
    - title slug for leetcode problem
    - language name extension WITHOUT the `.` (e.g. `py`, `ts`, `go`, `c`)
  - It should use the problem url, extract the title slug, and interact with the leetcode graphql api (postman collection and graphql queries attached) to set up various boilerplate per the notes below on file structure
  - toml should be auto-populated with reasonable values (e.g. attempt start datetime)
- `init`, which should set up any required file structure

## Plan

- use file system to keep attempts, so editor access is easy + composing with other terminal tools is easy

### File structure (`backticks` denote variables, asterisk denotes iterable set of values, / denotes folder)

- `language`\*/
  - `question_id+question_name`\*/
    - question.md
    - question_attributes.toml
      - difficulty
      - topics
    - `attempt_date_time`\*/
      - attempt.`language_code`
      - result_attributes.toml

### Things to record about an attempt

- success (bool)
- perceived trickiness (int; 1-7 point scale, 1 is brain-dead, 7 is devious)
- attempt start datetime (timestamp)
- attempt end datetime (timestamp)
- reflections (multiline string)
