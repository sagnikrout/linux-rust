//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2x/bnx2x_mfw_req.h
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


// bnx2x_mfw_req.h: Qlogic Everest network driver.
//
// Copyright (c) 2012-2013 Broadcom Corporation
// Copyright (c) 2014 QLogic Corporation
// All rights reserved
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
pub const PORT_0: c_int = 0;
pub const PORT_1: c_int = 1;
pub const PORT_MAX: c_int = 2;
pub const NVM_PATH_MAX: c_int = 2;
// FCoE capabilities required from the driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_capabilities {
    pub capability1: u32,
// Maximum number of I/Os per connection
pub const FCOE_IOS_PER_CONNECTION_MASK: c_uint = 0x0000ffff;
pub const FCOE_IOS_PER_CONNECTION_SHIFT: c_int = 0;
// Maximum number of Logins per port
pub const FCOE_LOGINS_PER_PORT_MASK: c_uint = 0xffff0000;
pub const FCOE_LOGINS_PER_PORT_SHIFT: c_int = 16;
    pub capability2: u32,
// Maximum number of exchanges
pub const FCOE_NUMBER_OF_EXCHANGES_MASK: c_uint = 0x0000ffff;
pub const FCOE_NUMBER_OF_EXCHANGES_SHIFT: c_int = 0;
// Maximum NPIV WWN per port
pub const FCOE_NPIV_WWN_PER_PORT_MASK: c_uint = 0xffff0000;
pub const FCOE_NPIV_WWN_PER_PORT_SHIFT: c_int = 16;
    pub capability3: u32,
// Maximum number of targets supported
pub const FCOE_TARGETS_SUPPORTED_MASK: c_uint = 0x0000ffff;
pub const FCOE_TARGETS_SUPPORTED_SHIFT: c_int = 0;
// Maximum number of outstanding commands across all connections
pub const FCOE_OUTSTANDING_COMMANDS_MASK: c_uint = 0xffff0000;
pub const FCOE_OUTSTANDING_COMMANDS_SHIFT: c_int = 16;
    pub capability4: u32,
pub const FCOE_CAPABILITY4_STATEFUL: c_uint = 0x00000001;
pub const FCOE_CAPABILITY4_STATELESS: c_uint = 0x00000002;
pub const FCOE_CAPABILITY4_CAPABILITIES_REPORTED_VALID: c_uint = 0x00000004;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glob_ncsi_oem_data {
    pub driver_version: u32,
    pub unused: [u32; 3],
    pub fcoe_features: [fcoe_capabilities; NVM_PATH_MAX][PORT_MAX],
}

// current drv_info version
pub const DRV_INFO_CUR_VER: c_int = 2;
// drv_info op codes supported
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_info_opcode {
    ETH_STATS_OPCODE,
    FCOE_STATS_OPCODE,
    ISCSI_STATS_OPCODE
}

pub const ETH_STAT_INFO_VERSION_LEN: c_int = 12;
// Per PCI Function Ethernet Statistics required from the driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_stats_info {
// Function's Driver Version. padded to 12
    pub version: [u8; ETH_STAT_INFO_VERSION_LEN],
// Locally Admin Addr. BigEndian EIU48. Actual size is 6 bytes
    pub mac_local: [u8; 8],
    pub /: *mut *mut u8 mac_add1[8]; / Additional Programmed MAC Addr 1.,
    pub /: *mut *mut u8 mac_add2[8]; / Additional Programmed MAC Addr 2.,
    pub /: *mut *mut u32 mtu_size; / MTU Size. Note : Negotiated MTU,
    pub /: *mut *mut u32 feature_flags; / Feature_Flags.,
pub const FEATURE_ETH_CHKSUM_OFFLOAD_MASK: c_uint = 0x01;
pub const FEATURE_ETH_LSO_MASK: c_uint = 0x02;
pub const FEATURE_ETH_BOOTMODE_MASK: c_uint = 0x1C;
pub const FEATURE_ETH_BOOTMODE_SHIFT: c_int = 2;

pub const FEATURE_ETH_TOE_MASK: c_uint = 0x20;
    pub /: *mut *mut u32 lso_max_size; / LSO MaxOffloadSize.,
    pub /: *mut *mut u32 lso_min_seg_cnt; / LSO MinSegmentCount.,
// Num Offloaded Connections TCP_IPv4.
    pub ipv4_ofld_cnt: u32,
// Num Offloaded Connections TCP_IPv6.
    pub ipv6_ofld_cnt: u32,
    pub /: *mut *mut u32 promiscuous_mode; / Promiscuous Mode. non-zero true,
    pub /: *mut *mut u32 txq_size; / TX Descriptors Queue Size,
    pub /: *mut *mut u32 rxq_size; / RX Descriptors Queue Size,
// TX Descriptor Queue Avg Depth. % Avg Queue Depth since last poll
    pub txq_avg_depth: u32,
