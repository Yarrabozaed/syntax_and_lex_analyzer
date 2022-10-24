use std::vec::Vec;
use std::env;
use std::io::Read;

//All tokens in Grammar
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token{
    ID(String),
    NUM(String),
    SEMICOLON,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    DEFINITIONS,
    OPERATIONS,
    POINT,
    CIRCLE,
    SQUARE,
    PRINT,
    CONTAINED,
    INTERSECTS,
    END,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Def{
    //ID, NUM, NUM
    PointDef(String, String, String),
    //ID, ID, NUM
    CircleDef(String, String, String),
    //ID, ID, NUM
    SquareDef (String, String, String),
    //ID
    PrintDef (String),
    //ID, ID
    ContainedDef (String, String),
    //ID, ID
    IntersectsDef (String, String),
}

//print bad lexeme & quit, used in match{} in token_scanner
fn lex_error(lex_crash: String) -> ! {
    println!("Lexical Error: {}", lex_crash);
    panic!();
}

//Produce a vector of type "Token" from string of data
//  match will take individual chars & using pattern matching will push the 
//  correct corresponding Token type to the vector
fn token_scanner(data: String) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let strlen = data.len();
    let mut i = 0;

    while i < strlen{
        let mut char = data.chars().nth(i);
        let mut tmp = data.chars().nth(i);

        //pattern matching for string literals & IDs
        if tmp.expect("REASON").is_alphabetic(){

            let mut tmp_str = "".to_string();
            let mut is_alpha = true;

            //checking for uppercase letters
            if (tmp.expect("REASON").is_uppercase()) && tmp.expect("REASON").is_alphabetic(){
                lex_error(format!("{:?}",tmp));
            }

            //putting together full strings, i.e. read letters until a non-alpha is found
            while is_alpha {

                let l : String = tmp.expect("REASON").to_string();

                tmp_str = format!("{}{}",tmp_str,l);

                i += 1;
                tmp = data.chars().nth(i);
                char = data.chars().nth(i);

                //checking for uppercase letters
                if (tmp.expect("REASON").is_uppercase()) && tmp.expect("REASON").is_alphabetic(){
                    lex_error(format!("{:?}",tmp));
                }

                if !tmp.expect("REASON").is_alphabetic(){
                    is_alpha = false;
                }
            }
            
            //default takes any unrecognized string and gives it the Token => ID
            match tmp_str.as_str(){
                "definitions" => tokens.push(Token::DEFINITIONS),
                "operations" => tokens.push(Token::OPERATIONS),
                "point" => tokens.push(Token::POINT),
                "circle" => tokens.push(Token::CIRCLE),
                "square" => tokens.push(Token::SQUARE),
                "print" => tokens.push(Token::PRINT),
                "contained" => tokens.push(Token::CONTAINED),
                "intersects" => tokens.push(Token::INTERSECTS),
                "end" => tokens.push(Token::END),
                _ => tokens.push(Token::ID(tmp_str)),
            }
        }

        //pattern matching for NUMs
        if tmp.expect("REASON").is_numeric(){
            let mut tmp_num = "".to_string();
            let mut is_num = true;

            while is_num {

                let n : String = tmp.expect("REASON").to_string();

                tmp_num = format!("{}{}",tmp_num,n);

                i += 1;
                tmp = data.chars().nth(i);
                char = data.chars().nth(i);
                if !tmp.expect("REASON").is_numeric(){
                    is_num = false;
                }
            }
            //Add NUM to Token vector
            tokens.push(Token::NUM(tmp_num));
        }

        match char{
            Some(';') => tokens.push(Token::SEMICOLON),
            Some(':') => tokens.push(Token::COLON),
            Some(',') => tokens.push(Token::COMMA),
            Some('.') => tokens.push(Token::PERIOD),
            Some('(') => tokens.push(Token::LPAREN),
            Some(')') => tokens.push(Token::RPAREN),
            Some('=') => tokens.push(Token::ASSIGN),
            Some(' ') => (),
            Some('\n') => (),
            Some('\t') => (),
            _ => lex_error(format!("{:?}",tmp)),
        };

        i += 1;
    };

    return tokens;
}

