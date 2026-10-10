//! Optional content groups.
//!
//! An optional content group is a named layer whose visibility a viewer lets the user toggle.
//! Register one with
//! [`Document::add_optional_content_group`](crate::document::Document::add_optional_content_group),
//! then wrap the content belonging to it in
//! [`Surface::start_optional_content`](crate::surface::Surface::start_optional_content) and
//! [`Surface::end_optional_content`](crate::surface::Surface::end_optional_content).

use pdf_writer::Ref;

/// A handle to an optional content group.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OptionalContentGroupId(pub(crate) Ref);

/// The settings of an optional content group.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OptionalContentGroup {
    pub(crate) name: String,
    pub(crate) view: bool,
    pub(crate) print: bool,
    pub(crate) export: bool,
}

impl OptionalContentGroup {
    /// Create a new group that is visible on screen, in print and in export.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            view: true,
            print: true,
            export: true,
        }
    }

    /// Change whether the group is initially visible on screen.
    pub fn with_view(mut self, view: bool) -> Self {
        self.view = view;
        self
    }

    /// Change whether the group is visible in print.
    pub fn with_print(mut self, print: bool) -> Self {
        self.print = print;
        self
    }

    /// Change whether the group is visible in export.
    pub fn with_export(mut self, export: bool) -> Self {
        self.export = export;
        self
    }

    pub(crate) fn has_usage(&self) -> bool {
        !self.print || !self.export
    }
}
