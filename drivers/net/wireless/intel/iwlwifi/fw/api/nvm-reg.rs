//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/api/nvm-reg.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2012-2014, 2018-2026 Intel Corporation
// Copyright (C) 2013-2015 Intel Mobile Communications GmbH
// Copyright (C) 2016-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_fw_api_nvm_reg_h__
//
// enum iwl_regulatory_and_nvm_subcmd_ids - regulatory/NVM commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_regulatory_and_nvm_subcmd_ids {
//
// @NVM_ACCESS_COMPLETE: &struct iwl_nvm_access_complete_cmd
//
    NVM_ACCESS_COMPLETE = 0x0,

//
// @LARI_CONFIG_CHANGE: &struct iwl_lari_config_change_cmd_v1,
// &struct iwl_lari_config_change_cmd_v6,
// &struct iwl_lari_config_change_cmd_v8,
// &struct iwl_lari_config_change_cmd
//
    LARI_CONFIG_CHANGE = 0x1,

//
// @NVM_GET_INFO:
// Command is &struct iwl_nvm_get_info,
// response is &struct iwl_nvm_get_info_rsp
//
    NVM_GET_INFO = 0x2,

//
// @TAS_CONFIG: &union iwl_tas_config_cmd
//
    TAS_CONFIG = 0x3,

//
// @SAR_OFFSET_MAPPING_TABLE_CMD: &struct iwl_sar_offset_mapping_cmd
//
    SAR_OFFSET_MAPPING_TABLE_CMD = 0x4,

//
// @MCC_ALLOWED_AP_TYPE_CMD: &struct iwl_mcc_allowed_ap_type_cmd
//
    MCC_ALLOWED_AP_TYPE_CMD = 0x5,

//
// @LARI_CONFIG_EXTENSION: &struct iwl_lari_config_extension_cmd
//
    LARI_CONFIG_EXTENSION = 0x8,

//
// @PNVM_INIT_COMPLETE_NTFY: &struct iwl_pnvm_init_complete_ntfy
//
    PNVM_INIT_COMPLETE_NTFY = 0xFE,
}

//
// enum iwl_nvm_access_op - NVM access opcode
// @IWL_NVM_READ: read NVM
// @IWL_NVM_WRITE: write NVM
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_access_op {
    IWL_NVM_READ	= 0,
    IWL_NVM_WRITE	= 1,
}

//
// enum iwl_nvm_access_target - target of the NVM_ACCESS_CMD
// @NVM_ACCESS_TARGET_CACHE: access the cache
// @NVM_ACCESS_TARGET_OTP: access the OTP
// @NVM_ACCESS_TARGET_EEPROM: access the EEPROM
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_access_target {
    NVM_ACCESS_TARGET_CACHE = 0,
    NVM_ACCESS_TARGET_OTP = 1,
    NVM_ACCESS_TARGET_EEPROM = 2,
}

//
// enum iwl_nvm_section_type - section types for NVM_ACCESS_CMD
// @NVM_SECTION_TYPE_SW: software section
// @NVM_SECTION_TYPE_REGULATORY: regulatory section
// @NVM_SECTION_TYPE_CALIBRATION: calibration section
// @NVM_SECTION_TYPE_PRODUCTION: production section
// @NVM_SECTION_TYPE_REGULATORY_SDP: regulatory section used by 3168 series
// @NVM_SECTION_TYPE_MAC_OVERRIDE: MAC override section
// @NVM_SECTION_TYPE_PHY_SKU: PHY SKU section
// @NVM_MAX_NUM_SECTIONS: number of sections
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_section_type {
    NVM_SECTION_TYPE_SW = 1,
    NVM_SECTION_TYPE_REGULATORY = 3,
    NVM_SECTION_TYPE_CALIBRATION = 4,
    NVM_SECTION_TYPE_PRODUCTION = 5,
    NVM_SECTION_TYPE_REGULATORY_SDP = 8,
    NVM_SECTION_TYPE_MAC_OVERRIDE = 11,
    NVM_SECTION_TYPE_PHY_SKU = 12,
    NVM_MAX_NUM_SECTIONS = 13,
}

