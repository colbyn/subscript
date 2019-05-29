pub mod values;

use crate::data::{self, Style, Rule, Property, Value};
use crate::api::values::*;

pub trait AlignContent {
	fn value(&self) -> data::Value;
}
pub fn align_content(value: impl AlignContent) -> Style {
	Style::Typed(Rule{property: Property::AlignContent, value: value.value()})
}

pub trait AlignItems {
	fn value(&self) -> data::Value;
}
pub fn align_items(value: impl AlignItems) -> Style {
	Style::Typed(Rule{property: Property::AlignItems, value: value.value()})
}

pub trait AlignSelf {
	fn value(&self) -> data::Value;
}
pub fn align_self(value: impl AlignSelf) -> Style {
	Style::Typed(Rule{property: Property::AlignSelf, value: value.value()})
}

pub trait All {
	fn value(&self) -> data::Value;
}
pub fn all(value: impl All) -> Style {
	Style::Typed(Rule{property: Property::All, value: value.value()})
}

pub trait Animation {
	fn value(&self) -> data::Value;
}
pub fn animation(value: impl Animation) -> Style {
	Style::Typed(Rule{property: Property::Animation, value: value.value()})
}

pub trait AnimationDelay {
	fn value(&self) -> data::Value;
}
pub fn animation_delay(value: impl AnimationDelay) -> Style {
	Style::Typed(Rule{property: Property::AnimationDelay, value: value.value()})
}

pub trait AnimationDirection {
	fn value(&self) -> data::Value;
}
pub fn animation_direction(value: impl AnimationDirection) -> Style {
	Style::Typed(Rule{property: Property::AnimationDirection, value: value.value()})
}

pub trait AnimationDuration {
	fn value(&self) -> data::Value;
}
pub fn animation_duration(value: impl AnimationDuration) -> Style {
	Style::Typed(Rule{property: Property::AnimationDuration, value: value.value()})
}

pub trait AnimationFillMode {
	fn value(&self) -> data::Value;
}
pub fn animation_fill_mode(value: impl AnimationFillMode) -> Style {
	Style::Typed(Rule{property: Property::AnimationFillMode, value: value.value()})
}

pub trait AnimationIterationCount {
	fn value(&self) -> data::Value;
}
pub fn animation_iteration_count(value: impl AnimationIterationCount) -> Style {
	Style::Typed(Rule{property: Property::AnimationIterationCount, value: value.value()})
}

pub trait AnimationName {
	fn value(&self) -> data::Value;
}
pub fn animation_name(value: impl AnimationName) -> Style {
	Style::Typed(Rule{property: Property::AnimationName, value: value.value()})
}

pub trait AnimationPlayState {
	fn value(&self) -> data::Value;
}
pub fn animation_play_state(value: impl AnimationPlayState) -> Style {
	Style::Typed(Rule{property: Property::AnimationPlayState, value: value.value()})
}

pub trait AnimationTimingFunction {
	fn value(&self) -> data::Value;
}
pub fn animation_timing_function(value: impl AnimationTimingFunction) -> Style {
	Style::Typed(Rule{property: Property::AnimationTimingFunction, value: value.value()})
}

pub trait Azimuth {
	fn value(&self) -> data::Value;
}
pub fn azimuth(value: impl Azimuth) -> Style {
	Style::Typed(Rule{property: Property::Azimuth, value: value.value()})
}

pub trait Background {
	fn value(&self) -> data::Value;
}
pub fn background(value: impl Background) -> Style {
	Style::Typed(Rule{property: Property::Background, value: value.value()})
}

pub trait BackgroundAttachment {
	fn value(&self) -> data::Value;
}
pub fn background_attachment(value: impl BackgroundAttachment) -> Style {
	Style::Typed(Rule{property: Property::BackgroundAttachment, value: value.value()})
}

pub trait BackgroundBlendMode {
	fn value(&self) -> data::Value;
}
pub fn background_blend_mode(value: impl BackgroundBlendMode) -> Style {
	Style::Typed(Rule{property: Property::BackgroundBlendMode, value: value.value()})
}

pub trait BackgroundClip {
	fn value(&self) -> data::Value;
}
pub fn background_clip(value: impl BackgroundClip) -> Style {
	Style::Typed(Rule{property: Property::BackgroundClip, value: value.value()})
}

pub trait BackgroundColor {
	fn value(&self) -> data::Value;
}
pub fn background_color(value: impl BackgroundColor) -> Style {
	Style::Typed(Rule{property: Property::BackgroundColor, value: value.value()})
}

pub trait BackgroundImage {
	fn value(&self) -> data::Value;
}
pub fn background_image(value: impl BackgroundImage) -> Style {
	Style::Typed(Rule{property: Property::BackgroundImage, value: value.value()})
}

pub trait BackgroundOrigin {
	fn value(&self) -> data::Value;
}
pub fn background_origin(value: impl BackgroundOrigin) -> Style {
	Style::Typed(Rule{property: Property::BackgroundOrigin, value: value.value()})
}

pub trait BackgroundPosition {
	fn value(&self) -> data::Value;
}
pub fn background_position(value: impl BackgroundPosition) -> Style {
	Style::Typed(Rule{property: Property::BackgroundPosition, value: value.value()})
}

pub trait BackgroundRepeat {
	fn value(&self) -> data::Value;
}
pub fn background_repeat(value: impl BackgroundRepeat) -> Style {
	Style::Typed(Rule{property: Property::BackgroundRepeat, value: value.value()})
}

pub trait BackgroundSize {
	fn value(&self) -> data::Value;
}
pub fn background_size(value: impl BackgroundSize) -> Style {
	Style::Typed(Rule{property: Property::BackgroundSize, value: value.value()})
}

pub trait Border {
	fn value(&self) -> data::Value;
}
pub fn border(value: impl Border) -> Style {
	Style::Typed(Rule{property: Property::Border, value: value.value()})
}

pub trait BorderBottom {
	fn value(&self) -> data::Value;
}
pub fn border_bottom(value: impl BorderBottom) -> Style {
	Style::Typed(Rule{property: Property::BorderBottom, value: value.value()})
}

pub trait BorderBottomColor {
	fn value(&self) -> data::Value;
}
pub fn border_bottom_color(value: impl BorderBottomColor) -> Style {
	Style::Typed(Rule{property: Property::BorderBottomColor, value: value.value()})
}

pub trait BorderBottomLeftRadius {
	fn value(&self) -> data::Value;
}
pub fn border_bottom_left_radius(value: impl BorderBottomLeftRadius) -> Style {
	Style::Typed(Rule{property: Property::BorderBottomLeftRadius, value: value.value()})
}

pub trait BorderBottomRightRadius {
	fn value(&self) -> data::Value;
}
pub fn border_bottom_right_radius(value: impl BorderBottomRightRadius) -> Style {
	Style::Typed(Rule{property: Property::BorderBottomRightRadius, value: value.value()})
}

pub trait BorderBottomStyle {
	fn value(&self) -> data::Value;
}
pub fn border_bottom_style(value: impl BorderBottomStyle) -> Style {
	Style::Typed(Rule{property: Property::BorderBottomStyle, value: value.value()})
}

pub trait BorderBottomWidth {
	fn value(&self) -> data::Value;
}
pub fn border_bottom_width(value: impl BorderBottomWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderBottomWidth, value: value.value()})
}

pub trait BorderCollapse {
	fn value(&self) -> data::Value;
}
pub fn border_collapse(value: impl BorderCollapse) -> Style {
	Style::Typed(Rule{property: Property::BorderCollapse, value: value.value()})
}

pub trait BorderColor {
	fn value(&self) -> data::Value;
}
pub fn border_color(value: impl BorderColor) -> Style {
	Style::Typed(Rule{property: Property::BorderColor, value: value.value()})
}

pub trait BorderImage {
	fn value(&self) -> data::Value;
}
pub fn border_image(value: impl BorderImage) -> Style {
	Style::Typed(Rule{property: Property::BorderImage, value: value.value()})
}

pub trait BorderImageOutset {
	fn value(&self) -> data::Value;
}
pub fn border_image_outset(value: impl BorderImageOutset) -> Style {
	Style::Typed(Rule{property: Property::BorderImageOutset, value: value.value()})
}

pub trait BorderImageRepeat {
	fn value(&self) -> data::Value;
}
pub fn border_image_repeat(value: impl BorderImageRepeat) -> Style {
	Style::Typed(Rule{property: Property::BorderImageRepeat, value: value.value()})
}

pub trait BorderImageSlice {
	fn value(&self) -> data::Value;
}
pub fn border_image_slice(value: impl BorderImageSlice) -> Style {
	Style::Typed(Rule{property: Property::BorderImageSlice, value: value.value()})
}

pub trait BorderImageSource {
	fn value(&self) -> data::Value;
}
pub fn border_image_source(value: impl BorderImageSource) -> Style {
	Style::Typed(Rule{property: Property::BorderImageSource, value: value.value()})
}

