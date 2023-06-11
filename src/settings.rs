//! Settings for rendering.

use pyo3::prelude::*;
use table_formatter::table;

use crate::errors::FormatterError;

pub fn regist_classes(py: Python, m: &PyModule) -> PyResult<()> {
    let setting_module = PyModule::new(py, "settings")?;
    Align::regist_self(py, setting_module)?;
    Overflow::regist_self(py, setting_module)?;
    Padding::regist_self(py, setting_module)?;
    Color::regist_self(py, setting_module)?;
    Formatter::regist_self(py, setting_module)?;
    Border::regist_self(py, setting_module)?;
    Renderer::regist_self(py, setting_module)?;
    m.add_submodule(setting_module)?;
    Ok(())
}

pub enum FormatFunc<T, U> {
    Anonymous(Box<dyn Fn(T) -> U>),
    Normal(fn(T) -> U),
}

impl<T, U> FormatFunc<T, U> {
    pub fn run(&self, p: T) -> U {
        match self {
            Self::Anonymous(ref f) => f(p),
            Self::Normal(ref f) => f(p),
        }
    }
}

/// Wrapper for formatting-functions.
/// 
/// Use `Formatter.color` to create a color for text.
/// 
/// Use `Formatter.on_color` to create a color for background.
#[derive(Clone, Copy)]
#[pyclass]
pub struct Formatter {
    formatter: _Formatter,
}

#[pymethods]
impl Formatter {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Clear: Formatter = Formatter::new(_Formatter::Clear);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Normal: Formatter = Formatter::new(_Formatter::Normal);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Bold: Formatter = Formatter::new(_Formatter::Bold);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Dimmed: Formatter = Formatter::new(_Formatter::Dimmed);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Italic: Formatter = Formatter::new(_Formatter::Italic);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Underline: Formatter = Formatter::new(_Formatter::Underline);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Blink: Formatter = Formatter::new(_Formatter::Blink);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Reversed: Formatter = Formatter::new(_Formatter::Reversed);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Hidden: Formatter = Formatter::new(_Formatter::Hidden);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Strikethrough: Formatter = Formatter::new(_Formatter::Strikethrough);

    #[staticmethod]
    #[inline]
    pub const fn color(color: Color) -> Formatter {
        Formatter::new(_Formatter::Color(_Color::Color(color)))
    }
    #[staticmethod]
    #[inline]
    pub const fn rbg_color(r: u8, g: u8, b: u8) -> Formatter {
        Formatter::new(_Formatter::Color(_Color::TrueColor { r, g, b }))
    }

    #[staticmethod]
    #[inline]
    pub const fn on_color(color: Color) -> Formatter {
        Formatter::new(_Formatter::OnColor(_Color::Color(color)))
    }
    #[staticmethod]
    #[inline]
    pub const fn on_rbg_color(r: u8, g: u8, b: u8) -> Formatter {
        Formatter::new(_Formatter::OnColor(_Color::TrueColor { r, g, b }))
    }

    pub fn __repr__(&self) -> String {
        format!("<settings.Formatter({})>", self.formatter.repr())
    }
}

impl Formatter {
    const fn new(fmt: _Formatter) -> Self {
        Self { formatter: fmt }
    }

