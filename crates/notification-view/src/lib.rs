pub mod context;
pub mod notification;

pub use self::context::*;
pub use self::notification::*;

pub mod hypertext_elements {
    use hypertext::define_elements;
    // Re-export all standard HTML elements
    pub use hypertext::validation::hypertext_elements::*;

    define_elements! {
        wa_icon { slot name }
        wa_callout { variant }
    }
}