pub trait BorderImageWidth {
	fn value(&self) -> data::Value;
}
pub fn border_image_width(value: impl BorderImageWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderImageWidth, value: value.value()})
}

pub trait BorderLeft {
	fn value(&self) -> data::Value;
}
pub fn border_left(value: impl BorderLeft) -> Style {
	Style::Typed(Rule{property: Property::BorderLeft, value: value.value()})
}

pub trait BorderLeftColor {
	fn value(&self) -> data::Value;
}
pub fn border_left_color(value: impl BorderLeftColor) -> Style {
	Style::Typed(Rule{property: Property::BorderLeftColor, value: value.value()})
}

pub trait BorderLeftStyle {
	fn value(&self) -> data::Value;
}
pub fn border_left_style(value: impl BorderLeftStyle) -> Style {
	Style::Typed(Rule{property: Property::BorderLeftStyle, value: value.value()})
}

pub trait BorderLeftWidth {
	fn value(&self) -> data::Value;
}
pub fn border_left_width(value: impl BorderLeftWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderLeftWidth, value: value.value()})
}

pub trait BorderRadius {
	fn value(&self) -> data::Value;
}
pub fn border_radius(value: impl BorderRadius) -> Style {
	Style::Typed(Rule{property: Property::BorderRadius, value: value.value()})
}

pub trait BorderRight {
	fn value(&self) -> data::Value;
}
pub fn border_right(value: impl BorderRight) -> Style {
	Style::Typed(Rule{property: Property::BorderRight, value: value.value()})
}

pub trait BorderRightColor {
	fn value(&self) -> data::Value;
}
pub fn border_right_color(value: impl BorderRightColor) -> Style {
	Style::Typed(Rule{property: Property::BorderRightColor, value: value.value()})
}

pub trait BorderRightStyle {
	fn value(&self) -> data::Value;
}
pub fn border_right_style(value: impl BorderRightStyle) -> Style {
	Style::Typed(Rule{property: Property::BorderRightStyle, value: value.value()})
}

pub trait BorderRightWidth {
	fn value(&self) -> data::Value;
}
pub fn border_right_width(value: impl BorderRightWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderRightWidth, value: value.value()})
}

pub trait BorderSpacing {
	fn value(&self) -> data::Value;
}
pub fn border_spacing(value: impl BorderSpacing) -> Style {
	Style::Typed(Rule{property: Property::BorderSpacing, value: value.value()})
}

pub trait BorderStyle {
	fn value(&self) -> data::Value;
}
pub fn border_style(value: impl BorderStyle) -> Style {
	Style::Typed(Rule{property: Property::BorderStyle, value: value.value()})
}

pub trait BorderTop {
	fn value(&self) -> data::Value;
}
pub fn border_top(value: impl BorderTop) -> Style {
	Style::Typed(Rule{property: Property::BorderTop, value: value.value()})
}

pub trait BorderTopColor {
	fn value(&self) -> data::Value;
}
pub fn border_top_color(value: impl BorderTopColor) -> Style {
	Style::Typed(Rule{property: Property::BorderTopColor, value: value.value()})
}

pub trait BorderTopLeftRadius {
	fn value(&self) -> data::Value;
}
pub fn border_top_left_radius(value: impl BorderTopLeftRadius) -> Style {
	Style::Typed(Rule{property: Property::BorderTopLeftRadius, value: value.value()})
}

pub trait BorderTopRightRadius {
	fn value(&self) -> data::Value;
}
pub fn border_top_right_radius(value: impl BorderTopRightRadius) -> Style {
	Style::Typed(Rule{property: Property::BorderTopRightRadius, value: value.value()})
}

pub trait BorderTopStyle {
	fn value(&self) -> data::Value;
}
pub fn border_top_style(value: impl BorderTopStyle) -> Style {
	Style::Typed(Rule{property: Property::BorderTopStyle, value: value.value()})
}

pub trait BorderTopWidth {
	fn value(&self) -> data::Value;
}
pub fn border_top_width(value: impl BorderTopWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderTopWidth, value: value.value()})
}

pub trait BorderWidth {
	fn value(&self) -> data::Value;
}
pub fn border_width(value: impl BorderWidth) -> Style {
	Style::Typed(Rule{property: Property::BorderWidth, value: value.value()})
}

pub trait Bottom {
	fn value(&self) -> data::Value;
}
pub fn bottom(value: impl Bottom) -> Style {
	Style::Typed(Rule{property: Property::Bottom, value: value.value()})
}

pub trait BoxDecorationBreak {
	fn value(&self) -> data::Value;
}
pub fn box_decoration_break(value: impl BoxDecorationBreak) -> Style {
	Style::Typed(Rule{property: Property::BoxDecorationBreak, value: value.value()})
}

pub trait BoxShadow {
	fn value(&self) -> data::Value;
}
pub fn box_shadow(value: impl BoxShadow) -> Style {
	Style::Typed(Rule{property: Property::BoxShadow, value: value.value()})
}

pub trait BoxSizing {
	fn value(&self) -> data::Value;
}
pub fn box_sizing(value: impl BoxSizing) -> Style {
	Style::Typed(Rule{property: Property::BoxSizing, value: value.value()})
}

pub trait BreakAfter {
	fn value(&self) -> data::Value;
}
pub fn break_after(value: impl BreakAfter) -> Style {
	Style::Typed(Rule{property: Property::BreakAfter, value: value.value()})
}

pub trait BreakBefore {
	fn value(&self) -> data::Value;
}
pub fn break_before(value: impl BreakBefore) -> Style {
	Style::Typed(Rule{property: Property::BreakBefore, value: value.value()})
}

pub trait BreakInside {
	fn value(&self) -> data::Value;
}
pub fn break_inside(value: impl BreakInside) -> Style {
	Style::Typed(Rule{property: Property::BreakInside, value: value.value()})
}

pub trait CaptionSide {
	fn value(&self) -> data::Value;
}
pub fn caption_side(value: impl CaptionSide) -> Style {
	Style::Typed(Rule{property: Property::CaptionSide, value: value.value()})
}

pub trait CaretColor {
	fn value(&self) -> data::Value;
}
pub fn caret_color(value: impl CaretColor) -> Style {
	Style::Typed(Rule{property: Property::CaretColor, value: value.value()})
}

pub trait Clear {
	fn value(&self) -> data::Value;
}
pub fn clear(value: impl Clear) -> Style {
	Style::Typed(Rule{property: Property::Clear, value: value.value()})
}

pub trait Clip {
	fn value(&self) -> data::Value;
}
pub fn clip(value: impl Clip) -> Style {
	Style::Typed(Rule{property: Property::Clip, value: value.value()})
}

pub trait ClipPath {
	fn value(&self) -> data::Value;
}
pub fn clip_path(value: impl ClipPath) -> Style {
	Style::Typed(Rule{property: Property::ClipPath, value: value.value()})
}

pub trait ClipRule {
	fn value(&self) -> data::Value;
}
pub fn clip_rule(value: impl ClipRule) -> Style {
	Style::Typed(Rule{property: Property::ClipRule, value: value.value()})
}

pub trait Color {
	fn value(&self) -> data::Value;
}
pub fn color(value: impl Color) -> Style {
	Style::Typed(Rule{property: Property::Color, value: value.value()})
}

pub trait ColorInterpolationFilters {
	fn value(&self) -> data::Value;
}
pub fn color_interpolation_filters(value: impl ColorInterpolationFilters) -> Style {
	Style::Typed(Rule{property: Property::ColorInterpolationFilters, value: value.value()})
}

pub trait ColumnCount {
	fn value(&self) -> data::Value;
}
pub fn column_count(value: impl ColumnCount) -> Style {
	Style::Typed(Rule{property: Property::ColumnCount, value: value.value()})
}

pub trait ColumnFill {
	fn value(&self) -> data::Value;
}
pub fn column_fill(value: impl ColumnFill) -> Style {
	Style::Typed(Rule{property: Property::ColumnFill, value: value.value()})
}

pub trait ColumnGap {
	fn value(&self) -> data::Value;
}
pub fn column_gap(value: impl ColumnGap) -> Style {
	Style::Typed(Rule{property: Property::ColumnGap, value: value.value()})
}

pub trait ColumnRule {
	fn value(&self) -> data::Value;
}
pub fn column_rule(value: impl ColumnRule) -> Style {
	Style::Typed(Rule{property: Property::ColumnRule, value: value.value()})
}

pub trait ColumnRuleColor {
	fn value(&self) -> data::Value;
}
pub fn column_rule_color(value: impl ColumnRuleColor) -> Style {
	Style::Typed(Rule{property: Property::ColumnRuleColor, value: value.value()})
}