    pub fn as_func(&self) -> FormatFunc<colored::ColoredString, colored::ColoredString> {
        self.formatter.as_func()
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Formatter>()?;
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum _Formatter {
    Color(_Color),
    OnColor(_Color),
    Clear,
    Normal,
    Bold,
    Dimmed,
    Italic,
    Underline,
    Blink,
    Reversed,
    Hidden,
    Strikethrough,
}

impl _Formatter {
    fn repr(&self) -> String {
        match self {
            Self::Color(c) => format!("Color({})", c.repr()),
            Self::OnColor(c) => format!("OnColor({})", c.repr()),
            Self::Clear => "Clear".to_string(),
            Self::Normal => "Normal".to_string(),
            Self::Bold => "Bold".to_string(),
            Self::Dimmed => "Dimmed".to_string(),
            Self::Italic => "Italic".to_string(),
            Self::Underline => "Underline".to_string(),
            Self::Blink => "Blink".to_string(),
            Self::Reversed => "Reversed".to_string(),
            Self::Hidden => "Hidden".to_string(),
            Self::Strikethrough => "Strikethrough".to_string(),
        }
    }

    fn as_func(&self) -> FormatFunc<colored::ColoredString, colored::ColoredString> {
        use colored::Colorize;
        match self {
            Self::Clear => FormatFunc::Normal(Colorize::clear),
            Self::Normal => FormatFunc::Normal(Colorize::normal),
            Self::Bold => FormatFunc::Normal(Colorize::bold),
            Self::Dimmed => FormatFunc::Normal(Colorize::dimmed),
            Self::Italic => FormatFunc::Normal(Colorize::italic),
            Self::Underline => FormatFunc::Normal(Colorize::underline),
            Self::Blink => FormatFunc::Normal(Colorize::blink),
            Self::Reversed => FormatFunc::Normal(Colorize::reversed),
            Self::Hidden => FormatFunc::Normal(Colorize::hidden),
            Self::Strikethrough => FormatFunc::Normal(Colorize::strikethrough),
            Self::Color(c) => match *c {
                _Color::TrueColor { r, g, b } => {
                    FormatFunc::Anonymous(Box::new(move |string: colored::ColoredString| {
                        string.truecolor(r, g, b)
                    }))
                }
                _Color::Color(ref color) => match color {
                    Color::Black => FormatFunc::Normal(Colorize::black),
                    Color::Red => FormatFunc::Normal(Colorize::red),
                    Color::Green => FormatFunc::Normal(Colorize::green),
                    Color::Yellow => FormatFunc::Normal(Colorize::yellow),
                    Color::Blue => FormatFunc::Normal(Colorize::blue),
                    Color::Magenta => FormatFunc::Normal(Colorize::magenta),
                    Color::Purple => FormatFunc::Normal(Colorize::purple),
                    Color::Cyan => FormatFunc::Normal(Colorize::cyan),
                    Color::White => FormatFunc::Normal(Colorize::white),
                    Color::BrightBlack => FormatFunc::Normal(Colorize::bright_black),
                    Color::BrightRed => FormatFunc::Normal(Colorize::bright_red),
                    Color::BrightGreen => FormatFunc::Normal(Colorize::bright_green),
                    Color::BrightYellow => FormatFunc::Normal(Colorize::bright_yellow),
                    Color::BrightBlue => FormatFunc::Normal(Colorize::bright_blue),
                    Color::BrightMagenta => FormatFunc::Normal(Colorize::bright_magenta),
                    Color::BrightPurple => FormatFunc::Normal(Colorize::bright_purple),
                    Color::BrightCyan => FormatFunc::Normal(Colorize::bright_cyan),
                    Color::BrightWhite => FormatFunc::Normal(Colorize::bright_white),
                },
            },
            Self::OnColor(c) => match *c {
                _Color::TrueColor { r, g, b } => {
                    FormatFunc::Anonymous(Box::new(move |string: colored::ColoredString| {
                        string.on_truecolor(r, g, b)
                    }))
                }
                _Color::Color(ref color) => match color {
                    Color::Black => FormatFunc::Normal(Colorize::on_black),
                    Color::Red => FormatFunc::Normal(Colorize::on_red),
                    Color::Green => FormatFunc::Normal(Colorize::on_green),
                    Color::Yellow => FormatFunc::Normal(Colorize::on_yellow),
                    Color::Blue => FormatFunc::Normal(Colorize::on_blue),
                    Color::Magenta => FormatFunc::Normal(Colorize::on_magenta),
                    Color::Purple => FormatFunc::Normal(Colorize::on_purple),
                    Color::Cyan => FormatFunc::Normal(Colorize::on_cyan),
                    Color::White => FormatFunc::Normal(Colorize::on_white),
                    Color::BrightBlack => FormatFunc::Normal(Colorize::on_bright_black),
                    Color::BrightRed => FormatFunc::Normal(Colorize::on_bright_red),
                    Color::BrightGreen => FormatFunc::Normal(Colorize::on_bright_green),
                    Color::BrightYellow => FormatFunc::Normal(Colorize::on_bright_yellow),
                    Color::BrightBlue => FormatFunc::Normal(Colorize::on_bright_blue),
                    Color::BrightMagenta => FormatFunc::Normal(Colorize::on_bright_magenta),
                    Color::BrightPurple => FormatFunc::Normal(Colorize::on_bright_purple),
                    Color::BrightCyan => FormatFunc::Normal(Colorize::on_bright_cyan),
                    Color::BrightWhite => FormatFunc::Normal(Colorize::on_bright_white),
                },
            },
        }
    }
}

#[derive(Clone, Copy)]
enum _Color {
    Color(Color),
    TrueColor { r: u8, g: u8, b: u8 },
}

impl _Color {
    fn repr(&self) -> String {
        match self {
            Self::TrueColor { r, g, b } => format!("<TrueColor(r: {}, g: {}, b: {})>", r, g, b),
            Self::Color(c) => c.repr(),
        }
    }
}

/// Pre-defined colors.
#[derive(Clone, Copy)]
#[pyclass]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightPurple,
    BrightCyan,
    BrightWhite,
}

#[pymethods]
impl Color {
    #[staticmethod]
    pub fn from_color_name(s: String) -> PyResult<Color> {
        let local_str = s.replace(' ', "").to_ascii_lowercase();
        match local_str.as_str() {
            "black" => Ok(Self::Black),
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            "blue" => Ok(Self::Blue),
            "magenta" => Ok(Self::Magenta),
            "purple" => Ok(Self::Purple),
            "cyan" => Ok(Self::Cyan),
            "white" => Ok(Self::White),
            "brightblack" => Ok(Self::BrightBlack),
            "brightred" => Ok(Self::BrightRed),
            "brightgreen" => Ok(Self::BrightGreen),
            "brightyellow" => Ok(Self::BrightYellow),
            "brightblue" => Ok(Self::BrightBlue),
            "brightmagenta" => Ok(Self::BrightMagenta),
            "brightpurple" => Ok(Self::BrightPurple),
            "brightcyan" => Ok(Self::BrightCyan),
            "brightwhite" => Ok(Self::BrightWhite),
            _ => Err(FormatterError::new(format!("Invalid Color Name: {}", s)).into()),
        }
    }

