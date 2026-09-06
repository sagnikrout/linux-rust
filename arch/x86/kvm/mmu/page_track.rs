//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/page_track.h
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
    pub fn kvm_page_track_write_tracking_enabled(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_page_track_write_tracking_alloc(slot: *mut kvm_memory_slot) -> c_int;
}
extern "C" {
    pub fn kvm_page_track_free_memslot(slot: *mut kvm_memory_slot);
}

extern "C" {
    pub fn kvm_page_track_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_page_track_cleanup(kvm: *mut kvm);
}
extern "C" {
    pub fn __kvm_page_track_write(kvm: *mut kvm, gpa: gpa_t, new: *const u8, bytes: c_int);
}
extern "C" {
    pub fn kvm_page_track_delete_slot(kvm: *mut kvm, slot: *mut kvm_memory_slot);
}