//
// struct iwl_nvm_access_cmd - Request the device to send an NVM section
// @op_code: &enum iwl_nvm_access_op
// @target: &enum iwl_nvm_access_target
// @type: &enum iwl_nvm_section_type
// @offset: offset in bytes into the section
// @length: in bytes, to read/write
// @data: if write operation, the data to write. On read its empty
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_access_cmd {
    pub op_code: u8,
    pub target: u8,
    pub type: __le16,
    pub offset: __le16,
    pub length: __le16,
    pub data: [u8; ],
    pub /: *mut *mut } __packed; / NVM_ACCESS_CMD_API_S_VER_2,
//
// struct iwl_nvm_access_resp - response to NVM_ACCESS_CMD
// @offset: offset in bytes into the section
// @length: in bytes, either how much was written or read
// @type: NVM_SECTION_TYPE_
// @status: 0 for success, fail otherwise
// @data: if read operation, the data returned. Empty on write.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_access_resp {
    pub offset: __le16,
    pub length: __le16,
    pub type: __le16,
    pub status: __le16,
    pub data: [u8; ],
    pub /: *mut *mut } __packed; / NVM_ACCESS_CMD_RESP_API_S_VER_2,
//
// struct iwl_nvm_get_info - request to get NVM data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info {
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_CMD_API_S_VER_1,
//
// enum iwl_nvm_info_general_flags - flags in NVM_GET_INFO resp
// @NVM_GENERAL_FLAGS_EMPTY_OTP: 1 if OTP is empty
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_info_general_flags {
    NVM_GENERAL_FLAGS_EMPTY_OTP	= BIT(0),
}

//
// struct iwl_nvm_get_info_general - general NVM data
// @flags: bit 0: 1 - empty, 0 - non-empty
// @nvm_version: nvm version
// @board_type: board type
// @n_hw_addrs: number of reserved MAC addresses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_general {
    pub flags: __le32,
    pub nvm_version: __le16,
    pub board_type: u8,
    pub n_hw_addrs: u8,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_GENERAL_S_VER_2,
//
// enum iwl_nvm_mac_sku_flags - flags in &iwl_nvm_get_info_sku
// @NVM_MAC_SKU_FLAGS_BAND_2_4_ENABLED: true if 2.4 band enabled
// @NVM_MAC_SKU_FLAGS_BAND_5_2_ENABLED: true if 5.2 band enabled
// @NVM_MAC_SKU_FLAGS_802_11N_ENABLED: true if 11n enabled
// @NVM_MAC_SKU_FLAGS_802_11AC_ENABLED: true if 11ac enabled
// @NVM_MAC_SKU_FLAGS_MIMO_DISABLED: true if MIMO disabled
// @NVM_MAC_SKU_FLAGS_WAPI_ENABLED: true if WAPI enabled
// @NVM_MAC_SKU_FLAGS_REG_CHECK_ENABLED: true if regulatory checker enabled
// @NVM_MAC_SKU_FLAGS_API_LOCK_ENABLED: true if API lock enabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_nvm_mac_sku_flags {
    NVM_MAC_SKU_FLAGS_BAND_2_4_ENABLED	= BIT(0),
    NVM_MAC_SKU_FLAGS_BAND_5_2_ENABLED	= BIT(1),
    NVM_MAC_SKU_FLAGS_802_11N_ENABLED	= BIT(2),
    NVM_MAC_SKU_FLAGS_802_11AC_ENABLED	= BIT(3),
//
// @NVM_MAC_SKU_FLAGS_802_11AX_ENABLED: true if 11ax enabled
//
    NVM_MAC_SKU_FLAGS_802_11AX_ENABLED	= BIT(4),
    NVM_MAC_SKU_FLAGS_MIMO_DISABLED		= BIT(5),
    NVM_MAC_SKU_FLAGS_WAPI_ENABLED		= BIT(8),
    NVM_MAC_SKU_FLAGS_REG_CHECK_ENABLED	= BIT(14),
    NVM_MAC_SKU_FLAGS_API_LOCK_ENABLED	= BIT(15),
}

//
// struct iwl_nvm_get_info_sku - mac information
// @mac_sku_flags: flags for SKU, see &enum iwl_nvm_mac_sku_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_sku {
    pub mac_sku_flags: __le32,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_MAC_SKU_SECTION_S_VER_2,
