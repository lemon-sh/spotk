use std::sync::atomic::{AtomicUsize, Ordering};

use colored::Color;

pub struct ColorSpinner {
    i: AtomicUsize,
    colors: &'static [Color],
}

impl ColorSpinner {
    pub fn new(colors: &'static [Color]) -> Self {
        Self {
            colors,
            i: AtomicUsize::default(),
        }
    }

    pub fn next(&self) -> Color {
        let color = self.colors[self.i.fetch_add(1, Ordering::Relaxed) % self.colors.len()];
        color
    }
}
