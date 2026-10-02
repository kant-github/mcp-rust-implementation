use crate::envelope::Envelope;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

struct MailBox {
    sender: UnboundedSender<Envelope>,
    reciever: Option<UnboundedReceiver<Envelope>>,
}

impl MailBox {
    pub fn new() -> Self {
        let (sender, reciever) = unbounded_channel();
        Self {
            sender,
            reciever: Some(reciever),
        }
    }
}

#[derive(Default, Clone)]
pub struct MailBoxes {
    inner: Arc<Mutex<HashMap<String, MailBox>>>,
}

impl MailBoxes {
    pub fn deliver(&self, envelope: Envelope) {
        let mut list = self.inner.lock().unwrap();
        let entry = list
            .entry(envelope.session_id.clone())
            .or_insert_with(MailBox::new);
        let _ = entry.sender.send(envelope);
    }

    pub fn open(&self, session_id: String) -> Option<UnboundedReceiver<Envelope>> {
        let mut list = self.inner.lock().unwrap();
        let entry = list
            .entry(session_id)
            .or_insert_with(MailBox::new)
            .reciever
            .take();
        entry
    }

    pub fn close(&self, session_id: &str) {
        let mut list = self.inner.lock().unwrap();
        list.remove(session_id);
    }
}
