//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/cxd2880/cxd2880_dvbt2.h
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
//
// cxd2880_dvbt2.h
// Sony CXD2880 DVB-T2/T tuner + demodulator driver
// DVB-T2 related definitions
//
// Copyright (C) 2016, 2017, 2018 Sony Semiconductor Solutions Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_profile {
    CXD2880_DVBT2_PROFILE_BASE,
    CXD2880_DVBT2_PROFILE_LITE,
    CXD2880_DVBT2_PROFILE_ANY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_version {
    CXD2880_DVBT2_V111,
    CXD2880_DVBT2_V121,
    CXD2880_DVBT2_V131
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_s1 {
    CXD2880_DVBT2_S1_BASE_SISO = 0x00,
    CXD2880_DVBT2_S1_BASE_MISO = 0x01,
    CXD2880_DVBT2_S1_NON_DVBT2 = 0x02,
    CXD2880_DVBT2_S1_LITE_SISO = 0x03,
    CXD2880_DVBT2_S1_LITE_MISO = 0x04,
    CXD2880_DVBT2_S1_RSVD3 = 0x05,
    CXD2880_DVBT2_S1_RSVD4 = 0x06,
    CXD2880_DVBT2_S1_RSVD5 = 0x07,
    CXD2880_DVBT2_S1_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_base_s2 {
    CXD2880_DVBT2_BASE_S2_M2K_G_ANY = 0x00,
    CXD2880_DVBT2_BASE_S2_M8K_G_DVBT = 0x01,
    CXD2880_DVBT2_BASE_S2_M4K_G_ANY = 0x02,
    CXD2880_DVBT2_BASE_S2_M1K_G_ANY = 0x03,
    CXD2880_DVBT2_BASE_S2_M16K_G_ANY = 0x04,
    CXD2880_DVBT2_BASE_S2_M32K_G_DVBT = 0x05,
    CXD2880_DVBT2_BASE_S2_M8K_G_DVBT2 = 0x06,
    CXD2880_DVBT2_BASE_S2_M32K_G_DVBT2 = 0x07,
    CXD2880_DVBT2_BASE_S2_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_lite_s2 {
    CXD2880_DVBT2_LITE_S2_M2K_G_ANY = 0x00,
    CXD2880_DVBT2_LITE_S2_M8K_G_DVBT = 0x01,
    CXD2880_DVBT2_LITE_S2_M4K_G_ANY = 0x02,
    CXD2880_DVBT2_LITE_S2_M16K_G_DVBT2 = 0x03,
    CXD2880_DVBT2_LITE_S2_M16K_G_DVBT = 0x04,
    CXD2880_DVBT2_LITE_S2_RSVD1 = 0x05,
    CXD2880_DVBT2_LITE_S2_M8K_G_DVBT2 = 0x06,
    CXD2880_DVBT2_LITE_S2_RSVD2 = 0x07,
    CXD2880_DVBT2_LITE_S2_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_guard {
    CXD2880_DVBT2_G1_32 = 0x00,
    CXD2880_DVBT2_G1_16 = 0x01,
    CXD2880_DVBT2_G1_8 = 0x02,
    CXD2880_DVBT2_G1_4 = 0x03,
    CXD2880_DVBT2_G1_128 = 0x04,
    CXD2880_DVBT2_G19_128 = 0x05,
    CXD2880_DVBT2_G19_256 = 0x06,
    CXD2880_DVBT2_G_RSVD1 = 0x07,
    CXD2880_DVBT2_G_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_mode {
    CXD2880_DVBT2_M2K = 0x00,
    CXD2880_DVBT2_M8K = 0x01,
    CXD2880_DVBT2_M4K = 0x02,
    CXD2880_DVBT2_M1K = 0x03,
    CXD2880_DVBT2_M16K = 0x04,
    CXD2880_DVBT2_M32K = 0x05,
    CXD2880_DVBT2_M_RSVD1 = 0x06,
    CXD2880_DVBT2_M_RSVD2 = 0x07
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_bw {
    CXD2880_DVBT2_BW_8 = 0x00,
    CXD2880_DVBT2_BW_7 = 0x01,
    CXD2880_DVBT2_BW_6 = 0x02,
    CXD2880_DVBT2_BW_5 = 0x03,
    CXD2880_DVBT2_BW_10 = 0x04,
    CXD2880_DVBT2_BW_1_7 = 0x05,
    CXD2880_DVBT2_BW_RSVD1 = 0x06,
    CXD2880_DVBT2_BW_RSVD2 = 0x07,
    CXD2880_DVBT2_BW_RSVD3 = 0x08,
    CXD2880_DVBT2_BW_RSVD4 = 0x09,
    CXD2880_DVBT2_BW_RSVD5 = 0x0a,
    CXD2880_DVBT2_BW_RSVD6 = 0x0b,
    CXD2880_DVBT2_BW_RSVD7 = 0x0c,
    CXD2880_DVBT2_BW_RSVD8 = 0x0d,
    CXD2880_DVBT2_BW_RSVD9 = 0x0e,
    CXD2880_DVBT2_BW_RSVD10 = 0x0f,
    CXD2880_DVBT2_BW_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_l1pre_type {
    CXD2880_DVBT2_L1PRE_TYPE_TS = 0x00,
    CXD2880_DVBT2_L1PRE_TYPE_GS = 0x01,
    CXD2880_DVBT2_L1PRE_TYPE_TS_GS = 0x02,
    CXD2880_DVBT2_L1PRE_TYPE_RESERVED = 0x03,
    CXD2880_DVBT2_L1PRE_TYPE_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_papr {
    CXD2880_DVBT2_PAPR_0 = 0x00,
    CXD2880_DVBT2_PAPR_1 = 0x01,
    CXD2880_DVBT2_PAPR_2 = 0x02,
    CXD2880_DVBT2_PAPR_3 = 0x03,
    CXD2880_DVBT2_PAPR_RSVD1 = 0x04,
    CXD2880_DVBT2_PAPR_RSVD2 = 0x05,
    CXD2880_DVBT2_PAPR_RSVD3 = 0x06,
    CXD2880_DVBT2_PAPR_RSVD4 = 0x07,
    CXD2880_DVBT2_PAPR_RSVD5 = 0x08,
    CXD2880_DVBT2_PAPR_RSVD6 = 0x09,
    CXD2880_DVBT2_PAPR_RSVD7 = 0x0a,
    CXD2880_DVBT2_PAPR_RSVD8 = 0x0b,
    CXD2880_DVBT2_PAPR_RSVD9 = 0x0c,
    CXD2880_DVBT2_PAPR_RSVD10 = 0x0d,
    CXD2880_DVBT2_PAPR_RSVD11 = 0x0e,
    CXD2880_DVBT2_PAPR_RSVD12 = 0x0f,
    CXD2880_DVBT2_PAPR_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_l1post_constell {
    CXD2880_DVBT2_L1POST_BPSK = 0x00,
    CXD2880_DVBT2_L1POST_QPSK = 0x01,
    CXD2880_DVBT2_L1POST_QAM16 = 0x02,
    CXD2880_DVBT2_L1POST_QAM64 = 0x03,
    CXD2880_DVBT2_L1POST_C_RSVD1 = 0x04,
    CXD2880_DVBT2_L1POST_C_RSVD2 = 0x05,
    CXD2880_DVBT2_L1POST_C_RSVD3 = 0x06,
    CXD2880_DVBT2_L1POST_C_RSVD4 = 0x07,
    CXD2880_DVBT2_L1POST_C_RSVD5 = 0x08,
    CXD2880_DVBT2_L1POST_C_RSVD6 = 0x09,
    CXD2880_DVBT2_L1POST_C_RSVD7 = 0x0a,
    CXD2880_DVBT2_L1POST_C_RSVD8 = 0x0b,
    CXD2880_DVBT2_L1POST_C_RSVD9 = 0x0c,
    CXD2880_DVBT2_L1POST_C_RSVD10 = 0x0d,
    CXD2880_DVBT2_L1POST_C_RSVD11 = 0x0e,
    CXD2880_DVBT2_L1POST_C_RSVD12 = 0x0f,
    CXD2880_DVBT2_L1POST_CONSTELL_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_l1post_cr {
    CXD2880_DVBT2_L1POST_R1_2 = 0x00,
    CXD2880_DVBT2_L1POST_R_RSVD1 = 0x01,
    CXD2880_DVBT2_L1POST_R_RSVD2 = 0x02,
    CXD2880_DVBT2_L1POST_R_RSVD3 = 0x03,
    CXD2880_DVBT2_L1POST_R_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_l1post_fec_type {
    CXD2880_DVBT2_L1POST_FEC_LDPC16K = 0x00,
    CXD2880_DVBT2_L1POST_FEC_RSVD1 = 0x01,
    CXD2880_DVBT2_L1POST_FEC_RSVD2 = 0x02,
    CXD2880_DVBT2_L1POST_FEC_RSVD3 = 0x03,
    CXD2880_DVBT2_L1POST_FEC_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_pp {
    CXD2880_DVBT2_PP1 = 0x00,
    CXD2880_DVBT2_PP2 = 0x01,
    CXD2880_DVBT2_PP3 = 0x02,
    CXD2880_DVBT2_PP4 = 0x03,
    CXD2880_DVBT2_PP5 = 0x04,
    CXD2880_DVBT2_PP6 = 0x05,
    CXD2880_DVBT2_PP7 = 0x06,
    CXD2880_DVBT2_PP8 = 0x07,
    CXD2880_DVBT2_PP_RSVD1 = 0x08,
    CXD2880_DVBT2_PP_RSVD2 = 0x09,
    CXD2880_DVBT2_PP_RSVD3 = 0x0a,
    CXD2880_DVBT2_PP_RSVD4 = 0x0b,
    CXD2880_DVBT2_PP_RSVD5 = 0x0c,
    CXD2880_DVBT2_PP_RSVD6 = 0x0d,
    CXD2880_DVBT2_PP_RSVD7 = 0x0e,
    CXD2880_DVBT2_PP_RSVD8 = 0x0f,
    CXD2880_DVBT2_PP_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_code_rate {
    CXD2880_DVBT2_R1_2 = 0x00,
    CXD2880_DVBT2_R3_5 = 0x01,
    CXD2880_DVBT2_R2_3 = 0x02,
    CXD2880_DVBT2_R3_4 = 0x03,
    CXD2880_DVBT2_R4_5 = 0x04,
    CXD2880_DVBT2_R5_6 = 0x05,
    CXD2880_DVBT2_R1_3 = 0x06,
    CXD2880_DVBT2_R2_5 = 0x07,
    CXD2880_DVBT2_PLP_CR_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_constell {
    CXD2880_DVBT2_QPSK = 0x00,
    CXD2880_DVBT2_QAM16 = 0x01,
    CXD2880_DVBT2_QAM64 = 0x02,
    CXD2880_DVBT2_QAM256 = 0x03,
    CXD2880_DVBT2_CON_RSVD1 = 0x04,
    CXD2880_DVBT2_CON_RSVD2 = 0x05,
    CXD2880_DVBT2_CON_RSVD3 = 0x06,
    CXD2880_DVBT2_CON_RSVD4 = 0x07,
    CXD2880_DVBT2_CONSTELL_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_type {
    CXD2880_DVBT2_PLP_TYPE_COMMON = 0x00,
    CXD2880_DVBT2_PLP_TYPE_DATA1 = 0x01,
    CXD2880_DVBT2_PLP_TYPE_DATA2 = 0x02,
    CXD2880_DVBT2_PLP_TYPE_RSVD1 = 0x03,
    CXD2880_DVBT2_PLP_TYPE_RSVD2 = 0x04,
    CXD2880_DVBT2_PLP_TYPE_RSVD3 = 0x05,
    CXD2880_DVBT2_PLP_TYPE_RSVD4 = 0x06,
    CXD2880_DVBT2_PLP_TYPE_RSVD5 = 0x07,
    CXD2880_DVBT2_PLP_TYPE_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_payload {
    CXD2880_DVBT2_PLP_PAYLOAD_GFPS = 0x00,
    CXD2880_DVBT2_PLP_PAYLOAD_GCS = 0x01,
    CXD2880_DVBT2_PLP_PAYLOAD_GSE = 0x02,
    CXD2880_DVBT2_PLP_PAYLOAD_TS = 0x03,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD1 = 0x04,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD2 = 0x05,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD3 = 0x06,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD4 = 0x07,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD5 = 0x08,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD6 = 0x09,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD7 = 0x0a,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD8 = 0x0b,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD9 = 0x0c,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD10 = 0x0d,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD11 = 0x0e,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD12 = 0x0f,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD13 = 0x10,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD14 = 0x11,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD15 = 0x12,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD16 = 0x13,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD17 = 0x14,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD18 = 0x15,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD19 = 0x16,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD20 = 0x17,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD21 = 0x18,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD22 = 0x19,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD23 = 0x1a,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD24 = 0x1b,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD25 = 0x1c,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD26 = 0x1d,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD27 = 0x1e,
    CXD2880_DVBT2_PLP_PAYLOAD_RSVD28 = 0x1f,
    CXD2880_DVBT2_PLP_PAYLOAD_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_fec {
    CXD2880_DVBT2_FEC_LDPC_16K = 0x00,
    CXD2880_DVBT2_FEC_LDPC_64K = 0x01,
    CXD2880_DVBT2_FEC_RSVD1 = 0x02,
    CXD2880_DVBT2_FEC_RSVD2 = 0x03,
    CXD2880_DVBT2_FEC_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_mode {
    CXD2880_DVBT2_PLP_MODE_NOTSPECIFIED = 0x00,
    CXD2880_DVBT2_PLP_MODE_NM = 0x01,
    CXD2880_DVBT2_PLP_MODE_HEM = 0x02,
    CXD2880_DVBT2_PLP_MODE_RESERVED = 0x03,
    CXD2880_DVBT2_PLP_MODE_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_plp_btype {
    CXD2880_DVBT2_PLP_COMMON,
    CXD2880_DVBT2_PLP_DATA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxd2880_dvbt2_stream {
    CXD2880_DVBT2_STREAM_GENERIC_PACKETIZED = 0x00,
    CXD2880_DVBT2_STREAM_GENERIC_CONTINUOUS = 0x01,
    CXD2880_DVBT2_STREAM_GENERIC_ENCAPSULATED = 0x02,
    CXD2880_DVBT2_STREAM_TRANSPORT = 0x03,
    CXD2880_DVBT2_STREAM_UNKNOWN = 0xff
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_l1pre {
    pub type: cxd2880_dvbt2_l1pre_type,
    pub bw_ext: u8,
    pub s1: cxd2880_dvbt2_s1,
    pub s2: u8,
    pub mixed: u8,
    pub fft_mode: cxd2880_dvbt2_mode,
    pub l1_rep: u8,
    pub gi: cxd2880_dvbt2_guard,
    pub papr: cxd2880_dvbt2_papr,
    pub mod: cxd2880_dvbt2_l1post_constell,
    pub cr: cxd2880_dvbt2_l1post_cr,
    pub fec: cxd2880_dvbt2_l1post_fec_type,
    pub l1_post_size: u32,
    pub l1_post_info_size: u32,
    pub pp: cxd2880_dvbt2_pp,
    pub tx_id_availability: u8,
    pub cell_id: u16,
    pub network_id: u16,
    pub sys_id: u16,
    pub num_frames: u8,
    pub num_symbols: u16,
    pub regen: u8,
    pub post_ext: u8,
    pub num_rf_freqs: u8,
    pub rf_idx: u8,
    pub t2_version: cxd2880_dvbt2_version,
    pub l1_post_scrambled: u8,
    pub t2_base_lite: u8,
    pub crc32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_plp {
    pub id: u8,
    pub type: cxd2880_dvbt2_plp_type,
    pub payload: cxd2880_dvbt2_plp_payload,
    pub ff: u8,
    pub first_rf_idx: u8,
    pub first_frm_idx: u8,
    pub group_id: u8,
    pub constell: cxd2880_dvbt2_plp_constell,
    pub plp_cr: cxd2880_dvbt2_plp_code_rate,
    pub rot: u8,
    pub fec: cxd2880_dvbt2_plp_fec,
    pub num_blocks_max: u16,
    pub frm_int: u8,
    pub til_len: u8,
    pub til_type: u8,
    pub in_band_a_flag: u8,
    pub in_band_b_flag: u8,
    pub rsvd: u16,
    pub plp_mode: cxd2880_dvbt2_plp_mode,
    pub static_flag: u8,
    pub static_padding_flag: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_l1post {
    pub sub_slices_per_frame: u16,
    pub num_plps: u8,
    pub num_aux: u8,
    pub aux_cfg_rfu: u8,
    pub rf_idx: u8,
    pub freq: u32,
    pub fef_type: u8,
    pub fef_length: u32,
    pub fef_intvl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_ofdm {
    pub mixed: u8,
    pub is_miso: u8,
    pub mode: cxd2880_dvbt2_mode,
    pub gi: cxd2880_dvbt2_guard,
    pub pp: cxd2880_dvbt2_pp,
    pub bw_ext: u8,
    pub papr: cxd2880_dvbt2_papr,
    pub num_symbols: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxd2880_dvbt2_bbheader {
    pub stream_input: cxd2880_dvbt2_stream,
    pub is_single_input_stream: u8,
    pub is_constant_coding_modulation: u8,
    pub issy_indicator: u8,
    pub null_packet_deletion: u8,
    pub ext: u8,
    pub input_stream_identifier: u8,
    pub user_packet_length: u16,
    pub data_field_length: u16,
    pub sync_byte: u8,
    pub issy: u32,
    pub plp_mode: cxd2880_dvbt2_plp_mode,
}
