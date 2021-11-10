// Este es el modulo completo del compilador con todas las definiciones de los analizadores
mod compiler {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    //Es un enumerador para los tokens del lenguaje para el que vamos a crear el compilador
    pub enum TokenType {
        // Token para definir bloques de comentarios
        TK_COMMENT_LINE = 1,
        TK_COMMENT_BLOCK,
        // para leer y escribir variables
        TK_READ,
        TK_WRITE,
        // Tokens multicaracter para numeros o identificadores
        TK_ID,
        TK_NUM,
        TK_INT,
        TK_FLOAT,
        TK_BOOL,
        TK_DECIMAL,
        //Símbolos especiales
        TK_LPAREN,
        TK_RPAREN,
        TK_SEMICOLON,
        TK_COMMA,
        TK_ASSIGN,
        TK_PLUS,
        TK_MINUS,
        TK_TIMES,
        TK_OVER,
        TK_EXP,
        TK_LT,
        TK_LTE,
        TK_GT,
        TK_GTE,
        TK_EQ,
        TK_DIF,
        TK_RKEY,
        TK_LKEY,
        //Tokens para casos especiales
        TK_EOF,
        TK_ERROR,
        //Tokens para flujo de operaciones
        TK_PROG,
        TK_IF,
        TK_ELSE,
        TK_FI,
        TK_DO,
        TK_UNTIL,
        TK_WHILE,
        TK_THEN,
        //Tokes para operaciones lógicas
        TK_NOT,
        TK_AND,
        TK_OR,
        //Expresiones literales booleanas
        TK_TRUE,
        TK_FALSE,
        NoToken,
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum TinyType {
        Integer = 1,
        Float,
        Boolean,
        NoType,
    }

    #[derive(Clone, PartialEq, Eq)]
    pub struct BucketList {
        lines: Vec<u32>,
        mem_location: u32,
        data_type: TinyType,
    }

    #[derive(Copy, Clone, PartialEq, Eq)]
    pub enum StatementType {
        Sequence = 1,
        Program,
        Literal,
        Binary,
        Arithmetic,
        Relational,
        Variable,
        Read,
        NoType,
        Assignment,
        Begin,
        End,
        If,
        Repeat,
        While,
        Comment,
        Write,
        Not,
        LiteralBoolExp,
        BooleanExp,
        VariableSeq,
        ListVariableDec,
    }

    #[derive(Debug, Clone)]
    pub struct Token {
        pub token: TokenType,
        pub lexema: String,
        pub line: u32,
    }

    #[derive(Clone)]
    pub struct TreeNode {
        token: Token,
        is_expression: bool,
        nodes: Vec<TreeNode>,
        statement_type: StatementType,
        val_type: TinyType,
        is_lvalue: bool,
    }

    pub fn null_tree() -> TreeNode {
        return TreeNode {
            token: Token {
                token: TokenType::NoToken,
                lexema: String::from(""),
                line: 0,
            },
            is_expression: false,
            nodes: vec![],
            statement_type: StatementType::NoType,
            val_type: TinyType::NoType,
            is_lvalue: false,
        };
    }

