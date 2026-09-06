//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/uv/bios.h
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
// UV BIOS layer definitions.
//
// (C) Copyright 2020 Hewlett Packard Enterprise Development LP
// Copyright (C) 2007-2017 Silicon Graphics, Inc. All rights reserved.
// Copyright (c) Russ Anderson <rja@sgi.com>
//

//
// Values for the BIOS calls.  It is passed as the first * argument in the
// BIOS call.  Passing any other value in the first argument will result
// in a BIOS_STATUS_UNIMPLEMENTED return status.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_bios_cmd {
    UV_BIOS_COMMON,
    UV_BIOS_GET_SN_INFO,
    UV_BIOS_FREQ_BASE,
    UV_BIOS_WATCHLIST_ALLOC,
    UV_BIOS_WATCHLIST_FREE,
    UV_BIOS_MEMPROTECT,
    UV_BIOS_GET_PARTITION_ADDR,
    UV_BIOS_SET_LEGACY_VGA_TARGET
}

pub const UV_BIOS_EXTRA: c_uint = 0x10000;
pub const UV_BIOS_GET_PCI_TOPOLOGY: c_uint = 0x10001;
pub const UV_BIOS_GET_GEOINFO: c_uint = 0x10003;
pub const UV_BIOS_EXTRA_OP_MEM_COPYIN: c_uint = 0x1000;
pub const UV_BIOS_EXTRA_OP_MEM_COPYOUT: c_uint = 0x2000;
pub const UV_BIOS_EXTRA_OP_MASK: c_uint = 0x0fff;
pub const UV_BIOS_EXTRA_GET_HEAPSIZE: c_int = 1;
pub const UV_BIOS_EXTRA_INSTALL_HEAP: c_int = 2;
pub const UV_BIOS_EXTRA_MASTER_NASID: c_int = 3;

//
// Status values returned from a BIOS call.
//
// Address map parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_gam_parameters {
    pub mmr_base: u64,
    pub gru_base: u64,
    pub /: *mut *mut u8 mmr_shift; / Convert PNode to MMR space offset,
    pub /: *mut *mut u8 gru_shift; / Convert PNode to GRU space offset,
    pub /: *mut *mut u8 gpa_shift; / Size of offset field in GRU phys addr,
    pub unused1: u8,
}

// UV_TABLE_GAM_RANGE_ENTRY values

pub const UV_GAM_RANGE_TYPE_MAX: c_int = 6;
// The structure stores PA bits 56:26, for 64MB granularity

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_gam_range_entry {
    pub /: *mut *mut char type; / Entry type: GAM_RANGE_TYPE_UNUSED, etc.,
    pub unused1: c_char,
    pub /: *mut *mut u16 nasid; / HNasid,
    pub /: *mut *mut u16 sockid; / Socket ID, high bits of APIC ID,
    pub /: *mut *mut u16 pnode; / Index to MMR and GRU spaces,
    pub unused2: u32,
    pub /: *mut *mut u32 limit; / PA bits 56:26 (UV_GAM_RANGE_SHFT),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_arch_type_entry {
    pub archtype: [c_char; UV_AT_SIZE],
}

pub const UV_SYSTAB_VERSION_UV4: c_uint = 0x400	/* UV4 BIOS base version */;
pub const UV_SYSTAB_VERSION_UV4_1: c_uint = 0x401	/* + gpa_shift */;
pub const UV_SYSTAB_VERSION_UV4_2: c_uint = 0x402	/* + TYPE_NVRAM/WINDOW/MBOX */;
pub const UV_SYSTAB_VERSION_UV4_3: c_uint = 0x403	/* - GAM Range PXM Value */;

pub const UV_SYSTAB_VERSION_UV5: c_uint = 0x500	/* UV5 GAM base version */;

pub const UV_SYSTAB_TYPE_MAX: c_int = 4;
//
// The UV system table describes specific firmware
// capabilities available to the Linux kernel at runtime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_systab {
    pub /: *mut *mut char signature[4]; / must be UV_SYSTAB_SIG,
    pub /: *mut *mut u32 revision; / distinguish different firmware revs,
    pub ...): *mut *mut u64 (__efiapi function)(enum uv_bios_cmd,,
// BIOS runtime callback function ptr
    pub /: *mut *mut u32 size; / systab size (starting with _VERSION_UV4),
    pub /: *mut *mut u32 type:8; / type of entry,
    pub /: *mut *mut u32 offset:24; / byte offset from struct start to entry,
    pub /: *mut *mut } entry[]; / additional entries follow,
}

pub const UV_BIOS_MAXSTRING: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_bios_hub_info {
    pub id: c_uint,
    pub this_part:1: c_ulonglong,
    pub is_shared:1: c_ulonglong,
    pub is_disabled:1: c_ulonglong,
    pub fields: },
    pub flags: c_ulonglong,
    pub reserved: c_ulonglong,
    pub b: },
    pub f: },
    pub name: [c_char; UV_BIOS_MAXSTRING],
    pub location: [c_char; UV_BIOS_MAXSTRING],
    pub ports: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uv_bios_port_info {
    pub port: c_uint,
    pub conn_id: c_uint,
    pub conn_port: c_uint,
}

// (... end of definitions from UV BIOS ...)
#[repr(C)]
#[derive(Copy, Clone)]
pub union partition_info_u {
    pub val: u64,
    pub 24: region_size :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uv_memprotect {
    UV_MEMPROT_RESTRICT_ACCESS,
    UV_MEMPROT_ALLOW_AMO,
    UV_MEMPROT_ALLOW_RW
}

extern "C" {
    pub fn uv_bios_get_sn_info(_arg: c_int, : *mut c_int, : *mut c_long, : *mut c_long, : *mut c_long, : *mut c_long) -> i64;
}
extern "C" {
    pub fn uv_bios_freq_base(_arg: u64, : *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_mq_watchlist_free(_arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn uv_bios_change_memprotect(_arg: u64, _arg: u64, uv_memprotect: enum) -> i64;
}
extern "C" {
    pub fn uv_bios_reserved_page_pa(_arg: u64, : *mut u64, : *mut u64, : *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_set_legacy_vga_target(decode: bool, domain: c_int, bus: c_int) -> c_int;
}
extern "C" {
    pub fn uv_bios_get_master_nasid(sz: u64, nasid: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_get_heapsize(nasid: u64, sz: u64, heap_sz: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_install_heap(nasid: u64, sz: u64, heap: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_obj_count(nasid: u64, sz: u64, objcnt: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_enum_objs(nasid: u64, sz: u64, objbuf: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_enum_ports(nasid: u64, obj_id: u64, sz: u64, portbuf: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_get_geoinfo(nasid: u64, sz: u64, geo: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_get_pci_topology(sz: u64, buf: *mut u64) -> i64;
}
extern "C" {
    pub fn uv_bios_init() -> c_int;
}
extern "C" {
    pub fn get_uv_systab_phys(msg: bool) -> c_ulong;
}
extern "C" {
    pub fn uv_get_archtype(buf: *mut c_char, len: c_int) -> isize;
}
extern "C" {
    pub fn uv_get_hubless_system() -> c_int;
}
//
// EFI runtime lock; cf. firmware/efi/runtime-wrappers.c for details
//
