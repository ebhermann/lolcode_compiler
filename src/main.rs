use std::env;
use std::fs;

//
// ===================== Compiler Trait =====================
//

// Compiler
/// Trait for a simple lolcompiler front-end. 
/// Errors should cause immediate exit inside the implementation.
pub trait Compiler {
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str);

    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String;

    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self);

    /// Get the current token being processed.
    fn current_token(&self) -> String;

    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String);
}

//
// ===================== Lexical Analyzer =====================
//

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char;

    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char);

    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool;
}


/// A concrete implementation of the lexical analyzer.
/// // DELETE ALL LOLSPEAK CODE FROM LAB 4
pub struct LolLexicalAnalyzer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>,
    pub previous_token: String,

    pub hai: Vec<String>,
    pub kbye: Vec<String>,
    pub obtw: Vec<String>,
    pub tldr: Vec<String>,
    pub maek: Vec<String>,
    pub oic: Vec<String>,
    pub gimmeh: Vec<String>,
    pub mkay: Vec<String>,
    pub head: Vec<String>,
    pub title: Vec<String>,
    pub paragraf: Vec<String>,
    pub bold: Vec<String>,
    pub list: Vec<String>,
    pub item: Vec<String>,
    pub newline: Vec<String>,
    pub linx: Vec<String>,
    pub ihaz: Vec<String>,
    pub itiz: Vec<String>,
    pub var: Vec<String>,
    pub varname: Vec<String>,
    pub text: Vec<String>,
    pub address: Vec<String>,


    // terminal from the solution BNF that I'm not including for now: varvalue
    // as of now, I think varvalue is just text, so that doesn't seem to need its own lexical rule
}

impl LolLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            previous_token: String::new(),
            
            hai: vec!["#HAI".into(), "#hai".into()],
            kbye: vec!["#KBYE".into(), "#kbye".into()],
            obtw: vec!["#OBTW".into(), "#obtw".into()],
            tldr: vec!["#TLDR".into(), "#tldr".into()],
            maek: vec!["#MAEK".into(), "#maek".into()],
            oic: vec!["#OIC".into(), "#oic".into()],
            gimmeh: vec!["#GIMMEH".into(), "#gimmeh".into()],
            mkay: vec!["#MKAY".into(), "#mkay".into()],
            head: vec!["HEAD".into(), "head".into()],
            title: vec!["TITLE".into(), "title".into()],
            paragraf: vec!["PARAGRAF".into(), "paragraf".into()],
            bold: vec!["BOLD".into(), "bold".into()],
            list: vec!["LIST".into(), "list".into()],
            item: vec!["ITEM".into(), "item".into()],
            newline: vec!["#NEWLINE".into(), "#newline".into()],
            linx: vec!["LINX".into(), "linx".into()],
            ihaz: vec!["#IHAZ".into(), "#ihaz".into()],
            itiz: vec!["#ITIZ".into(), "#itiz".into()],
            var: vec!["VAR".into(), "var".into()],
            varname: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into(), "G".into(), "H".into(), "I".into(), "J".into(), "K".into(),
            "L".into(), "M".into(), "N".into(), "O".into(), "P".into(), "Q".into(), "R".into(), "S".into(), "T".into(), "U".into(), "V".into(), "W".into(), "X".into(),
            "Y".into(), "Z".into(), "a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into(), "i".into(), "j".into(), "k".into(),
            "l".into(), "m".into(), "n".into(), "o".into(), "p".into(), "q".into(), "r".into(), "s".into(), "t".into(), "u".into(), "v".into(), "w".into(), "x".into(),
            "y".into(), "z".into()],
            text: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into(), "G".into(), "H".into(), "I".into(), "J".into(), "K".into(),
            "L".into(), "M".into(), "N".into(), "O".into(), "P".into(), "Q".into(), "R".into(), "S".into(), "T".into(), "U".into(), "V".into(), "W".into(), "X".into(),
            "Y".into(), "Z".into(), "a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into(), "i".into(), "j".into(), "k".into(),
            "l".into(), "m".into(), "n".into(), "o".into(), "p".into(), "q".into(), "r".into(), "s".into(), "t".into(), "u".into(), "v".into(), "w".into(), "x".into(),
            "y".into(), "z".into(), "0".into(), "1".into(), "2".into(), "3".into(), "4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "9".into(), ",".into(),
            ".".into(), "\"".into(), ":".into(), "?".into(), "!".into(), "%".into(), "=".into(), "/".into(), "\n".into(), "\t".into(), ' '.into(), '\r'.into()],
            address: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into(), "G".into(), "H".into(), "I".into(), "J".into(), "K".into(),
            "L".into(), "M".into(), "N".into(), "O".into(), "P".into(), "Q".into(), "R".into(), "S".into(), "T".into(), "U".into(), "V".into(), "W".into(), "X".into(),
            "Y".into(), "Z".into(), "a".into(), "b".into(), "c".into(), "d".into(), "e".into(), "f".into(), "g".into(), "h".into(), "i".into(), "j".into(), "k".into(),
            "l".into(), "m".into(), "n".into(), "o".into(), "p".into(), "q".into(), "r".into(), "s".into(), "t".into(), "u".into(), "v".into(), "w".into(), "x".into(),
            "y".into(), "z".into(), "0".into(), "1".into(), "2".into(), "3".into(), "4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "9".into(), ",".into(),
            ".".into(), "\"".into(), ":".into(), "?".into(), "!".into(), "%".into(), "=".into(), "/".into()],
        }
    } 


    // TODO: EVALUATE TO SEE IF THESE WILL WORK WITH VARNAME AND ADDRESS
    // Current assumption is that they should work under the else branch
    pub fn get_next_token(&mut self) -> String {
        self.current_build.clear();
        let mut c: char = self.get_char();

        if c == '\0' {
            return String::new();
        }
        if c == '#' || self.previous_token == "#GIMMEH" || self.previous_token == "#MAEK" || self.previous_token == "#gimmeh" || self.previous_token == "#maek" {
            while !c.is_whitespace() { //ignoring that this loses a whitespace token for now because I should still be able to translate to HTML
                if c == '\0' {
                    break;
                }
                self.add_char(c);
                c = self.get_char();
            }
            self.tokens.push(self.current_build.clone());
            self.previous_token = self.current_build.clone();
            return self.current_build.clone()
        } else { // this must be text, so one character is a token
            self.add_char(c);
            self.previous_token = self.current_build.clone();
            return self.current_build.clone()
        }
    }
}

