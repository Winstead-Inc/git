#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source_Language
{
    C,
    Cpp,
    CSharp,
    Generic
}

impl Source_Language
{
    pub fn FromExtension(PathString: &str) -> Self
    {
        let Lower = PathString.to_lowercase();
        if Lower.ends_with(".cs") { return Source_Language::CSharp; }
        if Lower.ends_with(".cpp") || Lower.ends_with(".cxx") || Lower.ends_with(".cc")
            || Lower.ends_with(".hpp") || Lower.ends_with(".hxx") || Lower.ends_with(".hh")
            || Lower.ends_with(".inl") || Lower.ends_with(".cu")
        {
            return Source_Language::Cpp;
        }
        if Lower.ends_with(".c") || Lower.ends_with(".h") { return Source_Language::C; }
        Source_Language::Generic
    }

    pub fn SupportsAngleGenerics(&self) -> bool
    {
        matches!(self, Source_Language::Cpp | Source_Language::CSharp)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Container_Kind
{
    Paren,
    Brace,
    Bracket,
    Angle
}

impl Container_Kind
{
    pub fn OpenGlyph(&self) -> &'static str
    {
        match self
        {
            Container_Kind::Paren => "(",
            Container_Kind::Brace => "{",
            Container_Kind::Bracket => "[",
            Container_Kind::Angle => "<"
        }
    }

    pub fn CloseGlyph(&self) -> &'static str
    {
        match self
        {
            Container_Kind::Paren => ")",
            Container_Kind::Brace => "}",
            Container_Kind::Bracket => "]",
            Container_Kind::Angle => ">"
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token_Kind
{
    Identifier(String),
    Keyword(String),
    Number(String),
    String_Lit(String),
    Char_Lit(String),
    Comment_Line(String),
    Comment_Block(String),
    Preprocessor(String),
    Open_Delim(Container_Kind),
    Close_Delim(Container_Kind),
    Operator(String),
    Punctuation(char),
    Whitespace(String),
    Newline,
    Eof
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token_Info
{
    pub Kind: Token_Kind,
    pub Text: String,
    pub Line: usize,
    pub Column: usize,
    pub Preceding_Newlines: usize
}

impl Token_Info
{
    pub fn New(Kind: Token_Kind, Text: String, Line: usize, Column: usize, Preceding_Newlines: usize) -> Self
    {
        Token_Info
        {
            Kind,
            Text,
            Line,
            Column,
            Preceding_Newlines
        }
    }

    pub fn IsTrivia(&self) -> bool
    {
        matches!(self.Kind, Token_Kind::Whitespace(_) | Token_Kind::Newline)
    }

    pub fn IsComment(&self) -> bool
    {
        matches!(self.Kind, Token_Kind::Comment_Line(_) | Token_Kind::Comment_Block(_))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout_State
{
    Monolith,
    Grouped,
    Expanded
}

#[derive(Debug, Clone)]
pub struct Config_Settings
{
    pub Canvas_Width: usize,
    pub Indent_Width: usize,
    pub Use_Tabs: bool,
    pub Max_Statements_In_Monolith: usize,
    pub Max_Block_Depth: usize,
    pub Max_Containers_In_Monolith: usize,
    pub Max_Blank_Lines: usize,
    pub Language: Source_Language,
    pub Allow_Brace_Grouping: bool
}

impl Default for Config_Settings
{
    fn default() -> Self
    {
        Config_Settings
        {
            Canvas_Width: 160,
            Indent_Width: 4,
            Use_Tabs: false,
            Max_Statements_In_Monolith: 4,
            Max_Block_Depth: 2,
            Max_Containers_In_Monolith: 12,
            Max_Blank_Lines: 1,
            Language: Source_Language::Generic,
            Allow_Brace_Grouping: true
        }
    }
}

impl Config_Settings
{
    pub fn IndentString(&self, Level: usize) -> String
    {
        if self.Use_Tabs
        {
            "\t".repeat(Level)
        }
        else
        {
            " ".repeat(Level * self.Indent_Width)
        }
    }
}
