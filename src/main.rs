use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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


/// Struct for the lexical analyzer.
pub struct LolLexicalAnalyzer {
    input: Vec<char>, // Input for the compiler
    position: usize, // Where the compiler is in the input
    current_build: String, // What's currently part of the token being constructed
    pub previous_token: String, // Previous token

    // Valid tokens in the language
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

}

// Lexical analyzer implementation
impl LolLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(), // Input is the characters in the source file
            position: 0, // Start at the beginning of the source file
            current_build: String::new(),
            previous_token: String::new(),
            
            // Specific valid tokens
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


    /* 
        This function gets the next token in the input. How it gets the next token depends on whether the next token is a single-word annotation,
        two-word annotation, or text
     */
    pub fn get_next_token(&mut self) -> String {
        self.current_build.clear(); // Clear the current build (which is the token being constructed) so the next token gets a fresh start
        let mut c: char = self.get_char(); // Get the next character in the input

        // if the character is empty, return an empty string since there is no valid token
        if c == '\0' {
            return String::new();
        }

        // If the current character is # or the previous token was gimmeh or maek (indicating a 2-word annotation), add characters to the token under
        // construction until a space or some other form of whitespace is encountered
        if c == '#' || self.previous_token == "#GIMMEH" || self.previous_token == "#MAEK" || self.previous_token == "#gimmeh" || self.previous_token == "#maek" {
            while !c.is_whitespace() { // while there isn't whitespace...
                if c == '\0' {
                    break;
                }
                self.add_char(c); // add the current character to the token under construction
                c = self.get_char(); // get the next character
            }
            // whitespace has now been encountered
            self.previous_token = self.current_build.clone(); // store this token as the previous token
            return self.current_build.clone() // return this token
        } else { // this must be text, a varname, a variable value, or an address, so one character is a token
            self.add_char(c); // current build is one character
            self.previous_token = self.current_build.clone(); // store this token as the previous token
            return self.current_build.clone() // return this one-character token
        }
    }
}

// Lexical analyzer trait implementation
impl LexicalAnalyzer for LolLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() { // If the input's length hasn't been exceeded
            let c = self.input[self.position]; // Character is the input character at the current position
            self.position += 1; // Increment position
            c
        } else {
            '\0' // otherwise return empty character
        }
    }

    // adds the character to the token under construction (current_build)
    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    // This method determines if the token is a valid token in this language
    // If the token is in any of the valid token categories, this function returns true
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
    // Functions for parsing
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
    pub parse_tree: Vec<String>,
    compiler: &'a mut LolcodeCompiler,
}

impl<'a> LolcodeSyntaxAnalyzer<'a> {
    pub fn new(compiler: &'a mut LolcodeCompiler) -> Self {
        Self {
            parse_tree: Vec::new(),
            compiler, // Borrow the current compiler so the same lexer is used
        }
    }

    // Methods for checking that the current token is part of a specific token category
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
  
    // This method adds the current token to the parse tree
    fn add_to_parse_tree(&mut self) {
        self.parse_tree.push(self.compiler.current_tok.clone());
    }

    // The following functions are used for syntax analysis and not part of the syntax analyzer trait

