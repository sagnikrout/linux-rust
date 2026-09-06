//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/fadump-internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Firmware-Assisted Dump internal code.
//
// Copyright 2011, Mahesh Salgaonkar, IBM Corporation.
// Copyright 2019, Hari Bathini, IBM Corporation.
//
// Maximum number of memory regions kernel supports
pub const FADUMP_MAX_MEM_REGS: c_int = 128;

// The upper limit percentage for user specified boot memory size (25%)
pub const MAX_BOOT_MEM_RATIO: c_int = 4;

// FAD commands
pub const FADUMP_REGISTER: c_int = 1;
pub const FADUMP_UNREGISTER: c_int = 2;
pub const FADUMP_INVALIDATE: c_int = 3;
//
// Copy the ascii values for first 8 characters from a string into u64
// variable at their respective indexes.
// e.g.
// The string "FADMPINF" will be converted into 0x4641444d50494e46
//

//
// The introduction of new fields in the fadump crash info header has
// led to a change in the magic key from `FADMPINF` to `FADMPSIG` for
// identifying a kernel crash from an old kernel.
//
// To prevent the need for further changes to the magic number in the
// event of future modifications to the fadump crash info header, a
// version field has been introduced to track the fadump crash info
// header version.
//
// Consider a few points before adding new members to the fadump crash info
// header structure:
//
// - Append new members; avoid adding them in between.
// - Non-primitive members should have a size member as well.
// - For every change in the fadump header, increment the
// fadump header version. This helps the updated kernel decide how to
// handle kernel dumps from older kernels.
//

pub const FADUMP_HEADER_VERSION: c_int = 1;
// fadump crash info structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fadump_crash_info_header {
    pub magic_number: u64,
    pub version: u32,
    pub crashing_cpu: u32,
    pub vmcoreinfo_raddr: u64,
    pub vmcoreinfo_size: u64,
    pub pt_regs_sz: u32,
    pub cpu_mask_sz: u32,
    pub regs: pt_regs,
    pub cpu_mask: cpumask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fadump_memory_range {
    pub base: u64,
    pub size: u64,
}

// fadump memory ranges info
pub const RNG_NAME_SZ: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fadump_mrange_info {
    pub name: [c_char; RNG_NAME_SZ],
    pub mem_ranges: *mut fadump_memory_range,
    pub mem_ranges_sz: u32,
    pub mem_range_cnt: u32,
    pub max_mem_ranges: u32,
    pub is_static: bool,
}

// Platform specific callback functions
// Firmware-assisted dump configuration details.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_dump {
    pub reserve_dump_area_start: c_ulong,
    pub reserve_dump_area_size: c_ulong,
// cmd line option during boot
    pub reserve_bootvar: c_ulong,
    pub cpu_state_data_size: c_ulong,
    pub cpu_state_dest_vaddr: u64,
    pub cpu_state_data_version: u32,
    pub cpu_state_entry_size: u32,
    pub hpte_region_size: c_ulong,
    pub boot_memory_size: c_ulong,
    pub boot_mem_dest_addr: u64,
    pub boot_mem_addr: [u64; FADUMP_MAX_MEM_REGS],
    pub boot_mem_sz: [u64; FADUMP_MAX_MEM_REGS],
    pub boot_mem_top: u64,
    pub boot_mem_regs_cnt: u64,
    pub fadumphdr_addr: c_ulong,
    pub elfcorehdr_addr: u64,
    pub elfcorehdr_size: u64,
    pub cpu_notes_buf_vaddr: c_ulong,
    pub cpu_notes_buf_size: c_ulong,
    pub param_area: c_ulong,
//
// Maximum size supported by firmware to copy from source to
// destination address per entry.
//
    pub max_copy_size: u64,
    pub kernel_metadata: u64,
    pub ibm_configure_kernel_dump: c_int,
    pub fadump_enabled:1: c_ulong,
    pub fadump_supported:1: c_ulong,
    pub dump_active:1: c_ulong,
    pub dump_registered:1: c_ulong,
    pub nocma:1: c_ulong,
    pub param_area_supported:1: c_ulong,
    pub ops: *mut fadump_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fadump_ops {
    pub fadump_conf): *mut *mut u64 (fadump_init_mem_struct)(struct fw_dump,
    pub (*fadump_get_metadata_size)(void): *mut u64,
    pub fadump_conf): *mut *mut int (fadump_setup_metadata)(struct fw_dump,
    pub (*fadump_get_bootmem_min)(void): *mut u64,
    pub fadump_conf): *mut *mut int (fadump_register)(struct fw_dump,
    pub fadump_conf): *mut *mut int (fadump_unregister)(struct fw_dump,
    pub fadump_conf): *mut *mut int (fadump_invalidate)(struct fw_dump,
    pub fadump_conf): *mut *mut void (fadump_cleanup)(struct fw_dump,
    pub fadump_conf): *mut *mut int (fadump_process)(struct fw_dump,
    pub m): *mut seq_file,
    pub msg): *const c_char,
    pub (*fadump_max_boot_mem_rgns)(void): *mut c_int,
}

// Helper functions
extern "C" {
    pub fn fadump_setup_cpu_notes_buf(num_cpus: u32) -> s32 __init;
}
extern "C" {
    pub fn fadump_free_cpu_notes_buf();
}
extern "C" {
    pub fn fadump_regs_to_elf_notes(buf: *mut u32, regs: *mut pt_regs) -> *mut u32 __init;
}
extern "C" {
    pub fn fadump_update_elfcore_header(bufp: *mut c_char) -> void __init;
}
extern "C" {
    pub fn is_fadump_reserved_mem_contiguous() -> bool;
}

// Firmware-assisted dump configuration details.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_dump {
    pub boot_mem_top: u64,
    pub dump_active: u64,
}

extern "C" {
    pub fn rtas_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64);
}

extern "C" {
    pub fn opal_fadump_dt_scan(fadump_conf: *mut fw_dump, node: u64);
}

