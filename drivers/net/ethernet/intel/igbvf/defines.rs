//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igbvf/defines.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.
// Number of Transmit and Receive Descriptors must be a multiple of 8
pub const REQ_TX_DESCRIPTOR_MULTIPLE: c_int = 8;
pub const REQ_RX_DESCRIPTOR_MULTIPLE: c_int = 8;
// IVAR valid bit
pub const E1000_IVAR_VALID: c_uint = 0x80;
// Receive Descriptor bit definitions
pub const E1000_RXD_STAT_DD: c_uint = 0x01    /* Descriptor Done */;
pub const E1000_RXD_STAT_EOP: c_uint = 0x02    /* End of Packet */;
pub const E1000_RXD_STAT_IXSM: c_uint = 0x04    /* Ignore checksum */;
pub const E1000_RXD_STAT_VP: c_uint = 0x08    /* IEEE VLAN Packet */;
pub const E1000_RXD_STAT_UDPCS: c_uint = 0x10    /* UDP xsum calculated */;
pub const E1000_RXD_STAT_TCPCS: c_uint = 0x20    /* TCP xsum calculated */;
pub const E1000_RXD_STAT_IPCS: c_uint = 0x40    /* IP xsum calculated */;
pub const E1000_RXD_ERR_SE: c_uint = 0x02    /* Symbol Error */;
pub const E1000_RXD_SPC_VLAN_MASK: c_uint = 0x0FFF  /* VLAN ID is in lower 12 bits */;
pub const E1000_RXDEXT_STATERR_LB: c_uint = 0x00040000;
pub const E1000_RXDEXT_STATERR_CE: c_uint = 0x01000000;
pub const E1000_RXDEXT_STATERR_SE: c_uint = 0x02000000;
pub const E1000_RXDEXT_STATERR_SEQ: c_uint = 0x04000000;
pub const E1000_RXDEXT_STATERR_CXE: c_uint = 0x10000000;
pub const E1000_RXDEXT_STATERR_TCPE: c_uint = 0x20000000;
pub const E1000_RXDEXT_STATERR_IPE: c_uint = 0x40000000;
pub const E1000_RXDEXT_STATERR_RXE: c_uint = 0x80000000;
// Same mask, but for extended and packet split descriptors

// Device Control
pub const E1000_CTRL_RST: c_uint = 0x04000000  /* Global reset */;
// Device Status
pub const E1000_STATUS_FD: c_uint = 0x00000001      /* Full duplex.0=half,1=full */;
pub const E1000_STATUS_LU: c_uint = 0x00000002      /* Link up.0=no,1=link */;
pub const E1000_STATUS_TXOFF: c_uint = 0x00000010      /* transmission paused */;
pub const E1000_STATUS_SPEED_10: c_uint = 0x00000000      /* Speed 10Mb/s */;
pub const E1000_STATUS_SPEED_100: c_uint = 0x00000040      /* Speed 100Mb/s */;
pub const E1000_STATUS_SPEED_1000: c_uint = 0x00000080      /* Speed 1000Mb/s */;
pub const SPEED_10: c_int = 10;
pub const SPEED_100: c_int = 100;
pub const SPEED_1000: c_int = 1000;
pub const HALF_DUPLEX: c_int = 1;
pub const FULL_DUPLEX: c_int = 2;
// Transmit Descriptor bit definitions
pub const E1000_TXD_POPTS_IXSM: c_uint = 0x01       /* Insert IP checksum */;
pub const E1000_TXD_POPTS_TXSM: c_uint = 0x02       /* Insert TCP/UDP checksum */;
pub const E1000_TXD_CMD_DEXT: c_uint = 0x20000000 /* Desc extension (0 = legacy) */;
pub const E1000_TXD_STAT_DD: c_uint = 0x00000001 /* Desc Done */;
pub const MAX_JUMBO_FRAME_SIZE: c_uint = 0x3F00;
pub const MAX_STD_JUMBO_FRAME_SIZE: c_int = 9216;
// 802.1q VLAN Packet Size

// Error Codes
pub const E1000_SUCCESS: c_int = 0;
pub const E1000_ERR_CONFIG: c_int = 3;
pub const E1000_ERR_MAC_INIT: c_int = 5;
pub const E1000_ERR_MBX: c_int = 15;
// SRRCTL bit definitions

pub const E1000_SRRCTL_BSIZEHDRSIZE_MASK: c_uint = 0x00000F00;

pub const E1000_SRRCTL_DESCTYPE_ADV_ONEBUF: c_uint = 0x02000000;
pub const E1000_SRRCTL_DESCTYPE_HDR_SPLIT_ALWAYS: c_uint = 0x0A000000;
pub const E1000_SRRCTL_DESCTYPE_MASK: c_uint = 0x0E000000;
pub const E1000_SRRCTL_DROP_EN: c_uint = 0x80000000;
pub const E1000_SRRCTL_BSIZEPKT_MASK: c_uint = 0x0000007F;
pub const E1000_SRRCTL_BSIZEHDR_MASK: c_uint = 0x00003F00;
// Additional Descriptor Control definitions
pub const E1000_TXDCTL_QUEUE_ENABLE: c_uint = 0x02000000 /* Enable specific Tx Que */;
pub const E1000_RXDCTL_QUEUE_ENABLE: c_uint = 0x02000000 /* Enable specific Rx Que */;
// Direct Cache Access (DCA) definitions

