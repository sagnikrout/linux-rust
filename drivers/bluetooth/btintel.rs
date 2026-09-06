//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/bluetooth/btintel.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Bluetooth support for Intel devices
//
// Copyright (C) 2015  Intel Corporation
//
// List of tlv type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_tlv {
    pub type: u8,
    pub len: u8,
    pub val: [u8; ],
    pub __packed: },
pub const BTINTEL_HCI_OP_RESET: c_uint = 0xfc01;
pub const BTINTEL_HCI_OP_DEBUG: c_uint = 0xfcd9;
pub const BTINTEL_CNVI_BLAZARI: c_uint = 0x900	/* BlazarI - Lunar Lake */;
pub const BTINTEL_CNVI_BLAZARIW: c_uint = 0x901	/* BlazarIW - Wildcat Lake */;
pub const BTINTEL_CNVI_GAP: c_uint = 0x910	/* Gale Peak2 - Meteor Lake */;
pub const BTINTEL_CNVI_BLAZARU: c_uint = 0x930	/* BlazarU - Meteor Lake */;
pub const BTINTEL_CNVI_SCP: c_uint = 0xA00	/* Scorpius Peak - Panther Lake */;
pub const BTINTEL_CNVI_SCP2: c_uint = 0xA10	/* Scorpius Peak2 - Nova Lake */;
pub const BTINTEL_CNVI_SCP2F: c_uint = 0xA20	/* Scorpius Peak2F - Nova Lake */;
// CNVR
pub const BTINTEL_CNVR_FMP2: c_uint = 0x910;
pub const BTINTEL_CNVR_WHP2: c_uint = 0xA10	/* Whale Peak2 - Panther Lake */;
pub const BTINTEL_IMG_BOOTLOADER: c_uint = 0x01	/* Bootloader image */;
pub const BTINTEL_IMG_IML: c_uint = 0x02	/* Intermediate image */;
pub const BTINTEL_IMG_OP: c_uint = 0x03	/* Operational image */;
pub const BTINTEL_FWID_MAXLEN: c_int = 64;
// CNVi Hardware variant
pub const BTINTEL_HWID_GAP: c_uint = 0x1c	/* Gale Peak2 - Meteor Lake */;
pub const BTINTEL_HWID_BZRI: c_uint = 0x1e	/* BlazarI - Lunar Lake */;
pub const BTINTEL_HWID_BZRU: c_uint = 0x1d	/* BlazarU - Meteor Lake */;
pub const BTINTEL_HWID_SCP: c_uint = 0x1f	/* Scorpius Peak - Panther Lake */;
pub const BTINTEL_HWID_SCP2: c_uint = 0x20	/* Scorpius Peak2 - Nova Lake */;
pub const BTINTEL_HWID_SCP2F: c_uint = 0x21	/* Scorpius Peak2-F - Nova Lake */;
pub const BTINTEL_HWID_BZRIW: c_uint = 0x22	/* BlazarIW - Wildcat Lake */;
    pub btintel_guid_dsm: extern guid_t,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_version_tlv {
    pub cnvi_top: u32,
    pub cnvr_top: u32,
    pub cnvi_bt: u32,
    pub cnvr_bt: u32,
    pub dev_rev_id: u16,
    pub img_type: u8,
    pub timestamp: u16,
    pub build_type: u8,
    pub build_num: u32,
    pub secure_boot: u8,
    pub otp_lock: u8,
    pub api_lock: u8,
    pub debug_lock: u8,
    pub min_fw_build_nn: u8,
    pub min_fw_build_cw: u8,
    pub min_fw_build_yy: u8,
    pub limited_cce: u8,
    pub sbe_type: u8,
    pub git_sha1: u32,
    pub fw_id: [u8; BTINTEL_FWID_MAXLEN],
    pub otp_bd_addr: bdaddr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_version {
    pub status: u8,
    pub hw_platform: u8,
    pub hw_variant: u8,
    pub hw_revision: u8,
    pub fw_variant: u8,
    pub fw_revision: u8,
    pub fw_build_num: u8,
    pub fw_build_ww: u8,
    pub fw_build_yy: u8,
    pub fw_patch_num: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_boot_params {
    pub status: __u8,
    pub otp_format: __u8,
    pub otp_content: __u8,
    pub otp_patch: __u8,
    pub dev_revid: __le16,
    pub secure_boot: __u8,
    pub key_from_hdr: __u8,
    pub key_type: __u8,
    pub otp_lock: __u8,
    pub api_lock: __u8,
    pub debug_lock: __u8,
    pub otp_bdaddr: bdaddr_t,
    pub min_fw_build_nn: __u8,
    pub min_fw_build_cw: __u8,
    pub min_fw_build_yy: __u8,
    pub limited_cce: __u8,
    pub unlocked_state: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_bootup {
    pub zero: __u8,
    pub num_cmds: __u8,
    pub source: __u8,
    pub reset_type: __u8,
    pub reset_reason: __u8,
    pub ddc_status: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_secure_send_result {
    pub result: __u8,
    pub opcode: __le16,
    pub status: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_reset {
    pub reset_type: __u8,
    pub patch_enable: __u8,
    pub ddc_reload: __u8,
    pub boot_option: __u8,
    pub boot_param: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_debug_features {
    pub page1: [__u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_offload_use_cases {
    pub status: __u8,
    pub preset: [__u8; 8],
    pub __packed: },
pub const INTEL_OP_PPAG_CMD: c_uint = 0xFE0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_ppag_enable_cmd {
    pub ppag_enable_flags: __le32,
    pub __packed: },
pub const INTEL_TLV_TYPE_ID: c_uint = 0x01;
pub const INTEL_TLV_SYSTEM_EXCEPTION: c_uint = 0x00;
pub const INTEL_TLV_FATAL_EXCEPTION: c_uint = 0x01;
pub const INTEL_TLV_DEBUG_EXCEPTION: c_uint = 0x02;
pub const INTEL_TLV_TEST_EXCEPTION: c_uint = 0xDE;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_cp_ddc_write {
    pub len: u8,
    pub id: __le16,
    pub data: [u8; ],
    pub __packed: },
// Bluetooth SAR feature (BRDS), Revision 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_sar_inc_pwr {
    pub revision: u8,
    pub /: *mut *mut u32 bt_sar_bios; / Mode of SAR control to be used, 1:enabled in bios,
    pub /: *mut *mut u32 inc_power_mode; / Increased power mode,
    pub /: *mut *mut u8 sar_2400_chain_a; / Sar power restriction LB,
    pub br: u8,
    pub edr2: u8,
    pub edr3: u8,
    pub le: u8,
    pub le_2mhz: u8,
    pub le_lr: u8,
}

// Bluetooth SAR feature (BRDS), Revision 2 - per-chain sub-band power limits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_sar_band_limits {
    pub subband_2g4: u8,
    pub subband_5g2: u8,
    pub subband_5g8_5g9: u8,
    pub subband_6g1: u8,
    pub subband_6g3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_sar_rev2 {
    pub revision: u8,
    pub /: *mut *mut u32 bt_sar_bios; / 1: BIOS-managed SAR enabled,
    pub /: *mut *mut u32 inc_power_mode; / 0: supported, 1: disabled,
    pub chain_a: btintel_sar_band_limits,
    pub chain_b: btintel_sar_band_limits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btintel_data {
    pub __INTEL_NUM_FLAGS): DECLARE_BITMAP(flags,,
    pub hdev): *mut *mut int (acpi_reset_method)(struct hci_dev,
}

extern "C" {
    pub fn btintel_check_bdaddr(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btintel_enter_mfg(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btintel_exit_mfg(hdev: *mut hci_dev, reset: bool, patched: bool) -> c_int;
}
extern "C" {
    pub fn btintel_set_bdaddr(hdev: *mut hci_dev, bdaddr: *const bdaddr_t) -> c_int;
}
extern "C" {
    pub fn btintel_set_diag(hdev: *mut hci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn btintel_version_info(hdev: *mut hci_dev, ver: *mut intel_version) -> c_int;
}
extern "C" {
    pub fn btintel_load_ddc_config(hdev: *mut hci_dev, ddc_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn btintel_set_event_mask_mfg(hdev: *mut hci_dev, debug: bool) -> c_int;
}
extern "C" {
    pub fn btintel_read_version(hdev: *mut hci_dev, ver: *mut intel_version) -> c_int;
}
extern "C" {
    pub fn btintel_send_intel_reset(hdev: *mut hci_dev, boot_param: u32) -> c_int;
}
extern "C" {
    pub fn btintel_configure_setup(hdev: *mut hci_dev, driver_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn btintel_recv_event(hdev: *mut hci_dev, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn btintel_bootup(hdev: *mut hci_dev, ptr: *const c_void, len: c_uint);
}
extern "C" {
    pub fn btintel_set_quality_report(hdev: *mut hci_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn btintel_set_msft_opcode(hdev: *mut hci_dev, hw_variant: u8);
}
extern "C" {
    pub fn btintel_shutdown_combined(hdev: *mut hci_dev) -> c_int;
}
extern "C" {
    pub fn btintel_hw_error(hdev: *mut hci_dev, code: u8);
}
extern "C" {
    pub fn btintel_print_fseq_info(hdev: *mut hci_dev);
}
extern "C" {
    pub fn btintel_acpi_reset_method(hdev: *mut hci_dev) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
