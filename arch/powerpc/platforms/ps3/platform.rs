//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/ps3/platform.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// PS3 platform declarations.
//
// Copyright (C) 2006 Sony Computer Entertainment Inc.
// Copyright 2006 Sony Corp.
//

// htab
extern "C" {
    pub fn ps3_hpte_init(htab_size: c_ulong) -> void __init;
}
extern "C" {
    pub fn ps3_map_htab() -> void __init;
}
// mm
extern "C" {
    pub fn ps3_mm_init() -> void __init;
}
extern "C" {
    pub fn ps3_mm_vas_create(htab_size: *mut *mut c_ulong) -> void __init;
}
extern "C" {
    pub fn ps3_mm_vas_destroy();
}
extern "C" {
    pub fn ps3_mm_shutdown();
}
// irq
extern "C" {
    pub fn ps3_init_IRQ();
}
extern "C" {
    pub fn ps3_shutdown_IRQ(cpu: c_int);
}
extern "C" {
    pub fn ps3_register_ipi_debug_brk(cpu: c_uint, virq: c_uint) -> void __init;
}
extern "C" {
    pub fn ps3_register_ipi_irq(cpu: c_uint, virq: c_uint) -> void __init;
}
// smp
extern "C" {
    pub fn smp_init_ps3() -> void __init;
}

extern "C" {
    pub fn ps3_smp_cleanup_cpu(cpu: c_int);
}

// time
extern "C" {
    pub fn ps3_calibrate_decr() -> void __init;
}
extern "C" {
    pub fn ps3_get_boot_time() -> time64_t __init;
}
extern "C" {
    pub fn ps3_get_rtc_time(time: *mut rtc_time);
}
extern "C" {
    pub fn ps3_set_rtc_time(time: *mut rtc_time) -> c_int;
}
// os area
extern "C" {
    pub fn ps3_os_area_save_params() -> void __init;
}
extern "C" {
    pub fn ps3_os_area_init() -> void __init;
}
// spu

extern "C" {
    pub fn ps3_spu_set_platform();
}

// repository bus info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_bus_type {
    PS3_BUS_TYPE_SB = 4,
    PS3_BUS_TYPE_STORAGE = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_dev_type {
    PS3_DEV_TYPE_STOR_DISK = TYPE_DISK,	/* 0 */
    PS3_DEV_TYPE_SB_GELIC = 3,
    PS3_DEV_TYPE_SB_USB = 4,
    PS3_DEV_TYPE_STOR_ROM = TYPE_ROM,	/* 5 */
    PS3_DEV_TYPE_SB_GPIO = 6,
    PS3_DEV_TYPE_STOR_FLASH = TYPE_RBC,	/* 14 */
}

extern "C" {
    pub fn ps3_repository_read_bus_id(bus_index: c_uint, bus_id: *mut u64) -> c_int;
}
// repository bus device info
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_interrupt_type {
    PS3_INTERRUPT_TYPE_EVENT_PORT = 2,
    PS3_INTERRUPT_TYPE_SB_OHCI = 3,
    PS3_INTERRUPT_TYPE_SB_EHCI = 4,
    PS3_INTERRUPT_TYPE_OTHER = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_reg_type {
    PS3_REG_TYPE_SB_OHCI = 3,
    PS3_REG_TYPE_SB_EHCI = 4,
    PS3_REG_TYPE_SB_GPIO = 5,
}

// repository bus enumerators
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_repository_device {
    pub bus_index: c_uint,
    pub dev_index: c_uint,
    pub bus_type: ps3_bus_type,
    pub dev_type: ps3_dev_type,
    pub bus_id: u64,
    pub dev_id: u64,
}

extern "C" {
    pub fn ps3_repository_find_device(repo: *mut ps3_repository_device) -> c_int;
}
// repository block device info
// repository logical pu and memory info
extern "C" {
    pub fn ps3_repository_read_num_pu(num_pu: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_pu_id(pu_index: c_uint, pu_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_rm_base(ppe_id: c_uint, rm_base: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_rm_size(ppe_id: c_uint, rm_size: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_region_total(region_total: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_highmem_region_count(region_count: *mut c_uint) -> c_int;
}

extern "C" {
    pub fn ps3_repository_write_highmem_region_count(region_count: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_repository_delete_highmem_info(region_index: c_uint) -> c_int;
}

// repository pme info
extern "C" {
    pub fn ps3_repository_read_num_be(num_be: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_be_node_id(be_index: c_uint, node_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_be_id(node_id: u64, be_id: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_tb_freq(node_id: u64, tb_freq: *mut u64) -> int __init;
}
extern "C" {
    pub fn ps3_repository_read_be_tb_freq(be_index: c_uint, tb_freq: *mut u64) -> int __init;
}
// repository performance monitor info
// repository 'Other OS' area
extern "C" {
    pub fn ps3_repository_read_boot_dat_addr(lpar_addr: *mut u64) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_boot_dat_size(size: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_boot_dat_info(lpar_addr: *mut u64, size: *mut c_uint) -> c_int;
}
// repository spu info
//
// enum spu_resource_type - Type of spu resource.
// @spu_resource_type_shared: Logical spu is shared with other partions.
// @spu_resource_type_exclusive: Logical spu is not shared with other partions.
//
// Returned by ps3_repository_read_spu_resource_id().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_spu_resource_type {
    PS3_SPU_RESOURCE_TYPE_SHARED = 0,
    PS3_SPU_RESOURCE_TYPE_EXCLUSIVE = 0x8000000000000000UL,
}

extern "C" {
    pub fn ps3_repository_read_num_spu_reserved(num_spu_reserved: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_repository_read_num_spu_resource_id(num_resource_id: *mut c_uint) -> c_int;
}
// repository vuart info
extern "C" {
    pub fn ps3_repository_read_vuart_av_port(port: *mut c_uint) -> int __init;
}
extern "C" {
    pub fn ps3_repository_read_vuart_sysmgr_port(port: *mut c_uint) -> int __init;
}
