//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/e820.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
pub const E820MAP: c_uint = 0x2d0		/* our map */;

//
// Legacy E820 BIOS limits us to 128 (E820MAX) nodes due to the
// constrained space in the zeropage.  If we have more nodes than
// that, and if we've booted off EFI firmware, then the EFI tables
// passed us from the EFI firmware can list more nodes.  Size our
// internal memory map tables to have room for these additional
// nodes, based on up to three entries per node for which the
// kernel was built: MAX_NUMNODES == (1 << CONFIG_NODES_SHIFT),
// plus E820MAX, allowing space for the possible duplicate E820
// entries that might need room in the same arrays, prior to the
// call to sanitize_e820_map() to remove duplicates.  The allowance
// of three memory map entries per node is "enough" entries for
// the initial hardware platform motivating this mechanism to make
// use of additional EFI map entries.  Future platforms may want
// to allow more than three entries per node or otherwise refine
// this size.
//

pub const E820NR: c_uint = 0x1e8		/* # entries in E820MAP */;
pub const E820_RAM: c_int = 1;
pub const E820_RESERVED: c_int = 2;
pub const E820_ACPI: c_int = 3;
pub const E820_NVS: c_int = 4;
pub const E820_UNUSABLE: c_int = 5;
pub const E820_PMEM: c_int = 7;
//
// This is a non-standardized way to represent ADR or NVDIMM regions that
// persist over a reboot.  The kernel will ignore their special capabilities
// unless the CONFIG_X86_PMEM_LEGACY option is set.
//
// ( Note that older platforms also used 6 for the same type of memory,
// but newer versions switched to 12 as 6 was assigned differently.  Some
// time they will learn... )
//
pub const E820_PRAM: c_int = 12;
//
// reserved RAM used by kernel itself
// if CONFIG_INTEL_TXT is enabled, memory of this type will be
// included in the S3 integrity calculation and so should not include
// any memory that BIOS might alter over the S3 transition
//
pub const E820_RESERVED_KERN: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e820entry {
    pub /: *mut *mut __u64 addr; / start of memory segment,
    pub /: *mut *mut __u64 size; / size of memory segment,
    pub /: *mut *mut __u32 type; / type of memory segment,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e820map {
    pub nr_map: __u32,
    pub map: [e820entry; E820_X_MAX],
}

pub const ISA_START_ADDRESS: c_uint = 0xa0000;
pub const ISA_END_ADDRESS: c_uint = 0x100000;
pub const BIOS_BEGIN: c_uint = 0x000a0000;
pub const BIOS_END: c_uint = 0x00100000;
pub const BIOS_ROM_BASE: c_uint = 0xffe00000;
pub const BIOS_ROM_END: c_uint = 0xffffffff;

