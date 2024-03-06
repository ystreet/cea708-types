// Copyright (C) 2023 Matthew Waters <matthew@centricular.com>
//
// Licensed under the MIT license <LICENSE-MIT> or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Module for the various [Code] tables available

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CodeError {
    /// Length of data does not match length advertised
    #[error("The length of the data ({actual}) does not match the advertised expected ({expected}) length")]
    LengthMismatch {
        /// The expected size
        expected: usize,
        /// The actual size
        actual: usize,
    },
}

/// Enum representing characters or commands accessible through the [Ext1] byte
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// must be ordered the same as the byte values
pub enum Ext1 {
    TransparentSpace,
    NonBreakingTransparentSpace,
    HorizontalElipses,
    LatinCapitalSWithCaron,
    LatinCapitalLigatureOE,
    FullBlock,
    SingleOpenQuote,
    SingleCloseQuote,
    DoubleOpenQuote,
    DoubleCloseQuote,
    SolidDot,
    TradeMarkSign,
    LatinLowerSWithCaron,
    LatinLowerLigatureOE,
    LatinCapitalYWithDiaeresis,
    Fraction18,
    Fraction38,
    Fraction58,
    Fraction78,
    VerticalBorder,
    UpperRightBorder,
    LowerLeftBorder,
    HorizontalBorder,
    LowerRightBorder,
    UpperLeftBorder,
    ClosedCaptionSign,

    Unknown(Vec<u8>),
}

/// Enum of all possible characters or commands available within [Service](super::Service) block
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// must be ordered the same as the byte values for binary search to be successful
pub enum Code {
    NUL,
    ETX,
    BS,
    FF,
    CR,
    HCR,
    Ext1(Ext1),
    P16(u16),
    // G0
    Space, // 0x20
    ExclamationMark,
    QuotationMark,
    NumberSign,
    DollarSign,
    PercentSign,
    Ampersand,
    Apostrophe,
    LeftParenthesis,
    RightParenthesis,
    Asterisk,
    PlusSign,
    Comma,
    HyphenMinus,
    FullStop,
    Solidus,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Colon,
    SemiColon,
    LessThan,
    Equals,
    GreaterThan,
    QuestionMark,
    CommercialAt,
    LatinCapitalA,
    LatinCapitalB,
    LatinCapitalC,
    LatinCapitalD,
    LatinCapitalE,
    LatinCapitalF,
    LatinCapitalG,
    LatinCapitalH,
    LatinCapitalI,
    LatinCapitalJ,
    LatinCapitalK,
    LatinCapitalL,
    LatinCapitalM,
    LatinCapitalN,
    LatinCapitalO,
    LatinCapitalP,
    LatinCapitalQ,
    LatinCapitalR,
    LatinCapitalS,
    LatinCapitalT,
    LatinCapitalU,
    LatinCapitalV,
    LatinCapitalW,
    LatinCapitalX,
    LatinCapitalY,
    LatinCapitalZ,
    LeftSquareBracket,
    ReverseSolidus,
    RightSquareBracket,
    CircumflexAccent,
    LowLine,
    GraveAccent,
    LatinLowerA,
    LatinLowerB,
    LatinLowerC,
    LatinLowerD,
    LatinLowerE,
    LatinLowerF,
    LatinLowerG,
    LatinLowerH,
    LatinLowerI,
    LatinLowerJ,
    LatinLowerK,
    LatinLowerL,
    LatinLowerM,
    LatinLowerN,
    LatinLowerO,
    LatinLowerP,
    LatinLowerQ,
    LatinLowerR,
    LatinLowerS,
    LatinLowerT,
    LatinLowerU,
    LatinLowerV,
    LatinLowerW,
    LatinLowerX,
    LatinLowerY,
    LatinLowerZ,
    LeftCurlyBracket,
    VerticalLine,
    RightCurlyBracket,
    Tilde,
    MusicalSymbolEighthNote, // 0x7F

    // C1
    SetCurrentWindow0, // 0x80
    SetCurrentWindow1,
    SetCurrentWindow2,
    SetCurrentWindow3,
    SetCurrentWindow4,
    SetCurrentWindow5,
    SetCurrentWindow6,
    SetCurrentWindow7,
    ClearWindows(WindowBits), // 0x88
    DisplayWindows(WindowBits),
    HideWindows(WindowBits),
    ToggleWindows(WindowBits),
    DeleteWindows(WindowBits),
    Delay(u8),
    DelayCancel,
    Reset,
    SetPenAttributes(SetPenAttributesArgs),
    SetPenColor(SetPenColorArgs),
    SetPenLocation(SetPenLocationArgs), // 0x92

    SetWindowAttributes(SetWindowAttributesArgs), // 0x97
    DefineWindow(DefineWindowArgs),               // [0x98, 0x9F]

    // G1
    NonBreakingSpace, // 0xA0
    InvertedExclamationMark,
    CentSign,
    PoundSign,
    GeneralCurrencySign,
    YenSign,
    BrokenVerticalBar,
    SectionSign,
    Umlaut,
    CopyrightSign,
    FeminineOrdinalSign,
    LeftDoubleAngleQuote,
    LogicalNotSign,
    SoftHyphen,
    RegisteredTrademarkSign,
    SpacingMacronLongAccent,
    DegreeSign,
    PlusOrMinusSign,
    Superscript2,
    Superscript3,
    SpacingAccuteAccent,
    MicroSign,
    ParagraphSign,
    MiddleDot,
    SpacingCedilla,
    Superscript1,
    MasculineOrdinalSign,
    RightDoubleAngleQuote,
    Fraction14,
    Fraction12,
    Fraction34,
    InvertedQuestionMark,
    LatinCapitalAWithGrave,
    LatinCapitalAWithAcute,
    LatinCapitalAWithCircumflex,
    LatinCapitalAWithTilde,
    LatinCapitalAWithDiaeresis,
    LatinCapitalAWithRingAbove,
    LatinCapitalAe,
    LatinCapitalCWithCedilla,
    LatinCapitalEWithGrave,
    LatinCapitalEWithAcute,
    LatinCapitalEWithCircumflex,
    LatinCapitalEWithDiaeseris,
    LatinCapitalIWithGrave,
    LatinCapitalIWithAcute,
    LatinCapitalIWithCircumflex,
    LatinCapitalIWithDiaeseris,
    LatinCapitalEth,
    LatinCapitalNWithTilde,
    LatinCapitalOWithGrave,
    LatinCapitalOWithAcute,
    LatinCapitalOWithCircumflex,
    LatinCapitalOWithTilde,
    LatinCapitalOWithDiaeresis,
    MultiplicationSign,
    LatinCapitalOWithStroke,
    LatinCapitalUWithGrave,
    LatinCapitalUWithAcute,
    LatinCapitalUWithCircumflex,
    LatinCapitalUWithDiaeresis,
    LatinCapitalYWithAcute,
    LatinCapitalThorn,
    LatinLowerSharpS,
    LatinLowerAWithGrave,
    LatinLowerAWithAcute,
    LatinLowerAWithCircumflex,
    LatinLowerAWithTilde,
    LatinLowerAWithDiaeresis,
    LatinLowerAWithRingAbove,
    LatinLowerAe,
    LatinLowerCWithCedilla,
    LatinLowerEWithGrave,
    LatinLowerEWithAcute,
    LatinLowerEWithCircumflex,
    LatinLowerEWithDiaeseris,
    LatinLowerIWithGrave,
    LatinLowerIWithAcute,
    LatinLowerIWithCircumflex,
    LatinLowerIWithDiaeseris,
    LatinLowerEth,
    LatinLowerNWithTilde,
    LatinLowerOWithGrave,
    LatinLowerOWithAcute,
    LatinLowerOWithCircumflex,
    LatinLowerOWithTilde,
    LatinLowerOWithDiaeresis,
    DivisionSign,
    LatinLowerOWithStroke,
    LatinLowerUWithGrave,
    LatinLowerUWithAcute,
    LatinLowerUWithCircumflex,
    LatinLowerUWithDiaeresis,
    LatinLowerYWithAcute,
    LatinLowerThorn,
    LatinLowerYWithDiaeresis,
    Unknown(Vec<u8>),
}

/// A collection of 8 Windows (0-7) represented as a bitfield
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowBits(u8);

impl From<u8> for WindowBits {
    fn from(bits: u8) -> Self {
        Self(bits)
    }
}

impl From<[u8; 1]> for WindowBits {
    fn from(bits: [u8; 1]) -> Self {
        Self(bits[0])
    }
}

impl From<WindowBits> for u8 {
    fn from(bits: WindowBits) -> Self {
        bits.0
    }
}

impl From<WindowBits> for [u8; 1] {
    fn from(bits: WindowBits) -> Self {
        [bits.0]
    }
}

impl WindowBits {
    pub const NONE: WindowBits = WindowBits(0x0);
    pub const ZERO: WindowBits = WindowBits(0x01);
    pub const ONE: WindowBits = WindowBits(0x02);
    pub const TWO: WindowBits = WindowBits(0x04);
    pub const THREE: WindowBits = WindowBits(0x08);
    pub const FOUR: WindowBits = WindowBits(0x10);
    pub const FIVE: WindowBits = WindowBits(0x20);
    pub const SIX: WindowBits = WindowBits(0x40);
    pub const SEVEN: WindowBits = WindowBits(0x80);

