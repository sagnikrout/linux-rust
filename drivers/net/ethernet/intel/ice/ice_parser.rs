//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_parser.h
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
// Copyright (C) 2024 Intel Corporation
pub const ICE_SEC_DATA_OFFSET: c_int = 4;
pub const ICE_SID_RXPARSER_IMEM_ENTRY_SIZE: c_int = 48;
pub const ICE_SID_RXPARSER_METADATA_INIT_ENTRY_SIZE: c_int = 24;
pub const ICE_SID_RXPARSER_CAM_ENTRY_SIZE: c_int = 16;
pub const ICE_SID_RXPARSER_PG_SPILL_ENTRY_SIZE: c_int = 17;
pub const ICE_SID_RXPARSER_NOMATCH_CAM_ENTRY_SIZE: c_int = 12;
pub const ICE_SID_RXPARSER_NOMATCH_SPILL_ENTRY_SIZE: c_int = 13;
pub const ICE_SID_RXPARSER_BOOST_TCAM_ENTRY_SIZE: c_int = 88;
pub const ICE_SID_RXPARSER_MARKER_TYPE_ENTRY_SIZE: c_int = 24;
pub const ICE_SID_RXPARSER_MARKER_GRP_ENTRY_SIZE: c_int = 8;
pub const ICE_SID_RXPARSER_PROTO_GRP_ENTRY_SIZE: c_int = 24;
pub const ICE_SID_RXPARSER_FLAG_REDIR_ENTRY_SIZE: c_int = 1;
pub const ICE_SEC_LBL_DATA_OFFSET: c_int = 2;
pub const ICE_SID_LBL_ENTRY_SIZE: c_int = 66;
// ICE_SID_RXPARSER_IMEM section
pub const ICE_IMEM_TABLE_SIZE: c_int = 192;
// TCAM boost Master; if bit is set, and TCAM hit, TCAM output overrides iMEM
// output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bst_main {
    pub alu0: bool,
    pub alu1: bool,
    pub alu2: bool,
    pub pg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bst_keybuilder {
    pub /: *mut *mut u8 prio; / 0-3: PG precedence within ALUs (3 highest),
    pub /: *mut *mut bool tsr_ctrl; / TCAM Search Register control,
}

// Next protocol Key builder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_np_keybuilder {
    pub opc: u8,
    pub start_reg0: u8,
    pub len_reg1: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_np_keybuilder_opcode {
    ICE_NPKB_OPC_EXTRACT	= 0,
    ICE_NPKB_OPC_BUILD	= 1,
    ICE_NPKB_OPC_BYPASS	= 2,
}

