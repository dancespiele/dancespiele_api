use super::mailing::add_stop_loss;
use celery::error::CeleryError;

pub async fn start_consumer() -> Result<(), CeleryError> {
    let tasks = celery::app!(
        broker = AMQP { std::env::var("AMPQ_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672".into())},
        tasks = [
            add_stop_loss,
        ],
        task_routes = [
            "add_stop_loss" => "stop_loss_queue",
        ]
    );

    tasks.consume_from(&["stop_loss_queue"]).await
}
