pub use double_dot_macro_derive::*;
pub use double_dot_macro_types::*;
pub trait DoubleState {
    /// Returns the name of the enum
    fn name(&self) -> &'static str;
    /// Attempts to find a valid linear transition for the current self state.
    /// 
    /// `panics` if no valid linear transition was found for self
    fn linear_transition(&self) -> Self;
    /// Attempts to find a valid arbirary transition for the current self state.
    /// 
    /// `panics` if no valid arbitrary transition was found for self
    fn arbitrary_transition(&self, next_state: &Self) -> Self;
    /// Converts the current self state to a String
    fn to_string(&self) -> String;
    /// Parse the arbitrary transition `String` from `StateFields` into a `Vec<String>`
    fn parse_arbs(&self, arbs: &str) -> Vec<String>;
}

/// Enum for Testing purposes
#[derive(Clone, Eq, PartialEq, Debug, Hash, DoubleState, Default)]
enum State {
    #[linear(MainMenu)]
    Loading,
    #[arbitrary(Playing, Exit)]
    MainMenu,
    #[default]
    #[linear(Paused)]
    Playing,
    #[arbitrary(MainMenu, Exit)]
    Paused,
    Exit
}