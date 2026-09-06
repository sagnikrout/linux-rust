//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/desc.h
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


//
// Copyright (c) 2004-2008 Reyk Floeter <reyk@openbsd.org>
// Copyright (c) 2006-2008 Nick Kossifidis <mickflemm@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// RX/TX descriptor structures
//
// struct ath5k_hw_rx_ctl - Common hardware RX control descriptor
// @rx_control_0: RX control word 0
// @rx_control_1: RX control word 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_rx_ctl {
    pub rx_control_0: u32,
    pub rx_control_1: u32,
    pub __aligned(4): } __packed,
// RX control word 1 fields/flags
pub const AR5K_DESC_RX_CTL1_BUF_LEN: c_uint = 0x00000fff /* data buffer length */;
pub const AR5K_DESC_RX_CTL1_INTREQ: c_uint = 0x00002000 /* RX interrupt request */;
//
// struct ath5k_hw_rx_status - Common hardware RX status descriptor
// @rx_status_0: RX status word 0
// @rx_status_1: RX status word 1
//
// 5210, 5211 and 5212 differ only in the fields and flags defined below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_rx_status {
    pub rx_status_0: u32,
    pub rx_status_1: u32,
    pub __aligned(4): } __packed,
// 5210/5211
// RX status word 0 fields/flags
pub const AR5K_5210_RX_DESC_STATUS0_DATA_LEN: c_uint = 0x00000fff /* RX data length */;
pub const AR5K_5210_RX_DESC_STATUS0_MORE: c_uint = 0x00001000 /* more desc for this frame */;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_ANT_5210: c_uint = 0x00004000 /* [5210] receive on ant 1 */;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_RATE: c_uint = 0x00078000 /* reception rate */;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_RATE_S: c_int = 15;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_SIGNAL: c_uint = 0x07f80000 /* rssi */;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_SIGNAL_S: c_int = 19;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_ANT_5211: c_uint = 0x38000000 /* [5211] receive antenna */;
pub const AR5K_5210_RX_DESC_STATUS0_RECEIVE_ANT_5211_S: c_int = 27;
// RX status word 1 fields/flags
pub const AR5K_5210_RX_DESC_STATUS1_DONE: c_uint = 0x00000001 /* descriptor complete */;
pub const AR5K_5210_RX_DESC_STATUS1_FRAME_RECEIVE_OK: c_uint = 0x00000002 /* reception success */;
pub const AR5K_5210_RX_DESC_STATUS1_CRC_ERROR: c_uint = 0x00000004 /* CRC error */;
pub const AR5K_5210_RX_DESC_STATUS1_FIFO_OVERRUN_5210: c_uint = 0x00000008 /* [5210] FIFO overrun */;
pub const AR5K_5210_RX_DESC_STATUS1_DECRYPT_CRC_ERROR: c_uint = 0x00000010 /* decryption CRC failure */;
pub const AR5K_5210_RX_DESC_STATUS1_PHY_ERROR: c_uint = 0x000000e0 /* PHY error */;
pub const AR5K_5210_RX_DESC_STATUS1_PHY_ERROR_S: c_int = 5;
pub const AR5K_5210_RX_DESC_STATUS1_KEY_INDEX_VALID: c_uint = 0x00000100 /* key index valid */;
pub const AR5K_5210_RX_DESC_STATUS1_KEY_INDEX: c_uint = 0x00007e00 /* decryption key index */;
pub const AR5K_5210_RX_DESC_STATUS1_KEY_INDEX_S: c_int = 9;
pub const AR5K_5210_RX_DESC_STATUS1_RECEIVE_TIMESTAMP: c_uint = 0x0fff8000 /* 13 bit of TSF */;
pub const AR5K_5210_RX_DESC_STATUS1_RECEIVE_TIMESTAMP_S: c_int = 15;
pub const AR5K_5210_RX_DESC_STATUS1_KEY_CACHE_MISS: c_uint = 0x10000000 /* key cache miss */;
// 5212
// RX status word 0 fields/flags
pub const AR5K_5212_RX_DESC_STATUS0_DATA_LEN: c_uint = 0x00000fff /* RX data length */;
pub const AR5K_5212_RX_DESC_STATUS0_MORE: c_uint = 0x00001000 /* more desc for this frame */;
pub const AR5K_5212_RX_DESC_STATUS0_DECOMP_CRC_ERROR: c_uint = 0x00002000 /* decompression CRC error */;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_RATE: c_uint = 0x000f8000 /* reception rate */;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_RATE_S: c_int = 15;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_SIGNAL: c_uint = 0x0ff00000 /* rssi */;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_SIGNAL_S: c_int = 20;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_ANTENNA: c_uint = 0xf0000000 /* receive antenna */;
pub const AR5K_5212_RX_DESC_STATUS0_RECEIVE_ANTENNA_S: c_int = 28;
// RX status word 1 fields/flags
pub const AR5K_5212_RX_DESC_STATUS1_DONE: c_uint = 0x00000001 /* descriptor complete */;
pub const AR5K_5212_RX_DESC_STATUS1_FRAME_RECEIVE_OK: c_uint = 0x00000002 /* frame reception success */;
pub const AR5K_5212_RX_DESC_STATUS1_CRC_ERROR: c_uint = 0x00000004 /* CRC error */;
pub const AR5K_5212_RX_DESC_STATUS1_DECRYPT_CRC_ERROR: c_uint = 0x00000008 /* decryption CRC failure */;
pub const AR5K_5212_RX_DESC_STATUS1_PHY_ERROR: c_uint = 0x00000010 /* PHY error */;
pub const AR5K_5212_RX_DESC_STATUS1_MIC_ERROR: c_uint = 0x00000020 /* MIC decrypt error */;
pub const AR5K_5212_RX_DESC_STATUS1_KEY_INDEX_VALID: c_uint = 0x00000100 /* key index valid */;
pub const AR5K_5212_RX_DESC_STATUS1_KEY_INDEX: c_uint = 0x0000fe00 /* decryption key index */;
pub const AR5K_5212_RX_DESC_STATUS1_KEY_INDEX_S: c_int = 9;
pub const AR5K_5212_RX_DESC_STATUS1_RECEIVE_TIMESTAMP: c_uint = 0x7fff0000 /* first 15bit of the TSF */;
pub const AR5K_5212_RX_DESC_STATUS1_RECEIVE_TIMESTAMP_S: c_int = 16;
pub const AR5K_5212_RX_DESC_STATUS1_KEY_CACHE_MISS: c_uint = 0x80000000 /* key cache miss */;
pub const AR5K_5212_RX_DESC_STATUS1_PHY_ERROR_CODE: c_uint = 0x0000ff00 /* phy error code overlays key index and valid fields */;
pub const AR5K_5212_RX_DESC_STATUS1_PHY_ERROR_CODE_S: c_int = 8;
//
// enum ath5k_phy_error_code - PHY Error codes
// @AR5K_RX_PHY_ERROR_UNDERRUN: Transmit underrun, [5210] No error
// @AR5K_RX_PHY_ERROR_TIMING: Timing error
// @AR5K_RX_PHY_ERROR_PARITY: Illegal parity
// @AR5K_RX_PHY_ERROR_RATE: Illegal rate
// @AR5K_RX_PHY_ERROR_LENGTH: Illegal length
// @AR5K_RX_PHY_ERROR_RADAR: Radar detect, [5210] 64 QAM rate
// @AR5K_RX_PHY_ERROR_SERVICE: Illegal service
// @AR5K_RX_PHY_ERROR_TOR: Transmit override receive
// @AR5K_RX_PHY_ERROR_OFDM_TIMING: OFDM Timing error [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_SIGNAL_PARITY: OFDM Signal parity error [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_RATE_ILLEGAL: OFDM Illegal rate [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_LENGTH_ILLEGAL: OFDM Illegal length [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_POWER_DROP: OFDM Power drop [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_SERVICE: OFDM Service (?) [5212+]
// @AR5K_RX_PHY_ERROR_OFDM_RESTART: OFDM Restart (?) [5212+]
// @AR5K_RX_PHY_ERROR_CCK_TIMING: CCK Timing error [5212+]
// @AR5K_RX_PHY_ERROR_CCK_HEADER_CRC: Header CRC error [5212+]
// @AR5K_RX_PHY_ERROR_CCK_RATE_ILLEGAL: Illegal rate [5212+]
// @AR5K_RX_PHY_ERROR_CCK_SERVICE: CCK Service (?) [5212+]
// @AR5K_RX_PHY_ERROR_CCK_RESTART: CCK Restart (?) [5212+]
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_phy_error_code {
    AR5K_RX_PHY_ERROR_UNDERRUN		= 0,
    AR5K_RX_PHY_ERROR_TIMING		= 1,
    AR5K_RX_PHY_ERROR_PARITY		= 2,
    AR5K_RX_PHY_ERROR_RATE			= 3,
    AR5K_RX_PHY_ERROR_LENGTH		= 4,
    AR5K_RX_PHY_ERROR_RADAR			= 5,
    AR5K_RX_PHY_ERROR_SERVICE		= 6,
    AR5K_RX_PHY_ERROR_TOR			= 7,
    AR5K_RX_PHY_ERROR_OFDM_TIMING		= 17,
    AR5K_RX_PHY_ERROR_OFDM_SIGNAL_PARITY	= 18,
    AR5K_RX_PHY_ERROR_OFDM_RATE_ILLEGAL	= 19,
    AR5K_RX_PHY_ERROR_OFDM_LENGTH_ILLEGAL	= 20,
    AR5K_RX_PHY_ERROR_OFDM_POWER_DROP	= 21,
    AR5K_RX_PHY_ERROR_OFDM_SERVICE		= 22,
    AR5K_RX_PHY_ERROR_OFDM_RESTART		= 23,
    AR5K_RX_PHY_ERROR_CCK_TIMING		= 25,
    AR5K_RX_PHY_ERROR_CCK_HEADER_CRC	= 26,
    AR5K_RX_PHY_ERROR_CCK_RATE_ILLEGAL	= 27,
    AR5K_RX_PHY_ERROR_CCK_SERVICE		= 30,
    AR5K_RX_PHY_ERROR_CCK_RESTART		= 31,
}

