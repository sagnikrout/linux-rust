//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igc/igc_base.h
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
// Copyright (c)  2018 Intel Corporation
// forward declaration
extern "C" {
    pub fn igc_rx_fifo_flush_base(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_power_down_phy_copper_base(hw: *mut igc_hw);
}
extern "C" {
    pub fn igc_is_device_id_i225(hw: *mut igc_hw) -> bool;
}
extern "C" {
    pub fn igc_is_device_id_i226(hw: *mut igc_hw) -> bool;
}
// Transmit Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union igc_adv_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igc_adv_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub launch_time: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

// Adv Transmit Descriptor Config Masks
pub const IGC_ADVTXD_MAC_TSTAMP: c_uint = 0x00080000 /* IEEE1588 Timestamp packet */;
pub const IGC_ADVTXD_TSTAMP_REG_1: c_uint = 0x00010000 /* Select register 1 for timestamp */;
pub const IGC_ADVTXD_TSTAMP_REG_2: c_uint = 0x00020000 /* Select register 2 for timestamp */;
pub const IGC_ADVTXD_TSTAMP_REG_3: c_uint = 0x00030000 /* Select register 3 for timestamp */;
pub const IGC_ADVTXD_TSTAMP_TIMER_1: c_uint = 0x00010000 /* Select timer 1 for timestamp */;
pub const IGC_ADVTXD_TSTAMP_TIMER_2: c_uint = 0x00020000 /* Select timer 2 for timestamp */;
pub const IGC_ADVTXD_TSTAMP_TIMER_3: c_uint = 0x00030000 /* Select timer 3 for timestamp */;
pub const IGC_ADVTXD_DTYP_CTXT: c_uint = 0x00200000 /* Advanced Context Descriptor */;
pub const IGC_ADVTXD_DTYP_DATA: c_uint = 0x00300000 /* Advanced Data Descriptor */;
pub const IGC_ADVTXD_DCMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const IGC_ADVTXD_DCMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const IGC_ADVTXD_DCMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const IGC_ADVTXD_DCMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (1=Adv) */;
pub const IGC_ADVTXD_DCMD_VLE: c_uint = 0x40000000 /* VLAN pkt enable */;
pub const IGC_ADVTXD_DCMD_TSE: c_uint = 0x80000000 /* TCP Seg enable */;

pub const IGC_RAR_ENTRIES: c_int = 16;
// Receive Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union igc_adv_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub data: __le32,
    pub type*/: *mut *mut __le16 pkt_info; /RSS type, Pkt,
// Split Header, header buffer len
    pub hdr_info: __le16,
    pub hs_rss: },
    pub lo_dword: },
    pub /: *mut *mut __le32 rss; / RSS Hash,
    pub /: *mut *mut __le16 ip_id; / IP id,
    pub /: *mut *mut __le16 csum; / Packet Checksum,
    pub csum_ip: },
    pub hi_dword: },
    pub lower: },
    pub /: *mut *mut __le32 status_error; / ext status/error,
    pub /: *mut *mut __le16 length; / Packet length,
    pub /: *mut *mut __le16 vlan; / VLAN tag,
    pub upper: },
    pub /: *mut *mut } wb; / writeback,
}

// SRRCTL bit definitions

