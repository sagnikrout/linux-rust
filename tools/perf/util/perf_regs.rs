//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/perf_regs.h
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

extern "C" {
    pub fn perf_sdt_arg_parse_op(e_machine: u16, old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn perf_intr_reg_mask(e_machine: u16) -> u64;
}
extern "C" {
    pub fn perf_user_reg_mask(e_machine: u16) -> u64;
}
extern "C" {
    pub fn perf_reg_value(valp: *mut u64, regs: *mut regs_dump, id: c_int) -> c_int;
}
extern "C" {
    pub fn perf_arch_reg_ip(e_machine: u16) -> u64;
}
extern "C" {
    pub fn perf_arch_reg_sp(e_machine: u16) -> u64;
}
extern "C" {
    pub fn __perf_sdt_arg_parse_op_arm64(old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __perf_reg_mask_arm64(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_arm64() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_arm64() -> u64;
}
extern "C" {
    pub fn __perf_reg_mask_arm(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_arm() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_arm() -> u64;
}
extern "C" {
    pub fn __perf_reg_mask_csky(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_csky() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_csky() -> u64;
}
extern "C" {
    pub fn __perf_reg_mask_loongarch(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_loongarch() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_loongarch() -> u64;
}
extern "C" {
    pub fn __perf_reg_mask_mips(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_mips() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_mips() -> u64;
}
extern "C" {
    pub fn __perf_sdt_arg_parse_op_powerpc(old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __perf_reg_mask_powerpc(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_powerpc() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_powerpc() -> u64;
}
extern "C" {
    pub fn __perf_sdt_arg_parse_op_riscv(old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __perf_reg_mask_riscv(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_riscv() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_riscv() -> u64;
}
extern "C" {
    pub fn __perf_reg_mask_s390(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_s390() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_s390() -> u64;
}
extern "C" {
    pub fn __perf_sdt_arg_parse_op_s390(old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __perf_sdt_arg_parse_op_x86(old_op: *mut c_char, new_op: *mut c_char) -> c_int;
}
extern "C" {
    pub fn __perf_reg_mask_x86(intr: bool) -> u64;
}
extern "C" {
    pub fn __perf_reg_ip_x86() -> u64;
}
extern "C" {
    pub fn __perf_reg_sp_x86() -> u64;
}
