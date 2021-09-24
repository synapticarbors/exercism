#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }

    string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|x| x as u64)
                .ok_or(Error::InvalidDigit(c))
        })
        .collect::<Result<Vec<_>, _>>()?
        .windows(span)
        .map(|x| x.iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}
