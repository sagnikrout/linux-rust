//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/mscan/mscan.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Definitions of consts/structs to drive the Freescale MSCAN.
//
// Copyright (C) 2005-2006 Andrey Volkov <avolkov@varma-el.com>,
// Varma Electronics Oy
//

// MSCAN control register 0 (CANCTL0) bits
pub const MSCAN_RXFRM: c_uint = 0x80;
pub const MSCAN_RXACT: c_uint = 0x40;
pub const MSCAN_CSWAI: c_uint = 0x20;
pub const MSCAN_SYNCH: c_uint = 0x10;
pub const MSCAN_TIME: c_uint = 0x08;
pub const MSCAN_WUPE: c_uint = 0x04;
pub const MSCAN_SLPRQ: c_uint = 0x02;
pub const MSCAN_INITRQ: c_uint = 0x01;
// MSCAN control register 1 (CANCTL1) bits
pub const MSCAN_CANE: c_uint = 0x80;
pub const MSCAN_CLKSRC: c_uint = 0x40;
pub const MSCAN_LOOPB: c_uint = 0x20;
pub const MSCAN_LISTEN: c_uint = 0x10;
pub const MSCAN_BORM: c_uint = 0x08;
pub const MSCAN_WUPM: c_uint = 0x04;
pub const MSCAN_SLPAK: c_uint = 0x02;
pub const MSCAN_INITAK: c_uint = 0x01;
// Use the MPC5XXX MSCAN variant?

// Macro flag: #define MSCAN_FOR_MPC5XXX

pub const MSCAN_CLKSRC_BUS: c_int = 0;

pub const MSCAN_CLKSRC_XTAL: c_int = 0;

// MSCAN receiver flag register (CANRFLG) bits
pub const MSCAN_WUPIF: c_uint = 0x80;
pub const MSCAN_CSCIF: c_uint = 0x40;
pub const MSCAN_RSTAT1: c_uint = 0x20;
pub const MSCAN_RSTAT0: c_uint = 0x10;
pub const MSCAN_TSTAT1: c_uint = 0x08;
pub const MSCAN_TSTAT0: c_uint = 0x04;
pub const MSCAN_OVRIF: c_uint = 0x02;
pub const MSCAN_RXF: c_uint = 0x01;

pub const MSCAN_STATE_ACTIVE: c_int = 0;
pub const MSCAN_STATE_WARNING: c_int = 1;
pub const MSCAN_STATE_PASSIVE: c_int = 2;
pub const MSCAN_STATE_BUSOFF: c_int = 3;
// MSCAN receiver interrupt enable register (CANRIER) bits
pub const MSCAN_WUPIE: c_uint = 0x80;
pub const MSCAN_CSCIE: c_uint = 0x40;
pub const MSCAN_RSTATE1: c_uint = 0x20;
pub const MSCAN_RSTATE0: c_uint = 0x10;
pub const MSCAN_TSTATE1: c_uint = 0x08;
pub const MSCAN_TSTATE0: c_uint = 0x04;
pub const MSCAN_OVRIE: c_uint = 0x02;
pub const MSCAN_RXFIE: c_uint = 0x01;
// MSCAN transmitter flag register (CANTFLG) bits
pub const MSCAN_TXE2: c_uint = 0x04;
pub const MSCAN_TXE1: c_uint = 0x02;
pub const MSCAN_TXE0: c_uint = 0x01;

// MSCAN transmitter interrupt enable register (CANTIER) bits
pub const MSCAN_TXIE2: c_uint = 0x04;
pub const MSCAN_TXIE1: c_uint = 0x02;
pub const MSCAN_TXIE0: c_uint = 0x01;

// MSCAN transmitter message abort request (CANTARQ) bits
pub const MSCAN_ABTRQ2: c_uint = 0x04;
pub const MSCAN_ABTRQ1: c_uint = 0x02;
pub const MSCAN_ABTRQ0: c_uint = 0x01;
// MSCAN transmitter message abort ack (CANTAAK) bits
pub const MSCAN_ABTAK2: c_uint = 0x04;
pub const MSCAN_ABTAK1: c_uint = 0x02;
pub const MSCAN_ABTAK0: c_uint = 0x01;
// MSCAN transmit buffer selection (CANTBSEL) bits
pub const MSCAN_TX2: c_uint = 0x04;
pub const MSCAN_TX1: c_uint = 0x02;
pub const MSCAN_TX0: c_uint = 0x01;
// MSCAN ID acceptance control register (CANIDAC) bits
pub const MSCAN_IDAM1: c_uint = 0x20;
pub const MSCAN_IDAM0: c_uint = 0x10;
pub const MSCAN_IDHIT2: c_uint = 0x04;
pub const MSCAN_IDHIT1: c_uint = 0x02;
pub const MSCAN_IDHIT0: c_uint = 0x01;
pub const MSCAN_AF_32BIT: c_uint = 0x00;

