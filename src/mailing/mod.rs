mod dtos;
mod filters;
mod helpers;
mod mail_handlers;

pub use dtos::StopLossSet;
pub use filters::mail;
pub use helpers::{get_email_client, get_set_stop_loss_context, get_tera};
pub use mail_handlers::send_stop_loss_email;
