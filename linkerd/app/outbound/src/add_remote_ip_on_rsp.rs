//! Adds `l5d-remote-ip` headers to http::Responses derived from the
//! `remote` of a `tls::accept::Meta`.

use super::HttpEndpoint;
use super::Target;
use bytes::Bytes;
use http::header::HeaderValue;
use linkerd2_app_core::{
    proxy::http::add_header::{self, response::ResHeader, Layer},
    L5D_REMOTE_IP,
};

pub fn layer() -> Layer<&'static str, Target<HttpEndpoint>, ResHeader> {
    add_header::response::layer(L5D_REMOTE_IP, |endpoint: &Target<HttpEndpoint>| {
        HeaderValue::from_maybe_shared(Bytes::from(endpoint.inner.addr.ip().to_string())).ok()
    })
}
