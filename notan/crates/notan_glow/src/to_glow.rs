use glow::*;
use notan_graphics::prelude::*;

pub trait ToGlow {
    fn to_glow(&self) -> u32;
}

pub trait ToOptionalGlow {
    fn to_glow(&self) -> Option<u32>;
}

impl ToGlow for StencilAction {
    fn to_glow(&self) -> u32 {
        use StencilAction::*;
        match self {
            Keep => glow::KEEP,
            Zero => glow::ZERO,
            Replace => glow::REPLACE,
            Increment => glow::INCR,
            IncrementWrap => glow::INCR_WRAP,
            Decrement => glow::DECR,
            DecrementWrap => glow::DECR_WRAP,
            Invert => glow::INVERT,
        }
    }
}

impl ToGlow for BlendOperation {
    fn to_glow(&self) -> u32 {
        use BlendOperation::*;
        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Max => glow::MAX,
            Min => glow::MIN,
        }
    }
}

impl ToGlow for BlendFactor {
    fn to_glow(&self) -> u32 {
        use BlendFactor::*;
        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            SourceAlpha => glow::SRC_ALPHA,
            SourceColor => glow::SRC_COLOR,
            InverseSourceAlpha => glow::ONE_MINUS_SRC_ALPHA,
            InverseSourceColor => glow::ONE_MINUS_SRC_COLOR,
            DestinationAlpha => glow::DST_ALPHA,
            DestinationColor => glow::SRC_COLOR,
            InverseDestinationAlpha => glow::ONE_MINUS_DST_ALPHA,
            InverseDestinationColor => glow::ONE_MINUS_DST_COLOR,
        }
    }
}

impl ToOptionalGlow for CompareMode {
    fn to_glow(&self) -> Option<u32> {
        use CompareMode::*;
        Some(match self {
            None => return Option::None,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        })
    }
}

impl ToOptionalGlow for CullMode {
    fn to_glow(&self) -> Option<u32> {
        use CullMode::*;
        Some(match self {
            None => return Option::None,
            Front => glow::FRONT,
            Back => glow::BACK,
        })
    }
}
