from tabler import tabler, utils

table = utils.auto_table_from(
    [
        ["123", 13],
        [
            utils.auto_cell(None),
            utils.auto_cell("777").with_formatter([tabler.settings.Formatter.Bold]),
        ],
    ]
)
table.border = tabler.settings.Border.All

print(type(table), table.table)

res = table.render(tabler.settings.Renderer.Normal)

print(res)
