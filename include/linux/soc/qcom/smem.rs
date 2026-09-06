//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/qcom/smem.h
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

extern "C" {
    pub fn qcom_smem_is_available() -> bool;
}
extern "C" {
    pub fn qcom_smem_alloc(host: unsigned, item: unsigned, size: usize) -> c_int;
}
extern "C" {
    pub fn qcom_smem_get_free_space(host: unsigned) -> c_int;
}
extern "C" {
    pub fn qcom_smem_virt_to_phys(p: *mut c_void) -> phys_addr_t;
}
extern "C" {
    pub fn qcom_smem_get_soc_id(id: *mut u32) -> c_int;
}
extern "C" {
    pub fn qcom_smem_get_feature_code(code: *mut u32) -> c_int;
}
extern "C" {
    pub fn qcom_smem_bust_hwspin_lock_by_host(host: c_uint) -> c_int;
}
extern "C" {
    pub fn qcom_smem_dram_get_hbb() -> c_int;
}
