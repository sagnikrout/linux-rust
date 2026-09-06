//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/cn20k/struct.h
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
// Copyright (C) 2024 Marvell.
//
pub const NIX_MAX_CTX_SIZE: c_int = 128;
//
// CN20k RVU PF MBOX Interrupt Vector Enumeration
//
// Vectors 0 - 3 are compatible with pre cn20k and hence
// existing macros are being reused.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_mbox_pf_int_vec_e {
    RVU_MBOX_PF_INT_VEC_VFPF_MBOX0	= 0x4,
    RVU_MBOX_PF_INT_VEC_VFPF_MBOX1	= 0x5,
    RVU_MBOX_PF_INT_VEC_VFPF1_MBOX0	= 0x6,
    RVU_MBOX_PF_INT_VEC_VFPF1_MBOX1	= 0x7,
    RVU_MBOX_PF_INT_VEC_AFPF_MBOX	= 0x8,
    RVU_MBOX_PF_INT_VEC_CNT		= 0x9,
}

// RVU Admin function Interrupt Vector Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_af_cn20k_int_vec_e {
    RVU_AF_CN20K_INT_VEC_POISON		= 0x0,
    RVU_AF_CN20K_INT_VEC_PFFLR0		= 0x1,
    RVU_AF_CN20K_INT_VEC_PFFLR1		= 0x2,
    RVU_AF_CN20K_INT_VEC_PFME0		= 0x3,
    RVU_AF_CN20K_INT_VEC_PFME1		= 0x4,
    RVU_AF_CN20K_INT_VEC_GEN		= 0x5,
    RVU_AF_CN20K_INT_VEC_PFAF_MBOX0		= 0x6,
    RVU_AF_CN20K_INT_VEC_PFAF_MBOX1		= 0x7,
    RVU_AF_CN20K_INT_VEC_PFAF1_MBOX0	= 0x8,
    RVU_AF_CN20K_INT_VEC_PFAF1_MBOX1	= 0x9,
    RVU_AF_CN20K_INT_VEC_CNT		= 0xa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn20k_sq_ctx_s {
    pub /: *mut *mut u64 ena : 1; / W0,
    pub 6: u64 qint_idx :,
    pub 20: u64 substream :,
    pub 1: u64 sdp_mcast :,
    pub 20: u64 cq :,
    pub 16: u64 sqe_way_mask :,
    pub /: *mut *mut u64 smq : 11; / W1,
    pub 1: u64 cq_ena :,
    pub 1: u64 xoff :,
    pub 1: u64 sso_ena :,
    pub 14: u64 smq_rr_weight :,
    pub 12: u64 default_chan :,
    pub 16: u64 sqb_count :,
    pub 1: u64 reserved_120_120 :,
    pub 7: u64 smq_rr_count_lb :,
    pub /: *mut *mut u64 smq_rr_count_ub : 25; / W2,
    pub 20: u64 sqb_aura :,
    pub 8: u64 sq_int :,
    pub 8: u64 sq_int_ena :,
    pub 2: u64 sqe_stype :,
    pub 1: u64 reserved_191_191 :,
    pub /: *mut *mut u64 max_sqe_size : 2; / W3,
    pub 8: u64 cq_limit :,
    pub 1: u64 lmt_dis :,
    pub 1: u64 mnq_dis :,
    pub 20: u64 smq_next_sq :,
    pub 8: u64 smq_lso_segnum :,
    pub 6: u64 tail_offset :,
    pub 6: u64 smenq_offset :,
    pub 6: u64 head_offset :,
    pub 1: u64 smenq_next_sqb_vld :,
    pub 1: u64 smq_pend :,
    pub 1: u64 smq_next_sq_vld :,
    pub 3: u64 reserved_253_255 :,
    pub /: *mut *mut u64 next_sqb : 64; / W4,
    pub /: *mut *mut u64 tail_sqb : 64; / W5,
    pub /: *mut *mut u64 smenq_sqb : 64; / W6,
    pub /: *mut *mut u64 smenq_next_sqb : 64; / W7,
    pub /: *mut *mut u64 head_sqb : 64; / W8,
    pub /: *mut *mut u64 reserved_576_583 : 8; / W9,
    pub 18: u64 vfi_lso_total :,
    pub 3: u64 vfi_lso_sizem1 :,
    pub 8: u64 vfi_lso_sb :,
    pub 14: u64 vfi_lso_mps :,
    pub 1: u64 vfi_lso_vlan0_ins_ena :,
    pub 1: u64 vfi_lso_vlan1_ins_ena :,
    pub 1: u64 vfi_lso_vld :,
    pub 10: u64 reserved_630_639 :,
    pub /: *mut *mut u64 scm_lso_rem : 18; / W10,
    pub 46: u64 reserved_658_703 :,
    pub /: *mut *mut u64 octs : 48; / W11,
    pub 16: u64 reserved_752_767 :,
    pub /: *mut *mut u64 pkts : 48; / W12,
    pub 16: u64 reserved_816_831 :,
    pub /: *mut *mut u64 aged_drop_octs : 32; / W13,
    pub 32: u64 aged_drop_pkts :,
    pub /: *mut *mut u64 dropped_octs : 48; / W14,
    pub 16: u64 reserved_944_959 :,
    pub /: *mut *mut u64 dropped_pkts : 48; / W15,
    pub 16: u64 reserved_1008_1023 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn20k_cq_ctx_s {
    pub /: *mut *mut u64 base : 64; / W0,
    pub /: *mut *mut u64 lbp_ena : 1; / W1,
    pub 3: u64 lbpid_low :,
    pub 1: u64 bp_ena :,
    pub 3: u64 lbpid_med :,
    pub 9: u64 bpid :,
    pub 3: u64 lbpid_high :,
    pub 7: u64 qint_idx :,
    pub 1: u64 cq_err :,
    pub 7: u64 cint_idx :,
    pub 9: u64 avg_con :,
    pub 20: u64 wrptr :,
    pub /: *mut *mut u64 tail : 20; / W2,
    pub 20: u64 head :,
    pub 8: u64 avg_level :,
    pub 16: u64 update_time :,
    pub /: *mut *mut u64 bp : 8; / W3,
    pub 8: u64 drop :,
    pub 1: u64 drop_ena :,
    pub 1: u64 ena :,
    pub 1: u64 cpt_drop_err_en :,
    pub 1: u64 reserved_211_211 :,
    pub 11: u64 msh_dst :,
    pub 1: u64 msh_valid :,
    pub 4: u64 stash_thresh :,
    pub 4: u64 lbp_frac :,
    pub 1: u64 caching :,
    pub 1: u64 stashing :,
    pub 2: u64 reserved_234_235 :,
    pub 4: u64 qsize :,
    pub 8: u64 cq_err_int :,
    pub 8: u64 cq_err_int_ena :,
    pub /: *mut *mut u64 bpid_ext : 2; / W4,
    pub 2: u64 reserved_258_259 :,
    pub 2: u64 lbpid_ext :,
    pub 58: u64 reserved_262_319 :,
    pub /: *mut *mut u64 reserved_320_383 : 64; / W5,
    pub /: *mut *mut u64 reserved_384_447 : 64; / W6,
    pub /: *mut *mut u64 reserved_448_511 : 64; / W7,
    pub padding: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn20k_rq_ctx_s {
    pub 1: u64 ena :,
    pub 1: u64 sso_ena :,
    pub 1: u64 ipsech_ena :,
    pub 1: u64 ena_wqwd :,
    pub 20: u64 cq :,
    pub 11: u64 reserved_24_34 :,
    pub 1: u64 port_il4_dis :,
    pub 1: u64 port_ol4_dis :,
    pub 1: u64 lenerr_dis :,
    pub 1: u64 csum_il4_dis :,
    pub 1: u64 csum_ol4_dis :,
    pub 1: u64 len_il4_dis :,
    pub 1: u64 len_il3_dis :,
    pub 1: u64 len_ol4_dis :,
    pub 1: u64 len_ol3_dis :,
    pub 20: u64 wqe_aura :,
    pub 20: u64 spb_aura :,
    pub 20: u64 lpb_aura :,
    pub 10: u64 sso_grp :,
    pub 2: u64 sso_tt :,
    pub 2: u64 pb_caching :,
    pub 1: u64 wqe_caching :,
    pub 1: u64 xqe_drop_ena :,
    pub 1: u64 spb_drop_ena :,
    pub 1: u64 lpb_drop_ena :,
    pub 1: u64 pb_stashing :,
    pub 1: u64 ipsecd_drop_en :,
    pub 1: u64 chi_ena :,
    pub 3: u64 reserved_125_127 :,
    pub 10: u64 band_prof_id_l :,
    pub 1: u64 sso_fc_ena :,
    pub 1: u64 policer_ena :,
    pub 6: u64 spb_sizem1 :,
    pub 2: u64 wqe_skip :,
    pub 3: u64 spb_high_sizem1 :,
    pub 1: u64 spb_ena :,
    pub 12: u64 lpb_sizem1 :,
    pub 7: u64 first_skip :,
    pub 1: u64 reserved_171_171 :,
    pub 6: u64 later_skip :,
    pub 6: u64 xqe_imm_size :,
    pub 4: u64 band_prof_id_h :,
    pub 2: u64 reserved_188_189 :,
    pub 1: u64 xqe_imm_copy :,
    pub 1: u64 xqe_hdr_split :,
    pub 8: u64 xqe_drop :,
    pub 8: u64 xqe_pass :,
    pub 8: u64 wqe_pool_drop :,
    pub 8: u64 wqe_pool_pass :,
    pub 8: u64 spb_aura_drop :,
    pub 8: u64 spb_aura_pass :,
    pub 8: u64 spb_pool_drop :,
    pub 8: u64 spb_pool_pass :,
    pub 8: u64 lpb_aura_drop :,
    pub 8: u64 lpb_aura_pass :,
    pub 8: u64 lpb_pool_drop :,
    pub 8: u64 lpb_pool_pass :,
    pub 4: u64 reserved_288_291 :,
    pub 8: u64 rq_int :,
    pub 8: u64 rq_int_ena :,
    pub 7: u64 qint_idx :,
    pub 5: u64 reserved_315_319 :,
    pub 24: u64 ltag :,
    pub 8: u64 good_utag :,
    pub 8: u64 bad_utag :,
    pub 6: u64 flow_tagw :,
    pub 1: u64 ipsec_vwqe :,
    pub 1: u64 vwqe_ena :,
    pub 8: u64 vtime_wait :,
    pub 4: u64 max_vsize_exp :,
    pub 2: u64 vwqe_skip :,
    pub 2: u64 reserved_382_383 :,
    pub 48: u64 octs :,
    pub 16: u64 reserved_432_447 :,
    pub 48: u64 pkts :,
    pub 16: u64 reserved_496_511 :,
    pub 48: u64 drop_octs :,
    pub 16: u64 reserved_560_575 :,
    pub 48: u64 drop_pkts :,
    pub 16: u64 reserved_624_639 :,
    pub 48: u64 re_pkts :,
    pub 16: u64 reserved_688_703 :,
    pub 64: u64 reserved_704_767 :,
    pub 64: u64 reserved_768_831 :,
    pub 64: u64 reserved_832_895 :,
    pub 64: u64 reserved_896_959 :,
    pub 64: u64 reserved_960_1023 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_cn20k_aura_s {
    pub /: *mut *mut u64 pool_addr; / W0,
    pub /: *mut *mut u64 ena : 1; / W1,
    pub 2: u64 reserved_65 :,
    pub 1: u64 pool_caching :,
    pub 16: u64 reserved_68 :,
    pub 9: u64 avg_con :,
    pub 1: u64 reserved_93 :,
    pub 1: u64 pool_drop_ena :,
    pub 1: u64 aura_drop_ena :,
    pub 1: u64 bp_ena :,
    pub 7: u64 reserved_97_103 :,
    pub 8: u64 aura_drop :,
    pub 6: u64 shift :,
    pub 2: u64 reserved_118_119 :,
    pub 8: u64 avg_level :,
    pub /: *mut *mut u64 count : 36; / W2,
    pub 4: u64 reserved_164_167 :,
    pub 12: u64 bpid :,
    pub 12: u64 reserved_180_191 :,
    pub /: *mut *mut u64 limit : 36; / W3,
    pub 4: u64 reserved_228_231 :,
    pub 7: u64 bp :,
    pub 5: u64 reserved_239_243 :,
    pub 1: u64 fc_ena :,
    pub 1: u64 fc_up_crossing :,
    pub 2: u64 fc_stype :,
    pub 4: u64 fc_hyst_bits :,
    pub 4: u64 reserved_252_255 :,
    pub /: *mut *mut u64 fc_addr; / W4,
    pub /: *mut *mut u64 pool_drop : 8; / W5,
    pub 16: u64 update_time :,
    pub 8: u64 err_int :,
    pub 8: u64 err_int_ena :,
    pub 1: u64 thresh_int :,
    pub 1: u64 thresh_int_ena :,
    pub 1: u64 thresh_up :,
    pub 1: u64 reserved_363 :,
    pub 7: u64 thresh_qint_idx :,
    pub 1: u64 reserved_371 :,
    pub 7: u64 err_qint_idx :,
    pub 5: u64 reserved_379_383 :,
    pub W6*/: *mut *mut u64 thresh : 36; /,
    pub 4: u64 rsvd_423_420 :,
    pub 11: u64 fc_msh_dst :,
    pub 4: u64 reserved_435_438 :,
    pub 1: u64 op_dpc_ena :,
    pub 5: u64 op_dpc_set :,
    pub 1: u64 reserved_445_445 :,
    pub 1: u64 stream_ctx :,
    pub 1: u64 unified_ctx :,
    pub /: *mut *mut u64 reserved_448_511; / W7,
    pub padding: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_cn20k_pool_s {
    pub /: *mut *mut u64 stack_base; / W0,
    pub 1: u64 ena :,
    pub 1: u64 nat_align :,
    pub 2: u64 reserved_66_67 :,
    pub 1: u64 stack_caching :,
    pub 19: u64 reserved_69_87 :,
    pub 12: u64 buf_offset :,
    pub 4: u64 reserved_100_103 :,
    pub 12: u64 buf_size :,
    pub 4: u64 reserved_116_119 :,
    pub 3: u64 ref_cnt_prof :,
    pub 5: u64 reserved_123_127 :,
    pub 32: u64 stack_max_pages :,
    pub 32: u64 stack_pages :,
    pub 7: uint64_t bp_0 :,
    pub 7: uint64_t bp_1 :,
    pub 7: uint64_t bp_2 :,
    pub 7: uint64_t bp_3 :,
    pub 7: uint64_t bp_4 :,
    pub 7: uint64_t bp_5 :,
    pub 7: uint64_t bp_6 :,
    pub 7: uint64_t bp_7 :,
    pub 1: uint64_t bp_ena_0 :,
    pub 1: uint64_t bp_ena_1 :,
    pub 1: uint64_t bp_ena_2 :,
    pub 1: uint64_t bp_ena_3 :,
    pub 1: uint64_t bp_ena_4 :,
    pub 1: uint64_t bp_ena_5 :,
    pub 1: uint64_t bp_ena_6 :,
    pub 1: uint64_t bp_ena_7 :,
    pub 4: u64 stack_offset :,
    pub 4: u64 reserved_260_263 :,
    pub 6: u64 shift :,
    pub 2: u64 reserved_270_271 :,
    pub 8: u64 avg_level :,
    pub 9: u64 avg_con :,
    pub 1: u64 fc_ena :,
    pub 2: u64 fc_stype :,
    pub 4: u64 fc_hyst_bits :,
    pub 1: u64 fc_up_crossing :,
    pub 3: u64 reserved_297_299 :,
    pub 16: u64 update_time :,
    pub 4: u64 reserved_316_319 :,
    pub /: *mut *mut u64 fc_addr; / W5,
    pub /: *mut *mut u64 ptr_start; / W6,
    pub /: *mut *mut u64 ptr_end; / W7,
    pub 12: u64 bpid_0 :,
    pub 12: u64 reserved_524_535 :,
    pub 8: u64 err_int :,
    pub 8: u64 err_int_ena :,
    pub 1: u64 thresh_int :,
    pub 1: u64 thresh_int_ena :,
    pub 1: u64 thresh_up :,
    pub 1: u64 reserved_555 :,
    pub 7: u64 thresh_qint_idx :,
    pub 1: u64 reserved_563 :,
    pub 7: u64 err_qint_idx :,
    pub 5: u64 reserved_571_575 :,
    pub 36: u64 thresh :,
    pub 4: u64 rsvd_612_615 :,
    pub 11: u64 fc_msh_dst :,
    pub 4: u64 reserved_627_630 :,
    pub 1: u64 op_dpc_ena :,
    pub 5: u64 op_dpc_set :,
    pub 1: u64 reserved_637_637 :,
    pub 1: u64 stream_ctx :,
    pub 1: u64 reserved_639 :,
    pub /: *mut *mut u64 reserved_640_703; / W10,
    pub /: *mut *mut u64 reserved_704_767; / W11,
    pub /: *mut *mut u64 reserved_768_831; / W12,
    pub /: *mut *mut u64 reserved_832_895; / W13,
    pub /: *mut *mut u64 reserved_896_959; / W14,
    pub /: *mut *mut u64 reserved_960_1023; / W15,
}