    fn hai(&mut self) {
        if self.is_hai(&self.compiler.current_tok) { // if the current token is hai
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // move to the next token
        } else { // otherwise error
            eprintln!("Syntax error: '{}' was found when the hai annotation (#HAI, #hai) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }

    // Recursive method for parsing multiple comments
    fn comment_list(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) { // if the current token is obtw
            // adds to parse tree
            self.add_to_parse_tree();
            self.compiler.next_token(); // move to the next token
            self.comment(); // call the comment parsing function
            self.comment_list(); // call the current method again for parsing multiple comments
        } // otherwise no error because comments aren't required
    }

    // Method for parsing a comment
    fn comment(&mut self) {
        if self.is_text(&self.compiler.current_tok) { // If the current token is text
            self.parse_text(); // Call the function to parse text

            if self.is_tldr(&self.compiler.current_tok) { // If the current token is tldr
                // adds to parse tree
                self.add_to_parse_tree(); // Add tldr to the parse tree
                self.compiler.next_syntax_token(); // Get the next syntax token (so whitespace is ignored)
            } else { // Otherwise error, because tldr is required
                eprintln!("Syntax error: '{}' was found when the tldr annotation (#TLDR, #tldr) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else { // Otherwise error, because text is required in a comment
            eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }


    // Function for parsing a list item
    fn list_item(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) { // If list_item has been called, gimmeh is required
            // adds to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // move to next token, ignoring whitespace
            if self.is_item(&self.compiler.current_tok) { // if current token is item
                // adds item to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token(); // move to next token, ignoring whitespace
                self.parse_variable_define(); // call the function for parsing a variable definition
                self.parse_inner_list(); // call the function for parsing the rest of the list (the "inner list")
                if self.is_oic(&self.compiler.current_tok) { // now at the end of the list item, the current token should be oic
                    // adds to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_syntax_token();
                } else { // otherwise error
                    eprintln!("Syntax error: '{}' was found when the oic annotation (#OIC, #oic) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            }
        } else { // if no gimmeh, error
           eprintln!("Syntax error: '{}' was found when the gimmeh annotation (#GIMMEH, #gimmeh) was expected.", self.compiler.current_tok);
            std::process::exit(1); 
        }
    }

    // Function for parsing list content
    // If list content is called, one of its valid constructs is required
    fn list_content(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use. All start with gimmeh, so need to determine which annotation is next
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // get next token, ignoring whitespace
            if self.is_bold(&self.compiler.current_tok) { // bold
                self.parse_bold();
            } else if self.is_linx(&self.compiler.current_tok) { // link
                self.parse_link();
            } else if self.is_var(&self.compiler.current_tok) { // variable use
                self.parse_variable_use();
            } else { // When gimmeh is used in list content, one of the three annotations is required after
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else { // An annotation is required in list content
            eprintln!("Syntax error: '{}' was found when the var, comment, bold, link, newline, or text annotations (VAR, var, #OBTW, #obtw, BOLD, bold #MAEK, #maek, LINX, linx, #NEWLINE, #newline) were expected.", self.compiler.current_tok);
                std::process::exit(1);
        }
    }


    // Recursive function for parsing texting
    fn text_list(&mut self) {
        if self.is_text(&self.compiler.current_tok) { // If current token is text
            self.text(); // Call parsing method for text
            self.text_list(); // Call text list for recursion
        }
    }

    fn text(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token(); // get next token, allowing whitespace
    }

    // Recursive function for parsing and building the variable name
    fn varname_list(&mut self) {
        if self.is_varname(&self.compiler.current_tok) { // if the current token is part of a variable name
            self.varname(); // call parsing function for a variable name
            self.varname_list(); // call varname list for recursion
        }
    }

    fn varname(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token(); // get next token, allowing whitespace because whitespace indicates the variable name is done being constructed
    }

    // Recursive function for parsing and building the address
     fn address_list(&mut self) {
        if self.is_address(&self.compiler.current_tok) { // if the current token is part of the address token category
            self.address(); // call parsing function for address
            self.address_list(); // call address list for recursion
        }
    }

    fn address(&mut self) {
        // adds token to parse tree
        self.add_to_parse_tree();
        self.compiler.next_token(); // get next token, allowing whitespace because whitespace indicates the address is complete
    }

    // Function for parsing kbye
    fn kbye(&mut self) {
        if self.is_kbye(&self.compiler.current_tok) { // if current token is kbye
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // get next token, ignoring whitespace
        } else { // else error: kbye is required
            eprintln!("Syntax error: '{}' was found when the kbye annotation (#KBYE, #kbye) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }
}

// Implementation of the syntax analyzer trait
impl<'a> SyntaxAnalyzer for LolcodeSyntaxAnalyzer<'a>{

    // Function for parsing an lolcode document
    fn parse_lolcode(&mut self) {
        self.hai(); // Parse leading tag
        self.parse_comment(); // Parse optional comment
        self.parse_head(); // Parse optional head
        self.parse_variable_define(); // Parse optional variable definition
        self.parse_body(); // Parse optional body
        self.kbye(); // Parse closing tag
    }

    // Parse head annotation
    fn parse_head(&mut self) {
        if self.is_maek(&self.compiler.current_tok) { // If the current token is maek, continue to parse
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // get next token, ignoring whitespace

            if self.is_head(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_syntax_token(); // get next token, ignoring whitespace
                self.parse_title(); // parse required title annotation

                if self.is_mkay(&self.compiler.current_tok) { // if next (closing) token is mkay
                    // adds token to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_syntax_token(); // get next token, ignoring whitespace
                } else { // otherwise error because mkay is required to close a head annotation
                    eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
            }
        } // otherwise no error because head isn't required
    }        

    // Function for parsing a title
    fn parse_title(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) { // Title must start with gimmeh annotation
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token(); // get next token, ignoring whitespace
            if self.is_title(&self.compiler.current_tok) {
                // adds token to parse tree
                self.add_to_parse_tree();
                self.compiler.next_token(); // get next token, not ignoring whitespace because the next token should be text
                if self.is_text(&self.compiler.current_tok) { // text is required within the title annotation
                    self.parse_text();
                } else {
                    eprintln!("Syntax error: '{}' was found when text was expected.", self.compiler.current_tok);
                    std::process::exit(1);
                }
                if self.is_oic(&self.compiler.current_tok) { // title annotation must close with oic annotation
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
        } else { // Otherwise error because the title annotation is required in the head annotation 
            eprintln!("Syntax error: '{}' was found when the gimmeh annotation (#GIMMEH, #gimmeh) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }  


    // Method for parsing a comment
    fn parse_comment(&mut self) {
        self.comment_list(); // Utilizes comment list because there can be multiple comments
    }     

    // Method for parsing the body of a lolcode file
    fn parse_body(&mut self) { 
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
            self.parse_body(); // recurse
        } else if self.is_maek(&self.compiler.current_tok) { // list and paragraf
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_list(&self.compiler.current_tok) { // list
                self.parse_list();
                self.parse_body(); // recurse
            } else if self.is_paragraf(&self.compiler.current_tok) { // paragraf
                self.parse_paragraph();
                self.parse_body(); // recurse
            } else { // only list and paragraf are valid tokens here
                eprintln!("Syntax error: '{}' was found when the paragraf or list annotations (PARAGRAF, paragraf, LIST, list) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
            self.parse_body(); // recurse
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
            self.parse_body(); // recurse
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_bold(&self.compiler.current_tok) { // bold
                self.parse_bold();
                self.parse_body(); // recurse
            } else if self.is_linx(&self.compiler.current_tok) { // linx
                self.parse_link();
                self.parse_body(); // recurse
            } else if self.is_var(&self.compiler.current_tok) { // var
                self.parse_variable_use();
                self.parse_body(); // recurse
            } else { // only bold, linx, or var are valid here
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        // If any token other than the ones listed above is used and that token is not kbye, it's an error because a lolcode body must either
        // have one of the above annotations or be empty
        } else if !self.is_kbye(&self.compiler.current_tok) {
            eprintln!("Syntax error: '{}' was found when a valid body element or the kbye annotation was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }     

    // Function for parsing a paragraph
    fn parse_paragraph(&mut self) {
        // Only reach this function if the current token is paragraf
        self.add_to_parse_tree(); // Current token is paragraf, so add it to the parse tree
        self.compiler.next_syntax_token();
        self.parse_variable_define(); // parse optional variable definition
        self.parse_inner_paragraph(); // parse optional inner paragraph
        if self.is_mkay(&self.compiler.current_tok) { // have reached the end of the paragraph, token must be mkay to close the annotation
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }   


    // Function for parsing the optional inner paragraph
    fn parse_inner_paragraph(&mut self) {
        if !self.is_mkay(&self.compiler.current_tok) { // If the token is not mkay (which indicates the end of a paragraph)...
            self.parse_inner_text(); // parse inner text
            self.parse_inner_paragraph(); // recurse
        }
    }


    fn parse_inner_text(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) { // comment
            self.parse_comment();
        } else if self.is_maek(&self.compiler.current_tok) { // list
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_list(&self.compiler.current_tok) {
                self.parse_list();
            } else { // maek must be followed by list in the inner text grammar rule
                eprintln!("Syntax error: '{}' was found when the list annotation (LIST, list) was expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else if self.is_newline(&self.compiler.current_tok) { // newline
            self.parse_newline();
        } else if self.is_text(&self.compiler.current_tok) { // text
            self.parse_text();
        } else if self.is_gimmeh(&self.compiler.current_tok) { // bold, link, and variable use
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_bold(&self.compiler.current_tok) { // bold
                self.parse_bold();
            } else if self.is_linx(&self.compiler.current_tok) { // linx
                self.parse_link();
            } else if self.is_var(&self.compiler.current_tok) { // var
                self.parse_variable_use();
            } else { // Only bold, linx, and var are valid after gimmeh inside inner text
                eprintln!("Syntax error: '{}' was found when the bold, linx, or var annotations (BOLD, bold, LINK, linx, VAR, var) were expected.", self.compiler.current_tok);
                std::process::exit(1);
            }
        } else { // If there is inner text, one of the valid annotations must be present, so error otherwise
            eprintln!("Syntax error: '{}' was found when the var, comment, bold, list, link, newline, or text annotations (VAR, var, #OBTW, #obtw, BOLD, bold #MAEK, #maek, LIST, list, LINX, linx, #NEWLINE, #newline) were expected.", self.compiler.current_tok);
                std::process::exit(1);
        }
    }   


    // Function for parsing an optional variable definition
    fn parse_variable_define(&mut self) {
        if self.is_ihaz(&self.compiler.current_tok) { // if the current token is ihaz, enter the grammar rule for a variable definition
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
            if self.is_varname(&self.compiler.current_tok) { // variable definition must have a variable name
                self.varname_list(); // parse variable name
                self.compiler.next_syntax_token();
                if self.is_itiz(&self.compiler.current_tok) { // variable definition must have itiz
                    // adds token to parse tree
                    self.add_to_parse_tree();
                    self.compiler.next_token();
                    if self.is_text(&self.compiler.current_tok) { // variable definition must have a value for the variable
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
        } // no error because the definition is optional
    }


    // start with var, not gimmeh
    // Function for parsing a variable name
    fn parse_variable_use(&mut self) {
        // gimmeh has already been added to the parse tree and var is the current token
        self.add_to_parse_tree();
        self.compiler.next_syntax_token();

        if self.is_varname(&self.compiler.current_tok) { // At this point, "gimmeh var" has been added to the parse tree, so a variable name is required
            self.varname_list();
            self.compiler.next_syntax_token();
            if self.is_oic(&self.compiler.current_tok) { // variable use ends with oic
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
    // Function for parsing the bold grammar rule
    fn parse_bold(&mut self) {
        // GIMMEH annotation has already been added to the parse tree at this point
        // current token is already bold or BOLD
        self.add_to_parse_tree(); // add bold to parse tree
        self.compiler.next_token(); // get next token including whitespace since next token should be text, and whitespace counts as text
        if self.is_text(&self.compiler.current_tok) { // text is required within the bold annotation
            self.parse_text(); // parse text
            // text has moved current token ahead
            if self.is_oic(&self.compiler.current_tok) { // current token should be oic to close out the bold annotation
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

    // Function for parsing a list
    fn parse_list(&mut self) {
        // maek has already been added to the parse tree
        // current token is list
        self.add_to_parse_tree(); // add list token to parse tree
        self.compiler.next_syntax_token();
        self.list_item(); // at least one list item is required
        self.parse_list_items(); // subsequent list items are optional
        if self.is_mkay(&self.compiler.current_tok) { // list annotation must end with mkay
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } else {
            eprintln!("Syntax error: '{}' was found when the mkay annotation (#MKAY, #mkay) was expected.", self.compiler.current_tok);
            std::process::exit(1);
        }
    }    

    // Recursive function for parsing multiple list items
    fn parse_list_items(&mut self) {
        if self.is_gimmeh(&self.compiler.current_tok) { // at this point, list items are optional
            self.list_item(); // parse the list item
            self.parse_list_items(); // recurse
        }
    }     


    // Function for parsing the inside of a list
    // This function is called by the list_item method
    fn parse_inner_list(&mut self) {
        if self.is_obtw(&self.compiler.current_tok) || self.is_newline(&self.compiler.current_tok) || self.is_text(&self.compiler.current_tok) 
        || self.is_gimmeh(&self.compiler.current_tok) { // Inner list is optional
            self.list_content();
            self.parse_inner_list();
        }
    }    

    // start with linx, not gimmeh
    // Function for parsing a link
    fn parse_link(&mut self) {
        // gimmeh has already been added to the parse tree at this point
        // already have linx as the current token
        self.add_to_parse_tree(); // add linx to parse tree
        self.compiler.next_syntax_token();
        if self.is_address(&self.compiler.current_tok) { // link address is required
            // adds token to parse tree
            self.address_list(); // parse address
            self.compiler.next_syntax_token();

            if self.is_oic(&self.compiler.current_tok) { // link annotation must end with oic
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

    // Function for parsing a newline
    fn parse_newline(&mut self) {
        if self.is_newline(&self.compiler.current_tok) {
            // adds token to parse tree
            self.add_to_parse_tree();
            self.compiler.next_syntax_token();
        } // No error because this function is only called when it has already been determined to be a newline, and newlines are never required
    }   

    // Function for parsing text: hands parsing off to recursive text list function
    fn parse_text(&mut self) {
        self.text_list();
    }

    
}

use std::collections::HashMap;

pub struct LolcodeSemanticAnalyzer {
    parse_tree: Vec<String>,
    resolved_parse_tree: Vec<String>,
    active_scope_stack: Vec<HashMap<String, String>>,
}

// Implementation of the LolcodeSemantic Analyzer
impl LolcodeSemanticAnalyzer {
    pub fn new(parse_tree: Vec<String>) -> Self {
        Self {
            parse_tree, // Takes parse tree that has been built
            resolved_parse_tree: Vec::new(), // Resolved parse tree contains variable resolutions and removes variable annotations since they aren't needed for html translation
            active_scope_stack: Vec::new(),
        }
    }

    // Completes semantic analysis with helper functions
    // Returns the resolved parse tree to be used for html translation
    fn semantic_analysis(&mut self) -> Vec<String> {
        self.push_scope(); // add a new scope to the active scope because the entire file has an active scope
        self.process_tokens(0, self.parse_tree.len()); // Process the tokens from the beginning to end of the parse tree
        self.pop_scope(); // have reached the end of the parse tree, so there are no further active scopes

        self.resolved_parse_tree.clone() // return resolved parse tree
    }

    // Process the tokens in the parse tree for a specified start and end position
    fn process_tokens(&mut self, start: usize, end: usize) {
        let mut i = start; // Start at the specified start position

        // Loops over the parse tree until the end position is reached
        while i < end {
            let token = self.parse_tree[i].clone(); // Current token is the token at the current index of the parse tree

            if token == "#IHAZ" || token == "#ihaz" { // If the token is ihaz, define a variable
                i = self.define_variable(i, end); // Call helper method for defining a variable
            }

            else if token == "#GIMMEH" || token == "#gimmeh" { // If the token is gimmeh
                if i + 1 < end
                    && (self.parse_tree[i + 1] == "VAR"
                    || self.parse_tree[i + 1] == "var")
                { // If the next token is var
                    i = self.resolve_variable(i, end); // Resolve the variable that's being referenced
                } else {
                    self.resolved_parse_tree.push(token); // Otherwise the token can just be added to the resolved parse tree
                    i += 1;
                }
            }

            else if token == "#MAEK" || token == "#maek" { // If the current token is maek
                if i + 1 < end {
                    let next_token = self.parse_tree[i + 1].clone(); // move to next token to determine route to take

                    // Paragraphs and lists can contain variable defintions and uses, so they need to be handled separately
                    if next_token == "PARAGRAF" || next_token == "paragraf" {
                        i = self.process_paragraph(i, end); // call paragraph helper method
                    }

                    else if next_token == "LIST" || next_token == "list" {
                        i = self.process_list(i, end); // call list helper method
                    }

                    else { // otherwise the token isn't involved in variable definition or use, so it can just be pushed
                        self.resolved_parse_tree.push(token);
                        i += 1;
                    }
                } else { // otherwise we have reached the end position and the token needs to be pushed to the resolved parse tree
                    self.resolved_parse_tree.push(token);
                    i += 1; // incrementing i will stop the loop when the while condition is evaluated next
                }
            }

            else { // otherwise the token isn't involved in variable use or definition so it can be pushed
                self.resolved_parse_tree.push(token);
                i += 1;
            }
        }
    }

    // Adds a new scope to the active scope stack
    fn push_scope(&mut self) {
        self.active_scope_stack.push(HashMap::new());
    }

    // Removes the most recent scope from the active scope stack
    fn pop_scope(&mut self) {
        self.active_scope_stack.pop();
    }

    // Function for defining a variable and pushing its name and value
    fn define_variable(&mut self, i: usize, end: usize) -> usize { // Returns where the semantic analyzer should continue analysis in the parse tree
        let mut name = String::new();
        let mut value = String::new();

        let mut j = i + 1;

        while j < end // While we haven't reached the end position and haven't reached itiz (which indicates the end of the name)
            && self.parse_tree[j] != "#ITIZ"
            && self.parse_tree[j] != "#itiz"
        {
            name.push_str(&self.parse_tree[j]); // add the token (character) to the variable name
            j += 1;
        }

        j += 1;

        while j < end // While we haven't reached the end position and haven't reached mkay (which indicates the end of the variable value)
            && self.parse_tree[j] != "#MKAY"
            && self.parse_tree[j] != "#mkay"
        {
            value.push_str(&self.parse_tree[j]); // add the token (character) to the variable value
            j += 1;
        }

        if let Some(scope) = self.active_scope_stack.last_mut() { // If there is an active scope, fetch the last active scope
            scope.insert(name, value); // Insert this variable name and value to the last active scope
        }

        if j < end { // if we haven't reached the end position, increment the position so semantic analysis doesn't continue at an index that's already been added to the parse tree
            j += 1;
        }

        j // return the position where the semantic analyzer should continue analysis
    }

    // Function for resolving a variable: getting its value from its name
    // Returns the position where the semantic analyzer should continue analysis
    fn resolve_variable(&mut self, i: usize, end: usize) -> usize {
        let mut name = String::new();
        let mut j = i + 2; // move position past gimmeh var

        while j < end // This method is only called if var has been found in the parse tree
        // While we haven't reached the end position and the current token isn't oic (indicating the end of the variable name)
            && self.parse_tree[j] != "#OIC"
            && self.parse_tree[j] != "#oic"
        {
            name.push_str(&self.parse_tree[j]); // add the current token (character) to the variable name
            j += 1;
        }

        if let Some(value) = self.find_variable(&name) { // If the variable name exists
            for c in value.chars() { // Construct the variable value
                self.resolved_parse_tree.push(c.to_string()); // Push the variable value to the parse tree
            }
        } else { // Otherwise the variable hasn't been defined
            eprintln!("Semantic error: Variable '{}' has not been defined.", name);
            std::process::exit(1);
        }

        if j < end { // if we haven't reached the end position, increment the position so semantic analysis doesn't continue at a position that has already been processed
            j += 1;
        }

        j
    }

    // Function for determining if a variable exists
    // Returns the value or none. None indicates the variable hasn't been defined
    fn find_variable(&self, name: &str) -> Option<String> {
        // Iterates through the active scopes, starting at the back (most recent)
        for scope in self.active_scope_stack.iter().rev() {
            if let Some(value) = scope.get(name) { // If the scope has the variable name
                return Some(value.clone()); // return the value associated with that variable name
            }
        }

        None // otherwise the variable hasn't been defined
    }

    // Function for handling paragraph scoping
    fn process_paragraph(&mut self, i: usize, end: usize) -> usize {
        // Position of the end of the paragraph
        let block_end = self.find_block_end(i + 2, end);

        // Adds maek paragraf to the resolved parse tree
        self.resolved_parse_tree.push(self.parse_tree[i].clone());
        self.resolved_parse_tree.push(self.parse_tree[i + 1].clone());

        // push paragraph scope to the parse tree
        self.push_scope();

        // perform semantic analysis on the tokens in the paragraph
        self.process_tokens(i + 2, block_end);

        // remove the paragraph's scope
        self.pop_scope();

        if block_end < end { // if the end of the paragraph is before the end position...
            self.resolved_parse_tree.push(self.parse_tree[block_end].clone()); // add the end of the block to the resolved parse tree
            return block_end + 1; // return the position after the block
        }

        block_end // otherwise return the end of the block as the position because we've reached the end of the paragraph
    }

    // Function for handling list scoping
    fn process_list(&mut self, i: usize, end: usize) -> usize {
        let block_end = self.find_block_end(i + 2, end); // position of the end of the list

        // push the maek list annotation
        self.resolved_parse_tree.push(self.parse_tree[i].clone());
        self.resolved_parse_tree.push(self.parse_tree[i + 1].clone());

        self.push_scope(); // push the scope of the list

        let mut j = i + 2; // move the position past the maek list annotations

        while j < block_end { // while we haven't reached the end of the list block
            if (self.parse_tree[j] == "#GIMMEH"
                || self.parse_tree[j] == "#gimmeh")
                && j + 1 < block_end
                && (self.parse_tree[j + 1] == "ITEM"
                || self.parse_tree[j + 1] == "item")
            { // if the token is a list item
                j = self.process_list_item(j, block_end); // process the list item because it can have its own scope
            } else { // 
                self.process_tokens(j, j + 1); // process the other tokens
                j += 1;
            }
        }

        self.pop_scope(); // remove the scope of the list

        if block_end < end { // if the end of the list isn't the end position
            self.resolved_parse_tree.push(self.parse_tree[block_end].clone()); // push the rest of the tokens to the resolved parse tree
            return block_end + 1; // return the position after the block's end
        }

        block_end // otherwise return the position of the block's end
    }

    // Function for handling list item scoping
    fn process_list_item(&mut self, i: usize, end: usize) -> usize {
        let item_end = self.find_item_end(i + 2, end);

        // push the gimmeh item tokens
        self.resolved_parse_tree.push(self.parse_tree[i].clone());
        self.resolved_parse_tree.push(self.parse_tree[i + 1].clone());

        self.push_scope(); // add the list item scope

        self.process_tokens(i + 2, item_end); // process the tokens until the end of the list item

        self.pop_scope(); // remove the list item's scope

        if item_end < end { // if the end of the list item is before the end position
            self.resolved_parse_tree.push(self.parse_tree[item_end].clone()); // push the end of the item to the resolved tree
            return item_end + 1; // return the position after the item's end
        }

        item_end // otherwise return the list item's end as the position to resume at
    }

    // Helper function for determining the end of a paragraph or list block
    fn find_block_end(&self, start: usize, end: usize) -> usize {
        let mut depth = 1;
        let mut i = start;

        while i < end {
            let token = &self.parse_tree[i];

            if token == "#IHAZ" || token == "#ihaz" { // continue to increase end for a variable definition
                i += 1;

                while i < end
                    && self.parse_tree[i] != "#MKAY"
                    && self.parse_tree[i] != "#mkay"
                {
                    i += 1;
                }

                i += 1;
            }

            else if token == "#MAEK" || token == "#maek" {
                if i + 1 < end {
                    let next_token = &self.parse_tree[i + 1];

                    if next_token == "PARAGRAF"
                        || next_token == "paragraf"
                        || next_token == "LIST"
                        || next_token == "list"
                    {
                        depth += 1; // increase the depth because another block has been added
                    }
                }

                i += 1; // increase end
            }

            else if token == "#MKAY" || token == "#mkay" {
                depth -= 1; // remove depth because a paragraph or list has been closed

                if depth == 0 {
                    return i; // return end
                }

                i += 1;
            }

            else {
                i += 1;
            }
        }

        end // otherwise, the end of the block is the end position
    }

    // Helper function for finding the end of a list item
    fn find_item_end(&self, start: usize, end: usize) -> usize {
        let mut i = start;

        while i < end {
            let token = &self.parse_tree[i];

            if token == "#GIMMEH" || token == "#gimmeh" {
                i += 1;

                while i < end
                    && self.parse_tree[i] != "#OIC"
                    && self.parse_tree[i] != "#oic"
                {
                    i += 1; // increase end while the end of the item hasn't been reached yet
                }

                i += 1;
            }

            else if token == "#OIC" || token == "#oic" { // once an item has been closed
                return i; // return the incremented end
            }

            else {
                i += 1;
            }
        }

        end // otherwise the end of the item is the end position
    }
}


pub struct HtmlTranslator;

// HTML translation
impl HtmlTranslator {
    pub fn translate(parse_tree: &[String]) -> String {
        let mut html = String::new(); // HTML is written to a String
        let mut closing_tags: Vec<&str> = Vec::new(); // Closing tags are kept in a String vector
        let mut i = 0; // start incrementor at 0

        while i < parse_tree.len() { // While the end of the resolved parse tree hasn't been reached
            let token = &parse_tree[i]; // The current token is the token at the incrementor index of the parse tree

            match token.as_str() { // Matches the current token to its HTML equivalent
                "#HAI" | "#hai" => { // If current token is hai, push <html> to the html String
                    html.push_str("<html>\n");
                }

                "#KBYE" | "#kbye" => { // If current token is kbye, push </html> to the html String
                    html.push_str("</html>\n");
                }

                "#OBTW" | "#obtw" => { // If current token is obtw, push <!-- to the html String
                    html.push_str("<!--");
                }

                "#TLDR" | "#tldr" => { // If current token is tldr, push --> to the html String
                    html.push_str("-->");
                }

                "#MAEK" | "#maek" => { // If current token is maek, there are several options for the html tag that needs to be pushed
                    if i + 1 < parse_tree.len() { // As long as the next token is less than the length of the resolved parse tree
                        match parse_tree[i + 1].as_str() { // match the token after maek to its html equivalent
                            "HEAD" | "head" => { // If current token is head, push <head> to the html String
                                html.push_str("<head>");
                                closing_tags.push("</head>"); // keep track of this head's closing tag. push </head> to the closing tags vector
                                i += 1;
                            }

                            "PARAGRAF" | "paragraf" => { // If current token is paragraf, push <p> to the html String
                                html.push_str("<p>");
                                closing_tags.push("</p>"); // keep track of this paragraf's closing tag
                                i += 1;
                            }

                            "LIST" | "list" => { // If current token is list, push <ul> to the html String
                                html.push_str("<ul>");
                                closing_tags.push("</ul>"); // keep track of this list's closing tag
                                i += 1;
                            }

                            _ => html.push_str(token), // otherwise the token is text that can be pushed onto the html String
                        }
                    }
                }

                "#GIMMEH" | "#gimmeh" => { // If current token is gimmeh, there are several options for the html tag that needs to be pushed
                    if i + 1 < parse_tree.len() {
                        match parse_tree[i + 1].as_str() { // match the token after gimmeh to its matching html tag
                            "TITLE" | "title" => { // If current token is title, push <title> to the html String
                                html.push_str("<title>");
                                closing_tags.push("</title>"); // keep track of this title's closing tag
                                i += 1;
                            }

                            "BOLD" | "bold" => { // If current token is bold, push <b> to the html String
                                html.push_str("<b>");
                                closing_tags.push("</b>"); // keep track of this bold's closing tag
                                i += 1;
                            }

                            "ITEM" | "item" => { // If current token is item, push <li> to the html String
                                html.push_str("<li>");
                                closing_tags.push("</li>"); // keep track of this item's closing tag
                                i += 1;
                            }

                            "LINX" | "linx" => { // If current token is linx
                                i += 2;

                                let mut address = String::new(); // Empty String to construct the address

                                while i < parse_tree.len() // While we haven't reached the end of the address
                                    && parse_tree[i] != "#OIC"
                                    && parse_tree[i] != "#oic"
                                {
                                    address.push_str(&parse_tree[i]); // continue adding characters from the resolved parse tree onto the address
                                    i += 1;
                                }

                                html.push_str("<a href=\""); // push beginning address tag onto html String
                                html.push_str(&address); // push address as a hyperlink onto html String
                                html.push_str("\">"); // close hyperlink
                                html.push_str(&address); // push address as the text for the link onto html String
                                html.push_str("</a>"); // push closing link tag onto html String
                            }

                            _ => html.push_str(token), // otherwise the token is text that can be pushed onto the html String
                        }
                    }
                }

                "#MKAY" | "#mkay" | "#OIC" | "#oic" => { // Have encountered a closing token
                    if let Some(closing_tag) = closing_tags.pop() { // remove the most recent closing tag that was pushed, because that's the one that's needed to correctly close the html element
                        html.push_str(closing_tag); // push the closing tag to the html String
                    }
                }

                "#NEWLINE" | "#newline" => { // If the token is newline, push <br> to the html String
                    html.push_str("<br>");
                }

                _ => {
                    html.push_str(token); // otherwise the token is text that can be pushed onto the html String
                }
            }

            i += 1; // move to the next token
        }

        html // return the HTML translation
    }
}


pub struct LolcodeCompiler {
    lexer: LolLexicalAnalyzer, // Lexer the compiler uses for lexical analysis
    current_tok: String, // current token in the compiler
    pub html_output: String, // html output from the html translator
}

impl LolcodeCompiler {
    pub fn new() -> Self { // Initiates a new compiler
        Self {
            lexer: LolLexicalAnalyzer::new(""),
            current_tok: String::new(),
            html_output: String::new(),
        }
    }

    // This function is required to get the next syntax (not text) token
    // This needed its own function so that whitespace between tokens is ignored, but whitespace in text needs to be retained
    fn next_syntax_token(&mut self) {
        let mut candidate = self.next_token();

        // Continue iterating while the candidate holds whitespace
        // That way, once the loop exits the candidate will hold a non-whitespace token
        while !candidate.is_empty() && candidate.chars().all(|c| c.is_whitespace()) {
            candidate = self.next_token();
        }
    }


    fn start(&mut self) {

        let mut candidate = self.lexer.get_next_token(); // Get the next token to start

        // Skip whitespace before the first annotation
        while !candidate.is_empty() && candidate.chars().all(|c| c.is_whitespace()) {
            candidate = self.lexer.get_next_token();
        } // By the time the loop exits, the first candidate is not whitespace

        if self.lexer.lookup(&candidate) { // Check if the candidate is a valid token
            self.current_tok = candidate; // If it is, assign it to the current token
        } else if !candidate.is_empty() { // Otherwise, report a lexical error
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        } else { // Otherwise, the file is empty
            eprintln!("Usage error: The provided file is empty.");
            std::process::exit(1);
        }
    }


}

// Implement the Compiler trait for the LolcodeCompiler
impl Compiler for LolcodeCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = LolLexicalAnalyzer::new(source); // Initialize the lexer
        self.start(); // Begin compilation
    }

    // This function gets the next token from the lexer and determines if there was a lexical error
    fn next_token(&mut self) -> String {
        let candidate = self.lexer.get_next_token(); // Get the next token
        if self.lexer.lookup(&candidate) { // If the candidate is valid
            self.current_tok = candidate.clone(); // assign the candidate to the current token
            candidate
        } else if candidate.is_empty() {
            self.current_tok.clear();
            String::new()
        } else { // otherwise return a lexical error
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    // Parse function initializes the syntax and semantic analyzers and html translator and calls their methods
    fn parse(&mut self) {
       let parse_tree = {
            let mut syntax_analyzer = LolcodeSyntaxAnalyzer::new(self); // Initialize the syntax analyzer
            syntax_analyzer.parse_lolcode(); // complete syntax analysis
            syntax_analyzer.parse_tree // return the parse tree from the syntax analyzer
        };

        if !self.current_tok.is_empty() {
            eprintln!("Syntax error: Additional tokens found after the kbye annotation.");
            std::process::exit(1);
        }

        let mut semantic_analyzer = LolcodeSemanticAnalyzer::new(parse_tree); // Initialize the semantic analyzer
        let resolved_parse_tree = semantic_analyzer.semantic_analysis(); // Complete semantic analysis and fetch the resolved parse tree

        self.html_output = HtmlTranslator::translate(&resolved_parse_tree); // complete html translation
    }

    fn current_token(&self) -> String { self.current_tok.clone() } // returns the current token
    fn set_current_token(&mut self, tok: String) { self.current_tok = tok; } // function for setting the current token
}

    // Function for opening the html file in the browser
    fn open_in_browser(path: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")] // If the operating system is Windows
    {
        // Method for opening file in browser in Windows
        Command::new("cmd")
            .args(["/C", "start", "", path])
            .status()?;
    }

    // If the operating system is macos
    #[cfg(target_os = "macos")]
    {
        // Method for opening file in browser in macos
        Command::new("open")
            .arg(path)
            .status()
    }

    // If the operating system is linux
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd"))]
    {
        // Method for opening browser in linux
        Command::new("xdg-open")
            .arg(path)
            .status()
    }

    Ok(())
}

fn main() {
// Usage in VS Code terminal:
    //   cargo run -- lolspeak.lol
    // where lolspeak.lol is in the project root (not src/) of the project.

    let args: Vec<String> = env::args().collect(); // Gets the arguments from the command line/terminal
    if args.len() < 2 {
        eprintln!("Usage error: {} <input_file>", args[0]);
        std::process::exit(1);
    } else if args.len() > 2 {
        eprintln!("Usage error: Too many arguments");
        std::process::exit(1);
    }

    // Prevents compiler from reading any files without a .lol extension
    let filename = &args[1];
    let mut extension = String::new();
    for c in filename.chars().rev() { // Starts at the end of the file name and works backwards until the .
        if c == '.' {
            break;
        }
        extension.push(c);    
    }

    if extension != "lol" {
        eprintln!("Usage error. Extension must be .lol Found extension .{}", extension);
        std::process::exit(1);
    }

    let file = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolcodeCompiler::new(); // Initialize compiler
    compiler.compile(&file); // Complete compilation with the input file
    compiler.parse(); // Complete parsing


    let output_path = Path::new(filename).with_extension("html"); // Open the html file (uses the file name but switches the extension for html)

    fs::write(&output_path, &compiler.html_output).unwrap_or_else(|err| {
        eprintln!("Error writing HTML file: {}", err);
        std::process::exit(1);
    });


     if let Err(e) = open_in_browser(&output_path.to_string_lossy()) {
        eprintln!("Failed to open browser: {}", e);
    }

}