    pub const fn or(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }

    pub const fn and(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }

    pub const fn not(self) -> Self {
        Self(!self.0)
    }
}

impl std::ops::BitOr for WindowBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for WindowBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::Not for WindowBits {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Debug for WindowBits {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WindowBits(b{:0>8b})", self.0)
    }
}

/// Anchor points
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anchor {
    TopLeft,
    TopMiddle,
    TopRight,
    CenterLeft,
    CenterMiddle,
    CenterRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
    Undefined9,
    Undefined10,
    Undefined11,
    Undefined12,
    Undefined13,
    Undefined14,
    Undefined15,
}

impl From<u8> for Anchor {
    fn from(a: u8) -> Self {
        match a {
            0 => Anchor::TopLeft,
            1 => Anchor::TopMiddle,
            2 => Anchor::TopRight,
            3 => Anchor::CenterLeft,
            4 => Anchor::CenterMiddle,
            5 => Anchor::CenterRight,
            6 => Anchor::BottomLeft,
            7 => Anchor::BottomMiddle,
            8 => Anchor::BottomRight,
            9 => Anchor::Undefined9,
            10 => Anchor::Undefined10,
            11 => Anchor::Undefined11,
            12 => Anchor::Undefined12,
            13 => Anchor::Undefined13,
            14 => Anchor::Undefined14,
            15 => Anchor::Undefined15,
            _ => unreachable!(),
        }
    }
}

impl From<Anchor> for u8 {
    fn from(a: Anchor) -> u8 {
        match a {
            Anchor::TopLeft => 0,
            Anchor::TopMiddle => 1,
            Anchor::TopRight => 2,
            Anchor::CenterLeft => 3,
            Anchor::CenterMiddle => 4,
            Anchor::CenterRight => 5,
            Anchor::BottomLeft => 6,
            Anchor::BottomMiddle => 7,
            Anchor::BottomRight => 8,
            Anchor::Undefined9 => 9,
            Anchor::Undefined10 => 10,
            Anchor::Undefined11 => 11,
            Anchor::Undefined12 => 12,
            Anchor::Undefined13 => 13,
            Anchor::Undefined14 => 14,
            Anchor::Undefined15 => 15,
        }
    }
}

/// Arguments required for the [Code::DefineWindow] command
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DefineWindowArgs {
    pub window_id: u8, // [0, 7]
    pub priority: u8,  // [0, 7]
    pub anchor_point: Anchor,
    pub relative_positioning: bool,
    pub anchor_vertical: u8,   // [0, 255]
    pub anchor_horizontal: u8, // [0, 255]
    pub row_count: u8,         // [0, 11]
    pub column_count: u8,      // [0, 31/41]
    pub row_lock: bool,
    pub column_lock: bool,
    pub visible: bool,
    pub window_style_id: u8, // [0, 7]
    pub pen_style_id: u8,    // [0, 7]
}

impl From<[u8; 6]> for DefineWindowArgs {
    fn from(args: [u8; 6]) -> Self {
        Self {
            window_id: 0, // needs to be filled in later
            priority: args[0] & 0x7,
            anchor_point: ((args[3] & 0xF0) >> 4).into(),
            relative_positioning: (args[1] & 0x80) > 0,
            anchor_vertical: args[1] & 0x7F,
            anchor_horizontal: args[2],
            row_count: args[3] & 0x0F,
            column_count: args[4] & 0x3F,
            row_lock: (args[0] & 0x10) > 0,
            column_lock: (args[0] & 0x08) > 0,
            visible: (args[0] & 0x20) > 0,
            window_style_id: (args[5] & 0x38) >> 3,
            pen_style_id: args[5] & 0x07,
        }
    }
}

impl From<DefineWindowArgs> for [u8; 6] {
    fn from(args: DefineWindowArgs) -> Self {
        [
            args.priority & 0x7
                | u8::from(args.column_lock) << 3
                | u8::from(args.row_lock) << 4
                | u8::from(args.visible) << 5,
            args.anchor_vertical & 0x7F | u8::from(args.relative_positioning) << 7,
            args.anchor_horizontal,
            (args.row_count & 0x0F) | u8::from(args.anchor_point) << 4,
            args.column_count & 0x3F,
            args.pen_style_id & 0x07 | (args.window_style_id & 0x7) << 3,
        ]
    }
}

impl DefineWindowArgs {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        window_id: u8,
        priority: u8,
        anchor_point: Anchor,
        relative_positioning: bool,
        anchor_vertical: u8,
        anchor_horizontal: u8,
        row_count: u8,
        column_count: u8,
        row_lock: bool,
        column_lock: bool,
        visible: bool,
        window_style_id: u8,
        pen_style_id: u8,
    ) -> Self {
        Self {
            window_id,
            priority,
            anchor_point,
            relative_positioning,
            anchor_vertical,
            anchor_horizontal,
            row_count,
            column_count,
            row_lock,
            column_lock,
            visible,
            window_style_id,
            pen_style_id,
        }
    }

    /// Retrieve the default window attributes for this [`DefineWindowArgs`]
    pub fn window_attributes(&self) -> SetWindowAttributesArgs {
        PREDEFINED_WINDOW_STYLES[self.window_style_id as usize - 1]
    }

    /// Retrieve the default pen attributes for this [`DefineWindowArgs`]
    pub fn pen_attributes(&self) -> SetPenAttributesArgs {
        PREDEFINED_PEN_STYLES_ATTRIBUTES[self.pen_style_id as usize - 1]
    }

    /// Retrieve the default pen color for this [`DefineWindowArgs`]
    pub fn pen_color(&self) -> SetPenColorArgs {
        PREDEFINED_PEN_STYLES_COLOR[self.pen_style_id as usize - 1]
    }
}

static PREDEFINED_WINDOW_STYLES: [SetWindowAttributesArgs; 7] = [
    // style 1
    SetWindowAttributesArgs {
        justify: Justify::Left,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: false,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Solid,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style w
    SetWindowAttributesArgs {
        justify: Justify::Left,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: false,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Transparent,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style 3
    SetWindowAttributesArgs {
        justify: Justify::Center,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: false,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Solid,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style 4
    SetWindowAttributesArgs {
        justify: Justify::Left,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: true,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Solid,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style 5
    SetWindowAttributesArgs {
        justify: Justify::Left,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: true,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Transparent,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style 6
    SetWindowAttributesArgs {
        justify: Justify::Center,
        print_direction: Direction::LeftToRight,
        scroll_direction: Direction::BottomToTop,
        wordwrap: true,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Solid,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
    // style 7
    SetWindowAttributesArgs {
        justify: Justify::Left,
        print_direction: Direction::TopToBottom,
        scroll_direction: Direction::RightToLeft,
        wordwrap: false,
        display_effect: DisplayEffect::Snap,
        effect_direction: Direction::LeftToRight,
        effect_speed: 1,
        fill_color: Color::BLACK,
        fill_opacity: Opacity::Solid,
        border_type: BorderType::None,
        border_color: Color::BLACK,
    },
];

static PREDEFINED_PEN_STYLES_ATTRIBUTES: [SetPenAttributesArgs; 7] = [
    // style 1
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::Default,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::None,
    },
    // style 2
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::MonospacedWithSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::None,
    },
    // style 3
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::ProportionallySpacedWithSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::None,
    },
    // style 4
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::MonospacedWithoutSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::None,
    },
    // style 5
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::ProportionallySpacedWithoutSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::None,
    },
    // style 6
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::MonospacedWithoutSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::Uniform,
    },
    // style 7
    SetPenAttributesArgs {
        pen_size: PenSize::Standard,
        font_style: FontStyle::ProportionallySpacedWithoutSerifs,
        text_tag: TextTag::Dialog,
        offset: TextOffset::Normal,
        italics: false,
        underline: false,
        edge_type: EdgeType::Uniform,
    },
];

static PREDEFINED_PEN_STYLES_COLOR: [SetPenColorArgs; 7] = [
    // style 1
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Solid,
        edge_color: Color::BLACK,
    },
    // style 2
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Solid,
        edge_color: Color::BLACK,
    },
    // style 3
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Solid,
        edge_color: Color::BLACK,
    },
    // style 4
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Solid,
        edge_color: Color::BLACK,
    },
    // style 5
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Solid,
        edge_color: Color::BLACK,
    },
    // style 6
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Transparent,
        edge_color: Color::BLACK,
    },
    // style 7
    SetPenColorArgs {
        foreground_color: Color::WHITE,
        foreground_opacity: Opacity::Solid,
        background_color: Color::BLACK,
        background_opacity: Opacity::Transparent,
        edge_color: Color::BLACK,
    },
];

/// Text tustification options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Justify {
    Left,
    Right,
    Center,
    Full,
}

impl From<u8> for Justify {
    fn from(j: u8) -> Self {
        match j {
            0 => Justify::Left,
            1 => Justify::Right,
            2 => Justify::Center,
            3 => Justify::Full,
            _ => unreachable!(),
        }
    }
}

