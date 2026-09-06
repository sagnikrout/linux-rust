//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_hdcp.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2017 Google, Inc.
//
// Authors:
// Sean Paul <seanpaul@chromium.org>
//

// Period of hdcp checks (to ensure we're still authenticated)

pub const DRM_HDCP2_CHECK_PERIOD_MS: c_int = 500;
// Shared lengths/masks between HDMI/DVI/DisplayPort
pub const DRM_HDCP_AN_LEN: c_int = 8;
pub const DRM_HDCP_BSTATUS_LEN: c_int = 2;
pub const DRM_HDCP_KSV_LEN: c_int = 5;
pub const DRM_HDCP_RI_LEN: c_int = 2;
pub const DRM_HDCP_V_PRIME_PART_LEN: c_int = 4;
pub const DRM_HDCP_V_PRIME_NUM_PARTS: c_int = 5;

// Slave address for the HDCP registers in the receiver
pub const DRM_HDCP_DDC_ADDR: c_uint = 0x3A;
// Value to use at the end of the SHA-1 bytestream used for repeaters
pub const DRM_HDCP_SHA1_TERMINATOR: c_uint = 0x80;
// HDCP register offsets for HDMI/DVI devices
pub const DRM_HDCP_DDC_BKSV: c_uint = 0x00;
pub const DRM_HDCP_DDC_RI_PRIME: c_uint = 0x08;
pub const DRM_HDCP_DDC_AKSV: c_uint = 0x10;
pub const DRM_HDCP_DDC_AN: c_uint = 0x18;

pub const DRM_HDCP_DDC_BCAPS: c_uint = 0x40;

pub const DRM_HDCP_DDC_BSTATUS: c_uint = 0x41;
pub const DRM_HDCP_DDC_KSV_FIFO: c_uint = 0x43;
pub const DRM_HDCP_1_4_SRM_ID: c_uint = 0x8;
pub const DRM_HDCP_1_4_VRL_LENGTH_SIZE: c_int = 3;
pub const DRM_HDCP_1_4_DCP_SIG_SIZE: c_int = 40;
// Protocol message definition for HDCP2.2 specification
//
// Protected content streams are classified into 2 types:
// - Type0: Can be transmitted with HDCP 1.4+
// - Type1: Can be transmitted with HDCP 2.2+
//
pub const HDCP_STREAM_TYPE0: c_uint = 0x00;
pub const HDCP_STREAM_TYPE1: c_uint = 0x01;
// HDCP2.2 Msg IDs
pub const HDCP_2_2_NULL_MSG: c_int = 1;
pub const HDCP_2_2_AKE_INIT: c_int = 2;
pub const HDCP_2_2_AKE_SEND_CERT: c_int = 3;
pub const HDCP_2_2_AKE_NO_STORED_KM: c_int = 4;
pub const HDCP_2_2_AKE_STORED_KM: c_int = 5;
pub const HDCP_2_2_AKE_SEND_HPRIME: c_int = 7;
pub const HDCP_2_2_AKE_SEND_PAIRING_INFO: c_int = 8;
pub const HDCP_2_2_LC_INIT: c_int = 9;
pub const HDCP_2_2_LC_SEND_LPRIME: c_int = 10;
pub const HDCP_2_2_SKE_SEND_EKS: c_int = 11;
pub const HDCP_2_2_REP_SEND_RECVID_LIST: c_int = 12;
pub const HDCP_2_2_REP_SEND_ACK: c_int = 15;
pub const HDCP_2_2_REP_STREAM_MANAGE: c_int = 16;
pub const HDCP_2_2_REP_STREAM_READY: c_int = 17;
pub const HDCP_2_2_RTX_LEN: c_int = 8;
pub const HDCP_2_2_RRX_LEN: c_int = 8;
pub const HDCP_2_2_K_PUB_RX_MOD_N_LEN: c_int = 128;
pub const HDCP_2_2_K_PUB_RX_EXP_E_LEN: c_int = 3;

pub const HDCP_2_2_DCP_LLC_SIG_LEN: c_int = 384;
pub const HDCP_2_2_E_KPUB_KM_LEN: c_int = 128;

