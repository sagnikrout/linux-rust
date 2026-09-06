//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/inat.h
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
// x86 instruction attributes
//
// Written by Masami Hiramatsu <mhiramat@redhat.com>
//

//
// Internal bits. Don't use bitmasks directly, because these bits are
// unstable. You should use checking functions.
//
pub const INAT_OPCODE_TABLE_SIZE: c_int = 256;
pub const INAT_GROUP_TABLE_SIZE: c_int = 8;
// Legacy last prefixes

// Other Legacy prefixes

// x86-64 REX prefix

// AVX VEX prefixes

// x86-64 REX2 prefix

// AMD XOP prefix

pub const INAT_LSTPFX_MAX: c_int = 3;
pub const INAT_LGCPFX_MAX: c_int = 11;
// Immediate size
pub const INAT_IMM_BYTE: c_int = 1;
pub const INAT_IMM_WORD: c_int = 2;
pub const INAT_IMM_DWORD: c_int = 3;
pub const INAT_IMM_QWORD: c_int = 4;
pub const INAT_IMM_PTR: c_int = 5;
pub const INAT_IMM_VWORD32: c_int = 6;
pub const INAT_IMM_VWORD: c_int = 7;
// Legacy prefix
pub const INAT_PFX_OFFS: c_int = 0;
pub const INAT_PFX_BITS: c_int = 5;

// Escape opcodes

pub const INAT_ESC_BITS: c_int = 2;

// Group opcodes (1-16)

pub const INAT_GRP_BITS: c_int = 5;

// Immediates

pub const INAT_IMM_BITS: c_int = 3;

// Flags

// Attribute making macros for attribute tables

// Identifiers for segment registers
pub const INAT_SEG_REG_IGNORE: c_int = 0;
pub const INAT_SEG_REG_DEFAULT: c_int = 1;
pub const INAT_SEG_REG_CS: c_int = 2;
pub const INAT_SEG_REG_SS: c_int = 3;
pub const INAT_SEG_REG_DS: c_int = 4;
pub const INAT_SEG_REG_ES: c_int = 5;
pub const INAT_SEG_REG_FS: c_int = 6;
pub const INAT_SEG_REG_GS: c_int = 7;
// Attribute search APIs
extern "C" {
    pub fn inat_get_opcode_attribute(opcode: insn_byte_t) -> insn_attr_t;
}
extern "C" {
    pub fn inat_get_last_prefix_id(last_pfx: insn_byte_t) -> c_int;
}
// Attribute checking functions
