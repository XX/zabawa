use std::borrow::Cow;

use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};
use zabawa_notification_domain::model::{Notification, NotificationLevel};
use zabawa_view_common::Animation;

use crate::{Notifications, hypertext_elements};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NotificationViewData<'a> {
    pub variant: &'static str,
    pub icon: &'static str,
    pub message: &'a str,
}

impl<'a> From<&'a Notification> for NotificationViewData<'a> {
    fn from(notification: &'a Notification) -> NotificationViewData<'a> {
        let message = &notification.message;
        match notification.level {
            NotificationLevel::Error => NotificationViewData {
                variant: "danger",
                icon: "circle-exclamation",
                message,
            },
            NotificationLevel::Warning => NotificationViewData {
                variant: "warning",
                icon: "triangle-exclamation",
                message,
            },
            NotificationLevel::Success => NotificationViewData {
                variant: "success",
                icon: "circle-check",
                message,
            },
            NotificationLevel::Info => NotificationViewData {
                variant: "brand",
                icon: "circle-info",
                message,
            },
            NotificationLevel::Note => NotificationViewData {
                variant: "neutral",
                icon: "pen-to-square",
                message,
            },
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NotificationView {
    pub animation: Option<Animation>,
    pub callout_script: Option<Cow<'static, str>>,
    pub close_button_enabled: bool,
}

impl NotificationView {
    pub fn new() -> Self {
        Self {
            animation: Some(Animation {
                name: Cow::Borrowed("zoomOut"),
                duration: 500,
                iterations: 1,
            }),
            callout_script: Some(Cow::Borrowed("close_notification()")),
            close_button_enabled: true,
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

    pub fn with_close_button(mut self, enabled: bool) -> Self {
        self.close_button_enabled = enabled;
        self
    }

    pub fn render<'a>(&self, view_data: NotificationViewData<'a>) -> impl Renderable {
        rsx! {
            @if let Some(animation) = &self.animation {
                (animation.render(self.render_callout(view_data)))
            } @else {
                (self.render_callout(view_data))
            }
        }
    }

    pub fn render_callout<'a>(
        &self,
        NotificationViewData { variant, icon, message }: NotificationViewData<'a>,
    ) -> impl Renderable {
        rsx! {
            <wa-callout class={ "notification-" (variant) } variant=(variant)>
                <wa-icon slot="icon" name=(icon)></wa-icon>
                @if self.close_button_enabled {
                    <div class="wa-flank:end wa-align-items-start">
                        <div>(message)</div>
                        <div>
                            <wa-button class="close" appearance="plain" variant=(variant) size="small">
                                <wa-icon name="xmark" library="system" variant="solid" label="Close" role="img" aria-label="Close"></wa-icon>
                            </wa-button>
                        </div>
                    </div>
                } @else {
                    (message)
                }
                @if let Some(script) = &self.callout_script {
                    <script>(script)</script>
                }
            </wa-callout>
        }
    }

    pub fn render_list<'a>(&self, notifications: Notifications<'a>) -> impl Renderable {
        rsx! {
            @for view_data in notifications.iter() {
                (self.render(view_data))
            }
        }
    }
}
