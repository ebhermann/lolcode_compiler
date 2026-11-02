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


    pub fn get_next_token(&mut self) -> String {
        self.current_build.clear();
        let mut c: char = self.get_char();

        if c == '\0' {
            return String::new();
        }
        if c == '#' || self.previous_token == "#GIMMEH" || self.previous_token == "#MAEK" || self.previous_token == "#gimmeh" || self.previous_token == "#maek" {
            while !c.is_whitespace() { //ignoring that this loses a whitespace token for now because I should still be able to translate to HTML
                // add to condition: add the !/n, !/t (or whatever the tab is), and /r
                // so this didn't fix it, but maybe there's a way i can add the fucntionality where if the previous character
                // was a tag, ignore the whitespace
                if c == '\0' {
                    break;
                }
                self.add_char(c);
                c = self.get_char();
            }
            self.tokens.push(self.current_build.clone());
            self.previous_token = self.current_build.clone();
            return self.current_build.clone()
        } else { // this must be text, a varname, or an address, so one character is a token
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

/// OPTION 1 - Trait for a recursive descent Syntax Analyzer 
/// over Vec<String>. Each function parses a nonterminal in 
/// the grammar. On error: exit immediately.
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);        
    fn parse_head(&mut self);           
    fn parse_title(&mut self);         
    fn parse_comment(&mut self);        
    fn parse_body(&mut self);           
    fn parse_paragraph(&mut self);      
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);     
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);   
    fn parse_bold(&mut self);           
    fn parse_list(&mut self);           
    fn parse_list_items(&mut self);     
    fn parse_inner_list(&mut self);     
    fn parse_link(&mut self);          
    fn parse_newline(&mut self);        
    fn parse_text(&mut self);           
}


pub struct LolcodeSyntaxAnalyzer<'a> {
    parse_tree: Vec<String>,
    compiler: &'a mut LolcodeCompiler,
}

impl<'a> LolcodeSyntaxAnalyzer<'a> {
    pub fn new(compiler: &'a mut LolcodeCompiler) -> Self {
        Self {
            parse_tree: Vec::new(),
            compiler,
        }
    }

    #[inline] fn is_hai(&self, s: &str) -> bool { self.compiler.lexer.hai.iter().any(|hai: &String| hai == s)}
    #[inline] fn is_obtw(&self, s: &str) -> bool { self.compiler.lexer.obtw.iter().any(|obtw: &String| obtw == s)}
    #[inline] fn is_tldr(&self, s: &str) -> bool { self.compiler.lexer.tldr.iter().any(|tldr: &String| tldr == s)}
    #[inline] fn is_maek(&self, s: &str) -> bool { self.compiler.lexer.maek.iter().any(|maek: &String| maek == s)}
    #[inline] fn is_head(&self, s: &str) -> bool { self.compiler.lexer.head.iter().any(|head: &String| head == s)}
    #[inline] fn is_mkay(&self, s: &str) -> bool { self.compiler.lexer.mkay.iter().any(|mkay: &String| mkay == s)}
    #[inline] fn is_gimmeh(&self, s: &str) -> bool { self.compiler.lexer.gimmeh.iter().any(|gimmeh: &String| gimmeh == s)}
    #[inline] fn is_oic(&self, s: &str) -> bool { self.compiler.lexer.oic.iter().any(|oic: &String| oic == s)}
    #[inline] fn is_title(&self, s: &str) -> bool { self.compiler.lexer.title.iter().any(|title: &String| title == s)}
    #[inline] fn is_ihaz(&self, s: &str) -> bool { self.compiler.lexer.ihaz.iter().any(|ihaz: &String| ihaz == s)}
    #[inline] fn is_varname(&self, s: &str) -> bool { self.compiler.lexer.varname.iter().any(|vname: &String| vname == s)}
    #[inline] fn is_itiz(&self, s: &str) -> bool { self.compiler.lexer.itiz.iter().any(|itiz: &String| itiz == s)}
    #[inline] fn is_newline(&self, s: &str) -> bool { self.compiler.lexer.newline.iter().any(|newline: &String| newline == s)}
    #[inline] fn is_paragraf(&self, s: &str) -> bool { self.compiler.lexer.paragraf.iter().any(|para: &String| para == s)}
    #[inline] fn is_linx(&self, s: &str) -> bool { self.compiler.lexer.linx.iter().any(|link: &String| link == s)}
    #[inline] fn is_var(&self, s: &str) -> bool { self.compiler.lexer.var.iter().any(|var: &String| var == s)}
    #[inline] fn is_list(&self, s: &str) -> bool { self.compiler.lexer.list.iter().any(|list: &String| list == s)}
    #[inline] fn is_item(&self, s: &str) -> bool { self.compiler.lexer.item.iter().any(|item: &String| item == s)}
    #[inline] fn is_bold(&self, s: &str) -> bool { self.compiler.lexer.bold.iter().any(|bold: &String| bold == s)}
    #[inline] fn is_address(&self, s: &str) -> bool { self.compiler.lexer.address.iter().any(|address: &String| address == s)}
    #[inline] fn is_text(&self, s: &str) -> bool { self.compiler.lexer.text.iter().any(|txt: &String| txt == s)}
    #[inline] fn is_kbye(&self, s: &str) -> bool { self.compiler.lexer.kbye.iter().any(|kbye: &String| kbye == s)}
  