// MSCAN Miscellaneous Register (CANMISC) bits
pub const MSCAN_BOHOLD: c_uint = 0x01;
// MSCAN Identifier Register (IDR) bits
pub const MSCAN_SFF_RTR_SHIFT: c_int = 4;
pub const MSCAN_EFF_RTR_SHIFT: c_int = 0;
pub const MSCAN_EFF_FLAGS: c_uint = 0x18	/* IDE + SRR */;

pub const _MSCAN_RESERVED_DSR_SIZE: c_int = 2;

pub const _MSCAN_RESERVED_DSR_SIZE: c_int = 0;

// Structure of the hardware registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mscan_regs {
// (see doc S12MSCANV3/D)		  MPC5200    MSCAN
    pub /: *mut *mut u8 canctl0; / + 0x00 0x00,
    pub /: *mut *mut u8 canctl1; / + 0x01 0x01,
    pub /: *mut *mut _MSCAN_RESERVED_(1, 2); / + 0x02,
    pub /: *mut *mut u8 canbtr0; / + 0x04 0x02,
    pub /: *mut *mut u8 canbtr1; / + 0x05 0x03,
    pub /: *mut *mut _MSCAN_RESERVED_(2, 2); / + 0x06,
    pub /: *mut *mut u8 canrflg; / + 0x08 0x04,
    pub /: *mut *mut u8 canrier; / + 0x09 0x05,
    pub /: *mut *mut _MSCAN_RESERVED_(3, 2); / + 0x0a,
    pub /: *mut *mut u8 cantflg; / + 0x0c 0x06,
    pub /: *mut *mut u8 cantier; / + 0x0d 0x07,
    pub /: *mut *mut _MSCAN_RESERVED_(4, 2); / + 0x0e,
    pub /: *mut *mut u8 cantarq; / + 0x10 0x08,
    pub /: *mut *mut u8 cantaak; / + 0x11 0x09,
    pub /: *mut *mut _MSCAN_RESERVED_(5, 2); / + 0x12,
    pub /: *mut *mut u8 cantbsel; / + 0x14 0x0a,
    pub /: *mut *mut u8 canidac; / + 0x15 0x0b,
    pub /: *mut *mut u8 reserved; / + 0x16 0x0c,
    pub /: *mut *mut _MSCAN_RESERVED_(6, 2); / + 0x17,
    pub /: *mut *mut u8 canmisc; / + 0x19 0x0d,
    pub /: *mut *mut _MSCAN_RESERVED_(7, 2); / + 0x1a,
    pub /: *mut *mut u8 canrxerr; / + 0x1c 0x0e,
    pub /: *mut *mut u8 cantxerr; / + 0x1d 0x0f,
    pub /: *mut *mut _MSCAN_RESERVED_(8, 2); / + 0x1e,
    pub /: *mut *mut u16 canidar1_0; / + 0x20 0x10,
    pub /: *mut *mut _MSCAN_RESERVED_(9, 2); / + 0x22,
    pub /: *mut *mut u16 canidar3_2; / + 0x24 0x12,
    pub /: *mut *mut _MSCAN_RESERVED_(10, 2); / + 0x26,
    pub /: *mut *mut u16 canidmr1_0; / + 0x28 0x14,
    pub /: *mut *mut _MSCAN_RESERVED_(11, 2); / + 0x2a,
    pub /: *mut *mut u16 canidmr3_2; / + 0x2c 0x16,
    pub /: *mut *mut _MSCAN_RESERVED_(12, 2); / + 0x2e,
    pub /: *mut *mut u16 canidar5_4; / + 0x30 0x18,
    pub /: *mut *mut _MSCAN_RESERVED_(13, 2); / + 0x32,
    pub /: *mut *mut u16 canidar7_6; / + 0x34 0x1a,
    pub /: *mut *mut _MSCAN_RESERVED_(14, 2); / + 0x36,
    pub /: *mut *mut u16 canidmr5_4; / + 0x38 0x1c,
    pub /: *mut *mut _MSCAN_RESERVED_(15, 2); / + 0x3a,
    pub /: *mut *mut u16 canidmr7_6; / + 0x3c 0x1e,
    pub /: *mut *mut _MSCAN_RESERVED_(16, 2); / + 0x3e,
    pub /: *mut *mut u16 idr1_0; / + 0x40 0x20,
    pub /: *mut *mut _MSCAN_RESERVED_(17, 2); / + 0x42,
    pub /: *mut *mut u16 idr3_2; / + 0x44 0x22,
    pub /: *mut *mut _MSCAN_RESERVED_(18, 2); / + 0x46,
    pub /: *mut *mut u16 dsr1_0; / + 0x48 0x24,
    pub /: *mut *mut _MSCAN_RESERVED_(19, 2); / + 0x4a,
    pub /: *mut *mut u16 dsr3_2; / + 0x4c 0x26,
    pub /: *mut *mut _MSCAN_RESERVED_(20, 2); / + 0x4e,
    pub /: *mut *mut u16 dsr5_4; / + 0x50 0x28,
    pub /: *mut *mut _MSCAN_RESERVED_(21, 2); / + 0x52,
    pub /: *mut *mut u16 dsr7_6; / + 0x54 0x2a,
    pub /: *mut *mut _MSCAN_RESERVED_(22, 2); / + 0x56,
    pub /: *mut *mut u8 dlr; / + 0x58 0x2c,
    pub /: *mut *mut u8 reserved; / + 0x59 0x2d,
    pub /: *mut *mut _MSCAN_RESERVED_(23, 2); / + 0x5a,
    pub /: *mut *mut u16 time; / + 0x5c 0x2e,
    pub rx: },
    pub /: *mut *mut _MSCAN_RESERVED_(24, 2); / + 0x5e,
    pub /: *mut *mut u16 idr1_0; / + 0x60 0x30,
    pub /: *mut *mut _MSCAN_RESERVED_(25, 2); / + 0x62,
    pub /: *mut *mut u16 idr3_2; / + 0x64 0x32,
    pub /: *mut *mut _MSCAN_RESERVED_(26, 2); / + 0x66,
    pub /: *mut *mut u16 dsr1_0; / + 0x68 0x34,
    pub /: *mut *mut _MSCAN_RESERVED_(27, 2); / + 0x6a,
    pub /: *mut *mut u16 dsr3_2; / + 0x6c 0x36,
    pub /: *mut *mut _MSCAN_RESERVED_(28, 2); / + 0x6e,
    pub /: *mut *mut u16 dsr5_4; / + 0x70 0x38,
    pub /: *mut *mut _MSCAN_RESERVED_(29, 2); / + 0x72,
    pub /: *mut *mut u16 dsr7_6; / + 0x74 0x3a,
    pub /: *mut *mut _MSCAN_RESERVED_(30, 2); / + 0x76,
    pub /: *mut *mut u8 dlr; / + 0x78 0x3c,
    pub /: *mut *mut u8 tbpr; / + 0x79 0x3d,
    pub /: *mut *mut _MSCAN_RESERVED_(31, 2); / + 0x7a,
    pub /: *mut *mut u16 time; / + 0x7c 0x3e,
    pub tx: },
    pub /: *mut *mut _MSCAN_RESERVED_(32, 2); / + 0x7e,
    pub __packed: },

