use zabawa_notification_domain::model::Notification;

use crate::NotificationViewData;

#[derive(Clone, Copy, Default, Debug)]
pub struct Notifications<'a>(pub &'a [Notification]);

impl<'a> Notifications<'a> {
    pub fn iter(&self) -> impl Iterator<Item = NotificationViewData<'a>> {
        self.0.iter().map(Into::into)
    }
}
