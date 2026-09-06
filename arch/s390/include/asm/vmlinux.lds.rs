//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/vmlinux.lds.h
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
// .boot.data section is shared between the decompressor code and the
// decompressed kernel. The decompressor will store values in it, and copy
// over to the decompressed image before starting it.
//
// .boot.data variables are kept in separate .boot.data.<var name> sections,
// which are sorted by alignment first, then by name before being merged
// into single .boot.data section. This way big holes cased by page aligned
// structs are avoided and linker produces consistent result.
//

// (SORT_BY_ALIGNMENT(SORT_BY_NAME(.boot.data*)))		\
//
// .boot.preserved.data is similar to .boot.data, but it is not part of the
// .init section and thus will be preserved for later use in the decompressed
// kernel.
//

// (SORT_BY_ALIGNMENT(SORT_BY_NAME(.boot.preserved.data*))) \
