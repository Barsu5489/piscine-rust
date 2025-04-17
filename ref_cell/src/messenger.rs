use std::rc::Rc;

// Logger trait for different types of messages
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// Tracker structure to monitor references
pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    max: usize,
}

impl<'a> Tracker<'a> {
    // Initialize a new Tracker
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            max,
        }
    }

    // Set the value and check against quotas
    pub fn set_value(&self, rc_value: &Rc<usize>) {
        let current = Rc::strong_count(rc_value);
        let percentage = (current as f32 / self.max as f32) * 100.0;
        
        if percentage >= 100.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70.0 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage.round() as usize
            ));
        }
    }

    // Check current usage percentage
    pub fn peek(&self, rc_value: &Rc<usize>) {
        let current = Rc::strong_count(rc_value);
        let percentage = (current as f32 / self.max as f32) * 100.0;
        
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage.round() as usize
        ));
    }
}