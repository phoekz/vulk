use super::*;

pub struct Builder(String);

impl Builder {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(String::new())
    }

    #[inline]
    #[must_use]
    pub fn build(self) -> String {
        self.0
    }

    #[inline]
    #[must_use]
    pub fn raw(mut self, str: impl AsRef<str>) -> Self {
        self.0.push_str(str.as_ref());
        self
    }

    #[inline]
    #[must_use]
    pub fn line(mut self, str: impl AsRef<str>) -> Self {
        if !self.0.is_empty() {
            self.0.push('\n');
        }
        self = self.raw(str);
        self
    }

    #[inline]
    #[must_use]
    pub fn must_use(self) -> Self {
        self.line("#[must_use]")
    }

    #[inline]
    #[must_use]
    pub fn inline(self) -> Self {
        self.line("#[inline]")
    }

    #[inline]
    #[must_use]
    pub fn repr(self, ty: impl AsRef<str>) -> Self {
        self.line(format!("#[repr({})]", ty.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn derive(self, traits: impl AsRef<str>) -> Self {
        self.line(format!("#[derive({})]", traits.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_str(self, str: impl AsRef<str>) -> Self {
        self.line(format!("#[doc = \"{}\"]", str.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_br(self) -> Self {
        self.doc_str("<br>")
    }

    #[inline]
    #[must_use]
    pub fn doc_chapter(self, chapter: impl AsRef<str>) -> Self {
        self.doc_str(format!("**Chapter**: {}", chapter.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_desc(self, desc: impl AsRef<str>) -> Self {
        self.doc_str(format!("**Description**: {}", desc.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_ref(self, ident: impl AsRef<str>) -> Self {
        self.doc_str(format!(
            "**Reference**: [`{}`]({})",
            ident.as_ref(),
            docs::reference_url(ident.as_ref())
        ))
    }

    #[inline]
    #[must_use]
    pub fn doc_extend(self, ident: impl AsRef<str>) -> Self {
        self.doc_str(format!(
            "**Extendable by**: [`{}`]({})",
            ident.as_ref(),
            docs::reference_url(ident.as_ref())
        ))
    }

    #[inline]
    #[must_use]
    pub fn doc_translated(self, ident: impl AsRef<str>) -> Self {
        self.doc_str(format!("**Translated from**: `{}`", ident.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_provided(self, ident: impl AsRef<str>) -> Self {
        self.doc_str(format!(
            "**Provided by**: [`{}`]({})",
            ident.as_ref(),
            docs::reference_url(ident.as_ref())
        ))
    }

    #[inline]
    #[must_use]
    pub fn doc_note(self, note: impl AsRef<str>) -> Self {
        self.doc_str(format!("**Note**: {}", note.as_ref()))
    }

    #[inline]
    #[must_use]
    pub fn doc_includes_ext(self, include: impl AsRef<str>) -> Self {
        self.doc_str(format!(
            "**Includes**: [`{}`]({})",
            include.as_ref(),
            docs::reference_url(include.as_ref())
        ))
    }
}

impl std::fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
