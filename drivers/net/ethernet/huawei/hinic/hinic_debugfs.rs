//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_debugfs.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

pub const TBL_ID_FUNC_CFG_SM_NODE: c_int = 11;
pub const TBL_ID_FUNC_CFG_SM_INST: c_int = 1;
pub const HINIC_FUNCTION_CONFIGURE_TABLE_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_cmd_lt_rd {
    pub status: u8,
    pub version: u8,
    pub rsvd0: [u8; 6],
    pub node: c_uchar,
    pub inst: c_uchar,
    pub entry_size: c_uchar,
    pub rsvd: c_uchar,
    pub lt_index: c_uint,
    pub offset: c_uint,
    pub len: c_uint,
    pub data: [c_uchar; 100],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tag_sml_funcfg_tbl {
    pub :8: u32 rsvd0,
    pub :5: u32 nic_rx_mode,
    pub :18: u32 rsvd1,
    pub :1: u32 valid,
    pub bs: },
    pub value: u32,
    pub dw0: },
    pub :12: u32 vlan_id,
    pub :3: u32 vlan_mode,
    pub :1: u32 fast_recycled_mode,
    pub :16: u32 mtu,
    pub bs: },
    pub value: u32,
    pub dw1: },
    pub dw2: u32,
    pub dw3: u32,
    pub dw4: u32,
    pub dw5: u32,
    pub dw6: u32,
    pub dw7: u32,
    pub dw8: u32,
    pub dw9: u32,
    pub dw10: u32,
    pub dw11: u32,
    pub dw12: u32,
    pub :15: u32 rsvd2,
    pub :9: u32 cfg_q_num,
    pub :6: u32 cfg_rq_depth,
    pub :2: u32 vhd_type,
    pub bs: },
    pub value: u32,
    pub dw13: },
    pub dw14: u32,
    pub dw15: u32,
}

extern "C" {
    pub fn hinic_sq_debug_add(dev: *mut hinic_dev, sq_id: u16) -> c_int;
}
extern "C" {
    pub fn hinic_sq_debug_rem(sq: *mut hinic_sq);
}
extern "C" {
    pub fn hinic_rq_debug_add(dev: *mut hinic_dev, rq_id: u16) -> c_int;
}
extern "C" {
    pub fn hinic_rq_debug_rem(rq: *mut hinic_rq);
}
extern "C" {
    pub fn hinic_func_table_debug_add(dev: *mut hinic_dev) -> c_int;
}
extern "C" {
    pub fn hinic_func_table_debug_rem(dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_sq_dbgfs_init(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_sq_dbgfs_uninit(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_rq_dbgfs_init(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_rq_dbgfs_uninit(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_func_tbl_dbgfs_init(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_func_tbl_dbgfs_uninit(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_dbg_init(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_dbg_uninit(nic_dev: *mut hinic_dev);
}
extern "C" {
    pub fn hinic_dbg_register_debugfs(debugfs_dir_name: *const c_char);
}
extern "C" {
    pub fn hinic_dbg_unregister_debugfs();
}
