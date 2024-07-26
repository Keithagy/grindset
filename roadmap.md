# Roadmap

## Future features

- `grindset` should copy in starter boilerplate
- `init <<language>>` command to set up language-specific boilerplate (e.g go mod init, poetry, cargo)
- `auth`, which should take some credentials and interact with leetcode's api to save in memory (never persisted!) some auth token for some parameterized period of time.
- `submit`, which should first check that valid credentials / tokens had first been obtained by previously executing the `auth` command below.
  - do fun things with solution stats
- `suggest-revise`, which should look at past attempts and suggest questions to revise, depending on how long ago, passed, perceived trickiness
- `hint`, which should take a question and either feed an already-existing hint, or provide an LLM-generated one
- visualize related questions (according to leetcode) in a tui
- `summarize-progress`, which should let you see what you've done, by language/topic/date/difficult etc

## Dubious possibilities

- building an llm coach thingy
- tight integration with leetcode gql atm... desirable to adapt freely between codeforces / hackerrank / neetcode etc?
- be able to define test tables in a language agnostic format and execute them (this sounds like a lot of work actually... and how should we represent graphs and such?)
