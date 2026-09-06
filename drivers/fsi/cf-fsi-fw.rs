//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fsi/cf-fsi-fw.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0+
//
// uCode file layout
//
// 0000...03ff : m68k exception vectors
// 0400...04ff : Header info & boot config block
// 0500....... : Code & stack
//
// Header info & boot config area
//
// The Header info is built into the ucode and provide version and
// platform information.
//
// the Boot config needs to be adjusted by the ARM prior to starting
// the ucode if the Command/Status area isn't at 0x320000 in CF space
// (ie. beginning of SRAM).
//
pub const HDR_OFFSET: c_uint = 0x400;
// Info: Signature & version
pub const HDR_SYS_SIG: c_uint = 0x00	/* 2 bytes system signature */;
pub const SYS_SIG_SHARED: c_uint = 0x5348;
pub const SYS_SIG_SPLIT: c_uint = 0x5350;
pub const HDR_FW_VERS: c_uint = 0x02	/* 2 bytes Major.Minor */;
pub const HDR_API_VERS: c_uint = 0x04	/* 2 bytes Major.Minor */;

pub const API_VERSION_MIN: c_int = 1;
pub const HDR_FW_OPTIONS: c_uint = 0x08	/* 4 bytes option flags */;
pub const FW_OPTION_TRACE_EN: c_uint = 0x00000001	/* FW tracing enabled */;
pub const FW_OPTION_CONT_CLOCK: c_uint = 0x00000002	/* Continuous clocking supported */;
pub const HDR_FW_SIZE: c_uint = 0x10	/* 4 bytes size for combo image */;
// Boot Config: Address of Command/Status area
pub const HDR_CMD_STAT_AREA: c_uint = 0x80	/* 4 bytes CF address */;
pub const HDR_FW_CONTROL: c_uint = 0x84	/* 4 bytes control flags */;
pub const FW_CONTROL_CONT_CLOCK: c_uint = 0x00000002	/* Continuous clocking enabled */;
pub const FW_CONTROL_DUMMY_RD: c_uint = 0x00000004	/* Extra dummy read (AST2400) */;
pub const FW_CONTROL_USE_STOP: c_uint = 0x00000008	/* Use STOP instructions */;
pub const HDR_CLOCK_GPIO_VADDR: c_uint = 0x90	/* 2 bytes offset from GPIO base */;
pub const HDR_CLOCK_GPIO_DADDR: c_uint = 0x92	/* 2 bytes offset from GPIO base */;
pub const HDR_DATA_GPIO_VADDR: c_uint = 0x94	/* 2 bytes offset from GPIO base */;
pub const HDR_DATA_GPIO_DADDR: c_uint = 0x96	/* 2 bytes offset from GPIO base */;
pub const HDR_TRANS_GPIO_VADDR: c_uint = 0x98	/* 2 bytes offset from GPIO base */;
pub const HDR_TRANS_GPIO_DADDR: c_uint = 0x9a	/* 2 bytes offset from GPIO base */;
pub const HDR_CLOCK_GPIO_BIT: c_uint = 0x9c	/* 1 byte bit number */;
pub const HDR_DATA_GPIO_BIT: c_uint = 0x9d	/* 1 byte bit number */;
pub const HDR_TRANS_GPIO_BIT: c_uint = 0x9e	/* 1 byte bit number */;
//
// Command/Status area layout: Main part
//
// Command/Status register:
//
// +---------------------------+
// | STAT | RLEN | CLEN | CMD  |
// |   8  |   8  |   8  |   8  |
// +---------------------------+
// |       |      |      |
// status  |      |      |
// Response len      |      |
// (in bits)         |      |
// |      |
// Command len      |
// (in bits)        |
// |
// Command code
//
// Due to the big endian layout, that means that a byte read will
// return the status byte
//
pub const CMD_STAT_REG: c_uint = 0x00;
pub const CMD_REG_CMD_MASK: c_uint = 0x000000ff;
pub const CMD_REG_CMD_SHIFT: c_int = 0;
pub const CMD_NONE: c_uint = 0x00;
pub const CMD_COMMAND: c_uint = 0x01;
pub const CMD_BREAK: c_uint = 0x02;
pub const CMD_IDLE_CLOCKS: c_uint = 0x03 /* clen = #clocks */;
pub const CMD_INVALID: c_uint = 0xff;
pub const CMD_REG_CLEN_MASK: c_uint = 0x0000ff00;
pub const CMD_REG_CLEN_SHIFT: c_int = 8;
pub const CMD_REG_RLEN_MASK: c_uint = 0x00ff0000;
pub const CMD_REG_RLEN_SHIFT: c_int = 16;
pub const CMD_REG_STAT_MASK: c_uint = 0xff000000;
pub const CMD_REG_STAT_SHIFT: c_int = 24;
pub const STAT_WORKING: c_uint = 0x00;
pub const STAT_COMPLETE: c_uint = 0x01;
pub const STAT_ERR_INVAL_CMD: c_uint = 0x80;
pub const STAT_ERR_INVAL_IRQ: c_uint = 0x81;
pub const STAT_ERR_MTOE: c_uint = 0x82;
// Response tag & CRC
pub const STAT_RTAG: c_uint = 0x04;
// Response CRC
pub const STAT_RCRC: c_uint = 0x05;
// Echo and Send delay
pub const ECHO_DLY_REG: c_uint = 0x08;
pub const SEND_DLY_REG: c_uint = 0x09;
// Command data area
//
// Last byte of message must be left aligned
//
pub const CMD_DATA: c_uint = 0x10 /* 64 bit of data */;
// Response data area, right aligned, unused top bits are 1
pub const RSP_DATA: c_uint = 0x20 /* 32 bit of data */;
// Misc
pub const INT_CNT: c_uint = 0x30 /* 32-bit interrupt count */;
pub const BAD_INT_VEC: c_uint = 0x34 /* 32-bit bad interrupt vector # */;
pub const CF_STARTED: c_uint = 0x38 /* byte, set to -1 when copro started */;
pub const CLK_CNT: c_uint = 0x3c /* 32-bit, clock count (debug only) */;
//
// SRAM layout: GPIO arbitration part
//
pub const ARB_REG: c_uint = 0x40;
pub const ARB_ARM_REQ: c_uint = 0x01;
pub const ARB_ARM_ACK: c_uint = 0x02;
// Misc2
pub const CF_RESET_D0: c_uint = 0x50;
pub const CF_RESET_D1: c_uint = 0x54;
pub const BAD_INT_S0: c_uint = 0x58;
pub const BAD_INT_S1: c_uint = 0x5c;
pub const STOP_CNT: c_uint = 0x60;
// Internal
//
// SRAM layout: Trace buffer (debug builds only)
//
pub const TRACEBUF: c_uint = 0x100;
pub const TR_CLKOBIT0: c_uint = 0xc0;
pub const TR_CLKOBIT1: c_uint = 0xc1;
pub const TR_CLKOSTART: c_uint = 0x82;
pub const TR_OLEN: c_uint = 0x83 /* + len */;
pub const TR_CLKZ: c_uint = 0x84 /* + count */;
pub const TR_CLKWSTART: c_uint = 0x85;
pub const TR_CLKTAG: c_uint = 0x86 /* + tag */;
pub const TR_CLKDATA: c_uint = 0x87 /* + len */;
pub const TR_CLKCRC: c_uint = 0x88 /* + raw crc */;
pub const TR_CLKIBIT0: c_uint = 0x90;
pub const TR_CLKIBIT1: c_uint = 0x91;
pub const TR_END: c_uint = 0xff;
