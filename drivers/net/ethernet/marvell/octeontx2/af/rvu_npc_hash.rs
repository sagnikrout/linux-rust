//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rvu_npc_hash.h
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
// Copyright (C) 2022 Marvell.
//
pub const RVU_NPC_HASH_SECRET_KEY0: c_uint = 0xa9d5af4c9fbc76b1;
pub const RVU_NPC_HASH_SECRET_KEY1: c_uint = 0xa9d5af4c9fbc87b4;
pub const RVU_NPC_HASH_SECRET_KEY2: c_uint = 0x5954c9e7;
pub const NPC_MAX_HASH: c_int = 2;
pub const NPC_MAX_HASH_MASK: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_mcam_kex_hash {
// NPC_AF_INTF(0..1)_LID(0..7)_LT(0..15)_LD(0..1)_CFG
    pub lid_lt_ld_hash_en: [bool; NPC_MAX_INTF][NPC_MAX_LID][NPC_MAX_LT][NPC_MAX_LD],
// NPC_AF_INTF(0..1)_HASH(0..1)_CFG
    pub hash: [u64; NPC_MAX_INTF][NPC_MAX_HASH],
// NPC_AF_INTF(0..1)_HASH(0..1)_MASK(0..1)
    pub hash_mask: [u64; NPC_MAX_INTF][NPC_MAX_HASH][NPC_MAX_HASH_MASK],
// NPC_AF_INTF(0..1)_HASH(0..1)_RESULT_CTRL
    pub hash_ctrl: [u64; NPC_MAX_INTF][NPC_MAX_HASH],
    pub __packed: },
    pub omask): *mut flow_msg,
    pub blkaddr): *mut *mut void npc_config_secret_key(struct rvu rvu, int,
    pub blkaddr): *mut *mut void npc_program_mkex_hash(struct rvu rvu, int,
    pub hash_idx): u8 intf, u8,
}

// If exact match table support is enabled, enable drop rules
pub const NPC_MCAM_DROP_RULE_MAX: c_int = 30;
pub const NPC_MCAM_SDP_DROP_RULE_IDX: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npc_exact_opc_type {
    NPC_EXACT_OPC_MEM,
    NPC_EXACT_OPC_CAM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_exact_table_entry {
    pub list: list_head,
    pub glist: list_head,
    pub /: *mut *mut u32 seq_id; / Sequence number of entry,
    pub /: *mut *mut u32 index; / Mem table or cam table index,
    pub mcam_idx: u32,
// Mcam index. This is valid only if "cmd" field is false
    pub opc_type: npc_exact_opc_type,
    pub chan: u16,
    pub pcifunc: u16,
    pub ways: u8,
    pub mac: [u8; ETH_ALEN],
    pub ctype: u8,
    pub cgx_id: u8,
    pub lmac_id: u8,
    pub /: *mut *mut bool cmd; / Is added by ethtool command ?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npc_exact_table {
    pub /: *mut *mut mutex lock; / entries update lock,
    pub id_bmap: *mut c_ulong,
    pub num_drop_rules: c_int,
    pub tot_ids: u32,
    pub cnt_cmd_rules: [u16; NPC_MCAM_DROP_RULE_MAX],
    pub counter_idx: [u16; NPC_MCAM_DROP_RULE_MAX],
    pub promisc_mode: [bool; NPC_MCAM_DROP_RULE_MAX],
    pub ways: c_int,
    pub depth: c_int,
    pub bmap: *mut c_ulong,
    pub calculation.: u64 mask; // Masks before hash,
    pub mask: u16 hash_mask; // 11 bits for hash,
    pub offset: u16 hash_offset; // 11 bits,
    pub mem_table: },
    pub depth: c_int,
    pub bmap: *mut c_ulong,
    pub cam_table: },
    pub valid: bool,
    pub chan_val: u16,
    pub chan_mask: u16,
    pub pcifunc: u16,
    pub drop_rule_idx: u8,
    pub drop_rule_map: [}; NPC_MCAM_DROP_RULE_MAX],
pub const NPC_EXACT_TBL_MAX_WAYS: c_int = 4;
    pub lhead_mem_tbl_entry: [list_head; NPC_EXACT_TBL_MAX_WAYS],
    pub mem_tbl_entry_cnt: c_int,
    pub lhead_cam_tbl_entry: list_head,
    pub cam_tbl_entry_cnt: c_int,
    pub lhead_gbl: list_head,
}

extern "C" {
    pub fn rvu_npc_exact_has_match_table(rvu: *mut rvu) -> bool;
}
extern "C" {
    pub fn rvu_npc_exact_get_max_entries(rvu: *mut rvu) -> u32;
}
extern "C" {
    pub fn rvu_npc_exact_init(rvu: *mut rvu) -> c_int;
}
extern "C" {
    pub fn rvu_npc_exact_reset(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_npc_exact_can_disable_feature(rvu: *mut rvu) -> bool;
}
extern "C" {
    pub fn rvu_npc_exact_disable_feature(rvu: *mut rvu);
}
extern "C" {
    pub fn rvu_npc_exact_reset(rvu: *mut rvu, pcifunc: u16);
}
extern "C" {
    pub fn rvu_npc_exact_drop_rule_to_pcifunc(rvu: *mut rvu, drop_rule_idx: u32) -> u16;
}
extern "C" {
    pub fn rvu_npc_exact_promisc_disable(rvu: *mut rvu, pcifunc: u16) -> c_int;
}
extern "C" {
    pub fn rvu_npc_exact_promisc_enable(rvu: *mut rvu, pcifunc: u16) -> c_int;
}