// Parse Graph Key builder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_keybuilder {
    pub flag0_ena: bool,
    pub flag1_ena: bool,
    pub flag2_ena: bool,
    pub flag3_ena: bool,
    pub flag0_idx: u8,
    pub flag1_idx: u8,
    pub flag2_idx: u8,
    pub flag3_idx: u8,
    pub alu_reg_idx: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_alu_idx {
    ICE_ALU0_IDX	= 0,
    ICE_ALU1_IDX	= 1,
    ICE_ALU2_IDX	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_alu_opcode {
    ICE_ALU_PARK	= 0,
    ICE_ALU_MOV_ADD	= 1,
    ICE_ALU_ADD	= 2,
    ICE_ALU_MOV_AND	= 4,
    ICE_ALU_AND	= 5,
    ICE_ALU_AND_IMM	= 6,
    ICE_ALU_MOV_OR	= 7,
    ICE_ALU_OR	= 8,
    ICE_ALU_MOV_XOR	= 9,
    ICE_ALU_XOR	= 10,
    ICE_ALU_NOP	= 11,
    ICE_ALU_BR	= 12,
    ICE_ALU_BREQ	= 13,
    ICE_ALU_BRNEQ	= 14,
    ICE_ALU_BRGT	= 15,
    ICE_ALU_BRLT	= 16,
    ICE_ALU_BRGEQ	= 17,
    ICE_ALU_BRLEG	= 18,
    ICE_ALU_SETEQ	= 19,
    ICE_ALU_ANDEQ	= 20,
    ICE_ALU_OREQ	= 21,
    ICE_ALU_SETNEQ	= 22,
    ICE_ALU_ANDNEQ	= 23,
    ICE_ALU_ORNEQ	= 24,
    ICE_ALU_SETGT	= 25,
    ICE_ALU_ANDGT	= 26,
    ICE_ALU_ORGT	= 27,
    ICE_ALU_SETLT	= 28,
    ICE_ALU_ANDLT	= 29,
    ICE_ALU_ORLT	= 30,
    ICE_ALU_MOV_SUB	= 31,
    ICE_ALU_SUB	= 32,
    ICE_ALU_INVALID	= 64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_proto_off_opcode {
    ICE_PO_OFF_REMAIN	= 0,
    ICE_PO_OFF_HDR_ADD	= 1,
    ICE_PO_OFF_HDR_SUB	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_alu {
    pub opc: ice_alu_opcode,
    pub src_start: u8,
    pub src_len: u8,
    pub shift_xlate_sel: bool,
    pub shift_xlate_key: u8,
    pub src_reg_id: u8,
    pub dst_reg_id: u8,
    pub inc0: bool,
    pub inc1: bool,
    pub proto_offset_opc: u8,
    pub proto_offset: u8,
    pub branch_addr: u8,
    pub imm: u16,
    pub dedicate_flags_ena: bool,
    pub dst_start: u8,
    pub dst_len: u8,
    pub flags_extr_imm: bool,
    pub flags_start_imm: u8,
}

// Parser program code (iMEM)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_imem_item {
    pub idx: u16,
    pub b_m: ice_bst_main,
    pub b_kb: ice_bst_keybuilder,
    pub pg_prio: u8,
    pub np_kb: ice_np_keybuilder,
    pub pg_kb: ice_pg_keybuilder,
    pub alu0: ice_alu,
    pub alu1: ice_alu,
    pub alu2: ice_alu,
}

// ICE_SID_RXPARSER_METADATA_INIT section
pub const ICE_METAINIT_TABLE_SIZE: c_int = 16;
// Metadata Initialization item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_metainit_item {
    pub idx: u16,
    pub /: *mut *mut u8 tsr; / TCAM Search key Register,
    pub /: *mut *mut u16 ho; / Header Offset register,
    pub /: *mut *mut u16 pc; / Program Counter register,
    pub /: *mut *mut u16 pg_rn; / Parse Graph Root Node,
    pub /: *mut *mut u8 cd; / Control Domain ID,
// General Purpose Registers
    pub gpr_a_ctrl: bool,
    pub gpr_a_data_mdid: u8,
    pub gpr_a_data_start: u8,
    pub gpr_a_data_len: u8,
    pub gpr_a_id: u8,
    pub gpr_b_ctrl: bool,
    pub gpr_b_data_mdid: u8,
    pub gpr_b_data_start: u8,
    pub gpr_b_data_len: u8,
    pub gpr_b_id: u8,
    pub gpr_c_ctrl: bool,
    pub gpr_c_data_mdid: u8,
    pub gpr_c_data_start: u8,
    pub gpr_c_data_len: u8,
    pub gpr_c_id: u8,
    pub gpr_d_ctrl: bool,
    pub gpr_d_data_mdid: u8,
    pub gpr_d_data_start: u8,
    pub gpr_d_data_len: u8,
    pub gpr_d_id: u8,
    pub /: *mut *mut u64 flags; / Initial value for all flags,
}

// ICE_SID_RXPARSER_CAM, ICE_SID_RXPARSER_PG_SPILL,
// ICE_SID_RXPARSER_NOMATCH_CAM and ICE_SID_RXPARSER_NOMATCH_CAM
// sections
pub const ICE_PG_CAM_TABLE_SIZE: c_int = 2048;
pub const ICE_PG_SP_CAM_TABLE_SIZE: c_int = 128;
pub const ICE_PG_NM_CAM_TABLE_SIZE: c_int = 1024;
pub const ICE_PG_NM_SP_CAM_TABLE_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_cam_key {
    pub valid: bool,
    pub /: *mut *mut u16 node_id; / Node ID of protocol in parse graph,
    pub flag0: bool,
    pub flag1: bool,
    pub flag2: bool,
    pub flag3: bool,
    pub /: *mut *mut u8 boost_idx; / Boost TCAM match index,
    pub alu_reg: u16,
    pub /: *mut *mut u32 next_proto; / next Protocol value (must be last),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_nm_cam_key {
    pub valid: bool,
    pub node_id: u16,
    pub flag0: bool,
    pub flag1: bool,
    pub flag2: bool,
    pub flag3: bool,
    pub boost_idx: u8,
    pub alu_reg: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_cam_action {
    pub /: *mut *mut u16 next_node; / Parser Node ID for the next round,
    pub /: *mut *mut u8 next_pc; / next Program Counter,
    pub /: *mut *mut bool is_pg; / is protocol group,
    pub /: *mut *mut u8 proto_id; / protocol ID or proto group ID,
    pub /: *mut *mut bool is_mg; / is marker group,
    pub /: *mut *mut u8 marker_id; / marker ID or marker group ID,
    pub is_last_round: bool,
    pub /: *mut *mut bool ho_polarity; / header offset polarity,
    pub ho_inc: u16,
}

// Parse Graph item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_cam_item {
    pub idx: u16,
    pub key: ice_pg_cam_key,
    pub action: ice_pg_cam_action,
}

// Parse Graph No Match item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pg_nm_cam_item {
    pub idx: u16,
    pub key: ice_pg_nm_cam_key,
    pub action: ice_pg_cam_action,
}

// ICE_SID_RXPARSER_BOOST_TCAM and ICE_SID_LBL_RXPARSER_TMEM sections
pub const ICE_BST_TCAM_TABLE_SIZE: c_int = 256;
pub const ICE_BST_TCAM_KEY_SIZE: c_int = 20;
// Boost TCAM item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_bst_tcam_item {
    pub addr: u16,
    pub key: [u8; ICE_BST_TCAM_KEY_SIZE],
    pub key_inv: [u8; ICE_BST_TCAM_KEY_SIZE],
    pub hit_idx_grp: u8,
    pub pg_prio: u8,
    pub np_kb: ice_np_keybuilder,
    pub pg_kb: ice_pg_keybuilder,
    pub alu0: ice_alu,
    pub alu1: ice_alu,
    pub alu2: ice_alu,
}

