//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icssg_config.h
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
// Texas Instruments ICSSG Ethernet driver
//
// Copyright (C) 2022 Texas Instruments Incorporated - https://www.ti.com
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_buffer_pool_cfg {
    pub addr: __le32,
    pub len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_flow_cfg {
    pub rx_base_flow: __le16,
    pub mgm_base_flow: __le16,
    pub __packed: },
pub const PRUETH_PKT_TYPE_CMD: c_uint = 0x10;

pub const PRUETH_MAX_TX_DESC: c_int = 512;
pub const PRUETH_MAX_RX_DESC: c_int = 512;

pub const PRUETH_RX_FLOW_DATA: c_int = 0;
// Defines for forwarding path buffer pools:
// - used by firmware to store packets to be forwarded to other port
// - 8 total pools per slice
// - only used in switch mode (as no forwarding in mac mode)
//
pub const PRUETH_NUM_FWD_BUF_POOLS_PER_SLICE: c_int = 8;

// Defines for local injection path buffer pools:
// - used by firmware to store packets received from host core
// - 16 total pools per slice
// - 8 pools per port per slice and each slice handles both ports
// - only 4 out of 8 pools used per port (as only 4 real QoS levels in ICSSG)
// - switch mode: 8 total pools used
// - mac mode:    4 total pools used
//
pub const PRUETH_NUM_LI_BUF_POOLS_PER_SLICE: c_int = 16;
pub const PRUETH_NUM_LI_BUF_POOLS_PER_PORT_PER_SLICE: c_int = 8;

pub const PRUETH_SW_USED_LI_BUF_POOLS_PER_SLICE: c_int = 8;
pub const PRUETH_SW_USED_LI_BUF_POOLS_PER_PORT_PER_SLICE: c_int = 4;

pub const PRUETH_EMAC_USED_LI_BUF_POOLS_PER_SLICE: c_int = 4;
pub const PRUETH_EMAC_USED_LI_BUF_POOLS_PER_PORT_PER_SLICE: c_int = 4;
// Defines for host egress path - express and preemptible buffers
// - used by firmware to store express and preemptible packets
// to be transmitted to host core
// - used by both mac/switch modes
//

// Buffer used by firmware to temporarily store packet to be dropped

// Total switch mode memory usage for buffers per slice

// Total switch mode memory usage for all buffers

// Total mac mode memory usage for buffers per slice

// Total mac mode memory usage for all buffers

// Size of 1 bank of MSMC/OC_SRAM memory

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_rxq_ctx {
    pub start: [__le32; 3],
    pub end: __le32,
    pub __packed: },
// Load time Fiwmware Configuration
pub const ICSSG_FW_MGMT_CMD_HEADER: c_uint = 0x81;
pub const ICSSG_FW_MGMT_FDB_CMD_TYPE: c_uint = 0x03;
pub const ICSSG_FW_MGMT_CMD_TYPE: c_uint = 0x04;
pub const ICSSG_FW_MGMT_PKT: c_uint = 0x80000000;
pub const ICSSG_FW_MGMT_FDB_CMD_TYPE_RX_FLOW: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_r30_cmd {
    pub cmd: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icssg_port_state_cmd {
    ICSSG_EMAC_PORT_DISABLE = 0,
    ICSSG_EMAC_PORT_BLOCK,
    ICSSG_EMAC_PORT_FORWARD,
    ICSSG_EMAC_PORT_FORWARD_WO_LEARNING,
    ICSSG_EMAC_PORT_ACCEPT_ALL,
    ICSSG_EMAC_PORT_ACCEPT_TAGGED,
    ICSSG_EMAC_PORT_ACCEPT_UNTAGGED_N_PRIO,
    ICSSG_EMAC_PORT_TAS_TRIGGER,
    ICSSG_EMAC_PORT_TAS_ENABLE,
    ICSSG_EMAC_PORT_TAS_RESET,
    ICSSG_EMAC_PORT_TAS_DISABLE,
    ICSSG_EMAC_PORT_UC_FLOODING_ENABLE,
    ICSSG_EMAC_PORT_UC_FLOODING_DISABLE,
    ICSSG_EMAC_PORT_MC_FLOODING_ENABLE,
    ICSSG_EMAC_PORT_MC_FLOODING_DISABLE,
    ICSSG_EMAC_PORT_PREMPT_TX_ENABLE,
    ICSSG_EMAC_PORT_PREMPT_TX_DISABLE,
    ICSSG_EMAC_PORT_VLAN_AWARE_ENABLE,
    ICSSG_EMAC_PORT_VLAN_AWARE_DISABLE,
    ICSSG_EMAC_HSR_RX_OFFLOAD_ENABLE,
    ICSSG_EMAC_HSR_RX_OFFLOAD_DISABLE,
    ICSSG_EMAC_PORT_MAX_COMMANDS
}

pub const EMAC_NONE: c_uint = 0xffff0000;
pub const EMAC_PRU0_P_DI: c_uint = 0xffff0004;
pub const EMAC_PRU1_P_DI: c_uint = 0xffff0040;
pub const EMAC_TX_P_DI: c_uint = 0xffff0100;
pub const EMAC_PRU0_P_EN: c_uint = 0xfffb0000;
pub const EMAC_PRU1_P_EN: c_uint = 0xffbf0000;
pub const EMAC_TX_P_EN: c_uint = 0xfeff0000;
pub const EMAC_P_BLOCK: c_uint = 0xffff0040;
pub const EMAC_TX_P_BLOCK: c_uint = 0xffff0200;
pub const EMAC_P_UNBLOCK: c_uint = 0xffbf0000;
pub const EMAC_TX_P_UNBLOCK: c_uint = 0xfdff0000;
pub const EMAC_LEAN_EN: c_uint = 0xfff70000;
pub const EMAC_LEAN_DI: c_uint = 0xffff0008;
pub const EMAC_ACCEPT_ALL: c_uint = 0xffff0001;
pub const EMAC_ACCEPT_TAG: c_uint = 0xfffe0002;
pub const EMAC_ACCEPT_PRIOR: c_uint = 0xfffc0000;
// Config area lies in DRAM
pub const ICSSG_CONFIG_OFFSET: c_uint = 0x0;
// Config area lies in shared RAM
pub const ICSSG_CONFIG_OFFSET_SLICE0: c_int = 0;
pub const ICSSG_CONFIG_OFFSET_SLICE1: c_uint = 0x8000;
pub const ICSSG_NUM_NORMAL_PDS: c_int = 64;
pub const ICSSG_NUM_SPECIAL_PDS: c_int = 16;
pub const ICSSG_NORMAL_PD_SIZE: c_int = 8;
pub const ICSSG_SPECIAL_PD_SIZE: c_int = 20;
pub const ICSSG_FLAG_MASK: c_uint = 0xff00ffff;
// SR1.0-specific bits

pub const PRUETH_MAX_RX_MGM_DESC_SR1: c_int = 8;

pub const PRUETH_RX_MGM_FLOW_RESPONSE_SR1: c_int = 0;
pub const PRUETH_RX_MGM_FLOW_TIMESTAMP_SR1: c_int = 1;
pub const PRUETH_NUM_BUF_POOLS_SR1: c_int = 16;
pub const PRUETH_EMAC_BUF_POOL_START_SR1: c_int = 8;
pub const PRUETH_EMAC_BUF_POOL_MIN_SIZE_SR1: c_int = 128;
pub const PRUETH_EMAC_BUF_SIZE_SR1: c_int = 1536;
pub const PRUETH_EMAC_NUM_BUF_SR1: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_sr1_config {
    pub /: *mut *mut __le32 status; / Firmware status,
    pub /: *mut *mut __le32 addr_lo; / MSMC Buffer pool base address low.,
    pub /: *mut *mut __le32 addr_hi; / MSMC Buffer pool base address high. Must be 0,
    pub /: *mut *mut __le32 tx_buf_sz[16]; / Array of buffer pool sizes,
    pub /: *mut *mut __le32 num_tx_threads; / Number of active egress threads, 1 to 4,
    pub /: *mut *mut __le32 tx_rate_lim_en; / Bitmask: Egress rate limit en per thread,
    pub /: *mut *mut __le32 rx_flow_id; / RX flow id for first rx ring,
    pub /: *mut *mut __le32 rx_mgr_flow_id; / RX flow id for the first management ring,
    pub /: *mut *mut __le32 flags; / TBD,
    pub /: *mut *mut __le32 n_burst; / for debug,
    pub /: *mut *mut __le32 rtu_status; / RTU status,
    pub /: *mut *mut __le32 info; / reserved,
    pub reserve: __le32,
    pub /: *mut *mut __le32 rand_seed; / Used for the random number generation at fw,
    pub __packed: },
// SR1.0 shutdown command to stop processing at firmware.
// Command format: 0x8101ss00, where
// - ss: sequence number. Currently not used by driver.
//
pub const ICSSG_SHUTDOWN_CMD_SR1: c_uint = 0x81010000;
// SR1.0 pstate speed/duplex command to set speed and duplex settings
// in firmware.
// Command format: 0x8102ssPN, where
// - ss: sequence number. Currently not used by driver.
// - P: port number (for switch mode).
// - N: Speed/Duplex state:
// 0x0 - 10Mbps/Half duplex;
// 0x8 - 10Mbps/Full duplex;
// 0x2 - 100Mbps/Half duplex;
// 0xa - 100Mbps/Full duplex;
// 0xc - 1Gbps/Full duplex;
// NOTE: The above are the same value as bits [3..1](slice 0)
// or bits [7..5](slice 1) of RGMII CFG register.
//
pub const ICSSG_PSTATE_SPEED_DUPLEX_CMD_SR1: c_uint = 0x81020000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_setclock_desc {
    pub request: u8,
    pub restore: u8,
    pub acknowledgment: u8,
    pub cmp_status: u8,
    pub margin: u32,
    pub cyclecounter0_set: u32,
    pub cyclecounter1_set: u32,
    pub iepcount_set: u32,
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub CMP0_current: u32,
    pub iepcount_current: u32,
    pub difference: u32,
    pub cyclecounter0_new: u32,
    pub cyclecounter1_new: u32,
    pub CMP0_new: u32,
    pub __packed: },
pub const ICSSG_CMD_POP_SLICE0: c_int = 56;
pub const ICSSG_CMD_POP_SLICE1: c_int = 60;
pub const ICSSG_CMD_PUSH_SLICE0: c_int = 57;
pub const ICSSG_CMD_PUSH_SLICE1: c_int = 61;
pub const ICSSG_RSP_POP_SLICE0: c_int = 58;
pub const ICSSG_RSP_POP_SLICE1: c_int = 62;
pub const ICSSG_RSP_PUSH_SLICE0: c_int = 56;
pub const ICSSG_RSP_PUSH_SLICE1: c_int = 60;
pub const ICSSG_TS_POP_SLICE0: c_int = 59;
pub const ICSSG_TS_POP_SLICE1: c_int = 63;
pub const ICSSG_TS_PUSH_SLICE0: c_int = 40;
pub const ICSSG_TS_PUSH_SLICE1: c_int = 41;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cmd {
    pub param: u8,
    pub seqnum: u8,
    pub type: u8,
    pub header: u8,
    pub cmd_args: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mgmt_cmd_rsp {
    pub reserved: u32,
    pub status: u8,
    pub seqnum: u8,
    pub type: u8,
    pub header: u8,
    pub cmd_args: [u32; 3],
}

// FDB FID_C2 flag definitions
// Indicates host port membership.

// Indicates that MAC ID is connected to physical port 1

// Indicates that MAC ID is connected to physical port 2

// Ageable bit is set for learned entries and cleared for static entries

// If set for DA then packet is determined to be a special packet

// If set for DA then the SA from the packet is not learned

// If set, it means packet has been seen recently with source address + FID
// matching MAC address/FID of entry
//

// Set if entry is valid

//
// struct prueth_vlan_tbl - VLAN table entries struct in ICSSG SMEM
// @fid_c1: membership and forwarding rules flag to this table. See
// above to defines for bit definitions
// @fid: FDB index for this VID (there is 1-1 mapping b/w VID and FID)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_vlan_tbl {
    pub fid_c1: u8,
    pub fid: u8,
    pub __packed: },
//
// struct prueth_fdb_slot - Result of FDB slot lookup
// @mac: MAC address
// @fid: fid to be associated with MAC
// @fid_c2: FID_C2 entry for this MAC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_fdb_slot {
    pub mac: [u8; ETH_ALEN],
    pub fid: u8,
    pub fid_c2: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icssg_ietfpe_verify_states {
    ICSSG_IETFPE_STATE_UNKNOWN = 0,
    ICSSG_IETFPE_STATE_INITIAL,
    ICSSG_IETFPE_STATE_VERIFYING,
    ICSSG_IETFPE_STATE_SUCCEEDED,
    ICSSG_IETFPE_STATE_FAILED,
    ICSSG_IETFPE_STATE_DISABLED
}
