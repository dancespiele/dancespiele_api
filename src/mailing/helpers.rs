use crate::error::TemplateError;
use mailgun_api::MailgunApi;
use std::env;
use tera::{Context, Tera};
use warp::{reject, Rejection};

pub fn get_email_client() -> MailgunApi {
    let mailgung_secret = env::var("MAILGUN_SECRET").expect("MAILGUN_SECRET must be set");
    let mailgung_domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
    let mailgung_endpoint = env::var("MAILGUN_ENDPOINT").expect("MAILGUN_ENDPOINT must be set");
    MailgunApi::new(&mailgung_secret, &mailgung_endpoint, &mailgung_domain)
}

pub fn get_tera() -> Result<Tera, Rejection> {
    let tera = Tera::new("static/email_templates/*.html")
        .map_err(|err| reject::custom(TemplateError { error: err }))?;
    Ok(tera)
}

pub fn get_set_stop_loss_context(
    tera: Tera,
    pair: &str,
    price: &str,
    benefit: &str,
) -> Result<String, Rejection> {
    let mut context = Context::new();
    context.insert("pair", pair);
    context.insert("price", price);
    context.insert("benefit", benefit);
    let template = tera
        .render("stop_loss.html", &context)
        .map_err(|err| reject::custom(TemplateError { error: err }))?;

    Ok(template)
}
