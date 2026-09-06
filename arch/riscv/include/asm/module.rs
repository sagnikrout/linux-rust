//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/module.h
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
// Copyright (C) 2017 Andes Technology Corporation

extern "C" {
    pub fn module_emit_got_entry(mod: *mut module, val: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn module_emit_plt_entry(mod: *mut module, val: c_ulong) -> c_ulong;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_section {
    pub shdr: *mut Elf_Shdr,
    pub num_entries: c_int,
    pub max_entries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_specific {
    pub got: mod_section,
    pub plt: mod_section,
    pub got_plt: mod_section,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct got_entry {
    pub /: *mut *mut unsigned long symbol_addr; / the real variable address,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plt_entry {
//
// Trampoline code to real target address. The return address
// should be the original (pc+4) before entring plt entry.
//
    pub /: *mut *mut u32 insn_auipc; / auipc t0, 0x0,
    pub /: *mut *mut u32 insn_ld; / ld t1, 0x10(t0),
    pub /: *mut *mut u32 insn_jr; / jr t1,
}

pub const OPC_AUIPC: c_uint = 0x0017;
pub const OPC_LD: c_uint = 0x3003;
pub const OPC_JALR: c_uint = 0x0067;
pub const REG_T0: c_uint = 0x5;
pub const REG_T1: c_uint = 0x6;
//
// U-Type encoding:
// +------------+----------+----------+
// | imm[31:12] | rd[11:7] | opc[6:0] |
// +------------+----------+----------+
//
// I-Type encoding:
// +------------+------------+--------+----------+----------+
// | imm[31:20] | rs1[19:15] | funct3 | rd[11:7] | opc[6:0] |
// +------------+------------+--------+----------+----------+
//

