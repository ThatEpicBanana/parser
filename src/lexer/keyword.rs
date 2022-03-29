use std::{fmt};

// https://stackoverflow.com/a/34324856
// count using recursion (i think)
macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! kw {
    { $( $lex:ident = $true:expr),* $(,)? } => {
        $(
            pub const $lex: Keyword = Keyword($true);
        )*

        #[allow(non_upper_case_globals)] // not upper case to differentiate from others
        pub const keywords: [Keyword; count!($($lex)*)] = [$($lex),*];
        // pub fn encode() -> HashMap<String, Keyword> {
        //     HashMap::from([
        //         $(($true.to_string(), $lex)),*
        //     ])
        // }

        // pub fn decode() -> HashMap<Keyword, String> {
        //     HashMap::from([
        //         $(($lex, $true.to_string())),*
        //     ])
        // }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Keyword(&'static str);

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn from_string(string: &String) -> Option<Keyword> {
    keywords.into_iter().find(|x| x.0 == string)
}

kw!{
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
    KW_ID = "id", KW_POS = "pos", KW_SEL = "sel", KW_DAT = "dat", KW_BRD = "brd", KW_CMD = "cmd", // string
    KW_B = "b", KW_I = "i", KW_S = "s", KW_L = "l", // integer
    KW_F = "f", KW_ANG = "ang", // float

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