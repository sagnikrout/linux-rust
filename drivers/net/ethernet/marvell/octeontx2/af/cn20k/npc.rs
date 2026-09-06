//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cn20k/npc.h
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
// Marvell RVU Admin Function driver
//
// Copyright (C) 2026 Marvell.
//
pub const MKEX_CN20K_SIGN: c_uint = 0x19bbfdbd160;
// MAX_NUM_BANKS, MAX_SUBBANK_DEPTH and MAX_NUM_SUB_BANKS represent
// hard limit on all silicon variants, preventing any possibility of
// out-of-bounds access on matrix defined using these values.
//
pub const MAX_NUM_BANKS: c_int = 2;
pub const MAX_NUM_SUB_BANKS: c_int = 32;
pub const MAX_SUBBANK_DEPTH: c_int = 256;
// strtoull of "mkexprof" with base:36
pub const MKEX_END_SIGN: c_uint = 0xdeadbeef;

pub const NPC_CN20K_TOTAL_NIBBLE: c_int = 23;

// NPC_PARSE_KEX_S nibble definitions for each field

// Rx parse key extract nibble enable

// Tx parse key extract nibble enable

//
// enum npc_subbank_flag - NPC subbank status
//
// subbank flag indicates whether the subbank is free
// or used.
//
// @NPC_SUBBANK_FLAG_UNINIT: Subbank is not initialized.
// @NPC_SUBBANK_FLAG_FREE: Subbank is free.
// @NPC_SUBBANK_FLAG_USED: Subbank is used.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_subbank_flag {
    NPC_SUBBANK_FLAG_UNINIT,
    NPC_SUBBANK_FLAG_FREE = BIT(0),
    NPC_SUBBANK_FLAG_USED = BIT(1),
}

//
// enum npc_dft_rule_id - Default rule type
//
// Mcam default rule type.
//
// @NPC_DFT_RULE_START_ID:	Not used
// @NPC_DFT_RULE_PROMISC_ID:	promiscuous rule
// @NPC_DFT_RULE_MCAST_ID:	multicast rule
// @NPC_DFT_RULE_BCAST_ID:	broadcast rule
// @NPC_DFT_RULE_UCAST_ID:	unicast rule
// @NPC_DFT_RULE_MAX_ID:	Maximum index.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_dft_rule_id {
    NPC_DFT_RULE_START_ID = 1,
    NPC_DFT_RULE_PROMISC_ID = NPC_DFT_RULE_START_ID,
    NPC_DFT_RULE_MCAST_ID,
    NPC_DFT_RULE_BCAST_ID,
    NPC_DFT_RULE_UCAST_ID,
    NPC_DFT_RULE_MAX_ID,
}

//
// struct npc_subbank - Subbank fields.
// @b0b:	Subbanks bottom index for bank0
// @b1b:	Subbanks bottom index for bank1
// @b0t:	Subbanks top index for bank0
// @b1t:	Subbanks top index for bank1
// @flags:	Subbank flags
// @lock:	Mutex lock for flags and rsrc mofiication
// @b0map:	Bitmap map for bank0 indexes
// @b1map:	Bitmap map for bank1 indexes
// @idx:	Subbank index
// @arr_idx:	Index to the free array or used array
// @free_cnt:	Number of free slots in the subbank.
// @key_type:	X4 or X2 subbank.
//
// MCAM resource is divided horizontally into multiple subbanks and
// Resource allocation from each subbank is managed by this data
// structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_subbank {
    pub b1b: u16 b0t, b0b, b1t,,
    pub flags: npc_subbank_flag,
    pub /: *mut *mut mutex lock; / Protect subbank resources,
    pub MAX_SUBBANK_DEPTH): DECLARE_BITMAP(b0map,,
    pub MAX_SUBBANK_DEPTH): DECLARE_BITMAP(b1map,,
    pub idx: u16,
    pub arr_idx: u16,
    pub free_cnt: u16,
    pub key_type: u8,
}

