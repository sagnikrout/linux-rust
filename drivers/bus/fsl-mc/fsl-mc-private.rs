//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bus/fsl-mc/fsl-mc-private.h
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
// Freescale Management Complex (MC) bus private declarations
//
// Copyright (C) 2016 Freescale Semiconductor, Inc.
//

//
// Data Path Management Complex (DPMNG) General API
//
// DPMNG command versioning
pub const DPMNG_CMD_BASE_VERSION: c_int = 1;
pub const DPMNG_CMD_ID_OFFSET: c_int = 4;

// DPMNG command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmng_rsp_get_version {
    pub revision: __le32,
    pub version_major: __le32,
    pub version_minor: __le32,
}

//
// Data Path Management Command Portal (DPMCP) API
//
// Minimal supported DPMCP Version
pub const DPMCP_MIN_VER_MAJOR: c_int = 3;
pub const DPMCP_MIN_VER_MINOR: c_int = 0;
// DPMCP command versioning
pub const DPMCP_CMD_BASE_VERSION: c_int = 1;
pub const DPMCP_CMD_ID_OFFSET: c_int = 4;

// DPMCP command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpmcp_cmd_open {
    pub dpmcp_id: __le32,
}

//
// Initialization and runtime control APIs for DPMCP
//
// Data Path Resource Container (DPRC) API
//
// Minimal supported DPRC Version
pub const DPRC_MIN_VER_MAJOR: c_int = 6;
pub const DPRC_MIN_VER_MINOR: c_int = 0;
// DPRC command versioning
pub const DPRC_CMD_BASE_VERSION: c_int = 1;
pub const DPRC_CMD_2ND_VERSION: c_int = 2;
pub const DPRC_CMD_3RD_VERSION: c_int = 3;
pub const DPRC_CMD_ID_OFFSET: c_int = 4;

