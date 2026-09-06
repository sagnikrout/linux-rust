//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/liquidio/liquidio_common.h
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
// Author: Cavium, Inc.
//
// Contact: support@cavium.com
// Please include "LiquidIO" in the subject.
//
// Copyright (c) 2003-2016 Cavium, Inc.
//
// This file is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License, Version 2, as
// published by the Free Software Foundation.
//
// This file is distributed in the hope that it will be useful, but
// AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
// NONINFRINGEMENT.  See the GNU General Public License for more details.
//
// !  \file  liquidio_common.h
// \brief Common: Structures and macros used in PCI-NIC package by core and
// host driver.
//

pub const LIQUIDIO_BASE_MAJOR_VERSION: c_int = 1;
pub const LIQUIDIO_BASE_MINOR_VERSION: c_int = 7;
pub const LIQUIDIO_BASE_MICRO_VERSION: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_version {
    pub major: u16,
    pub minor: u16,
    pub micro: u16,
    pub reserved: u16,
}

pub const CONTROL_IQ: c_int = 0;
// Tag types used by Octeon cores in its work.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octeon_tag_type {
    ORDERED_TAG = 0,
    ATOMIC_TAG = 1,
    NULL_TAG = 2,
    NULL_NULL_TAG = 3
}

// pre-defined host->NIC tag values

// Opcodes used by host driver/apps to perform operations on the core.
// These are used to identify the major subsystem that the operation
// is for.
//

// Subcodes are used by host driver/apps to identify the sub-operation
// for the core. They only need to by unique for a given subsystem.
//

// OPCODE_CORE subcodes. For future use.
// OPCODE_NIC subcodes
// This subcode is sent by core PCI driver to indicate cores are ready.
pub const OPCODE_NIC_CORE_DRV_ACTIVE: c_uint = 0x01;
pub const OPCODE_NIC_NW_DATA: c_uint = 0x02     /* network packet data */;
pub const OPCODE_NIC_CMD: c_uint = 0x03;
pub const OPCODE_NIC_INFO: c_uint = 0x04;
pub const OPCODE_NIC_PORT_STATS: c_uint = 0x05;
pub const OPCODE_NIC_MDIO45: c_uint = 0x06;
pub const OPCODE_NIC_TIMESTAMP: c_uint = 0x07;
pub const OPCODE_NIC_INTRMOD_CFG: c_uint = 0x08;
pub const OPCODE_NIC_IF_CFG: c_uint = 0x09;
pub const OPCODE_NIC_VF_DRV_NOTICE: c_uint = 0x0A;
pub const OPCODE_NIC_INTRMOD_PARAMS: c_uint = 0x0B;
pub const OPCODE_NIC_QCOUNT_UPDATE: c_uint = 0x12;
pub const OPCODE_NIC_SET_TRUSTED_VF: c_uint = 0x13;
pub const OPCODE_NIC_SYNC_OCTEON_TIME: c_uint = 0x14;
pub const VF_DRV_LOADED: c_int = 1;

pub const VF_DRV_MACADDR_CHANGED: c_int = 2;
pub const OPCODE_NIC_VF_REP_PKT: c_uint = 0x15;
pub const OPCODE_NIC_VF_REP_CMD: c_uint = 0x16;
pub const OPCODE_NIC_UBOOT_CTL: c_uint = 0x17;
pub const CORE_DRV_TEST_SCATTER_OP: c_uint = 0xFFF5;
// Application codes advertised by the core driver initialization packet.
pub const CVM_DRV_APP_START: c_uint = 0x0;
pub const CVM_DRV_NO_APP: c_int = 0;
pub const CVM_DRV_APP_COUNT: c_uint = 0x2;

pub const BYTES_PER_DHLEN_UNIT: c_int = 8;

pub const INTRNAMSIZ: c_int = 32;

