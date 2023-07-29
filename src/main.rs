use std::collections::HashMap;


enum TypeVar {
    Integer(usize),
    Float(f64),
    String(String),
    Array(Vec<Box<TypeVar>>)
}

impl TypeVar {

    fn parse_string(content: String) -> Result<TypeVar, String> {
        let first_quote = content.find('"');
        let last_quote = content.rfind('"');
        match (first_quote, last_quote) {
            (Some(first), Some(last)) => 
                if first == last {
                    Err(format!("Couldn't find ending quote for string: {}", content))
                } else {
                    Ok(TypeVar::String(content[first..last].to_owned()))
                },
            _ => Err(format!("Invalid format for a string: {}", content))
        }
    }

    fn parse_float(content: String) -> Result<TypeVar, String> {
        let trimmed_content = content.trim()[1..].to_owned();
        match trimmed_content.parse::<f64>() {
            Ok(v) => Ok(TypeVar::Float(v)),
            Err(_) => Err(format!("Invalid float: {}", content))
        }
    }

    fn parse_int(content: String) -> Result<TypeVar, String> {
        match content.trim().parse::<usize>() {
            Ok(v) => Ok(TypeVar::Integer(v)),
            Err(_) => Err(format!("Invalid float: {}", content))
        }
    }

    fn make_array(content: &str) -> Result<TypeVar, String> {
        let results: Vec<Result<TypeVar, String>> = content.split(',')
            .map(|v| v.trim().to_string())
            .map(|v| Self::parse_type_var(v))
            .collect();
        
        for result in &results {
            if let Err(value) = result {
                return Err(value.clone())
            }
        }

        let foo: Vec<Box<TypeVar>> = results.into_iter()
            .filter_map(|v| v.ok())
            .map(|v| Box::new(v)) 
            .collect();

        Ok(TypeVar::Array(foo))
    }

    fn parse_list(content: String) -> Result<TypeVar, String> {
        let first_bracket = content.find('[');
        let last_bracket = content.rfind(']');
        match (first_bracket, last_bracket) {
            (Some(first), Some(last)) => 
                if first == last {
                    Err(format!("Couldn't find ending quote for string: {}", content))
                } else {
                    Self::make_array(&content[first..last])
                },
            _ => Err(format!("Invalid format for a string: {}", content))
        }
    }

    fn check_type_parse(first_letter: char, content: String) -> Result<TypeVar, String> {
        match first_letter {
            '"' => Self::parse_string(content),
            'f' => Self::parse_float(content),
            '[' => Self::parse_list(content),
            value if value.is_numeric() => { Self::parse_int(content) },
            _ => Ok(TypeVar::Integer(0))
        }
    }

    pub fn parse_type_var(content: String) -> Result<TypeVar, String> {
        let stripped_content = content.trim().clone().to_string();
        match stripped_content.chars().nth(0) {
            Some(c) => Self::check_type_parse(c, content),
            None => Err("No value specified. ".to_owned())
        }
    }

}

struct Assignment {
    var_name: String,
    value: TypeVar
}

impl Assignment {

    pub fn parse_assignment(content: String) -> Result<Assignment, String> {

        Ok(Assignment { var_name: "".to_owned(), value: TypeVar::Integer(0) })
    }

}

struct MethodCall {
    method_name: String,
    args: Vec<TypeVar>,
    ret: Option<TypeVar>
}

impl MethodCall {
    
    pub fn parse_method_call(content: String) -> Result<MethodCall, String> {

        Ok(MethodCall { method_name:  "".to_string(), args: vec![], ret: None })
    }

}

enum LineContent {
    Comment(String),
    Assignment(Assignment),
    MethodCall(MethodCall)
}

impl LineContent {
    
    pub fn parse_content(content: &str) -> Result<LineContent, String> {


        Ok(LineContent::Comment("".to_owned()))
    }

}

struct Line {
    number: usize,
    content: LineContent
}

impl Line {

    pub fn parse_line(line: String) -> Result<Line, String> {
        let space_index = match line.find(' ') {
            Some(i) => i,
            None => return Err("Could not find line number. ".to_owned())
        };
        let (number, content) = line.split_at(space_index);
        let line_no = match number.parse::<usize>() {
            Ok(i) => i,
            Err(_) => return Err(format!("Failed to parse line number: {}", number))
        };
        let line_content = match LineContent::parse_content(content) {
            Ok(i) => i,
            Err(v) => return Err(v)
        };

        Ok(Line { number: line_no, content: line_content })
    }
}

struct Method {
    lines: HashMap<usize, Line>,

}


fn main() {
    let variables: HashMap<String, TypeVar> = HashMap::new();
    
    
    println!("Hello, world!");
}
