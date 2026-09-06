//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vma/include/custom.h
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
// Contains declarations that exist in the kernel which have been CUSTOMISED for
// testing purposes to faciliate userland VMA testing.
//

//
// The shared stubs do not implement this, it amounts to an fprintf(STDERR,...)
// either way :)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma {
    pub root: *mut anon_vma,
    pub rb_root: rb_root_cached,
// Test fields.
    pub was_cloned: bool,
    pub was_unlinked: bool,
}

// For testing purposes, indicate that the anon_vma was unlinked.
// Used to indicate to tests that a write operation has begun.
// For testing purposes. We indicate that an anon_vma has been cloned.
extern "C" {
    pub fn __anon_vma_prepare(_arg: vma) -> return;
}