pub const SCR2_BIT_FW_LOADED: c_int = 63;
// App specific capabilities from firmware to pf driver
pub const LIQUIDIO_TIME_SYNC_CAP: c_uint = 0x1;
pub const LIQUIDIO_SWITCHDEV_CAP: c_uint = 0x2;
pub const LIQUIDIO_SPOOFCHK_CAP: c_uint = 0x4;
// error status return from firmware
pub const OCTEON_REQUEST_NO_PERMISSION: c_uint = 0xc;
pub const OCT_BOARD_NAME: c_int = 32;
pub const OCT_SERIAL_LEN: c_int = 64;
// Structure used by core driver to send indication that the Octeon
// application is ready.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_core_setup {
    pub corefreq: u64,
    pub boardname: [c_char; OCT_BOARD_NAME],
    pub board_serial_number: [c_char; OCT_SERIAL_LEN],
    pub board_rev_major: u64,
    pub board_rev_minor: u64,
}

// ---------------------------  SCATTER GATHER ENTRY  -----------------------
// The Scatter-Gather List Entry. The scatter or gather component used with
// a Octeon input instruction has this format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_sg_entry {
// The first 64 bit gives the size of data in each dptr.
    pub size: [u16; 4],
    pub size64: u64,
    pub u: },
// The 4 dptr pointers for this entry.
    pub ptr: [u64; 4],
}

// \brief Add size to gather list
// @param sg_entry scatter/gather entry
// @param size size to add
// @param pos position to add it.
//

// ------------------------- End Scatter/Gather ---------------------------
pub const OCTNET_FRM_LENGTH_SIZE: c_int = 8;
pub const OCTNET_FRM_PTP_HEADER_SIZE: c_int = 8;

pub const OCTNET_MIN_FRM_SIZE: c_int = 64;

// NIC Commands are sent using this Octeon Input Queue
pub const OCTNET_CMD_Q: c_int = 0;
// NIC Command types
pub const OCTNET_CMD_CHANGE_MTU: c_uint = 0x1;
pub const OCTNET_CMD_CHANGE_MACADDR: c_uint = 0x2;
pub const OCTNET_CMD_CHANGE_DEVFLAGS: c_uint = 0x3;
pub const OCTNET_CMD_RX_CTL: c_uint = 0x4;
pub const OCTNET_CMD_SET_MULTI_LIST: c_uint = 0x5;
pub const OCTNET_CMD_CLEAR_STATS: c_uint = 0x6;
// command for setting the speed, duplex & autoneg
pub const OCTNET_CMD_SET_SETTINGS: c_uint = 0x7;
pub const OCTNET_CMD_SET_FLOW_CTL: c_uint = 0x8;
pub const OCTNET_CMD_MDIO_READ_WRITE: c_uint = 0x9;
pub const OCTNET_CMD_GPIO_ACCESS: c_uint = 0xA;
pub const OCTNET_CMD_LRO_ENABLE: c_uint = 0xB;
pub const OCTNET_CMD_LRO_DISABLE: c_uint = 0xC;
pub const OCTNET_CMD_SET_RSS: c_uint = 0xD;
pub const OCTNET_CMD_WRITE_SA: c_uint = 0xE;
pub const OCTNET_CMD_DELETE_SA: c_uint = 0xF;
pub const OCTNET_CMD_UPDATE_SA: c_uint = 0x12;
pub const OCTNET_CMD_TNL_RX_CSUM_CTL: c_uint = 0x10;
pub const OCTNET_CMD_TNL_TX_CSUM_CTL: c_uint = 0x11;
pub const OCTNET_CMD_IPSECV2_AH_ESP_CTL: c_uint = 0x13;
pub const OCTNET_CMD_VERBOSE_ENABLE: c_uint = 0x14;
pub const OCTNET_CMD_VERBOSE_DISABLE: c_uint = 0x15;
pub const OCTNET_CMD_VLAN_FILTER_CTL: c_uint = 0x16;
pub const OCTNET_CMD_ADD_VLAN_FILTER: c_uint = 0x17;
pub const OCTNET_CMD_DEL_VLAN_FILTER: c_uint = 0x18;
pub const OCTNET_CMD_VXLAN_PORT_CONFIG: c_uint = 0x19;
pub const OCTNET_CMD_ID_ACTIVE: c_uint = 0x1a;
pub const OCTNET_CMD_SET_UC_LIST: c_uint = 0x1b;
pub const OCTNET_CMD_SET_VF_LINKSTATE: c_uint = 0x1c;
pub const OCTNET_CMD_QUEUE_COUNT_CTL: c_uint = 0x1f;
pub const OCTNET_CMD_GROUP1: c_int = 1;
pub const OCTNET_CMD_SET_VF_SPOOFCHK: c_uint = 0x1;

