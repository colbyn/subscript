pub mod custom;
pub mod labels;

pub use custom::*;
pub use labels::*;

/// The "add" value.
pub fn add() -> Add {
    Add{}
}
/// The "additive" value.
pub fn additive() -> Additive {
    Additive{}
}
/// The "alias" value.
pub fn alias() -> Alias {
    Alias{}
}
/// The "all" value.
pub fn all() -> All {
    All{}
}
/// The "allow-end" value.
pub fn allow_end() -> AllowEnd {
    AllowEnd{}
}
/// The "all-scroll" value.
pub fn all_scroll() -> AllScroll {
    AllScroll{}
}
/// The "alpha" value.
pub fn alpha() -> Alpha {
    Alpha{}
}
/// The "alphabetic" value.
pub fn alphabetic() -> Alphabetic {
    Alphabetic{}
}
/// The "alternate" value.
pub fn alternate() -> Alternate {
    Alternate{}
}
/// The "alternate-reverse" value.
pub fn alternate_reverse() -> AlternateReverse {
    AlternateReverse{}
}
/// The "always" value.
pub fn always() -> Always {
    Always{}
}
/// The "anywhere" value.
pub fn anywhere() -> Anywhere {
    Anywhere{}
}
/// The "arabic-indic" value.
pub fn arabic_indic() -> ArabicIndic {
    ArabicIndic{}
}
/// The "arithmetic" value.
pub fn arithmetic() -> Arithmetic {
    Arithmetic{}
}
/// The "armenian" value.
pub fn armenian() -> Armenian {
    Armenian{}
}
/// The "atop" value.
pub fn atop() -> Atop {
    Atop{}
}
/// The "aural" value.
pub fn aural() -> Aural {
    Aural{}
}
/// The "auto" value.
pub fn auto() -> Auto {
    Auto{}
}
/// The "auto-fill" value.
pub fn auto_fill() -> AutoFill {
    AutoFill{}
}
/// The "auto-fit" value.
pub fn auto_fit() -> AutoFit {
    AutoFit{}
}
/// The "avoid" value.
pub fn avoid() -> Avoid {
    Avoid{}
}
/// The "avoid-column" value.
pub fn avoid_column() -> AvoidColumn {
    AvoidColumn{}
}
/// The "avoid-page" value.
pub fn avoid_page() -> AvoidPage {
    AvoidPage{}
}
/// The "avoid-region" value.
pub fn avoid_region() -> AvoidRegion {
    AvoidRegion{}
}
/// The "backgroundalpha" value.
pub fn backgroundalpha() -> Backgroundalpha {
    Backgroundalpha{}
}
/// The "backgroundimage" value.
pub fn backgroundimage() -> Backgroundimage {
    Backgroundimage{}
}
/// The "backwards" value.
pub fn backwards() -> Backwards {
    Backwards{}
}
/// The "balance" value.
pub fn balance() -> Balance {
    Balance{}
}
/// The "balance-all" value.
pub fn balance_all() -> BalanceAll {
    BalanceAll{}
}
/// The "baseline" value.
pub fn baseline() -> Baseline {
    Baseline{}
}
/// The "bengali" value.
pub fn bengali() -> Bengali {
    Bengali{}
}
/// The "bidi-override" value.
pub fn bidi_override() -> BidiOverride {
    BidiOverride{}
}
/// The "blink" value.
pub fn blink() -> Blink {
    Blink{}
}
/// The "block" value.
pub fn block() -> Block {
    Block{}
}
/// The "border-box" value.
pub fn border_box() -> BorderBox {
    BorderBox{}
}
/// The "both" value.
pub fn both() -> Both {
    Both{}
}
/// The "bottom" value.
pub fn bottom() -> Bottom {
    Bottom{}
}
/// The "braille" value.
pub fn braille() -> Braille {
    Braille{}
}
/// The "break-all" value.
pub fn break_all() -> BreakAll {
    BreakAll{}
}
/// The "break-spaces" value.
pub fn break_spaces() -> BreakSpaces {
    BreakSpaces{}
}
/// The "break-word" value.
pub fn break_word() -> BreakWord {
    BreakWord{}
}
/// The "bullets" value.
pub fn bullets() -> Bullets {
    Bullets{}
}
/// The "cambodian" value.
pub fn cambodian() -> Cambodian {
    Cambodian{}
}
/// The "capitalize" value.
pub fn capitalize() -> Capitalize {
    Capitalize{}
}
/// The "cell" value.
pub fn cell() -> Cell {
    Cell{}
}
/// The "center" value.
pub fn center() -> Center {
    Center{}
}
/// The "ch" value.
pub fn ch() -> Ch {
    Ch{}
}
/// The "circle" value.
pub fn circle() -> Circle {
    Circle{}
}
/// The "cjk-decimal" value.
pub fn cjk_decimal() -> CjkDecimal {
    CjkDecimal{}
}
/// The "cjk-earthly-branch" value.
pub fn cjk_earthly_branch() -> CjkEarthlyBranch {
    CjkEarthlyBranch{}
}
/// The "cjk-heavenly-stem" value.
pub fn cjk_heavenly_stem() -> CjkHeavenlyStem {
    CjkHeavenlyStem{}
}
/// The "cjk-ideographic" value.
pub fn cjk_ideographic() -> CjkIdeographic {
    CjkIdeographic{}
}
/// The "clip" value.
pub fn clip() -> Clip {
    Clip{}
}
/// The "clone" value.
pub fn clone() -> Clone {
    Clone{}
}
/// The "close-quote" value.
pub fn close_quote() -> CloseQuote {
    CloseQuote{}
}
/// The "closest-corner" value.
pub fn closest_corner() -> ClosestCorner {
    ClosestCorner{}
}
/// The "closest-side" value.
pub fn closest_side() -> ClosestSide {
    ClosestSide{}
}
/// The "cm" value.
pub fn cm(x: i32) -> Length {
    unimplemented!()
}
/// The "coarse" value.
pub fn coarse() -> Coarse {
    Coarse{}
}
/// The "color" value.
pub fn color() -> Color {
    Color{}
}
/// The "color-burn" value.
pub fn color_burn() -> ColorBurn {
    ColorBurn{}
}
/// The "color-dodge" value.
pub fn color_dodge() -> ColorDodge {
    ColorDodge{}
}
/// The "col-resize" value.
pub fn col_resize() -> ColResize {
    ColResize{}
}
/// The "column" value.
pub fn column() -> Column {
    Column{}
}
/// The "column-reverse" value.
pub fn column_reverse() -> ColumnReverse {
    ColumnReverse{}
}
/// The "contain" value.
pub fn contain() -> Contain {
    Contain{}
}
/// The "content" value.
pub fn content() -> Content {
    Content{}
}
/// The "content-box" value.
pub fn content_box() -> ContentBox {
    ContentBox{}
}
/// The "contents" value.
pub fn contents() -> Contents {
    Contents{}
}
/// The "context-menu" value.
pub fn context_menu() -> ContextMenu {
    ContextMenu{}
}
/// The "copy" value.
pub fn copy() -> Copy {
    Copy{}
}
/// The "cover" value.
pub fn cover() -> Cover {
    Cover{}
}
/// The "crisp-edges" value.
pub fn crisp_edges() -> CrispEdges {
    CrispEdges{}
}
/// The "crosshair" value.
pub fn crosshair() -> Crosshair {
    Crosshair{}
}
/// The "currentcolor" value.
pub fn currentcolor() -> Currentcolor {
    Currentcolor{}
}
/// The "cyclic" value.
pub fn cyclic() -> Cyclic {
    Cyclic{}
}
/// The "darken" value.
pub fn darken() -> Darken {
    Darken{}
}
/// The "dashed" value.
pub fn dashed() -> Dashed {
    Dashed{}
}
/// The "decimal" value.
pub fn decimal() -> Decimal {
    Decimal{}
}
/// The "decimal-leading-zero" value.
pub fn decimal_leading_zero() -> DecimalLeadingZero {
    DecimalLeadingZero{}
}
/// The "default" value.
pub fn default() -> Default {
    Default{}
}
/// The "deg" value.
pub fn deg() -> Angle {
    Angle::Deg
}
/// The "dense" value.
pub fn dense() -> Dense {
    Dense{}
}
/// The "devanagari" value.
pub fn devanagari() -> Devanagari {
    Devanagari{}
}
/// The "difference" value.
pub fn difference() -> Difference {
    Difference{}
}
/// The "disc" value.
pub fn disc() -> Disc {
    Disc{}
}
/// The "disclosure-closed" value.
pub fn disclosure_closed() -> DisclosureClosed {
    DisclosureClosed{}
}
/// The "disclosure-open" value.
pub fn disclosure_open() -> DisclosureOpen {
    DisclosureOpen{}
}
/// The "discrete" value.
pub fn discrete() -> Discrete {
    Discrete{}
}
/// The "distribute" value.
pub fn distribute() -> Distribute {
    Distribute{}
}
/// The "dot" value.
pub fn dot() -> Dot {
    Dot{}
}
/// The "dotted" value.
pub fn dotted() -> Dotted {
    Dotted{}
}
/// The "double" value.
pub fn double() -> Double {
    Double{}
}
/// The "double-circle" value.
pub fn double_circle() -> DoubleCircle {
    DoubleCircle{}
}
/// The "dpcm" value.
pub fn dpcm() -> Dpcm {
    Dpcm{}
}
/// The "dpi" value.
pub fn dpi() -> Dpi {
    Dpi{}
}
/// The "dppx" value.
pub fn dppx() -> Dppx {
    Dppx{}
}
/// The "duplicate" value.
pub fn duplicate() -> Duplicate {
    Duplicate{}
}
/// The "each-line" value.
pub fn each_line() -> EachLine {
    EachLine{}
}
/// The "ease" value.
pub fn ease() -> Ease {
    Ease{}
}
/// The "ease-in" value.
pub fn ease_in() -> EaseIn {
    EaseIn{}
}
/// The "ease-in-out" value.
pub fn ease_in_out() -> EaseInOut {
    EaseInOut{}
}
/// The "ease-out" value.
pub fn ease_out() -> EaseOut {
    EaseOut{}
}
/// The "ellipse" value.
pub fn ellipse() -> Ellipse {
    Ellipse{}
}
/// The "ellipsis" value.
pub fn ellipsis() -> Ellipsis {
    Ellipsis{}
}
/// The "em" value.
pub fn em(x: i32) -> Length {
    unimplemented!()
}
/// The "embed" value.
pub fn embed() -> Embed {
    Embed{}
}
/// The "embossed" value.
pub fn embossed() -> Embossed {
    Embossed{}
}
/// The "end" value.
pub fn end() -> End {
    End{}
}
/// The "e-resize" value.
pub fn e_resize() -> EResize {
    EResize{}
}
/// The "ethiopic-numeric" value.
pub fn ethiopic_numeric() -> EthiopicNumeric {
    EthiopicNumeric{}
}
/// The "evenodd" value.
pub fn evenodd() -> Evenodd {
    Evenodd{}
}
/// The "ew-resize" value.
pub fn ew_resize() -> EwResize {
    EwResize{}
}
/// The "ex" value.
pub fn ex() -> Ex {
    Ex{}
}
/// The "exclude" value.
pub fn exclude() -> Exclude {
    Exclude{}
}
/// The "exclusion" value.
pub fn exclusion() -> Exclusion {
    Exclusion{}
}
/// The "extends" value.
pub fn extends() -> Extends {
    Extends{}
}
/// The "farthest-corner" value.
pub fn farthest_corner() -> FarthestCorner {
    FarthestCorner{}
}
/// The "farthest-side" value.
pub fn farthest_side() -> FarthestSide {
    FarthestSide{}
}
/// The "fast" value.
pub fn fast() -> Fast {
    Fast{}
}
/// The "fill" value.
pub fn fill() -> Fill {
    Fill{}
}
/// The "fill-box" value.
pub fn fill_box() -> FillBox {
    FillBox{}
}
/// The "filled" value.
pub fn filled() -> Filled {
    Filled{}
}
/// The "fillpaint" value.
pub fn fillpaint() -> Fillpaint {
    Fillpaint{}
}
/// The "fine" value.
pub fn fine() -> Fine {
    Fine{}
}
/// The "first" value.
pub fn first() -> First {
    First{}
}
// first baseline
/// The "fit-content()" value.
pub fn fit_content() -> FitContent {
    FitContent{}
}
/// The "fixed" value.
pub fn fixed() -> Fixed {
    Fixed{}
}
/// The "flex" value.
pub fn flex() -> Flex {
    Flex{}
}
/// The "flex-end" value.
pub fn flex_end() -> FlexEnd {
    FlexEnd{}
}
/// The "flex-start" value.
pub fn flex_start() -> FlexStart {
    FlexStart{}
}
/// The "font-feature-settings" value.
pub fn font_feature_settings() -> FontFeatureSettings {
    FontFeatureSettings{}
}
/// The "font-variant" value.
pub fn font_variant() -> FontVariant {
    FontVariant{}
}
/// The "force-end" value.
pub fn force_end() -> ForceEnd {
    ForceEnd{}
}
/// The "forwards" value.
pub fn forwards() -> Forwards {
    Forwards{}
}
/// The "fr" value.
pub fn fr() -> Fr {
    Fr{}
}
/// The "from-image" value.
pub fn from_image() -> FromImage {
    FromImage{}
}
// fr unit
/// The "full-size-kana" value.
pub fn full_size_kana() -> FullSizeKana {
    FullSizeKana{}
}
/// The "full-width" value.
pub fn full_width() -> FullWidth {
    FullWidth{}
}
/// The "gamma" value.
pub fn gamma() -> Gamma {
    Gamma{}
}
/// The "georgian" value.
pub fn georgian() -> Georgian {
    Georgian{}
}
/// The "grab" value.
pub fn grab() -> Grab {
    Grab{}
}
/// The "grabbing" value.
pub fn grabbing() -> Grabbing {
    Grabbing{}
}
/// The "grad" value.
pub fn grad() -> Angle {
    Angle::Grad
}
/// The "grid" value.
pub fn grid() -> Grid {
    Grid{}
}
/// The "groove" value.
pub fn groove() -> Groove {
    Groove{}
}
/// The "gujarati" value.
pub fn gujarati() -> Gujarati {
    Gujarati{}
}
/// The "gurmukhi" value.
pub fn gurmukhi() -> Gurmukhi {
    Gurmukhi{}
}
/// The "handheld" value.
pub fn handheld() -> Handheld {
    Handheld{}
}
/// The "hanging" value.
pub fn hanging() -> Hanging {
    Hanging{}
}
/// The "hard-light" value.
pub fn hard_light() -> HardLight {
    HardLight{}
}
/// The "hebrew" value.
pub fn hebrew() -> Hebrew {
    Hebrew{}
}
/// The "help" value.
pub fn help() -> Help {
    Help{}
}
/// The "hidden" value.
pub fn hidden() -> Hidden {
    Hidden{}
}
/// The "high-quality" value.
pub fn high_quality() -> HighQuality {
    HighQuality{}
}
/// The "hiragana" value.
pub fn hiragana() -> Hiragana {
    Hiragana{}
}
/// The "hiragana-iroha" value.
pub fn hiragana_iroha() -> HiraganaIroha {
    HiraganaIroha{}
}
/// The "horizontal-tb" value.
pub fn horizontal_tb() -> HorizontalTb {
    HorizontalTb{}
}
/// The "hover" value.
pub fn hover() -> Hover {
    Hover{}
}
/// The "hue" value.
pub fn hue() -> Hue {
    Hue{}
}
/// The "hz" value.
pub fn hz() -> Hz {
    Hz{}
}
/// The "identity" value.
pub fn identity() -> Identity {
    Identity{}
}
/// The "in" value.
pub fn in_(x: i32) -> Length {
    unimplemented!()
}
/// The "infinite" value.
pub fn infinite() -> Infinite {
    Infinite{}
}
/// The "inherit" value.
pub fn inherit() -> Inherit {
    Inherit{}
}
/// The "initial" value.
pub fn initial() -> Initial {
    Initial{}
}
/// The "inline" value.
pub fn inline() -> Inline {
    Inline{}
}
/// The "inline-block" value.
pub fn inline_block() -> InlineBlock {
    InlineBlock{}
}
/// The "inline-flex" value.
pub fn inline_flex() -> InlineFlex {
    InlineFlex{}
}
/// The "inline-grid" value.
pub fn inline_grid() -> InlineGrid {
    InlineGrid{}
}
/// The "inline-table" value.
pub fn inline_table() -> InlineTable {
    InlineTable{}
}
/// The "inset" value.
pub fn inset() -> Inset {
    Inset{}
}
/// The "inter-character" value.
pub fn inter_character() -> InterCharacter {
    InterCharacter{}
}
/// The "interlace" value.
pub fn interlace() -> Interlace {
    Interlace{}
}
/// The "intersect" value.
pub fn intersect() -> Intersect {
    Intersect{}
}
/// The "inter-word" value.
pub fn inter_word() -> InterWord {
    InterWord{}
}
/// The "invert" value.
pub fn invert() -> Invert {
    Invert{}
}
/// The "isolate" value.
pub fn isolate() -> Isolate {
    Isolate{}
}
/// The "isolate-override" value.
pub fn isolate_override() -> IsolateOverride {
    IsolateOverride{}
}
/// The "japanese-formal" value.
pub fn japanese_formal() -> JapaneseFormal {
    JapaneseFormal{}
}
/// The "japanese-informal" value.
pub fn japanese_informal() -> JapaneseInformal {
    JapaneseInformal{}
}
/// The "jump-both" value.
pub fn jump_both() -> JumpBoth {
    JumpBoth{}
}
/// The "jump-end" value.
pub fn jump_end() -> JumpEnd {
    JumpEnd{}
}
/// The "jump-none" value.
pub fn jump_none() -> JumpNone {
    JumpNone{}
}
/// The "jump-start" value.
pub fn jump_start() -> JumpStart {
    JumpStart{}
}
/// The "justify" value.
pub fn justify() -> Justify {
    Justify{}
}
/// The "justify-all" value.
pub fn justify_all() -> JustifyAll {
    JustifyAll{}
}
/// The "kannada" value.
pub fn kannada() -> Kannada {
    Kannada{}
}
/// The "katakana" value.
pub fn katakana() -> Katakana {
    Katakana{}
}
/// The "katakana-iroha" value.
pub fn katakana_iroha() -> KatakanaIroha {
    KatakanaIroha{}
}
/// The "keep-all" value.
pub fn keep_all() -> KeepAll {
    KeepAll{}
}
/// The "khmer" value.
pub fn khmer() -> Khmer {
    Khmer{}
}
/// The "khz" value.
pub fn khz() -> Khz {
    Khz{}
}
/// The "korean-hangul-formal" value.
pub fn korean_hangul_formal() -> KoreanHangulFormal {
    KoreanHangulFormal{}
}
/// The "korean-hanja-formal" value.
pub fn korean_hanja_formal() -> KoreanHanjaFormal {
    KoreanHanjaFormal{}
}
/// The "korean-hanja-informal" value.
pub fn korean_hanja_informal() -> KoreanHanjaInformal {
    KoreanHanjaInformal{}
}
/// The "landscape" value.
pub fn landscape() -> Landscape {
    Landscape{}
}
/// The "lao" value.
pub fn lao() -> Lao {
    Lao{}
}
/// The "last" value.
pub fn last() -> Last {
    Last{}
}
// last baseline
/// The "layout" value.
pub fn layout() -> Layout {
    Layout{}
}
/// The "left" value.
pub fn left() -> Left {
    Left{}
}
/// The "legacy" value.
pub fn legacy() -> Legacy {
    Legacy{}
}
/// The "lighten" value.
pub fn lighten() -> Lighten {
    Lighten{}
}
/// The "linear" value.
pub fn linear() -> Linear {
    Linear{}
}
/// The "linearrgb" value.
pub fn linearrgb() -> Linearrgb {
    Linearrgb{}
}
/// The "line-through" value.
pub fn line_through() -> LineThrough {
    LineThrough{}
}
/// The "list-item" value.
pub fn list_item() -> ListItem {
    ListItem{}
}
/// The "local" value.
pub fn local() -> Local {
    Local{}
}
/// The "loose" value.
pub fn loose() -> Loose {
    Loose{}
}
/// The "lower-alpha" value.
pub fn lower_alpha() -> LowerAlpha {
    LowerAlpha{}
}
/// The "lower-armenian" value.
pub fn lower_armenian() -> LowerArmenian {
    LowerArmenian{}
}
/// The "lowercase" value.
pub fn lowercase() -> Lowercase {
    Lowercase{}
}
/// The "lower-greek" value.
pub fn lower_greek() -> LowerGreek {
    LowerGreek{}
}
/// The "lower-latin" value.
pub fn lower_latin() -> LowerLatin {
    LowerLatin{}
}
/// The "lower-roman" value.
pub fn lower_roman() -> LowerRoman {
    LowerRoman{}
}
/// The "ltr" value.
pub fn ltr() -> Ltr {
    Ltr{}
}
/// The "luminance" value.
pub fn luminance() -> Luminance {
    Luminance{}
}
/// The "luminosity" value.
pub fn luminosity() -> Luminosity {
    Luminosity{}
}
/// The "malayalam" value.
pub fn malayalam() -> Malayalam {
    Malayalam{}
}
/// The "mandatory" value.
pub fn mandatory() -> Mandatory {
    Mandatory{}
}
/// The "manual" value.
pub fn manual() -> Manual {
    Manual{}
}
/// The "margin-box" value.
pub fn margin_box() -> MarginBox {
    MarginBox{}
}
/// The "match-parent" value.
pub fn match_parent() -> MatchParent {
    MatchParent{}
}
/// The "match-source" value.
pub fn match_source() -> MatchSource {
    MatchSource{}
}
/// The "max-content" value.
pub fn max_content() -> MaxContent {
    MaxContent{}
}
/// The "medium" value.
pub fn medium() -> Medium {
    Medium{}
}
/// The "min-content" value.
pub fn min_content() -> MinContent {
    MinContent{}
}
/// The "minmax()" value.
pub fn minmax() -> Minmax {
    Minmax{}
}
/// The "mixed" value.
pub fn mixed() -> Mixed {
    Mixed{}
}
/// The "mm" value.
pub fn mm(x: i32) -> Length {
    unimplemented!()
}
/// The "mongolian" value.
pub fn mongolian() -> Mongolian {
    Mongolian{}
}
/// The "move" value.
pub fn move_() -> Move {
    Move{}
}
/// The "ms" value.
pub fn ms(x: i32) -> Time {
    Time::Ms(x)
}
/// The "multiply" value.
pub fn multiply() -> Multiply {
    Multiply{}
}
/// The "myanmar" value.
pub fn myanmar() -> Myanmar {
    Myanmar{}
}
/// The "ne-resize" value.
pub fn ne_resize() -> NeResize {
    NeResize{}
}
/// The "nesw-resize" value.
pub fn nesw_resize() -> NeswResize {
    NeswResize{}
}
/// The "no-clip" value.
pub fn no_clip() -> NoClip {
    NoClip{}
}
/// The "no-close-quote" value.
pub fn no_close_quote() -> NoCloseQuote {
    NoCloseQuote{}
}
/// The "no-composite" value.
pub fn no_composite() -> NoComposite {
    NoComposite{}
}
/// The "no-drop" value.
pub fn no_drop() -> NoDrop {
    NoDrop{}
}
/// The "none" value.
pub fn none() -> None {
    None{}
}
// none!!font-variant
/// The "nonzero" value.
pub fn nonzero() -> Nonzero {
    Nonzero{}
}
/// The "no-open-quote" value.
pub fn no_open_quote() -> NoOpenQuote {
    NoOpenQuote{}
}
/// The "no-repeat" value.
pub fn no_repeat() -> NoRepeat {
    NoRepeat{}
}
/// The "normal" value.
pub fn normal() -> Normal {
    Normal{}
}
// normal!!font-feature-settings
// normal!!font-variant
/// The "not" value.
pub fn not() -> Not {
    Not{}
}
/// The "not-allowed" value.
pub fn not_allowed() -> NotAllowed {
    NotAllowed{}
}
/// The "nowrap" value.
pub fn nowrap() -> Nowrap {
    Nowrap{}
}
/// The "n-resize" value.
pub fn n_resize() -> NResize {
    NResize{}
}
/// The "ns-resize" value.
pub fn ns_resize() -> NsResize {
    NsResize{}
}
/// The "numbers" value.
pub fn numbers() -> Numbers {
    Numbers{}
}
/// The "numeric" value.
pub fn numeric() -> Numeric {
    Numeric{}
}
/// The "nw-resize" value.
pub fn nw_resize() -> NwResize {
    NwResize{}
}
/// The "nwse-resize" value.
pub fn nwse_resize() -> NwseResize {
    NwseResize{}
}
/// The "objectboundingbox" value.
pub fn objectboundingbox() -> Objectboundingbox {
    Objectboundingbox{}
}
/// The "only" value.
pub fn only() -> Only {
    Only{}
}
/// The "open" value.
pub fn open() -> Open {
    Open{}
}
/// The "open-quote" value.
pub fn open_quote() -> OpenQuote {
    OpenQuote{}
}
/// The "optional-paged" value.
pub fn optional_paged() -> OptionalPaged {
    OptionalPaged{}
}
/// The "oriya" value.
pub fn oriya() -> Oriya {
    Oriya{}
}
/// The "outset" value.
pub fn outset() -> Outset {
    Outset{}
}
/// The "over" value.
pub fn over() -> Over {
    Over{}
}
/// The "overlay" value.
pub fn overlay() -> Overlay {
    Overlay{}
}
/// The "overline" value.
pub fn overline() -> Overline {
    Overline{}
}
/// The "p3" value.
pub fn p3() -> P3 {
    P3{}
}
/// The "padding-box" value.
pub fn padding_box() -> PaddingBox {
    PaddingBox{}
}
/// The "page" value.
pub fn page() -> Page {
    Page{}
}
/// The "paged" value.
pub fn paged() -> Paged {
    Paged{}
}
/// The "paint" value.
pub fn paint() -> Paint {
    Paint{}
}
/// The "paused" value.
pub fn paused() -> Paused {
    Paused{}
}
/// The "pc" value.
pub fn pc() -> Pc {
    Pc{}
}
/// The "persian" value.
pub fn persian() -> Persian {
    Persian{}
}
/// The "pixelated" value.
pub fn pixelated() -> Pixelated {
    Pixelated{}
}
// pixel unit
/// The "plaintext" value.
pub fn plaintext() -> Plaintext {
    Plaintext{}
}
/// The "pointer" value.
pub fn pointer() -> Pointer {
    Pointer{}
}
/// The "portrait" value.
pub fn portrait() -> Portrait {
    Portrait{}
}
/// The "pre" value.
pub fn pre() -> Pre {
    Pre{}
}
/// The "pre-line" value.
pub fn pre_line() -> PreLine {
    PreLine{}
}
/// The "pre-wrap" value.
pub fn pre_wrap() -> PreWrap {
    PreWrap{}
}
/// The "print" value.
pub fn print() -> Print {
    Print{}
}
/// The "progress" value.
pub fn progress() -> Progress {
    Progress{}
}
/// The "progressive" value.
pub fn progressive() -> Progressive {
    Progressive{}
}
/// The "projection" value.
pub fn projection() -> Projection {
    Projection{}
}
/// The "proximity" value.
pub fn proximity() -> Proximity {
    Proximity{}
}
/// The "pt" value.
pub fn pt(x: i32) -> Length {
    unimplemented!()
}
/// The "px" value.
pub fn px(x: impl Number) -> Length {
    Length::Px {
        v: x.normalize(),
    }
}
/// The "q" value.
pub fn q(x: i32) -> Length {
    unimplemented!()
}
/// The "rad" value.
pub fn rad() -> Angle {
    Angle::Rad
}
/// The "rec2020" value.
pub fn rec2020() -> Rec2020 {
    Rec2020{}
}
/// The "recto" value.
pub fn recto() -> Recto {
    Recto{}
}
/// The "region" value.
pub fn region() -> Region {
    Region{}
}
/// The "rem" value.
pub fn rem(x: i32) -> Length {
    unimplemented!()
}
/// The "repeat" value.
pub fn repeat() -> Repeat {
    Repeat{}
}
/// The "repeat-x" value.
pub fn repeat_x() -> RepeatX {
    RepeatX{}
}
/// The "repeat-y" value.
pub fn repeat_y() -> RepeatY {
    RepeatY{}
}
/// The "reverse" value.
pub fn reverse() -> Reverse {
    Reverse{}
}
/// The "revert" value.
pub fn revert() -> Revert {
    Revert{}
}
/// The "ridge" value.
pub fn ridge() -> Ridge {
    Ridge{}
}
/// The "right" value.
pub fn right() -> Right {
    Right{}
}
/// The "rotate()" value.
pub fn rotate() -> Rotate {
    Rotate{}
}
/// The "round" value.
pub fn round() -> Round {
    Round{}
}
/// The "row" value.
pub fn row() -> Row {
    Row{}
}
/// The "row-resize" value.
pub fn row_resize() -> RowResize {
    RowResize{}
}
/// The "row-reverse" value.
pub fn row_reverse() -> RowReverse {
    RowReverse{}
}
/// The "rtl" value.
pub fn rtl() -> Rtl {
    Rtl{}
}
/// The "running" value.
pub fn running() -> Running {
    Running{}
}
/// The "s" value.
pub fn s(value: i32) -> Time {
    Time::S(value)
}
/// The "safe" value.
pub fn safe() -> Safe {
    Safe{}
}
/// The "saturation" value.
pub fn saturation() -> Saturation {
    Saturation{}
}
/// The "scale()" value.
pub fn scale() -> Scale {
    Scale{}
}
/// The "scale-down" value.
pub fn scale_down() -> ScaleDown {
    ScaleDown{}
}
/// The "scalex()" value.
pub fn scalex() -> Scalex {
    Scalex{}
}
/// The "scaley()" value.
pub fn scaley() -> Scaley {
    Scaley{}
}
/// The "screen" value.
pub fn screen() -> Screen {
    Screen{}
}
/// The "scroll" value.
pub fn scroll() -> Scroll {
    Scroll{}
}
/// The "scroll-position" value.
pub fn scroll_position() -> ScrollPosition {
    ScrollPosition{}
}
/// The "self-end" value.
pub fn self_end() -> SelfEnd {
    SelfEnd{}
}
/// The "self-start" value.
pub fn self_start() -> SelfStart {
    SelfStart{}
}
/// The "se-resize" value.
pub fn se_resize() -> SeResize {
    SeResize{}
}
/// The "sesame" value.
pub fn sesame() -> Sesame {
    Sesame{}
}
/// The "sideways" value.
pub fn sideways() -> Sideways {
    Sideways{}
}
/// The "sideways-right" value.
pub fn sideways_right() -> SidewaysRight {
    SidewaysRight{}
}
/// The "simp-chinese-formal" value.
pub fn simp_chinese_formal() -> SimpChineseFormal {
    SimpChineseFormal{}
}
/// The "simp-chinese-informal" value.
pub fn simp_chinese_informal() -> SimpChineseInformal {
    SimpChineseInformal{}
}
/// The "size" value.
pub fn size() -> Size {
    Size{}
}
/// The "skew()" value.
pub fn skew() -> Skew {
    Skew{}
}
/// The "skewx()" value.
pub fn skewx() -> Skewx {
    Skewx{}
}
/// The "skewy()" value.
pub fn skewy() -> Skewy {
    Skewy{}
}
/// The "slice" value.
pub fn slice() -> Slice {
    Slice{}
}
/// The "slow" value.
pub fn slow() -> Slow {
    Slow{}
}
/// The "smooth" value.
pub fn smooth() -> Smooth {
    Smooth{}
}
/// The "soft-light" value.
pub fn soft_light() -> SoftLight {
    SoftLight{}
}
/// The "solid" value.
pub fn solid() -> Solid {
    Solid{}
}
/// The "sourcealpha" value.
pub fn sourcealpha() -> Sourcealpha {
    Sourcealpha{}
}
/// The "sourcegraphic" value.
pub fn sourcegraphic() -> Sourcegraphic {
    Sourcegraphic{}
}
/// The "space" value.
pub fn space() -> Space {
    Space{}
}
/// The "space-around" value.
pub fn space_around() -> SpaceAround {
    SpaceAround{}
}
/// The "space-between" value.
pub fn space_between() -> SpaceBetween {
    SpaceBetween{}
}
/// The "space-evenly" value.
pub fn space_evenly() -> SpaceEvenly {
    SpaceEvenly{}
}
// span && [ <integer> || <custom-ident> ]
/// The "speech" value.
pub fn speech() -> Speech {
    Speech{}
}
/// The "spell-out" value.
pub fn spell_out() -> SpellOut {
    SpellOut{}
}
/// The "square" value.
pub fn square() -> Square {
    Square{}
}
/// The "s-resize" value.
pub fn s_resize() -> SResize {
    SResize{}
}
/// The "srgb" value.
pub fn srgb() -> Srgb {
    Srgb{}
}
/// The "start" value.
pub fn start() -> Start {
    Start{}
}
/// The "step-end" value.
pub fn step_end() -> StepEnd {
    StepEnd{}
}
/// The "step-start" value.
pub fn step_start() -> StepStart {
    StepStart{}
}
/// The "stretch" value.
pub fn stretch() -> Stretch {
    Stretch{}
}
/// The "strict" value.
pub fn strict() -> Strict {
    Strict{}
}
/// The "stroke-box" value.
pub fn stroke_box() -> StrokeBox {
    StrokeBox{}
}
/// The "strokepaint" value.
pub fn strokepaint() -> Strokepaint {
    Strokepaint{}
}
/// The "style" value.
pub fn style() -> Style {
    Style{}
}
/// The "subtract" value.
pub fn subtract() -> Subtract {
    Subtract{}
}
/// The "sw-resize" value.
pub fn sw_resize() -> SwResize {
    SwResize{}
}
/// The "symbolic" value.
pub fn symbolic() -> Symbolic {
    Symbolic{}
}
/// The "table" value.
pub fn table() -> Table {
    Table{}
}
/// The "table-caption" value.
pub fn table_caption() -> TableCaption {
    TableCaption{}
}
/// The "table-cell" value.
pub fn table_cell() -> TableCell {
    TableCell{}
}
/// The "table-column" value.
pub fn table_column() -> TableColumn {
    TableColumn{}
}
/// The "table-column-group" value.
pub fn table_column_group() -> TableColumnGroup {
    TableColumnGroup{}
}
/// The "table-footer-group" value.
pub fn table_footer_group() -> TableFooterGroup {
    TableFooterGroup{}
}
/// The "table-header-group" value.
pub fn table_header_group() -> TableHeaderGroup {
    TableHeaderGroup{}
}
/// The "table-row" value.
pub fn table_row() -> TableRow {
    TableRow{}
}
/// The "table-row-group" value.
pub fn table_row_group() -> TableRowGroup {
    TableRowGroup{}
}
/// The "tamil" value.
pub fn tamil() -> Tamil {
    Tamil{}
}
/// The "telugu" value.
pub fn telugu() -> Telugu {
    Telugu{}
}
/// The "text" value.
pub fn text() -> Text {
    Text{}
}
/// The "thai" value.
pub fn thai() -> Thai {
    Thai{}
}
/// The "thick" value.
pub fn thick() -> Thick {
    Thick{}
}
/// The "thin" value.
pub fn thin() -> Thin {
    Thin{}
}
/// The "tibetan" value.
pub fn tibetan() -> Tibetan {
    Tibetan{}
}
/// The "top" value.
pub fn top() -> Top {
    Top{}
}
/// The "trad-chinese-formal" value.
pub fn trad_chinese_formal() -> TradChineseFormal {
    TradChineseFormal{}
}
/// The "trad-chinese-informal" value.
pub fn trad_chinese_informal() -> TradChineseInformal {
    TradChineseInformal{}
}
/// The "translate()" value.
pub fn translate() -> Translate {
    Translate{}
}
/// The "translatex()" value.
pub fn translatex() -> Translatex {
    Translatex{}
}
/// The "translatey()" value.
pub fn translatey() -> Translatey {
    Translatey{}
}
/// The "triangle" value.
pub fn triangle() -> Triangle {
    Triangle{}
}
/// The "tty" value.
pub fn tty() -> Tty {
    Tty{}
}
/// The "turn" value.
pub fn turn() -> Angle {
    Angle::Turn
}
/// The "tv" value.
pub fn tv() -> Tv {
    Tv{}
}
/// The "under" value.
pub fn under() -> Under {
    Under{}
}
/// The "underline" value.
pub fn underline() -> Underline {
    Underline{}
}
/// The "unsafe" value.
pub fn unsafe_() -> Unsafe {
    Unsafe{}
}
/// The "unset" value.
pub fn unset() -> Unset {
    Unset{}
}
/// The "upper-alpha" value.
pub fn upper_alpha() -> UpperAlpha {
    UpperAlpha{}
}
/// The "upper-armenian" value.
pub fn upper_armenian() -> UpperArmenian {
    UpperArmenian{}
}
/// The "uppercase" value.
pub fn uppercase() -> Uppercase {
    Uppercase{}
}
/// The "upper-latin" value.
pub fn upper_latin() -> UpperLatin {
    UpperLatin{}
}
/// The "upper-roman" value.
pub fn upper_roman() -> UpperRoman {
    UpperRoman{}
}
/// The "upright" value.
pub fn upright() -> Upright {
    Upright{}
}
/// The "userspaceonuse" value.
pub fn userspaceonuse() -> Userspaceonuse {
    Userspaceonuse{}
}
pub fn url(value: String) -> Url {
    Url(value)
}
/// The "verso" value.
pub fn verso() -> Verso {
    Verso{}
}
/// The "vertical-lr" value.
pub fn vertical_lr() -> VerticalLr {
    VerticalLr{}
}
/// The "vertical-rl" value.
pub fn vertical_rl() -> VerticalRl {
    VerticalRl{}
}
/// The "vertical-text" value.
pub fn vertical_text() -> VerticalText {
    VerticalText{}
}
/// The "vh" value.
pub fn vh() -> Vh {
    Vh{}
}
/// The "view-box" value.
pub fn view_box() -> ViewBox {
    ViewBox{}
}
/// The "vmax" value.
pub fn vmax() -> Vmax {
    Vmax{}
}
/// The "vmin" value.
pub fn vmin() -> Vmin {
    Vmin{}
}
/// The "vw" value.
pub fn vw() -> Vw {
    Vw{}
}
/// The "wait" value.
pub fn wait() -> Wait {
    Wait{}
}
/// The "words" value.
pub fn words() -> Words {
    Words{}
}
/// The "wrap" value.
pub fn wrap() -> Wrap {
    Wrap{}
}
/// The "wrap-reverse" value.
pub fn wrap_reverse() -> WrapReverse {
    WrapReverse{}
}
/// The "w-resize" value.
pub fn w_resize() -> WResize {
    WResize{}
}
/// The "x" value.
pub fn x() -> X {
    X{}
}
/// The "xor" value.
pub fn xor() -> Xor {
    Xor{}
}
/// The "y" value.
pub fn y() -> Y {
    Y{}
}
/// The "zoom-in" value.
pub fn zoom_in() -> ZoomIn {
    ZoomIn{}
}
/// The "zoom-out" value.
pub fn zoom_out() -> ZoomOut {
    ZoomOut{}
}