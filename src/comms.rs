use std::sync::mpsc::{Receiver, Sender, channel};

pub fn unin_channel() -> (Sender<&'static str>, Receiver<&'static str>) {
    channel()
}
