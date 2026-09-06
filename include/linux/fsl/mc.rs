//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/mc.h
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
// Freescale Management Complex (MC) bus public interface
//
// Copyright (C) 2014-2016 Freescale Semiconductor, Inc.
// Copyright 2019-2020 NXP
// Author: German Rivera <German.Rivera@freescale.com>
//

pub const FSL_MC_VENDOR_FREESCALE: c_uint = 0x1957;
//
// struct fsl_mc_driver - MC object device driver object
// @driver: Generic device driver
// @match_id_table: table of supported device matching Ids
// @probe: Function called when a device is added
// @remove: Function called when a device is removed
// @shutdown: Function called at shutdown time to quiesce the device
// @suspend: Function called when a device is stopped
// @resume: Function called when a device is resumed
// @driver_managed_dma: Device driver doesn't use kernel DMA API for DMA.
// For most device drivers, no need to care about this flag
// as long as all DMAs are handled through the kernel DMA API.
// For some special ones, for example VFIO drivers, they know
// how to manage the DMA themselves and set this flag so that
// the IOMMU layer will allow them to setup and manage their
// own I/O address space.
//
// Generic DPAA device driver object for device drivers that are registered
// with a DPRC bus. This structure is to be embedded in each device-specific
// driver structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_driver {
    pub driver: device_driver,
    pub match_id_table: *const fsl_mc_device_id,
    pub dev): *mut *mut int (probe)(struct fsl_mc_device,
    pub dev): *mut *mut void (remove)(struct fsl_mc_device,
    pub dev): *mut *mut void (shutdown)(struct fsl_mc_device,
    pub state): *mut *mut *mut int (suspend)(struct fsl_mc_device dev, pm_message_t,
    pub dev): *mut *mut int (resume)(struct fsl_mc_device,
    pub driver_managed_dma: bool,
}

//
// enum fsl_mc_pool_type - Types of allocatable MC bus resources
//
// Entries in these enum are used as indices in the array of resource
// pools of an fsl_mc_bus object.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsl_mc_pool_type {
    FSL_MC_POOL_DPMCP = 0x0,    /* corresponds to "dpmcp" in the MC */
    FSL_MC_POOL_DPBP,	    /* corresponds to "dpbp" in the MC */
    FSL_MC_POOL_DPCON,	    /* corresponds to "dpcon" in the MC */
    FSL_MC_POOL_IRQ,

//
// NOTE: New resource pool types must be added before this entry
//
    FSL_MC_NUM_POOL_TYPES
}

//
// struct fsl_mc_resource - MC generic resource
// @type: type of resource
// @id: unique MC resource Id within the resources of the same type
// @data: pointer to resource-specific data if the resource is currently
// allocated, or NULL if the resource is not currently allocated.
// @parent_pool: pointer to the parent resource pool from which this
// resource is allocated from.
// @node: Node in the free list of the corresponding resource pool
//
// NOTE: This structure is to be embedded as a field of specific
// MC resource structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_resource {
    pub type: fsl_mc_pool_type,
    pub id: i32,
    pub data: *mut c_void,
    pub parent_pool: *mut fsl_mc_resource_pool,
    pub node: list_head,
}

//
// struct fsl_mc_device_irq - MC object device message-based interrupt
// @virq: Linux virtual interrupt number
// @mc_dev: MC object device that owns this interrupt
// @dev_irq_index: device-relative IRQ index
// @resource: MC generic resource associated with the interrupt
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_device_irq {
    pub virq: c_uint,
    pub mc_dev: *mut fsl_mc_device,
    pub dev_irq_index: u8,
    pub resource: fsl_mc_resource,
}

// Opened state - Indicates that an object is open by at least one owner
pub const FSL_MC_OBJ_STATE_OPEN: c_uint = 0x00000001;
// Plugged state - Indicates that the object is plugged
pub const FSL_MC_OBJ_STATE_PLUGGED: c_uint = 0x00000002;
//
// Shareability flag - Object flag indicating no memory shareability.
// the object generates memory accesses that are non coherent with other
// masters;
// user is responsible for proper memory handling through IOMMU configuration.
//
pub const FSL_MC_OBJ_FLAG_NO_MEM_SHAREABILITY: c_uint = 0x0001;
//
// struct fsl_mc_obj_desc - Object descriptor
// @type: Type of object: NULL terminated string
// @id: ID of logical object resource
// @vendor: Object vendor identifier
// @ver_major: Major version number
// @ver_minor:  Minor version number
// @irq_count: Number of interrupts supported by the object
// @region_count: Number of mappable regions supported by the object
// @state: Object state: combination of FSL_MC_OBJ_STATE_ states
// @label: Object label: NULL terminated string
// @flags: Object's flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_obj_desc {
    pub type: [c_char; 16],
    pub id: c_int,
    pub vendor: u16,
    pub ver_major: u16,
    pub ver_minor: u16,
    pub irq_count: u8,
    pub region_count: u8,
    pub state: u32,
    pub label: [c_char; 16],
    pub flags: u16,
}

