//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/ntmp.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
// Copyright 2025-2026 NXP

pub const NTMP_NULL_ENTRY_ID: c_uint = 0xffffffffU;
pub const IPFT_MAX_PLD_LEN: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct maft_keye_data {
    pub mac_addr: [u8; ETH_ALEN],
    pub resv: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maft_cfge_data {
    pub si_bitmap: __le16,
    pub resv: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_cbdr_regs {
    pub pir: *mut void __iomem,
    pub cir: *mut void __iomem,
    pub mr: *mut void __iomem,
    pub bar0: *mut void __iomem,
    pub bar1: *mut void __iomem,
    pub lenr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_tbl_vers {
    pub maft_ver: u8,
    pub rsst_ver: u8,
    pub fdbt_ver: u8,
    pub vft_ver: u8,
    pub bpt_ver: u8,
    pub ipft_ver: u8,
    pub ett_ver: u8,
    pub ect_ver: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_swcbd {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netc_cbdr {
    pub dev: *mut device,
    pub regs: netc_cbdr_regs,
    pub bd_num: c_int,
    pub next_to_use: c_int,
    pub next_to_clean: c_int,
    pub dma_size: c_int,
    pub addr_base: *mut c_void,
    pub addr_base_align: *mut c_void,
    pub dma_base: dma_addr_t,
    pub dma_base_align: dma_addr_t,
    pub swcbd: *mut netc_swcbd,
// Serialize the order of command BD ring
    pub ring_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntmp_user {
    pub /: *mut *mut int cbdr_num; / number of control BD ring,
    pub dev: *mut device,
    pub ring: *mut netc_cbdr,
    pub tbl: netc_tbl_vers,
// NTMP table bitmaps for resource management
    pub ett_bitmap_size: u32,
    pub ect_bitmap_size: u32,
    pub maft_num_entries: u16,
    pub /: *mut *mut *mut unsigned long ett_gid_bitmap; / only valid for switch,
    pub /: *mut *mut *mut unsigned long ect_gid_bitmap; / only valid for switch,
    pub /: *mut *mut *mut unsigned long maft_eid_bitmap; / only valid for ENETC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct maft_entry_data {
    pub keye: maft_keye_data,
    pub cfge: maft_cfge_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_pld_byte {
    pub data: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_keye_data {
    pub precedence: __le16,
    pub resv0: [__le16; 3],
    pub frm_attr_flags: __le16,

pub const IPFT_FAF_TCP_HDR: c_int = 1;
pub const IPFT_FAF_UDP_HDR: c_int = 2;
pub const IPFT_FAF_SCTP_HDR: c_int = 3;

    pub frm_attr_flags_mask: __le16,
    pub dscp: __le16,

pub const IPFT_DSCP_MASK_ALL: c_uint = 0x3f;
    pub /: *mut *mut __le16 src_port; / This field is reserved for ENETC,

pub const IPFT_SRC_PORT_MASK_ALL: c_uint = 0x1f;
    pub outer_vlan_tci: __be16,
    pub outer_vlan_tci_mask: __be16,
    pub dmac: [u8; ETH_ALEN],
    pub dmac_mask: [u8; ETH_ALEN],
    pub smac: [u8; ETH_ALEN],
    pub smac_mask: [u8; ETH_ALEN],
    pub inner_vlan_tci: __be16,
    pub inner_vlan_tci_mask: __be16,
    pub ethertype: __be16,
    pub ethertype_mask: __be16,
    pub ip_protocol: u8,
    pub ip_protocol_mask: u8,
    pub resv1: [__le16; 7],
    pub ip_src: [__be32; 4],
    pub resv2: [__le32; 2],
    pub ip_src_mask: [__be32; 4],
    pub l4_src_port: __be16,
    pub l4_src_port_mask: __be16,
    pub resv3: __le32,
    pub ip_dst: [__be32; 4],
    pub resv4: [__le32; 2],
    pub ip_dst_mask: [__be32; 4],
    pub l4_dst_port: __be16,
    pub l4_dst_port_mask: __be16,
    pub resv5: __le32,
    pub byte: [ipft_pld_byte; IPFT_MAX_PLD_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_cfge_data {
    pub cfg: __le32,

pub const IPFT_FLTFA_DISCARD: c_int = 0;
pub const IPFT_FLTFA_PERMIT: c_int = 1;
// Redirect is only for switch
pub const IPFT_FLTFA_REDIRECT: c_int = 2;

pub const IPFT_FLTA_RP: c_int = 1;
pub const IPFT_FLTA_IS: c_int = 2;
pub const IPFT_FLTA_SI_BITMAP: c_int = 3;

    pub flta_tgt: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipft_entry_data {
    pub /: *mut *mut u32 entry_id; / hardware assigns entry ID,
    pub keye: ipft_keye_data,
    pub cfge: ipft_cfge_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_keye_data {
    pub /: *mut *mut u8 mac_addr[ETH_ALEN]; / big-endian,
    pub resv0: __le16,
    pub fid: __le16,

    pub resv1: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_cfge_data {
    pub port_bitmap: __le32,

    pub cfg: __le32,

    pub et_eid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdbt_entry_data {
    pub entry_id: u32,
    pub keye: fdbt_keye_data,
    pub cfge: fdbt_cfge_data,
    pub acte: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vft_cfge_data {
    pub bitmap_stg: __le32,

    pub fid: __le16,

    pub cfg: __le16,

    pub eta_port_bitmap: __le32,

    pub et_eid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ett_cfge_data {
    pub efm_cfg: __le16,

pub const ETT_ECA_INC: c_int = 1;

pub const ETT_FRM_LEN_DEL_VLAN: c_uint = 0x7c;
pub const ETT_FRM_LEN_DEL_RTAG: c_uint = 0x7a;
pub const ETT_FRM_LEN_DEL_VLAN_RTAG: c_uint = 0x76;
    pub efm_data_len: __le16,

    pub efm_eid: __le32,
    pub ec_eid: __le32,
    pub esqa_tgt_eid: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpt_bpse_data {
    pub amount_used: __le32,
    pub amount_used_hwm: __le32,
    pub bpd_fc_state: u8,

    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpt_cfge_data {
    pub fccfg_sbpen: u8,

pub const BPT_FC_CFG_EN_BPFC: c_int = 1;
    pub pfc_vector: u8,
    pub max_thresh: __le16,
    pub fc_on_thresh: __le16,
    pub fc_off_thresh: __le16,
    pub sbp_thresh: __le16,
    pub resv: __le16,
    pub sbp_eid: __le32,
    pub fc_ports: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ntmp_fmt_eid {
    pub index: __le32,

    pub vuda_sqta: __le32,

pub const FMTEID_VUDA_DEL_OTAG: c_int = 2;

pub const FMTEID_SQTA_DEL: c_int = 2;

    pub vara_vid: __le32,

}

extern "C" {
    pub fn ntmp_free_cbdr(cbdr: *mut netc_cbdr);
}
extern "C" {
    pub fn ntmp_lookup_free_eid(bitmap: *mut c_ulong, size: u32) -> u32;
}
extern "C" {
    pub fn ntmp_clear_eid_bitmap(bitmap: *mut c_ulong, entry_id: u32);
}
// NTMP APIs
extern "C" {
    pub fn ntmp_maft_delete_entry(user: *mut ntmp_user, entry_id: u32) -> c_int;
}
extern "C" {
    pub fn ntmp_ipft_delete_entry(user: *mut ntmp_user, entry_id: u32) -> c_int;
}
extern "C" {
    pub fn ntmp_fdbt_delete_entry(user: *mut ntmp_user, entry_id: u32) -> c_int;
}
extern "C" {
    pub fn ntmp_fdbt_update_activity_element(user: *mut ntmp_user) -> c_int;
}
extern "C" {
    pub fn ntmp_fdbt_delete_ageing_entries(user: *mut ntmp_user, act_cnt: u8) -> c_int;
}
extern "C" {
    pub fn ntmp_fdbt_delete_port_dynamic_entries(user: *mut ntmp_user, port: c_int) -> c_int;
}
extern "C" {
    pub fn ntmp_vft_delete_entry(user: *mut ntmp_user, vid: u16) -> c_int;
}
extern "C" {
    pub fn ntmp_ett_delete_entry(user: *mut ntmp_user, entry_id: u32) -> c_int;
}
extern "C" {
    pub fn ntmp_ect_update_entry(user: *mut ntmp_user, entry_id: u32) -> c_int;
}