impl From<Justify> for u8 {
    fn from(j: Justify) -> u8 {
        match j {
            Justify::Left => 0,
            Justify::Right => 1,
            Justify::Center => 2,
            Justify::Full => 3,
        }
    }
}

/// Text/Scroll/etc direction options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

impl From<u8> for Direction {
    fn from(d: u8) -> Self {
        match d {
            0 => Direction::LeftToRight,
            1 => Direction::RightToLeft,
            2 => Direction::TopToBottom,
            3 => Direction::BottomToTop,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for u8 {
    fn from(j: Direction) -> u8 {
        match j {
            Direction::LeftToRight => 0,
            Direction::RightToLeft => 1,
            Direction::TopToBottom => 2,
            Direction::BottomToTop => 3,
        }
    }
}

/// Display effect options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DisplayEffect {
    Snap,
    Fade,
    Wipe,
    Undefined,
}

impl From<u8> for DisplayEffect {
    fn from(d: u8) -> Self {
        match d {
            0 => DisplayEffect::Snap,
            1 => DisplayEffect::Fade,
            2 => DisplayEffect::Wipe,
            3 => DisplayEffect::Undefined,
            _ => unreachable!(),
        }
    }
}

impl From<DisplayEffect> for u8 {
    fn from(de: DisplayEffect) -> u8 {
        match de {
            DisplayEffect::Snap => 0,
            DisplayEffect::Fade => 1,
            DisplayEffect::Wipe => 2,
            DisplayEffect::Undefined => 3,
        }
    }
}

/// Opacity options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Opacity {
    Solid,
    Flash,
    Translucent,
    Transparent,
}

impl From<u8> for Opacity {
    fn from(op: u8) -> Opacity {
        match op {
            0 => Opacity::Solid,
            1 => Opacity::Flash,
            2 => Opacity::Translucent,
            3 => Opacity::Transparent,
            _ => unreachable!(),
        }
    }
}

impl From<Opacity> for u8 {
    fn from(op: Opacity) -> u8 {
        match op {
            Opacity::Solid => 0,
            Opacity::Flash => 1,
            Opacity::Translucent => 2,
            Opacity::Transparent => 3,
        }
    }
}

/// Color value options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorValue {
    None,
    OneThird,
    TwoThirds,
    Full,
}

impl From<u8> for ColorValue {
    fn from(val: u8) -> ColorValue {
        match val {
            0 => ColorValue::None,
            1 => ColorValue::OneThird,
            2 => ColorValue::TwoThirds,
            3 => ColorValue::Full,
            _ => unreachable!(),
        }
    }
}

impl From<ColorValue> for u8 {
    fn from(cv: ColorValue) -> u8 {
        match cv {
            ColorValue::None => 0,
            ColorValue::OneThird => 1,
            ColorValue::TwoThirds => 2,
            ColorValue::Full => 3,
        }
    }
}

/// A RGB color
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: ColorValue,
    pub g: ColorValue,
    pub b: ColorValue,
}

impl From<Color> for u8 {
    fn from(c: Color) -> Self {
        u8::from(c.r) << 4 | u8::from(c.g) << 2 | u8::from(c.b)
    }
}

impl From<u8> for Color {
    fn from(c: u8) -> Color {
        Color {
            r: ((c & 0x30) >> 4).into(),
            g: ((c & 0x0C) >> 2).into(),
            b: (c & 0x03).into(),
        }
    }
}

impl Color {
    pub const BLACK: Color = Color::new(ColorValue::None, ColorValue::None, ColorValue::None);
    pub const WHITE: Color = Color::new(ColorValue::Full, ColorValue::Full, ColorValue::Full);
    pub const RED: Color = Color::new(ColorValue::Full, ColorValue::None, ColorValue::None);
    pub const GREEN: Color = Color::new(ColorValue::None, ColorValue::Full, ColorValue::None);
    pub const BLUE: Color = Color::new(ColorValue::None, ColorValue::None, ColorValue::Full);

    pub const fn new(r: ColorValue, g: ColorValue, b: ColorValue) -> Self {
        Self { r, g, b }
    }
}

struct ColorOpacity(Color, Opacity);

impl From<ColorOpacity> for u8 {
    fn from(c_o: ColorOpacity) -> Self {
        u8::from(c_o.1) << 6 | u8::from(c_o.0)
    }
}

impl From<u8> for ColorOpacity {
    fn from(c_o: u8) -> Self {
        Self((c_o & 0x3F).into(), ((c_o & 0xC0) >> 6).into())
    }
}

/// Border options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BorderType {
    None,
    Raised,
    Depressed,
    Uniform,
    ShadowLeft,
    ShadowRight,
    Undefined6,
    Undefined7,
}

impl From<BorderType> for u8 {
    fn from(bt: BorderType) -> Self {
        match bt {
            BorderType::None => 0,
            BorderType::Raised => 1,
            BorderType::Depressed => 2,
            BorderType::Uniform => 3,
            BorderType::ShadowLeft => 4,
            BorderType::ShadowRight => 5,
            BorderType::Undefined6 => 6,
            BorderType::Undefined7 => 7,
        }
    }
}

impl From<u8> for BorderType {
    fn from(bt: u8) -> Self {
        match bt {
            0 => BorderType::None,
            1 => BorderType::Raised,
            2 => BorderType::Depressed,
            3 => BorderType::Uniform,
            4 => BorderType::ShadowLeft,
            5 => BorderType::ShadowRight,
            6 => BorderType::Undefined6,
            7 => BorderType::Undefined7,
            _ => unreachable!(),
        }
    }
}

/// Arguments required for the [Code::SetWindowAttributes] command
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetWindowAttributesArgs {
    pub justify: Justify,
    pub print_direction: Direction,
    pub scroll_direction: Direction,
    pub wordwrap: bool,
    pub display_effect: DisplayEffect,
    pub effect_direction: Direction,
    pub effect_speed: u8, // [1, 15] in units of 500ms
    pub fill_color: Color,
    pub fill_opacity: Opacity,
    pub border_type: BorderType,
    pub border_color: Color,
}

impl From<SetWindowAttributesArgs> for [u8; 4] {
    fn from(args: SetWindowAttributesArgs) -> Self {
        let bt = u8::from(args.border_type);
        [
            ColorOpacity(args.fill_color, args.fill_opacity).into(),
            (bt & 0x3) << 6 | u8::from(args.border_color),
            (bt & 0x4) << 5
                | u8::from(args.wordwrap) << 6
                | u8::from(args.print_direction) << 4
                | u8::from(args.scroll_direction) << 2
                | u8::from(args.justify),
            args.effect_speed << 4
                | u8::from(args.effect_direction) << 2
                | u8::from(args.display_effect),
        ]
    }
}

impl From<[u8; 4]> for SetWindowAttributesArgs {
    fn from(args: [u8; 4]) -> Self {
        let fill: ColorOpacity = args[0].into();
        let border_type = (args[1] & 0xC0) >> 6 | (args[2] & 0x80) >> 5;
        Self {
            justify: (args[2] & 0x03).into(),
            print_direction: ((args[2] & 0x30) >> 4).into(),
            scroll_direction: ((args[2] & 0x0C) >> 2).into(),
            wordwrap: (args[2] & 0x40) > 0,
            display_effect: (args[3] & 0x03).into(),
            effect_direction: ((args[3] & 0x0C) >> 2).into(),
            effect_speed: (args[3] & 0xF0) >> 4,
            fill_color: fill.0,
            fill_opacity: fill.1,
            border_type: border_type.into(),
            border_color: args[1].into(),
        }
    }
}

impl SetWindowAttributesArgs {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        justify: Justify,
        print_direction: Direction,
        scroll_direction: Direction,
        wordwrap: bool,
        display_effect: DisplayEffect,
        effect_direction: Direction,
        effect_speed: u8,
        fill_color: Color,
        fill_opacity: Opacity,
        border_type: BorderType,
        border_color: Color,
    ) -> Self {
        Self {
            justify,
            print_direction,
            scroll_direction,
            wordwrap,
            display_effect,
            effect_direction,
            effect_speed,
            fill_color,
            fill_opacity,
            border_type,
            border_color,
        }
    }
}

/// Pen size options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PenSize {
    Small,
    Standard,
    Large,
    Undefined,
}

impl From<PenSize> for u8 {
    fn from(pen_size: PenSize) -> Self {
        match pen_size {
            PenSize::Small => 0,
            PenSize::Standard => 1,
            PenSize::Large => 2,
            PenSize::Undefined => 3,
        }
    }
}

impl From<u8> for PenSize {
    fn from(pen_size: u8) -> Self {
        match pen_size {
            0 => PenSize::Small,
            1 => PenSize::Standard,
            2 => PenSize::Large,
            3 => PenSize::Undefined,
            _ => unreachable!(),
        }
    }
}

/// Font style options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontStyle {
    Default,
    MonospacedWithSerifs,
    ProportionallySpacedWithSerifs,
    MonospacedWithoutSerifs,
    ProportionallySpacedWithoutSerifs,
    CasualFontType,
    CursiveFontType,
    SmallCapitals,
}

