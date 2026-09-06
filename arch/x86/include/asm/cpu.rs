//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpu.h
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
    pub fn soft_restart_cpu();
}

extern "C" {
    pub fn ap_init_aperfmperf();
}
extern "C" {
    pub fn mwait_usable(: *const cpuinfo_x86) -> c_int;
}
extern "C" {
    pub fn x86_family(sig: c_uint) -> c_uint;
}
extern "C" {
    pub fn x86_model(sig: c_uint) -> c_uint;
}
extern "C" {
    pub fn x86_stepping(sig: c_uint) -> c_uint;
}
extern "C" {
    pub fn cpuid_family(l: *const leaf_0x1_0) -> c_uint;
}
extern "C" {
    pub fn cpuid_model(l: *const leaf_0x1_0) -> c_uint;
}

extern "C" {
    pub fn sld_setup(c: *mut cpuinfo_x86) -> void __init;
}
extern "C" {
    pub fn handle_user_split_lock(regs: *mut pt_regs, error_code: c_long) -> bool;
}
extern "C" {
    pub fn handle_guest_split_lock(ip: c_ulong) -> bool;
}
extern "C" {
    pub fn handle_bus_lock(regs: *mut pt_regs);
}
extern "C" {
    pub fn split_lock_init();
}
extern "C" {
    pub fn bus_lock_init();
}

extern "C" {
    pub fn init_ia32_feat_ctl(c: *mut cpuinfo_x86);
}

extern "C" {
    pub fn cet_disable() -> __noendbr void;
}
extern "C" {
    pub fn intel_collect_cpu_info(sig: *mut cpu_signature);
}
extern "C" {
    pub fn x86_read_arch_cap_msr() -> u64;
}
extern "C" {
    pub fn intel_find_matching_signature(mc: *mut c_void, sig: *mut cpu_signature) -> bool;
}
extern "C" {
    pub fn intel_microcode_sanity_check(mc: *mut c_void, print_err: bool, hdr_type: c_int) -> c_int;
}
