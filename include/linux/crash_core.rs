//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crash_core.h
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
pub struct crash_mem {
    pub max_nr_ranges: c_uint,
    pub nr_ranges: c_uint,
    pub __counted_by(max_nr_ranges): range ranges[],
}

extern "C" {
    pub fn crash_shrink_memory(new_size: c_ulong) -> c_int;
}
extern "C" {
    pub fn crash_get_memory_size() -> isize;
}

//
// Protection mechanism for crashkernel reserved memory after
// the kdump kernel is loaded.
//
// Provide an empty default implementation here -- architecture
// code may override this
//

extern "C" {
    pub fn crash_check_hotplug_support() -> c_int;
}

// Alignment required for elf header segment
pub const ELF_CORE_HEADER_ALIGN: c_int = 4096;
extern "C" {
    pub fn crash_exclude_core_ranges(cmem: *mut crash_mem) -> c_int;
}
pub const KEXEC_CRASH_HP_NONE: c_int = 0;
pub const KEXEC_CRASH_HP_ADD_CPU: c_int = 1;
pub const KEXEC_CRASH_HP_REMOVE_CPU: c_int = 2;
pub const KEXEC_CRASH_HP_ADD_MEMORY: c_int = 3;
pub const KEXEC_CRASH_HP_REMOVE_MEMORY: c_int = 4;

extern "C" {
    pub fn __crash_kexec(regs: *mut pt_regs);
}
extern "C" {
    pub fn crash_kexec(regs: *mut pt_regs);
}
extern "C" {
    pub fn kexec_should_crash(p: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn kexec_crash_loaded() -> c_int;
}
extern "C" {
    pub fn crash_save_cpu(regs: *mut pt_regs, cpu: c_int);
}
extern "C" {
    pub fn kimage_crash_copy_vmcoreinfo(image: *mut kimage) -> c_int;
}
extern "C" {
    pub fn arch_get_system_nr_ranges() -> c_uint;
}
extern "C" {
    pub fn arch_crash_populate_cmem(cmem: *mut crash_mem) -> c_int;
}
extern "C" {
    pub fn arch_crash_exclude_ranges(cmem: *mut crash_mem) -> c_int;
}

extern "C" {
    pub fn crash_load_dm_crypt_keys(image: *mut kimage) -> c_int;
}
extern "C" {
    pub fn dm_crypt_keys_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize;
}

