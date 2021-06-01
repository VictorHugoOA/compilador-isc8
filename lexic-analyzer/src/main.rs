// Este es el modulo completo del compilador con todas las definiciones de los analizadores
mod compiler{
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    //Es un enumerador para los tokens del lenguaje para el que vamos a crear el compilador
    pub enum TokenType{
        // Token para definir bloques de comentarios
        TK_COMMENT_LINE = 1, TK_COMMENT_BLOCK,
        // para leer y escribir variables
        TK_READ, TK_WRITE,
        // Tokens multicaracter para numeros o identificadores
        TK_ID, TK_NUM, TK_INT, TK_FLOAT, TK_BOOL,
        //Símbolos especiales 
        TK_LPAREN, TK_RPAREN, TK_SEMICOLON, TK_COMMA, TK_ASSIGN, TK_PLUS, TK_MINUS, TK_TIMES,
        TK_OVER, TK_EXP, TK_LT, TK_LTE, TK_GT, TK_GTE, TK_EQ, TK_DIF, TK_RKEY, TK_LKEY,
        //Tokens para casos especiales
        TK_EOF, TK_ERROR,
        //Tokens para flujo de operaciones
        TK_PROG, TK_IF, TK_ELSE, TK_FI, TK_DO, TK_UNTIL, TK_WHILE, TK_THEN,
        //Tokes para operaciones lógicas
        TK_NOT, TK_AND, TK_OR,
        //Expresiones literales booleanas
        TK_TRUE, TK_FALSE,
        NoToken
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum StatementType{
        Sequence = 1, Program, Literal,
        Binary, Arithmetic, Relational,
        Variable, Read, NoType,
        Assignment, Begin, End,
        If, Repeat, While, Comment,
        Write, Not, LiteralBoolExp,
        BooleanExp, VariableSeq, ListVariableDec
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum TinyType{
        Integer = 1, Float, Boolean, NoType
    }
    
    #[derive(Debug, Clone)]
    pub struct Token{
        pub token: TokenType,
        pub lexema: String,
        pub line: u32
    }

    #[derive(Clone)]
    pub struct TreeNode{
        token: Token,
        is_expression: bool,
        nodes: Vec<TreeNode>,
        statement_type: StatementType,
        val_type: TinyType,
        is_lvalue: bool,
    }

    pub fn null_tree() ->  TreeNode{
        return TreeNode {token: Token{token: TokenType::NoToken, lexema: String::from(""), line: 0}, is_expression: false, nodes: vec![], statement_type: StatementType::NoType, val_type: TinyType::NoType, is_lvalue: false};
    }

    pub fn new_literal(token: &Token) -> TreeNode{
        let result = TreeNode {token: token.copy_token(), is_expression: true, is_lvalue: false, nodes: vec![], statement_type: StatementType::Literal, val_type: TinyType::NoType};
        return result;
    }
    pub fn new_var(token: &Token) -> TreeNode{
        let result = TreeNode {token: token.copy_token(), is_expression: true, is_lvalue: false, nodes: vec![], statement_type: StatementType::Variable, val_type: TinyType::NoType};
        return result;
    }
    pub fn new_arithmetic(token: &Token, left_value: &TreeNode, right_value: &TreeNode) -> TreeNode{
        let result = TreeNode{token: token.copy_token(), is_expression: true, nodes: vec![left_value.copy(), right_value.copy()], statement_type: StatementType::NoType, val_type: TinyType::NoType, is_lvalue: false};
        return result;
    }

    pub fn new_relational(token: &Token, left_value: &TreeNode, right_value: &TreeNode) -> TreeNode{
        let result = TreeNode{token: token.copy_token(), is_expression: true, nodes: vec![left_value.copy(), right_value.copy()], statement_type: StatementType::NoType, val_type: TinyType::NoType, is_lvalue: false};
        return result;
    }

    pub fn new_assignment(token: &Token, _var: &TreeNode, rvalue: &TreeNode) -> TreeNode{
        let result = TreeNode{token: token.copy_token(), is_expression: false, nodes: vec![_var.copy(), rvalue.copy()], statement_type: StatementType::Assignment, val_type: TinyType::NoType, is_lvalue: false};
        return result;
    }

    pub fn new_unary(token: &Token, b_factor: &TreeNode) -> TreeNode{
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![b_factor.copy()],
            statement_type: StatementType::Not,
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_if(token: &Token, condition: &TreeNode, selection: &TreeNode, otherwise: &TreeNode) -> TreeNode{
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![condition.copy(), selection.copy(), otherwise.copy()],
            statement_type: StatementType::If, 
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_repeat(token: &Token, condition: &TreeNode, selection: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![condition.copy(), selection.copy()],
            statement_type: StatementType::Repeat,
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_while(token: &Token, condition: &TreeNode, selection: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![condition.copy(), selection.copy()],
            statement_type: StatementType::While,
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_program(token: &Token, variables: &TreeNode,  program: &TreeNode)-> TreeNode{
        let result = TreeNode{token: token.copy_token(),
        is_expression: false,
        is_lvalue: false,
        nodes: vec![variables.copy(), program.copy()],
        statement_type: StatementType::Program,
        val_type: TinyType::NoType};
        return result;
    }

    pub fn new_sequence(stmt: &TreeNode)-> TreeNode{
        let result = TreeNode{
            token: Token{token: TokenType::NoToken, lexema: String::from(""), line: 0},
            is_expression: false,
            is_lvalue: false,
            nodes: vec![stmt.copy()],
            statement_type: StatementType::Sequence,
            val_type: TinyType::NoType};
        return result;
    }

    pub fn new_read(token: &Token, variable: &TreeNode) -> TreeNode{
        let result = TreeNode{
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variable.copy()],
            statement_type: StatementType::Read,
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_write(token: &Token, exp: &TreeNode) -> TreeNode{
        let result = TreeNode{
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![exp.copy()],
            statement_type: StatementType::Write,
            val_type: TinyType::NoType
        };
        return result;
    }

    pub fn new_literal_boolean(token: &Token) -> TreeNode{
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![],
            statement_type: StatementType::LiteralBoolExp,
            val_type: TinyType::Boolean
        };
        return result;
    }

    pub fn new_boolean_exp(token: &Token, left: &TreeNode, right: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![left.copy(), right.copy()],
            statement_type: StatementType::BooleanExp,
            val_type: TinyType::Boolean
        };
        return result;
    }

    pub fn new_sequence_var(variables: &TreeNode) -> TreeNode {
        let result = TreeNode{
            token: Token{token: TokenType::NoToken, lexema: String::from(""), line: 0},
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variables.copy()],
            statement_type: StatementType::VariableSeq,
            val_type: TinyType::NoType
        };
        return result;

    }
    pub fn new_list_dec(token: &Token, variables: &TreeNode, val_type: TinyType) -> TreeNode{
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variables.copy()],
            statement_type: StatementType::ListVariableDec,
            val_type: val_type
        };
        return result;
    }

    impl TreeNode{
        pub fn copy(&self) -> TreeNode{
            return TreeNode{token: self.token.copy_token(), is_expression: self.is_expression, nodes: self.nodes.to_vec(), statement_type: self.statement_type, val_type: self.val_type, is_lvalue: self.is_lvalue}
        }
        pub fn type_name(&self) -> &str {
            match &self.val_type{
                TinyType::Integer => {
                    return "int";
                }
                TinyType::Float => {
                    return "float";
                }
                TinyType::Boolean => {
                    return "bool";
                }
                 _ => {
                    return "no_type";
                }
            }
        }
        pub fn is_expression(&self) -> bool {
            self.is_expression
        }
        pub fn get_type(&self) -> TinyType{
            return self.val_type;
        }
        pub fn set_type(&mut self, new_type: &TinyType) {
            self.val_type = *new_type;
        }
        pub fn is_lvalue(&self) -> bool{
            self.is_lvalue
        }
        pub fn set_lvalue(&mut self, x: bool){
            self.is_lvalue = x;
        }
        pub fn append(&mut self, next: &TreeNode){
            self.nodes.push(next.copy());
        }
        pub fn print_grammar_tree(&self, number_idents: u32){

            if self.statement_type != StatementType::Sequence && self.statement_type != StatementType::VariableSeq && self.token.token != TokenType::NoToken{

                for n in 1..number_idents{
                    print!(" ");
                }

                println!("{:?}", self.token.token);

                for n in 1..=(number_idents + 1){
                    print!(" ");
                }
                println!("{}", self.token.lexema);
            }

            for node in &self.nodes {
                node.print_grammar_tree(number_idents + 1);
            }

        }

        pub fn print_syntax_tree(&self, number_idents: u32){
            if self.statement_type != StatementType::Sequence && self.statement_type != StatementType::VariableSeq && self.token.token != TokenType::NoToken{
                for n in 1..number_idents {
                    print!(" ");
                }
                println!("{}", self.token.lexema);
            }

            for node in &self.nodes {
                node.print_syntax_tree(number_idents + 1);
            }

        }

    }
   
    pub fn null_token() -> Token {
        let result = Token {
            token: TokenType::NoToken,
            lexema: String::from(""),
            line: 0
        };
        return result;
    }
    impl Token{
        pub fn copy_token(&self) -> Token{
            let new_token: Token = Token { token: self.token, lexema: self.lexema.clone(), line: self.line };
            return new_token;
        }
    }

    pub mod scanner{
        use super::TokenType;
        use super::Token;
        use std::fs;
        use std::io::BufReader;
        use std::io::prelude::*;
        
        //Son los estados de la máquina para el autómata
        #[derive(Debug, PartialEq, Eq)]
        enum StateType{
            Start = 1, Id, Num, CommentLine, CommentBlock, Error, IsDone
        }
        pub struct Scanner{
            file_name: String,
            file_buffer: BufReader<fs::File>,
            current_pos: usize,
            current_line: u32,
            _trace: bool,
            _echo_source: bool,
            initialized: bool,
            line_buff: Vec<char>,
            in_eof: bool
        }
        impl Scanner{
            pub fn set_echo_source(&mut self, x: bool){
                self._echo_source = x;
            }
            pub fn echo_source(&self) -> bool{
                self._echo_source
            }
            pub fn set_trace(&mut self, x: bool){
                self._trace = x;
            }
            pub fn trace(&self) -> bool{
                self._trace
            }

            pub fn get_line(&self) -> u32{
                self.current_line
            }

            pub fn get_pos(&self) -> usize{
                self.current_pos
            }

            fn get_next_char(&mut self) -> char{
                let mut buf = String::new();
                let zero_bytes: usize = 0;
                if self.current_pos >= self.line_buff.len() {
                    let num_bytes = self.file_buffer.read_line(&mut buf).expect("Couldn't read file'");
                    if num_bytes > zero_bytes {
                        self.line_buff = buf.chars().collect();
                        self.current_pos = 0;
                        self.current_line += 1;
                    }
                    else{
                        self.in_eof = true;
                        return '\0';
                    }
                }
                let result = self.line_buff[self.current_pos];
                self.current_pos += 1;
                return result;
            }
            fn unget_next_char(&mut self){
                if !self.in_eof {
                    self.current_pos -= 1;
                }
            }
            
            pub fn new(file_name: &str, token_trace: bool) -> Scanner{

                let file = fs::File::open(file_name).expect("Please, to use the program use lexic-analyzer <filename>");
                let buf_reader = BufReader::new(file);

                let result = Scanner {
                    file_name: String::from(file_name),
                    _trace: token_trace,
                    current_line: 0,
                    current_pos: 0,
                    _echo_source: false,
                    file_buffer: buf_reader,
                    initialized: false,
                    line_buff: vec![],
                    in_eof: false
                };
                return result;
            }

            fn is_delimiter (&self, c: char) -> bool{
                c == ' ' || c == '\t' || c == '\n'
            }

            fn is_reserved_word(&mut self, lexema: &str)-> TokenType{
               match lexema {
                   "program" => {
                       return TokenType::TK_PROG
                   }
                   "if" => {
                       return TokenType::TK_IF
                   }
                   "fi" => {
                       return TokenType::TK_FI
                   }
                   "else" => {
                       return TokenType::TK_ELSE
                   }
                   "do" => {
                       return TokenType::TK_DO
                   }
                   "until" => {
                       return TokenType::TK_UNTIL
                   }
                   "while" => {
                       return TokenType::TK_WHILE
                   }
                   "read" => {
                       return TokenType::TK_READ
                   }
                   "write" => {
                       return TokenType::TK_WRITE
                   }
                   "float" => {
                       return TokenType::TK_FLOAT
                   }
                   "int" => {
                       return TokenType::TK_INT
                   }
                   "bool" => {
                       return TokenType::TK_BOOL
                   }
                   "not" => {
                       return TokenType::TK_NOT
                   }
                   "and" => {
                       return TokenType::TK_AND
                   }
                   "or" => {
                       return TokenType::TK_OR
                   }
                   "then" => {
                       return TokenType::TK_THEN
                   }
                   "true" => {
                        return TokenType::TK_TRUE;
                   }
                   "false" => {
                        return TokenType::TK_FALSE;
                   }
                   _ => {
                       return TokenType::TK_ID
                   }
               }
            }

            pub fn get_token(&mut self) -> Token{

                let mut lexema: String = String::new();
                let mut token: TokenType = TokenType::TK_ERROR;
                let mut start_line: u32 = 0;

                let mut state: StateType = StateType::Start;
                while state != StateType::IsDone {
                    let mut c: char = self.get_next_char();
                    let mut save: bool = true;
                    match state {
                        StateType::Start => {
                            start_line = self.get_line();
                            if self.in_eof {
                                state = StateType::IsDone;
                                token = TokenType::TK_EOF;
                            }
                            else if c.is_digit(10) {
                                state = StateType::Num;
                            }
                            else if c.is_alphabetic() || c == '_' {
                                state = StateType::Id;
                            }
                            else if self.is_delimiter(c) {
                                save = false;
                            }
                            else if c == '(' {
                                state = StateType::IsDone;
                                token = TokenType::TK_LPAREN;
                            }
                            else if c == ')' {
                                state = StateType::IsDone;
                                token = TokenType::TK_RPAREN;
                            }
                            else if c == '{' {
                                state = StateType::IsDone;
                                token = TokenType::TK_LKEY;
                            }
                            else if c == '}' {
                                state = StateType::IsDone;
                                token = TokenType::TK_RKEY;
                            }
                            else if c == ';' {
                                state = StateType::IsDone;
                                token = TokenType::TK_SEMICOLON;
                            }
                            else if c == ',' {
                                token = TokenType::TK_COMMA;
                                state = StateType::IsDone;
                            }
                            else if c == '+' {
                                token = TokenType::TK_PLUS;
                                state = StateType::IsDone;
                            }
                            else if c == '-' {
                                token = TokenType::TK_MINUS;
                                state = StateType::IsDone;
                            }
                            else if c == '*' {
                                token = TokenType::TK_TIMES;
                                state = StateType::IsDone;
                            }
                            else if c == '/' {
                                token = TokenType::TK_OVER;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '/' {
                                    state = StateType::CommentLine;
                                    token = TokenType::TK_COMMENT_LINE;
                                    save = false
                                }
                                else if tempC == '*'{
                                    state = StateType::CommentBlock;
                                    token = TokenType::TK_COMMENT_BLOCK;
                                    save = false;
                                }
                                else{
                                    self.unget_next_char();
                                }
                            }
                            else if c == '^' {
                                token = TokenType::TK_EXP;
                                state = StateType::IsDone;
                            }
                            else if c == '=' {
                                token = TokenType::TK_ASSIGN;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '='{
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_EQ;
                                }
                                else {
                                    self.unget_next_char();
                                }
                            }
                            else if c == '<' {
                                token = TokenType::TK_LT;
                                state = StateType::IsDone;

                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_LTE;
                                }
                                else {
                                    self.unget_next_char();
                                }

                            }
                            else if c == '>' {
                                token = TokenType::TK_GT;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_GTE;
                                }
                                else {
                                    self.unget_next_char();
                                }
                            }
                            else if c == '!' {
                                token = TokenType::TK_ERROR;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_DIF;
                                }
                                else {
                                    self.unget_next_char();
                                }
                            }
                        }
                        StateType::Num => {
                            if !c.is_digit(10) {
                                self.unget_next_char();
                                state = StateType::IsDone;
                                save = false;
                                token = TokenType::TK_NUM;
                            }
                        }
                        StateType::Id => {
                            if !(c.is_alphanumeric() || c == '_') {
                                self.unget_next_char();
                                state = StateType::IsDone;
                                save = false;
                                token = TokenType::TK_ID;
                            }
                        }

                        StateType::CommentLine => {
                            save = false;
                            if self.current_pos >= self.line_buff.len() {
                                token = TokenType::TK_COMMENT_LINE;
                                state = StateType::Start;
                            }
                        }
                        StateType::CommentBlock => {
                            save = false;
                            let tempC: char = self.get_next_char();
                            if (c == '*') && (tempC == '/') {
                                token = TokenType::TK_COMMENT_BLOCK;
                                state = StateType::Start;
                            }
                            else{
                                self.unget_next_char();
                            }
                        }

                        //Should never happen
                        StateType::IsDone => {
                        }
                        _ => {
                        }
                    }

                    if save {
                        lexema.push(c);
                    }
                    if state == StateType::IsDone {
                        if token == TokenType::TK_ID {
                            token = self.is_reserved_word(&lexema);
                        }
                    }
                }
                let result_token = Token {token: token, lexema: String::from(lexema), line: start_line};
                if self._trace {
                    println!("({:?},{}, {})", result_token.token, result_token.lexema, result_token.line);
                }
                return result_token;
            }
        }
    }

    pub mod parser{
        use super::Token;
        use super::TokenType;
        use super::TinyType;
        use super::TreeNode;
        use super::StatementType;
        use super::{null_tree, new_var, new_literal, new_arithmetic, new_relational, new_assignment,
                    new_program, new_sequence, new_if, new_repeat, new_while, null_token, new_read,
                    new_write, new_unary, new_literal_boolean, new_boolean_exp, new_list_dec, new_sequence_var};
        use super::scanner::Scanner;

        pub struct TokenParser{
            current_token: Token,
            program: TreeNode,
            scanner: Scanner,
            _error: bool

        }
        pub fn new(scanner: Scanner) -> TokenParser{
            return TokenParser {current_token: null_token(), program: null_tree(), scanner: scanner, _error: false };
        }
        impl TokenParser{

            fn error_msg(&mut self, label: &str, token: &Token, msg: &str){
                eprintln!("error: line - {} error: {}, token: {:?}, msg: {}", token.line, label, token.token, msg);
                self._error = true;
            }

            fn type_error(&mut self, token: &Token, msg: &str){
                self.error_msg("type", token, msg);
            }

            fn syntax_error(&mut self, token: &Token, msg: &str){
                self.error_msg("syntax", token, msg);
            }

            pub fn parse(&mut self)-> &TreeNode{
                self.current_token = self.get_next_token();
                let program: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_PROG);
                self.match_token(&TokenType::TK_LKEY);
                self.program = new_program(&program, &self.seq_declaration(), &self.seq_stmt());
                self.match_token(&TokenType::TK_RKEY);
                if self.current_token.token != TokenType::TK_EOF {
                    self.syntax_error(&self.current_token.copy_token(), "Code ends before file");
                }

                return &self.program;
            }

            fn match_token(&mut self, token_match: &TokenType){
                if &self.current_token.token == token_match {
                    let new_token = self.get_next_token();
                    self.current_token = new_token;
                }
                else{
                    self.syntax_error(&self.current_token.copy_token(), "unexpected token");
                }
            }

            fn get_next_token(&mut self) -> Token{
                return self.scanner.get_token();
            }

            pub fn print_grammar_parser(&self){
                println!("ARBOL GRAMATICAL");
                self.program.print_grammar_tree(1);
            }

            pub fn print_syntax_parser(&self){
                println!("ARBOL SINTACTICO");
                self.program.print_syntax_tree(0);
            }


            fn seq_stmt(&mut self) -> TreeNode{

                if self.current_token.token == TokenType::TK_RKEY {
                    return new_sequence(&null_tree());
                }

                let mut retval: TreeNode = new_sequence(&self.stmt());
                while (self.current_token.token != TokenType::TK_EOF) && (self.current_token.token != TokenType::TK_RKEY) &&
                    (self.current_token.token != TokenType::TK_ELSE) && (self.current_token.token != TokenType::TK_UNTIL) {
                        let q: TreeNode = self.stmt();
                        if (q.token.token != TokenType::NoToken) && (q.statement_type != StatementType::NoType) {
                            if q.statement_type != StatementType::Comment {
                                retval.append(&q);
                            }
                        }
                        else {
                            break;
                        }
                    }
                return retval;
            }

            fn seq_declaration(&mut self) -> TreeNode{

                if self.current_token.token == TokenType::TK_RKEY {
                    return new_sequence_var(&null_tree());
                }

                let mut retval: TreeNode = new_sequence_var(&self.declaration());
                while self.current_token.token == TokenType::TK_INT ||
                self.current_token.token == TokenType::TK_FLOAT ||
                self.current_token.token == TokenType::TK_BOOL {
                    let q: TreeNode = self.declaration();
                    retval.append(&q);
                }
                return retval;
            }

            fn declaration(&mut self) -> TreeNode {
                let mut statement: TreeNode = null_tree();
                match self.current_token.token {
                    TokenType::TK_INT => {
                        let _int: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_INT);
                        statement = self.list_declaration(&_int);
                        self.match_token(&TokenType::TK_SEMICOLON);

                    }
                    TokenType::TK_FLOAT => {
                        let _float: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_FLOAT);
                        statement = self.list_declaration(&_float);
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    TokenType::TK_BOOL=> {
                        let _bool: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_BOOL);
                        statement = self.list_declaration(&_bool);
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    _ => {
                        self.current_token = self.get_next_token();
                        self.syntax_error(&self.current_token.copy_token(), "token in initial list variable declaration");
                    }
                    
                }
                return statement;
            }

            fn list_declaration(&mut self, token: &Token) -> TreeNode{
                let _id: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_ID);
                let mut first_var: TreeNode = new_var(&_id);
                let mut val_type: TinyType = TinyType::NoType;
                match token.token {
                    TokenType::TK_INT => {val_type = TinyType::Integer}
                    TokenType::TK_FLOAT => {val_type = TinyType::Float}
                    TokenType::TK_BOOL => {val_type = TinyType::Boolean}
                    _ => {}
                }
                first_var.set_type(&val_type);
                let mut retval: TreeNode = new_list_dec(token, &first_var, val_type);
                while self.current_token.token == TokenType::TK_COMMA {
                    self.match_token(&TokenType::TK_COMMA);
                    let _id: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_ID);
                    let mut _var: TreeNode = new_var(&_id);
                    _var.set_type(&val_type);
                    retval.append(&_var);
                }
                return retval;
            }

            fn stmt(&mut self) -> TreeNode{
                let mut statement: TreeNode = null_tree();
                match self.current_token.token {
                    TokenType::TK_ID => {
                        statement = self.assign_stmt();
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    TokenType::TK_IF => {
                        statement = self.if_stmt();
                    }
                    TokenType::TK_DO => {
                        statement = self.repeat_stmt();
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    TokenType::TK_WHILE => {
                        statement = self.while_stmt();
                    }
                    TokenType::TK_READ => {
                        statement = self.read_stmt();
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    TokenType::TK_WRITE => {
                        statement = self.write_stmt();
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    TokenType::TK_LKEY => {
                        statement = self.seq_stmt();
                        self.match_token(&TokenType::TK_RKEY);
                    }
                    _ => {
                        self.current_token = self.get_next_token();
                        self.syntax_error(&self.current_token.copy_token(), "Code ends before file");
                    }
                }
                return statement;
            }

            fn write_stmt(&mut self) -> TreeNode {
                let _write: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_WRITE);
                return new_write(&_write, &self.b_expression());
            }

            fn read_stmt(&mut self) -> TreeNode {

                let mut variable: TreeNode = null_tree();
                let _read: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_READ);
                let _var: Token = self.current_token.copy_token();
                if _var.token == TokenType::TK_ID {
                    variable = new_var(&_var);
                }
                self.match_token(&TokenType::TK_ID);
                return new_read(&_read, &variable);
            }

            fn while_stmt(&mut self) -> TreeNode {
                let while_token: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_WHILE);
                self.match_token(&TokenType::TK_LPAREN);
                let condition: TreeNode = self.b_expression();
                self.match_token(&TokenType::TK_RPAREN);
                self.match_token(&TokenType::TK_LKEY);
                let selection: TreeNode = self.seq_stmt();
                self.match_token(&TokenType::TK_RKEY);
                
                return new_while(&while_token, &condition, &selection);
            }

            fn if_stmt(&mut self) -> TreeNode{

                let if_token: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_IF);

                self.match_token(&TokenType::TK_LPAREN);
                let condition: TreeNode = self.b_expression();
                self.match_token(&TokenType::TK_RPAREN);

                self.match_token(&TokenType::TK_THEN);

                self.match_token(&TokenType::TK_LKEY);
                let selection: TreeNode = self.seq_stmt();
                self.match_token(&TokenType::TK_RKEY);
                let mut otherwise: TreeNode = null_tree();

                if self.current_token.token == TokenType::TK_ELSE {
                    self.match_token(&TokenType::TK_ELSE);
                    self.match_token(&TokenType::TK_LKEY);
                    otherwise = self.seq_stmt();
                    self.match_token(&TokenType::TK_RKEY);
                }
                self.match_token(&TokenType::TK_FI);
                return new_if(&if_token, &condition, &selection, &otherwise);
            }

            fn repeat_stmt(&mut self) -> TreeNode {
                let do_token: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_DO);
                self.match_token(&TokenType::TK_LKEY);
                let selection: TreeNode = self.seq_stmt();
                self.match_token(&TokenType::TK_RKEY);
                self.match_token(&TokenType::TK_UNTIL);
                self.match_token(&TokenType::TK_LPAREN);
                let condition: TreeNode = self.b_expression();
                self.match_token(&TokenType::TK_RPAREN);
                
                return new_repeat(&do_token, &condition, &selection);

            }

            fn assign_stmt(&mut self) -> TreeNode{
                let mut variable: TreeNode = null_tree();
                let _var: Token = self.current_token.copy_token();
                if _var.token == TokenType::TK_ID {
                    variable = new_var(&_var);
                }
                variable.set_lvalue(true);
                self.match_token(&TokenType::TK_ID);
                let _assign: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_ASSIGN);
                let rvalue: TreeNode = self.b_expression();
                return new_assignment(&_assign, &variable, &rvalue);

            }
            fn b_expression(&mut self) -> TreeNode{
                let result: TreeNode = self.b_term();
                if self.current_token.token == TokenType::TK_OR {
                    let _or: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_OR);
                    return new_boolean_exp(&_or, &result, &self.b_term());
                }
                else{
                    return result;
                }
            }
            fn b_term (&mut self) -> TreeNode {
                let result: TreeNode = self.not_factor();
                if self.current_token.token == TokenType::TK_AND {
                    let _and: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_AND);
                    return new_boolean_exp(&_and, &result, &self.not_factor());
                }
                else{
                    return result;
                }
            }

            fn not_factor(&mut self) -> TreeNode{
                if self.current_token.token == TokenType::TK_NOT {
                    let _not: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_NOT);
                    return new_unary(&_not, &self.b_factor());
                }
                return self.b_factor();
            }

            fn b_factor(&mut self) -> TreeNode {
                if self.current_token.token == TokenType::TK_TRUE ||
                   self.current_token.token == TokenType::TK_FALSE{
                    let _bool: Token = self.current_token.copy_token();
                    self.match_token(&_bool.token);
                    return new_literal_boolean(&_bool);

                }
                return self.expression();
            }

            fn expression(&mut self) -> TreeNode{
                let result: TreeNode = self.simple_exp();
                if self.current_token.token == TokenType::TK_LT || self.current_token.token == TokenType::TK_LTE ||
                    self.current_token.token == TokenType::TK_GT || self.current_token.token == TokenType::TK_GTE || 
                    self.current_token.token == TokenType::TK_DIF || self.current_token.token == TokenType::TK_EQ {
                        let _op = self.current_token.copy_token();
                        self.match_token(&_op.token);
                        return new_relational(&_op, &result, &self.simple_exp());
                    }
                else {
                    return result;
                }
            }
            fn simple_exp(&mut self) -> TreeNode{
                let result: TreeNode = self.term();
                if self.current_token.token == TokenType::TK_MINUS || self.current_token.token == TokenType::TK_PLUS {
                    let _op = self.current_token.copy_token();
                    self.match_token(&_op.token);
                    return new_arithmetic(&_op, &result, &self.term());
                }
                else{
                    return result;
                }
            }
            fn term(&mut self) -> TreeNode{
                let result: TreeNode = self.factor();
                if self.current_token.token == TokenType::TK_TIMES || self.current_token.token == TokenType::TK_OVER {
                    let _op = self.current_token.copy_token();
                    self.match_token(&_op.token);
                    return new_arithmetic(&_op, &result, &self.factor());
                } 
                else{
                    return result;
                }
            }
            fn factor(&mut self) -> TreeNode{
                match self.current_token.token{
                    TokenType::TK_NUM => {
                        let _num: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_NUM);
                        return new_literal(&_num);
                    }
                    TokenType::TK_ID => {
                        let _id: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_ID);
                        return new_var(&_id);
                    }
                    TokenType::TK_LPAREN => {
                        self.match_token(&TokenType::TK_LPAREN);
                        let exp: TreeNode = self.expression();
                        self.match_token(&TokenType::TK_RPAREN);
                        return exp;
                    }
                    _ => {
                        self.syntax_error(&self.current_token.copy_token(), "Code ends before file");
                        return null_tree();
                    }
                }
            }
        }
    }
}

// use crate::compiler::Scanner;
use std::fs;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;

// use crate::compiler::Token;
use crate::compiler::TokenType;
use crate::compiler::parser;
use crate::compiler::scanner;

const ONLY_TOKENS:&str = "--only-tokens";
const ONLY_GRAMAR:&str = "--only-gramar";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut Scanner  = scanner::Scanner::new(&args[args.len()-1], true);

    let mut parser: parser::TokenParser = parser::new(Scanner);
    parser.parse();

    parser.print_grammar_parser();
    parser.print_syntax_parser();

}
