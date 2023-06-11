//! Core library and the main entry point.

use std::rc::Rc;

use itertools::Itertools;
use pyo3::{exceptions, prelude::*, types::PyList};
use table_formatter::table;

use crate::{
    errors::FormatterError,
    settings::{Align, Border, Formatter, Overflow, Padding, Renderer},
};

pub fn regist_table(py: Python, m: &PyModule) -> PyResult<()> {
    let table_module = PyModule::new(py, "table")?;
    Content::regist_self(py, table_module)?;
    Cell::regist_self(py, table_module)?;
    Table::regist_self(py, table_module)?;
    m.add_submodule(table_module)?;
    Ok(())
}

/// Any possible content in a cell.
/// 
/// - Normal Content (str): pure text.
/// - Splitter: horizontal splitter. Looks like "─".
/// - Empty: empty content.
#[derive(Clone, Default)]
#[pyclass]
pub struct Content {
    content: table::Content,
}

#[pymethods]
impl Content {
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Empty: Content = Content::new(table::Content::None);
    #[classattr]
    #[allow(non_upper_case_globals)]
    pub const Splitter: Content = Content::new(table::Content::Splitter);

    #[new]
    pub fn __new__(c: String) -> Content {
        Content::new(table::Content::Text(c))
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<table.Content({})>",
            match self.content {
                table::Content::Splitter => "<Splitter>".to_string(),
                table::Content::None => "<None>".to_string(),
                table::Content::Text(ref text) => format!("\"{}\"", text),
            }
        )
    }
}

impl Content {
    #[inline]
    pub const fn new(content: table::Content) -> Content {
        Content { content }
    }

    #[inline]
    pub fn into_native(self) -> table::Content {
        self.content
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Content>()?;
        Ok(())
    }
}

/// Basic item for rendering a table.
#[derive(Clone, Default)]
#[pyclass]
pub struct Cell {
    #[pyo3(get, set)]
    content: Content,
    #[pyo3(get, set)]
    overflow: Overflow,
    #[pyo3(get, set)]
    width: Option<usize>,
    #[pyo3(get, set)]
    align: Align,
    #[pyo3(get, set)]
    padding: Padding,
    #[pyo3(get, set)]
    merge: Option<usize>,
    #[pyo3(get, set)]
    formatter: Vec<Formatter>,
}

#[pymethods]
impl Cell {
    #[new]
    pub fn __new__(c: Content) -> Self {
        Cell::default().native_with_content(c)
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<table.Cell(content={}, overflow={}, width={:?}, align={}, padding={}, merge={:?}, formatter=[{}])>",
            self.content.__repr__(),
            self.overflow.__repr__(),
            self.width,
            self.align.__repr__(),
            self.padding.__repr__(),
            self.merge,
            self.formatter.iter().map(|f| f.__repr__()).join(",")
        )
    }

    pub fn with_content(mut this: PyRefMut<Self>, content: Content) -> PyRefMut<Self> {
        this.width = content.content.get_width();
        this.content = content;
        this
    }
    pub fn with_overflow(mut this: PyRefMut<Self>, overflow: Overflow) -> PyRefMut<Self> {
        this.overflow = overflow;
        this
    }
    pub fn with_width(mut this: PyRefMut<Self>, width: Option<usize>) -> PyRefMut<Self> {
        this.width = width;
        this
    }
    pub fn with_align(mut this: PyRefMut<Self>, align: Align) -> PyRefMut<Self> {
        this.align = align;
        this
    }
    pub fn with_padding(mut this: PyRefMut<Self>, padding: Padding) -> PyRefMut<Self> {
        this.padding = padding;
        this
    }
    pub fn with_merge(mut this: PyRefMut<Self>, merge: Option<usize>) -> PyRefMut<Self> {
        this.merge = merge;
        this
    }
    pub fn with_formatter(mut this: PyRefMut<Self>, formatter: Vec<Formatter>) -> PyRefMut<Self> {
        this.formatter = formatter;
        this
    }
    pub fn append_formatter(mut this: PyRefMut<Self>, formatter: &PyList) -> PyResult<()> {
        for i in formatter.iter() {
            let local_i = i.extract::<Formatter>()?;
            this.formatter.push(local_i);
        }
        Ok(())
    }
    pub fn with_appended_formatter<'a>(
        mut this: PyRefMut<'a, Self>,
        formatter: &PyList,
    ) -> PyResult<PyRefMut<'a, Self>> {
        for i in formatter.iter() {
            let local_i = i.extract::<Formatter>()?;
            this.formatter.push(local_i);
        }
        Ok(this)
    }
    
    /// Automatically generate a cross-cell item.
    /// 
    /// Using `with_span(x)` will generate a vector with the cell *itself* and **x** empty cells.
    pub fn with_span(&self, span: usize) -> Vec<Self> {
        let this = self.clone().native_with_merge(Some(span)).native_with_width(None);
        let mut v = vec![this];
        v.extend(std::iter::repeat(Self::default()).take(span));
        v
    }
}

