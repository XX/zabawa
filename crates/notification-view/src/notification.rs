use std::borrow::Cow;

use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};
use zabawa_view_common::Animation;

use crate::{NotificationViewData, Notifications, hypertext_elements};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NotificationView {
    pub animation: Option<Animation>,
    pub callout_script: Option<Cow<'static, str>>,
}

impl NotificationView {
    pub fn new() -> Self {
        Self {
            animation: Some(Animation {
                name: Cow::Borrowed("zoomOut"),
                duration: 500,
                iterations: 1,
            }),
            callout_script: Some(Cow::Borrowed("close_callout()")),
        }
    }

    pub fn with_animation(mut self, animation: Animation) -> Self {
        self.animation = Some(animation);
        self
    }

    pub fn without_animation(mut self) -> Self {
        self.animation = None;
        self
    }

    pub fn with_callout_script(mut self, script: impl Into<Cow<'static, str>>) -> Self {
        self.callout_script = Some(script.into());
        self
    }

    pub fn without_callout_script(mut self) -> Self {
        self.callout_script = None;
        self
    }

    pub fn render<'a>(
        &self,
        NotificationViewData { variant, icon, message }: NotificationViewData<'a>,
    ) -> impl Renderable {
        rsx! {
            <wa-callout class={ "notification-" (variant) } variant=(variant)>
                <wa-icon slot="icon" name=(icon)></wa-icon>
                <div class="wa-flank:end wa-align-items-start">
                    <div>(message)</div>
                    <div>
                        <wa-button class="close" appearance="plain" variant=(variant) size="small">
                            <wa-icon name="xmark" library="system" variant="solid" label="Close" role="img" aria-label="Close"></wa-icon>
                        </wa-button>
                    </div>
                </div>
                @if let Some(script) = &self.callout_script {
                    <script>(script)</script>
                }
            </wa-callout>
        }
    }

    pub fn render_list<'a>(&self, notifications: Notifications<'a>) -> impl Renderable {
        rsx! {
            @for view_data in notifications.iter() {
                @if let Some(animation) = &self.animation {
                    (animation.render(self.render(view_data)))
                } @else {
                    (self.render(view_data))
                }
            }
        }
    }
}
