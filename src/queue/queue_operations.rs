use super::queueable::Queueable;

#[derive(Debug, Clone, Copy)]
pub struct QueuePause;

impl Queueable for QueuePause {}

#[derive(Debug, Clone, Copy)]
pub struct QueueStop;

impl Queueable for QueueStop {}