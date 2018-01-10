pub trait End {
    fn breaks_loop(&self) -> bool;
    fn to_break_continue(&self) -> BreakContinue;
}

impl End for () {
    fn breaks_loop(&self) -> bool { false }
    fn to_break_continue(&self) -> BreakContinue { BreakContinue::Continue }
}

impl End for BreakContinue {
    fn breaks_loop(&self) -> bool { 
        match self {
            &BreakContinue::Break => true,
            &BreakContinue::Continue => false
        } 
    }
    fn to_break_continue(&self) -> BreakContinue {
        match self {
            &BreakContinue::Break => BreakContinue::Break,
            &BreakContinue::Continue => BreakContinue::Continue
        } 
    }
}

pub enum BreakContinue {
    Break,
    Continue
}
use experimental::loops::BreakContinue::*;