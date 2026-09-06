//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/mm.h
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

extern "C" {
    pub fn hyp_create_fixmap() -> c_int;
}
extern "C" {
    pub fn hyp_fixmap_unmap();
}
extern "C" {
    pub fn hyp_fixblock_unmap();
}
extern "C" {
    pub fn hyp_create_idmap(hyp_va_bits: u32) -> c_int;
}
extern "C" {
    pub fn hyp_map_vectors() -> c_int;
}
extern "C" {
    pub fn hyp_back_vmemmap(back: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn pkvm_cpu_set_vector(slot: arm64_hyp_spectre_vector) -> c_int;
}
extern "C" {
    pub fn pkvm_create_mappings(from: *mut c_void, to: *mut c_void, prot: kvm_pgtable_prot) -> c_int;
}
extern "C" {
    pub fn pkvm_create_mappings_locked(from: *mut c_void, to: *mut c_void, prot: kvm_pgtable_prot) -> c_int;
}
extern "C" {
    pub fn pkvm_create_stack(phys: phys_addr_t, haddr: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn pkvm_alloc_private_va_range(size: usize, haddr: *mut c_ulong) -> c_int;
}