pub const HDCP_2_2_H_PRIME_LEN: c_int = 32;
pub const HDCP_2_2_E_KH_KM_LEN: c_int = 16;
pub const HDCP_2_2_RN_LEN: c_int = 8;
pub const HDCP_2_2_L_PRIME_LEN: c_int = 32;
pub const HDCP_2_2_E_DKEY_KS_LEN: c_int = 16;
pub const HDCP_2_2_RIV_LEN: c_int = 8;
pub const HDCP_2_2_SEQ_NUM_LEN: c_int = 3;

pub const HDCP_2_2_MAX_DEVICE_COUNT: c_int = 31;

pub const HDCP_2_2_MPRIME_LEN: c_int = 32;
// Following Macros take a byte at a time for bit(s) masking
//
// TODO: HDCP_2_2_MAX_CONTENT_STREAMS_CNT is based upon actual
// H/W MST streams capacity.
// This required to be moved out to platform specific header.
//
pub const HDCP_2_2_MAX_CONTENT_STREAMS_CNT: c_int = 4;
pub const HDCP_2_2_TXCAP_MASK_LEN: c_int = 2;
pub const HDCP_2_2_RXCAPS_LEN: c_int = 3;

pub const HDCP_2_2_RXINFO_LEN: c_int = 2;
// HDCP1.x compliant device in downstream

