// Copyright 2020 Oxide Computer Company

use dropshot::endpoint;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::Query;
use dropshot::RequestContext;
use std::sync::Arc;

#[allow(dead_code)]
struct QueryParams {
    x: String,
    y: u32,
}

#[endpoint {
    method = GET,
    path = "/test",
}]
async fn bad_endpoint(
    _rqctx: Arc<RequestContext>,
    _params: Query<QueryParams>,
) -> Result<HttpResponseOk<()>, HttpError> {
    Ok(HttpResponseOk(()))
}

fn main() {}