pub const OCTNET_CMD_VXLAN_PORT_ADD: c_uint = 0x0;
pub const OCTNET_CMD_VXLAN_PORT_DEL: c_uint = 0x1;
pub const OCTNET_CMD_RXCSUM_ENABLE: c_uint = 0x0;
pub const OCTNET_CMD_RXCSUM_DISABLE: c_uint = 0x1;
pub const OCTNET_CMD_TXCSUM_ENABLE: c_uint = 0x0;
pub const OCTNET_CMD_TXCSUM_DISABLE: c_uint = 0x1;
pub const OCTNET_CMD_VLAN_FILTER_ENABLE: c_uint = 0x1;
pub const OCTNET_CMD_VLAN_FILTER_DISABLE: c_uint = 0x0;
pub const OCTNET_CMD_FAIL: c_uint = 0x1;
pub const SEAPI_CMD_FEC_SET: c_uint = 0x0;
pub const SEAPI_CMD_FEC_SET_DISABLE: c_uint = 0x0;
pub const SEAPI_CMD_FEC_SET_RS: c_uint = 0x1;
pub const SEAPI_CMD_FEC_GET: c_uint = 0x1;
pub const SEAPI_CMD_SPEED_SET: c_uint = 0x2;
pub const SEAPI_CMD_SPEED_GET: c_uint = 0x3;
pub const OPCODE_NIC_VF_PORT_STATS: c_uint = 0x22;
pub const LIO_CMD_WAIT_TM: c_int = 100;
// RX(packets coming from wire) Checksum verification flags
// TCP/UDP csum
pub const CNNIC_L4SUM_VERIFIED: c_uint = 0x1;
pub const CNNIC_IPSUM_VERIFIED: c_uint = 0x2;
pub const CNNIC_TUN_CSUM_VERIFIED: c_uint = 0x4;

// LROIPV4 and LROIPV6 Flags
pub const OCTNIC_LROIPV4: c_uint = 0x1;
pub const OCTNIC_LROIPV6: c_uint = 0x2;
// Interface flags communicated between host driver and core app.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum octnet_ifflags {
    OCTNET_IFFLAG_PROMISC   = 0x01,
    OCTNET_IFFLAG_ALLMULTI  = 0x02,
    OCTNET_IFFLAG_MULTICAST = 0x04,
    OCTNET_IFFLAG_BROADCAST = 0x08,
    OCTNET_IFFLAG_UNICAST   = 0x10
}

// wqe
// ---------------  0
// |  wqe  word0-3 |
// ---------------  32
// |    PCI IH     |
// ---------------  40
// |     RPTR      |
// ---------------  48
// |    PCI IRH    |
// ---------------  56
// |  OCT_NET_CMD  |
// ---------------  64
// | Addtl 8-BData |
// |               |
// ---------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union octnet_cmd {
    pub u64: u64,

    pub cmd:5: u64,
    pub /: *mut *mut u64 more:6; / How many udd words follow the command,
    pub cmdgroup:8: u64,
    pub reserved:21: u64,
    pub param1:16: u64,
    pub param2:8: u64,

    pub param2:8: u64,
    pub param1:16: u64,
    pub reserved:21: u64,
    pub cmdgroup:8: u64,
    pub more:6: u64,
    pub cmd:5: u64,

    pub s: },
}