impl Cell {
    pub fn native_with_content(mut self, content: Content) -> Self {
        self.width = content.content.get_width();
        self.content = content;
        self
    }
    pub fn native_with_overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }
    pub fn native_with_width(mut self, width: Option<usize>) -> Self {
        self.width = width;
        self
    }
    pub fn native_with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }
    pub fn native_with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
    pub fn native_with_merge(mut self, merge: Option<usize>) -> Self {
        self.merge = merge;
        self
    }
    pub fn native_with_formatter(mut self, formatter: Vec<Formatter>) -> Self {
        self.formatter = formatter;
        self
    }
    pub fn native_append_formatter(&mut self, formatter: &mut Vec<Formatter>) {
        self.formatter.append(formatter);
    }
    pub fn native_with_appended_formatter(mut self, formatter: &mut Vec<Formatter>) -> Self {
        self.formatter.append(formatter);
        self
    }
    pub fn native_set_overflow(&mut self, overflow: Overflow) {
        self.overflow = overflow;
    }

    pub fn to_native(&self) -> table::Cell {
        table::Cell::default()
            .with_content(self.content.clone().into_native())
            .with_align(self.align.to_native())
            .with_overflow(self.overflow.to_native())
            .with_width(self.width)
            .with_padding(self.padding.to_native())
            .with_merge(self.merge)
            .with_formatter(
                self.formatter
                    .iter()
                    .cloned()
                    .map(|f| {
                        table::FormatterFunc::Boxed(Rc::new(Box::new(move |s| f.as_func().run(s))))
                    })
                    .collect_vec(),
            )
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Cell>()?;
        Ok(())
    }
}

/// This is the main entry point of the lib, which represents the table to render.
///
/// For more information, please see the lib's documentation.
#[pyclass]
pub struct Table {
    #[pyo3(get)]
    table: Vec<Vec<Cell>>,
    #[pyo3(get, set)]
    border: Border,
}

#[pymethods]
impl Table {
    pub fn __repr__(&self) -> String {
        format!(
            "<table.Table(border={}, size=({}))>",
            self.border.__repr__(),
            if self.table.is_empty() {
                "0x0".to_string()
            } else {
                let h = self.table.len();
                let w = self.table[0].len();
                format!("{}x{}", w, h)
            }
        )
    }

    pub fn with_border(mut this: PyRefMut<Self>, border: Border) -> PyRefMut<Self> {
        this.border = border;
        this
    }

    /// This will render the table according to the render settings. See the lib's documentation for more information.
    pub fn render(&self, setting: Renderer) -> PyResult<String> {
        let setting = setting.to_native();
        let table = self.to_native();
        let mut v = Vec::new();
        table
            .rendered_by(setting, &mut v)
            .map_err(|e| FormatterError::new(format!("{e}")))?;
        String::from_utf8(v).map_err(PyErr::new::<exceptions::PyUnicodeError, _>)
    }
    
    /// This function will overwrite the `overflow` property of every cells in the table.
    pub fn overwrite_overflow(&mut self, overflow: Overflow) {
        for row in self.table.iter_mut() {
            for cell in row.iter_mut() {
                cell.native_set_overflow(overflow)
            }
        }
    }

    /// Create a new table with some rows.
    #[new]
    pub fn __new__(data: &PyList) -> PyResult<Table> {
        let mut v = Vec::new();
        for i in data.iter() {
            let local_i = i.extract::<&PyList>()?;
            let mut vi = Vec::new();
            for j in local_i.iter() {
                let local_cell = j.extract::<Cell>()?;
                vi.push(local_cell);
            }
            v.push(vi);
        }
        Ok(Table::new(v))
    }

    /// Create a new table with a header and some rows.
    ///
    /// When `splitter` is set to true, this will automatically add a splitter between header and contents.
    ///
    /// > This is the recommended way to create a new table, so for details see the lib's documentation.
    #[staticmethod]
    pub fn create(header: &PyList, content: &PyList, splitter: bool) -> PyResult<Table> {
        let header_v = {
            let mut v = Vec::new();
            for i in header.iter() {
                let li = i.extract::<Cell>()?;
                v.push(li);
            }
            v
        };

        let content_v = {
            let mut v = Vec::new();
            for i in content.iter() {
                let local_i = i.extract::<&PyList>()?;
                let mut vi = Vec::new();
                for j in local_i.iter() {
                    let local_cell = j.extract::<Cell>()?;
                    vi.push(local_cell);
                }
                v.push(vi);
            }
            v
        };

        Ok(Table::native_create(header_v, content_v, splitter))
    }
}

impl Table {
    pub fn native_create(header: Vec<Cell>, mut cell: Vec<Vec<Cell>>, splitter: bool) -> Table {
        let mut v = if splitter {
            let dat = header
                .iter()
                .map(|_| {
                    Cell::default()
                        .native_with_content(Content::Splitter)
                        .native_with_formatter(vec![Formatter::Bold])
                })
                .collect_vec();
            let mut v = vec![header
                .into_iter()
                .map(|c| c.native_with_formatter(vec![Formatter::Bold]))
                .collect_vec()];
            v.push(dat);
            v
        } else {
            vec![header]
        };
        v.append(&mut cell);
        Self {
            table: v,
            border: Border::Default,
        }
    }

    pub fn new(table: Vec<Vec<Cell>>) -> Table {
        Self {
            table,
            border: Border::Default,
        }
    }

    pub fn to_native(&self) -> table::Table {
        table::Table::new(
            self.table
                .iter()
                .map(|row| row.iter().map(|cell| cell.to_native()).collect_vec())
                .collect_vec(),
        )
        .with_border(self.border.to_native())
    }

    fn regist_self(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<Table>()?;
        Ok(())
    }
}
