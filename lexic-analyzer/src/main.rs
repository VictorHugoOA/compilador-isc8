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
        TK_PROG, TK_IF, TK_ELSE, TK_FI, TK_DO, TK_UNITL, TK_WHILE,
        //Tokes para operaciones lógicas
        TK_NOT, TK_AND, TK_OR,
        NoToken
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum StatementType{
        Sequence = 1, Program, Literal, Binary, Arithmetic, Relational, Variable, Read, NoType, Assignment, Begin, End
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

    pub fn new_begin(token: &Token)-> TreeNode{
        let result = TreeNode {token: token.copy_token(), is_expression: false, is_lvalue: false, nodes: vec![], statement_type: StatementType::Begin, val_type: TinyType::NoType};
        return result;
    }

    pub fn new_end(token: &Token)-> TreeNode{
        let result = TreeNode {token: token.copy_token(), is_expression: false, is_lvalue: false, nodes: vec![], statement_type: StatementType::End, val_type: TinyType::NoType};
        return result;
    }

    pub fn new_program(token: &Token, begin: &TreeNode, program: &TreeNode)-> TreeNode{
        let result = TreeNode{token: token.copy_token(),
        is_expression: false,
        is_lvalue: false,
        nodes: vec![begin.copy(),
        program.copy()],
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
   
    impl Token{
        pub fn copy_token(&self) -> Token{
            let new_token: Token = Token { token: self.token, lexema: self.lexema.clone(), line: self.line };
            return new_token;
        }
    }

    pub mod parser{
        use super::Token;
        use super::TokenType;
        use super::TreeNode;
        use super::StatementType;
        use super::{null_tree, new_var, new_literal, new_arithmetic, new_relational, new_assignment, new_program, new_sequence, new_begin, new_end};

        pub struct TokenParser{
            current_token: Token,
            program: TreeNode,
            list_tokens: Vec<Token>,
            next_token: usize
        }
        pub fn new(token_list: &Vec<Token>) -> TokenParser{
            return TokenParser {current_token: token_list[0].copy_token(), program: null_tree(), list_tokens: token_list.to_vec(), next_token: 0};
        }
        impl TokenParser{
            pub fn parse(&mut self)-> &TreeNode{
                let program: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_PROG);
                let lkey: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_LKEY);
                self.program = new_program(&program, &new_begin(&lkey), &self.seq_stmt());
                let rkey: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_RKEY);
                self.program.append(&new_end(&rkey));
                if self.current_token.token != TokenType::TK_EOF {
                    panic!("Error in file");
                }

                return &self.program;
            }

            fn match_token(&mut self, token_match: &TokenType){
                if &self.current_token.token == token_match {
                    let new_token = self.get_next_token();
                    self.current_token = new_token;
                }
                else{
                    panic!("Error");
                }
            }

            fn get_next_token(&mut self) -> Token{
                self.next_token += 1;
                let result = self.list_tokens[self.next_token].copy_token();
                return result;
            }

            pub fn print_parser(&self){
                self.program.print_tree();
            }

            fn seq_stmt(&mut self) -> TreeNode{
                let mut retval: TreeNode = new_sequence(&self.stmt());
                while self.current_token.token != TokenType::TK_EOF || self.current_token.token != TokenType::TK_RKEY ||
                    self.current_token.token != TokenType::TK_ELSE || self.current_token.token != TokenType::TK_UNITL {
                        self.match_token(&TokenType::TK_SEMICOLON);
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
                    }
                    _ => {
                        
                    }
                }
                return statement;
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
    pub mod Scanner{
        use super::TokenType;
        use super::Token;
        //Son los estados de la máquina para el autómata
        #[derive(Debug)]
        enum StateType{
            ST_START, ST_ID, ST_NUM, ST_COMMENT_LINE, ST_COMMENT_BLOCK, ST_ERROR, ST_DONE
        }

        fn is_delimiter (c: char) -> bool{
            c == ' ' || c == '\t' || c == '\n'
        }
        fn is_reserved_word(lexema: &str) -> TokenType{
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
                   return TokenType::TK_UNITL
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

               _ => {
                   return TokenType::TK_ID
               }
           } 
        }

        fn get_next_char(line: &Vec<char>, current: usize) -> (char, bool){
            if current >= line.len() {
                return ('\0', false);
            }
            let result: (char, bool) = (line[current], true);
            return result;
        }

        pub fn get_tokens(line: &Vec<char>, number_line: u32) -> Vec<Token>{

            let mut c: char = '-';
            let mut tokens: Vec<Token> = Vec::new();
            let mut state: StateType = StateType::ST_START;
            let mut current_char: usize = 0;
            let mut done: bool = false;
            let mut exit_loop: bool = false;

            while !exit_loop {
                let mut token: Token = Token{token: TokenType::TK_ERROR, lexema: String::from(""), line: number_line};
                done = false;
                state = StateType::ST_START;
                if !get_next_char(line, current_char).1 {
                    exit_loop = true;
                    continue;
                }
                while !done {
                    match state {
                        StateType::ST_START => {

                            c = get_next_char(line, current_char).0;
                            current_char += 1;

                            while is_delimiter(c) {
                                let (character, end_of_line) = get_next_char(line, current_char);
                                current_char += 1;
                                c = character;
                            }

                            if c.is_alphabetic() {
                                state = StateType::ST_ID;
                                token.lexema.push(c);
                            }
                            else if c.is_numeric() {
                                state = StateType::ST_NUM;
                                token.lexema.push(c);
                            }
                            else if c == '(' {
                                token.token = TokenType::TK_LPAREN;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == ')' {
                                token.token = TokenType::TK_RPAREN;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '{' {
                                token.token = TokenType::TK_LKEY;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '}' {
                                token.token = TokenType::TK_RKEY;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == ';' {
                                token.token = TokenType::TK_SEMICOLON;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == ',' {
                                token.token = TokenType::TK_COMMA;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '+' {
                                token.token = TokenType::TK_PLUS;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '-' {
                                token.token = TokenType::TK_MINUS;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '*'{
                                token.token = TokenType::TK_TIMES;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }
                            else if c == '/'{
                                token.token = TokenType::TK_OVER;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                c = get_next_char(line, current_char).0;
                                current_char += 1;
                                if c == '/' {
                                    state = StateType::ST_COMMENT_LINE;
                                    token.token = TokenType::TK_COMMENT_LINE;
                                    token.lexema.push(c);
                                }
                                else if c == '*' {
                                    state = StateType::ST_COMMENT_BLOCK;
                                    token.token = TokenType::TK_COMMENT_BLOCK;
                                    token.lexema.push(c);
                                }
                                else{
                                    current_char -= 1;
                                }
                            }

                            else if c == '^'{
                                token.token = TokenType::TK_EXP;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                            }

                            else if c == '='{
                                token.lexema.push(c);
                                token.token = TokenType::TK_ASSIGN;
                                state = StateType::ST_DONE;
                                c = get_next_char(line, current_char).0;
                                current_char += 1;
                                if(c == '='){
                                    token.lexema.push(c);
                                    token.token = TokenType::TK_EQ;
                                }
                                else{
                                    current_char = current_char - 1;
                                }
                            }

                            else if c == '<'{
                                token.lexema.push(c);
                                token.token = TokenType::TK_LT;
                                state = StateType::ST_DONE;
                                c = get_next_char(line, current_char).0;
                                current_char += 1;
                                if(c == '='){
                                    token.lexema.push(c);
                                    token.token = TokenType::TK_LTE;
                                }
                                else{
                                    current_char = current_char - 1;
                                }
                            }

                            else if c == '>'{
                                token.token = TokenType::TK_GT;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                c = get_next_char(line, current_char).0;
                                current_char += 1;
                                if(c == '='){
                                    token.lexema.push(c);
                                    token.token = TokenType::TK_GTE;
                                }
                                else{
                                    current_char = current_char - 1;
                                }
                            }

                            else if c == '!'{
                                token.lexema.push(c);
                                token.token = TokenType::TK_ERROR;
                                state = StateType::ST_DONE;
                                c = get_next_char(line, current_char).0;
                                current_char += 1;
                                if(c == '='){
                                    token.lexema.push(c);
                                    token.token = TokenType::TK_DIF;
                                }
                                else{
                                    current_char = current_char - 1;
                                }
                            }
                        }
                        StateType::ST_NUM => {
                            let (character, valid) = get_next_char(line, current_char);
                            current_char += 1;
                            c = character;
                            token.lexema.push(c);
                            if !c.is_digit(10) || !valid {
                                token.token = TokenType::TK_NUM;
                                state= StateType::ST_DONE;
                                current_char -= 1;
                                token.lexema.pop();
                            }
                        }
                        StateType::ST_ID => {
                            let (character, valid) = get_next_char(line, current_char);
                            current_char += 1;
                            c = character;
                            token.lexema.push(c);
                            if !(c.is_alphanumeric() || c == '_') {
                                state = StateType::ST_DONE;
                                token.token= TokenType::TK_ID;
                                current_char = current_char - 1;
                                token.lexema.pop();
                                token.token = is_reserved_word(&token.lexema);
                            }
                        }
                        StateType::ST_COMMENT_LINE => {
                            let (character, valid) = get_next_char(line, current_char);
                            current_char += 1;
                            c = character;
                            token.lexema.push(c);
                            if !valid {
                                state = StateType::ST_DONE;
                            }
                        }
                        StateType::ST_COMMENT_BLOCK => {
                            let (character, valid) = get_next_char(line, current_char);
                            current_char += 1;
                            c = character;
                            token.lexema.push(c);
                            if c == '*'{
                                let (character, valid) = get_next_char(line, current_char);
                                current_char += 1;
                                c = character;
                                token.lexema.push(c);
                                if c == '/' {
                                    state = StateType::ST_DONE;
                                }
                            }
                            if !valid{
                                state = StateType::ST_ERROR;
                            }
                        }
                        StateType::ST_DONE => {
                            done = true;
                            continue;
                        }
                        _ =>{
                            token.token = TokenType::TK_ERROR;
                            state = StateType::ST_DONE;
                            token.lexema.push(c);
                        }
                    }
                }
                // println!("{:?}", token);
                tokens.push(token);
            }
            tokens
        }
    }

}

use crate::compiler::Scanner;
use std::fs;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use crate::compiler::Token;
use crate::compiler::TokenType;
use crate::compiler::parser;
const ONLY_TOKENS:&str = "--only-tokens";
const ONLY_GRAMAR:&str = "--only-gramar";

fn main() {
    let mut show_tokens = true;
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
    let file = fs::File::open(&args[args.len()-1]).expect("Please, to use the program use lexic-analyzer <filename>");

    let mut buf_reader = BufReader::new(file);

    let mut token_vector: Vec<Token> = Vec::new();

    let mut number_line: u32 = 0;
    for line in buf_reader.lines() {
        let unwrapped_line = line.unwrap();
        number_line += 1;
        let vector_chars: Vec<char> = unwrapped_line.chars().collect();
        let tokens = Scanner::get_tokens(&vector_chars, number_line);
        for token in &tokens{
            let copy_token = token.copy_token();
            token_vector.push(copy_token);
        }
    }
    number_line += 1;

    let token_eof = Token { token: TokenType::TK_EOF, lexema: String::from(""), line: number_line };
    token_vector.push(token_eof);

    if show_tokens {
        for token in &token_vector{
            println!("({:?},{}, {})", token.token, token.lexema, token.line);
        }
    }
    let mut parser: parser::TokenParser = parser::new(&token_vector);
    parser.parse();
    if show_gramar{
        parser.print_parser();
    }
}