pub const ICE_LBL_LEN: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_lbl_type {
    ICE_LBL_BST_TYPE_UNKNOWN,
    ICE_LBL_BST_TYPE_DVM,
    ICE_LBL_BST_TYPE_SVM,
    ICE_LBL_BST_TYPE_VXLAN,
    ICE_LBL_BST_TYPE_GENEVE,
    ICE_LBL_BST_TYPE_UDP_ECPRI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_lbl_item {
    pub idx: u16,
    pub label: [c_char; ICE_LBL_LEN],
// must be at the end, not part of the DDP section
    pub type: ice_lbl_type,
}

// ICE_SID_RXPARSER_MARKER_PTYPE section
pub const ICE_PTYPE_MK_TCAM_TABLE_SIZE: c_int = 1024;
pub const ICE_PTYPE_MK_TCAM_KEY_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_ptype_mk_tcam_item {
    pub address: u16,
    pub ptype: u16,
    pub key: [u8; ICE_PTYPE_MK_TCAM_KEY_SIZE],
    pub key_inv: [u8; ICE_PTYPE_MK_TCAM_KEY_SIZE],
    pub __packed: },
    pub len): *mut *mut u8 pat, int,
// ICE_SID_RXPARSER_MARKER_GRP section
pub const ICE_MK_GRP_TABLE_SIZE: c_int = 128;
pub const ICE_MK_COUNT_PER_GRP: c_int = 8;
// Marker Group item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_mk_grp_item {
    pub idx: c_int,
    pub markers: [u8; ICE_MK_COUNT_PER_GRP],
}

// ICE_SID_RXPARSER_PROTO_GRP section
pub const ICE_PROTO_COUNT_PER_GRP: c_int = 8;
pub const ICE_PROTO_GRP_TABLE_SIZE: c_int = 192;
pub const ICE_PROTO_GRP_ITEM_SIZE: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_proto_off {
    pub /: *mut *mut bool polarity; / true: positive, false: negative,
    pub proto_id: u8,
    pub /: *mut *mut u16 offset; / 10 bit protocol offset,
}

// Protocol Group item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_proto_grp_item {
    pub idx: u16,
    pub po: [ice_proto_off; ICE_PROTO_COUNT_PER_GRP],
}

// ICE_SID_RXPARSER_FLAG_REDIR section
pub const ICE_FLG_RD_TABLE_SIZE: c_int = 64;
pub const ICE_FLG_RDT_SIZE: c_int = 64;
// Flags Redirection item
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_flg_rd_item {
    pub idx: u16,
    pub expose: bool,
    pub /: *mut *mut u8 intr_flg_id; / Internal Flag ID,
}

