//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192se/fw.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const RTL8190_MAX_FIRMWARE_CODE_SIZE: c_int = 64000;
pub const RTL8190_MAX_RAW_FIRMWARE_CODE_SIZE: c_int = 90000;
pub const RTL8190_CPU_START_OFFSET: c_uint = 0x80;
// Firmware Local buffer size. 64k
pub const MAX_FIRMWARE_CODE_SIZE: c_uint = 0xFF00;
pub const RT_8192S_FIRMWARE_HDR_SIZE: c_int = 80;
pub const RT_8192S_FIRMWARE_HDR_EXCLUDE_PRI_SIZE: c_int = 32;
// support till 64 bit bus width OS
pub const MAX_DEV_ADDR_SIZE: c_int = 8;
pub const MAX_FIRMWARE_INFORMATION_SIZE: c_int = 32;

pub const ENCRYPTION_MAX_OVERHEAD: c_int = 128;
pub const MAX_FRAGMENT_COUNT: c_int = 8;

pub const H2C_TX_CMD_HDR_LEN: c_int = 8;
// The following DM control code are for Reg0x364,

pub const FW_DISABLE_ALL_DM: c_int = 0;
pub const FW_PWR_TRK_PARAM_CLR: c_uint = 0x0000ffff;
pub const FW_RA_PARAM_CLR: c_uint = 0xffff0000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum desc_packet_type {
    DESC_PACKET_TYPE_INIT = 0,
    DESC_PACKET_TYPE_NORMAL = 1,
}

// 8-bytes alignment required
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_priv {
// --- long word 0 ----
// 0x12: CE product, 0x92: IT product
    pub signature_0: u8,
// 0x87: CE product, 0x81: IT product
    pub signature_1: u8,
// 0x81: PCI-AP, 01:PCIe, 02: 92S-U,
// 0x82: USB-AP, 0x12: 72S-U, 03:SDIO
    pub hci_sel: u8,
// the same value as reigster value
    pub chip_version: u8,
// customer  ID low byte
    pub customer_id_0: u8,
// customer  ID high byte
    pub customer_id_1: u8,
// 0x11:  1T1R, 0x12: 1T2R,
// 0x92: 1T2R turbo, 0x22: 2T2R
    pub rf_config: u8,
// 4: 4EP, 6: 6EP, 11: 11EP
    pub usb_ep_num: u8,
// --- long word 1 ----
// regulatory class bit map 0
    pub regulatory_class_0: u8,
// regulatory class bit map 1
    pub regulatory_class_1: u8,
// regulatory class bit map 2
    pub regulatory_class_2: u8,
// regulatory class bit map 3
    pub regulatory_class_3: u8,
// 0:SWSI, 1:HWSI, 2:HWPI
    pub rfintfs: u8,
    pub def_nettype: u8,
    pub rsvd010: u8,
    pub rsvd011: u8,
// --- long word 2 ----
// 0x00: normal, 0x03: MACLBK, 0x01: PHYLBK
    pub lbk_mode: u8,
// 1: for MP use, 0: for normal
// driver (to be discussed)
    pub mp_mode: u8,
    pub rsvd020: u8,
    pub rsvd021: u8,
    pub rsvd022: u8,
    pub rsvd023: u8,
    pub rsvd024: u8,
    pub rsvd025: u8,
// --- long word 3 ----
// QoS enable
    pub qos_en: u8,
// 40MHz BW enable
// 4181 convert AMSDU to AMPDU, 0: disable
    pub bw_40mhz_en: u8,
    pub amsdu2ampdu_en: u8,
// 11n AMPDU enable
    pub ampdu_en: u8,
// FW offloads, 0: driver handles
    pub rate_control_offload: u8,
// FW offloads, 0: driver handles
    pub aggregation_offload: u8,
    pub rsvd030: u8,
    pub rsvd031: u8,
// --- long word 4 ----
// 1. FW offloads, 0: driver handles
    pub beacon_offload: u8,
// 2. FW offloads, 0: driver handles
    pub mlme_offload: u8,
// 3. FW offloads, 0: driver handles
    pub hwpc_offload: u8,
// 4. FW offloads, 0: driver handles
    pub tcp_checksum_offload: u8,
// 5. FW offloads, 0: driver handles
    pub tcp_offload: u8,
// 6. FW offloads, 0: driver handles
    pub ps_control_offload: u8,
// 7. FW offloads, 0: driver handles
    pub wwlan_offload: u8,
    pub rsvd040: u8,
// --- long word 5 ----
// tcp tx packet length low byte
    pub tcp_tx_frame_len_L: u8,
// tcp tx packet length high byte
    pub tcp_tx_frame_len_H: u8,
// tcp rx packet length low byte
    pub tcp_rx_frame_len_L: u8,
// tcp rx packet length high byte
    pub tcp_rx_frame_len_H: u8,
    pub rsvd050: u8,
    pub rsvd051: u8,
    pub rsvd052: u8,
    pub rsvd053: u8,
}

