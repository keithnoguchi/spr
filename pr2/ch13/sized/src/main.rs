use std::fmt::{Debug, Display};
use tracing::{info, instrument};

struct RcBox<T: ?Sized> {
    ref_count: usize,
    value: T,
}

fn main() {
    tracing_subscriber::fmt::init();
    let debugging = std::env::args().len() > 1;
    let boxed_lunch = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };
    if debugging {
        debug(&boxed_lunch);
    } else {
        display(&boxed_lunch);
    }
}

#[instrument(skip(boxed))]
fn display(boxed: &RcBox<dyn Display>) {
    info!(%boxed.ref_count, %boxed.value);
}

#[instrument(skip(boxed))]
fn debug(boxed: &RcBox<dyn Debug>) {
    info!(%boxed.ref_count, ?boxed.value);
}
