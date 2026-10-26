# FratmScript plugin for AI agents

A [Claude Code](https://code.claude.com) plugin that teaches agents to write FratmScript and lets them check and run it:

- the `fratmscript` skill: language rules, keyword table, example and workflow;
- the `fratm` MCP server (`fratm mcp`): tools `fratm_reference`, `fratm_check`, `fratm_compile`, `fratm_run`, `fratm_format`, `fratm_gomorra` (SQL in Neapolitan), `fratm_stile` (CSS in Neapolitan) and `fratm_nuovo` (the files of a new web app).

## Install

The MCP server needs the `fratm` CLI on your `PATH`:

```bash
curl -fsSL https://raw.githubusercontent.com/kkratterf/fratmscript/main/install.sh | sh
```

Then, in Claude Code:

```text
/plugin marketplace add kkratterf/fratmscript
/plugin install fratmscript@fratmscript
```

Other agents: the skill in [`skills/fratmscript/SKILL.md`](skills/fratmscript/SKILL.md) follows the open Agent Skills format, and any MCP client can run the server as the command `fratm` with the argument `mcp`. The full reference is also online at https://kkratterf.github.io/fratmscript/llms-full.txt.
