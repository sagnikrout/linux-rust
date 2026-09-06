//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hns/hns_roce_debugfs.h
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
// Copyright (c) 2023 Hisilicon Limited.
//
// debugfs seqfile
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_debugfs_seqfile {
    pub data): *mut *mut *mut int (read)(struct seq_file seq, void,
    pub data): *mut *mut *mut ssize_t (write)(char buf, size_t count, void,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_sw_stat_debugfs {
    pub root: *mut dentry,
    pub sw_stat: hns_debugfs_seqfile,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_cc_param_attr {
    pub name: *const c_char,
    pub algo_type: c_int,
    pub offset: u32,
    pub size: u32,
    pub max: u32,
    pub min: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_cc_param_seqfile {
    pub seqfile: hns_debugfs_seqfile,
    pub param_attr: *const hns_roce_cc_param_attr,
    pub index: c_int,
}

pub const HNS_ROCE_CC_PARAM_MAX_NUM: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_cc_param_debugfs {
    pub root: *mut dentry,
    pub params: [hns_cc_param_seqfile; HNS_ROCE_CC_PARAM_MAX_NUM],
}

pub const CONG_TYPE_MAX_NUM: c_int = 4;
// Debugfs for device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns_roce_dev_debugfs {
    pub root: *mut dentry,
    pub sw_stat_root: hns_sw_stat_debugfs,
    pub cc_param_root: [hns_cc_param_debugfs; CONG_TYPE_MAX_NUM],
}

extern "C" {
    pub fn hns_roce_init_debugfs();
}
extern "C" {
    pub fn hns_roce_cleanup_debugfs();
}
extern "C" {
    pub fn hns_roce_register_debugfs(hr_dev: *mut hns_roce_dev);
}
extern "C" {
    pub fn hns_roce_unregister_debugfs(hr_dev: *mut hns_roce_dev);
}
