//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crash_dump.h
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

// For IS_ENABLED(CONFIG_CRASH_DUMP)

extern "C" {
    pub fn elfcorehdr_alloc(addr: *mut c_ulonglong, size: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn elfcorehdr_free(addr: c_ulonglong);
}
extern "C" {
    pub fn elfcorehdr_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize;
}
extern "C" {
    pub fn elfcorehdr_read_notes(buf: *mut c_char, count: usize, ppos: *mut u64) -> isize;
}
extern "C" {
    pub fn vmcore_cleanup();
}
// Architecture code defines this if there are other possible ELF
// machine types, e.g. on bi-arch capable hardware.

pub const vmcore_elf_check_arch_cross(x): c_int = 0;

//
// Architecture code can redefine this if there are any special checks
// needed for 32-bit ELF or 64-bit ELF vmcores.  In case of 32-bit
// only architecture, vmcore_elf64_check_arch can be set to zero.
//

//
// is_kdump_kernel() checks whether this kernel is booting after a panic of
// previous kernel or not. This is determined by checking if previous kernel
// has passed the elf core header address on command line.
//
// This is not just a test if CONFIG_CRASH_DUMP is enabled or not. It will
// return true if CONFIG_CRASH_DUMP=y and if kernel is booting after a panic
// of previous kernel.
//

// is_vmcore_usable() checks if the kernel is booting after a panic and
// the vmcore region is usable.
//
// This makes use of the fact that due to alignment -2ULL is not
// a valid pointer, much in the vain of IS_ERR(), except
// dealing directly with an unsigned long long rather than a pointer.
//
// vmcore_unusable() marks the vmcore as unusable,
// without disturbing the logic of is_kdump_kernel()
//
// struct vmcore_cb - driver callbacks for /proc/vmcore handling
// @pfn_is_ram: check whether a PFN really is RAM and should be accessed when
// reading the vmcore. Will return "true" if it is RAM or if the
// callback cannot tell. If any callback returns "false", it's not
// RAM and the page must not be accessed; zeroes should be
// indicated in the vmcore instead. For example, a ballooned page
// contains no data and reading from such a page will cause high
// load in the hypervisor.
// @get_device_ram: query RAM ranges that can only be detected by device
// drivers, such as the virtio-mem driver, so they can be included in
// the crash dump on architectures that allocate the elfcore hdr in the dump
// ("2nd") kernel. Indicated RAM ranges may contain holes to reduce the
// total number of ranges; such holes can be detected using the pfn_is_ram
// callback just like for other RAM.
// @next: List head to manage registered callbacks internally; initialized by
// register_vmcore_cb().
//
// vmcore callbacks allow drivers managing physical memory ranges to
// coordinate with vmcore handling code, for example, to prevent accessing
// physical memory ranges that should not be accessed when reading the vmcore,
// although included in the vmcore header as memory ranges to dump.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcore_cb {
    pub pfn): *mut *mut *mut bool (pfn_is_ram)(struct vmcore_cb cb, unsigned long,
    pub list): *mut *mut *mut int (get_device_ram)(struct vmcore_cb cb, struct list_head,
    pub next: list_head,
}

extern "C" {
    pub fn register_vmcore_cb(cb: *mut vmcore_cb);
}
extern "C" {
    pub fn unregister_vmcore_cb(cb: *mut vmcore_cb);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcore_range {
    pub list: list_head,
    pub paddr: c_ulonglong,
    pub size: c_ulonglong,
    pub offset: loff_t,
}

// Allocate a vmcore range and add it to the list.
// Free a list of vmcore ranges.

// Device Dump information to be filled by drivers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmcoredd_data {
    pub /: *mut *mut char dump_name[VMCOREDD_MAX_NAME_BYTES]; / Unique name of the dump,
    pub /: *mut *mut unsigned int size; / Size of the dump,
// Driver's registered callback to be invoked to collect dump
    pub buf): *mut *mut *mut int (vmcoredd_callback)(struct vmcoredd_data data, void,
}

extern "C" {
    pub fn vmcore_add_device_dump(data: *mut vmcoredd_data) -> c_int;
}

