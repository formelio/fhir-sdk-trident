//! Sanitization and structuring of documentation comments.

use ::regex::Regex;
use ::std::sync::LazyLock;

/// Sanitize a doc-comment.
/// - Replaces \r with \n
/// - Makes code blocks have a tag to avoid being treated as Rust code
pub fn sanitize(comment: &str) -> String {
	/// Regex for replacing code blocks with text blocks.
	static CODE_REGEX: LazyLock<Regex> =
		LazyLock::new(|| Regex::new(r"(?s)```(.*?)```").expect("compiling Regex"));
	let comment = CODE_REGEX.replace_all(comment, "```text$1```");

	comment.replace('\r', "\n")
}