// pkiih3 + irh + ossp[0] + ossp[1] + rdp + rptr = 40 bytes
pub const LIO_SOFTCMDRESP_IH2: c_int = 40;

pub const LIO_PCICMD_O2: c_int = 24;

// Instruction Header(DPI) - for OCTEON-III models
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_ih3 {

// Reserved3
    pub reserved3:1: u64,
// Gather indicator 1=gather
    pub gather:1: u64,
// Data length OR no. of entries in gather list
    pub dlengsz:14: u64,
// Front Data size
    pub fsz:6: u64,
// Reserved2
    pub reserved2:4: u64,
// PKI port kind - PKIND
    pub pkind:6: u64,
// Reserved1
    pub reserved1:32: u64,

// Reserved1
    pub reserved1:32: u64,
// PKI port kind - PKIND
    pub pkind:6: u64,
// Reserved2
    pub reserved2:4: u64,
// Front Data size
    pub fsz:6: u64,
// Data length OR no. of entries in gather list
    pub dlengsz:14: u64,
// Gather indicator 1=gather
    pub gather:1: u64,
// Reserved3
    pub reserved3:1: u64,

}

// Optional PKI Instruction Header(PKI IH) - for OCTEON-III models
// BIG ENDIAN format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_pki_ih3 {

// Wider bit
    pub w:1: u64,
// Raw mode indicator 1 = RAW
    pub raw:1: u64,
// Use Tag
    pub utag:1: u64,
// Use QPG
    pub uqpg:1: u64,
// Reserved2
    pub reserved2:1: u64,
// Parse Mode
    pub pm:3: u64,
// Skip Length
    pub sl:8: u64,
// Use Tag Type
    pub utt:1: u64,
// Tag type
    pub tagtype:2: u64,
// Reserved1
    pub reserved1:2: u64,
// QPG Value
    pub qpg:11: u64,
// Tag Value
    pub tag:32: u64,

// Tag Value
    pub tag:32: u64,
// QPG Value
    pub qpg:11: u64,
// Reserved1
    pub reserved1:2: u64,
// Tag type
    pub tagtype:2: u64,
// Use Tag Type
    pub utt:1: u64,
// Skip Length
    pub sl:8: u64,
// Parse Mode
    pub pm:3: u64,
// Reserved2
    pub reserved2:1: u64,
// Use QPG
    pub uqpg:1: u64,
// Use Tag
    pub utag:1: u64,
// Raw mode indicator 1 = RAW
    pub raw:1: u64,
// Wider bit
    pub w:1: u64,

}

// Instruction Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_ih2 {

// Raw mode indicator 1 = RAW
    pub raw:1: u64,
// Gather indicator 1=gather
    pub gather:1: u64,
// Data length OR no. of entries in gather list
    pub dlengsz:14: u64,
// Front Data size
    pub fsz:6: u64,
// Packet Order / Work Unit selection (1 of 8)
    pub qos:3: u64,
// Core group selection (1 of 16)
    pub grp:4: u64,
// Short Raw Packet Indicator 1=short raw pkt
    pub rs:1: u64,
// Tag type
    pub tagtype:2: u64,
// Tag Value
    pub tag:32: u64,

// Tag Value
    pub tag:32: u64,
// Tag type
    pub tagtype:2: u64,
// Short Raw Packet Indicator 1=short raw pkt
    pub rs:1: u64,
// Core group selection (1 of 16)
    pub grp:4: u64,
// Packet Order / Work Unit selection (1 of 8)
    pub qos:3: u64,
// Front Data size
    pub fsz:6: u64,
// Data length OR no. of entries in gather list
    pub dlengsz:14: u64,
// Gather indicator 1=gather
    pub gather:1: u64,
// Raw mode indicator 1 = RAW
    pub raw:1: u64,

}

