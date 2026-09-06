//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/af/rvu_struct.h
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
// Copyright (C) 2018 Marvell.
//
// RVU Block revision IDs
pub const RVU_BLK_RVUM_REVID: c_uint = 0x01;
pub const RVU_MULTI_BLK_VER: c_uint = 0x7ULL;
pub const NIX_MAX_CTX_SIZE: c_int = 128;
// RVU Block Address Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_block_addr_e {
    BLKADDR_RVUM		= 0x0ULL,
    BLKADDR_LMT		= 0x1ULL,
    BLKADDR_MSIX		= 0x2ULL,
    BLKADDR_NPA		= 0x3ULL,
    BLKADDR_NIX0		= 0x4ULL,
    BLKADDR_NIX1		= 0x5ULL,
    BLKADDR_NPC		= 0x6ULL,
    BLKADDR_SSO		= 0x7ULL,
    BLKADDR_SSOW		= 0x8ULL,
    BLKADDR_TIM		= 0x9ULL,
    BLKADDR_CPT0		= 0xaULL,
    BLKADDR_CPT1		= 0xbULL,
    BLKADDR_NDC_NIX0_RX	= 0xcULL,
    BLKADDR_NDC_NIX0_TX	= 0xdULL,
    BLKADDR_NDC_NPA0	= 0xeULL,
    BLKADDR_NDC_NIX1_RX	= 0x10ULL,
    BLKADDR_NDC_NIX1_TX	= 0x11ULL,
    BLKADDR_APR		= 0x16ULL,
    BLKADDR_MBOX		= 0x1bULL,
    BLK_COUNT		= 0x1cULL,
}

// RVU Block Type Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_block_type_e {
    BLKTYPE_RVUM = 0x0,
    BLKTYPE_MSIX = 0x1,
    BLKTYPE_LMT  = 0x2,
    BLKTYPE_NIX  = 0x3,
    BLKTYPE_NPA  = 0x4,
    BLKTYPE_NPC  = 0x5,
    BLKTYPE_SSO  = 0x6,
    BLKTYPE_SSOW = 0x7,
    BLKTYPE_TIM  = 0x8,
    BLKTYPE_CPT  = 0x9,
    BLKTYPE_NDC  = 0xa,
    BLKTYPE_MBOX = 0x13,
    BLKTYPE_MAX  = 0x13,
}

// RVU Admin function Interrupt Vector Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_af_int_vec_e {
    RVU_AF_INT_VEC_POISON = 0x0,
    RVU_AF_INT_VEC_PFFLR  = 0x1,
    RVU_AF_INT_VEC_PFME   = 0x2,
    RVU_AF_INT_VEC_GEN    = 0x3,
    RVU_AF_INT_VEC_MBOX   = 0x4,
    RVU_AF_INT_VEC_CNT    = 0x5,
}

// CPT Admin function Interrupt Vector Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpt_af_int_vec_e {
    CPT_AF_INT_VEC_FLT0	= 0x0,
    CPT_AF_INT_VEC_FLT1	= 0x1,
    CPT_AF_INT_VEC_RVU	= 0x2,
    CPT_AF_INT_VEC_RAS	= 0x3,
    CPT_AF_INT_VEC_CNT	= 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpt_cn10k_flt_int_vec_e {
    CPT_10K_AF_INT_VEC_FLT0	= 0x0,
    CPT_10K_AF_INT_VEC_FLT1	= 0x1,
    CPT_10K_AF_INT_VEC_FLT2	= 0x2,
    CPT_10K_AF_INT_VEC_FLT_MAX = 0x3,
}

// NPA Admin function Interrupt Vector Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_af_int_vec_e {
    NPA_AF_INT_VEC_RVU	= 0x0,
    NPA_AF_INT_VEC_GEN	= 0x1,
    NPA_AF_INT_VEC_AQ_DONE	= 0x2,
    NPA_AF_INT_VEC_AF_ERR	= 0x3,
    NPA_AF_INT_VEC_POISON	= 0x4,
    NPA_AF_INT_VEC_CNT	= 0x5,
}

