use std::fmt::Display;
use std::cell::RefCell;

#[derive(Default)]
pub struct Ctxt {
    errors: RefCell<Option<Vec<String>>>,
}

impl Ctxt {
    pub fn new() -> Self {
        Ctxt {
            errors: RefCell::new(Some(Vec::new())),
        }
    }

    pub fn error<T: Display>(&self, msg: T) {
        self.errors.borrow_mut().as_mut().unwrap().push(msg.to_string());
    }

    pub fn check(self) -> Result<(), String> {
        let mut errors = self.errors.borrow_mut().take().unwrap();
        match errors.len() {
            0 => Ok(()),
            1 => Err(errors.pop().unwrap()),
            n => {
                let mut msg = format!("{} errors:", n);
                for err in errors {
                    msg += "\n\t# ";
                    msg += &err;
                }
                Err(msg)
            }
        }
    }
}

impl Drop for Ctxt {
    fn drop(&mut self) {
        if self.errors.borrow().is_some() {
            panic!("forgot to check for errors");
        }
    }
}
