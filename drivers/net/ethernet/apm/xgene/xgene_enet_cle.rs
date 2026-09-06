//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_cle.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Applied Micro X-Gene SoC Ethernet Classifier structures
//
// Copyright (c) 2016, Applied Micro Circuits Corporation
// Authors: Khuong Dinh <kdinh@apm.com>
// Tanmay Inamdar <tinamdar@apm.com>
// Iyappan Subramanian <isubramanian@apm.com>
//

// Register offsets
pub const INDADDR: c_uint = 0x04;
pub const INDCMD: c_uint = 0x08;
pub const INDCMD_STATUS: c_uint = 0x0c;
pub const DATA_RAM0: c_uint = 0x10;
pub const SNPTR0: c_uint = 0x0100;
pub const SPPTR0: c_uint = 0x0104;
pub const DFCLSRESDBPTR0: c_uint = 0x0108;
pub const DFCLSRESDB00: c_uint = 0x010c;
pub const RSS_CTRL0: c_uint = 0x0000013c;

pub const CLE_PORT_OFFSET: c_uint = 0x200;
pub const CLE_DRAM_REGS: c_int = 17;
pub const CLE_DN_TYPE_LEN: c_int = 2;
pub const CLE_DN_TYPE_POS: c_int = 0;
pub const CLE_DN_LASTN_LEN: c_int = 1;
pub const CLE_DN_LASTN_POS: c_int = 2;
pub const CLE_DN_HLS_LEN: c_int = 1;
pub const CLE_DN_HLS_POS: c_int = 3;
pub const CLE_DN_EXT_LEN: c_int = 2;
pub const CLE_DN_EXT_POS: c_int = 4;
pub const CLE_DN_BSTOR_LEN: c_int = 2;
pub const CLE_DN_BSTOR_POS: c_int = 6;
pub const CLE_DN_SBSTOR_LEN: c_int = 2;
pub const CLE_DN_SBSTOR_POS: c_int = 8;
pub const CLE_DN_RPTR_LEN: c_int = 12;
pub const CLE_DN_RPTR_POS: c_int = 12;
pub const CLE_BR_VALID_LEN: c_int = 1;
pub const CLE_BR_VALID_POS: c_int = 0;
pub const CLE_BR_NPPTR_LEN: c_int = 9;
pub const CLE_BR_NPPTR_POS: c_int = 1;
pub const CLE_BR_JB_LEN: c_int = 1;
pub const CLE_BR_JB_POS: c_int = 10;
pub const CLE_BR_JR_LEN: c_int = 1;
pub const CLE_BR_JR_POS: c_int = 11;
pub const CLE_BR_OP_LEN: c_int = 3;
pub const CLE_BR_OP_POS: c_int = 12;
pub const CLE_BR_NNODE_LEN: c_int = 9;
pub const CLE_BR_NNODE_POS: c_int = 15;
pub const CLE_BR_NBR_LEN: c_int = 5;
pub const CLE_BR_NBR_POS: c_int = 24;
pub const CLE_BR_DATA_LEN: c_int = 16;
pub const CLE_BR_DATA_POS: c_int = 0;
pub const CLE_BR_MASK_LEN: c_int = 16;
pub const CLE_BR_MASK_POS: c_int = 16;
pub const CLE_KN_PRIO_POS: c_int = 0;
pub const CLE_KN_PRIO_LEN: c_int = 3;
pub const CLE_KN_RPTR_POS: c_int = 3;
pub const CLE_KN_RPTR_LEN: c_int = 10;
pub const CLE_TYPE_POS: c_int = 0;
pub const CLE_TYPE_LEN: c_int = 2;
pub const CLE_DROP_POS: c_int = 28;
pub const CLE_DROP_LEN: c_int = 1;
pub const CLE_DSTQIDL_POS: c_int = 25;
pub const CLE_DSTQIDL_LEN: c_int = 7;
pub const CLE_DSTQIDH_POS: c_int = 0;
pub const CLE_DSTQIDH_LEN: c_int = 5;
pub const CLE_FPSEL_POS: c_int = 21;
pub const CLE_FPSEL_LEN: c_int = 4;
pub const CLE_NFPSEL_POS: c_int = 17;
pub const CLE_NFPSEL_LEN: c_int = 4;
pub const CLE_PRIORITY_POS: c_int = 5;
pub const CLE_PRIORITY_LEN: c_int = 3;
pub const JMP_ABS: c_int = 0;
pub const JMP_REL: c_int = 1;
pub const JMP_FW: c_int = 0;
pub const JMP_BW: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_ptree_nodes {
    PKT_TYPE_NODE,
    PKT_PROT_NODE,
    RSS_IPV4_TCP_NODE,
    RSS_IPV4_UDP_NODE,
    RSS_IPV4_OTHERS_NODE,
    LAST_NODE,
    MAX_NODES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_byte_store {
    NO_BYTE,
    FIRST_BYTE,
    SECOND_BYTE,
    BOTH_BYTES
}

// Preclassification operation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_node_type {
    INV,
    KN,
    EWDN,
    RES_NODE
}

