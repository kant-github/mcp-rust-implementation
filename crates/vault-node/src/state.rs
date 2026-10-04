use crate::{mailbox::MailBoxes, outbox::Outbox};


#[derive(Clone, Debug)]
pub struct AppState {
    pub mailboxes: MailBoxes,
    pub node_index: u16,
    pub outbox: Outbox,
}