pub trait ColumnRuleStyle {
	fn value(&self) -> data::Value;
}
pub fn column_rule_style(value: impl ColumnRuleStyle) -> Style {
	Style::Typed(Rule{property: Property::ColumnRuleStyle, value: value.value()})
}

pub trait ColumnRuleWidth {
	fn value(&self) -> data::Value;
}
pub fn column_rule_width(value: impl ColumnRuleWidth) -> Style {
	Style::Typed(Rule{property: Property::ColumnRuleWidth, value: value.value()})
}

pub trait Columns {
	fn value(&self) -> data::Value;
}
pub fn columns(value: impl Columns) -> Style {
	Style::Typed(Rule{property: Property::Columns, value: value.value()})
}

pub trait ColumnSpan {
	fn value(&self) -> data::Value;
}
pub fn column_span(value: impl ColumnSpan) -> Style {
	Style::Typed(Rule{property: Property::ColumnSpan, value: value.value()})
}

pub trait ColumnWidth {
	fn value(&self) -> data::Value;
}
pub fn column_width(value: impl ColumnWidth) -> Style {
	Style::Typed(Rule{property: Property::ColumnWidth, value: value.value()})
}

pub trait Contain {
	fn value(&self) -> data::Value;
}
pub fn contain(value: impl Contain) -> Style {
	Style::Typed(Rule{property: Property::Contain, value: value.value()})
}

pub trait Content {
	fn value(&self) -> data::Value;
}
pub fn content(value: impl Content) -> Style {
	Style::Typed(Rule{property: Property::Content, value: value.value()})
}

pub trait CounterIncrement {
	fn value(&self) -> data::Value;
}
pub fn counter_increment(value: impl CounterIncrement) -> Style {
	Style::Typed(Rule{property: Property::CounterIncrement, value: value.value()})
}

pub trait CounterReset {
	fn value(&self) -> data::Value;
}
pub fn counter_reset(value: impl CounterReset) -> Style {
	Style::Typed(Rule{property: Property::CounterReset, value: value.value()})
}

pub trait Cue {
	fn value(&self) -> data::Value;
}
pub fn cue(value: impl Cue) -> Style {
	Style::Typed(Rule{property: Property::Cue, value: value.value()})
}

pub trait CueAfter {
	fn value(&self) -> data::Value;
}
pub fn cue_after(value: impl CueAfter) -> Style {
	Style::Typed(Rule{property: Property::CueAfter, value: value.value()})
}

pub trait CueBefore {
	fn value(&self) -> data::Value;
}
pub fn cue_before(value: impl CueBefore) -> Style {
	Style::Typed(Rule{property: Property::CueBefore, value: value.value()})
}

pub trait Cursor {
	fn value(&self) -> data::Value;
}
pub fn cursor(value: impl Cursor) -> Style {
	Style::Typed(Rule{property: Property::Cursor, value: value.value()})
}

pub trait Direction {
	fn value(&self) -> data::Value;
}
pub fn direction(value: impl Direction) -> Style {
	Style::Typed(Rule{property: Property::Direction, value: value.value()})
}

pub trait Display
{
	fn value(&self) -> data::Value;
}
impl Display for Block {fn value(&self) -> Value {Value::Block}}
impl Display for Inline {fn value(&self) -> Value {Value::Inline}}
impl Display for RunIn {fn value(&self) -> Value {Value::RunIn}}
impl Display for Flow {fn value(&self) -> Value {Value::Flow}}
impl Display for FlowRoot {fn value(&self) -> Value {Value::FlowRoot}}
impl Display for Table {fn value(&self) -> Value {Value::Table}}
impl Display for Flex {fn value(&self) -> Value {Value::Flex}}
impl Display for Grid {fn value(&self) -> Value {Value::Grid}}
impl Display for Contents {fn value(&self) -> Value {Value::Contents}}
impl Display for None {fn value(&self) -> Value {Value::None}}
pub fn display(value: impl Display) -> Style {
	Style::Typed(Rule{property: Property::Display, value: value.value()})
}

pub trait Elevation {
	fn value(&self) -> data::Value;
}
pub fn elevation(value: impl Elevation) -> Style {
	Style::Typed(Rule{property: Property::Elevation, value: value.value()})
}

pub trait EmptyCells {
	fn value(&self) -> data::Value;
}
pub fn empty_cells(value: impl EmptyCells) -> Style {
	Style::Typed(Rule{property: Property::EmptyCells, value: value.value()})
}

pub trait Filter {
	fn value(&self) -> data::Value;
}
pub fn filter(value: impl Filter) -> Style {
	Style::Typed(Rule{property: Property::Filter, value: value.value()})
}

// pub trait Flex {
// 	fn value(&self) -> data::Value;
// }
// pub fn flex(value: impl Flex) -> Style {
// 	unimplemented!()
// }

pub trait FlexBasis {
	fn value(&self) -> data::Value;
}
pub fn flex_basis(value: impl FlexBasis) -> Style {
	Style::Typed(Rule{property: Property::FlexBasis, value: value.value()})
}

pub trait FlexDirection {
	fn value(&self) -> data::Value;
}
pub fn flex_direction(value: impl FlexDirection) -> Style {
	Style::Typed(Rule{property: Property::FlexDirection, value: value.value()})
}

pub trait FlexFlow {
	fn value(&self) -> data::Value;
}
pub fn flex_flow(value: impl FlexFlow) -> Style {
	Style::Typed(Rule{property: Property::FlexFlow, value: value.value()})
}

pub trait FlexGrow {
	fn value(&self) -> data::Value;
}
pub fn flex_grow(value: impl FlexGrow) -> Style {
	Style::Typed(Rule{property: Property::FlexGrow, value: value.value()})
}

pub trait FlexShrink {
	fn value(&self) -> data::Value;
}
pub fn flex_shrink(value: impl FlexShrink) -> Style {
	Style::Typed(Rule{property: Property::FlexShrink, value: value.value()})
}

pub trait FlexWrap {
	fn value(&self) -> data::Value;
}
pub fn flex_wrap(value: impl FlexWrap) -> Style {
	Style::Typed(Rule{property: Property::FlexWrap, value: value.value()})
}

pub trait Float {
	fn value(&self) -> data::Value;
}
pub fn float(value: impl Float) -> Style {
	Style::Typed(Rule{property: Property::Float, value: value.value()})
}

pub trait FloodColor {
	fn value(&self) -> data::Value;
}
pub fn flood_color(value: impl FloodColor) -> Style {
	Style::Typed(Rule{property: Property::FloodColor, value: value.value()})
}

pub trait FloodOpacity {
	fn value(&self) -> data::Value;
}
pub fn flood_opacity(value: impl FloodOpacity) -> Style {
	Style::Typed(Rule{property: Property::FloodOpacity, value: value.value()})
}

pub trait Font {
	fn value(&self) -> data::Value;
}
pub fn font(value: impl Font) -> Style {
	Style::Typed(Rule{property: Property::Font, value: value.value()})
}

pub trait FontFamily {
	fn value(&self) -> data::Value;
}
pub fn font_family(value: impl FontFamily) -> Style {
	Style::Typed(Rule{property: Property::FontFamily, value: value.value()})
}

pub trait FontFeatureSettings {
	fn value(&self) -> data::Value;
}
pub fn font_feature_settings(value: impl FontFeatureSettings) -> Style {
	Style::Typed(Rule{property: Property::FontFeatureSettings, value: value.value()})
}

pub trait FontKerning {
	fn value(&self) -> data::Value;
}
pub fn font_kerning(value: impl FontKerning) -> Style {
	Style::Typed(Rule{property: Property::FontKerning, value: value.value()})
}

pub trait FontSize {
	fn value(&self) -> data::Value;
}
pub fn font_size(value: impl FontSize) -> Style {
	Style::Typed(Rule{property: Property::FontSize, value: value.value()})
}

pub trait FontSizeAdjust {
	fn value(&self) -> data::Value;
}
pub fn font_size_adjust(value: impl FontSizeAdjust) -> Style {
	Style::Typed(Rule{property: Property::FontSizeAdjust, value: value.value()})
}

pub trait FontStretch {
	fn value(&self) -> data::Value;
}
pub fn font_stretch(value: impl FontStretch) -> Style {
	Style::Typed(Rule{property: Property::FontStretch, value: value.value()})
}

pub trait FontStyle {
	fn value(&self) -> data::Value;
}
pub fn font_style(value: impl FontStyle) -> Style {
	Style::Typed(Rule{property: Property::FontStyle, value: value.value()})
}

pub trait FontSynthesis {
	fn value(&self) -> data::Value;
}
pub fn font_synthesis(value: impl FontSynthesis) -> Style {
	Style::Typed(Rule{property: Property::FontSynthesis, value: value.value()})
}

pub trait FontVariant {
	fn value(&self) -> data::Value;
}
pub fn font_variant(value: impl FontVariant) -> Style {
	Style::Typed(Rule{property: Property::FontVariant, value: value.value()})
}

