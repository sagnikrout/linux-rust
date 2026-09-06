//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/libunwind-arch/libunwind-arch.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_info {
    pub machine: *mut machine,
    pub thread: *mut thread,
    pub sample: *mut perf_sample,
    pub cursor: *mut c_void,
    pub ips: *mut u64,
    pub cur_ip: c_int,
    pub max_ips: c_int,
    pub unw_word_t_size: c_uint,
    pub e_machine: u16,
    pub best_effort: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libarch_unwind__dyn_info {
    pub start_ip: u64,
    pub end_ip: u64,
    pub segbase: u64,
    pub table_data: u64,
    pub table_len: u64,
}

extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_arm(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_arm64(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_loongarch(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_mips(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_ppc32(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_ppc64(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_riscv(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_s390(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_i386(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __get_perf_regnum_for_unw_regnum_x86_64(unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn get_perf_regnum_for_unw_regnum(e_machine: c_uint, unw_regnum: c_int) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__flush_access_arm(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_arm64(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_loongarch(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_mips(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_ppc32(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_ppc64(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_riscv(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_s390(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_i386(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__flush_access_x86_64(maps: *mut maps);
}
extern "C" {
    pub fn libunwind_arch__flush_access(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_arm(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_arm64(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_loongarch(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_mips(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_ppc32(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_ppc64(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_riscv(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_s390(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_i386(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind_arch__finish_access_x86_64(maps: *mut maps);
}
extern "C" {
    pub fn libunwind_arch__finish_access(maps: *mut maps);
}
extern "C" {
    pub fn __libunwind__find_proc_info(as: *mut c_void, ip: u64, pi: *mut c_void, need_unwind_info: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __libunwind__access_mem(as: *mut c_void, addr: u64, valp_word: *mut c_void, __write: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn __libunwind__access_reg(as: *mut c_void, regnum: c_int, valp_word: *mut c_void, __write: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn libunwind_arch_unwind_info__delete(ui: *mut unwind_info);
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_arm(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_arm64(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_loongarch(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_mips(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_ppc32(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_ppc64(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_riscv(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_s390(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_i386(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn __libunwind_arch__unwind_step_x86_64(ui: *mut unwind_info) -> c_int;
}
extern "C" {
    pub fn libunwind_arch__unwind_step(ui: *mut unwind_info) -> c_int;
}
