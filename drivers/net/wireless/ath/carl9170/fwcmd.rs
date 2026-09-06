//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/fwcmd.h
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
// Shared Atheros AR9170 Header
//
// Firmware command interface definitions
//
// Copyright 2008, Johannes Berg <johannes@sipsolutions.net>
// Copyright 2009-2011 Christian Lamparter <chunkeey@googlemail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; see the file COPYING.  If not, see
// http://www.gnu.org/licenses/.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// Copyright (c) 2007-2008 Atheros Communications, Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
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
pub const CARL9170_MAX_CMD_LEN: c_int = 64;
pub const CARL9170_MAX_CMD_PAYLOAD_LEN: c_int = 60;
pub const CARL9170FW_API_MIN_VER: c_int = 1;
pub const CARL9170FW_API_MAX_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170_cmd_oids {
    CARL9170_CMD_RREG		= 0x00,
    CARL9170_CMD_WREG		= 0x01,
    CARL9170_CMD_ECHO		= 0x02,
    CARL9170_CMD_SWRST		= 0x03,
    CARL9170_CMD_REBOOT		= 0x04,
    CARL9170_CMD_BCN_CTRL		= 0x05,
    CARL9170_CMD_READ_TSF		= 0x06,
    CARL9170_CMD_RX_FILTER		= 0x07,
    CARL9170_CMD_WOL		= 0x08,
    CARL9170_CMD_TALLY		= 0x09,
    CARL9170_CMD_WREGB		= 0x0a,

// CAM
    CARL9170_CMD_EKEY		= 0x10,
    CARL9170_CMD_DKEY		= 0x11,

// RF / PHY
    CARL9170_CMD_FREQUENCY		= 0x20,
    CARL9170_CMD_RF_INIT		= 0x21,
    CARL9170_CMD_SYNTH		= 0x22,
    CARL9170_CMD_FREQ_START		= 0x23,
    CARL9170_CMD_PSM		= 0x24,

// Asychronous command flag
    CARL9170_CMD_ASYNC_FLAG		= 0x40,
    CARL9170_CMD_WREG_ASYNC		= (CARL9170_CMD_WREG |
    CARL9170_CMD_ASYNC_FLAG),
    CARL9170_CMD_REBOOT_ASYNC	= (CARL9170_CMD_REBOOT |
    CARL9170_CMD_ASYNC_FLAG),
    CARL9170_CMD_BCN_CTRL_ASYNC	= (CARL9170_CMD_BCN_CTRL |
    CARL9170_CMD_ASYNC_FLAG),
    CARL9170_CMD_PSM_ASYNC		= (CARL9170_CMD_PSM |
    CARL9170_CMD_ASYNC_FLAG),

// responses and traps
    CARL9170_RSP_FLAG		= 0xc0,
    CARL9170_RSP_PRETBTT		= 0xc0,
    CARL9170_RSP_TXCOMP		= 0xc1,
    CARL9170_RSP_BEACON_CONFIG	= 0xc2,
    CARL9170_RSP_ATIM		= 0xc3,
    CARL9170_RSP_WATCHDOG		= 0xc6,
    CARL9170_RSP_TEXT		= 0xca,
    CARL9170_RSP_HEXDUMP		= 0xcc,
    CARL9170_RSP_RADAR		= 0xcd,
    CARL9170_RSP_GPIO		= 0xce,
    CARL9170_RSP_BOOT		= 0xcf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_set_key_cmd {
    pub user: __le16,
    pub keyId: __le16,
    pub type: __le16,
    pub macAddr: [u8; 6],
    pub key: [u32; 4],
    pub __aligned(4): } __packed,
pub const CARL9170_SET_KEY_CMD_SIZE: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_disable_key_cmd {
    pub user: __le16,
    pub padding: __le16,
    pub __aligned(4): } __packed,
pub const CARL9170_DISABLE_KEY_CMD_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_u32_list {
    pub vals: [u32; 0],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_reg_list {
    pub regs: [__le32; 0],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_write_reg {
    pub addr: __le32,
    pub val: __le32,
    pub regs): } __packed,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_write_reg_byte {
    pub addr: __le32,
    pub count: __le32,
    pub val: [u8; ],
    pub __packed: },
pub const CARL9170FW_PHY_HT_ENABLE: c_uint = 0x4;
pub const CARL9170FW_PHY_HT_DYN2040: c_uint = 0x8;
pub const CARL9170FW_PHY_HT_EXT_CHAN_OFF: c_uint = 0x3;
pub const CARL9170FW_PHY_HT_EXT_CHAN_OFF_S: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_rf_init {
    pub freq: __le32,
    pub ht_settings: u8,
    pub padding2: [u8; 3],
    pub delta_slope_coeff_exp: __le32,
    pub delta_slope_coeff_man: __le32,
    pub delta_slope_coeff_exp_shgi: __le32,
    pub delta_slope_coeff_man_shgi: __le32,
    pub finiteLoopCount: __le32,
    pub __packed: },
pub const CARL9170_RF_INIT_SIZE: c_int = 28;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_rf_init_result {
    pub /: *mut *mut __le32 ret; / AR9170_PHY_REG_AGC_CONTROL,
    pub __packed: },
pub const CARL9170_RF_INIT_RESULT_SIZE: c_int = 4;
pub const CARL9170_PSM_SLEEP: c_uint = 0x1000;
pub const CARL9170_PSM_SOFTWARE: c_int = 0;

pub const CARL9170_PSM_COUNTER: c_uint = 0xfff;
pub const CARL9170_PSM_COUNTER_S: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_psm {
    pub state: __le32,
    pub __packed: },
pub const CARL9170_PSM_SIZE: c_int = 4;
//
// Note: If a bit in rx_filter is set, then it
// means that the particular frames which matches
// the condition are FILTERED/REMOVED/DISCARDED!
// (This is can be a bit confusing, especially
// because someone people think it's the exact
// opposite way, so watch out!)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_rx_filter_cmd {
    pub rx_filter: __le32,
    pub __packed: },
pub const CARL9170_RX_FILTER_CMD_SIZE: c_int = 4;
pub const CARL9170_RX_FILTER_BAD: c_uint = 0x01;
pub const CARL9170_RX_FILTER_OTHER_RA: c_uint = 0x02;
pub const CARL9170_RX_FILTER_DECRY_FAIL: c_uint = 0x04;
pub const CARL9170_RX_FILTER_CTL_OTHER: c_uint = 0x08;
pub const CARL9170_RX_FILTER_CTL_PSPOLL: c_uint = 0x10;
pub const CARL9170_RX_FILTER_CTL_BACKR: c_uint = 0x20;
pub const CARL9170_RX_FILTER_MGMT: c_uint = 0x40;
pub const CARL9170_RX_FILTER_DATA: c_uint = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_bcn_ctrl_cmd {
    pub vif_id: __le32,
    pub mode: __le32,
    pub bcn_addr: __le32,
    pub bcn_len: __le32,
    pub __packed: },
pub const CARL9170_BCN_CTRL_CMD_SIZE: c_int = 16;
pub const CARL9170_BCN_CTRL_DRAIN: c_int = 0;
pub const CARL9170_BCN_CTRL_CAB_TRIGGER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_wol_cmd {
    pub flags: __le32,
    pub mac: [u8; 6],
    pub bssid: [u8; 6],
    pub null_interval: __le32,
    pub free_for_use2: __le32,
    pub mask: __le32,
    pub pattern: [u8; 32],
    pub __packed: },
pub const CARL9170_WOL_CMD_SIZE: c_int = 60;
pub const CARL9170_WOL_DISCONNECT: c_int = 1;
pub const CARL9170_WOL_MAGIC_PKT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_cmd_head {
    pub len: u8,
    pub cmd: u8,
    pub seq: u8,
    pub ext: u8,
    pub __packed: },
    pub hdr_data: u32,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_cmd {
    pub hdr: carl9170_cmd_head,
    pub setkey: carl9170_set_key_cmd,
    pub disablekey: carl9170_disable_key_cmd,
    pub echo: carl9170_u32_list,
    pub rreg: carl9170_reg_list,
    pub wreg: carl9170_write_reg,
    pub wregb: carl9170_write_reg_byte,
    pub rf_init: carl9170_rf_init,
    pub psm: carl9170_psm,
    pub wol: carl9170_wol_cmd,
    pub bcn_ctrl: carl9170_bcn_ctrl_cmd,
    pub rx_filter: carl9170_rx_filter_cmd,
    pub data: [u8; CARL9170_MAX_CMD_PAYLOAD_LEN],
    pub __aligned(4): } __packed,
    pub __aligned(4): } __packed,
