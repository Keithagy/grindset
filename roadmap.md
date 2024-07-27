# Roadmap

## Future features

### Ergonomic Setup

- [x] `grindset` should copy in starter boilerplate
- [ ] `question.md` should include id and title of question as a header (which is also a link to the question on leetcode)
- [ ] `grindset` should output the filepath of the created attempt log, so you can do a pipe
- [ ] `init` subcommand to mark a directory as project root (e.g. with a `grindset.toml` file)
  - [ ] subsequently, `grindset` should be able to search for project root and always create new files there, instead of at cwd
  - [ ] `init <<language>>` subcommand to set up language-specific boilerplate (e.g go mod init, poetry, cargo)

### Submitting to Leetcode

- [ ] `auth` subcommand, which should take some credentials and interact with leetcode's api to save in memory (never persisted!) some auth token for some parameterized period of time.
- [ ] `submit` subcommand, which should first check that valid credentials / tokens had first been obtained by previously executing the `auth` command below.
  - [ ] do fun things with solution stats

### Improving upon Leetcode's practice tooling

- [ ] `suggest-revise` subcommand, which should look at past attempts and suggest questions to revise, depending on how long ago, passed, perceived trickiness
- [ ] `hint` subcommand, which should take a question and either feed an already-existing hint, or provide an LLM-generated one
- [ ] visualize related questions (according to leetcode) in a tui
- [ ] `summarize-progress` subcommand, which should let you see what you've done, by language/topic/date/difficult etc

### Others

- [ ] `done` subcommand, which takes you through a TUI flow to record your success, automatically record the stop datetime, etc

## Dubious possibilities

- [ ] building an llm coach thingy
- [ ] tight integration with leetcode gql atm... desirable to adapt freely between codeforces / hackerrank / neetcode etc?
- [ ] be able to define test tables in a language agnostic format and execute them (this sounds like a lot of work actually... and how should we represent graphs and such?)
