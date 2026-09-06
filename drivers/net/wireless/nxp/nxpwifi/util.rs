//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/nxp/nxpwifi/util.h
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
// NXP Wireless LAN device driver: utility functions
//
// Copyright 2011-2024 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_dma_mapping {
    pub addr: dma_addr_t,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_cb {
    pub dma_mapping: nxpwifi_dma_mapping,
    pub rx_info: nxpwifi_rxinfo,
    pub tx_info: nxpwifi_txinfo,
}

// size/addr for nxpwifi_debug_info

// size/addr for struct nxpwifi_adapter

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nxpwifi_debug_data {
    pub /: *mut *mut char name[32]; / variable/array name,
    pub /: *mut *mut u32 size; / size of the variable/array,
    pub /: *mut *mut size_t addr; / address of the variable/array,
    pub /: *mut *mut int num; / number of variables in an array,
}

//
// Iterate over TLVs safely.
// Ensures no out-of-bound access even if firmware sends malformed data.
//

// Return first TLV matching @type in given buffer.
extern "C" {
    pub fn nxpwifi_append_data_tlv(id: u16, data: *mut u8, len: c_int, pos: *mut u8, cmd_end: *mut u8) -> c_int;
}
extern "C" {
    pub fn nxpwifi_roc_cookie(adapter: *mut nxpwifi_adapter) -> u64;
}
//
// Firmware cannot run AP and STA on different channels simultaneously,
// and doing so may trigger a crash. Check whether check_chan can be set
// safely; return true if allowed, false if another channel is already
// active in firmware.
//
