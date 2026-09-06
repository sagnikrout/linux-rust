//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igbvf/vf.h
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
// Copyright(c) 2009 - 2018 Intel Corporation.

pub const E1000_DEV_ID_82576_VF: c_uint = 0x10CA;
pub const E1000_DEV_ID_I350_VF: c_uint = 0x1520;
pub const E1000_REVISION_0: c_int = 0;
pub const E1000_REVISION_1: c_int = 1;
pub const E1000_REVISION_2: c_int = 2;
pub const E1000_REVISION_3: c_int = 3;
pub const E1000_REVISION_4: c_int = 4;
pub const E1000_FUNC_0: c_int = 0;
pub const E1000_FUNC_1: c_int = 1;
// Receive Address Register Count
// Number of high/low register pairs in the RAR.  The RAR (Receive Address
// Registers) holds the directed and multicast addresses that we monitor.
// These entries are also used for MAC-based filtering.
//
pub const E1000_RAR_ENTRIES_VF: c_int = 1;
// Receive Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_adv_rx_desc {
    pub /: *mut *mut __le64 pkt_addr; / Packet buffer address,
    pub /: *mut *mut __le64 hdr_addr; / Header buffer address,
    pub read: },
    pub data: __le32,
    pub /: *mut *mut __le16 pkt_info; / RSS/Packet type,
// Split Header, hdr buffer length
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

pub const E1000_RXDADV_HDRBUFLEN_MASK: c_uint = 0x7FE0;
pub const E1000_RXDADV_HDRBUFLEN_SHIFT: c_int = 5;
// Transmit Descriptor - Advanced
#[repr(C)]
#[derive(Copy, Clone)]
pub union e1000_adv_tx_desc {
    pub /: *mut *mut __le64 buffer_addr; / Address of descriptor's data buf,
    pub cmd_type_len: __le32,
    pub olinfo_status: __le32,
    pub read: },
    pub /: *mut *mut __le64 rsvd; / Reserved,
    pub nxtseq_seed: __le32,
    pub status: __le32,
    pub wb: },
}

// Adv Transmit Descriptor Config Masks
pub const E1000_ADVTXD_DTYP_CTXT: c_uint = 0x00200000 /* Advanced Context Descriptor */;
pub const E1000_ADVTXD_DTYP_DATA: c_uint = 0x00300000 /* Advanced Data Descriptor */;
pub const E1000_ADVTXD_DCMD_EOP: c_uint = 0x01000000 /* End of Packet */;
pub const E1000_ADVTXD_DCMD_IFCS: c_uint = 0x02000000 /* Insert FCS (Ethernet CRC) */;
pub const E1000_ADVTXD_DCMD_RS: c_uint = 0x08000000 /* Report Status */;
pub const E1000_ADVTXD_DCMD_DEXT: c_uint = 0x20000000 /* Descriptor extension (1=Adv) */;
pub const E1000_ADVTXD_DCMD_VLE: c_uint = 0x40000000 /* VLAN pkt enable */;
pub const E1000_ADVTXD_DCMD_TSE: c_uint = 0x80000000 /* TCP Seg enable */;

// Context descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_adv_tx_context_desc {
    pub vlan_macip_lens: __le32,
    pub seqnum_seed: __le32,
    pub type_tucmd_mlhl: __le32,
    pub mss_l4len_idx: __le32,
}