// 8-byte alinment required
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_hdr {
// --- LONG WORD 0 ----
    pub signature: u16,
// 0x8000 ~ 0x8FFF for FPGA version,
// 0x0000 ~ 0x7FFF for ASIC version,
    pub version: u16,
// define the size of boot loader
    pub dmem_size: u32,
// --- LONG WORD 1 ----
// define the size of FW in IMEM
    pub img_imem_size: u32,
// define the size of FW in SRAM
    pub img_sram_size: u32,
// --- LONG WORD 2 ----
// define the size of DMEM variable
    pub fw_priv_size: u32,
    pub rsvd0: u32,
// --- LONG WORD 3 ----
    pub rsvd1: u32,
    pub rsvd2: u32,
    pub fwpriv: fw_priv,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_status {
    FW_STATUS_INIT = 0,
    FW_STATUS_LOAD_IMEM = 1,
    FW_STATUS_LOAD_EMEM = 2,
    FW_STATUS_LOAD_DMEM = 3,
    FW_STATUS_READY = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt_firmware {
    pub pfwheader: *mut fw_hdr,
    pub fwstatus: fw_status,
    pub firmwareversion: u16,
    pub fw_imem: [u8; RTL8190_MAX_FIRMWARE_CODE_SIZE],
    pub fw_emem: [u8; RTL8190_MAX_FIRMWARE_CODE_SIZE],
    pub fw_imem_len: u32,
    pub fw_emem_len: u32,
    pub sz_fw_tmpbuffer: [u8; RTL8190_MAX_RAW_FIRMWARE_CODE_SIZE],
    pub sz_fw_tmpbufferlen: u32,
    pub cmdpacket_fragthresold: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_set_pwrmode_parm {
    pub mode: u8,
    pub flag_low_traffic_en: u8,
    pub flag_lpnav_en: u8,
    pub flag_rf_low_snr_en: u8,
// 1: dps, 0: 32k
    pub flag_dps_en: u8,
    pub bcn_rx_en: u8,
    pub bcn_pass_cnt: u8,
// beacon TO (ms). ¡§=0¡¨ no limit.
    pub bcn_to: u8,
    pub bcn_itv: u16,
// only for VOIP mode.
    pub app_itv: u8,
    pub awake_bcn_itvl: u8,
    pub smart_ps: u8,
// unit: 100 ms
    pub bcn_pass_period: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_joinbss_rpt_parm {
    pub opmode: u8,
    pub ps_qos_info: u8,
    pub bssid: [u8; 6],
    pub bcnitv: u16,
    pub aid: u16,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_wpa_ptk {
// EAPOL-Key Key Confirmation Key (KCK)
    pub kck: [u8; 16],
// EAPOL-Key Key Encryption Key (KEK)
    pub kek: [u8; 16],
// Temporal Key 1 (TK1)
    pub tk1: [u8; 16],
// Temporal Key 2 (TK2)
    pub tk2: [u8; 16],
    pub tx_mic_key: [u8; 8],
    pub rx_mic_key: [u8; 8],
    pub athu: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h2c_wpa_two_way_parm {
// algorithm TKIP or AES
    pub pairwise_en_alg: u8,
    pub group_en_alg: u8,
    pub wpa_ptk_value: h2c_wpa_ptk,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum h2c_cmd {
    FW_H2C_SETPWRMODE = 0,
    FW_H2C_JOINBSSRPT = 1,
    FW_H2C_WOWLAN_UPDATE_GTK = 2,
    FW_H2C_WOWLAN_UPDATE_IV = 3,
    FW_H2C_WOWLAN_OFFLOAD = 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_h2c_cmd {
    H2C_READ_MACREG_CMD,				/*0*/
    H2C_WRITE_MACREG_CMD,
    H2C_READBB_CMD,
    H2C_WRITEBB_CMD,
    H2C_READRF_CMD,
    H2C_WRITERF_CMD,				/*5*/
    H2C_READ_EEPROM_CMD,
    H2C_WRITE_EEPROM_CMD,
    H2C_READ_EFUSE_CMD,
    H2C_WRITE_EFUSE_CMD,
    H2C_READ_CAM_CMD,				/*10*/
    H2C_WRITE_CAM_CMD,
    H2C_SETBCNITV_CMD,
    H2C_SETMBIDCFG_CMD,
    H2C_JOINBSS_CMD,
    H2C_DISCONNECT_CMD,				/*15*/
    H2C_CREATEBSS_CMD,
    H2C_SETOPMODE_CMD,
    H2C_SITESURVEY_CMD,
    H2C_SETAUTH_CMD,
    H2C_SETKEY_CMD,					/*20*/
    H2C_SETSTAKEY_CMD,
    H2C_SETASSOCSTA_CMD,
    H2C_DELASSOCSTA_CMD,
    H2C_SETSTAPWRSTATE_CMD,
    H2C_SETBASICRATE_CMD,				/*25*/
    H2C_GETBASICRATE_CMD,
    H2C_SETDATARATE_CMD,
    H2C_GETDATARATE_CMD,
    H2C_SETPHYINFO_CMD,
    H2C_GETPHYINFO_CMD,				/*30*/
    H2C_SETPHY_CMD,
    H2C_GETPHY_CMD,
    H2C_READRSSI_CMD,
    H2C_READGAIN_CMD,
    H2C_SETATIM_CMD,				/*35*/
    H2C_SETPWRMODE_CMD,
    H2C_JOINBSSRPT_CMD,
    H2C_SETRATABLE_CMD,
    H2C_GETRATABLE_CMD,
    H2C_GETCCXREPORT_CMD,				/*40*/
    H2C_GETDTMREPORT_CMD,
    H2C_GETTXRATESTATICS_CMD,
    H2C_SETUSBSUSPEND_CMD,
    H2C_SETH2CLBK_CMD,
    H2C_TMP1,					/*45*/
    H2C_WOWLAN_UPDATE_GTK_CMD,
    H2C_WOWLAN_FW_OFFLOAD,
    H2C_TMP2,
    H2C_TMP3,
    H2C_WOWLAN_UPDATE_IV_CMD,			/*50*/
    H2C_TMP4,
}

// The following macros are used for FW
// CMD map and parameter updated.

    pub \: udelay(1000);,
    pub \: rtlpriv->rtlhal.fwcmd_iomap &= (~_bit);,

    pub _val: rtlpriv->rtlhal.fwcmd_iomap =,

    pub \: rtl_write_word(rtlpriv, LBUS_MON_ADDR, (u16)_val);,
    pub \: FW_CMD_IO_UPDATE(rtlpriv, _val);,

    pub \: rtl_write_dword(rtlpriv, LBUS_ADDR_MASK, _val);,
    pub \: rtlpriv->rtlhal.fwcmd_ioparam = _val;,

    pub hw): *mut int rtl92s_download_fw(struct ieee80211_hw,
    pub mode): *mut *mut void rtl92s_set_fw_pwrmode_cmd(struct ieee80211_hw hw, u8,
    pub ps_qosinfo): u8 mstatus, u8,
