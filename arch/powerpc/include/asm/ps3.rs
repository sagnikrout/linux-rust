//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ps3.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union ps3_firmware_version {
    pub raw: u64,
    pub pad: u16,
    pub major: u16,
    pub minor: u16,
    pub rev: u16,
}

extern "C" {
    pub fn ps3_get_firmware_version(v: *mut ps3_firmware_version);
}
extern "C" {
    pub fn ps3_compare_firmware_version(major: u16, minor: u16, rev: u16) -> c_int;
}
// 'Other OS' area
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_param_av_multi_out {
    PS3_PARAM_AV_MULTI_OUT_NTSC = 0,
    PS3_PARAM_AV_MULTI_OUT_PAL_RGB = 1,
    PS3_PARAM_AV_MULTI_OUT_PAL_YCBCR = 2,
    PS3_PARAM_AV_MULTI_OUT_SECAM = 3,
}

extern "C" {
    pub fn ps3_os_area_get_av_multi_out() -> ps3_param_av_multi_out;
}
extern "C" {
    pub fn ps3_os_area_get_rtc_diff() -> u64;
}
extern "C" {
    pub fn ps3_os_area_set_rtc_diff(rtc_diff: u64);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_os_area_flash_ops {
    pub pos): *mut *mut *mut ssize_t (read)(void buf, size_t count, loff_t,
    pub pos): *const *const *const ssize_t (write)(void buf, size_t count, loff_t,
}

extern "C" {
    pub fn ps3_os_area_flash_register(ops: *const ps3_os_area_flash_ops);
}
// dma routines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_dma_page_size {
    PS3_DMA_4K = 12U,
    PS3_DMA_64K = 16U,
    PS3_DMA_1M = 20U,
    PS3_DMA_16M = 24U,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_dma_region_type {
    PS3_DMA_OTHER = 0,
    PS3_DMA_INTERNAL = 2,
}

//
// struct ps3_dma_region - A per device dma state variables structure
// @dev: device structure
// @did: The HV device id.
// @page_size: The ioc pagesize.
// @region_type: The HV region type.
// @bus_addr: The 'translated' bus address of the region.
// @len: The length in bytes of the region.
// @offset: The offset from the start of memory of the region.
// @dma_mask: Device dma_mask.
// @ioid: The IOID of the device who owns this region
// @chunk_list: Opaque variable used by the ioc page manager.
// @region_ops: struct ps3_dma_region_ops - dma region operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_dma_region {
    pub dev: *mut ps3_system_bus_device,
// device variables
    pub region_ops: *const ps3_dma_region_ops,
    pub ioid: c_uchar,
    pub page_size: ps3_dma_page_size,
    pub region_type: ps3_dma_region_type,
    pub len: c_ulong,
    pub offset: c_ulong,
    pub dma_mask: u64,
// driver variables  (set by ps3_dma_region_create)
    pub bus_addr: c_ulong,
    pub lock: spinlock_t,
    pub head: list_head,
    pub chunk_list: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_dma_region_ops {
    pub ): *mut *mut int (create)(struct ps3_dma_region,
    pub ): *mut *mut int (free)(struct ps3_dma_region,
    pub iopte_pp): u64,
    pub len): c_ulong,
}

//
// struct ps3_dma_region_init - Helper to initialize structure variables
//
// Helper to properly initialize variables prior to calling
// ps3_system_bus_device_register.
//
extern "C" {
    pub fn ps3_dma_region_create(r: *mut ps3_dma_region) -> c_int;
}
extern "C" {
    pub fn ps3_dma_region_free(r: *mut ps3_dma_region) -> c_int;
}
// mmio routines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_mmio_page_size {
    PS3_MMIO_4K = 12U,
    PS3_MMIO_64K = 16U
}

//
// struct ps3_mmio_region - a per device mmio state variables structure
//
// Current systems can be supported with a single region per device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_mmio_region {
    pub dev: *mut ps3_system_bus_device,
    pub mmio_ops: *const ps3_mmio_region_ops,
    pub bus_addr: c_ulong,
    pub len: c_ulong,
    pub page_size: ps3_mmio_page_size,
    pub lpar_addr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_mmio_region_ops {
    pub ): *mut *mut int (create)(struct ps3_mmio_region,
    pub ): *mut *mut int (free)(struct ps3_mmio_region,
}

//
// ps3_mmio_region_init - Helper to initialize structure variables
//
// Helper to properly initialize variables prior to calling
// ps3_system_bus_device_register.
//
// Returns: %0 on success, %-errno on error (or BUG())
//
extern "C" {
    pub fn ps3_mmio_region_create(r: *mut ps3_mmio_region) -> c_int;
}
extern "C" {
    pub fn ps3_free_mmio_region(r: *mut ps3_mmio_region) -> c_int;
}
extern "C" {
    pub fn ps3_mm_phys_to_lpar(phys_addr: c_ulong) -> c_ulong;
}
// inrerrupt routines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_cpu_binding {
    PS3_BINDING_CPU_ANY = -1,
    PS3_BINDING_CPU_0 = 0,
    PS3_BINDING_CPU_1 = 1,
}

