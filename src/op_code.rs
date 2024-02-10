use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Cls,
    Ret,
    Jmp(u16),
    Call(u16),
    Se { vx: usize, value: u8 },
    Sne { vx: usize, value: u8 },
    SeVxVy { vx: usize, vy: usize },
    LdVx { vx: usize, value: u8 },
    LdVxVy { vx: usize, vy: usize },
    OrVxVy { vx: usize, vy: usize },
    AndVxVy { vx: usize, vy: usize },
    XorVxVy { vx: usize, vy: usize },
    AddVxVy { vx: usize, vy: usize },
    Sub { vx: usize, vy: usize },
    SubN { vx: usize, vy: usize },
    Shr { vx: usize, vy: usize },
    Shl { vx: usize, vy: usize },
    AddVx { vx: usize, value: u8 },
    SneVxVy { vx: usize, vy: usize },
    LdI(u16),
    JmpV0(u16),
    Rnd { vx: usize, value: u8 },
    Drw { vx: usize, vy: usize, n: usize },
    AddIVx(usize),
    LdIVx(usize),
    LdVxI(usize),
    LdVxK(usize),
    LdDtVx(usize),
    LdStVx(usize),
    Unknown,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            OpCode::Cls => write!(f, "CLS"),
            OpCode::Ret => write!(f, "RET"),
            OpCode::Jmp(addr) => write!(f, "JMP address:{:#06X}", addr),
            OpCode::Call(addr) => write!(f, "CALL address:{:#06X}", addr),
            OpCode::Se { vx, value } => write!(f, "SE VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::Sne { vx, value } => write!(f, "SNE VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::SeVxVy { vx, vy } => write!(f, "SE VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::LdVx { vx, value } => write!(f, "LD VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::LdVxVy { vx, vy } => write!(f, "LD VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::OrVxVy { vx, vy } => write!(f, "OR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::AndVxVy { vx, vy } => write!(f, "AND VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::XorVxVy { vx, vy } => write!(f, "XOR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::AddVxVy { vx, vy } => write!(f, "ADD VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::Sub { vx, vy } => write!(f, "SUB VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::SubN { vx, vy } => write!(f, "SUBn VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::Shr { vx, vy } => write!(f, "SHR VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::Shl { vx, vy } => write!(f, "SHL VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::AddVx { vx, value } => write!(f, "ADD VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::SneVxVy { vx, vy } => write!(f, "SNE VX:{:#06X} VY:{:#06X}", vx, vy),
            OpCode::LdI(value) => write!(f, "LDI value:{:#06X}", value),
            OpCode::JmpV0(value) => write!(f, "JMPV0 value:{:#06X}", value),
            OpCode::Rnd { vx, value } => write!(f, "RND VX:{:#06X} value:{:#06X}", vx, value),
            OpCode::AddIVx(vx) => write!(f, "ADD I VX:{:#06X}", vx),
            OpCode::LdIVx(vx) => write!(f, "LD I VX:{:#06X}", vx),
            OpCode::LdVxI(vx) => write!(f, "LD VX:{:#06X} I", vx),
            OpCode::LdVxK(vx) => write!(f, "LD VX:{:#06X} K", vx),
            OpCode::LdDtVx(vx) => write!(f, "LD DT VX:{:#06X}", vx),
            OpCode::LdStVx(vx) => write!(f, "LD ST VX:{:#06X}", vx),
            OpCode::Drw { vx, vy, n } => {
                write!(f, "DRW VX:{:#06X} VX:{:#06X} n:{:#06X}", vx, vy, n)
            }
            OpCode::Unknown => write!(f, "Unknown"),
        }
    }
}

pub fn decode(op: u16) -> OpCode {
    match op {
        0x00E0 => OpCode::Cls,
        0x00EE => OpCode::Ret,
        _ => match op & 0xF000 {
            0x1000 => OpCode::Jmp(op & 0x0FFF),
            0x2000 => OpCode::Call(op & 0x0FFF),
            0x3000 => OpCode::Se {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x4000 => OpCode::Sne {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x5000 => OpCode::SeVxVy {
                vx: ((op & 0x0F00) >> 8) as usize,
                vy: ((op & 0x00F0) >> 4) as usize,
            },
            0x6000 => OpCode::LdVx {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x7000 => OpCode::AddVx {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0x8000 => match op & 0x000F {
                0x0000 => OpCode::LdVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0001 => OpCode::OrVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0002 => OpCode::AndVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0003 => OpCode::XorVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0004 => OpCode::AddVxVy {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0005 => OpCode::Sub {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0006 => OpCode::Shr {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x0007 => OpCode::SubN {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                0x000E => OpCode::Shl {
                    vx: ((op & 0x0F00) >> 8) as usize,
                    vy: ((op & 0x00F0) >> 4) as usize,
                },
                _ => OpCode::Unknown,
            },
            0x9000 => OpCode::SneVxVy {
                // may need to check that the last byte is also zero... here
                // its masked out, but its the only op withe a MSD of 9 in the
                // spec so I didn't check it.
                vx: ((op & 0x0F00) >> 8) as usize,
                vy: ((op & 0x00F0) >> 4) as usize,
            },
            0xA000 => OpCode::LdI(op & 0x0FFF),
            0xB000 => OpCode::JmpV0(op & 0x0FFF),
            0xC000 => OpCode::Rnd {
                vx: ((op & 0x0F00) >> 8) as usize,
                value: (op & 0x00FF) as u8,
            },
            0xD000 => OpCode::Drw {
                vx: ((op & 0x0F00) >> 8) as usize,
                vy: ((op & 0x00F0) >> 4) as usize,
                n: (op & 0x000F) as usize,
            },
            0xF000 => match op & 0x00FF {
                0x000A => OpCode::LdVxK(((op & 0x0F00) >> 8) as usize),
                0x001E => OpCode::AddIVx(((op & 0x0F00) >> 8) as usize),
                0x0015 => OpCode::LdDtVx(((op & 0x0F00) >> 8) as usize),
                0x0018 => OpCode::LdStVx(((op & 0x0F00) >> 8) as usize),
                0x0055 => OpCode::LdIVx(((op & 0x0F00) >> 8) as usize),
                0x0065 => OpCode::LdVxI(((op & 0x0F00) >> 8) as usize),
                _ => OpCode::Unknown,
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
        assert_eq!(OpCode::Cls, result);
    }

    #[test]
    fn ret() {
        let result = decode(0x00EE);
        assert_eq!(OpCode::Ret, result);
    }

    #[test]
    fn jmp() {
        let result = decode(0x1ABC);
        assert_eq!(OpCode::Jmp(0x0ABC), result);
    }

    #[test]
    fn call() {
        let result = decode(0x2ABC);
        assert_eq!(OpCode::Call(0x0ABC), result);
    }

    #[test]
    fn se() {
        let result = decode(0x31AB);
        assert_eq!(
            OpCode::Se {
                vx: 0x0001,
                value: 0x00AB
            },
            result
        );
    }

    #[test]
    fn sne() {
        let result = decode(0x41AB);
        assert_eq!(
            OpCode::Sne {
                vx: 0x0001,
                value: 0x00AB
            },
            result
        );
    }

    #[test]
    fn se_vx_vy() {
        let result = decode(0x51A0);
        assert_eq!(
            OpCode::SeVxVy {
                vx: 0x0001,
                vy: 0x000A
            },
            result
        );
    }

    #[test]
    fn ld_vx() {
        let result = decode(0x61AB);
        assert_eq!(
            OpCode::LdVx {
                vx: 0x0001,
                value: 0x00AB
            },
            result
        );
    }

    #[test]
    fn ldi() {
        let result = decode(0xA123);
        assert_eq!(OpCode::LdI(0x0123), result);
    }

    #[test]
    fn jmp_v0() {
        let result = decode(0xB123);
        assert_eq!(OpCode::JmpV0(0x0123), result);
    }

    #[test]
    fn add() {
        let result = decode(0x7234);
        assert_eq!(
            OpCode::AddVx {
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
            OpCode::LdVxVy {
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
            OpCode::OrVxVy {
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
            OpCode::AndVxVy {
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
            OpCode::XorVxVy {
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
            OpCode::AddVxVy {
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
            OpCode::Sub {
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
            OpCode::SubN {
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
            OpCode::Shr {
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
            OpCode::Shl {
                vx: 0x000A,
                vy: 0x0001,
            },
            result
        );
    }

    #[test]
    fn sne_vx_vy() {
        let result = decode(0x9A10);
        assert_eq!(
            OpCode::SneVxVy {
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
            OpCode::Drw {
                vx: 0x000A,
                vy: 0x000B,
                n: 0x0001
            },
            result
        );
    }
}
