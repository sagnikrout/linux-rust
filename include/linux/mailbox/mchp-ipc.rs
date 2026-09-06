//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mailbox/mchp-ipc.h
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
// Copyright (c) 2024 Microchip Technology Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_ipc_msg {
    pub buf: *mut u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_ipc_sbi_chan {
    pub buf_base_tx: *mut c_void,
    pub buf_base_rx: *mut c_void,
    pub msg_buf_tx: *mut c_void,
    pub msg_buf_rx: *mut c_void,
    pub buf_base_tx_addr: phys_addr_t,
    pub buf_base_rx_addr: phys_addr_t,
    pub msg_buf_tx_addr: phys_addr_t,
    pub msg_buf_rx_addr: phys_addr_t,
    pub chan_aggregated_irq: c_int,
    pub mp_irq: c_int,
    pub mc_irq: c_int,
    pub id: u32,
    pub max_msg_size: u32,
}
