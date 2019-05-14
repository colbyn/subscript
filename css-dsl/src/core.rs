use std::iter::FromIterator;
use std::hash::{Hash};
use std::collections::*;
pub use crate::values;
pub use crate::properties;

///////////////////////////////////////////////////////////////////////////////
// MISCELLANEOUS
///////////////////////////////////////////////////////////////////////////////

pub type NodeAttrId = String;

///////////////////////////////////////////////////////////////////////////////
// SYNTAX RENDERING
///////////////////////////////////////////////////////////////////////////////

pub fn render_stylesheets(stylesheets: Vec<Stylesheet>) -> String {
    let mut global_styling: Vec<String> = Vec::new();
    let mut global_media_queries = GlobalMediaQueries::new();
    for Stylesheet{node_id, locals, medias, states} in stylesheets {
        fn render_locals(node_id: &str, locals: Vec<Style>) -> String {
            let body = locals
                .into_iter()
                .map(|x| format!("{};", x.css_syntax()))
                .collect::<Vec<String>>()
                .join("\n");
            let result = format!(
                "[{node_id}] {{{body}}}",
                node_id=node_id,
                body=body,
            );
            result
        }
        // FIRST
        global_styling.push(render_locals(&node_id, locals));
        // SECOND
        for state in states {
            global_styling.push(
                state.css_decl_syntax(&node_id)
            );
        }
        for media in medias {
            global_media_queries.insert(&node_id, &media);
        }
    }
    // THIRD
    global_styling.push(global_media_queries.css_syntax());
    // DONE
    global_styling.join("\n")
}


///////////////////////////////////////////////////////////////////////////////
// GLOBAL-MEDIA-QUERIES
///////////////////////////////////////////////////////////////////////////////