// Input Request Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_irh {

    pub opcode:4: u64,
    pub rflag:1: u64,
    pub subcode:7: u64,
    pub vlan:12: u64,
    pub priority:3: u64,
    pub reserved:5: u64,
    pub /: *mut *mut u64 ossp:32; / opcode/subcode specific parameters,

    pub /: *mut *mut u64 ossp:32; / opcode/subcode specific parameters,
    pub reserved:5: u64,
    pub priority:3: u64,
    pub vlan:12: u64,
    pub subcode:7: u64,
    pub rflag:1: u64,
    pub opcode:4: u64,

}

// Return Data Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_instr_rdp {

    pub reserved:49: u64,
    pub pcie_port:3: u64,
    pub rlen:12: u64,

    pub rlen:12: u64,
    pub pcie_port:3: u64,
    pub reserved:49: u64,

}

// Receive Header
#[repr(C)]
#[derive(Copy, Clone)]
pub union octeon_rh {

    pub u64: u64,
    pub opcode:4: u64,
    pub subcode:8: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub reserved:17: u64,
    pub /: *mut *mut *mut u64 ossp:32; / opcode/subcode specific parameters,
    pub r: },
    pub opcode:4: u64,
    pub subcode:8: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub extra:28: u64,
    pub vlan:12: u64,
    pub priority:3: u64,
    pub /: *mut *mut *mut u64 csum_verified:3; / checksum verified.,
    pub /: *mut *mut *mut u64 has_hwtstamp:1; / Has hardware timestamp. 1 = yes.,
    pub encap_on:1: u64,
    pub /: *mut *mut *mut u64 has_hash:1; / Has hash (rth or rss). 1 = yes.,
    pub r_dh: },
    pub opcode:4: u64,
    pub subcode:8: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub reserved:11: u64,
    pub num_gmx_ports:8: u64,
    pub max_nic_ports:10: u64,
    pub app_cap_flags:4: u64,
    pub app_mode:8: u64,
    pub pkind:8: u64,
    pub r_core_drv_init: },
    pub opcode:4: u64,
    pub subcode:8: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub reserved:8: u64,
    pub extra:25: u64,
    pub gmxport:16: u64,
    pub r_nic_info: },

    pub u64: u64,
    pub /: *mut *mut *mut u64 ossp:32; / opcode/subcode specific parameters,
    pub reserved:17: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub subcode:8: u64,
    pub opcode:4: u64,
    pub r: },
    pub /: *mut *mut *mut u64 has_hash:1; / Has hash (rth or rss). 1 = yes.,
    pub encap_on:1: u64,
    pub /: *mut *mut *mut u64 has_hwtstamp:1; / 1 = has hwtstamp,
    pub /: *mut *mut *mut u64 csum_verified:3; / checksum verified.,
    pub priority:3: u64,
    pub vlan:12: u64,
    pub extra:28: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub subcode:8: u64,
    pub opcode:4: u64,
    pub r_dh: },
    pub pkind:8: u64,
    pub app_mode:8: u64,
    pub app_cap_flags:4: u64,
    pub max_nic_ports:10: u64,
    pub num_gmx_ports:8: u64,
    pub reserved:11: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub subcode:8: u64,
    pub opcode:4: u64,
    pub r_core_drv_init: },
    pub gmxport:16: u64,
    pub extra:25: u64,
    pub reserved:8: u64,
    pub /: *mut *mut *mut u64 len:3; / additional 64-bit words,
    pub subcode:8: u64,
    pub opcode:4: u64,
    pub r_nic_info: },

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union octnic_packet_params {
    pub u32: u32,

    pub reserved:24: u32,
    pub /: *mut *mut u32 ip_csum:1; / Perform IP header checksum(s),
// Perform Outer transport header checksum
    pub transport_csum:1: u32,
// Find tunnel, and perform transport csum.
    pub tnl_csum:1: u32,
    pub /: *mut *mut u32 tsflag:1; / Timestamp this packet,
    pub /: *mut *mut u32 ipsec_ops:4; / IPsec operation,

    pub ipsec_ops:4: u32,
    pub tsflag:1: u32,
    pub tnl_csum:1: u32,
    pub transport_csum:1: u32,
    pub ip_csum:1: u32,
    pub reserved:24: u32,

    pub s: },
}

