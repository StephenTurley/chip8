use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    CLS,
    JMP(u16),
    LDVx { vx: usize, value: u8 },
    LDVxVy { vx: usize, vy: usize },
    ORVxVy { vx: usize, vy: usize },
    ANDVxVy { vx: usize, vy: usize },
    XORVxVy { vx: usize, vy: usize },
    ADDVxVy { vx: usize, vy: usize },
    SUB { vx: usize, vy: usize },
    SUBn { vx: usize, vy: usize },
    SHR { vx: usize, vy: usize },
    SHL { vx: usize, vy: usize },
    ADDVx { vx: usize, value: u8 },
    LDI(u16),
    DRW { vx: usize, vy: usize, n: usize },
    Unknown,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OpCode::CLS => write!(f, "CLS"),
            OpCode::JMP(addr) => write!(f, "JMP address:{:#06X}", addr),
            OpCode::LDVx { vx, value } => write!(f, "LD VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::LDVxVy { vx, vy } => write!(f, "LD VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::ORVxVy { vx, vy } => write!(f, "OR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::ANDVxVy { vx, vy } => write!(f, "AND VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::XORVxVy { vx, vy } => write!(f, "XOR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::ADDVxVy { vx, vy } => write!(f, "ADD VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::SUB { vx, vy } => write!(f, "SUB VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::SUBn { vx, vy } => write!(f, "SUBn VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::SHR { vx, vy } => write!(f, "SHR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::SHL { vx, vy } => write!(f, "SHL VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::ADDVx { vx, value } => write!(f, "ADD VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::LDI(value) => write!(f, "LDI value:{:#06X}", value),
            OpCode::DRW { vx, vy, n } => {
                write!(f, "DRW VX:{:#06X} VX:{:#06X} n:{:#06X}", vx, vy, n)
            }
            OpCode::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn decode(op: u16) -> OpCode {
    match op {
        0x00E0 => OpCode::CLS,
        _ => match op & 0xF000 {
            0x1000 => OpCode::JMP(op & 0x0FFF),
            0x6000 => OpCode::LDVx {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x7000 => OpCode::ADDVx {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x8000 => match op & 0x000F {
                0x0000 => OpCode::LDVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0001 => OpCode::ORVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0002 => OpCode::ANDVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0003 => OpCode::XORVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0004 => OpCode::ADDVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0005 => OpCode::SUB {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0006 => OpCode::SHR {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0007 => OpCode::SUBn {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x000E => OpCode::SHL {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                _ => OpCode::Unknown,
            },
            0xA000 => OpCode::LDI(op & 0x0FFF),
            0xD000 => OpCode::DRW {
                vx: ((op & 0x0F00) >> 8) as usize,
                vy: ((op & 0x00F0) >> 4) as usize,
                n: (op & 0x000F) as usize,
            },
            _ => OpCode::Unknown,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown() {
        let result = decode(0x0);
        assert_eq!(OpCode::Unknown, result);
    }

    #[test]
    fn cls() {
        let result = decode(0x00E0);
        assert_eq!(OpCode::CLS, result);
    }

    #[test]
    fn jmp() {
        let result = decode(0x1ABC);
        assert_eq!(OpCode::JMP(0x0ABC), result);
    }

    #[test]
    fn ld_vx() {
        let result = decode(0x61AB);
        assert_eq!(
            OpCode::LDVx {
                vx: 0x0001,
                value: 0x00AB
            },
            result
        );
    }

    #[test]
    fn ldi() {
        let result = decode(0xA123);
        assert_eq!(OpCode::LDI(0x0123), result);
    }

    #[test]
    fn add() {
        let result = decode(0x7234);
        assert_eq!(
            OpCode::ADDVx {
                vx: 0x0002,
                value: 0x0034
            },
            result
        );
    }

    #[test]
    fn ld_vx_vy() {
        let result = decode(0x8A10);
        assert_eq!(
            OpCode::LDVxVy {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn or_vx_vy() {
        let result = decode(0x8A11);
        assert_eq!(
            OpCode::ORVxVy {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn and_vx_vy() {
        let result = decode(0x8A12);
        assert_eq!(
            OpCode::ANDVxVy {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn xor_vx_vy() {
        let result = decode(0x8A13);
        assert_eq!(
            OpCode::XORVxVy {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn add_vx_vy() {
        let result = decode(0x8A14);
        assert_eq!(
            OpCode::ADDVxVy {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn sub() {
        let result = decode(0x8A15);
        assert_eq!(
            OpCode::SUB {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn sub_n() {
        let result = decode(0x8A17);
        assert_eq!(
            OpCode::SUBn {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn shr() {
        let result = decode(0x8A16);
        assert_eq!(
            OpCode::SHR {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn shl() {
        let result = decode(0x8A1E);
        assert_eq!(
            OpCode::SHL {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn drw() {
        let result = decode(0xDAB1);
        assert_eq!(
            OpCode::DRW {
                vx: 0x000A,
                vy: 0x000B,
                n: 0x0001
            },
            result
        );
    }
}