//
// struct ath5k_hw_2w_tx_ctl  - 5210/5211 hardware 2-word TX control descriptor
// @tx_control_0: TX control word 0
// @tx_control_1: TX control word 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_2w_tx_ctl {
    pub tx_control_0: u32,
    pub tx_control_1: u32,
    pub __aligned(4): } __packed,
// TX control word 0 fields/flags
pub const AR5K_2W_TX_DESC_CTL0_FRAME_LEN: c_uint = 0x00000fff /* frame length */;
pub const AR5K_2W_TX_DESC_CTL0_HEADER_LEN_5210: c_uint = 0x0003f000 /* [5210] header length */;
pub const AR5K_2W_TX_DESC_CTL0_HEADER_LEN_5210_S: c_int = 12;
pub const AR5K_2W_TX_DESC_CTL0_XMIT_RATE: c_uint = 0x003c0000 /* tx rate */;
pub const AR5K_2W_TX_DESC_CTL0_XMIT_RATE_S: c_int = 18;
pub const AR5K_2W_TX_DESC_CTL0_RTSENA: c_uint = 0x00400000 /* RTS/CTS enable */;
pub const AR5K_2W_TX_DESC_CTL0_LONG_PACKET_5210: c_uint = 0x00800000 /* [5210] long packet */;
pub const AR5K_2W_TX_DESC_CTL0_VEOL_5211: c_uint = 0x00800000 /* [5211] virtual end-of-list */;
pub const AR5K_2W_TX_DESC_CTL0_CLRDMASK: c_uint = 0x01000000 /* clear destination mask */;
pub const AR5K_2W_TX_DESC_CTL0_ANT_MODE_XMIT_5210: c_uint = 0x02000000 /* [5210] antenna selection */;
pub const AR5K_2W_TX_DESC_CTL0_ANT_MODE_XMIT_5211: c_uint = 0x1e000000 /* [5211] antenna selection */;

