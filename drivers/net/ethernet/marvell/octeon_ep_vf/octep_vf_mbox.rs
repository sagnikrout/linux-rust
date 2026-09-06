//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeon_ep_vf/octep_vf_mbox.h
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
// When a new command is implemented, VF Mbox version should be bumped.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_mbox_version {
    OCTEP_PFVF_MBOX_VERSION_V0,
    OCTEP_PFVF_MBOX_VERSION_V1,
    OCTEP_PFVF_MBOX_VERSION_V2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_mbox_opcode {
    OCTEP_PFVF_MBOX_CMD_VERSION,
    OCTEP_PFVF_MBOX_CMD_SET_MTU,
    OCTEP_PFVF_MBOX_CMD_SET_MAC_ADDR,
    OCTEP_PFVF_MBOX_CMD_GET_MAC_ADDR,
    OCTEP_PFVF_MBOX_CMD_GET_LINK_INFO,
    OCTEP_PFVF_MBOX_CMD_GET_STATS,
    OCTEP_PFVF_MBOX_CMD_SET_RX_STATE,
    OCTEP_PFVF_MBOX_CMD_SET_LINK_STATUS,
    OCTEP_PFVF_MBOX_CMD_GET_LINK_STATUS,
    OCTEP_PFVF_MBOX_CMD_GET_MTU,
    OCTEP_PFVF_MBOX_CMD_DEV_REMOVE,
    OCTEP_PFVF_MBOX_CMD_GET_FW_INFO,
    OCTEP_PFVF_MBOX_CMD_SET_OFFLOADS,
    OCTEP_PFVF_MBOX_NOTIF_LINK_STATUS,
    OCTEP_PFVF_MBOX_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_mbox_word_type {
    OCTEP_PFVF_MBOX_TYPE_CMD,
    OCTEP_PFVF_MBOX_TYPE_RSP_ACK,
    OCTEP_PFVF_MBOX_TYPE_RSP_NACK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_mbox_cmd_status {
    OCTEP_PFVF_MBOX_CMD_STATUS_NOT_SETUP = 1,
    OCTEP_PFVF_MBOX_CMD_STATUS_TIMEDOUT = 2,
    OCTEP_PFVF_MBOX_CMD_STATUS_NACK = 3,
    OCTEP_PFVF_MBOX_CMD_STATUS_BUSY = 4,
    OCTEP_PFVF_MBOX_CMD_STATUS_ERR = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_link_status {
    OCTEP_PFVF_LINK_STATUS_DOWN,
    OCTEP_PFVF_LINK_STATUS_UP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_link_speed {
    OCTEP_PFVF_LINK_SPEED_NONE,
    OCTEP_PFVF_LINK_SPEED_1000,
    OCTEP_PFVF_LINK_SPEED_10000,
    OCTEP_PFVF_LINK_SPEED_25000,
    OCTEP_PFVF_LINK_SPEED_40000,
    OCTEP_PFVF_LINK_SPEED_50000,
    OCTEP_PFVF_LINK_SPEED_100000,
    OCTEP_PFVF_LINK_SPEED_LAST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_link_duplex {
    OCTEP_PFVF_LINK_HALF_DUPLEX,
    OCTEP_PFVF_LINK_FULL_DUPLEX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octep_pfvf_link_autoneg {
    OCTEP_PFVF_LINK_AUTONEG,
    OCTEP_PFVF_LINK_FIXED,
}

pub const OCTEP_PFVF_MBOX_TIMEOUT_WAIT_COUNT: c_int = 8000;
pub const OCTEP_PFVF_MBOX_TIMEOUT_WAIT_UDELAY: c_int = 1000;
pub const OCTEP_PFVF_MBOX_MAX_RETRIES: c_int = 2;
pub const OCTEP_PFVF_MBOX_VERSION: c_int = 0;
pub const OCTEP_PFVF_MBOX_MAX_DATA_SIZE: c_int = 6;
pub const OCTEP_PFVF_MBOX_MAX_DATA_BUF_SIZE: c_int = 320;
pub const OCTEP_PFVF_MBOX_MORE_FRAG_FLAG: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub union octep_pfvf_mbox_word {
    pub u64: u64,
    pub opcode:8: u64,
    pub type:2: u64,
    pub rsvd:6: u64,
    pub data:48: u64,
    pub s: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub frag:1: u64,
    pub rsvd:5: u64,
    pub data: [u8; 6],
    pub s_data: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub rsvd:6: u64,
    pub version:48: u64,
    pub s_version: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub rsvd:6: u64,
    pub mac_addr: [u8; 6],
    pub s_set_mac: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub rsvd:6: u64,
    pub mtu:48: u64,
    pub s_set_mtu: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub state:1: u64,
    pub rsvd:53: u64,
    pub s_link_state: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub status:1: u64,
    pub rsvd:53: u64,
    pub s_link_status: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub pkind:8: u64,
    pub fsz:8: u64,
    pub rx_ol_flags:16: u64,
    pub tx_ol_flags:16: u64,
    pub rsvd:6: u64,
    pub s_fw_info: },
    pub opcode:8: u64,
    pub type:2: u64,
    pub rsvd:22: u64,
    pub rx_ol_flags:16: u64,
    pub tx_ol_flags:16: u64,
    pub s_offloads: },
    pub __packed: },
    pub oct): *mut int octep_vf_setup_mbox(struct octep_vf_device,
    pub oct): *mut void octep_vf_delete_mbox(struct octep_vf_device,
    pub rsp): *mut octep_pfvf_mbox_word,
    pub size): *mut *mut u8 data, int,
    pub mtu): *mut *mut int octep_vf_mbox_set_mtu(struct octep_vf_device oct, int,
    pub mac_addr): *mut *mut int octep_vf_mbox_set_mac_addr(struct octep_vf_device oct, char,
    pub mac_addr): *mut *mut int octep_vf_mbox_get_mac_addr(struct octep_vf_device oct, char,
    pub oct): *mut int octep_vf_mbox_version_check(struct octep_vf_device,
    pub state): *mut *mut int octep_vf_mbox_set_rx_state(struct octep_vf_device oct, bool,
    pub status): *mut *mut int octep_vf_mbox_set_link_status(struct octep_vf_device oct, bool,
    pub oper_up): *mut *mut int octep_vf_mbox_get_link_status(struct octep_vf_device oct, u8,
    pub oct): *mut int octep_vf_mbox_dev_remove(struct octep_vf_device,
    pub oct): *mut int octep_vf_mbox_get_fw_info(struct octep_vf_device,
    pub rx_offloads): *mut *mut int octep_vf_mbox_set_offloads(struct octep_vf_device oct, u16 tx_offloads, u16,
