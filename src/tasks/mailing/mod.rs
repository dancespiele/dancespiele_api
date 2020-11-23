mod dtos;
mod helpers;
mod mail_task;

pub use dtos::NotifyEmail;
pub use helpers::{get_email_client, get_set_notify_email_context, get_tera};
pub use mail_task::add_stop_loss;
