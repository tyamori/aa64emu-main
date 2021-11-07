use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::peek,
    error::ErrorKind,
    IResult,
};

/// AArch64レジスタ
#[derive(Debug)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
}

/// 算術演算命令
#[derive(Debug)]
pub enum ArithOpcode {
    Add,
    Sub,
    Mul,
    Div,
}

/// 分岐命令
#[derive(Debug)]
pub enum BranchOpcode {
    Beq, // ==
    Blt, // <
    Bgt, // >
}

/// 命令
#[derive(Debug)]
pub enum Op {
    Mov(Register, RegOrNum),                          // mov命令
    Cmp(Register, Register),                          // 比較命令
    Arith(ArithOpcode, Register, Register, Register), // 算術演算命令
    Branch(BranchOpcode, u64),                        // 分岐命令
}

/// レジスタか数値を表す型
#[derive(Debug)]
pub enum RegOrNum {
    Reg(Register), // レジスタ
    Num(u64),      // 数値
}

pub fn parse_asm(input: &str) -> IResult<&str, Vec<Op>> {
    let mut v = Vec::new();
    for line in input.lines() {
        let (i, _) = multispace0(line)?;
        // 空行をスキップ
        if i.is_empty() {
            continue;
        }

        // 命令を判別
        let (i, val) = alt((
            tag("mov"),
            tag("cmp"),
            tag("add"),
            tag("sub"),
            tag("mul"),
            tag("div"),
            tag("b.eq"),
            tag("b.lt"),
            tag("b.gt"),
        ))(i)?;

        let (i, op) = match val {
            "mov" => parse_mov(i)?, // mov命令
            "cmp" => parse_cmp(i)?, // 比較命令
            "add" | "sub" | "mul" | "div" => {
                // 算術演算命令
                let opcode = get_aop(val).unwrap();
                parse_arith(opcode, i)?
            }
            _ => {
                // 分岐命令
                let opcode = get_brop(val).unwrap();
                parse_branch(opcode, i)?
            }
        };

        let (i, _) = multispace0(i)?;
        if !i.is_empty() {
            // 行末に余計な文字列がある場合エラー
            let err = nom::error::Error::new(i, ErrorKind::Eof);
            return Err(nom::Err::Failure(err));
        }

        v.push(op);
    }

    Ok(("", v))
}

/// 算術演算命令のオペランドを解釈
pub fn get_aop(op: &str) -> Option<ArithOpcode> {
    match op {
        "add" => Some(ArithOpcode::Add),
        "sub" => Some(ArithOpcode::Sub),
        "mul" => Some(ArithOpcode::Mul),
        "div" => Some(ArithOpcode::Div),
        _ => None,
    }
}

/// 算術演算命令をパース
pub fn parse_arith(opcode: ArithOpcode, i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?; // レジスタ

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?; // カンマ
    let (i, _) = multispace0(i)?;

    let (i, reg2) = parse_reg(i)?; // レジスタ

    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?; // カンマ
    let (i, _) = multispace0(i)?;
    let (i, reg3) = parse_reg(i)?; // レジスタ

    Ok((i, Op::Arith(opcode, reg1, reg2, reg3)))
}

/// 分岐命令を解釈
pub fn get_brop(op: &str) -> Option<BranchOpcode> {
    match op {
        "b.eq" => Some(BranchOpcode::Beq),
        "b.lt" => Some(BranchOpcode::Blt),
        "b.gt" => Some(BranchOpcode::Bgt),
        _ => None,
    }
}

/// 分岐命令をパース
pub fn parse_branch(opcode: BranchOpcode, i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    // #123 というような即値をパース
    let (i, _) = char('#')(i)?;
    let (i, n) = digit1(i)?;
    Ok((i, Op::Branch(opcode, n.parse().unwrap())))
}

/// mov命令をパース
pub fn parse_mov(i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?; // レジスタ
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?; // カンマ
    let (i, _) = multispace0(i)?;

    // 1文字先読みして即値かレジスタかを判定
    let (i, c) = peek(alt((char('#'), char('x'))))(i)?;

    if c == '#' {
        // 即値
        let (i, _) = char('#')(i)?;
        let (i, n) = digit1(i)?;
        Ok((i, Op::Mov(reg1, RegOrNum::Num(n.parse().unwrap()))))
    } else {
        // レジスタ
        let (i, reg2) = parse_reg(i)?;
        Ok((i, Op::Mov(reg1, RegOrNum::Reg(reg2))))
    }
}

/// 比較命令をパース
pub fn parse_cmp(i: &str) -> IResult<&str, Op> {
    let (i, _) = multispace1(i)?;
    let (i, reg1) = parse_reg(i)?; // レジスタ
    let (i, _) = multispace0(i)?;
    let (i, _) = char(',')(i)?; // カンマ
    let (i, _) = multispace0(i)?;
    let (i, reg2) = parse_reg(i)?; // レジスタ
    Ok((i, Op::Cmp(reg1, reg2)))
}

/// レジスタをパース
fn parse_reg(i: &str) -> IResult<&str, Register> {
    let (i1, val) = alt((
        alt((
            tag("x10"),
            tag("x11"),
            tag("x12"),
            tag("x13"),
            tag("x14"),
            tag("x15"),
            tag("x16"),
            tag("x17"),
            tag("x18"),
            tag("x19"),
            tag("x20"),
            tag("x21"),
            tag("x22"),
            tag("x23"),
            tag("x24"),
            tag("x25"),
            tag("x26"),
            tag("x27"),
            tag("x28"),
            tag("x29"),
            tag("x30"),
        )),
        alt((
            tag("x0"),
            tag("x1"),
            tag("x2"),
            tag("x3"),
            tag("x4"),
            tag("x5"),
            tag("x6"),
            tag("x7"),
            tag("x8"),
            tag("x9"),
        )),
    ))(i)?;

    match val {
        "x0" => Ok((i1, Register::X0)),
        "x1" => Ok((i1, Register::X1)),
        "x2" => Ok((i1, Register::X2)),
        "x3" => Ok((i1, Register::X3)),
        "x4" => Ok((i1, Register::X4)),
        "x5" => Ok((i1, Register::X5)),
        "x6" => Ok((i1, Register::X6)),
        "x7" => Ok((i1, Register::X7)),
        "x8" => Ok((i1, Register::X8)),
        "x9" => Ok((i1, Register::X9)),
        "x10" => Ok((i1, Register::X10)),
        "x11" => Ok((i1, Register::X11)),
        "x12" => Ok((i1, Register::X12)),
        "x13" => Ok((i1, Register::X13)),
        "x14" => Ok((i1, Register::X14)),
        "x15" => Ok((i1, Register::X15)),
        "x16" => Ok((i1, Register::X16)),
        "x17" => Ok((i1, Register::X17)),
        "x18" => Ok((i1, Register::X18)),
        "x19" => Ok((i1, Register::X19)),
        "x20" => Ok((i1, Register::X20)),
        "x21" => Ok((i1, Register::X21)),
        "x22" => Ok((i1, Register::X22)),
        "x23" => Ok((i1, Register::X23)),
        "x24" => Ok((i1, Register::X24)),
        "x25" => Ok((i1, Register::X25)),
        "x26" => Ok((i1, Register::X26)),
        "x27" => Ok((i1, Register::X27)),
        "x28" => Ok((i1, Register::X28)),
        "x29" => Ok((i1, Register::X29)),
        "x30" => Ok((i1, Register::X30)),
        _ => {
            let err = nom::error::Error::new(i, ErrorKind::Fail);
            Err(nom::Err::Failure(err))
        }
    }
}
