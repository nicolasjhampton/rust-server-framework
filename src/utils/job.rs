pub type Job = Box<dyn FnOnce() + Send + 'static>;
// pub type Job = impl FnOnce() + Send + 'static;