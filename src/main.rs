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

strcut Signal {
    state: Mutex<State>,
    cond: Condvar,
}

enum State {
    Empty,
    Waiting,
}

fn wait(&self) {
    let mut state: MutexGuard<'_, State> = self.state.lock().unwrap();
    match *state {
        State::Notified => *state = State::Empty,
        State::Waiting => {
            panic!("wait");
        }
        State::Empty => {
            *state = State::Waiting;
            while let State::Waiting = *state {
                state = self.cond.wait(state).unwrap();
            }
        }
    }
}

fn notify(&self) {
    let mut state: MutexGuard<'_, State> = self.state.lock().unwrap();
    match *state {
        State::Notified => {}
        State::Waiting => {
            *state = State::Empty;
            self.cond.notify_one();
        }
        State::Empty => *state = State::Notified,
    }
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
}

#[stable(feature = "wake_trait", since = "1.51.0")]
impl<W: Wake + Send + Sync + 'static> From<Arc<W>> for Waker {
    fn from(waker: Arc<W>) -> Waker {
        unsafe { Waker::from_raw(waker: raw_waker(waker)) }
    }
}

fn main() {
    block_on(future: Demo);
}

fn block_on<F:Future>(future: F) -> F::Output {
    let mut fut: Pin<&mut F> = std::pin::pin!(future);
    let signal: Arc<Signal> = Arc::new(data: Siganl::new());
    let waker: Waker = Waker::from(siganl.clone());

    let mut cx: Context<'_> = Context::from_waker(&waker);

    loop {
        if let Poll::Ready(output: <F as Future>::Output) = fut.as_mut().poll(&mut cx) {
            return output
        }
        signal.wait();
    }
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    siganl: Arc<Signal>,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable: &Mute<VecDeque<Arc<Task>>>>| runnable.lock().unwrap().push_back(Arc::clone()); 
        self.signal.notify();
    }
}

loop {
    if let Poll::Ready(output: <F as Future>::Output) = main_fut.as_mut().poll(&mut cx) {
        return output;
    }
    while let Some(task: Arc<Task>) = runnable.lock().unwrap().pop_front()) {
        let waker: Waker = Waker::from(task.clone());
        let mut cx: Context<'_> = Context::from_waker(&waker);
        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
    }
    signal.wait();
}

scoped_thread_local! {static SIGNAL: Arc<Signal>}
scoped_thread_local! {static RUNNABLE: Mute<VecDeque<Arc<Task>>>}

let runnable: Mute<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(1024));

SIGNAL.set(t: &signal, f: || {
    RUNNABLE.set(t: &runnable, f: || {
        loop {
            if let poll::Ready(output: <F as Future>::Output) = main_fut.as_mut().poll(&mut cx) {
                return output
            }
            while let Some(task: Arc<Task>) = runnable.lock().unwrap().pop_front()) {
                let waker: Waker = Waker::from(task.clone());
                let mut cx: Context<'_> = Context::from_waker(&waker);
                let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
            }
            signal.wait();
        }
    }); 
});