// RX Descriptors Queue Avg Depth. % Avg Queue Depth since last poll
    pub rxq_avg_depth: u32,
// IOV_Offload. 0=none; 1=MultiQueue, 2=VEB 3= VEPA
    pub iov_offload: u32,
// Number of NetQueue/VMQ Config'd.
    pub netq_cnt: u32,
    pub /: *mut *mut u32 vf_cnt; / Num VF assigned to this PF.,
}

// Per PCI Function FCOE Statistics required from the driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_stats_info {
    pub /: *mut *mut u8 version[12]; / Function's Driver Version.,
    pub /: *mut *mut u8 mac_local[8]; / Locally Admin Addr.,
    pub /: *mut *mut u8 mac_add1[8]; / Additional Programmed MAC Addr 1.,
    pub /: *mut *mut u8 mac_add2[8]; / Additional Programmed MAC Addr 2.,
// QoS Priority (per 802.1p). 0-7255
    pub qos_priority: u32,
    pub /: *mut *mut u32 txq_size; / FCoE TX Descriptors Queue Size.,
    pub /: *mut *mut u32 rxq_size; / FCoE RX Descriptors Queue Size.,
// FCoE TX Descriptor Queue Avg Depth.
    pub txq_avg_depth: u32,
// FCoE RX Descriptors Queue Avg Depth.
    pub rxq_avg_depth: u32,
    pub /: *mut *mut u32 rx_frames_lo; / FCoE RX Frames received.,
    pub /: *mut *mut u32 rx_frames_hi; / FCoE RX Frames received.,
    pub /: *mut *mut u32 rx_bytes_lo; / FCoE RX Bytes received.,
    pub /: *mut *mut u32 rx_bytes_hi; / FCoE RX Bytes received.,
    pub /: *mut *mut u32 tx_frames_lo; / FCoE TX Frames sent.,
    pub /: *mut *mut u32 tx_frames_hi; / FCoE TX Frames sent.,
    pub /: *mut *mut u32 tx_bytes_lo; / FCoE TX Bytes sent.,
    pub /: *mut *mut u32 tx_bytes_hi; / FCoE TX Bytes sent.,
}

// Per PCI  Function iSCSI Statistics required from the driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_stats_info {
    pub /: *mut *mut u8 version[12]; / Function's Driver Version.,
    pub /: *mut *mut u8 mac_local[8]; / Locally Admin iSCSI MAC Addr.,
    pub /: *mut *mut u8 mac_add1[8]; / Additional Programmed MAC Addr 1.,
// QoS Priority (per 802.1p). 0-7255
    pub qos_priority: u32,
    pub /: *mut *mut u8 initiator_name[64]; / iSCSI Boot Initiator Node name.,
    pub /: *mut *mut u8 ww_port_name[64]; / iSCSI World wide port name,
    pub /: *mut *mut u8 boot_target_name[64];/ iSCSI Boot Target Name.,
    pub /: *mut *mut u8 boot_target_ip[16]; / iSCSI Boot Target IP.,
    pub /: *mut *mut u32 boot_target_portal; / iSCSI Boot Target Portal.,
    pub /: *mut *mut u8 boot_init_ip[16]; / iSCSI Boot Initiator IP Address.,
    pub /: *mut *mut u32 max_frame_size; / Max Frame Size. bytes,
    pub /: *mut *mut u32 txq_size; / PDU TX Descriptors Queue Size.,
    pub /: *mut *mut u32 rxq_size; / PDU RX Descriptors Queue Size.,
    pub /: *mut *mut u32 txq_avg_depth; / PDU TX Descriptor Queue Avg Depth.,
    pub /: *mut *mut u32 rxq_avg_depth; / PDU RX Descriptors Queue Avg Depth.,
    pub /: *mut *mut u32 rx_pdus_lo; / iSCSI PDUs received.,
    pub /: *mut *mut u32 rx_pdus_hi; / iSCSI PDUs received.,
    pub /: *mut *mut u32 rx_bytes_lo; / iSCSI RX Bytes received.,
    pub /: *mut *mut u32 rx_bytes_hi; / iSCSI RX Bytes received.,
    pub /: *mut *mut u32 tx_pdus_lo; / iSCSI PDUs sent.,
    pub /: *mut *mut u32 tx_pdus_hi; / iSCSI PDUs sent.,
    pub /: *mut *mut u32 tx_bytes_lo; / iSCSI PDU TX Bytes sent.,
    pub /: *mut *mut u32 tx_bytes_hi; / iSCSI PDU TX Bytes sent.,
    pub MapTable.: *mut *mut u32 pcp_prior_map_tbl; / C-PCP to S-PCP Priority,
// 9 nibbles, the position of each nibble
// represents the C-PCP value, the value
// of the nibble = S-PCP value.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drv_info_to_mcp {
    pub ether_stat: eth_stats_info,
    pub fcoe_stat: fcoe_stats_info,
    pub iscsi_stat: iscsi_stats_info,
}