pub const AR5K_2W_TX_DESC_CTL0_ANT_MODE_XMIT_S: c_int = 25;
pub const AR5K_2W_TX_DESC_CTL0_FRAME_TYPE_5210: c_uint = 0x1c000000 /* [5210] frame type */;
pub const AR5K_2W_TX_DESC_CTL0_FRAME_TYPE_5210_S: c_int = 26;
pub const AR5K_2W_TX_DESC_CTL0_INTREQ: c_uint = 0x20000000 /* TX interrupt request */;
pub const AR5K_2W_TX_DESC_CTL0_ENCRYPT_KEY_VALID: c_uint = 0x40000000 /* key is valid */;
// TX control word 1 fields/flags
pub const AR5K_2W_TX_DESC_CTL1_BUF_LEN: c_uint = 0x00000fff /* data buffer length */;
pub const AR5K_2W_TX_DESC_CTL1_MORE: c_uint = 0x00001000 /* more desc for this frame */;
pub const AR5K_2W_TX_DESC_CTL1_ENC_KEY_IDX_5210: c_uint = 0x0007e000 /* [5210] key table index */;
pub const AR5K_2W_TX_DESC_CTL1_ENC_KEY_IDX_5211: c_uint = 0x000fe000 /* [5211] key table index */;

pub const AR5K_2W_TX_DESC_CTL1_ENC_KEY_IDX_S: c_int = 13;
pub const AR5K_2W_TX_DESC_CTL1_FRAME_TYPE_5211: c_uint = 0x00700000 /* [5211] frame type */;
pub const AR5K_2W_TX_DESC_CTL1_FRAME_TYPE_5211_S: c_int = 20;
pub const AR5K_2W_TX_DESC_CTL1_NOACK_5211: c_uint = 0x00800000 /* [5211] no ACK */;
pub const AR5K_2W_TX_DESC_CTL1_RTS_DURATION_5210: c_uint = 0xfff80000 /* [5210] lower 13 bit of duration */;
// Frame types
pub const AR5K_AR5210_TX_DESC_FRAME_TYPE_NORMAL: c_int = 0;
pub const AR5K_AR5210_TX_DESC_FRAME_TYPE_ATIM: c_int = 1;
pub const AR5K_AR5210_TX_DESC_FRAME_TYPE_PSPOLL: c_int = 2;
pub const AR5K_AR5210_TX_DESC_FRAME_TYPE_NO_DELAY: c_int = 3;
pub const AR5K_AR5211_TX_DESC_FRAME_TYPE_BEACON: c_int = 3;
pub const AR5K_AR5210_TX_DESC_FRAME_TYPE_PIFS: c_int = 4;
pub const AR5K_AR5211_TX_DESC_FRAME_TYPE_PRESP: c_int = 4;
//
// struct ath5k_hw_4w_tx_ctl - 5212 hardware 4-word TX control descriptor
// @tx_control_0: TX control word 0
// @tx_control_1: TX control word 1
// @tx_control_2: TX control word 2
// @tx_control_3: TX control word 3
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_4w_tx_ctl {
    pub tx_control_0: u32,
    pub tx_control_1: u32,
    pub tx_control_2: u32,
    pub tx_control_3: u32,
    pub __aligned(4): } __packed,