//
// Bit masks for a MC object device (struct fsl_mc_device) flags
//
pub const FSL_MC_IS_DPRC: c_uint = 0x0001;
// Region flags
// Indicates that region can be mapped as cacheable
pub const FSL_MC_REGION_CACHEABLE: c_uint = 0x00000001;
// Indicates that region can be mapped as shareable
pub const FSL_MC_REGION_SHAREABLE: c_uint = 0x00000002;
//
// struct fsl_mc_device - MC object device object
// @dev: Linux driver model device object
// @dma_mask: Default DMA mask
// @flags: MC object device flags
// @icid: Isolation context ID for the device
// @mc_handle: MC handle for the corresponding MC object opened
// @mc_io: Pointer to MC IO object assigned to this device or
// NULL if none.
// @obj_desc: MC description of the DPAA device
// @regions: pointer to array of MMIO region entries
// @irqs: pointer to array of pointers to interrupts allocated to this device
// @resource: generic resource associated with this MC object device, if any.
//
// Generic device object for MC object devices that are "attached" to a
// MC bus.
//
// NOTES:
// - For a non-DPRC object its icid is the same as its parent DPRC's icid.
// - The SMMU notifier callback gets invoked after device_add() has been
// called for an MC object device, but before the device-specific probe
// callback gets called.
// - DP_OBJ_DPRC objects are the only MC objects that have built-in MC
// portals. For all other MC objects, their device drivers are responsible for
// allocating MC portals for them by calling fsl_mc_portal_allocate().
// - Some types of MC objects (e.g., DP_OBJ_DPBP, DP_OBJ_DPCON) are
// treated as resources that can be allocated/deallocated from the
// corresponding resource pool in the object's parent DPRC, using the
// fsl_mc_object_allocate()/fsl_mc_object_free() functions. These MC objects
// are known as "allocatable" objects. For them, the corresponding
// fsl_mc_device's 'resource' points to the associated resource object.
// For MC objects that are not allocatable (e.g., DP_OBJ_DPRC, DP_OBJ_DPNI),
// 'resource' is NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_device {
    pub dev: device,
    pub dma_mask: u64,
    pub flags: u16,
    pub icid: u32,
    pub mc_handle: u16,
    pub mc_io: *mut fsl_mc_io,
    pub obj_desc: fsl_mc_obj_desc,
    pub regions: *mut resource,
    pub irqs: *mut fsl_mc_device_irq,
    pub resource: *mut fsl_mc_resource,
    pub consumer_link: *mut device_link,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_cmd_header {
    pub src_id: u8,
    pub flags_hw: u8,
    pub status: u8,
    pub flags_sw: u8,
    pub token: __le16,
    pub cmd_id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mc_cmd_status {
    MC_CMD_STATUS_OK = 0x0, /* Completed successfully */
    MC_CMD_STATUS_READY = 0x1, /* Ready to be processed */
    MC_CMD_STATUS_AUTH_ERR = 0x3, /* Authentication error */
    MC_CMD_STATUS_NO_PRIVILEGE = 0x4, /* No privilege */
    MC_CMD_STATUS_DMA_ERR = 0x5, /* DMA or I/O error */
    MC_CMD_STATUS_CONFIG_ERR = 0x6, /* Configuration error */
    MC_CMD_STATUS_TIMEOUT = 0x7, /* Operation timed out */
    MC_CMD_STATUS_NO_RESOURCE = 0x8, /* No resources */
    MC_CMD_STATUS_NO_MEMORY = 0x9, /* No memory available */
    MC_CMD_STATUS_BUSY = 0xA, /* Device is busy */
    MC_CMD_STATUS_UNSUPPORTED_OP = 0xB, /* Unsupported operation */
    MC_CMD_STATUS_INVALID_STATE = 0xC /* Invalid state */
}

//
// MC command flags
//
// High priority flag
pub const MC_CMD_FLAG_PRI: c_uint = 0x80;
// Command completion flag
pub const MC_CMD_FLAG_INTR_DIS: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_rsp_create {
    pub object_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_rsp_api_ver {
    pub major_ver: __le16,
    pub minor_ver: __le16,
}

extern "C" {
    pub fn le32_to_cpu(_arg: rsp_params->object_id) -> return;
}
// major_ver = le16_to_cpu(rsp_params->major_ver);
// minor_ver = le16_to_cpu(rsp_params->minor_ver);
//
// Bit masks for a MC I/O object (struct fsl_mc_io) flags
//
pub const FSL_MC_IO_ATOMIC_CONTEXT_PORTAL: c_uint = 0x0001;
//
// struct fsl_mc_io - MC I/O object to be passed-in to mc_send_command()
// @dev: device associated with this Mc I/O object
// @flags: flags for mc_send_command()
// @portal_size: MC command portal size in bytes
// @portal_phys_addr: MC command portal physical address
// @portal_virt_addr: MC command portal virtual address
// @dpmcp_dev: pointer to the DPMCP device associated with the MC portal.
//
// Fields are only meaningful if the FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag is not
// set:
// @mutex: Mutex to serialize mc_send_command() calls that use the same MC
// portal, if the fsl_mc_io object was created with the
// FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag off. mc_send_command() calls for this
// fsl_mc_io object must be made only from non-atomic context.
//
// Fields are only meaningful if the FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag is
// set:
// @spinlock: Spinlock to serialize mc_send_command() calls that use the same MC
// portal, if the fsl_mc_io object was created with the
// FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag on. mc_send_command() calls for this
// fsl_mc_io object can be made from atomic or non-atomic context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_io {
    pub dev: *mut device,
    pub flags: u16,
    pub portal_size: u32,
    pub portal_phys_addr: phys_addr_t,
    pub portal_virt_addr: *mut void __iomem,
    pub dpmcp_dev: *mut fsl_mc_device,
//
// This field is only meaningful if the
// FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag is not set
//
    pub /: *mut *mut mutex mutex; / serializes mc_send_command(),
//
// This field is only meaningful if the
// FSL_MC_IO_ATOMIC_CONTEXT_PORTAL flag is set
//
    pub /: *mut *mut raw_spinlock_t spinlock; / serializes mc_send_command(),
}

extern "C" {
    pub fn mc_send_command(mc_io: *mut fsl_mc_io, cmd: *mut fsl_mc_command) -> c_int;
}

extern "C" {
    pub fn fsl_mc_get_msi_id(dev: *mut device) -> u32;
}

// If fsl-mc bus is not present device cannot belong to fsl-mc bus

// Macro to check if a device is a container device

// Macro to get the container device of a MC device

//
// module_fsl_mc_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

//
// Macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn fsl_mc_driver_unregister(driver: *mut fsl_mc_driver);
}
//
// struct fsl_mc_version
// @major: Major version number: incremented on API compatibility changes
// @minor: Minor version number: incremented on API additions (that are
// backward compatible); reset when major version is incremented
// @revision: Internal revision number: incremented on implementation changes
// and/or bug fixes that have no impact on API
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_version {
    pub major: u32,
    pub minor: u32,
    pub revision: u32,
}

