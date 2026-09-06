//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/agp/agp.h
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


//
// AGPGART
// Copyright (C) 2004 Silicon Graphics, Inc.
// Copyright (C) 2002-2004 Dave Jones
// Copyright (C) 1999 Jeff Hartmann
// Copyright (C) 1999 Precision Insight, Inc.
// Copyright (C) 1999 Xi Graphics, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// JEFF HARTMANN, OR ANY OTHER CONTRIBUTORS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
pub const _AGP_BACKEND_PRIV_H: c_int = 1;

// #define AGP_DEBUG 1

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aper_size_type {
    U8_APER_SIZE,
    U16_APER_SIZE,
    U32_APER_SIZE,
    LVL2_APER_SIZE,
    FIXED_APER_SIZE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gatt_mask {
    pub mask: c_ulong,
    pub type: u32,
// totally device specific, for integrated chipsets that
// might have different types of memory masks.  For other
// devices this will probably be ignored
}

pub const AGP_PAGE_DESTROY_UNMAP: c_int = 1;
pub const AGP_PAGE_DESTROY_FREE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aper_size_info_8 {
    pub size: c_int,
    pub num_entries: c_int,
    pub page_order: c_int,
    pub size_value: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aper_size_info_16 {
    pub size: c_int,
    pub num_entries: c_int,
    pub page_order: c_int,
    pub size_value: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aper_size_info_32 {
    pub size: c_int,
    pub num_entries: c_int,
    pub page_order: c_int,
    pub size_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aper_size_info_lvl2 {
    pub size: c_int,
    pub num_entries: c_int,
    pub size_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aper_size_info_fixed {
    pub size: c_int,
    pub num_entries: c_int,
    pub page_order: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_bridge_driver {
    pub owner: *mut module,
    pub aperture_sizes: *const c_void,
    pub num_aperture_sizes: c_int,
    pub size_type: aper_size_type,
    pub cant_use_aperture: bool,
    pub needs_scratch_page: bool,
    pub masks: *const gatt_mask,
    pub (*fetch_size)(void): *mut c_int,
    pub (*configure)(void): *mut c_int,
    pub u32): *mut *mut *mut void (agp_enable)(struct agp_bridge_data ,,
    pub (*cleanup)(void): *mut c_void,
    pub ): *mut *mut void (tlb_flush)(struct agp_memory,
    pub int): *mut *mut *mut unsigned long (mask_memory)(struct agp_bridge_data , dma_addr_t,,
    pub (*cache_flush)(void): *mut c_void,
    pub ): *mut *mut int (create_gatt_table)(struct agp_bridge_data,
    pub ): *mut *mut int (free_gatt_table)(struct agp_bridge_data,
    pub int): *mut *mut *mut int (insert_memory)(struct agp_memory , off_t,,
    pub int): *mut *mut *mut int (remove_memory)(struct agp_memory , off_t,,
    pub int): *mut *mut *mut agp_memory (alloc_by_type) (size_t,,
    pub ): *mut *mut void (free_by_type)(struct agp_memory,
    pub ): *mut *mut *mut page (agp_alloc_page)(agp_bridge_data,
    pub size_t): *mut *mut *mut *mut int (agp_alloc_pages)(struct agp_bridge_data , struct agp_memory ,,
    pub flags): *mut *mut *mut void (agp_destroy_page)(struct page , int,
    pub ): *mut *mut void (agp_destroy_pages)(struct agp_memory,
    pub int): *mut *mut *mut int (agp_type_to_mask_type) (struct agp_bridge_data ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_bridge_data {
    pub version: *const agp_version,
    pub driver: *const agp_bridge_driver,
    pub vm_ops: *const vm_operations_struct,
    pub previous_size: *mut c_void,
    pub current_size: *mut c_void,
    pub dev_private_data: *mut c_void,
    pub dev: *mut pci_dev,
    pub gatt_table: *mut u32 __iomem,
    pub gatt_table_real: *mut u32,
    pub scratch_page: c_ulong,
    pub scratch_page_page: *mut page,
    pub scratch_page_dma: dma_addr_t,
    pub gart_bus_addr: c_ulong,
    pub gatt_bus_addr: c_ulong,
    pub mode: u32,
    pub key_list: *mut c_ulong,
    pub current_memory_agp: core::sync::atomic::AtomicI32,
    pub agp_in_use: core::sync::atomic::AtomicI32,
    pub /: *mut *mut int max_memory_agp; / in number of pages,
    pub aperture_size_idx: c_int,
    pub capndx: c_int,
    pub flags: c_int,
    pub major_version: c_char,
    pub minor_version: c_char,
    pub list: list_head,
    pub apbase_config: u32,
// list of agp_memory mapped to the aperture
    pub mapped_list: list_head,
    pub mapped_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agp_device_ids {
    pub /: *mut *mut unsigned short device_id; / first, to make table easier to read,
    pub chipset: chipset_type,
    pub chipset_name: *const c_char,
    pub /: *mut *mut *mut *mut int (chipset_setup) (struct pci_dev pdev); / used to override generic,
}

// Driver registration
extern "C" {
    pub fn agp_put_bridge(bridge: *mut agp_bridge_data);
}
extern "C" {
    pub fn agp_add_bridge(bridge: *mut agp_bridge_data) -> c_int;
}
extern "C" {
    pub fn agp_remove_bridge(bridge: *mut agp_bridge_data);
}
// Generic routines.
extern "C" {
    pub fn agp_generic_enable(bridge: *mut agp_bridge_data, mode: u32);
}
extern "C" {
    pub fn agp_generic_create_gatt_table(bridge: *mut agp_bridge_data) -> c_int;
}
extern "C" {
    pub fn agp_generic_free_gatt_table(bridge: *mut agp_bridge_data) -> c_int;
}
extern "C" {
    pub fn agp_generic_insert_memory(mem: *mut agp_memory, pg_start: off_t, type: c_int) -> c_int;
}
extern "C" {
    pub fn agp_generic_remove_memory(mem: *mut agp_memory, pg_start: off_t, type: c_int) -> c_int;
}
extern "C" {
    pub fn agp_generic_free_by_type(curr: *mut agp_memory);
}
extern "C" {
    pub fn agp_generic_destroy_page(page: *mut page, flags: c_int);
}
extern "C" {
    pub fn agp_generic_destroy_pages(memory: *mut agp_memory);
}
extern "C" {
    pub fn agp_free_key(key: c_int);
}
extern "C" {
    pub fn agp_num_entries() -> c_int;
}
extern "C" {
    pub fn agp_collect_device_status(bridge: *mut agp_bridge_data, mode: u32, command: u32) -> u32;
}
extern "C" {
    pub fn agp_device_command(command: u32, agp_v3: bool);
}
extern "C" {
    pub fn agp_3_5_enable(bridge: *mut agp_bridge_data) -> c_int;
}
extern "C" {
    pub fn global_cache_flush();
}
extern "C" {
    pub fn get_agp_version(bridge: *mut agp_bridge_data);
}
// generic functions for user-populated AGP memory types
extern "C" {
    pub fn agp_alloc_page_array(size: usize, mem: *mut agp_memory);
}
// generic routines for agp>=3
extern "C" {
    pub fn agp3_generic_fetch_size() -> c_int;
}
extern "C" {
    pub fn agp3_generic_tlbflush(mem: *mut agp_memory);
}
extern "C" {
    pub fn agp3_generic_configure() -> c_int;
}
extern "C" {
    pub fn agp3_generic_cleanup();
}
// GATT allocation. Returns/accepts GATT kernel virtual address.

// aperture sizes have been standardised since v3
pub const AGP_GENERIC_SIZES_ENTRIES: c_int = 11;
extern "C" {
    pub fn compat_agp_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
// Chipset independent registers (from AGP Spec)
pub const AGP_APBASE: c_uint = 0x10;
pub const AGP_APERTURE_BAR: c_int = 0;
pub const AGPSTAT: c_uint = 0x4;
pub const AGPCMD: c_uint = 0x8;
pub const AGPNISTAT: c_uint = 0xc;
pub const AGPCTRL: c_uint = 0x10;
pub const AGPAPSIZE: c_uint = 0x14;
pub const AGPNEPG: c_uint = 0x16;
pub const AGPGARTLO: c_uint = 0x18;
pub const AGPGARTHI: c_uint = 0x1c;
pub const AGPNICMD: c_uint = 0x20;

pub const AGPSTAT_RQ_DEPTH_SHIFT: c_int = 24;

pub const AGPSTAT_ARQSZ_SHIFT: c_int = 13;

pub const AGP2_RESERVED_MASK: c_uint = 0x00fffcc8;
pub const AGP3_RESERVED_MASK: c_uint = 0x00ff00c4;

