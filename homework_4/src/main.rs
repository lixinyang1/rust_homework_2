use std::{
    alloc::System,
    cell::RefCell,
    future::{self, Future},
    sync::{Arc, Condvar, Mutex},
    task::{Context, Poll, RawWaker, RawWakerVTable, Wake, Waker},
};

use futures::future::BoxFuture;

use scoped_tls::scoped_thread_local;
use std::collections::VecDeque;

fn dummy_waker() -> Waker {
    static DATA: () = ();
    unsafe { 
        Waker::from_raw(RawWaker::new(&DATA, &VTABLE)) 
    }
}

const VTABLE: RawWakerVTable =
    RawWakerVTable::new(table_clone, table_wake, table_wake_by_ref, table_drop);

unsafe fn table_clone(ptr: *const ()) -> RawWaker {
    println!("table_clone");
    RawWaker::new(ptr, &VTABLE)
}

unsafe fn table_wake(_ptr: *const ()) {
    println!("table_wake");
}

unsafe fn table_wake_by_ref(_ptr: *const ()) {
    println!("table_wake_by_ref");
}

unsafe fn table_drop(_ptr: *const ()) {
    println!("table_drop");
}
struct Signal {
    state: Mutex<State>,
    cond: Condvar,
}
enum State {
    Empty,
    Wating,
    Notified,
}

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.notify();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.notify();
    }
}

impl Signal {
    fn new() -> Self {
        Self {
            cond: Condvar::new(),
            state: Mutex::new(State::Empty),
        }
    }
    fn wait(&self) {
        let mut state: std::sync::MutexGuard<'_, State> = self.state.lock().unwrap();
        match *state {
            State::Empty => {
                *state = State::Wating;
                while let State::Wating = *state {
                    state = self.cond.wait(state).unwrap();
                }
            }
            State::Wating => {
                panic!("cannot wait twice");
            }
            State::Notified => {
                *state = State::Empty;
            }
        }
    }
    fn notify(&self) {
        let mut state: std::sync::MutexGuard<'_, State> = self.state.lock().unwrap();
        match *state {
            State::Empty => {
                *state = State::Notified;
            }
            State::Wating => {
                *state = State::Empty;
                self.cond.notify_one();
            }
            State::Notified => {
                println!("already notified")
            }
        }
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut: std::pin::Pin<&mut F> = std::pin::pin!(future);
    let signal: Arc<Signal> = Arc::new(Signal::new());
    let waker: Waker = Waker::from(signal.clone());

    let mut cx: Context<'_> = Context::from_waker(&waker);

    let runnable: Mutex<VecDeque<Arc<Task>>> = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || loop {
            if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
                return output;
            }
            while let Some(task) = runnable.lock().unwrap().pop_front() {
                let waker: Waker = Waker::from(task.clone());
                let mut cx: Context<'_> = Context::from_waker(&waker);
                let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
            }
            signal.wait();
        })
    })
}

fn spawn<F: Future<Output = ()> + 'static + Send>(future: F) {
    let signal: Arc<Signal> = Arc::new(Signal::new());
    let waker: Waker = Waker::from(signal.clone());
    let task: Arc<Task> = Arc::new(Task {
        future: RefCell::new(Box::pin(future)),
        signal: signal.clone(),
    });
    let mut cx: Context<'_> = Context::from_waker(&waker);
    if let Poll::Ready(_) = task.future.borrow_mut().as_mut().poll(&mut cx) {
        return;
    }
    RUNNABLE.with(|runnable: &Mutex<VecDeque<Arc<Task>>>| {
        runnable.lock().unwrap().push_back(task);
        signal.notify();
    })
}

async fn demo() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        // std::thread::sleep(std::time::Duration::from_secs(2));
        tx.send_blocking(()).unwrap();
    });
    let _ = rx.recv().await;
    println!("Hello, world!111");
    //sleep
}

async fn demo1() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    println!("Hello, world!222");
    spawn(demo2(tx));
    println!("Hello, world!444");
    let _ = rx.recv().await;
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("Hello, world!333");
    tx.send(()).await.unwrap();
}

struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable: &Mutex<VecDeque<Arc<Task>>>| {
            runnable.lock().unwrap().push_back(self.clone());
            self.signal.notify();
        })
    }
}

scoped_thread_local!(static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
scoped_thread_local!(static SIGNAL: Arc<Signal>);

fn main() {
    println!("Hello, world!");

    block_on(demo());
    block_on(demo());
    block_on(demo());
    block_on(demo1());
}