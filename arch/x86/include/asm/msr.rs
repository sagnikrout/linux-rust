//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/msr.h
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
pub struct msr_info {
    pub msr_no: u32,
    pub reg: msr,
    pub msrs: *mut msr __percpu,
    pub err: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_regs_info {
    pub regs: *mut u32,
    pub err: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_msr {
    pub valid: bool,
    pub info: msr_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_msrs {
    pub num: c_uint,
    pub array: *mut saved_msr,
}

//
// Be very careful with includes. This header is prone to include loops.
//

extern "C" {
    pub fn do_trace_write_msr(msr: u32, val: u64, failed: c_int);
}
extern "C" {
    pub fn do_trace_read_msr(msr: u32, val: u64, failed: c_int);
}
extern "C" {
    pub fn do_trace_rdpmc(msr: u32, val: u64, failed: c_int);
}

//
// __rdmsr() and __wrmsr() are the two primitives which are the bare minimum MSR
// accessors and should not have any tracing or other functionality piggybacking
// on them - those are *purely* for accessing MSRs and nothing more. So don't even
// think of extending them - you will be slapped with a stinking trout or a frozen
// shark will reach you, wherever you are! You've been warned.
//
extern "C" {
    pub fn EAX_EDX_VAL(_arg: val, _arg: low, _arg: high) -> return;
}

extern "C" {
    pub fn __rdmsr(_arg: msr) -> return;
}

// p = EAX_EDX_VAL(val, low, high);
// Can be uninlined because referenced by paravirt
extern "C" {
    pub fn rdmsr_safe_regs(regs[8]: u32) -> c_int;
}
extern "C" {
    pub fn wrmsr_safe_regs(regs[8]: u32) -> c_int;
}
extern "C" {
    pub fn volatile(EAX_EDX_RET(val: "rdpmc" :, _arg: low, (counter): high) : "c") -> asm;
}
extern "C" {
    pub fn EAX_EDX_VAL(_arg: val, _arg: low, _arg: high) -> return;
}

//
// Access to machine-specific registers (available on 586 and better only)
// Note: the rd* operations modify the parameters directly (without using
// pointer indirection), this allows gcc to optimize better
//

// wrmsr with exception handling
extern "C" {
    pub fn native_write_msr_safe(_arg: msr, _arg: val) -> return;
}
// rdmsr with exception handling

extern "C" {
    pub fn native_read_msr_safe(_arg: msr, _arg: p) -> return;
}
extern "C" {
    pub fn native_read_pmc(_arg: counter) -> return;
}

// Instruction opcode for WRMSRNS supported in binutils >= 2.40

// Non-serializing WRMSR, when available.  Falls back to a serializing WRMSR.
//
// WRMSR is 2 bytes.  WRMSRNS is 3 bytes.  Pad WRMSR with a redundant
// DS prefix to avoid a trailing NOP.
//
// Dual u32 version of wrmsrq_safe():
//
extern "C" {
    pub fn wrmsrq_safe(_arg: msr, low: (u64)high << 32 |) -> return;
}
extern "C" {
    pub fn msrs_free(msrs: *mut msr __percpu);
}
extern "C" {
    pub fn msr_set_bit(msr: u32, bit: u8) -> c_int;
}
extern "C" {
    pub fn msr_clear_bit(msr: u32, bit: u8) -> c_int;
}

extern "C" {
    pub fn rdmsrq_on_cpu(cpu: c_uint, msr_no: u32, q: *mut u64) -> c_int;
}
extern "C" {
    pub fn wrmsrq_on_cpu(cpu: c_uint, msr_no: u32, q: u64) -> c_int;
}
extern "C" {
    pub fn rdmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr __percpu);
}
extern "C" {
    pub fn wrmsr_on_cpus(mask: *const cpumask, msr_no: u32, msrs: *mut msr __percpu);
}
extern "C" {
    pub fn rdmsrq_safe_on_cpu(cpu: c_uint, msr_no: u32, q: *mut u64) -> c_int;
}
extern "C" {
    pub fn wrmsrq_safe_on_cpu(cpu: c_uint, msr_no: u32, q: u64) -> c_int;
}
extern "C" {
    pub fn rdmsr_safe_regs_on_cpu(cpu: c_uint, regs[8]: u32) -> c_int;
}
extern "C" {
    pub fn wrmsr_safe_regs_on_cpu(cpu: c_uint, regs[8]: u32) -> c_int;
}

extern "C" {
    pub fn rdmsrq_safe(_arg: msr_no, _arg: q) -> return;
}
extern "C" {
    pub fn wrmsrq_safe(_arg: msr_no, _arg: q) -> return;
}
extern "C" {
    pub fn rdmsr_safe_regs(_arg: regs) -> return;
}
extern "C" {
    pub fn wrmsr_safe_regs(_arg: regs) -> return;
}

