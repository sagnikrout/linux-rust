//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_hal.h
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
// Copyright (c) 2017 Redpine Signals Inc.
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
// Device Operating modes
pub const DEV_OPMODE_WIFI_ALONE: c_int = 1;
pub const DEV_OPMODE_BT_ALONE: c_int = 4;
pub const DEV_OPMODE_BT_LE_ALONE: c_int = 8;
pub const DEV_OPMODE_BT_DUAL: c_int = 12;
pub const DEV_OPMODE_STA_BT: c_int = 5;
pub const DEV_OPMODE_STA_BT_LE: c_int = 9;
pub const DEV_OPMODE_STA_BT_DUAL: c_int = 13;
pub const DEV_OPMODE_AP_BT: c_int = 6;
pub const DEV_OPMODE_AP_BT_DUAL: c_int = 14;

pub const FLASH_SIZE_ADDR: c_uint = 0x04000016;
pub const PING_BUFFER_ADDRESS: c_uint = 0x19000;
pub const PONG_BUFFER_ADDRESS: c_uint = 0x1a000;
pub const SWBL_REGIN: c_uint = 0x41050034;
pub const SWBL_REGOUT: c_uint = 0x4105003c;
pub const PING_WRITE: c_uint = 0x1;
pub const PONG_WRITE: c_uint = 0x2;
pub const BL_CMD_TIMEOUT: c_int = 2000;

pub const REGIN_VALID: c_uint = 0xA;
pub const REGIN_INPUT: c_uint = 0xA0;
pub const REGOUT_VALID: c_uint = 0xAB;

pub const CMD_PASS: c_uint = 0xAA;
pub const CMD_FAIL: c_uint = 0xCC;

pub const RSI_ULP_RESET_REG: c_uint = 0x161;
pub const RSI_WATCH_DOG_TIMER_1: c_uint = 0x16c;
pub const RSI_WATCH_DOG_TIMER_2: c_uint = 0x16d;
pub const RSI_WATCH_DOG_DELAY_TIMER_1: c_uint = 0x16e;
pub const RSI_WATCH_DOG_DELAY_TIMER_2: c_uint = 0x16f;
pub const RSI_WATCH_DOG_TIMER_ENABLE: c_uint = 0x170;
// Watchdog timer addresses for 9116
pub const NWP_AHB_BASE_ADDR: c_uint = 0x41300000;

// Watchdog timer values
pub const NWP_WWD_INT_TIMER_CLKS: c_int = 5;
pub const NWP_WWD_SYS_RESET_TIMER_CLKS: c_int = 4;
pub const NWP_WWD_TIMER_DISABLE: c_uint = 0xAA0001;
pub const RSI_ULP_WRITE_0: c_int = 00;
pub const RSI_ULP_WRITE_2: c_int = 02;
pub const RSI_ULP_WRITE_50: c_int = 50;

pub const RSI_RF_SPI_PROG_REG_BASE_ADDR: c_uint = 0x40080000;

pub const RSI_GSPI_CTRL_REG0_VALUE: c_uint = 0x340;

// Boot loader commands

pub const FLASH_START_ADDRESS: c_int = 16;
pub const COMMON_HAL_CARD_READY_IND: c_uint = 0x0;
pub const COMMAN_HAL_WAIT_FOR_CARD_READY: c_int = 1;
pub const RSI_DEV_OPMODE_WIFI_ALONE: c_int = 1;
pub const RSI_DEV_COEX_MODE_WIFI_ALONE: c_int = 1;
pub const BBP_INFO_40MHZ: c_uint = 0x6;
pub const FW_FLASH_OFFSET: c_uint = 0x820;

pub const LMAC_VER_OFFSET_9116: c_uint = 0x22C2;
pub const MAX_DWORD_ALIGN_BYTES: c_int = 64;
pub const RSI_COMMON_REG_SIZE: c_int = 2;
pub const RSI_9116_REG_SIZE: c_int = 4;
pub const FW_ALIGN_SIZE: c_int = 4;
pub const RSI_9116_FW_MAGIC_WORD: c_uint = 0x5aa5;
pub const MEM_ACCESS_CTRL_FROM_HOST: c_uint = 0x41300000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_header {
    pub flags: __le32,
    pub image_no: __le32,
    pub check_sum: __le32,
    pub flash_start_address: __le32,
    pub flash_len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ta_metadata {
    pub name: *mut c_char,
    pub address: c_uint,
}

pub const RSI_BL_CTRL_LEN_MASK: c_uint = 0xFFFFFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootload_entry {
    pub control: __le32,
    pub dst_addr: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bootload_ds {
    pub fixed_pattern: __le16,
    pub offset: __le16,
    pub reserved: __le32,
    pub bl_entry: [bootload_entry; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_mgmt_desc {
    pub len_qno: __le16,
    pub frame_type: u8,
    pub misc_flags: u8,
    pub xtend_desc_size: u8,
    pub header_len: u8,
    pub frame_info: __le16,
    pub rate_info: __le16,
    pub bbp_info: __le16,
    pub seq_ctrl: __le16,
    pub reserved2: u8,
    pub sta_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_data_desc {
    pub len_qno: __le16,
    pub cfm_frame_type: u8,
    pub misc_flags: u8,
    pub xtend_desc_size: u8,
    pub header_len: u8,
    pub frame_info: __le16,
    pub rate_info: __le16,
    pub bbp_info: __le16,
    pub mac_flags: __le16,
    pub qid_tid: u8,
    pub sta_id: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_bt_desc {
    pub len_qno: __le16,
    pub reserved1: __le16,
    pub reserved2: __le32,
    pub reserved3: __le32,
    pub reserved4: __le16,
    pub bt_pkt_type: __le16,
    pub __packed: },
    pub adapter): *mut int rsi_hal_device_init(struct rsi_hw,
    pub skb): *mut *mut int rsi_prepare_mgmt_desc(struct rsi_common common, struct sk_buff,
    pub skb): *mut *mut int rsi_prepare_data_desc(struct rsi_common common, struct sk_buff,
    pub skb): *mut *mut int rsi_prepare_beacon(struct rsi_common common, struct sk_buff,
    pub skb): *mut *mut int rsi_send_pkt_to_bus(struct rsi_common common, struct sk_buff,
    pub skb): *mut *mut int rsi_send_bt_pkt(struct rsi_common common, struct sk_buff,
