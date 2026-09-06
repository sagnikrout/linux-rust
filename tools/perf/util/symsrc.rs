//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/symsrc.h
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
pub const __PERF_SYMSRC_: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct symsrc {
    pub name: *mut c_char,
    pub fd: c_int,
    pub type: dso_binary_type,

    pub elf: *mut Elf,
    pub ehdr: GElf_Ehdr,
    pub opdsec: *mut Elf_Scn,
    pub opdidx: usize,
    pub opdshdr: GElf_Shdr,
    pub symtab: *mut Elf_Scn,
    pub symtab_idx: usize,
    pub symshdr: GElf_Shdr,
    pub dynsym: *mut Elf_Scn,
    pub dynsym_idx: usize,
    pub dynshdr: GElf_Shdr,
    pub adjust_symbols: bool,
    pub is_64_bit: bool,

}

extern "C" {
    pub fn symsrc__init(ss: *mut symsrc, dso: *mut dso, name: *const c_char, type: dso_binary_type) -> c_int;
}
extern "C" {
    pub fn symsrc__destroy(ss: *mut symsrc);
}
extern "C" {
    pub fn symsrc__has_symtab(ss: *mut symsrc) -> bool;
}
extern "C" {
    pub fn symsrc__possibly_runtime(ss: *mut symsrc) -> bool;
}
