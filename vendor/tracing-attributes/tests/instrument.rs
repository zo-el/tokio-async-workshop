mod support;
use support::*;

use tracing::subscriber::with_default;
use tracing::Level;
use tracing_attributes::instrument;

#[test]
fn override_everything() {
    #[instrument(target = "my_target", level = "debug")]
    fn my_fn() {}

    #[instrument(level = "debug", target = "my_target")]
    fn my_other_fn() {}

    let span = span::mock()
        .named("my_fn")
        .at_level(Level::DEBUG)
        .with_target("my_target");
    let span2 = span::mock()
        .named("my_other_fn")
        .at_level(Level::DEBUG)
        .with_target("my_target");
    let (subscriber, handle) = subscriber::mock()
        .new_span(span.clone())
        .enter(span.clone())
        .exit(span.clone())
        .drop_span(span)
        .new_span(span2.clone())
        .enter(span2.clone())
        .exit(span2.clone())
        .drop_span(span2)
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        my_fn();
        my_other_fn();
    });

    handle.assert_finished();
}

#[test]
fn fields() {
    #[instrument(target = "my_target", level = "debug")]
    fn my_fn(arg1: usize, arg2: bool) {}

    let span = span::mock()
        .named("my_fn")
        .at_level(Level::DEBUG)
        .with_target("my_target");

    let span2 = span::mock()
        .named("my_fn")
        .at_level(Level::DEBUG)
        .with_target("my_target");
    let (subscriber, handle) = subscriber::mock()
        .new_span(
            span.clone().with_field(
                field::mock("arg1")
                    .with_value(&format_args!("2"))
                    .and(field::mock("arg2").with_value(&format_args!("false"))),
            ),
        )
        .enter(span.clone())
        .exit(span.clone())
        .drop_span(span)
        .new_span(
            span2.clone().with_field(
                field::mock("arg1")
                    .with_value(&format_args!("3"))
                    .and(field::mock("arg2").with_value(&format_args!("true"))),
            ),
        )
        .enter(span2.clone())
        .exit(span2.clone())
        .drop_span(span2)
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        my_fn(2, false);
        my_fn(3, true);
    });

    handle.assert_finished();
}

#[test]
fn generics() {
    #[derive(Debug)]
    struct Foo;

    #[instrument]
    fn my_fn<S, T: std::fmt::Debug>(arg1: S, arg2: T)
    where
        S: std::fmt::Debug,
    {
    }

    let span = span::mock().named("my_fn");

    let (subscriber, handle) = subscriber::mock()
        .new_span(
            span.clone().with_field(
                field::mock("arg1")
                    .with_value(&format_args!("Foo"))
                    .and(field::mock("arg2").with_value(&format_args!("false"))),
            ),
        )
        .enter(span.clone())
        .exit(span.clone())
        .drop_span(span)
        .done()
        .run_with_handle();

    with_default(subscriber, || {
        my_fn(Foo, false);
    });

    handle.assert_finished();
}