    pub fn new_literal(token: &Token, data_type: TinyType) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![],
            statement_type: StatementType::Literal,
            val_type: data_type,
        };
        return result;
    }
    pub fn new_var(token: &Token) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![],
            statement_type: StatementType::Variable,
            val_type: TinyType::NoType,
        };
        return result;
    }
    pub fn new_arithmetic(
        token: &Token,
        left_value: &TreeNode,
        right_value: &TreeNode,
    ) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            nodes: vec![left_value.copy(), right_value.copy()],
            statement_type: StatementType::Arithmetic,
            val_type: TinyType::NoType,
            is_lvalue: false,
        };
        return result;
    }

    pub fn new_relational(
        token: &Token,
        left_value: &TreeNode,
        right_value: &TreeNode,
    ) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            nodes: vec![left_value.copy(), right_value.copy()],
            statement_type: StatementType::Relational,
            val_type: TinyType::NoType,
            is_lvalue: false,
        };
        return result;
    }

    pub fn new_assignment(token: &Token, _var: &TreeNode, rvalue: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            nodes: vec![_var.copy(), rvalue.copy()],
            statement_type: StatementType::Assignment,
            val_type: TinyType::NoType,
            is_lvalue: false,
        };
        return result;
    }

    pub fn new_unary(token: &Token, b_factor: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![b_factor.copy()],
            statement_type: StatementType::Not,
            val_type: TinyType::Boolean,
        };
        return result;
    }

    pub fn new_if(
        token: &Token,
        condition: &TreeNode,
        selection: &TreeNode,
        otherwise: &TreeNode,
    ) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![condition.copy(), selection.copy(), otherwise.copy()],
            statement_type: StatementType::If,
            val_type: TinyType::NoType,
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
            val_type: TinyType::NoType,
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
            val_type: TinyType::NoType,
        };
        return result;
    }

    pub fn new_program(token: &Token, variables: &TreeNode, program: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variables.copy(), program.copy()],
            statement_type: StatementType::Program,
            val_type: TinyType::NoType,
        };
        return result;
    }

    pub fn new_sequence(stmt: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: Token {
                token: TokenType::NoToken,
                lexema: String::from(""),
                line: 0,
            },
            is_expression: false,
            is_lvalue: false,
            nodes: vec![stmt.copy()],
            statement_type: StatementType::Sequence,
            val_type: TinyType::NoType,
        };
        return result;
    }

    pub fn new_read(token: &Token, variable: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variable.copy()],
            statement_type: StatementType::Read,
            val_type: TinyType::NoType,
        };
        return result;
    }

    pub fn new_write(token: &Token, exp: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![exp.copy()],
            statement_type: StatementType::Write,
            val_type: TinyType::NoType,
        };
        return result;
    }

    pub fn new_literal_boolean(token: &Token) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: true,
            is_lvalue: false,
            nodes: vec![],
            statement_type: StatementType::LiteralBoolExp,
            val_type: TinyType::Boolean,
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
            val_type: TinyType::Boolean,
        };
        return result;
    }

    pub fn new_sequence_var(variables: &TreeNode) -> TreeNode {
        let result = TreeNode {
            token: Token {
                token: TokenType::NoToken,
                lexema: String::from(""),
                line: 0,
            },
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variables.copy()],
            statement_type: StatementType::VariableSeq,
            val_type: TinyType::NoType,
        };
        return result;
    }
    pub fn new_list_dec(token: &Token, variables: &TreeNode, val_type: TinyType) -> TreeNode {
        let result = TreeNode {
            token: token.copy_token(),
            is_expression: false,
            is_lvalue: false,
            nodes: vec![variables.copy()],
            statement_type: StatementType::ListVariableDec,
            val_type: val_type,
        };
        return result;
    }

    impl TreeNode {
        pub fn copy(&self) -> TreeNode {
            return TreeNode {
                token: self.token.copy_token(),
                is_expression: self.is_expression,
                nodes: self.nodes.to_vec(),
                statement_type: self.statement_type,
                val_type: self.val_type,
                is_lvalue: self.is_lvalue,
            };
        }
        pub fn type_name(&self) -> &str {
            match &self.val_type {
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

        pub fn get_type(&self) -> TinyType {
            return self.val_type;
        }

        pub fn set_type(&mut self, new_type: &TinyType) {
            self.val_type = *new_type;
        }

        pub fn is_lvalue(&self) -> bool {
            self.is_lvalue
        }

        pub fn set_lvalue(&mut self, x: bool) {
            self.is_lvalue = x;
        }
        pub fn append(&mut self, next: &TreeNode) {
            self.nodes.push(next.copy());
        }

        pub fn print_grammar_tree(&self, number_idents: u32) {
            if self.statement_type != StatementType::Sequence
                && self.statement_type != StatementType::VariableSeq
                && self.token.token != TokenType::NoToken
            {
                for n in 1..number_idents {
                    print!(" ");
                }
                println!("{} {:?}", self.token.lexema, self.val_type);
            }

            for node in &self.nodes {
                node.print_grammar_tree(number_idents + 1);
            }
        }

        pub fn print_syntax_tree(&self, number_idents: u32) {
            if self.statement_type != StatementType::Sequence
                && self.statement_type != StatementType::VariableSeq
                && self.token.token != TokenType::NoToken
            {
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
            line: 0,
        };
        return result;
    }
    impl Token {
        pub fn copy_token(&self) -> Token {
            let new_token: Token = Token {
                token: self.token,
                lexema: self.lexema.clone(),
                line: self.line,
            };
            return new_token;
        }
    }

    pub mod scanner {
        use super::Token;
        use super::TokenType;
        use std::fs;
        use std::io::prelude::*;
        use std::io::BufReader;
        //Son los estados de la máquina para el autómata
        #[derive(Debug, PartialEq, Eq)]
        enum StateType {
            Start = 1,
            Id,
            Num,
            CommentLine,
            CommentBlock,
            Error,
            IsDone,
            Decimal,
        }
        pub struct Scanner {
            file_name: String,
            file_buffer: BufReader<fs::File>,
            current_pos: usize,
            current_line: u32,
            _trace: bool,
            _echo_source: bool,
            initialized: bool,
            line_buff: Vec<char>,
            in_eof: bool,
        }
        impl Scanner {
            pub fn set_echo_source(&mut self, x: bool) {
                self._echo_source = x;
            }
            pub fn echo_source(&self) -> bool {
                self._echo_source
            }
            pub fn set_trace(&mut self, x: bool) {
                self._trace = x;
            }
            pub fn trace(&self) -> bool {
                self._trace
            }

            pub fn get_line(&self) -> u32 {
                self.current_line
            }

            pub fn get_pos(&self) -> usize {
                self.current_pos
            }

            fn get_next_char(&mut self) -> char {
                let mut buf = String::new();
                let zero_bytes: usize = 0;
                if self.current_pos >= self.line_buff.len() {
                    let num_bytes = self
                        .file_buffer
                        .read_line(&mut buf)
                        .expect("Couldn't read file'");
                    if num_bytes > zero_bytes {
                        self.line_buff = buf.chars().collect();
                        self.current_pos = 0;
                        self.current_line += 1;
                    } else {
                        self.in_eof = true;
                        return '\0';
                    }
                }
                let result = self.line_buff[self.current_pos];
                self.current_pos += 1;
                return result;
            }
            fn unget_next_char(&mut self) {
                if !self.in_eof {
                    self.current_pos -= 1;
                }
            }

            pub fn new(file_name: &str, token_trace: bool) -> Scanner {
                let file = fs::File::open(file_name)
                    .expect("Please, to use the program use lexic-analyzer <filename>");
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
                    in_eof: false,
                };
                return result;
            }

            fn is_delimiter(&self, c: char) -> bool {
                c == ' ' || c == '\t' || c == '\n'
            }

            fn is_reserved_word(&mut self, lexema: &str) -> TokenType {
                match lexema {
                    "program" => return TokenType::TK_PROG,
                    "if" => return TokenType::TK_IF,
                    "fi" => return TokenType::TK_FI,
                    "else" => return TokenType::TK_ELSE,
                    "do" => return TokenType::TK_DO,
                    "until" => return TokenType::TK_UNTIL,
                    "while" => return TokenType::TK_WHILE,
                    "read" => return TokenType::TK_READ,
                    "write" => return TokenType::TK_WRITE,
                    "float" => return TokenType::TK_FLOAT,
                    "int" => return TokenType::TK_INT,
                    "bool" => return TokenType::TK_BOOL,
                    "not" => return TokenType::TK_NOT,
                    "and" => return TokenType::TK_AND,
                    "or" => return TokenType::TK_OR,
                    "then" => return TokenType::TK_THEN,
                    "true" => {
                        return TokenType::TK_TRUE;
                    }
                    "false" => {
                        return TokenType::TK_FALSE;
                    }
                    _ => return TokenType::TK_ID,
                }
            }

            pub fn get_token(&mut self) -> Token {
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
                            } else if c.is_digit(10) {
                                state = StateType::Num;
                            } else if c.is_alphabetic() || c == '_' {
                                state = StateType::Id;
                            } else if self.is_delimiter(c) {
                                save = false;
                            } else if c == '(' {
                                state = StateType::IsDone;
                                token = TokenType::TK_LPAREN;
                            } else if c == ')' {
                                state = StateType::IsDone;
                                token = TokenType::TK_RPAREN;
                            } else if c == '{' {
                                state = StateType::IsDone;
                                token = TokenType::TK_LKEY;
                            } else if c == '}' {
                                state = StateType::IsDone;
                                token = TokenType::TK_RKEY;
                            } else if c == ';' {
                                state = StateType::IsDone;
                                token = TokenType::TK_SEMICOLON;
                            } else if c == ',' {
                                token = TokenType::TK_COMMA;
                                state = StateType::IsDone;
                            } else if c == '+' {
                                token = TokenType::TK_PLUS;
                                state = StateType::IsDone;
                            } else if c == '-' {
                                token = TokenType::TK_MINUS;
                                state = StateType::IsDone;
                            } else if c == '*' {
                                token = TokenType::TK_TIMES;
                                state = StateType::IsDone;
                            } else if c == '/' {
                                token = TokenType::TK_OVER;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '/' {
                                    state = StateType::CommentLine;
                                    token = TokenType::TK_COMMENT_LINE;
                                    save = false
                                } else if tempC == '*' {
                                    state = StateType::CommentBlock;
                                    token = TokenType::TK_COMMENT_BLOCK;
                                    save = false;
                                } else {
                                    self.unget_next_char();
                                }
                            } else if c == '^' {
                                token = TokenType::TK_EXP;
                                state = StateType::IsDone;
                            } else if c == '=' {
                                token = TokenType::TK_ASSIGN;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_EQ;
                                } else {
                                    self.unget_next_char();
                                }
                            } else if c == '<' {
                                token = TokenType::TK_LT;
                                state = StateType::IsDone;

                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_LTE;
                                } else {
                                    self.unget_next_char();
                                }
                            } else if c == '>' {
                                token = TokenType::TK_GT;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_GTE;
                                } else {
                                    self.unget_next_char();
                                }
                            } else if c == '!' {
                                token = TokenType::TK_ERROR;
                                state = StateType::IsDone;
                                let tempC: char = self.get_next_char();
                                if tempC == '=' {
                                    lexema.push(c);
                                    c = tempC;
                                    token = TokenType::TK_DIF;
                                } else {
                                    self.unget_next_char();
                                }
                            }
                        }
                        StateType::Num => {
                            if !c.is_digit(10) && c != '.' {
                                self.unget_next_char();
                                state = StateType::IsDone;
                                save = false;
                                token = TokenType::TK_NUM;
                            } else if c == '.' {
                                state = StateType::Decimal;
                                token = TokenType::TK_DECIMAL;
                            }
                        }
                        StateType::Decimal => {
                            if !c.is_digit(10) {
                                self.unget_next_char();
                                state = StateType::IsDone;
                                save = false;
                                token = TokenType::TK_DECIMAL;
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
                            } else {
                                self.unget_next_char();
                            }
                        }
                        //Should never happen
                        StateType::IsDone => {}
                        _ => {}
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
                let result_token = Token {
                    token: token,
                    lexema: String::from(lexema),
                    line: start_line,
                };
                if self._trace {
                    println!(
                        "({:?},{}, {})",
                        result_token.token, result_token.lexema, result_token.line
                    );
                }
                return result_token;
            }
        }
    }

    pub mod codegen {
        use super::TreeNode;
        use super::analyzer::SymbolTable;
        use super::StatementType;
        use super::TokenType;

        pub struct CodeGenResult {
            emit_loc: i64,
            high_emit_loc: i64,
            tmp_offset: i64
        }

        impl CodeGenResult {
            pub fn new() -> CodeGenResult {
                CodeGenResult {
                    emit_loc: 0,
                    high_emit_loc: 0,
                    tmp_offset: 0
                }
            }

            pub fn code_gen(&mut self, node: &TreeNode, st: &mut SymbolTable){
                self.emit_comment("TINY Compilation to TM Code");
                self.emit_comment("Standard prelude:");
                self.emit_rm("LD", 6, 0, 0, "load maxaddress from location 0");
                self.emit_rm("ST", 0, 0, 0, "clear location 0");
                self.emit_comment("End of standard prelude.");
                self.code_gen_helper(node, st);
                self.emit_comment("End of execution.");
                self.emit_ro("HALT", 0, 0, 0, "");
            }
    
            fn emit_comment(&mut self, comment: &str){
                println!("; {}", comment);
            }

            fn emit_rm(&mut self, op: &str, r: i64, d: i64, s: i64, comment: &str){
                println!("{}: {} {},{}({})", self.emit_loc, op, r, d, s);
                self.emit_loc += 1;
                self.emit_comment(comment);
                if self.emit_loc > self.high_emit_loc {
                    self.high_emit_loc = self.emit_loc;
                }
            }

            fn emit_ro(&mut self, op: &str, r: i64, s: i64, t: i64, comment: &str){
                println!("{}: {} {},{},{}", self.emit_loc, op, r, s, t);
                self.emit_loc += 1;
                self.emit_comment(comment);
                if self.emit_loc > self.high_emit_loc {
                    self.high_emit_loc = self.emit_loc;
                }
            }

            fn emit_skip(&mut self, many: i64) -> i64{
                let mut i = self.emit_loc;
                self.emit_loc += many;
                if self.high_emit_loc < self.emit_loc {
                    self.high_emit_loc = self.emit_loc;
                }
                return i;
            }
    
            fn emit_backup(&mut self, loc: i64){
                if loc > self.high_emit_loc {
                    self.emit_comment("BUG in emit backup");
                    self.emit_loc = loc;
                }
            }

            fn emit_restore(&mut self){
                self.emit_loc = self.high_emit_loc;
            }

            fn emit_rm_abs(&mut self, op: &str, r: i64, a: i64, comment: &str){
                println!("{}: {} {},{}({})", self.emit_loc, op, r, (a-(self.emit_loc-1)), 7);
                self.emit_loc += 1;
                self.emit_comment(comment);
                if self.emit_loc > self.high_emit_loc {
                    self.high_emit_loc = self.emit_loc;
                }
            }
    
            fn code_gen_helper(&mut self, node: &TreeNode, st: &mut SymbolTable){
                match node.statement_type {
                    StatementType::Program => {
                        for child in &node.nodes {
                            self.code_gen_helper(child, st);
                        }
                    },
                    StatementType::Sequence => {
                        for child in &node.nodes {
                            self.code_gen_helper(child, st);
                        }
                    },
                    StatementType::VariableSeq => {
                        for child in &node.nodes {
                            self.code_gen_helper(child, st);
                        }
                    },
                    StatementType::If => {
                        self.emit_comment("-> if");
                        self.code_gen_helper(&node.nodes[0], st);
                        let mut saved_loc_1 = self.emit_skip(1);
                        self.emit_comment("if: jump to else belongs here");

                        self.code_gen_helper(&node.nodes[1], st);
                        let mut saved_loc_2 = self.emit_skip(1);
                        let mut current_loc = self.emit_skip(0);
                        self.emit_backup(saved_loc_1);
                        self.emit_rm_abs("JEQ", 0, current_loc, "jmp to end");
                        self.emit_restore();

                        self.code_gen_helper(&node.nodes[2], st);
                        current_loc = self.emit_skip(0);
                        self.emit_backup(saved_loc_2);
                        self.emit_rm_abs("LDA", 7, current_loc, "jmp to end");
                        self.emit_restore();
                        self.emit_comment("<- if");
                    },
                    StatementType::Repeat => {
                        self.emit_comment("-> repeat");
                        let mut saved_loc_1 = self.emit_skip(0);
                        self.emit_comment("repeat: jump after body comes back here");
                        self.code_gen_helper(&node.nodes[1], st);
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_rm_abs("JEQ", 0, saved_loc_1, "repeat: jmp back to body");
                        self.emit_comment("<- repeat");
                    },
                    StatementType::While => {
                        self.emit_comment("-> while");
                        let mut saved_loc_1 = self.emit_skip(1);
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_comment("while: jmp here for check");
                        let mut current_loc = self.emit_skip(0);
                        self.code_gen_helper(&node.nodes[1], st);                        
                        self.emit_comment("<- while");
                    },
                    StatementType::Assignment => {
                        self.emit_comment("-> assign");
                        self.code_gen_helper(&node.nodes[1], st);
                        let mut loc = st.lookup_no_decl(&node.nodes[0].token.lexema).unwrap().mem_location;
                        self.emit_rm("ST", 0, loc.into(), 5, "assign: store value");
                        self.emit_comment("<- assign");
                    },
                    StatementType::Read => {
                        self.emit_ro("IN", 0, 0, 0, "read value");
                        let loc = st.lookup_no_decl(&node.nodes[0].token.lexema).unwrap().mem_location;
                        self.emit_rm("ST", 0, loc.into(), 5, "read: store value");
                    },
                    StatementType::Write => {
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_ro("OUT", 0, 0, 0, "write ac");
                    },
                    StatementType::LiteralBoolExp => {
                        self.emit_comment("-> literal boolean");
                        let value: &str = &node.token.lexema;
                        match value {
                            "true" => {
                                self.emit_rm("LDC", 0, 1, 0, "load const true value");
                            },
                            "false" => {
                                self.emit_rm("LDC", 0, 0, 0, "load const false value");
                            }
                            _ =>{}
                        }
                        self.emit_comment("<- literal boolean");
                    },
                    StatementType::Literal => {
                        self.emit_comment("-> const");
                        self.emit_rm("LDC", 0, node.token.lexema.parse().unwrap_or(0), 0, "load const");
                        self.emit_comment("<- const");
                    },
                    StatementType::Variable => {
                        self.emit_comment("-> id");
                        let loc: u32 = st.lookup_no_decl(&node.token.lexema).unwrap().mem_location;
                        self.emit_rm("LD", 0, loc.into(), 5, "load id value");
                        self.emit_comment("<- id");
                    },
                    StatementType::Not => {
                        self.emit_comment("-> bool expression");
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_rm("ST", 0, self.temp_offset+1, 6, "bool op: push left");
                        self.tmp_offset += 1;
                        selr.emit_ro("NOT", 0, 1, 0, "bool op: not");
                        self.emit_comment("-> bool expression");
                    },
                    StatementType::BooleanExp => {
                        self.emit_comment("-> bool expression");
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_rm("ST", 0, self.temp_offset-1, 6, "bool op: push left");
                        self.code_gen_helper(&nodde.nodes[1], st);
                        self.emit_rm("LD", 1, self.tmp_offset+1, 6, "bool op: push right");
                        self.tmp_offset += 1;
                        match node.token.token {
                            TokenType::TK_AND => {
                                self.emit_ro("AND", 0, 1, 0, "bool op: and");
                            },
                            TokenType::TK_OR => {
                                self.emit_ro("OR", 0, 1, 0, "bool op: or");
                            }
                            _ => {}
                        }
                        self.emit_comment("-> bool expression");
                    },
                    StatementType::Arithmetic => {
                        self.emit_comment("-> op");
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_rm("ST", 0, self.tmp_offset-1, 6, "op: push left");
                        self.code_gen_helper(&node.nodes[1], st);
                        self.emit_rm("LD", 1, self.tmp_offset+1, 6, "op:push right");
                        self.tmp_offset += 1;
                        match node.token.token {
                            TokenType::TK_PLUS => {
                                self.emit_ro("ADD", 0, 1, 0, "op: +");
                            },
                            TokenType::TK_MINUS => {
                                self.emit_ro("SUB", 0, 1, 0, "op: -");
                            },
                            TokenType::TK_TIMES => {
                                self.emit_ro("MUL", 0, 1, 0, "op: *");
                            },
                            TokenType::TK_OVER => {
                                self.emit_ro("DIV", 0, 1, 0, "op: /");
                            }
                            _ => {}
                        }
                        self.emit_comment("<- op");
                    },
                    StatementType::Relational => {
                        self.emit_comment("-> rel");
                        self.code_gen_helper(&node.nodes[0], st);
                        self.emit_rm("ST", 0, self.tmp_offset-1, 6, "rel: push left");
                        self.code_gen_helper(&node.nodes[1], st);
                        self.emit_rm("LD", 1, self.tmp_offset+1, 6, "rel:push right");
                        self.tmp_offset += 1;
                        match node.token.token {
                            TokenType::TK_LT => {
                                self.emit_ro("SUB", 0, 1, 0, "op <");
                                self.emit_rm("JLT", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            TokenType::TK_LTE => {
                                self.emit_ro("SUB", 0, 1, 0, "op <=");
                                self.emit_rm("JLE", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            TokenType::TK_GT => {
                                self.emit_ro("SUB", 0, 1, 0, "op >");
                                self.emit_rm("JGT", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            TokenType::TK_GTE => {
                                self.emit_ro("SUB", 0, 1, 0, "op >=");
                                self.emit_rm("JGE", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            TokenType::TK_EQ => {
                                self.emit_ro("SUB", 0, 1, 0, "op ==");
                                self.emit_rm("JEQ", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            TokenType::TK_DIF => {
                                self.emit_ro("SUB", 0, 1, 0, "op <");
                                self.emit_rm("JNE", 0, 2, 7, "br if true");
                                self.emit_rm("LDC", 0, 0, 0, "false case");
                                self.emit_rm("LDA", 7, 1, 7, "unconditional jmp");
                                self.emit_rm("LDC", 0, 1, 0, "true case");
                            },
                            _ => {}
                        }
                        self.emit_comment("<- op");
                    },
                    _ => {}
                }
            }
        }
    }

    pub mod analyzer {
        use super::BucketList;
        use super::StatementType;
        use super::TinyType;
        use super::TreeNode;
        use std::collections::HashMap;

        pub struct SymbolTable {
            pub table: HashMap<String, BucketList>,
            pub init_mem: u32,
        }

        impl SymbolTable {
            pub fn new() -> SymbolTable {
                return SymbolTable {
                    table: HashMap::new(),
                    init_mem: 0,
                };
            }

            pub fn insert(&mut self, name: &str, lineno: u32, loc: u32, data_type: TinyType) {
                if !self.table.contains_key(name) {
                    let new_variable: BucketList = BucketList {
                        lines: vec![lineno],
                        mem_location: loc,
                        data_type: data_type,
                    };
                    self.table.insert(String::from(name), new_variable);
                } else {
                    self.declarationError(name, lineno, "variable double declaration");
                }
            }

            pub fn build_table(&mut self, node: &TreeNode) {
                match node.statement_type {
                    StatementType::Program
                    | StatementType::VariableSeq
                    | StatementType::Sequence
                    | StatementType::If
                    | StatementType::Repeat
                    | StatementType::While
                    | StatementType::Read
                    | StatementType::Write
                    | StatementType::Not => {
                        for nodes in &node.nodes {
                            self.build_table(nodes);
                        }
                    }
                    StatementType::ListVariableDec => {
                        for nodes in &node.nodes {
                            self.insert(
                                &nodes.token.lexema,
                                nodes.token.line,
                                self.init_mem,
                                node.val_type,
                            );
                            self.init_mem += 1;
                        }
                    }
                    StatementType::Assignment
                    | StatementType::Arithmetic
                    | StatementType::Relational
                    | StatementType::BooleanExp => {
                        self.build_table(&node.nodes[0]);
                        self.build_table(&node.nodes[1]);
                    }
                    StatementType::Variable => {
                        self.lookup(&node.token.lexema, node.token.line);
                    }
                    _ => {}
                }
            }

            fn error_msg(&mut self, label: &str, name: &str, line: u32, msg: &str) {
                eprintln!(
                    "error: line - {} error: {}, msg: {} for {}",
                    line, label, msg, name
                );
            }

            fn declarationError(&mut self, name: &str, line: u32, msg: &str) {
                self.error_msg("symbol table", name, line, msg);
            }

            pub fn lookup(&mut self, name: &str, lineno: u32) -> Option<&BucketList> {
                if !self.table.contains_key(name) {
                    self.declarationError(name, 0, "variable no declared");
                } else {
                    self.table.get_mut(name).unwrap().lines.push(lineno);
                }
                return self.table.get(name);
            }

            pub fn lookup_no_decl(&mut self, name: &str) -> Option<&BucketList> {
                if !self.table.contains_key(name) {
                    self.declarationError(name, 0, "variable no declared");
                }
                return self.table.get(name);
            }

            pub fn print(&mut self) {
                println!("TABLA DE SIMBOLOS");
                println!("Variable Name  Location  Line Numbers  Data Type");
                println!("*************  ********  ************  **********");
                for (name, bucket) in &self.table {
                    for lines in &bucket.lines {
                        println!(
                            "{}  {}  {}  {:?}",
                            name, bucket.mem_location, lines, bucket.data_type
                        );
                    }
                }
            }
        }
    }

    pub mod checker {
        use super::analyzer::SymbolTable;
        use super::StatementType;
        use super::TinyType;
        use super::TreeNode;

        fn postProc(node: &mut TreeNode, sym_table: &mut SymbolTable) {
            match node.statement_type {
                StatementType::If => {
                    if node.nodes[0].val_type != TinyType::Boolean {
                        eprintln!(
                            "error: line - {} error: if condition is not boolean",
                            node.token.line
                        );
                    }
                }
                StatementType::While => {
                    if node.nodes[0].val_type != TinyType::Boolean {
                        eprintln!(
                            "error: line - {} error: while condition is not boolean",
                            node.token.line
                        );
                    }
                }
                StatementType::Repeat => {
                    if node.nodes[0].val_type != TinyType::Boolean {
                        eprintln!(
                            "error: line - {} error: repeat condition is not boolean",
                            node.token.line
                        );
                    }
                }
                StatementType::Not => {
                    if node.nodes[0].val_type != TinyType::Boolean {
                        eprintln!(
                            "error: line - {} error: not in not a boolean value",
                            node.token.line
                        );
                    }
                }

                StatementType::BooleanExp => {
                    if node.nodes[0].val_type == TinyType::Boolean
                        && node.nodes[1].val_type == TinyType::Boolean {
                            node.val_type = TinyType::Boolean;
                    } else{
                        eprintln!(
                            "error: line - {} error: cannot compare {:?} to {:?} for {}",
                            node.token.line,
                            node.nodes[0].val_type,
                            node.nodes[1].val_type,
                            &node.nodes[0].token.lexema
                        );
                    }
                }

                StatementType::Arithmetic => {
                    if node.nodes[0].val_type != TinyType::Boolean
                    && node.nodes[0].val_type != TinyType::NoType{
                        if node.nodes[1].val_type != TinyType::Boolean
                            && node.nodes[1].val_type != TinyType::NoType{
                                node.val_type = node.nodes[0].val_type;
                            }
                        else{
                            eprintln!(
                                "error: line - {} error: cannot do operation {:?} on {:?} for {}",
                                node.token.line,
                                node.nodes[0].val_type,
                                node.val_type,
                                &node.nodes[0].token.lexema
                            );
                        }
                    }
                    else{
                        eprintln!(
                            "error: line - {} error: cannot do operation {:?} on {:?} for {}",
                            node.token.line,
                            node.nodes[0].val_type,
                            node.val_type,
                            &node.nodes[0].token.lexema
                        );
                    }
                }
                StatementType::Relational => {
                    if node.nodes[0].val_type != TinyType::Boolean
                        && node.nodes[0].val_type != TinyType::NoType{
                            if node.nodes[1].val_type != TinyType::Boolean
                                && node.nodes[1].val_type != TinyType::NoType{
                                    node.val_type = TinyType::Boolean;
                                }
                            else{
                                eprintln!(
                                    "error: line - {} error: cannot compare {:?} to {:?} for {}",
                                    node.token.line,
                                    node.nodes[0].val_type,
                                    node.val_type,
                                    &node.nodes[0].token.lexema
                                );
                            }
                        }
                        else{
                            eprintln!(
                                "error: line - {} error: cannot compare {:?} to {:?} for {}",
                                node.token.line,
                                node.nodes[0].val_type,
                                node.val_type,
                                &node.nodes[0].token.lexema
                            );
                        }
                }
                StatementType::Assignment => {
                    if node.nodes[0].val_type != TinyType::Boolean
                        && node.nodes[0].val_type != TinyType::NoType
                    {
                        if node.nodes[1].val_type != TinyType::Boolean
                            && node.nodes[1].val_type != TinyType::NoType
                        {
                            node.val_type = node.nodes[0].val_type;
                        } else {
                            eprintln!(
                                "error: line - {} error: cannot assign {:?} to {:?} for {}",
                                node.token.line,
                                node.nodes[0].val_type,
                                node.val_type,
                                &node.nodes[0].token.lexema
                            );
                        }
                    } else {
                        if node.nodes[1].val_type != TinyType::Boolean {
                            eprintln!(
                                "error: line - {} error: cannot assign {:?} to {:?} for {}",
                                node.token.line,
                                node.nodes[0].val_type,
                                node.val_type,
                                &node.nodes[0].token.lexema
                            );
                        } else {

                                node.val_type = node.nodes[0].val_type;
                        }
                    }
                }
                StatementType::Variable => {
                    let var = sym_table.lookup_no_decl(&node.token.lexema);
                    node.val_type = var.unwrap().data_type;
                }
                _ => {}
            }
        }

        pub fn typeChecking(node: &mut TreeNode, sym_table: &mut SymbolTable) {
            for child in &mut node.nodes {
                typeChecking(child, sym_table);
            }
            postProc(node, sym_table);
        }
    }

    pub mod parser {
        use super::scanner::Scanner;
        use super::StatementType;
        use super::TinyType;
        use super::Token;
        use super::TokenType;
        use super::TreeNode;
        use super::{
            new_arithmetic, new_assignment, new_boolean_exp, new_if, new_list_dec, new_literal,
            new_literal_boolean, new_program, new_read, new_relational, new_repeat, new_sequence,
            new_sequence_var, new_unary, new_var, new_while, new_write, null_token, null_tree,
        };

        pub struct TokenParser {
            current_token: Token,
            pub program: TreeNode,
            scanner: Scanner,
            _error: bool,
        }
        pub fn new(scanner: Scanner) -> TokenParser {
            return TokenParser {
                current_token: null_token(),
                program: null_tree(),
                scanner: scanner,
                _error: false,
            };
        }
        impl TokenParser {
            fn error_msg(&mut self, label: &str, token: &Token, msg: &str) {
                eprintln!(
                    "error: line - {} error: {}, token: {:?}, msg: {}",
                    token.line, label, token.token, msg
                );
                self._error = true;
            }

            fn type_error(&mut self, token: &Token, msg: &str) {
                self.error_msg("type", token, msg);
            }

            fn syntax_error(&mut self, token: &Token, msg: &str) {
                self.error_msg("syntax", token, msg);
            }

            pub fn parse(&mut self) -> &TreeNode {
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

            fn match_token(&mut self, token_match: &TokenType) {
                if &self.current_token.token == token_match {
                    let new_token = self.get_next_token();
                    self.current_token = new_token;
                } else {
                    self.syntax_error(&self.current_token.copy_token(), "unexpected token");
                }
            }

            fn get_next_token(&mut self) -> Token {
                return self.scanner.get_token();
            }

            pub fn print_grammar_parser(&self) {
                println!("ARBOL GRAMATICAL");
                self.program.print_grammar_tree(1);
            }

            pub fn print_syntax_parser(&self) {
                println!("ARBOL SINTACTICO");
                self.program.print_syntax_tree(0);
            }

            fn seq_stmt(&mut self) -> TreeNode {
                if self.current_token.token == TokenType::TK_RKEY {
                    return new_sequence(&null_tree());
                }

                let mut retval: TreeNode = new_sequence(&self.stmt());
                while (self.current_token.token != TokenType::TK_EOF)
                    && (self.current_token.token != TokenType::TK_RKEY)
                    && (self.current_token.token != TokenType::TK_ELSE)
                    && (self.current_token.token != TokenType::TK_UNTIL)
                {
                    let q: TreeNode = self.stmt();
                    if (q.token.token != TokenType::NoToken)
                        && (q.statement_type != StatementType::NoType)
                    {
                        if q.statement_type != StatementType::Comment {
                            retval.append(&q);
                        }
                    } else {
                        break;
                    }
                }
                return retval;
            }

            fn seq_declaration(&mut self) -> TreeNode {
                if self.current_token.token == TokenType::TK_RKEY {
                    return new_sequence_var(&null_tree());
                }

                let mut retval: TreeNode = new_sequence_var(&self.declaration());
                while self.current_token.token == TokenType::TK_INT
                    || self.current_token.token == TokenType::TK_FLOAT
                    || self.current_token.token == TokenType::TK_BOOL
                {
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
                    TokenType::TK_BOOL => {
                        let _bool: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_BOOL);
                        statement = self.list_declaration(&_bool);
                        self.match_token(&TokenType::TK_SEMICOLON);
                    }
                    _ => {
                        self.current_token = self.get_next_token();
                        self.syntax_error(
                            &self.current_token.copy_token(),
                            "token in initial list variable declaration",
                        );
                    }
                }
                return statement;
            }

            fn list_declaration(&mut self, token: &Token) -> TreeNode {
                let _id: Token = self.current_token.copy_token();
                self.match_token(&TokenType::TK_ID);
                let mut first_var: TreeNode = new_var(&_id);
                let mut val_type: TinyType = TinyType::NoType;
                match token.token {
                    TokenType::TK_INT => val_type = TinyType::Integer,
                    TokenType::TK_FLOAT => val_type = TinyType::Float,
                    TokenType::TK_BOOL => val_type = TinyType::Boolean,
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

            fn stmt(&mut self) -> TreeNode {
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
                        self.syntax_error(
                            &self.current_token.copy_token(),
                            "Code ends before file",
                        );
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

            fn if_stmt(&mut self) -> TreeNode {
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

            fn assign_stmt(&mut self) -> TreeNode {
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
            fn b_expression(&mut self) -> TreeNode {
                let result: TreeNode = self.b_term();
                if self.current_token.token == TokenType::TK_OR {
                    let _or: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_OR);
                    return new_boolean_exp(&_or, &result, &self.b_term());
                } else {
                    return result;
                }
            }
            fn b_term(&mut self) -> TreeNode {
                let result: TreeNode = self.not_factor();
                if self.current_token.token == TokenType::TK_AND {
                    let _and: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_AND);
                    return new_boolean_exp(&_and, &result, &self.not_factor());
                } else {
                    return result;
                }
            }

            fn not_factor(&mut self) -> TreeNode {
                if self.current_token.token == TokenType::TK_NOT {
                    let _not: Token = self.current_token.copy_token();
                    self.match_token(&TokenType::TK_NOT);
                    return new_unary(&_not, &self.b_factor());
                }
                return self.b_factor();
            }

            fn b_factor(&mut self) -> TreeNode {
                if self.current_token.token == TokenType::TK_TRUE
                    || self.current_token.token == TokenType::TK_FALSE
                {
                    let _bool: Token = self.current_token.copy_token();
                    self.match_token(&_bool.token);
                    return new_literal_boolean(&_bool);
                }
                return self.expression();
            }

            fn expression(&mut self) -> TreeNode {
                let result: TreeNode = self.simple_exp();
                if self.current_token.token == TokenType::TK_LT
                    || self.current_token.token == TokenType::TK_LTE
                    || self.current_token.token == TokenType::TK_GT
                    || self.current_token.token == TokenType::TK_GTE
                    || self.current_token.token == TokenType::TK_DIF
                    || self.current_token.token == TokenType::TK_EQ
                {
                    let _op = self.current_token.copy_token();
                    self.match_token(&_op.token);
                    return new_relational(&_op, &result, &self.simple_exp());
                } else {
                    return result;
                }
            }
            fn simple_exp(&mut self) -> TreeNode {
                let result: TreeNode = self.term();
                if self.current_token.token == TokenType::TK_MINUS
                    || self.current_token.token == TokenType::TK_PLUS
                {
                    let _op = self.current_token.copy_token();
                    self.match_token(&_op.token);
                    return new_arithmetic(&_op, &result, &self.term());
                } else {
                    return result;
                }
            }
            fn term(&mut self) -> TreeNode {
                let result: TreeNode = self.factor();
                if self.current_token.token == TokenType::TK_TIMES
                    || self.current_token.token == TokenType::TK_OVER
                {
                    let _op = self.current_token.copy_token();
                    self.match_token(&_op.token);
                    return new_arithmetic(&_op, &result, &self.factor());
                } else {
                    return result;
                }
            }
            fn factor(&mut self) -> TreeNode {
                match self.current_token.token {
                    TokenType::TK_DECIMAL => {
                        let _decimal: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_DECIMAL);
                        return new_literal(&_decimal, TinyType::Float);
                    }
                    TokenType::TK_NUM => {
                        let _num: Token = self.current_token.copy_token();
                        self.match_token(&TokenType::TK_NUM);
                        return new_literal(&_num, TinyType::Integer);
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
                        self.syntax_error(
                            &self.current_token.copy_token(),
                            "Code ends before file",
                        );
                        return null_tree();
                    }
                }
            }
        }
    }
}

// use crate::compiler::Scanner;
use std::env;

// use crate::compiler::Token;
use crate::compiler::analyzer;
use crate::compiler::checker::typeChecking;
use crate::compiler::parser;
use crate::compiler::scanner;
use crate::compiler::codegen;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut scanner = scanner::Scanner::new(&args[args.len() - 1], true);

    let mut parser: parser::TokenParser = parser::new(scanner);
    parser.parse();
    let mut symbol_table: analyzer::SymbolTable = analyzer::SymbolTable::new();
    symbol_table.build_table(&parser.program);

    typeChecking(&mut parser.program, &mut symbol_table);

    let mut code_gen: codegen::CodeGenResult = codegen::CodeGenResult::new();

    parser.print_grammar_parser();
    parser.print_syntax_parser();
    symbol_table.print();
    code_gen.code_gen(&parser.program, &mut symbol_table);
}
