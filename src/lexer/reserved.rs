macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

pub mod keyword {
    use std::fmt;

    // macro for constructing the keywords
    macro_rules! kw_macro {
        { $( $lex:ident = $true:expr),* $(,)? } => {
            $(
                pub const $lex: Keyword = Keyword($true);
            )*
    
            #[allow(non_upper_case_globals)] // not upper case to differentiate from others
            pub const keywords: [Keyword; count!($($lex)*)] = [$($lex),*];
        };
    }
    
    // - various stuff for storing a keyword -
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Keyword(&'static str);
    
    impl fmt::Display for Keyword {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    // - utility -
    pub fn from_string(string: &String) -> Option<Keyword> {
        keywords.into_iter().find(|x| x.0 == string)
    }

    // in seperate module for prelude
    pub(crate) mod for_export {
        use crate::lexer::Token;
        use super::*;

        pub fn kw(keyword: Keyword) -> Token { Token::KEYWORD(keyword) }
        
        // - actual definitions -
        kw_macro!{
            // various types
            KW_LET = "let",
            KW_DATA = "data",
            KW_SCORE = "score",
            KW_ENTITY = "entity",
            KW_BLOCK = "block",
            // items
            KW_MOD = "mod",
            KW_CLASS = "class", KW_EXTENDS = "extends",
            KW_ENUM = "enum",
            KW_IMPORT = "import",
            KW_MACRO = "macro", KW_PROC_MACRO = "proc_macro",
            KW_TYPE = "type",
            KW_FUNC = "func",
            KW_GET = "get", KW_SET = "set", // properties
        
            // statements
            KW_BREAK = "break",
            KW_CONTINUE = "continue",
            KW_RETURN = "return",
        
            // suffixes
            // could be removed
            // KW_ID = "id", KW_POS = "pos", KW_SEL = "sel", KW_DAT = "dat", KW_BRD = "brd", KW_CMD = "cmd", // string
            // KW_B = "b", KW_I = "i", KW_S = "s", KW_L = "l", // integer
            // KW_F = "f", KW_ANG = "ang", // float
        
            // expressions
            KW_NEW = "new", KW_SUMMON = "summon", KW_INST = "inst", // instantiating
            KW_AS = "as", KW_IMPL = "impl", KW_INIT = "init", // casting
            KW_LOOP = "loop", KW_WHILE = "while", KW_FOR = "for", KW_IN = "in", // loops
            KW_IF = "if", KW_ELSE = "else", 
            KW_MATCH = "match",
            KW_TRY = "try", KW_CATCH = "catch",
            //   execute
            KW_ALIGN = "align", KW_ANCHORED = "anchored", // in
            KW_AT = "at", KW_LIKE = "like", // as
            KW_FACING = "facing", 
            KW_POSITIONED = "positioned", KW_ROTATED = "rotated", 
            KW_EYES = "eyes", KW_FEET = "feet",
        
            // visibility
            KW_PUB = "pub",
            KW_PRV = "prv",
        
            // paths
            KW_SUPER = "super",
            KW_SELF = "self",
            KW_BASKET = "basket",
            KW_THIS = "this",
        }
    } pub use for_export::*;
}

pub mod operator {
    use std::fmt;

    // macro for constructing the operators
    macro_rules! op_macro {
        { $( $lex:ident = $true:expr),* $(,)? } => {
            $(
                pub const $lex: Operator = Operator($true);
            )*
    
            #[allow(non_upper_case_globals)] // not upper case to differentiate from others
            pub const operators: [Operator; count!($($lex)*)] = [$($lex),*];
        };
    }
    
    // - various stuff for storing an operator -
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    pub struct Operator(&'static str);
    
    impl fmt::Display for Operator {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    
    // - utility -
    pub fn from_string(string: &String) -> Option<Operator> {
        operators.into_iter().find(|x| x.0 == string)
    }

    // in seperate module for prelude
    pub(crate) mod for_export {
        use crate::lexer::Token;
        use super::*;

        pub fn op(operator: Operator) -> Token { Token::OPERATOR(operator) }
        
        // - actual definitions -
        op_macro!{
            // other / in multiple
            OP_COLON = ":", OP_DOUBLE_COLON = "::",
            OP_DOUBLE_DOT = "..", OP_TRIPLE_DOT = "...",
            OP_BAR = "|",
            OP_AT = "@", OP_BACKTICK = "`",
        
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
        }
    } pub use for_export::*;
}