pub trait FontVariantCaps {
	fn value(&self) -> data::Value;
}
pub fn font_variant_caps(value: impl FontVariantCaps) -> Style {
	Style::Typed(Rule{property: Property::FontVariantCaps, value: value.value()})
}

pub trait FontVariantEastAsian {
	fn value(&self) -> data::Value;
}
pub fn font_variant_east_asian(value: impl FontVariantEastAsian) -> Style {
	Style::Typed(Rule{property: Property::FontVariantEastAsian, value: value.value()})
}

pub trait FontVariantLigatures {
	fn value(&self) -> data::Value;
}
pub fn font_variant_ligatures(value: impl FontVariantLigatures) -> Style {
	Style::Typed(Rule{property: Property::FontVariantLigatures, value: value.value()})
}

pub trait FontVariantNumeric {
	fn value(&self) -> data::Value;
}
pub fn font_variant_numeric(value: impl FontVariantNumeric) -> Style {
	Style::Typed(Rule{property: Property::FontVariantNumeric, value: value.value()})
}

pub trait FontVariantPosition {
	fn value(&self) -> data::Value;
}
pub fn font_variant_position(value: impl FontVariantPosition) -> Style {
	Style::Typed(Rule{property: Property::FontVariantPosition, value: value.value()})
}

pub trait FontWeight {
	fn value(&self) -> data::Value;
}
pub fn font_weight(value: impl FontWeight) -> Style {
	Style::Typed(Rule{property: Property::FontWeight, value: value.value()})
}

pub trait Gap {
	fn value(&self) -> data::Value;
}
pub fn gap(value: impl Gap) -> Style {
	Style::Typed(Rule{property: Property::Gap, value: value.value()})
}

pub trait Globalcompositeoperation {
	fn value(&self) -> data::Value;
}
pub fn globalcompositeoperation(value: impl Globalcompositeoperation) -> Style {
	Style::Typed(Rule{property: Property::Globalcompositeoperation, value: value.value()})
}

pub trait GlyphOrientationVertical {
	fn value(&self) -> data::Value;
}
pub fn glyph_orientation_vertical(value: impl GlyphOrientationVertical) -> Style {
	Style::Typed(Rule{property: Property::GlyphOrientationVertical, value: value.value()})
}

pub trait Grid {
	fn value(&self) -> data::Value;
}
pub fn grid(value: impl Grid) -> Style {
	Style::Typed(Rule{property: Property::Grid, value: value.value()})
}

pub trait GridArea {
	fn value(&self) -> data::Value;
}
pub fn grid_area(value: impl GridArea) -> Style {
	Style::Typed(Rule{property: Property::GridArea, value: value.value()})
}

pub trait GridAutoColumns {
	fn value(&self) -> data::Value;
}
pub fn grid_auto_columns(value: impl GridAutoColumns) -> Style {
	Style::Typed(Rule{property: Property::GridAutoColumns, value: value.value()})
}

pub trait GridAutoFlow {
	fn value(&self) -> data::Value;
}
pub fn grid_auto_flow(value: impl GridAutoFlow) -> Style {
	Style::Typed(Rule{property: Property::GridAutoFlow, value: value.value()})
}

pub trait GridAutoRows {
	fn value(&self) -> data::Value;
}
pub fn grid_auto_rows(value: impl GridAutoRows) -> Style {
	Style::Typed(Rule{property: Property::GridAutoRows, value: value.value()})
}

pub trait GridColumn {
	fn value(&self) -> data::Value;
}
pub fn grid_column(value: impl GridColumn) -> Style {
	Style::Typed(Rule{property: Property::GridColumn, value: value.value()})
}

pub trait GridColumnEnd {
	fn value(&self) -> data::Value;
}
pub fn grid_column_end(value: impl GridColumnEnd) -> Style {
	Style::Typed(Rule{property: Property::GridColumnEnd, value: value.value()})
}

pub trait GridColumnGap {
	fn value(&self) -> data::Value;
}
pub fn grid_column_gap(value: impl GridColumnGap) -> Style {
	Style::Typed(Rule{property: Property::GridColumnGap, value: value.value()})
}

pub trait GridColumnStart {
	fn value(&self) -> data::Value;
}
pub fn grid_column_start(value: impl GridColumnStart) -> Style {
	Style::Typed(Rule{property: Property::GridColumnStart, value: value.value()})
}

pub trait GridGap {
	fn value(&self) -> data::Value;
}
pub fn grid_gap(value: impl GridGap) -> Style {
	Style::Typed(Rule{property: Property::GridGap, value: value.value()})
}

pub trait GridRow {
	fn value(&self) -> data::Value;
}
pub fn grid_row(value: impl GridRow) -> Style {
	Style::Typed(Rule{property: Property::GridRow, value: value.value()})
}

pub trait GridRowEnd {
	fn value(&self) -> data::Value;
}
pub fn grid_row_end(value: impl GridRowEnd) -> Style {
	Style::Typed(Rule{property: Property::GridRowEnd, value: value.value()})
}

pub trait GridRowGap {
	fn value(&self) -> data::Value;
}
pub fn grid_row_gap(value: impl GridRowGap) -> Style {
	Style::Typed(Rule{property: Property::GridRowGap, value: value.value()})
}

pub trait GridRowStart {
	fn value(&self) -> data::Value;
}
pub fn grid_row_start(value: impl GridRowStart) -> Style {
	Style::Typed(Rule{property: Property::GridRowStart, value: value.value()})
}

pub trait GridTemplate {
	fn value(&self) -> data::Value;
}
pub fn grid_template(value: impl GridTemplate) -> Style {
	Style::Typed(Rule{property: Property::GridTemplate, value: value.value()})
}

pub trait GridTemplateAreas {
	fn value(&self) -> data::Value;
}
pub fn grid_template_areas(value: impl GridTemplateAreas) -> Style {
	Style::Typed(Rule{property: Property::GridTemplateAreas, value: value.value()})
}

pub trait GridTemplateColumns {
	fn value(&self) -> data::Value;
}
pub fn grid_template_columns(value: impl GridTemplateColumns) -> Style {
	Style::Typed(Rule{property: Property::GridTemplateColumns, value: value.value()})
}

pub trait GridTemplateRows {
	fn value(&self) -> data::Value;
}
pub fn grid_template_rows(value: impl GridTemplateRows) -> Style {
	Style::Typed(Rule{property: Property::GridTemplateRows, value: value.value()})
}

pub trait HangingPunctuation {
	fn value(&self) -> data::Value;
}
pub fn hanging_punctuation(value: impl HangingPunctuation) -> Style {
	Style::Typed(Rule{property: Property::HangingPunctuation, value: value.value()})
}

pub trait Height {
	fn value(&self) -> data::Value;
}
pub fn height(value: impl Height) -> Style {
	Style::Typed(Rule{property: Property::Height, value: value.value()})
}

pub trait Hyphens {
	fn value(&self) -> data::Value;
}
pub fn hyphens(value: impl Hyphens) -> Style {
	Style::Typed(Rule{property: Property::Hyphens, value: value.value()})
}

pub trait ImageOrientation {
	fn value(&self) -> data::Value;
}
pub fn image_orientation(value: impl ImageOrientation) -> Style {
	Style::Typed(Rule{property: Property::ImageOrientation, value: value.value()})
}

pub trait ImageRendering {
	fn value(&self) -> data::Value;
}
pub fn image_rendering(value: impl ImageRendering) -> Style {
	Style::Typed(Rule{property: Property::ImageRendering, value: value.value()})
}

pub trait ImageResolution {
	fn value(&self) -> data::Value;
}
pub fn image_resolution(value: impl ImageResolution) -> Style {
	Style::Typed(Rule{property: Property::ImageResolution, value: value.value()})
}

pub trait Isolation {
	fn value(&self) -> data::Value;
}
pub fn isolation(value: impl Isolation) -> Style {
	Style::Typed(Rule{property: Property::Isolation, value: value.value()})
}

pub trait JustifyContent {
	fn value(&self) -> data::Value;
}
pub fn justify_content(value: impl JustifyContent) -> Style {
	Style::Typed(Rule{property: Property::JustifyContent, value: value.value()})
}

pub trait JustifyItems {
	fn value(&self) -> data::Value;
}
pub fn justify_items(value: impl JustifyItems) -> Style {
	Style::Typed(Rule{property: Property::JustifyItems, value: value.value()})
}

pub trait JustifySelf {
	fn value(&self) -> data::Value;
}
pub fn justify_self(value: impl JustifySelf) -> Style {
	Style::Typed(Rule{property: Property::JustifySelf, value: value.value()})
}

pub trait Left {
	fn value(&self) -> data::Value;
}
pub fn left(value: impl Left) -> Style {
	Style::Typed(Rule{property: Property::Left, value: value.value()})
}

