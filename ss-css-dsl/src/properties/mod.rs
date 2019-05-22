pub mod input;
pub mod output;
use crate::core::Style;

/// The "align-content" property.
pub fn align_content(value: impl input::AlignContent) -> Style {
    Style::AlignContent(value.normalize())
}
/// The "align-items" property.
pub fn align_items(value: impl input::AlignItems) -> Style {
    Style::AlignItems(value.normalize())
}
/// The "align-self" property.
pub fn align_self(value: impl input::AlignSelf) -> Style {
    Style::AlignSelf(value.normalize())
}
/// The "all" property.
pub fn all(value: impl input::All) -> Style {
    Style::All(value.normalize())
}
/// The "animation" property.
pub fn animation(value: impl input::Animation) -> Style {
    Style::Animation(value.normalize())
}
/// The "animation-delay" property.
pub fn animation_delay(value: impl input::AnimationDelay) -> Style {
    Style::AnimationDelay(value.normalize())
}
/// The "animation-direction" property.
pub fn animation_direction(value: impl input::AnimationDirection) -> Style {
    Style::AnimationDirection(value.normalize())
}
/// The "animation-duration" property.
pub fn animation_duration(value: impl input::AnimationDuration) -> Style {
    Style::AnimationDuration(value.normalize())
}
/// The "animation-fill-mode" property.
pub fn animation_fill_mode(value: impl input::AnimationFillMode) -> Style {
    Style::AnimationFillMode(value.normalize())
}
/// The "animation-iteration-count" property.
pub fn animation_iteration_count(value: impl input::AnimationIterationCount) -> Style {
    Style::AnimationIterationCount(value.normalize())
}
/// The "animation-name" property.
pub fn animation_name(value: impl input::AnimationName) -> Style {
    Style::AnimationName(value.normalize())
}
/// The "animation-play-state" property.
pub fn animation_play_state(value: impl input::AnimationPlayState) -> Style {
    Style::AnimationPlayState(value.normalize())
}
/// The "animation-timing-function" property.
pub fn animation_timing_function(value: impl input::AnimationTimingFunction) -> Style {
    Style::AnimationTimingFunction(value.normalize())
}
/// The "azimuth" property.
pub fn azimuth(value: impl input::Azimuth) -> Style {
    Style::Azimuth(value.normalize())
}
/// The "background" property.
pub fn background(value: impl input::Background) -> Style {
    Style::Background(value.normalize())
}
/// The "background-attachment" property.
pub fn background_attachment(value: impl input::BackgroundAttachment) -> Style {
    Style::BackgroundAttachment(value.normalize())
}
/// The "background-blend-mode" property.
pub fn background_blend_mode(value: impl input::BackgroundBlendMode) -> Style {
    Style::BackgroundBlendMode(value.normalize())
}
/// The "background-clip" property.
pub fn background_clip(value: impl input::BackgroundClip) -> Style {
    Style::BackgroundClip(value.normalize())
}
/// The "background-color" property.
pub fn background_color(value: impl input::BackgroundColor) -> Style {
    Style::BackgroundColor(value.normalize())
}
/// The "background-image" property.
pub fn background_image(value: impl input::BackgroundImage) -> Style {
    Style::BackgroundImage(value.normalize())
}
/// The "background-origin" property.
pub fn background_origin(value: impl input::BackgroundOrigin) -> Style {
    Style::BackgroundOrigin(value.normalize())
}
/// The "background-position" property.
pub fn background_position(value: impl input::BackgroundPosition) -> Style {
    Style::BackgroundPosition(value.normalize())
}
/// The "background-repeat" property.
pub fn background_repeat(value: impl input::BackgroundRepeat) -> Style {
    Style::BackgroundRepeat(value.normalize())
}
/// The "background-size" property.
pub fn background_size(value: impl input::BackgroundSize) -> Style {
    Style::BackgroundSize(value.normalize())
}
/// The "border" property.
pub fn border(value: impl input::Border) -> Style {
    Style::Border(value.normalize())
}
/// The "border-bottom" property.
pub fn border_bottom(value: impl input::BorderBottom) -> Style {
    Style::BorderBottom(value.normalize())
}
/// The "border-bottom-color" property.
pub fn border_bottom_color(value: impl input::BorderBottomColor) -> Style {
    Style::BorderBottomColor(value.normalize())
}
/// The "border-bottom-left-radius" property.
pub fn border_bottom_left_radius(value: impl input::BorderBottomLeftRadius) -> Style {
    Style::BorderBottomLeftRadius(value.normalize())
}
/// The "border-bottom-right-radius" property.
pub fn border_bottom_right_radius(value: impl input::BorderBottomRightRadius) -> Style {
    Style::BorderBottomRightRadius(value.normalize())
}
/// The "border-bottom-style" property.
pub fn border_bottom_style(value: impl input::BorderBottomStyle) -> Style {
    Style::BorderBottomStyle(value.normalize())
}
/// The "border-bottom-width" property.
pub fn border_bottom_width(value: impl input::BorderBottomWidth) -> Style {
    Style::BorderBottomWidth(value.normalize())
}
/// The "border-collapse" property.
pub fn border_collapse(value: impl input::BorderCollapse) -> Style {
    Style::BorderCollapse(value.normalize())
}
/// The "border-color" property.
pub fn border_color(value: impl input::BorderColor) -> Style {
    Style::BorderColor(value.normalize())
}
/// The "border-image" property.
pub fn border_image(value: impl input::BorderImage) -> Style {
    Style::BorderImage(value.normalize())
}
/// The "border-image-outset" property.
pub fn border_image_outset(value: impl input::BorderImageOutset) -> Style {
    Style::BorderImageOutset(value.normalize())
}
/// The "border-image-repeat" property.
pub fn border_image_repeat(value: impl input::BorderImageRepeat) -> Style {
    Style::BorderImageRepeat(value.normalize())
}
/// The "border-image-slice" property.
pub fn border_image_slice(value: impl input::BorderImageSlice) -> Style {
    Style::BorderImageSlice(value.normalize())
}
/// The "border-image-source" property.
pub fn border_image_source(value: impl input::BorderImageSource) -> Style {
    Style::BorderImageSource(value.normalize())
}
/// The "border-image-width" property.
pub fn border_image_width(value: impl input::BorderImageWidth) -> Style {
    Style::BorderImageWidth(value.normalize())
}
/// The "border-left" property.
pub fn border_left(value: impl input::BorderLeft) -> Style {
    Style::BorderLeft(value.normalize())
}
/// The "border-left-color" property.
pub fn border_left_color(value: impl input::BorderLeftColor) -> Style {
    Style::BorderLeftColor(value.normalize())
}
/// The "border-left-style" property.
pub fn border_left_style(value: impl input::BorderLeftStyle) -> Style {
    Style::BorderLeftStyle(value.normalize())
}
/// The "border-left-width" property.
pub fn border_left_width(value: impl input::BorderLeftWidth) -> Style {
    Style::BorderLeftWidth(value.normalize())
}
/// The "border-radius" property.
pub fn border_radius(value: impl input::BorderRadius) -> Style {
    Style::BorderRadius(value.normalize())
}
/// The "border-right" property.
pub fn border_right(value: impl input::BorderRight) -> Style {
    Style::BorderRight(value.normalize())
}
/// The "border-right-color" property.
pub fn border_right_color(value: impl input::BorderRightColor) -> Style {
    Style::BorderRightColor(value.normalize())
}
/// The "border-right-style" property.
pub fn border_right_style(value: impl input::BorderRightStyle) -> Style {
    Style::BorderRightStyle(value.normalize())
}
/// The "border-right-width" property.
pub fn border_right_width(value: impl input::BorderRightWidth) -> Style {
    Style::BorderRightWidth(value.normalize())
}
/// The "border-spacing" property.
pub fn border_spacing(value: impl input::BorderSpacing) -> Style {
    Style::BorderSpacing(value.normalize())
}
/// The "border-style" property.
pub fn border_style(value: impl input::BorderStyle) -> Style {
    Style::BorderStyle(value.normalize())
}
/// The "border-top" property.
pub fn border_top(value: impl input::BorderTop) -> Style {
    Style::BorderTop(value.normalize())
}
/// The "border-top-color" property.
pub fn border_top_color(value: impl input::BorderTopColor) -> Style {
    Style::BorderTopColor(value.normalize())
}
/// The "border-top-left-radius" property.
pub fn border_top_left_radius(value: impl input::BorderTopLeftRadius) -> Style {
    Style::BorderTopLeftRadius(value.normalize())
}
/// The "border-top-right-radius" property.
pub fn border_top_right_radius(value: impl input::BorderTopRightRadius) -> Style {
    Style::BorderTopRightRadius(value.normalize())
}
/// The "border-top-style" property.
pub fn border_top_style(value: impl input::BorderTopStyle) -> Style {
    Style::BorderTopStyle(value.normalize())
}
/// The "border-top-width" property.
pub fn border_top_width(value: impl input::BorderTopWidth) -> Style {
    Style::BorderTopWidth(value.normalize())
}
/// The "border-width" property.
pub fn border_width(value: impl input::BorderWidth) -> Style {
    Style::BorderWidth(value.normalize())
}
/// The "bottom" property.
pub fn bottom(value: impl input::Bottom) -> Style {
    Style::Bottom(value.normalize())
}
/// The "box-decoration-break" property.
pub fn box_decoration_break(value: impl input::BoxDecorationBreak) -> Style {
    Style::BoxDecorationBreak(value.normalize())
}
/// The "box-shadow" property.
pub fn box_shadow(value: impl input::BoxShadow) -> Style {
    Style::BoxShadow(value.normalize())
}
/// The "box-sizing" property.
pub fn box_sizing(value: impl input::BoxSizing) -> Style {
    Style::BoxSizing(value.normalize())
}
/// The "break-after" property.
pub fn break_after(value: impl input::BreakAfter) -> Style {
    Style::BreakAfter(value.normalize())
}
/// The "break-before" property.
pub fn break_before(value: impl input::BreakBefore) -> Style {
    Style::BreakBefore(value.normalize())
}
/// The "break-inside" property.
pub fn break_inside(value: impl input::BreakInside) -> Style {
    Style::BreakInside(value.normalize())
}
/// The "caption-side" property.
pub fn caption_side(value: impl input::CaptionSide) -> Style {
    Style::CaptionSide(value.normalize())
}
/// The "caret-color" property.
pub fn caret_color(value: impl input::CaretColor) -> Style {
    Style::CaretColor(value.normalize())
}
/// The "clear" property.
pub fn clear(value: impl input::Clear) -> Style {
    Style::Clear(value.normalize())
}
/// The "clip" property.
pub fn clip(value: impl input::Clip) -> Style {
    Style::Clip(value.normalize())
}
/// The "clip-path" property.
pub fn clip_path(value: impl input::ClipPath) -> Style {
    Style::ClipPath(value.normalize())
}
/// The "clip-rule" property.
pub fn clip_rule(value: impl input::ClipRule) -> Style {
    Style::ClipRule(value.normalize())
}
/// The "color" property.
pub fn color(value: impl input::Color) -> Style {
    Style::Color(value.normalize())
}
/// The "color-interpolation-filters" property.
pub fn color_interpolation_filters(value: impl input::ColorInterpolationFilters) -> Style {
    Style::ColorInterpolationFilters(value.normalize())
}
/// The "column-count" property.
pub fn column_count(value: impl input::ColumnCount) -> Style {
    Style::ColumnCount(value.normalize())
}
/// The "column-fill" property.
pub fn column_fill(value: impl input::ColumnFill) -> Style {
    Style::ColumnFill(value.normalize())
}
/// The "column-gap" property.
pub fn column_gap(value: impl input::ColumnGap) -> Style {
    Style::ColumnGap(value.normalize())
}
/// The "column-rule" property.
pub fn column_rule(value: impl input::ColumnRule) -> Style {
    Style::ColumnRule(value.normalize())
}
/// The "column-rule-color" property.
pub fn column_rule_color(value: impl input::ColumnRuleColor) -> Style {
    Style::ColumnRuleColor(value.normalize())
}
/// The "column-rule-style" property.
pub fn column_rule_style(value: impl input::ColumnRuleStyle) -> Style {
    Style::ColumnRuleStyle(value.normalize())
}
/// The "column-rule-width" property.
pub fn column_rule_width(value: impl input::ColumnRuleWidth) -> Style {
    Style::ColumnRuleWidth(value.normalize())
}
/// The "columns" property.
pub fn columns(value: impl input::Columns) -> Style {
    Style::Columns(value.normalize())
}
/// The "column-span" property.
pub fn column_span(value: impl input::ColumnSpan) -> Style {
    Style::ColumnSpan(value.normalize())
}
/// The "column-width" property.
pub fn column_width(value: impl input::ColumnWidth) -> Style {
    Style::ColumnWidth(value.normalize())
}
/// The "contain" property.
pub fn contain(value: impl input::Contain) -> Style {
    Style::Contain(value.normalize())
}
/// The "content" property.
pub fn content(value: impl input::Content) -> Style {
    Style::Content(value.normalize())
}
/// The "counter-increment" property.
pub fn counter_increment(value: impl input::CounterIncrement) -> Style {
    Style::CounterIncrement(value.normalize())
}
/// The "counter-reset" property.
pub fn counter_reset(value: impl input::CounterReset) -> Style {
    Style::CounterReset(value.normalize())
}
/// The "cue" property.
pub fn cue(value: impl input::Cue) -> Style {
    Style::Cue(value.normalize())
}
/// The "cue-after" property.
pub fn cue_after(value: impl input::CueAfter) -> Style {
    Style::CueAfter(value.normalize())
}
/// The "cue-before" property.
pub fn cue_before(value: impl input::CueBefore) -> Style {
    Style::CueBefore(value.normalize())
}
/// The "cursor" property.
pub fn cursor(value: impl input::Cursor) -> Style {
    Style::Cursor(value.normalize())
}
/// The "direction" property.
pub fn direction(value: impl input::Direction) -> Style {
    Style::Direction(value.normalize())
}
/// The "display" property.
pub fn display(value: impl input::Display) -> Style {
    Style::Display(value.normalize())
}
/// The "elevation" property.
pub fn elevation(value: impl input::Elevation) -> Style {
    Style::Elevation(value.normalize())
}
/// The "empty-cells" property.
pub fn empty_cells(value: impl input::EmptyCells) -> Style {
    Style::EmptyCells(value.normalize())
}
/// The "filter" property.
pub fn filter(value: impl input::Filter) -> Style {
    Style::Filter(value.normalize())
}
/// The "flex" property.
pub fn flex_(value: impl input::Flex) -> Style {
    Style::Flex(value.normalize())
}
/// The "flex-basis" property.
pub fn flex_basis(value: impl input::FlexBasis) -> Style {
    Style::FlexBasis(value.normalize())
}
/// The "flex-direction" property.
pub fn flex_direction(value: impl input::FlexDirection) -> Style {
    Style::FlexDirection(value.normalize())
}
/// The "flex-flow" property.
pub fn flex_flow(value: impl input::FlexFlow) -> Style {
    Style::FlexFlow(value.normalize())
}
/// The "flex-grow" property.
pub fn flex_grow(value: impl input::FlexGrow) -> Style {
    Style::FlexGrow(value.normalize())
}
/// The "flex-shrink" property.
pub fn flex_shrink(value: impl input::FlexShrink) -> Style {
    Style::FlexShrink(value.normalize())
}
/// The "flex-wrap" property.
pub fn flex_wrap(value: impl input::FlexWrap) -> Style {
    Style::FlexWrap(value.normalize())
}
/// The "float" property.
pub fn float(value: impl input::Float) -> Style {
    Style::Float(value.normalize())
}
/// The "flood-color" property.
pub fn flood_color(value: impl input::FloodColor) -> Style {
    Style::FloodColor(value.normalize())
}
/// The "flood-opacity" property.
pub fn flood_opacity(value: impl input::FloodOpacity) -> Style {
    Style::FloodOpacity(value.normalize())
}
/// The "font" property.
pub fn font(value: impl input::Font) -> Style {
    Style::Font(value.normalize())
}
/// The "font-family" property.
pub fn font_family(value: impl input::FontFamily) -> Style {
    Style::FontFamily(value.normalize())
}
/// The "font-feature-settings" property.
pub fn font_feature_settings(value: impl input::FontFeatureSettings) -> Style {
    Style::FontFeatureSettings(value.normalize())
}
/// The "font-kerning" property.
pub fn font_kerning(value: impl input::FontKerning) -> Style {
    Style::FontKerning(value.normalize())
}
/// The "font-size" property.
pub fn font_size(value: impl input::FontSize) -> Style {
    Style::FontSize(value.normalize())
}
/// The "font-size-adjust" property.
pub fn font_size_adjust(value: impl input::FontSizeAdjust) -> Style {
    Style::FontSizeAdjust(value.normalize())
}
/// The "font-stretch" property.
pub fn font_stretch(value: impl input::FontStretch) -> Style {
    Style::FontStretch(value.normalize())
}
/// The "font-style" property.
pub fn font_style(value: impl input::FontStyle) -> Style {
    Style::FontStyle(value.normalize())
}
/// The "font-synthesis" property.
pub fn font_synthesis(value: impl input::FontSynthesis) -> Style {
    Style::FontSynthesis(value.normalize())
}
/// The "font-variant" property.
pub fn font_variant(value: impl input::FontVariant) -> Style {
    Style::FontVariant(value.normalize())
}
/// The "font-variant-caps" property.
pub fn font_variant_caps(value: impl input::FontVariantCaps) -> Style {
    Style::FontVariantCaps(value.normalize())
}
/// The "font-variant-east-asian" property.
pub fn font_variant_east_asian(value: impl input::FontVariantEastAsian) -> Style {
    Style::FontVariantEastAsian(value.normalize())
}
/// The "font-variant-ligatures" property.
pub fn font_variant_ligatures(value: impl input::FontVariantLigatures) -> Style {
    Style::FontVariantLigatures(value.normalize())
}
/// The "font-variant-numeric" property.
pub fn font_variant_numeric(value: impl input::FontVariantNumeric) -> Style {
    Style::FontVariantNumeric(value.normalize())
}
/// The "font-variant-position" property.
pub fn font_variant_position(value: impl input::FontVariantPosition) -> Style {
    Style::FontVariantPosition(value.normalize())
}
/// The "font-weight" property.
pub fn font_weight(value: impl input::FontWeight) -> Style {
    Style::FontWeight(value.normalize())
}
/// The "gap" property.
pub fn gap(value: impl input::Gap) -> Style {
    Style::Gap(value.normalize())
}
/// The "globalcompositeoperation" property.
pub fn globalcompositeoperation(value: impl input::Globalcompositeoperation) -> Style {
    Style::Globalcompositeoperation(value.normalize())
}
/// The "glyph-orientation-vertical" property.
pub fn glyph_orientation_vertical(value: impl input::GlyphOrientationVertical) -> Style {
    Style::GlyphOrientationVertical(value.normalize())
}
/// The "grid" property.
pub fn grid(value: impl input::Grid) -> Style {
    Style::Grid(value.normalize())
}
/// The "grid-area" property.
pub fn grid_area(value: impl input::GridArea) -> Style {
    Style::GridArea(value.normalize())
}
/// The "grid-auto-columns" property.
pub fn grid_auto_columns(value: impl input::GridAutoColumns) -> Style {
    Style::GridAutoColumns(value.normalize())
}
/// The "grid-auto-flow" property.
pub fn grid_auto_flow(value: impl input::GridAutoFlow) -> Style {
    Style::GridAutoFlow(value.normalize())
}
/// The "grid-auto-rows" property.
pub fn grid_auto_rows(value: impl input::GridAutoRows) -> Style {
    Style::GridAutoRows(value.normalize())
}
/// The "grid-column" property.
pub fn grid_column(value: impl input::GridColumn) -> Style {
    Style::GridColumn(value.normalize())
}
/// The "grid-column-end" property.
pub fn grid_column_end(value: impl input::GridColumnEnd) -> Style {
    Style::GridColumnEnd(value.normalize())
}
/// The "grid-column-gap" property.
pub fn grid_column_gap(value: impl input::GridColumnGap) -> Style {
    Style::GridColumnGap(value.normalize())
}
/// The "grid-column-start" property.
pub fn grid_column_start(value: impl input::GridColumnStart) -> Style {
    Style::GridColumnStart(value.normalize())
}
/// The "grid-gap" property.
pub fn grid_gap(value: impl input::GridGap) -> Style {
    Style::GridGap(value.normalize())
}
/// The "grid-row" property.
pub fn grid_row(value: impl input::GridRow) -> Style {
    Style::GridRow(value.normalize())
}
/// The "grid-row-end" property.
pub fn grid_row_end(value: impl input::GridRowEnd) -> Style {
    Style::GridRowEnd(value.normalize())
}
/// The "grid-row-gap" property.
pub fn grid_row_gap(value: impl input::GridRowGap) -> Style {
    Style::GridRowGap(value.normalize())
}
/// The "grid-row-start" property.
pub fn grid_row_start(value: impl input::GridRowStart) -> Style {
    Style::GridRowStart(value.normalize())
}
/// The "grid-template" property.
pub fn grid_template(value: impl input::GridTemplate) -> Style {
    Style::GridTemplate(value.normalize())
}
/// The "grid-template-areas" property.
pub fn grid_template_areas(value: impl input::GridTemplateAreas) -> Style {
    Style::GridTemplateAreas(value.normalize())
}
/// The "grid-template-columns" property.
pub fn grid_template_columns(value: impl input::GridTemplateColumns) -> Style {
    Style::GridTemplateColumns(value.normalize())
}
/// The "grid-template-rows" property.
pub fn grid_template_rows(value: impl input::GridTemplateRows) -> Style {
    Style::GridTemplateRows(value.normalize())
}
/// The "hanging-punctuation" property.
pub fn hanging_punctuation(value: impl input::HangingPunctuation) -> Style {
    Style::HangingPunctuation(value.normalize())
}
/// The "height" property.
pub fn height(value: impl input::Height) -> Style {
    Style::Height(value.normalize())
}
/// The "hyphens" property.
pub fn hyphens(value: impl input::Hyphens) -> Style {
    Style::Hyphens(value.normalize())
}
/// The "image-orientation" property.
pub fn image_orientation(value: impl input::ImageOrientation) -> Style {
    Style::ImageOrientation(value.normalize())
}
/// The "image-rendering" property.
pub fn image_rendering(value: impl input::ImageRendering) -> Style {
    Style::ImageRendering(value.normalize())
}
/// The "image-resolution" property.
pub fn image_resolution(value: impl input::ImageResolution) -> Style {
    Style::ImageResolution(value.normalize())
}
/// The "isolation" property.
pub fn isolation(value: impl input::Isolation) -> Style {
    Style::Isolation(value.normalize())
}
/// The "justify-content" property.
pub fn justify_content(value: impl input::JustifyContent) -> Style {
    Style::JustifyContent(value.normalize())
}
/// The "justify-items" property.
pub fn justify_items(value: impl input::JustifyItems) -> Style {
    Style::JustifyItems(value.normalize())
}
/// The "justify-self" property.
pub fn justify_self(value: impl input::JustifySelf) -> Style {
    Style::JustifySelf(value.normalize())
}
/// The "left" property.
pub fn left(value: impl input::Left) -> Style {
    Style::Left(value.normalize())
}
/// The "letter-spacing" property.
pub fn letter_spacing(value: impl input::LetterSpacing) -> Style {
    Style::LetterSpacing(value.normalize())
}
/// The "lighting-color" property.
pub fn lighting_color(value: impl input::LightingColor) -> Style {
    Style::LightingColor(value.normalize())
}
/// The "line-break" property.
pub fn line_break(value: impl input::LineBreak) -> Style {
    Style::LineBreak(value.normalize())
}
/// The "line-height" property.
pub fn line_height(value: impl input::LineHeight) -> Style {
    Style::LineHeight(value.normalize())
}
/// The "list-style" property.
pub fn list_style(value: impl input::ListStyle) -> Style {
    Style::ListStyle(value.normalize())
}
/// The "list-style-image" property.
pub fn list_style_image(value: impl input::ListStyleImage) -> Style {
    Style::ListStyleImage(value.normalize())
}
/// The "list-style-position" property.
pub fn list_style_position(value: impl input::ListStylePosition) -> Style {
    Style::ListStylePosition(value.normalize())
}
/// The "list-style-type" property.
pub fn list_style_type(value: impl input::ListStyleType) -> Style {
    Style::ListStyleType(value.normalize())
}
/// The "margin" property.
pub fn margin(value: impl input::Margin) -> Style {
    Style::Margin(value.normalize())
}
/// The "margin-bottom" property.
pub fn margin_bottom(value: impl input::MarginBottom) -> Style {
    Style::MarginBottom(value.normalize())
}
/// The "margin-left" property.
pub fn margin_left(value: impl input::MarginLeft) -> Style {
    Style::MarginLeft(value.normalize())
}
/// The "margin-right" property.
pub fn margin_right(value: impl input::MarginRight) -> Style {
    Style::MarginRight(value.normalize())
}
/// The "margin-top" property.
pub fn margin_top(value: impl input::MarginTop) -> Style {
    Style::MarginTop(value.normalize())
}
/// The "mask" property.
pub fn mask(value: impl input::Mask) -> Style {
    Style::Mask(value.normalize())
}
/// The "mask-border" property.
pub fn mask_border(value: impl input::MaskBorder) -> Style {
    Style::MaskBorder(value.normalize())
}
/// The "mask-border-mode" property.
pub fn mask_border_mode(value: impl input::MaskBorderMode) -> Style {
    Style::MaskBorderMode(value.normalize())
}
/// The "mask-border-outset" property.
pub fn mask_border_outset(value: impl input::MaskBorderOutset) -> Style {
    Style::MaskBorderOutset(value.normalize())
}
/// The "mask-border-repeat" property.
pub fn mask_border_repeat(value: impl input::MaskBorderRepeat) -> Style {
    Style::MaskBorderRepeat(value.normalize())
}
/// The "mask-border-slice" property.
pub fn mask_border_slice(value: impl input::MaskBorderSlice) -> Style {
    Style::MaskBorderSlice(value.normalize())
}
/// The "mask-border-source" property.
pub fn mask_border_source(value: impl input::MaskBorderSource) -> Style {
    Style::MaskBorderSource(value.normalize())
}
/// The "mask-border-width" property.
pub fn mask_border_width(value: impl input::MaskBorderWidth) -> Style {
    Style::MaskBorderWidth(value.normalize())
}
/// The "mask-clip" property.
pub fn mask_clip(value: impl input::MaskClip) -> Style {
    Style::MaskClip(value.normalize())
}
/// The "mask-composite" property.
pub fn mask_composite(value: impl input::MaskComposite) -> Style {
    Style::MaskComposite(value.normalize())
}
/// The "mask-image" property.
pub fn mask_image(value: impl input::MaskImage) -> Style {
    Style::MaskImage(value.normalize())
}
/// The "mask-mode" property.
pub fn mask_mode(value: impl input::MaskMode) -> Style {
    Style::MaskMode(value.normalize())
}
/// The "mask-origin" property.
pub fn mask_origin(value: impl input::MaskOrigin) -> Style {
    Style::MaskOrigin(value.normalize())
}
/// The "mask-position" property.
pub fn mask_position(value: impl input::MaskPosition) -> Style {
    Style::MaskPosition(value.normalize())
}
/// The "mask-repeat" property.
pub fn mask_repeat(value: impl input::MaskRepeat) -> Style {
    Style::MaskRepeat(value.normalize())
}
/// The "mask-size" property.
pub fn mask_size(value: impl input::MaskSize) -> Style {
    Style::MaskSize(value.normalize())
}
/// The "mask-type" property.
pub fn mask_type(value: impl input::MaskType) -> Style {
    Style::MaskType(value.normalize())
}
/// The "max-height" property.
pub fn max_height(value: impl input::MaxHeight) -> Style {
    Style::MaxHeight(value.normalize())
}
/// The "max-width" property.
pub fn max_width(value: impl input::MaxWidth) -> Style {
    Style::MaxWidth(value.normalize())
}
/// The "min-height" property.
pub fn min_height(value: impl input::MinHeight) -> Style {
    Style::MinHeight(value.normalize())
}
/// The "min-width" property.
pub fn min_width(value: impl input::MinWidth) -> Style {
    Style::MinWidth(value.normalize())
}
/// The "mix-blend-mode" property.
pub fn mix_blend_mode(value: impl input::MixBlendMode) -> Style {
    Style::MixBlendMode(value.normalize())
}
/// The "object-fit" property.
pub fn object_fit(value: impl input::ObjectFit) -> Style {
    Style::ObjectFit(value.normalize())
}
/// The "object-position" property.
pub fn object_position(value: impl input::ObjectPosition) -> Style {
    Style::ObjectPosition(value.normalize())
}
/// The "opacity" property.
pub fn opacity(value: impl input::Opacity) -> Style {
    Style::Opacity(value.normalize())
}
/// The "order" property.
pub fn order(value: impl input::Order) -> Style {
    Style::Order(value.normalize())
}
/// The "orphans" property.
pub fn orphans(value: impl input::Orphans) -> Style {
    Style::Orphans(value.normalize())
}
/// The "outline" property.
pub fn outline(value: impl input::Outline) -> Style {
    Style::Outline(value.normalize())
}
/// The "outline-color" property.
pub fn outline_color(value: impl input::OutlineColor) -> Style {
    Style::OutlineColor(value.normalize())
}
/// The "outline-offset" property.
pub fn outline_offset(value: impl input::OutlineOffset) -> Style {
    Style::OutlineOffset(value.normalize())
}
/// The "outline-style" property.
pub fn outline_style(value: impl input::OutlineStyle) -> Style {
    Style::OutlineStyle(value.normalize())
}
/// The "outline-width" property.
pub fn outline_width(value: impl input::OutlineWidth) -> Style {
    Style::OutlineWidth(value.normalize())
}
/// The "overflow" property.
pub fn overflow(value: impl input::Overflow) -> Style {
    Style::Overflow(value.normalize())
}
/// The "overflow-wrap" property.
pub fn overflow_wrap(value: impl input::OverflowWrap) -> Style {
    Style::OverflowWrap(value.normalize())
}
/// The "padding" property.
pub fn padding(value: impl input::Padding) -> Style {
    Style::Padding(value.normalize())
}
/// The "padding-bottom" property.
pub fn padding_bottom(value: impl input::PaddingBottom) -> Style {
    Style::PaddingBottom(value.normalize())
}
/// The "padding-left" property.
pub fn padding_left(value: impl input::PaddingLeft) -> Style {
    Style::PaddingLeft(value.normalize())
}
/// The "padding-right" property.
pub fn padding_right(value: impl input::PaddingRight) -> Style {
    Style::PaddingRight(value.normalize())
}
/// The "padding-top" property.
pub fn padding_top(value: impl input::PaddingTop) -> Style {
    Style::PaddingTop(value.normalize())
}
/// The "page-break-after" property.
pub fn page_break_after(value: impl input::PageBreakAfter) -> Style {
    Style::PageBreakAfter(value.normalize())
}
/// The "page-break-before" property.
pub fn page_break_before(value: impl input::PageBreakBefore) -> Style {
    Style::PageBreakBefore(value.normalize())
}
/// The "page-break-inside" property.
pub fn page_break_inside(value: impl input::PageBreakInside) -> Style {
    Style::PageBreakInside(value.normalize())
}
/// The "pause" property.
pub fn pause(value: impl input::Pause) -> Style {
    Style::Pause(value.normalize())
}
/// The "pause-after" property.
pub fn pause_after(value: impl input::PauseAfter) -> Style {
    Style::PauseAfter(value.normalize())
}
/// The "pause-before" property.
pub fn pause_before(value: impl input::PauseBefore) -> Style {
    Style::PauseBefore(value.normalize())
}
/// The "pitch" property.
pub fn pitch(value: impl input::Pitch) -> Style {
    Style::Pitch(value.normalize())
}
/// The "pitch-range" property.
pub fn pitch_range(value: impl input::PitchRange) -> Style {
    Style::PitchRange(value.normalize())
}
/// The "place-content" property.
pub fn place_content(value: impl input::PlaceContent) -> Style {
    Style::PlaceContent(value.normalize())
}
/// The "place-items" property.
pub fn place_items(value: impl input::PlaceItems) -> Style {
    Style::PlaceItems(value.normalize())
}
/// The "place-self" property.
pub fn place_self(value: impl input::PlaceSelf) -> Style {
    Style::PlaceSelf(value.normalize())
}
/// The "play-during" property.
pub fn play_during(value: impl input::PlayDuring) -> Style {
    Style::PlayDuring(value.normalize())
}
/// The "position" property.
pub fn position(value: impl input::Position) -> Style {
    Style::Position(value.normalize())
}
/// The "quotes" property.
pub fn quotes(value: impl input::Quotes) -> Style {
    Style::Quotes(value.normalize())
}
/// The "resize" property.
pub fn resize(value: impl input::Resize) -> Style {
    Style::Resize(value.normalize())
}
/// The "rest" property.
pub fn rest(value: impl input::Rest) -> Style {
    Style::Rest(value.normalize())
}
/// The "rest-after" property.
pub fn rest_after(value: impl input::RestAfter) -> Style {
    Style::RestAfter(value.normalize())
}
/// The "rest-before" property.
pub fn rest_before(value: impl input::RestBefore) -> Style {
    Style::RestBefore(value.normalize())
}
/// The "richness" property.
pub fn richness(value: impl input::Richness) -> Style {
    Style::Richness(value.normalize())
}
/// The "right" property.
pub fn right(value: impl input::Right) -> Style {
    Style::Right(value.normalize())
}
/// The "row-gap" property.
pub fn row_gap(value: impl input::RowGap) -> Style {
    Style::RowGap(value.normalize())
}
/// The "scroll-margin" property.
pub fn scroll_margin(value: impl input::ScrollMargin) -> Style {
    Style::ScrollMargin(value.normalize())
}
/// The "scroll-margin-block" property.
pub fn scroll_margin_block(value: impl input::ScrollMarginBlock) -> Style {
    Style::ScrollMarginBlock(value.normalize())
}
/// The "scroll-margin-block-end" property.
pub fn scroll_margin_block_end(value: impl input::ScrollMarginBlockEnd) -> Style {
    Style::ScrollMarginBlockEnd(value.normalize())
}
/// The "scroll-margin-block-start" property.
pub fn scroll_margin_block_start(value: impl input::ScrollMarginBlockStart) -> Style {
    Style::ScrollMarginBlockStart(value.normalize())
}
/// The "scroll-margin-bottom" property.
pub fn scroll_margin_bottom(value: impl input::ScrollMarginBottom) -> Style {
    Style::ScrollMarginBottom(value.normalize())
}
/// The "scroll-margin-inline" property.
pub fn scroll_margin_inline(value: impl input::ScrollMarginInline) -> Style {
    Style::ScrollMarginInline(value.normalize())
}
/// The "scroll-margin-inline-end" property.
pub fn scroll_margin_inline_end(value: impl input::ScrollMarginInlineEnd) -> Style {
    Style::ScrollMarginInlineEnd(value.normalize())
}
/// The "scroll-margin-inline-start" property.
pub fn scroll_margin_inline_start(value: impl input::ScrollMarginInlineStart) -> Style {
    Style::ScrollMarginInlineStart(value.normalize())
}
/// The "scroll-margin-left" property.
pub fn scroll_margin_left(value: impl input::ScrollMarginLeft) -> Style {
    Style::ScrollMarginLeft(value.normalize())
}
/// The "scroll-margin-right" property.
pub fn scroll_margin_right(value: impl input::ScrollMarginRight) -> Style {
    Style::ScrollMarginRight(value.normalize())
}
/// The "scroll-margin-top" property.
pub fn scroll_margin_top(value: impl input::ScrollMarginTop) -> Style {
    Style::ScrollMarginTop(value.normalize())
}
/// The "scroll-padding" property.
pub fn scroll_padding(value: impl input::ScrollPadding) -> Style {
    Style::ScrollPadding(value.normalize())
}
/// The "scroll-padding-block" property.
pub fn scroll_padding_block(value: impl input::ScrollPaddingBlock) -> Style {
    Style::ScrollPaddingBlock(value.normalize())
}
/// The "scroll-padding-block-end" property.
pub fn scroll_padding_block_end(value: impl input::ScrollPaddingBlockEnd) -> Style {
    Style::ScrollPaddingBlockEnd(value.normalize())
}
/// The "scroll-padding-block-start" property.
pub fn scroll_padding_block_start(value: impl input::ScrollPaddingBlockStart) -> Style {
    Style::ScrollPaddingBlockStart(value.normalize())
}
/// The "scroll-padding-bottom" property.
pub fn scroll_padding_bottom(value: impl input::ScrollPaddingBottom) -> Style {
    Style::ScrollPaddingBottom(value.normalize())
}
/// The "scroll-padding-inline" property.
pub fn scroll_padding_inline(value: impl input::ScrollPaddingInline) -> Style {
    Style::ScrollPaddingInline(value.normalize())
}
/// The "scroll-padding-inline-end" property.
pub fn scroll_padding_inline_end(value: impl input::ScrollPaddingInlineEnd) -> Style {
    Style::ScrollPaddingInlineEnd(value.normalize())
}
/// The "scroll-padding-inline-start" property.
pub fn scroll_padding_inline_start(value: impl input::ScrollPaddingInlineStart) -> Style {
    Style::ScrollPaddingInlineStart(value.normalize())
}
/// The "scroll-padding-left" property.
pub fn scroll_padding_left(value: impl input::ScrollPaddingLeft) -> Style {
    Style::ScrollPaddingLeft(value.normalize())
}
/// The "scroll-padding-right" property.
pub fn scroll_padding_right(value: impl input::ScrollPaddingRight) -> Style {
    Style::ScrollPaddingRight(value.normalize())
}
/// The "scroll-padding-top" property.
pub fn scroll_padding_top(value: impl input::ScrollPaddingTop) -> Style {
    Style::ScrollPaddingTop(value.normalize())
}
/// The "scroll-snap-align" property.
pub fn scroll_snap_align(value: impl input::ScrollSnapAlign) -> Style {
    Style::ScrollSnapAlign(value.normalize())
}
/// The "scroll-snap-stop" property.
pub fn scroll_snap_stop(value: impl input::ScrollSnapStop) -> Style {
    Style::ScrollSnapStop(value.normalize())
}
/// The "scroll-snap-type" property.
pub fn scroll_snap_type(value: impl input::ScrollSnapType) -> Style {
    Style::ScrollSnapType(value.normalize())
}
/// The "shape-image-threshold" property.
pub fn shape_image_threshold(value: impl input::ShapeImageThreshold) -> Style {
    Style::ShapeImageThreshold(value.normalize())
}
/// The "shape-margin" property.
pub fn shape_margin(value: impl input::ShapeMargin) -> Style {
    Style::ShapeMargin(value.normalize())
}
/// The "shape-outside" property.
pub fn shape_outside(value: impl input::ShapeOutside) -> Style {
    Style::ShapeOutside(value.normalize())
}
/// The "speak" property.
pub fn speak(value: impl input::Speak) -> Style {
    Style::Speak(value.normalize())
}
/// The "speak-as" property.
pub fn speak_as(value: impl input::SpeakAs) -> Style {
    Style::SpeakAs(value.normalize())
}
/// The "speak-header" property.
pub fn speak_header(value: impl input::SpeakHeader) -> Style {
    Style::SpeakHeader(value.normalize())
}
/// The "speak-numeral" property.
pub fn speak_numeral(value: impl input::SpeakNumeral) -> Style {
    Style::SpeakNumeral(value.normalize())
}
/// The "speak-punctuation" property.
pub fn speak_punctuation(value: impl input::SpeakPunctuation) -> Style {
    Style::SpeakPunctuation(value.normalize())
}
/// The "speech-rate" property.
pub fn speech_rate(value: impl input::SpeechRate) -> Style {
    Style::SpeechRate(value.normalize())
}
/// The "stress" property.
pub fn stress(value: impl input::Stress) -> Style {
    Style::Stress(value.normalize())
}
/// The "table-layout" property.
pub fn table_layout(value: impl input::TableLayout) -> Style {
    Style::TableLayout(value.normalize())
}
/// The "tab-size" property.
pub fn tab_size(value: impl input::TabSize) -> Style {
    Style::TabSize(value.normalize())
}
/// The "text-align" property.
pub fn text_align(value: impl input::TextAlign) -> Style {
    Style::TextAlign(value.normalize())
}
/// The "text-align-all" property.
pub fn text_align_all(value: impl input::TextAlignAll) -> Style {
    Style::TextAlignAll(value.normalize())
}
/// The "text-align-last" property.
pub fn text_align_last(value: impl input::TextAlignLast) -> Style {
    Style::TextAlignLast(value.normalize())
}
/// The "text-combine-upright" property.
pub fn text_combine_upright(value: impl input::TextCombineUpright) -> Style {
    Style::TextCombineUpright(value.normalize())
}
/// The "text-decoration" property.
pub fn text_decoration(value: impl input::TextDecoration) -> Style {
    Style::TextDecoration(value.normalize())
}
/// The "text-decoration-color" property.
pub fn text_decoration_color(value: impl input::TextDecorationColor) -> Style {
    Style::TextDecorationColor(value.normalize())
}
/// The "text-decoration-line" property.
pub fn text_decoration_line(value: impl input::TextDecorationLine) -> Style {
    Style::TextDecorationLine(value.normalize())
}
/// The "text-decoration-style" property.
pub fn text_decoration_style(value: impl input::TextDecorationStyle) -> Style {
    Style::TextDecorationStyle(value.normalize())
}
/// The "text-emphasis" property.
pub fn text_emphasis(value: impl input::TextEmphasis) -> Style {
    Style::TextEmphasis(value.normalize())
}
/// The "text-emphasis-color" property.
pub fn text_emphasis_color(value: impl input::TextEmphasisColor) -> Style {
    Style::TextEmphasisColor(value.normalize())
}
/// The "text-emphasis-position" property.
pub fn text_emphasis_position(value: impl input::TextEmphasisPosition) -> Style {
    Style::TextEmphasisPosition(value.normalize())
}
/// The "text-emphasis-style" property.
pub fn text_emphasis_style(value: impl input::TextEmphasisStyle) -> Style {
    Style::TextEmphasisStyle(value.normalize())
}
/// The "text-indent" property.
pub fn text_indent(value: impl input::TextIndent) -> Style {
    Style::TextIndent(value.normalize())
}
/// The "text-justify" property.
pub fn text_justify(value: impl input::TextJustify) -> Style {
    Style::TextJustify(value.normalize())
}
/// The "text-orientation" property.
pub fn text_orientation(value: impl input::TextOrientation) -> Style {
    Style::TextOrientation(value.normalize())
}
/// The "text-overflow" property.
pub fn text_overflow(value: impl input::TextOverflow) -> Style {
    Style::TextOverflow(value.normalize())
}
/// The "text-shadow" property.
pub fn text_shadow(value: impl input::TextShadow) -> Style {
    Style::TextShadow(value.normalize())
}
/// The "text-transform" property.
pub fn text_transform(value: impl input::TextTransform) -> Style {
    Style::TextTransform(value.normalize())
}
/// The "text-underline-position" property.
pub fn text_underline_position(value: impl input::TextUnderlinePosition) -> Style {
    Style::TextUnderlinePosition(value.normalize())
}
/// The "top" property.
pub fn top(value: impl input::Top) -> Style {
    Style::Top(value.normalize())
}
/// The "transform" property.
pub fn transform(value: impl input::Transform) -> Style {
    Style::Transform(value.normalize())
}
/// The "transform-box" property.
pub fn transform_box(value: impl input::TransformBox) -> Style {
    Style::TransformBox(value.normalize())
}
/// The "transform-origin" property.
pub fn transform_origin(value: impl input::TransformOrigin) -> Style {
    Style::TransformOrigin(value.normalize())
}
/// The "transition" property.
pub fn transition(value: impl input::Transition) -> Style {
    Style::Transition(value.normalize())
}
/// The "transition-delay" property.
pub fn transition_delay(value: impl input::TransitionDelay) -> Style {
    Style::TransitionDelay(value.normalize())
}
/// The "transition-duration" property.
pub fn transition_duration(value: impl input::TransitionDuration) -> Style {
    Style::TransitionDuration(value.normalize())
}
/// The "transition-property" property.
pub fn transition_property(value: impl input::TransitionProperty) -> Style {
    Style::TransitionProperty(value.normalize())
}
/// The "transition-timing-function" property.
pub fn transition_timing_function(value: impl input::TransitionTimingFunction) -> Style {
    Style::TransitionTimingFunction(value.normalize())
}
/// The "unicode-bidi" property.
pub fn unicode_bidi(value: impl input::UnicodeBidi) -> Style {
    Style::UnicodeBidi(value.normalize())
}
/// The "vertical-align" property.
pub fn vertical_align(value: impl input::VerticalAlign) -> Style {
    Style::VerticalAlign(value.normalize())
}
/// The "visibility" property.
pub fn visibility(value: impl input::Visibility) -> Style {
    Style::Visibility(value.normalize())
}
/// The "voice-balance" property.
pub fn voice_balance(value: impl input::VoiceBalance) -> Style {
    Style::VoiceBalance(value.normalize())
}
/// The "voice-duration" property.
pub fn voice_duration(value: impl input::VoiceDuration) -> Style {
    Style::VoiceDuration(value.normalize())
}
/// The "voice-family" property.
pub fn voice_family(value: impl input::VoiceFamily) -> Style {
    Style::VoiceFamily(value.normalize())
}
/// The "voice-pitch" property.
pub fn voice_pitch(value: impl input::VoicePitch) -> Style {
    Style::VoicePitch(value.normalize())
}
/// The "voice-range" property.
pub fn voice_range(value: impl input::VoiceRange) -> Style {
    Style::VoiceRange(value.normalize())
}
/// The "voice-rate" property.
pub fn voice_rate(value: impl input::VoiceRate) -> Style {
    Style::VoiceRate(value.normalize())
}
/// The "voice-stress" property.
pub fn voice_stress(value: impl input::VoiceStress) -> Style {
    Style::VoiceStress(value.normalize())
}
/// The "voice-volume" property.
pub fn voice_volume(value: impl input::VoiceVolume) -> Style {
    Style::VoiceVolume(value.normalize())
}
/// The "volume" property.
pub fn volume(value: impl input::Volume) -> Style {
    Style::Volume(value.normalize())
}
/// The "white-space" property.
pub fn white_space(value: impl input::WhiteSpace) -> Style {
    Style::WhiteSpace(value.normalize())
}
/// The "widows" property.
pub fn widows(value: impl input::Widows) -> Style {
    Style::Widows(value.normalize())
}
/// The "width" property.
pub fn width(value: impl input::Width) -> Style {
    Style::Width(value.normalize())
}
/// The "will-change" property.
pub fn will_change(value: impl input::WillChange) -> Style {
    Style::WillChange(value.normalize())
}
/// The "word-break" property.
pub fn word_break(value: impl input::WordBreak) -> Style {
    Style::WordBreak(value.normalize())
}
/// The "word-spacing" property.
pub fn word_spacing(value: impl input::WordSpacing) -> Style {
    Style::WordSpacing(value.normalize())
}
/// The "word-wrap" property.
pub fn word_wrap(value: impl input::WordWrap) -> Style {
    Style::WordWrap(value.normalize())
}
/// The "writing-mode" property.
pub fn writing_mode(value: impl input::WritingMode) -> Style {
    Style::WritingMode(value.normalize())
}
/// The "z-index" property.
pub fn z_index(value: impl input::ZIndex) -> Style {
    Style::ZIndex(value.normalize())
}