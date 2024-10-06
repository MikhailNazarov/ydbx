
use tracing::error;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use ydbx::{error::YdbxError, Ydbx};

#[tokio::main]
async fn main() -> Result<(), YdbxError> {
    dotenvy::dotenv().ok();
    init_logs();

    if let Err(e) = run().await{
        error!("{:?}",e);
    }

    Ok(())
}

async fn run()->Result<(), YdbxError>{

    let ydb = Ydbx::connect("grpc://localhost:2136?database=/local".to_string()).await?;

    let session = ydb.create_session().await?;
    
    Ok(())
}


pub fn init_logs() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