pub trait LetterSpacing {
	fn value(&self) -> data::Value;
}
pub fn letter_spacing(value: impl LetterSpacing) -> Style {
	Style::Typed(Rule{property: Property::LetterSpacing, value: value.value()})
}

pub trait LightingColor {
	fn value(&self) -> data::Value;
}
pub fn lighting_color(value: impl LightingColor) -> Style {
	Style::Typed(Rule{property: Property::LightingColor, value: value.value()})
}

pub trait LineBreak {
	fn value(&self) -> data::Value;
}
pub fn line_break(value: impl LineBreak) -> Style {
	Style::Typed(Rule{property: Property::LineBreak, value: value.value()})
}

pub trait LineHeight {
	fn value(&self) -> data::Value;
}
pub fn line_height(value: impl LineHeight) -> Style {
	Style::Typed(Rule{property: Property::LineHeight, value: value.value()})
}

pub trait ListStyle {
	fn value(&self) -> data::Value;
}
pub fn list_style(value: impl ListStyle) -> Style {
	Style::Typed(Rule{property: Property::ListStyle, value: value.value()})
}

pub trait ListStyleImage {
	fn value(&self) -> data::Value;
}
pub fn list_style_image(value: impl ListStyleImage) -> Style {
	Style::Typed(Rule{property: Property::ListStyleImage, value: value.value()})
}

pub trait ListStylePosition {
	fn value(&self) -> data::Value;
}
pub fn list_style_position(value: impl ListStylePosition) -> Style {
	Style::Typed(Rule{property: Property::ListStylePosition, value: value.value()})
}

pub trait ListStyleType {
	fn value(&self) -> data::Value;
}
pub fn list_style_type(value: impl ListStyleType) -> Style {
	Style::Typed(Rule{property: Property::ListStyleType, value: value.value()})
}

pub trait Margin {
	fn value(&self) -> data::Value;
}
pub fn margin(value: impl Margin) -> Style {
	Style::Typed(Rule{property: Property::Margin, value: value.value()})
}

pub trait MarginBottom {
	fn value(&self) -> data::Value;
}
pub fn margin_bottom(value: impl MarginBottom) -> Style {
	Style::Typed(Rule{property: Property::MarginBottom, value: value.value()})
}

pub trait MarginLeft {
	fn value(&self) -> data::Value;
}
pub fn margin_left(value: impl MarginLeft) -> Style {
	Style::Typed(Rule{property: Property::MarginLeft, value: value.value()})
}

pub trait MarginRight {
	fn value(&self) -> data::Value;
}
pub fn margin_right(value: impl MarginRight) -> Style {
	Style::Typed(Rule{property: Property::MarginRight, value: value.value()})
}

pub trait MarginTop {
	fn value(&self) -> data::Value;
}
pub fn margin_top(value: impl MarginTop) -> Style {
	Style::Typed(Rule{property: Property::MarginTop, value: value.value()})
}

pub trait Mask {
	fn value(&self) -> data::Value;
}
pub fn mask(value: impl Mask) -> Style {
	Style::Typed(Rule{property: Property::Mask, value: value.value()})
}

pub trait MaskBorder {
	fn value(&self) -> data::Value;
}
pub fn mask_border(value: impl MaskBorder) -> Style {
	Style::Typed(Rule{property: Property::MaskBorder, value: value.value()})
}

pub trait MaskBorderMode {
	fn value(&self) -> data::Value;
}
pub fn mask_border_mode(value: impl MaskBorderMode) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderMode, value: value.value()})
}

pub trait MaskBorderOutset {
	fn value(&self) -> data::Value;
}
pub fn mask_border_outset(value: impl MaskBorderOutset) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderOutset, value: value.value()})
}

pub trait MaskBorderRepeat {
	fn value(&self) -> data::Value;
}
pub fn mask_border_repeat(value: impl MaskBorderRepeat) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderRepeat, value: value.value()})
}

pub trait MaskBorderSlice {
	fn value(&self) -> data::Value;
}
pub fn mask_border_slice(value: impl MaskBorderSlice) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderSlice, value: value.value()})
}

pub trait MaskBorderSource {
	fn value(&self) -> data::Value;
}
pub fn mask_border_source(value: impl MaskBorderSource) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderSource, value: value.value()})
}

pub trait MaskBorderWidth {
	fn value(&self) -> data::Value;
}
pub fn mask_border_width(value: impl MaskBorderWidth) -> Style {
	Style::Typed(Rule{property: Property::MaskBorderWidth, value: value.value()})
}

pub trait MaskClip {
	fn value(&self) -> data::Value;
}
pub fn mask_clip(value: impl MaskClip) -> Style {
	Style::Typed(Rule{property: Property::MaskClip, value: value.value()})
}

pub trait MaskComposite {
	fn value(&self) -> data::Value;
}
pub fn mask_composite(value: impl MaskComposite) -> Style {
	Style::Typed(Rule{property: Property::MaskComposite, value: value.value()})
}

pub trait MaskImage {
	fn value(&self) -> data::Value;
}
pub fn mask_image(value: impl MaskImage) -> Style {
	Style::Typed(Rule{property: Property::MaskImage, value: value.value()})
}

pub trait MaskMode {
	fn value(&self) -> data::Value;
}
pub fn mask_mode(value: impl MaskMode) -> Style {
	Style::Typed(Rule{property: Property::MaskMode, value: value.value()})
}

pub trait MaskOrigin {
	fn value(&self) -> data::Value;
}
pub fn mask_origin(value: impl MaskOrigin) -> Style {
	Style::Typed(Rule{property: Property::MaskOrigin, value: value.value()})
}

pub trait MaskPosition {
	fn value(&self) -> data::Value;
}
pub fn mask_position(value: impl MaskPosition) -> Style {
	Style::Typed(Rule{property: Property::MaskPosition, value: value.value()})
}

pub trait MaskRepeat {
	fn value(&self) -> data::Value;
}
pub fn mask_repeat(value: impl MaskRepeat) -> Style {
	Style::Typed(Rule{property: Property::MaskRepeat, value: value.value()})
}

pub trait MaskSize {
	fn value(&self) -> data::Value;
}
pub fn mask_size(value: impl MaskSize) -> Style {
	Style::Typed(Rule{property: Property::MaskSize, value: value.value()})
}

pub trait MaskType {
	fn value(&self) -> data::Value;
}
pub fn mask_type(value: impl MaskType) -> Style {
	Style::Typed(Rule{property: Property::MaskType, value: value.value()})
}

pub trait MaxHeight {
	fn value(&self) -> data::Value;
}
pub fn max_height(value: impl MaxHeight) -> Style {
	Style::Typed(Rule{property: Property::MaxHeight, value: value.value()})
}

pub trait MaxWidth {
	fn value(&self) -> data::Value;
}
pub fn max_width(value: impl MaxWidth) -> Style {
	Style::Typed(Rule{property: Property::MaxWidth, value: value.value()})
}

pub trait MinHeight {
	fn value(&self) -> data::Value;
}
pub fn min_height(value: impl MinHeight) -> Style {
	Style::Typed(Rule{property: Property::MinHeight, value: value.value()})
}

pub trait MinWidth {
	fn value(&self) -> data::Value;
}
pub fn min_width(value: impl MinWidth) -> Style {
	Style::Typed(Rule{property: Property::MinWidth, value: value.value()})
}

pub trait MixBlendMode {
	fn value(&self) -> data::Value;
}
pub fn mix_blend_mode(value: impl MixBlendMode) -> Style {
	Style::Typed(Rule{property: Property::MixBlendMode, value: value.value()})
}

pub trait ObjectFit {
	fn value(&self) -> data::Value;
}
pub fn object_fit(value: impl ObjectFit) -> Style {
	Style::Typed(Rule{property: Property::ObjectFit, value: value.value()})
}

pub trait ObjectPosition {
	fn value(&self) -> data::Value;
}
pub fn object_position(value: impl ObjectPosition) -> Style {
	Style::Typed(Rule{property: Property::ObjectPosition, value: value.value()})
}

pub trait Opacity {
	fn value(&self) -> data::Value;
}
pub fn opacity(value: impl Opacity) -> Style {
	Style::Typed(Rule{property: Property::Opacity, value: value.value()})
}

pub trait Order {
	fn value(&self) -> data::Value;
}
pub fn order(value: impl Order) -> Style {
	Style::Typed(Rule{property: Property::Order, value: value.value()})
}

pub trait Orphans {
	fn value(&self) -> data::Value;
}
pub fn orphans(value: impl Orphans) -> Style {
	Style::Typed(Rule{property: Property::Orphans, value: value.value()})
}

pub trait Outline {
	fn value(&self) -> data::Value;
}
pub fn outline(value: impl Outline) -> Style {
	Style::Typed(Rule{property: Property::Outline, value: value.value()})
}

