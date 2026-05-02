use std::sync::mpsc::{channel, Receiver, Sender};

pub fn unin_channel() -> (Sender<&'static str>, Receiver<&'static str>) {
	channel()
}