    pub fn __repr__(&self) -> String {
        self.repr()
    }
}

impl Color {
    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Color>()?;
        Ok(())
    }

    pub fn repr(&self) -> String {
        format!(
            "<settings.Color({})>",
            match self {
                Self::Black => "black",
                Self::Red => "red",
                Self::Green => "green",
                Self::Yellow => "yellow",
                Self::Blue => "blue",
                Self::Magenta => "magenta",
                Self::Purple => "purple",
                Self::Cyan => "cyan",
                Self::White => "white",
                Self::BrightBlack => "bright black",
                Self::BrightRed => "bright red",
                Self::BrightGreen => "bright green",
                Self::BrightYellow => "bright yellow",
                Self::BrightBlue => "bright blue",
                Self::BrightMagenta => "bright magenta",
                Self::BrightPurple => "bright purple",
                Self::BrightCyan => "bright cyan",
                Self::BrightWhite => "bright white",
            }
        )
    }
}

/// Padding around the content.
/// 
/// A string `"hello"` with `Padding{ left: 1, right: 1 }` will become `" ell "` but not `" hello "`.
#[derive(Clone, Copy, Default)]
#[pyclass]
pub struct Padding {
    #[pyo3(get, set)]
    left: usize,
    #[pyo3(get, set)]
    right: usize,
}

#[pymethods]
impl Padding {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Empty: Padding = Padding::new(0, 0);
    #[new]
    #[inline]
    pub const fn new(left: usize, right: usize) -> Padding {
        Padding { left, right }
    }
    pub fn __repr__(&self) -> String {
        format!(
            "<settings.Padding(left: {}, right: {})>",
            self.left, self.right
        )
    }
}

