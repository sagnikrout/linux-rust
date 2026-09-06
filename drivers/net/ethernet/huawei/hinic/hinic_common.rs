//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/huawei/hinic/hinic_common.h
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
// Huawei HiNIC PCI Express Linux driver
// Copyright(c) 2017 Huawei Technologies Co., Ltd
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hinic_sge {
    pub hi_addr: u32,
    pub lo_addr: u32,
    pub len: u32,
}

extern "C" {
    pub fn hinic_cpu_to_be32(data: *mut c_void, len: c_int);
}
extern "C" {
    pub fn hinic_be32_to_cpu(data: *mut c_void, len: c_int);
}
extern "C" {
    pub fn hinic_set_sge(sge: *mut hinic_sge, addr: dma_addr_t, len: c_int);
}
extern "C" {
    pub fn hinic_sge_to_dma(sge: *mut hinic_sge) -> dma_addr_t;
}
