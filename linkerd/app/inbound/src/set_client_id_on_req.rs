//! Adds `l5d-client-id` headers to http::Requests derived from the
//! TlsIdentity of a `tls::accept::Meta`.

use http::HeaderValue;
use linkerd2_app_core::{
    proxy::http::add_header::{self, request::ReqHeader, Layer},
    Conditional, 
    L5D_CLIENT_ID,
};

use crate::TcpAccept;

pub fn layer() -> Layer<&'static str, TcpAccept, ReqHeader> {
    add_header::request::layer(L5D_CLIENT_ID, |source: &TcpAccept| {
        if let Conditional::Some(ref id) = source.peer_id {
            if let Ok(value) = HeaderValue::from_str(id.as_ref()) {
                return Some(value);
            }
        }
        None
    })
}