use std::sync::atomic::{AtomicUsize, Ordering};

use owo_colors::AnsiColors;

pub struct ColorSpinner {
    i: AtomicUsize,
    colors: &'static [AnsiColors],
}

impl ColorSpinner {
    pub fn new(colors: &'static [AnsiColors]) -> Self {
        Self {
            colors,
            i: AtomicUsize::default(),
        }
    }

    pub fn next(&self) -> AnsiColors {
        let color = self.colors[self.i.fetch_add(1, Ordering::Relaxed) % self.colors.len()];
        color
    }
}