extern "C" {
    pub fn ice_flg_redirect(table: *mut ice_flg_rd_item, psr_flg: u64) -> u64;
}
// ICE_SID_XLT_KEY_BUILDER_SW, ICE_SID_XLT_KEY_BUILDER_ACL,
// ICE_SID_XLT_KEY_BUILDER_FD and ICE_SID_XLT_KEY_BUILDER_RSS
// sections
pub const ICE_XLT_KB_FLAG0_14_CNT: c_int = 15;
pub const ICE_XLT_KB_TBL_CNT: c_int = 8;
pub const ICE_XLT_KB_TBL_ENTRY_SIZE: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt_kb_entry {
    pub xlt1_ad_sel: u8,
    pub xlt2_ad_sel: u8,
    pub flg0_14_sel: [u16; ICE_XLT_KB_FLAG0_14_CNT],
    pub xlt1_md_sel: u8,
    pub xlt2_md_sel: u8,
}

// XLT Key Builder
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_xlt_kb {
    pub /: *mut *mut u8 xlt1_pm; / XLT1 Partition Mode,
    pub /: *mut *mut u8 xlt2_pm; / XLT2 Partition Mode,
    pub /: *mut *mut u8 prof_id_pm; / Profile ID Partition Mode,
    pub flag15: u64,
    pub entries: [ice_xlt_kb_entry; ICE_XLT_KB_TBL_CNT],
}

extern "C" {
    pub fn ice_xlt_kb_flag_get(kb: *mut ice_xlt_kb, pkt_flag: u64) -> u16;
}
// Parser API
pub const ICE_GPR_HV_IDX: c_int = 64;
pub const ICE_GPR_HV_SIZE: c_int = 32;
pub const ICE_GPR_ERR_IDX: c_int = 84;
pub const ICE_GPR_FLG_IDX: c_int = 104;
pub const ICE_GPR_FLG_SIZE: c_int = 16;

pub const ICE_PARSER_MAX_PKT_LEN: c_int = 504;
pub const ICE_PARSER_PKT_REV: c_int = 32;
pub const ICE_PARSER_GPR_NUM: c_int = 128;
pub const ICE_PARSER_FLG_NUM: c_int = 64;
pub const ICE_PARSER_ERR_NUM: c_int = 16;
pub const ICE_MARKER_ID_SIZE: c_int = 9;