// NIX Admin function Interrupt Vector Enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_af_int_vec_e {
    NIX_AF_INT_VEC_RVU	= 0x0,
    NIX_AF_INT_VEC_GEN	= 0x1,
    NIX_AF_INT_VEC_AQ_DONE	= 0x2,
    NIX_AF_INT_VEC_AF_ERR	= 0x3,
    NIX_AF_INT_VEC_POISON	= 0x4,
    NIX_AF_INT_VEC_CNT	= 0x5,
}

//
// RVU PF Interrupt Vector Enumeration
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rvu_pf_int_vec_e {
    RVU_PF_INT_VEC_VFFLR0     = 0x0,
    RVU_PF_INT_VEC_VFFLR1     = 0x1,
    RVU_PF_INT_VEC_VFME0      = 0x2,
    RVU_PF_INT_VEC_VFME1      = 0x3,
    RVU_PF_INT_VEC_VFPF_MBOX0 = 0x4,
    RVU_PF_INT_VEC_VFPF_MBOX1 = 0x5,
    RVU_PF_INT_VEC_AFPF_MBOX  = 0x6,
    RVU_PF_INT_VEC_CNT	  = 0x7,
}

// NPA admin queue completion enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_aq_comp {
    NPA_AQ_COMP_NOTDONE    = 0x0,
    NPA_AQ_COMP_GOOD       = 0x1,
    NPA_AQ_COMP_SWERR      = 0x2,
    NPA_AQ_COMP_CTX_POISON = 0x3,
    NPA_AQ_COMP_CTX_FAULT  = 0x4,
    NPA_AQ_COMP_LOCKERR    = 0x5,
}

// NPA admin queue context types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_aq_ctype {
    NPA_AQ_CTYPE_AURA = 0x0,
    NPA_AQ_CTYPE_POOL = 0x1,
}

// NPA admin queue instruction opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_aq_instop {
    NPA_AQ_INSTOP_NOP    = 0x0,
    NPA_AQ_INSTOP_INIT   = 0x1,
    NPA_AQ_INSTOP_WRITE  = 0x2,
    NPA_AQ_INSTOP_READ   = 0x3,
    NPA_AQ_INSTOP_LOCK   = 0x4,
    NPA_AQ_INSTOP_UNLOCK = 0x5,
}

// ALLOC/FREE input queues Enumeration from coprocessors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum npa_inpq {
    NPA_INPQ_NIX0_RX       = 0x0,
    NPA_INPQ_NIX0_TX       = 0x1,
    NPA_INPQ_NIX1_RX       = 0x2,
    NPA_INPQ_NIX1_TX       = 0x3,
    NPA_INPQ_SSO           = 0x4,
    NPA_INPQ_TIM           = 0x5,
    NPA_INPQ_DPI           = 0x6,
    NPA_INPQ_AURA_OP       = 0xe,
    NPA_INPQ_INTERNAL_RSV  = 0xf,
}

// NPA admin queue instruction structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_inst_s {
    pub /: *mut *mut u64 op : 4; / W0,
    pub 4: u64 ctype :,
    pub 9: u64 lf :,
    pub 7: u64 reserved_17_23 :,
    pub 20: u64 cindex :,
    pub 19: u64 reserved_44_62 :,
    pub 1: u64 doneint :,
    pub /: *mut *mut u64 res_addr; / W1,
}

