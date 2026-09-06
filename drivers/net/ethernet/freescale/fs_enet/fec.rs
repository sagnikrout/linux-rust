//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/fs_enet/fec.h
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
pub const FEC_MAX_MULTICAST_ADDRS: c_int = 64;
// Interrupt events/masks.
//
pub const FEC_ENET_HBERR: c_uint = 0x80000000U	/* Heartbeat error          */;
pub const FEC_ENET_BABR: c_uint = 0x40000000U	/* Babbling receiver        */;
pub const FEC_ENET_BABT: c_uint = 0x20000000U	/* Babbling transmitter     */;
pub const FEC_ENET_GRA: c_uint = 0x10000000U	/* Graceful stop complete   */;
pub const FEC_ENET_TXF: c_uint = 0x08000000U	/* Full frame transmitted   */;
pub const FEC_ENET_TXB: c_uint = 0x04000000U	/* A buffer was transmitted */;
pub const FEC_ENET_RXF: c_uint = 0x02000000U	/* Full frame received      */;
pub const FEC_ENET_RXB: c_uint = 0x01000000U	/* A buffer was received    */;
pub const FEC_ENET_MII: c_uint = 0x00800000U	/* MII interrupt            */;
pub const FEC_ENET_EBERR: c_uint = 0x00400000U	/* SDMA bus error           */;
pub const FEC_ECNTRL_PINMUX: c_uint = 0x00000004;
pub const FEC_ECNTRL_ETHER_EN: c_uint = 0x00000002;
pub const FEC_ECNTRL_RESET: c_uint = 0x00000001;
// RMII mode enabled only when MII_MODE bit is set too.

pub const FEC_RCNTRL_FCE: c_uint = 0x00000020;
pub const FEC_RCNTRL_BC_REJ: c_uint = 0x00000010;
pub const FEC_RCNTRL_PROM: c_uint = 0x00000008;
pub const FEC_RCNTRL_MII_MODE: c_uint = 0x00000004;
pub const FEC_RCNTRL_DRT: c_uint = 0x00000002;
pub const FEC_RCNTRL_LOOP: c_uint = 0x00000001;
pub const FEC_TCNTRL_FDEN: c_uint = 0x00000004;
pub const FEC_TCNTRL_HBC: c_uint = 0x00000002;
pub const FEC_TCNTRL_GTS: c_uint = 0x00000001;
//
// Delay to wait for FEC reset command to complete (in us)
//
pub const FEC_RESET_DELAY: c_int = 50;
