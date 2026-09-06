//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/alternative-macros.h
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
// Usage:
// ALTERNATIVE(old_content, new_content, vendor_id, patch_id, CONFIG_k)
// in the assembly code. Otherwise,
// asm(ALTERNATIVE(old_content, new_content, vendor_id, patch_id, CONFIG_k));
//
// old_content: The old content which is probably replaced with new content.
// new_content: The new content.
// vendor_id: The CPU vendor ID.
// patch_id: The patch ID (erratum ID or cpufeature ID).
// CONFIG_k: The Kconfig of this patch ID. When Kconfig is disabled, the old
// content will always be executed.
//

//
// A vendor wants to replace an old_content, but another vendor has used
// ALTERNATIVE() to patch its customized content at the same location. In
// this case, this vendor can create a new macro ALTERNATIVE_2() based
// on the following sample code and then replace ALTERNATIVE() with
// ALTERNATIVE_2() to append its customized content.
//