impl From<FontStyle> for u8 {
    fn from(font_style: FontStyle) -> Self {
        match font_style {
            FontStyle::Default => 0,
            FontStyle::MonospacedWithSerifs => 1,
            FontStyle::ProportionallySpacedWithSerifs => 2,
            FontStyle::MonospacedWithoutSerifs => 3,
            FontStyle::ProportionallySpacedWithoutSerifs => 4,
            FontStyle::CasualFontType => 5,
            FontStyle::CursiveFontType => 6,
            FontStyle::SmallCapitals => 7,
        }
    }
}

impl From<u8> for FontStyle {
    fn from(font_style: u8) -> Self {
        match font_style {
            0 => FontStyle::Default,
            1 => FontStyle::MonospacedWithSerifs,
            2 => FontStyle::ProportionallySpacedWithSerifs,
            3 => FontStyle::MonospacedWithoutSerifs,
            4 => FontStyle::ProportionallySpacedWithoutSerifs,
            5 => FontStyle::CasualFontType,
            6 => FontStyle::CursiveFontType,
            7 => FontStyle::SmallCapitals,
            _ => unreachable!(),
        }
    }
}

/// Text tag options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextTag {
    Dialog,
    SourceOrSpeakerId,
    ElectronicallyReproducedVoice,
    DialogInNonPrimaryLanguage,
    Voiceover,
    AudibleTranslation,
    SubtitleTranslation,
    VoiceQualityDescription,
    SongLyrics,
    SoundEffectDescription,
    MusicalScoreDescription,
    Expletive,
    Undefined12,
    Undefined13,
    Undefined14,
    TextNotToBeDisplayed,
}

impl From<TextTag> for u8 {
    fn from(text_tag: TextTag) -> Self {
        match text_tag {
            TextTag::Dialog => 0,
            TextTag::SourceOrSpeakerId => 1,
            TextTag::ElectronicallyReproducedVoice => 2,
            TextTag::DialogInNonPrimaryLanguage => 3,
            TextTag::Voiceover => 4,
            TextTag::AudibleTranslation => 5,
            TextTag::SubtitleTranslation => 6,
            TextTag::VoiceQualityDescription => 7,
            TextTag::SongLyrics => 8,
            TextTag::SoundEffectDescription => 9,
            TextTag::MusicalScoreDescription => 10,
            TextTag::Expletive => 11,
            TextTag::Undefined12 => 12,
            TextTag::Undefined13 => 13,
            TextTag::Undefined14 => 14,
            TextTag::TextNotToBeDisplayed => 15,
        }
    }
}

impl From<u8> for TextTag {
    fn from(text_tag: u8) -> Self {
        match text_tag {
            0 => TextTag::Dialog,
            1 => TextTag::SourceOrSpeakerId,
            2 => TextTag::ElectronicallyReproducedVoice,
            3 => TextTag::DialogInNonPrimaryLanguage,
            4 => TextTag::Voiceover,
            5 => TextTag::AudibleTranslation,
            6 => TextTag::SubtitleTranslation,
            7 => TextTag::VoiceQualityDescription,
            8 => TextTag::SongLyrics,
            9 => TextTag::SoundEffectDescription,
            10 => TextTag::MusicalScoreDescription,
            11 => TextTag::Expletive,
            12 => TextTag::Undefined12,
            13 => TextTag::Undefined13,
            14 => TextTag::Undefined14,
            15 => TextTag::TextNotToBeDisplayed,
            _ => unreachable!(),
        }
    }
}

/// Text offset options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextOffset {
    Subscript,
    Normal,
    Superscript,
    Undefined,
}

impl From<TextOffset> for u8 {
    fn from(text_offset: TextOffset) -> Self {
        match text_offset {
            TextOffset::Subscript => 0,
            TextOffset::Normal => 1,
            TextOffset::Superscript => 2,
            TextOffset::Undefined => 3,
        }
    }
}

impl From<u8> for TextOffset {
    fn from(text_offset: u8) -> Self {
        match text_offset {
            0 => TextOffset::Subscript,
            1 => TextOffset::Normal,
            2 => TextOffset::Superscript,
            3 => TextOffset::Undefined,
            _ => unreachable!(),
        }
    }
}

/// Edge type options
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeType {
    None,
    Raised,
    Depressed,
    Uniform,
    LeftDropShadow,
    RightDropShadow,
    Undefined6,
    Undefined7,
}

impl From<u8> for EdgeType {
    fn from(edge_type: u8) -> Self {
        match edge_type {
            0 => EdgeType::None,
            1 => EdgeType::Raised,
            2 => EdgeType::Depressed,
            3 => EdgeType::Uniform,
            4 => EdgeType::LeftDropShadow,
            5 => EdgeType::RightDropShadow,
            6 => EdgeType::Undefined6,
            7 => EdgeType::Undefined7,
            _ => unreachable!(),
        }
    }
}

impl From<EdgeType> for u8 {
    fn from(edge_type: EdgeType) -> Self {
        match edge_type {
            EdgeType::None => 0,
            EdgeType::Raised => 1,
            EdgeType::Depressed => 2,
            EdgeType::Uniform => 3,
            EdgeType::LeftDropShadow => 4,
            EdgeType::RightDropShadow => 5,
            EdgeType::Undefined6 => 6,
            EdgeType::Undefined7 => 7,
        }
    }
}

/// Arguments required for the [Code::SetPenAttributes] command
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetPenAttributesArgs {
    pub pen_size: PenSize,
    pub font_style: FontStyle,
    pub text_tag: TextTag,
    pub offset: TextOffset,
    pub italics: bool,
    pub underline: bool,
    pub edge_type: EdgeType,
}

impl From<SetPenAttributesArgs> for [u8; 2] {
    fn from(args: SetPenAttributesArgs) -> Self {
        [
            u8::from(args.pen_size) | u8::from(args.offset) << 2 | u8::from(args.text_tag) << 4,
            u8::from(args.font_style)
                | u8::from(args.edge_type) << 3
                | u8::from(args.underline) << 6
                | u8::from(args.italics) << 7,
        ]
    }
}

impl From<[u8; 2]> for SetPenAttributesArgs {
    fn from(args: [u8; 2]) -> Self {
        Self {
            pen_size: (args[0] & 0x3).into(),
            font_style: (args[1] & 0x07).into(),
            text_tag: ((args[0] & 0xF0) >> 4).into(),
            offset: ((args[0] & 0x0C) >> 2).into(),
            italics: (args[1] & 0x80) > 0,
            underline: (args[1] & 0x40) > 0,
            edge_type: ((args[1] & 0x38) >> 3).into(),
        }
    }
}

impl SetPenAttributesArgs {
    pub const fn new(
        pen_size: PenSize,
        font_style: FontStyle,
        text_tag: TextTag,
        text_offset: TextOffset,
        italics: bool,
        underline: bool,
        edge_type: EdgeType,
    ) -> Self {
        Self {
            pen_size,
            font_style,
            text_tag,
            offset: text_offset,
            italics,
            underline,
            edge_type,
        }
    }
}

#[derive(Debug, Clone)]
struct CodeMap<'a> {
    pub cea708_bytes: &'a [u8],
    pub code: Code,
    pub utf8: Option<char>,
}

/// Arguments required for the [Code::SetPenColor] command
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetPenColorArgs {
    pub foreground_color: Color,
    pub foreground_opacity: Opacity,
    pub background_color: Color,
    pub background_opacity: Opacity,
    pub edge_color: Color,
}

impl SetPenColorArgs {
    pub const fn new(
        foreground_color: Color,
        foreground_opacity: Opacity,
        background_color: Color,
        background_opacity: Opacity,
        edge_color: Color,
    ) -> Self {
        Self {
            foreground_color,
            foreground_opacity,
            background_color,
            background_opacity,
            edge_color,
        }
    }
}

impl From<[u8; 3]> for SetPenColorArgs {
    fn from(data: [u8; 3]) -> Self {
        let foreground: ColorOpacity = data[0].into();
        let background: ColorOpacity = data[1].into();
        let edge: Color = data[2].into();
        Self {
            foreground_color: foreground.0,
            foreground_opacity: foreground.1,
            background_color: background.0,
            background_opacity: background.1,
            edge_color: edge,
        }
    }
}

impl From<SetPenColorArgs> for [u8; 3] {
    fn from(data: SetPenColorArgs) -> Self {
        [
            ColorOpacity(data.foreground_color, data.foreground_opacity).into(),
            ColorOpacity(data.background_color, data.background_opacity).into(),
            data.edge_color.into(),
        ]
    }
}

/// Arguments required for the [Code::SetPenLocation] command
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetPenLocationArgs {
    pub row: u8,    // [0, 14]
    pub column: u8, // [0, 31/41]
}

impl SetPenLocationArgs {
    pub const fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }
}

impl From<[u8; 2]> for SetPenLocationArgs {
    fn from(data: [u8; 2]) -> Self {
        Self {
            row: data[0] & 0x0F,
            column: data[1] & 0x3F,
        }
    }
}

