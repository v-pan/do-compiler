#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub size: usize
}

impl Default for Span {
    fn default() -> Self {
        Span {
            start: 0,
            size: 0
        }
    }
}

impl Span {
    pub fn new(start: usize, size: usize) -> Self {
        Span { start, size }
    }

    /**
     *  Creates a span of size 0 at the end of the given span
     */
    pub fn after(value: Span) -> Self {
        Span { start: value.start + value.size, size: 0 }
    }

    pub fn extend(&mut self, by: usize) -> Self {
        self.size += by;

        return *self;
    }
}
