//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cn20k/api.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2024 Marvell.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ng_rvu {
    pub rvu_mbox_ops: *mut mbox_ops,
    pub pf_mbox_addr: *mut qmem,
    pub vf_mbox_addr: *mut qmem,
}

// Mbox related APIs
extern "C" {
    pub fn cn20k_rvu_mbox_init(rvu: *mut rvu, type: c_int, num: c_int) -> c_int;
}
extern "C" {
    pub fn cn20k_free_mbox_memory(rvu: *mut rvu);
}
extern "C" {
    pub fn cn20k_free_mbox_memory_type(rvu: *mut rvu, type: c_int);
}
extern "C" {
    pub fn cn20k_register_afpf_mbox_intr(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn cn20k_register_afvf_mbox_intr(rvu: *mut rvu, pf_vec_start: c_int) -> c_int;
}
extern "C" {
    pub fn cn20k_rvu_enable_mbox_intr(rvu: *mut rvu);
}
extern "C" {
    pub fn cn20k_rvu_unregister_interrupts(rvu: *mut rvu);
}
extern "C" {
    pub fn cn20k_rvu_enable_afvf_intr(rvu: *mut rvu, vfs: c_int);
}
extern "C" {
    pub fn cn20k_rvu_disable_afvf_intr(rvu: *mut rvu, vfs: c_int);
}