// NPA admin queue result structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aq_res_s {
    pub /: *mut *mut u64 op : 4; / W0,
    pub 4: u64 ctype :,
    pub 8: u64 compcode :,
    pub 1: u64 doneint :,
    pub 47: u64 reserved_17_63 :,
    pub /: *mut *mut u64 reserved_64_127; / W1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_aura_s {
    pub /: *mut *mut u64 pool_addr; / W0,
    pub /: *mut *mut u64 ena : 1; / W1,
    pub 2: u64 reserved_65 :,
    pub 1: u64 pool_caching :,
    pub 16: u64 pool_way_mask :,
    pub 9: u64 avg_con :,
    pub 1: u64 reserved_93 :,
    pub 1: u64 pool_drop_ena :,
    pub 1: u64 aura_drop_ena :,
    pub 2: u64 bp_ena :,
    pub 6: u64 reserved_98_103 :,
    pub 8: u64 aura_drop :,
    pub 6: u64 shift :,
    pub 2: u64 reserved_118_119 :,
    pub 8: u64 avg_level :,
    pub /: *mut *mut u64 count : 36; / W2,
    pub 4: u64 reserved_164_167 :,
    pub 9: u64 nix0_bpid :,
    pub 3: u64 reserved_177_179 :,
    pub 9: u64 nix1_bpid :,
    pub 3: u64 reserved_189_191 :,
    pub /: *mut *mut u64 limit : 36; / W3,
    pub 4: u64 reserved_228_231 :,
    pub 8: u64 bp :,
    pub 3: u64 reserved_241_243 :,
    pub 1: u64 fc_be :,
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
    pub 13: u64 reserved_435_447 :,
    pub /: *mut *mut u64 reserved_448_511; / W7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct npa_pool_s {
    pub /: *mut *mut u64 stack_base; / W0,
    pub 1: u64 ena :,
    pub 1: u64 nat_align :,
    pub 2: u64 reserved_66_67 :,
    pub 1: u64 stack_caching :,
    pub 3: u64 reserved_70_71 :,
    pub 16: u64 stack_way_mask :,
    pub 12: u64 buf_offset :,
    pub 4: u64 reserved_100_103 :,
    pub 11: u64 buf_size :,
    pub 13: u64 reserved_115_127 :,
    pub 32: u64 stack_max_pages :,
    pub 32: u64 stack_pages :,
    pub 48: u64 op_pc :,
    pub 16: u64 reserved_240_255 :,
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
    pub 1: u64 fc_be :,
    pub 2: u64 reserved_298_299 :,
    pub 16: u64 update_time :,
    pub 4: u64 reserved_316_319 :,
    pub /: *mut *mut u64 fc_addr; / W5,
    pub /: *mut *mut u64 ptr_start; / W6,
    pub /: *mut *mut u64 ptr_end; / W7,
    pub 24: u64 reserved_512_535 :,
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
    pub 4: u64 rsvd_615_612 :,
    pub 11: u64 fc_msh_dst :,
    pub 13: u64 reserved_627_639 :,
    pub /: *mut *mut u64 reserved_640_703; / W10,
    pub /: *mut *mut u64 reserved_704_767; / W11,
    pub /: *mut *mut u64 reserved_768_831; / W12,
    pub /: *mut *mut u64 reserved_832_895; / W13,
    pub /: *mut *mut u64 reserved_896_959; / W14,
    pub /: *mut *mut u64 reserved_960_1023; / W15,
}

// NIX admin queue completion status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_aq_comp {
    NIX_AQ_COMP_NOTDONE        = 0x0,
    NIX_AQ_COMP_GOOD           = 0x1,
    NIX_AQ_COMP_SWERR          = 0x2,
    NIX_AQ_COMP_CTX_POISON     = 0x3,
    NIX_AQ_COMP_CTX_FAULT      = 0x4,
    NIX_AQ_COMP_LOCKERR        = 0x5,
    NIX_AQ_COMP_SQB_ALLOC_FAIL = 0x6,
}

// NIX admin queue context types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_aq_ctype {
    NIX_AQ_CTYPE_RQ   = 0x0,
    NIX_AQ_CTYPE_SQ   = 0x1,
    NIX_AQ_CTYPE_CQ   = 0x2,
    NIX_AQ_CTYPE_MCE  = 0x3,
    NIX_AQ_CTYPE_RSS  = 0x4,
    NIX_AQ_CTYPE_DYNO = 0x5,
    NIX_AQ_CTYPE_BANDPROF = 0x6,
}

// NIX admin queue instruction opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_aq_instop {
    NIX_AQ_INSTOP_NOP    = 0x0,
    NIX_AQ_INSTOP_INIT   = 0x1,
    NIX_AQ_INSTOP_WRITE  = 0x2,
    NIX_AQ_INSTOP_READ   = 0x3,
    NIX_AQ_INSTOP_LOCK   = 0x4,
    NIX_AQ_INSTOP_UNLOCK = 0x5,
}

// NIX admin queue instruction structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_aq_inst_s {
    pub 4: u64 op :,
    pub 4: u64 ctype :,
    pub 9: u64 lf :,
    pub 7: u64 reserved_17_23 :,
    pub 20: u64 cindex :,
    pub 19: u64 reserved_44_62 :,
    pub 1: u64 doneint :,
    pub /: *mut *mut u64 res_addr; / W1,
}

