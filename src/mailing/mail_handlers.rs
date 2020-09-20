use super::{get_email_client, get_set_stop_loss_context, get_tera, StopLossSet};
use crate::error::MailgunError;
use crate::guard::UserDto;
use mailgun_api::api::EmailParams;
use std::collections::HashMap;
use std::env;
use warp::{reject, reply, Rejection, Reply};

pub async fn send_stop_loss_email(
    user: UserDto,
    stop_loss: StopLossSet,
) -> Result<impl Reply, Rejection> {
    let sender = env::var("MAIL_FROM").expect("MAIL_FROM must be set");
    let tera = get_tera()?;
    let mut email_client = get_email_client();

    let template =
        get_set_stop_loss_context(tera, &stop_loss.pair, &stop_loss.price, &stop_loss.benefit)?;

    let params = EmailParams {
        to: user.email,
        from: sender,
        subject: "Realizada nueva orden de stop loss".to_string(),
        text: None,
        html: Some(template),
    };

    email_client
        .send_email::<HashMap<String, String>>(params)
        .await
        .map_err(|err| reject::custom(MailgunError { error: err }))?;

    Ok(reply::json(&"Stop loss order notified"))
}
