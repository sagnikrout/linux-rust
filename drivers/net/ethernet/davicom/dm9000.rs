//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/davicom/dm9000.h
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
// dm9000 Ethernet
//
pub const DM9000_ID: c_uint = 0x90000A46;
// although the registers are 16 bit, they are 32-bit aligned.
//
pub const DM9000_NCR: c_uint = 0x00;
pub const DM9000_NSR: c_uint = 0x01;
pub const DM9000_TCR: c_uint = 0x02;
pub const DM9000_TSR1: c_uint = 0x03;
pub const DM9000_TSR2: c_uint = 0x04;
pub const DM9000_RCR: c_uint = 0x05;
pub const DM9000_RSR: c_uint = 0x06;
pub const DM9000_ROCR: c_uint = 0x07;
pub const DM9000_BPTR: c_uint = 0x08;
pub const DM9000_FCTR: c_uint = 0x09;
pub const DM9000_FCR: c_uint = 0x0A;
pub const DM9000_EPCR: c_uint = 0x0B;
pub const DM9000_EPAR: c_uint = 0x0C;
pub const DM9000_EPDRL: c_uint = 0x0D;
pub const DM9000_EPDRH: c_uint = 0x0E;
pub const DM9000_WCR: c_uint = 0x0F;
pub const DM9000_PAR: c_uint = 0x10;
pub const DM9000_MAR: c_uint = 0x16;
pub const DM9000_GPCR: c_uint = 0x1e;
pub const DM9000_GPR: c_uint = 0x1f;
pub const DM9000_TRPAL: c_uint = 0x22;
pub const DM9000_TRPAH: c_uint = 0x23;
pub const DM9000_RWPAL: c_uint = 0x24;
pub const DM9000_RWPAH: c_uint = 0x25;
pub const DM9000_VIDL: c_uint = 0x28;
pub const DM9000_VIDH: c_uint = 0x29;
pub const DM9000_PIDL: c_uint = 0x2A;
pub const DM9000_PIDH: c_uint = 0x2B;
pub const DM9000_CHIPR: c_uint = 0x2C;
pub const DM9000_SMCR: c_uint = 0x2F;
pub const DM9000_ETXCSR: c_uint = 0x30;
pub const DM9000_TCCR: c_uint = 0x31;
pub const DM9000_RCSR: c_uint = 0x32;
pub const CHIPR_DM9000A: c_uint = 0x19;
pub const CHIPR_DM9000B: c_uint = 0x1A;
pub const DM9000_MRCMDX: c_uint = 0xF0;
pub const DM9000_MRCMD: c_uint = 0xF2;
pub const DM9000_MRRL: c_uint = 0xF4;
pub const DM9000_MRRH: c_uint = 0xF5;
pub const DM9000_MWCMDX: c_uint = 0xF6;
pub const DM9000_MWCMD: c_uint = 0xF8;
pub const DM9000_MWRL: c_uint = 0xFA;
pub const DM9000_MWRH: c_uint = 0xFB;
pub const DM9000_TXPLL: c_uint = 0xFC;
pub const DM9000_TXPLH: c_uint = 0xFD;
pub const DM9000_ISR: c_uint = 0xFE;
pub const DM9000_IMR: c_uint = 0xFF;

pub const DM9000_PKT_RDY: c_uint = 0x01	/* Packet ready to receive */;
pub const DM9000_PKT_ERR: c_uint = 0x02;

// DM9000A / DM9000B definitions

// Davicom MII registers.
//
pub const MII_DM_DSPCR: c_uint = 0x1b    /* DSP Control Register */;
pub const DSPCR_INIT_PARAM: c_uint = 0xE100	/* DSP init parameter */;
