use std::fmt::{self, Write};

pub trait PrettyPrint {
    fn pretty_fmt(&self, w: &mut impl Write, depth: usize) -> fmt::Result;
    fn pretty_print(&self) -> String {
        let mut buffer = String::new();
        self.pretty_fmt(&mut buffer, 0).unwrap();
        buffer
    }
}
pub(crate) const INDENT: &'static str = " ";

pub(crate) fn indent(depth: usize) -> String {
    INDENT.repeat(depth)
}
