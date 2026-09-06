//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_struct.h
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
// Marvell RVU Ethernet driver
//
// Copyright (C) 2020 Marvell.
//
// NIX WQE/CQE size 128 byte or 512 byte
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_cqesz_e {
    NIX_XQESZ_W64 = 0x0,
    NIX_XQESZ_W16 = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sqes_e {
    NIX_SQESZ_W16 = 0x0,
    NIX_SQESZ_W8 = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_send_ldtype {
    NIX_SEND_LDTYPE_LDD  = 0x0,
    NIX_SEND_LDTYPE_LDT  = 0x1,
    NIX_SEND_LDTYPE_LDWB = 0x2,
}

// CSUM offload
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sendl3type {
    NIX_SENDL3TYPE_NONE = 0x0,
    NIX_SENDL3TYPE_IP4 = 0x2,
    NIX_SENDL3TYPE_IP4_CKSUM = 0x3,
    NIX_SENDL3TYPE_IP6 = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sendl4type {
    NIX_SENDL4TYPE_NONE,
    NIX_SENDL4TYPE_TCP_CKSUM,
    NIX_SENDL4TYPE_SCTP_CKSUM,
    NIX_SENDL4TYPE_UDP_CKSUM,
}

// NIX wqe/cqe types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_xqe_type {
    NIX_XQE_TYPE_INVALID   = 0x0,
    NIX_XQE_TYPE_RX        = 0x1,
    NIX_XQE_TYPE_RX_IPSECS = 0x2,
    NIX_XQE_TYPE_RX_IPSECH = 0x3,
    NIX_XQE_TYPE_RX_IPSECD = 0x4,
    NIX_XQE_TYPE_SEND      = 0x8,
}

// NIX CQE/SQE subdescriptor types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_subdc {
    NIX_SUBDC_NOP  = 0x0,
    NIX_SUBDC_EXT  = 0x1,
    NIX_SUBDC_CRC  = 0x2,
    NIX_SUBDC_IMM  = 0x3,
    NIX_SUBDC_SG   = 0x4,
    NIX_SUBDC_MEM  = 0x5,
    NIX_SUBDC_JUMP = 0x6,
    NIX_SUBDC_WORK = 0x7,
    NIX_SUBDC_SOD  = 0xf,
}

// Algorithm for nix_sqe_mem_s header (value of the `alg` field)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sendmemalg {
    NIX_SENDMEMALG_E_SET       = 0x0,
    NIX_SENDMEMALG_E_SETTSTMP  = 0x1,
    NIX_SENDMEMALG_E_SETRSLT   = 0x2,
    NIX_SENDMEMALG_E_ADD       = 0x8,
    NIX_SENDMEMALG_E_SUB       = 0x9,
    NIX_SENDMEMALG_E_ADDLEN    = 0xa,
    NIX_SENDMEMALG_E_SUBLEN    = 0xb,
    NIX_SENDMEMALG_E_ADDMBUF   = 0xc,
    NIX_SENDMEMALG_E_SUBMBUF   = 0xd,
    NIX_SENDMEMALG_E_ENUM_LAST = 0xe,
}

// NIX CQE header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cqe_hdr_s {
    pub 32: u64 flow_tag :,
    pub 20: u64 q :,
    pub 6: u64 reserved_52_57 :,
    pub 2: u64 node :,
    pub 4: u64 cqe_type :,
}