pub const ICE_MARKER_ID_NUM: c_int = 8;
pub const ICE_PO_PAIR_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_gpr_pu {
// array of flags to indicate if GRP needs to be updated
    pub gpr_val_upd: [bool; ICE_PARSER_GPR_NUM],
    pub gpr_val: [u16; ICE_PARSER_GPR_NUM],
    pub flg_msk: u64,
    pub flg_val: u64,
    pub err_msk: u16,
    pub err_val: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_pg_prio {
    ICE_PG_P0	= 0,
    ICE_PG_P1	= 1,
    ICE_PG_P2	= 2,
    ICE_PG_P3	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser_rt {
    pub psr: *mut ice_parser,
    pub gpr: [u16; ICE_PARSER_GPR_NUM],
    pub ICE_PARSER_PKT_REV]: u8 pkt_buf[ICE_PARSER_MAX_PKT_LEN +,
    pub pkt_len: u16,
    pub po: u16,
    pub bst_key: [u8; ICE_BST_TCAM_KEY_SIZE],
    pub pg_key: ice_pg_cam_key,
    pub pg_prio: u8,
    pub alu0: *mut ice_alu,
    pub alu1: *mut ice_alu,
    pub alu2: *mut ice_alu,
    pub action: *mut ice_pg_cam_action,
    pub pu: ice_gpr_pu,
    pub markers: [u8; ICE_MARKER_ID_SIZE],
    pub protocols: [bool; ICE_PO_PAIR_SIZE],
    pub offsets: [u16; ICE_PO_PAIR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser_proto_off {
    pub /: *mut *mut u8 proto_id; / hardware protocol ID,
    pub /: *mut *mut u16 offset; / offset from the start of the protocol header,
}

pub const ICE_PARSER_PROTO_OFF_PAIR_SIZE: c_int = 16;
pub const ICE_PARSER_FLAG_PSR_SIZE: c_int = 8;
pub const ICE_PARSER_FV_SIZE: c_int = 48;
pub const ICE_PARSER_FV_MAX: c_int = 24;
pub const ICE_BT_TUN_PORT_OFF_H: c_int = 16;
pub const ICE_BT_TUN_PORT_OFF_L: c_int = 15;
pub const ICE_BT_VM_OFF: c_int = 0;
pub const ICE_UDP_PORT_OFF_H: c_int = 1;
pub const ICE_UDP_PORT_OFF_L: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser_result {
    pub /: *mut *mut u16 ptype; / 16 bits hardware PTYPE,
// array of protocol and header offset pairs
    pub po: [ice_parser_proto_off; ICE_PARSER_PROTO_OFF_PAIR_SIZE],
    pub /: *mut *mut int po_num; / # of protocol-offset pairs must <= 16,
    pub /: *mut *mut u64 flags_psr; / parser flags,
    pub /: *mut *mut u64 flags_pkt; / packet flags,
    pub /: *mut *mut u16 flags_sw; / key builder flags for SW,
    pub /: *mut *mut u16 flags_acl; / key builder flags for ACL,
    pub /: *mut *mut u16 flags_fd; / key builder flags for FD,
    pub /: *mut *mut u16 flags_rss; / key builder flags for RSS,
}

extern "C" {
    pub fn ice_parser_rt_reset(rt: *mut ice_parser_rt);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser {
    pub /: *mut *mut *mut ice_hw hw; / pointer to the hardware structure,
    pub imem_table: *mut ice_imem_item,
    pub mi_table: *mut ice_metainit_item,
    pub pg_cam_table: *mut ice_pg_cam_item,
    pub pg_sp_cam_table: *mut ice_pg_cam_item,
    pub pg_nm_cam_table: *mut ice_pg_nm_cam_item,
    pub pg_nm_sp_cam_table: *mut ice_pg_nm_cam_item,
    pub bst_tcam_table: *mut ice_bst_tcam_item,
    pub bst_lbl_table: *mut ice_lbl_item,
    pub ptype_mk_tcam_table: *mut ice_ptype_mk_tcam_item,
    pub mk_grp_table: *mut ice_mk_grp_item,
    pub proto_grp_table: *mut ice_proto_grp_item,
    pub flg_rd_table: *mut ice_flg_rd_item,
    pub xlt_kb_sw: *mut ice_xlt_kb,
    pub xlt_kb_acl: *mut ice_xlt_kb,
    pub xlt_kb_fd: *mut ice_xlt_kb,
    pub xlt_kb_rss: *mut ice_xlt_kb,
    pub rt: ice_parser_rt,
}

extern "C" {
    pub fn ice_parser_destroy(psr: *mut ice_parser);
}
extern "C" {
    pub fn ice_parser_dvm_set(psr: *mut ice_parser, on: bool);
}
extern "C" {
    pub fn ice_parser_vxlan_tunnel_set(psr: *mut ice_parser, udp_port: u16, on: bool) -> c_int;
}
extern "C" {
    pub fn ice_parser_geneve_tunnel_set(psr: *mut ice_parser, udp_port: u16, on: bool) -> c_int;
}
extern "C" {
    pub fn ice_parser_ecpri_tunnel_set(psr: *mut ice_parser, udp_port: u16, on: bool) -> c_int;
}
extern "C" {
    pub fn ice_parser_result_dump(hw: *mut ice_hw, rslt: *mut ice_parser_result);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser_fv {
    pub /: *mut *mut u8 proto_id; / hardware protocol ID,
    pub /: *mut *mut u16 offset; / offset from the start of the protocol header,
    pub /: *mut *mut u16 spec; / pattern to match,
    pub /: *mut *mut u16 msk; / pattern mask,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_parser_profile {
// array of field vectors
    pub fv: [ice_parser_fv; ICE_PARSER_FV_SIZE],
    pub /: *mut *mut int fv_num; / # of field vectors must <= 48,
    pub /: *mut *mut u16 flags; / key builder flags,
    pub /: *mut *mut u16 flags_msk; / key builder flag mask,
    pub /: *mut *mut DECLARE_BITMAP(ptypes, ICE_FLOW_PTYPE_MAX); / PTYPE bitmap,
}