// TX control word 0 fields/flags
pub const AR5K_4W_TX_DESC_CTL0_FRAME_LEN: c_uint = 0x00000fff /* frame length */;
pub const AR5K_4W_TX_DESC_CTL0_XMIT_POWER: c_uint = 0x003f0000 /* transmit power */;
pub const AR5K_4W_TX_DESC_CTL0_XMIT_POWER_S: c_int = 16;
pub const AR5K_4W_TX_DESC_CTL0_RTSENA: c_uint = 0x00400000 /* RTS/CTS enable */;
pub const AR5K_4W_TX_DESC_CTL0_VEOL: c_uint = 0x00800000 /* virtual end-of-list */;
pub const AR5K_4W_TX_DESC_CTL0_CLRDMASK: c_uint = 0x01000000 /* clear destination mask */;
pub const AR5K_4W_TX_DESC_CTL0_ANT_MODE_XMIT: c_uint = 0x1e000000 /* TX antenna selection */;
pub const AR5K_4W_TX_DESC_CTL0_ANT_MODE_XMIT_S: c_int = 25;
pub const AR5K_4W_TX_DESC_CTL0_INTREQ: c_uint = 0x20000000 /* TX interrupt request */;
pub const AR5K_4W_TX_DESC_CTL0_ENCRYPT_KEY_VALID: c_uint = 0x40000000 /* destination index valid */;
pub const AR5K_4W_TX_DESC_CTL0_CTSENA: c_uint = 0x80000000 /* precede frame with CTS */;
// TX control word 1 fields/flags
pub const AR5K_4W_TX_DESC_CTL1_BUF_LEN: c_uint = 0x00000fff /* data buffer length */;
pub const AR5K_4W_TX_DESC_CTL1_MORE: c_uint = 0x00001000 /* more desc for this frame */;
pub const AR5K_4W_TX_DESC_CTL1_ENCRYPT_KEY_IDX: c_uint = 0x000fe000 /* destination table index */;
pub const AR5K_4W_TX_DESC_CTL1_ENCRYPT_KEY_IDX_S: c_int = 13;
pub const AR5K_4W_TX_DESC_CTL1_FRAME_TYPE: c_uint = 0x00f00000 /* frame type */;
pub const AR5K_4W_TX_DESC_CTL1_FRAME_TYPE_S: c_int = 20;
pub const AR5K_4W_TX_DESC_CTL1_NOACK: c_uint = 0x01000000 /* no ACK */;
pub const AR5K_4W_TX_DESC_CTL1_COMP_PROC: c_uint = 0x06000000 /* compression processing */;
pub const AR5K_4W_TX_DESC_CTL1_COMP_PROC_S: c_int = 25;
pub const AR5K_4W_TX_DESC_CTL1_COMP_IV_LEN: c_uint = 0x18000000 /* length of frame IV */;
pub const AR5K_4W_TX_DESC_CTL1_COMP_IV_LEN_S: c_int = 27;
pub const AR5K_4W_TX_DESC_CTL1_COMP_ICV_LEN: c_uint = 0x60000000 /* length of frame ICV */;
pub const AR5K_4W_TX_DESC_CTL1_COMP_ICV_LEN_S: c_int = 29;
// TX control word 2 fields/flags
pub const AR5K_4W_TX_DESC_CTL2_RTS_DURATION: c_uint = 0x00007fff /* RTS/CTS duration */;
pub const AR5K_4W_TX_DESC_CTL2_DURATION_UPD_EN: c_uint = 0x00008000 /* frame duration update */;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES0: c_uint = 0x000f0000 /* series 0 max attempts */;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES0_S: c_int = 16;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES1: c_uint = 0x00f00000 /* series 1 max attempts */;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES1_S: c_int = 20;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES2: c_uint = 0x0f000000 /* series 2 max attempts */;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES2_S: c_int = 24;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES3: c_uint = 0xf0000000 /* series 3 max attempts */;
pub const AR5K_4W_TX_DESC_CTL2_XMIT_TRIES3_S: c_int = 28;
// TX control word 3 fields/flags
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE0: c_uint = 0x0000001f /* series 0 tx rate */;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE1: c_uint = 0x000003e0 /* series 1 tx rate */;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE1_S: c_int = 5;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE2: c_uint = 0x00007c00 /* series 2 tx rate */;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE2_S: c_int = 10;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE3: c_uint = 0x000f8000 /* series 3 tx rate */;
pub const AR5K_4W_TX_DESC_CTL3_XMIT_RATE3_S: c_int = 15;
pub const AR5K_4W_TX_DESC_CTL3_RTS_CTS_RATE: c_uint = 0x01f00000 /* RTS or CTS rate */;
pub const AR5K_4W_TX_DESC_CTL3_RTS_CTS_RATE_S: c_int = 20;
//
// struct ath5k_hw_tx_status - Common TX status descriptor
// @tx_status_0: TX status word 0
// @tx_status_1: TX status word 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_tx_status {
    pub tx_status_0: u32,
    pub tx_status_1: u32,
    pub __aligned(4): } __packed,