// Preclassification operation types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_op_type {
    EQT,
    NEQT,
    LTEQT,
    GTEQT,
    AND,
    NAND
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_parser {
    PARSER0,
    PARSER1,
    PARSER2,
    PARSER_ALL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_dram_type {
    PKT_RAM,
    RSS_IDT,
    RSS_IPV4_HASH_SKEY,
    PTREE_RAM = 0xc,
    AVL_RAM,
    DB_RAM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_cmd_type {
    CLE_CMD_WR = 1,
    CLE_CMD_RD = 2,
    CLE_CMD_AVL_ADD = 8,
    CLE_CMD_AVL_DEL = 16,
    CLE_CMD_AVL_SRCH = 32
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_ipv4_rss_hashtype {
    RSS_IPV4_8B,
    RSS_IPV4_12B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_prot_type {
    XGENE_CLE_TCP,
    XGENE_CLE_UDP,
    XGENE_CLE_ESP,
    XGENE_CLE_OTHER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_prot_version {
    XGENE_CLE_IPV4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_cle_ptree_dbptrs {
    DB_RES_DROP,
    DB_RES_DEF,
    DB_RES_ACCEPT,
    DB_MAX_PTRS
}

// RSS sideband signal info
pub const SB_IPFRAG_POS: c_int = 0;
pub const SB_IPFRAG_LEN: c_int = 1;
pub const SB_IPPROT_POS: c_int = 1;
pub const SB_IPPROT_LEN: c_int = 2;
pub const SB_IPVER_POS: c_int = 3;
pub const SB_IPVER_LEN: c_int = 1;
pub const SB_HDRLEN_POS: c_int = 4;
pub const SB_HDRLEN_LEN: c_int = 12;
// RSS indirection table
pub const XGENE_CLE_IDT_ENTRIES: c_int = 128;
pub const IDT_DSTQID_POS: c_int = 0;
pub const IDT_DSTQID_LEN: c_int = 12;
pub const IDT_FPSEL_POS: c_int = 12;
pub const IDT_FPSEL_LEN: c_int = 5;
pub const IDT_NFPSEL_POS: c_int = 17;
pub const IDT_NFPSEL_LEN: c_int = 5;
pub const IDT_FPSEL1_POS: c_int = 12;
pub const IDT_FPSEL1_LEN: c_int = 4;
pub const IDT_NFPSEL1_POS: c_int = 16;
pub const IDT_NFPSEL1_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ptree_branch {
    pub valid: bool,
    pub next_packet_pointer: u16,
    pub jump_bw: bool,
    pub jump_rel: bool,
    pub operation: u8,
    pub next_node: u16,
    pub next_branch: u8,
    pub data: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ptree_ewdn {
    pub node_type: u8,
    pub last_node: bool,
    pub hdr_len_store: bool,
    pub hdr_extn: u8,
    pub byte_store: u8,
    pub search_byte_store: u8,
    pub result_pointer: u16,
    pub num_branches: u8,
    pub branch: [xgene_cle_ptree_branch; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ptree_key {
    pub priority: u8,
    pub result_pointer: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ptree_kn {
    pub node_type: u8,
    pub num_keys: u8,
    pub key: [xgene_cle_ptree_key; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_dbptr {
    pub split_boundary: u8,
    pub mirror_nxtfpsel: u8,
    pub mirror_fpsel: u8,
    pub mirror_dstqid: u16,
    pub drop: u8,
    pub mirror: u8,
    pub hdr_data_split: u8,
    pub hopinfomsbs: u64,
    pub DR: u8,
    pub HR: u8,
    pub hopinfomlsbs: u64,
    pub h0enq_num: u16,
    pub h0fpsel: u8,
    pub nxtfpsel: u8,
    pub fpsel: u8,
    pub dstqid: u16,
    pub cle_priority: u8,
    pub cle_flowgroup: u8,
    pub cle_perflow: u8,
    pub cle_insert_timestamp: u8,
    pub stash: u8,
    pub in: u8,
    pub perprioen: u8,
    pub perflowgroupen: u8,
    pub perflowen: u8,
    pub selhash: u8,
    pub selhdrext: u8,
    pub mirror_nxtfpsel_msb: u8,
    pub mirror_fpsel_msb: u8,
    pub hfpsel_msb: u8,
    pub nxtfpsel_msb: u8,
    pub fpsel_msb: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ptree {
    pub kn: *mut xgene_cle_ptree_kn,
    pub dbptr: *mut xgene_cle_dbptr,
    pub num_kn: u32,
    pub num_dbptr: u32,
    pub start_node: u32,
    pub start_pkt: u32,
    pub start_dbptr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_enet_cle {
    pub base: *mut void __iomem,
    pub ptree: xgene_cle_ptree,
    pub active_parser: xgene_cle_parser,
    pub parsers: u32,
    pub max_nodes: u32,
    pub max_dbptrs: u32,
    pub jump_bytes: u32,
}