fn enough_tokens(tkn_count : usize, i : usize) -> bool {
    if (tkn_count-1) < (i+1) {
        panic!("Syntax Error: Not enough tokens.");
    }
    return true;
}

fn syntax_scanner(tokens: &Vec<Token>) -> Vec<Def>{
    let mut objects: Vec<Def> = Vec::new();
    let mut id_vec : Vec<String> = Vec::new();

    if tokens.len() < 2{
        panic!("Syntax Error: Not enough tokens.");
    }

    if tokens[0] != Token::DEFINITIONS{
        println!("Syntax Error: Missing \"definitions\" token in program start.");
        panic!();
    }
    if tokens[1] != Token::COLON{
        println!("Syntax Error: Missing \"colon\" token in program start.");
        panic!();
    }

    let mut i = 2;
    let vec_len = tokens.len();
    let mut scanning_ids = true;

    let mut item_val1;
    let mut item_val2;

    //scanning IDs
    while i < vec_len && (scanning_ids == true){

        match &tokens[i]{

            Token::ID(string) => {
                //get tuple ID value
                let item_id = string.to_string();
 
                let id_tmp_vct = string.to_string();
                
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                //check for missing assignment operator
                if tokens[i] != Token::ASSIGN{
                    panic!("Syntax Error: Missing Assignment Operator For ID {:?}", item_id);
                }
                
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::POINT => {
                        
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::LPAREN{
                            panic!("Syntax Error: Missing Left Par. For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::NUM(num) => {
                                item_val1 = num.to_string();
                            },
                            _ => panic!("Syntax Error: Point requires (NUM,NUM). Missing first NUM for ID {:?}", item_id),
                        }

                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::COMMA{
                            panic!("Syntax Error: Missing comma For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::NUM(num) => {
                                item_val2 = num.to_string();
                            },
                            _ => panic!("Syntax Error: Point requires (NUM,NUM). Missing second NUM for ID {:?}", item_id),
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::RPAREN{
                            panic!("Syntax Error: Missing Right Par. For ID {:?}", item_id);
                        }

                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] == Token::OPERATIONS {
                            i-=1;
                            scanning_ids = false;
                        }
                        else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::OPERATIONS{
                            panic!("Syntax Error: Extra Semicolon. after def for ID {:?}", item_id);
                        }
                        else{
                            if tokens[i] != Token::SEMICOLON{
                                panic!("Syntax Error: Missing Semicolon. For ID {:?}", item_id);
                            }
                        }

                        objects.push(Def::PointDef(item_id, item_val1, item_val2));
                        id_vec.push(id_tmp_vct);
                    },

                    Token::CIRCLE => {
        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::LPAREN{
                            panic!("Syntax Error: Missing Left Par. For ID {:?}", item_id);
                        }
                    
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::ID(id) => {
                                item_val1 = id.to_string();

                                let mut id_exists = false;
                                let mut iter = 0;

                                while iter < id_vec.len(){
                                    if id_vec[iter] == item_val1{
                                        id_exists = true;
                                    }
                                    iter+=1;
                                }

                                if id_exists == false{
                                    panic!("Syntax Error: In Circle {:?} , ID {:?} does not exist.", item_id, item_val1);
                                }
                            },
                            _ => panic!("Syntax Error: Circle requires (ID,NUM). Missing first ID for ID {:?}", item_id),
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::COMMA{
                            panic!("Syntax Error: Missing comma For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::NUM(num) => {
                                item_val2 = num.to_string();
                            },
                            _ => panic!("Syntax Error: Circle requires (ID,NUM). Missing second NUM for ID {:?}", item_id),
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::RPAREN{
                            panic!("Syntax Error: Missing Right Par. For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] == Token::OPERATIONS {
                            i-=1;
                            scanning_ids = false;
                        }else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::OPERATIONS{
                            panic!("Syntax Error: Extra Semicolon. after def for ID {:?}", item_id);
                        }
                        else{
                            if tokens[i] != Token::SEMICOLON{
                                panic!("Syntax Error: Missing Semicolon. For ID {:?}", item_id);
                            }
                        }

                        objects.push(Def::CircleDef(item_id, item_val1, item_val2));
                        id_vec.push(id_tmp_vct);

                    },

                    Token::SQUARE => {
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::LPAREN{
                            panic!("Syntax Error: Missing Left Par. For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::ID(id) => {
                                item_val1 = id.to_string();

                                let mut id_exists = false;
                                let mut iter = 0;

                                while iter < id_vec.len(){
                                    if id_vec[iter] == item_val1{
                                        id_exists = true;
                                    }
                                    iter+=1;
                                }

                                if id_exists == false{
                                    panic!("Syntax Error: In Square {:?} , ID {:?} does not exist.", item_id, item_val1);
                                }
                            },
                            _ => panic!("Syntax Error: Square requires (ID,NUM). Missing first ID for ID {:?}", item_id),
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::COMMA{
                            panic!("Syntax Error: Missing comma For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        match &tokens[i]{
                            Token::NUM(num) => {
                                item_val2 = num.to_string();
                            },
                            _ => panic!("Syntax Error: Square requires (ID,NUM). Missing second NUM for ID {:?}", item_id),
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] != Token::RPAREN{
                            panic!("Syntax Error: Missing Right Par. For ID {:?}", item_id);
                        }
                        
                        if enough_tokens(vec_len, i) == true{
                            i+=1;
                        }

                        if tokens[i] == Token::OPERATIONS {
                            i-=1;
                            scanning_ids = false;
                        }
                        else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::OPERATIONS{
                            panic!("Syntax Error: Extra Semicolon. after def for ID {:?}", item_id);
                        }
                        else{
                            if tokens[i] != Token::SEMICOLON{
                                panic!("Syntax Error: Missing Semicolon. For ID {:?}", item_id);
                            }
                        }

                        objects.push(Def::SquareDef(item_id, item_val1, item_val2));
                        id_vec.push(id_tmp_vct);

                    },

                    _ => (),
                };
            },
            _ => {
                if i == 2{
                    panic!("Syntax Error: Grammar requires at least one definition");
                }
                else if (tokens[i] == Token::PRINT) || (tokens[i] == Token::INTERSECTS) || (tokens[i] == Token::CONTAINED) || (tokens[i] == Token::END) || (tokens[i] == Token::PERIOD){
                    panic!("Syntax Error: definition statement expected");
                }
            },
        }
         
        if enough_tokens(vec_len, i) == true{
            i+=1;
        }
    }

    let ref_i = i;
    //scanning operations 
    if tokens[i] != Token::OPERATIONS{
        println!("Syntax Error: Missing \"operations\" token in program.");
        panic!();
    }
    
    if enough_tokens(vec_len, i) == true{
        i+=1;
    }

    if tokens[i] != Token::COLON{
        println!("Syntax Error: Missing \"colon\" token in program.");
        panic!();
    }
    
    if enough_tokens(vec_len, i) == true{
        i+=1;
    }

    let mut scanning_ops = true;

    while i < vec_len && (scanning_ops == true){
        match &tokens[i]{

            Token::PRINT => {
                
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::LPAREN{
                    panic!("Syntax Error: Missing Left Par. For PRINT");
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::ID(id) => {
                        item_val1 = id.to_string();

                        let mut id_exists = false;
                        let mut iter = 0;

                        while iter < id_vec.len(){
                            if id_vec[iter] == item_val1{
                                id_exists = true;
                            }
                            iter+=1;
                        }

                        if id_exists == false{
                            panic!("Syntax Error: In Print , ID {:?} does not exist.", item_val1);
                        }
                    },
                    _ => panic!("Syntax Error: Print requires (ID)."),
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::RPAREN{
                    panic!("Syntax Error: Missing Right Par. for PRINT");
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] == Token::END {
                    i-=1;
                    scanning_ops = false;
                }
                else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::END{
                    panic!("Syntax Error: Extra Semicolon. For PRINT");
                }
                else{
                    if tokens[i] != Token::SEMICOLON{
                        panic!("Syntax Error: Missing Semicolon. For Print");
                    }
                }

                objects.push(Def::PrintDef(item_val1));
            },

            Token::CONTAINED => {
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::LPAREN{
                    panic!("Syntax Error: Missing Left Par. For CONTAINED");
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::ID(id) => {
                        item_val1 = id.to_string();

                        let mut id_exists = false;
                        let mut iter = 0;

                        while iter < id_vec.len(){
                            if id_vec[iter] == item_val1{
                                id_exists = true;
                            }
                            iter+=1;
                        }

                        if id_exists == false{
                            panic!("Syntax Error: In Contained , ID {:?} does not exist.", item_val1);
                        }
                    },
                    _ => panic!("Syntax Error: Contained requires (ID,ID)."),
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::COMMA{
                    panic!("Syntax Error: Missing comma For CONTAINED");
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::ID(id) => {
                        item_val2 = id.to_string();

                        let mut id_exists = false;
                        let mut iter = 0;

                        while iter < id_vec.len(){
                            if id_vec[iter] == item_val2{
                                id_exists = true;
                            }
                            iter+=1;
                        }

                        if id_exists == false{
                            panic!("Syntax Error: In CONTAINED , ID {:?} does not exist.", item_val2);
                        }
                    },
                    _ => panic!("Syntax Error: CONTAINED requires (ID,ID)."),
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::RPAREN{
                    panic!("Syntax Error: Missing Right Par. for CONTAINEDs");
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] == Token::END {
                    i-=1;
                    scanning_ops = false;
                }
                else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::END{
                    panic!("Syntax Error: Extra Semicolon. For CONTAINED");
                }
                else{
                    if tokens[i] != Token::SEMICOLON{
                        panic!("Syntax Error: Missing Semicolon. For CONTAINED");
                    }
                }

                objects.push(Def::ContainedDef(item_val1, item_val2));
            },

            Token::INTERSECTS => {
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::LPAREN{
                    panic!("Syntax Error: Missing Left Par. For INTERSECTS");
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::ID(id) => {
                        item_val1 = id.to_string();

                        let mut id_exists = false;
                        let mut iter = 0;

                        while iter < id_vec.len(){
                            if id_vec[iter] == item_val1{
                                id_exists = true;
                            }
                            iter+=1;
                        }

                        if id_exists == false{
                            panic!("Syntax Error: In INTERSECTS , ID {:?} does not exist.", item_val1);
                        }
                    },
                    _ => panic!("Syntax Error: INTERSECTS requires (ID,ID)."),
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::COMMA{
                    panic!("Syntax Error: Missing comma For INTERSECTS");
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                match &tokens[i]{
                    Token::ID(id) => {
                        item_val2 = id.to_string();

                        let mut id_exists = false;
                        let mut iter = 0;

                        while iter < id_vec.len(){
                            if id_vec[iter] == item_val2{
                                id_exists = true;
                            }
                            iter+=1;
                        }

                        if id_exists == false{
                            panic!("Syntax Error: In INTERSECTS , ID {:?} does not exist.", item_val2);
                        }
                    },
                    _ => panic!("Syntax Error: INTERSECTS requires (ID,ID)."),
                }
                 
                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] != Token::RPAREN{
                    panic!("Syntax Error: Missing Right Par. for INTERSECTS");
                }

                if enough_tokens(vec_len, i) == true{
                    i+=1;
                }

                if tokens[i] == Token::END {
                    i-=1;
                    scanning_ops = false;
                }
                else if tokens[i] == Token::SEMICOLON && tokens[i+1] == Token::END{
                    panic!("Syntax Error: Extra Semicolon. For INTERSECTS");
                }
                else{
                    if tokens[i] != Token::SEMICOLON{
                        panic!("Syntax Error: Missing Semicolon. For INTERSECTS");
                    }
                }

                objects.push(Def::IntersectsDef(item_val1, item_val2));
            },

            _ => {
                match &tokens[i]{
                    Token::ID(_s) => {
                        panic!("Syntax Error: print, intersects or contained expected");
                    },
                    _ => {

                    },
                }
                if (i - ref_i) == 2{
                    panic!("Syntax Error: Grammar requires at least one operation");
                }
                else if (tokens[i] == Token::POINT) || (tokens[i] == Token::CIRCLE) || (tokens[i] == Token::SQUARE) || (tokens[i] == Token::DEFINITIONS){
                    panic!("Syntax Error: print, intersects or contained expected");
                }
            },
        }
         
        if enough_tokens(vec_len, i) == true{
            i+=1;
        }
    }

    if tokens[i] != Token::END{
        println!("Syntax Error: Missing \"end\" token in program.");
        panic!();
    }
     
    if enough_tokens(vec_len, i) == true{
        i+=1;
    }
    if tokens[i] != Token::PERIOD{
        println!("Syntax Error: Missing \"period\" token in program.");
        panic!();
    }
    
    return objects;
}

//take vector of defs and turn components into scheme syntax
fn make_scheme_syntax(components: &Vec<Def>){
    let components_len = components.len();
    let mut i = 0;
    let mut tmp_var_br;
    let mut end_components : String = "".to_string();
    let mut builder : String = "".to_string();

    while i < components_len {
        match &components[i]{
            Def::PrintDef(id) => {
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id;                

                print!("(print");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("-circle ");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("-square ");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("(makepoint {} {}) ", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                println!("{}",end_components);
                end_components = "".to_string();
            },
            Def::IntersectsDef(id1,id2) => {
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id1;     
                let mut end1 : String = "".to_string();     
                let mut end2 : String = "".to_string();         
                
                print!("(intersects");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("-circle");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned();
                                end1 = end1 + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("-square");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                 let new_comp = tmp_var_br.to_owned();
                                end1 = end1 + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {

                                builder += &String::from("(makepoint ");
                                tmp_var_br = pnt_c1;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(" ");
                                tmp_var_br = pnt_c2;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(")");

                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
                
                id_ref = id2;                
                itr = 0;
                reached_point = false;
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("-circle {} {} ",builder,end1);

                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                 let new_comp = tmp_var_br.to_owned();
                                end2 = end2 + &new_comp.to_owned(); 
                                builder = "".to_string();

                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("-square {} {} ",builder,end1);
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                 let new_comp = tmp_var_br.to_owned();
                                end2 = end2 + &new_comp.to_owned(); 
                                builder = "".to_string();
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                builder += &String::from("(makepoint ");
                                tmp_var_br = pnt_c1;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(" ");
                                tmp_var_br = pnt_c2;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(") ");

                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                end_components = builder.to_owned() +  &end2.to_owned();
                println!("{})",end_components);
                end_components = "".to_string();
                builder = "".to_string();
            },
            Def::ContainedDef(id1,id2) => {
                
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id1;     
                let mut end1 : String = "".to_string();     
                let mut end2 : String = "".to_string();         
                
                print!("(contained");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("-circle");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                 let new_comp = tmp_var_br.to_owned();
                                end1 = end1 + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("-square");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                 let new_comp = tmp_var_br.to_owned();
                                end1 = end1 + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {

                                builder += &String::from("(makepoint ");
                                tmp_var_br = pnt_c1;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(" ");
                                tmp_var_br = pnt_c2;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(")");

                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
                
                id_ref = id2;                
                itr = 0;
                reached_point = false;
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("-circle {} {} ",builder,end1);

                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                 let new_comp = tmp_var_br.to_owned();
                                end2 = end2 + &new_comp.to_owned(); 
                                builder = "".to_string();

                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("-square {} {} ",builder,end1);
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                 let new_comp = tmp_var_br.to_owned();
                                end2 = end2 + &new_comp.to_owned(); 
                                builder = "".to_string();
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                builder += &String::from("(makepoint ");
                                tmp_var_br = pnt_c1;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(" ");
                                tmp_var_br = pnt_c2;
                                builder += &tmp_var_br.to_owned();
                                builder += &String::from(") ");

                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                end_components = builder.to_owned() +  &end2.to_owned();
                println!("{})",end_components);
                end_components = "".to_string();
                builder = "".to_string();
            },
            _ => (),
        }

        i+=1;
    }
    println!("(exit)");
}


//take vector of defs and turn components into prolog syntax
fn make_prolog_syntax(components: &Vec<Def>){
    let components_len = components.len();
    let mut i = 0;
    let mut tmp_var_br;
    let mut end_components : String = "".to_string();

    while i < components_len {
        match &components[i]{
            Def::PrintDef(id) => {
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id;                

                print!("query(");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("circle(");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("square(");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("point2d({},{})", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                println!("{}).",end_components);
                end_components = "".to_string();
            },
            Def::IntersectsDef(id1,id2) => {
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id1;            
                
                print!("query(intersects(");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("circle(");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("square(");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("point2d({},{})", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                print!("{}), ",end_components);
                end_components = "".to_string();
                
                id_ref = id2;                
                itr = 0;
                reached_point = false;
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("circle(");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("square(");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("point2d({},{})", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                println!("{})).",end_components);
                end_components = "".to_string();
            },
            Def::ContainedDef(id1,id2) => {
                let mut itr = 0;
                let mut reached_point = false;
                let mut id_ref = id1;         
                
                print!("query(contained(");
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("circle(");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("square(");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("point2d({},{})", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                print!("{}), ",end_components);
                end_components = "".to_string();
                
                id_ref = id2;                
                itr = 0;
                reached_point = false;
                while itr < components_len && (reached_point == false){
                    match &components[itr]{
                        Def::CircleDef(cir_id, cir_c1, cir_c2) => {
                            if cir_id == id_ref {
                                print!("circle(");
                                id_ref = cir_c1;
                                itr = 0;
                                tmp_var_br = cir_c2;
                                
                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::SquareDef(sqr_id, sqr_c1, sqr_c2) => {
                            if sqr_id == id_ref {
                                print!("square(");
                                id_ref = sqr_c1;
                                itr = 0;
                                tmp_var_br = sqr_c2;

                                let new_comp = tmp_var_br.to_owned() + &String::from(")");
                                end_components +=  &String::from(", ");
                                end_components = end_components + &new_comp.to_owned(); 
                            }
                            else{
                                itr+=1;
                            }
                        },
                        Def::PointDef(pnt_id, pnt_c1, pnt_c2) =>{
                            
                            if pnt_id == id_ref {
                                print!("point2d({},{})", pnt_c1.to_string(),pnt_c2.to_string());
                                //end of print, 'point' obj reached
                                reached_point = true;
                            }
                            else{
                                itr+=1;
                            }
                        }
                        _ => (),
                    }
                }
            
                println!("{})).",end_components);
                end_components = "".to_string();
            },
            _ => (),
        }

        i+=1;
    }

    println!("writeln(T) :- write(T), nl. \nmain:- forall(query(Q), Q-> (writeln(\'yes\')) ; (writeln(\'no\'))),
          halt.");
}

fn main() {
    //get the arguments used, index 1 -> file name, index 2 -> flag (-s or -p)
    let args: Vec<String> = env::args().collect();
    
    //concat. ./ to file name
    let filename = String::from("./") + &args[1];

    //get flag value
    let flag = &args[2];

    //open file, read data vector
    let mut file = std::fs::File::open(filename).unwrap();
    let mut data_tmp = String::new();
    file.read_to_string(&mut data_tmp).unwrap();
    
    if flag == "-s"{
        println!("; processing input file {}", &args[1]);
    }
    else if flag == "-p"{
        println!("/* processing input file {}", &args[1]);
    }

    //split string of file content to components (by whitespace) 
    let split = data_tmp.split_whitespace();
    let _data: Vec<&str> = split.collect();
    
    //generate all program tokens -> lex analyzer
    let tkn: Vec<Token> = token_scanner(data_tmp);

    //generate all sentences -> syntax analyzer 
    let defs: Vec <Def> = syntax_scanner(&tkn);

    if flag == "-s"{
        println!("; Lexical and Syntax analysis passed");
    }
    else if flag == "-p"{
        println!("   Lexical and Syntax analysis passed */");
    }

    if flag == "-s"{
        make_scheme_syntax(&defs);
    }

    if flag == "-p"{
        make_prolog_syntax(&defs);
    }
}