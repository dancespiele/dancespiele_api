use super::{get_email_client, get_set_notify_email_context, get_tera, NotifyEmail};
use crate::db::init_tree;
use celery::{error::TaskError, task::Task, TaskResult};
use mailgun_api::api::EmailParams;
use std::collections::HashMap;
use std::env;

#[celery::task(
    on_failure = failure_task,
    on_success = email_sent
)]
pub async fn add_stop_loss(notify: NotifyEmail) -> TaskResult<String> {
    let lang = env::var("LANGUAGE").unwrap_or_else(|_| String::from("en"));

    let lang_folder = match lang.as_ref() {
        "en" => "en",
        "es" => "es",
        &_ => "en",
    };
    let sender = env::var("MAIL_FROM").expect("MAIL_FROM must be set");
    let tera = get_tera(lang_folder)?;
    let mut email_client = get_email_client();

    let template =
        get_set_notify_email_context(tera, &notify.pair, &notify.price, &notify.benefit)?;

    let params = EmailParams {
        to: notify.email,
        from: sender,
        subject: if lang_folder == "es" {
            "Realizada nueva orden de stop loss".to_string()
        } else {
            "Executed new stop loss order".to_string()
        },
        text: None,
        html: Some(template),
    };

    email_client
        .send_email::<HashMap<String, String>>(params)
        .await
        .map_err(|err| TaskError::ExpectedError(err.to_string()))?;

    Ok(String::from("Stop loss order notified"))
}

async fn email_sent<T: Task>(task: &T, resp: &T::Returns)
where
    T::Returns: ToString,
{
    let tree_result = init_tree();

    let id = task.request().id.clone();
    let name = task.name();
    let content = resp.to_string();

    if let Ok(tree) = tree_result {
        let task_saved_result = tree.insert(id.clone(), content.as_bytes());

        if task_saved_result.is_ok() {
            info!("Task {} with id {} is saved", name, id);
        } else {
            error!("Task {} with id {} fail saving in database", name, id);
        }
    } else {
        info!("Task {} with id {} fail to connect with sled", name, id);
    }
}

async fn failure_task<T: Task>(task: &T, err: &TaskError) {
    match err {
        TaskError::TimeoutError => error!("Task {}: error timeout!", task.name()),
        _ => error!("Task {} failed with {:?}", task.name(), err),
    }
}
