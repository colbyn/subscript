pub mod output;
pub mod types;

pub use types::*;
pub mod o {
    pub use super::output::*;
}

/// The "add" value.
pub fn add() -> o::Add {
    o::Add{}
}
/// The "additive" value.
pub fn additive() -> o::Additive {
    o::Additive{}
}
/// The "alias" value.
pub fn alias() -> o::Alias {
    o::Alias{}
}
/// The "all" value.
pub fn all() -> o::All {
    o::All{}
}
/// The "allow-end" value.
pub fn allow_end() -> o::AllowEnd {
    o::AllowEnd{}
}
/// The "all-scroll" value.
pub fn all_scroll() -> o::AllScroll {
    o::AllScroll{}
}
/// The "alpha" value.
pub fn alpha() -> o::Alpha {
    o::Alpha{}
}
/// The "alphabetic" value.
pub fn alphabetic() -> o::Alphabetic {
    o::Alphabetic{}
}
/// The "alternate" value.
pub fn alternate() -> o::Alternate {
    o::Alternate{}
}
/// The "alternate-reverse" value.
pub fn alternate_reverse() -> o::AlternateReverse {
    o::AlternateReverse{}
}
/// The "always" value.
pub fn always() -> o::Always {
    o::Always{}
}
/// The "anywhere" value.
pub fn anywhere() -> o::Anywhere {
    o::Anywhere{}
}
/// The "arabic-indic" value.
pub fn arabic_indic() -> o::ArabicIndic {
    o::ArabicIndic{}
}
/// The "arithmetic" value.
pub fn arithmetic() -> o::Arithmetic {
    o::Arithmetic{}
}
/// The "armenian" value.
pub fn armenian() -> o::Armenian {
    o::Armenian{}
}
/// The "atop" value.
pub fn atop() -> o::Atop {
    o::Atop{}
}
/// The "aural" value.
pub fn aural() -> o::Aural {
    o::Aural{}
}
/// The "auto" value.
pub fn auto() -> o::Auto {
    o::Auto{}
}
/// The "auto-fill" value.
pub fn auto_fill() -> o::AutoFill {
    o::AutoFill{}
}
/// The "auto-fit" value.
pub fn auto_fit() -> o::AutoFit {
    o::AutoFit{}
}
/// The "avoid" value.
pub fn avoid() -> o::Avoid {
    o::Avoid{}
}
/// The "avoid-column" value.
pub fn avoid_column() -> o::AvoidColumn {
    o::AvoidColumn{}
}
/// The "avoid-page" value.
pub fn avoid_page() -> o::AvoidPage {
    o::AvoidPage{}
}
/// The "avoid-region" value.
pub fn avoid_region() -> o::AvoidRegion {
    o::AvoidRegion{}
}
/// The "backgroundalpha" value.
pub fn backgroundalpha() -> o::Backgroundalpha {
    o::Backgroundalpha{}
}
/// The "backgroundimage" value.
pub fn backgroundimage() -> o::Backgroundimage {
    o::Backgroundimage{}
}
/// The "backwards" value.
pub fn backwards() -> o::Backwards {
    o::Backwards{}
}
/// The "balance" value.
pub fn balance() -> o::Balance {
    o::Balance{}
}
/// The "balance-all" value.
pub fn balance_all() -> o::BalanceAll {
    o::BalanceAll{}
}
/// The "baseline" value.
pub fn baseline() -> o::Baseline {
    o::Baseline{}
}
/// The "bengali" value.
pub fn bengali() -> o::Bengali {
    o::Bengali{}
}
/// The "bidi-override" value.
pub fn bidi_override() -> o::BidiOverride {
    o::BidiOverride{}
}
/// The "blink" value.
pub fn blink() -> o::Blink {
    o::Blink{}
}
/// The "block" value.
pub fn block() -> o::Block {
    o::Block{}
}
/// The "border-box" value.
pub fn border_box() -> o::BorderBox {
    o::BorderBox{}
}
/// The "both" value.
pub fn both() -> o::Both {
    o::Both{}
}
/// The "bottom" value.
pub fn bottom() -> o::Bottom {
    o::Bottom{}
}
/// The "braille" value.
pub fn braille() -> o::Braille {
    o::Braille{}
}
/// The "break-all" value.
pub fn break_all() -> o::BreakAll {
    o::BreakAll{}
}
/// The "break-spaces" value.
pub fn break_spaces() -> o::BreakSpaces {
    o::BreakSpaces{}
}
/// The "break-word" value.
pub fn break_word() -> o::BreakWord {
    o::BreakWord{}
}
/// The "bullets" value.
pub fn bullets() -> o::Bullets {
    o::Bullets{}
}
/// The "cambodian" value.
pub fn cambodian() -> o::Cambodian {
    o::Cambodian{}
}
/// The "capitalize" value.
pub fn capitalize() -> o::Capitalize {
    o::Capitalize{}
}
/// The "cell" value.
pub fn cell() -> o::Cell {
    o::Cell{}
}
/// The "center" value.
pub fn center() -> o::Center {
    o::Center{}
}
/// The "ch" value.
pub fn ch() -> o::Ch {
    o::Ch{}
}
/// The "circle" value.
pub fn circle() -> o::Circle {
    o::Circle{}
}
/// The "cjk-decimal" value.
pub fn cjk_decimal() -> o::CjkDecimal {
    o::CjkDecimal{}
}
/// The "cjk-earthly-branch" value.
pub fn cjk_earthly_branch() -> o::CjkEarthlyBranch {
    o::CjkEarthlyBranch{}
}
/// The "cjk-heavenly-stem" value.
pub fn cjk_heavenly_stem() -> o::CjkHeavenlyStem {
    o::CjkHeavenlyStem{}
}
/// The "cjk-ideographic" value.
pub fn cjk_ideographic() -> o::CjkIdeographic {
    o::CjkIdeographic{}
}
/// The "clip" value.
pub fn clip() -> o::Clip {
    o::Clip{}
}
/// The "clone" value.
pub fn clone() -> o::Clone {
    o::Clone{}
}
/// The "close-quote" value.
pub fn close_quote() -> o::CloseQuote {
    o::CloseQuote{}
}
/// The "closest-corner" value.
pub fn closest_corner() -> o::ClosestCorner {
    o::ClosestCorner{}
}
/// The "closest-side" value.
pub fn closest_side() -> o::ClosestSide {
    o::ClosestSide{}
}
/// The "cm" value.
pub fn cm(x: i32) -> o::Length {
    unimplemented!()
}
/// The "coarse" value.
pub fn coarse() -> o::Coarse {
    o::Coarse{}
}
/// The "color" value.
pub fn color() -> o::Color {
    o::Color{}
}
/// The "color-burn" value.
pub fn color_burn() -> o::ColorBurn {
    o::ColorBurn{}
}
/// The "color-dodge" value.
pub fn color_dodge() -> o::ColorDodge {
    o::ColorDodge{}
}
/// The "col-resize" value.
pub fn col_resize() -> o::ColResize {
    o::ColResize{}
}
/// The "column" value.
pub fn column() -> o::Column {
    o::Column{}
}
/// The "column-reverse" value.
pub fn column_reverse() -> o::ColumnReverse {
    o::ColumnReverse{}
}
/// The "contain" value.
pub fn contain() -> o::Contain {
    o::Contain{}
}
/// The "content" value.
pub fn content() -> o::Content {
    o::Content{}
}
/// The "content-box" value.
pub fn content_box() -> o::ContentBox {
    o::ContentBox{}
}
/// The "contents" value.
pub fn contents() -> o::Contents {
    o::Contents{}
}
/// The "context-menu" value.
pub fn context_menu() -> o::ContextMenu {
    o::ContextMenu{}
}
/// The "copy" value.
pub fn copy() -> o::Copy {
    o::Copy{}
}
/// The "cover" value.
pub fn cover() -> o::Cover {
    o::Cover{}
}
/// The "crisp-edges" value.
pub fn crisp_edges() -> o::CrispEdges {
    o::CrispEdges{}
}
/// The "crosshair" value.
pub fn crosshair() -> o::Crosshair {
    o::Crosshair{}
}
/// The "currentcolor" value.
pub fn currentcolor() -> o::Currentcolor {
    o::Currentcolor{}
}
/// The "cyclic" value.
pub fn cyclic() -> o::Cyclic {
    o::Cyclic{}
}
/// The "darken" value.
pub fn darken() -> o::Darken {
    o::Darken{}
}
/// The "dashed" value.
pub fn dashed() -> o::Dashed {
    o::Dashed{}
}
/// The "decimal" value.
pub fn decimal() -> o::Decimal {
    o::Decimal{}
}
/// The "decimal-leading-zero" value.
pub fn decimal_leading_zero() -> o::DecimalLeadingZero {
    o::DecimalLeadingZero{}
}
/// The "default" value.
pub fn default() -> o::Default {
    o::Default{}
}
/// The "deg" value.
pub fn deg() -> o::Angle {
    o::Angle::Deg
}
/// The "dense" value.
pub fn dense() -> o::Dense {
    o::Dense{}
}
/// The "devanagari" value.
pub fn devanagari() -> o::Devanagari {
    o::Devanagari{}
}
/// The "difference" value.
pub fn difference() -> o::Difference {
    o::Difference{}
}
/// The "disc" value.
pub fn disc() -> o::Disc {
    o::Disc{}
}
/// The "disclosure-closed" value.
pub fn disclosure_closed() -> o::DisclosureClosed {
    o::DisclosureClosed{}
}
/// The "disclosure-open" value.
pub fn disclosure_open() -> o::DisclosureOpen {
    o::DisclosureOpen{}
}
/// The "discrete" value.
pub fn discrete() -> o::Discrete {
    o::Discrete{}
}
/// The "distribute" value.
pub fn distribute() -> o::Distribute {
    o::Distribute{}
}
/// The "dot" value.
pub fn dot() -> o::Dot {
    o::Dot{}
}
/// The "dotted" value.
pub fn dotted() -> o::Dotted {
    o::Dotted{}
}
/// The "double" value.
pub fn double() -> o::Double {
    o::Double{}
}
/// The "double-circle" value.
pub fn double_circle() -> o::DoubleCircle {
    o::DoubleCircle{}
}
/// The "dpcm" value.
pub fn dpcm() -> o::Dpcm {
    o::Dpcm{}
}
/// The "dpi" value.
pub fn dpi() -> o::Dpi {
    o::Dpi{}
}
/// The "dppx" value.
pub fn dppx() -> o::Dppx {
    o::Dppx{}
}
/// The "duplicate" value.
pub fn duplicate() -> o::Duplicate {
    o::Duplicate{}
}
/// The "each-line" value.
pub fn each_line() -> o::EachLine {
    o::EachLine{}
}
/// The "ease" value.
pub fn ease() -> o::Ease {
    o::Ease{}
}
/// The "ease-in" value.
pub fn ease_in() -> o::EaseIn {
    o::EaseIn{}
}
/// The "ease-in-out" value.
pub fn ease_in_out() -> o::EaseInOut {
    o::EaseInOut{}
}
/// The "ease-out" value.
pub fn ease_out() -> o::EaseOut {
    o::EaseOut{}
}
/// The "ellipse" value.
pub fn ellipse() -> o::Ellipse {
    o::Ellipse{}
}
/// The "ellipsis" value.
pub fn ellipsis() -> o::Ellipsis {
    o::Ellipsis{}
}
/// The "em" value.
pub fn em(x: i32) -> o::Length {
    unimplemented!()
}
/// The "embed" value.
pub fn embed() -> o::Embed {
    o::Embed{}
}
/// The "embossed" value.
pub fn embossed() -> o::Embossed {
    o::Embossed{}
}
/// The "end" value.
pub fn end() -> o::End {
    o::End{}
}
/// The "e-resize" value.
pub fn e_resize() -> o::EResize {
    o::EResize{}
}
/// The "ethiopic-numeric" value.
pub fn ethiopic_numeric() -> o::EthiopicNumeric {
    o::EthiopicNumeric{}
}
/// The "evenodd" value.
pub fn evenodd() -> o::Evenodd {
    o::Evenodd{}
}
/// The "ew-resize" value.
pub fn ew_resize() -> o::EwResize {
    o::EwResize{}
}
/// The "ex" value.
pub fn ex() -> o::Ex {
    o::Ex{}
}
/// The "exclude" value.
pub fn exclude() -> o::Exclude {
    o::Exclude{}
}
/// The "exclusion" value.
pub fn exclusion() -> o::Exclusion {
    o::Exclusion{}
}
/// The "extends" value.
pub fn extends() -> o::Extends {
    o::Extends{}
}
/// The "farthest-corner" value.
pub fn farthest_corner() -> o::FarthestCorner {
    o::FarthestCorner{}
}
/// The "farthest-side" value.
pub fn farthest_side() -> o::FarthestSide {
    o::FarthestSide{}
}
/// The "fast" value.
pub fn fast() -> o::Fast {
    o::Fast{}
}
/// The "fill" value.
pub fn fill() -> o::Fill {
    o::Fill{}
}
/// The "fill-box" value.
pub fn fill_box() -> o::FillBox {
    o::FillBox{}
}
/// The "filled" value.
pub fn filled() -> o::Filled {
    o::Filled{}
}
/// The "fillpaint" value.
pub fn fillpaint() -> o::Fillpaint {
    o::Fillpaint{}
}
/// The "fine" value.
pub fn fine() -> o::Fine {
    o::Fine{}
}
/// The "first" value.
pub fn first() -> o::First {
    o::First{}
}
// first baseline
/// The "fit-content()" value.
pub fn fit_content() -> o::FitContent {
    o::FitContent{}
}
/// The "fixed" value.
pub fn fixed() -> o::Fixed {
    o::Fixed{}
}
/// The "flex" value.
pub fn flex() -> o::Flex {
    o::Flex{}
}
/// The "flex-end" value.
pub fn flex_end() -> o::FlexEnd {
    o::FlexEnd{}
}
/// The "flex-start" value.
pub fn flex_start() -> o::FlexStart {
    o::FlexStart{}
}
/// The "font-feature-settings" value.
pub fn font_feature_settings() -> o::FontFeatureSettings {
    o::FontFeatureSettings{}
}
/// The "font-variant" value.
pub fn font_variant() -> o::FontVariant {
    o::FontVariant{}
}
/// The "force-end" value.
pub fn force_end() -> o::ForceEnd {
    o::ForceEnd{}
}
/// The "forwards" value.
pub fn forwards() -> o::Forwards {
    o::Forwards{}
}
/// The "fr" value.
pub fn fr() -> o::Fr {
    o::Fr{}
}
/// The "from-image" value.
pub fn from_image() -> o::FromImage {
    o::FromImage{}
}
// fr unit
/// The "full-size-kana" value.
pub fn full_size_kana() -> o::FullSizeKana {
    o::FullSizeKana{}
}
/// The "full-width" value.
pub fn full_width() -> o::FullWidth {
    o::FullWidth{}
}
/// The "gamma" value.
pub fn gamma() -> o::Gamma {
    o::Gamma{}
}
/// The "georgian" value.
pub fn georgian() -> o::Georgian {
    o::Georgian{}
}
/// The "grab" value.
pub fn grab() -> o::Grab {
    o::Grab{}
}
/// The "grabbing" value.
pub fn grabbing() -> o::Grabbing {
    o::Grabbing{}
}
/// The "grad" value.
pub fn grad() -> o::Angle {
    o::Angle::Grad
}
/// The "grid" value.
pub fn grid() -> o::Grid {
    o::Grid{}
}
/// The "groove" value.
pub fn groove() -> o::Groove {
    o::Groove{}
}
/// The "gujarati" value.
pub fn gujarati() -> o::Gujarati {
    o::Gujarati{}
}
/// The "gurmukhi" value.
pub fn gurmukhi() -> o::Gurmukhi {
    o::Gurmukhi{}
}
/// The "handheld" value.
pub fn handheld() -> o::Handheld {
    o::Handheld{}
}
/// The "hanging" value.
pub fn hanging() -> o::Hanging {
    o::Hanging{}
}
/// The "hard-light" value.
pub fn hard_light() -> o::HardLight {
    o::HardLight{}
}
/// The "hebrew" value.
pub fn hebrew() -> o::Hebrew {
    o::Hebrew{}
}
/// The "help" value.
pub fn help() -> o::Help {
    o::Help{}
}
/// The "hidden" value.
pub fn hidden() -> o::Hidden {
    o::Hidden{}
}
/// The "high-quality" value.
pub fn high_quality() -> o::HighQuality {
    o::HighQuality{}
}
/// The "hiragana" value.
pub fn hiragana() -> o::Hiragana {
    o::Hiragana{}
}
/// The "hiragana-iroha" value.
pub fn hiragana_iroha() -> o::HiraganaIroha {
    o::HiraganaIroha{}
}
/// The "horizontal-tb" value.
pub fn horizontal_tb() -> o::HorizontalTb {
    o::HorizontalTb{}
}
/// The "hover" value.
pub fn hover() -> o::Hover {
    o::Hover{}
}
/// The "hue" value.
pub fn hue() -> o::Hue {
    o::Hue{}
}
/// The "hz" value.
pub fn hz() -> o::Hz {
    o::Hz{}
}
/// The "identity" value.
pub fn identity() -> o::Identity {
    o::Identity{}
}
/// The "in" value.
pub fn in_(x: i32) -> o::Length {
    unimplemented!()
}
/// The "infinite" value.
pub fn infinite() -> o::Infinite {
    o::Infinite{}
}
/// The "inherit" value.
pub fn inherit() -> o::Inherit {
    o::Inherit{}
}
/// The "initial" value.
pub fn initial() -> o::Initial {
    o::Initial{}
}
/// The "inline" value.
pub fn inline() -> o::Inline {
    o::Inline{}
}
/// The "inline-block" value.
pub fn inline_block() -> o::InlineBlock {
    o::InlineBlock{}
}
/// The "inline-flex" value.
pub fn inline_flex() -> o::InlineFlex {
    o::InlineFlex{}
}
/// The "inline-grid" value.
pub fn inline_grid() -> o::InlineGrid {
    o::InlineGrid{}
}
/// The "inline-table" value.
pub fn inline_table() -> o::InlineTable {
    o::InlineTable{}
}
/// The "inset" value.
pub fn inset() -> o::Inset {
    o::Inset{}
}
/// The "inter-character" value.
pub fn inter_character() -> o::InterCharacter {
    o::InterCharacter{}
}
/// The "interlace" value.
pub fn interlace() -> o::Interlace {
    o::Interlace{}
}
/// The "intersect" value.
pub fn intersect() -> o::Intersect {
    o::Intersect{}
}
/// The "inter-word" value.
pub fn inter_word() -> o::InterWord {
    o::InterWord{}
}
/// The "invert" value.
pub fn invert() -> o::Invert {
    o::Invert{}
}
/// The "isolate" value.
pub fn isolate() -> o::Isolate {
    o::Isolate{}
}
/// The "isolate-override" value.
pub fn isolate_override() -> o::IsolateOverride {
    o::IsolateOverride{}
}
/// The "japanese-formal" value.
pub fn japanese_formal() -> o::JapaneseFormal {
    o::JapaneseFormal{}
}
/// The "japanese-informal" value.
pub fn japanese_informal() -> o::JapaneseInformal {
    o::JapaneseInformal{}
}
/// The "jump-both" value.
pub fn jump_both() -> o::JumpBoth {
    o::JumpBoth{}
}
/// The "jump-end" value.
pub fn jump_end() -> o::JumpEnd {
    o::JumpEnd{}
}
/// The "jump-none" value.
pub fn jump_none() -> o::JumpNone {
    o::JumpNone{}
}
/// The "jump-start" value.
pub fn jump_start() -> o::JumpStart {
    o::JumpStart{}
}
/// The "justify" value.
pub fn justify() -> o::Justify {
    o::Justify{}
}
/// The "justify-all" value.
pub fn justify_all() -> o::JustifyAll {
    o::JustifyAll{}
}
/// The "kannada" value.
pub fn kannada() -> o::Kannada {
    o::Kannada{}
}
/// The "katakana" value.
pub fn katakana() -> o::Katakana {
    o::Katakana{}
}
/// The "katakana-iroha" value.
pub fn katakana_iroha() -> o::KatakanaIroha {
    o::KatakanaIroha{}
}
/// The "keep-all" value.
pub fn keep_all() -> o::KeepAll {
    o::KeepAll{}
}
/// The "khmer" value.
pub fn khmer() -> o::Khmer {
    o::Khmer{}
}
/// The "khz" value.
pub fn khz() -> o::Khz {
    o::Khz{}
}
/// The "korean-hangul-formal" value.
pub fn korean_hangul_formal() -> o::KoreanHangulFormal {
    o::KoreanHangulFormal{}
}
/// The "korean-hanja-formal" value.
pub fn korean_hanja_formal() -> o::KoreanHanjaFormal {
    o::KoreanHanjaFormal{}
}
/// The "korean-hanja-informal" value.
pub fn korean_hanja_informal() -> o::KoreanHanjaInformal {
    o::KoreanHanjaInformal{}
}
/// The "landscape" value.
pub fn landscape() -> o::Landscape {
    o::Landscape{}
}
/// The "lao" value.
pub fn lao() -> o::Lao {
    o::Lao{}
}
/// The "last" value.
pub fn last() -> o::Last {
    o::Last{}
}
// last baseline
/// The "layout" value.
pub fn layout() -> o::Layout {
    o::Layout{}
}
/// The "left" value.
pub fn left() -> o::Left {
    o::Left{}
}
/// The "legacy" value.
pub fn legacy() -> o::Legacy {
    o::Legacy{}
}
/// The "lighten" value.
pub fn lighten() -> o::Lighten {
    o::Lighten{}
}
/// The "linear" value.
pub fn linear() -> o::Linear {
    o::Linear{}
}
/// The "linearrgb" value.
pub fn linearrgb() -> o::Linearrgb {
    o::Linearrgb{}
}
/// The "line-through" value.
pub fn line_through() -> o::LineThrough {
    o::LineThrough{}
}
/// The "list-item" value.
pub fn list_item() -> o::ListItem {
    o::ListItem{}
}
/// The "local" value.
pub fn local() -> o::Local {
    o::Local{}
}
/// The "loose" value.
pub fn loose() -> o::Loose {
    o::Loose{}
}
/// The "lower-alpha" value.
pub fn lower_alpha() -> o::LowerAlpha {
    o::LowerAlpha{}
}
/// The "lower-armenian" value.
pub fn lower_armenian() -> o::LowerArmenian {
    o::LowerArmenian{}
}
/// The "lowercase" value.
pub fn lowercase() -> o::Lowercase {
    o::Lowercase{}
}
/// The "lower-greek" value.
pub fn lower_greek() -> o::LowerGreek {
    o::LowerGreek{}
}
/// The "lower-latin" value.
pub fn lower_latin() -> o::LowerLatin {
    o::LowerLatin{}
}
/// The "lower-roman" value.
pub fn lower_roman() -> o::LowerRoman {
    o::LowerRoman{}
}
/// The "ltr" value.
pub fn ltr() -> o::Ltr {
    o::Ltr{}
}
/// The "luminance" value.
pub fn luminance() -> o::Luminance {
    o::Luminance{}
}
/// The "luminosity" value.
pub fn luminosity() -> o::Luminosity {
    o::Luminosity{}
}
/// The "malayalam" value.
pub fn malayalam() -> o::Malayalam {
    o::Malayalam{}
}
/// The "mandatory" value.
pub fn mandatory() -> o::Mandatory {
    o::Mandatory{}
}
/// The "manual" value.
pub fn manual() -> o::Manual {
    o::Manual{}
}
/// The "margin-box" value.
pub fn margin_box() -> o::MarginBox {
    o::MarginBox{}
}
/// The "match-parent" value.
pub fn match_parent() -> o::MatchParent {
    o::MatchParent{}
}
/// The "match-source" value.
pub fn match_source() -> o::MatchSource {
    o::MatchSource{}
}
/// The "max-content" value.
pub fn max_content() -> o::MaxContent {
    o::MaxContent{}
}
/// The "medium" value.
pub fn medium() -> o::Medium {
    o::Medium{}
}
/// The "min-content" value.
pub fn min_content() -> o::MinContent {
    o::MinContent{}
}
/// The "minmax()" value.
pub fn minmax() -> o::Minmax {
    o::Minmax{}
}
/// The "mixed" value.
pub fn mixed() -> o::Mixed {
    o::Mixed{}
}
/// The "mm" value.
pub fn mm(x: i32) -> o::Length {
    unimplemented!()
}
/// The "mongolian" value.
pub fn mongolian() -> o::Mongolian {
    o::Mongolian{}
}
/// The "move" value.
pub fn move_() -> o::Move {
    o::Move{}
}
/// The "ms" value.
pub fn ms(x: i32) -> o::Time {
    o::Time::Ms(x)
}
/// The "multiply" value.
pub fn multiply() -> o::Multiply {
    o::Multiply{}
}
/// The "myanmar" value.
pub fn myanmar() -> o::Myanmar {
    o::Myanmar{}
}
/// The "ne-resize" value.
pub fn ne_resize() -> o::NeResize {
    o::NeResize{}
}
/// The "nesw-resize" value.
pub fn nesw_resize() -> o::NeswResize {
    o::NeswResize{}
}
/// The "no-clip" value.
pub fn no_clip() -> o::NoClip {
    o::NoClip{}
}
/// The "no-close-quote" value.
pub fn no_close_quote() -> o::NoCloseQuote {
    o::NoCloseQuote{}
}
/// The "no-composite" value.
pub fn no_composite() -> o::NoComposite {
    o::NoComposite{}
}
/// The "no-drop" value.
pub fn no_drop() -> o::NoDrop {
    o::NoDrop{}
}
/// The "none" value.
pub fn none() -> o::None {
    o::None{}
}
// none!!font-variant
/// The "nonzero" value.
pub fn nonzero() -> o::Nonzero {
    o::Nonzero{}
}
/// The "no-open-quote" value.
pub fn no_open_quote() -> o::NoOpenQuote {
    o::NoOpenQuote{}
}
/// The "no-repeat" value.
pub fn no_repeat() -> o::NoRepeat {
    o::NoRepeat{}
}
/// The "normal" value.
pub fn normal() -> o::Normal {
    o::Normal{}
}
// normal!!font-feature-settings
// normal!!font-variant
/// The "not" value.
pub fn not() -> o::Not {
    o::Not{}
}
/// The "not-allowed" value.
pub fn not_allowed() -> o::NotAllowed {
    o::NotAllowed{}
}
/// The "nowrap" value.
pub fn nowrap() -> o::Nowrap {
    o::Nowrap{}
}
/// The "n-resize" value.
pub fn n_resize() -> o::NResize {
    o::NResize{}
}
/// The "ns-resize" value.
pub fn ns_resize() -> o::NsResize {
    o::NsResize{}
}
/// The "numbers" value.
pub fn numbers() -> o::Numbers {
    o::Numbers{}
}
/// The "numeric" value.
pub fn numeric() -> o::Numeric {
    o::Numeric{}
}
/// The "nw-resize" value.
pub fn nw_resize() -> o::NwResize {
    o::NwResize{}
}
/// The "nwse-resize" value.
pub fn nwse_resize() -> o::NwseResize {
    o::NwseResize{}
}
/// The "objectboundingbox" value.
pub fn objectboundingbox() -> o::Objectboundingbox {
    o::Objectboundingbox{}
}
/// The "only" value.
pub fn only() -> o::Only {
    o::Only{}
}
/// The "open" value.
pub fn open() -> o::Open {
    o::Open{}
}
/// The "open-quote" value.
pub fn open_quote() -> o::OpenQuote {
    o::OpenQuote{}
}
/// The "optional-paged" value.
pub fn optional_paged() -> o::OptionalPaged {
    o::OptionalPaged{}
}
/// The "oriya" value.
pub fn oriya() -> o::Oriya {
    o::Oriya{}
}
/// The "outset" value.
pub fn outset() -> o::Outset {
    o::Outset{}
}
/// The "over" value.
pub fn over() -> o::Over {
    o::Over{}
}
/// The "overlay" value.
pub fn overlay() -> o::Overlay {
    o::Overlay{}
}
/// The "overline" value.
pub fn overline() -> o::Overline {
    o::Overline{}
}
/// The "p3" value.
pub fn p3() -> o::P3 {
    o::P3{}
}
/// The "padding-box" value.
pub fn padding_box() -> o::PaddingBox {
    o::PaddingBox{}
}
/// The "page" value.
pub fn page() -> o::Page {
    o::Page{}
}
/// The "paged" value.
pub fn paged() -> o::Paged {
    o::Paged{}
}
/// The "paint" value.
pub fn paint() -> o::Paint {
    o::Paint{}
}
/// The "paused" value.
pub fn paused() -> o::Paused {
    o::Paused{}
}
/// The "pc" value.
pub fn pc() -> o::Pc {
    o::Pc{}
}
/// The "persian" value.
pub fn persian() -> o::Persian {
    o::Persian{}
}
/// The "pixelated" value.
pub fn pixelated() -> o::Pixelated {
    o::Pixelated{}
}
// pixel unit
/// The "plaintext" value.
pub fn plaintext() -> o::Plaintext {
    o::Plaintext{}
}
/// The "pointer" value.
pub fn pointer() -> o::Pointer {
    o::Pointer{}
}
/// The "portrait" value.
pub fn portrait() -> o::Portrait {
    o::Portrait{}
}
/// The "pre" value.
pub fn pre() -> o::Pre {
    o::Pre{}
}
/// The "pre-line" value.
pub fn pre_line() -> o::PreLine {
    o::PreLine{}
}
/// The "pre-wrap" value.
pub fn pre_wrap() -> o::PreWrap {
    o::PreWrap{}
}
/// The "print" value.
pub fn print() -> o::Print {
    o::Print{}
}
/// The "progress" value.
pub fn progress() -> o::Progress {
    o::Progress{}
}
/// The "progressive" value.
pub fn progressive() -> o::Progressive {
    o::Progressive{}
}
/// The "projection" value.
pub fn projection() -> o::Projection {
    o::Projection{}
}
/// The "proximity" value.
pub fn proximity() -> o::Proximity {
    o::Proximity{}
}
/// The "pt" value.
pub fn pt(x: i32) -> o::Length {
    unimplemented!()
}
/// The "px" value.
pub fn px(x: impl o::Number) -> o::Length {
    o::Length::Px {
        v: x.normalize(),
    }
}
/// The "q" value.
pub fn q(x: i32) -> o::Length {
    unimplemented!()
}
/// The "rad" value.
pub fn rad() -> o::Angle {
    o::Angle::Rad
}
/// The "rec2020" value.
pub fn rec2020() -> o::Rec2020 {
    o::Rec2020{}
}
/// The "recto" value.
pub fn recto() -> o::Recto {
    o::Recto{}
}
/// The "region" value.
pub fn region() -> o::Region {
    o::Region{}
}
/// The "rem" value.
pub fn rem(x: i32) -> o::Length {
    unimplemented!()
}
/// The "repeat" value.
pub fn repeat() -> o::Repeat {
    o::Repeat{}
}
/// The "repeat-x" value.
pub fn repeat_x() -> o::RepeatX {
    o::RepeatX{}
}
/// The "repeat-y" value.
pub fn repeat_y() -> o::RepeatY {
    o::RepeatY{}
}
/// The "reverse" value.
pub fn reverse() -> o::Reverse {
    o::Reverse{}
}
/// The "revert" value.
pub fn revert() -> o::Revert {
    o::Revert{}
}
/// The "ridge" value.
pub fn ridge() -> o::Ridge {
    o::Ridge{}
}
/// The "right" value.
pub fn right() -> o::Right {
    o::Right{}
}
/// The "rotate()" value.
pub fn rotate() -> o::Rotate {
    o::Rotate{}
}
/// The "round" value.
pub fn round() -> o::Round {
    o::Round{}
}
/// The "row" value.
pub fn row() -> o::Row {
    o::Row{}
}
/// The "row-resize" value.
pub fn row_resize() -> o::RowResize {
    o::RowResize{}
}
/// The "row-reverse" value.
pub fn row_reverse() -> o::RowReverse {
    o::RowReverse{}
}
/// The "rtl" value.
pub fn rtl() -> o::Rtl {
    o::Rtl{}
}
/// The "running" value.
pub fn running() -> o::Running {
    o::Running{}
}
/// The "s" value.
pub fn s(value: i32) -> o::Time {
    o::Time::S(value)
}
/// The "safe" value.
pub fn safe() -> o::Safe {
    o::Safe{}
}
/// The "saturation" value.
pub fn saturation() -> o::Saturation {
    o::Saturation{}
}
/// The "scale()" value.
pub fn scale() -> o::Scale {
    o::Scale{}
}
/// The "scale-down" value.
pub fn scale_down() -> o::ScaleDown {
    o::ScaleDown{}
}
/// The "scalex()" value.
pub fn scalex() -> o::Scalex {
    o::Scalex{}
}
/// The "scaley()" value.
pub fn scaley() -> o::Scaley {
    o::Scaley{}
}
/// The "screen" value.
pub fn screen() -> o::Screen {
    o::Screen{}
}
/// The "scroll" value.
pub fn scroll() -> o::Scroll {
    o::Scroll{}
}
/// The "scroll-position" value.
pub fn scroll_position() -> o::ScrollPosition {
    o::ScrollPosition{}
}
/// The "self-end" value.
pub fn self_end() -> o::SelfEnd {
    o::SelfEnd{}
}
/// The "self-start" value.
pub fn self_start() -> o::SelfStart {
    o::SelfStart{}
}
/// The "se-resize" value.
pub fn se_resize() -> o::SeResize {
    o::SeResize{}
}
/// The "sesame" value.
pub fn sesame() -> o::Sesame {
    o::Sesame{}
}
/// The "sideways" value.
pub fn sideways() -> o::Sideways {
    o::Sideways{}
}
/// The "sideways-right" value.
pub fn sideways_right() -> o::SidewaysRight {
    o::SidewaysRight{}
}
/// The "simp-chinese-formal" value.
pub fn simp_chinese_formal() -> o::SimpChineseFormal {
    o::SimpChineseFormal{}
}
/// The "simp-chinese-informal" value.
pub fn simp_chinese_informal() -> o::SimpChineseInformal {
    o::SimpChineseInformal{}
}
/// The "size" value.
pub fn size() -> o::Size {
    o::Size{}
}
/// The "skew()" value.
pub fn skew() -> o::Skew {
    o::Skew{}
}
/// The "skewx()" value.
pub fn skewx() -> o::Skewx {
    o::Skewx{}
}
/// The "skewy()" value.
pub fn skewy() -> o::Skewy {
    o::Skewy{}
}
/// The "slice" value.
pub fn slice() -> o::Slice {
    o::Slice{}
}
/// The "slow" value.
pub fn slow() -> o::Slow {
    o::Slow{}
}
/// The "smooth" value.
pub fn smooth() -> o::Smooth {
    o::Smooth{}
}
/// The "soft-light" value.
pub fn soft_light() -> o::SoftLight {
    o::SoftLight{}
}
/// The "solid" value.
pub fn solid() -> o::Solid {
    o::Solid{}
}
/// The "sourcealpha" value.
pub fn sourcealpha() -> o::Sourcealpha {
    o::Sourcealpha{}
}
/// The "sourcegraphic" value.
pub fn sourcegraphic() -> o::Sourcegraphic {
    o::Sourcegraphic{}
}
/// The "space" value.
pub fn space() -> o::Space {
    o::Space{}
}
/// The "space-around" value.
pub fn space_around() -> o::SpaceAround {
    o::SpaceAround{}
}
/// The "space-between" value.
pub fn space_between() -> o::SpaceBetween {
    o::SpaceBetween{}
}
/// The "space-evenly" value.
pub fn space_evenly() -> o::SpaceEvenly {
    o::SpaceEvenly{}
}
// span && [ <integer> || <custom-ident> ]
/// The "speech" value.
pub fn speech() -> o::Speech {
    o::Speech{}
}
/// The "spell-out" value.
pub fn spell_out() -> o::SpellOut {
    o::SpellOut{}
}
/// The "square" value.
pub fn square() -> o::Square {
    o::Square{}
}
/// The "s-resize" value.
pub fn s_resize() -> o::SResize {
    o::SResize{}
}
/// The "srgb" value.
pub fn srgb() -> o::Srgb {
    o::Srgb{}
}
/// The "start" value.
pub fn start() -> o::Start {
    o::Start{}
}
/// The "step-end" value.
pub fn step_end() -> o::StepEnd {
    o::StepEnd{}
}
/// The "step-start" value.
pub fn step_start() -> o::StepStart {
    o::StepStart{}
}
/// The "stretch" value.
pub fn stretch() -> o::Stretch {
    o::Stretch{}
}
/// The "strict" value.
pub fn strict() -> o::Strict {
    o::Strict{}
}
/// The "stroke-box" value.
pub fn stroke_box() -> o::StrokeBox {
    o::StrokeBox{}
}
/// The "strokepaint" value.
pub fn strokepaint() -> o::Strokepaint {
    o::Strokepaint{}
}
/// The "style" value.
pub fn style() -> o::Style {
    o::Style{}
}
/// The "subtract" value.
pub fn subtract() -> o::Subtract {
    o::Subtract{}
}
/// The "sw-resize" value.
pub fn sw_resize() -> o::SwResize {
    o::SwResize{}
}
/// The "symbolic" value.
pub fn symbolic() -> o::Symbolic {
    o::Symbolic{}
}
/// The "table" value.
pub fn table() -> o::Table {
    o::Table{}
}
/// The "table-caption" value.
pub fn table_caption() -> o::TableCaption {
    o::TableCaption{}
}
/// The "table-cell" value.
pub fn table_cell() -> o::TableCell {
    o::TableCell{}
}
/// The "table-column" value.
pub fn table_column() -> o::TableColumn {
    o::TableColumn{}
}
/// The "table-column-group" value.
pub fn table_column_group() -> o::TableColumnGroup {
    o::TableColumnGroup{}
}
/// The "table-footer-group" value.
pub fn table_footer_group() -> o::TableFooterGroup {
    o::TableFooterGroup{}
}
/// The "table-header-group" value.
pub fn table_header_group() -> o::TableHeaderGroup {
    o::TableHeaderGroup{}
}
/// The "table-row" value.
pub fn table_row() -> o::TableRow {
    o::TableRow{}
}
/// The "table-row-group" value.
pub fn table_row_group() -> o::TableRowGroup {
    o::TableRowGroup{}
}
/// The "tamil" value.
pub fn tamil() -> o::Tamil {
    o::Tamil{}
}
/// The "telugu" value.
pub fn telugu() -> o::Telugu {
    o::Telugu{}
}
/// The "text" value.
pub fn text() -> o::Text {
    o::Text{}
}
/// The "thai" value.
pub fn thai() -> o::Thai {
    o::Thai{}
}
/// The "thick" value.
pub fn thick() -> o::Thick {
    o::Thick{}
}
/// The "thin" value.
pub fn thin() -> o::Thin {
    o::Thin{}
}
/// The "tibetan" value.
pub fn tibetan() -> o::Tibetan {
    o::Tibetan{}
}
/// The "top" value.
pub fn top() -> o::Top {
    o::Top{}
}
/// The "trad-chinese-formal" value.
pub fn trad_chinese_formal() -> o::TradChineseFormal {
    o::TradChineseFormal{}
}
/// The "trad-chinese-informal" value.
pub fn trad_chinese_informal() -> o::TradChineseInformal {
    o::TradChineseInformal{}
}
/// The "translate()" value.
pub fn translate() -> o::Translate {
    o::Translate{}
}
/// The "translatex()" value.
pub fn translatex() -> o::Translatex {
    o::Translatex{}
}
/// The "translatey()" value.
pub fn translatey() -> o::Translatey {
    o::Translatey{}
}
/// The "triangle" value.
pub fn triangle() -> o::Triangle {
    o::Triangle{}
}
/// The "tty" value.
pub fn tty() -> o::Tty {
    o::Tty{}
}
/// The "turn" value.
pub fn turn() -> o::Angle {
    o::Angle::Turn
}
/// The "tv" value.
pub fn tv() -> o::Tv {
    o::Tv{}
}
/// The "under" value.
pub fn under() -> o::Under {
    o::Under{}
}
/// The "underline" value.
pub fn underline() -> o::Underline {
    o::Underline{}
}
/// The "unsafe" value.
pub fn unsafe_() -> o::Unsafe {
    o::Unsafe{}
}
/// The "unset" value.
pub fn unset() -> o::Unset {
    o::Unset{}
}
/// The "upper-alpha" value.
pub fn upper_alpha() -> o::UpperAlpha {
    o::UpperAlpha{}
}
/// The "upper-armenian" value.
pub fn upper_armenian() -> o::UpperArmenian {
    o::UpperArmenian{}
}
/// The "uppercase" value.
pub fn uppercase() -> o::Uppercase {
    o::Uppercase{}
}
/// The "upper-latin" value.
pub fn upper_latin() -> o::UpperLatin {
    o::UpperLatin{}
}
/// The "upper-roman" value.
pub fn upper_roman() -> o::UpperRoman {
    o::UpperRoman{}
}
/// The "upright" value.
pub fn upright() -> o::Upright {
    o::Upright{}
}
/// The "userspaceonuse" value.
pub fn userspaceonuse() -> o::Userspaceonuse {
    o::Userspaceonuse{}
}
pub fn url(value: String) -> o::Url {
    o::Url(value)
}
/// The "verso" value.
pub fn verso() -> o::Verso {
    o::Verso{}
}
/// The "vertical-lr" value.
pub fn vertical_lr() -> o::VerticalLr {
    o::VerticalLr{}
}
/// The "vertical-rl" value.
pub fn vertical_rl() -> o::VerticalRl {
    o::VerticalRl{}
}
/// The "vertical-text" value.
pub fn vertical_text() -> o::VerticalText {
    o::VerticalText{}
}
/// The "vh" value.
pub fn vh() -> o::Vh {
    o::Vh{}
}
/// The "view-box" value.
pub fn view_box() -> o::ViewBox {
    o::ViewBox{}
}
/// The "vmax" value.
pub fn vmax() -> o::Vmax {
    o::Vmax{}
}
/// The "vmin" value.
pub fn vmin() -> o::Vmin {
    o::Vmin{}
}
/// The "vw" value.
pub fn vw() -> o::Vw {
    o::Vw{}
}
/// The "wait" value.
pub fn wait() -> o::Wait {
    o::Wait{}
}
/// The "words" value.
pub fn words() -> o::Words {
    o::Words{}
}
/// The "wrap" value.
pub fn wrap() -> o::Wrap {
    o::Wrap{}
}
/// The "wrap-reverse" value.
pub fn wrap_reverse() -> o::WrapReverse {
    o::WrapReverse{}
}
/// The "w-resize" value.
pub fn w_resize() -> o::WResize {
    o::WResize{}
}
/// The "x" value.
pub fn x() -> o::X {
    o::X{}
}
/// The "xor" value.
pub fn xor() -> o::Xor {
    o::Xor{}
}
/// The "y" value.
pub fn y() -> o::Y {
    o::Y{}
}
/// The "zoom-in" value.
pub fn zoom_in() -> o::ZoomIn {
    o::ZoomIn{}
}
/// The "zoom-out" value.
pub fn zoom_out() -> o::ZoomOut {
    o::ZoomOut{}
}