impl From<SetPenLocationArgs> for [u8; 2] {
    fn from(data: SetPenLocationArgs) -> Self {
        [data.row & 0x0F, data.column & 0x3F]
    }
}

macro_rules! code_map_bytes {
    ($bytes:expr, $code:expr, $utf8:expr) => {
        CodeMap {
            cea708_bytes: &$bytes,
            code: $code,
            utf8: $utf8,
        }
    };
}

macro_rules! code_map_single_byte {
    ($byte:expr, $code:expr, $utf8:expr) => {
        code_map_bytes!([$byte], $code, $utf8)
    };
}

// needs to be sorted by bytes and Code
static CODE_MAP_TABLE: [CodeMap; 234] = [
    code_map_single_byte!(0x00, Code::NUL, None),
    code_map_single_byte!(0x03, Code::ETX, None),
    code_map_single_byte!(0x08, Code::BS, None),
    code_map_single_byte!(0x0C, Code::FF, None),
    code_map_single_byte!(0x0D, Code::CR, None),
    code_map_single_byte!(0x0E, Code::HCR, None),
    code_map_bytes!([0x10, 0x20], Code::Ext1(Ext1::TransparentSpace), None),
    code_map_bytes!(
        [0x10, 0x21],
        Code::Ext1(Ext1::NonBreakingTransparentSpace),
        None
    ),
    code_map_bytes!([0x10, 0x25], Code::Ext1(Ext1::HorizontalElipses), Some('…')),
    code_map_bytes!(
        [0x10, 0x2A],
        Code::Ext1(Ext1::LatinCapitalSWithCaron),
        Some('Š')
    ),
    code_map_bytes!(
        [0x10, 0x2C],
        Code::Ext1(Ext1::LatinCapitalLigatureOE),
        Some('Œ')
    ),
    code_map_bytes!([0x10, 0x30], Code::Ext1(Ext1::FullBlock), Some('█')),
    code_map_bytes!([0x10, 0x31], Code::Ext1(Ext1::SingleOpenQuote), Some('‘')),
    code_map_bytes!([0x10, 0x32], Code::Ext1(Ext1::SingleCloseQuote), Some('’')),
    code_map_bytes!([0x10, 0x33], Code::Ext1(Ext1::DoubleOpenQuote), Some('“')),
    code_map_bytes!([0x10, 0x34], Code::Ext1(Ext1::DoubleCloseQuote), Some('”')),
    code_map_bytes!([0x10, 0x35], Code::Ext1(Ext1::SolidDot), None),
    code_map_bytes!([0x10, 0x39], Code::Ext1(Ext1::TradeMarkSign), Some('™')),
    code_map_bytes!(
        [0x10, 0x3A],
        Code::Ext1(Ext1::LatinLowerSWithCaron),
        Some('š')
    ),
    code_map_bytes!(
        [0x10, 0x3C],
        Code::Ext1(Ext1::LatinLowerLigatureOE),
        Some('œ')
    ),
    code_map_bytes!(
        [0x10, 0x3F],
        Code::Ext1(Ext1::LatinCapitalYWithDiaeresis),
        Some('Ÿ')
    ),
    code_map_bytes!([0x10, 0x76], Code::Ext1(Ext1::Fraction18), Some('⅛')),
    code_map_bytes!([0x10, 0x77], Code::Ext1(Ext1::Fraction38), Some('⅜')),
    code_map_bytes!([0x10, 0x78], Code::Ext1(Ext1::Fraction58), Some('⅝')),
    code_map_bytes!([0x10, 0x79], Code::Ext1(Ext1::Fraction78), Some('⅞')),
    code_map_bytes!([0x10, 0x7A], Code::Ext1(Ext1::VerticalBorder), None),
    code_map_bytes!([0x10, 0x7B], Code::Ext1(Ext1::UpperRightBorder), None),
    code_map_bytes!([0x10, 0x7C], Code::Ext1(Ext1::LowerLeftBorder), None),
    code_map_bytes!([0x10, 0x7D], Code::Ext1(Ext1::HorizontalBorder), None),
    code_map_bytes!([0x10, 0x7E], Code::Ext1(Ext1::LowerRightBorder), None),
    code_map_bytes!([0x10, 0x7F], Code::Ext1(Ext1::UpperLeftBorder), None),
    code_map_bytes!([0x10, 0xA0], Code::Ext1(Ext1::ClosedCaptionSign), None),
    code_map_single_byte!(0x20, Code::Space, Some(' ')),
    code_map_single_byte!(0x21, Code::ExclamationMark, Some('!')),
    code_map_single_byte!(0x22, Code::QuotationMark, Some('\"')),
    code_map_single_byte!(0x23, Code::NumberSign, Some('#')),
    code_map_single_byte!(0x24, Code::DollarSign, Some('$')),
    code_map_single_byte!(0x25, Code::PercentSign, Some('%')),
    code_map_single_byte!(0x26, Code::Ampersand, Some('&')),
    code_map_single_byte!(0x27, Code::Apostrophe, Some('\'')),
    code_map_single_byte!(0x28, Code::LeftParenthesis, Some('(')),
    code_map_single_byte!(0x29, Code::RightParenthesis, Some(')')),
    code_map_single_byte!(0x2A, Code::Asterisk, Some('*')),
    code_map_single_byte!(0x2B, Code::PlusSign, Some('+')),
    code_map_single_byte!(0x2C, Code::Comma, Some(',')),
    code_map_single_byte!(0x2D, Code::HyphenMinus, Some('-')),
    code_map_single_byte!(0x2E, Code::FullStop, Some('.')),
    code_map_single_byte!(0x2F, Code::Solidus, Some('/')),
    code_map_single_byte!(0x30, Code::Zero, Some('0')),
    code_map_single_byte!(0x31, Code::One, Some('1')),
    code_map_single_byte!(0x32, Code::Two, Some('2')),
    code_map_single_byte!(0x33, Code::Three, Some('3')),
    code_map_single_byte!(0x34, Code::Four, Some('4')),
    code_map_single_byte!(0x35, Code::Five, Some('5')),
    code_map_single_byte!(0x36, Code::Six, Some('6')),
    code_map_single_byte!(0x37, Code::Seven, Some('7')),
    code_map_single_byte!(0x38, Code::Eight, Some('8')),
    code_map_single_byte!(0x39, Code::Nine, Some('9')),
    code_map_single_byte!(0x3A, Code::Colon, Some(':')),
    code_map_single_byte!(0x3B, Code::SemiColon, Some(';')),
    code_map_single_byte!(0x3C, Code::LessThan, Some('<')),
    code_map_single_byte!(0x3D, Code::Equals, Some('=')),
    code_map_single_byte!(0x3E, Code::GreaterThan, Some('>')),
    code_map_single_byte!(0x3F, Code::QuestionMark, Some('?')),
    code_map_single_byte!(0x40, Code::CommercialAt, Some('@')),
    code_map_single_byte!(0x41, Code::LatinCapitalA, Some('A')),
    code_map_single_byte!(0x42, Code::LatinCapitalB, Some('B')),
    code_map_single_byte!(0x43, Code::LatinCapitalC, Some('C')),
    code_map_single_byte!(0x44, Code::LatinCapitalD, Some('D')),
    code_map_single_byte!(0x45, Code::LatinCapitalE, Some('E')),
    code_map_single_byte!(0x46, Code::LatinCapitalF, Some('F')),
    code_map_single_byte!(0x47, Code::LatinCapitalG, Some('G')),
    code_map_single_byte!(0x48, Code::LatinCapitalH, Some('H')),
    code_map_single_byte!(0x49, Code::LatinCapitalI, Some('I')),
    code_map_single_byte!(0x4A, Code::LatinCapitalJ, Some('J')),
    code_map_single_byte!(0x4B, Code::LatinCapitalK, Some('K')),
    code_map_single_byte!(0x4C, Code::LatinCapitalL, Some('L')),
    code_map_single_byte!(0x4D, Code::LatinCapitalM, Some('M')),
    code_map_single_byte!(0x4E, Code::LatinCapitalN, Some('N')),
    code_map_single_byte!(0x4F, Code::LatinCapitalO, Some('O')),
    code_map_single_byte!(0x50, Code::LatinCapitalP, Some('P')),
    code_map_single_byte!(0x51, Code::LatinCapitalQ, Some('Q')),
    code_map_single_byte!(0x52, Code::LatinCapitalR, Some('R')),
    code_map_single_byte!(0x53, Code::LatinCapitalS, Some('S')),
    code_map_single_byte!(0x54, Code::LatinCapitalT, Some('T')),
    code_map_single_byte!(0x55, Code::LatinCapitalU, Some('U')),
    code_map_single_byte!(0x56, Code::LatinCapitalV, Some('V')),
    code_map_single_byte!(0x57, Code::LatinCapitalW, Some('W')),
    code_map_single_byte!(0x58, Code::LatinCapitalX, Some('X')),
    code_map_single_byte!(0x59, Code::LatinCapitalY, Some('Y')),
    code_map_single_byte!(0x5A, Code::LatinCapitalZ, Some('Z')),
    code_map_single_byte!(0x5B, Code::LeftSquareBracket, Some('[')),
    code_map_single_byte!(0x5C, Code::ReverseSolidus, Some('\\')),
    code_map_single_byte!(0x5D, Code::RightSquareBracket, Some(']')),
    code_map_single_byte!(0x5E, Code::CircumflexAccent, Some('^')),
    code_map_single_byte!(0x5F, Code::LowLine, Some('_')),
    code_map_single_byte!(0x60, Code::GraveAccent, Some('`')),
    code_map_single_byte!(0x61, Code::LatinLowerA, Some('a')),
    code_map_single_byte!(0x62, Code::LatinLowerB, Some('b')),
    code_map_single_byte!(0x63, Code::LatinLowerC, Some('c')),
    code_map_single_byte!(0x64, Code::LatinLowerD, Some('d')),
    code_map_single_byte!(0x65, Code::LatinLowerE, Some('e')),
    code_map_single_byte!(0x66, Code::LatinLowerF, Some('f')),
    code_map_single_byte!(0x67, Code::LatinLowerG, Some('g')),
    code_map_single_byte!(0x68, Code::LatinLowerH, Some('h')),
    code_map_single_byte!(0x69, Code::LatinLowerI, Some('i')),
    code_map_single_byte!(0x6A, Code::LatinLowerJ, Some('j')),
    code_map_single_byte!(0x6B, Code::LatinLowerK, Some('k')),
    code_map_single_byte!(0x6C, Code::LatinLowerL, Some('l')),
    code_map_single_byte!(0x6D, Code::LatinLowerM, Some('m')),
    code_map_single_byte!(0x6E, Code::LatinLowerN, Some('n')),
    code_map_single_byte!(0x6F, Code::LatinLowerO, Some('o')),
    code_map_single_byte!(0x70, Code::LatinLowerP, Some('p')),
    code_map_single_byte!(0x71, Code::LatinLowerQ, Some('q')),
    code_map_single_byte!(0x72, Code::LatinLowerR, Some('r')),
    code_map_single_byte!(0x73, Code::LatinLowerS, Some('s')),
    code_map_single_byte!(0x74, Code::LatinLowerT, Some('t')),
    code_map_single_byte!(0x75, Code::LatinLowerU, Some('u')),
    code_map_single_byte!(0x76, Code::LatinLowerV, Some('v')),
    code_map_single_byte!(0x77, Code::LatinLowerW, Some('w')),
    code_map_single_byte!(0x78, Code::LatinLowerX, Some('x')),
    code_map_single_byte!(0x79, Code::LatinLowerY, Some('y')),
    code_map_single_byte!(0x7A, Code::LatinLowerZ, Some('z')),
    code_map_single_byte!(0x7B, Code::LeftCurlyBracket, Some('{')),
    code_map_single_byte!(0x7C, Code::VerticalLine, Some('|')),
    code_map_single_byte!(0x7D, Code::RightCurlyBracket, Some('}')),
    code_map_single_byte!(0x7E, Code::Tilde, Some('~')),
    code_map_single_byte!(0x7F, Code::MusicalSymbolEighthNote, Some('♪')),
    code_map_single_byte!(0x80, Code::SetCurrentWindow0, None),
    code_map_single_byte!(0x81, Code::SetCurrentWindow1, None),
    code_map_single_byte!(0x82, Code::SetCurrentWindow2, None),
    code_map_single_byte!(0x83, Code::SetCurrentWindow3, None),
    code_map_single_byte!(0x84, Code::SetCurrentWindow4, None),
    code_map_single_byte!(0x85, Code::SetCurrentWindow5, None),
    code_map_single_byte!(0x86, Code::SetCurrentWindow6, None),
    code_map_single_byte!(0x87, Code::SetCurrentWindow7, None),
    code_map_single_byte!(0x8E, Code::DelayCancel, None),
    code_map_single_byte!(0x8F, Code::Reset, None),
    code_map_single_byte!(0xA0, Code::NonBreakingSpace, Some('\u{A0}')),
    code_map_single_byte!(0xA1, Code::InvertedExclamationMark, Some('¡')),
    code_map_single_byte!(0xA2, Code::CentSign, Some('¢')),
    code_map_single_byte!(0xA3, Code::PoundSign, Some('£')),
    code_map_single_byte!(0xA4, Code::GeneralCurrencySign, Some('¤')),
    code_map_single_byte!(0xA5, Code::YenSign, Some('¥')),
    code_map_single_byte!(0xA6, Code::BrokenVerticalBar, Some('¦')),
    code_map_single_byte!(0xA7, Code::SectionSign, Some('§')),
    code_map_single_byte!(0xA8, Code::Umlaut, Some('¨')),
    code_map_single_byte!(0xA9, Code::CopyrightSign, Some('©')),
    code_map_single_byte!(0xAA, Code::FeminineOrdinalSign, Some('ª')),
    code_map_single_byte!(0xAB, Code::LeftDoubleAngleQuote, Some('«')),
    code_map_single_byte!(0xAC, Code::LogicalNotSign, Some('¬')),
    code_map_single_byte!(0xAD, Code::SoftHyphen, Some('\u{00ad}')),
    code_map_single_byte!(0xAE, Code::RegisteredTrademarkSign, Some('Ⓡ')),
    code_map_single_byte!(0xAF, Code::SpacingMacronLongAccent, Some('¯')),
    code_map_single_byte!(0xB0, Code::DegreeSign, Some('°')),
    code_map_single_byte!(0xB1, Code::PlusOrMinusSign, Some('±')),
    code_map_single_byte!(0xB2, Code::Superscript2, Some('²')),
    code_map_single_byte!(0xB3, Code::Superscript3, Some('³')),
    code_map_single_byte!(0xB4, Code::SpacingAccuteAccent, Some('´')),
    code_map_single_byte!(0xB5, Code::MicroSign, Some('µ')),
    code_map_single_byte!(0xB6, Code::ParagraphSign, Some('¶')),
    code_map_single_byte!(0xB7, Code::MiddleDot, Some('·')),
    code_map_single_byte!(0xB8, Code::SpacingCedilla, Some('¸')),
    code_map_single_byte!(0xB9, Code::Superscript1, Some('¹')),
    code_map_single_byte!(0xBA, Code::MasculineOrdinalSign, Some('º')),
    code_map_single_byte!(0xBB, Code::RightDoubleAngleQuote, Some('»')),
    code_map_single_byte!(0xBC, Code::Fraction14, Some('¼')),
    code_map_single_byte!(0xBD, Code::Fraction12, Some('½')),
    code_map_single_byte!(0xBE, Code::Fraction34, Some('¾')),
    code_map_single_byte!(0xBF, Code::InvertedQuestionMark, Some('¿')),
    code_map_single_byte!(0xC0, Code::LatinCapitalAWithGrave, Some('À')),
    code_map_single_byte!(0xC1, Code::LatinCapitalAWithAcute, Some('Á')),
    code_map_single_byte!(0xC2, Code::LatinCapitalAWithCircumflex, Some('Â')),
    code_map_single_byte!(0xC3, Code::LatinCapitalAWithTilde, Some('Ã')),
    code_map_single_byte!(0xC4, Code::LatinCapitalAWithDiaeresis, Some('Ä')),
    code_map_single_byte!(0xC5, Code::LatinCapitalAWithRingAbove, Some('Å')),
    code_map_single_byte!(0xC6, Code::LatinCapitalAe, Some('Æ')),
    code_map_single_byte!(0xC7, Code::LatinCapitalCWithCedilla, Some('Ç')),
    code_map_single_byte!(0xC8, Code::LatinCapitalEWithGrave, Some('È')),
    code_map_single_byte!(0xC9, Code::LatinCapitalEWithAcute, Some('É')),
    code_map_single_byte!(0xCA, Code::LatinCapitalEWithCircumflex, Some('Ê')),
    code_map_single_byte!(0xCB, Code::LatinCapitalEWithDiaeseris, Some('Ë')),
    code_map_single_byte!(0xCC, Code::LatinCapitalIWithGrave, Some('Ì')),
    code_map_single_byte!(0xCD, Code::LatinCapitalIWithAcute, Some('Í')),
    code_map_single_byte!(0xCE, Code::LatinCapitalIWithCircumflex, Some('Î')),
    code_map_single_byte!(0xCF, Code::LatinCapitalIWithDiaeseris, Some('Ï')),
    code_map_single_byte!(0xD0, Code::LatinCapitalEth, Some('Đ')),
    code_map_single_byte!(0xD1, Code::LatinCapitalNWithTilde, Some('Ñ')),
    code_map_single_byte!(0xD2, Code::LatinCapitalOWithGrave, Some('Ò')),
    code_map_single_byte!(0xD3, Code::LatinCapitalOWithAcute, Some('Ó')),
    code_map_single_byte!(0xD4, Code::LatinCapitalOWithCircumflex, Some('Ô')),
    code_map_single_byte!(0xD5, Code::LatinCapitalOWithTilde, Some('Õ')),
    code_map_single_byte!(0xD6, Code::LatinCapitalOWithDiaeresis, Some('Ö')),
    code_map_single_byte!(0xD7, Code::MultiplicationSign, Some('×')),
    code_map_single_byte!(0xD8, Code::LatinCapitalOWithStroke, Some('Ø')),
    code_map_single_byte!(0xD9, Code::LatinCapitalUWithGrave, Some('Ù')),
    code_map_single_byte!(0xDA, Code::LatinCapitalUWithAcute, Some('Ú')),
    code_map_single_byte!(0xDB, Code::LatinCapitalUWithCircumflex, Some('Û')),
    code_map_single_byte!(0xDC, Code::LatinCapitalUWithDiaeresis, Some('Ü')),
    code_map_single_byte!(0xDD, Code::LatinCapitalYWithAcute, Some('Ý')),
    code_map_single_byte!(0xDE, Code::LatinCapitalThorn, Some('Þ')),
    code_map_single_byte!(0xDF, Code::LatinLowerSharpS, Some('ß')),
    code_map_single_byte!(0xE0, Code::LatinLowerAWithGrave, Some('à')),
    code_map_single_byte!(0xE1, Code::LatinLowerAWithAcute, Some('á')),
    code_map_single_byte!(0xE2, Code::LatinLowerAWithCircumflex, Some('â')),
    code_map_single_byte!(0xE3, Code::LatinLowerAWithTilde, Some('ã')),
    code_map_single_byte!(0xE4, Code::LatinLowerAWithDiaeresis, Some('ä')),
    code_map_single_byte!(0xE5, Code::LatinLowerAWithRingAbove, Some('å')),
    code_map_single_byte!(0xE6, Code::LatinLowerAe, Some('æ')),
    code_map_single_byte!(0xE7, Code::LatinLowerCWithCedilla, Some('ç')),
    code_map_single_byte!(0xE8, Code::LatinLowerEWithGrave, Some('è')),
    code_map_single_byte!(0xE9, Code::LatinLowerEWithAcute, Some('é')),
    code_map_single_byte!(0xEA, Code::LatinLowerEWithCircumflex, Some('ê')),
    code_map_single_byte!(0xEB, Code::LatinLowerEWithDiaeseris, Some('ë')),
    code_map_single_byte!(0xEC, Code::LatinLowerIWithGrave, Some('ì')),
    code_map_single_byte!(0xED, Code::LatinLowerIWithAcute, Some('í')),
    code_map_single_byte!(0xEE, Code::LatinLowerIWithCircumflex, Some('î')),
    code_map_single_byte!(0xEF, Code::LatinLowerIWithDiaeseris, Some('ï')),
    code_map_single_byte!(0xF0, Code::LatinLowerEth, Some('ð')),
    code_map_single_byte!(0xF1, Code::LatinLowerNWithTilde, Some('ñ')),
    code_map_single_byte!(0xF2, Code::LatinLowerOWithGrave, Some('ò')),
    code_map_single_byte!(0xF3, Code::LatinLowerOWithAcute, Some('ó')),
    code_map_single_byte!(0xF4, Code::LatinLowerOWithCircumflex, Some('ô')),
    code_map_single_byte!(0xF5, Code::LatinLowerOWithTilde, Some('õ')),
    code_map_single_byte!(0xF6, Code::LatinLowerOWithDiaeresis, Some('ö')),
    code_map_single_byte!(0xF7, Code::DivisionSign, Some('÷')),
    code_map_single_byte!(0xF8, Code::LatinLowerOWithStroke, Some('ø')),
    code_map_single_byte!(0xF9, Code::LatinLowerUWithGrave, Some('ù')),
    code_map_single_byte!(0xFA, Code::LatinLowerUWithAcute, Some('ú')),
    code_map_single_byte!(0xFB, Code::LatinLowerUWithCircumflex, Some('û')),
    code_map_single_byte!(0xFC, Code::LatinLowerUWithDiaeresis, Some('ü')),
    code_map_single_byte!(0xFD, Code::LatinLowerYWithAcute, Some('ý')),
    code_map_single_byte!(0xFE, Code::LatinLowerThorn, Some('þ')),
    code_map_single_byte!(0xFF, Code::LatinLowerYWithDiaeresis, Some('ÿ')),
];

