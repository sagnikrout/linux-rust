//! Automatically rewritten from C Header to Rust Module
//! Source: net/hsr/hsr_main.h
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
// Copyright 2011-2014 Autronica Fire and Security AS
//
// Author(s):
// 2011-2014 Arvid Brodin, arvid.brodin@alten.se
//
// include file for HSR and PRP.
//

// Time constants as specified in the HSR specification (IEC-62439-3 2010)
// Table 8.
// All values in milliseconds.
//

// By how much may slave1 and slave2 timestamps of latest received frame from
// each node differ before we notify of communication problem?
//

// How often shall we check for broken ring and remove node entries older than
// HSR_NODE_FORGET_TIME?
//

pub const HSR_TLV_ANNOUNCE: c_int = 22;
pub const HSR_TLV_LIFE_CHECK: c_int = 23;
// PRP V1 life check for Duplicate discard
pub const PRP_TLV_LIFE_CHECK_DD: c_int = 20;
// PRP V1 life check for Duplicate Accept
pub const PRP_TLV_LIFE_CHECK_DA: c_int = 21;
// PRP V1 life redundancy box MAC address
pub const PRP_TLV_REDBOX_MAC: c_int = 30;
pub const HSR_V1_SUP_LSDUSIZE: c_int = 52;
// The helper functions below assumes that 'path' occupies the 4 most
// significant bits of the 16-bit field shared by 'path' and 'LSDU_size' (or
// equivalently, the 4 most significant bits of HSR tag byte 14).
//
// This is unclear in the IEC specification; its definition of MAC addresses
// indicates the spec is written with the least significant bit first (to the
// left). This, however, would mean that the LSDU field would be split in two
// with the path field in-between, which seems strange. I'm guessing the MAC
// address definition is in error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_ethhdr {
    pub ethhdr: ethhdr,
    pub hsr_tag: hsr_tag,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_vlan_ethhdr {
    pub vlanhdr: vlan_ethhdr,
    pub hsr_tag: hsr_tag,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_sup_tlv {
    pub HSR_TLV_type: u8,
    pub HSR_TLV_length: u8,
    pub __packed: },
// HSR/PRP Supervision Frame data types.
// Field names as defined in the IEC:2010 standard for HSR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_sup_tag {
    pub path_and_HSR_ver: __be16,
    pub sequence_nr: __be16,
    pub tlv: hsr_sup_tlv,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_sup_payload {
    pub macaddress_A: [c_uchar; ETH_ALEN],
    pub __packed: },
    pub path): *mut *mut set_hsr_tag_path((struct hsr_tag )hst,,
    pub HSR_ver): *mut *mut set_hsr_tag_LSDU_size((struct hsr_tag )hst,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsrv0_ethhdr_sp {
    pub ethhdr: ethhdr,
    pub hsr_sup: hsr_sup_tag,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsrv1_ethhdr_sp {
    pub ethhdr: ethhdr,
    pub hsr: hsr_tag,
    pub hsr_sup: hsr_sup_tag,
    pub __packed: },
// PRP Redunancy Control Trailor (RCT).
// As defined in IEC-62439-4:2012, the PRP RCT is really { sequence Nr,
// Lan indentifier (LanId), LSDU_size and PRP_suffix = 0x88FB }.
//
// Field names as defined in the IEC:2012 standard for PRP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prp_rct {
    pub sequence_nr: __be16,
    pub lan_id_and_LSDU_size: __be16,
    pub PRP_suffix: __be16,
    pub __packed: },
    pub 0x0FFF: return ntohs(rct->lan_id_and_LSDU_size) &,
    pub 12)): 0x0FFF) | (lan_id <<,
    pub 0x0FFF)): 0xF000) | (LSDU_size &,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_port {
    pub port_list: list_head,
    pub dev: *mut net_device,
    pub hsr: *mut hsr_priv,
    pub type: hsr_port_type,
    pub rcu: rcu_head,
    pub original_macaddress: [c_uchar; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_proto_ops {
// format and send supervision frame
    pub addr[ETH_ALEN]): c_uchar,
    pub node): *mut hsr_node,
    pub port): *mut *mut *mut bool (drop_frame)(struct hsr_frame_info frame, struct hsr_port,
    pub port): *mut hsr_port,
    pub port): *mut hsr_port,
    pub frame): *mut hsr_frame_info,
    pub protocol): *mut *mut bool (invalid_dan_ingress_frame)(__be16,
    pub is_sup): *mut *mut *mut void (update_san_info)(struct hsr_node node, bool,
    pub frame): *mut hsr_frame_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_self_node {
    pub macaddress_A: [c_uchar; ETH_ALEN],
    pub macaddress_B: [c_uchar; ETH_ALEN],
    pub rcu_head: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsr_priv {
    pub rcu_head: rcu_head,
    pub ports: list_head,
    pub /: *mut *mut list_head node_db; / Known HSR nodes,
    pub /: *mut *mut list_head proxy_node_db; / RedBox HSR proxy nodes,
    pub /: *mut *mut *mut hsr_self_node __rcu self_node; / MACs of slaves,
    pub /: *mut *mut timer_list announce_timer; / Supervision frame dispatch,
    pub announce_proxy_timer: timer_list,
    pub prune_timer: timer_list,
    pub prune_proxy_timer: timer_list,
    pub announce_count: c_int,
    pub sequence_nr: u16,
    pub /: *mut *mut u16 sup_sequence_nr; / For HSRv1 separate seq_nr for supervision,
    pub /: *mut *mut hsr_version prot_version; / Indicate if HSRv0, HSRv1 or PRPv1,
    pub /: *mut *mut spinlock_t seqnr_lock; / locking for sequence_nr,
    pub /: *mut *mut spinlock_t list_lock; / locking for node list,
    pub proto_ops: *const hsr_proto_ops,
pub const PRP_LAN_ID: c_uint = 0x5     /* 0x1010 for A and 0x1011 for B. Bit 0 is set;
// based on SLAVE_A or SLAVE_B
//
    pub bits: *mut *mut u8 net_id; / for PRP, it occupies most significant 3,
// of lan_id
//
    pub /: *mut *mut bool fwd_offloaded; / Forwarding offloaded to HW,
    pub /: *mut *mut bool redbox; / Device supports HSR RedBox,
    pub __aligned(sizeof(u16)): unsigned char macaddress_redbox[ETH_ALEN],
    pub __aligned(sizeof(u16)): unsigned char sup_multicast_addr[ETH_ALEN],
// Align to u16 boundary to avoid unaligned access
// in ether_addr_equal
//

    pub node_tbl_root: *mut dentry,

}

// Caller must ensure skb is a valid HSR frame
extern "C" {
    pub fn ntohs(_arg: hsr_ethhdr->hsr_tag.sequence_nr) -> return;
}
// Assume caller has confirmed this skb is PRP suffixed
extern "C" {
    pub fn ntohs(_arg: rct->sequence_nr) -> return;
}
// assume there is a valid rct

extern "C" {
    pub fn hsr_debugfs_rename(dev: *mut net_device);
}
extern "C" {
    pub fn hsr_debugfs_init(priv: *mut hsr_priv, hsr_dev: *mut net_device);
}
extern "C" {
    pub fn hsr_debugfs_term(priv: *mut hsr_priv);
}
extern "C" {
    pub fn hsr_debugfs_create_root();
}
extern "C" {
    pub fn hsr_debugfs_remove_root();
}

