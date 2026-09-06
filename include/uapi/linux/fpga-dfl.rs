//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/fpga-dfl.h
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
//
// Header File for FPGA DFL User API
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Zhang Yi <yi.z.zhang@intel.com>
// Wu Hao <hao.wu@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
//

pub const DFL_FPGA_API_VERSION: c_int = 0;
//
// The IOCTL interface for DFL based FPGA is designed for extensibility by
// embedding the structure length (argsz) and flags into structures passed
// between kernel and userspace. This design referenced the VFIO IOCTL
// interface (include/uapi/linux/vfio.h).
//
pub const DFL_FPGA_MAGIC: c_uint = 0xB6;
pub const DFL_FPGA_BASE: c_int = 0;
pub const DFL_PORT_BASE: c_uint = 0x40;
pub const DFL_FME_BASE: c_uint = 0x80;
// Common IOCTLs for both FME and AFU file descriptor
//
// DFL_FPGA_GET_API_VERSION - _IO(DFL_FPGA_MAGIC, DFL_FPGA_BASE + 0)
//
// Report the version of the driver API.
// Return: Driver API Version.
//

//
// DFL_FPGA_CHECK_EXTENSION - _IO(DFL_FPGA_MAGIC, DFL_FPGA_BASE + 1)
//
// Check whether an extension is supported.
// Return: 0 if not supported, otherwise the extension is supported.
//

// IOCTLs for AFU file descriptor
//
// DFL_FPGA_PORT_RESET - _IO(DFL_FPGA_MAGIC, DFL_PORT_BASE + 0)
//
// Reset the FPGA Port and its AFU. No parameters are supported.
// Userspace can do Port reset at any time, e.g. during DMA or PR. But
// it should never cause any system level issue, only functional failure
// (e.g. DMA or PR operation failure) and be recoverable from the failure.
// Return: 0 on success, -errno of failure
//

//
// DFL_FPGA_PORT_GET_INFO - _IOR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 1,
// struct dfl_fpga_port_info)
//
// Retrieve information about the fpga port.
// Driver fills the info in provided struct dfl_fpga_port_info.
// Return: 0 on success, -errno on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_info {
// Input
    pub /: *mut *mut __u32 argsz; / Structure length,
// Output
    pub /: *mut *mut __u32 flags; / Zero for now,
    pub /: *mut *mut __u32 num_regions; / The number of supported regions,
    pub /: *mut *mut __u32 num_umsgs; / The number of allocated umsgs,
}

//
// FPGA_PORT_GET_REGION_INFO - _IOWR(FPGA_MAGIC, PORT_BASE + 2,
// struct dfl_fpga_port_region_info)
//
// Retrieve information about a device memory region.
// Caller provides struct dfl_fpga_port_region_info with index value set.
// Driver returns the region info in other fields.
// Return: 0 on success, -errno on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_region_info {
// input
    pub /: *mut *mut __u32 argsz; / Structure length,
// Output
    pub /: *mut *mut __u32 flags; / Access permission,

// Input
    pub /: *mut *mut __u32 index; / Region index,

    pub padding: __u32,
// Output
    pub /: *mut *mut __u64 size; / Region size (bytes),
    pub /: *mut *mut __u64 offset; / Region offset from start of device fd,
}

//
// DFL_FPGA_PORT_DMA_MAP - _IOWR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 3,
// struct dfl_fpga_port_dma_map)
//
// Map the dma memory per user_addr and length which are provided by caller.
// Driver fills the iova in provided struct afu_port_dma_map.
// This interface only accepts page-size aligned user memory for dma mapping.
// Return: 0 on success, -errno on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_dma_map {
// Input
    pub /: *mut *mut __u32 argsz; / Structure length,
    pub /: *mut *mut __u32 flags; / Zero for now,
    pub /: *mut *mut __u64 user_addr; / Process virtual address,
    pub (bytes)*/: *mut *mut __u64 length; / Length of mapping,
// Output
    pub /: *mut *mut __u64 iova; / IO virtual address,
}

