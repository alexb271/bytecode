#[derive(Clone, Debug)]
pub struct FunctionBody {
    control_flow_structures: Vec<ControlFlow>,
}

impl FunctionBody {
    #[inline]
    pub fn new(logic: Vec<ControlFlow>) -> FunctionBody {
        FunctionBody {
            control_flow_structures: logic,
        }
    }

    #[inline]
    pub fn control_flow_structures(&self) -> &Vec<ControlFlow> {
        &self.control_flow_structures
    }
}

#[derive(Clone, Debug)]
pub enum ControlFlow {
    BasicBlock(BasicBlock),
    WhileLoop(WhileLoop),
}

#[derive(Clone, Debug)]
pub struct WhileLoop {
    span: Span,
    condition: Expression,
    body: BasicBlock,
}

impl WhileLoop {
    #[inline]
    pub fn new(span: Span, condition: Expression, body: BasicBlock) -> WhileLoop {
        WhileLoop {
            span,
            condition,
            body,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn condition(&self) -> &Expression {
        &self.condition
    }

    #[inline]
    pub fn body(&self) -> &BasicBlock {
        &self.body
    }
}

#[derive(Clone, Debug)]
pub struct BasicBlock {
    statements: Vec<Statement>,
}

impl BasicBlock {
    #[inline]
    pub fn new(statements: Vec<Statement>) -> BasicBlock {
        BasicBlock { statements }
    }

    #[inline]
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    LetStatement(LetStatement),
    Assignment(Assignment),
    ReturnStatement(ReturnStatement),
    Expression(Expression),
}

#[derive(Clone, Debug)]
pub struct LetStatement {
    span: Span,
    identifier: Identifier,
    #[allow(dead_code)]
    operator_span: Span,
    expression: Box<Expression>,
}

impl LetStatement {
    #[inline]
    pub fn new(
        span: Span,
        identifier: Identifier,
        #[allow(dead_code)] operator_span: Span,
        expression: Box<Expression>,
    ) -> Self {
        LetStatement {
            span,
            identifier,
            operator_span,
            expression,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    #[allow(dead_code)]
    #[inline]
    pub fn operator_span(&self) -> Span {
        self.operator_span
    }

    #[inline]
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Assignment {
    span: Span,
    lhs: Identifier,
    operator: AssignmentOperator,
    operator_span: Span,
    rhs: Box<Expression>,
}

#[allow(dead_code)]
impl Assignment {
    #[inline]
    pub fn new(
        span: Span,
        lhs: Identifier,
        operator: AssignmentOperator,
        operator_span: Span,
        rhs: Box<Expression>,
    ) -> Assignment {
        Assignment {
            span,
            lhs,
            operator,
            operator_span,
            rhs,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn lhs(&self) -> &Identifier {
        &self.lhs
    }

    #[inline]
    pub fn operator(&self) -> AssignmentOperator {
        self.operator
    }

    #[inline]
    pub fn operator_span(&self) -> Span {
        self.operator_span
    }

    #[inline]
    pub fn rhs(&self) -> &Expression {
        &self.rhs
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum AssignmentOperator {
    Basic,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    #[allow(dead_code)]
    span: Span,
    expression: Box<Expression>,
}

impl ReturnStatement {
    #[inline]
    pub fn new(span: Span, expression: Box<Expression>) -> ReturnStatement {
        ReturnStatement { span, expression }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
}

impl Expression {
    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(literal) => literal.span(),
            Expression::Identifier(identifier) => identifier.span(),
            Expression::BinaryOperation(binop) => binop.span(),
            Expression::UnaryOperation(unop) => unop.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinaryOperation {
    span: Span,
    operator_span: Span,
    left: Box<Expression>,
    operator: BinaryOperator,
    right: Box<Expression>,
}

impl BinaryOperation {
    #[inline]
    pub fn new(
        span: Span,
        operator_span: Span,
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    ) -> BinaryOperation {
        BinaryOperation {
            span,
            operator_span,
            left,
            operator,
            right,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn operator_span(&self) -> Span {
        self.operator_span
    }

    #[inline]
    pub fn left(&self) -> &Expression {
        &self.left
    }

    #[inline]
    pub fn operator(&self) -> BinaryOperator {
        self.operator
    }

    #[inline]
    pub fn right(&self) -> &Expression {
        &self.right
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Or,
    And,
    Equal,
    NotEqual,
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessEq => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterEq => write!(f, ">="),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
        }
    }
}
#[derive(Clone, Debug)]
pub struct UnaryOperation {
    span: Span,
    operator_span: Span,
    operator: UnaryOperator,
    operand: Box<Expression>,
}

impl UnaryOperation {
    #[inline]
    pub fn new(
        span: Span,
        operator_span: Span,
        operator: UnaryOperator,
        operand: Box<Expression>,
    ) -> UnaryOperation {
        UnaryOperation {
            span,
            operator_span,
            operator,
            operand,
        }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn operator_span(&self) -> Span {
        self.operator_span
    }

    #[inline]
    pub fn operator(&self) -> UnaryOperator {
        self.operator
    }

    #[inline]
    pub fn operand(&self) -> &Expression {
        &self.operand
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Not,
    Neg,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "not"),
            UnaryOperator::Neg => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    name: String,
    span: Span,
}

impl Identifier {
    #[inline]
    pub fn new(name: String, span: Span) -> Identifier {
        Identifier { name, span }
    }

    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    value: Value,
    span: Span,
}

impl Literal {
    #[inline]
    pub fn new(value: Value, span: Span) -> Literal {
        Literal { value, span }
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    #[inline]
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    Str,
    Char,
}

impl DataType {
    pub fn typename(&self) -> String {
        match self {
            DataType::Int => "int".to_string(),
            DataType::Float => "float".to_string(),
            DataType::Bool => "bool".to_string(),
            DataType::Str => "string".to_string(),
            DataType::Char => "char".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Box<String>),
    Char(char),
}

impl Value {
    #[inline]
    pub fn data_type(&self) -> DataType {
        match self {
            Value::Int(_) => DataType::Int,
            Value::Float(_) => DataType::Float,
            Value::Bool(_) => DataType::Bool,
            Value::Str(_) => DataType::Str,
            Value::Char(_) => DataType::Char,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
        }
    }
}
