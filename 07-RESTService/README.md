# Rust as a Service

> Please have Rust installed (via RustUp) and a working IDE/editor setup.

Brief overview of the content

# Summary

- [Introduction](./intro.md)
- [REST Service](./rest_service.md)
  - [Minimal HTTP Server](./simple_http_server.md)
  - [Service Stack](./axum_hyper_tower_tokio.md)
  - [Extractors](./axum_extractors.md)
    - [Path Extraction](./axum_extractors_path.md)
    - [Query Extraction](./axum_extractors_query.md)
    - [Header Extraction](./axum_extractors_headers.md)
    - [More Extractors](./axum_extractors_more.md)
  - [Add a Simple Tower Layer (State)](./simple_tower_layer.md)
  - [Add a Simple Tower Layer (Mutable State)](./simple_tower_layer_mut.md)
  - [Multiple States - Extension Layers](./simple_tower_layer_multi_state.md)
  - [Quick Recap on State and Layers](./simple_tower_recap.md)
  - [Nesting Multiple Routers](./nesting.md)
  - [Nested Routers with State](./nesting_state.md)
  - [Calling Other Services with Hyper](./calling_other_services.md)
  - [Returning Status Codes](./status_codes.md)
  - [Using IntoResponse](./into_response.md)
  - [Error Handling with IntoResponse](./error_handling.md)
  - [Quick Recap on Nesting, Making Calls and Responses](./error_handling_recap.md)
  - [Serving Static Content with Tower](./static_content.md)
  - [Simple Header-Based Authentication](./header_auth1.md)
  - [Simple Header-Based Auth with Middleware](./header_auth2.md)
  - [Middleware Auth with Injection](./header_auth3.md)
  - [Selectively Applying Layers](./layer_targets.md)
  - [Router Layers](./layer_router.md)
  - [Layer Recap](./layer_recap.md)