// NIX CQE RX parse structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_parse_s {
    pub 12: u64 chan :,
    pub 5: u64 desc_sizem1 :,
    pub 1: u64 rsvd_17 :,
    pub 1: u64 express :,
    pub 1: u64 wqwd :,
    pub 4: u64 errlev :,
    pub 8: u64 errcode :,
    pub 4: u64 latype :,
    pub 4: u64 lbtype :,
    pub 4: u64 lctype :,
    pub 4: u64 ldtype :,
    pub 4: u64 letype :,
    pub 4: u64 lftype :,
    pub 4: u64 lgtype :,
    pub 4: u64 lhtype :,
    pub /: *mut *mut u64 pkt_lenm1 : 16; / W1,
    pub 1: u64 l2m :,
    pub 1: u64 l2b :,
    pub 1: u64 l3m :,
    pub 1: u64 l3b :,
    pub 1: u64 vtag0_valid :,
    pub 1: u64 vtag0_gone :,
    pub 1: u64 vtag1_valid :,
    pub 1: u64 vtag1_gone :,
    pub 6: u64 pkind :,
    pub 2: u64 rsvd_95_94 :,
    pub 16: u64 vtag0_tci :,
    pub 16: u64 vtag1_tci :,
    pub /: *mut *mut u64 laflags : 8; / W2,
    pub 8: u64 lbflags :,
    pub 8: u64 lcflags :,
    pub 8: u64 ldflags :,
    pub 8: u64 leflags :,
    pub 8: u64 lfflags :,
    pub 8: u64 lgflags :,
    pub 8: u64 lhflags :,
    pub /: *mut *mut u64 eoh_ptr : 8; / W3,
    pub 20: u64 wqe_aura :,
    pub 20: u64 pb_aura :,
    pub 16: u64 match_id :,
    pub /: *mut *mut u64 laptr : 8; / W4,
    pub 8: u64 lbptr :,
    pub 8: u64 lcptr :,
    pub 8: u64 ldptr :,
    pub 8: u64 leptr :,
    pub 8: u64 lfptr :,
    pub 8: u64 lgptr :,
    pub 8: u64 lhptr :,
    pub /: *mut *mut u64 vtag0_ptr : 8; / W5,
    pub 8: u64 vtag1_ptr :,
    pub 5: u64 flow_key_alg :,
    pub 19: u64 rsvd_359_341 :,
    pub 2: u64 color :,
    pub 22: u64 rsvd_383_362 :,
    pub /: *mut *mut u64 rsvd_447_384; / W6,
}

