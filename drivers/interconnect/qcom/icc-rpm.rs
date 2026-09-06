//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/qcom/icc-rpm.h
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
// Copyright (C) 2020 Linaro Ltd
//

pub const RPM_BUS_MASTER_REQ: c_uint = 0x73616d62;
pub const RPM_BUS_SLAVE_REQ: c_uint = 0x766c7362;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_icc_type {
    QCOM_ICC_NOC,
    QCOM_ICC_BIMC,
    QCOM_ICC_QNOC,
}

//
// struct rpm_clk_resource - RPM bus clock resource
// @resource_type: RPM resource type of the clock resource
// @clock_id: index of the clock resource of a specific resource type
// @branch: whether the resource represents a branch clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpm_clk_resource {
    pub resource_type: u32,
    pub clock_id: u32,
    pub branch: bool,
}

//
// struct qcom_icc_provider - Qualcomm specific interconnect provider
// @provider: generic interconnect provider
// @type: the ICC provider type
// @regmap: regmap for QoS registers read/write access
// @qos_offset: offset to QoS registers
// @ab_coeff: a percentage-based coefficient for compensating the AB calculations
// @ib_coeff: an inverse-percentage-based coefficient for compensating the IB calculations
// @bus_clk_rate: bus clock rate in Hz
// @bus_clk_desc: a pointer to a rpm_clk_resource description of bus clocks
// @bus_clk: a pointer to a HLOS-owned bus clock
// @keep_alive: whether to always keep a minimum vote on the bus clocks
// @ignore_enxio: whether to ignore ENXIO errors (for MSM8974)
// @num_intf_clks: the total number of intf_clks clk_bulk_data entries
// @intf_clks: a clk_bulk_data array of interface clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_provider {
    pub provider: icc_provider,
    pub type: qcom_icc_type,
    pub regmap: *mut regmap,
    pub qos_offset: c_uint,
    pub ab_coeff: u16,
    pub ib_coeff: u16,
    pub bus_clk_rate: [u32; QCOM_SMD_RPM_STATE_NUM],
    pub bus_clk_desc: *const rpm_clk_resource,
    pub bus_clk: *mut clk,
    pub keep_alive: bool,
    pub ignore_enxio: bool,
    pub num_intf_clks: c_int,
    pub __counted_by(num_intf_clks): clk_bulk_data intf_clks[],
}

//
// struct qcom_icc_qos - Qualcomm specific interconnect QoS parameters
// @areq_prio: node requests priority
// @prio_level: priority level for bus communication
// @limit_commands: activate/deactivate limiter mode during runtime
// @ap_owned: indicates if the node is owned by the AP or by the RPM
// @qos_mode: default qos mode for this node
// @qos_port: qos port number for finding qos registers of this node
// @urg_fwd_en: enable urgent forwarding
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_qos {
    pub areq_prio: u32,
    pub prio_level: u32,
    pub limit_commands: bool,
    pub ap_owned: bool,
    pub qos_mode: c_int,
    pub qos_port: c_int,
    pub urg_fwd_en: bool,
}

//
// struct qcom_icc_node - Qualcomm specific interconnect nodes
// @name: the node name used in debugfs
// @id: a unique node identifier
// @links: an array of nodes where we can go next while traversing
// @num_links: the total number of @links
// @channels: number of channels at this node (e.g. DDR channels)
// @buswidth: width of the interconnect between a node and the bus (bytes)
// @bus_clk_desc: a pointer to a rpm_clk_resource description of bus clocks
// @sum_avg: current sum aggregate value of all avg bw requests
// @max_peak: current max aggregate value of all peak bw requests
// @mas_rpm_id:	RPM id for devices that are bus masters
// @slv_rpm_id:	RPM id for devices that are bus slaves
// @qos: NoC QoS setting parameters
// @ab_coeff: a percentage-based coefficient for compensating the AB calculations
// @ib_coeff: an inverse-percentage-based coefficient for compensating the IB calculations
// @bus_clk_rate: a pointer to an array containing bus clock rates in Hz
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_node {
    pub name: *mut c_uchar,
    pub id: u16,
    pub links: *const u16,
    pub num_links: u16,
    pub channels: u16,
    pub buswidth: u16,
    pub bus_clk_desc: *const rpm_clk_resource,
    pub sum_avg: [u64; QCOM_SMD_RPM_STATE_NUM],
    pub max_peak: [u64; QCOM_SMD_RPM_STATE_NUM],
    pub mas_rpm_id: c_int,
    pub slv_rpm_id: c_int,
    pub qos: qcom_icc_qos,
    pub ab_coeff: u16,
    pub ib_coeff: u16,
    pub bus_clk_rate: [u32; QCOM_SMD_RPM_STATE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_desc {
    pub nodes: *const *const qcom_icc_node,
    pub num_nodes: usize,
    pub bus_clk_desc: *const rpm_clk_resource,
    pub intf_clocks: *const *const c_char,
    pub num_intf_clocks: usize,
    pub keep_alive: bool,
    pub type: qcom_icc_type,
    pub regmap_cfg: *const regmap_config,
    pub qos_offset: c_uint,
    pub ab_coeff: u16,
    pub ib_coeff: u16,
    pub peak): *mut *mut *mut *mut int (get_bw)(struct icc_node node, u32 avg, u32,
    pub ignore_enxio: bool,
}

// Valid for all bus types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qos_mode {
    NOC_QOS_MODE_INVALID = 0,
    NOC_QOS_MODE_FIXED,
    NOC_QOS_MODE_BYPASS,
}

extern "C" {
    pub fn qnoc_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn qnoc_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn qcom_icc_rpm_smd_available() -> bool;
}
extern "C" {
    pub fn qcom_icc_rpm_smd_send(ctx: c_int, rsc_type: c_int, id: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn qcom_icc_rpm_set_bus_rate(clk: *const rpm_clk_resource, ctx: c_int, rate: u32) -> c_int;
}