//
// struct npc_defrag_show_node - Defragmentation show node
// @old_midx:	Old mcam index.
// @new_midx:	New mcam index.
// @vidx:	Virtual index
// @list:	Linked list of these nodes
//
// This structure holds information on last defragmentation
// executed on mcam resource.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_defrag_show_node {
    pub old_midx: u16,
    pub new_midx: u16,
    pub vidx: u16,
    pub list: list_head,
}

//
// struct npc_priv_t - NPC private structure.
// @bank_depth:		Total entries in each bank.
// @num_banks:		Number of banks.
// @num_subbanks:	Number of subbanks.
// @subbank_depth:	Depth of subbank.
// @en_map:		Enable/disable status.
// @kw:			Kex configured key type.
// @sb:			Subbank array.
// @xa_sb_used:		Array of used subbanks.
// @xa_sb_free:		Array of free subbanks.
// @xa_pf2idx_map:	PF to mcam index map.
// @xa_idx2pf_map:	Mcam index to PF map.
// @xa_pf_map:		Pcifunc to index map.
// @pf_cnt:		Number of PFs.
// @xa_pf2dfl_rmap:	PF to default rule index map.
// @xa_idx2vidx_map:	Mcam index to virtual index map.
// @xa_vidx2idx_map:	virtual index to mcam index map.
// @defrag_lh:		Defrag list head.
// @lock:		Lock for defrag list
//
// This structure is populated during probing time by reading
// HW csr registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_priv_t {
    pub bank_depth: c_int,
    pub num_banks: c_int,
    pub num_subbanks: c_int,
    pub subbank_depth: c_int,
    pub kw: u8,
    pub sb: *mut npc_subbank,
    pub xa_sb_used: xarray,
    pub xa_sb_free: xarray,
    pub xa_pf2idx_map: *mut xarray,
    pub xa_idx2pf_map: xarray,
    pub xa_pf_map: xarray,
    pub xa_pf2dfl_rmap: xarray,
    pub xa_idx2vidx_map: xarray,
    pub xa_vidx2idx_map: xarray,
    pub defrag_lh: list_head,
    pub /: *mut *mut mutex lock; / protect defrag nodes,
    pub pf_cnt: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_kpm_action0 {

    pub 7: u64 rsvd_63_57 :,
    pub 3: u64 byp_count :,
    pub 1: u64 capture_ena :,
    pub 1: u64 parse_done :,
    pub 8: u64 next_state :,
    pub 1: u64 rsvd_43 :,
    pub 3: u64 capture_lid :,
    pub 4: u64 capture_ltype :,
    pub 4: u64 rsvd_32_35 :,
    pub 4: u64 capture_flags :,
    pub 8: u64 ptr_advance :,
    pub 8: u64 var_len_offset :,
    pub 8: u64 var_len_mask :,
    pub 1: u64 var_len_right :,
    pub 3: u64 var_len_shift :,

    pub 3: u64 var_len_shift :,
    pub 1: u64 var_len_right :,
    pub 8: u64 var_len_mask :,
    pub 8: u64 var_len_offset :,
    pub 8: u64 ptr_advance :,
    pub 4: u64 capture_flags :,
    pub 4: u64 rsvd_32_35 :,
    pub 4: u64 capture_ltype :,
    pub 3: u64 capture_lid :,
    pub 1: u64 rsvd_43 :,
    pub 8: u64 next_state :,
    pub 1: u64 parse_done :,
    pub 1: u64 capture_ena :,
    pub 3: u64 byp_count :,
    pub 7: u64 rsvd_63_57 :,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_kex_extr {
// MKEX Profle Header
    pub /: *mut *mut u64 mkex_sign; / "mcam-kex-profile" (8 bytes/ASCII characters),
    pub /: *mut *mut u8 name[MKEX_NAME_LEN]; / MKEX Profile name,
    pub /: *mut *mut u64 cpu_model; / Format as profiled by CPU hardware,
    pub /: *mut *mut u64 kpu_version; / KPU firmware/profile version,
    pub /: *mut *mut u64 reserved; / Reserved for extension,
// MKEX Profle Data
    pub /: *mut *mut u64 keyx_cfg[NPC_MAX_INTF]; / NPC_AF_INTF(0..1)_KEX_CFG,
pub const NPC_MAX_EXTRACTOR: c_int = 24;
// MKEX Extractor data
    pub intf_extr_lid: [u64; NPC_MAX_INTF][NPC_MAX_EXTRACTOR],
// KEX configuration per extractor
    pub intf_extr_lt: [u64; NPC_MAX_INTF][NPC_MAX_EXTRACTOR][NPC_MAX_LT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_cn20k_kpu_profile_fwdata {
pub const KPU_SIGN: c_uint = 0x00666f727075706b;
pub const KPU_NAME_LEN: c_int = 32;
// Maximum number of custom KPU entries supported by
// the built-in profile.
//
pub const KPU_CN20K_MAX_CST_ENT: c_int = 6;
// KPU Profle Header
    pub /: *mut *mut __le64 signature; / "kpuprof\0" (8 bytes/ASCII characters),
    pub /: *mut *mut u8 name[KPU_NAME_LEN]; / KPU Profile name,
    pub /: *mut *mut __le64 version; / KPU profile version,
    pub kpus: u8,
    pub reserved: [u8; 7],
// Default MKEX profile to be used with this KPU profile. May be
// overridden with mkex_profile module parameter.
// Format is same as for the MKEX profile to streamline processing.
//
    pub mkex: npc_mcam_kex_extr,
// LTYPE values for specific HW offloaded protocols.
    pub lt_def: npc_lt_def_cfg,
// Dynamically sized data:
// Custom KPU CAM and ACTION configuration entries.
// struct npc_kpu_fwdata kpu[kpus];
//
    pub data: [u8; ],
    pub __packed: },
    pub rvu: struct,
    pub npc_priv_get(void): *mut npc_priv_t,
    pub rvu): *mut int npc_cn20k_init(struct rvu,
    pub rvu): *mut void npc_cn20k_deinit(struct rvu,
    pub sb_free): *mut *mut int x4_free, int,
    pub virt): bool contig, int count, bool,
    pub count): *mut *mut *mut int npc_cn20k_idx_free(struct rvu rvu, u16 mcam_idx, int,
    pub blkaddr): *mut *mut void npc_cn20k_parser_profile_init(struct rvu rvu, int,
    pub npc_mkex_extr_default_get(void): *mut npc_mcam_kex_extr,
    pub mkex_profile): *const c_char,
    pub profile): *mut npc_kpu_profile_adapter,
    pub pfl): *mut npc_kpu_profile_adapter,
    pub pcifunc): *mut *mut int npc_cn20k_dft_rules_alloc(struct rvu rvu, u16,
    pub pcifunc): *mut *mut void npc_cn20k_dft_rules_free(struct rvu rvu, u16,
    pub ucast): *mut *mut *mut u16 mcast, u16 promisc, u16,
    pub req_kw_type): bool enable, u8 hw_prio, u8,
    pub enable): int index, bool,
    pub dest): u16 src, u16,
    pub hw_prio): *mut *mut u8 ena, u8,
    pub index): *mut *mut int npc_cn20k_clear_mcam_entry(struct rvu rvu, int blkaddr, int,
    pub key_type): *mut *mut int npc_mcam_idx_2_key_type(struct rvu rvu, u16 mcam_idx, u8,
    pub index): u16 npc_cn20k_vidx2idx(u16,
    pub idx): u16 npc_cn20k_idx2vidx(u16,
    pub rvu): *mut int npc_cn20k_defrag(struct rvu,
    pub pcifunc): *mut *mut bool npc_is_cgx_or_lbk(struct rvu rvu, u16,
    pub sb_off): *mut c_int,
    pub sz): *const *const *const u32 npc_cn20k_search_order_get(bool restricted_order, u32,
    pub cnt): c_int,
