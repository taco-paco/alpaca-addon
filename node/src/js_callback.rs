use neon::prelude::*;
use std::marker::PhantomData;
use std::{
    clone::Clone,
    sync::{
        mpsc::{channel, Receiver, TryRecvError},
        Arc, Mutex,
    },
};

use crate::js_traits::IntoJsType;

pub struct JsCallbackHolder<T> {
    js_callback: Arc<Mutex<Root<JsFunction>>>,
    event_channel: Arc<Mutex<Channel>>,
    receivers: Vec<Receiver<()>>,
    _p: PhantomData<T>,
}

impl<T> Drop for JsCallbackHolder<T> {
    fn drop(&mut self) {
        for receiver in self.receivers.iter() {
            receiver.recv().ok();
        }
    }
}

impl<T> Clone for JsCallbackHolder<T> {
    fn clone(&self) -> JsCallbackHolder<T> {
        JsCallbackHolder {
            js_callback: self.js_callback.clone(),
            event_channel: self.event_channel.clone(),
            receivers: vec![],
            _p: Default::default(),
        }
    }
}

impl<'b, T> JsCallbackHolder<T>
where
    T: 'static + IntoJsType + Send,
{
    pub fn new(js_callback: Root<JsFunction>, event_channel: Channel) -> Self {
        Self {
            js_callback: Arc::new(Mutex::new(js_callback)),
            event_channel: Arc::new(Mutex::new(event_channel)),
            receivers: vec![],
            _p: Default::default(),
        }
    }

    pub fn call(&mut self, args: T) {
        self.prune_finished_tasks();

        let (sender, receiver) = channel::<()>();

        let event_channel = self.event_channel.clone();
        let js_callback = self.js_callback.clone();

        std::thread::spawn(move || {
            if let Ok(event_channel) = event_channel.lock() {
                event_channel.send(move |mut cx| {
                    if let Ok(locked_callback) = js_callback.lock() {
                        let callback = locked_callback.to_inner(&mut cx);
                        let this = cx.undefined();

                        match args.into_js_type(&mut cx) {
                            Ok(val) => {
                                callback.call(&mut cx, this, val)?;
                            }
                            Err(err) => {
                                let js_error = err.into_js_type(&mut cx).expect("Nani? Error creation shall not panic");
                                callback.call(&mut cx, this, js_error)?;
                            }
                        };
                    } else {
                        // TODO: log poisoned callback mutex
                    }

                    drop(js_callback);
                    sender.send(()).ok();

                    Ok(())
                });
            } else {
                // TODO: log poisoned mutex
            }
        });

        self.receivers.push(receiver);
    }

    fn prune_finished_tasks(&mut self) {
        self.receivers.retain(|receiver| match receiver.try_recv() {
            Ok(_) => false,
            Err(error) => match error {
                TryRecvError::Empty => true,
                TryRecvError::Disconnected => false,
            },
        })
    }
}
