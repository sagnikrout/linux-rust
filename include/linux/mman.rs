//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mman.h
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
// Arrange for legacy / undefined architecture specific flags to be
// ignored by mmap handling code.
//

pub const MAP_32BIT: c_int = 0;

pub const MAP_ABOVE4G: c_int = 0;

pub const MAP_HUGE_2MB: c_int = 0;

pub const MAP_HUGE_1GB: c_int = 0;

pub const MAP_UNINITIALIZED: c_int = 0;

pub const MAP_SYNC: c_int = 0;

//
// The historical set of flags that all mmap implementations implicitly
// support when a ->mmap_validate() op is not provided in file_operations.
//
// MAP_EXECUTABLE and MAP_DENYWRITE are completely ignored throughout the
// kernel.
//

extern "C" {
    pub fn mm_compute_batch(overcommit_policy: c_int);
}

pub const vm_committed_as_batch: c_int = 0;

extern "C" {
    pub fn vm_memory_committed() -> c_ulong;
}
//
// Allow architectures to handle additional protection and flag bits. The
// overriding macros must be defined in the arch-specific asm/mman.h file.
//

//
// This is called from mprotect().  PROT_GROWSDOWN and PROT_GROWSUP have
// already been masked out.
//
// Returns true if the prot flags are valid
//

//
// This is called from mmap() and mprotect() with the updated vma->vm_flags.
//
// Returns true if the VM_* flags are valid.
//

//
// Optimisation macro.  It is equivalent to:
// (x & bit1) ? bit2 : 0
// but this version is faster.
// ("bit1" and "bit2" must be single bits)
//

//
// Combine the mmap "prot" argument into "vm_flags" used internally.
//
// Combine the mmap "flags" argument into "vm_flags" used internally.
//

extern "C" {
    pub fn vm_commit_limit() -> c_ulong;
}

