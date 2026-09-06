//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/seeq/ether3.h
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
// linux/drivers/acorn/net/ether3.h
//
// Copyright (C) 1995-2000 Russell King
//
// network driver for Acorn/ANT Ether3 cards
//

// Macro flag: #define _LINUX_ether3_H
// use 0 for production, 1 for verification, >2 for debug. debug flags:
pub const DEBUG_TX: c_int = 2;
pub const DEBUG_RX: c_int = 4;
pub const DEBUG_INT: c_int = 8;
pub const DEBUG_IC: c_int = 16;

pub const NET_DEBUG: c_int = 0;

// Command register definitions & bits

pub const CMD_ENINTDMA: c_uint = 0x0001;
pub const CMD_ENINTRX: c_uint = 0x0002;
pub const CMD_ENINTTX: c_uint = 0x0004;
pub const CMD_ENINTBUFWIN: c_uint = 0x0008;
pub const CMD_ACKINTDMA: c_uint = 0x0010;
pub const CMD_ACKINTRX: c_uint = 0x0020;
pub const CMD_ACKINTTX: c_uint = 0x0040;
pub const CMD_ACKINTBUFWIN: c_uint = 0x0080;
pub const CMD_DMAON: c_uint = 0x0100;
pub const CMD_RXON: c_uint = 0x0200;
pub const CMD_TXON: c_uint = 0x0400;
pub const CMD_DMAOFF: c_uint = 0x0800;
pub const CMD_RXOFF: c_uint = 0x1000;
pub const CMD_TXOFF: c_uint = 0x2000;
pub const CMD_FIFOREAD: c_uint = 0x4000;
pub const CMD_FIFOWRITE: c_uint = 0x8000;
// status register

pub const STAT_ENINTSTAT: c_uint = 0x0001;
pub const STAT_ENINTRX: c_uint = 0x0002;
pub const STAT_ENINTTX: c_uint = 0x0004;
pub const STAT_ENINTBUFWIN: c_uint = 0x0008;
pub const STAT_INTDMA: c_uint = 0x0010;
pub const STAT_INTRX: c_uint = 0x0020;
pub const STAT_INTTX: c_uint = 0x0040;
pub const STAT_INTBUFWIN: c_uint = 0x0080;
pub const STAT_DMAON: c_uint = 0x0100;
pub const STAT_RXON: c_uint = 0x0200;
pub const STAT_TXON: c_uint = 0x0400;
pub const STAT_FIFOFULL: c_uint = 0x2000;
pub const STAT_FIFOEMPTY: c_uint = 0x4000;
pub const STAT_FIFODIR: c_uint = 0x8000;
// configuration register 1

pub const CFG1_BUFSELSTAT0: c_uint = 0x0000;
pub const CFG1_BUFSELSTAT1: c_uint = 0x0001;
pub const CFG1_BUFSELSTAT2: c_uint = 0x0002;
pub const CFG1_BUFSELSTAT3: c_uint = 0x0003;
pub const CFG1_BUFSELSTAT4: c_uint = 0x0004;
pub const CFG1_BUFSELSTAT5: c_uint = 0x0005;
pub const CFG1_ADDRPROM: c_uint = 0x0006;
pub const CFG1_TRANSEND: c_uint = 0x0007;
pub const CFG1_LOCBUFMEM: c_uint = 0x0008;
pub const CFG1_INTVECTOR: c_uint = 0x0009;
pub const CFG1_RECVSPECONLY: c_uint = 0x0000;
pub const CFG1_RECVSPECBROAD: c_uint = 0x4000;
pub const CFG1_RECVSPECBRMULTI: c_uint = 0x8000;
pub const CFG1_RECVPROMISC: c_uint = 0xC000;
// The following aren't in 8004
pub const CFG1_DMABURSTCONT: c_uint = 0x0000;
pub const CFG1_DMABURST800NS: c_uint = 0x0010;
pub const CFG1_DMABURST1600NS: c_uint = 0x0020;
pub const CFG1_DMABURST3200NS: c_uint = 0x0030;
pub const CFG1_DMABURST1: c_uint = 0x0000;
pub const CFG1_DMABURST4: c_uint = 0x0040;
pub const CFG1_DMABURST8: c_uint = 0x0080;
pub const CFG1_DMABURST16: c_uint = 0x00C0;
pub const CFG1_RECVCOMPSTAT0: c_uint = 0x0100;
pub const CFG1_RECVCOMPSTAT1: c_uint = 0x0200;
pub const CFG1_RECVCOMPSTAT2: c_uint = 0x0400;
pub const CFG1_RECVCOMPSTAT3: c_uint = 0x0800;
pub const CFG1_RECVCOMPSTAT4: c_uint = 0x1000;
pub const CFG1_RECVCOMPSTAT5: c_uint = 0x2000;
// configuration register 2

pub const CFG2_BYTESWAP: c_uint = 0x0001;
pub const CFG2_ERRENCRC: c_uint = 0x0008;
pub const CFG2_ERRENDRIBBLE: c_uint = 0x0010;
pub const CFG2_ERRSHORTFRAME: c_uint = 0x0020;
pub const CFG2_SLOTSELECT: c_uint = 0x0040;
pub const CFG2_PREAMSELECT: c_uint = 0x0080;
pub const CFG2_ADDRLENGTH: c_uint = 0x0100;
pub const CFG2_RECVCRC: c_uint = 0x0200;
pub const CFG2_XMITNOCRC: c_uint = 0x0400;
pub const CFG2_LOOPBACK: c_uint = 0x0800;
pub const CFG2_CTRLO: c_uint = 0x1000;
pub const CFG2_RESET: c_uint = 0x8000;

//
// Cards transmit/receive headers
//

pub const TX_START: c_uint = 0x0000;
pub const TX_END: c_uint = 0x6000;
pub const RX_START: c_uint = 0x6000;
pub const RX_LEN: c_uint = 0xA000;
pub const RX_END: c_uint = 0x10000;
// must be a power of 2 and greater than MAX_TX_BUFFERED
pub const MAX_TXED: c_int = 16;
pub const MAX_TX_BUFFERED: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_priv {
    pub base: *mut void __iomem,
    pub seeq: *mut void __iomem,
    pub command: c_uint,
    pub config1: c_uint,
    pub config2: c_uint,
    pub regs: },
    pub /: *mut *mut unsigned char tx_head; / buffer nr to insert next packet,
    pub /: *mut *mut unsigned char tx_tail; / buffer nr of transmitting packet,
    pub /: *mut *mut unsigned int rx_head; / address to fetch next packet from,
    pub timer: timer_list,
    pub dev: *mut net_device,
    pub /: *mut *mut int broken; / 0 = ok, 1 = something went wrong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ether3_data {
    pub name: [c_char; 8],
    pub base_offset: c_ulong,
}