// DPRC command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_open {
    pub container_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_reset_container {
    pub child_container_id: __le32,
    pub options: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_set_irq {
// cmd word 0
    pub irq_val: __le32,
    pub irq_index: u8,
    pub pad: [u8; 3],
// cmd word 1
    pub irq_addr: __le64,
// cmd word 2
    pub irq_num: __le32,
}

pub const DPRC_ENABLE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_set_irq_enable {
    pub enable: u8,
    pub pad: [u8; 3],
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_set_irq_mask {
    pub mask: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_get_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_irq_status {
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_clear_irq_status {
    pub status: __le32,
    pub irq_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_attributes {
// response word 0
    pub container_id: __le32,
    pub icid: __le32,
// response word 1
    pub options: __le32,
    pub portal_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_obj_count {
    pub pad: __le32,
    pub obj_count: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_get_obj {
    pub obj_index: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_obj {
// response word 0
    pub pad0: __le32,
    pub id: __le32,
// response word 1
    pub vendor: __le16,
    pub irq_count: u8,
    pub region_count: u8,
    pub state: __le32,
// response word 2
    pub version_major: __le16,
    pub version_minor: __le16,
    pub flags: __le16,
    pub pad1: __le16,
// response word 3-4
    pub type: [u8; 16],
// response word 5-6
    pub label: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_get_obj_region {
// cmd word 0
    pub obj_id: __le32,
    pub pad0: __le16,
    pub region_index: u8,
    pub pad1: u8,
// cmd word 1-2
    pub pad2: [__le64; 2],
// cmd word 3-4
    pub obj_type: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_obj_region {
// response word 0
    pub pad0: __le64,
// response word 1
    pub base_offset: __le64,
// response word 2
    pub size: __le32,
    pub type: u8,
    pub pad2: [u8; 3],
// response word 3
    pub flags: __le32,
    pub pad3: __le32,
// response word 4
// base_addr may be zero if older MC firmware is used
    pub base_addr: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_set_obj_irq {
// cmd word 0
    pub irq_val: __le32,
    pub irq_index: u8,
    pub pad: [u8; 3],
// cmd word 1
    pub irq_addr: __le64,
// cmd word 2
    pub irq_num: __le32,
    pub obj_id: __le32,
// cmd word 3-4
    pub obj_type: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_cmd_get_connection {
    pub ep1_id: __le32,
    pub ep1_interface_id: __le16,
    pub pad: [u8; 2],
    pub ep1_type: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_rsp_get_connection {
    pub pad: [__le64; 3],
    pub ep2_id: __le32,
    pub ep2_interface_id: __le16,
    pub pad1: __le16,
    pub ep2_type: [u8; 16],
    pub state: __le32,
}

//
// DPRC API for managing and querying DPAA resources
//
// DPRC IRQ events
// IRQ event - Indicates that a new object added to the container
pub const DPRC_IRQ_EVENT_OBJ_ADDED: c_uint = 0x00000001;
// IRQ event - Indicates that an object was removed from the container
pub const DPRC_IRQ_EVENT_OBJ_REMOVED: c_uint = 0x00000002;
//
// IRQ event - Indicates that one of the descendant containers that opened by
// this container is destroyed
//
pub const DPRC_IRQ_EVENT_CONTAINER_DESTROYED: c_uint = 0x00000010;
//
// IRQ event - Indicates that on one of the container's opened object is
// destroyed
//
pub const DPRC_IRQ_EVENT_OBJ_DESTROYED: c_uint = 0x00000020;
// Irq event - Indicates that object is created at the container
pub const DPRC_IRQ_EVENT_OBJ_CREATED: c_uint = 0x00000040;
//
// struct dprc_irq_cfg - IRQ configuration
// @paddr:	Address that must be written to signal a message-based interrupt
// @val:	Value to write into irq_addr address
// @irq_num:	A user defined number associated with this IRQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_irq_cfg {
    pub paddr: phys_addr_t,
    pub val: u32,
    pub irq_num: c_int,
}

//
// struct dprc_attributes - Container attributes
// @container_id: Container's ID
// @icid: Container's ICID
// @portal_id: Container's portal ID
// @options: Container's options as set at container's creation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_attributes {
    pub container_id: c_int,
    pub icid: u32,
    pub portal_id: c_int,
    pub options: u64,
}

//
// enum dprc_region_type - Region type
// @DPRC_REGION_TYPE_MC_PORTAL: MC portal region
// @DPRC_REGION_TYPE_QBMAN_PORTAL: Qbman portal region
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dprc_region_type {
    DPRC_REGION_TYPE_MC_PORTAL,
    DPRC_REGION_TYPE_QBMAN_PORTAL,
    DPRC_REGION_TYPE_QBMAN_MEM_BACKED_PORTAL
}

//
// struct dprc_region_desc - Mappable region descriptor
// @base_offset: Region offset from region's base address.
// For DPMCP and DPRC objects, region base is offset from SoC MC portals
// base address; For DPIO, region base is offset from SoC QMan portals
// base address
// @size: Region size (in bytes)
// @flags: Region attributes
// @type: Portal region type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_region_desc {
    pub base_offset: u32,
    pub size: u32,
    pub flags: u32,
    pub type: dprc_region_type,
    pub base_address: u64,
}

//
// struct dprc_endpoint - Endpoint description for link connect/disconnect
// operations
// @type:	Endpoint object type: NULL terminated string
// @id:		Endpoint object ID
// @if_id:	Interface ID; should be set for endpoints with multiple
// interfaces ("dpsw", "dpdmux"); for others, always set to 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dprc_endpoint {
    pub type: [c_char; 16],
    pub id: c_int,
    pub if_id: u16,
}

//
// Data Path Buffer Pool (DPBP) API
//
// DPBP Version
pub const DPBP_VER_MAJOR: c_int = 3;
pub const DPBP_VER_MINOR: c_int = 2;
// Command versioning
pub const DPBP_CMD_BASE_VERSION: c_int = 1;
pub const DPBP_CMD_ID_OFFSET: c_int = 4;

// Command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpbp_cmd_open {
    pub dpbp_id: __le32,
}

pub const DPBP_ENABLE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpbp_rsp_get_attributes {
// response word 0
    pub pad: __le16,
    pub bpid: __le16,
    pub id: __le32,
// response word 1
    pub version_major: __le16,
    pub version_minor: __le16,
}

//
// Data Path Concentrator (DPCON) API
//
// DPCON Version
pub const DPCON_VER_MAJOR: c_int = 3;
pub const DPCON_VER_MINOR: c_int = 2;
// Command versioning
pub const DPCON_CMD_BASE_VERSION: c_int = 1;
pub const DPCON_CMD_ID_OFFSET: c_int = 4;

// Command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcon_cmd_open {
    pub dpcon_id: __le32,
}

pub const DPCON_ENABLE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcon_rsp_get_attr {
// response word 0
    pub id: __le32,
    pub qbman_ch_id: __le16,
    pub num_priorities: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpcon_cmd_set_notification {
// cmd word 0
    pub dpio_id: __le32,
    pub priority: u8,
    pub pad: [u8; 3],
// cmd word 1
    pub user_ctx: __le64,
}

//
// Generic FSL MC API
//
// generic command versioning
pub const OBJ_CMD_BASE_VERSION: c_int = 1;
pub const OBJ_CMD_ID_OFFSET: c_int = 4;

// open command codes

// Generic object command IDs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_obj_cmd_open {
    pub obj_id: __le32,
}

//
// struct fsl_mc_resource_pool - Pool of MC resources of a given
// type
// @type: type of resources in the pool
// @max_count: maximum number of resources in the pool
// @free_count: number of free resources in the pool
// @mutex: mutex to serialize access to the pool's free list
// @free_list: anchor node of list of free resources in the pool
// @mc_bus: pointer to the MC bus that owns this resource pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_resource_pool {
    pub type: fsl_mc_pool_type,
    pub max_count: c_int,
    pub free_count: c_int,
    pub /: *mut *mut mutex mutex; / serializes access to free_list,
    pub free_list: list_head,
    pub mc_bus: *mut fsl_mc_bus,
}

//
// struct fsl_mc_uapi - information associated with a device file
// @misc: struct miscdevice linked to the root dprc
// @device: newly created device in /dev
// @mutex: mutex lock to serialize the open/release operations
// @local_instance_in_use: local MC I/O instance in use or not
// @static_mc_io: pointer to the static MC I/O object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_uapi {
    pub misc: miscdevice,
    pub device: *mut device,
    pub /: *mut *mut mutex mutex; / serialize open/release operations,
    pub local_instance_in_use: u32,
    pub static_mc_io: *mut fsl_mc_io,
}

//
// struct fsl_mc_bus - logical bus that corresponds to a physical DPRC
// @mc_dev: fsl-mc device for the bus device itself.
// @resource_pools: array of resource pools (one pool per resource type)
// for this MC bus. These resources represent allocatable entities
// from the physical DPRC.
// @irq_resources: Pointer to array of IRQ objects for the IRQ pool
// @scan_mutex: Serializes bus scanning
// @dprc_attr: DPRC attributes
// @uapi_misc: struct that abstracts the interaction with userspace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_mc_bus {
    pub mc_dev: fsl_mc_device,
    pub resource_pools: [fsl_mc_resource_pool; FSL_MC_NUM_POOL_TYPES],
    pub irq_resources: *mut fsl_mc_device_irq,
    pub /: *mut *mut mutex scan_mutex; / serializes bus scanning,
    pub dprc_attr: dprc_attributes,
    pub uapi_misc: fsl_mc_uapi,
    pub irq_enabled: c_int,
}

extern "C" {
    pub fn fsl_mc_device_remove(mc_dev: *mut fsl_mc_device);
}
extern "C" {
    pub fn dprc_driver_init() -> int __init;
}
extern "C" {
    pub fn dprc_driver_exit();
}
extern "C" {
    pub fn fsl_mc_allocator_driver_init() -> int __init;
}
extern "C" {
    pub fn fsl_mc_init_all_resource_pools(mc_bus_dev: *mut fsl_mc_device);
}
// new_resource);
extern "C" {
    pub fn fsl_mc_resource_free(resource: *mut fsl_mc_resource);
}
extern "C" {
    pub fn fsl_mc_msi_domain_free_irqs(dev: *mut device);
}
extern "C" {
    pub fn fsl_destroy_mc_io(mc_io: *mut fsl_mc_io);
}
extern "C" {
    pub fn fsl_mc_is_root_dprc(dev: *mut device) -> bool;
}
extern "C" {
    pub fn mc_cmd_hdr_read_cmdid(cmd: *mut fsl_mc_command) -> u16;
}

extern "C" {
    pub fn fsl_mc_uapi_create_device_file(mc_bus: *mut fsl_mc_bus) -> c_int;
}
extern "C" {
    pub fn fsl_mc_uapi_remove_device_file(mc_bus: *mut fsl_mc_bus);
}

extern "C" {
    pub fn disable_dprc_irq(mc_dev: *mut fsl_mc_device) -> c_int;
}
extern "C" {
    pub fn enable_dprc_irq(mc_dev: *mut fsl_mc_device) -> c_int;
}
extern "C" {
    pub fn get_dprc_irq_state(mc_dev: *mut fsl_mc_device) -> c_int;
}