macro_rules! parse_control_code {
    ($data:expr, $arg_len:expr, $enum_val:path) => {{
        let args: [u8; $arg_len] = $data[1..$arg_len + 1].try_into().unwrap();
        $enum_val(args.into())
    }};
}

macro_rules! write_control_code {
    ($control_byte:expr, $w:expr, $args:expr, $arg_len:expr) => {{
        $w.write_all(&[$control_byte])?;
        let args: [u8; $arg_len] = $args.into();
        $w.write_all(&args)
    }};
}

impl Code {
    fn expected_size(bytes: &[u8]) -> Result<usize, CodeError> {
        if bytes.is_empty() {
            return Err(CodeError::LengthMismatch {
                expected: 1,
                actual: 0,
            });
        }
        match bytes[0] {
            0x00..=0x0F => Ok(1),
            0x10 => Ok(Ext1::expected_size(&bytes[1..])? + 1),
            0x11..=0x17 => Ok(2),
            0x18..=0x1F => Ok(3),
            0x20..=0x7F => Ok(1),
            0x80..=0x87 => Ok(1), // CWx
            0x88..=0x8C => Ok(2), // CLW, DSW, HDW, TGW, DLW
            0x8D => Ok(2),        // DLY
            0x8E => Ok(1),        // DLC
            0x8F => Ok(1),        // RST
            0x90 => Ok(3),        // SPA
            0x91 => Ok(4),        // SPC
            0x92 => Ok(3),        // SPL
            0x93..=0x96 => Ok(1), // reserved
            0x97 => Ok(5),        // SWA
            0x98..=0x9F => Ok(7), // DFx
            0xA0..=0xFF => Ok(1),
        }
    }

