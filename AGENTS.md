# lib-type-handl

Type: pure-handl predicates and kind guards — Handl stdlib port.

## Layout

| path | role |
| --- | --- |
| `src/type-handl.hnl` | the library |
| `src/type-handl.test.hnl` | the .hnl test suite |
| `pkg.hnl` | package manifest — `modules: ["type-handl"]` |
| `scripts/hnl-gate.sh`, `ci.sh` | the gate / bupha entry |

## Contract

- Pure-handl type predicates/kind guards; sibling of `lib-type`
  (which carries the native `typeof.*` ops).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
