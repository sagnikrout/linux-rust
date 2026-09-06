//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kexec.h
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
// On FSL-BookE we setup a 1:1 mapping which covers the first 2GiB of memory
// and therefore we can only deal with memory within this range
//

//
// Maximum page that is mapped directly into kernel memory.
// XXX: Since we copy virt we can use any page we allocate
//

//
// Maximum address we can reach in physical address mode.
// XXX: I want to allow initrd in highmem. Otherwise set to rmo on LPAR.
//

// Maximum address we can use for the control code buffer

// TASK_SIZE, probably left over from use_mm ??

pub const KEXEC_CONTROL_PAGE_SIZE: c_int = 4096;
// The native architecture

pub const KEXEC_STATE_NONE: c_int = 0;
pub const KEXEC_STATE_IRQS_OFF: c_int = 1;
pub const KEXEC_STATE_REAL_MODE: c_int = 2;

extern "C" {
    pub fn void(_arg: *mut crash_shutdown_t)(void) -> typedef;
}

extern "C" {
    pub fn default_machine_kexec(image: *mut kimage);
}
extern "C" {
    pub fn kexec_copy_flush(image: *mut kimage);
}

// Macro flag: #define ARCH_HAS_KIMAGE_ARCH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
    pub exclude_ranges: *mut crash_mem,
    pub backup_start: c_ulong,
    pub backup_buf: *mut c_void,
    pub fdt: *mut c_void,
}

extern "C" {
    pub fn arch_kexec_kernel_image_probe(image: *mut kimage, buf: *mut c_void, buf_len: c_ulong) -> c_int;
}

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

extern "C" {
    pub fn kexec_extra_fdt_size_ppc64(image: *mut kimage, rmem: *mut crash_mem) -> c_uint;
}
extern "C" {
    pub fn setup_new_fdt_ppc64(image: *const kimage, fdt: *mut c_void, rmem: *mut crash_mem) -> c_int;
}

extern "C" {
    pub fn overlaps_crashkernel(start: c_ulong, size: c_ulong) -> int __init;
}
extern "C" {
    pub fn arch_reserve_crashkernel();
}
extern "C" {
    pub fn kdump_cma_reserve();
}

//
// This function is responsible for capturing register states if coming
// via panic or invoking dump using sysrq-trigger.
//

extern "C" {
    pub fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void);
}

extern "C" {
    pub fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> c_int;
}

extern "C" {
    pub fn arch_crash_get_elfcorehdr_size() -> c_uint;
}

extern "C" {
    pub fn machine_kexec_post_load(image: *mut kimage) -> c_int;
}

extern "C" {
    pub fn crash_send_ipi(): *mut *mut void (crash_ipi_callback)(struct pt_regs);
}
extern "C" {
    pub fn crash_ipi_callback(regs: *mut pt_regs);
}
extern "C" {
    pub fn crash_shutdown_register(handler: crash_shutdown_t) -> c_int;
}
extern "C" {
    pub fn crash_shutdown_unregister(handler: crash_shutdown_t) -> c_int;
}
extern "C" {
    pub fn default_machine_crash_shutdown(regs: *mut pt_regs);
}
extern "C" {
    pub fn crash_kexec_prepare();
}
extern "C" {
    pub fn crash_kexec_secondary(regs: *mut pt_regs);
}
extern "C" {
    pub fn is_kdump_kernel() -> bool;
}

extern "C" {
    pub fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong);
}

extern "C" {
    pub fn update_cpus_node(fdt: *mut c_void) -> c_int;
}