//
// struct iwl_nvm_get_info_phy - phy information
// @tx_chains: BIT 0 chain A, BIT 1 chain B
// @rx_chains: BIT 0 chain A, BIT 1 chain B
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_phy {
    pub tx_chains: __le32,
    pub rx_chains: __le32,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_PHY_SKU_SECTION_S_VER_1,
pub const IWL_NUM_CHANNELS_V1: c_int = 51;
pub const IWL_NUM_CHANNELS_V2: c_int = 110;
pub const IWL_NUM_CHANNELS_V3: c_int = 115;
//
// struct iwl_nvm_get_info_regulatory_v1 - regulatory information
// @lar_enabled: is LAR enabled
// @channel_profile: regulatory data of this channel
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_regulatory_v1 {
    pub lar_enabled: __le32,
    pub channel_profile: [__le16; IWL_NUM_CHANNELS_V1],
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_REGULATORY_S_VER_1,
//
// struct iwl_nvm_get_info_regulatory_v2 - regulatory information
// @lar_enabled: is LAR enabled
// @n_channels: number of valid channels in the array
// @channel_profile: regulatory data of this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_regulatory_v2 {
    pub lar_enabled: __le32,
    pub n_channels: __le32,
    pub channel_profile: [__le32; IWL_NUM_CHANNELS_V2],
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_REGULATORY_S_VER_2,
//
// struct iwl_nvm_get_info_rsp_v3 - response to get NVM data
// @general: general NVM data
// @mac_sku: data relating to MAC sku
// @phy_sku: data relating to PHY sku
// @regulatory: regulatory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_rsp_v3 {
    pub general: iwl_nvm_get_info_general,
    pub mac_sku: iwl_nvm_get_info_sku,
    pub phy_sku: iwl_nvm_get_info_phy,
    pub regulatory: iwl_nvm_get_info_regulatory_v1,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_RSP_API_S_VER_3,
//
// struct iwl_nvm_get_info_rsp_v4 - response to get NVM data
// @general: general NVM data
// @mac_sku: data relating to MAC sku
// @phy_sku: data relating to PHY sku
// @regulatory: regulatory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_rsp_v4 {
    pub general: iwl_nvm_get_info_general,
    pub mac_sku: iwl_nvm_get_info_sku,
    pub phy_sku: iwl_nvm_get_info_phy,
    pub regulatory: iwl_nvm_get_info_regulatory_v2,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_RSP_API_S_VER_4,
//
// struct iwl_nvm_get_info_regulatory - regulatory information
// @lar_enabled: is LAR enabled
// @n_channels: number of valid channels in the array
// @channel_profile: regulatory data of this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_regulatory {
    pub lar_enabled: __le32,
    pub n_channels: __le32,
    pub channel_profile: [__le32; IWL_NUM_CHANNELS_V3],
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_REGULATORY_S_VER_3,
//
// struct iwl_nvm_get_info_rsp - response to get NVM data
// @general: general NVM data
// @mac_sku: data relating to MAC sku
// @phy_sku: data relating to PHY sku
// @regulatory: regulatory data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_get_info_rsp {
    pub general: iwl_nvm_get_info_general,
    pub mac_sku: iwl_nvm_get_info_sku,
    pub phy_sku: iwl_nvm_get_info_phy,
    pub regulatory: iwl_nvm_get_info_regulatory,
    pub /: *mut *mut } __packed; / REGULATORY_NVM_GET_INFO_RSP_API_S_VER_5,
//
// struct iwl_nvm_access_complete_cmd - NVM_ACCESS commands are completed
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_nvm_access_complete_cmd {
    pub reserved: __le32,
    pub /: *mut *mut } __packed; / NVM_ACCESS_COMPLETE_CMD_API_S_VER_1,
pub const IWL_MCC_US: c_uint = 0x5553;
pub const IWL_MCC_CANADA: c_uint = 0x4341;
//
// struct iwl_mcc_update_cmd - Request the device to update geographic
// regulatory profile according to the given MCC (Mobile Country Code).
// The MCC is two letter-code, ascii upper case[A-Z] or '00' for world domain.
// 'ZZ' MCC will be used to switch to NVM default profile; in this case, the
// MCC in the cmd response will be the relevant MCC in the NVM.
// @mcc: given mobile country code
// @source_id: the source from where we got the MCC, see iwl_mcc_source
// @reserved: reserved for alignment
// @key: integrity key for MCC API OEM testing
// @reserved2: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_update_cmd {
    pub mcc: __le16,
    pub source_id: u8,
    pub reserved: u8,
    pub key: __le32,
    pub reserved2: [u8; 20],
    pub /: *mut *mut } __packed; / LAR_UPDATE_MCC_CMD_API_S_VER_2,
