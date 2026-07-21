//! Formatter configuration.
//!
//! The LaTeX-specific [`WrapMode`] (paragraph line-break policy, modeled on the
//! `panache` formatter) is the one field specific to badness.

/// How the formatter lays out the line breaks *inside* a paragraph. Modeled on
/// panache's `WrapMode` (`crates/panache-formatter/src/config.rs`).
///
/// The sentence-boundary detection behind [`WrapMode::Sentence`] and
/// [`WrapMode::Semantic`] is a per-language abbreviation profile
/// (`formatter::sentence`); the language and any user no-break abbreviations are
/// resolved from config into the [`SentenceOptions`](super::SentenceOptions)
/// threaded through the lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WrapMode {
    /// Greedy fill: pack words up to `line_width`, breaking only where the next
    /// word would not fit. The default.
    #[default]
    Reflow,
    /// Wrap after each sentence (one sentence per line). Line width is ignored — a
    /// long sentence stays on one line.
    Sentence,
    /// Semantic line breaks (<https://sembr.org/>): keep the author's soft line
    /// breaks *and* add a break after each sentence. Like [`WrapMode::Sentence`]
    /// plus preserving authored newlines; clause boundaries survive only where the
    /// author placed a break (no comma/colon detection).
    Semantic,
    /// Leave paragraph line breaks exactly as authored (only collapse trailing
    /// whitespace and blank-line runs, as before reflow existed).
    Preserve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FormatStyle {
    pub line_width: usize,
    pub indent_width: usize,
    pub wrap: WrapMode,
    /// Whether `tabular`/`array` and the math grids (`align`, matrix, `gather`, …)
    /// are laid out as column-spec-aware aligned grids. `true` (the default) keeps
    /// the ampersand-aligning behavior; `false` renders those environments through
    /// the generic environment lowering, leaving hand-tuned column layout untouched
    /// (the equivalent of tex-fmt's `format-tables = false`). A single-formula
    /// `equation` is not a grid, so its relation-aware breaking is unaffected.
    pub align_tables: bool,
    /// Whether a *multi-line* optional argument (`[key=val, …]`) is reflowed one
    /// comma-separated item per line. `false` (the default) leaves optional-argument
    /// layout to the width-driven engine (authored line breaks preserved); `true`
    /// normalizes an authored-multi-line list to strictly one item per line (the
    /// equivalent of tex-fmt's `format-options = true`). A single-line optional is
    /// never expanded.
    pub format_options: bool,
}

impl Default for FormatStyle {
    fn default() -> Self {
        Self {
            line_width: 80,
            indent_width: 2,
            wrap: WrapMode::default(),
            align_tables: true,
            format_options: false,
        }
    }
}
