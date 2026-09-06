//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/i825xx/ether1.h
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
// linux/drivers/acorn/net/ether1.h
//
// Copyright (C) 1996 Russell King
//
// Network driver for Acorn Ether1 cards.
//

// Macro flag: #define _LINUX_ether1_H

// use 0 for production, 1 for verification, >2 for debug

pub const NET_DEBUG: c_int = 0;

// Page register

// Control register

pub const CTRL_RST: c_uint = 0x01;
pub const CTRL_LOOPBACK: c_uint = 0x02;
pub const CTRL_CA: c_uint = 0x04;
pub const CTRL_ACK: c_uint = 0x08;

// HW address

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ether1_priv {
    pub base: *mut void __iomem,
    pub tx_link: c_uint,
    pub tx_head: c_uint,
    pub tx_tail: volatile unsigned int,
    pub rx_head: volatile unsigned int,
    pub rx_tail: volatile unsigned int,
    pub bus_type: c_uchar,
    pub resetting: c_uchar,
    pub 1: unsigned char initialising :,
    pub 1: unsigned char restart :,
}

// this address must be 0xfff6
pub const SCP_SY_16BBUS: c_uint = 0x00;
pub const SCP_SY_8BBUS: c_uint = 0x01;
// commands
pub const CMD_NOP: c_int = 0;
pub const CMD_SETADDRESS: c_int = 1;
pub const CMD_CONFIG: c_int = 2;
pub const CMD_SETMULTICAST: c_int = 3;
pub const CMD_TX: c_int = 4;
pub const CMD_TDR: c_int = 5;
pub const CMD_DUMP: c_int = 6;
pub const CMD_DIAGNOSE: c_int = 7;
pub const CMD_MASK: c_int = 7;

//
// Ether1 card definitions:
//
// FAST accesses:
// +0	Page register
// 16 pages
// +4	Control
// '1' = reset
// '2' = loopback
// '4' = CA
// '8' = int ack
//
// RAM at address + 0x2000
// Pod. Prod id = 3
// Words after ID block [base + 8 words]
// +0 pcb issue (0x0c and 0xf3 invalid)
// +1 - +6 eth hw address
//
