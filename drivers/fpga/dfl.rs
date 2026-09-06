//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/dfl.h
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
// Driver Header File for FPGA Device Feature List (DFL) Support
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Zhang Yi <yi.z.zhang@intel.com>
// Wu Hao <hao.wu@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
//

// maximum supported number of ports
pub const MAX_DFL_FPGA_PORT_NUM: c_int = 4;
// plus one for fme device

// Reserved 0xfe for Header Group Register and 0xff for AFU
pub const FEATURE_ID_FIU_HEADER: c_uint = 0xfe;
pub const FEATURE_ID_AFU: c_uint = 0xff;

pub const FME_FEATURE_ID_THERMAL_MGMT: c_uint = 0x1;
pub const FME_FEATURE_ID_POWER_MGMT: c_uint = 0x2;
pub const FME_FEATURE_ID_GLOBAL_IPERF: c_uint = 0x3;
pub const FME_FEATURE_ID_GLOBAL_ERR: c_uint = 0x4;
pub const FME_FEATURE_ID_PR_MGMT: c_uint = 0x5;
pub const FME_FEATURE_ID_HSSI: c_uint = 0x6;
pub const FME_FEATURE_ID_GLOBAL_DPERF: c_uint = 0x7;

pub const PORT_FEATURE_ID_ERROR: c_uint = 0x10;
pub const PORT_FEATURE_ID_UMSG: c_uint = 0x11;
pub const PORT_FEATURE_ID_UINT: c_uint = 0x12;
pub const PORT_FEATURE_ID_STP: c_uint = 0x13;
//
// Device Feature Header Register Set
//
// For FIUs, they all have DFH + GUID + NEXT_AFU as common header registers.
// For AFUs, they have DFH + GUID as common header registers.
// For private features, they only have DFH register as common header.
//
pub const DFH: c_uint = 0x0;
pub const GUID_L: c_uint = 0x8;
pub const GUID_H: c_uint = 0x10;
pub const NEXT_AFU: c_uint = 0x18;
pub const DFH_SIZE: c_uint = 0x8;
// Device Feature Header Register Bitfield

pub const DFH_ID_FIU_FME: c_int = 0;
pub const DFH_ID_FIU_PORT: c_int = 1;

pub const DFH_TYPE_AFU: c_int = 1;
pub const DFH_TYPE_PRIVATE: c_int = 3;
pub const DFH_TYPE_FIU: c_int = 4;
//
// DFHv1 Register Offset definitions
// In DHFv1, DFH + GUID + CSR_START + CSR_SIZE_GROUP + PARAM_HDR + PARAM_DATA
// as common header registers
//
pub const DFHv1_CSR_ADDR: c_uint = 0x18  /* CSR Register start address */;
pub const DFHv1_CSR_SIZE_GRP: c_uint = 0x20  /* Size of Reg Block and Group/tag */;
pub const DFHv1_PARAM_HDR: c_uint = 0x28  /* Optional First Param header */;
//
// CSR Rel Bit, 1'b0 = relative (offset from feature DFH start),
// 1'b1 = absolute (ARM or other non-PCIe use)
//

// CSR Header Register Bit Definitions

// CSR SIZE Goup Register Bit Definitions

// PARAM Header Register Bit Definitions

pub const DFHv1_PARAM_DATA: c_uint = 0x08  /* Offset of Param data from Param header */;
pub const DFHv1_PARAM_ID_MSI_X: c_uint = 0x1;

// Next AFU Register Bitfield

// FME Header Register Set

pub const FME_HDR_CAP: c_uint = 0x30;

pub const FME_PORT_OFST_BAR_SKIP: c_int = 7;
pub const FME_HDR_BITSTREAM_ID: c_uint = 0x60;
pub const FME_HDR_BITSTREAM_MD: c_uint = 0x68;
// FME Fab Capability Register Bitfield

// FME Port Offset Register Bitfield
// Offset to port device feature header

// PCI Bar ID for this port

// AFU MMIO access permission. 1 - VF, 0 - PF.

pub const FME_PORT_OFST_ACC_PF: c_int = 0;
pub const FME_PORT_OFST_ACC_VF: c_int = 1;

// FME Error Capability Register
pub const FME_ERROR_CAP: c_uint = 0x70;
// FME Error Capability Register Bitfield

