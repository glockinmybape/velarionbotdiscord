use warp::Filter;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::blockchain::Blockchain;
use crate::state::State;

#[derive(Debug, Serialize)]
struct RpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct RpcResponse<T> {
    jsonrpc: String,
    result: Option<T>,
    error: Option<RpcError>,
    id: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct GetBalanceParams {
    address: String,
}

#[derive(Deserialize)]
struct SendTransactionParams {
    from: String,
    to: String,
    amount: u64,
    signature: String,
}

#[derive(Deserialize)]
struct GetBlockParams {
    block_hash: String,
}

#[derive(Deserialize)]
struct GetTransactionParams {
    tx_hash: String,
}

pub fn create_rpc_router(
    blockchain: Arc<Blockchain>,
    state: Arc<State>,
) -> impl Filter<Fn() -> Box<dyn warp::Reply>> + Clone + Send + Sync + 'static {
    warp::path!("api" / "v1" / "rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and(warp::any().map(move || blockchain.clone()))
        .and(warp::any().map(move || state.clone()))
        .and_then(rpc_handler)
}

async fn rpc_handler(
    request: RpcRequest,
    blockchain: Arc<Blockchain>,
    state: Arc<State>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = match request.method.as_str() {
        "getBalance" => {
            let params: GetBalanceParams = serde_json::from_value(request.params).map_err(|e| {
                warp::reject::custom(RpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                })
            })?;

            let balance = state.get_balance(&params.address);

            RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(balance),
                error: None,
                id: request.id,
            }
        }

        "sendTransaction" => {
            let params: SendTransactionParams = serde_json::from_value(request.params).map_err(|e| {
                warp::reject::custom(RpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                })
            })?;

            let tx = crate::transaction::Transaction::new(&params.from, &params.to, params.amount);
            blockchain.add_transaction(tx);

            RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some("tx_hash".to_string()),
                error: None,
                id: request.id,
            }
        }

        "getBlock" => {
            let params: GetBlockParams = serde_json::from_value(request.params).map_err(|e| {
                warp::reject::custom(RpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                })
            })?;

            let block = blockchain.chain.iter()
                .find(|b| b.hash == params.block_hash)
                .cloned();

            RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: block,
                error: None,
                id: request.id,
            }
        }

        "getTransaction" => {
            let params: GetTransactionParams = serde_json::from_value(request.params).map_err(|e| {
                warp::reject::custom(RpcError {
                    code: -32602,
                    message: format!("Invalid params: {}", e),
                })
            })?;

            let transaction = blockchain.chain.iter()
                .flat_map(|b| b.transactions.iter())
                .find(|t| t.hash == params.tx_hash)
                .cloned();

            RpcResponse {
                jsonrpc: "2.0".to_string(),
                result: transaction,
                error: None,
                id: request.id,
            }
        }

        _ => RpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(RpcError {
                code: -32601,
                message: "Method not found".to_string(),
            }),
            id: request.id,
        },
    };

    Ok(warp::reply::json(&response))
}
