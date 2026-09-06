//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/qcom/icc-rpmh.h
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
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

//
// struct qcom_icc_provider - Qualcomm specific interconnect provider
// @provider: generic interconnect provider
// @dev: reference to the NoC device
// @bcms: list of bcms that maps to the provider
// @num_bcms: number of @bcms
// @voter: bcm voter targeted by this provider
// @nodes: list of icc nodes that maps to the provider
// @num_nodes: number of @nodes
// @regmap: used for QoS, register access
// @clks : clks required for register access
// @num_clks: number of @clks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_provider {
    pub provider: icc_provider,
    pub dev: *mut device,
    pub bcms: *const *const qcom_icc_bcm,
    pub num_bcms: usize,
    pub voter: *mut bcm_voter,
    pub nodes: *const *const qcom_icc_node,
    pub num_nodes: usize,
    pub regmap: *mut regmap,
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
}

//
// struct bcm_db - Auxiliary data pertaining to each Bus Clock Manager (BCM)
// @unit: divisor used to convert bytes/sec bw value to an RPMh msg
// @width: multiplier used to convert bytes/sec bw value to an RPMh msg
// @vcd: virtual clock domain that this bcm belongs to
// @reserved: reserved field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_db {
    pub unit: __le32,
    pub width: __le16,
    pub vcd: u8,
    pub reserved: u8,
}

pub const MAX_PORTS: c_int = 4;
//
// struct qcom_icc_qosbox - Qualcomm specific QoS config
// @prio: priority value assigned to requests on the node
// @urg_fwd: whether to forward the urgency promotion issued by master
// (endpoint), or discard
// @prio_fwd_disable: whether to forward the priority driven by master, or
// override by @prio
// @num_ports: number of @ports
// @port_offsets: qos register offsets
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_qosbox {
    pub prio: u32,
    pub urg_fwd: bool,
    pub prio_fwd_disable: bool,
    pub num_ports: u32,
    pub port_offsets: [u32; MAX_PORTS],
}

pub const MAX_LINKS: c_int = 128;
pub const MAX_BCMS: c_int = 64;
pub const MAX_BCM_PER_NODE: c_int = 3;
pub const MAX_VCD: c_int = 10;
//
// struct qcom_icc_node - Qualcomm specific interconnect nodes
// @name: the node name used in debugfs
// @link_nodes: links associated with this node
// @node: icc_node associated with this node
// @num_links: the total number of @links
// @channels: num of channels at this node
// @buswidth: width of the interconnect between a node and the bus
// @sum_avg: current sum aggregate value of all avg bw requests
// @max_peak: current max aggregate value of all peak bw requests
// @bcms: list of bcms associated with this logical node
// @num_bcms: num of @bcms
// @qosbox: QoS config data associated with node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_node {
    pub name: *const c_char,
    pub node: *mut icc_node,
    pub num_links: u16,
    pub channels: u16,
    pub buswidth: u16,
    pub sum_avg: [u64; QCOM_ICC_NUM_BUCKETS],
    pub max_peak: [u64; QCOM_ICC_NUM_BUCKETS],
    pub bcms: [*mut qcom_icc_bcm; MAX_BCM_PER_NODE],
    pub num_bcms: usize,
    pub qosbox: *const qcom_icc_qosbox,
    pub link_nodes: [*mut qcom_icc_node; ],
}

//
// struct qcom_icc_bcm - Qualcomm specific hardware accelerator nodes
// known as Bus Clock Manager (BCM)
// @name: the bcm node name used to fetch BCM data from command db
// @type: latency or bandwidth bcm
// @addr: address offsets used when voting to RPMH
// @vote_x: aggregated threshold values, represents sum_bw when @type is bw bcm
// @vote_y: aggregated threshold values, represents peak_bw when @type is bw bcm
// @vote_scale: scaling factor for vote_x and vote_y
// @enable_mask: optional mask to send as vote instead of vote_x/vote_y
// @dirty: flag used to indicate whether the bcm needs to be committed
// @keepalive: flag used to indicate whether a keepalive is required
// @aux_data: auxiliary data used when calculating threshold values and
// communicating with RPMh
// @list: used to link to other bcms when compiling lists for commit
// @ws_list: used to keep track of bcms that may transition between wake/sleep
// @num_nodes: total number of @num_nodes
// @nodes: list of qcom_icc_nodes that this BCM encapsulates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_bcm {
    pub name: *const c_char,
    pub type: u32,
    pub addr: u32,
    pub vote_x: [u64; QCOM_ICC_NUM_BUCKETS],
    pub vote_y: [u64; QCOM_ICC_NUM_BUCKETS],
    pub vote_scale: u64,
    pub enable_mask: u32,
    pub dirty: bool,
    pub keepalive: bool,
    pub aux_data: bcm_db,
    pub list: list_head,
    pub ws_list: list_head,
    pub num_nodes: usize,
    pub nodes: [*mut qcom_icc_node; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_fabric {
    pub nodes: *mut qcom_icc_node,
    pub num_nodes: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_desc {
    pub config: *const regmap_config,
    pub nodes: *const *const qcom_icc_node,
    pub num_nodes: usize,
    pub bcms: *const *const qcom_icc_bcm,
    pub num_bcms: usize,
    pub qos_requires_clocks: bool,
}

extern "C" {
    pub fn qcom_icc_set(src: *mut icc_node, dst: *mut icc_node) -> c_int;
}
extern "C" {
    pub fn qcom_icc_bcm_init(bcm: *mut qcom_icc_bcm, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn qcom_icc_pre_aggregate(node: *mut icc_node);
}
extern "C" {
    pub fn qcom_icc_rpmh_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn qcom_icc_rpmh_remove(pdev: *mut platform_device);
}
