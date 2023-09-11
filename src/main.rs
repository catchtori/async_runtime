use std::future::future;
use std::task;

strcut Demo;

impl Future for Demo {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        println!("Hello");
        std::task::Poll::Ready(())
    }
}

fn dummy_waker() -> std::task::Waker {
    static DATA: () = ();
    unsafe{Waker::from_raw(RawWaker::new(&DATA,&VTABLE))}
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(
    vtable_clone,
    vtable_wake,
    vtable_wake_by_ref,
    vtable_drop,
);

unsafe fn vtable_clone(_p: *const ()) -> RawWaker {
    RawWaker::new(_p, &VTABLE)
}

unsafe fn vtable_wake(_p: *const ()) {
}

unsafe fn vtable_wake_by_ref(_p: *const ()) {
}

unsafe fn vtable_drop(_p: *const ()) {
}

fn block_on<F:Future>(future: F) -> F::Output {
    let mut fut: Pin<&mut F> = std::pin::pin!(future);
    let waker: Waker = dummy_waker();

    let mut cx = std::task::Context::from_waker(&waker);
    loop {
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output
        }
    }
}