pub trait OutlineColor {
	fn value(&self) -> data::Value;
}
pub fn outline_color(value: impl OutlineColor) -> Style {
	Style::Typed(Rule{property: Property::OutlineColor, value: value.value()})
}

pub trait OutlineOffset {
	fn value(&self) -> data::Value;
}
pub fn outline_offset(value: impl OutlineOffset) -> Style {
	Style::Typed(Rule{property: Property::OutlineOffset, value: value.value()})
}

pub trait OutlineStyle {
	fn value(&self) -> data::Value;
}
pub fn outline_style(value: impl OutlineStyle) -> Style {
	Style::Typed(Rule{property: Property::OutlineStyle, value: value.value()})
}

pub trait OutlineWidth {
	fn value(&self) -> data::Value;
}
pub fn outline_width(value: impl OutlineWidth) -> Style {
	Style::Typed(Rule{property: Property::OutlineWidth, value: value.value()})
}

pub trait Overflow {
	fn value(&self) -> data::Value;
}
pub fn overflow(value: impl Overflow) -> Style {
	Style::Typed(Rule{property: Property::Overflow, value: value.value()})
}

pub trait OverflowWrap {
	fn value(&self) -> data::Value;
}
pub fn overflow_wrap(value: impl OverflowWrap) -> Style {
	Style::Typed(Rule{property: Property::OverflowWrap, value: value.value()})
}

pub trait Padding {
	fn value(&self) -> data::Value;
}
impl Padding for f64 {
	fn value(&self) -> data::Value {
		Value::Length(Length::px(self.clone()))
	}
}
impl Padding for (f64, f64) {
	fn value(&self) -> data::Value {
		Value::Length2(
			Length::px(self.0.clone()),
			Length::px(self.1.clone())
		)
	}
}
impl Padding for (f64, f64, f64) {
	fn value(&self) -> data::Value {
		Value::Length3(
			Length::px(self.0.clone()),
			Length::px(self.1.clone()),
			Length::px(self.2.clone()),
		)
	}
}
impl Padding for (f64, f64, f64, f64) {
	fn value(&self) -> data::Value {
		Value::Length4(
			Length::px(self.0.clone()),
			Length::px(self.1.clone()),
			Length::px(self.2.clone()),
			Length::px(self.3.clone()),
		)
	}
}
impl Padding for Length {
	fn value(&self) -> data::Value {
		Value::Length(self.clone())
	}
}
impl Padding for (Length, Length) {
	fn value(&self) -> data::Value {
		Value::Length2(
			self.0.clone(),
			self.1.clone()
		)
	}
}
impl Padding for (Length, Length, Length) {
	fn value(&self) -> data::Value {
		Value::Length3(
			self.0.clone(),
			self.1.clone(),
			self.2.clone(),
		)
	}
}
impl Padding for (Length, Length, Length, Length) {
	fn value(&self) -> data::Value {
		Value::Length4(
			self.0.clone(),
			self.1.clone(),
			self.2.clone(),
			self.3.clone(),
		)
	}
}
pub fn padding(value: impl Padding) -> Style {
	Style::Typed(Rule{property: Property::Padding, value: value.value()})
}

pub trait PaddingBottom {
	fn value(&self) -> data::Value;
}
pub fn padding_bottom(value: impl PaddingBottom) -> Style {
	Style::Typed(Rule{property: Property::PaddingBottom, value: value.value()})
}

pub trait PaddingLeft {
	fn value(&self) -> data::Value;
}
pub fn padding_left(value: impl PaddingLeft) -> Style {
	Style::Typed(Rule{property: Property::PaddingLeft, value: value.value()})
}

pub trait PaddingRight {
	fn value(&self) -> data::Value;
}
pub fn padding_right(value: impl PaddingRight) -> Style {
	Style::Typed(Rule{property: Property::PaddingRight, value: value.value()})
}

pub trait PaddingTop {
	fn value(&self) -> data::Value;
}
pub fn padding_top(value: impl PaddingTop) -> Style {
	Style::Typed(Rule{property: Property::PaddingTop, value: value.value()})
}

pub trait PageBreakAfter {
	fn value(&self) -> data::Value;
}
pub fn page_break_after(value: impl PageBreakAfter) -> Style {
	Style::Typed(Rule{property: Property::PageBreakAfter, value: value.value()})
}

pub trait PageBreakBefore {
	fn value(&self) -> data::Value;
}
pub fn page_break_before(value: impl PageBreakBefore) -> Style {
	Style::Typed(Rule{property: Property::PageBreakBefore, value: value.value()})
}

pub trait PageBreakInside {
	fn value(&self) -> data::Value;
}
pub fn page_break_inside(value: impl PageBreakInside) -> Style {
	Style::Typed(Rule{property: Property::PageBreakInside, value: value.value()})
}

pub trait Pause {
	fn value(&self) -> data::Value;
}
pub fn pause(value: impl Pause) -> Style {
	Style::Typed(Rule{property: Property::Pause, value: value.value()})
}

pub trait PauseAfter {
	fn value(&self) -> data::Value;
}
pub fn pause_after(value: impl PauseAfter) -> Style {
	Style::Typed(Rule{property: Property::PauseAfter, value: value.value()})
}

pub trait PauseBefore {
	fn value(&self) -> data::Value;
}
pub fn pause_before(value: impl PauseBefore) -> Style {
	Style::Typed(Rule{property: Property::PauseBefore, value: value.value()})
}

pub trait Pitch {
	fn value(&self) -> data::Value;
}
pub fn pitch(value: impl Pitch) -> Style {
	Style::Typed(Rule{property: Property::Pitch, value: value.value()})
}

pub trait PitchRange {
	fn value(&self) -> data::Value;
}
pub fn pitch_range(value: impl PitchRange) -> Style {
	Style::Typed(Rule{property: Property::PitchRange, value: value.value()})
}

pub trait PlaceContent {
	fn value(&self) -> data::Value;
}
pub fn place_content(value: impl PlaceContent) -> Style {
	Style::Typed(Rule{property: Property::PlaceContent, value: value.value()})
}

pub trait PlaceItems {
	fn value(&self) -> data::Value;
}
pub fn place_items(value: impl PlaceItems) -> Style {
	Style::Typed(Rule{property: Property::PlaceItems, value: value.value()})
}

pub trait PlaceSelf {
	fn value(&self) -> data::Value;
}
pub fn place_self(value: impl PlaceSelf) -> Style {
	Style::Typed(Rule{property: Property::PlaceSelf, value: value.value()})
}

pub trait PlayDuring {
	fn value(&self) -> data::Value;
}
pub fn play_during(value: impl PlayDuring) -> Style {
	Style::Typed(Rule{property: Property::PlayDuring, value: value.value()})
}

pub trait Position {
	fn value(&self) -> data::Value;
}
pub fn position(value: impl Position) -> Style {
	Style::Typed(Rule{property: Property::Position, value: value.value()})
}

pub trait Quotes {
	fn value(&self) -> data::Value;
}
pub fn quotes(value: impl Quotes) -> Style {
	Style::Typed(Rule{property: Property::Quotes, value: value.value()})
}

pub trait Resize {
	fn value(&self) -> data::Value;
}
pub fn resize(value: impl Resize) -> Style {
	Style::Typed(Rule{property: Property::Resize, value: value.value()})
}

pub trait Rest {
	fn value(&self) -> data::Value;
}
pub fn rest(value: impl Rest) -> Style {
	Style::Typed(Rule{property: Property::Rest, value: value.value()})
}

pub trait RestAfter {
	fn value(&self) -> data::Value;
}
pub fn rest_after(value: impl RestAfter) -> Style {
	Style::Typed(Rule{property: Property::RestAfter, value: value.value()})
}

pub trait RestBefore {
	fn value(&self) -> data::Value;
}
pub fn rest_before(value: impl RestBefore) -> Style {
	Style::Typed(Rule{property: Property::RestBefore, value: value.value()})
}

pub trait Richness {
	fn value(&self) -> data::Value;
}
pub fn richness(value: impl Richness) -> Style {
	Style::Typed(Rule{property: Property::Richness, value: value.value()})
}

pub trait Right {
	fn value(&self) -> data::Value;
}
pub fn right(value: impl Right) -> Style {
	Style::Typed(Rule{property: Property::Right, value: value.value()})
}

pub trait RowGap {
	fn value(&self) -> data::Value;
}
pub fn row_gap(value: impl RowGap) -> Style {
	Style::Typed(Rule{property: Property::RowGap, value: value.value()})
}

pub trait ScrollMargin {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin(value: impl ScrollMargin) -> Style {
	Style::Typed(Rule{property: Property::ScrollMargin, value: value.value()})
}

pub trait ScrollMarginBlock {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_block(value: impl ScrollMarginBlock) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginBlock, value: value.value()})
}

