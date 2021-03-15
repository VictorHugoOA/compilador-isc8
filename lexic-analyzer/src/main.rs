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
        TK_ID, TK_NUM,
        //Símbolos especiales
        TK_LPAREN, TK_RPAREN, TK_SEMICOLON, TK_COMMA, TK_ASSIGN, TK_PLUS, TK_MINUS, 
        //Tokens para casos especiales
        TK_EOF, TK_ERROR
    }
    
    pub mod Scanner{
        use super::TokenType;
        //Son los estados de la máquina para el autómata
        enum StateType{
            ST_START, ST_ID, ST_NUM, ST_LPAREN, ST_RPAREN, ST_SEMICOLON, ST_COMMA, ST_ASSIGN, ST_ADD, ST_MINUS, ST_EOF, ST_ERROR, ST_DONE
        }
        //la linea actual
        fn get_next_char() -> char{
            'a'
        }
        pub fn get_token() -> TokenType{
            TokenType::TK_BEGIN
        }
    }

}

use crate::compiler::Scanner;
fn main() {
    println!("Hello, world!");
    Scanner::get_token();
}
