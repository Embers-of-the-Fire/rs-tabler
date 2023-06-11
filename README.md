# Rs-Tabler

This lib is used to format plain-text table, written in both Rust and Python.

## Example

Code:

```python
from tabler import tabler, utils

table = utils.auto_table_create(
    header = ["Header Left", utils.auto_cell("Header Right").with_align(tabler.settings.Align.Right)],
    content = [["Row", utils.auto_cell("Content").with_align(tabler.settings.Align.Right)] for _ in range(2)],
    splitter = True
)
table.border = tabler.settings.Border.All

res = table.render(tabler.settings.Renderer.Normal)

print(res)
```

Output:

```
─────────────────────────────
| Header Left  Header Right |
|───────────────────────────|
| Row               Content |
| Row               Content |
─────────────────────────────
```

> Actually the border & header of the table is bold, but it cannot be rendered in markdown.

## Future Plan

Waiting for report :)

## Change Log

### V0.1.0

Initial version.

## Origin

This project is an encapsulation based on my rust crate [table_formatter](https://crates.io/crates/table_formatter), with almost the same apis.

If you have any problems or this project has any bugs, you can make an issue on both [this repo](https://github.com/Embers-of-the-Fire/rs-tabler) as well as the [original repo](https://github.com/Embers-of-the-Fire/rust-table-formatter).

## License

[MIT][MIT-License] or [Apache-2.0][Apache-License]

[MIT-License]: LICENSE-MIT
[Apache-License]: LICENSE-APACHE