pub trait ScrollMarginBlockEnd {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_block_end(value: impl ScrollMarginBlockEnd) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginBlockEnd, value: value.value()})
}

pub trait ScrollMarginBlockStart {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_block_start(value: impl ScrollMarginBlockStart) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginBlockStart, value: value.value()})
}

pub trait ScrollMarginBottom {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_bottom(value: impl ScrollMarginBottom) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginBottom, value: value.value()})
}

pub trait ScrollMarginInline {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_inline(value: impl ScrollMarginInline) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginInline, value: value.value()})
}

pub trait ScrollMarginInlineEnd {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_inline_end(value: impl ScrollMarginInlineEnd) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginInlineEnd, value: value.value()})
}

pub trait ScrollMarginInlineStart {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_inline_start(value: impl ScrollMarginInlineStart) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginInlineStart, value: value.value()})
}

pub trait ScrollMarginLeft {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_left(value: impl ScrollMarginLeft) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginLeft, value: value.value()})
}

pub trait ScrollMarginRight {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_right(value: impl ScrollMarginRight) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginRight, value: value.value()})
}

pub trait ScrollMarginTop {
	fn value(&self) -> data::Value;
}
pub fn scroll_margin_top(value: impl ScrollMarginTop) -> Style {
	Style::Typed(Rule{property: Property::ScrollMarginTop, value: value.value()})
}

pub trait ScrollPadding {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding(value: impl ScrollPadding) -> Style {
	Style::Typed(Rule{property: Property::ScrollPadding, value: value.value()})
}

pub trait ScrollPaddingBlock {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_block(value: impl ScrollPaddingBlock) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingBlock, value: value.value()})
}

pub trait ScrollPaddingBlockEnd {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_block_end(value: impl ScrollPaddingBlockEnd) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingBlockEnd, value: value.value()})
}

pub trait ScrollPaddingBlockStart {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_block_start(value: impl ScrollPaddingBlockStart) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingBlockStart, value: value.value()})
}

pub trait ScrollPaddingBottom {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_bottom(value: impl ScrollPaddingBottom) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingBottom, value: value.value()})
}

pub trait ScrollPaddingInline {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_inline(value: impl ScrollPaddingInline) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingInline, value: value.value()})
}

pub trait ScrollPaddingInlineEnd {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_inline_end(value: impl ScrollPaddingInlineEnd) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingInlineEnd, value: value.value()})
}

pub trait ScrollPaddingInlineStart {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_inline_start(value: impl ScrollPaddingInlineStart) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingInlineStart, value: value.value()})
}

pub trait ScrollPaddingLeft {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_left(value: impl ScrollPaddingLeft) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingLeft, value: value.value()})
}

pub trait ScrollPaddingRight {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_right(value: impl ScrollPaddingRight) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingRight, value: value.value()})
}

pub trait ScrollPaddingTop {
	fn value(&self) -> data::Value;
}
pub fn scroll_padding_top(value: impl ScrollPaddingTop) -> Style {
	Style::Typed(Rule{property: Property::ScrollPaddingTop, value: value.value()})
}

pub trait ScrollSnapAlign {
	fn value(&self) -> data::Value;
}
pub fn scroll_snap_align(value: impl ScrollSnapAlign) -> Style {
	Style::Typed(Rule{property: Property::ScrollSnapAlign, value: value.value()})
}

pub trait ScrollSnapStop {
	fn value(&self) -> data::Value;
}
pub fn scroll_snap_stop(value: impl ScrollSnapStop) -> Style {
	Style::Typed(Rule{property: Property::ScrollSnapStop, value: value.value()})
}

pub trait ScrollSnapType {
	fn value(&self) -> data::Value;
}
pub fn scroll_snap_type(value: impl ScrollSnapType) -> Style {
	Style::Typed(Rule{property: Property::ScrollSnapType, value: value.value()})
}

pub trait ShapeImageThreshold {
	fn value(&self) -> data::Value;
}
pub fn shape_image_threshold(value: impl ShapeImageThreshold) -> Style {
	Style::Typed(Rule{property: Property::ShapeImageThreshold, value: value.value()})
}

pub trait ShapeMargin {
	fn value(&self) -> data::Value;
}
pub fn shape_margin(value: impl ShapeMargin) -> Style {
	Style::Typed(Rule{property: Property::ShapeMargin, value: value.value()})
}

pub trait ShapeOutside {
	fn value(&self) -> data::Value;
}
pub fn shape_outside(value: impl ShapeOutside) -> Style {
	Style::Typed(Rule{property: Property::ShapeOutside, value: value.value()})
}

pub trait Speak {
	fn value(&self) -> data::Value;
}
pub fn speak(value: impl Speak) -> Style {
	Style::Typed(Rule{property: Property::Speak, value: value.value()})
}

pub trait SpeakAs {
	fn value(&self) -> data::Value;
}
pub fn speak_as(value: impl SpeakAs) -> Style {
	Style::Typed(Rule{property: Property::SpeakAs, value: value.value()})
}

pub trait SpeakHeader {
	fn value(&self) -> data::Value;
}
pub fn speak_header(value: impl SpeakHeader) -> Style {
	Style::Typed(Rule{property: Property::SpeakHeader, value: value.value()})
}

pub trait SpeakNumeral {
	fn value(&self) -> data::Value;
}
pub fn speak_numeral(value: impl SpeakNumeral) -> Style {
	Style::Typed(Rule{property: Property::SpeakNumeral, value: value.value()})
}

pub trait SpeakPunctuation {
	fn value(&self) -> data::Value;
}
pub fn speak_punctuation(value: impl SpeakPunctuation) -> Style {
	Style::Typed(Rule{property: Property::SpeakPunctuation, value: value.value()})
}

pub trait SpeechRate {
	fn value(&self) -> data::Value;
}
pub fn speech_rate(value: impl SpeechRate) -> Style {
	Style::Typed(Rule{property: Property::SpeechRate, value: value.value()})
}

pub trait Stress {
	fn value(&self) -> data::Value;
}
pub fn stress(value: impl Stress) -> Style {
	Style::Typed(Rule{property: Property::Stress, value: value.value()})
}

pub trait TableLayout {
	fn value(&self) -> data::Value;
}
pub fn table_layout(value: impl TableLayout) -> Style {
	Style::Typed(Rule{property: Property::TableLayout, value: value.value()})
}

pub trait TabSize {
	fn value(&self) -> data::Value;
}
pub fn tab_size(value: impl TabSize) -> Style {
	Style::Typed(Rule{property: Property::TabSize, value: value.value()})
}

pub trait TextAlign {
	fn value(&self) -> data::Value;
}
pub fn text_align(value: impl TextAlign) -> Style {
	Style::Typed(Rule{property: Property::TextAlign, value: value.value()})
}

pub trait TextAlignAll {
	fn value(&self) -> data::Value;
}
pub fn text_align_all(value: impl TextAlignAll) -> Style {
	Style::Typed(Rule{property: Property::TextAlignAll, value: value.value()})
}

pub trait TextAlignLast {
	fn value(&self) -> data::Value;
}
pub fn text_align_last(value: impl TextAlignLast) -> Style {
	Style::Typed(Rule{property: Property::TextAlignLast, value: value.value()})
}

pub trait TextCombineUpright {
	fn value(&self) -> data::Value;
}
pub fn text_combine_upright(value: impl TextCombineUpright) -> Style {
	Style::Typed(Rule{property: Property::TextCombineUpright, value: value.value()})
}

pub trait TextDecoration {
	fn value(&self) -> data::Value;
}
pub fn text_decoration(value: impl TextDecoration) -> Style {
	Style::Typed(Rule{property: Property::TextDecoration, value: value.value()})
}

pub trait TextDecorationColor {
	fn value(&self) -> data::Value;
}
pub fn text_decoration_color(value: impl TextDecorationColor) -> Style {
	Style::Typed(Rule{property: Property::TextDecorationColor, value: value.value()})
}

pub trait TextDecorationLine {
	fn value(&self) -> data::Value;
}
pub fn text_decoration_line(value: impl TextDecorationLine) -> Style {
	Style::Typed(Rule{property: Property::TextDecorationLine, value: value.value()})
}

pub trait TextDecorationStyle {
	fn value(&self) -> data::Value;
}
pub fn text_decoration_style(value: impl TextDecorationStyle) -> Style {
	Style::Typed(Rule{property: Property::TextDecorationStyle, value: value.value()})
}

pub trait TextEmphasis {
	fn value(&self) -> data::Value;
}
pub fn text_emphasis(value: impl TextEmphasis) -> Style {
	Style::Typed(Rule{property: Property::TextEmphasis, value: value.value()})
}

pub trait TextEmphasisColor {
	fn value(&self) -> data::Value;
}
pub fn text_emphasis_color(value: impl TextEmphasisColor) -> Style {
	Style::Typed(Rule{property: Property::TextEmphasisColor, value: value.value()})
}

