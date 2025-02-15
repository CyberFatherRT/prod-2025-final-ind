use std::{any::type_name, future::Future};

use axum::Router;
use setup::{get_app, initialize_containers};
use tracing::info;

mod clients;
mod setup;

async fn run_test<F, Fut>(app: &Router, test_callback: F)
where
    F: FnOnce(Router) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    info!("Running test: {}", type_name::<F>());
    match test_callback(app.clone()).await {
        Ok(_) => info!("Test passed successfully for {}", type_name::<F>()),
        Err(e) => panic!("Test failed: {:?}", e),
    }
    println!();
}

macro_rules! run_tests {
    ($app:expr, $module:ident, $group:ident, [$($test_fn:ident),*]) => {
        {
            use tracing::info;

            info!("Running `{}` tests for `{}` module", stringify!($group), stringify!($module));
            println!();

            $(
                run_test(&$app, $module::$group::$test_fn).await;
            )*

            info!("All `{}` tests for `{}` module passed successfully", stringify!($group), stringify!($module));
            println!("\n");
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup::init_tracing();
    let containers = initialize_containers().await;
    let app = get_app(&containers).await;
    println!();

    run_tests!(app, clients, valid, [bulk, get]);
    run_tests!(app, clients, missing, [bulk, get]);
    run_tests!(app, clients, wrong, [bulk]);

    info!("All tests passed successfully");
    Ok(())
}