pub const MSCAN_NORMAL_MODE: c_int = 0;

pub const MSCAN_SET_MODE_RETRIES: c_int = 255;
pub const MSCAN_ECHO_SKB_MAX: c_int = 3;

// MSCAN type variants
}

pub const BTR0_BRP_MASK: c_uint = 0x3f;
pub const BTR0_SJW_SHIFT: c_int = 6;

pub const BTR1_TSEG1_MASK: c_uint = 0xf;
pub const BTR1_TSEG2_SHIFT: c_int = 4;

pub const BTR1_SAM_SHIFT: c_int = 7;

pub const F_RX_PROGRESS: c_int = 0;
pub const F_TX_PROGRESS: c_int = 1;
pub const F_TX_WAIT_ALL: c_int = 2;
pub const TX_QUEUE_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_queue_entry {
    pub list: list_head,
    pub mask: u8,
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mscan_priv {
    pub /: *mut *mut can_priv can; / must be the first member,
    pub /: *mut *mut unsigned int type; / MSCAN type variants,
    pub flags: c_ulong,
    pub /: *mut *mut *mut void __iomem reg_base; / ioremap'ed address to registers,
    pub /: *mut *mut *mut clk clk_ipg; / clock for registers,
    pub /: *mut *mut *mut clk clk_can; / clock for bitrates,
    pub shadow_statflg: u8,
    pub shadow_canrier: u8,
    pub cur_pri: u8,
    pub prev_buf_id: u8,
    pub tx_active: u8,
    pub tx_head: list_head,
    pub tx_queue: [tx_queue_entry; TX_QUEUE_SIZE],
    pub napi: napi_struct,
}

extern "C" {
    pub fn register_mscandev(dev: *mut net_device, mscan_clksrc: c_int) -> c_int;
}
extern "C" {
    pub fn unregister_mscandev(dev: *mut net_device);
}
