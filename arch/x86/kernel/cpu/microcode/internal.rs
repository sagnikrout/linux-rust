//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/microcode/internal.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucode_state {
    UCODE_OK	= 0,
    UCODE_NEW,
    UCODE_NEW_SAFE,
    UCODE_UPDATED,
    UCODE_NFOUND,
    UCODE_ERROR,
    UCODE_TIMEOUT,
    UCODE_OFFLINE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct microcode_ops {
    pub dev): *mut *mut ucode_state (request_microcode_fw)(int cpu, struct device,
    pub cpu): *mut *mut void (microcode_fini_cpu)(int,
//
// The generic 'microcode_core' part guarantees that the callbacks
// below run on a target CPU when they are being called.
// See also the "Synchronization" section in microcode_core.c.
//
    pub cpu): *mut *mut ucode_state (apply_microcode)(int,
    pub (*stage_microcode)(void): *mut c_void,
    pub csig): *mut *mut int (collect_cpu_info)(int cpu, struct cpu_signature,
    pub result): *mut *mut void (finalize_late_load)(int,
    pub 1: use_staging :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct early_load_data {
    pub old_rev: u32,
    pub new_rev: u32,
}

extern "C" {
    pub fn find_microcode_in_initrd(path: *const c_char) -> cpio_data;
}
pub const MAX_UCODE_COUNT: c_int = 128;

//
// In early loading microcode phase on BSP, boot_cpu_data is not set up yet.
// x86_cpuid_vendor() gets vendor id for BSP.
//
// In 32 bit AP case, accessing boot_cpu_data needs linear address. To simplify
// coding, we still use x86_cpuid_vendor() to get vendor id for AP.
//
// x86_cpuid_vendor() gets vendor information directly from CPUID.
//
extern "C" {
    pub fn x86_family(_arg: eax) -> return;
}

extern "C" {
    pub fn load_ucode_amd_bsp(ed: *mut early_load_data, family: c_uint);
}
extern "C" {
    pub fn load_ucode_amd_ap(family: c_uint);
}
extern "C" {
    pub fn reload_ucode_amd(cpu: c_uint);
}
extern "C" {
    pub fn exit_amd_microcode();
}

extern "C" {
    pub fn load_ucode_intel_bsp(ed: *mut early_load_data);
}
extern "C" {
    pub fn load_ucode_intel_ap();
}
extern "C" {
    pub fn reload_ucode_intel();
}

