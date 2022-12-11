//use router::ClosureRouter;
use router::FnPointerRouter;
use router::{Request, Response};

fn main() {
    //let router = ClosureRouter::new()
    let router = FnPointerRouter::new()
        .add("/test", |_| Response::default())
        .add("/post", |_req| Response::default());
    println!("{router:?}");

    let resp = router.route(&Request {
        url: "/test".to_string(),
        ..Default::default()
    });
    assert!(!resp.is_error());
    let resp = router.route(&Request {
        url: "/something".to_string(),
        ..Default::default()
    });
    assert!(resp.is_error());
}
