pub mod animation;

pub use self::animation::*;

pub mod hypertext_elements {
    use hypertext::define_elements;
    // Re-export all standard HTML elements
    pub use hypertext::validation::hypertext_elements::*;

    define_elements! {
        wa_animation { name duration iterations }
    }
}