extern "C" {
    pub fn ps3_irq_plug_destroy(virq: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_event_receive_port_setup(cpu: ps3_cpu_binding, virq: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_event_receive_port_destroy(virq: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_send_event_locally(virq: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_io_irq_destroy(virq: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_vuart_irq_destroy(virq: c_uint) -> c_int;
}
extern "C" {
    pub fn ps3_spe_irq_destroy(virq: c_uint) -> c_int;
}
// lv1 result codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lv1_result {
    LV1_SUCCESS                     = 0,
// not used                       -1
    LV1_RESOURCE_SHORTAGE           = -2,
    LV1_NO_PRIVILEGE                = -3,
    LV1_DENIED_BY_POLICY            = -4,
    LV1_ACCESS_VIOLATION            = -5,
    LV1_NO_ENTRY                    = -6,
    LV1_DUPLICATE_ENTRY             = -7,
    LV1_TYPE_MISMATCH               = -8,
    LV1_BUSY                        = -9,
    LV1_EMPTY                       = -10,
    LV1_WRONG_STATE                 = -11,
// not used                       -12
    LV1_NO_MATCH                    = -13,
    LV1_ALREADY_CONNECTED           = -14,
    LV1_UNSUPPORTED_PARAMETER_VALUE = -15,
    LV1_CONDITION_NOT_SATISFIED     = -16,
    LV1_ILLEGAL_PARAMETER_VALUE     = -17,
    LV1_BAD_OPTION                  = -18,
    LV1_IMPLEMENTATION_LIMITATION   = -19,
    LV1_NOT_IMPLEMENTED             = -20,
    LV1_INVALID_CLASS_ID            = -21,
    LV1_CONSTRAINT_NOT_SATISFIED    = -22,
    LV1_ALIGNMENT_ERROR             = -23,
    LV1_HARDWARE_ERROR              = -24,
    LV1_INVALID_DATA_FORMAT         = -25,
    LV1_INVALID_OPERATION           = -26,
    LV1_INTERNAL_ERROR              = -32768,
}

// system bus routines
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_match_id {
    PS3_MATCH_ID_EHCI		= 1,
    PS3_MATCH_ID_OHCI		= 2,
    PS3_MATCH_ID_GELIC		= 3,
    PS3_MATCH_ID_AV_SETTINGS	= 4,
    PS3_MATCH_ID_SYSTEM_MANAGER	= 5,
    PS3_MATCH_ID_STOR_DISK		= 6,
    PS3_MATCH_ID_STOR_ROM		= 7,
    PS3_MATCH_ID_STOR_FLASH		= 8,
    PS3_MATCH_ID_SOUND		= 9,
    PS3_MATCH_ID_GPU		= 10,
    PS3_MATCH_ID_LPM		= 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_match_sub_id {
    PS3_MATCH_SUB_ID_GPU_FB		= 1,
    PS3_MATCH_SUB_ID_GPU_RAMDISK	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_system_bus_device_type {
    PS3_DEVICE_TYPE_IOC0 = 1,
    PS3_DEVICE_TYPE_SB,
    PS3_DEVICE_TYPE_VUART,
    PS3_DEVICE_TYPE_LPM,
}

//
// struct ps3_system_bus_device - a device on the system bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_system_bus_device {
    pub match_id: ps3_match_id,
    pub match_sub_id: ps3_match_sub_id,
    pub dev_type: ps3_system_bus_device_type,
    pub /: *mut *mut u64 bus_id; / SB,
    pub /: *mut *mut u64 dev_id; / SB,
    pub /: *mut *mut unsigned int interrupt_id; / SB,
    pub /: *mut *mut *mut ps3_dma_region d_region; / SB, IOC0,
    pub IOC0*/: *mut *mut *mut ps3_mmio_region m_region; / SB,,
    pub /: *mut *mut unsigned int port_number; / VUART,
    pub node_id: u64,
    pub pu_id: u64,
    pub rights: u64,
    pub lpm: },
// struct iommu_table *iommu_table; -- waiting for BenH's cleanups
    pub core: device,
    pub /: *mut *mut *mut void driver_priv; / private driver variables,
}

extern "C" {
    pub fn ps3_open_hv_device(dev: *mut ps3_system_bus_device) -> c_int;
}
extern "C" {
    pub fn ps3_close_hv_device(dev: *mut ps3_system_bus_device) -> c_int;
}
//
// struct ps3_system_bus_driver - a driver for a device on the system bus
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_system_bus_driver {
    pub match_id: ps3_match_id,
    pub match_sub_id: ps3_match_sub_id,
    pub core: device_driver,
    pub ): *mut *mut int (probe)(struct ps3_system_bus_device,
    pub ): *mut *mut void (remove)(struct ps3_system_bus_device,
    pub ): *mut *mut void (shutdown)(struct ps3_system_bus_device,
// int (*suspend)(struct ps3_system_bus_device *, pm_message_t);
// int (*resume)(struct ps3_system_bus_device *);
}

extern "C" {
    pub fn ps3_system_bus_device_register(dev: *mut ps3_system_bus_device) -> c_int;
}
extern "C" {
    pub fn ps3_system_bus_driver_register(drv: *mut ps3_system_bus_driver) -> c_int;
}
extern "C" {
    pub fn ps3_system_bus_driver_unregister(drv: *mut ps3_system_bus_driver);
}

extern "C" {
    pub fn container_of(_arg: _dev, ps3_system_bus_device: struct, _arg: core) -> return;
}
extern "C" {
    pub fn ps3_drv_to_system_bus_drv(_arg: _dev->core.driver) -> return;
}
//
// ps3_system_bus_set_drvdata - set driver's private data for this device
// @dev: device structure
// @data: Data to set
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->core) -> return;
}
// system manager
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_sys_manager_ops {
    pub dev: *mut ps3_system_bus_device,
    pub dev): *mut *mut void (power_off)(struct ps3_system_bus_device,
    pub dev): *mut *mut void (restart)(struct ps3_system_bus_device,
}

extern "C" {
    pub fn ps3_sys_manager_register_ops(ops: *const ps3_sys_manager_ops);
}
extern "C" {
    pub fn ps3_sys_manager_power_off() -> void __noreturn;
}
extern "C" {
    pub fn ps3_sys_manager_restart() -> void __noreturn;
}
extern "C" {
    pub fn ps3_sys_manager_halt() -> void __noreturn;
}
extern "C" {
    pub fn ps3_sys_manager_get_wol() -> c_int;
}
extern "C" {
    pub fn ps3_sys_manager_set_wol(state: c_int);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ps3_prealloc {
    pub name: *const c_char,
    pub address: *mut c_void,
    pub size: c_ulong,
    pub align: c_ulong,
}

// logical performance monitor
//
// enum ps3_lpm_rights - Rigths granted by the system policy module.
//
// @PS3_LPM_RIGHTS_USE_LPM: The right to use the lpm.
// @PS3_LPM_RIGHTS_USE_TB: The right to use the internal trace buffer.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_lpm_rights {
    PS3_LPM_RIGHTS_USE_LPM = 0x001,
    PS3_LPM_RIGHTS_USE_TB = 0x100,
}

//
// enum ps3_lpm_tb_type - Type of trace buffer lv1 should use.
//
// @PS3_LPM_TB_TYPE_NONE: Do not use a trace buffer.
// @PS3_LPM_TB_TYPE_INTERNAL: Use the lv1 internal trace buffer.  Must have
// rights @PS3_LPM_RIGHTS_USE_TB.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps3_lpm_tb_type {
    PS3_LPM_TB_TYPE_NONE = 0,
    PS3_LPM_TB_TYPE_INTERNAL = 1,
}

extern "C" {
    pub fn ps3_lpm_close() -> c_int;
}
extern "C" {
    pub fn ps3_set_bookmark(bookmark: u64);
}
extern "C" {
    pub fn ps3_set_pm_bookmark(tag: u64, incident: u64, th_id: u64);
}
extern "C" {
    pub fn ps3_read_phys_ctr(cpu: u32, phys_ctr: u32) -> u32;
}
extern "C" {
    pub fn ps3_write_phys_ctr(cpu: u32, phys_ctr: u32, val: u32);
}
extern "C" {
    pub fn ps3_read_ctr(cpu: u32, ctr: u32) -> u32;
}
extern "C" {
    pub fn ps3_write_ctr(cpu: u32, ctr: u32, val: u32);
}
extern "C" {
    pub fn ps3_read_pm07_control(cpu: u32, ctr: u32) -> u32;
}
extern "C" {
    pub fn ps3_write_pm07_control(cpu: u32, ctr: u32, val: u32);
}
extern "C" {
    pub fn ps3_read_pm(cpu: u32, reg: pm_reg_name) -> u32;
}
extern "C" {
    pub fn ps3_write_pm(cpu: u32, reg: pm_reg_name, val: u32);
}
extern "C" {
    pub fn ps3_get_ctr_size(cpu: u32, phys_ctr: u32) -> u32;
}
extern "C" {
    pub fn ps3_set_ctr_size(cpu: u32, phys_ctr: u32, ctr_size: u32);
}
extern "C" {
    pub fn ps3_enable_pm(cpu: u32);
}
extern "C" {
    pub fn ps3_disable_pm(cpu: u32);
}
extern "C" {
    pub fn ps3_enable_pm_interrupts(cpu: u32, thread: u32, mask: u32);
}
extern "C" {
    pub fn ps3_disable_pm_interrupts(cpu: u32);
}
extern "C" {
    pub fn ps3_get_and_clear_pm_interrupts(cpu: u32) -> u32;
}
extern "C" {
    pub fn ps3_sync_irq(node: c_int);
}
extern "C" {
    pub fn ps3_get_hw_thread_id(cpu: c_int) -> u32;
}
extern "C" {
    pub fn ps3_get_spe_id(arg: *mut c_void) -> u64;
}
extern "C" {
    pub fn ps3_early_mm_init();
}

extern "C" {
    pub fn udbg_shutdown_ps3gelic();
}

