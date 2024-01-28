use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    CLS,
    JMP(u16),
    LDVx { vx: u8, value: u8 },
    LDVxVy { vx: u8, vy: u8 },
    ADDVx { vx: u8, value: u8 },
    LDI(u16),
    DRW { vx: u8, vy: u8, n: u8 },
    Unknown,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OpCode::CLS => write!(f, "CLS"),
            OpCode::JMP(addr) => write!(f, "JMP address:{:#06X}", addr),
            OpCode::LDVx { vx, value } => write!(f, "LDVx VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::LDVxVy { vx, vy } => write!(f, "LDVxVy VX:{:#06X} VY:{:#06X}", vx, vy),
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
                vx: ((op & 0x0F00) >> 8) as u8,
                value: (op & 0x00FF) as u8,
            },
            0x7000 => OpCode::ADDVx {
                vx: ((op & 0x0F00) >> 8) as u8,
                value: (op & 0x00FF) as u8,
            },
            0x8000 => OpCode::LDVxVy {
                vx: ((op & 0x0F00) >> 8) as u8,
                vy: ((op & 0x00F0) >> 4) as u8,
            },
            0xA000 => OpCode::LDI(op & 0x0FFF),
            0xD000 => OpCode::DRW {
                vx: ((op & 0x0F00) >> 8) as u8,
                vy: ((op & 0x00F0) >> 4) as u8,
                n: (op & 0x000F) as u8,
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
