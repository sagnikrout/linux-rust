//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/keyboard.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const KG_SHIFT: c_int = 0;
pub const KG_CTRL: c_int = 2;
pub const KG_ALT: c_int = 3;
pub const KG_ALTGR: c_int = 1;
pub const KG_SHIFTL: c_int = 4;
pub const KG_KANASHIFT: c_int = 4;
pub const KG_SHIFTR: c_int = 5;
pub const KG_CTRLL: c_int = 6;
pub const KG_CTRLR: c_int = 7;
pub const KG_CAPSSHIFT: c_int = 8;
pub const NR_SHIFT: c_int = 9;
pub const NR_KEYS: c_int = 256;
pub const MAX_NR_KEYMAPS: c_int = 256;
// This means 128Kb if all keymaps are allocated. Only the superuser

pub const KT_FN: c_int = 1;
pub const KT_SPEC: c_int = 2;
pub const KT_PAD: c_int = 3;
pub const KT_DEAD: c_int = 4;
pub const KT_CONS: c_int = 5;
pub const KT_CUR: c_int = 6;
pub const KT_SHIFT: c_int = 7;
pub const KT_META: c_int = 8;
pub const KT_ASCII: c_int = 9;
pub const KT_LOCK: c_int = 10;

pub const KT_SLOCK: c_int = 12;
pub const KT_DEAD2: c_int = 13;
pub const KT_BRL: c_int = 14;

pub const NR_PAD: c_int = 20;

pub const NR_DEAD: c_int = 27;

pub const NR_ASCII: c_int = 26;

pub const NR_LOCK: c_int = 9;

pub const NR_BRL: c_int = 11;
// KT_CSI keys: value is the CSI parameter number for ESC [ <value> ~

pub const MAX_DIACR: c_int = 256;
