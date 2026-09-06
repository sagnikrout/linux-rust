//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/oa_tc6.h
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
// OPEN Alliance 10BASE‑T1x MAC‑PHY Serial Interface framework
//
// Link: https://opensig.org/download/document/OPEN_Alliance_10BASET1x_MAC-PHY_Serial_Interface_V1.1.pdf
//
// Author: Parthiban Veerasooran <parthiban.veerasooran@microchip.com>
//

// OPEN Alliance TC6 registers
// Standard Capabilities Register
pub const OA_TC6_REG_STDCAP: c_uint = 0x0002;

// Reset Control and Status Register
pub const OA_TC6_REG_RESET: c_uint = 0x0003;

// Configuration Register #0
pub const OA_TC6_REG_CONFIG0: c_uint = 0x0004;

// Configuration Register #2
pub const OA_TC6_REG_CONFIG2: c_uint = 0x0006;
// Status Register #0
pub const OA_TC6_REG_STATUS0: c_uint = 0x0008;

// Buffer Status Register
pub const OA_TC6_REG_BUFFER_STATUS: c_uint = 0x000B;

// Interrupt Mask Register #0
pub const OA_TC6_REG_INT_MASK0: c_uint = 0x000C;

// PHY Clause 22 registers base address and mask
pub const OA_TC6_PHY_STD_REG_ADDR_BASE: c_uint = 0xFF00;
pub const OA_TC6_PHY_STD_REG_ADDR_MASK: c_uint = 0x1F;
// Memory map selector (MMS) values as per table 6 in the
// OPEN Alliance specification.
//
pub const OA_TC6_MAC_MMS1: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum oa_tc6_quirk_flag {
    OA_TC6_BROKEN_PHY = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oa_tc6_quirks {
    pub quirk_flags: oa_tc6_quirk_flag,
}

extern "C" {
    pub fn oa_tc6_exit(tc6: *mut oa_tc6);
}
extern "C" {
    pub fn oa_tc6_write_register(tc6: *mut oa_tc6, address: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn oa_tc6_read_register(tc6: *mut oa_tc6, address: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn oa_tc6_start_xmit(tc6: *mut oa_tc6, skb: *mut sk_buff) -> netdev_tx_t;
}
extern "C" {
    pub fn oa_tc6_zero_align_receive_frame_enable(tc6: *mut oa_tc6) -> c_int;
}
