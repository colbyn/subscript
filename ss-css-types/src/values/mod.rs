pub mod custom;
pub use custom::*;


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// TODO:
/// * eemove all redundant length labels (in favor of unified Length enum)
pub enum Value {
	Length(Length),
	Length2(Length, Length),
	Length3(Length, Length, Length),
	Length4(Length, Length, Length, Length),
	RunIn,
	Flow,
	FlowRoot,
	Inline,
	Add,
	Additive,
	Alias,
	All,
	AllowEnd,
	AllScroll,
	Alpha,
	Alphabetic,
	Alternate,
	AlternateReverse,
	Always,
	Anywhere,
	ArabicIndic,
	Arithmetic,
	Armenian,
	Atop,
	Aural,
	Auto,
	AutoFill,
	AutoFit,
	Avoid,
	AvoidColumn,
	AvoidPage,
	AvoidRegion,
	Backgroundalpha,
	Backgroundimage,
	Backwards,
	Balance,
	BalanceAll,
	Baseline,
	Bengali,
	BidiOverride,
	Blink,
	Block,
	BorderBox,
	Both,
	Bottom,
	Braille,
	BreakAll,
	BreakSpaces,
	BreakWord,
	Bullets,
	Cambodian,
	Capitalize,
	Cell,
	Center,
	Ch,
	Circle,
	CjkDecimal,
	CjkEarthlyBranch,
	CjkHeavenlyStem,
	CjkIdeographic,
	Clip,
	Clone,
	CloseQuote,
	ClosestCorner,
	ClosestSide,
	Coarse,
	Color,
	ColorBurn,
	ColorDodge,
	ColResize,
	Column,
	ColumnReverse,
	Contain,
	Content,
	ContentBox,
	Contents,
	ContextMenu,
	Copy,
	Cover,
	CrispEdges,
	Crosshair,
	Currentcolor,
	Cyclic,
	Darken,
	Dashed,
	Decimal,
	DecimalLeadingZero,
	Default,
	Deg,
	Dense,
	Devanagari,
	Difference,
	Disc,
	DisclosureClosed,
	DisclosureOpen,
	Discrete,
	Distribute,
	Dot,
	Dotted,
	Double,
	DoubleCircle,
	Dpcm,
	Dpi,
	Dppx,
	Duplicate,
	EachLine,
	Ease,
	EaseIn,
	EaseInOut,
	EaseOut,
	Ellipse,
	Ellipsis,
	Embed,
	Embossed,
	End,
	EResize,
	EthiopicNumeric,
	Evenodd,
	EwResize,
	Ex,
	Exclude,
	Exclusion,
	Extends,
	FarthestCorner,
	FarthestSide,
	Fast,
	Fill,
	FillBox,
	Filled,
	Fillpaint,
	Fine,
	First,
	FitContent,
	Fixed,
	Flex,
	FlexEnd,
	FlexStart,
	FontFeatureSettings,
	FontVariant,
	ForceEnd,
	Forwards,
	Fr,
	FromImage,
	FullSizeKana,
	FullWidth,
	Gamma,
	Georgian,
	Grab,
	Grabbing,
	Grad,
	Grid,
	Groove,
	Gujarati,
	Gurmukhi,
	Handheld,
	Hanging,
	HardLight,
	Hebrew,
	Help,
	Hidden,
	HighQuality,
	Hiragana,
	HiraganaIroha,
	HorizontalTb,
	Hover,
	Hue,
	Hz,
	Identity,
	InterCharacter,
	Interlace,
	Intersect,
	InterWord,
	Invert,
	Isolate,
	IsolateOverride,
	JapaneseFormal,
	JapaneseInformal,
	JumpBoth,
	JumpEnd,
	JumpNone,
	JumpStart,
	Justify,
	JustifyAll,
	Kannada,
	Katakana,
	KatakanaIroha,
	KeepAll,
	Khmer,
	Khz,
	KoreanHangulFormal,
	KoreanHanjaFormal,
	KoreanHanjaInformal,
	Landscape,
	Lao,
	Last,
	Layout,
	Left,
	Legacy,
	Lighten,
	Linear,
	Linearrgb,
	LineThrough,
	ListItem,
	Local,
	Loose,
	LowerAlpha,
	LowerArmenian,
	Lowercase,
	LowerGreek,
	LowerLatin,
	LowerRoman,
	Ltr,
	Luminance,
	Luminosity,
	Malayalam,
	Mandatory,
	Manual,
	MarginBox,
	MatchParent,
	MatchSource,
	MaxContent,
	Medium,
	MinContent,
	Minmax,
	Mixed,
	Mm,
	Mongolian,
	Move,
	Ms,
	Multiply,
	Myanmar,
	NeResize,
	NeswResize,
	NoClip,
	NoCloseQuote,
	NoComposite,
	NoDrop,
	None,
	Nonzero,
	NoOpenQuote,
	NoRepeat,
	Normal,
	Not,
	NotAllowed,
	Nowrap,
	NResize,
	NsResize,
	Numbers,
	Numeric,
	NwResize,
	NwseResize,
	Objectboundingbox,
	Only,
	Open,
	OpenQuote,
	OptionalPaged,
	Oriya,
	Outset,
	Over,
	Overlay,
	Overline,
	P3,
	PaddingBox,
	Page,
	Paged,
	Paint,
	Paused,
	Pc,
	Persian,
	Pixelated,
	Pixel,
	Plaintext,
	Pointer,
	Portrait,
	Pre,
	PreLine,
	PreWrap,
	Print,
	Progress,
	Progressive,
	Projection,
	Proximity,
	Rec2020,
	Recto,
	Region,
	Repeat,
	RepeatX,
	RepeatY,
	Reverse,
	Revert,
	Ridge,
	Right,
	Rotate,
	Round,
	Row,
	RowResize,
	RowReverse,
	Rtl,
	Running,
	S,
	Safe,
	Saturation,
	Scale,
	ScaleDown,
	Scalex,
	Scaley,
	Screen,
	Scroll,
	ScrollPosition,
	SelfEnd,
	SelfStart,
	SeResize,
	Sesame,
	Sideways,
	SidewaysRight,
	SimpChineseFormal,
	SimpChineseInformal,
	Size,
	Skew,
	Skewx,
	Skewy,
	Slice,
	Slow,
	Smooth,
	SoftLight,
	Solid,
	Sourcealpha,
	Sourcegraphic,
	Space,
	SpaceAround,
	SpaceBetween,
	SpaceEvenly,
	Speech,
	SpellOut,
	Square,
	SResize,
	Srgb,
	Start,
	StepEnd,
	StepStart,
	Stretch,
	Strict,
	StrokeBox,
	Strokepaint,
	Style,
	Subtract,
	SwResize,
	Symbolic,
	Table,
	TableCaption,
	TableCell,
	TableColumn,
	TableColumnGroup,
	TableFooterGroup,
	TableHeaderGroup,
	TableRow,
	TableRowGroup,
	Tamil,
	Telugu,
	Text,
	Thai,
	Thick,
	Thin,
	Tibetan,
	Top,
	TradChineseFormal,
	TradChineseInformal,
	Translate,
	Translatex,
	Translatey,
	Triangle,
	Tty,
	Turn,
	Tv,
	Under,
	Underline,
	Unsafe,
	Unset,
	UpperAlpha,
	UpperArmenian,
	Uppercase,
	UpperLatin,
	UpperRoman,
	Upright,
	Userspaceonuse,
	Verso,
	VerticalLr,
	VerticalRl,
	VerticalText,
	Vh,
	ViewBox,
	Vmax,
	Vmin,
	Vw,
	Wait,
	Words,
	Wrap,
	WrapReverse,
	WResize,
	X,
	Xor,
	Y,
	ZoomIn,
	ZoomOut,
}






