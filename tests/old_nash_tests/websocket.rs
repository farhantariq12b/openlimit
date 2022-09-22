use std::{env, sync::mpsc::sync_channel};
use std::str::FromStr;
use std::time::Duration as NativeDuration;

use dotenv::dotenv;
use nash_native_client::Environment;
use rust_decimal::Decimal;
use tokio::time::Duration;

use openlimits::{exchange::nash::NashWebsocket, model::websocket::Subscription};
use openlimits::exchange::nash::{Nash, NashCredentials, NashParameters};
use openlimits::exchange::traits::stream::{ExchangeWs, OpenLimitsWs};
use openlimits::model::{CancelAllOrdersRequest, OpenLimitOrderRequest, TimeInForce};
use openlimits::model::websocket::AccountOrders;
use openlimits::OpenLimits;
use openlimits::prelude::*;

async fn init_exchange() -> Nash {
    dotenv().ok();

    let parameters = NashParameters {
        affiliate_code: None,
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        client_id: 1,
        timeout: NativeDuration::new(10, 0),
        sign_states_loop_interval: None,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}

async fn test_subscription_callback(websocket: OpenLimitsWs<NashWebsocket>, sub: Subscription) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Couldn't get response.");
            tx.send(()).expect("Couldn't send sync message.");
        })
        .await
        .expect("Couldn't subscribe.");

    rx.recv().expect("Couldn't receive sync message.");
}

async fn test_account_subscription_callback(
    websocket: OpenLimitsWs<NashWebsocket>,
    sub: Subscription,
    cancel_orders: bool,
) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Couldn't get response.");
            tx.send(()).expect("Couldn't send sync message.");
        })
        .await
        .expect("Couldn't subscribe.");

    let exchange = init_exchange().await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        time_in_force: TimeInForce::GoodTillCancelled,
        price: Decimal::from_str("0.01").expect("Couldn't parse string."),
        size: Decimal::from_str("0.1").expect("Couldn't parse string."),
        market_pair: String::from("eth_btc"),
        post_only: false,
    };

    exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't limit sell.");

    if cancel_orders {
        let req = CancelAllOrdersRequest {
            market_pair: Some("eth_btc".to_string()),
        };

        let resp = exchange
            .cancel_all_orders(&req)
            .await
            .expect("Couldn't cancel all orders.");
        println!("{:?}", resp);
    }

    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn account_orders() {
    let client = init().await;
    let sub = Subscription::AccountOrders(AccountOrders {
        market: Some("eth_btc".to_string()),
        order_type: None,
        status: None,
        buy_or_sell: None,
        range: None,
    });
    test_account_subscription_callback(client, sub, true).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn account_trades() {
    let client = init().await;
    let sub = Subscription::AccountTrades("eth_btc".to_string());
    test_account_subscription_callback(client, sub, false).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn account_balance() {
    let client = init().await;
    let sub = Subscription::AccountBalance("eth".to_string());
    test_account_subscription_callback(client, sub, false).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    let client = init().await;
    let sub = Subscription::Trades("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

async fn init() -> OpenLimitsWs<NashWebsocket> {
    dotenv().ok();

    let websocket = NashWebsocket::new(NashParameters {
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        affiliate_code: None,
        client_id: 1234,
        environment: Environment::Sandbox,
        timeout: Duration::from_secs(10),
        sign_states_loop_interval: None,
    })
    .await
    .expect("Couldn't connect.");

    OpenLimitsWs { websocket }
}