// PORT Header Register Set

pub const PORT_HDR_CAP: c_uint = 0x30;
pub const PORT_HDR_CTRL: c_uint = 0x38;
pub const PORT_HDR_STS: c_uint = 0x40;
pub const PORT_HDR_USRCLK_CMD0: c_uint = 0x50;
pub const PORT_HDR_USRCLK_CMD1: c_uint = 0x58;
pub const PORT_HDR_USRCLK_STS0: c_uint = 0x60;
pub const PORT_HDR_USRCLK_STS1: c_uint = 0x68;
// Port Capability Register Bitfield

// Port Control Register Bitfield

// Latency tolerance reporting. '1' >= 40us, '0' < 40us.

// Port Status Register Bitfield

pub const PORT_STS_PWR_STATE_NORM: c_int = 0;

// Port Error Capability Register
pub const PORT_ERROR_CAP: c_uint = 0x38;
// Port Error Capability Register Bitfield

// Port Uint Capability Register
pub const PORT_UINT_CAP: c_uint = 0x8;
// Port Uint Capability Register Bitfield

//
// struct dfl_fpga_port_ops - port ops
//
// @name: name of this port ops, to match with port platform device.
// @owner: pointer to the module which owns this port ops.
// @node: node to link port ops to global list.
// @get_id: get port id from hardware.
// @enable_set: enable/disable the port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_ops {
    pub name: *const c_char,
    pub owner: *mut module,
    pub node: list_head,
    pub fdata): *mut *mut int (get_id)(struct dfl_feature_dev_data,
    pub enable): *mut *mut *mut int (enable_set)(struct dfl_feature_dev_data fdata, bool,
}

extern "C" {
    pub fn dfl_fpga_port_ops_add(ops: *mut dfl_fpga_port_ops);
}
extern "C" {
    pub fn dfl_fpga_port_ops_del(ops: *mut dfl_fpga_port_ops);
}
extern "C" {
    pub fn dfl_fpga_port_ops_put(ops: *mut dfl_fpga_port_ops);
}
extern "C" {
    pub fn dfl_fpga_check_port_id(fdata: *mut dfl_feature_dev_data, pport_id: *mut c_void) -> c_int;
}
//
// struct dfl_feature_id - dfl private feature id
//
// @id: unique dfl private feature id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_id {
    pub id: u16,
}

//
// struct dfl_feature_driver - dfl private feature driver
//
// @id_table: id_table for dfl private features supported by this driver.
// @ops: ops of this dfl private feature driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_driver {
    pub id_table: *const dfl_feature_id,
    pub ops: *const dfl_feature_ops,
}

//
// struct dfl_feature_irq_ctx - dfl private feature interrupt context
//
// @irq: Linux IRQ number of this interrupt.
// @trigger: eventfd context to signal when interrupt happens.
// @name: irq name needed when requesting irq.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_irq_ctx {
    pub irq: c_int,
    pub trigger: *mut eventfd_ctx,
    pub name: *mut c_char,
}

//
// struct dfl_feature - sub feature of the feature devices
//
// @dev: ptr to pdev of the feature device which has the sub feature.
// @id: sub feature id.
// @revision: revision of this sub feature.
// @resource_index: each sub feature has one mmio resource for its registers.
// this index is used to find its mmio resource from the
// feature dev (platform device)'s resources.
// @ioaddr: mapped mmio resource address.
// @irq_ctx: interrupt context list.
// @nr_irqs: number of interrupt contexts.
// @ops: ops of this sub feature.
// @ddev: ptr to the dfl device of this sub feature.
// @priv: priv data of this feature.
// @dfh_version: version of the DFH
// @param_size: size of dfh parameters
// @params: point to memory copy of dfh parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature {
    pub dev: *mut platform_device,
    pub id: u16,
    pub revision: u8,
    pub resource_index: c_int,
    pub ioaddr: *mut void __iomem,
    pub irq_ctx: *mut dfl_feature_irq_ctx,
    pub nr_irqs: c_uint,
    pub ops: *const dfl_feature_ops,
    pub ddev: *mut dfl_device,
    pub priv: *mut c_void,
    pub dfh_version: u8,
    pub param_size: c_uint,
    pub params: *mut c_void,
}

