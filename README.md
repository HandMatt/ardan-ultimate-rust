# Ultimate Rust Foundations

![](/images/ardanlabs-logo.png)

Presented by [Ardan Labs](https://www.ardanlabs.com/), Ultimate Rust: Foundations gives you a "zero to hero" class to get you started with Rust. You'll learn the basics of Rust, before diving into hands-on learning that can help you build a Rust foundation into your architecture. Take advantage of the speed and safety of Rust, tame the borrow and lifetime checkers, and pick up some tricks to help you become productive in Rust.

This is presented as 5x 1-day classes, which may be taken together or individually.

> Please have Rust installed (via RustUp) and a text editor or IDE setup when you take this class.

## Sessions

### [Getting Started with Rust](/01-GettingStarted/README.md)

> Code examples to support this readme can be found [here](/code/01_foundation)

* 1.00 - [Introduction](/01-GettingStarted/ClassIntro.md)
* 1.01 - [Setup & Update Rust](/01-GettingStarted/SetupRust.md)
* 1.02 - [IDE Configuration](/01-GettingStarted/SetupEditor.md)
* 1.03 - [Cargo Init and Hello World](/01-GettingStarted/HelloWorld.md)
* 1.04 - [Workspaces & Cargo.toml](/01-GettingStarted/Workspaces.md)
* 1.05 - [Some Rust Fundamentals](/01-GettingStarted/RustFundamentals.md)
* 1.06 - [Console Text Input](/01-GettingStarted/TextInput.md)
* 1.07 - [Create a Library](/01-GettingStarted/CreateLibrary.md)
* 1.08 - [World's Simplest Login System](/01-GettingStarted/SimpleLogin.md)
* 1.09 - [Enumerations](/01-GettingStarted/Enumerations.md)
* 1.10 - [Structures](/01-GettingStarted/Structures.md)
* 1.11 - [Vectors](/01-GettingStarted/Vectors.md)
* 1.12 - [HashMaps](/01-GettingStarted/HashMaps.md)
* 1.13 - [Serialization/Deserialization](/01-GettingStarted/Serialization.md)
* 1.14 - [Hashing Passwords](/01-GettingStarted/Hashing.md)
* 1.15 - [Start a CLI App](/01-GettingStarted/Cli.md)

### [Fearless System Thread Concurrency](/02-SystemThreads/README.md)

> Code examples to support this readme can be found [here](/code/02_threads/)

* 2.00 - [Introduction](/02-SystemThreads/Intro.md)
* 2.01 - [System Threads: Overview](/02-SystemThreads/SystemThreadOverview.md)
* 2.02 - [Create Your First Thread](/02-SystemThreads/FirstThread.md)
* 2.03 - [Spawning Threads with Parameters and Closures](/02-SystemThreads/ThreadClosures.md)
* 2.04 - [Returning Data from Threads](/02-SystemThreads/ReturnFromThreads.md)
* 2.05 - [Dividing Workloads](/02-SystemThreads/DividingWorkloads.md)
* 2.06 - [The ThreadBuilder Pattern](/02-SystemThreads/ThreadBuilder.md)
* 2.07 - [Scoped Threads for Easy Local Data Sharing](/02-SystemThreads/ScopedThreads.md)
* 2.08 - [Sharing Data with Atomics](/02-SystemThreads/Atomics.md)
* 2.09 - [Sharing Data with Mutexes](/02-SystemThreads/Mutexes.md)
* 2.10 - [Read/Write Locks](/02-SystemThreads/ReadWriteLocks.md)
* 2.11 - [Deadlocks, Panics and Poisoning](/02-SystemThreads/Deadlocks.md)
* 2.12 - [Sharing Data with Lock-Free Structures](/02-SystemThreads/LockFree.md)
* 2.13 - [Parking Threads](/02-SystemThreads/ParkingThreads.md)
* 2.14 - [Sending Data Between Threads with Channels](/02-SystemThreads/Channels.md)
* 2.15 - [Channels and Ownership](/02-SystemThreads/ChannelOwnership.md)
* 2.16 - [Sending Functions to Worker Threads](/02-SystemThreads/SendingFunctions.md)
* 2.17 - [Let's build a work queue with a thread pool](/02-SystemThreads/WorkQueue.md)
* 2.18 - [Thread CPU/Core Affinity](/02-SystemThreads/ThreadAffinity.md)
* 2.19 - [Thread Priority](/02-SystemThreads/ThreadPriority.md)
* 2.20 - [Making it Easy with Rayon](/02-SystemThreads/Rayon.md)
* 2.21 - [Scopes and Pooled Threads with Rayon](/02-SystemThreads/RayonScopes.md)

### [Async/Await Concurrency (Green Threads)](/03-Async/README.md)

> Code examples to support this readme can be found [here](/code/03_async/)

* 3.00 - [Async/Await: Overview](/03-Async/AsyncAwaitOverview.md)
* 3.01 - [Hello Async/Await](/03-Async/HelloAsync.md)
* 3.02 - [Executors and Async Run-times](/03-Async/Executors.md)
* 3.03 - [Getting Started with Tokio](/03-Async/TokioIntro.md)
* 3.04 - [Working with Tokio Futures: Awaiting, Yielding and Spawning](/03-Async/TokioFutures.md)
* 3.05 - [Blocking Tasks](/03-Async/Blocking.md)
* 3.06 - [Unit Testing Tokio](/03-Async/TokioTesting.md)
* 3.07 - [Handling Errors](/03-Async/ErrorHandling.md)
* 3.08 - [File I/O](/03-Async/FileIO.md)
* 3.09 - [Basic Network I/O](/03-Async/BasicNetworkIO.md)
* 3.10 - [Async Channels (Tokio)](/03-Async/AsyncChannels.md)
* 3.11 - [Shared State (Tokio)](/03-Async/SharedState.md)
* 3.12 - [Selecting Futures](/03-Async/SelectingFutures.md)
* 3.13 - [Pinning](/03-Async/Pinning.md)
* 3.14 - [Tokio Tracing](/03-Async/TokioTracing.md)
* 3.15 - [Working with Databases](/03-Async/Databases.md)
* 3.16 - [Axum - A Web Framework built on Tokio](/03-Async/Axum.md)
* 3.17 - [Let's Build a Thumbnail Server](/03-Async/ThumbnailServer.md)

### [Managing Memory & Resources](/04-Memory/README.md)

> Code examples to support this readme can be found [here](/code/04_mem/)

* 4.00 - [Why Haven't We Manually Managed Any Memory Yet?](/04-Memory/ManualMemoryManagement.md)
* 4.01 - [The `unsafe` Keyword](/04-Memory/Unsafe.md)
* 4.02 - [Low-Level Memory Management](/04-Memory/MemoryAllocFree.md)
* 4.03 - [The Drop Trait and RAII (Resource Acquisition is Initialization)](/04-Memory/DropTrait.md)
* 4.04 - [Reference Counting](/04-Memory/ReferenceCounting.md)
* 4.05 - [Lifetimes](/04-Memory/Lifetimes.md)
* 4.06 - [Traits](/04-Memory/Traits.md)
* 4.07 - [Generics](/04-Memory/Generics.md)
* 4.08 - [Iterators](/04-Memory/Iterators.md)
* 4.09 - [Cycles and the Difficulty of Linked Lists](/04-Memory/Cycles.md)
* 4.10 - [Memory Fragmentation, Allocators and Arenas](/04-Memory/MemoryFragmentation.md)
* 4.11 - [Packing, Reordering & Mangling](/04-Memory/Packing.md)
* 4.12 - [From Bytes to Types](/04-Memory/BytesToTypes.md)
* 4.13 - [Safely Interacting with Other Languages](/04-Memory/FFI.md)
* 4.14 - [Surprise: Memory Leaks are Safe!](/04-Memory/MemoryLeaks.md)

### [Build a Network Service](/05-Server/README.md)

> Code examples to support this readme can be found [here](/code/05_server/)

* 5.00 - [Planning Our Project](/05-Server/Planning.md)
* 5.01 - [Shared Data Structures](/05-Server/SharedDataStructures.md)
* 5.02 - [Collection Daemon Mk 1](/05-Server/CollectionDaemon1.md)
* 5.03 - [Collection Server Mk 1](/05-Server/CollectionServer1.md)
* 5.04 - [Error Handling in the Collector](/05-Server/CollectionDaemon2.md)
* 5.05 - [Setting the Collector ID](/05-Server/CollectorId.md)
* 5.06 - [Web Service Mk 1](/05-Server/WebService1.md)
* 5.07 - [Web Server](/05-Server/WebServer1.md)
* 5.08 - [Let's Use Less Bandwidth](/05-Server/CollectionDaemon3.md)
* 5.09 - [Bi-Directional Communication](/05-Server/BiDirectional.md)
* 5.10 - [Sending Commands](/05-Server/SendingCommands.md)
* 5.11 - [Prevent Unbounded Growth](/05-Server/CollectionDaemon4.md)
* 5.12 - [Giving the Collector a Diet](/05-Server/SmallerCollector.md)