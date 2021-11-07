use super::parser::{ArithOpcode, BranchOpcode, Op, RegOrNum, Register};
use core::panic;
use std::cmp::Ordering;

/// 比較命令実行後の結果
#[derive(Debug, Eq, PartialEq)]
pub enum Condition {
    Eq, // ==
    Lt, // <
    Gt, // >
}

/// AArch64レジスタ
#[derive(Debug)]
pub struct Context {
    pub cond: Condition,
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
}

/// 実行
pub fn run(ops: &[Op]) -> Context {
    // レジスタの初期化
    let mut ctx = Context::new();

    let mut pc = 0; // プログラムカウンタ
    loop {
        match pc.cmp(&ops.len()) {
            Ordering::Equal => return ctx,
            Ordering::Greater => panic!("invalid PC"),
            _ => (),
        }

        // オペコードの種類によって実行する処理を切り替える
        match &ops[pc] {
            Op::Mov(dst, src) => {
                eval_mov(&mut ctx, dst, src);
            }
            Op::Cmp(reg1, reg2) => {
                eval_cmp(&mut ctx, reg1, reg2);
            }
            Op::Arith(opcode, reg1, reg2, reg3) => {
                // ここを実装
                // 次のような関数を定義して実装せよ
                // eval_arith(&mut ctx, opcode, reg1, reg2, reg3);
                // todo!()は削除すること
                todo!()
            }
            Op::Branch(opcode, line) => {
                if eval_branch(&ctx, opcode) {
                    pc = *line as usize;
                    continue;
                }
            }
        }

        pc += 1; // 1つ次のアセンブリを実行
    }
}

/// 比較命令を実行
fn eval_cmp(ctx: &mut Context, reg1: &Register, reg2: &Register) {
    // ここを実装
    // todo!()は削除すること
    todo!()
}

/// mov命令を実行
fn eval_mov(ctx: &mut Context, dst: &Register, src: &RegOrNum) {
    match src {
        RegOrNum::Num(n) => {
            ctx.set_reg(dst, *n);
        }
        RegOrNum::Reg(r) => {
            let n = ctx.get_reg(r);
            ctx.set_reg(dst, n);
        }
    }
}

/// 分岐命令を実行
fn eval_branch(ctx: &Context, opcode: &BranchOpcode) -> bool {
    match opcode {
        BranchOpcode::Beq => ctx.cond == Condition::Eq,
        BranchOpcode::Blt => ctx.cond == Condition::Lt,
        BranchOpcode::Bgt => ctx.cond == Condition::Gt,
    }
}

/// コンテキストを生成、設定、取得するためのインターフェース
impl Context {
    fn new() -> Context {
        Context {
            cond: Condition::Eq,
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
        }
    }

    fn set_reg(&mut self, r: &Register, val: u64) {
        match r {
            Register::X0 => {
                self.x0 = val;
            }
            Register::X1 => {
                self.x1 = val;
            }
            Register::X2 => {
                self.x2 = val;
            }
            Register::X3 => {
                self.x3 = val;
            }
            Register::X4 => {
                self.x4 = val;
            }
            Register::X5 => {
                self.x5 = val;
            }
            Register::X6 => {
                self.x6 = val;
            }
            Register::X7 => {
                self.x7 = val;
            }
            Register::X8 => {
                self.x8 = val;
            }
            Register::X9 => {
                self.x9 = val;
            }
            Register::X10 => {
                self.x10 = val;
            }
            Register::X11 => {
                self.x11 = val;
            }
            Register::X12 => {
                self.x12 = val;
            }
            Register::X13 => {
                self.x13 = val;
            }
            Register::X14 => {
                self.x14 = val;
            }
            Register::X15 => {
                self.x15 = val;
            }
            Register::X16 => {
                self.x16 = val;
            }
            Register::X17 => {
                self.x17 = val;
            }
            Register::X18 => {
                self.x18 = val;
            }
            Register::X19 => {
                self.x19 = val;
            }
            Register::X20 => {
                self.x20 = val;
            }
            Register::X21 => {
                self.x21 = val;
            }
            Register::X22 => {
                self.x22 = val;
            }
            Register::X23 => {
                self.x23 = val;
            }
            Register::X24 => {
                self.x24 = val;
            }
            Register::X25 => {
                self.x25 = val;
            }
            Register::X26 => {
                self.x26 = val;
            }
            Register::X27 => {
                self.x27 = val;
            }
            Register::X28 => {
                self.x28 = val;
            }
            Register::X29 => {
                self.x29 = val;
            }
            Register::X30 => {
                self.x30 = val;
            }
        }
    }

    fn get_reg(&self, r: &Register) -> u64 {
        match r {
            Register::X0 => self.x0,
            Register::X1 => self.x1,
            Register::X2 => self.x2,
            Register::X3 => self.x3,
            Register::X4 => self.x4,
            Register::X5 => self.x5,
            Register::X6 => self.x6,
            Register::X7 => self.x7,
            Register::X8 => self.x8,
            Register::X9 => self.x9,
            Register::X10 => self.x10,
            Register::X11 => self.x11,
            Register::X12 => self.x12,
            Register::X13 => self.x13,
            Register::X14 => self.x14,
            Register::X15 => self.x15,
            Register::X16 => self.x16,
            Register::X17 => self.x17,
            Register::X18 => self.x18,
            Register::X19 => self.x19,
            Register::X20 => self.x20,
            Register::X21 => self.x21,
            Register::X22 => self.x22,
            Register::X23 => self.x23,
            Register::X24 => self.x24,
            Register::X25 => self.x25,
            Register::X26 => self.x26,
            Register::X27 => self.x27,
            Register::X28 => self.x28,
            Register::X29 => self.x29,
            Register::X30 => self.x30,
        }
    }
}
