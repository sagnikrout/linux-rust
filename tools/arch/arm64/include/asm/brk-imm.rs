//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/arm64/include/asm/brk-imm.h
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
// Copyright (C) 2012 ARM Ltd.
//
// #imm16 values used for BRK instruction generation
// 0x004: for installing kprobes
// 0x005: for installing uprobes
// 0x006: for kprobe software single-step
// 0x007: for kretprobe return
// Allowed values for kgdb are 0x400 - 0x7ff
// 0x100: for triggering a fault on purpose (reserved)
// 0x400: for dynamic BRK instruction
// 0x401: for compile time BRK instruction
// 0x800: kernel-mode BUG() and WARN() traps
// 0x9xx: tag-based KASAN trap (allowed values 0x900 - 0x9ff)
// 0x55xx: Undefined Behavior Sanitizer traps ('U' << 8)
// 0x8xxx: Control-Flow Integrity traps
//
pub const KPROBES_BRK_IMM: c_uint = 0x004;
pub const UPROBES_BRK_IMM: c_uint = 0x005;
pub const KPROBES_BRK_SS_IMM: c_uint = 0x006;
pub const KRETPROBES_BRK_IMM: c_uint = 0x007;
pub const FAULT_BRK_IMM: c_uint = 0x100;
pub const KGDB_DYN_DBG_BRK_IMM: c_uint = 0x400;
pub const KGDB_COMPILED_DBG_BRK_IMM: c_uint = 0x401;
pub const BUG_BRK_IMM: c_uint = 0x800;
pub const KASAN_BRK_IMM: c_uint = 0x900;
pub const KASAN_BRK_MASK: c_uint = 0x0ff;
pub const UBSAN_BRK_IMM: c_uint = 0x5500;
pub const UBSAN_BRK_MASK: c_uint = 0x00ff;

pub const CFI_BRK_IMM_BASE: c_uint = 0x8000;

