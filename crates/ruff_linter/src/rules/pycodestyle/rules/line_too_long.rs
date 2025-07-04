use ruff_macros::{ViolationMetadata, derive_message_formats};
use ruff_python_trivia::CommentRanges;
use ruff_source_file::Line;

use crate::rules::pycodestyle::overlong::Overlong;
use crate::settings::LinterSettings;
use crate::{Diagnostic, Violation};

/// ## What it does
/// Checks for lines that exceed the specified maximum character length.
///
/// ## Why is this bad?
/// Overlong lines can hurt readability. [PEP 8], for example, recommends
/// limiting lines to 79 characters. By default, this rule enforces a limit
/// of 88 characters for compatibility with Black and the Ruff formatter,
/// though that limit is configurable via the [`line-length`] setting.
///
/// In the interest of pragmatism, this rule makes a few exceptions when
/// determining whether a line is overlong. Namely, it:
///
/// 1. Ignores lines that consist of a single "word" (i.e., without any
///    whitespace between its characters).
/// 2. Ignores lines that end with a URL, as long as the URL starts before
///    the line-length threshold.
/// 3. Ignores line that end with a pragma comment (e.g., `# type: ignore`
///    or `# noqa`), as long as the pragma comment starts before the
///    line-length threshold. That is, a line will not be flagged as
///    overlong if a pragma comment _causes_ it to exceed the line length.
///    (This behavior aligns with that of the Ruff formatter.)
/// 4. Ignores SPDX license identifiers and copyright notices
///    (e.g., `# SPDX-License-Identifier: MIT`), which are machine-readable
///    and should _not_ wrap over multiple lines.
///
/// If [`lint.pycodestyle.ignore-overlong-task-comments`] is `true`, this rule will
/// also ignore comments that start with any of the specified [`lint.task-tags`]
/// (e.g., `# TODO:`).
///
/// ## Example
/// ```python
/// my_function(param1, param2, param3, param4, param5, param6, param7, param8, param9, param10)
/// ```
///
/// Use instead:
/// ```python
/// my_function(
///     param1, param2, param3, param4, param5,
///     param6, param7, param8, param9, param10
/// )
/// ```
///
/// ## Error suppression
/// Hint: when suppressing `E501` errors within multi-line strings (like
/// docstrings), the `noqa` directive should come at the end of the string
/// (after the closing triple quote), and will apply to the entire string, like
/// so:
///
/// ```python
/// """Lorem ipsum dolor sit amet.
///
/// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.
/// """  # noqa: E501
/// ```
///
/// ## Options
/// - `line-length`
/// - `lint.task-tags`
/// - `lint.pycodestyle.ignore-overlong-task-comments`
/// - `lint.pycodestyle.max-line-length`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#maximum-line-length
#[derive(ViolationMetadata)]
pub(crate) struct LineTooLong(usize, usize);

impl Violation for LineTooLong {
    #[derive_message_formats]
    fn message(&self) -> String {
        let LineTooLong(width, limit) = self;
        format!("Line too long ({width} > {limit})")
    }
}

/// E501
pub(crate) fn line_too_long(
    line: &Line,
    comment_ranges: &CommentRanges,
    settings: &LinterSettings,
) -> Option<Diagnostic> {
    let limit = settings.pycodestyle.max_line_length;

    Overlong::try_from_line(
        line,
        comment_ranges,
        limit,
        if settings.pycodestyle.ignore_overlong_task_comments {
            &settings.task_tags
        } else {
            &[]
        },
        settings.tab_size,
    )
    .map(|overlong| {
        Diagnostic::new(
            LineTooLong(overlong.width(), limit.value() as usize),
            overlong.range(),
        )
    })
}
