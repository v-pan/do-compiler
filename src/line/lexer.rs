pub struct LineLexer<'a> {
    line: &'a String,
    parts: Vec<&'a str>,
    cur_idx: usize,
}

impl<'a> LineLexer<'a> {
    pub fn new(line: &'a String) -> Self {
        LineLexer { line, parts: vec![], cur_idx: 0 }
    }

    fn cache_parts(&mut self) {
        if self.cur_idx == 0 {
            // These delimiters should be connectives like brackets, ->, commas, etc.
            self.parts = self.line.split(&[' ', '(', ')'][..]).collect();
        }
    }

    fn peek(&mut self, next: usize) -> Option<&[&str]> {
        self.cache_parts();

        if self.cur_idx + (next - 1) < self.parts.len() {
            self.parts.get(self.cur_idx..self.cur_idx + (next - 1))
        } else {
            None
        }
    }
}

impl<'a> Iterator for LineLexer<'a> {
    type Item = &'a str;


    fn next(&mut self) -> Option<Self::Item> {
        self.cache_parts();

        // While we have parts to process
        if self.cur_idx < self.parts.len() {
            let next_token = self.parts[self.cur_idx];
            self.cur_idx += 1;

            Some(next_token)
        } else {
            None // We finished the line
        }
    }
}