pub trait TextEmphasisPosition {
	fn value(&self) -> data::Value;
}
pub fn text_emphasis_position(value: impl TextEmphasisPosition) -> Style {
	Style::Typed(Rule{property: Property::TextEmphasisPosition, value: value.value()})
}

pub trait TextEmphasisStyle {
	fn value(&self) -> data::Value;
}
pub fn text_emphasis_style(value: impl TextEmphasisStyle) -> Style {
	Style::Typed(Rule{property: Property::TextEmphasisStyle, value: value.value()})
}

pub trait TextIndent {
	fn value(&self) -> data::Value;
}
pub fn text_indent(value: impl TextIndent) -> Style {
	Style::Typed(Rule{property: Property::TextIndent, value: value.value()})
}

pub trait TextJustify {
	fn value(&self) -> data::Value;
}
pub fn text_justify(value: impl TextJustify) -> Style {
	Style::Typed(Rule{property: Property::TextJustify, value: value.value()})
}

pub trait TextOrientation {
	fn value(&self) -> data::Value;
}
pub fn text_orientation(value: impl TextOrientation) -> Style {
	Style::Typed(Rule{property: Property::TextOrientation, value: value.value()})
}

pub trait TextOverflow {
	fn value(&self) -> data::Value;
}
pub fn text_overflow(value: impl TextOverflow) -> Style {
	Style::Typed(Rule{property: Property::TextOverflow, value: value.value()})
}

pub trait TextShadow {
	fn value(&self) -> data::Value;
}
pub fn text_shadow(value: impl TextShadow) -> Style {
	Style::Typed(Rule{property: Property::TextShadow, value: value.value()})
}

pub trait TextTransform {
	fn value(&self) -> data::Value;
}
pub fn text_transform(value: impl TextTransform) -> Style {
	Style::Typed(Rule{property: Property::TextTransform, value: value.value()})
}

pub trait TextUnderlinePosition {
	fn value(&self) -> data::Value;
}
pub fn text_underline_position(value: impl TextUnderlinePosition) -> Style {
	Style::Typed(Rule{property: Property::TextUnderlinePosition, value: value.value()})
}

pub trait Top {
	fn value(&self) -> data::Value;
}
pub fn top(value: impl Top) -> Style {
	Style::Typed(Rule{property: Property::Top, value: value.value()})
}

pub trait Transform {
	fn value(&self) -> data::Value;
}
pub fn transform(value: impl Transform) -> Style {
	Style::Typed(Rule{property: Property::Transform, value: value.value()})
}

pub trait TransformBox {
	fn value(&self) -> data::Value;
}
pub fn transform_box(value: impl TransformBox) -> Style {
	Style::Typed(Rule{property: Property::TransformBox, value: value.value()})
}

pub trait TransformOrigin {
	fn value(&self) -> data::Value;
}
pub fn transform_origin(value: impl TransformOrigin) -> Style {
	Style::Typed(Rule{property: Property::TransformOrigin, value: value.value()})
}

pub trait Transition {
	fn value(&self) -> data::Value;
}
pub fn transition(value: impl Transition) -> Style {
	Style::Typed(Rule{property: Property::Transition, value: value.value()})
}

pub trait TransitionDelay {
	fn value(&self) -> data::Value;
}
pub fn transition_delay(value: impl TransitionDelay) -> Style {
	Style::Typed(Rule{property: Property::TransitionDelay, value: value.value()})
}

pub trait TransitionDuration {
	fn value(&self) -> data::Value;
}
pub fn transition_duration(value: impl TransitionDuration) -> Style {
	Style::Typed(Rule{property: Property::TransitionDuration, value: value.value()})
}

pub trait TransitionProperty {
	fn value(&self) -> data::Value;
}
pub fn transition_property(value: impl TransitionProperty) -> Style {
	Style::Typed(Rule{property: Property::TransitionProperty, value: value.value()})
}

pub trait TransitionTimingFunction {
	fn value(&self) -> data::Value;
}
pub fn transition_timing_function(value: impl TransitionTimingFunction) -> Style {
	Style::Typed(Rule{property: Property::TransitionTimingFunction, value: value.value()})
}

pub trait UnicodeBidi {
	fn value(&self) -> data::Value;
}
pub fn unicode_bidi(value: impl UnicodeBidi) -> Style {
	Style::Typed(Rule{property: Property::UnicodeBidi, value: value.value()})
}

pub trait VerticalAlign {
	fn value(&self) -> data::Value;
}
pub fn vertical_align(value: impl VerticalAlign) -> Style {
	Style::Typed(Rule{property: Property::VerticalAlign, value: value.value()})
}

pub trait Visibility {
	fn value(&self) -> data::Value;
}
pub fn visibility(value: impl Visibility) -> Style {
	Style::Typed(Rule{property: Property::Visibility, value: value.value()})
}

pub trait VoiceBalance {
	fn value(&self) -> data::Value;
}
pub fn voice_balance(value: impl VoiceBalance) -> Style {
	Style::Typed(Rule{property: Property::VoiceBalance, value: value.value()})
}

pub trait VoiceDuration {
	fn value(&self) -> data::Value;
}
pub fn voice_duration(value: impl VoiceDuration) -> Style {
	Style::Typed(Rule{property: Property::VoiceDuration, value: value.value()})
}

pub trait VoiceFamily {
	fn value(&self) -> data::Value;
}
pub fn voice_family(value: impl VoiceFamily) -> Style {
	Style::Typed(Rule{property: Property::VoiceFamily, value: value.value()})
}

pub trait VoicePitch {
	fn value(&self) -> data::Value;
}
pub fn voice_pitch(value: impl VoicePitch) -> Style {
	Style::Typed(Rule{property: Property::VoicePitch, value: value.value()})
}

pub trait VoiceRange {
	fn value(&self) -> data::Value;
}
pub fn voice_range(value: impl VoiceRange) -> Style {
	Style::Typed(Rule{property: Property::VoiceRange, value: value.value()})
}

pub trait VoiceRate {
	fn value(&self) -> data::Value;
}
pub fn voice_rate(value: impl VoiceRate) -> Style {
	Style::Typed(Rule{property: Property::VoiceRate, value: value.value()})
}

pub trait VoiceStress {
	fn value(&self) -> data::Value;
}
pub fn voice_stress(value: impl VoiceStress) -> Style {
	Style::Typed(Rule{property: Property::VoiceStress, value: value.value()})
}

pub trait VoiceVolume {
	fn value(&self) -> data::Value;
}
pub fn voice_volume(value: impl VoiceVolume) -> Style {
	Style::Typed(Rule{property: Property::VoiceVolume, value: value.value()})
}

pub trait Volume {
	fn value(&self) -> data::Value;
}
pub fn volume(value: impl Volume) -> Style {
	Style::Typed(Rule{property: Property::Volume, value: value.value()})
}

pub trait WhiteSpace {
	fn value(&self) -> data::Value;
}
pub fn white_space(value: impl WhiteSpace) -> Style {
	Style::Typed(Rule{property: Property::WhiteSpace, value: value.value()})
}

pub trait Widows {
	fn value(&self) -> data::Value;
}
pub fn widows(value: impl Widows) -> Style {
	Style::Typed(Rule{property: Property::Widows, value: value.value()})
}

pub trait Width {
	fn value(&self) -> data::Value;
}
pub fn width(value: impl Width) -> Style {
	Style::Typed(Rule{property: Property::Width, value: value.value()})
}

pub trait WillChange {
	fn value(&self) -> data::Value;
}
pub fn will_change(value: impl WillChange) -> Style {
	Style::Typed(Rule{property: Property::WillChange, value: value.value()})
}

pub trait WordBreak {
	fn value(&self) -> data::Value;
}
pub fn word_break(value: impl WordBreak) -> Style {
	Style::Typed(Rule{property: Property::WordBreak, value: value.value()})
}

pub trait WordSpacing {
	fn value(&self) -> data::Value;
}
pub fn word_spacing(value: impl WordSpacing) -> Style {
	Style::Typed(Rule{property: Property::WordSpacing, value: value.value()})
}

pub trait WordWrap {
	fn value(&self) -> data::Value;
}
pub fn word_wrap(value: impl WordWrap) -> Style {
	Style::Typed(Rule{property: Property::WordWrap, value: value.value()})
}

pub trait WritingMode {
	fn value(&self) -> data::Value;
}
pub fn writing_mode(value: impl WritingMode) -> Style {
	Style::Typed(Rule{property: Property::WritingMode, value: value.value()})
}

pub trait ZIndex {
	fn value(&self) -> data::Value;
}
pub fn z_index(value: impl ZIndex) -> Style {
	Style::Typed(Rule{property: Property::ZIndex, value: value.value()})
}
