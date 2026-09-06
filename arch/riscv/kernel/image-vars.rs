//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/kernel/image-vars.h
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
//
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
// Linker script variables to be set after section resolution, as
// ld.lld does not like variables assigned before SECTIONS is processed.
// Based on arch/arm64/kernel/image-vars.h
//

//
// The EFI stub has its own symbol namespace prefixed by __efistub_, to
// isolate it from the kernel proper. The following symbols are legally
// accessed by the stub, so provide some aliases to make them accessible.
// Only include data symbols here, or text symbols of functions that are
// guaranteed to be safe when executed at another offset than they were
// linked at. The routines below are all implemented in assembler in a
// position independent manner
//

//
// These double-word integer shifts are used by the library code, and
// the first two of them are required to link EFI stub. Note __ashrdi3()
// is not actually used by the stub but this may change in the future.
//

