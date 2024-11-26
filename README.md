# Ultimate Rust Bundle

![](images/ardanlabs-logo.png)

Presented by [Ardan Labs](https://www.ardanlabs.com/), Ultimate Rust: Foundations gives you a "zero to hero" class to get you started with Rust. You'll learn the basics of Rust, before diving into hands-on learning that can help you build a Rust foundation into your architecture. Take advantage of the speed and safety of Rust, tame the borrow and lifetime checkers, and pick up some tricks to help you become productive in Rust.

This is presented as 5x 1-day classes, which may be taken together or individually.

> Please have Rust installed (via RustUp) and a text editor or IDE setup when you take this class.

## Sessions

### [Getting Started with Rust](01-GettingStarted/README.md)

> Code examples to support this readme can be found [here](code/01_foundation)

- 01.00 - [Introduction](01-GettingStarted/ClassIntro.md)
- 01.01 - [Setup & Update Rust](01-GettingStarted/SetupRust.md)
- 01.02 - [IDE Configuration](01-GettingStarted/SetupEditor.md)
- 01.03 - [Cargo Init and Hello World](01-GettingStarted/HelloWorld.md)
- 01.04 - [Workspaces & Cargo.toml](01-GettingStarted/Workspaces.md)
- 01.05 - [Some Rust Fundamentals](01-GettingStarted/RustFundamentals.md)
- 01.06 - [Console Text Input](01-GettingStarted/TextInput.md)
- 01.07 - [Create a Library](01-GettingStarted/CreateLibrary.md)
- 01.08 - [World's Simplest Login System](01-GettingStarted/SimpleLogin.md)
- 01.09 - [Enumerations](01-GettingStarted/Enumerations.md)
- 01.10 - [Structures](01-GettingStarted/Structures.md)
- 01.11 - [Vectors](01-GettingStarted/Vectors.md)
- 01.12 - [HashMaps](01-GettingStarted/HashMaps.md)
- 01.13 - [Serialization/Deserialization](01-GettingStarted/Serialization.md)
- 01.14 - [Hashing Passwords](01-GettingStarted/Hashing.md)
- 01.15 - [Start a CLI App](01-GettingStarted/Cli.md)

### [Fearless System Thread Concurrency](02-SystemThreads/README.md)

> Code examples to support this readme can be found [here](code/02_threads/)

- 02.00 - [Introduction](02-SystemThreads/Intro.md)
- 02.01 - [System Threads: Overview](02-SystemThreads/SystemThreadOverview.md)
- 02.02 - [Create Your First Thread](02-SystemThreads/FirstThread.md)
- 02.03 - [Spawning Threads with Parameters and Closures](02-SystemThreads/ThreadClosures.md)
- 02.04 - [Returning Data from Threads](02-SystemThreads/ReturnFromThreads.md)
- 02.05 - [Dividing Workloads](02-SystemThreads/DividingWorkloads.md)
- 02.06 - [The ThreadBuilder Pattern](02-SystemThreads/ThreadBuilder.md)
- 02.07 - [Scoped Threads for Easy Local Data Sharing](02-SystemThreads/ScopedThreads.md)
- 02.08 - [Sharing Data with Atomics](02-SystemThreads/Atomics.md)
- 02.09 - [Sharing Data with Mutexes](02-SystemThreads/Mutexes.md)
- 02.10 - [Read/Write Locks](02-SystemThreads/ReadWriteLocks.md)
- 02.11 - [Deadlocks, Panics and Poisoning](02-SystemThreads/Deadlocks.md)
- 02.12 - [Sharing Data with Lock-Free Structures](02-SystemThreads/LockFree.md)
- 02.13 - [Parking Threads](02-SystemThreads/ParkingThreads.md)
- 02.14 - [Sending Data Between Threads with Channels](02-SystemThreads/Channels.md)
- 02.15 - [Channels and Ownership](02-SystemThreads/ChannelOwnership.md)
- 02.16 - [Sending Functions to Worker Threads](02-SystemThreads/SendingFunctions.md)
- 02.17 - [Let's build a work queue with a thread pool](02-SystemThreads/WorkQueue.md)
- 02.18 - [Thread CPU/Core Affinity](02-SystemThreads/ThreadAffinity.md)
- 02.19 - [Thread Priority](02-SystemThreads/ThreadPriority.md)
- 02.20 - [Making it Easy with Rayon](02-SystemThreads/Rayon.md)
- 02.21 - [Scopes and Pooled Threads with Rayon](02-SystemThreads/RayonScopes.md)

### [Async/Await Concurrency (Green Threads)](03-Async/README.md)

> Code examples to support this readme can be found [here](code/03_async/)

- 03.00 - [Async/Await: Overview](03-Async/AsyncAwaitOverview.md)
- 03.01 - [Hello Async/Await](03-Async/HelloAsync.md)
- 03.02 - [Executors and Async Run-times](03-Async/Executors.md)
- 03.03 - [Getting Started with Tokio](03-Async/TokioIntro.md)
- 03.04 - [Working with Tokio Futures: Awaiting, Yielding and Spawning](03-Async/TokioFutures.md)
- 03.05 - [Blocking Tasks](03-Async/Blocking.md)
- 03.06 - [Unit Testing Tokio](03-Async/TokioTesting.md)
- 03.07 - [Handling Errors](03-Async/ErrorHandling.md)
- 03.08 - [File I/O](03-Async/FileIO.md)
- 03.09 - [Basic Network I/O](03-Async/BasicNetworkIO.md)
- 03.10 - [Async Channels (Tokio)](03-Async/AsyncChannels.md)
- 03.11 - [Shared State (Tokio)](03-Async/SharedState.md)
- 03.12 - [Selecting Futures](03-Async/SelectingFutures.md)
- 03.13 - [Pinning](03-Async/Pinning.md)
- 03.14 - [Tokio Tracing](03-Async/TokioTracing.md)
- 03.15 - [Working with Databases](03-Async/Databases.md)
- 03.16 - [Axum - A Web Framework built on Tokio](03-Async/Axum.md)
- 03.17 - [Let's Build a Thumbnail Server](03-Async/ThumbnailServer.md)

### [Managing Memory & Resources](04-Memory/README.md)

> Code examples to support this readme can be found [here](code/04_mem/)

- 04.00 - [Why Haven't We Manually Managed Any Memory Yet?](04-Memory/ManualMemoryManagement.md)
- 04.01 - [The `unsafe` Keyword](04-Memory/Unsafe.md)
- 04.02 - [Low-Level Memory Management](04-Memory/MemoryAllocFree.md)
- 04.03 - [The Drop Trait and RAII (Resource Acquisition is Initialization)](04-Memory/DropTrait.md)
- 04.04 - [Reference Counting](04-Memory/ReferenceCounting.md)
- 04.05 - [Lifetimes](04-Memory/Lifetimes.md)
- 04.06 - [Traits](04-Memory/Traits.md)
- 04.07 - [Generics](04-Memory/Generics.md)
- 04.08 - [Iterators](04-Memory/Iterators.md)
- 04.09 - [Cycles and the Difficulty of Linked Lists](04-Memory/Cycles.md)
- 04.10 - [Memory Fragmentation, Allocators and Arenas](04-Memory/MemoryFragmentation.md)
- 04.11 - [Packing, Reordering & Mangling](04-Memory/Packing.md)
- 04.12 - [From Bytes to Types](04-Memory/BytesToTypes.md)
- 04.13 - [Safely Interacting with Other Languages](04-Memory/FFI.md)
- 04.14 - [Surprise: Memory Leaks are Safe!](04-Memory/MemoryLeaks.md)

### [Build a Network Service](05-Server/README.md)

> Code examples to support this readme can be found [here](code/05_server/)

- 05.00 - [Planning Our Project](05-Server/Planning.md)
- 05.01 - [Shared Data Structures](05-Server/SharedDataStructures.md)
- 05.02 - [Collection Daemon Mk 1](05-Server/CollectionDaemon1.md)
- 05.03 - [Collection Server Mk 1](05-Server/CollectionServer1.md)
- 05.04 - [Error Handling in the Collector](05-Server/CollectionDaemon2.md)
- 05.05 - [Setting the Collector ID](05-Server/CollectorId.md)
- 05.06 - [Web Service Mk 1](05-Server/WebService1.md)
- 05.07 - [Web Server](05-Server/WebServer1.md)
- 05.08 - [Let's Use Less Bandwidth](05-Server/CollectionDaemon3.md)
- 05.09 - [Bi-Directional Communication](05-Server/BiDirectional.md)
- 05.10 - [Sending Commands](05-Server/SendingCommands.md)
- 05.11 - [Prevent Unbounded Growth](05-Server/CollectionDaemon4.md)
- 05.12 - [Giving the Collector a Diet](05-Server/SmallerCollector.md)

---

### [Rust Best Practice](06-BestPractice/README.md)

#### Tooling

- 06.00 - [Formatting](06-BestPractice/FormattingCode.md)
- 06.01 - [Clippy (the linter)](06-BestPractice/Clippy.md)
- 06.02 - [Documenting Your Code](06-BestPractice/Documentation.md)
- 06.03 - [Understanding Dependencies](06-BestPractice/Dependencies.md)
- 06.04 - [Manage Your Own Dependencies](06-BestPractice/ManageDependencies.md)
- 06.05 - [Checking for Vulnerabilities](06-BestPractice/Audit.md)
- 06.06 - [Check for Outdated Dependencies](06-BestPractice/Outdated.md)
- 06.07 - [Denying Dependencies by Licencing](06-BestPractice/Deny.md)

#### Code Best Practices

- 06.08 - [Favour Iterators](06-BestPractice/Iterators.md)
- 06.09 - [Minimize Cloning](06-BestPractice/Clone.md)
- 06.10 - [Don't Emulate OOP](06-BestPractice/OOPs.md)
- 06.11 - [Favour Small Functions](06-BestPractice/SmallFunctions.md)
- 06.12 - [Clever Code](06-BestPractice/Cleverness.md)
- 06.13 - [Floating Point Numbers](06-BestPractice/Floats.md)
- 06.14 - [Platform & Feature Specific Code](06-BestPractice/PlatformSpecific.md)

#### General Best Practices

- 06.15 - [TANSTAAFL](06-BestPractice/TANSTAAFL.md)
- 06.16 - [YAGNI](06-BestPractice/YAGNI.md)
- 06.17 - [Domain Boundaries](06-BestPractice/DomainBoundaries.md)
- 06.18 - [Taming Compile Times](06-BestPractice/CompileTimes.md)

---

### [Building a REST Service](07-RESTService/README.md)

> Code samples to support this readme can be found [here](code/07_rest_service)

- 07.00 - [Introduction](07-RESTService/intro.md)
- 07.01 - [REST Service](07-RESTService/rest_service.md)
- 07.02 - [Minimal HTTP Server](07-RESTService/simple_http_server.md)
- 07.03 - [Service Stack](07-RESTService/axum_hyper_tower_tokio.md)
- 07.04 - [Extractors](07-RESTService/axum_extractors.md)
  - 07.05 - [Path Extraction](07-RESTService/axum_extractors_path.md)
  - 07.06 - [Query Extraction](07-RESTService/axum_extractors_query.md)
  - 07.07 - [Header Extraction](07-RESTService/axum_extractors_headers.md)
  - 07.08 - [More Extractors](07-RESTService/axum_extractors_more.md)
- 07.09 - [Add a Simple Tower Layer (State)](07-RESTService/simple_tower_layer.md)
- 07.10 - [Add a Simple Tower Layer (Mutable State)](07-RESTService/simple_tower_layer_mut.md)
- 07.11 - [Multiple States - Extension Layers](07-RESTService/simple_tower_layer_multi_state.md)
- 07.12 - [Quick Recap on State and Layers](07-RESTService/simple_tower_recap.md)
- 07.13 - [Nesting Multiple Routers](07-RESTService/nesting.md)
- 07.14 - [Nested Routers with State](07-RESTService/nesting_state.md)
- 07.15 - [Calling Other Services with Hyper](07-RESTService/calling_other_services.md)
- 07.16 - [Returning Status Codes](07-RESTService/status_codes.md)
- 07.17 - [Using IntoResponse](07-RESTService/into_response.md)
- 07.18 - [Error Handling with IntoResponse](07-RESTService/error_handling.md)
- 07.19 - [Quick Recap on Nesting, Making Calls and Responses](07-RESTService/error_handling_recap.md)
- 07.20 - [Serving Static Content with Tower](07-RESTService/static_content.md)
- 07.21 - [Simple Header-Based Authentication](07-RESTService/header_auth1.md)
- 07.22 - [Simple Header-Based Auth with Middleware](07-RESTService/header_auth2.md)
- 07.23 - [Middleware Auth with Injection](07-RESTService/header_auth3.md)
- 07.24 - [Selectively Applying Layers](07-RESTService/layer_targets.md)
- 07.25 - [Router Layers](07-RESTService/layer_router.md)
- 07.26 - [Layer Recap](07-RESTService/layer_recap.md)

### [Tracing](08-Tracing/README.md)

> Code samples to support this readme can be found [here](code/08_tracing)

- 08.00 - [Minimal Example](08-Tracing/axum_tracing_minimal.md)
- 08.01 - [Logging Axum/Tower](08-Tracing/axum_tracing_tower.md)
- 08.02 - [Timing Spans](08-Tracing/axum_spans_own.md)
- 08.03 - [Axum Spans](08-Tracing/axum_spans.md)
- 08.04 - [Logging to a File](08-Tracing/axum_file_log.md)
- 08.05 - [Structured Logging to JSON](08-Tracing/axum_log_json.md)
- 08.06 - [OpenTelemetry](08-Tracing/otel_top.md)
  - 08.07 - [Hello Telemetry](08-Tracing/otel_hello.md) 

### [OpenAPI Documentation](09-OpenAPIDocumenation/openapi.md)

> Code sample to support this readme can be found [here](code/09_openapi)

### [Handling Service Configuration](10-HandlingServiceConfiguration/README.md)

> Code samples to support this readme can be found [here](code/10_config)

- 10.00 - [Environment Variables with .env](10-HandlingServiceConfiguration/config_dot_env.md)
- 10.01 - [The Config Crate - Basics](10-HandlingServiceConfiguration/config_crate.md)
- 10.02 - [Loading Config via HTTP](10-HandlingServiceConfiguration/config_http.md)
- 10.03 - [CLI configuration with Clap](10-HandlingServiceConfiguration/config_clap.md)
- 10.04 - [Recap](10-HandlingServiceConfiguration/config_recap1.md)

### [gRPC](11-gRPC/README.md)

> Code samples to support this readme can be found [here](code/11_grpc)

- 11.00 - [Hello Tonic - Protocol Definition](11-gRPC/grpc_hello.md)
- 11.01 - [Hello Tonic - Project Definition and Build](11-gRPC/grpc_hello2.md)
- 11.02 - [Hello Tonic - The Server](11-gRPC/grpc_hello3.md)
- 11.03 - [Hello Tonic - The Client](11-gRPC/grpc_hello4.md)
- 11.04 - [gRPC Streaming](11-gRPC/grpc_stream.md)
- 11.05 - [gRPC Streaming - Protocol Definition](11-gRPC/grpc_stream2.md)
- 11.06 - [gRPC Streaming - The Server](11-gRPC/grpc_stream3.md)
- 11.07 - [gRPC Streaming - The Client](11-gRPC/grpc_stream4.md)
- 11.08 - [Recap So Far](11-gRPC/grpc_recap1.md)
- 11.09 - [Authentication](11-gRPC/grpc_auth.md)
- 11.10 - [Tracing](11-gRPC/grpc_tracing.md)
- 11.11 - [When to use gRPC](11-gRPC/grpc_conclude.md)

### [Web Sockets](12-WebSockets/README.md)

> Code samples to support this readme can be found [here](code/12_websockets)

- 12.00 - [Minimal Echo Server](12-WebSockets/ws_echo_server.md)
- 12.01 - [A native WS client](12-WebSockets/ws_client.md)
- 12.02 - [JSON](12-WebSockets/ws_json.md)

### [Service Deployment](13-ServiceDeployment/README.md)

- 13.00 - [Test Service](13-ServiceDeployment/svc_deploy_test.md)
- 13.01 - [Native Host Deployment](13-ServiceDeployment/svc_deploy_native.md)
- 13.02 - [Docker Deployment](13-ServiceDeployment/svc_deploy_docker.md)

### [Service Design](14-ServiceDesign/README.md)

- 14.00 - [Understanding Your Company Architecture](14-ServiceDesign/design_company_arch.md)
- 14.01 - [Designing Individual Services](14-ServiceDesign/design_svc_intro.md)
  - 14.02 - [Layout](14-ServiceDesign/design_svc_layout.md)
  - 14.03 - [Per-Service Configuration](14-ServiceDesign/design_svc_config.md)
  - 14.04 - [Per-Service Database](14-ServiceDesign/design_svc_db.md)
  - 14.05 - [Layers and API Systems for Other Systems](14-ServiceDesign/design_svc_layer.md)
  - 14.06 - [Finally, the service module itself](14-ServiceDesign/design_svc_module.md)
- 14.07 - [Combining Services into a Modular Monolith](14-ServiceDesign/design_svc_modular_monolith.md)
- 14.08 - [Service Exposure](14-ServiceDesign/design_svc_exposure.md)
- 14.09 - [Scaling Out](14-ServiceDesign/design_svc_scaling_out.md)

---

> Original materials found on Herbert's Github:
>
> - [Ultimate Rust - Foundations](https://github.com/thebracket/ArdanUltimateRustFoundations)
> - [Ultimate Rust - 5 Days](https://github.com/thebracket/ArdanUltimateRust-5Days)
> - [Ultimate Rust - Best Practices](https://github.com/thebracket/Ardan-NR-2023-07)
> - [Rust as a Service](https://github.com/thebracket/ArdanRustService)