//
// DFL_FPGA_PORT_DMA_UNMAP - _IOW(FPGA_MAGIC, PORT_BASE + 4,
// struct dfl_fpga_port_dma_unmap)
//
// Unmap the dma memory per iova provided by caller.
// Return: 0 on success, -errno on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_port_dma_unmap {
// Input
    pub /: *mut *mut __u32 argsz; / Structure length,
    pub /: *mut *mut __u32 flags; / Zero for now,
    pub /: *mut *mut __u64 iova; / IO virtual address,
}

//
// struct dfl_fpga_irq_set - the argument for DFL_FPGA_XXX_SET_IRQ ioctl.
//
// @start: Index of the first irq.
// @count: The number of eventfd handler.
// @evtfds: Eventfd handlers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_irq_set {
    pub start: __u32,
    pub count: __u32,
    pub evtfds: [__s32; ],
}

//
// DFL_FPGA_PORT_ERR_GET_IRQ_NUM - _IOR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 5,
// __u32 num_irqs)
//
// Get the number of irqs supported by the fpga port error reporting private
// feature. Currently hardware supports up to 1 irq.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_PORT_ERR_SET_IRQ - _IOW(DFL_FPGA_MAGIC, DFL_PORT_BASE + 6,
// struct dfl_fpga_irq_set)
//
// Set fpga port error reporting interrupt trigger if evtfds[n] is valid.
// Unset related interrupt trigger if evtfds[n] is a negative value.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_PORT_UINT_GET_IRQ_NUM - _IOR(DFL_FPGA_MAGIC, DFL_PORT_BASE + 7,
// __u32 num_irqs)
//
// Get the number of irqs supported by the fpga AFU interrupt private
// feature.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_PORT_UINT_SET_IRQ - _IOW(DFL_FPGA_MAGIC, DFL_PORT_BASE + 8,
// struct dfl_fpga_irq_set)
//
// Set fpga AFU interrupt trigger if evtfds[n] is valid.
// Unset related interrupt trigger if evtfds[n] is a negative value.
// Return: 0 on success, -errno on failure.
//

// IOCTLs for FME file descriptor
//
// DFL_FPGA_FME_PORT_PR - _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 0,
// struct dfl_fpga_fme_port_pr)
//
// Driver does Partial Reconfiguration based on Port ID and Buffer (Image)
// provided by caller.
// Return: 0 on success, -errno on failure.
// If DFL_FPGA_FME_PORT_PR returns -EIO, that indicates the HW has detected
// some errors during PR, under this case, the user can fetch HW error info
// from the status of FME's fpga manager.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fpga_fme_port_pr {
// Input
    pub /: *mut *mut __u32 argsz; / Structure length,
    pub /: *mut *mut __u32 flags; / Zero for now,
    pub port_id: __u32,
    pub buffer_size: __u32,
    pub /: *mut *mut __u64 buffer_address; / Userspace address to the buffer for PR,
}

//
// DFL_FPGA_FME_PORT_RELEASE - _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 1,
// int port_id)
//
// Driver releases the port per Port ID provided by caller.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_FME_PORT_ASSIGN - _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 2,
// int port_id)
//
// Driver assigns the port back per Port ID provided by caller.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_FME_ERR_GET_IRQ_NUM - _IOR(DFL_FPGA_MAGIC, DFL_FME_BASE + 3,
// __u32 num_irqs)
//
// Get the number of irqs supported by the fpga fme error reporting private
// feature. Currently hardware supports up to 1 irq.
// Return: 0 on success, -errno on failure.
//

//
// DFL_FPGA_FME_ERR_SET_IRQ - _IOW(DFL_FPGA_MAGIC, DFL_FME_BASE + 4,
// struct dfl_fpga_irq_set)
//
// Set fpga fme error reporting interrupt trigger if evtfds[n] is valid.
// Unset related interrupt trigger if evtfds[n] is a negative value.
// Return: 0 on success, -errno on failure.
//