// NIX admin queue result structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_aq_res_s {
    pub 4: u64 op :,
    pub 4: u64 ctype :,
    pub 8: u64 compcode :,
    pub 1: u64 doneint :,
    pub 47: u64 reserved_17_63 :,
    pub /: *mut *mut u64 reserved_64_127; / W1,
}

// NIX Completion queue context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cq_ctx_s {
    pub base: u64,
    pub 1: u64 lbp_ena :,
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
    pub 20: u64 tail :,
    pub 20: u64 head :,
    pub 8: u64 avg_level :,
    pub 16: u64 update_time :,
    pub 8: u64 bp :,
    pub 8: u64 drop :,
    pub 1: u64 drop_ena :,
    pub 1: u64 ena :,
    pub 1: u64 cpt_drop_err_en :,
    pub 1: u64 rsvd_211 :,
    pub 12: u64 substream :,
    pub 4: u64 stash_thresh :,
    pub 4: u64 lbp_frac :,
    pub 1: u64 caching :,
    pub 1: u64 stashing :,
    pub 2: u64 rsvd_234_235 :,
    pub 4: u64 qsize :,
    pub 8: u64 cq_err_int :,
    pub 8: u64 cq_err_int_ena :,
// Ensure all context sizes are 128 bytes
    pub padding: [u64; 12],
}

// CN10K NIX Receive queue context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn10k_rq_ctx_s {
    pub 1: u64 ena :,
    pub 1: u64 sso_ena :,
    pub 1: u64 ipsech_ena :,
    pub 1: u64 ena_wqwd :,
    pub 20: u64 cq :,
    pub 13: u64 rsvd_36_24 :,
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
    pub 1: u64 ipsecd_drop_ena :,
    pub 1: u64 chi_ena :,
    pub 3: u64 rsvd_127_125 :,
    pub /: *mut *mut u64 band_prof_id : 10; / W2,
    pub 1: u64 rsvd_138 :,
    pub 1: u64 policer_ena :,
    pub 6: u64 spb_sizem1 :,
    pub 2: u64 wqe_skip :,
    pub 3: u64 rsvd_150_148 :,
    pub 1: u64 spb_ena :,
    pub 12: u64 lpb_sizem1 :,
    pub 7: u64 first_skip :,
    pub 1: u64 rsvd_171 :,
    pub 6: u64 later_skip :,
    pub 6: u64 xqe_imm_size :,
    pub 4: u64 band_prof_id_h :,
    pub 2: u64 rsvd_189_188 :,
    pub 1: u64 xqe_imm_copy :,
    pub 1: u64 xqe_hdr_split :,
    pub /: *mut *mut u64 xqe_drop : 8; / W3,
    pub 8: u64 xqe_pass :,
    pub 8: u64 wqe_pool_drop :,
    pub 8: u64 wqe_pool_pass :,
    pub 8: u64 spb_aura_drop :,
    pub 8: u64 spb_aura_pass :,
    pub 8: u64 spb_pool_drop :,
    pub 8: u64 spb_pool_pass :,
    pub /: *mut *mut u64 lpb_aura_drop : 8; / W4,
    pub 8: u64 lpb_aura_pass :,
    pub 8: u64 lpb_pool_drop :,
    pub 8: u64 lpb_pool_pass :,
    pub 4: u64 rsvd_291_288 :,
    pub 8: u64 rq_int :,
    pub 8: u64 rq_int_ena :,
    pub 7: u64 qint_idx :,
    pub 5: u64 rsvd_319_315 :,
    pub /: *mut *mut u64 ltag : 24; / W5,
    pub 8: u64 good_utag :,
    pub 8: u64 bad_utag :,
    pub 6: u64 flow_tagw :,
    pub 1: u64 ipsec_vwqe :,
    pub 1: u64 vwqe_ena :,
    pub 8: u64 vwqe_wait :,
    pub 4: u64 max_vsize_exp :,
    pub 2: u64 vwqe_skip :,
    pub 2: u64 rsvd_383_382 :,
    pub /: *mut *mut u64 octs : 48; / W6,
    pub 16: u64 rsvd_447_432 :,
    pub /: *mut *mut u64 pkts : 48; / W7,
    pub 16: u64 rsvd_511_496 :,
    pub /: *mut *mut u64 drop_octs : 48; / W8,
    pub 16: u64 rsvd_575_560 :,
    pub /: *mut *mut u64 drop_pkts : 48; / W9,
    pub 16: u64 rsvd_639_624 :,
    pub /: *mut *mut u64 re_pkts : 48; / W10,
    pub 16: u64 rsvd_703_688 :,
    pub /: *mut *mut u64 rsvd_767_704; / W11,
    pub /: *mut *mut u64 rsvd_831_768; / W12,
    pub /: *mut *mut u64 rsvd_895_832; / W13,
    pub /: *mut *mut u64 rsvd_959_896; / W14,
    pub /: *mut *mut u64 rsvd_1023_960; / W15,
}

