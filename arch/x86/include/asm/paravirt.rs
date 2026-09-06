//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/paravirt.h
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
// Various instructions on x86 need to be replaced for
// para-virtualization: those hooks are defined here.

extern "C" {
    pub fn native_flush_tlb_local();
}
extern "C" {
    pub fn native_flush_tlb_global();
}
extern "C" {
    pub fn native_flush_tlb_one_user(addr: c_ulong);
}

// The paravirtualized CPUID instruction.
//
// These special macros can be used to get or set a debugging register
//
extern "C" {
    pub fn PVOP_CALL1(long: unsigned, _arg: pv_ops, _arg: cpu.get_debugreg, _arg: reg) -> return;
}

extern "C" {
    pub fn PVOP_CALL0(long: unsigned, _arg: pv_ops, _arg: cpu.read_cr0) -> return;
}
extern "C" {
    pub fn PVOP_CALL1(_arg: u64, _arg: pv_ops, _arg: cpu.read_msr, _arg: msr) -> return;
}
extern "C" {
    pub fn PVOP_CALL2(_arg: c_int, _arg: pv_ops, _arg: cpu.read_msr_safe, _arg: msr, _arg: val) -> return;
}
extern "C" {
    pub fn PVOP_CALL2(_arg: c_int, _arg: pv_ops, _arg: cpu.write_msr_safe, _arg: msr, _arg: val) -> return;
}

extern "C" {
    pub fn paravirt_write_msr_safe(_arg: msr, _arg: val) -> return;
}
// rdmsr with exception handling

extern "C" {
    pub fn paravirt_read_msr_safe(_arg: msr, _arg: p) -> return;
}
extern "C" {
    pub fn PVOP_CALL1(_arg: u64, _arg: pv_ops, _arg: cpu.read_pmc, _arg: counter) -> return;
}
extern "C" {
    pub fn PVOP_CALL0(long: unsigned, _arg: pv_ops, _arg: cpu.store_tr) -> return;
}

extern "C" {
    pub fn PVOP_CALL1(_arg: c_int, _arg: pv_ops, _arg: mmu.pgd_alloc, _arg: mm) -> return;
}

extern "C" {
    pub fn PARA_INDIRECT(_arg: pv_ops+PV_IRQ_save_fl) -> call;
}

