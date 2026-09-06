//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/include/dwarf-regs.h
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

// EM_HOST gives the ELF machine for host, EF_HOST gives additional flags.

// Unknown host ELF machine type.

pub const EF_HOST: c_int = 0;

pub const DWARF_REG_PC: c_uint = 0xd3af9c /* random number */;
pub const DWARF_REG_FB: c_uint = 0xd3affb /* random number */;

//
// get_dwarf_regstr() - Returns ftrace register string from DWARF regnum.
// @n: DWARF register number.
// @machine: ELF machine signature (EM_*).
// @flags: ELF flags for things like ABI differences.
//
extern "C" {
    pub fn __get_csky_regnum(name: *const c_char, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_i386(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_x86_64(name: *const c_char) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_csky(perf_regnum: c_int, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum: c_int) -> c_int;
}
//
// get_dwarf_regnum - Returns DWARF regnum from register name
// name: architecture register name
// machine: ELF machine signature (EM_*)
//
extern "C" {
    pub fn get_dwarf_regnum(name: *const c_char, machine: c_uint, flags: c_uint) -> c_int;
}
//
// get_dwarf_regnum - Returns DWARF regnum from perf register number.
//
extern "C" {
    pub fn get_powerpc_regs(raw_insn: u32, is_source: c_int, op_loc: *mut annotated_op_loc);
}

