#[derive(Debug)]
enum WasmExpr {
    WasmOp((String, Vec<WasmExpr>)),
    WasmName(String),
    WasmGlobal(String),
    WasmString(String),
    WasmNum(i64),
}

impl WasmExpr {
    fn serialize(&self, depth: usize) -> String {
        match self {
            WasmExpr::WasmOp((name, args)) => format!("\n{}({} {})", " ".repeat(depth), name, args.iter().map(|v| v.serialize(depth + 2)).collect::<Vec<String>>().join(" ")),
            WasmExpr::WasmName(name) => name.clone(),
            WasmExpr::WasmGlobal(name) => name.clone(),
            WasmExpr::WasmString(buf) => format!("\"{}\"", buf),
            WasmExpr::WasmNum(num) => format!("{}", num)
        }
    }
}

fn maxslice(s: &str) -> String {
    s[..usize::min(s.len(), 20)].to_owned()
}

fn is_whitespace(c: char) -> bool { c == ' ' || c == '\n' || c == '\t' }
fn is_digit(c: char) -> bool { c.is_digit(10) }
fn is_num_minus(c: char) -> bool { is_digit(c) || c == '-' }
fn is_alpha(c: char) -> bool { c.is_alphabetic() || c == '_' }
fn is_alnum(c: char) -> bool { is_alpha(c) || is_num_minus(c) }
fn is_name(c: char) -> bool { is_alnum(c) || c == '.' || c == '=' }
fn is_global(c: char) -> bool { is_alpha(c) || is_digit(c) || c == '$' }
fn is_hex(c: char) -> bool { c.is_digit(16) }

fn parse_name(expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with(is_alpha) { Err(format!("\"{}\" is not a name", expr))? }
    for (i, c) in expr.chars().enumerate() {
        if !is_name(c) {
            return Ok((WasmExpr::WasmName(
                        expr.chars().take(i).collect::<String>()),
                &expr[i..]));
        }
    }
    Err("reached end of expression while parsing name".to_owned())
}

fn parse_global(expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with('$') { Err(format!("\"{}\" is not a global", expr))? }
    for (i, c) in expr.chars().enumerate() {
        if !is_global(c) {
            return Ok((WasmExpr::WasmGlobal(
                        expr.chars().take(i).collect::<String>()),
                &expr[i..]));
        }
    }
    Err("reached end of expression while parsing global".to_owned())
}

fn parse_num(mut expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with(is_num_minus) { Err(format!("\"{}\" is not a number", expr))? }
    let is_neg = expr.starts_with('-');
    if is_neg {
        expr = &expr[1..];
    }
    for (i, c) in expr.chars().enumerate() {
        if !is_digit(c) {
            return Ok((WasmExpr::WasmNum(
                        expr[..i].parse::<i64>().map(|v| if is_neg { -v } else { v }).unwrap()),
                &expr[i..]));
        }
    }
    Err("reached end of expression while parsing number".to_owned())
}

fn parse_string<'a>(expr: &'a str) -> Result<(WasmExpr, &'a str), String> {
    if !expr.starts_with('"') { Err(format!("\"{}\" is not a string", expr))? }
    let expr = &expr[1..];
    let mut escape = 0;
    for (i, c) in expr.chars().enumerate() {
        if escape == 1 {
            if c == 'n' || c == 't' || c == '"' || c == '\'' || c == '\\' { escape = 0; }
            else if is_hex(c) { escape = 2; }
            else { Err(format!("unknown escape character \"{}\"", c))? }
        } else if escape == 2 {
            escape = 0;
        } else {
            if c == '\\' { escape = 1; }
            else if c == '"' { return Ok((WasmExpr::WasmString(expr[..i].to_owned()),
                                    &expr[i+1..])); }
        }
    }
    Err("reached end of expression while parsing string".to_owned())
}

fn parse_op<'a>(expr: &'a str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with('(') { Err(format!("\"{}\" is not an operation", expr))? }
    let off_expr: &'a str = &expr[1..];
    let (name, mut args): (_, &'a str) = parse_name(off_expr)?;
    let mut ops = Vec::new();
    while !args.starts_with(')') {
        let (arg, _args) = parse_expr(args)?;
        args = _args;
        ops.push(arg);
        args = args.trim_start_matches(is_whitespace);
    }
    if let WasmExpr::WasmName(name) = name {
        Ok((WasmExpr::WasmOp((name, ops)), &args[1..] as &'a str))
    } else { Err(format!("operation name \"{}\" is not valid", expr))? }
}

fn parse_line_comment(expr: &str) -> Result<&str, String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with(";;") { Err(format!("\"{}\" is not a comment", expr))? }
    if let Some(i) = expr.find("\n") {  
        Ok(&expr[i..])
    } else {
        Err(format!("reached end of expression while parsing \";;\" comment"))
    }
}
fn parse_comment(expr: &str) -> Result<&str, String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with("(;") { Err(format!("\"{}\" is not a comment", expr))? }
    if let Some(i) = expr.find(";)") {
        Ok(&expr[i+2..])
    } else {
        Err(format!("reached end of expression while parsing \"(;\" comment"))
    }
}

fn parse_expr(expr: &str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if expr.starts_with("(;") {
        parse_expr(parse_comment(expr)?)
    } else if expr.starts_with(";;") {
        parse_expr(parse_line_comment(expr)?)
    } else if expr.starts_with("(") {
        parse_op(expr)
    } else if expr.starts_with("$") {
        parse_global(expr)
    } else if expr.starts_with("\"") {
        parse_string(expr)
    } else if expr.starts_with(is_num_minus) {
        parse_num(expr)
    } else if expr.starts_with(is_alpha) {
        parse_name(expr)
    } else {
        Err(format!("invalid expression ´{}´", maxslice(&expr)))
    }
}

fn parse_wasm(expr: String) -> Result<WasmExpr, String> {
    parse_expr(&expr).map(|(x, _)| x)
}

fn main() {
    if let Some(arg) = std::env::args().skip(1).next().as_ref() {
        let wasm = std::fs::read_to_string(arg).unwrap();
        //println!("{:?}", parse_wasm(wasm));
        let wasm = parse_wasm(wasm).unwrap();
        let wasm = wasm.serialize(0);

        std::fs::write(arg, wasm).unwrap();
    } else {
        eprintln!("error: needing one file argument");
    }
}
