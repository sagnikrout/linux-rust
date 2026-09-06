//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep/octep_ctrl_net.h
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
// Marvell Octeon EP (EndPoint) Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//

// Supported commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_net_cmd {
    OCTEP_CTRL_NET_CMD_GET = 0,
    OCTEP_CTRL_NET_CMD_SET,
}

// Supported states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_net_state {
    OCTEP_CTRL_NET_STATE_DOWN = 0,
    OCTEP_CTRL_NET_STATE_UP,
}

// Supported replies
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_net_reply {
    OCTEP_CTRL_NET_REPLY_OK = 0,
    OCTEP_CTRL_NET_REPLY_GENERIC_FAIL,
    OCTEP_CTRL_NET_REPLY_INVALID_PARAM,
}

// Supported host to fw commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_net_h2f_cmd {
    OCTEP_CTRL_NET_H2F_CMD_INVALID = 0,
    OCTEP_CTRL_NET_H2F_CMD_MTU,
    OCTEP_CTRL_NET_H2F_CMD_MAC,
    OCTEP_CTRL_NET_H2F_CMD_GET_IF_STATS,
    OCTEP_CTRL_NET_H2F_CMD_GET_XSTATS,
    OCTEP_CTRL_NET_H2F_CMD_GET_Q_STATS,
    OCTEP_CTRL_NET_H2F_CMD_LINK_STATUS,
    OCTEP_CTRL_NET_H2F_CMD_RX_STATE,
    OCTEP_CTRL_NET_H2F_CMD_LINK_INFO,
    OCTEP_CTRL_NET_H2F_CMD_GET_INFO,
    OCTEP_CTRL_NET_H2F_CMD_DEV_REMOVE,
    OCTEP_CTRL_NET_H2F_CMD_OFFLOADS,
    OCTEP_CTRL_NET_H2F_CMD_MAX
}

// Supported fw to host commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_ctrl_net_f2h_cmd {
    OCTEP_CTRL_NET_F2H_CMD_INVALID = 0,
    OCTEP_CTRL_NET_F2H_CMD_LINK_STATUS,
    OCTEP_CTRL_NET_F2H_CMD_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union octep_ctrl_net_req_hdr {
    pub words: [u64; 1],
// sender id
    pub sender: u16,
// receiver id
    pub receiver: u16,
// octep_ctrl_net_h2t_cmd
    pub cmd: u16,
// reserved
    pub rsvd0: u16,
    pub s: },
}

// get/set mtu request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req_cmd_mtu {
// enum octep_ctrl_net_cmd
    pub cmd: u16,
// 0-65535
    pub val: u16,
}

// get/set mac request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req_cmd_mac {
// enum octep_ctrl_net_cmd
    pub cmd: u16,
// xx:xx:xx:xx:xx:xx
    pub addr: [u8; ETH_ALEN],
}

// get/set link state, rx state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req_cmd_state {
// enum octep_ctrl_net_cmd
    pub cmd: u16,
// enum octep_ctrl_net_state
    pub state: u16,
}

// link info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_link_info {
// Bitmap of Supported link speeds/modes
    pub supported_modes: u64,
// Bitmap of Advertised link speeds/modes
    pub advertised_modes: u64,
// Autonegotation state; bit 0=disabled; bit 1=enabled
    pub autoneg: u8,
// Pause frames setting. bit 0=disabled; bit 1=enabled
    pub pause: u8,
// Negotiated link speed in Mbps
    pub speed: u32,
}

// get/set link info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req_cmd_link_info {
// enum octep_ctrl_net_cmd
    pub cmd: u16,
// struct octep_ctrl_net_link_info
    pub info: octep_ctrl_net_link_info,
}

// offloads
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_offloads {
// supported rx offloads OCTEP_RX_OFFLOAD_*
    pub rx_offloads: u16,
// supported tx offloads OCTEP_TX_OFFLOAD_*
    pub tx_offloads: u16,
// reserved
    pub reserved_offloads: u32,
// extra offloads
    pub ext_offloads: u64,
}

// get/set offloads
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req_cmd_offloads {
// enum octep_ctrl_net_cmd
    pub cmd: u16,
// struct octep_ctrl_net_offloads
    pub offloads: octep_ctrl_net_offloads,
}

// Host to fw request data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_req {
    pub hdr: octep_ctrl_net_req_hdr,
    pub mtu: octep_ctrl_net_h2f_req_cmd_mtu,
    pub mac: octep_ctrl_net_h2f_req_cmd_mac,
    pub link: octep_ctrl_net_h2f_req_cmd_state,
    pub rx: octep_ctrl_net_h2f_req_cmd_state,
    pub link_info: octep_ctrl_net_h2f_req_cmd_link_info,
    pub offloads: octep_ctrl_net_h2f_req_cmd_offloads,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union octep_ctrl_net_resp_hdr {
    pub words: [u64; 1],
// sender id
    pub sender: u16,
// receiver id
    pub receiver: u16,
// octep_ctrl_net_h2t_cmd
    pub cmd: u16,
// octep_ctrl_net_reply
    pub reply: u16,
    pub s: },
}

// get mtu response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp_cmd_mtu {
// 0-65535
    pub val: u16,
}

