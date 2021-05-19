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
        NoToken
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum StatementType{
        Sequence = 1, Program, Literal, Binary, Arithmetic, Relational, Variable, Read, NoType, Assignment, Begin, End, If, Repeat, While
    }

    #[derive(Copy, Clone)]
    pub enum TinyType{
        Integer, Float, Boolean, NoType
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

    pub fn new_program(token: &Token, program: &TreeNode)-> TreeNode{
        let result = TreeNode{token: token.copy_token(),
        is_expression: false,
        is_lvalue: false,
        nodes: vec![program.copy()],
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
        pub fn append(&mut self, next: &TreeNode){
            self.nodes.push(next.copy());
        }
        pub fn print_tree(&self){
            if self.statement_type != StatementType::Sequence {
                println!("({:?}, {})", self.token.token, self.token.lexema);
            }
            for node in &self.nodes {
                node.print_tree();
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
                if(self._trace){
                    println!("({:?},{}, {})", result_token.token, result_token.lexema, result_token.line);
                }
                return result_token;
            }
        }
    }

    pub mod parser{
        use super::Token;
        use super::TokenType;
        use super::TreeNode;
        use super::StatementType;
        use super::{null_tree, new_var, new_literal, new_arithmetic, new_relational, new_assignment, new_program, new_sequence, new_if, new_repeat, new_while, null_token};
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
                eprintln!("error: line - {} error: {}, msg: {}", token.line, label, msg);
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
                self.program = new_program(&program, &self.seq_stmt());
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

            pub fn print_parser(&self){
                self.program.print_tree();
            }

            fn seq_stmt(&mut self) -> TreeNode{
                let mut retval: TreeNode = new_sequence(&self.stmt());
                while (self.current_token.token != TokenType::TK_EOF) && (self.current_token.token != TokenType::TK_RKEY) &&
                    (self.current_token.token != TokenType::TK_ELSE) && (self.current_token.token != TokenType::TK_UNTIL) {
                        let q: TreeNode = self.stmt();
                        if q.token.token != TokenType::NoToken && q.statement_type != StatementType::NoType {
                            retval.append(&q);
                        }
                        else {
                            break;
                        }
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
                    _ => {
                        self.current_token = self.get_next_token();
                    }
                }
                return statement;
            }

            fn while_stmt(&mut self) -> TreeNode {
                let while_token: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_WHILE);
                self.match_token(&TokenType::TK_LPAREN);
                let condition: TreeNode = self.expression();
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
                let condition: TreeNode = self.expression();
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
                let condition: TreeNode = self.expression();
                self.match_token(&TokenType::TK_RPAREN);
                
                return new_repeat(&do_token, &condition, &selection);

            }

            fn assign_stmt(&mut self) -> TreeNode{
                let mut variable: TreeNode = null_tree();
                let _var: Token = self.current_token.copy_token();
                if _var.token == TokenType::TK_ID {
                    variable = new_var(&_var);
                }
                self.match_token(&TokenType::TK_ID);
                let _assign: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_ASSIGN);
                let rvalue: TreeNode = self.expression();
                return new_assignment(&_assign, &variable, &rvalue);

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
                    return new_arithmetic(&_op, &result, &self.simple_exp());
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
                    _ => {
                        panic!("Error, this token couldn't be processed: {}", &self.current_token.lexema)
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
    let mut show_tokens = false;
    let mut show_gramar = true;
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|s| ONLY_TOKENS == s){
        show_tokens = true;
        show_gramar = false;
    }
    if args.iter().any(|s| ONLY_GRAMAR == s) {
        show_tokens = false;
        show_gramar = true;
    }
    let mut Scanner  = scanner::Scanner::new(&args[args.len()-1], false);
    let mut parser: parser::TokenParser = parser::new(Scanner);
    parser.parse();
    if show_gramar{
        parser.print_parser();
    }
}