pub const CARL9170_TX_STATUS_QUEUE: c_int = 3;
pub const CARL9170_TX_STATUS_QUEUE_S: c_int = 0;
pub const CARL9170_TX_STATUS_RIX_S: c_int = 2;

pub const CARL9170_TX_STATUS_TRIES_S: c_int = 4;

pub const CARL9170_TX_STATUS_SUCCESS: c_uint = 0x80;

//
// NOTE:
// Both structs [carl9170_tx_status and _carl9170_tx_status]
// need to be "bit for bit" in sync.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tx_status {
//
// Beware of compiler bugs in all gcc pre 4.4!
//
    pub cookie: u8,
    pub queue:2: u8,
    pub rix:2: u8,
    pub tries:3: u8,
    pub success:1: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _carl9170_tx_status {
//
// This version should be immune to all alignment bugs.
//
    pub cookie: u8,
    pub info: u8,
    pub __packed: },
pub const CARL9170_TX_STATUS_SIZE: c_int = 2;

pub const CARL9170_TX_MAX_RATE_TRIES: c_int = 7;
pub const CARL9170_TX_MAX_RATES: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_gpio {
    pub gpio: __le32,
    pub __packed: },
pub const CARL9170_GPIO_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tsf_rsp {
    pub tsf: [__le32; 2],
    pub tsf_64: __le64,
    pub __packed: },
    pub __packed: },
pub const CARL9170_TSF_RSP_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_tally_rsp {
    pub active: __le32,
    pub cca: __le32,
    pub tx_time: __le32,
    pub rx_total: __le32,
    pub rx_overrun: __le32,
    pub tick: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170_rsp {
    pub hdr: carl9170_cmd_head,
    pub rf_init_res: carl9170_rf_init_result,
    pub rreg_res: carl9170_u32_list,
    pub echo: carl9170_u32_list,

    pub tx_status): DECLARE_FLEX_ARRAY(struct carl9170_tx_status,,

    pub _tx_status): DECLARE_FLEX_ARRAY(struct _carl9170_tx_status,,
    pub gpio: carl9170_gpio,
    pub tsf: carl9170_tsf_rsp,
    pub psm: carl9170_psm,
    pub tally: carl9170_tally_rsp,
    pub data: [u8; CARL9170_MAX_CMD_PAYLOAD_LEN],
    pub __packed: },
    pub __aligned(4): } __packed,
