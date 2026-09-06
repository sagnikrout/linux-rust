//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/bpf_common.h
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
// Instruction classes

pub const BPF_LD: c_uint = 0x00;
pub const BPF_LDX: c_uint = 0x01;
pub const BPF_ST: c_uint = 0x02;
pub const BPF_STX: c_uint = 0x03;
pub const BPF_ALU: c_uint = 0x04;
pub const BPF_JMP: c_uint = 0x05;
pub const BPF_RET: c_uint = 0x06;
pub const BPF_MISC: c_uint = 0x07;
// ld/ldx fields

pub const BPF_W: c_uint = 0x00 /* 32-bit */;
pub const BPF_H: c_uint = 0x08 /* 16-bit */;
pub const BPF_B: c_uint = 0x10 /*  8-bit */;
// eBPF		BPF_DW		0x18    64-bit

pub const BPF_IMM: c_uint = 0x00;
pub const BPF_ABS: c_uint = 0x20;
pub const BPF_IND: c_uint = 0x40;
pub const BPF_MEM: c_uint = 0x60;
pub const BPF_LEN: c_uint = 0x80;
pub const BPF_MSH: c_uint = 0xa0;
// alu/jmp fields

pub const BPF_ADD: c_uint = 0x00;
pub const BPF_SUB: c_uint = 0x10;
pub const BPF_MUL: c_uint = 0x20;
pub const BPF_DIV: c_uint = 0x30;
pub const BPF_OR: c_uint = 0x40;
pub const BPF_AND: c_uint = 0x50;
pub const BPF_LSH: c_uint = 0x60;
pub const BPF_RSH: c_uint = 0x70;
pub const BPF_NEG: c_uint = 0x80;
pub const BPF_MOD: c_uint = 0x90;
pub const BPF_XOR: c_uint = 0xa0;
pub const BPF_JA: c_uint = 0x00;
pub const BPF_JEQ: c_uint = 0x10;
pub const BPF_JGT: c_uint = 0x20;
pub const BPF_JGE: c_uint = 0x30;
pub const BPF_JSET: c_uint = 0x40;

pub const BPF_K: c_uint = 0x00;
pub const BPF_X: c_uint = 0x08;

pub const BPF_MAXINSNS: c_int = 4096;

