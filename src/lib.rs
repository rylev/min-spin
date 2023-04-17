use bindings::inbound_http;

struct Component;

impl inbound_http::InboundHttp for Component {
    fn handle_request(_request: inbound_http::Request) -> inbound_http::Response {
        inbound_http::Response {
            status: 200,
            body: Some(String::from("Hello, world!").into_bytes()),
            headers: None,
        }
    }
}

bindings::export!(Component);
