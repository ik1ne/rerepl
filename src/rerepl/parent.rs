use crate::rerepl::Rerepl;

impl Rerepl {
    pub(crate) fn init_as_parent() {
        todo!("if parent, spawn child and organize stdin/stdout")
    }

    pub(crate) fn run_as_parent(&self) {
        todo!("poll parent stdin/child stdout and manage handlers")
    }
}