// Status of a RGMII Link on Octeon as seen by core driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub union oct_link_status {
    pub u64: u64,

    pub duplex:8: u64,
    pub mtu:16: u64,
    pub speed:16: u64,
    pub link_up:1: u64,
    pub autoneg:1: u64,
    pub if_mode:5: u64,
    pub pause:1: u64,
    pub flashing:1: u64,
    pub phy_type:5: u64,
    pub reserved:10: u64,

    pub reserved:10: u64,
    pub phy_type:5: u64,
    pub flashing:1: u64,
    pub pause:1: u64,
    pub if_mode:5: u64,
    pub autoneg:1: u64,
    pub link_up:1: u64,
    pub speed:16: u64,
    pub mtu:16: u64,
    pub duplex:8: u64,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lio_phy_type {
    LIO_PHY_PORT_TP = 0x0,
    LIO_PHY_PORT_FIBRE = 0x1,
    LIO_PHY_PORT_UNKNOWN,
}

// The txpciq info passed to host from the firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub union oct_txpciq {
    pub u64: u64,

    pub q_no:8: u64,
    pub port:8: u64,
    pub pkind:6: u64,
    pub use_qpg:1: u64,
    pub qpg:11: u64,
    pub reserved0:10: u64,
    pub ctrl_qpg:11: u64,
    pub reserved:9: u64,

    pub reserved:9: u64,
    pub ctrl_qpg:11: u64,
    pub reserved0:10: u64,
    pub qpg:11: u64,
    pub use_qpg:1: u64,
    pub pkind:6: u64,
    pub port:8: u64,
    pub q_no:8: u64,

    pub s: },
}

// The rxpciq info passed to host from the firmware
#[repr(C)]
#[derive(Copy, Clone)]
pub union oct_rxpciq {
    pub u64: u64,

    pub q_no:8: u64,
    pub reserved:56: u64,

    pub reserved:56: u64,
    pub q_no:8: u64,

    pub s: },
}

