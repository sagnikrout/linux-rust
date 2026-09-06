//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/module.h
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

pub const RELA_STACK_DEPTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_section {
    pub shndx: c_int,
    pub num_entries: c_int,
    pub max_entries: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_specific {
    pub got: mod_section,
    pub plt: mod_section,
    pub plt_idx: mod_section,

    pub num_orcs: c_uint,
    pub orc_unwind_ip: *mut c_int,
    pub orc_unwind: *mut orc_entry,

// For CONFIG_DYNAMIC_FTRACE
    pub ftrace_trampolines: *mut plt_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct got_entry {
    pub symbol_addr: Elf_Addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plt_entry {
    pub inst_lu12iw: u32,

    pub inst_lu32id: u32,
    pub inst_lu52id: u32,

    pub inst_jirl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plt_idx_entry {
    pub symbol_addr: Elf_Addr,
}

extern "C" {
    pub fn module_emit_got_entry(mod: *mut module, sechdrs: *mut Elf_Shdr, val: Elf_Addr) -> Elf_Addr;
}
extern "C" {
    pub fn module_emit_plt_entry(mod: *mut module, sechdrs: *mut Elf_Shdr, val: Elf_Addr) -> Elf_Addr;
}

