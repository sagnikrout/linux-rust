//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/elf.h
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
// Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
// Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
// Copyright (C) 2012 Regents of the University of California
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//

// ELF register definitions
pub type elf_greg_t = c_ulong;
pub type elf_gregset_t = user_regs_struct;

// We don't support f without d, or q.
pub type elf_fpreg_t = __u64;
pub type elf_fpregset_t = __riscv_fp_state;

//
// RISC-V relocation types
//
// Relocation types used by the dynamic linker
pub const R_RISCV_NONE: c_int = 0;
pub const R_RISCV_32: c_int = 1;
pub const R_RISCV_64: c_int = 2;
pub const R_RISCV_RELATIVE: c_int = 3;
pub const R_RISCV_COPY: c_int = 4;
pub const R_RISCV_JUMP_SLOT: c_int = 5;
pub const R_RISCV_TLS_DTPMOD32: c_int = 6;
pub const R_RISCV_TLS_DTPMOD64: c_int = 7;
pub const R_RISCV_TLS_DTPREL32: c_int = 8;
pub const R_RISCV_TLS_DTPREL64: c_int = 9;
pub const R_RISCV_TLS_TPREL32: c_int = 10;
pub const R_RISCV_TLS_TPREL64: c_int = 11;
pub const R_RISCV_IRELATIVE: c_int = 58;
// Relocation types not used by the dynamic linker
pub const R_RISCV_BRANCH: c_int = 16;
pub const R_RISCV_JAL: c_int = 17;
pub const R_RISCV_CALL: c_int = 18;
pub const R_RISCV_CALL_PLT: c_int = 19;
pub const R_RISCV_GOT_HI20: c_int = 20;
pub const R_RISCV_TLS_GOT_HI20: c_int = 21;
pub const R_RISCV_TLS_GD_HI20: c_int = 22;
pub const R_RISCV_PCREL_HI20: c_int = 23;
pub const R_RISCV_PCREL_LO12_I: c_int = 24;
pub const R_RISCV_PCREL_LO12_S: c_int = 25;
pub const R_RISCV_HI20: c_int = 26;
pub const R_RISCV_LO12_I: c_int = 27;
pub const R_RISCV_LO12_S: c_int = 28;
pub const R_RISCV_TPREL_HI20: c_int = 29;
pub const R_RISCV_TPREL_LO12_I: c_int = 30;
pub const R_RISCV_TPREL_LO12_S: c_int = 31;
pub const R_RISCV_TPREL_ADD: c_int = 32;
pub const R_RISCV_ADD8: c_int = 33;
pub const R_RISCV_ADD16: c_int = 34;
pub const R_RISCV_ADD32: c_int = 35;
pub const R_RISCV_ADD64: c_int = 36;
pub const R_RISCV_SUB8: c_int = 37;
pub const R_RISCV_SUB16: c_int = 38;
pub const R_RISCV_SUB32: c_int = 39;
pub const R_RISCV_SUB64: c_int = 40;
pub const R_RISCV_GNU_VTINHERIT: c_int = 41;
pub const R_RISCV_GNU_VTENTRY: c_int = 42;
pub const R_RISCV_ALIGN: c_int = 43;
pub const R_RISCV_RVC_BRANCH: c_int = 44;
pub const R_RISCV_RVC_JUMP: c_int = 45;
pub const R_RISCV_GPREL_I: c_int = 47;
pub const R_RISCV_GPREL_S: c_int = 48;
pub const R_RISCV_TPREL_I: c_int = 49;
pub const R_RISCV_TPREL_S: c_int = 50;
pub const R_RISCV_RELAX: c_int = 51;
pub const R_RISCV_SUB6: c_int = 52;
pub const R_RISCV_SET6: c_int = 53;
pub const R_RISCV_SET8: c_int = 54;
pub const R_RISCV_SET16: c_int = 55;
pub const R_RISCV_SET32: c_int = 56;
pub const R_RISCV_32_PCREL: c_int = 57;
pub const R_RISCV_PLT32: c_int = 59;
pub const R_RISCV_SET_ULEB128: c_int = 60;
pub const R_RISCV_SUB_ULEB128: c_int = 61;
