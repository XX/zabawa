use zabawa_notification_domain::model::{Notification, NotificationLevel};

#[derive(Clone, Copy, Default, Debug)]
pub struct Notifications<'a>(pub &'a [Notification]);

impl<'a> Notifications<'a> {
    pub fn iter(&self) -> impl Iterator<Item = NotificationViewData<'a>> {
        self.0.iter().map(Into::into)
    }
}

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
