use crossbeam_channel::{Receiver, Sender, unbounded};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

pub type HandlerIdentifier = usize;

pub trait Dispatcher: Send + Sync {
    fn register_handler<E, F>(&self, handler: F) -> HandlerIdentifier
    where
        E: Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static;

    fn dispatch<E>(&self, event: E)
    where
        E: Send + Sync + 'static;

    fn unregister_handler(&self, id: HandlerIdentifier);
}

#[derive(Clone)]
struct HandlerEntry {
    id: HandlerIdentifier,
    callback: Arc<dyn Fn(&Arc<dyn Any + Send + Sync>) + Send + Sync>,
}

pub struct EventDispatcher {
    handlers: RwLock<HashMap<TypeId, Vec<HandlerEntry>>>,
    tx: Sender<Arc<dyn Any + Send + Sync>>,
    next_id: AtomicUsize,
}

impl EventDispatcher {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = unbounded();
        let disp = Arc::new(Self {
            handlers: RwLock::new(HashMap::new()),
            tx,
            next_id: AtomicUsize::new(1),
        });
        Self::spawn_runner(Arc::clone(&disp), rx);
        disp
    }

    fn spawn_runner(this: Arc<Self>, rx: Receiver<Arc<dyn Any + Send + Sync>>) {
        thread::spawn(move || {
            for event in rx.iter() {
                let t = (&*event).type_id();
                let handlers = {
                    let map = this.handlers.read().unwrap();
                    map.get(&t).cloned()
                };
                if let Some(list) = handlers {
                    for h in &list {
                        (h.callback)(&event);
                    }
                }
            }
        });
    }
}

impl Dispatcher for Arc<EventDispatcher> {
    fn register_handler<E, F>(&self, handler: F) -> HandlerIdentifier
    where
        E: Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut map = self.handlers.write().unwrap();
        let entry = map.entry(TypeId::of::<E>()).or_default();
        let cb: Arc<dyn Fn(&Arc<dyn Any + Send + Sync>) + Send + Sync> =
            Arc::new(move |ev: &Arc<dyn Any + Send + Sync>| {
                if let Some(e) = ev.clone().downcast::<E>().ok() {
                    handler(&e);
                }
            });
        entry.push(HandlerEntry { id, callback: cb });
        id
    }

    fn dispatch<E>(&self, event: E)
    where
        E: Send + Sync + 'static,
    {
        let _ = self.tx.send(Arc::new(event));
    }

    fn unregister_handler(&self, id: HandlerIdentifier) {
        let mut map = self.handlers.write().unwrap();
        for handlers in map.values_mut() {
            handlers.retain(|h| h.id != id);
        }
    }
}
