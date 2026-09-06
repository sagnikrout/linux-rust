//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_cudbg.h
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
// Copyright (C) 2017 Chelsio Communications.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_collect_entity {
    pub entity: cudbg_dbg_entity_type,
    pub collect_cb: cudbg_collect_callback_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CXGB4_ETHTOOL_DUMP_FLAGS {
    CXGB4_ETH_DUMP_NONE = ETH_FW_DUMP_DISABLE,
    CXGB4_ETH_DUMP_MEM = (1 << 0), /* On-Chip Memory Dumps */
    CXGB4_ETH_DUMP_HW = (1 << 1), /* various FW and HW dumps */
    CXGB4_ETH_DUMP_FLASH = (1 << 2), /* Dump flash memory */
}

extern "C" {
    pub fn cxgb4_get_dump_length(adap: *mut adapter, flag: u32) -> u32;
}
extern "C" {
    pub fn cxgb4_init_ethtool_dump(adapter: *mut adapter);
}
extern "C" {
    pub fn cxgb4_cudbg_vmcore_add_dump(adap: *mut adapter) -> c_int;
}