//
// struct dfl_feature_dev_data - dfl enumeration data for dfl feature dev.
//
// @node: node to link the data structure to container device's port_dev_list.
// @lock: mutex to protect feature dev data.
// @dev: ptr to the feature's platform device linked with this structure.
// @type: type of DFL FIU for the feature dev. See enum dfl_id_type.
// @pdev_id: platform device id for the feature dev.
// @pdev_name: platform device name for the feature dev.
// @dfl_cdev: ptr to container device.
// @id: id used for the feature device.
// @disable_count: count for port disable.
// @excl_open: set on feature device exclusive open.
// @open_count: count for feature device open.
// @num: number for sub features.
// @private: ptr to feature dev private data.
// @features: sub features for the feature dev.
// @resource_num: number of resources for the feature dev.
// @resources: resources for the feature dev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_dev_data {
    pub node: list_head,
    pub lock: mutex,
    pub dev: *mut platform_device,
    pub type: dfl_id_type,
    pub pdev_id: c_int,
    pub pdev_name: *const c_char,
    pub dfl_cdev: *mut dfl_fpga_cdev,
    pub id: c_int,
    pub disable_count: c_uint,
    pub excl_open: bool,
    pub open_count: c_int,
    pub private: *mut c_void,
    pub num: c_int,
    pub features: *mut dfl_feature,
    pub resource_num: c_int,
    pub resources: *mut resource,
}

//
// struct dfl_feature_platform_data - platform data for feature devices
//
// @cdev: cdev of feature dev.
// @fdata: dfl enumeration data for the dfl feature device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_platform_data {
    pub cdev: cdev,
    pub fdata: *mut dfl_feature_dev_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_feature_ops {
    pub feature): *mut *mut *mut int (init)(struct platform_device pdev, struct dfl_feature,
    pub feature): *mut dfl_feature,
    pub arg): unsigned int cmd, unsigned long,
}

extern "C" {
    pub fn dfl_fpga_dev_feature_uinit(pdev: *mut platform_device);
}
extern "C" {
    pub fn dfl_fpga_dev_ops_unregister(pdev: *mut platform_device);
}

//
// struct dfl_fpga_enum_info - DFL FPGA enumeration information
//
// @dev: parent device.
// @dfls: list of device feature lists.
// @nr_irqs: number of irqs for all feature devices.
// @irq_table: Linux IRQ numbers for all irqs, indexed by hw irq numbers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_enum_info {
    pub dev: *mut device,
    pub dfls: list_head,
    pub nr_irqs: c_uint,
    pub irq_table: *mut c_int,
}

//
// struct dfl_fpga_enum_dfl - DFL FPGA enumeration device feature list info
//
// @start: base address of this device feature list.
// @len: size of this device feature list.
// @node: node in list of device feature lists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_enum_dfl {
    pub start: resource_size_t,
    pub len: resource_size_t,
    pub node: list_head,
}

extern "C" {
    pub fn dfl_fpga_enum_info_free(info: *mut dfl_fpga_enum_info);
}
//
// struct dfl_fpga_cdev - container device of DFL based FPGA
//
// @parent: parent device of this container device.
// @region: base fpga region.
// @fme_dev: FME feature device under this container device.
// @lock: mutex lock to protect the port device list.
// @port_dev_list: list of all port feature devices under this container device.
// @released_port_num: released port number under this container device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_cdev {
    pub parent: *mut device,
    pub region: *mut fpga_region,
    pub fme_dev: *mut device,
    pub lock: mutex,
    pub port_dev_list: list_head,
    pub released_port_num: c_int,
}

extern "C" {
    pub fn dfl_fpga_feature_devs_remove(cdev: *mut dfl_fpga_cdev);
}
extern "C" {
    pub fn dfl_fpga_cdev_release_port(cdev: *mut dfl_fpga_cdev, port_id: c_int) -> c_int;
}
extern "C" {
    pub fn dfl_fpga_cdev_assign_port(cdev: *mut dfl_fpga_cdev, port_id: c_int) -> c_int;
}
extern "C" {
    pub fn dfl_fpga_cdev_config_ports_pf(cdev: *mut dfl_fpga_cdev);
}
extern "C" {
    pub fn dfl_fpga_cdev_config_ports_vf(cdev: *mut dfl_fpga_cdev, num_vf: c_int) -> c_int;
}
