use internal_baml_diagnostics::{SourceFile, Span};

pub fn mk_pos(left: usize, right: usize) -> Span {
    Span::new(
        SourceFile::new_static("todo.baml".into(), "todo"),
        left,
        right,
    )
}

pub trait WithPos {
    fn with_pos(self, pos: Span) -> Self;
}