// Information for a OCTEON ethernet interface shared between core & host.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_link_info {
    pub link: oct_link_status,
    pub hw_addr: u64,

    pub gmxport:16: u64,
    pub macaddr_is_admin_asgnd:1: u64,
    pub rsvd:13: u64,
    pub macaddr_spoofchk:1: u64,
    pub rsvd1:17: u64,
    pub num_txpciq:8: u64,
    pub num_rxpciq:8: u64,

    pub num_rxpciq:8: u64,
    pub num_txpciq:8: u64,
    pub rsvd1:17: u64,
    pub macaddr_spoofchk:1: u64,
    pub rsvd:13: u64,
    pub macaddr_is_admin_asgnd:1: u64,
    pub gmxport:16: u64,
    pub txpciq: [oct_txpciq; MAX_IOQS_PER_NICIF],
    pub rxpciq: [oct_rxpciq; MAX_IOQS_PER_NICIF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct liquidio_if_cfg_info {
    pub /: *mut *mut *mut u64 iqmask; / mask for IQs enabled for the port,
    pub /: *mut *mut *mut u64 oqmask; / mask for OQs enabled for the port,
    pub /: *mut *mut *mut oct_link_info linfo; / initial link information,
    pub liquidio_firmware_version: [c_char; 32],
}

// Stats for each NIC port in RX direction.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic_rx_stats {
// link-level stats
    pub /: *mut *mut u64 total_rcvd; / Received packets,
    pub /: *mut *mut u64 bytes_rcvd; / Octets of received packets,
    pub /: *mut *mut u64 total_bcst; / Number of non-dropped L2 broadcast packets,
    pub /: *mut *mut u64 total_mcst; / Number of non-dropped L2 multicast packets,
    pub /: *mut *mut u64 runts; / Packets shorter than allowed,
    pub /: *mut *mut u64 ctl_rcvd; / Received PAUSE packets,
    pub /: *mut *mut u64 fifo_err; / Packets dropped due to RX FIFO full,
    pub /: *mut *mut u64 dmac_drop; / Packets dropped by the DMAC filter,
    pub /: *mut *mut u64 fcs_err; / Sum of fragment, overrun, and FCS errors,
    pub /: *mut *mut u64 jabber_err; / Packets larger than allowed,
    pub memory,: *mut *mut u64 l2_err; / Sum of DMA, parity, PCAM access, no,
// buffer overflow, malformed L2 header or
// length, oversize errors
//
    pub /: *mut *mut u64 frame_err; / Sum of IPv4 and L4 checksum errors,
    pub buffer: *mut *mut u64 red_drops; / Packets dropped by RED due to,
// exhaustion
//
// firmware stats
    pub fw_total_rcvd: u64,
    pub fw_total_fwd: u64,
    pub fw_total_fwd_bytes: u64,
    pub fw_total_mcast: u64,
    pub fw_total_bcast: u64,
    pub fw_err_pko: u64,
    pub fw_err_link: u64,
    pub fw_err_drop: u64,
    pub fw_rx_vxlan: u64,
    pub fw_rx_vxlan_err: u64,
// LRO
    pub /: *mut *mut u64 fw_lro_pkts; / Number of packets that are LROed,
    pub /: *mut *mut u64 fw_lro_octs; / Number of octets that are LROed,
    pub /: *mut *mut u64 fw_total_lro; / Number of LRO packets formed,
    pub /: *mut *mut u64 fw_lro_aborts; / Number of times LRO of packet aborted,
    pub fw_lro_aborts_port: u64,
    pub fw_lro_aborts_seq: u64,
    pub fw_lro_aborts_tsval: u64,
    pub /: *mut *mut u64 fw_lro_aborts_timer; / Timer setting error,
// intrmod: packet forward rate
    pub fwd_rate: u64,
}

// Stats for each NIC port in RX direction.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nic_tx_stats {
// link-level stats
    pub /: *mut *mut u64 total_pkts_sent; / Total frames sent on the interface,
    pub /: *mut *mut u64 total_bytes_sent; / Total octets sent on the interface,
    pub /: *mut *mut u64 mcast_pkts_sent; / Packets sent to the multicast DMAC,
    pub /: *mut *mut u64 bcast_pkts_sent; / Packets sent to a broadcast DMAC,
    pub /: *mut *mut u64 ctl_sent; / Control/PAUSE packets sent,
    pub a: *mut *mut u64 one_collision_sent; / Packets sent that experienced,
// single collision before successful
// transmission
//
    pub experienced: *mut *mut u64 multi_collision_sent; / Packets sent that,
// multiple collisions before successful
// transmission
//
    pub excessive: *mut *mut u64 max_collision_fail; / Packets dropped due to,
// collisions
//
    pub max: *mut *mut u64 max_deferral_fail; / Packets not sent due to,
// deferrals
//
    pub a: *mut *mut u64 fifo_err; / Packets sent that experienced,
// transmit underflow and were
// truncated
//
    pub count: *mut *mut u64 runts; / Packets sent with an octet,
// lessthan 64
//
    pub excessive: *mut *mut u64 total_collisions; / Packets dropped due to,
// collisions
//
// firmware stats
    pub fw_total_sent: u64,
    pub fw_total_fwd: u64,
    pub fw_total_fwd_bytes: u64,
    pub fw_total_mcast_sent: u64,
    pub fw_total_bcast_sent: u64,
    pub fw_err_pko: u64,
    pub fw_err_link: u64,
    pub fw_err_drop: u64,
    pub fw_err_tso: u64,
    pub /: *mut *mut u64 fw_tso; / number of tso requests,
    pub /: *mut *mut u64 fw_tso_fwd; / number of packets segmented in tso,
    pub fw_tx_vxlan: u64,
    pub fw_err_pki: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_link_stats {
    pub fromwire: nic_rx_stats,
    pub fromhost: nic_tx_stats,
}

