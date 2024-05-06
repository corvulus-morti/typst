use std::fmt::{self, Debug, Formatter};

use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{
	elem, Args, Cast, Construct, Content, NativeElement, Packed, Set, Smart, StyleChain,
	Unlabellable,
};
use crate::layout::{Em, Fragment, Length, Size, HElem, LayoutMultiple, Regions};
use crate::model::InlineElem;

/// Arranges text, spacing and inline-level elements into a paragraph.
///
/// Although this function is primarily used in set rules to affect paragraph
/// properties, it can also be used to explicitly render its argument onto a
/// paragraph of its own.
///
/// # Example
/// ```example
/// #show par: set block(spacing: 0.65em)
/// #set par(
///   first-line-indent: 1em,
///   justify: true,
/// )
///
/// We proceed by contradiction.
/// Suppose that there exists a set
/// of positive integers $a$, $b$, and
/// $c$ that satisfies the equation
/// $a^n + b^n = c^n$ for some
/// integer value of $n > 2$.
///
/// Without loss of generality,
/// let $a$ be the smallest of the
/// three integers. Then, we ...
/// ```
#[elem(title = "Paragraph", Debug, Construct)]
pub struct ParElem {
	/// The spacing between lines.
	#[resolve]
	#[ghost]
	#[default(Em::new(0.65).into())]
	pub leading: Length,

	/// Whether to justify text in its line.
	///
	/// Hyphenation will be enabled for justified paragraphs if the
	/// [text function's `hyphenate` property]($text.hyphenate) is set to
	/// `{auto}` and the current language is known.
	///
	/// Note that the current [alignment]($align.alignment) still has an effect
	/// on the placement of the last line except if it ends with a
	/// [justified line break]($linebreak.justify).
	#[ghost]
	#[default(false)]
	pub justify: bool,

	/// The indent the first line of a paragraph should have.
	///
	/// Only the first line of a consecutive paragraph will be indented (not
	/// the first one in a block or on the page).
	///
	/// By typographic convention, paragraph breaks are indicated either by some
	/// space between paragraphs or by indented first lines. Consider reducing
	/// the [paragraph spacing]($block.spacing) to the [`leading`]($par.leading)
	/// when using this property (e.g. using
	/// `[#show par: set block(spacing: 0.65em)]`).
	#[ghost]
	pub first_line_indent: Length,

	/// Whether or not the first line is always intented, or only when it is a consecutive paragraph
	#[ghost]
	#[default(false)]
	pub always_indent_first_line: bool,

	/// Indicates wheter an overflowing line should be shrunk.
	///
	/// This property is set to `false` on raw blocks, because shrinking a line
	/// could visually break the indentation.
	#[ghost]
	#[internal]
	#[default(true)]
	pub shrink: bool,

	/// The paragraph's children.
	#[internal]
	#[variadic]
	pub children: Vec<Content>,
}

impl Construct for ParElem {
	fn construct(engine: &mut Engine, args: &mut Args) -> SourceResult<Content> {
		// The paragraph constructor is special: It doesn't create a paragraph
		// element. Instead, it just ensures that the passed content lives in a
		// separate paragraph and styles it.
		let styles = Self::set(engine, args)?;
		let body = args.expect::<Content>("body")?;
		Ok(Content::sequence([
			ParbreakElem::new().pack(),
			body.styled_with_map(styles),
			ParbreakElem::new().pack(),
		]))
	}
}

impl LayoutMultiple for Packed<ParElem> {
	/// Layout the paragraph into a collection of inline and block elements.
	#[typst_macros::time(name = "par", span = self.span())]
	fn layout(
		&self,
		engine: &mut Engine,
		styles: StyleChain,
		regions: Regions,
	) -> SourceResult<Fragment> {
		let mut frames = Vec::new();
		// for (i,child) in self.children.iter().enumerate() {
		// 	if i == 0 && child.is::<InlineElem>() {
		// 		child.children.insert(0,HElem::new(self.first_line_indent))
		// 	}
		// 	let frameVec = child.layout(
		// 		self.children(),
		// 		engine,
		// 		styles,
		// 		region,
		// 		expand,
		// 	)
		// 	.into_frames();
		// 	frames.append(frameVec);
		// }
		Ok(Fragment::frames(frames))
	}
}

impl Debug for ParElem {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Par ")?;
		f.debug_list().entries(&self.children).finish()
	}
}

/// A paragraph break.
///
/// This starts a new paragraph. Especially useful when used within code like
/// [for loops]($scripting/#loops). Multiple consecutive
/// paragraph breaks collapse into a single one.
///
/// # Example
/// ```example
/// #for i in range(3) {
///   [Blind text #i: ]
///   lorem(5)
///   parbreak()
/// }
/// ```
///
/// # Syntax
/// Instead of calling this function, you can insert a blank line into your
/// markup to create a paragraph break.
#[elem(title = "Paragraph Break", Unlabellable)]
pub struct ParbreakElem {}

impl Unlabellable for Packed<ParbreakElem> {}
