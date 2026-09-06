//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/cn10k_ipsec.h
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
// Marvell IPSEC offload driver
//
// Copyright (C) 2024 Marvell.
//

// CPT instruction size in bytes
pub const CN10K_CPT_INST_SIZE: c_int = 64;
// CPT instruction (CPT_INST_S) queue length
pub const CN10K_CPT_INST_QLEN: c_int = 8200;
// CPT instruction queue size passed to HW is in units of
// 40*CPT_INST_S messages.
//

// CPT needs 320 free entries

// CPT instruction queue length in bytes

// CPT instruction group queue length in bytes

// CPT FC length in bytes
pub const CN10K_CPT_Q_FC_LEN: c_int = 128;
// Default CPT engine group for ipsec offload
pub const CN10K_DEF_CPT_IPSEC_EGRP: c_int = 1;
// CN10K CPT LF registers

// IPSEC Instruction opcodes
pub const CN10K_IPSEC_MAJOR_OP_WRITE_SA: c_uint = 0x01UL;
pub const CN10K_IPSEC_MINOR_OP_WRITE_SA: c_uint = 0x09UL;
pub const CN10K_IPSEC_MAJOR_OP_OUTB_IPSEC: c_uint = 0x2AUL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cn10k_cpt_comp_e {
    CN10K_CPT_COMP_E_NOTDONE = 0x00,
    CN10K_CPT_COMP_E_GOOD = 0x01,
    CN10K_CPT_COMP_E_FAULT = 0x02,
    CN10K_CPT_COMP_E_HWERR = 0x04,
    CN10K_CPT_COMP_E_INSTERR = 0x05,
    CN10K_CPT_COMP_E_WARN = 0x06,
    CN10K_CPT_COMP_E_MASK = 0x3F
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_cpt_inst_queue {
    pub vaddr: *mut u8,
    pub real_vaddr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub real_dma_addr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cn10k_cpt_hw_state_e {
    CN10K_CPT_HW_UNAVAILABLE,
    CN10K_CPT_HW_AVAILABLE,
    CN10K_CPT_HW_IN_USE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_ipsec {
// Outbound CPT
    pub io_addr: u64,
    pub cpt_state: core::sync::atomic::AtomicI32,
    pub iq: cn10k_cpt_inst_queue,
// SA info
    pub sa_size: u32,
    pub outb_sa_count: u32,
    pub sa_work: work_struct,
    pub sa_workq: *mut workqueue_struct,
}

// CN10K IPSEC Security Association (SA)
// SA direction
pub const CN10K_IPSEC_SA_DIR_INB: c_int = 0;
pub const CN10K_IPSEC_SA_DIR_OUTB: c_int = 1;
// SA protocol
pub const CN10K_IPSEC_SA_IPSEC_PROTO_AH: c_int = 0;
pub const CN10K_IPSEC_SA_IPSEC_PROTO_ESP: c_int = 1;
// SA Encryption Type
pub const CN10K_IPSEC_SA_ENCAP_TYPE_AES_GCM: c_int = 5;
// SA IPSEC mode Transport/Tunnel
pub const CN10K_IPSEC_SA_IPSEC_MODE_TRANSPORT: c_int = 0;
pub const CN10K_IPSEC_SA_IPSEC_MODE_TUNNEL: c_int = 1;
// SA AES Key Length
pub const CN10K_IPSEC_SA_AES_KEY_LEN_128: c_int = 1;
pub const CN10K_IPSEC_SA_AES_KEY_LEN_192: c_int = 2;
pub const CN10K_IPSEC_SA_AES_KEY_LEN_256: c_int = 3;
// IV Source
pub const CN10K_IPSEC_SA_IV_SRC_COUNTER: c_int = 0;
pub const CN10K_IPSEC_SA_IV_SRC_PACKET: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_tx_sa_s {
    pub /: *mut *mut u64 esn_en : 1; / W0,
    pub 8: u64 rsvd_w0_1_8 :,
    pub 7: u64 hw_ctx_off :,
    pub 16: u64 ctx_id :,
    pub 16: u64 rsvd_w0_32_47 :,
    pub 7: u64 ctx_push_size :,
    pub 1: u64 rsvd_w0_55 :,
    pub 2: u64 ctx_hdr_size :,
    pub 1: u64 aop_valid :,
    pub 1: u64 rsvd_w0_59 :,
    pub 4: u64 ctx_size :,
    pub /: *mut *mut u64 w1; / W1,
    pub /: *mut *mut u64 sa_valid : 1; / W2,
    pub 1: u64 sa_dir :,
    pub 2: u64 rsvd_w2_2_3 :,
    pub 1: u64 ipsec_mode :,
    pub 1: u64 ipsec_protocol :,
    pub 2: u64 aes_key_len :,
    pub 3: u64 enc_type :,
    pub 9: u64 rsvd_w2_11_19 :,
    pub 2: u64 iv_src :,
    pub 10: u64 rsvd_w2_22_31 :,
    pub 32: u64 rsvd_w2_32_63 :,
    pub /: *mut *mut u64 w3; / W3,
    pub /: *mut *mut u8 cipher_key[32]; / W4 - W7,
    pub /: *mut *mut u32 rsvd_w8_0_31; / W8 : IV,
    pub iv_gcm_salt: u32,
    pub /: *mut *mut u64 rsvd_w9_w30[22]; / W9 - W30,
    pub /: *mut *mut u64 hw_ctx[6]; / W31 - W36,
}

// CPT instruction parameter-1
pub const CN10K_IPSEC_INST_PARAM1_DIS_L4_CSUM: c_uint = 0x1;
pub const CN10K_IPSEC_INST_PARAM1_DIS_L3_CSUM: c_uint = 0x2;
pub const CN10K_IPSEC_INST_PARAM1_CRYPTO_MODE: c_uint = 0x20;
pub const CN10K_IPSEC_INST_PARAM1_IV_OFFSET_SHIFT: c_int = 8;
// CPT instruction parameter-2
pub const CN10K_IPSEC_INST_PARAM2_ENC_DATA_OFFSET_SHIFT: c_int = 0;
pub const CN10K_IPSEC_INST_PARAM2_AUTH_DATA_OFFSET_SHIFT: c_int = 8;
// CPT Instruction Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_inst_s {
    pub /: *mut *mut u64 nixtxl : 3; / W0,
    pub 1: u64 doneint :,
    pub 12: u64 rsvd_w0_4_15 :,
    pub 8: u64 dat_offset :,
    pub 8: u64 ext_param1 :,
    pub 20: u64 nixtx_offset :,
    pub 12: u64 rsvd_w0_52_63 :,
    pub /: *mut *mut u64 res_addr; / W1,
    pub /: *mut *mut u64 tag : 32; / W2,
    pub 2: u64 tt :,
    pub 10: u64 grp :,
    pub 4: u64 rsvd_w2_44_47 :,
    pub 16: u64 rvu_pf_func :,
    pub /: *mut *mut u64 qord : 1; / W3,
    pub 2: u64 rsvd_w3_1_2 :,
    pub 61: u64 wqe_ptr :,
    pub /: *mut *mut u64 dlen : 16; / W4,
    pub 16: u64 param2 :,
    pub 16: u64 param1 :,
    pub 8: u64 opcode_major :,
    pub 8: u64 opcode_minor :,
    pub /: *mut *mut u64 dptr; / W5,
    pub /: *mut *mut u64 rptr; / W6,
    pub /: *mut *mut u64 cptr : 60; / W7,
    pub 1: u64 ctx_val :,
    pub 3: u64 egrp :,
}

// CPT Instruction Result Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_res_s {
    pub /: *mut *mut u64 compcode : 7; / W0,
    pub 1: u64 doneint :,
    pub 8: u64 uc_compcode :,
    pub 48: u64 uc_info :,
    pub /: *mut *mut u64 esn; / W1,
}

// CPT SG structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_sg_s {
    pub 16: u64 seg1_size :,
    pub 16: u64 seg2_size :,
    pub 16: u64 seg3_size :,
    pub 2: u64 segs :,
    pub 14: u64 rsvd_63_50 :,
}

// CPT LF_INPROG Register

// CPT LF_Q_GRP_PTR Register

// CPT LF_Q_SIZE Register

// CPT LF_Q_SIZE Register

// CPT LF CTX Flush Register

extern "C" {
    pub fn cn10k_ipsec_init(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cn10k_ipsec_clean(pf: *mut otx2_nic);
}
extern "C" {
    pub fn cn10k_ipsec_ethtool_init(netdev: *mut net_device, enable: bool) -> c_int;
}

