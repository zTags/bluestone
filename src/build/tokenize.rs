use std::fs;

pub struct Token {
    line: i32,
    defines: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, defines: {}", self.line, self.defines)
    }
}


pub fn tokenize(file: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    print!("\u{001b}[32mStarting tokenizing...\u{001b}[0m\n");
    let fileContents = fs::read_to_string(file).expect("Reading file failed, you use it exists?");

    // TODO clean this stuff up
    let fileLines = fileContents.split("\n");
    let mut currentToken = Token {line: 0, defines: "".to_string()};
    let mut linen = 0;
    for line in fileLines {
        if line.starts_with("func") {
            let mut iter = 0;
            let mut isCompileTime = true;
            for word in line.split(" ") {

                if iter == 1 {
                    if word.ends_with("!") {
                        isCompileTime = false;
                    }
                }
                iter = iter + 1;
            }

            if isCompileTime {
                currentToken = Token {
                    line: linen,
                    defines: "compiletimefn".to_string()
                }
            } else {
                currentToken = Token {
                line: linen,
                defines: "runtimefn".to_string()
                }
            }

            tokens.push(currentToken);

            currentToken = Token {
                line: 0,
                defines: "".to_string(),            
            }
        }
        linen = linen + 1;
    }

    return tokens;
}