//
// enum iwl_geo_information - geographic information.
// @GEO_NO_INFO: no special info for this geo profile.
// @GEO_WMM_ETSI_5GHZ_INFO: this geo profile limits the WMM params
// for the 5 GHz band.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_geo_information {
    GEO_NO_INFO =			0,
    GEO_WMM_ETSI_5GHZ_INFO =	BIT(0),
}

//
// struct iwl_mcc_update_resp_v3 - response to MCC_UPDATE_CMD.
// Contains the new channel control profile map, if changed, and the new MCC
// (mobile country code).
// The new MCC may be different than what was requested in MCC_UPDATE_CMD.
// @status: see &enum iwl_mcc_update_status
// @mcc: the new applied MCC
// @cap: capabilities for all channels which matches the MCC
// @source_id: the MCC source, see iwl_mcc_source
// @time: time elapsed from the MCC test start (in units of 30 seconds)
// @geo_info: geographic specific profile information
// see &enum iwl_geo_information.
// @n_channels: number of channels in @channels_data.
// @channels: channel control data map, DWORD for each channel. Only the first
// 16bits are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_update_resp_v3 {
    pub status: __le32,
    pub mcc: __le16,
    pub cap: u8,
    pub source_id: u8,
    pub time: __le16,
    pub geo_info: __le16,
    pub n_channels: __le32,
    pub channels: [__le32; ],
    pub /: *mut *mut } __packed; / LAR_UPDATE_MCC_CMD_RESP_S_VER_3,
//
// struct iwl_mcc_update_resp_v4 - response to MCC_UPDATE_CMD.
// Contains the new channel control profile map, if changed, and the new MCC
// (mobile country code).
// The new MCC may be different than what was requested in MCC_UPDATE_CMD.
// @status: see &enum iwl_mcc_update_status
// @mcc: the new applied MCC
// @cap: capabilities for all channels which matches the MCC
// @time: time elapsed from the MCC test start (in units of 30 seconds)
// @geo_info: geographic specific profile information
// see &enum iwl_geo_information.
// @source_id: the MCC source, see iwl_mcc_source
// @reserved: for four bytes alignment.
// @n_channels: number of channels in @channels_data.
// @channels: channel control data map, DWORD for each channel. Only the first
// 16bits are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_update_resp_v4 {
    pub status: __le32,
    pub mcc: __le16,
    pub cap: __le16,
    pub time: __le16,
    pub geo_info: __le16,
    pub source_id: u8,
    pub reserved: [u8; 3],
    pub n_channels: __le32,
    pub channels: [__le32; ],
    pub /: *mut *mut } __packed; / LAR_UPDATE_MCC_CMD_RESP_S_VER_4,
//
// struct iwl_mcc_update_resp_v8 - response to MCC_UPDATE_CMD.
// Contains the new channel control profile map, if changed, and the new MCC
// (mobile country code).
// The new MCC may be different than what was requested in MCC_UPDATE_CMD.
// @status: see &enum iwl_mcc_update_status
// @mcc: the new applied MCC
// @padding: padding for 2 bytes.
// @cap: capabilities for all channels which matches the MCC
// @time: time elapsed from the MCC test start (in units of 30 seconds)
// @geo_info: geographic specific profile information
// see &enum iwl_geo_information.
// @source_id: the MCC source, see iwl_mcc_source
// @reserved: for four bytes alignment.
// @n_channels: number of channels in @channels_data.
// @channels: channel control data map, DWORD for each channel. Only the first
// 16bits are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_update_resp_v8 {
    pub status: __le32,
    pub mcc: __le16,
    pub padding: [u8; 2],
    pub cap: __le32,
    pub time: __le16,
    pub geo_info: __le16,
    pub source_id: u8,
    pub reserved: [u8; 3],
    pub n_channels: __le32,
    pub channels: [__le32; ],
    pub /: *mut *mut } __packed; / LAR_UPDATE_MCC_CMD_RESP_S_VER_8,
//
// struct iwl_mcc_chub_notif - chub notifies of mcc change
// (MCC_CHUB_UPDATE_CMD = 0xc9)
// The Chub (Communication Hub, CommsHUB) is a HW component that connects to
// the cellular and connectivity cores that gets updates of the mcc, and
// notifies the ucode directly of any mcc change.
// The ucode requests the driver to request the device to update geographic
// regulatory  profile according to the given MCC (Mobile Country Code).
// The MCC is two letter-code, ascii upper case[A-Z] or '00' for world domain.
// 'ZZ' MCC will be used to switch to NVM default profile; in this case, the
// MCC in the cmd response will be the relevant MCC in the NVM.
// @mcc: given mobile country code
// @source_id: identity of the change originator, see iwl_mcc_source
// @reserved1: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_chub_notif {
    pub mcc: __le16,
    pub source_id: u8,
    pub reserved1: u8,
    pub __packed: },
