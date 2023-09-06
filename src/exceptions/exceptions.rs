#[derive(Debug)]
pub enum GelExceptions {
    GelGlobalExceptions(String), // (message)
}

impl GelExceptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GelExceptions::GelGlobalExceptions(message) => {
                write!(f, "GelGlobalExceptions An error has been caught: {}", message);
                std::process::exit(1);
            }
            _ => { panic!("GelExceptions::fmt() called with an invalid exception type.") }
        }
    }
}

impl GelExceptions {
    pub(crate) fn throw_exception(&self) -> ! {
        panic!("{:?}", self);
    }
}

pub fn throw_gel_global_exception(exception: GelExceptions) -> ! {
    exception.throw_exception();
}