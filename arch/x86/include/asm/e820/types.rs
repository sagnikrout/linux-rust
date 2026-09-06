//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/e820/types.h
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
// These are the E820 types known to the kernel:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e820_type {
    E820_TYPE_RAM		= 1,
    E820_TYPE_RESERVED	= 2,
    E820_TYPE_ACPI		= 3,
    E820_TYPE_NVS		= 4,
    E820_TYPE_UNUSABLE	= 5,
    E820_TYPE_PMEM		= 7,

//
// This is a non-standardized way to represent ADR or
// NVDIMM regions that persist over a reboot.
//
// The kernel will ignore their special capabilities
// unless the CONFIG_X86_PMEM_LEGACY=y option is set.
//
// ( Note that older platforms also used 6 for the same
// type of memory, but newer versions switched to 12 as
// 6 was assigned differently. Some time they will learn... )
//
    E820_TYPE_PRAM		= 12,

//
// Special-purpose memory is indicated to the system via the
// EFI_MEMORY_SP attribute. Define an e820 translation of this
// memory type for the purpose of reserving this range and
// marking it with the IORES_DESC_SOFT_RESERVED designation.
//
    E820_TYPE_SOFT_RESERVED	= 0xefffffff,
}

//
// A single E820 map entry, describing a memory range of [addr...addr+size-1],
// of 'type' memory type:
//
// (We pack it because there can be thousands of them on large systems.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e820_entry {
    pub addr: u64,
    pub size: u64,
    pub type: e820_type,
    pub __attribute__((packed)): },
//
// The legacy E820 BIOS limits us to 128 (E820_MAX_ENTRIES_ZEROPAGE) nodes
// due to the constrained space in the zeropage.
//
// On large systems we can easily have thousands of nodes with RAM,
// which cannot be fit into so few entries - so we have a mechanism
// to extend the e820 table size at build-time, via the E820_MAX_ENTRIES
// define below.
//
// ( Those extra entries are enumerated via the EFI memory map, not
// via the legacy zeropage mechanism. )
//
// Size our internal memory map tables to have room for these additional
// entries, based on a heuristic calculation: up to three entries per
// NUMA node, plus E820_MAX_ENTRIES_ZEROPAGE for some extra space.
//
// This allows for bootstrap/firmware quirks such as possible duplicate
// E820 entries that might need room in the same arrays, prior to the
// call to e820__update_table() to remove duplicates.  The allowance
// of three memory map entries per node is "enough" entries for
// the initial hardware platform motivating this mechanism to make
// use of additional EFI map entries.  Future platforms may want
// to allow more than three entries per node or otherwise refine
// this size.
//

//
// The whole array of E820 entries:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e820_table {
    pub nr_entries: u32,
    pub entries: [e820_entry; E820_MAX_ENTRIES],
}

//
// Various well-known legacy memory ranges in physical memory:
//
pub const ISA_START_ADDRESS: c_uint = 0x000a0000;
pub const ISA_END_ADDRESS: c_uint = 0x00100000;
pub const BIOS_BEGIN: c_uint = 0x000a0000;
pub const BIOS_END: c_uint = 0x00100000;
pub const HIGH_MEMORY: c_uint = 0x00100000;
pub const BIOS_ROM_BASE: c_uint = 0xffe00000;
pub const BIOS_ROM_END: c_uint = 0xffffffff;
