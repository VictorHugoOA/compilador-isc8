// Este es el modulo completo del compilador con todas las definiciones de los analizadores
mod compiler{
    #[derive(Debug)]
    //Es un enumerador para los tokens del lenguaje para el que vamos a crear el compilador
    pub enum TokenType{
        // Tokens para definir bloques de programa
        TK_BEGIN, TK_END, 
        // para leer y escribir variables
        TK_READ, TK_WRITE,
        // Tokens multicaracter para numeros o identificadores
        TK_ID, TK_NUM, TK_INT, TK_FLOAT, TK_BOOL,
        //Símbolos especiales 
        TK_LPAREN, TK_RPAREN, TK_SEMICOLON, TK_COMMA, TK_ASSIGN, TK_PLUS, TK_MINUS, TK_ASTERISC, TK_SLASH, TK_EXP, TK_LT, TK_LTE, TK_GT, TK_GTE, TK_EQ, TK_DIF, TK_LKEY, TK_RKEY,
        //Tokens para casos especiales
        TK_EOF, TK_ERROR,
        //Tokens para flujo de operaciones
        TK_PROG, TK_IF, TK_ELSE, TK_FI, TK_DO, TK_UNITL, TK_WHILE,
        //Tokes para operaciones lógicas
        TK_NOT, TK_AND, TK_OR
    }
    
    #[derive(Debug)]
    pub struct Token{
        token: TokenType,
        lexema: String
    }

    pub mod Scanner{
        use super::TokenType;
        use super::Token;
        
        //Son los estados de la máquina para el autómata
        #[derive(Debug)]
        enum StateType{
            ST_START, ST_ID, ST_NUM, ST_LPAREN, ST_RPAREN, ST_SEMICOLON, ST_COMMA, ST_ASSIGN, ST_ADD, ST_MINUS, ST_EOF, ST_ERROR, ST_DONE
        }

        fn is_delimiter (c: char) -> bool{
            c == ' ' || c == '\t' || c == '\n'
        }
        pub fn get_tokens(file: &Vec<char>) -> Vec<Token>{
            let mut c: char = '-';
            let mut tokens: Vec<Token> = Vec::new();
            let mut state: StateType = StateType::ST_START;
            let mut current_char: usize = 0;
            let mut done: bool = false;
            while current_char < (file.len() - 1) {
                let mut token: Token = Token{token: TokenType::TK_ERROR, lexema: String::from("")};
                done = false;
                while !done {
                    // println!("{}", done);

                    match state {
                        StateType::ST_START => {

                            c = file[current_char];
                            current_char = current_char + 1;

                            while is_delimiter(c) {
                                c = file[current_char];
                                current_char = current_char + 1;
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
                                done = true;
                            }
                            else if c == ')' {
                                token.token = TokenType::TK_RPAREN;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else if c == ';' {
                                token.token = TokenType::TK_SEMICOLON;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else if c == ',' {
                                token.token = TokenType::TK_COMMA;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else if c == ':' {
                                token.lexema.push(c);
                                state = StateType::ST_ASSIGN;
                            }
                            else if c == '+' {
                                token.token = TokenType::TK_PLUS;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else if c == '-' {
                                token.token = TokenType::TK_MINUS;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else if current_char == (file.len() - 1) {
                                token.token = TokenType::TK_EOF;
                                state = StateType::ST_DONE;
                                token.lexema.push(c);
                                done = true;
                            }
                            else {
                                token.token = TokenType::TK_ERROR;
                                state = StateType::ST_DONE;
                                done = true;
                            }
                            
                        }
                        StateType::ST_NUM => {
                            c = file[current_char];
                            current_char = current_char + 1;
                            token.lexema.push(c);
                            if !c.is_digit(10) {
                                token.token = TokenType::TK_NUM;
                                state = StateType::ST_DONE;
                                current_char = current_char - 1;
                                done = true;
                            }
                        }
                        StateType::ST_ID => {

                        }
                        _ =>{}
                    }
                }
                tokens.push(token);
            }
            tokens
        }
    }

}

use crate::compiler::Scanner;
use std::fs;
fn main() {
    let contents = fs::read_to_string("prueba.tny").expect("Could find file");
    println!("{}", contents);
    let vectorChars: Vec<char> = contents.chars().collect();
    let length = vectorChars.len();
    println!("{}", length);
    println!("{:?}", Scanner::get_tokens(&vectorChars));
}
