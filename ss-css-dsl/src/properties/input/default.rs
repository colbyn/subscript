use crate::properties::output;

/// The "align-content" property.
pub trait AlignContent {
    fn normalize(self) -> output::AlignContent;
}
/// The "align-items" property.
pub trait AlignItems {
    fn normalize(self) -> output::AlignItems;
}
/// The "align-self" property.
pub trait AlignSelf {
    fn normalize(self) -> output::AlignSelf;
}
/// The "all" property.
pub trait All {
    fn normalize(self) -> output::All;
}
/// The "animation" property.
pub trait Animation {
    fn normalize(self) -> output::Animation;
}
/// The "animation-delay" property.
pub trait AnimationDelay {
    fn normalize(self) -> output::AnimationDelay;
}
/// The "animation-direction" property.
pub trait AnimationDirection {
    fn normalize(self) -> output::AnimationDirection;
}
/// The "animation-duration" property.
pub trait AnimationDuration {
    fn normalize(self) -> output::AnimationDuration;
}
/// The "animation-fill-mode" property.
pub trait AnimationFillMode {
    fn normalize(self) -> output::AnimationFillMode;
}
/// The "animation-iteration-count" property.
pub trait AnimationIterationCount {
    fn normalize(self) -> output::AnimationIterationCount;
}
/// The "animation-name" property.
pub trait AnimationName {
    fn normalize(self) -> output::AnimationName;
}
/// The "animation-play-state" property.
pub trait AnimationPlayState {
    fn normalize(self) -> output::AnimationPlayState;
}
/// The "animation-timing-function" property.
pub trait AnimationTimingFunction {
    fn normalize(self) -> output::AnimationTimingFunction;
}
/// The "azimuth" property.
pub trait Azimuth {
    fn normalize(self) -> output::Azimuth;
}
/// The "background" property.
pub trait Background {
    fn normalize(self) -> output::Background;
}
/// The "background-attachment" property.
pub trait BackgroundAttachment {
    fn normalize(self) -> output::BackgroundAttachment;
}
/// The "background-blend-mode" property.
pub trait BackgroundBlendMode {
    fn normalize(self) -> output::BackgroundBlendMode;
}
/// The "background-clip" property.
pub trait BackgroundClip {
    fn normalize(self) -> output::BackgroundClip;
}
/// The "background-color" property.
pub trait BackgroundColor {
    fn normalize(self) -> output::BackgroundColor;
}
/// The "background-image" property.
pub trait BackgroundImage {
    fn normalize(self) -> output::BackgroundImage;
}
/// The "background-origin" property.
pub trait BackgroundOrigin {
    fn normalize(self) -> output::BackgroundOrigin;
}
/// The "background-position" property.
pub trait BackgroundPosition {
    fn normalize(self) -> output::BackgroundPosition;
}
/// The "background-repeat" property.
pub trait BackgroundRepeat {
    fn normalize(self) -> output::BackgroundRepeat;
}
/// The "background-size" property.
pub trait BackgroundSize {
    fn normalize(self) -> output::BackgroundSize;
}
/// The "border" property.
pub trait Border {
    fn normalize(self) -> output::Border;
}
/// The "border-bottom" property.
pub trait BorderBottom {
    fn normalize(self) -> output::BorderBottom;
}
/// The "border-bottom-color" property.
pub trait BorderBottomColor {
    fn normalize(self) -> output::BorderBottomColor;
}
/// The "border-bottom-left-radius" property.
pub trait BorderBottomLeftRadius {
    fn normalize(self) -> output::BorderBottomLeftRadius;
}
/// The "border-bottom-right-radius" property.
pub trait BorderBottomRightRadius {
    fn normalize(self) -> output::BorderBottomRightRadius;
}
/// The "border-bottom-style" property.
pub trait BorderBottomStyle {
    fn normalize(self) -> output::BorderBottomStyle;
}
/// The "border-bottom-width" property.
pub trait BorderBottomWidth {
    fn normalize(self) -> output::BorderBottomWidth;
}
/// The "border-collapse" property.
pub trait BorderCollapse {
    fn normalize(self) -> output::BorderCollapse;
}
/// The "border-color" property.
pub trait BorderColor {
    fn normalize(self) -> output::BorderColor;
}
/// The "border-image" property.
pub trait BorderImage {
    fn normalize(self) -> output::BorderImage;
}
/// The "border-image-outset" property.
pub trait BorderImageOutset {
    fn normalize(self) -> output::BorderImageOutset;
}
/// The "border-image-repeat" property.
pub trait BorderImageRepeat {
    fn normalize(self) -> output::BorderImageRepeat;
}
/// The "border-image-slice" property.
pub trait BorderImageSlice {
    fn normalize(self) -> output::BorderImageSlice;
}
/// The "border-image-source" property.
pub trait BorderImageSource {
    fn normalize(self) -> output::BorderImageSource;
}
/// The "border-image-width" property.
pub trait BorderImageWidth {
    fn normalize(self) -> output::BorderImageWidth;
}
/// The "border-left" property.
pub trait BorderLeft {
    fn normalize(self) -> output::BorderLeft;
}
/// The "border-left-color" property.
pub trait BorderLeftColor {
    fn normalize(self) -> output::BorderLeftColor;
}
/// The "border-left-style" property.
pub trait BorderLeftStyle {
    fn normalize(self) -> output::BorderLeftStyle;
}
/// The "border-left-width" property.
pub trait BorderLeftWidth {
    fn normalize(self) -> output::BorderLeftWidth;
}
/// The "border-radius" property.
pub trait BorderRadius {
    fn normalize(self) -> output::BorderRadius;
}
/// The "border-right" property.
pub trait BorderRight {
    fn normalize(self) -> output::BorderRight;
}
/// The "border-right-color" property.
pub trait BorderRightColor {
    fn normalize(self) -> output::BorderRightColor;
}
/// The "border-right-style" property.
pub trait BorderRightStyle {
    fn normalize(self) -> output::BorderRightStyle;
}
/// The "border-right-width" property.
pub trait BorderRightWidth {
    fn normalize(self) -> output::BorderRightWidth;
}
/// The "border-spacing" property.
pub trait BorderSpacing {
    fn normalize(self) -> output::BorderSpacing;
}
/// The "border-style" property.
pub trait BorderStyle {
    fn normalize(self) -> output::BorderStyle;
}
/// The "border-top" property.
pub trait BorderTop {
    fn normalize(self) -> output::BorderTop;
}
/// The "border-top-color" property.
pub trait BorderTopColor {
    fn normalize(self) -> output::BorderTopColor;
}
/// The "border-top-left-radius" property.
pub trait BorderTopLeftRadius {
    fn normalize(self) -> output::BorderTopLeftRadius;
}
/// The "border-top-right-radius" property.
pub trait BorderTopRightRadius {
    fn normalize(self) -> output::BorderTopRightRadius;
}
/// The "border-top-style" property.
pub trait BorderTopStyle {
    fn normalize(self) -> output::BorderTopStyle;
}
/// The "border-top-width" property.
pub trait BorderTopWidth {
    fn normalize(self) -> output::BorderTopWidth;
}
/// The "border-width" property.
pub trait BorderWidth {
    fn normalize(self) -> output::BorderWidth;
}
/// The "bottom" property.
pub trait Bottom {
    fn normalize(self) -> output::Bottom;
}
/// The "box-decoration-break" property.
pub trait BoxDecorationBreak {
    fn normalize(self) -> output::BoxDecorationBreak;
}
/// The "box-shadow" property.
pub trait BoxShadow {
    fn normalize(self) -> output::BoxShadow;
}
/// The "box-sizing" property.
pub trait BoxSizing {
    fn normalize(self) -> output::BoxSizing;
}
/// The "break-after" property.
pub trait BreakAfter {
    fn normalize(self) -> output::BreakAfter;
}
/// The "break-before" property.
pub trait BreakBefore {
    fn normalize(self) -> output::BreakBefore;
}
/// The "break-inside" property.
pub trait BreakInside {
    fn normalize(self) -> output::BreakInside;
}
/// The "caption-side" property.
pub trait CaptionSide {
    fn normalize(self) -> output::CaptionSide;
}
/// The "caret-color" property.
pub trait CaretColor {
    fn normalize(self) -> output::CaretColor;
}
/// The "clear" property.
pub trait Clear {
    fn normalize(self) -> output::Clear;
}
/// The "clip" property.
pub trait Clip {
    fn normalize(self) -> output::Clip;
}
/// The "clip-path" property.
pub trait ClipPath {
    fn normalize(self) -> output::ClipPath;
}
/// The "clip-rule" property.
pub trait ClipRule {
    fn normalize(self) -> output::ClipRule;
}
/// The "color" property.
pub trait Color {
    fn normalize(self) -> output::Color;
}
/// The "color-interpolation-filters" property.
pub trait ColorInterpolationFilters {
    fn normalize(self) -> output::ColorInterpolationFilters;
}
/// The "column-count" property.
pub trait ColumnCount {
    fn normalize(self) -> output::ColumnCount;
}
/// The "column-fill" property.
pub trait ColumnFill {
    fn normalize(self) -> output::ColumnFill;
}
/// The "column-gap" property.
pub trait ColumnGap {
    fn normalize(self) -> output::ColumnGap;
}
/// The "column-rule" property.
pub trait ColumnRule {
    fn normalize(self) -> output::ColumnRule;
}
/// The "column-rule-color" property.
pub trait ColumnRuleColor {
    fn normalize(self) -> output::ColumnRuleColor;
}
/// The "column-rule-style" property.
pub trait ColumnRuleStyle {
    fn normalize(self) -> output::ColumnRuleStyle;
}
/// The "column-rule-width" property.
pub trait ColumnRuleWidth {
    fn normalize(self) -> output::ColumnRuleWidth;
}
/// The "columns" property.
pub trait Columns {
    fn normalize(self) -> output::Columns;
}
/// The "column-span" property.
pub trait ColumnSpan {
    fn normalize(self) -> output::ColumnSpan;
}
/// The "column-width" property.
pub trait ColumnWidth {
    fn normalize(self) -> output::ColumnWidth;
}
/// The "contain" property.
pub trait Contain {
    fn normalize(self) -> output::Contain;
}
/// The "content" property.
pub trait Content {
    fn normalize(self) -> output::Content;
}
/// The "counter-increment" property.
pub trait CounterIncrement {
    fn normalize(self) -> output::CounterIncrement;
}
/// The "counter-reset" property.
pub trait CounterReset {
    fn normalize(self) -> output::CounterReset;
}
/// The "cue" property.
pub trait Cue {
    fn normalize(self) -> output::Cue;
}
/// The "cue-after" property.
pub trait CueAfter {
    fn normalize(self) -> output::CueAfter;
}
/// The "cue-before" property.
pub trait CueBefore {
    fn normalize(self) -> output::CueBefore;
}
/// The "cursor" property.
pub trait Cursor {
    fn normalize(self) -> output::Cursor;
}
/// The "direction" property.
pub trait Direction {
    fn normalize(self) -> output::Direction;
}
/// The "elevation" property.
pub trait Elevation {
    fn normalize(self) -> output::Elevation;
}
/// The "empty-cells" property.
pub trait EmptyCells {
    fn normalize(self) -> output::EmptyCells;
}
/// The "filter" property.
pub trait Filter {
    fn normalize(self) -> output::Filter;
}
/// The "flex" property.
pub trait Flex {
    fn normalize(self) -> output::Flex;
}
/// The "flex-basis" property.
pub trait FlexBasis {
    fn normalize(self) -> output::FlexBasis;
}
/// The "flex-direction" property.
pub trait FlexDirection {
    fn normalize(self) -> output::FlexDirection;
}
/// The "flex-flow" property.
pub trait FlexFlow {
    fn normalize(self) -> output::FlexFlow;
}
/// The "flex-grow" property.
pub trait FlexGrow {
    fn normalize(self) -> output::FlexGrow;
}
/// The "flex-shrink" property.
pub trait FlexShrink {
    fn normalize(self) -> output::FlexShrink;
}
/// The "flex-wrap" property.
pub trait FlexWrap {
    fn normalize(self) -> output::FlexWrap;
}
/// The "float" property.
pub trait Float {
    fn normalize(self) -> output::Float;
}
/// The "flood-color" property.
pub trait FloodColor {
    fn normalize(self) -> output::FloodColor;
}
/// The "flood-opacity" property.
pub trait FloodOpacity {
    fn normalize(self) -> output::FloodOpacity;
}
/// The "font" property.
pub trait Font {
    fn normalize(self) -> output::Font;
}
/// The "font-family" property.
pub trait FontFamily {
    fn normalize(self) -> output::FontFamily;
}
/// The "font-feature-settings" property.
pub trait FontFeatureSettings {
    fn normalize(self) -> output::FontFeatureSettings;
}
/// The "font-kerning" property.
pub trait FontKerning {
    fn normalize(self) -> output::FontKerning;
}
/// The "font-size" property.
pub trait FontSize {
    fn normalize(self) -> output::FontSize;
}
/// The "font-size-adjust" property.
pub trait FontSizeAdjust {
    fn normalize(self) -> output::FontSizeAdjust;
}
/// The "font-stretch" property.
pub trait FontStretch {
    fn normalize(self) -> output::FontStretch;
}
/// The "font-style" property.
pub trait FontStyle {
    fn normalize(self) -> output::FontStyle;
}
/// The "font-synthesis" property.
pub trait FontSynthesis {
    fn normalize(self) -> output::FontSynthesis;
}
/// The "font-variant" property.
pub trait FontVariant {
    fn normalize(self) -> output::FontVariant;
}
/// The "font-variant-caps" property.
pub trait FontVariantCaps {
    fn normalize(self) -> output::FontVariantCaps;
}
/// The "font-variant-east-asian" property.
pub trait FontVariantEastAsian {
    fn normalize(self) -> output::FontVariantEastAsian;
}
/// The "font-variant-ligatures" property.
pub trait FontVariantLigatures {
    fn normalize(self) -> output::FontVariantLigatures;
}
/// The "font-variant-numeric" property.
pub trait FontVariantNumeric {
    fn normalize(self) -> output::FontVariantNumeric;
}
/// The "font-variant-position" property.
pub trait FontVariantPosition {
    fn normalize(self) -> output::FontVariantPosition;
}
/// The "font-weight" property.
pub trait FontWeight {
    fn normalize(self) -> output::FontWeight;
}
/// The "gap" property.
pub trait Gap {
    fn normalize(self) -> output::Gap;
}
/// The "globalcompositeoperation" property.
pub trait Globalcompositeoperation {
    fn normalize(self) -> output::Globalcompositeoperation;
}
/// The "glyph-orientation-vertical" property.
pub trait GlyphOrientationVertical {
    fn normalize(self) -> output::GlyphOrientationVertical;
}
/// The "grid" property.
pub trait Grid {
    fn normalize(self) -> output::Grid;
}
/// The "grid-area" property.
pub trait GridArea {
    fn normalize(self) -> output::GridArea;
}
/// The "grid-auto-columns" property.
pub trait GridAutoColumns {
    fn normalize(self) -> output::GridAutoColumns;
}
/// The "grid-auto-flow" property.
pub trait GridAutoFlow {
    fn normalize(self) -> output::GridAutoFlow;
}
/// The "grid-auto-rows" property.
pub trait GridAutoRows {
    fn normalize(self) -> output::GridAutoRows;
}
/// The "grid-column" property.
pub trait GridColumn {
    fn normalize(self) -> output::GridColumn;
}
/// The "grid-column-end" property.
pub trait GridColumnEnd {
    fn normalize(self) -> output::GridColumnEnd;
}
/// The "grid-column-gap" property.
pub trait GridColumnGap {
    fn normalize(self) -> output::GridColumnGap;
}
/// The "grid-column-start" property.
pub trait GridColumnStart {
    fn normalize(self) -> output::GridColumnStart;
}
/// The "grid-gap" property.
pub trait GridGap {
    fn normalize(self) -> output::GridGap;
}
/// The "grid-row" property.
pub trait GridRow {
    fn normalize(self) -> output::GridRow;
}
/// The "grid-row-end" property.
pub trait GridRowEnd {
    fn normalize(self) -> output::GridRowEnd;
}
/// The "grid-row-gap" property.
pub trait GridRowGap {
    fn normalize(self) -> output::GridRowGap;
}
/// The "grid-row-start" property.
pub trait GridRowStart {
    fn normalize(self) -> output::GridRowStart;
}
/// The "grid-template" property.
pub trait GridTemplate {
    fn normalize(self) -> output::GridTemplate;
}
/// The "grid-template-areas" property.
pub trait GridTemplateAreas {
    fn normalize(self) -> output::GridTemplateAreas;
}
/// The "grid-template-columns" property.
pub trait GridTemplateColumns {
    fn normalize(self) -> output::GridTemplateColumns;
}
/// The "grid-template-rows" property.
pub trait GridTemplateRows {
    fn normalize(self) -> output::GridTemplateRows;
}
/// The "hanging-punctuation" property.
pub trait HangingPunctuation {
    fn normalize(self) -> output::HangingPunctuation;
}
/// The "hyphens" property.
pub trait Hyphens {
    fn normalize(self) -> output::Hyphens;
}
/// The "image-orientation" property.
pub trait ImageOrientation {
    fn normalize(self) -> output::ImageOrientation;
}
/// The "image-rendering" property.
pub trait ImageRendering {
    fn normalize(self) -> output::ImageRendering;
}
/// The "image-resolution" property.
pub trait ImageResolution {
    fn normalize(self) -> output::ImageResolution;
}
/// The "isolation" property.
pub trait Isolation {
    fn normalize(self) -> output::Isolation;
}
/// The "justify-content" property.
pub trait JustifyContent {
    fn normalize(self) -> output::JustifyContent;
}
/// The "justify-items" property.
pub trait JustifyItems {
    fn normalize(self) -> output::JustifyItems;
}
/// The "justify-self" property.
pub trait JustifySelf {
    fn normalize(self) -> output::JustifySelf;
}
/// The "left" property.
pub trait Left {
    fn normalize(self) -> output::Left;
}
/// The "letter-spacing" property.
pub trait LetterSpacing {
    fn normalize(self) -> output::LetterSpacing;
}
/// The "lighting-color" property.
pub trait LightingColor {
    fn normalize(self) -> output::LightingColor;
}
/// The "line-break" property.
pub trait LineBreak {
    fn normalize(self) -> output::LineBreak;
}
/// The "line-height" property.
pub trait LineHeight {
    fn normalize(self) -> output::LineHeight;
}
/// The "list-style" property.
pub trait ListStyle {
    fn normalize(self) -> output::ListStyle;
}
/// The "list-style-image" property.
pub trait ListStyleImage {
    fn normalize(self) -> output::ListStyleImage;
}
/// The "list-style-position" property.
pub trait ListStylePosition {
    fn normalize(self) -> output::ListStylePosition;
}
/// The "list-style-type" property.
pub trait ListStyleType {
    fn normalize(self) -> output::ListStyleType;
}
/// The "margin" property.
pub trait Margin {
    fn normalize(self) -> output::Margin;
}
/// The "margin-bottom" property.
pub trait MarginBottom {
    fn normalize(self) -> output::MarginBottom;
}
/// The "margin-left" property.
pub trait MarginLeft {
    fn normalize(self) -> output::MarginLeft;
}
/// The "margin-right" property.
pub trait MarginRight {
    fn normalize(self) -> output::MarginRight;
}
/// The "margin-top" property.
pub trait MarginTop {
    fn normalize(self) -> output::MarginTop;
}
/// The "mask" property.
pub trait Mask {
    fn normalize(self) -> output::Mask;
}
/// The "mask-border" property.
pub trait MaskBorder {
    fn normalize(self) -> output::MaskBorder;
}
/// The "mask-border-mode" property.
pub trait MaskBorderMode {
    fn normalize(self) -> output::MaskBorderMode;
}
/// The "mask-border-outset" property.
pub trait MaskBorderOutset {
    fn normalize(self) -> output::MaskBorderOutset;
}
/// The "mask-border-repeat" property.
pub trait MaskBorderRepeat {
    fn normalize(self) -> output::MaskBorderRepeat;
}
/// The "mask-border-slice" property.
pub trait MaskBorderSlice {
    fn normalize(self) -> output::MaskBorderSlice;
}
/// The "mask-border-source" property.
pub trait MaskBorderSource {
    fn normalize(self) -> output::MaskBorderSource;
}
/// The "mask-border-width" property.
pub trait MaskBorderWidth {
    fn normalize(self) -> output::MaskBorderWidth;
}
/// The "mask-clip" property.
pub trait MaskClip {
    fn normalize(self) -> output::MaskClip;
}
/// The "mask-composite" property.
pub trait MaskComposite {
    fn normalize(self) -> output::MaskComposite;
}
/// The "mask-image" property.
pub trait MaskImage {
    fn normalize(self) -> output::MaskImage;
}
/// The "mask-mode" property.
pub trait MaskMode {
    fn normalize(self) -> output::MaskMode;
}
/// The "mask-origin" property.
pub trait MaskOrigin {
    fn normalize(self) -> output::MaskOrigin;
}
/// The "mask-position" property.
pub trait MaskPosition {
    fn normalize(self) -> output::MaskPosition;
}
/// The "mask-repeat" property.
pub trait MaskRepeat {
    fn normalize(self) -> output::MaskRepeat;
}
/// The "mask-size" property.
pub trait MaskSize {
    fn normalize(self) -> output::MaskSize;
}
/// The "mask-type" property.
pub trait MaskType {
    fn normalize(self) -> output::MaskType;
}
/// The "mix-blend-mode" property.
pub trait MixBlendMode {
    fn normalize(self) -> output::MixBlendMode;
}
/// The "object-fit" property.
pub trait ObjectFit {
    fn normalize(self) -> output::ObjectFit;
}
/// The "object-position" property.
pub trait ObjectPosition {
    fn normalize(self) -> output::ObjectPosition;
}
/// The "opacity" property.
pub trait Opacity {
    fn normalize(self) -> output::Opacity;
}
/// The "order" property.
pub trait Order {
    fn normalize(self) -> output::Order;
}
/// The "orphans" property.
pub trait Orphans {
    fn normalize(self) -> output::Orphans;
}
/// The "outline" property.
pub trait Outline {
    fn normalize(self) -> output::Outline;
}
/// The "outline-color" property.
pub trait OutlineColor {
    fn normalize(self) -> output::OutlineColor;
}
/// The "outline-offset" property.
pub trait OutlineOffset {
    fn normalize(self) -> output::OutlineOffset;
}
/// The "outline-style" property.
pub trait OutlineStyle {
    fn normalize(self) -> output::OutlineStyle;
}
/// The "outline-width" property.
pub trait OutlineWidth {
    fn normalize(self) -> output::OutlineWidth;
}
/// The "overflow" property.
pub trait Overflow {
    fn normalize(self) -> output::Overflow;
}
/// The "overflow-wrap" property.
pub trait OverflowWrap {
    fn normalize(self) -> output::OverflowWrap;
}
/// The "page-break-after" property.
pub trait PageBreakAfter {
    fn normalize(self) -> output::PageBreakAfter;
}
/// The "page-break-before" property.
pub trait PageBreakBefore {
    fn normalize(self) -> output::PageBreakBefore;
}
/// The "page-break-inside" property.
pub trait PageBreakInside {
    fn normalize(self) -> output::PageBreakInside;
}
/// The "pause" property.
pub trait Pause {
    fn normalize(self) -> output::Pause;
}
/// The "pause-after" property.
pub trait PauseAfter {
    fn normalize(self) -> output::PauseAfter;
}
/// The "pause-before" property.
pub trait PauseBefore {
    fn normalize(self) -> output::PauseBefore;
}
/// The "pitch" property.
pub trait Pitch {
    fn normalize(self) -> output::Pitch;
}
/// The "pitch-range" property.
pub trait PitchRange {
    fn normalize(self) -> output::PitchRange;
}
/// The "place-content" property.
pub trait PlaceContent {
    fn normalize(self) -> output::PlaceContent;
}
/// The "place-items" property.
pub trait PlaceItems {
    fn normalize(self) -> output::PlaceItems;
}
/// The "place-self" property.
pub trait PlaceSelf {
    fn normalize(self) -> output::PlaceSelf;
}
/// The "play-during" property.
pub trait PlayDuring {
    fn normalize(self) -> output::PlayDuring;
}
/// The "position" property.
pub trait Position {
    fn normalize(self) -> output::Position;
}
/// The "quotes" property.
pub trait Quotes {
    fn normalize(self) -> output::Quotes;
}
/// The "resize" property.
pub trait Resize {
    fn normalize(self) -> output::Resize;
}
/// The "rest" property.
pub trait Rest {
    fn normalize(self) -> output::Rest;
}
/// The "rest-after" property.
pub trait RestAfter {
    fn normalize(self) -> output::RestAfter;
}
/// The "rest-before" property.
pub trait RestBefore {
    fn normalize(self) -> output::RestBefore;
}
/// The "richness" property.
pub trait Richness {
    fn normalize(self) -> output::Richness;
}
/// The "right" property.
pub trait Right {
    fn normalize(self) -> output::Right;
}
/// The "row-gap" property.
pub trait RowGap {
    fn normalize(self) -> output::RowGap;
}
/// The "scroll-margin" property.
pub trait ScrollMargin {
    fn normalize(self) -> output::ScrollMargin;
}
/// The "scroll-margin-block" property.
pub trait ScrollMarginBlock {
    fn normalize(self) -> output::ScrollMarginBlock;
}
/// The "scroll-margin-block-end" property.
pub trait ScrollMarginBlockEnd {
    fn normalize(self) -> output::ScrollMarginBlockEnd;
}
/// The "scroll-margin-block-start" property.
pub trait ScrollMarginBlockStart {
    fn normalize(self) -> output::ScrollMarginBlockStart;
}
/// The "scroll-margin-bottom" property.
pub trait ScrollMarginBottom {
    fn normalize(self) -> output::ScrollMarginBottom;
}
/// The "scroll-margin-inline" property.
pub trait ScrollMarginInline {
    fn normalize(self) -> output::ScrollMarginInline;
}
/// The "scroll-margin-inline-end" property.
pub trait ScrollMarginInlineEnd {
    fn normalize(self) -> output::ScrollMarginInlineEnd;
}
/// The "scroll-margin-inline-start" property.
pub trait ScrollMarginInlineStart {
    fn normalize(self) -> output::ScrollMarginInlineStart;
}
/// The "scroll-margin-left" property.
pub trait ScrollMarginLeft {
    fn normalize(self) -> output::ScrollMarginLeft;
}
/// The "scroll-margin-right" property.
pub trait ScrollMarginRight {
    fn normalize(self) -> output::ScrollMarginRight;
}
/// The "scroll-margin-top" property.
pub trait ScrollMarginTop {
    fn normalize(self) -> output::ScrollMarginTop;
}
/// The "scroll-padding" property.
pub trait ScrollPadding {
    fn normalize(self) -> output::ScrollPadding;
}
/// The "scroll-padding-block" property.
pub trait ScrollPaddingBlock {
    fn normalize(self) -> output::ScrollPaddingBlock;
}
/// The "scroll-padding-block-end" property.
pub trait ScrollPaddingBlockEnd {
    fn normalize(self) -> output::ScrollPaddingBlockEnd;
}
/// The "scroll-padding-block-start" property.
pub trait ScrollPaddingBlockStart {
    fn normalize(self) -> output::ScrollPaddingBlockStart;
}
/// The "scroll-padding-bottom" property.
pub trait ScrollPaddingBottom {
    fn normalize(self) -> output::ScrollPaddingBottom;
}
/// The "scroll-padding-inline" property.
pub trait ScrollPaddingInline {
    fn normalize(self) -> output::ScrollPaddingInline;
}
/// The "scroll-padding-inline-end" property.
pub trait ScrollPaddingInlineEnd {
    fn normalize(self) -> output::ScrollPaddingInlineEnd;
}
/// The "scroll-padding-inline-start" property.
pub trait ScrollPaddingInlineStart {
    fn normalize(self) -> output::ScrollPaddingInlineStart;
}
/// The "scroll-padding-left" property.
pub trait ScrollPaddingLeft {
    fn normalize(self) -> output::ScrollPaddingLeft;
}
/// The "scroll-padding-right" property.
pub trait ScrollPaddingRight {
    fn normalize(self) -> output::ScrollPaddingRight;
}
/// The "scroll-padding-top" property.
pub trait ScrollPaddingTop {
    fn normalize(self) -> output::ScrollPaddingTop;
}
/// The "scroll-snap-align" property.
pub trait ScrollSnapAlign {
    fn normalize(self) -> output::ScrollSnapAlign;
}
/// The "scroll-snap-stop" property.
pub trait ScrollSnapStop {
    fn normalize(self) -> output::ScrollSnapStop;
}
/// The "scroll-snap-type" property.
pub trait ScrollSnapType {
    fn normalize(self) -> output::ScrollSnapType;
}
/// The "shape-image-threshold" property.
pub trait ShapeImageThreshold {
    fn normalize(self) -> output::ShapeImageThreshold;
}
/// The "shape-margin" property.
pub trait ShapeMargin {
    fn normalize(self) -> output::ShapeMargin;
}
/// The "shape-outside" property.
pub trait ShapeOutside {
    fn normalize(self) -> output::ShapeOutside;
}
/// The "speak" property.
pub trait Speak {
    fn normalize(self) -> output::Speak;
}
/// The "speak-as" property.
pub trait SpeakAs {
    fn normalize(self) -> output::SpeakAs;
}
/// The "speak-header" property.
pub trait SpeakHeader {
    fn normalize(self) -> output::SpeakHeader;
}
/// The "speak-numeral" property.
pub trait SpeakNumeral {
    fn normalize(self) -> output::SpeakNumeral;
}
/// The "speak-punctuation" property.
pub trait SpeakPunctuation {
    fn normalize(self) -> output::SpeakPunctuation;
}
/// The "speech-rate" property.
pub trait SpeechRate {
    fn normalize(self) -> output::SpeechRate;
}
/// The "stress" property.
pub trait Stress {
    fn normalize(self) -> output::Stress;
}
/// The "table-layout" property.
pub trait TableLayout {
    fn normalize(self) -> output::TableLayout;
}
/// The "tab-size" property.
pub trait TabSize {
    fn normalize(self) -> output::TabSize;
}
/// The "text-align" property.
pub trait TextAlign {
    fn normalize(self) -> output::TextAlign;
}
/// The "text-align-all" property.
pub trait TextAlignAll {
    fn normalize(self) -> output::TextAlignAll;
}
/// The "text-align-last" property.
pub trait TextAlignLast {
    fn normalize(self) -> output::TextAlignLast;
}
/// The "text-combine-upright" property.
pub trait TextCombineUpright {
    fn normalize(self) -> output::TextCombineUpright;
}
/// The "text-decoration" property.
pub trait TextDecoration {
    fn normalize(self) -> output::TextDecoration;
}
/// The "text-decoration-color" property.
pub trait TextDecorationColor {
    fn normalize(self) -> output::TextDecorationColor;
}
/// The "text-decoration-line" property.
pub trait TextDecorationLine {
    fn normalize(self) -> output::TextDecorationLine;
}
/// The "text-decoration-style" property.
pub trait TextDecorationStyle {
    fn normalize(self) -> output::TextDecorationStyle;
}
/// The "text-emphasis" property.
pub trait TextEmphasis {
    fn normalize(self) -> output::TextEmphasis;
}
/// The "text-emphasis-color" property.
pub trait TextEmphasisColor {
    fn normalize(self) -> output::TextEmphasisColor;
}
/// The "text-emphasis-position" property.
pub trait TextEmphasisPosition {
    fn normalize(self) -> output::TextEmphasisPosition;
}
/// The "text-emphasis-style" property.
pub trait TextEmphasisStyle {
    fn normalize(self) -> output::TextEmphasisStyle;
}
/// The "text-indent" property.
pub trait TextIndent {
    fn normalize(self) -> output::TextIndent;
}
/// The "text-justify" property.
pub trait TextJustify {
    fn normalize(self) -> output::TextJustify;
}
/// The "text-orientation" property.
pub trait TextOrientation {
    fn normalize(self) -> output::TextOrientation;
}
/// The "text-overflow" property.
pub trait TextOverflow {
    fn normalize(self) -> output::TextOverflow;
}
/// The "text-shadow" property.
pub trait TextShadow {
    fn normalize(self) -> output::TextShadow;
}
/// The "text-transform" property.
pub trait TextTransform {
    fn normalize(self) -> output::TextTransform;
}
/// The "text-underline-position" property.
pub trait TextUnderlinePosition {
    fn normalize(self) -> output::TextUnderlinePosition;
}
/// The "top" property.
pub trait Top {
    fn normalize(self) -> output::Top;
}
/// The "transform" property.
pub trait Transform {
    fn normalize(self) -> output::Transform;
}
/// The "transform-box" property.
pub trait TransformBox {
    fn normalize(self) -> output::TransformBox;
}
/// The "transform-origin" property.
pub trait TransformOrigin {
    fn normalize(self) -> output::TransformOrigin;
}
/// The "transition" property.
pub trait Transition {
    fn normalize(self) -> output::Transition;
}
/// The "transition-delay" property.
pub trait TransitionDelay {
    fn normalize(self) -> output::TransitionDelay;
}
/// The "transition-duration" property.
pub trait TransitionDuration {
    fn normalize(self) -> output::TransitionDuration;
}
/// The "transition-property" property.
pub trait TransitionProperty {
    fn normalize(self) -> output::TransitionProperty;
}
/// The "transition-timing-function" property.
pub trait TransitionTimingFunction {
    fn normalize(self) -> output::TransitionTimingFunction;
}
/// The "unicode-bidi" property.
pub trait UnicodeBidi {
    fn normalize(self) -> output::UnicodeBidi;
}
/// The "vertical-align" property.
pub trait VerticalAlign {
    fn normalize(self) -> output::VerticalAlign;
}
/// The "visibility" property.
pub trait Visibility {
    fn normalize(self) -> output::Visibility;
}
/// The "voice-balance" property.
pub trait VoiceBalance {
    fn normalize(self) -> output::VoiceBalance;
}
/// The "voice-duration" property.
pub trait VoiceDuration {
    fn normalize(self) -> output::VoiceDuration;
}
/// The "voice-family" property.
pub trait VoiceFamily {
    fn normalize(self) -> output::VoiceFamily;
}
/// The "voice-pitch" property.
pub trait VoicePitch {
    fn normalize(self) -> output::VoicePitch;
}
/// The "voice-range" property.
pub trait VoiceRange {
    fn normalize(self) -> output::VoiceRange;
}
/// The "voice-rate" property.
pub trait VoiceRate {
    fn normalize(self) -> output::VoiceRate;
}
/// The "voice-stress" property.
pub trait VoiceStress {
    fn normalize(self) -> output::VoiceStress;
}
/// The "voice-volume" property.
pub trait VoiceVolume {
    fn normalize(self) -> output::VoiceVolume;
}
/// The "volume" property.
pub trait Volume {
    fn normalize(self) -> output::Volume;
}
/// The "white-space" property.
pub trait WhiteSpace {
    fn normalize(self) -> output::WhiteSpace;
}
/// The "widows" property.
pub trait Widows {
    fn normalize(self) -> output::Widows;
}
/// The "will-change" property.
pub trait WillChange {
    fn normalize(self) -> output::WillChange;
}
/// The "word-break" property.
pub trait WordBreak {
    fn normalize(self) -> output::WordBreak;
}
/// The "word-spacing" property.
pub trait WordSpacing {
    fn normalize(self) -> output::WordSpacing;
}
/// The "word-wrap" property.
pub trait WordWrap {
    fn normalize(self) -> output::WordWrap;
}
/// The "writing-mode" property.
pub trait WritingMode {
    fn normalize(self) -> output::WritingMode;
}
/// The "z-index" property.
pub trait ZIndex {
    fn normalize(self) -> output::ZIndex;
}