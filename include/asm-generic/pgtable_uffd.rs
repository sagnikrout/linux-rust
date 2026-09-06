//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/pgtable_uffd.h
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


//
// Some platforms can customize the uffd PTE bit, making it unavailable
// even if the architecture provides the resource.
// Adding this API allows architectures to add their own checks for the
// devices on which the kernel is running.
// Note: When overriding it, please make sure the
// CONFIG_HAVE_ARCH_USERFAULTFD_WP is part of this macro.
//

extern "C" {
    pub fn pgtable_supports_uffd(IS_ENABLED(CONFIG_PTE_MARKER_UFFD_WP: ) &&) -> return;
}

