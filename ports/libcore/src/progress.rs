use indicatif::ProgressBar;

pub struct Progress {
    bar: ProgressBar,
}

impl Progress {
    pub fn new(len: u64) -> Self {
        Self {
            bar: ProgressBar::new(len),
        }
    }

    pub fn go(&self) {
        self.bar.inc(1)
    }

    pub fn finish(&self) {
        self.bar.finish_and_clear();
    }
}