// LAR_MCC_NOTIFY_S_VER_1
// LAR_MCC_NOTIFY_S_VER_2
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mcc_update_status {
    MCC_RESP_NEW_CHAN_PROFILE,
    MCC_RESP_SAME_CHAN_PROFILE,
    MCC_RESP_INVALID,
    MCC_RESP_NVM_DISABLED,
    MCC_RESP_ILLEGAL,
    MCC_RESP_LOW_PRIORITY,
    MCC_RESP_TEST_MODE_ACTIVE,
    MCC_RESP_TEST_MODE_NOT_ACTIVE,
    MCC_RESP_TEST_MODE_DENIAL_OF_SERVICE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_mcc_source {
    MCC_SOURCE_OLD_FW = 0,
    MCC_SOURCE_ME = 1,
    MCC_SOURCE_BIOS = 2,
    MCC_SOURCE_3G_LTE_HOST = 3,
    MCC_SOURCE_3G_LTE_DEVICE = 4,
    MCC_SOURCE_WIFI = 5,
    MCC_SOURCE_RESERVED = 6,
    MCC_SOURCE_DEFAULT = 7,
    MCC_SOURCE_UNINITIALIZED = 8,
    MCC_SOURCE_MCC_API = 9,
    MCC_SOURCE_GET_CURRENT = 0x10,
    MCC_SOURCE_GETTING_MCC_TEST_MODE = 0x11,
}

pub const IWL_WTAS_BLACK_LIST_MAX: c_int = 16;
//
// struct iwl_tas_config_cmd_common - configures the TAS.
// This is also the v2 structure.
// @block_list_size: size of relevant field in block_list_array
// @block_list_array: list of countries where TAS must be disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_config_cmd_common {
    pub block_list_size: __le32,
    pub block_list_array: [__le32; IWL_WTAS_BLACK_LIST_MAX],
    pub /: *mut *mut } __packed; / TAS_CONFIG_CMD_API_S_VER_2,
//
// struct iwl_tas_config_cmd_v3 - configures the TAS
// @override_tas_iec: indicates whether to override default value of IEC regulatory
// @enable_tas_iec: in case override_tas_iec is set -
// indicates whether IEC regulatory is enabled or disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_config_cmd_v3 {
    pub override_tas_iec: __le16,
    pub enable_tas_iec: __le16,
    pub /: *mut *mut } __packed; / TAS_CONFIG_CMD_API_S_VER_3,
//
// enum iwl_tas_uhb_allowed_flags - per country TAS UHB allowed flags.
// @TAS_UHB_ALLOWED_CANADA: TAS UHB is allowed in Canada. This flag is valid
// only when fw has %IWL_UCODE_TLV_CAPA_UHB_CANADA_TAS_SUPPORT capability.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_tas_uhb_allowed_flags {
    TAS_UHB_ALLOWED_CANADA	= BIT(0),
}

//
// struct iwl_tas_config_cmd_v4 - configures the TAS
// @override_tas_iec: indicates whether to override default value of IEC regulatory
// @enable_tas_iec: in case override_tas_iec is set -
// indicates whether IEC regulatory is enabled or disabled
// @usa_tas_uhb_allowed: if set, allow TAS UHB in the USA
// @uhb_allowed_flags: see &enum iwl_tas_uhb_allowed_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_config_cmd_v4 {
    pub override_tas_iec: u8,
    pub enable_tas_iec: u8,
    pub usa_tas_uhb_allowed: u8,
    pub uhb_allowed_flags: u8,
    pub /: *mut *mut } __packed; / TAS_CONFIG_CMD_API_S_VER_4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_config_cmd_v2_v4 {
    pub common: iwl_tas_config_cmd_common,
    pub v3: iwl_tas_config_cmd_v3,
    pub v4: iwl_tas_config_cmd_v4,
}