    fn add_to_parse_tree(&mut self) {
        self.parse_tree.push(self.compiler.current_tok.clone());
    }


    fn hai(&mut self) {
        if self.is_hai(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the hai annotation (#HAI, #hai) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }

    fn comment_list(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) {
            // adds to parse tree
            self.add_to_parse_tree();
            self.add_to_parse_tree();
            self.compiler.next_token();
            self.comment();
            self.comment_list();
        }
    }

    fn comment(&mut self) {
        if self.is_text(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.parse_text();

            if self.is_tldr(&self.compiler.current_tok) {
                // adds to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
            } else {
                eprintln!("Syntax error: '{}' was found when the tldr annotation (#TLDR, #tldr) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }


    fn list_item(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) {
            // adds to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_item(&self.compiler.current_tok) {
                // adds item to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
                self.parse_variable_define();
                self.parse_inner_list();
                if self.is_oic(&self.compiler.current_tok) {
                    // adds to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_syntax_token();
                } else {
                    eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            }
        }
    }

    fn list_content(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
            self.parse_body();
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
            self.parse_body();
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
            self.parse_body();
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_bold(&self.compiler.current_tok) {
                self.parse_bold();
                self.parse_body();
            } else if self.is_linx(&self.compiler.current_tok) {
                self.parse_link();
                self.parse_body();
            } else if self.is_var(&self.compiler.current_tok) {
                self.parse_variable_use();
                self.parse_body();
            } else {
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when the var, comment, bold, link, newline, or text annotations (VAR, var, #OBTW, #obtw, BOLD, bold #MAEK, #maek, LINX, linx, #NEWLINE, #newline) were expected.", self.compiler.current_tok);
                std::process::exit(1);
        }
    }


     // implemented now to test lexer rules for text. As of now, things seem to be working
    fn text_list(&mut self) {
        if self.is_text(&self.compiler.current_tok) {
            self.text();
            self.text_list();
        }
    }

    fn text(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token();
    }

    fn varname_list(&mut self) {
        if self.is_varname(&self.compiler.current_tok) {
            self.varname();
            self.varname_list();
        }
    }

    fn varname(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token();
    }

     fn address_list(&mut self) {
        if self.is_address(&self.compiler.current_tok) {
            self.address();
            self.address_list();
        }
    }

    fn address(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token();
    }

    fn kbye(&mut self) {
        if self.is_kbye(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the kbye annotation (#KBYE, #kbye) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }
}

impl<'a> SyntaxAnalyzer for LolcodeSyntaxAnalyzer<'a>{
    fn parse_lolcode(&mut self) {
        self.hai();
        self.parse_comment();
        self.parse_head();
        self.parse_variable_define();
        self.parse_body();
        self.kbye();
    }
    fn parse_head(&mut self) {
        if self.is_maek(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();

            if self.is_head(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
                self.parse_title();

                if self.is_mkay(&self.compiler.current_tok) {
                    // adds token to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_syntax_token();
                } else {
                    eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            }
        }
    }        

    fn parse_title(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_title(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_token();
                if self.is_text(&self.compiler.current_tok) {
                    self.parse_text();
                } else {
                    eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
                if self.is_oic(&self.compiler.current_tok) {
                    // adds token to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_syntax_token();

                } else {
                    eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Syntax error: '{}' was found when the title annotation (#MAEK, #maek) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when the gimmeh annotation (#GIMMEH, #gimmeh) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }  


    fn parse_comment(&mut self) {
        self.comment_list();
    }     


    fn parse_body(&mut self) { 
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
            self.parse_body();
        } else if self.is_maek(&self.compiler.current_tok) { // list and paragraf
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_list(&self.compiler.current_tok) {
                self.parse_list();
                self.parse_body();
            } else if self.is_paragraf(&self.compiler.current_tok) {
                self.parse_paragraph();
                self.parse_body();
            } else {
                eprintln!("Syntax error: '{}' was found when the paragraf or list annotations (PARAGRAF, paragraf, LIST, list) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
            self.parse_body();
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
            self.parse_body();
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_bold(&self.compiler.current_tok) {
                self.parse_bold();
                self.parse_body();
            } else if self.is_linx(&self.compiler.current_tok) {
                self.parse_link();
                self.parse_body();
            } else if self.is_var(&self.compiler.current_tok) {
                self.parse_variable_use();
                self.parse_body();
            } else {
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        }
    }     


    fn parse_paragraph(&mut self) {
        // Only reach this function if the current token is paragraf
        self.add_to_parse_tree();
        self.compiler.next_syntax_token();
        self.parse_variable_define();
        self.parse_inner_paragraph();
        if self.is_mkay(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }   


    fn parse_inner_paragraph(&mut self) {
        if !self.is_mkay(&self.compiler.current_tok) {
            self.parse_inner_text();
            self.parse_inner_paragraph();
        }
    }


    fn parse_inner_text(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
            self.parse_body();
        } else if self.is_maek(&self.compiler.current_tok) { // list
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_list(&self.compiler.current_tok) {
                self.parse_list();
                self.parse_body();
            } else {
                eprintln!("Syntax error: '{}' was found when the list annotation (LIST, list) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
            self.parse_body();
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
            self.parse_body();
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_bold(&self.compiler.current_tok) {
                self.parse_bold();
                self.parse_body();
            } else if self.is_linx(&self.compiler.current_tok) {
                self.parse_link();
                self.parse_body();
            } else if self.is_var(&self.compiler.current_tok) {
                self.parse_variable_use();
                self.parse_body();
            } else {
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when the var, comment, bold, list, link, newline, or text annotations (VAR, var, #OBTW, #obtw, BOLD, bold #MAEK, #maek, LIST, list, LINX, linx, #NEWLINE, #newline) were expected.", self.compiler.current_tok);
                std::process::exit(1);
        }
    }   


    fn parse_variable_define(&mut self) {
        if self.is_ihaz(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_varname(&self.compiler.current_tok) {
                self.varname_list();
                self.compiler.next_syntax_token();
                if self.is_itiz(&self.compiler.current_tok) {
                    // adds token to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_token();
                    if self.is_text(&self.compiler.current_tok) {
                        self.parse_text(); // Since VARVALUE is just valid text characters
                        if self.is_mkay(&self.compiler.current_tok) {
                            // adds token to parse tree
                            self.add_to_parse_tree();
                            self.compiler.next_token();
                        } else {
                            eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("Syntax error: '{}' was found when a variable value was expected.", self.compiler.current_tok);
                        std::process::exit(1);
                    }

                } else {
                    eprintln!("Syntax error: '{}' was found when the itiz annotation (#ITIZ, #itiz) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            } else {
                eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        }
    }


    // start with var, not gimmeh
    fn parse_variable_use(&mut self) {
        // gimmeh has already been added to the parse tree and var is the current token
        self.add_to_parse_tree();
        self.compiler.next_syntax_token();

        if self.is_varname(&self.compiler.current_tok) {
            self.varname_list();
            self.compiler.next_syntax_token();
            if self.is_oic(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
            } else {
                eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when a variable name was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    } 


    // start with bold not gimmeh
    fn parse_bold(&mut self) {
        // GIMMEH annotation has already been added to the parse tree at this point
        // current token is already bold or BOLD
        self.add_to_parse_tree();
        self.compiler.next_token();
        if self.is_text(&self.compiler.current_tok) {
            self.parse_text();
            // text has moved current token ahead
            if self.is_oic(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
            } else {
                eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }   


    fn parse_list(&mut self) {
        // maek has already been added to the parse tree
        // current token is list
        self.add_to_parse_tree();
        self.compiler.next_syntax_token();
        self.list_item();
        self.parse_list_items();
        if self.is_mkay(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }    


    fn parse_list_items(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) {
            self.list_item();
            self.parse_list_items();
        }
    }     


    fn parse_inner_list(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) || self.is_newline(&self.compiler.current_tok) || self.is_text(&self.compiler.current_tok) 
        || self.is_gimmeh(&self.compiler.current_tok) {
            self.list_content();
            self.parse_inner_list();
        }
    }    

    // start with linx, not gimmeh
    fn parse_link(&mut self) {
        // gimmeh has already been added to the parse tree at this point
        // // already have linx as the current token
        self.add_to_parse_tree();
        self.compiler.next_syntax_token();
        if self.is_address(&self.compiler.current_tok) {
            // adds token to parse tree
            self.address_list();
            self.compiler.next_syntax_token();

            if self.is_oic(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token();
            } else {
                eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: '{}' was found when a link address was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }  


    fn parse_newline(&mut self) {
        if self.is_newline(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the newline annotation (#NEWLINE, #newline) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }   


    fn parse_text(&mut self) {
        self.text_list();
    }

    
}


pub struct LolcodeCompiler {
    lexer: LolLexicalAnalyzer,
    current_tok: String,
}

impl LolcodeCompiler {
    pub fn new() -> Self {
        Self {
            lexer: LolLexicalAnalyzer::new(""),
            current_tok: String::new(),
        }
    }

    fn next_syntax_token(&mut self) {
        let mut candidate = self.next_token();

        while !candidate.is_empty() && candidate.chars().all(|c| c.is_whitespace()) {
            candidate = self.next_token();
        }
    }


    fn start(&mut self) {

        let mut candidate = self.lexer.get_next_token();

        // Skip whitespace before the first annotation
        while !candidate.is_empty() && candidate.chars().all(|c| c.is_whitespace()) {
            candidate = self.lexer.get_next_token();
        }

        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate;
        } else if !candidate.is_empty() {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        } else {
            eprintln!("User error: The provided sentence is empty.");
            std::process::exit(1);
        }
    }





}

impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = LolLexicalAnalyzer::new(source);
        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.get_next_token();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            // need to delete this before submitting the project
            print!("{}", self.current_tok);
            candidate
        } else if candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    fn parse(&mut self) {
        {
            let mut syntax_analyzer = LolcodeSyntaxAnalyzer::new(self);
            syntax_analyzer.parse_lolcode();
        }
        if !self.current_tok.is_empty() {
            eprintln!("Syntax error: Additional tokens found after the kbye annotation.");
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

    // Prevents compiler from reading any files without a .lol extension
    let filename = &args[1];
    let mut extension = String::new();
    for c in filename.chars().rev() {
        if c == '.' {
            break;
        }
        extension.push(c);    
    }

    if extension != "lol" {
        eprintln!("Usage error. Extension must be .lol Found extension .{}", extension);
        std::process::exit(1);
    }

    let sentence = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolcodeCompiler::new();
    compiler.compile(&sentence);
    compiler.parse();

   // println!("The sentence '{}' follows the lolspeak grammar!", sentence.trim());
}