// TX status word 0 fields/flags
pub const AR5K_DESC_TX_STATUS0_FRAME_XMIT_OK: c_uint = 0x00000001 /* TX success */;
pub const AR5K_DESC_TX_STATUS0_EXCESSIVE_RETRIES: c_uint = 0x00000002 /* excessive retries */;
pub const AR5K_DESC_TX_STATUS0_FIFO_UNDERRUN: c_uint = 0x00000004 /* FIFO underrun */;
pub const AR5K_DESC_TX_STATUS0_FILTERED: c_uint = 0x00000008 /* TX filter indication */;
// according to the HAL sources the spec has short/long retry counts reversed.
// we have it reversed to the HAL sources as well, for 5210 and 5211.
// For 5212 these fields are defined as RTS_FAIL_COUNT and DATA_FAIL_COUNT,
// but used respectively as SHORT and LONG retry count in the code later. This
// is consistent with the definitions here... TODO: check
pub const AR5K_DESC_TX_STATUS0_SHORT_RETRY_COUNT: c_uint = 0x000000f0 /* short retry count */;
pub const AR5K_DESC_TX_STATUS0_SHORT_RETRY_COUNT_S: c_int = 4;
pub const AR5K_DESC_TX_STATUS0_LONG_RETRY_COUNT: c_uint = 0x00000f00 /* long retry count */;
pub const AR5K_DESC_TX_STATUS0_LONG_RETRY_COUNT_S: c_int = 8;
pub const AR5K_DESC_TX_STATUS0_VIRTCOLL_CT_5211: c_uint = 0x0000f000 /* [5211+] virtual collision count */;
pub const AR5K_DESC_TX_STATUS0_VIRTCOLL_CT_5212_S: c_int = 12;
pub const AR5K_DESC_TX_STATUS0_SEND_TIMESTAMP: c_uint = 0xffff0000 /* TX timestamp */;
pub const AR5K_DESC_TX_STATUS0_SEND_TIMESTAMP_S: c_int = 16;
// TX status word 1 fields/flags
pub const AR5K_DESC_TX_STATUS1_DONE: c_uint = 0x00000001 /* descriptor complete */;
pub const AR5K_DESC_TX_STATUS1_SEQ_NUM: c_uint = 0x00001ffe /* TX sequence number */;
pub const AR5K_DESC_TX_STATUS1_SEQ_NUM_S: c_int = 1;
pub const AR5K_DESC_TX_STATUS1_ACK_SIG_STRENGTH: c_uint = 0x001fe000 /* signal strength of ACK */;
pub const AR5K_DESC_TX_STATUS1_ACK_SIG_STRENGTH_S: c_int = 13;
pub const AR5K_DESC_TX_STATUS1_FINAL_TS_IX_5212: c_uint = 0x00600000 /* [5212] final TX attempt series ix */;
pub const AR5K_DESC_TX_STATUS1_FINAL_TS_IX_5212_S: c_int = 21;
pub const AR5K_DESC_TX_STATUS1_COMP_SUCCESS_5212: c_uint = 0x00800000 /* [5212] compression status */;
pub const AR5K_DESC_TX_STATUS1_XMIT_ANTENNA_5212: c_uint = 0x01000000 /* [5212] transmit antenna */;
//
// struct ath5k_hw_5210_tx_desc - 5210/5211 hardware TX descriptor
// @tx_ctl: The &struct ath5k_hw_2w_tx_ctl
// @tx_stat: The &struct ath5k_hw_tx_status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_5210_tx_desc {
    pub tx_ctl: ath5k_hw_2w_tx_ctl,
    pub tx_stat: ath5k_hw_tx_status,
    pub __aligned(4): } __packed,
