use super::language_components::{BinaryOperator, Span, UnaryOperator};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    InvalidBinaryOperation(BinaryOperator, String, String),
    InvalidUnaryOperation(UnaryOperator, String),
    IdentifierNotFound,
    IdentifierIsKeyword,
    InvalidAssignment(String, String),
    ArgumentInvalidType(String, String),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidBinaryOperation(operator, lhs_typename, rhs_typename) => {
                write!(
                    f,
                    "Invalid operation '{}' for types '{}' and '{}'",
                    operator, lhs_typename, rhs_typename
                )
            }
            ErrorKind::InvalidUnaryOperation(operator, typename) => {
                write!(
                    f,
                    "Invalid operation '{}' for type '{}'",
                    operator, typename
                )
            }
            ErrorKind::IdentifierNotFound => write!(f, "Identifier not found"),
            ErrorKind::IdentifierIsKeyword => write!(f, "Expected identifier, found keyword"),
            ErrorKind::InvalidAssignment(t1, t2) => {
                write!(
                    f,
                    "Cannot assign to a variable of type '{}' a value of type '{}'",
                    t1, t2
                )
            }
            ErrorKind::ArgumentInvalidType(t1, t2) => {
                write!(f, "Expected type '{}', found type '{}'", t1, t2)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    filename: String,
    source_code: String,
    kind: ErrorKind,
    context: Span,
    error: Span,
}

impl Error {
    pub fn new(
        filename: String,
        source_code: String,
        kind: ErrorKind,
        context: Span,
        error: Span,
    ) -> Error {
        Error {
            filename,
            source_code,
            kind,
            context,
            error,
        }
    }

    pub fn format_to_string(&self) -> String {
        let mut result = format!("In file: {}\n", self.filename);
        let mut buffer = String::new();
        let mut line_count = 1;
        let padding = self.source_code.lines().count().to_string().len();
        let mut iter = self.source_code.char_indices();
        let mut iter_at_last_newline = iter.clone();

        // Find the most recent newline before the context_start
        // Also count which line the context starts on
        while let Some((idx, ch)) = iter.next() {
            if idx == self.context.start() {
                break;
            }
            if ch == '\n' {
                line_count += 1;
                iter_at_last_newline = iter.clone();
            }
        }
        iter = iter_at_last_newline;
        result.push_str(format!("In line {}:\n\n", line_count).as_str());
        Self::add_line_count_prefix(line_count, padding, &mut result, &mut buffer);

        // Print '^' characters under the error range, even if the error range has newlines in it
        // Print the whole line which contains the context_end
        while let Some((idx, ch)) = iter.next() {
            if ch == '\n' {
                if idx >= self.context.end() {
                    break;
                } else {
                    result.push('\n');
                    result.push_str(&buffer);
                    result.push('\n');
                    buffer.clear();
                    line_count += 1;
                    Self::add_line_count_prefix(line_count, padding, &mut result, &mut buffer);
                }
            } else if idx < self.error.start() {
                result.push(ch);
                buffer.push(' ');
            } else if idx >= self.error.start() && idx < self.error.end() {
                result.push(ch);
                buffer.push('^');
            } else {
                result.push(ch);
            }
        }

        result.push('\n');
        result.push_str(&buffer);
        result.push_str(format!("\n\nError: {}", self.kind.to_string()).as_str());
        result
    }

    #[inline]
    fn add_line_count_prefix(
        line_count: usize,
        padding: usize,
        result: &mut String,
        buffer: &mut String,
    ) {
        let padding = padding + 1;
        let line_prefix = format!("{:>padding$}| ", line_count);
        result.push_str(&line_prefix);
        buffer.push_str(" ".repeat(line_prefix.len()).as_str());
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_format() {
        let error = Error::new(
            String::from("test"),
            String::from("1 + true"),
            ErrorKind::InvalidBinaryOperation(
                BinaryOperator::Add,
                String::from("int"),
                String::from("bool"),
            ),
            Span::new(0, 8),
            Span::new(2, 3),
        );
        let expected = "In file: test
In line 1:

 1| 1 + true
      ^

Error: Invalid operation '+' for types 'int' and 'bool'";
        assert_eq!(error.to_string(), expected);

        let error = Error::new(
            String::from("test"),
            String::from("1 + true"),
            ErrorKind::InvalidBinaryOperation(
                BinaryOperator::Add,
                String::from("int"),
                String::from("bool"),
            ),
            Span::new(0, 8),
            Span::new(0, 8),
        );
        let expected = "In file: test
In line 1:

 1| 1 + true
    ^^^^^^^^

Error: Invalid operation '+' for types 'int' and 'bool'";
        assert_eq!(error.to_string(), expected);

        let error = Error::new(
            String::from("test"),
            String::from("5 + 1 * true and false"),
            ErrorKind::InvalidBinaryOperation(
                BinaryOperator::Mul,
                String::from("int"),
                String::from("bool"),
            ),
            Span::new(4, 12),
            Span::new(4, 12),
        );
        let expected = "In file: test
In line 1:

 1| 5 + 1 * true and false
        ^^^^^^^^

Error: Invalid operation '*' for types 'int' and 'bool'";
        assert_eq!(error.to_string(), expected);

        let error = Error::new(
            String::from("test"),
            String::from(
                "// comment 1
// comment 2
// comment 3
// comment 4
// comment 5
// comment 6
// comment 7
// comment 8
5 % 2 -
'C' and false and true
// final comment",
            ),
            ErrorKind::InvalidBinaryOperation(
                BinaryOperator::Sub,
                String::from("int"),
                String::from("char"),
            ),
            Span::new(108, 115),
            Span::new(108, 115),
        );
        let expected = "In file: test
In line 9:

  9| 5 % 2 -
         ^^^
 10| 'C' and false and true
     ^^^

Error: Invalid operation '-' for types 'int' and 'char'";
        assert_eq!(error.to_string(), expected);
    }
}