pub const E1000_ADVTXD_TUCMD_IPV4: c_uint = 0x00000400 /* IP Packet Type: 1=IPv4 */;
pub const E1000_ADVTXD_TUCMD_L4T_TCP: c_uint = 0x00000800 /* L4 Packet TYPE of TCP */;
pub const E1000_ADVTXD_TUCMD_L4T_SCTP: c_uint = 0x00001000 /* L4 packet TYPE of SCTP */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_mac_type {
    e1000_undefined = 0,
    e1000_vfadapt,
    e1000_vfadapt_i350,
    e1000_num_macs  /* List is 1-based, so subtract 1 for true count. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_vf_stats {
    pub base_gprc: u64,
    pub base_gptc: u64,
    pub base_gorc: u64,
    pub base_gotc: u64,
    pub base_mprc: u64,
    pub base_gotlbc: u64,
    pub base_gptlbc: u64,
    pub base_gorlbc: u64,
    pub base_gprlbc: u64,
    pub last_gprc: u32,
    pub last_gptc: u32,
    pub last_gorc: u32,
    pub last_gotc: u32,
    pub last_mprc: u32,
    pub last_gotlbc: u32,
    pub last_gptlbc: u32,
    pub last_gorlbc: u32,
    pub last_gprlbc: u32,
    pub gprc: u64,
    pub gptc: u64,
    pub gorc: u64,
    pub gotc: u64,
    pub mprc: u64,
    pub gotlbc: u64,
    pub gptlbc: u64,
    pub gorlbc: u64,
    pub gprlbc: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_operations {
// Function pointers for the MAC.
    pub ): *mut *mut s32 (init_params)(struct e1000_hw,
    pub ): *mut *mut s32 (check_for_link)(struct e1000_hw,
    pub ): *mut *mut void (clear_vfta)(struct e1000_hw,
    pub ): *mut *mut s32 (get_bus_info)(struct e1000_hw,
    pub ): *mut *mut *mut *mut s32 (get_link_up_info)(struct e1000_hw , u16 , u16,
    pub u32): *mut *mut *mut *mut void (update_mc_addr_list)(struct e1000_hw , u8 , u32, u32,,
    pub ): *mut *mut *mut s32 (set_uc_addr)(struct e1000_hw , u32, u8,
    pub ): *mut *mut s32 (reset_hw)(struct e1000_hw,
    pub ): *mut *mut s32 (init_hw)(struct e1000_hw,
    pub ): *mut *mut s32 (setup_link)(struct e1000_hw,
    pub u32): *mut *mut *mut void (write_vfta)(struct e1000_hw , u32,,
    pub u32): *mut *mut *mut void (mta_set)(struct e1000_hw ,,
    pub u32): *mut *mut *mut *mut void (rar_set)(struct e1000_hw , u8,,
    pub ): *mut *mut s32 (read_mac_addr)(struct e1000_hw,
    pub bool): *mut *mut *mut s32 (set_vfta)(struct e1000_hw , u16,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mac_info {
    pub ops: e1000_mac_operations,
    pub addr: [u8; 6],
    pub perm_addr: [u8; 6],
    pub type: e1000_mac_type,
    pub mta_reg_count: u16,
    pub rar_entry_count: u16,
    pub get_link_status: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_operations {
    pub hw): *mut *mut s32 (init_params)(struct e1000_hw,
    pub u16): *mut *mut *mut *mut s32 (read)(struct e1000_hw , u32 ,,
    pub u16): *mut *mut *mut *mut s32 (write)(struct e1000_hw , u32 ,,
    pub u16): *mut *mut *mut *mut s32 (read_posted)(struct e1000_hw , u32 ,,
    pub u16): *mut *mut *mut *mut s32 (write_posted)(struct e1000_hw , u32 ,,
    pub ): *mut *mut s32 (check_for_msg)(struct e1000_hw,
    pub ): *mut *mut s32 (check_for_ack)(struct e1000_hw,
    pub ): *mut *mut s32 (check_for_rst)(struct e1000_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_stats {
    pub msgs_tx: u32,
    pub msgs_rx: u32,
    pub acks: u32,
    pub reqs: u32,
    pub rsts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_mbx_info {
    pub ops: e1000_mbx_operations,
    pub stats: e1000_mbx_stats,
    pub timeout: u32,
    pub usec_delay: u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_dev_spec_vf {
    pub vf_number: u32,
    pub v2p_mailbox: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e1000_hw {
    pub back: *mut c_void,
    pub hw_addr: *mut u8 __iomem,
    pub flash_address: *mut u8 __iomem,
    pub io_base: c_ulong,
    pub mac: e1000_mac_info,
    pub mbx: e1000_mbx_info,
    pub /: *mut *mut spinlock_t mbx_lock; / serializes mailbox ops,
    pub vf: e1000_dev_spec_vf,
    pub dev_spec: },
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub vendor_id: u16,
    pub revision_id: u8,
}

// These functions must be implemented by drivers
extern "C" {
    pub fn e1000_rlpml_set_vf(: *mut e1000_hw, _arg: u16);
}
extern "C" {
    pub fn e1000_init_function_pointers_vf(hw: *mut e1000_hw);
}