//
// struct ath5k_hw_5212_tx_desc - 5212 hardware TX descriptor
// @tx_ctl: The &struct ath5k_hw_4w_tx_ctl
// @tx_stat: The &struct ath5k_hw_tx_status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_5212_tx_desc {
    pub tx_ctl: ath5k_hw_4w_tx_ctl,
    pub tx_stat: ath5k_hw_tx_status,
    pub __aligned(4): } __packed,
//
// struct ath5k_hw_all_rx_desc - Common hardware RX descriptor
// @rx_ctl: The &struct ath5k_hw_rx_ctl
// @rx_stat: The &struct ath5k_hw_rx_status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw_all_rx_desc {
    pub rx_ctl: ath5k_hw_rx_ctl,
    pub rx_stat: ath5k_hw_rx_status,
    pub __aligned(4): } __packed,
//
// struct ath5k_desc - Atheros hardware DMA descriptor
// @ds_link: Physical address of the next descriptor
// @ds_data: Physical address of data buffer (skb)
// @ud: Union containing hw_5xxx_tx_desc structs and hw_all_rx_desc
//
// This is read and written to by the hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_desc {
    pub ds_link: u32,
    pub ds_data: u32,
    pub ds_tx5210: ath5k_hw_5210_tx_desc,
    pub ds_tx5212: ath5k_hw_5212_tx_desc,
    pub ds_rx: ath5k_hw_all_rx_desc,
    pub ud: },
    pub __aligned(4): } __packed,
pub const AR5K_RXDESC_INTREQ: c_uint = 0x0020;
pub const AR5K_TXDESC_CLRDMASK: c_uint = 0x0001;
pub const AR5K_TXDESC_NOACK: c_uint = 0x0002	/*[5211+]*/;
pub const AR5K_TXDESC_RTSENA: c_uint = 0x0004;
pub const AR5K_TXDESC_CTSENA: c_uint = 0x0008;
pub const AR5K_TXDESC_INTREQ: c_uint = 0x0010;
pub const AR5K_TXDESC_VEOL: c_uint = 0x0020	/*[5211+]*/;
