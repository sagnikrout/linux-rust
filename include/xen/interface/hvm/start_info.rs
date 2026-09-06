//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/hvm/start_info.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (c) 2016, Citrix Systems, Inc.
//
// Start of day structure passed to PVH guests and to HVM guests in %ebx.
//
// NOTE: nothing will be loaded at physical address 0, so a 0 value in any
// of the address fields should be treated as not present.
//
// 0 +----------------+
// | magic          | Contains the magic value XEN_HVM_START_MAGIC_VALUE
// |                | ("xEn3" with the 0x80 bit of the "E" set).
// 4 +----------------+
// | version        | Version of this structure. Current version is 1. New
// |                | versions are guaranteed to be backwards-compatible.
// 8 +----------------+
// | flags          | SIF_xxx flags.
// 12 +----------------+
// | nr_modules     | Number of modules passed to the kernel.
// 16 +----------------+
// | modlist_paddr  | Physical address of an array of modules
// |                | (layout of the structure below).
// 24 +----------------+
// | cmdline_paddr  | Physical address of the command line,
// |                | a zero-terminated ASCII string.
// 32 +----------------+
// | rsdp_paddr     | Physical address of the RSDP ACPI data structure.
// 40 +----------------+
// | memmap_paddr   | Physical address of the (optional) memory map. Only
// |                | present in version 1 and newer of the structure.
// 48 +----------------+
// | memmap_entries | Number of entries in the memory map table. Zero
// |                | if there is no memory map being provided. Only
// |                | present in version 1 and newer of the structure.
// 52 +----------------+
// | reserved       | Version 1 and newer only.
// 56 +----------------+
//
// The layout of each entry in the module structure is the following:
//
// 0 +----------------+
// | paddr          | Physical address of the module.
// 8 +----------------+
// | size           | Size of the module in bytes.
// 16 +----------------+
// | cmdline_paddr  | Physical address of the command line,
// |                | a zero-terminated ASCII string.
// 24 +----------------+
// | reserved       |
// 32 +----------------+
//
// The layout of each entry in the memory map table is as follows:
//
// 0 +----------------+
// | addr           | Base address
// 8 +----------------+
// | size           | Size of mapping in bytes
// 16 +----------------+
// | type           | Type of mapping as defined between the hypervisor
// |                | and guest. See XEN_HVM_MEMMAP_TYPE_* values below.
// 20 +----------------|
// | reserved       |
// 24 +----------------+
//
// The address and sizes are always a 64bit little endian unsigned integer.
//
// NB: Xen on x86 will always try to place all the data below the 4GiB
// boundary.
//
// Version numbers of the hvm_start_info structure have evolved like this:
//
// Version 0:  Initial implementation.
//
// Version 1:  Added the memmap_paddr/memmap_entries fields (plus 4 bytes of
// padding) to the end of the hvm_start_info struct. These new
// fields can be used to pass a memory map to the guest. The
// memory map is optional and so guests that understand version 1
// of the structure must check that memmap_entries is non-zero
// before trying to read the memory map.
//
pub const XEN_HVM_START_MAGIC_VALUE: c_uint = 0x336ec578;
//
// The values used in the type field of the memory map table entries are
// defined below and match the Address Range Types as defined in the "System
// Address Map Interfaces" section of the ACPI Specification. Please refer to
// section 15 in version 6.2 of the ACPI spec: http://uefi.org/specifications
//
pub const XEN_HVM_MEMMAP_TYPE_RAM: c_int = 1;
pub const XEN_HVM_MEMMAP_TYPE_RESERVED: c_int = 2;
pub const XEN_HVM_MEMMAP_TYPE_ACPI: c_int = 3;
pub const XEN_HVM_MEMMAP_TYPE_NVS: c_int = 4;
pub const XEN_HVM_MEMMAP_TYPE_UNUSABLE: c_int = 5;
pub const XEN_HVM_MEMMAP_TYPE_DISABLED: c_int = 6;
pub const XEN_HVM_MEMMAP_TYPE_PMEM: c_int = 7;
//
// C representation of the x86/HVM start info layout.
//
// The canonical definition of this layout is above, this is just a way to
// represent the layout described there using C types.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvm_start_info {
    pub /: *mut *mut uint32_t magic; / Contains the magic value 0x336ec578,
// ("xEn3" with the 0x80 bit of the "E" set).
    pub /: *mut *mut uint32_t version; / Version of this structure.,
    pub /: *mut *mut uint32_t flags; / SIF_xxx flags.,
    pub /: *mut *mut uint32_t nr_modules; / Number of modules passed to the kernel.,
    pub /: *mut *mut uint64_t modlist_paddr; / Physical address of an array of,
// hvm_modlist_entry.
    pub /: *mut *mut uint64_t cmdline_paddr; / Physical address of the command line.,
    pub /: *mut *mut uint64_t rsdp_paddr; / Physical address of the RSDP ACPI data,
// structure.
// All following fields only present in version 1 and newer
    pub /: *mut *mut uint64_t memmap_paddr; / Physical address of an array of,
// hvm_memmap_table_entry.
    pub /: *mut *mut uint32_t memmap_entries; / Number of entries in the memmap table.,
// Value will be zero if there is no memory
// map being provided.
    pub /: *mut *mut uint32_t reserved; / Must be zero.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvm_modlist_entry {
    pub /: *mut *mut uint64_t paddr; / Physical address of the module.,
    pub /: *mut *mut uint64_t size; / Size of the module in bytes.,
    pub /: *mut *mut uint64_t cmdline_paddr; / Physical address of the command line.,
    pub reserved: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hvm_memmap_table_entry {
    pub /: *mut *mut uint64_t addr; / Base address of the memory region,
    pub /: *mut *mut uint64_t size; / Size of the memory region in bytes,
    pub /: *mut *mut uint32_t type; / Mapping type,
    pub /: *mut *mut uint32_t reserved; / Must be zero for Version 1.,
}