//
// enum bios_source - source of bios data
// @BIOS_SOURCE_NONE: BIOS source is not defined
// @BIOS_SOURCE_ACPI: BIOS source is ACPI
// @BIOS_SOURCE_UEFI: BIOS source is UEFI
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bios_source {
    BIOS_SOURCE_NONE,
    BIOS_SOURCE_ACPI,
    BIOS_SOURCE_UEFI,
}

//
// struct iwl_bios_config_hdr - BIOS configuration header
// @table_source: see &enum bios_source
// @table_revision: table revision.
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bios_config_hdr {
    pub table_source: u8,
    pub table_revision: u8,
    pub reserved: [u8; 2],
    pub /: *mut *mut } __packed; / BIOS_CONFIG_HDR_API_S_VER_1,
//
// struct iwl_lari_config_extension_cmd - extend LARI configuration
//
// LARI_CONFIG_CHANGE's version must remain stable for frozen firmware.
// Because the driver might not know this version but still load that
// frozen FW and then send some default old version of LARI, causing the FW to
// assert about the bad size of it. To handle this, we do the following:
// 1. For newer firmware: increase the LARI_CONFIG_CHANGE version to support the
// new firmware API with extra UHB bits.
// 2. For frozen firmware: add a special alternative API that doesn't require
// modifying the frozen LARI_CONFIG_CHANGE's version.
// @dsm_table_hdr: BIOS DSM table source and revision
// @oem_uhb_allow_extension_bitmap: extension bitmap for OEM UHB config
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lari_config_extension_cmd {
    pub dsm_table_hdr: iwl_bios_config_hdr,
    pub oem_uhb_allow_extension_bitmap: __le32,
    pub reserved: [__le32; 10],
    pub /: *mut *mut } __packed; / LARI_CONFIG_EXTENSION_CMD_API_S_VER_1,
//
// struct bios_value_u32 - BIOS configuration.
// @hdr: bios config header
// @value: value in bios.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_value_u32 {
    pub hdr: iwl_bios_config_hdr,
    pub value: __le32,
    pub /: *mut *mut } __packed; / BIOS_CONFIG_DATA_U32_API_S_VER_1,
//
// struct iwl_tas_config_cmd - configures the TAS.
// @block_list_size: size of relevant field in block_list_array
// @block_list_array: list of countries where TAS must be disabled
// @reserved: reserved
// @tas_config_info: see @struct bios_value_u32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tas_config_cmd {
    pub block_list_size: __le16,
    pub block_list_array: [__le16; IWL_WTAS_BLACK_LIST_MAX],
    pub reserved: [u8; 2],
    pub tas_config_info: bios_value_u32,
    pub /: *mut *mut } __packed; / TAS_CONFIG_CMD_API_S_VER_5,
//
// enum iwl_lari_config_masks - bit masks for the various LARI config operations
// @LARI_CONFIG_DISABLE_11AC_UKRAINE_MSK: disable 11ac in ukraine
// @LARI_CONFIG_CHANGE_ETSI_TO_PASSIVE_MSK: ETSI 5.8GHz SRD passive scan
// @LARI_CONFIG_CHANGE_ETSI_TO_DISABLED_MSK: ETSI 5.8GHz SRD disabled
// @LARI_CONFIG_ENABLE_5G2_IN_INDONESIA_MSK: enable 5.15/5.35GHz bands in
// Indonesia
// @LARI_CONFIG_ENABLE_CHINA_22_REG_SUPPORT_MSK: enable 2022 china regulatory
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_lari_config_masks {
    LARI_CONFIG_DISABLE_11AC_UKRAINE_MSK		= BIT(0),
    LARI_CONFIG_CHANGE_ETSI_TO_PASSIVE_MSK		= BIT(1),
    LARI_CONFIG_CHANGE_ETSI_TO_DISABLED_MSK		= BIT(2),
    LARI_CONFIG_ENABLE_5G2_IN_INDONESIA_MSK		= BIT(3),
    LARI_CONFIG_ENABLE_CHINA_22_REG_SUPPORT_MSK	= BIT(7),
}

pub const IWL_11AX_UKRAINE_MASK: c_int = 3;
pub const IWL_11AX_UKRAINE_SHIFT: c_int = 8;
//
// struct iwl_lari_config_change_cmd_v1 - change LARI configuration
// @config_bitmap: bit map of the config commands. each bit will trigger a
// different predefined FW config operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lari_config_change_cmd_v1 {
    pub config_bitmap: __le32,
    pub /: *mut *mut } __packed; / LARI_CHANGE_CONF_CMD_S_VER_1,