// HDCP2.0 Compliant repeater in downstream

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_cert_rx {
    pub receiver_id: [u8; HDCP_2_2_RECEIVER_ID_LEN],
    pub kpub_rx: [u8; HDCP_2_2_K_PUB_RX_LEN],
    pub reserved: [u8; 2],
    pub dcp_signature: [u8; HDCP_2_2_DCP_LLC_SIG_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_streamid_type {
    pub stream_id: u8,
    pub stream_type: u8,
    pub __packed: },
//
// The TxCaps field specified in the HDCP HDMI, DP specs
// This field is big endian as specified in the errata.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_tx_caps {
// Transmitter must set this to 0x2
    pub version: u8,
// Reserved for HDCP and DP Spec. Read as Zero
    pub tx_cap_mask: [u8; HDCP_2_2_TXCAP_MASK_LEN],
    pub __packed: },
// Main structures for HDCP2.2 protocol communication
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_init {
    pub msg_id: u8,
    pub r_tx: [u8; HDCP_2_2_RTX_LEN],
    pub tx_caps: hdcp2_tx_caps,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_send_cert {
    pub msg_id: u8,
    pub cert_rx: hdcp2_cert_rx,
    pub r_rx: [u8; HDCP_2_2_RRX_LEN],
    pub rx_caps: [u8; HDCP_2_2_RXCAPS_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_no_stored_km {
    pub msg_id: u8,
    pub e_kpub_km: [u8; HDCP_2_2_E_KPUB_KM_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_stored_km {
    pub msg_id: u8,
    pub e_kh_km_m: [u8; HDCP_2_2_E_KH_KM_M_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_send_hprime {
    pub msg_id: u8,
    pub h_prime: [u8; HDCP_2_2_H_PRIME_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ake_send_pairing_info {
    pub msg_id: u8,
    pub e_kh_km: [u8; HDCP_2_2_E_KH_KM_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_lc_init {
    pub msg_id: u8,
    pub r_n: [u8; HDCP_2_2_RN_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_lc_send_lprime {
    pub msg_id: u8,
    pub l_prime: [u8; HDCP_2_2_L_PRIME_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_ske_send_eks {
    pub msg_id: u8,
    pub e_dkey_ks: [u8; HDCP_2_2_E_DKEY_KS_LEN],
    pub riv: [u8; HDCP_2_2_RIV_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_rep_send_receiverid_list {
    pub msg_id: u8,
    pub rx_info: [u8; HDCP_2_2_RXINFO_LEN],
    pub seq_num_v: [u8; HDCP_2_2_SEQ_NUM_LEN],
    pub v_prime: [u8; HDCP_2_2_V_PRIME_HALF_LEN],
    pub receiver_ids: [u8; HDCP_2_2_RECEIVER_IDS_MAX_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_rep_send_ack {
    pub msg_id: u8,
    pub v: [u8; HDCP_2_2_V_PRIME_HALF_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_rep_stream_manage {
    pub msg_id: u8,
    pub seq_num_m: [u8; HDCP_2_2_SEQ_NUM_LEN],
    pub k: __be16,
    pub streams: [hdcp2_streamid_type; HDCP_2_2_MAX_CONTENT_STREAMS_CNT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp2_rep_stream_ready {
    pub msg_id: u8,
    pub m_prime: [u8; HDCP_2_2_MPRIME_LEN],
    pub __packed: },
// HDCP2.2 TIMEOUTs in mSec
pub const HDCP_2_2_CERT_TIMEOUT_MS: c_int = 100;
pub const HDCP_2_2_DP_CERT_READ_TIMEOUT_MS: c_int = 110;
pub const HDCP_2_2_HPRIME_NO_PAIRED_TIMEOUT_MS: c_int = 1000;
pub const HDCP_2_2_HPRIME_PAIRED_TIMEOUT_MS: c_int = 200;
pub const HDCP_2_2_DP_HPRIME_READ_TIMEOUT_MS: c_int = 7;
pub const HDCP_2_2_PAIRING_TIMEOUT_MS: c_int = 200;
pub const HDCP_2_2_DP_PAIRING_READ_TIMEOUT_MS: c_int = 5;
pub const HDCP_2_2_HDMI_LPRIME_TIMEOUT_MS: c_int = 20;
pub const HDCP_2_2_DP_LPRIME_TIMEOUT_MS: c_int = 16;
pub const HDCP_2_2_RECVID_LIST_TIMEOUT_MS: c_int = 3000;
pub const HDCP_2_2_STREAM_READY_TIMEOUT_MS: c_int = 100;
// HDMI HDCP2.2 Register Offsets
pub const HDCP_2_2_HDMI_REG_VER_OFFSET: c_uint = 0x50;
pub const HDCP_2_2_HDMI_REG_WR_MSG_OFFSET: c_uint = 0x60;
pub const HDCP_2_2_HDMI_REG_RXSTATUS_OFFSET: c_uint = 0x70;
pub const HDCP_2_2_HDMI_REG_RD_MSG_OFFSET: c_uint = 0x80;
pub const HDCP_2_2_HDMI_REG_DBG_OFFSET: c_uint = 0xC0;

pub const HDCP_2_2_RX_CAPS_VERSION_VAL: c_uint = 0x02;
pub const HDCP_2_2_SEQ_NUM_MAX: c_uint = 0xFFFFFF;
pub const HDCP_2_2_DELAY_BEFORE_ENCRYPTION_EN: c_int = 200;
// Below macros take a byte at a time and mask the bit(s)
pub const HDCP_2_2_HDMI_RXSTATUS_LEN: c_int = 2;

//
// Helper functions to convert 24bit big endian hdcp sequence number to
// host format and back
//
    pub 16): return (u32)(seq_num[2] | seq_num[1] << 8 | seq_num[0] <<,
    pub 16: seq_num[0] = val >>,
    pub 8: seq_num[1] = val >>,
    pub val: seq_num[2] =,

pub const DRM_HDCP_1_4_SRM_ID: c_uint = 0x8;

pub const DRM_HDCP_1_4_VRL_LENGTH_SIZE: c_int = 3;
pub const DRM_HDCP_1_4_DCP_SIG_SIZE: c_int = 40;
pub const DRM_HDCP_2_SRM_ID: c_uint = 0x9;
pub const DRM_HDCP_2_INDICATOR: c_uint = 0x1;
pub const DRM_HDCP_2_INDICATOR_MASK: c_uint = 0xF;
pub const DRM_HDCP_2_VRL_LENGTH_SIZE: c_int = 3;
pub const DRM_HDCP_2_DCP_SIG_SIZE: c_int = 384;
pub const DRM_HDCP_2_NO_OF_DEV_PLUS_RESERVED_SZ: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp_srm_header {
    pub srm_id: u8,
    pub reserved: u8,
    pub srm_version: __be16,
    pub srm_gen_no: u8,
    pub __packed: },
// Content Type classification for HDCP2.2 vs others
pub const DRM_MODE_HDCP_CONTENT_TYPE0: c_int = 0;
pub const DRM_MODE_HDCP_CONTENT_TYPE1: c_int = 1;
