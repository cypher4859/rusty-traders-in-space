Tools to use for prettyprinting

| Need                               | Crate(s)                             | What you get                                            | One-line install                                      |
| ---------------------------------- | ------------------------------------ | ------------------------------------------------------- | ----------------------------------------------------- |
| **Colorized & styled text**        | `owo-colors` · `colored` · `console` | Cross-platform ANSI colors, bold/italic, RGB support.   | `owo-colors = "4"`                                    |
| **Progress bars & spinners**       | `indicatif`                          | Multi-line progress bars, spinners, ETA, byte-thruput.  | `indicatif = "0.17"`                                  |
| **Pretty tables**                  | `comfy-table` · `tabled`             | Unicode borders, alignment, auto-wrap, colors.          | `comfy-table = "7"`                                   |
| **Interactive prompts**            | `inquire` · `dialoguer`              | Yes/No, select menus, password prompts, multi-select.   | `inquire = "0.6"`                                     |
| **Dashboards / TUIs**              | `ratatui` (fork of tui-rs)           | Full-screen widgets: charts, lists, gauges—like `htop`. | `ratatui = "0.26"`                                    |
| **Syntax-highlighted JSON / YAML** | `bat` (lib: `bat`) or `nu-ansi-term` | Color-highlight and paginate any text blob.             | `bat = { version = "0.24", features=["serde_json"] }` |



### Done Commands
These command groups are finished with the exception of error handling, cleanup, etc. Simply put: These are just wired up to their respective HTTP call. Currently the agent de/activation logic needs improved, i.e. I'm actually hardcoding the agent to use and that needs to be dynamic
- [X] Factions
- [-] Agents
- [X] Contracts
- [] Fleet
- [X] Systems
- [X] Waypoints
- [X] Data
- [X] Global