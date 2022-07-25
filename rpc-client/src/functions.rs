use jsonrpc_v2::Error;

use rpc_server::{
    api::{NetworkPutFileParams, NetworkPutFileResult, NETWORK_PUT_FILE},
    rpc::api::{
        NetworkGetParams, NetworkGetResult, NetworkPutCarParams, NetworkPutCarResult, NETWORK_GET,
        NETWORK_PUT_CAR,
    },
};

use crate::{
    call,
    HttpMethod::{Get, Put},
};

pub type Result<T> = anyhow::Result<T, Error>;

pub async fn get_block(params: NetworkGetParams) -> Result<NetworkGetResult> {
    call(NETWORK_GET, params, Get).await
}

pub async fn put_car(params: NetworkPutCarParams) -> Result<NetworkPutCarResult> {
    call(NETWORK_PUT_CAR, params, Put).await
}

pub async fn put_file(params: NetworkPutFileParams) -> Result<NetworkPutFileResult> {
    call(NETWORK_PUT_FILE, params, Put).await
}