impl Padding {
    #[inline]
    pub const fn to_native(self) -> table::Padding {
        table::Padding {
            left: self.left,
            right: self.right,
        }
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Padding>()?;
        Ok(())
    }
}

/// Text alignment.
#[derive(Clone, Copy, Default)]
#[pyclass]
pub struct Align {
    align: table::Align,
}

#[pymethods]
impl Align {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Left: Align = Align::new(table::Align::Left);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Right: Align = Align::new(table::Align::Right);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Center: Align = Align::new(table::Align::Center);
    pub fn __repr__(&self) -> String {
        format!(
            "<settings.Align({})>",
            match self.align {
                table::Align::Left => "Left",
                table::Align::Right => "Right",
                table::Align::Center => "Center",
            }
        )
    }
}

impl Align {
    #[inline]
    pub const fn new(align: table::Align) -> Align {
        Align { align }
    }

    #[inline]
    pub const fn to_native(self) -> table::Align {
        self.align
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Align>()?;
        Ok(())
    }
}

/// Overflow setting for cells.
/// 
/// `Overflow.Ellipse`: "hello world" -> "he..."
///
/// `Overflow.Hide`: "hello" -> "hello"
#[derive(Clone, Copy, Default)]
#[pyclass]
pub struct Overflow {
    overflow: table::Overflow,
}

#[pymethods]
impl Overflow {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Hide: Overflow = Overflow::new(table::Overflow::Hidden);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Ellipse: Overflow = Overflow::new(table::Overflow::Ellipsis);
    pub fn __repr__(&self) -> String {
        format!(
            "<settings.Overflow({})>",
            match self.overflow {
                table::Overflow::Ellipsis => "Ellipsis",
                table::Overflow::Hidden => "Hidden",
            }
        )
    }
}

impl Overflow {
    #[inline]
    pub const fn new(overflow: table::Overflow) -> Overflow {
        Overflow { overflow }
    }

    #[inline]
    pub const fn to_native(self) -> table::Overflow {
        self.overflow
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Overflow>()?;
        Ok(())
    }
}

/// Border of the table.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[pyclass]
pub struct Border {
    #[pyo3(get, set)]
    left: bool,
    #[pyo3(get, set)]
    right: bool,
    #[pyo3(get, set)]
    top: bool,
    #[pyo3(get, set)]
    bottom: bool,
}

#[pymethods]
impl Border {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Default: Border = Border::new(false, false, false, false);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Horizontal: Border = Border::new(false, false, true, true);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Vertical: Border = Border::new(true, true, false, false);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const All: Border = Border::new(true, true, true, true);
    #[new]
    #[inline]
    pub const fn __new__(left: bool, right: bool, top: bool, bottom: bool) -> Self {
        Self::new(left, right, top, bottom)
    }

    pub fn __repr__(&self) -> String {
        format!("<settings.Border(l:{}, r:{}, t:{}, b:{})>", self.left, self.right, self.top, self.bottom)
    }
}

impl Border {
    #[inline]
    pub const fn new(left: bool, right: bool, top: bool, bottom: bool) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    #[inline]
    pub const fn to_native(self) -> table::Border {
        table::Border::new(self.left, self.right, self.top, self.bottom)
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Border>()?;
        Ok(())
    }
}

/// Render settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[pyclass]
pub enum Renderer {
    /// Render a normal table, with ansi color settings.
    Normal,
    /// Render a raw table, containing only contents.
    Raw,
    /// Render a markdown-formatted table. The alignment is determined by **the first row**, and the alignment of the rest of the table will be *ignored*.
    Markdown,
}

#[pymethods]
impl Renderer {
    pub fn __repr__(&self) -> String {
        format!("<settings.Renderer({})>", match self {
            Self::Normal => "Normal",
            Self::Raw => "Raw",
            Self::Markdown => "Markdown",
        })
    }
}

impl Renderer {
    #[inline]
    pub fn to_native(self) -> table::Renderer {
        match self {
            Self::Normal => table::Renderer::Normal,
            Self::Raw => table::Renderer::Raw,
            Self::Markdown => table::Renderer::Markdown,
        }
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Renderer>()?;
        Ok(())
    }
}