extern "C" {
    pub fn fsl_mc_portal_free(mc_io: *mut fsl_mc_io);
}
extern "C" {
    pub fn fsl_mc_object_free(mc_adev: *mut fsl_mc_device);
}
extern "C" {
    pub fn fsl_mc_allocate_irqs(mc_dev: *mut fsl_mc_device) -> int __must_check;
}
extern "C" {
    pub fn fsl_mc_free_irqs(mc_dev: *mut fsl_mc_device);
}
pub const DPRC_RESET_OPTION_NON_RECURSIVE: c_uint = 0x00000001;
extern "C" {
    pub fn dprc_cleanup(mc_dev: *mut fsl_mc_device) -> c_int;
}
extern "C" {
    pub fn dprc_setup(mc_dev: *mut fsl_mc_device) -> c_int;
}
//
// Maximum number of total IRQs that can be pre-allocated for an MC bus'
// IRQ pool
//
pub const FSL_MC_IRQ_POOL_MAX_TOTAL_IRQS: c_int = 256;
extern "C" {
    pub fn fsl_mc_cleanup_irq_pool(mc_bus_dev: *mut fsl_mc_device);
}
//
// Data Path Buffer Pool (DPBP) API
// Contains initialization APIs and runtime control APIs for DPBP
//
// struct dpbp_attr - Structure representing DPBP attributes
// @id:		DPBP object ID
// @bpid:	Hardware buffer pool ID; should be used as an argument in
// acquire/release operations on buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpbp_attr {
    pub id: c_int,
    pub bpid: u16,
}

// Data Path Concentrator (DPCON) API
// Contains initialization APIs and runtime control APIs for DPCON
//
// Use it to disable notifications; see dpcon_set_notification()
//

//
// struct dpcon_attr - Structure representing DPCON attributes
// @id: DPCON object ID
// @qbman_ch_id: Channel ID to be used by dequeue operation
// @num_priorities: Number of priorities for the DPCON channel (1-8)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcon_attr {
    pub id: c_int,
    pub qbman_ch_id: u16,
    pub num_priorities: u8,
}

//
// struct dpcon_notification_cfg - Structure representing notification params
// @dpio_id:	DPIO object ID; must be configured with a notification channel;
// to disable notifications set it to 'DPCON_INVALID_DPIO_ID';
// @priority:	Priority selection within the DPIO channel; valid values
// are 0-7, depending on the number of priorities in that channel
// @user_ctx:	User context value provided with each CDAN message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcon_notification_cfg {
    pub dpio_id: c_int,
    pub priority: u8,
    pub user_ctx: u64,
}
