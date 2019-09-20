#![doc(html_root_url = "https://docs.rs/tracing/0.1.9")]
#![deny(missing_debug_implementations, missing_docs, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

//! A scoped, structured logging and diagnostics system.
//!
//! # Overview
//!
//! `tracing` is a framework for instrumenting Rust programs to collect
//! structured, event-based diagnostic information.
//!
//! In asynchronous systems like Tokio, interpreting traditional log messages can
//! often be quite challenging. Since individual tasks are multiplexed on the same
//! thread, associated events and log lines are intermixed making it difficult to
//! trace the logic flow. `tracing` expands upon logging-style diagnostics by
//! allowing libraries and applications to record structured events with additional
//! information about *temporality* and *causality* — unlike a log message, a span
//! in `tracing` has a beginning and end time, may be entered and exited by the
//! flow of execution, and may exist within a nested tree of similar spans. In
//! addition, `tracing` spans are *structured*, with the ability to record typed
//! data as well as textual messages.
//!
//! The `tracing` crate provides the APIs necessary for instrumenting libraries
//! and applications to emit trace data.
//!
//! # Core Concepts
//!
//! The core of `tracing`'s API is composed of _spans_, _events_ and
//! _subscribers_. We'll cover these in turn.
//!
//! ## Spans
//!
//! To record the flow of execution through a program, `tracing` introduces the
//! concept of [spans][span]. Unlike a log line that represents a _moment in
//! time_, a span represents a _period of time_ with a beginning and an end. When a
//! program begins executing in a context or performing a unit of work, it
//! _enters_ that context's span, and when it stops executing in that context,
//! it _exits_ the span. The span in which a thread is currently executing is
//! referred to as that thread's _current_ span.
//!
//! For example:
//! ```
//! use tracing::{span, Level};
//! # fn main() {
//! let span = span!(Level::TRACE, "my_span");
//! // `enter` returns a RAII guard which, when dropped, exits the span. this
//! // indicates that we are in the span for the current lexical scope.
//! let _enter = span.enter();
//! // perform some work in the context of `my_span`...
//! # }
//!```
//!
//! The [`span` module][span]'s documentation provides further details on how to
//! use spans.
//!
//! ## Events
//!
//! An [`Event`] represents a _moment_ in time. It signifies something that
//! happened while a trace was being recorded. `Event`s are comparable to the log
//! records emitted by unstructured logging code, but unlike a typical log line,
//! an `Event` may occur within the context of a span.
//!
//! For example:
//! ```
//! use tracing::{event, span, Level};
//!
//! # fn main() {
//! // records an event outside of any span context:
//! event!(Level::INFO, "something happened");
//!
//! let span = span!(Level::INFO, "my_span");
//! let _guard = span.enter();
//!
//! // records an event within "my_span".
//! event!(Level::DEBUG, "something happened inside my_span");
//! # }
//!```
//!
//! In general, events should be used to represent points in time _within_ a
//! span — a request returned with a given status code, _n_ new items were
//! taken from a queue, and so on.
//!
//! The [`Event` struct][`Event`] documentation provides further details on using
//! events.
//!
//! ## Subscribers
//!
//! As `Span`s and `Event`s occur, they are recorded or aggregated by
//! implementations of the [`Subscriber`] trait. `Subscriber`s are notified
//! when an `Event` takes place and when a `Span` is entered or exited. These
//! notifications are represented by the following `Subscriber` trait methods:
//!
//! + [`event`][Subscriber::event], called when an `Event` takes place,
//! + [`enter`], called when execution enters a `Span`,
//! + [`exit`], called when execution exits a `Span`
//!
//! In addition, subscribers may implement the [`enabled`] function to _filter_
//! the notifications they receive based on [metadata] describing each `Span`
//! or `Event`. If a call to `Subscriber::enabled` returns `false` for a given
//! set of metadata, that `Subscriber` will *not* be notified about the
//! corresponding `Span` or `Event`. For performance reasons, if no currently
//! active subscribers express  interest in a given set of metadata by returning
//! `true`, then the corresponding `Span` or `Event` will never be constructed.
//!
//! # Usage
//!
//! First, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tracing = "0.1"
//! ```
//!
//! ## Recording Spans and Events
//!
//! Spans and events are recorded using macros.
//!
//! ### Spans
//!
//! The [`span!`] macro expands to a [`Span` struct][`Span`] which is used to
//! record a span. The [`Span::enter`] method on that struct records that the
//! span has been entered, and returns a [RAII] guard object, which will exit
//! the span when dropped.
//!
//! For example:
//!
//! ```rust
//! use tracing::{span, Level};
//! # fn main() {
//! // Construct a new span named "my span" with trace log level.
//! let span = span!(Level::TRACE, "my span");
//!
//! // Enter the span, returning a guard object.
//! let _enter = span.enter();
//!
//! // Any trace events that occur before the guard is dropped will occur
//! // within the span.
//!
//! // Dropping the guard will exit the span.
//! # }
//! ```
//!
//! The [`#[instrument]`][instrument] attribute provides an easy way to
//! add `tracing` spans to functions. A function annotated with `#[instrument]`
//! will create and enter a span with that function's name every time the
//! function is called, with arguments to that function will be recorded as
//! fields using `fmt::Debug`.
//!
//! For example:
//! ```
//! use tracing::{Level, event, instrument};
//!
//! #[instrument]
//! pub fn my_function(my_arg: usize) {
//!     // This event will be recorded inside a span named `my_function` with the
//!     // field `my_arg`.
//!     event!(Level::INFO, "inside my_function!");
//!     // ...
//! }
//! # fn main() {}
//! ```
//!
//! **Note**: using `#[instrument]` on `async fn`s requires the
//! [`tracing-futures`] crate as a dependency, as well.
//!
//! You can find more examples showing how to use this crate [here][examples].
//!
//! [RAII]: https://github.com/rust-unofficial/patterns/blob/master/patterns/RAII.md
//! [examples]: https://github.com/tokio-rs/tracing/tree/master/tracing/examples
//!
//! ### Events
//!
//! [`Event`]s are recorded using the [`event!`] macro:
//!
//! ```rust
//! # fn main() {
//! use tracing::{event, Level};
//! event!(Level::INFO, "something has happened!");
//! # }
//! ```
//!
//! ## Using the Macros
//!
//! The [`span!`] and [`event!`] macros use fairly similar syntax, with some
//! exceptions.
//!
//! ### Configuring Attributes
//!
//! Both macros require a [`Level`] specifying the verbosity of the span or
//! event. Optionally, the [target] and [parent span] may be overridden. If the
//! target and parent span are not overridden, they will default to the
//! module path where the macro was invoked and the current span (as determined
//! by the subscriber), respectively.
//!
//! For example:
//!
//! ```
//! # use tracing::{span, event, Level};
//! # fn main() {
//! span!(target: "app_spans", Level::TRACE, "my span");
//! event!(target: "app_events", Level::INFO, "something has happened!");
//! # }
//! ```
//! ```
//! # use tracing::{span, event, Level};
//! # fn main() {
//! let span = span!(Level::TRACE, "my span");
//! event!(parent: &span, Level::INFO, "something has happened!");
//! # }
//! ```
//!
//! The span macros also take a string literal after the level, to set the name
//! of the span.
//!
//! ### Recording Fields
//!
//! Structured fields on spans and events are specified using the syntax
//! `field_name = field_value`. Fields are separated by commas.
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! // records an event with two fields:
//! //  - "answer", with the value 42
//! //  - "question", with the value "life, the universe and everything"
//! event!(Level::INFO, answer = 42, question = "life, the universe, and everything");
//! # }
//! ```
//!
//! As shorthand, local variables may be used as field values without an
//! assignment, similar to [struct initializers]. For example:
//!
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! let user = "ferris";
//!
//! span!(Level::TRACE, "login", user);
//! // is equivalent to:
//! span!(Level::TRACE, "login", user = user);
//! # }
//!```
//!
//! Field names can include dots, but should not be terminated by them:
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! let user = "ferris";
//! let email = "ferris@rust-lang.org";
//! span!(Level::TRACE, "login", user, user.email = email);
//! # }
//!```
//!
//! Since field names can include dots, fields on local structs can be used
//! using the local variable shorthand:
//! ```
//! # use tracing::{span, Level};
//! # fn main() {
//! # struct User {
//! #    name: &'static str,
//! #    email: &'static str,
//! # }
//! let user = User {
//!     name: "ferris",
//!     email: "ferris@rust-lang.org",
//! };
//! // the span will have the fields `user.name = "ferris"` and
//! // `user.email = "ferris@rust-lang.org"`.
//! span!(Level::TRACE, "login", user.name, user.email);
//! # }
//!```
//!
//! The `?` sigil is shorthand that specifies a field should be recorded using
//! its [`fmt::Debug`] implementation:
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! #[derive(Debug)]
//! struct MyStruct {
//!     field: &'static str,
//! }
//!
//! let my_struct = MyStruct {
//!     field: "Hello world!"
//! };
//!
//! // `my_struct` will be recorded using its `fmt::Debug` implementation.
//! event!(Level::TRACE, greeting = ?my_struct);
//! // is equivalent to:
//! event!(Level::TRACE, greeting = tracing::field::debug(&my_struct));
//! # }
//! ```
//!
//! The `%` sigil operates similarly, but indicates that the value should be
//! recorded using its [`fmt::Display`] implementation:
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! # #[derive(Debug)]
//! # struct MyStruct {
//! #     field: &'static str,
//! # }
//! #
//! # let my_struct = MyStruct {
//! #     field: "Hello world!"
//! # };
//! // `my_struct.field` will be recorded using its `fmt::Display` implementation.
//! event!(Level::TRACE, greeting = %my_struct.field);
//! // is equivalent to:
//! event!(Level::TRACE, greeting = tracing::field::display(&my_struct.field));
//! # }
//! ```
//!
//! The `%` and `?` sigils may also be used with local variable shorthand:
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! # #[derive(Debug)]
//! # struct MyStruct {
//! #     field: &'static str,
//! # }
//! #
//! # let my_struct = MyStruct {
//! #     field: "Hello world!"
//! # };
//! // `my_struct.field` will be recorded using its `fmt::Display` implementation.
//! event!(Level::TRACE,  %my_struct.field);
//! # }
//! ```
//!
//! Note that a span may have up to 32 fields. The following will not compile:
//!
//! ```rust,compile_fail
//! # use tracing::Level;
//! # fn main() {
//! let bad_span = span!(
//!     Level::TRACE,
//!     "too many fields!",
//!     a = 1, b = 2, c = 3, d = 4, e = 5, f = 6, g = 7, h = 8, i = 9,
//!     j = 10, k = 11, l = 12, m = 13, n = 14, o = 15, p = 16, q = 17,
//!     r = 18, s = 19, t = 20, u = 21, v = 22, w = 23, x = 24, y = 25,
//!     z = 26, aa = 27, bb = 28, cc = 29, dd = 30, ee = 31, ff = 32, gg = 33
//! );
//! # }
//! ```
//!
//! Finally, events may also include human-readable messages, in the form of a
//! [format string][fmt] and (optional) arguments, **after** the event's
//! key-value fields. If a format string and arguments are provided,
//! they will implicitly create a new field named `message` whose value is the
//! provided set of format arguments.
//!
//! For example:
//!
//! ```
//! # use tracing::{event, Level};
//! # fn main() {
//! let question = "the answer to the ultimate question of life, the universe, and everything";
//! let answer = 42;
//! // records an event with the following fields:
//! // - `question.answer` with the value 42,
//! // - `question.tricky` with the value `true`,
//! // - "message", with the value "the answer to the ultimate question of life, the
//! //    universe, and everthing is 42."
//! event!(
//!     Level::DEBUG,
//!     question.answer = answer,
//!     question.tricky = true,
//!     "the answer to {} is {}.", question, answer
//! );
//! # }
//! ```
//!
//! Specifying a formatted message in this manner does not allocate by default.
//!
//! [struct initializers]: https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name
//! [target]: struct.Metadata.html#method.target
//! [parent span]: span/struct.Attributes.html#method.parent
//! [determined contextually]: span/struct.Attributes.html#method.is_contextual
//! [`fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
//! [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
//! [fmt]: https://doc.rust-lang.org/std/fmt/#usage
//!
//! ### Shorthand Macros
//!
//! `tracing` also offers a number of macros with preset verbosity levels.
//! The [`trace!`], [`debug!`], [`info!`], [`warn!]`, and [`error!`] behave
//! similarly to the [`event!`] macro, but with the [`Level`] argument already
//! specified, while the corresponding [`trace_span!`], [`debug_span!`],
//! [`info_span!`], [`warn_span!`], and [`error_span!`] macros are the same,
//! but for the `[span!`] macro.
//!
//! These are intended both as a shorthand, and for compatibility with the [`log`]
//! crate (see the next section).
//!
//! [`span!`]: macro.span.html
//! [`event!`]: macro.event.html
//! [`trace!`]: macro.trace.html
//! [`debug!`]: macro.debug.html
//! [`info!`]: macro.info.html
//! [`warn!`]: macro.warn.html
//! [`error!`]: macro.error.html
//! [`trace_span!`]: macro.trace_span.html
//! [`debug_span!`]: macro.debug_span.html
//! [`info_span!`]: macro.info_span.html
//! [`warn_span!`]: macro.warn_span.html
//! [`error_span!`]: macro.error_span.html
//! [`Level`]: struct.Level.html
//!
//! ### For `log` Users
//!
//! Users of the [`log`] crate should note that `tracing` exposes a set of
//! macros for creating `Event`s (`trace!`, `debug!`, `info!`, `warn!`, and
//! `error!`) which may be invoked with the same syntax as the similarly-named
//! macros from the `log` crate. Often, the process of converting a project to
//! use `tracing` can begin with a simple drop-in replacement.
//!
//! Let's consider the `log` crate's yak-shaving example:
//!
//! ```rust
//! use tracing::{info, span, warn, Level};
//!
//! # #[derive(Debug)] pub struct Yak(String);
//! # impl Yak { fn shave(&mut self, _: u32) {} }
//! # fn find_a_razor() -> Result<u32, u32> { Ok(1) }
//! # fn main() {
//! pub fn shave_the_yak(yak: &mut Yak) {
//!     let span = span!(Level::TRACE, "shave_the_yak", ?yak);
//!     let _enter = span.enter();
//!
//!     // Since the span is annotated with the yak, it is part of the context
//!     // for everything happening inside the span. Therefore, we don't need
//!     // to add it to the message for this event, as the `log` crate does.
//!     info!(target: "yak_events", "Commencing yak shaving");
//!     loop {
//!         match find_a_razor() {
//!             Ok(razor) => {
//!                 // We can add the razor as a field rather than formatting it
//!                 // as part of the message, allowing subscribers to consume it
//!                 // in a more structured manner:
//!                 info!(%razor, "Razor located");
//!                 yak.shave(razor);
//!                 break;
//!             }
//!             Err(err) => {
//!                 // However, we can also create events with formatted messages,
//!                 // just as we would for log records.
//!                 warn!("Unable to locate a razor: {}, retrying", err);
//!             }
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! ## In libraries
//!
//! Libraries should link only to the `tracing` crate, and use the provided
//! macros to record whatever information will be useful to downstream
//! consumers.
//!
//! ## In executables
//!
//! In order to record trace events, executables have to use a `Subscriber`
//! implementation compatible with `tracing`. A `Subscriber` implements a
//! way of collecting trace data, such as by logging it to standard output.
//!
//! This library does not contain any `Subscriber` implementations; these are
//! provided by [other crates](#related-crates).
//!
//! The simplest way to use a subscriber is to call the [`set_global_default`]
//! function:
//!
//! ```
//! extern crate tracing;
//! # pub struct FooSubscriber;
//! # use tracing::{span::{Id, Attributes, Record}, Metadata};
//! # impl tracing::Subscriber for FooSubscriber {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &tracing::Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! # }
//! # impl FooSubscriber {
//! #   fn new() -> Self { FooSubscriber }
//! # }
//! # fn main() {
//!
//! let my_subscriber = FooSubscriber::new();
//! tracing::subscriber::set_global_default(my_subscriber)
//!     .expect("setting tracing default failed");
//! # }
//! ```
//!
//! **Note:** Libraries should *NOT* call `set_global_default()`! That will
//! cause conflicts when executables try to set the default later.
//!
//! This subscriber will be used as the default in all threads for the
//! remainder of the duration of the program, similar to setting the logger
//! in the `log` crate.
//!
//! In addition, the default subscriber can be set through using the
//! [`with_default`] function. This follows the `tokio` pattern of using
//! closures to represent executing code in a context that is exited at the end
//! of the closure. For example:
//!
//! ```rust
//! # pub struct FooSubscriber;
//! # use tracing::{span::{Id, Attributes, Record}, Metadata};
//! # impl tracing::Subscriber for FooSubscriber {
//! #   fn new_span(&self, _: &Attributes) -> Id { Id::from_u64(0) }
//! #   fn record(&self, _: &Id, _: &Record) {}
//! #   fn event(&self, _: &tracing::Event) {}
//! #   fn record_follows_from(&self, _: &Id, _: &Id) {}
//! #   fn enabled(&self, _: &Metadata) -> bool { false }
//! #   fn enter(&self, _: &Id) {}
//! #   fn exit(&self, _: &Id) {}
//! # }
//! # impl FooSubscriber {
//! #   fn new() -> Self { FooSubscriber }
//! # }
//! # fn main() {
//!
//! let my_subscriber = FooSubscriber::new();
//! # #[cfg(feature = "std")]
//! tracing::subscriber::with_default(my_subscriber, || {
//!     // Any trace events generated in this closure or by functions it calls
//!     // will be collected by `my_subscriber`.
//! })
//! # }
//! ```
//!
//! This approach allows trace data to be collected by multiple subscribers
//! within different contexts in the program. Note that the override only applies to the
//! currently executing thread; other threads will not see the change from with_default.
//!
//! Any trace events generated outside the context of a subscriber will not be collected.
//!
//! Once a subscriber has been set, instrumentation points may be added to the
//! executable using the `tracing` crate's macros.
//!
//! ## Related Crates
//!
//! In addition to `tracing` and `tracing-core`, the [`tokio-rs/tracing`] repository
//! contains several additional crates designed to be used with the `tracing` ecosystem.
//! This includes a collection of `Subscriber` implementations, as well as utility
//! and adapter crates to assist in writing `Subscriber`s and instrumenting
//! applications.
//!
//! In particular, the following crates are likely to be of interest:
//!
//!  - [`tracing-futures`] provides a compatibility layer with the `futures`
//!    crate, allowing spans to be attached to `Future`s, `Stream`s, and `Executor`s.
//!  - [`tracing-subscriber`] provides `Subscriber` implementations and
//!    utilities for working with `Subscriber`s. This includes a [`FmtSubscriber`]
//!    `FmtSubscriber` for logging formatted trace data to stdout, with similar
//!    filtering and formatting to the [`env_logger`] crate.
//!  - [`tracing-log`] provides a compatibility layer with the [`log`] crate,
//!    allowing log messages to be recorded as `tracing` `Event`s within the
//!    trace tree. This is useful when a project using `tracing` have
//!    dependencies which use `log`.
//!  - [`tracing-timing`] implements inter-event timing metrics on top of `tracing`.
//!    It provides a subscriber that records the time elapsed between pairs of
//!    `tracing` events and generates histograms.
//!
//! **Note:** that some of the ecosystem crates are currently unreleased and
//! undergoing active development. They may be less stable than `tracing` and
//! `tracing-core`.
//!
//! ##  Crate Feature Flags
//!
//! The following crate feature flags are available:
//!
//! * A set of features controlling the [static verbosity level].
//! * `log`: causes trace instrumentation points to emit [`log`] records as well
//!   as trace events, if a default `tracing` subscriber has not been set. This
//!   is intended for use in libraries whose users may be using either `tracing`
//!   or `log`.
//!   **Note:** `log` support will not work when `tracing` is renamed in `Cargo.toml`,
//!   due to oddities in macro expansion.
//! * `log-always`: Emit `log` records from all `tracing` spans and events, even
//!   a `tracing` subscriber has been set. This should be set only by
//!   applications which intend to collect traces and logs separately; if an
//!   adapter is used to convert `log` records into `tracing` events, this will
//!   cause duplicate events to occur.
//! * `std`: Depend on the Rust standard library (enabled by default).
//!
//!   `no_std` users may disable this feature with `default-features = false`:
//!
//!   ```toml
//!   [dependencies]
//!   tracing = { version = "0.1.9", default-features = false }
//!   ```
//!   **Note**:`tracing`'s `no_std` support requires `liballoc`.
//!
//! [`log`]: https://docs.rs/log/0.4.6/log/
//! [span]: span/index.html
//! [`Span`]: span/struct.Span.html
//! [`in_scope`]: span/struct.Span.html#method.in_scope
//! [`Event`]: struct.Event.html
//! [`Subscriber`]: subscriber/trait.Subscriber.html
//! [Subscriber::event]: subscriber/trait.Subscriber.html#tymethod.event
//! [`enter`]: subscriber/trait.Subscriber.html#tymethod.enter
//! [`exit`]: subscriber/trait.Subscriber.html#tymethod.exit
//! [`enabled`]: subscriber/trait.Subscriber.html#tymethod.enabled
//! [metadata]: struct.Metadata.html
//! [`field::display`]: field/fn.display.html
//! [`field::debug`]: field/fn.debug.html
//! [`set_global_default`]: subscriber/fn.set_global_default.html
//! [`with_default`]: subscriber/fn.with_default.html
//! [`tokio-rs/tracing`]: https://github.com/tokio-rs/tracing
//! [`tracing-futures`]: https://crates.io/crates/tracing-futures
//! [`tracing-subscriber`]: https://crates.io/crates/tracing-subscriber
//! [`tracing-log`]: https://crates.io/crates/tracing-log
//! [`tracing-timing`]: https://crates.io/crates/tracing-timing
//! [`env_logger`]: https://crates.io/crates/env_logger
//! [`FmtSubscriber`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/struct.Subscriber.html
//! [static verbosity level]: level_filters/index.html#compile-time-filters
//! [instrument]: https://docs.rs/tracing-attributes/latest/tracing_attributes/attr.instrument.html
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[macro_use]
extern crate cfg_if;
use tracing_core;

