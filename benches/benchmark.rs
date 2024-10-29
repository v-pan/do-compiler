use std::{
    fs::File,
    io::{BufReader, Read},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use llvm_compiler::{
    lexer::AsciiLexer,
    parse::parser::{parse, Parser},
    token::Token,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lexing", |b| {
        const READER_CAPACITY: usize = 100_000_000;
        let file = File::open("./examples/infix.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, file);

        let mut lexer = AsciiLexer::new();
        let mut buf = String::new();

        // Read in as much as we can at once
        let bytes_read = reader.read_to_string(&mut buf).unwrap(); // TODO: There is some edge
                                                                   // case behaviour here when
                                                                   // a file is too long to
                                                                   // store in memory. This is
                                                                   // currently unhandled.

        b.iter(|| {
            let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);
            black_box(tokens);
            buf.clear();
        });
    });

    c.bench_function("parsing", |b| {
        const READER_CAPACITY: usize = 100_000_000;
        let file = File::open("./examples/infix.src").unwrap();
        let mut reader = BufReader::with_capacity(READER_CAPACITY, file);

        let mut lexer = AsciiLexer::new();
        let mut buf = String::new();

        // Read in as much as we can at once
        let bytes_read = reader.read_to_string(&mut buf).unwrap(); // TODO: There is some edge
                                                                   // case behaviour here when
                                                                   // a file is too long to
                                                                   // store in memory. This is
                                                                   // currently unhandled.

        let tokens: Vec<Token<'_>> = lexer.tokenize(&buf);

        b.iter(|| {
            let mut parser = Parser::new(0, &tokens);

            let source = buf.clone();
            let parsed = parse(&mut parser).map_err(|report| report.with_source_code(source));

            black_box(parsed);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
