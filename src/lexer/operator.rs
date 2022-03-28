use std::fmt;

// https://stackoverflow.com/a/34324856
// count using recursion (i think)
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! op {
    { $( $lex:ident = $true:expr),* $(,)? } => {
        $(
            pub const $lex: Operator = Operator($true);
        )*

        #[allow(non_upper_case_globals)] // not upper case to differentiate from others
        pub const operators: [Operator; count!($($lex)*)] = [$($lex),*];
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Operator(&'static str);

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

op!{
    // other / in multiple
    OP_COLON = ":", OP_DOUBLE_COLON = "::",
    OP_DOUBLE_DOT = "..", OP_TRIPLE_DOT = "...",
    OP_BAR = "|",

    // seperation
    OP_SEMI = ";",
    OP_COMM = ",",

    // brackets
    OP_LCURLY  = "{", OP_RCURLY  = "}",
    OP_LPARA   = "(", OP_RPARA   = ")",
    OP_LSQUARE = "[", OP_RSQUARE = "]",
    OP_LANGLE  = "<", OP_RANGLE  = ">",

    // item
    OP_EQUAL = "=",
    OP_EQUAL_ARROW = "=>", 
    OP_RETURN = "->",
    
    // operator expression
    //   multiple
    OP_DOUBLE_PLUS = "++", OP_DOUBLE_MINUS = "--",
    OP_PLUS = "+", OP_MINUS = "-",
    OP_STAR = "*",

    //   prefix
    OP_AND = "&", OP_NOT = "!",

    //   binary
    OP_UP_ARROW = "^",
    OP_SLASH = "/", OP_MODULO = "%",
    OP_RANGE_OPEN = "..<", // OP_DOUBLE_DOT
    OP_DOUBLE_EQUAL = "==", OP_FUZZY_EQUAL = "~=", OP_NOT_EQUAL = "!=", OP_FUZZY_NOT_EQUAL = "!~=",
    OP_LESS = "<", OP_MORE = ">", OP_LESS_EQUAL = "<=", OP_MORE_EQUAL = ">=",
    OP_DOUBLE_AND = "&&", 
    OP_OR = "||",

    // pattern
    OP_WILDCARD = "_",
    OP_BOUND = "@",
}