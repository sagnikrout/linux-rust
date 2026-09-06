//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mac53c94.h
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


// SPDX-License-Identifier: GPL-2.0
//
// mac53c94.h: definitions for the driver for the 53c94 SCSI bus adaptor
// found on Power Macintosh computers, controlling the external SCSI chain.
//
// Copyright (C) 1996 Paul Mackerras.
//
// Registers in the 53C94 controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac53c94_regs {
    pub count_lo: c_uchar,
    pub pad0: [c_char; 15],
    pub count_mid: c_uchar,
    pub pad1: [c_char; 15],
    pub fifo: c_uchar,
    pub pad2: [c_char; 15],
    pub command: c_uchar,
    pub pad3: [c_char; 15],
    pub status: c_uchar,
    pub pad4: [c_char; 15],
    pub interrupt: c_uchar,
    pub pad5: [c_char; 15],
    pub seqstep: c_uchar,
    pub pad6: [c_char; 15],
    pub flags: c_uchar,
    pub pad7: [c_char; 15],
    pub config1: c_uchar,
    pub pad8: [c_char; 15],
    pub clk_factor: c_uchar,
    pub pad9: [c_char; 15],
    pub test: c_uchar,
    pub pad10: [c_char; 15],
    pub config2: c_uchar,
    pub pad11: [c_char; 15],
    pub config3: c_uchar,
    pub pad12: [c_char; 15],
    pub config4: c_uchar,
    pub pad13: [c_char; 15],
    pub count_hi: c_uchar,
    pub pad14: [c_char; 15],
    pub fifo_res: c_uchar,
    pub pad15: [c_char; 15],
}

//
// Alternate functions for some registers.
//

//
// Bits in command register.
//
pub const CMD_DMA_MODE: c_uint = 0x80;
pub const CMD_MODE_MASK: c_uint = 0x70;
pub const CMD_MODE_INIT: c_uint = 0x10;
pub const CMD_MODE_TARG: c_uint = 0x20;
pub const CMD_MODE_DISC: c_uint = 0x40;
pub const CMD_NOP: c_int = 0;
pub const CMD_FLUSH: c_int = 1;
pub const CMD_RESET: c_int = 2;
pub const CMD_SCSI_RESET: c_int = 3;
pub const CMD_XFER_DATA: c_uint = 0x10;
pub const CMD_I_COMPLETE: c_uint = 0x11;
pub const CMD_ACCEPT_MSG: c_uint = 0x12;
pub const CMD_XFER_PAD: c_uint = 0x18;
pub const CMD_SET_ATN: c_uint = 0x1a;
pub const CMD_CLR_ATN: c_uint = 0x1b;
pub const CMD_SEND_MSG: c_uint = 0x20;
pub const CMD_SEND_STATUS: c_uint = 0x21;
pub const CMD_SEND_DATA: c_uint = 0x22;
pub const CMD_DISC_SEQ: c_uint = 0x23;
pub const CMD_TERMINATE: c_uint = 0x24;
pub const CMD_T_COMPLETE: c_uint = 0x25;
pub const CMD_DISCONNECT: c_uint = 0x27;
pub const CMD_RECV_MSG: c_uint = 0x28;
pub const CMD_RECV_CDB: c_uint = 0x29;
pub const CMD_RECV_DATA: c_uint = 0x2a;
pub const CMD_RECV_CMD: c_uint = 0x2b;
pub const CMD_ABORT_DMA: c_uint = 0x04;
pub const CMD_RESELECT: c_uint = 0x40;
pub const CMD_SELECT: c_uint = 0x41;
pub const CMD_SELECT_ATN: c_uint = 0x42;
pub const CMD_SELATN_STOP: c_uint = 0x43;
pub const CMD_ENABLE_SEL: c_uint = 0x44;
pub const CMD_DISABLE_SEL: c_uint = 0x45;
pub const CMD_SEL_ATN3: c_uint = 0x46;
pub const CMD_RESEL_ATN3: c_uint = 0x47;
//
// Bits in status register.
//
pub const STAT_IRQ: c_uint = 0x80;
pub const STAT_ERROR: c_uint = 0x40;
pub const STAT_PARITY: c_uint = 0x20;
pub const STAT_TC_ZERO: c_uint = 0x10;
pub const STAT_DONE: c_uint = 0x08;
pub const STAT_PHASE: c_uint = 0x07;
pub const STAT_MSG: c_uint = 0x04;
pub const STAT_CD: c_uint = 0x02;
pub const STAT_IO: c_uint = 0x01;
//
// Bits in interrupt register.
//
pub const INTR_RESET: c_uint = 0x80	/* SCSI bus was reset */;
pub const INTR_ILL_CMD: c_uint = 0x40	/* illegal command */;
pub const INTR_DISCONNECT: c_uint = 0x20	/* we got disconnected */;
pub const INTR_BUS_SERV: c_uint = 0x10	/* bus service requested */;
pub const INTR_DONE: c_uint = 0x08	/* function completed */;
pub const INTR_RESELECTED: c_uint = 0x04	/* we were reselected */;
pub const INTR_SEL_ATN: c_uint = 0x02	/* we were selected, ATN asserted */;
pub const INTR_SELECT: c_uint = 0x01	/* we were selected, ATN negated */;
//
// Encoding for the select timeout.
//

