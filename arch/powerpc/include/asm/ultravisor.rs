//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ultravisor.h
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
// Ultravisor definitions
//
// Copyright 2019, IBM Corporation.
//

//
// In ultravisor enabled systems, PTCR becomes ultravisor privileged only for
// writing and an attempt to write to it will cause a Hypervisor Emulation
// Assistance interrupt.
//
extern "C" {
    pub fn ucall_norets(_arg: UV_WRITE_PATE, _arg: lpid, _arg: dw0, _arg: dw1) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_SHARE_PAGE, _arg: pfn, _arg: npages) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_UNSHARE_PAGE, _arg: pfn, _arg: npages) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_UNSHARE_ALL_PAGES) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_UNREGISTER_MEM_SLOT, _arg: lpid, _arg: slotid) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_PAGE_INVAL, _arg: lpid, _arg: gpa, _arg: page_shift) -> return;
}
extern "C" {
    pub fn ucall_norets(_arg: UV_SVM_TERMINATE, _arg: lpid) -> return;
}
