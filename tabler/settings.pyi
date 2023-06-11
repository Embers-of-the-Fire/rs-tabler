class Align:
    Left: Align
    Right: Align
    Center: Align

class Overflow:
    Hide: Overflow
    Ellipse: Overflow

class Padding:
    Empty: Padding
    def __init__(self, left: int, right: int) -> Padding:
        self.left: int
        self.right: int

class Color:
    Black: Color
    Red: Color
    Green: Color
    Yellow: Color
    Blue: Color
    Magenta: Color
    Purple: Color
    Cyan: Color
    White: Color
    BrightBlack: Color
    BrightRed: Color
    BrightGreen: Color
    BrightYellow: Color
    BrightBlue: Color
    BrightMagenta: Color
    BrightPurple: Color
    BrightCyan: Color
    BrightWhite: Color
    @staticmethod
    def from_color_name(s: str) -> Color: ...

class Formatter:
    Clear: Formatter
    Normal: Formatter
    Bold: Formatter
    Dimmed: Formatter
    Italic: Formatter
    Underline: Formatter
    Blink: Formatter
    Reversed: Formatter
    Hidden: Formatter
    Strikethrough: Formatter
    @staticmethod
    def color(color: Color) -> Formatter: ...
    @staticmethod
    def rbg_color(r: int, g: int, b: int) -> Formatter: ...
    @staticmethod
    def on_color(color: Color) -> Formatter: ...
    @staticmethod
    def on_rbg_color(r: int, g: int, b: int) -> Formatter: ...

class Border:
    Default: Border
    Horizontal: Border
    Vertical: Border
    All: Border
    def __init__(self, left: bool, right: bool, top: bool, bottom: bool) -> Border:
        self.left: bool
        self.right: bool
        self.top: bool
        self.bottom: bool

class Renderer:
    Normal: Renderer
    Raw: Renderer
    Markdown: Renderer
