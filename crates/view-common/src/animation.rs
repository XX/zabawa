use std::borrow::Cow;

use hypertext::{Renderable, rsx};

use crate::hypertext_elements;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Animation {
    pub name: Cow<'static, str>,
    pub duration: usize,
    pub iterations: usize,
}

impl Animation {
    pub fn new(name: impl Into<Cow<'static, str>>, duration: usize, iterations: usize) -> Self {
        Self {
            name: name.into(),
            duration,
            iterations,
        }
    }

    pub fn with_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_duration(mut self, duration: usize) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn render(&self, children: impl Renderable) -> impl Renderable {
        rsx! {
            <wa-animation name=(self.name) duration=(self.duration) iterations=(self.iterations)>
                (children)
            </wa-animation>
        }
    }
}
