
from tabler import tabler, utils

table = utils.auto_table_create(
    header = ["Header Left", utils.auto_cell("Header Right").with_align(tabler.settings.Align.Right)],
    content = [["Row", utils.auto_cell("Content").with_align(tabler.settings.Align.Right)] for _ in range(2)],
    splitter = True
)
table.border = tabler.settings.Border.All

res = table.render(tabler.settings.Renderer.Normal)

print(res)