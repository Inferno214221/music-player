use super::queable::Queable;

#[derive(Debug)]
pub struct QuePause;

impl Queable for QuePause {}

#[derive(Debug)]
pub struct QueStop;

impl Queable for QueStop {}