// get mac response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp_cmd_mac {
// xx:xx:xx:xx:xx:xx
    pub addr: [u8; ETH_ALEN],
}

// get if_stats, xstats, q_stats request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp_cmd_get_stats {
    pub rx_stats: octep_iface_rx_stats,
    pub tx_stats: octep_iface_tx_stats,
}

// get link state, rx state response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp_cmd_state {
// enum octep_ctrl_net_state
    pub state: u16,
}

// get info request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp_cmd_get_info {
    pub fw_info: octep_fw_info,
}

// Host to fw response data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_h2f_resp {
    pub hdr: octep_ctrl_net_resp_hdr,
    pub mtu: octep_ctrl_net_h2f_resp_cmd_mtu,
    pub mac: octep_ctrl_net_h2f_resp_cmd_mac,
    pub if_stats: octep_ctrl_net_h2f_resp_cmd_get_stats,
    pub link: octep_ctrl_net_h2f_resp_cmd_state,
    pub rx: octep_ctrl_net_h2f_resp_cmd_state,
    pub link_info: octep_ctrl_net_link_info,
    pub info: octep_ctrl_net_h2f_resp_cmd_get_info,
    pub offloads: octep_ctrl_net_offloads,
}

// link state notofication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_f2h_req_cmd_state {
// enum octep_ctrl_net_state
    pub state: u16,
}

// Fw to host request data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_f2h_req {
    pub hdr: octep_ctrl_net_req_hdr,
    pub link: octep_ctrl_net_f2h_req_cmd_state,
}

// Fw to host response data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_f2h_resp {
    pub hdr: octep_ctrl_net_resp_hdr,
}

// Max data size to be transferred over mbox
#[repr(C)]
#[derive(Copy, Clone)]
pub union octep_ctrl_net_max_data {
    pub h2f_req: octep_ctrl_net_h2f_req,
    pub h2f_resp: octep_ctrl_net_h2f_resp,
    pub f2h_req: octep_ctrl_net_f2h_req,
    pub f2h_resp: octep_ctrl_net_f2h_resp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct octep_ctrl_net_wait_data {
    pub list: list_head,
    pub done: c_int,
    pub msg: octep_ctrl_mbox_msg,
    pub req: octep_ctrl_net_h2f_req,
    pub resp: octep_ctrl_net_h2f_resp,
    pub data: },
}

//
// octep_ctrl_net_init() - Initialize data for ctrl net.
//
// @oct: non-null pointer to struct octep_device.
//
// return value: 0 on success, -errno on error.
//
extern "C" {
    pub fn octep_ctrl_net_init(oct: *mut octep_device) -> c_int;
}
//
// octep_ctrl_net_get_link_status() - Get link status from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
//
// return value: link status 0=down, 1=up.
//
extern "C" {
    pub fn octep_ctrl_net_get_link_status(oct: *mut octep_device, vfid: c_int) -> c_int;
}
//
// octep_ctrl_net_set_link_status() - Set link status in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @up: boolean status.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure
//
// octep_ctrl_net_set_rx_state() - Set rx state in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @up: boolean status.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_get_mac_addr() - Get mac address from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @addr: non-null pointer to mac address.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_net_get_mac_addr(oct: *mut octep_device, vfid: c_int, addr: *mut u8) -> c_int;
}
//
// octep_ctrl_net_set_mac_addr() - Set mac address in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @addr: non-null pointer to mac address.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_get_mtu() - Get max MTU from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
//
// return value: mtu on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_net_get_mtu(oct: *mut octep_device, vfid: c_int) -> c_int;
}
//
// octep_ctrl_net_set_mtu() - Set mtu in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @mtu: mtu.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_get_if_stats() - Get interface statistics from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @rx_stats: non-null pointer struct octep_iface_rx_stats.
// @tx_stats: non-null pointer struct octep_iface_tx_stats.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_get_link_info() - Get link info from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @link_info: non-null pointer to struct octep_iface_link_info.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_set_link_info() - Set link info in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @link_info: non-null pointer to struct octep_iface_link_info.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_recv_fw_messages() - Poll for firmware messages and process them.
//
// @oct: non-null pointer to struct octep_device.
//
extern "C" {
    pub fn octep_ctrl_net_recv_fw_messages(oct: *mut octep_device);
}
//
// octep_ctrl_net_get_info() - Get info from firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @info: non-null pointer to struct octep_fw_info.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_dev_remove() - Indicate to firmware that a device unload has happened.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
//
// return value: 0 on success, -errno on failure.
//
extern "C" {
    pub fn octep_ctrl_net_dev_remove(oct: *mut octep_device, vfid: c_int) -> c_int;
}
//
// octep_ctrl_net_set_offloads() - Set offloads in firmware.
//
// @oct: non-null pointer to struct octep_device.
// @vfid: Index of virtual function.
// @offloads: non-null pointer to struct octep_ctrl_net_offloads.
// @wait_for_response: poll for response.
//
// return value: 0 on success, -errno on failure.
//
// octep_ctrl_net_uninit() - Uninitialize data for ctrl net.
//
// @oct: non-null pointer to struct octep_device.
//
// return value: 0 on success, -errno on error.
//
extern "C" {
    pub fn octep_ctrl_net_uninit(oct: *mut octep_device) -> c_int;
}
