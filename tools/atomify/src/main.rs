#[derive(Debug, PartialEq, Clone)]
enum WasmExpr<'a> {
    WasmOp(&'a str, Vec<WasmExpr<'a>>),
    WasmName(&'a str),
    WasmGlobal(&'a str),
    WasmString(&'a str),
    WasmNum(i64),
    WasmNone,
}

impl<'a> WasmExpr<'a> {
    fn serialize(&self, depth: usize) -> String {
        match self {
            WasmExpr::WasmOp(name, args) => format!(
                "\n{}({} {})",
                " ".repeat(depth),
                name,
                args.iter()
                    .map(|v| v.serialize(depth + 2))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            WasmExpr::WasmName(name) => name.to_string(),
            WasmExpr::WasmGlobal(name) => name.to_string(),
            WasmExpr::WasmString(buf) => format!("\"{}\"", buf),
            WasmExpr::WasmNum(num) => format!("{}", num),
            WasmExpr::WasmNone => "".to_owned(),
        }
    }
}

fn maxslice(s: &str, b: usize, e: usize) -> &str {
    let sz = s.len();
    if b >= sz || e <= b {
        &s[0..0]
    } else if e >= sz {
        &s[b..]
    } else {
        &s[b..e]
    }
}

fn compile_error(s: &str, len: usize) -> String {
    compile_error_off(s, 0, len)
}

fn compile_error_off(s: &str, off: usize, len: usize) -> String {
    format!(
        "\x1b[m\n```\n{}\x1b[31;1m{}\x1b[m{}\n```",
        maxslice(s, 0, off),
        maxslice(s, off, off + len),
        maxslice(s, len + off, off + len + 140)
    )
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\t'
}
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}
fn is_num_minus(c: char) -> bool {
    is_digit(c) || c == '-'
}
fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}
fn is_alnum(c: char) -> bool {
    is_alpha(c) || is_num_minus(c)
}
fn is_name(c: char) -> bool {
    is_alnum(c) || c == '.' || c == '='
}
fn is_global(c: char) -> bool {
    is_alpha(c) || is_digit(c) || c == '$'
}
fn is_hex(c: char) -> bool {
    c.is_digit(16)
}

fn parse_name(expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with(is_alpha) {
        Err(format!("\"{}\" is not a name", compile_error(expr, 1)))?
    }
    for (i, c) in expr.chars().enumerate() {
        if !is_name(c) {
            return Ok((WasmExpr::WasmName(&expr[..i]), &expr[i..]));
        }
    }
    Err("reached end of expression while parsing name".to_owned())
}

fn parse_global(expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with('$') {
        Err(format!("\"{}\" is not a global", compile_error(expr, 1)))?
    }
    for (i, c) in expr.chars().enumerate() {
        if !is_global(c) {
            return Ok((WasmExpr::WasmGlobal(&expr[..i]), &expr[i..]));
        }
    }
    Err("reached end of expression while parsing global".to_owned())
}

fn parse_num(mut expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with(is_num_minus) {
        Err(format!("\"{}\" is not a number", compile_error(expr, 1)))?
    }
    let is_neg = expr.starts_with('-');
    if is_neg {
        expr = &expr[1..];
    }
    for (i, c) in expr.chars().enumerate() {
        if !is_digit(c) {
            return Ok((
                WasmExpr::WasmNum(
                    expr[..i]
                        .parse::<i64>()
                        .map(|v| if is_neg { -v } else { v })
                        .unwrap(),
                ),
                &expr[i..],
            ));
        }
    }
    Err("reached end of expression while parsing number".to_owned())
}

fn parse_string(expr: &str) -> Result<(WasmExpr, &str), String> {
    if !expr.starts_with('"') {
        Err(format!("\"{}\" is not a string", compile_error(expr, 1)))?
    }
    let expr = &expr[1..];
    let mut escape = 0;
    for (i, c) in expr.chars().enumerate() {
        if escape == 1 {
            if c == 'n' || c == 't' || c == '"' || c == '\'' || c == '\\' {
                escape = 0;
            } else if is_hex(c) {
                escape = 2;
            } else {
                Err(format!(
                    "unknown escape character{}",
                    compile_error_off(expr, i - 1, 2)
                ))?
            }
        } else if escape == 2 {
            escape = 0;
        } else {
            if c == '\\' {
                escape = 1;
            } else if c == '"' {
                return Ok((WasmExpr::WasmString(&expr[..i]), &expr[i + 1..]));
            }
        }
    }
    Err("reached end of expression while parsing string".to_owned())
}

fn parse_op(expr: &str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with('(') {
        Err(format!(
            "\"{}\" is not an operation",
            compile_error(expr, 1)
        ))?
    }
    let off_expr: &str = &expr[1..];
    let (name, mut args): (_, &str) = parse_name(off_expr)?;
    let mut ops = Vec::new();
    args = args.trim_start_matches(is_whitespace);
    while !args.starts_with(')') {
        let (arg, _args) = parse_expr(args)?;
        args = _args;
        if arg != WasmExpr::WasmNone {
            ops.push(arg);
        }
        args = args.trim_start_matches(is_whitespace);
    }
    if let WasmExpr::WasmName(name) = name {
        Ok((WasmExpr::WasmOp(name, ops), &args[1..] as &str))
    } else {
        Err(format!("operation name \"{}\" is not valid", expr))?
    }
}

fn parse_block_comment(expr: &str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with("(;") {
        Err(format!(
            "\"{}\" is not a block comment",
            compile_error(expr, 2)
        ))?
    }
    if let Some(i) = expr.find(";)") {
        Ok((WasmExpr::WasmNone, &expr[i + 2..]))
    } else {
        Err(format!(
            "reached end of expression while parsing \"(;\" comment{}",
            compile_error(expr, expr.len())
        ))
    }
}

fn parse_line_comment(expr: &str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if !expr.starts_with(";;") {
        Err(format!(
            "\"{}\" is not a line comment",
            compile_error(expr, 2)
        ))?
    }
    if let Some(i) = expr.find("\n") {
        Ok((WasmExpr::WasmNone, &expr[i..]))
    } else {
        Err(format!(
            "reached end of expression while parsing \";;\" comment{}",
            compile_error(expr, expr.len())
        ))
    }
}

fn parse_expr(expr: &str) -> Result<(WasmExpr, &str), String> {
    let expr = expr.trim_matches(is_whitespace);
    if expr.starts_with("(;") {
        parse_block_comment(expr)
    } else if expr.starts_with(";;") {
        parse_line_comment(expr)
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
        Err(format!(
            "expecting expression (`(;`, `;;`, `(`, `$`, LB, ...), got{}",
            compile_error(&expr, 2)
        ))
    }
}

fn parse_wasm(expr: &str) -> Result<WasmExpr, String> {
    parse_expr(expr).map(|(x, _)| x)
}

fn atomify(mut expr: WasmExpr) -> Result<WasmExpr, String> {
    match &mut expr {
        WasmExpr::WasmOp("module", ref mut args) => {
            let (mut atomic_read, mut atomic_write) = (None, None);
            let mut removable_exports = Vec::with_capacity(2);
            for (i, arg) in args.iter().enumerate() {
                match arg {
                    WasmExpr::WasmOp("export", export_args) => match export_args.as_slice() {
                        [WasmExpr::WasmString("atomic_read"), WasmExpr::WasmOp("func", fun_args)] => {
                            if let [WasmExpr::WasmNum(fun_id)] = fun_args.as_slice() {
                                atomic_read = Some(*fun_id);
                                removable_exports.push(i);
                            }
                        }
                        [WasmExpr::WasmString("atomic_write"), WasmExpr::WasmOp("func", fun_args)] => {
                            if let [WasmExpr::WasmNum(fun_id)] = fun_args.as_slice() {
                                atomic_write = Some(*fun_id);
                                removable_exports.push(i);
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
                if let (Some(_), Some(_)) = (atomic_read, atomic_write) {
                    break;
                }
            }
            removable_exports.reverse();
            if removable_exports.len() != 2 {
                println!("warning: could not remove exports");
            }
            for i in removable_exports {
                args.remove(i);
            }
            if let (Some(atomic_read), Some(atomic_write)) = (atomic_read, atomic_write) {
                let mut function_counter = 0;
                for ref mut arg in args.iter_mut() {
                    match arg {
                        WasmExpr::WasmOp("import", ref import_args) => {
                            if let [WasmExpr::WasmString(_scope), WasmExpr::WasmString(_import_name), WasmExpr::WasmOp("func", _fun_args)] =
                                import_args.as_slice()
                            {
                                function_counter += 1
                            }
                        }
                        WasmExpr::WasmOp("func", ref mut fun_args) => {
                            if function_counter == atomic_read {
                                let op_ptr: &mut WasmExpr = match fun_args.last_mut() {
                                    Some(v) => v,
                                    _ => continue,
                                };
                                if let WasmExpr::WasmOp("i32.load8_u", op_args) = op_ptr {
                                    *op_ptr =
                                        WasmExpr::WasmOp("i32.atomic.load8_u", op_args.clone())
                                }
                            } else if function_counter == atomic_write {
                                let op_ptr: &mut WasmExpr = match fun_args.last_mut() {
                                    Some(v) => v,
                                    _ => continue,
                                };
                                if let WasmExpr::WasmOp("i32.store8", op_args) = op_ptr {
                                    *op_ptr = WasmExpr::WasmOp("i32.atomic.store8", op_args.clone())
                                }
                            }
                            function_counter += 1;
                        }
                        _ => (),
                    }
                }
                Ok(())
            } else {
                Err("expecting \"atomic_read\" and \"atomic_write\" exports".to_owned())
            }
        }
        _ => Err("expecting \"module\" s-expression at top level".to_owned()),
    }?;
    Ok(expr)
}

fn main() {
    if let Some(arg) = std::env::args().nth(1).as_ref() {
        let res = std::fs::read_to_string(arg)
            .as_ref()
            .map_err(|e| format!("failed to read error ('{}'): \"{}\"", arg, e))
            .and_then(|content| parse_wasm(content))
            .and_then(atomify)
            .map(|wasm| wasm.serialize(0))
            .and_then(|wasm| {
                std::fs::write(arg, wasm)
                    .map_err(|e| format!("failed to write error ('{}'): \"{}\"", arg, e))
            });
        if let Err(e) = res {
            eprintln!("\x1b[31;1merror:\x1b[m {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("error: needing one file argument");
        std::process::exit(1);
    }
}