    /// The length in bytes of this [Code]
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::tables::Code;
    /// assert_eq!(Code::LatinCapitalA.byte_len(), 1);
    /// ```
    pub fn byte_len(&self) -> usize {
        if let Ok(idx) = CODE_MAP_TABLE.binary_search_by_key(&self, |code_map| &code_map.code) {
            return CODE_MAP_TABLE[idx].cea708_bytes.len();
        }
        match self {
            Code::Ext1(ext1) => ext1.byte_len(),
            Code::P16(_) => 3,
            Code::ClearWindows(_args) => 2,
            Code::DisplayWindows(_args) => 2,
            Code::HideWindows(_args) => 2,
            Code::ToggleWindows(_args) => 2,
            Code::DeleteWindows(_args) => 2,
            Code::SetPenAttributes(_args) => 3,
            Code::SetPenColor(_args) => 4,
            Code::SetPenLocation(_args) => 3,
            Code::SetWindowAttributes(_args) => 5,
            Code::DefineWindow(_args) => 7,
            Code::Unknown(data) => data.len(),
            _ => unreachable!(),
        }
    }

    fn parse_element(data: &[u8]) -> Result<Code, CodeError> {
        let size = Code::expected_size(data)?;
        if data.len() > size {
            return Err(CodeError::LengthMismatch {
                expected: size,
                actual: data.len(),
            });
        }
        if let Ok(idx) =
            CODE_MAP_TABLE.binary_search_by_key(&data, |code_map| code_map.cea708_bytes)
        {
            return Ok(CODE_MAP_TABLE[idx].code.clone());
        }
        Ok(match data[0] {
            0x10 => Code::Ext1(Ext1::parse(&data[1..])?),
            0x18 => Code::P16((data[1] as u16) << 8 | data[2] as u16),
            0x88 => parse_control_code!(data, 1, Code::ClearWindows),
            0x89 => parse_control_code!(data, 1, Code::DisplayWindows),
            0x8A => parse_control_code!(data, 1, Code::HideWindows),
            0x8B => parse_control_code!(data, 1, Code::ToggleWindows),
            0x8C => parse_control_code!(data, 1, Code::DeleteWindows),
            0x90 => parse_control_code!(data, 2, Code::SetPenAttributes),
            0x91 => parse_control_code!(data, 3, Code::SetPenColor),
            0x92 => parse_control_code!(data, 2, Code::SetPenLocation),
            0x97 => parse_control_code!(data, 4, Code::SetWindowAttributes),
            0x98..=0x9F => {
                let args: [u8; 6] = data[1..7].try_into().unwrap();
                let args = args.into();
                let args = DefineWindowArgs {
                    window_id: data[0] & 0x07,
                    ..args
                };
                Code::DefineWindow(args)
            }
            _ => Code::Unknown(data.to_vec()),
        })
    }

    /// Parse a byte sequence into a list of [Code]s
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::tables::Code;
    /// assert_eq!(Code::from_data(&[0x41]), Ok(vec![Code::LatinCapitalA]));
    /// ```
    pub fn from_data(data: &[u8]) -> Result<Vec<Code>, CodeError> {
        let mut data_iter = data;
        let mut ret = vec![];
        while !data_iter.is_empty() {
            let size = Code::expected_size(data_iter)?;
            if data_iter.len() < size {
                return Err(CodeError::LengthMismatch {
                    expected: size,
                    actual: data_iter.len(),
                });
            }
            let element = &data_iter[..size];
            let element = Code::parse_element(element)?;
            ret.push(element);

            data_iter = &data_iter[size..];
        }
        Ok(ret)
    }

