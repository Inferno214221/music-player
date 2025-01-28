use super::queueable::Queueable;

#[derive(Debug)]
pub struct QueuePause;

impl Queueable for QueuePause {}

#[derive(Debug)]
pub struct QueueStop;

impl Queueable for QueueStop {}