#[cfg(feature = "log")]
#[doc(hidden)]
pub extern crate log;

// Somehow this `use` statement is necessary for us to re-export the `core`
// macros on Rust 1.26.0. I'm not sure how this makes it work, but it does.
#[allow(unused_imports)]
#[doc(hidden)]
use tracing_core::*;

pub use self::{
    dispatcher::Dispatch,
    event::Event,
    field::Value,
    subscriber::Subscriber,
    tracing_core::{dispatcher, event, Level, Metadata},
};

#[doc(hidden)]
pub use self::{
    span::Id,
    tracing_core::{
        callsite::{self, Callsite},
        metadata,
    },
};

#[doc(inline)]
pub use self::span::Span;
#[doc(inline)]
pub use tracing_attributes::instrument;

#[macro_use]
mod macros;

pub mod field;
pub mod level_filters;
pub mod span;
pub(crate) mod stdlib;
pub mod subscriber;

#[doc(hidden)]
pub mod __macro_support {
    pub use crate::stdlib::sync::atomic::{AtomicUsize, Ordering};

    #[cfg(feature = "std")]
    pub use crate::stdlib::sync::Once;

    #[cfg(not(feature = "std"))]
    pub type Once = spin::Once<()>;
}

mod sealed {
    pub trait Sealed {}
}