/// Helper type for rendering into syntactic form.
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalMediaQueries {
    pub entries: HashMap<BTreeSet<Style>, Vec<MediaQueryChild>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MediaQueryChild {
    node_attr_id: String,
    body: Vec<Style>
}

impl GlobalMediaQueries {
    pub fn new() -> Self {
        GlobalMediaQueries {
            entries: HashMap::new()
        }
    }
    pub fn insert(&mut self, node_attr_id: &str, media_query: &MediaSelector) {
        let MediaSelector {header, body} = media_query.clone();
        let header = BTreeSet::from_iter(header.into_iter());
        match self.entries.get_mut(&header) {
            Some(value) => {
                value.push(MediaQueryChild {
                    node_attr_id: String::from(node_attr_id),
                    body: body
                });
            }
            None => {
                self.entries.insert(header, vec![MediaQueryChild {
                    node_attr_id: String::from(node_attr_id),
                    body: body
                }]);
            }
        }
    }
    pub fn css_syntax(&self) -> String {
        fn render_media_child(x: MediaQueryChild) -> String {
            let MediaQueryChild {node_attr_id, body} = x;
            let body = body
                .into_iter()
                .map(|x| format!("{};", x.css_syntax()))
                .collect::<Vec<String>>()
                .join("\n");
            let result = format!(
                "[{node_id}] {{{body}}}",
                node_id=node_attr_id,
                body=body,
            );
            result
        }
        fn render_media_query(header: BTreeSet<Style>, body: Vec<MediaQueryChild>) -> String {
            let media_selector = format!(
                "@media {header}",
                header=header
                    .into_iter()
                    .map(|x| format!("({})", x.css_syntax()))
                    .collect::<Vec<String>>()
                    .join(" and ")
            );
            let body = body
                .into_iter()
                .map(|x| render_media_child(x))
                .collect::<Vec<String>>()
                .join("\n\t");
            let result = format!(
                "{media_selector} {{{body}}}",
                media_selector=media_selector,
                body=body,
            );
            result
        }
        self.entries
            .clone()
            .into_iter()
            .filter_map(|(k, v)| {
                let is_valid = {
                    !k.is_empty()
                };
                if is_valid {
                    Some(render_media_query(k, v))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}


///////////////////////////////////////////////////////////////////////////////
// BASICS
///////////////////////////////////////////////////////////////////////////////

pub trait CssRuleSyntax {
    fn css_syntax(&self) -> String;
}

pub trait CssDeclSyntax {
    fn css_decl_syntax(&self, node_attr_id: &str) -> String;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct MediaSelector {
    pub header: Vec<Style>,
    pub body: Vec<Style>,
}

impl CssDeclSyntax for MediaSelector {
    fn css_decl_syntax(&self, node_attr_id: &str) -> String {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum StateSelector {
    Active(Vec<Style>),
    After(Vec<Style>),
    Before(Vec<Style>),
    Checked(Vec<Style>),
    Disabled(Vec<Style>),
    Empty(Vec<Style>),
    Enabled(Vec<Style>),
    FirstChild(Vec<Style>),
    FirstLetter(Vec<Style>),
    FirstLine(Vec<Style>),
    Focus(Vec<Style>),
    Hover(Vec<Style>),
    LastChild(Vec<Style>),
    OnlyChild(Vec<Style>),
    Link(Vec<Style>),
    Visited(Vec<Style>),
    SpellingError(Vec<Style>),
    GrammarError(Vec<Style>),
    Selection(Vec<Style>),
    Placeholder(Vec<Style>),
    Marker(Vec<Style>),
    Cue(Vec<Style>),
    Backdrop(Vec<Style>),
}

impl CssDeclSyntax for StateSelector {
    fn css_decl_syntax(&self, node_attr_id: &str) -> String {
        fn render_decl(node_attr_id: &str, selector: &str, body: &Vec<Style>) -> String {
            let body = body
                .clone()
                .into_iter()
                .map(|x| format!("{};", x.css_syntax()))
                .collect::<Vec<String>>()
                .join("\n");
            let result = format!(
                "[{node_id}]{selector} {{{body}}}",
                node_id=node_attr_id,
                selector=selector,
                body=body,
            );
            result
        }
        match self {
            StateSelector::Active(xs) => render_decl(node_attr_id, ":active", xs),
            StateSelector::After(xs) => render_decl(node_attr_id, "::after", xs),
            StateSelector::Before(xs) => render_decl(node_attr_id, "::before", xs),
            StateSelector::Checked(xs) => render_decl(node_attr_id, ":checked", xs),
            StateSelector::Disabled(xs) => render_decl(node_attr_id, ":disabled", xs),
            StateSelector::Empty(xs) => render_decl(node_attr_id, ":empty", xs),
            StateSelector::Enabled(xs) => render_decl(node_attr_id, ":enabled", xs),
            StateSelector::FirstChild(xs) => render_decl(node_attr_id, ":first-child", xs),
            StateSelector::FirstLetter(xs) => render_decl(node_attr_id, "::first-letter", xs),
            StateSelector::FirstLine(xs) => render_decl(node_attr_id, "::first-line", xs),
            StateSelector::Focus(xs) => render_decl(node_attr_id, ":focus", xs),
            StateSelector::Hover(xs) => render_decl(node_attr_id, ":hover", xs),
            StateSelector::LastChild(xs) => render_decl(node_attr_id, ":last-child", xs),
            StateSelector::OnlyChild(xs) => render_decl(node_attr_id, ":only-child", xs),
            StateSelector::Link(xs) => render_decl(node_attr_id, ":link", xs),
            StateSelector::Visited(xs) => render_decl(node_attr_id, ":visited", xs),
            StateSelector::SpellingError(xs) => render_decl(node_attr_id, "::spelling-error", xs),
            StateSelector::GrammarError(xs) => render_decl(node_attr_id, "::grammar-error", xs),
            StateSelector::Selection(xs) => render_decl(node_attr_id, "::selection", xs),
            StateSelector::Placeholder(xs) => render_decl(node_attr_id, "::placeholder", xs),
            StateSelector::Marker(xs) => render_decl(node_attr_id, "::marker", xs),
            StateSelector::Cue(xs) => render_decl(node_attr_id, "::cue", xs),
            StateSelector::Backdrop(xs) => render_decl(node_attr_id, "::backdrop", xs),
        }
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Style {
    AlignContent(properties::output::AlignContent),
    AlignItems(properties::output::AlignItems),
    AlignSelf(properties::output::AlignSelf),
    All(properties::output::All),
    Animation(properties::output::Animation),
    AnimationDelay(properties::output::AnimationDelay),
    AnimationDirection(properties::output::AnimationDirection),
    AnimationDuration(properties::output::AnimationDuration),
    AnimationFillMode(properties::output::AnimationFillMode),
    AnimationIterationCount(properties::output::AnimationIterationCount),
    AnimationName(properties::output::AnimationName),
    AnimationPlayState(properties::output::AnimationPlayState),
    AnimationTimingFunction(properties::output::AnimationTimingFunction),
    Azimuth(properties::output::Azimuth),
    Background(properties::output::Background),
    BackgroundAttachment(properties::output::BackgroundAttachment),
    BackgroundBlendMode(properties::output::BackgroundBlendMode),
    BackgroundClip(properties::output::BackgroundClip),
    BackgroundColor(properties::output::BackgroundColor),
    BackgroundImage(properties::output::BackgroundImage),
    BackgroundOrigin(properties::output::BackgroundOrigin),
    BackgroundPosition(properties::output::BackgroundPosition),
    BackgroundRepeat(properties::output::BackgroundRepeat),
    BackgroundSize(properties::output::BackgroundSize),
    Border(properties::output::Border),
    BorderBottom(properties::output::BorderBottom),
    BorderBottomColor(properties::output::BorderBottomColor),
    BorderBottomLeftRadius(properties::output::BorderBottomLeftRadius),
    BorderBottomRightRadius(properties::output::BorderBottomRightRadius),
    BorderBottomStyle(properties::output::BorderBottomStyle),
    BorderBottomWidth(properties::output::BorderBottomWidth),
    BorderCollapse(properties::output::BorderCollapse),
    BorderColor(properties::output::BorderColor),
    BorderImage(properties::output::BorderImage),
    BorderImageOutset(properties::output::BorderImageOutset),
    BorderImageRepeat(properties::output::BorderImageRepeat),
    BorderImageSlice(properties::output::BorderImageSlice),
    BorderImageSource(properties::output::BorderImageSource),
    BorderImageWidth(properties::output::BorderImageWidth),
    BorderLeft(properties::output::BorderLeft),
    BorderLeftColor(properties::output::BorderLeftColor),
    BorderLeftStyle(properties::output::BorderLeftStyle),
    BorderLeftWidth(properties::output::BorderLeftWidth),
    BorderRadius(properties::output::BorderRadius),
    BorderRight(properties::output::BorderRight),
    BorderRightColor(properties::output::BorderRightColor),
    BorderRightStyle(properties::output::BorderRightStyle),
    BorderRightWidth(properties::output::BorderRightWidth),
    BorderSpacing(properties::output::BorderSpacing),
    BorderStyle(properties::output::BorderStyle),
    BorderTop(properties::output::BorderTop),
    BorderTopColor(properties::output::BorderTopColor),
    BorderTopLeftRadius(properties::output::BorderTopLeftRadius),
    BorderTopRightRadius(properties::output::BorderTopRightRadius),
    BorderTopStyle(properties::output::BorderTopStyle),
    BorderTopWidth(properties::output::BorderTopWidth),
    BorderWidth(properties::output::BorderWidth),
    Bottom(properties::output::Bottom),
    BoxDecorationBreak(properties::output::BoxDecorationBreak),
    BoxShadow(properties::output::BoxShadow),
    BoxSizing(properties::output::BoxSizing),
    BreakAfter(properties::output::BreakAfter),
    BreakBefore(properties::output::BreakBefore),
    BreakInside(properties::output::BreakInside),
    CaptionSide(properties::output::CaptionSide),
    CaretColor(properties::output::CaretColor),
    Clear(properties::output::Clear),
    Clip(properties::output::Clip),
    ClipPath(properties::output::ClipPath),
    ClipRule(properties::output::ClipRule),
    Color(properties::output::Color),
    ColorInterpolationFilters(properties::output::ColorInterpolationFilters),
    ColumnCount(properties::output::ColumnCount),
    ColumnFill(properties::output::ColumnFill),
    ColumnGap(properties::output::ColumnGap),
    ColumnRule(properties::output::ColumnRule),
    ColumnRuleColor(properties::output::ColumnRuleColor),
    ColumnRuleStyle(properties::output::ColumnRuleStyle),
    ColumnRuleWidth(properties::output::ColumnRuleWidth),
    Columns(properties::output::Columns),
    ColumnSpan(properties::output::ColumnSpan),
    ColumnWidth(properties::output::ColumnWidth),
    Contain(properties::output::Contain),
    Content(properties::output::Content),
    CounterIncrement(properties::output::CounterIncrement),
    CounterReset(properties::output::CounterReset),
    Cue(properties::output::Cue),
    CueAfter(properties::output::CueAfter),
    CueBefore(properties::output::CueBefore),
    Cursor(properties::output::Cursor),
    Direction(properties::output::Direction),
    Display(properties::output::Display),
    Elevation(properties::output::Elevation),
    EmptyCells(properties::output::EmptyCells),
    Filter(properties::output::Filter),
    Flex(properties::output::Flex),
    FlexBasis(properties::output::FlexBasis),
    FlexDirection(properties::output::FlexDirection),
    FlexFlow(properties::output::FlexFlow),
    FlexGrow(properties::output::FlexGrow),
    FlexShrink(properties::output::FlexShrink),
    FlexWrap(properties::output::FlexWrap),
    Float(properties::output::Float),
    FloodColor(properties::output::FloodColor),
    FloodOpacity(properties::output::FloodOpacity),
    Font(properties::output::Font),
    FontFamily(properties::output::FontFamily),
    FontFeatureSettings(properties::output::FontFeatureSettings),
    FontKerning(properties::output::FontKerning),
    FontSize(properties::output::FontSize),
    FontSizeAdjust(properties::output::FontSizeAdjust),
    FontStretch(properties::output::FontStretch),
    FontStyle(properties::output::FontStyle),
    FontSynthesis(properties::output::FontSynthesis),
    FontVariant(properties::output::FontVariant),
    FontVariantCaps(properties::output::FontVariantCaps),
    FontVariantEastAsian(properties::output::FontVariantEastAsian),
    FontVariantLigatures(properties::output::FontVariantLigatures),
    FontVariantNumeric(properties::output::FontVariantNumeric),
    FontVariantPosition(properties::output::FontVariantPosition),
    FontWeight(properties::output::FontWeight),
    Gap(properties::output::Gap),
    Globalcompositeoperation(properties::output::Globalcompositeoperation),
    GlyphOrientationVertical(properties::output::GlyphOrientationVertical),
    Grid(properties::output::Grid),
    GridArea(properties::output::GridArea),
    GridAutoColumns(properties::output::GridAutoColumns),
    GridAutoFlow(properties::output::GridAutoFlow),
    GridAutoRows(properties::output::GridAutoRows),
    GridColumn(properties::output::GridColumn),
    GridColumnEnd(properties::output::GridColumnEnd),
    GridColumnGap(properties::output::GridColumnGap),
    GridColumnStart(properties::output::GridColumnStart),
    GridGap(properties::output::GridGap),
    GridRow(properties::output::GridRow),
    GridRowEnd(properties::output::GridRowEnd),
    GridRowGap(properties::output::GridRowGap),
    GridRowStart(properties::output::GridRowStart),
    GridTemplate(properties::output::GridTemplate),
    GridTemplateAreas(properties::output::GridTemplateAreas),
    GridTemplateColumns(properties::output::GridTemplateColumns),
    GridTemplateRows(properties::output::GridTemplateRows),
    HangingPunctuation(properties::output::HangingPunctuation),
    Height(properties::output::Height),
    Hyphens(properties::output::Hyphens),
    ImageOrientation(properties::output::ImageOrientation),
    ImageRendering(properties::output::ImageRendering),
    ImageResolution(properties::output::ImageResolution),
    Isolation(properties::output::Isolation),
    JustifyContent(properties::output::JustifyContent),
    JustifyItems(properties::output::JustifyItems),
    JustifySelf(properties::output::JustifySelf),
    Left(properties::output::Left),
    LetterSpacing(properties::output::LetterSpacing),
    LightingColor(properties::output::LightingColor),
    LineBreak(properties::output::LineBreak),
    LineHeight(properties::output::LineHeight),
    ListStyle(properties::output::ListStyle),
    ListStyleImage(properties::output::ListStyleImage),
    ListStylePosition(properties::output::ListStylePosition),
    ListStyleType(properties::output::ListStyleType),
    Margin(properties::output::Margin),
    MarginBottom(properties::output::MarginBottom),
    MarginLeft(properties::output::MarginLeft),
    MarginRight(properties::output::MarginRight),
    MarginTop(properties::output::MarginTop),
    Mask(properties::output::Mask),
    MaskBorder(properties::output::MaskBorder),
    MaskBorderMode(properties::output::MaskBorderMode),
    MaskBorderOutset(properties::output::MaskBorderOutset),
    MaskBorderRepeat(properties::output::MaskBorderRepeat),
    MaskBorderSlice(properties::output::MaskBorderSlice),
    MaskBorderSource(properties::output::MaskBorderSource),
    MaskBorderWidth(properties::output::MaskBorderWidth),
    MaskClip(properties::output::MaskClip),
    MaskComposite(properties::output::MaskComposite),
    MaskImage(properties::output::MaskImage),
    MaskMode(properties::output::MaskMode),
    MaskOrigin(properties::output::MaskOrigin),
    MaskPosition(properties::output::MaskPosition),
    MaskRepeat(properties::output::MaskRepeat),
    MaskSize(properties::output::MaskSize),
    MaskType(properties::output::MaskType),
    MaxHeight(properties::output::MaxHeight),
    MaxWidth(properties::output::MaxWidth),
    MinHeight(properties::output::MinHeight),
    MinWidth(properties::output::MinWidth),
    MixBlendMode(properties::output::MixBlendMode),
    ObjectFit(properties::output::ObjectFit),
    ObjectPosition(properties::output::ObjectPosition),
    Opacity(properties::output::Opacity),
    Order(properties::output::Order),
    Orphans(properties::output::Orphans),
    Outline(properties::output::Outline),
    OutlineColor(properties::output::OutlineColor),
    OutlineOffset(properties::output::OutlineOffset),
    OutlineStyle(properties::output::OutlineStyle),
    OutlineWidth(properties::output::OutlineWidth),
    Overflow(properties::output::Overflow),
    OverflowWrap(properties::output::OverflowWrap),
    Padding(properties::output::Padding),
    PaddingBottom(properties::output::PaddingBottom),
    PaddingLeft(properties::output::PaddingLeft),
    PaddingRight(properties::output::PaddingRight),
    PaddingTop(properties::output::PaddingTop),
    PageBreakAfter(properties::output::PageBreakAfter),
    PageBreakBefore(properties::output::PageBreakBefore),
    PageBreakInside(properties::output::PageBreakInside),
    Pause(properties::output::Pause),
    PauseAfter(properties::output::PauseAfter),
    PauseBefore(properties::output::PauseBefore),
    Pitch(properties::output::Pitch),
    PitchRange(properties::output::PitchRange),
    PlaceContent(properties::output::PlaceContent),
    PlaceItems(properties::output::PlaceItems),
    PlaceSelf(properties::output::PlaceSelf),
    PlayDuring(properties::output::PlayDuring),
    Position(properties::output::Position),
    Quotes(properties::output::Quotes),
    Resize(properties::output::Resize),
    Rest(properties::output::Rest),
    RestAfter(properties::output::RestAfter),
    RestBefore(properties::output::RestBefore),
    Richness(properties::output::Richness),
    Right(properties::output::Right),
    RowGap(properties::output::RowGap),
    ScrollMargin(properties::output::ScrollMargin),
    ScrollMarginBlock(properties::output::ScrollMarginBlock),
    ScrollMarginBlockEnd(properties::output::ScrollMarginBlockEnd),
    ScrollMarginBlockStart(properties::output::ScrollMarginBlockStart),
    ScrollMarginBottom(properties::output::ScrollMarginBottom),
    ScrollMarginInline(properties::output::ScrollMarginInline),
    ScrollMarginInlineEnd(properties::output::ScrollMarginInlineEnd),
    ScrollMarginInlineStart(properties::output::ScrollMarginInlineStart),
    ScrollMarginLeft(properties::output::ScrollMarginLeft),
    ScrollMarginRight(properties::output::ScrollMarginRight),
    ScrollMarginTop(properties::output::ScrollMarginTop),
    ScrollPadding(properties::output::ScrollPadding),
    ScrollPaddingBlock(properties::output::ScrollPaddingBlock),
    ScrollPaddingBlockEnd(properties::output::ScrollPaddingBlockEnd),
    ScrollPaddingBlockStart(properties::output::ScrollPaddingBlockStart),
    ScrollPaddingBottom(properties::output::ScrollPaddingBottom),
    ScrollPaddingInline(properties::output::ScrollPaddingInline),
    ScrollPaddingInlineEnd(properties::output::ScrollPaddingInlineEnd),
    ScrollPaddingInlineStart(properties::output::ScrollPaddingInlineStart),
    ScrollPaddingLeft(properties::output::ScrollPaddingLeft),
    ScrollPaddingRight(properties::output::ScrollPaddingRight),
    ScrollPaddingTop(properties::output::ScrollPaddingTop),
    ScrollSnapAlign(properties::output::ScrollSnapAlign),
    ScrollSnapStop(properties::output::ScrollSnapStop),
    ScrollSnapType(properties::output::ScrollSnapType),
    ShapeImageThreshold(properties::output::ShapeImageThreshold),
    ShapeMargin(properties::output::ShapeMargin),
    ShapeOutside(properties::output::ShapeOutside),
    Speak(properties::output::Speak),
    SpeakAs(properties::output::SpeakAs),
    SpeakHeader(properties::output::SpeakHeader),
    SpeakNumeral(properties::output::SpeakNumeral),
    SpeakPunctuation(properties::output::SpeakPunctuation),
    SpeechRate(properties::output::SpeechRate),
    Stress(properties::output::Stress),
    TableLayout(properties::output::TableLayout),
    TabSize(properties::output::TabSize),
    TextAlign(properties::output::TextAlign),
    TextAlignAll(properties::output::TextAlignAll),
    TextAlignLast(properties::output::TextAlignLast),
    TextCombineUpright(properties::output::TextCombineUpright),
    TextDecoration(properties::output::TextDecoration),
    TextDecorationColor(properties::output::TextDecorationColor),
    TextDecorationLine(properties::output::TextDecorationLine),
    TextDecorationStyle(properties::output::TextDecorationStyle),
    TextEmphasis(properties::output::TextEmphasis),
    TextEmphasisColor(properties::output::TextEmphasisColor),
    TextEmphasisPosition(properties::output::TextEmphasisPosition),
    TextEmphasisStyle(properties::output::TextEmphasisStyle),
    TextIndent(properties::output::TextIndent),
    TextJustify(properties::output::TextJustify),
    TextOrientation(properties::output::TextOrientation),
    TextOverflow(properties::output::TextOverflow),
    TextShadow(properties::output::TextShadow),
    TextTransform(properties::output::TextTransform),
    TextUnderlinePosition(properties::output::TextUnderlinePosition),
    Top(properties::output::Top),
    Transform(properties::output::Transform),
    TransformBox(properties::output::TransformBox),
    TransformOrigin(properties::output::TransformOrigin),
    Transition(properties::output::Transition),
    TransitionDelay(properties::output::TransitionDelay),
    TransitionDuration(properties::output::TransitionDuration),
    TransitionProperty(properties::output::TransitionProperty),
    TransitionTimingFunction(properties::output::TransitionTimingFunction),
    UnicodeBidi(properties::output::UnicodeBidi),
    VerticalAlign(properties::output::VerticalAlign),
    Visibility(properties::output::Visibility),
    VoiceBalance(properties::output::VoiceBalance),
    VoiceDuration(properties::output::VoiceDuration),
    VoiceFamily(properties::output::VoiceFamily),
    VoicePitch(properties::output::VoicePitch),
    VoiceRange(properties::output::VoiceRange),
    VoiceRate(properties::output::VoiceRate),
    VoiceStress(properties::output::VoiceStress),
    VoiceVolume(properties::output::VoiceVolume),
    Volume(properties::output::Volume),
    WhiteSpace(properties::output::WhiteSpace),
    Widows(properties::output::Widows),
    Width(properties::output::Width),
    WillChange(properties::output::WillChange),
    WordBreak(properties::output::WordBreak),
    WordSpacing(properties::output::WordSpacing),
    WordWrap(properties::output::WordWrap),
    WritingMode(properties::output::WritingMode),
    ZIndex(properties::output::ZIndex),
}

impl CssRuleSyntax for Style {
    fn css_syntax(&self) -> String {
        match self {
            Style::Display(x) => x.css_syntax(),
            Style::Padding(x) => x.css_syntax(),
            Style::PaddingBottom(x) => x.css_syntax(),
            Style::PaddingLeft(x) => x.css_syntax(),
            Style::PaddingRight(x) => x.css_syntax(),
            Style::PaddingTop(x) => x.css_syntax(),
            Style::MaxHeight(x) => x.css_syntax(),
            Style::MaxWidth(x) => x.css_syntax(),
            Style::MinHeight(x) => x.css_syntax(),
            Style::MinWidth(x) => x.css_syntax(),
            Style::Width(x) => x.css_syntax(),
            Style::Height(x) => x.css_syntax(),
            
            Style::AlignContent(x) => unimplemented!(),
            Style::AlignItems(x) => unimplemented!(),
            Style::AlignSelf(x) => unimplemented!(),
            Style::All(x) => unimplemented!(),
            Style::Animation(x) => unimplemented!(),
            Style::AnimationDelay(x) => unimplemented!(),
            Style::AnimationDirection(x) => unimplemented!(),
            Style::AnimationDuration(x) => unimplemented!(),
            Style::AnimationFillMode(x) => unimplemented!(),
            Style::AnimationIterationCount(x) => unimplemented!(),
            Style::AnimationName(x) => unimplemented!(),
            Style::AnimationPlayState(x) => unimplemented!(),
            Style::AnimationTimingFunction(x) => unimplemented!(),
            Style::Azimuth(x) => unimplemented!(),
            Style::Background(x) => unimplemented!(),
            Style::BackgroundAttachment(x) => unimplemented!(),
            Style::BackgroundBlendMode(x) => unimplemented!(),
            Style::BackgroundClip(x) => unimplemented!(),
            Style::BackgroundColor(x) => unimplemented!(),
            Style::BackgroundImage(x) => unimplemented!(),
            Style::BackgroundOrigin(x) => unimplemented!(),
            Style::BackgroundPosition(x) => unimplemented!(),
            Style::BackgroundRepeat(x) => unimplemented!(),
            Style::BackgroundSize(x) => unimplemented!(),
            Style::Border(x) => unimplemented!(),
            Style::BorderBottom(x) => unimplemented!(),
            Style::BorderBottomColor(x) => unimplemented!(),
            Style::BorderBottomLeftRadius(x) => unimplemented!(),
            Style::BorderBottomRightRadius(x) => unimplemented!(),
            Style::BorderBottomStyle(x) => unimplemented!(),
            Style::BorderBottomWidth(x) => unimplemented!(),
            Style::BorderCollapse(x) => unimplemented!(),
            Style::BorderColor(x) => unimplemented!(),
            Style::BorderImage(x) => unimplemented!(),
            Style::BorderImageOutset(x) => unimplemented!(),
            Style::BorderImageRepeat(x) => unimplemented!(),
            Style::BorderImageSlice(x) => unimplemented!(),
            Style::BorderImageSource(x) => unimplemented!(),
            Style::BorderImageWidth(x) => unimplemented!(),
            Style::BorderLeft(x) => unimplemented!(),
            Style::BorderLeftColor(x) => unimplemented!(),
            Style::BorderLeftStyle(x) => unimplemented!(),
            Style::BorderLeftWidth(x) => unimplemented!(),
            Style::BorderRadius(x) => unimplemented!(),
            Style::BorderRight(x) => unimplemented!(),
            Style::BorderRightColor(x) => unimplemented!(),
            Style::BorderRightStyle(x) => unimplemented!(),
            Style::BorderRightWidth(x) => unimplemented!(),
            Style::BorderSpacing(x) => unimplemented!(),
            Style::BorderStyle(x) => unimplemented!(),
            Style::BorderTop(x) => unimplemented!(),
            Style::BorderTopColor(x) => unimplemented!(),
            Style::BorderTopLeftRadius(x) => unimplemented!(),
            Style::BorderTopRightRadius(x) => unimplemented!(),
            Style::BorderTopStyle(x) => unimplemented!(),
            Style::BorderTopWidth(x) => unimplemented!(),
            Style::BorderWidth(x) => unimplemented!(),
            Style::Bottom(x) => unimplemented!(),
            Style::BoxDecorationBreak(x) => unimplemented!(),
            Style::BoxShadow(x) => unimplemented!(),
            Style::BoxSizing(x) => unimplemented!(),
            Style::BreakAfter(x) => unimplemented!(),
            Style::BreakBefore(x) => unimplemented!(),
            Style::BreakInside(x) => unimplemented!(),
            Style::CaptionSide(x) => unimplemented!(),
            Style::CaretColor(x) => unimplemented!(),
            Style::Clear(x) => unimplemented!(),
            Style::Clip(x) => unimplemented!(),
            Style::ClipPath(x) => unimplemented!(),
            Style::ClipRule(x) => unimplemented!(),
            Style::Color(x) => unimplemented!(),
            Style::ColorInterpolationFilters(x) => unimplemented!(),
            Style::ColumnCount(x) => unimplemented!(),
            Style::ColumnFill(x) => unimplemented!(),
            Style::ColumnGap(x) => unimplemented!(),
            Style::ColumnRule(x) => unimplemented!(),
            Style::ColumnRuleColor(x) => unimplemented!(),
            Style::ColumnRuleStyle(x) => unimplemented!(),
            Style::ColumnRuleWidth(x) => unimplemented!(),
            Style::Columns(x) => unimplemented!(),
            Style::ColumnSpan(x) => unimplemented!(),
            Style::ColumnWidth(x) => unimplemented!(),
            Style::Contain(x) => unimplemented!(),
            Style::Content(x) => unimplemented!(),
            Style::CounterIncrement(x) => unimplemented!(),
            Style::CounterReset(x) => unimplemented!(),
            Style::Cue(x) => unimplemented!(),
            Style::CueAfter(x) => unimplemented!(),
            Style::CueBefore(x) => unimplemented!(),
            Style::Cursor(x) => unimplemented!(),
            Style::Direction(x) => unimplemented!(),
            Style::Elevation(x) => unimplemented!(),
            Style::EmptyCells(x) => unimplemented!(),
            Style::Filter(x) => unimplemented!(),
            Style::Flex(x) => unimplemented!(),
            Style::FlexBasis(x) => unimplemented!(),
            Style::FlexDirection(x) => unimplemented!(),
            Style::FlexFlow(x) => unimplemented!(),
            Style::FlexGrow(x) => unimplemented!(),
            Style::FlexShrink(x) => unimplemented!(),
            Style::FlexWrap(x) => unimplemented!(),
            Style::Float(x) => unimplemented!(),
            Style::FloodColor(x) => unimplemented!(),
            Style::FloodOpacity(x) => unimplemented!(),
            Style::Font(x) => unimplemented!(),
            Style::FontFamily(x) => unimplemented!(),
            Style::FontFeatureSettings(x) => unimplemented!(),
            Style::FontKerning(x) => unimplemented!(),
            Style::FontSize(x) => unimplemented!(),
            Style::FontSizeAdjust(x) => unimplemented!(),
            Style::FontStretch(x) => unimplemented!(),
            Style::FontStyle(x) => unimplemented!(),
            Style::FontSynthesis(x) => unimplemented!(),
            Style::FontVariant(x) => unimplemented!(),
            Style::FontVariantCaps(x) => unimplemented!(),
            Style::FontVariantEastAsian(x) => unimplemented!(),
            Style::FontVariantLigatures(x) => unimplemented!(),
            Style::FontVariantNumeric(x) => unimplemented!(),
            Style::FontVariantPosition(x) => unimplemented!(),
            Style::FontWeight(x) => unimplemented!(),
            Style::Gap(x) => unimplemented!(),
            Style::Globalcompositeoperation(x) => unimplemented!(),
            Style::GlyphOrientationVertical(x) => unimplemented!(),
            Style::Grid(x) => unimplemented!(),
            Style::GridArea(x) => unimplemented!(),
            Style::GridAutoColumns(x) => unimplemented!(),
            Style::GridAutoFlow(x) => unimplemented!(),
            Style::GridAutoRows(x) => unimplemented!(),
            Style::GridColumn(x) => unimplemented!(),
            Style::GridColumnEnd(x) => unimplemented!(),
            Style::GridColumnGap(x) => unimplemented!(),
            Style::GridColumnStart(x) => unimplemented!(),
            Style::GridGap(x) => unimplemented!(),
            Style::GridRow(x) => unimplemented!(),
            Style::GridRowEnd(x) => unimplemented!(),
            Style::GridRowGap(x) => unimplemented!(),
            Style::GridRowStart(x) => unimplemented!(),
            Style::GridTemplate(x) => unimplemented!(),
            Style::GridTemplateAreas(x) => unimplemented!(),
            Style::GridTemplateColumns(x) => unimplemented!(),
            Style::GridTemplateRows(x) => unimplemented!(),
            Style::HangingPunctuation(x) => unimplemented!(),
            Style::Hyphens(x) => unimplemented!(),
            Style::ImageOrientation(x) => unimplemented!(),
            Style::ImageRendering(x) => unimplemented!(),
            Style::ImageResolution(x) => unimplemented!(),
            Style::Isolation(x) => unimplemented!(),
            Style::JustifyContent(x) => unimplemented!(),
            Style::JustifyItems(x) => unimplemented!(),
            Style::JustifySelf(x) => unimplemented!(),
            Style::Left(x) => unimplemented!(),
            Style::LetterSpacing(x) => unimplemented!(),
            Style::LightingColor(x) => unimplemented!(),
            Style::LineBreak(x) => unimplemented!(),
            Style::LineHeight(x) => unimplemented!(),
            Style::ListStyle(x) => unimplemented!(),
            Style::ListStyleImage(x) => unimplemented!(),
            Style::ListStylePosition(x) => unimplemented!(),
            Style::ListStyleType(x) => unimplemented!(),
            Style::Margin(x) => unimplemented!(),
            Style::MarginBottom(x) => unimplemented!(),
            Style::MarginLeft(x) => unimplemented!(),
            Style::MarginRight(x) => unimplemented!(),
            Style::MarginTop(x) => unimplemented!(),
            Style::Mask(x) => unimplemented!(),
            Style::MaskBorder(x) => unimplemented!(),
            Style::MaskBorderMode(x) => unimplemented!(),
            Style::MaskBorderOutset(x) => unimplemented!(),
            Style::MaskBorderRepeat(x) => unimplemented!(),
            Style::MaskBorderSlice(x) => unimplemented!(),
            Style::MaskBorderSource(x) => unimplemented!(),
            Style::MaskBorderWidth(x) => unimplemented!(),
            Style::MaskClip(x) => unimplemented!(),
            Style::MaskComposite(x) => unimplemented!(),
            Style::MaskImage(x) => unimplemented!(),
            Style::MaskMode(x) => unimplemented!(),
            Style::MaskOrigin(x) => unimplemented!(),
            Style::MaskPosition(x) => unimplemented!(),
            Style::MaskRepeat(x) => unimplemented!(),
            Style::MaskSize(x) => unimplemented!(),
            Style::MaskType(x) => unimplemented!(),
            Style::MixBlendMode(x) => unimplemented!(),
            Style::ObjectFit(x) => unimplemented!(),
            Style::ObjectPosition(x) => unimplemented!(),
            Style::Opacity(x) => unimplemented!(),
            Style::Order(x) => unimplemented!(),
            Style::Orphans(x) => unimplemented!(),
            Style::Outline(x) => unimplemented!(),
            Style::OutlineColor(x) => unimplemented!(),
            Style::OutlineOffset(x) => unimplemented!(),
            Style::OutlineStyle(x) => unimplemented!(),
            Style::OutlineWidth(x) => unimplemented!(),
            Style::Overflow(x) => unimplemented!(),
            Style::OverflowWrap(x) => unimplemented!(),
            Style::PageBreakAfter(x) => unimplemented!(),
            Style::PageBreakBefore(x) => unimplemented!(),
            Style::PageBreakInside(x) => unimplemented!(),
            Style::Pause(x) => unimplemented!(),
            Style::PauseAfter(x) => unimplemented!(),
            Style::PauseBefore(x) => unimplemented!(),
            Style::Pitch(x) => unimplemented!(),
            Style::PitchRange(x) => unimplemented!(),
            Style::PlaceContent(x) => unimplemented!(),
            Style::PlaceItems(x) => unimplemented!(),
            Style::PlaceSelf(x) => unimplemented!(),
            Style::PlayDuring(x) => unimplemented!(),
            Style::Position(x) => unimplemented!(),
            Style::Quotes(x) => unimplemented!(),
            Style::Resize(x) => unimplemented!(),
            Style::Rest(x) => unimplemented!(),
            Style::RestAfter(x) => unimplemented!(),
            Style::RestBefore(x) => unimplemented!(),
            Style::Richness(x) => unimplemented!(),
            Style::Right(x) => unimplemented!(),
            Style::RowGap(x) => unimplemented!(),
            Style::ScrollMargin(x) => unimplemented!(),
            Style::ScrollMarginBlock(x) => unimplemented!(),
            Style::ScrollMarginBlockEnd(x) => unimplemented!(),
            Style::ScrollMarginBlockStart(x) => unimplemented!(),
            Style::ScrollMarginBottom(x) => unimplemented!(),
            Style::ScrollMarginInline(x) => unimplemented!(),
            Style::ScrollMarginInlineEnd(x) => unimplemented!(),
            Style::ScrollMarginInlineStart(x) => unimplemented!(),
            Style::ScrollMarginLeft(x) => unimplemented!(),
            Style::ScrollMarginRight(x) => unimplemented!(),
            Style::ScrollMarginTop(x) => unimplemented!(),
            Style::ScrollPadding(x) => unimplemented!(),
            Style::ScrollPaddingBlock(x) => unimplemented!(),
            Style::ScrollPaddingBlockEnd(x) => unimplemented!(),
            Style::ScrollPaddingBlockStart(x) => unimplemented!(),
            Style::ScrollPaddingBottom(x) => unimplemented!(),
            Style::ScrollPaddingInline(x) => unimplemented!(),
            Style::ScrollPaddingInlineEnd(x) => unimplemented!(),
            Style::ScrollPaddingInlineStart(x) => unimplemented!(),
            Style::ScrollPaddingLeft(x) => unimplemented!(),
            Style::ScrollPaddingRight(x) => unimplemented!(),
            Style::ScrollPaddingTop(x) => unimplemented!(),
            Style::ScrollSnapAlign(x) => unimplemented!(),
            Style::ScrollSnapStop(x) => unimplemented!(),
            Style::ScrollSnapType(x) => unimplemented!(),
            Style::ShapeImageThreshold(x) => unimplemented!(),
            Style::ShapeMargin(x) => unimplemented!(),
            Style::ShapeOutside(x) => unimplemented!(),
            Style::Speak(x) => unimplemented!(),
            Style::SpeakAs(x) => unimplemented!(),
            Style::SpeakHeader(x) => unimplemented!(),
            Style::SpeakNumeral(x) => unimplemented!(),
            Style::SpeakPunctuation(x) => unimplemented!(),
            Style::SpeechRate(x) => unimplemented!(),
            Style::Stress(x) => unimplemented!(),
            Style::TableLayout(x) => unimplemented!(),
            Style::TabSize(x) => unimplemented!(),
            Style::TextAlign(x) => unimplemented!(),
            Style::TextAlignAll(x) => unimplemented!(),
            Style::TextAlignLast(x) => unimplemented!(),
            Style::TextCombineUpright(x) => unimplemented!(),
            Style::TextDecoration(x) => unimplemented!(),
            Style::TextDecorationColor(x) => unimplemented!(),
            Style::TextDecorationLine(x) => unimplemented!(),
            Style::TextDecorationStyle(x) => unimplemented!(),
            Style::TextEmphasis(x) => unimplemented!(),
            Style::TextEmphasisColor(x) => unimplemented!(),
            Style::TextEmphasisPosition(x) => unimplemented!(),
            Style::TextEmphasisStyle(x) => unimplemented!(),
            Style::TextIndent(x) => unimplemented!(),
            Style::TextJustify(x) => unimplemented!(),
            Style::TextOrientation(x) => unimplemented!(),
            Style::TextOverflow(x) => unimplemented!(),
            Style::TextShadow(x) => unimplemented!(),
            Style::TextTransform(x) => unimplemented!(),
            Style::TextUnderlinePosition(x) => unimplemented!(),
            Style::Top(x) => unimplemented!(),
            Style::Transform(x) => unimplemented!(),
            Style::TransformBox(x) => unimplemented!(),
            Style::TransformOrigin(x) => unimplemented!(),
            Style::Transition(x) => unimplemented!(),
            Style::TransitionDelay(x) => unimplemented!(),
            Style::TransitionDuration(x) => unimplemented!(),
            Style::TransitionProperty(x) => unimplemented!(),
            Style::TransitionTimingFunction(x) => unimplemented!(),
            Style::UnicodeBidi(x) => unimplemented!(),
            Style::VerticalAlign(x) => unimplemented!(),
            Style::Visibility(x) => unimplemented!(),
            Style::VoiceBalance(x) => unimplemented!(),
            Style::VoiceDuration(x) => unimplemented!(),
            Style::VoiceFamily(x) => unimplemented!(),
            Style::VoicePitch(x) => unimplemented!(),
            Style::VoiceRange(x) => unimplemented!(),
            Style::VoiceRate(x) => unimplemented!(),
            Style::VoiceStress(x) => unimplemented!(),
            Style::VoiceVolume(x) => unimplemented!(),
            Style::Volume(x) => unimplemented!(),
            Style::WhiteSpace(x) => unimplemented!(),
            Style::Widows(x) => unimplemented!(),
            Style::WillChange(x) => unimplemented!(),
            Style::WordBreak(x) => unimplemented!(),
            Style::WordSpacing(x) => unimplemented!(),
            Style::WordWrap(x) => unimplemented!(),
            Style::WritingMode(x) => unimplemented!(),
            Style::ZIndex(x) => unimplemented!(),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
// STYLESHEET
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Stylesheet {
    pub node_id: NodeAttrId,
    pub locals: Vec<Style>,
    pub medias: Vec<MediaSelector>,
    pub states: Vec<StateSelector>,
}

impl Stylesheet {
    pub fn new(node_id: &str) -> Self {
        Stylesheet {
            node_id: String::from(node_id),
            locals: Vec::new(),
            medias: Vec::new(),
            states: Vec::new(),
        }
    }
    pub fn add_local(&mut self, x: Style) {
        self.locals.push(x);
    }
    pub fn add_locals(&mut self, xs: Vec<Style>) {
        for x in xs {
            self.locals.push(x);
        }
    }
    pub fn add_media(&mut self, x: MediaSelector) {
        self.medias.push(x);
    }
    pub fn add_state(&mut self, x: StateSelector) {
        self.states.push(x);
    }
}

