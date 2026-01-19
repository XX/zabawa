use hypertext::prelude::GlobalAttributes;
use hypertext::{Renderable, rsx};

use crate::{NotificationViewData, Notifications, hypertext_elements};

pub fn render_notification_list<'a>(notifications: Notifications<'a>) -> impl Renderable {
    rsx! {
        @for NotificationViewData { variant, icon, message } in notifications.iter() {
            <wa-callout class={ "notification-" (variant) } variant=(variant)>
                <wa-icon slot="icon" name=(icon)></wa-icon>
                (message)
            </wa-callout>
        }
    }
}