// CN10K NIX Send queue context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_cn10k_sq_ctx_s {
    pub 1: u64 ena :,
    pub 6: u64 qint_idx :,
    pub 20: u64 substream :,
    pub 1: u64 sdp_mcast :,
    pub 20: u64 cq :,
    pub 16: u64 sqe_way_mask :,
    pub /: *mut *mut u64 smq : 10; / W1,
    pub 1: u64 cq_ena :,
    pub 1: u64 xoff :,
    pub 1: u64 sso_ena :,
    pub 14: u64 smq_rr_weight :,
    pub 12: u64 default_chan :,
    pub 16: u64 sqb_count :,
    pub 2: u64 rsvd_120_119 :,
    pub 7: u64 smq_rr_count_lb :,
    pub /: *mut *mut u64 smq_rr_count_ub : 25; / W2,
    pub 20: u64 sqb_aura :,
    pub 8: u64 sq_int :,
    pub 8: u64 sq_int_ena :,
    pub 2: u64 sqe_stype :,
    pub 1: u64 rsvd_191 :,
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
    pub 3: u64 rsvd_255_253 :,
    pub /: *mut *mut u64 next_sqb : 64; / W4,
    pub /: *mut *mut u64 tail_sqb : 64; / W5,
    pub /: *mut *mut u64 smenq_sqb : 64; / W6,
    pub /: *mut *mut u64 smenq_next_sqb : 64; / W7,
    pub /: *mut *mut u64 head_sqb : 64; / W8,
    pub /: *mut *mut u64 rsvd_583_576 : 8; / W9,
    pub 18: u64 vfi_lso_total :,
    pub 3: u64 vfi_lso_sizem1 :,
    pub 8: u64 vfi_lso_sb :,
    pub 14: u64 vfi_lso_mps :,
    pub 1: u64 vfi_lso_vlan0_ins_ena :,
    pub 1: u64 vfi_lso_vlan1_ins_ena :,
    pub 1: u64 vfi_lso_vld :,
    pub 10: u64 rsvd_639_630 :,
    pub /: *mut *mut u64 scm_lso_rem : 18; / W10,
    pub 46: u64 rsvd_703_658 :,
    pub /: *mut *mut u64 octs : 48; / W11,
    pub 16: u64 rsvd_767_752 :,
    pub /: *mut *mut u64 pkts : 48; / W12,
    pub 16: u64 rsvd_831_816 :,
    pub /: *mut *mut u64 rsvd_895_832 : 64; / W13,
    pub 48: u64 dropped_octs :,
    pub 16: u64 rsvd_959_944 :,
    pub 48: u64 dropped_pkts :,
    pub 16: u64 rsvd_1023_1008 :,
}

