//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/carl9170/fwdesc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Shared CARL9170 Header
//
// Firmware descriptor format
//
// Copyright 2009-2011 Christian Lamparter <chunkeey@googlemail.com>
//
// NOTE: Don't mess with the order of the flags!
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum carl9170fw_feature_list {
// Always set
    CARL9170FW_DUMMY_FEATURE,

//
// Indicates that this image has special boot block which prevents
// legacy drivers to drive the firmware.
//
    CARL9170FW_MINIBOOT,

// usb registers are initialized by the firmware
    CARL9170FW_USB_INIT_FIRMWARE,

// command traps & notifications are send through EP2
    CARL9170FW_USB_RESP_EP2,

// usb download (app -> fw) stream
    CARL9170FW_USB_DOWN_STREAM,

// usb upload (fw -> app) stream
    CARL9170FW_USB_UP_STREAM,

// unusable - reserved to flag non-functional debug firmwares
    CARL9170FW_UNUSABLE,

// AR9170_CMD_RF_INIT, AR9170_CMD_FREQ_START, AR9170_CMD_FREQUENCY
    CARL9170FW_COMMAND_PHY,

// AR9170_CMD_EKEY, AR9170_CMD_DKEY
    CARL9170FW_COMMAND_CAM,

// Firmware has a software Content After Beacon Queueing mechanism
    CARL9170FW_WLANTX_CAB,

// The firmware is capable of responding to incoming BAR frames
    CARL9170FW_HANDLE_BACK_REQ,

// GPIO Interrupt | CARL9170_RSP_GPIO
    CARL9170FW_GPIO_INTERRUPT,

// Firmware PSM support | CARL9170_CMD_PSM
    CARL9170FW_PSM,

// Firmware RX filter | CARL9170_CMD_RX_FILTER
    CARL9170FW_RX_FILTER,

// Wake up on WLAN
    CARL9170FW_WOL,

// Firmware supports PSM in the 5GHZ Band
    CARL9170FW_FIXED_5GHZ_PSM,

// HW (ANI, CCA, MIB) tally counters
    CARL9170FW_HW_COUNTERS,

// Firmware will pass BA when BARs are queued
    CARL9170FW_RX_BA_FILTER,

// Firmware has support to write a byte at a time
    CARL9170FW_HAS_WREGB_CMD,

// Pattern generator
    CARL9170FW_PATTERN_GENERATOR,

// KEEP LAST
    __CARL9170FW_FEATURE_NUM
}

pub const CARL9170FW_MAGIC_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_desc_head {
    pub magic: [u8; CARL9170FW_MAGIC_SIZE],
    pub length: __le16,
    pub min_ver: u8,
    pub cur_ver: u8,
    pub __packed: },

pub const CARL9170FW_OTUS_DESC_MIN_VER: c_int = 6;
pub const CARL9170FW_OTUS_DESC_CUR_VER: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_otus_desc {
    pub head: carl9170fw_desc_head,
    pub feature_set: __le32,
    pub fw_address: __le32,
    pub bcn_addr: __le32,
    pub bcn_len: __le16,
    pub miniboot_size: __le16,
    pub tx_frag_len: __le16,
    pub rx_max_frame_len: __le16,
    pub tx_descs: u8,
    pub cmd_bufs: u8,
    pub api_ver: u8,
    pub vif_num: u8,
    pub __packed: },

pub const CARL9170FW_MOTD_STRING_LEN: c_int = 24;
pub const CARL9170FW_MOTD_RELEASE_LEN: c_int = 20;
pub const CARL9170FW_MOTD_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_MOTD_DESC_CUR_VER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_motd_desc {
    pub head: carl9170fw_desc_head,
    pub fw_year_month_day: __le32,
    pub desc: [c_char; CARL9170FW_MOTD_STRING_LEN],
    pub release: [c_char; CARL9170FW_MOTD_RELEASE_LEN],
    pub __packed: },

pub const CARL9170FW_FIX_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_FIX_DESC_CUR_VER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_fix_entry {
    pub address: __le32,
    pub mask: __le32,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_fix_desc {
    pub head: carl9170fw_desc_head,
    pub data: [carl9170fw_fix_entry; ],
    pub __packed: },

pub const CARL9170FW_DBG_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_DBG_DESC_CUR_VER: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_dbg_desc {
    pub head: carl9170fw_desc_head,
    pub bogoclock_addr: __le32,
    pub counter_addr: __le32,
    pub rx_total_addr: __le32,
    pub rx_overrun_addr: __le32,
    pub rx_filter: __le32,
// Put your debugging definitions here
    pub __packed: },

pub const CARL9170FW_CHK_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_CHK_DESC_CUR_VER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_chk_desc {
    pub head: carl9170fw_desc_head,
    pub fw_crc32: __le32,
    pub hdr_crc32: __le32,
    pub __packed: },

pub const CARL9170FW_TXSQ_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_TXSQ_DESC_CUR_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_txsq_desc {
    pub head: carl9170fw_desc_head,
    pub seq_table_addr: __le32,
    pub __packed: },

pub const CARL9170FW_WOL_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_WOL_DESC_CUR_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_wol_desc {
    pub head: carl9170fw_desc_head,
    pub /: *mut *mut __le32 supported_triggers; / CARL9170_WOL_,
    pub __packed: },

pub const CARL9170FW_LAST_DESC_MIN_VER: c_int = 1;
pub const CARL9170FW_LAST_DESC_CUR_VER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct carl9170fw_last_desc {
    pub head: carl9170fw_desc_head,
    pub __packed: },

pub const CARL9170FW_DESC_MAX_LENGTH: c_int = 8192;
    pub magic: [head->magic[0] =; 0],
    pub magic: [head->magic[1] =; 1],
    pub magic: [head->magic[2] =; 2],
    pub magic: [head->magic[3] =; 3],
    pub length: head->length =,
    pub min_ver: head->min_ver =,
    pub cur_ver: head->cur_ver =,

    pub \: for (desc = fw_desc;,
    pub \: le16_to_cpu(desc->length) < CARL9170FW_DESC_MAX_LENGTH;,

    pub BIT(feature): return le32_to_cpu(list) &,
    pub true: return,
    pub false: return,
pub const CARL9170FW_MIN_SIZE: c_int = 32;
pub const CARL9170FW_MAX_SIZE: c_int = 16384;
    pub CARL9170FW_MIN_SIZE): return (len <= CARL9170FW_MAX_SIZE && len >=,