//
// struct iwl_lari_config_change_cmd_v6 - change LARI configuration
// @config_bitmap: Bitmap of the config commands. Each bit will trigger a
// different predefined FW config operation.
// @oem_uhb_allow_bitmap: Bitmap of UHB enabled MCC sets.
// @oem_11ax_allow_bitmap: Bitmap of 11ax allowed MCCs. There are two bits
// per country, one to indicate whether to override and the other to
// indicate the value to use.
// @oem_unii4_allow_bitmap: Bitmap of unii4 allowed MCCs.There are two bits
// per country, one to indicate whether to override and the other to
// indicate allow/disallow unii4 channels.
// @chan_state_active_bitmap: Bitmap for overriding channel state to active.
// Each bit represents a country or region to activate, according to the BIOS
// definitions.
// @force_disable_channels_bitmap: Bitmap of disabled bands/channels.
// Each bit represents a set of channels in a specific band that should be disabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lari_config_change_cmd_v6 {
    pub config_bitmap: __le32,
    pub oem_uhb_allow_bitmap: __le32,
    pub oem_11ax_allow_bitmap: __le32,
    pub oem_unii4_allow_bitmap: __le32,
    pub chan_state_active_bitmap: __le32,
    pub force_disable_channels_bitmap: __le32,
    pub /: *mut *mut } __packed; / LARI_CHANGE_CONF_CMD_S_VER_6,
//
// struct iwl_lari_config_change_cmd_v8 - change LARI configuration
// @config_bitmap: Bitmap of the config commands. Each bit will trigger a
// different predefined FW config operation.
// @oem_uhb_allow_bitmap: Bitmap of UHB enabled MCC sets.
// @oem_11ax_allow_bitmap: Bitmap of 11ax allowed MCCs. There are two bits
// per country, one to indicate whether to override and the other to
// indicate the value to use.
// @oem_unii4_allow_bitmap: Bitmap of unii4 allowed MCCs.There are two bits
// per country, one to indicate whether to override and the other to
// indicate allow/disallow unii4 channels.
// bit 0 - 3: supported.
// @chan_state_active_bitmap: Bitmap to enable different bands per country
// or region.
// Each bit represents a country or region, and a band to activate
// according to the BIOS definitions.
// bit 0 - 4: supported.
// @force_disable_channels_bitmap: Bitmap of disabled bands/channels.
// Each bit represents a set of channels in a specific band that should be
// disabled
// @edt_bitmap: Bitmap of energy detection threshold table.
// Disable/enable the EDT optimization method for different band.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lari_config_change_cmd_v8 {
    pub config_bitmap: __le32,
    pub oem_uhb_allow_bitmap: __le32,
    pub oem_11ax_allow_bitmap: __le32,
    pub oem_unii4_allow_bitmap: __le32,
    pub chan_state_active_bitmap: __le32,
    pub force_disable_channels_bitmap: __le32,
    pub edt_bitmap: __le32,
    pub __packed: },
// LARI_CHANGE_CONF_CMD_S_VER_8
//
// struct iwl_lari_config_change_cmd - change LARI configuration
// @config_bitmap: Bitmap of the config commands. Each bit will trigger a
// different predefined FW config operation.
// @oem_uhb_allow_bitmap: Bitmap of UHB enabled MCC sets.
// @oem_11ax_allow_bitmap: Bitmap of 11ax allowed MCCs. There are two bits
// per country, one to indicate whether to override and the other to
// indicate the value to use.
// @oem_unii4_allow_bitmap: Bitmap of unii4 allowed MCCs.There are two bits
// per country, one to indicate whether to override and the other to
// indicate allow/disallow unii4 channels.
// @chan_state_active_bitmap: Bitmap to enable different bands per country
// or region.
// Each bit represents a country or region, and a band to activate
// according to the BIOS definitions.
// bit 0 - 6: supported.
// @force_disable_channels_bitmap: Bitmap of disabled bands/channels.
// Each bit represents a set of channels in a specific band that should be
// disabled
// @edt_bitmap: Bitmap of energy detection threshold table.
// Disable/enable the EDT optimization method for different band.
// @oem_320mhz_allow_bitmap: 320Mhz bandwidth enablement bitmap per MCC.
// bit0: enable 320Mhz in Japan.
// bit1: enable 320Mhz in South Korea.
// bit 2 - 31: reserved.
// @oem_11be_allow_bitmap: Bitmap of 11be allowed MCCs. No need to mask out the
// unsupported bits
// bit0: enable 11be in China(CB/CN).
// bit1: enable 11be in South Korea.
// bit 2 - 31: reserved.
// @oem_11bn_allow_bitmap: Bitmap of 11bn allowed MCCs. The firmware expects to
// get the data from the BIOS.
// @oem_unii9_enable: UNII-9 enablement as read from the BIOS
// @bios_hdr: bios config header
// @oem_uhb_allow_extension_bitmap: DSM Function 4 data as an extension of UHB
// enabled MCC sets
// @bios_wcpe_hdr: puncturing config header
// @wcpe_bitmap: bitmap of puncturing enablement per MCC
// @bios_wbem_hdr: 320 MHz per-MCC WBEM config header
// @reserved: reserved
// @oem_supported_dsm_bitmap: DSM function 0 bitmap describing supported
// DSM function indices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_lari_config_change_cmd {
    pub config_bitmap: __le32,
    pub oem_uhb_allow_bitmap: __le32,
    pub oem_11ax_allow_bitmap: __le32,
    pub oem_unii4_allow_bitmap: __le32,
    pub chan_state_active_bitmap: __le32,
    pub force_disable_channels_bitmap: __le32,
    pub edt_bitmap: __le32,
    pub oem_320mhz_allow_bitmap: __le32,
    pub oem_11be_allow_bitmap: __le32,