// NIX Receive queue context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rq_ctx_s {
    pub 1: u64 ena :,
    pub 1: u64 sso_ena :,
    pub 1: u64 ipsech_ena :,
    pub 1: u64 ena_wqwd :,
    pub 20: u64 cq :,
    pub 20: u64 substream :,
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
    pub 6: u64 rsvd_127_122 :,
    pub /: *mut *mut u64 rsvd_139_128 : 12; / W2,
    pub 6: u64 spb_sizem1 :,
    pub 2: u64 wqe_skip :,
    pub 3: u64 rsvd_150_148 :,
    pub 1: u64 spb_ena :,
    pub 12: u64 lpb_sizem1 :,
    pub 7: u64 first_skip :,
    pub 1: u64 rsvd_171 :,
    pub 6: u64 later_skip :,
    pub 6: u64 xqe_imm_size :,
    pub 6: u64 rsvd_189_184 :,
    pub 1: u64 xqe_imm_copy :,
    pub 1: u64 xqe_hdr_split :,
    pub W3*/: *mut *mut u64 xqe_drop : 8; /,
    pub 8: u64 xqe_pass :,
    pub 8: u64 wqe_pool_drop :,
    pub 8: u64 wqe_pool_pass :,
    pub 8: u64 spb_aura_drop :,
    pub 8: u64 spb_aura_pass :,
    pub 8: u64 spb_pool_drop :,
    pub 8: u64 spb_pool_pass :,
    pub /: *mut *mut u64 lpb_aura_drop : 8; / W4,
    pub 8: u64 lpb_aura_pass :,
    pub 8: u64 lpb_pool_drop :,
    pub 8: u64 lpb_pool_pass :,
    pub 4: u64 rsvd_291_288 :,
    pub 8: u64 rq_int :,
    pub 8: u64 rq_int_ena :,
    pub 7: u64 qint_idx :,
    pub 5: u64 rsvd_319_315 :,
    pub /: *mut *mut u64 ltag : 24; / W5,
    pub 8: u64 good_utag :,
    pub 8: u64 bad_utag :,
    pub 6: u64 flow_tagw :,
    pub 18: u64 rsvd_383_366 :,
    pub /: *mut *mut u64 octs : 48; / W6,
    pub 16: u64 rsvd_447_432 :,
    pub /: *mut *mut u64 pkts : 48; / W7,
    pub 16: u64 rsvd_511_496 :,
    pub /: *mut *mut u64 drop_octs : 48; / W8,
    pub 16: u64 rsvd_575_560 :,
    pub /: *mut *mut u64 drop_pkts : 48; / W9,
    pub 16: u64 rsvd_639_624 :,
    pub /: *mut *mut u64 re_pkts : 48; / W10,
    pub 16: u64 rsvd_703_688 :,
    pub /: *mut *mut u64 rsvd_767_704; / W11,
    pub /: *mut *mut u64 rsvd_831_768; / W12,
    pub /: *mut *mut u64 rsvd_895_832; / W13,
    pub /: *mut *mut u64 rsvd_959_896; / W14,
    pub /: *mut *mut u64 rsvd_1023_960; / W15,
}

// NIX sqe sizes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_maxsqesz {
    NIX_MAXSQESZ_W16 = 0x0,
    NIX_MAXSQESZ_W8  = 0x1,
}

// NIX SQB caching type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_stype {
    NIX_STYPE_STF = 0x0,
    NIX_STYPE_STT = 0x1,
    NIX_STYPE_STP = 0x2,
}

// NIX Send queue context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_sq_ctx_s {
    pub 1: u64 ena :,
    pub 6: u64 qint_idx :,
    pub 20: u64 substream :,
    pub 1: u64 sdp_mcast :,
    pub 20: u64 cq :,
    pub 16: u64 sqe_way_mask :,
    pub 9: u64 smq :,
    pub 1: u64 cq_ena :,
    pub 1: u64 xoff :,
    pub 1: u64 sso_ena :,
    pub 24: u64 smq_rr_quantum :,
    pub 12: u64 default_chan :,
    pub 16: u64 sqb_count :,
    pub 25: u64 smq_rr_count :,
    pub 20: u64 sqb_aura :,
    pub 8: u64 sq_int :,
    pub 8: u64 sq_int_ena :,
    pub 2: u64 sqe_stype :,
    pub 1: u64 rsvd_191 :,
    pub 2: u64 max_sqe_size :,
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
    pub 3: u64 rsvd_255_253 :,
    pub /: *mut *mut u64 next_sqb : 64;/ W4,
    pub /: *mut *mut u64 tail_sqb : 64;/ W5,
    pub /: *mut *mut u64 smenq_sqb : 64;/ W6,
    pub /: *mut *mut u64 smenq_next_sqb : 64;/ W7,
    pub /: *mut *mut u64 head_sqb : 64;/ W8,
    pub 8: u64 rsvd_583_576 :,
    pub 18: u64 vfi_lso_total :,
    pub 3: u64 vfi_lso_sizem1 :,
    pub 8: u64 vfi_lso_sb :,
    pub 14: u64 vfi_lso_mps :,
    pub 1: u64 vfi_lso_vlan0_ins_ena :,
    pub 1: u64 vfi_lso_vlan1_ins_ena :,
    pub 1: u64 vfi_lso_vld :,
    pub 10: u64 rsvd_639_630 :,
    pub 18: u64 scm_lso_rem :,
    pub 46: u64 rsvd_703_658 :,
    pub 48: u64 octs :,
    pub 16: u64 rsvd_767_752 :,
    pub 48: u64 pkts :,
    pub 16: u64 rsvd_831_816 :,
    pub /: *mut *mut u64 rsvd_895_832 : 64;/ W13,
    pub 48: u64 dropped_octs :,
    pub 16: u64 rsvd_959_944 :,
    pub 48: u64 dropped_pkts :,
    pub 16: u64 rsvd_1023_1008 :,
}