impl LexicalAnalyzer for LolLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
            self.hai.iter().any(|hai: &String| hai == s)
            || self.kbye.iter().any(|kbye: &String| kbye == s)
            || self.obtw.iter().any(|obtw: &String| obtw == s)
            || self.tldr.iter().any(|tldr: &String| tldr == s)
            || self.maek.iter().any(|maek: &String| maek == s)
            || self.oic.iter().any(|oic: &String| oic == s)
            || self.gimmeh.iter().any(|gim: &String| gim == s)
            || self.mkay.iter().any(|mkay: &String| mkay == s)
            || self.head.iter().any(|head: &String| head == s)
            || self.title.iter().any(|title: &String| title == s)
            || self.paragraf.iter().any(|para: &String| para == s)
            || self.bold.iter().any(|bold: &String| bold == s)
            || self.list.iter().any(|list: &String| list == s)
            || self.item.iter().any(|item: &String| item == s)
            || self.newline.iter().any(|new: &String| new == s)
            || self.linx.iter().any(|linx: &String| linx == s)
            || self.ihaz.iter().any(|haz: &String| haz == s)
            || self.itiz.iter().any(|itiz: &String| itiz == s)
            || self.var.iter().any(|var: &String| var == s)
            || self.text.iter().any(|txt: &String| txt == s)
        // Var name and address are not included here because they're composed of valid tokens, which are checked under text

    }
}

//
// ===================== Syntax Analyzer =====================
//


pub struct LolspeakCompiler {
    lexer: LolLexicalAnalyzer,
    current_tok: String,
}

impl LolspeakCompiler {
    pub fn new() -> Self {
        Self {
            lexer: LolLexicalAnalyzer::new(""),
            current_tok: String::new(),
        }
    }

    fn start(&mut self) {
        let candidate = self.lexer.get_next_token();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate;
            print!("{}", self.current_tok);
        } else if !candidate.is_empty() {
            eprintln!("Lexical error: '{:?}' is not a recognized token.", candidate);
            std::process::exit(1);
        } else {
            eprintln!("User error: The provided sentence is empty.");
            std::process::exit(1);
        }
    }


    #[inline] fn is_text(&self, s: &str) -> bool { self.lexer.text.iter().any(|txt: &String| txt == s)}
    #[inline] fn is_hai(&self, s: &str) -> bool { self.lexer.hai.iter().any(|hai: &String| hai == s)}
    #[inline] fn is_maek(&self, s: &str) -> bool { self.lexer.maek.iter().any(|maek: &String| maek == s)}
    #[inline] fn is_paragraf(&self, s: &str) -> bool { self.lexer.paragraf.iter().any(|para: &String| para == s)}
  

    // VARNAME AND ADDRESS NEED TO BE ADDED HERE TO TEST LEXER BEFORE MOVING ONTO SYNTAX ANALYSIS
    fn lolspeak(&mut self) {
        self.hai();
        self.maek();
        self.paragraf();
        self.text_list();
    }

    // implemented now to test lexer rules for text. As of now, things seem to be working
    fn text_list(&mut self) {
        if self.is_text(&self.current_tok) {
            self.text();
            self.text_list();
        }
    }

    fn text(&mut self) {
        if self.is_text(&self.current_tok) {
            // add token to parse tree: to be done
            self.next_token();
        }
    }

    fn hai(&mut self) {
        if self.is_hai(&self.current_tok) {
            // add token to parse tree: to be done
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the hai annotation (#HAI, #hai) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn maek(&mut self) {
        if self.is_maek(&self.current_tok) {
            // add token to parse tree: to be done
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the maek annotation (#MAEK, #maek) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }

    fn paragraf(&mut self) {
        if self.is_paragraf(&self.current_tok) {
            // add token to parse tree: to be done
            self.next_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the paragraf annotation (#PARAGRAF, #paragraf) was expected.", self.current_tok);
            std::process::exit(1);
        }
    }


}

impl Compiler for LolspeakCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = LolLexicalAnalyzer::new(source);
        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.get_next_token();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            print!("{}", self.current_tok);
            candidate
        } else if candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{:?}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    fn parse(&mut self) {
        self.lolspeak();
        if !self.current_tok.is_empty() {
            eprintln!("Syntax error: Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    }

    fn current_token(&self) -> String { self.current_tok.clone() }
    fn set_current_token(&mut self, tok: String) { self.current_tok = tok; }
}


fn main() {
// Usage in VS Code terminal:
    //   cargo run -- lolspeak.txt
    // where lolspeak.txt is in the project root (not src/) of the project.

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    // compiler can currently read from a file with a .lol extension, but need to add the functionality that prevents it from reading other file types
    let filename = &args[1];
    let sentence = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolspeakCompiler::new();
    compiler.compile(&sentence);
    compiler.parse();

   // println!("The sentence '{}' follows the lolspeak grammar!", sentence.trim());
}