// since version 13
    pub oem_11bn_allow_bitmap: __le32,
// since version 13
    pub oem_unii9_enable: __le32,
// since version 13
    pub bios_hdr: iwl_bios_config_hdr,
// All the below are since version 14
    pub oem_uhb_allow_extension_bitmap: __le32,
    pub bios_wcpe_hdr: iwl_bios_config_hdr,
    pub wcpe_bitmap: __le32,
    pub bios_wbem_hdr: iwl_bios_config_hdr,
    pub reserved: [__le32; 10],
// since version 15
    pub oem_supported_dsm_bitmap: __le32,
    pub __packed: },
// LARI_CHANGE_CONF_CMD_S_VER_12
// LARI_CHANGE_CONF_CMD_S_VER_13
// LARI_CHANGE_CONF_CMD_S_VER_14
// LARI_CHANGE_CONF_CMD_S_VER_15
//
// Activate UNII-1 (5.2GHz) for World Wide

pub const CHAN_STATE_ACTIVE_BITMAP_CMD_V8: c_uint = 0x1F;
pub const CHAN_STATE_ACTIVE_BITMAP_CMD_V12: c_uint = 0x7F;
//
// struct iwl_pnvm_init_complete_ntfy - PNVM initialization complete
// @status: PNVM image loading status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_pnvm_init_complete_ntfy {
    pub status: __le32,
    pub /: *mut *mut } __packed; / PNVM_INIT_COMPLETE_NTFY_S_VER_1,
pub const UATS_TABLE_ROW_SIZE: c_int = 26;
pub const UATS_TABLE_COL_SIZE: c_int = 13;
//
// struct iwl_mcc_allowed_ap_type_cmd_v1 - struct for MCC_ALLOWED_AP_TYPE_CMD
// @mcc_to_ap_type_map: mapping an MCC to 6 GHz AP type support (UATS)
// @reserved: reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_allowed_ap_type_cmd_v1 {
    pub mcc_to_ap_type_map: [u8; UATS_TABLE_ROW_SIZE][UATS_TABLE_COL_SIZE],
    pub reserved: __le16,
    pub /: *mut *mut } __packed; / MCC_ALLOWED_AP_TYPE_CMD_API_S_VER_1,
//
// struct iwl_mcc_allowed_ap_type_cmd - struct for MCC_ALLOWED_AP_TYPE_CMD
// @mcc_to_ap_type_map: mapping an MCC to 6 GHz AP type support (UATS)
// @mcc_to_ap_type_unii9_map: mapping an MCC to UNII-9 AP type support allowed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mcc_allowed_ap_type_cmd {
    pub mcc_to_ap_type_map: [u8; UATS_TABLE_ROW_SIZE][UATS_TABLE_COL_SIZE],
    pub mcc_to_ap_type_unii9_map: [u8; UATS_TABLE_ROW_SIZE][UATS_TABLE_COL_SIZE],
    pub /: *mut *mut } __packed; / MCC_ALLOWED_AP_TYPE_CMD_API_S_VER_2,
