use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(|event: LambdaEvent<Value>| async move {
        Ok::<_, Error>(json!({
            "requestId": event.context.request_id,
            "schemaVersion": "ores.lambda-health.v1",
            "provider": "aws-lambda",
            "ok": true,
            "result": { "status": "ok" }
        }))
    }))
    .await
}
