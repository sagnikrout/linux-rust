//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btqca.h
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
// Bluetooth supports for Qualcomm Atheros ROME chips
//
// Copyright (c) 2015 The Linux Foundation. All rights reserved.
//
pub const EDL_PATCH_CMD_OPCODE: c_uint = 0xFC00;
pub const EDL_NVM_ACCESS_OPCODE: c_uint = 0xFC0B;
pub const EDL_WRITE_BD_ADDR_OPCODE: c_uint = 0xFC14;
pub const EDL_PATCH_CMD_LEN: c_int = 1;
pub const EDL_PATCH_VER_REQ_CMD: c_uint = 0x19;
pub const EDL_PATCH_TLV_REQ_CMD: c_uint = 0x1E;
pub const EDL_GET_BUILD_INFO_CMD: c_uint = 0x20;
pub const EDL_GET_BID_REQ_CMD: c_uint = 0x23;
pub const EDL_NVM_ACCESS_SET_REQ_CMD: c_uint = 0x01;
pub const EDL_PATCH_CONFIG_CMD: c_uint = 0x28;
pub const MAX_SIZE_PER_TLV_SEGMENT: c_int = 243;
pub const QCA_PRE_SHUTDOWN_CMD: c_uint = 0xFC08;
pub const QCA_DISABLE_LOGGING: c_uint = 0xFC17;
pub const EDL_CMD_REQ_RES_EVT: c_uint = 0x00;
pub const EDL_PATCH_VER_RES_EVT: c_uint = 0x19;
pub const EDL_APP_VER_RES_EVT: c_uint = 0x02;
pub const EDL_TVL_DNLD_RES_EVT: c_uint = 0x04;
pub const EDL_CMD_EXE_STATUS_EVT: c_uint = 0x00;
pub const EDL_SET_BAUDRATE_RSP_EVT: c_uint = 0x92;
pub const EDL_NVM_ACCESS_CODE_EVT: c_uint = 0x0B;
pub const EDL_PATCH_CONFIG_RES_EVT: c_uint = 0x00;
pub const QCA_DISABLE_LOGGING_SUB_OP: c_uint = 0x14;
pub const EDL_TAG_ID_BD_ADDR: c_int = 2;
pub const EDL_TAG_ID_HCI: c_int = 17;
pub const EDL_TAG_ID_DEEP_SLEEP: c_int = 27;
pub const QCA_WCN3990_POWERON_PULSE: c_uint = 0xFC;
pub const QCA_WCN3990_POWEROFF_PULSE: c_uint = 0xC0;
pub const QCA_HCI_CC_OPCODE: c_uint = 0xFC00;
pub const QCA_HCI_CC_SUCCESS: c_uint = 0x00;
pub const QCA_WCN3991_SOC_ID: c_uint = 0x40014320;
pub const QCA_WCN3950_SOC_ID_T: c_uint = 0x40074130;
pub const QCA_WCN3950_SOC_ID_S: c_uint = 0x40075130;
// QCA chipset version can be decided by patch and SoC
// version, combination with upper 2 bytes from SoC
// and lower 2 bytes from patch will be used.
//

pub const QCA_HSP_GF_SOC_ID: c_uint = 0x1200;
pub const QCA_HSP_GF_SOC_MASK: c_uint = 0x0000ff00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca_baudrate {
    QCA_BAUDRATE_115200	= 0,
    QCA_BAUDRATE_57600,
    QCA_BAUDRATE_38400,
    QCA_BAUDRATE_19200,
    QCA_BAUDRATE_9600,
    QCA_BAUDRATE_230400,
    QCA_BAUDRATE_250000,
    QCA_BAUDRATE_460800,
    QCA_BAUDRATE_500000,
    QCA_BAUDRATE_720000,
    QCA_BAUDRATE_921600,
    QCA_BAUDRATE_1000000,
    QCA_BAUDRATE_1250000,
    QCA_BAUDRATE_2000000,
    QCA_BAUDRATE_3000000,
    QCA_BAUDRATE_4000000,
    QCA_BAUDRATE_1600000,
    QCA_BAUDRATE_3200000,
    QCA_BAUDRATE_3500000,
    QCA_BAUDRATE_AUTO	= 0xFE,
    QCA_BAUDRATE_RESERVED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca_tlv_dnld_mode {
    QCA_SKIP_EVT_NONE,
    QCA_SKIP_EVT_VSE,
    QCA_SKIP_EVT_CC,
    QCA_SKIP_EVT_VSE_CC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca_tlv_type {
    TLV_TYPE_PATCH = 1,
    TLV_TYPE_NVM,
    ELF_TYPE_PATCH,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca_fw_config {
    pub type: u8,
    pub fwname: [c_char; 64],
    pub user_baud_rate: u8,
    pub dnld_mode: qca_tlv_dnld_mode,
    pub dnld_type: qca_tlv_dnld_mode,
    pub bdaddr: bdaddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edl_event_hdr {
    pub cresp: __u8,
    pub rtype: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qca_btsoc_version {
    pub product_id: __le32,
    pub patch_ver: __le16,
    pub rom_ver: __le16,
    pub soc_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_seg_resp {
    pub result: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_type_patch {
    pub total_size: __le32,
    pub data_length: __le32,
    pub format_version: __u8,
    pub signature: __u8,
    pub download_mode: __u8,
    pub reserved1: __u8,
    pub product_id: __le16,
    pub rom_build: __le16,
    pub patch_version: __le16,
    pub reserved2: __le16,
    pub entry: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_type_nvm {
    pub tag_id: __le16,
    pub tag_len: __le16,
    pub reserve1: __le32,
    pub reserve2: __le32,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv_type_hdr {
    pub type_len: __le32,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qca_btsoc_type {
    QCA_INVALID = -1,
    QCA_AR3002,
    QCA_ROME,
    QCA_WCN3950,
    QCA_WCN3988,
    QCA_WCN3990,
    QCA_WCN3998,
    QCA_WCN3991,
    QCA_QCA2066,
    QCA_QCA6390,
    QCA_WCN6750,
    QCA_WCN6855,
    QCA_WCN7850,
}

    pub bdaddr): *const *const int qca_set_bdaddr_rome(struct hci_dev hdev, bdaddr_t,
    pub rampatch_name): *const *const char firmware_name, char,
    pub qca_btsoc_type): enum,
    pub bdaddr): *const *const int qca_set_bdaddr(struct hci_dev hdev, bdaddr_t,
    pub hdev): *mut int qca_send_pre_shutdown_cmd(struct hci_dev,

    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
    pub -EOPNOTSUPP: return,
