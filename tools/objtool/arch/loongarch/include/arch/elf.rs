//! Automatically rewritten from C Header to Rust Module
//! Source: tools/objtool/arch/loongarch/include/arch/elf.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// See the following link for more info about ELF Relocation types:
// https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html#_relocations
//

pub const R_LARCH_NONE: c_int = 0;

pub const R_LARCH_32: c_int = 1;

pub const R_LARCH_64: c_int = 2;

pub const R_LARCH_32_PCREL: c_int = 99;

pub const R_LARCH_64_PCREL: c_int = 109;

pub const EM_LOONGARCH: c_int = 258;

