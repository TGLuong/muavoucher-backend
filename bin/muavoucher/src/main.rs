use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;
use time::macros::format_description;
use tokio::time::sleep;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

use crate::{
    auth_token::jwt::JwtAuthToken,
    logic::Logic,
    otp_notifier::zalo_gmail::ZaloGmailNotifier,
    storage::{kv_store::memory::MemoryCache, migration::Migrator, repository::init_repository},
    transport::http::{HttpServer, context::HttpContext},
    webhook_validator::sepay::SepayWebhookValidator,
};

pub mod auth_token;
pub mod logic;
pub mod otp_notifier;
pub mod storage;
pub mod transport;
pub mod utils;
pub mod webhook_validator;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env, default_value = "postgresql://user:password@localhost:9990/muavoucher")]
    databse: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Start(StartCommand),
    #[clap(subcommand)]
    Migrate(MigrateCommand),
}

#[derive(Debug, Parser)]
struct StartCommand {
    #[arg(long, env, default_value = "127.0.0.1:8888")]
    http_addr: SocketAddr,
    #[arg(long, env, default_value = "true")]
    migration: bool,
    #[arg(long, env)]
    secret: String,
    #[arg(long, env)]
    sepay_key: String,
    #[arg(long, env)]
    gmail_sender: String,
    #[arg(long, env)]
    gmail_app_key: String,
    #[arg(long, env)]
    zalo_key: String,
    #[arg(long, env)]
    get_link_script: String,
    #[arg(long, env)]
    product_info_base: String,
}

#[derive(Debug, Parser)]
enum MigrateCommand {
    Up(MigrationArgs),
    Down(MigrationArgs),
}

#[derive(Debug, Parser)]
struct MigrationArgs {
    #[arg(long, env)]
    step: Option<u32>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_timer(LocalTime::new(&format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:6]"
        )))
        .with_env_filter(EnvFilter::builder().with_default_directive(tracing::Level::INFO.into()).from_env_lossy())
        .init();
    let args = Args::parse();
    log::info!("[main] start with {args:?}");

    match args.command {
        Command::Start(cmd) => start(args.databse, cmd).await,
        Command::Migrate(cmd) => migrate(args.databse, cmd).await,
    }
}

async fn start(database: String, cmd: StartCommand) -> anyhow::Result<()> {
    let connection = Database::connect(database).await?;
    if cmd.migration {
        Migrator::up(&connection, None).await?;
    }
    let otp_notifier = ZaloGmailNotifier::new(cmd.zalo_key, cmd.gmail_sender, cmd.gmail_app_key);
    let kv_store = MemoryCache::default();
    let auth_token = JwtAuthToken::new(cmd.secret);
    let sepay_validator = SepayWebhookValidator::new(cmd.sepay_key);
    let database = init_repository(connection);
    let logic = Logic::new(
        otp_notifier,
        kv_store,
        auth_token,
        sepay_validator,
        database.clone(),
        cmd.get_link_script,
        cmd.product_info_base,
    );
    let http_context = HttpContext::new(logic, database.clone());
    let http = HttpServer::new(cmd.http_addr, http_context).await?;
    http.run();
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

async fn migrate(database: String, cmd: MigrateCommand) -> anyhow::Result<()> {
    let connection = Database::connect(database).await?;
    match cmd {
        MigrateCommand::Up(args) => Migrator::up(&connection, args.step).await?,
        MigrateCommand::Down(args) => Migrator::down(&connection, args.step).await?,
    }
    Ok(())
}