// NIX CQE RX scatter/gather subdescriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_sg_s {
    pub /: *mut *mut u64 seg_size : 16; / W0,
    pub 16: u64 seg2_size :,
    pub 16: u64 seg3_size :,
    pub 2: u64 segs :,
    pub 10: u64 rsvd_59_50 :,
    pub 4: u64 subdc :,
    pub seg_addr: u64,
    pub seg2_addr: u64,
    pub seg3_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_send_comp_s {
    pub 8: u64 status :,
    pub 16: u64 sqe_id :,
    pub 40: u64 rsvd_24_63 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cqe_rx_s {
    pub hdr: nix_cqe_hdr_s,
    pub parse: nix_rx_parse_s,
    pub sg: nix_rx_sg_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cqe_tx_s {
    pub hdr: nix_cqe_hdr_s,
    pub comp: nix_send_comp_s,
}

// NIX SQE header structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_sqe_hdr_s {
    pub /: *mut *mut u64 total : 18; / W0,
    pub 1: u64 reserved_18 :,
    pub 1: u64 df :,
    pub 20: u64 aura :,
    pub 3: u64 sizem1 :,
    pub 1: u64 pnc :,
    pub 20: u64 sq :,
    pub /: *mut *mut u64 ol3ptr : 8; / W1,
    pub 8: u64 ol4ptr :,
    pub 8: u64 il3ptr :,
    pub 8: u64 il4ptr :,
    pub 4: u64 ol3type :,
    pub 4: u64 ol4type :,
    pub 4: u64 il3type :,
    pub 4: u64 il4type :,
    pub 16: u64 sqe_id :,
}

// NIX send extended header subdescriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_sqe_ext_s {
    pub /: *mut *mut u64 lso_mps : 14; / W0,
    pub 1: u64 lso :,
    pub 1: u64 tstmp :,
    pub 8: u64 lso_sb :,
    pub 5: u64 lso_format :,
    pub 3: u64 rsvd_31_29 :,
    pub 9: u64 shp_chg :,
    pub 1: u64 shp_dis :,
    pub 2: u64 shp_ra :,
    pub 8: u64 markptr :,
    pub 7: u64 markform :,
    pub 1: u64 mark_en :,
    pub 4: u64 subdc :,
    pub /: *mut *mut u64 vlan0_ins_ptr : 8; / W1,
    pub 16: u64 vlan0_ins_tci :,
    pub 8: u64 vlan1_ins_ptr :,
    pub 16: u64 vlan1_ins_tci :,
    pub 1: u64 vlan0_ins_ena :,
    pub 1: u64 vlan1_ins_ena :,
    pub 2: u64 init_color :,
    pub 12: u64 rsvd_127_116 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_sqe_sg_s {
    pub 16: u64 seg1_size :,
    pub 16: u64 seg2_size :,
    pub 16: u64 seg3_size :,
    pub 2: u64 segs :,
    pub 5: u64 rsvd_54_50 :,
    pub 1: u64 i1 :,
    pub 1: u64 i2 :,
    pub 1: u64 i3 :,
    pub 2: u64 ld_type :,
    pub 4: u64 subdc :,
}

// NIX send memory subdescriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_sqe_mem_s {
    pub 8: u64 start_offset :,
    pub 4: u64 rsvd_11_8 :,
    pub 1: u64 rsvd_12 :,
    pub 1: u64 udp_csum_crt :,
    pub 1: u64 update64 :,
    pub 1: u64 rsvd_15_16 :,
    pub 32: u64 base_ns :,
    pub 1: u64 step_type :,
    pub 3: u64 rsvd_51_49 :,
    pub 1: u64 per_lso_seg :,
    pub 1: u64 wmem :,
    pub 2: u64 dsz :,
    pub 4: u64 alg :,
    pub 4: u64 subdc :,
    pub /: *mut *mut u64 addr; / W1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_cqerrint_e {
    NIX_CQERRINT_DOOR_ERR = 0,
    NIX_CQERRINT_WR_FULL = 1,
    NIX_CQERRINT_CQE_FAULT = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_rqint_e {
    NIX_RQINT_DROP = 0,
    NIX_RQINT_RED = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sqint_e {
    NIX_SQINT_LMT_ERR = 0,
    NIX_SQINT_MNQ_ERR = 1,
    NIX_SQINT_SEND_ERR = 2,
    NIX_SQINT_SQB_ALLOC_FAIL = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_sqoperr_e {
    NIX_SQOPERR_OOR = 0,
    NIX_SQOPERR_CTX_FAULT = 1,
    NIX_SQOPERR_CTX_POISON = 2,
    NIX_SQOPERR_DISABLED = 3,
    NIX_SQOPERR_SIZE_ERR = 4,
    NIX_SQOPERR_OFLOW = 5,
    NIX_SQOPERR_SQB_NULL = 6,
    NIX_SQOPERR_SQB_FAULT = 7,
    NIX_SQOPERR_SQE_SZ_ZERO = 8,
    NIX_SQOPERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_mnqerr_e {
    NIX_MNQERR_SQ_CTX_FAULT = 0,
    NIX_MNQERR_SQ_CTX_POISON = 1,
    NIX_MNQERR_SQB_FAULT = 2,
    NIX_MNQERR_SQB_POISON = 3,
    NIX_MNQERR_TOTAL_ERR = 4,
    NIX_MNQERR_LSO_ERR = 5,
    NIX_MNQERR_CQ_QUERY_ERR = 6,
    NIX_MNQERR_MAX_SQE_SIZE_ERR = 7,
    NIX_MNQERR_MAXLEN_ERR = 8,
    NIX_MNQERR_SQE_SIZEM1_ZERO = 9,
    NIX_MNQERR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_snd_status_e {
    NIX_SND_STATUS_GOOD = 0x0,
    NIX_SND_STATUS_SQ_CTX_FAULT = 0x1,
    NIX_SND_STATUS_SQ_CTX_POISON = 0x2,
    NIX_SND_STATUS_SQB_FAULT = 0x3,
    NIX_SND_STATUS_SQB_POISON = 0x4,
    NIX_SND_STATUS_HDR_ERR = 0x5,
    NIX_SND_STATUS_EXT_ERR = 0x6,
    NIX_SND_STATUS_JUMP_FAULT = 0x7,
    NIX_SND_STATUS_JUMP_POISON = 0x8,
    NIX_SND_STATUS_CRC_ERR = 0x10,
    NIX_SND_STATUS_IMM_ERR = 0x11,
    NIX_SND_STATUS_SG_ERR = 0x12,
    NIX_SND_STATUS_MEM_ERR = 0x13,
    NIX_SND_STATUS_INVALID_SUBDC = 0x14,
    NIX_SND_STATUS_SUBDC_ORDER_ERR = 0x15,
    NIX_SND_STATUS_DATA_FAULT = 0x16,
    NIX_SND_STATUS_DATA_POISON = 0x17,
    NIX_SND_STATUS_NPC_DROP_ACTION = 0x20,
    NIX_SND_STATUS_LOCK_VIOL = 0x21,
    NIX_SND_STATUS_NPC_UCAST_CHAN_ERR = 0x22,
    NIX_SND_STATUS_NPC_MCAST_CHAN_ERR = 0x23,
    NIX_SND_STATUS_NPC_MCAST_ABORT = 0x24,
    NIX_SND_STATUS_NPC_VTAG_PTR_ERR = 0x25,
    NIX_SND_STATUS_NPC_VTAG_SIZE_ERR = 0x26,
    NIX_SND_STATUS_SEND_MEM_FAULT = 0x27,
    NIX_SND_STATUS_SEND_STATS_ERR = 0x28,
    NIX_SND_STATUS_MAX,
}