    /// Write a [Code] to a byte stream
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::tables::Code;
    /// let mut written = vec![];
    /// Code::LatinCapitalA.write(&mut written).unwrap();
    /// assert_eq!(written, [0x41]);
    /// ```
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        if let Ok(idx) = CODE_MAP_TABLE.binary_search_by_key(&self, |code_map| &code_map.code) {
            return w.write_all(CODE_MAP_TABLE[idx].cea708_bytes);
        }
        match self {
            Code::Ext1(ext1) => {
                w.write_all(&[0x10])?;
                ext1.write(w)
            }
            Code::P16(c) => w.write_all(&[0x18, ((c & 0xFF00) >> 8) as u8, (c & 0xFF) as u8]),
            Code::ClearWindows(args) => write_control_code!(0x88, w, *args, 1),
            Code::DisplayWindows(args) => write_control_code!(0x89, w, *args, 1),
            Code::HideWindows(args) => write_control_code!(0x8A, w, *args, 1),
            Code::ToggleWindows(args) => write_control_code!(0x8B, w, *args, 1),
            Code::DeleteWindows(args) => write_control_code!(0x8C, w, *args, 1),
            Code::SetPenAttributes(args) => write_control_code!(0x90, w, *args, 2),
            Code::SetPenColor(args) => write_control_code!(0x91, w, *args, 3),
            Code::SetPenLocation(args) => write_control_code!(0x92, w, *args, 2),
            Code::SetWindowAttributes(args) => write_control_code!(0x97, w, *args, 4),
            Code::DefineWindow(args) => {
                write_control_code!(0x98 | (args.window_id & 0x07), w, *args, 6)
            }
            Code::Unknown(data) => w.write_all(data),
            _ => unreachable!(),
        }
    }

    /// The utf8 char for this [Code]
    ///
    /// [Code]s that represent a command will return None.
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::tables::Code;
    /// assert_eq!(Code::LatinCapitalA.char(), Some('A'));
    /// ```
    pub fn char(&self) -> Option<char> {
        // table is not currently sorted by utf8 value so cannot binary search through it.  May
        // need another lookup table if this is a performance concern
        CODE_MAP_TABLE.iter().find_map(|code_map| {
            if code_map.code == *self {
                code_map.utf8
            } else {
                None
            }
        })
    }

    /// Retrieve a [Code] for a utf8 char
    ///
    /// If the char is not representable as a [Code], None will be returned.
    ///
    /// # Examples
    /// ```
    /// # use cea708_types::tables::Code;
    /// assert_eq!(Code::from_char('A'), Some(Code::LatinCapitalA));
    /// ```
    pub fn from_char(c: char) -> Option<Code> {
        // table is not currently sorted by utf8 value so cannot binary search through it.  May
        // need another lookup table if this is a performance concern
        CODE_MAP_TABLE.iter().find_map(|code_map| {
            if code_map.utf8 == Some(c) {
                Some(code_map.code.clone())
            } else {
                None
            }
        })
    }
}

impl Ext1 {
    fn expected_size(bytes: &[u8]) -> Result<usize, CodeError> {
        if bytes.is_empty() {
            return Err(CodeError::LengthMismatch {
                expected: 1,
                actual: 0,
            });
        }
        match bytes[0] {
            0x00..=0x07 => Ok(1),
            0x08..=0x0F => Ok(2),
            0x10..=0x17 => Ok(3),
            0x18..=0x1F => Ok(4),
            0x20..=0x7F => Ok(1), // G2
            0x80..=0x87 => Ok(5),
            0x88..=0x8F => Ok(6),
            0x90..=0x9F => {
                if bytes.len() < 2 {
                    return Err(CodeError::LengthMismatch {
                        expected: 2,
                        actual: 0,
                    });
                }
                Ok(((bytes[1] & 0x3F) as usize) + 1)
            }
            0xA0..=0xFF => Ok(1), // G3
        }
    }

    fn byte_len(&self) -> usize {
        // All currently known Ext1 codes are covered in the static table
        match self {
            Ext1::Unknown(data) => data.len(),
            _ => unreachable!(),
        }
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // All currently known Ext1 codes are covered in the static table
        match self {
            Ext1::Unknown(data) => w.write_all(data),
            _ => unreachable!(),
        }
    }

    fn parse(data: &[u8]) -> Result<Ext1, CodeError> {
        // All currently known Ext1 codes are covered in the static table
        Ok(Ext1::Unknown(data.to_vec()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tests::*;

    #[test]
    fn codes_table_ordered() {
        test_init_log();
        let mut iter = CODE_MAP_TABLE.iter().peekable();
        while let Some(code_map) = iter.next() {
            if let Some(peek) = iter.peek() {
                trace!("checking ordinality for {code_map:?} and {peek:?}");
                assert!(peek.code > code_map.code);
                assert!(peek.cea708_bytes > code_map.cea708_bytes);
            }
        }
    }

    static VARIABLE_TEST_CODES: [CodeMap; 10] = [
        code_map_bytes!(
            [0x9A, 0x38, 0x4A, 0xD1, 0x8B, 0x0F, 0x11],
            Code::DefineWindow(DefineWindowArgs::new(
                2,
                0,
                Anchor::BottomRight,
                false,
                74,
                209,
                11,
                15,
                true,
                true,
                true,
                2,
                1,
            )),
            None
        ),
        code_map_bytes!(
            [0x97, 0x64, 0x53, 0x88, 0x22],
            Code::SetWindowAttributes(SetWindowAttributesArgs::new(
                Justify::Left,
                Direction::LeftToRight,
                Direction::TopToBottom,
                false,
                DisplayEffect::Wipe,
                Direction::LeftToRight,
                2,
                Color::new(
                    ColorValue::TwoThirds,
                    ColorValue::OneThird,
                    ColorValue::None
                ),
                Opacity::Flash,
                BorderType::ShadowRight,
                Color::new(ColorValue::OneThird, ColorValue::None, ColorValue::Full)
            )),
            None
        ),
        code_map_bytes!(
            [0x8B, 0xF6],
            Code::ToggleWindows(WindowBits::ZERO.or(WindowBits::THREE).not()),
            None
        ),
        code_map_bytes!(
            [0x8A, 0x7E],
            Code::HideWindows(WindowBits::ZERO.or(WindowBits::SEVEN).not()),
            None
        ),
        code_map_bytes!([0x89, 0x80], Code::DisplayWindows(WindowBits::SEVEN), None),
        code_map_bytes!(
            [0x8C, 0xFE],
            Code::DeleteWindows(WindowBits::ZERO.not()),
            None
        ),
        code_map_bytes!(
            [0x88, 0x13],
            Code::ClearWindows(WindowBits::ZERO.or(WindowBits::ONE).or(WindowBits::FOUR)),
            None
        ),
        code_map_bytes!(
            [0x90, 0x4A, 0xCA],
            Code::SetPenAttributes(SetPenAttributesArgs::new(
                PenSize::Large,
                FontStyle::ProportionallySpacedWithSerifs,
                TextTag::Voiceover,
                TextOffset::Superscript,
                true,
                true,
                EdgeType::Raised
            )),
            None
        ),
        code_map_bytes!(
            [0x91, 0x3F, 0xC0, 0x15],
            Code::SetPenColor(SetPenColorArgs::new(
                Color::new(ColorValue::Full, ColorValue::Full, ColorValue::Full),
                Opacity::Solid,
                Color::new(ColorValue::None, ColorValue::None, ColorValue::None),
                Opacity::Transparent,
                Color::new(
                    ColorValue::OneThird,
                    ColorValue::OneThird,
                    ColorValue::OneThird
                )
            )),
            None
        ),
        code_map_bytes!(
            [0x92, 0x05, 0x08],
            Code::SetPenLocation(SetPenLocationArgs::new(5, 8)),
            None
        ),
    ];

    #[test]
    fn codes_to_from_bytes() {
        test_init_log();
        for code_map in CODE_MAP_TABLE.iter().chain(VARIABLE_TEST_CODES.iter()) {
            trace!("parsing {code_map:?}");
            let parsed_code = Code::parse_element(code_map.cea708_bytes).unwrap();
            assert_eq!(parsed_code, code_map.code);
            let mut written = vec![];
            parsed_code.write(&mut written).unwrap();
            assert_eq!(written, code_map.cea708_bytes);
            assert_eq!(written.len(), code_map.code.byte_len());
        }
    }

    #[test]
    fn codes_to_from_char() {
        test_init_log();
        for code_map in CODE_MAP_TABLE.iter() {
            trace!("parsing {code_map:?}");
            if let Some(c) = code_map.utf8 {
                let parsed_code = Code::from_char(c).unwrap();
                assert_eq!(parsed_code.char(), code_map.utf8);
                assert_eq!(parsed_code, code_map.code);
                let mut written = vec![];
                parsed_code.write(&mut written).unwrap();
                assert_eq!(written, code_map.cea708_bytes);
            }
        }
    }
}