// NIX Receive side scaling entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rsse_s {
    pub 20: uint32_t rq :,
    pub 12: uint32_t reserved_20_31 :,
// Ensure all context sizes are minimum 128 bytes
    pub padding: [u64; 15],
}

// NIX receive multicast/mirror entry structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_mce_s {
    pub 2: uint64_t op :,
    pub 1: uint64_t rsvd_2 :,
    pub 1: uint64_t eol :,
    pub 20: uint64_t index :,
    pub 8: uint64_t rsvd_31_24 :,
    pub 16: uint64_t pf_func :,
    pub 16: uint64_t next :,
// Ensure all context sizes are minimum 128 bytes
    pub padding: [u64; 15],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_band_prof_layers {
    BAND_PROF_LEAF_LAYER = 0,
    BAND_PROF_INVAL_LAYER = 1,
    BAND_PROF_MID_LAYER = 2,
    BAND_PROF_TOP_LAYER = 3,
    BAND_PROF_NUM_LAYERS = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NIX_RX_BAND_PROF_ACTIONRESULT_E {
    NIX_RX_BAND_PROF_ACTIONRESULT_PASS = 0x0,
    NIX_RX_BAND_PROF_ACTIONRESULT_DROP = 0x1,
    NIX_RX_BAND_PROF_ACTIONRESULT_RED = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_band_prof_pc_mode {
    NIX_RX_PC_MODE_VLAN = 0,
    NIX_RX_PC_MODE_DSCP = 1,
    NIX_RX_PC_MODE_GEN = 2,
    NIX_RX_PC_MODE_RSVD = 3,
}

// NIX ingress policer bandwidth profile structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_bandprof_s {
    pub /: *mut *mut uint64_t pc_mode : 2; / W0,
    pub 2: uint64_t icolor :,
    pub 1: uint64_t tnl_ena :,
    pub 3: uint64_t reserved_5_7 :,
    pub 5: uint64_t peir_exponent :,
    pub 3: uint64_t reserved_13_15 :,
    pub 5: uint64_t pebs_exponent :,
    pub 3: uint64_t reserved_21_23 :,
    pub 5: uint64_t cir_exponent :,
    pub 3: uint64_t reserved_29_31 :,
    pub 5: uint64_t cbs_exponent :,
    pub 3: uint64_t reserved_37_39 :,
    pub 8: uint64_t peir_mantissa :,
    pub 8: uint64_t pebs_mantissa :,
    pub 8: uint64_t cir_mantissa :,
    pub /: *mut *mut uint64_t cbs_mantissa : 8; / W1,
    pub 1: uint64_t lmode :,
    pub 3: uint64_t l_sellect :,
    pub 4: uint64_t rdiv :,
    pub 5: uint64_t adjust_exponent :,
    pub 2: uint64_t reserved_85_86 :,
    pub 9: uint64_t adjust_mantissa :,
    pub 2: uint64_t gc_action :,
    pub 2: uint64_t yc_action :,
    pub 2: uint64_t rc_action :,
    pub 2: uint64_t meter_algo :,
    pub 7: uint64_t band_prof_id :,
    pub 4: uint64_t band_prof_id_h :,
    pub 4: uint64_t reserved_115_118 :,
    pub 1: uint64_t hl_en :,
    pub 8: uint64_t reserved_120_127 :,
    pub /: *mut *mut uint64_t ts : 48; / W2,
    pub 16: uint64_t reserved_176_191 :,
    pub /: *mut *mut uint64_t pe_accum : 32; / W3,
    pub 32: uint64_t c_accum :,
    pub /: *mut *mut uint64_t green_pkt_pass : 48; / W4,
    pub 16: uint64_t reserved_304_319 :,
    pub /: *mut *mut uint64_t yellow_pkt_pass : 48; / W5,
    pub 16: uint64_t reserved_368_383 :,
    pub /: *mut *mut uint64_t red_pkt_pass : 48; / W6,
    pub 16: uint64_t reserved_432_447 :,
    pub /: *mut *mut uint64_t green_octs_pass : 48; / W7,
    pub 16: uint64_t reserved_496_511 :,
    pub /: *mut *mut uint64_t yellow_octs_pass : 48; / W8,
    pub 16: uint64_t reserved_560_575 :,
    pub /: *mut *mut uint64_t red_octs_pass : 48; / W9,
    pub 16: uint64_t reserved_624_639 :,
    pub /: *mut *mut uint64_t green_pkt_drop : 48; / W10,
    pub 16: uint64_t reserved_688_703 :,
    pub /: *mut *mut uint64_t yellow_pkt_drop : 48; / W11,
    pub 16: uint64_t reserved_752_767 :,
    pub /: *mut *mut uint64_t red_pkt_drop : 48; / W12,
    pub 16: uint64_t reserved_816_831 :,
    pub /: *mut *mut uint64_t green_octs_drop : 48; / W13,
    pub 16: uint64_t reserved_880_895 :,
    pub /: *mut *mut uint64_t yellow_octs_drop : 48; / W14,
    pub 16: uint64_t reserved_944_959 :,
    pub /: *mut *mut uint64_t red_octs_drop : 48; / W15,
    pub 16: uint64_t reserved_1008_1023 :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_lsoalg {
    NIX_LSOALG_NOP,
    NIX_LSOALG_ADD_SEGNUM,
    NIX_LSOALG_ADD_PAYLEN,
    NIX_LSOALG_ADD_OFFSET,
    NIX_LSOALG_TCP_FLAGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_txlayer {
    NIX_TXLAYER_OL3,
    NIX_TXLAYER_OL4,
    NIX_TXLAYER_IL3,
    NIX_TXLAYER_IL4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_lso_format {
    pub 8: u64 offset :,
    pub 2: u64 layer :,
    pub 2: u64 rsvd_10_11 :,
    pub 2: u64 sizem1 :,
    pub 2: u64 rsvd_14_15 :,
    pub 3: u64 alg :,
    pub 45: u64 rsvd_19_63 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nix_rx_flowkey_alg {
    pub :6: u64 key_offset,
    pub :1: u64 ln_mask,
    pub :1: u64 fn_mask,
    pub :8: u64 hdr_offset,
    pub :5: u64 bytesm1,
    pub :3: u64 lid,
    pub :1: u64 reserved_24_24,
    pub :1: u64 ena,
    pub :1: u64 sel_chan,
    pub :4: u64 ltype_mask,
    pub :4: u64 ltype_match,
    pub :29: u64 reserved_35_63,
}

// NIX VTAG size
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_vtag_size {
    VTAGSIZE_T4   = 0x0,
    VTAGSIZE_T8   = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_tx_vtag_op {
    NOP		= 0x0,
    VTAG_INSERT	= 0x1,
    VTAG_REPLACE	= 0x2,
}

// NIX RX VTAG actions

// NIX TX stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_stat_lf_tx {
    TX_UCAST	= 0x0,
    TX_BCAST	= 0x1,
    TX_MCAST	= 0x2,
    TX_DROP		= 0x3,
    TX_OCTS		= 0x4,
    TX_STATS_ENUM_LAST,
}

// NIX RX stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nix_stat_lf_rx {
    RX_OCTS		= 0x0,
    RX_UCAST	= 0x1,
    RX_BCAST	= 0x2,
    RX_MCAST	= 0x3,
    RX_DROP		= 0x4,
    RX_DROP_OCTS	= 0x5,
    RX_FCS		= 0x6,
    RX_ERR		= 0x7,
    RX_DRP_BCAST	= 0x8,
    RX_DRP_MCAST	= 0x9,
    RX_DRP_L3BCAST	= 0xa,
    RX_DRP_L3MCAST	= 0xb,
    RX_STATS_ENUM_LAST,
}
