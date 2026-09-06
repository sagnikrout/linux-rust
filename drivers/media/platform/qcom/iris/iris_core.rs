//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_core.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_info {
    pub name: *const c_char,
    pub bw_min_kbps: u32,
    pub bw_max_kbps: u32,
}

pub const IRIS_FW_VERSION_LENGTH: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum domain_type {
    ENCODER	= BIT(0),
    DECODER	= BIT(1),
}

//
// struct iris_core - holds core parameters valid for all instances
//
// @dev: reference to device structure
// @reg_base: IO memory base address
// @irq: iris irq
// @v4l2_dev: a holder for v4l2 device structure
// @vdev_dec: iris video device structure for decoder
// @vdev_enc: iris video device structure for encoder
// @iris_v4l2_file_ops: iris v4l2 file ops
// @iris_v4l2_ioctl_ops_dec: iris v4l2 ioctl ops for decoder
// @iris_v4l2_ioctl_ops_enc: iris v4l2 ioctl ops for encoder
// @iris_vb2_ops: iris vb2 ops
// @icc_tbl: table of iris interconnects
// @icc_count: count of iris interconnects
// @pmdomain_tbl: table of iris power domains
// @opp_pmdomain_tbl: table of opp power domains
// @clock_tbl: table of iris clocks
// @clk_count: count of iris clocks
// @resets: table of iris reset clocks
// @controller_resets: table of controller reset clocks
// @iris_platform_data: a structure for platform data
// @iris_firmware_data: a pointer to the firmware (or HFI) specific data
// @iris_firmware_desc: a pointer to the firmware-specific descriptive data
// @ubwc_cfg: UBWC configuration for the platform
// @state: current state of core
// @iface_q_table_daddr: device address for interface queue table memory
// @sfr_daddr: device address for SFR (Sub System Failure Reason) register memory
// @iface_q_table_vaddr: virtual address for interface queue table memory
// @sfr_vaddr: virtual address for SFR (Sub System Failure Reason) register memory
// @command_queue: shared interface queue to send commands to firmware
// @message_queue: shared interface queue to receive responses from firmware
// @debug_queue: shared interface queue to receive debug info from firmware
// @lock: a lock for this strucure
// @response_packet: a pointer to response packet from fw to driver
// @header_id: id of packet header
// @packet_id: id of packet
// @power: a structure for clock and bw information
// @hfi_sys_ops: iris HFI system ops
// @core_init_done: structure of signal completion for system response
// @intr_status: interrupt status
// @sys_error_handler: a delayed work for handling system fatal error
// @instances: a list_head of all instances
// @inst_fw_caps_dec: an array of supported instance capabilities by decoder
// @inst_fw_caps_enc: an array of supported instance capabilities by encoder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iris_core {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub irq: c_int,
    pub v4l2_dev: v4l2_device,
    pub vdev_dec: *mut video_device,
    pub vdev_enc: *mut video_device,
    pub iris_v4l2_file_ops: *const v4l2_file_operations,
    pub iris_v4l2_ioctl_ops_dec: *const v4l2_ioctl_ops,
    pub iris_v4l2_ioctl_ops_enc: *const v4l2_ioctl_ops,
    pub iris_vb2_ops: *const vb2_ops,
    pub icc_tbl: *mut icc_bulk_data,
    pub icc_count: u32,
    pub pmdomain_tbl: *mut dev_pm_domain_list,
    pub opp_pmdomain_tbl: *mut dev_pm_domain_list,
    pub clock_tbl: *mut clk_bulk_data,
    pub clk_count: u32,
    pub resets: *mut reset_control_bulk_data,
    pub controller_resets: *mut reset_control_bulk_data,
    pub iris_platform_data: *const iris_platform_data,
    pub iris_firmware_data: *const iris_firmware_data,
    pub iris_firmware_desc: *const iris_firmware_desc,
    pub ubwc_cfg: *const qcom_ubwc_cfg_data,
    pub state: iris_core_state,
    pub iface_q_table_daddr: dma_addr_t,
    pub sfr_daddr: dma_addr_t,
    pub iface_q_table_vaddr: *mut c_void,
    pub sfr_vaddr: *mut c_void,
    pub command_queue: iris_iface_q_info,
    pub message_queue: iris_iface_q_info,
    pub debug_queue: iris_iface_q_info,
    pub /: *mut *mut mutex lock; / lock for core related operations,
    pub response_packet: *mut u8,
    pub header_id: u32,
    pub packet_id: u32,
    pub power: iris_core_power,
    pub hfi_sys_ops: *const iris_hfi_sys_ops,
    pub core_init_done: completion,
    pub intr_status: u32,
    pub sys_error_handler: delayed_work,
    pub instances: list_head,
// encoder and decoder have overlapping caps, so two different arrays are required
    pub inst_fw_caps_dec: [platform_inst_fw_cap; INST_FW_CAP_MAX],
    pub inst_fw_caps_enc: [platform_inst_fw_cap; INST_FW_CAP_MAX],
}

extern "C" {
    pub fn iris_core_init(core: *mut iris_core) -> c_int;
}
extern "C" {
    pub fn iris_core_deinit(core: *mut iris_core);
}