pub const LIO68XX_LED_CTRL_ADDR: c_uint = 0x3501;
pub const LIO68XX_LED_CTRL_CFGON: c_uint = 0x1f;
pub const LIO68XX_LED_CTRL_CFGOFF: c_uint = 0x100;
pub const LIO68XX_LED_BEACON_ADDR: c_uint = 0x3508;
pub const LIO68XX_LED_BEACON_CFGON: c_uint = 0x47fd;
pub const LIO68XX_LED_BEACON_CFGOFF: c_uint = 0x11fc;
pub const VITESSE_PHY_GPIO_DRIVEON: c_uint = 0x1;
pub const VITESSE_PHY_GPIO_CFG: c_uint = 0x8;
pub const VITESSE_PHY_GPIO_DRIVEOFF: c_uint = 0x4;
pub const VITESSE_PHY_GPIO_HIGH: c_uint = 0x2;
pub const VITESSE_PHY_GPIO_LOW: c_uint = 0x3;
pub const LED_IDENTIFICATION_ON: c_uint = 0x1;
pub const LED_IDENTIFICATION_OFF: c_uint = 0x0;
pub const LIO23XX_COPPERHEAD_LED_GPIO: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_mdio_cmd {
    pub op: u64,
    pub mdio_addr: u64,
    pub value1: u64,
    pub value2: u64,
    pub value3: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oct_intrmod_cfg {
    pub rx_enable: u64,
    pub tx_enable: u64,
    pub check_intrvl: u64,
    pub maxpkt_ratethr: u64,
    pub minpkt_ratethr: u64,
    pub rx_maxcnt_trigger: u64,
    pub rx_mincnt_trigger: u64,
    pub rx_maxtmr_trigger: u64,
    pub rx_mintmr_trigger: u64,
    pub tx_mincnt_trigger: u64,
    pub tx_maxcnt_trigger: u64,
    pub rx_frames: u64,
    pub tx_frames: u64,
    pub rx_usecs: u64,
}

pub const BASE_QUEUE_NOT_REQUESTED: c_int = 65535;
#[repr(C)]
#[derive(Copy, Clone)]
pub union oct_nic_if_cfg {
    pub u64: u64,

    pub base_queue:16: u64,
    pub num_iqueues:16: u64,
    pub num_oqueues:16: u64,
    pub gmx_port_id:8: u64,
    pub vf_id:8: u64,

    pub vf_id:8: u64,
    pub gmx_port_id:8: u64,
    pub num_oqueues:16: u64,
    pub num_iqueues:16: u64,
    pub base_queue:16: u64,

    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_trusted_vf {
    pub 1: uint64_t active:,
    pub 8: uint64_t id :,
    pub 55: uint64_t reserved:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_time {
    pub /: *mut *mut s64 sec; / seconds,
    pub /: *mut *mut s64 nsec; / nanoseconds,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_stats {
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub tx_dropped: u64,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lio_vf_rep_req_type {
    LIO_VF_REP_REQ_NONE,
    LIO_VF_REP_REQ_STATE,
    LIO_VF_REP_REQ_MTU,
    LIO_VF_REP_REQ_STATS,
    LIO_VF_REP_REQ_DEVNAME
}

pub const LIO_IF_NAME_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_req {
    pub req_type: u8,
    pub ifidx: u8,
    pub rsvd: [u8; 6],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_name {
    pub name: [c_char; LIO_IF_NAME_SIZE],
    pub rep_name: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_mtu {
    pub mtu: u32,
    pub rsvd: u32,
    pub rep_mtu: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_state {
    pub state: u8,
    pub rsvd: [u8; 7],
    pub rep_state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lio_vf_rep_resp {
    pub rh: u64,
    pub status: u8,
    pub rsvd: [u8; 7],
}