//
// Bits in sequence step register.
//
pub const SS_MASK: c_int = 7;

//
// Encoding for sync transfer period.
//
pub const SYNCP_MASK: c_uint = 0x1f;
pub const SYNCP_MIN: c_int = 4;
pub const SYNCP_MAX: c_int = 31;
//
// Bits in flags register.
//
pub const FLAGS_FIFO_LEV: c_uint = 0x1f;
pub const FLAGS_SEQ_STEP: c_uint = 0xe0;
//
// Encoding for sync offset.
//
pub const SYNCO_MASK: c_uint = 0x0f;
pub const SYNCO_ASS_CTRL: c_uint = 0x30	/* REQ/ACK assertion control */;
pub const SYNCO_NEG_CTRL: c_uint = 0xc0	/* REQ/ACK negation control */;
//
// Bits in config1 register.
//
pub const CF1_SLOW_CABLE: c_uint = 0x80	/* Slow cable mode */;
pub const CF1_NO_RES_REP: c_uint = 0x40	/* Disable SCSI reset reports */;
pub const CF1_PAR_TEST: c_uint = 0x20	/* Parity test mode enable */;
pub const CF1_PAR_ENABLE: c_uint = 0x10	/* Enable parity checks */;
pub const CF1_TEST: c_uint = 0x08	/* Chip tests */;
pub const CF1_MY_ID: c_uint = 0x07	/* Controller's address on bus */;
//
// Encoding for clk_factor register.
//
pub const CLKF_MASK: c_int = 7;

//
// Bits in test mode register.
//

//
// Bits in config2 register.
//
pub const CF2_RFB: c_uint = 0x80;
pub const CF2_FEATURE_EN: c_uint = 0x40	/* enable features / phase latch */;
pub const CF2_BYTECTRL: c_uint = 0x20;
pub const CF2_DREQ_HIZ: c_uint = 0x10;
pub const CF2_SCSI2: c_uint = 0x08;
pub const CF2_PAR_ABORT: c_uint = 0x04	/* bad parity target abort */;
pub const CF2_REG_PARERR: c_uint = 0x02	/* register parity error */;
pub const CF2_DMA_PARERR: c_uint = 0x01	/* DMA parity error */;
//
// Bits in the config3 register.
//
pub const CF3_ID_MSG_CHK: c_uint = 0x80;
pub const CF3_3B_MSGS: c_uint = 0x40;
pub const CF3_CDB10: c_uint = 0x20;
pub const CF3_FASTSCSI: c_uint = 0x10	/* enable fast SCSI support */;
pub const CF3_FASTCLOCK: c_uint = 0x08;
pub const CF3_SAVERESID: c_uint = 0x04;
pub const CF3_ALT_DMA: c_uint = 0x02;
pub const CF3_THRESH_8: c_uint = 0x01;
//
// Bits in the config4 register.
//
pub const CF4_EAN: c_uint = 0x04;
pub const CF4_TEST: c_uint = 0x02;
pub const CF4_BBTE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac53c94_cmd_priv {
    pub this_residual: c_int,
    pub status: c_int,
    pub message: c_int,
}

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
