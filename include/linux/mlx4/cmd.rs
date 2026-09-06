//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx4/cmd.h
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
// Copyright (c) 2006 Cisco Systems, Inc.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

// initialization and general commands
// master notify fw on finish for slave's flr
// TPT commands
// EQ commands
// CQ commands
// SRQ commands
// QP/EE commands
// special QP and management commands
// multicast commands
// miscellaneous commands
// Ethernet specific commands
// Communication channel commands
// virtual commands
// debug commands
// statistics commands
// register/delete flow steering network rules
// Update and read QCN parameters
// virtual to physical port mapping opcode modifiers
// Set port opcode modifiers
// Set port Ethernet input modifiers
//
// MLX4_RX_CSUM_MODE_VAL_NON_TCP_UDP -
// Receive checksum value is reported in CQE also for non TCP/UDP packets.
//
// MLX4_RX_CSUM_MODE_L4 -
// L4_CSUM bit in CQE, which indicates whether or not L4 checksum
// was validated correctly, is supported.
//
// MLX4_RX_CSUM_MODE_IP_OK_IP_NON_TCP_UDP -
// IP_OK CQE's field is supported also for non TCP/UDP IP packets.
//
// MLX4_RX_CSUM_MODE_MULTI_VLAN -
// Receive Checksum offload is supported for packets with more than 2 vlan headers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_rx_csum_mode {
    MLX4_RX_CSUM_MODE_VAL_NON_TCP_UDP		= 1UL << 0,
    MLX4_RX_CSUM_MODE_L4				= 1UL << 1,
    MLX4_RX_CSUM_MODE_IP_OK_IP_NON_TCP_UDP		= 1UL << 2,
    MLX4_RX_CSUM_MODE_MULTI_VLAN			= 1UL << 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_config_dev_params {
    pub vxlan_udp_dport: u16,
    pub rx_csum_flags_port_1: u8,
    pub rx_csum_flags_port_2: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_en_congestion_control_algorithm {
    MLX4_CTRL_ALGO_802_1_QAU_REACTION_POINT = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx4_en_congestion_control_opmod {
    MLX4_CONGESTION_CONTROL_GET_PARAMS,
    MLX4_CONGESTION_CONTROL_GET_STATISTICS,
    MLX4_CONGESTION_CONTROL_SET_PARAMS = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_cmd_mailbox {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
}

// Invoke a command with no output parameter
// Invoke a command with an output mailbox
//
// Invoke a command with an immediate output parameter (and copy the
// output into the caller's out_param pointer after the command
// executes).
//
extern "C" {
    pub fn mlx4_free_cmd_mailbox(dev: *mut mlx4_dev, mailbox: *mut mlx4_cmd_mailbox);
}
extern "C" {
    pub fn mlx4_comm_get_version() -> u32;
}
extern "C" {
    pub fn mlx4_set_vf_mac(dev: *mut mlx4_dev, port: c_int, vf: c_int, mac: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx4_set_vf_spoofchk(dev: *mut mlx4_dev, port: c_int, vf: c_int, setting: bool) -> c_int;
}
extern "C" {
    pub fn mlx4_get_vf_config(dev: *mut mlx4_dev, port: c_int, vf: c_int, ivf: *mut ifla_vf_info) -> c_int;
}
extern "C" {
    pub fn mlx4_set_vf_link_state(dev: *mut mlx4_dev, port: c_int, vf: c_int, link_state: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_cmd_wake_completions(dev: *mut mlx4_dev);
}
extern "C" {
    pub fn mlx4_report_internal_err_comm_event(dev: *mut mlx4_dev);
}
//
// mlx4_get_slave_default_vlan -
// return true if VST ( default vlan)
// if VST, will return vlan & qos (if not NULL)
//

