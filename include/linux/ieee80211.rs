//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211.h
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
// IEEE 802.11 defines
//
// Copyright (c) 2001-2002, SSH Communications Security Corp and Jouni Malinen
// <jkmaline@cc.hut.fi>
// Copyright (c) 2002-2003, Jouni Malinen <jkmaline@cc.hut.fi>
// Copyright (c) 2005, Devicescape Software, Inc.
// Copyright (c) 2006, Michael Wu <flamingice@sourmilk.net>
// Copyright (c) 2013 - 2014 Intel Mobile Communications GmbH
// Copyright (c) 2016 - 2017 Intel Deutschland GmbH
// Copyright (c) 2018 - 2026 Intel Corporation
//

//
// DS bit usage
//
// TA = transmitter address
// RA = receiver address
// DA = destination address
// SA = source address
//
// ToDS    FromDS  A1(RA)  A2(TA)  A3      A4      Use
// -----------------------------------------------------------------
// 0       0       DA      SA      BSSID   -       IBSS/DLS
// 0       1       DA      BSSID   SA      -       AP -> STA
// 1       0       BSSID   SA      DA      -       AP <- STA
// 1       1       RA      TA      DA      SA      unspecified (WDS)
//
pub const FCS_LEN: c_int = 4;
pub const IEEE80211_FCTL_VERS: c_uint = 0x0003;
pub const IEEE80211_FCTL_FTYPE: c_uint = 0x000c;
pub const IEEE80211_FCTL_STYPE: c_uint = 0x00f0;

pub const IEEE80211_FCTL_TODS: c_uint = 0x0100;
pub const IEEE80211_FCTL_FROMDS: c_uint = 0x0200;
pub const IEEE80211_FCTL_MOREFRAGS: c_uint = 0x0400;
pub const IEEE80211_FCTL_RETRY: c_uint = 0x0800;
pub const IEEE80211_FCTL_PM: c_uint = 0x1000;
pub const IEEE80211_FCTL_MOREDATA: c_uint = 0x2000;
pub const IEEE80211_FCTL_PROTECTED: c_uint = 0x4000;
pub const IEEE80211_FCTL_ORDER: c_uint = 0x8000;
pub const IEEE80211_FCTL_CTL_EXT: c_uint = 0x0f00;
pub const IEEE80211_SCTL_FRAG: c_uint = 0x000F;
pub const IEEE80211_SCTL_SEQ: c_uint = 0xFFF0;
pub const IEEE80211_FTYPE_MGMT: c_uint = 0x0000;
pub const IEEE80211_FTYPE_CTL: c_uint = 0x0004;
pub const IEEE80211_FTYPE_DATA: c_uint = 0x0008;
pub const IEEE80211_FTYPE_EXT: c_uint = 0x000c;
// management
pub const IEEE80211_STYPE_ASSOC_REQ: c_uint = 0x0000;
pub const IEEE80211_STYPE_ASSOC_RESP: c_uint = 0x0010;
pub const IEEE80211_STYPE_REASSOC_REQ: c_uint = 0x0020;
pub const IEEE80211_STYPE_REASSOC_RESP: c_uint = 0x0030;
pub const IEEE80211_STYPE_PROBE_REQ: c_uint = 0x0040;
pub const IEEE80211_STYPE_PROBE_RESP: c_uint = 0x0050;
pub const IEEE80211_STYPE_BEACON: c_uint = 0x0080;
pub const IEEE80211_STYPE_ATIM: c_uint = 0x0090;
pub const IEEE80211_STYPE_DISASSOC: c_uint = 0x00A0;
pub const IEEE80211_STYPE_AUTH: c_uint = 0x00B0;
pub const IEEE80211_STYPE_DEAUTH: c_uint = 0x00C0;
pub const IEEE80211_STYPE_ACTION: c_uint = 0x00D0;
// control
pub const IEEE80211_STYPE_TRIGGER: c_uint = 0x0020;
pub const IEEE80211_STYPE_CTL_EXT: c_uint = 0x0060;
pub const IEEE80211_STYPE_BACK_REQ: c_uint = 0x0080;
pub const IEEE80211_STYPE_BACK: c_uint = 0x0090;
pub const IEEE80211_STYPE_PSPOLL: c_uint = 0x00A0;
pub const IEEE80211_STYPE_RTS: c_uint = 0x00B0;
pub const IEEE80211_STYPE_CTS: c_uint = 0x00C0;
pub const IEEE80211_STYPE_ACK: c_uint = 0x00D0;
pub const IEEE80211_STYPE_CFEND: c_uint = 0x00E0;
pub const IEEE80211_STYPE_CFENDACK: c_uint = 0x00F0;
// data
pub const IEEE80211_STYPE_DATA: c_uint = 0x0000;
pub const IEEE80211_STYPE_DATA_CFACK: c_uint = 0x0010;
pub const IEEE80211_STYPE_DATA_CFPOLL: c_uint = 0x0020;
pub const IEEE80211_STYPE_DATA_CFACKPOLL: c_uint = 0x0030;
pub const IEEE80211_STYPE_NULLFUNC: c_uint = 0x0040;
pub const IEEE80211_STYPE_CFACK: c_uint = 0x0050;
pub const IEEE80211_STYPE_CFPOLL: c_uint = 0x0060;
pub const IEEE80211_STYPE_CFACKPOLL: c_uint = 0x0070;
pub const IEEE80211_STYPE_QOS_DATA: c_uint = 0x0080;
pub const IEEE80211_STYPE_QOS_DATA_CFACK: c_uint = 0x0090;
pub const IEEE80211_STYPE_QOS_DATA_CFPOLL: c_uint = 0x00A0;
pub const IEEE80211_STYPE_QOS_DATA_CFACKPOLL: c_uint = 0x00B0;
pub const IEEE80211_STYPE_QOS_NULLFUNC: c_uint = 0x00C0;
pub const IEEE80211_STYPE_QOS_CFACK: c_uint = 0x00D0;
pub const IEEE80211_STYPE_QOS_CFPOLL: c_uint = 0x00E0;
pub const IEEE80211_STYPE_QOS_CFACKPOLL: c_uint = 0x00F0;
// extension, added by 802.11ad
pub const IEEE80211_STYPE_DMG_BEACON: c_uint = 0x0000;
pub const IEEE80211_STYPE_S1G_BEACON: c_uint = 0x0010;
pub const IEEE80211_NDP_FTYPE_CTS: c_int = 0;
pub const IEEE80211_NDP_FTYPE_CF_END: c_int = 0;
pub const IEEE80211_NDP_FTYPE_PS_POLL: c_int = 1;
pub const IEEE80211_NDP_FTYPE_ACK: c_int = 2;
pub const IEEE80211_NDP_FTYPE_PS_POLL_ACK: c_int = 3;
pub const IEEE80211_NDP_FTYPE_BA: c_int = 4;
pub const IEEE80211_NDP_FTYPE_BF_REPORT_POLL: c_int = 5;
pub const IEEE80211_NDP_FTYPE_PAGING: c_int = 6;
pub const IEEE80211_NDP_FTYPE_PREQ: c_int = 7;

// NDP CMAC frame fields
pub const IEEE80211_NDP_FTYPE: c_uint = 0x0000000000000007;
pub const IEEE80211_NDP_FTYPE_S: c_uint = 0x0000000000000000;
// 1M Probe Request 11ah 9.9.3.1.1
pub const IEEE80211_NDP_1M_PREQ_ANO: c_uint = 0x0000000000000008;
pub const IEEE80211_NDP_1M_PREQ_ANO_S: c_int = 3;
pub const IEEE80211_NDP_1M_PREQ_CSSID: c_uint = 0x00000000000FFFF0;
pub const IEEE80211_NDP_1M_PREQ_CSSID_S: c_int = 4;
pub const IEEE80211_NDP_1M_PREQ_RTYPE: c_uint = 0x0000000000100000;
pub const IEEE80211_NDP_1M_PREQ_RTYPE_S: c_int = 20;
pub const IEEE80211_NDP_1M_PREQ_RSV: c_uint = 0x0000000001E00000;
pub const IEEE80211_NDP_1M_PREQ_RSV: c_uint = 0x0000000001E00000;
// 2M Probe Request 11ah 9.9.3.1.2
pub const IEEE80211_NDP_2M_PREQ_ANO: c_uint = 0x0000000000000008;
pub const IEEE80211_NDP_2M_PREQ_ANO_S: c_int = 3;
pub const IEEE80211_NDP_2M_PREQ_CSSID: c_uint = 0x0000000FFFFFFFF0;
pub const IEEE80211_NDP_2M_PREQ_CSSID_S: c_int = 4;
pub const IEEE80211_NDP_2M_PREQ_RTYPE: c_uint = 0x0000001000000000;
pub const IEEE80211_NDP_2M_PREQ_RTYPE_S: c_int = 36;
pub const IEEE80211_ANO_NETTYPE_WILD: c_int = 15;
// control extension - for IEEE80211_FTYPE_CTL | IEEE80211_STYPE_CTL_EXT
pub const IEEE80211_CTL_EXT_POLL: c_uint = 0x2000;
pub const IEEE80211_CTL_EXT_SPR: c_uint = 0x3000;
pub const IEEE80211_CTL_EXT_GRANT: c_uint = 0x4000;
pub const IEEE80211_CTL_EXT_DMG_CTS: c_uint = 0x5000;
pub const IEEE80211_CTL_EXT_DMG_DTS: c_uint = 0x6000;
pub const IEEE80211_CTL_EXT_SSW: c_uint = 0x8000;
pub const IEEE80211_CTL_EXT_SSW_FBACK: c_uint = 0x9000;
pub const IEEE80211_CTL_EXT_SSW_ACK: c_uint = 0xa000;

// PV1 Layout IEEE 802.11-2020 9.8.3.1
pub const IEEE80211_PV1_FCTL_VERS: c_uint = 0x0003;
pub const IEEE80211_PV1_FCTL_FTYPE: c_uint = 0x001c;
pub const IEEE80211_PV1_FCTL_STYPE: c_uint = 0x00e0;
pub const IEEE80211_PV1_FCTL_FROMDS: c_uint = 0x0100;
pub const IEEE80211_PV1_FCTL_MOREFRAGS: c_uint = 0x0200;
pub const IEEE80211_PV1_FCTL_PM: c_uint = 0x0400;
pub const IEEE80211_PV1_FCTL_MOREDATA: c_uint = 0x0800;
pub const IEEE80211_PV1_FCTL_PROTECTED: c_uint = 0x1000;
pub const IEEE80211_PV1_FCTL_END_SP: c_uint = 0x2000;
pub const IEEE80211_PV1_FCTL_RELAYED: c_uint = 0x4000;
pub const IEEE80211_PV1_FCTL_ACK_POLICY: c_uint = 0x8000;
pub const IEEE80211_PV1_FCTL_CTL_EXT: c_uint = 0x0f00;
extern "C" {
    pub fn ieee80211_sn_add(_arg: sn, _arg: 1) -> return;
}

// miscellaneous IEEE 802.11 constants
pub const IEEE80211_MAX_FRAG_THRESHOLD: c_int = 2352;
pub const IEEE80211_MAX_RTS_THRESHOLD: c_int = 2353;
pub const IEEE80211_MAX_AID: c_int = 2007;
pub const IEEE80211_MAX_AID_S1G: c_int = 8191;
pub const IEEE80211_MAX_TIM_LEN: c_int = 251;
pub const IEEE80211_MAX_MESH_PEERINGS: c_int = 63;
// Maximum size for the MA-UNITDATA primitive, 802.11 standard section
pub const IEEE80211_MAX_DATA_LEN: c_int = 2304;
// 802.11ad extends maximum MSDU size for DMG (freq > 40Ghz) networks
// to 7920 bytes, see 8.2.3 General frame format
//
pub const IEEE80211_MAX_DATA_LEN_DMG: c_int = 7920;
// 30 byte 4 addr hdr, 2 byte QoS, 2304 byte MSDU, 12 byte crypt, 4 byte FCS
pub const IEEE80211_MAX_FRAME_LEN: c_int = 2352;
pub const IEEE80211_MAX_SSID_LEN: c_int = 32;
pub const IEEE80211_FIRST_TSPEC_TSID: c_int = 8;
pub const IEEE80211_NUM_TIDS: c_int = 16;
// number of user priorities 802.11 uses
pub const IEEE80211_NUM_UPS: c_int = 8;
// number of ACs
pub const IEEE80211_NUM_ACS: c_int = 4;
pub const IEEE80211_QOS_CTL_LEN: c_int = 2;
// 1d tag mask
pub const IEEE80211_QOS_CTL_TAG1D_MASK: c_uint = 0x0007;
// TID mask
pub const IEEE80211_QOS_CTL_TID_MASK: c_uint = 0x000f;
// EOSP
pub const IEEE80211_QOS_CTL_EOSP: c_uint = 0x0010;
// ACK policy
pub const IEEE80211_QOS_CTL_ACK_POLICY_NORMAL: c_uint = 0x0000;
pub const IEEE80211_QOS_CTL_ACK_POLICY_NOACK: c_uint = 0x0020;
pub const IEEE80211_QOS_CTL_ACK_POLICY_NO_EXPL: c_uint = 0x0040;
pub const IEEE80211_QOS_CTL_ACK_POLICY_BLOCKACK: c_uint = 0x0060;
pub const IEEE80211_QOS_CTL_ACK_POLICY_MASK: c_uint = 0x0060;
// A-MSDU 802.11n
pub const IEEE80211_QOS_CTL_A_MSDU_PRESENT: c_uint = 0x0080;
// Mesh Control 802.11s
pub const IEEE80211_QOS_CTL_MESH_CONTROL_PRESENT: c_uint = 0x0100;
// Mesh Power Save Level
pub const IEEE80211_QOS_CTL_MESH_PS_LEVEL: c_uint = 0x0200;
// Mesh Receiver Service Period Initiated
pub const IEEE80211_QOS_CTL_RSPI: c_uint = 0x0400;
// U-APSD queue for WMM IEs sent by AP

pub const IEEE80211_WMM_IE_AP_QOSINFO_PARAM_SET_CNT_MASK: c_uint = 0x0f;
// U-APSD queues for WMM IEs sent by STA

pub const IEEE80211_WMM_IE_STA_QOSINFO_AC_MASK: c_uint = 0x0f;
// U-APSD max SP length for WMM IEs sent by STA
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_ALL: c_uint = 0x00;
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_2: c_uint = 0x01;
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_4: c_uint = 0x02;
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_6: c_uint = 0x03;
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_MASK: c_uint = 0x03;
pub const IEEE80211_WMM_IE_STA_QOSINFO_SP_SHIFT: c_int = 5;
// trigger type within common_info of trigger frame
pub const IEEE80211_TRIGGER_TYPE_MASK: c_uint = 0xf;
pub const IEEE80211_TRIGGER_TYPE_BASIC: c_uint = 0x0;
pub const IEEE80211_TRIGGER_TYPE_BFRP: c_uint = 0x1;
pub const IEEE80211_TRIGGER_TYPE_MU_BAR: c_uint = 0x2;
pub const IEEE80211_TRIGGER_TYPE_MU_RTS: c_uint = 0x3;
pub const IEEE80211_TRIGGER_TYPE_BSRP: c_uint = 0x4;
pub const IEEE80211_TRIGGER_TYPE_GCR_MU_BAR: c_uint = 0x5;
pub const IEEE80211_TRIGGER_TYPE_BQRP: c_uint = 0x6;
pub const IEEE80211_TRIGGER_TYPE_NFRP: c_uint = 0x7;
// UL-bandwidth within common_info of trigger frame
pub const IEEE80211_TRIGGER_ULBW_MASK: c_uint = 0xc0000;
pub const IEEE80211_TRIGGER_ULBW_20MHZ: c_uint = 0x0;
pub const IEEE80211_TRIGGER_ULBW_40MHZ: c_uint = 0x1;
pub const IEEE80211_TRIGGER_ULBW_80MHZ: c_uint = 0x2;
pub const IEEE80211_TRIGGER_ULBW_160_80P80MHZ: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_hdr {
    pub frame_control: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub addr4: [u8; ETH_ALEN],
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_hdr_3addr {
    pub frame_control: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_qos_hdr {
    pub frame_control: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub qos_ctrl: __le16,
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_qos_hdr_4addr {
    pub frame_control: __le16,
    pub duration_id: __le16,
    pub addr1: [u8; ETH_ALEN],
    pub addr2: [u8; ETH_ALEN],
    pub addr3: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub addr4: [u8; ETH_ALEN],
    pub qos_ctrl: __le16,
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_trigger {
    pub frame_control: __le16,
    pub duration: __le16,
    pub ra: [u8; ETH_ALEN],
    pub ta: [u8; ETH_ALEN],
    pub common_info: __le64,
    pub variable: [u8; ],
    pub __aligned(2): } __packed,
//
// ieee80211_has_tods - check if IEEE80211_FCTL_TODS is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame has to-DS set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_TODS)) !=,
//
// ieee80211_has_fromds - check if IEEE80211_FCTL_FROMDS is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame has from-DS set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_FROMDS)) !=,
//
// ieee80211_has_a4 - check if IEEE80211_FCTL_TODS and IEEE80211_FCTL_FROMDS are set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not it's a 4-address frame (from-DS and to-DS set)
//
    pub IEEE80211_FCTL_FROMDS): __le16 tmp = cpu_to_le16(IEEE80211_FCTL_TODS |,
    pub tmp: return (fc & tmp) ==,
//
// ieee80211_has_morefrags - check if IEEE80211_FCTL_MOREFRAGS is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame has more fragments (more frags bit set)
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_MOREFRAGS)) !=,
//
// ieee80211_has_retry - check if IEEE80211_FCTL_RETRY is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the retry flag is set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_RETRY)) !=,
//
// ieee80211_has_pm - check if IEEE80211_FCTL_PM is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the power management flag is set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_PM)) !=,
//
// ieee80211_has_moredata - check if IEEE80211_FCTL_MOREDATA is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the more data flag is set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_MOREDATA)) !=,
//
// ieee80211_has_protected - check if IEEE80211_FCTL_PROTECTED is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the protected flag is set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_PROTECTED)) !=,
//
// ieee80211_has_order - check if IEEE80211_FCTL_ORDER is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the order flag is set
//
    pub 0: return (fc & cpu_to_le16(IEEE80211_FCTL_ORDER)) !=,
//
// ieee80211_is_mgmt - check if type is IEEE80211_FTYPE_MGMT
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame type is management
//
// ieee80211_is_ctl - check if type is IEEE80211_FTYPE_CTL
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame type is control
//
// ieee80211_is_data - check if type is IEEE80211_FTYPE_DATA
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a data frame
//
// ieee80211_is_ext - check if type is IEEE80211_FTYPE_EXT
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame type is extended
//
// ieee80211_is_data_qos - check if type is IEEE80211_FTYPE_DATA and IEEE80211_STYPE_QOS_DATA is set
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a QoS data frame
//
// mask with QOS_DATA rather than IEEE80211_FCTL_STYPE as we just need
// to check the one bit
//
    pub IEEE80211_STYPE_QOS_DATA): cpu_to_le16(IEEE80211_FTYPE_DATA |,
//
// ieee80211_is_data_present - check if type is IEEE80211_FTYPE_DATA and has data
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a QoS data frame that has data
// (i.e. is not null data)
//
// mask with 0x40 and test that that bit is clear to only return true
// for the data-containing substypes.
//
// ieee80211_is_assoc_req - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_ASSOC_REQ
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an association request
//
    pub IEEE80211_STYPE_ASSOC_REQ): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_assoc_resp - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_ASSOC_RESP
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an association response
//
    pub IEEE80211_STYPE_ASSOC_RESP): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_reassoc_req - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_REASSOC_REQ
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a reassociation request
//
    pub IEEE80211_STYPE_REASSOC_REQ): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_reassoc_resp - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_REASSOC_RESP
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a reassociation response
//
    pub IEEE80211_STYPE_REASSOC_RESP): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_assoc - check if (Re)association request/response frame
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an (re)association request or response
//
    pub ieee80211_is_reassoc_resp(fc): ieee80211_is_assoc_resp(fc) ||,
//
// ieee80211_is_probe_req - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_PROBE_REQ
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a probe request
//
    pub IEEE80211_STYPE_PROBE_REQ): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_probe_resp - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_PROBE_RESP
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a probe response
//
    pub IEEE80211_STYPE_PROBE_RESP): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_beacon - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_BEACON
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a (regular, not S1G) beacon
//
    pub IEEE80211_STYPE_BEACON): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_atim - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_ATIM
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an ATIM frame
//
    pub IEEE80211_STYPE_ATIM): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_disassoc - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_DISASSOC
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a disassociation frame
//
    pub IEEE80211_STYPE_DISASSOC): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_auth - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_AUTH
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an authentication frame
//
    pub IEEE80211_STYPE_AUTH): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_deauth - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_DEAUTH
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a deauthentication frame
//
    pub IEEE80211_STYPE_DEAUTH): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_action - check if IEEE80211_FTYPE_MGMT && IEEE80211_STYPE_ACTION
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an action frame
//
    pub IEEE80211_STYPE_ACTION): cpu_to_le16(IEEE80211_FTYPE_MGMT |,
//
// ieee80211_is_back_req - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_BACK_REQ
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a block-ACK request frame
//
    pub IEEE80211_STYPE_BACK_REQ): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_back - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_BACK
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a block-ACK frame
//
    pub IEEE80211_STYPE_BACK): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_pspoll - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_PSPOLL
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a PS-poll frame
//
    pub IEEE80211_STYPE_PSPOLL): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_rts - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_RTS
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an RTS frame
//
    pub IEEE80211_STYPE_RTS): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_cts - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_CTS
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a CTS frame
//
    pub IEEE80211_STYPE_CTS): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_ack - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_ACK
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is an ACK frame
//
    pub IEEE80211_STYPE_ACK): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_cfend - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_CFEND
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a CF-end frame
//
    pub IEEE80211_STYPE_CFEND): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_cfendack - check if IEEE80211_FTYPE_CTL && IEEE80211_STYPE_CFENDACK
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a CF-end-ack frame
//
    pub IEEE80211_STYPE_CFENDACK): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_nullfunc - check if frame is a regular (non-QoS) nullfunc frame
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a nullfunc frame
//
    pub IEEE80211_STYPE_NULLFUNC): cpu_to_le16(IEEE80211_FTYPE_DATA |,
//
// ieee80211_is_qos_nullfunc - check if frame is a QoS nullfunc frame
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a QoS nullfunc frame
//
    pub IEEE80211_STYPE_QOS_NULLFUNC): cpu_to_le16(IEEE80211_FTYPE_DATA |,
//
// ieee80211_is_trigger - check if frame is trigger frame
// @fc: frame control field in little-endian byteorder
// Return: whether or not the frame is a trigger frame
//
    pub IEEE80211_STYPE_TRIGGER): cpu_to_le16(IEEE80211_FTYPE_CTL |,
//
// ieee80211_is_any_nullfunc - check if frame is regular or QoS nullfunc frame
// @fc: frame control bytes in little-endian byteorder
// Return: whether or not the frame is a nullfunc or QoS nullfunc frame
//
    pub ieee80211_is_qos_nullfunc(fc)): return (ieee80211_is_nullfunc(fc) ||,
//
// ieee80211_is_first_frag - check if IEEE80211_SCTL_FRAG is not set
// @seq_ctrl: frame sequence control bytes in little-endian byteorder
// Return: whether or not the frame is the first fragment (also true if
// it's not fragmented at all)
//
    pub 0: return (seq_ctrl & cpu_to_le16(IEEE80211_SCTL_FRAG)) ==,
//
// ieee80211_is_frag - check if a frame is a fragment
// @hdr: 802.11 header of the frame
// Return: whether or not the frame is a fragment
//
    pub cpu_to_le16(IEEE80211_SCTL_FRAG): hdr->seq_ctrl &,
    pub IEEE80211_SCTL_SEQ): return le16_get_bits(hdr->seq_ctrl,,
//
// struct ieee80211_quiet_ie - Quiet element
// @count: Quiet Count
// @period: Quiet Period
// @duration: Quiet Duration
// @offset: Quiet Offset
//
// This structure represents the payload of the "Quiet element" as
// described in IEEE Std 802.11-2020 section 9.4.2.22.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_quiet_ie {
    pub count: u8,
    pub period: u8,
    pub duration: __le16,
    pub offset: __le16,
    pub __packed: },
//
// struct ieee80211_msrment_ie - Measurement element
// @token: Measurement Token
// @mode: Measurement Report Mode
// @type: Measurement Type
// @request: Measurement Request or Measurement Report
//
// This structure represents the payload of both the "Measurement
// Request element" and the "Measurement Report element" as described
// in IEEE Std 802.11-2020 sections 9.4.2.20 and 9.4.2.21.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_msrment_ie {
    pub token: u8,
    pub mode: u8,
    pub type: u8,
    pub request: [u8; ],
    pub __packed: },
//
// struct ieee80211_channel_sw_ie - Channel Switch Announcement element
// @mode: Channel Switch Mode
// @new_ch_num: New Channel Number
// @count: Channel Switch Count
//
// This structure represents the payload of the "Channel Switch
// Announcement element" as described in IEEE Std 802.11-2020 section
// 9.4.2.18.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_channel_sw_ie {
    pub mode: u8,
    pub new_ch_num: u8,
    pub count: u8,
    pub __packed: },
//
// struct ieee80211_ext_chansw_ie - Extended Channel Switch Announcement element
// @mode: Channel Switch Mode
// @new_operating_class: New Operating Class
// @new_ch_num: New Channel Number
// @count: Channel Switch Count
//
// This structure represents the "Extended Channel Switch Announcement
// element" as described in IEEE Std 802.11-2020 section 9.4.2.52.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ext_chansw_ie {
    pub mode: u8,
    pub new_operating_class: u8,
    pub new_ch_num: u8,
    pub count: u8,
    pub __packed: },
//
// struct ieee80211_sec_chan_offs_ie - secondary channel offset IE
// @sec_chan_offs: secondary channel offset, uses IEEE80211_HT_PARAM_CHA_SEC_
// values here
// This structure represents the "Secondary Channel Offset element"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_sec_chan_offs_ie {
    pub sec_chan_offs: u8,
    pub __packed: },
//
// struct ieee80211_wide_bw_chansw_ie - wide bandwidth channel switch IE
// @new_channel_width: New Channel Width
// @new_center_freq_seg0: New Channel Center Frequency Segment 0
// @new_center_freq_seg1: New Channel Center Frequency Segment 1
//
// This structure represents the payload of the "Wide Bandwidth
// Channel Switch element" as described in IEEE Std 802.11-2020
// section 9.4.2.160.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_wide_bw_chansw_ie {
    pub new_channel_width: u8,
    pub new_center_freq_seg1: u8 new_center_freq_seg0,,
    pub __packed: },
//
// struct ieee80211_tim_ie - Traffic Indication Map information element
// @dtim_count: DTIM Count
// @dtim_period: DTIM Period
// @bitmap_ctrl: Bitmap Control
// @required_octet: "Syntatic sugar" to force the struct size to the
// minimum valid size when carried in a non-S1G PPDU
// @virtual_map: Partial Virtual Bitmap
//
// This structure represents the payload of the "TIM element" as
// described in IEEE Std 802.11-2020 section 9.4.2.5. Note that this
// definition is only applicable when the element is carried in a
// non-S1G PPDU. When the TIM is carried in an S1G PPDU, the Bitmap
// Control and Partial Virtual Bitmap may not be present.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tim_ie {
    pub dtim_count: u8,
    pub dtim_period: u8,
    pub bitmap_ctrl: u8,
    pub required_octet: u8,
    pub virtual_map): DECLARE_FLEX_ARRAY(u8,,
}

pub const WLAN_SA_QUERY_TR_ID_LEN: c_int = 2;
pub const WLAN_MEMBERSHIP_LEN: c_int = 8;
pub const WLAN_USER_POSITION_LEN: c_int = 16;
//
// struct ieee80211_tpc_report_ie - TPC Report element
// @tx_power: Transmit Power
// @link_margin: Link Margin
//
// This structure represents the payload of the "TPC Report element" as
// described in IEEE Std 802.11-2020 section 9.4.2.16.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tpc_report_ie {
    pub tx_power: u8,
    pub link_margin: u8,
    pub __packed: },

pub const IEEE80211_ADDBA_EXT_FRAG_LEVEL_SHIFT: c_int = 1;

pub const IEEE80211_ADDBA_EXT_BUF_SIZE_SHIFT: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_addba_ext_ie {
    pub data: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ext {
    pub frame_control: __le16,
    pub duration: __le16,
    pub sa: [u8; ETH_ALEN],
    pub timestamp: __le32,
    pub change_seq: u8,
    pub variable: [u8; ],
    pub s1g_beacon: } __packed,
    pub u: },
    pub __aligned(2): } __packed,
//
// struct ieee80211_bss_load_elem - BSS Load elemen
//
// Defined in section 9.4.2.26 in IEEE 802.11-REVme D4.1
//
// @sta_count: total number of STAs currently associated with the AP.
// @channel_util: Percentage of time that the access point sensed the channel
// was busy. This value is in range [0, 255], the highest value means
// 100% busy.
// @avail_admission_capa: remaining amount of medium time used for admission
// control.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bss_load_elem {
    pub sta_count: __le16,
    pub channel_util: u8,
    pub avail_admission_capa: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mgmt {
    pub frame_control: __le16,
    pub duration: __le16,
    pub da: [u8; ETH_ALEN],
    pub sa: [u8; ETH_ALEN],
    pub bssid: [u8; ETH_ALEN],
    pub seq_ctrl: __le16,
    pub auth_alg: __le16,
    pub auth_transaction: __le16,
    pub status_code: __le16,
// possibly followed by Challenge text
    pub variable: [u8; ],
    pub auth: } __packed,
    pub reason_code: __le16,
    pub deauth: } __packed,
    pub capab_info: __le16,
    pub listen_interval: __le16,
// followed by SSID and Supported rates
    pub variable: [u8; ],
    pub assoc_req: } __packed,
    pub capab_info: __le16,
    pub status_code: __le16,
    pub aid: __le16,
// followed by Supported rates
    pub variable: [u8; ],
    pub reassoc_resp: } __packed assoc_resp,,
    pub capab_info: __le16,
    pub status_code: __le16,
    pub variable: [u8; ],
    pub s1g_reassoc_resp: } __packed s1g_assoc_resp,,
    pub capab_info: __le16,
    pub listen_interval: __le16,
    pub current_ap: [u8; ETH_ALEN],
// followed by SSID and Supported rates
    pub variable: [u8; ],
    pub reassoc_req: } __packed,
    pub reason_code: __le16,
    pub disassoc: } __packed,
    pub timestamp: __le64,
    pub beacon_int: __le16,
    pub capab_info: __le16,
// followed by some of SSID, Supported rates,
// FH Params, DS Params, CF Params, IBSS Params, TIM
    pub variable: [u8; ],
    pub beacon: } __packed,
// only variable items: SSID, Supported rates
    pub variable): DECLARE_FLEX_ARRAY(u8,,
    pub probe_req: } __packed,
    pub timestamp: __le64,
    pub beacon_int: __le16,
    pub capab_info: __le16,
// followed by some of SSID, Supported rates,
// FH Params, DS Params, CF Params, IBSS Params
    pub variable: [u8; ],
    pub probe_resp: } __packed,
    pub category: u8,
    pub action_code: u8,
    pub dialog_token: u8,
    pub status_code: u8,
    pub variable: [u8; ],
    pub wme_action: } __packed,
    pub no_fixed_fields: [u8; 0],
    pub variable: [u8; ],
    pub chan_switch: } __packed,
    pub data: ieee80211_ext_chansw_ie,
    pub variable: [u8; ],
    pub ext_chan_switch: } __packed,
    pub dialog_token: u8,
    pub element_id: u8,
    pub length: u8,
    pub msr_elem: ieee80211_msrment_ie,
    pub measurement: } __packed,
    pub dialog_token: u8,
    pub capab: __le16,
    pub timeout: __le16,
    pub start_seq_num: __le16,
// followed by BA Extension
    pub variable: [u8; ],
    pub addba_req: } __packed,
    pub dialog_token: u8,
    pub status: __le16,
    pub capab: __le16,
    pub timeout: __le16,
// followed by BA Extension
    pub variable: [u8; ],
    pub addba_resp: } __packed,
    pub params: __le16,
    pub reason_code: __le16,
    pub delba: } __packed,
    pub no_fixed_fields: [u8; 0],
    pub variable: [u8; ],
    pub self_prot: } __packed,
    pub no_fixed_fields: [u8; 0],
    pub variable: [u8; ],
    pub mesh_action: } __packed,
    pub trans_id: [u8; WLAN_SA_QUERY_TR_ID_LEN],
    pub sa_query: } __packed,
    pub smps_control: u8,
    pub ht_smps: } __packed,
    pub chanwidth: u8,
    pub ht_notify_cw: } __packed,
    pub dialog_token: u8,
    pub capability: __le16,
    pub variable: [u8; ],
    pub tdls_discover_resp: } __packed,
    pub operating_mode: u8,
    pub vht_opmode_notif: } __packed,
    pub membership: [u8; WLAN_MEMBERSHIP_LEN],
    pub position: [u8; WLAN_USER_POSITION_LEN],
    pub vht_group_notif: } __packed,
    pub dialog_token: u8,
    pub tpc_elem_id: u8,
    pub tpc_elem_length: u8,
    pub tpc: ieee80211_tpc_report_ie,
    pub tpc_report: } __packed,
    pub dialog_token: u8,
    pub follow_up: u8,
    pub tod: [u8; 6],
    pub toa: [u8; 6],
    pub tod_error: __le16,
    pub toa_error: __le16,
    pub variable: [u8; ],
    pub ftm: } __packed,
    pub no_fixed_fields: [u8; 0],
    pub variable: [u8; ],
    pub s1g: } __packed,
    pub dialog_token: u8,
    pub follow_up: u8,
    pub tod: u32,
    pub toa: u32,
    pub max_tod_error: u8,
    pub max_toa_error: u8,
    pub wnm_timing_msr: } __packed,
    pub dialog_token: u8,
    pub variable: [u8; ],
    pub ttlm_req: } __packed,
    pub dialog_token: u8,
    pub status_code: __le16,
    pub variable: [u8; ],
    pub ttlm_res: } __packed,
    pub no_fixed_fields: [u8; 0],
// no variable fields either
    pub ttlm_tear_down: } __packed,
    pub dialog_token: u8,
    pub variable: [u8; ],
    pub ml_reconf_req: } __packed,
    pub dialog_token: u8,
    pub count: u8,
    pub variable: [u8; ],
    pub ml_reconf_resp: } __packed,
    pub no_fixed_fields: [u8; 0],
    pub variable: [u8; ],
    pub epcs: } __packed,
    pub dialog_token: u8,
    pub control: u8,
    pub variable: [u8; ],
    pub eml_omn: } __packed,
    pub dialog_token: u8,
    pub type: u8,
    pub variable: [u8; ],
    pub uhr_link_reconf_req: } __packed,
    pub dialog_token: u8,
    pub type: u8,
    pub count: u8,
    pub variable: [u8; ],
    pub uhr_link_reconf_resp: } __packed,
    pub dialog_token: u8,
    pub type: u8,
    pub variable: [u8; ],
    pub uhr_link_reconf_notif: } __packed,
}

// Supported rates membership selectors
pub const BSS_MEMBERSHIP_SELECTOR_HT_PHY: c_int = 127;
pub const BSS_MEMBERSHIP_SELECTOR_VHT_PHY: c_int = 126;
pub const BSS_MEMBERSHIP_SELECTOR_GLK: c_int = 125;
pub const BSS_MEMBERSHIP_SELECTOR_EPD: c_int = 124;
pub const BSS_MEMBERSHIP_SELECTOR_SAE_H2E: c_int = 123;
pub const BSS_MEMBERSHIP_SELECTOR_HE_PHY: c_int = 122;
pub const BSS_MEMBERSHIP_SELECTOR_EHT_PHY: c_int = 121;
pub const BSS_MEMBERSHIP_SELECTOR_UHR_PHY: c_int = 120;

// Link Reconfiguration Status Duple field
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ml_reconf_status {
    pub info: u8,
    pub status: __le16,
    pub __packed: },
pub const IEEE80211_ML_RECONF_LINK_ID_MASK: c_uint = 0xf;
// Management MIC information element (IEEE 802.11w) for CMAC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mmie {
    pub element_id: u8,
    pub length: u8,
    pub key_id: __le16,
    pub sequence_number: [u8; 6],
    pub mic: [u8; 8],
    pub __packed: },
// Management MIC information element (IEEE 802.11w) for GMAC and CMAC-256
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mmie_16 {
    pub element_id: u8,
    pub length: u8,
    pub key_id: __le16,
    pub sequence_number: [u8; 6],
    pub mic: [u8; 16],
    pub __packed: },
// Management MIC information element (IEEE 802.11w) for all variants
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_mmie_var {
    pub element_id: u8,
    pub length: u8,
    pub key_id: __le16,
    pub sequence_number: [u8; 6],
    pub /: *mut *mut u8 mic[]; / 8 or 16 bytes,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_vendor_ie {
    pub element_id: u8,
    pub len: u8,
    pub oui: [u8; 3],
    pub oui_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_wmm_ac_param {
    pub /: *mut *mut u8 aci_aifsn; / AIFSN, ACM, ACI,
    pub /: *mut *mut u8 cw; / ECWmin, ECWmax (CW = 2^ECW - 1),
    pub txop_limit: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_wmm_param_ie {
    pub /: *mut *mut u8 element_id; / Element ID: 221 (0xdd);,
    pub /: *mut *mut u8 len; / Length: 24,
// required fields for WMM version 1
    pub /: *mut *mut u8 oui[3]; / 00:50:f2,
    pub /: *mut *mut u8 oui_type; / 2,
    pub /: *mut *mut u8 oui_subtype; / 1,
    pub /: *mut *mut u8 version; / 1 for WMM version 1.0,
    pub /: *mut *mut u8 qos_info; / AP/STA specific QoS info,
    pub /: *mut *mut u8 reserved; / 0,
// AC_BE, AC_BK, AC_VI, AC_VO
    pub ac: [ieee80211_wmm_ac_param; 4],
    pub __packed: },
// Control frames
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_rts {
    pub frame_control: __le16,
    pub duration: __le16,
    pub ra: [u8; ETH_ALEN],
    pub ta: [u8; ETH_ALEN],
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_cts {
    pub frame_control: __le16,
    pub duration: __le16,
    pub ra: [u8; ETH_ALEN],
    pub __aligned(2): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_pspoll {
    pub frame_control: __le16,
    pub aid: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub ta: [u8; ETH_ALEN],
    pub __aligned(2): } __packed,
// TDLS
// Channel switch timing
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_ch_switch_timing {
    pub switch_time: __le16,
    pub switch_timeout: __le16,
    pub __packed: },
// Link-id information element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tdls_lnkie {
    pub /: *mut *mut u8 ie_type; / Link Identifier IE,
    pub ie_len: u8,
    pub bssid: [u8; ETH_ALEN],
    pub init_sta: [u8; ETH_ALEN],
    pub resp_sta: [u8; ETH_ALEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tdls_data {
    pub da: [u8; ETH_ALEN],
    pub sa: [u8; ETH_ALEN],
    pub ether_type: __be16,
    pub payload_type: u8,
    pub category: u8,
    pub action_code: u8,
    pub dialog_token: u8,
    pub capability: __le16,
    pub variable: [u8; ],
    pub setup_req: } __packed,
    pub status_code: __le16,
    pub dialog_token: u8,
    pub capability: __le16,
    pub variable: [u8; ],
    pub setup_resp: } __packed,
    pub status_code: __le16,
    pub dialog_token: u8,
    pub variable: [u8; ],
    pub setup_cfm: } __packed,
    pub reason_code: __le16,
    pub variable: [u8; ],
    pub teardown: } __packed,
    pub dialog_token: u8,
    pub variable: [u8; ],
    pub discover_req: } __packed,
    pub target_channel: u8,
    pub oper_class: u8,
    pub variable: [u8; ],
    pub chan_switch_req: } __packed,
    pub status_code: __le16,
    pub variable: [u8; ],
    pub chan_switch_resp: } __packed,
    pub u: },
    pub __packed: },
// Authentication algorithms
pub const WLAN_AUTH_OPEN: c_int = 0;
pub const WLAN_AUTH_SHARED_KEY: c_int = 1;
pub const WLAN_AUTH_FT: c_int = 2;
pub const WLAN_AUTH_SAE: c_int = 3;
pub const WLAN_AUTH_FILS_SK: c_int = 4;
pub const WLAN_AUTH_FILS_SK_PFS: c_int = 5;
pub const WLAN_AUTH_FILS_PK: c_int = 6;
pub const WLAN_AUTH_IEEE8021X: c_int = 8;
pub const WLAN_AUTH_EPPKE: c_int = 9;
pub const WLAN_AUTH_LEAP: c_int = 128;
pub const WLAN_AUTH_CHALLENGE_LEN: c_int = 128;

//
// A mesh STA sets the ESS and IBSS capability bits to zero.
// however, this holds true for p2p probe responses (in the p2p_find
// phase) as well.
//

// 802.11h

// DMG (60gHz) 802.11ad
// type - bits 0..1

// measurement

pub const IEEE80211_SPCT_MSR_RPRT_TYPE_BASIC: c_int = 0;
pub const IEEE80211_SPCT_MSR_RPRT_TYPE_CCA: c_int = 1;
pub const IEEE80211_SPCT_MSR_RPRT_TYPE_RPI: c_int = 2;
pub const IEEE80211_SPCT_MSR_RPRT_TYPE_LCI: c_int = 8;
pub const IEEE80211_SPCT_MSR_RPRT_TYPE_CIVIC: c_int = 11;
// 802.11g ERP information element

// WLAN_ERP_BARKER_PREAMBLE values
}

// Band ID, 802.11ad #8.4.1.45
// Status codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_statuscode {
    WLAN_STATUS_SUCCESS = 0,
    WLAN_STATUS_UNSPECIFIED_FAILURE = 1,
    WLAN_STATUS_CAPS_UNSUPPORTED = 10,
    WLAN_STATUS_REASSOC_NO_ASSOC = 11,
    WLAN_STATUS_ASSOC_DENIED_UNSPEC = 12,
    WLAN_STATUS_NOT_SUPPORTED_AUTH_ALG = 13,
    WLAN_STATUS_UNKNOWN_AUTH_TRANSACTION = 14,
    WLAN_STATUS_CHALLENGE_FAIL = 15,
    WLAN_STATUS_AUTH_TIMEOUT = 16,
    WLAN_STATUS_AP_UNABLE_TO_HANDLE_NEW_STA = 17,
    WLAN_STATUS_ASSOC_DENIED_RATES = 18,
// 802.11b
    WLAN_STATUS_ASSOC_DENIED_NOSHORTPREAMBLE = 19,
    WLAN_STATUS_ASSOC_DENIED_NOPBCC = 20,
    WLAN_STATUS_ASSOC_DENIED_NOAGILITY = 21,
// 802.11h
    WLAN_STATUS_ASSOC_DENIED_NOSPECTRUM = 22,
    WLAN_STATUS_ASSOC_REJECTED_BAD_POWER = 23,
    WLAN_STATUS_ASSOC_REJECTED_BAD_SUPP_CHAN = 24,
// 802.11g
    WLAN_STATUS_ASSOC_DENIED_NOSHORTTIME = 25,
    WLAN_STATUS_ASSOC_DENIED_NODSSSOFDM = 26,
// 802.11w
    WLAN_STATUS_ASSOC_REJECTED_TEMPORARILY = 30,
    WLAN_STATUS_ROBUST_MGMT_FRAME_POLICY_VIOLATION = 31,
// 802.11i
    WLAN_STATUS_INVALID_IE = 40,
    WLAN_STATUS_INVALID_GROUP_CIPHER = 41,
    WLAN_STATUS_INVALID_PAIRWISE_CIPHER = 42,
    WLAN_STATUS_INVALID_AKMP = 43,
    WLAN_STATUS_UNSUPP_RSN_VERSION = 44,
    WLAN_STATUS_INVALID_RSN_IE_CAP = 45,
    WLAN_STATUS_CIPHER_SUITE_REJECTED = 46,
// 802.11e
    WLAN_STATUS_UNSPECIFIED_QOS = 32,
    WLAN_STATUS_ASSOC_DENIED_NOBANDWIDTH = 33,
    WLAN_STATUS_ASSOC_DENIED_LOWACK = 34,
    WLAN_STATUS_ASSOC_DENIED_UNSUPP_QOS = 35,
    WLAN_STATUS_REQUEST_DECLINED = 37,
    WLAN_STATUS_INVALID_QOS_PARAM = 38,
    WLAN_STATUS_CHANGE_TSPEC = 39,
    WLAN_STATUS_WAIT_TS_DELAY = 47,
    WLAN_STATUS_NO_DIRECT_LINK = 48,
    WLAN_STATUS_STA_NOT_PRESENT = 49,
    WLAN_STATUS_STA_NOT_QSTA = 50,
// 802.11s
    WLAN_STATUS_ANTI_CLOG_REQUIRED = 76,
    WLAN_STATUS_FCG_NOT_SUPP = 78,
    WLAN_STATUS_STA_NO_TBTT = 78,
// 802.11ad
    WLAN_STATUS_REJECTED_WITH_SUGGESTED_CHANGES = 39,
    WLAN_STATUS_REJECTED_FOR_DELAY_PERIOD = 47,
    WLAN_STATUS_REJECT_WITH_SCHEDULE = 83,
    WLAN_STATUS_PENDING_ADMITTING_FST_SESSION = 86,
    WLAN_STATUS_PERFORMING_FST_NOW = 87,
    WLAN_STATUS_PENDING_GAP_IN_BA_WINDOW = 88,
    WLAN_STATUS_REJECT_U_PID_SETTING = 89,
    WLAN_STATUS_REJECT_DSE_BAND = 96,
    WLAN_STATUS_DENIED_WITH_SUGGESTED_BAND_AND_CHANNEL = 99,
    WLAN_STATUS_DENIED_DUE_TO_SPECTRUM_MANAGEMENT = 103,
// 802.11ah
    WLAN_STATUS_REJECTED_NDP_BLOCK_ACK_SUGGESTED = 109,
// 802.11ai
    WLAN_STATUS_FILS_AUTHENTICATION_FAILURE = 112,
    WLAN_STATUS_UNKNOWN_AUTHENTICATION_SERVER = 113,
    WLAN_STATUS_SAE_HASH_TO_ELEMENT = 126,
    WLAN_STATUS_SAE_PK = 127,
    WLAN_STATUS_DENIED_TID_TO_LINK_MAPPING = 133,
    WLAN_STATUS_PREF_TID_TO_LINK_MAPPING_SUGGESTED = 134,
    WLAN_STATUS_8021X_AUTH_SUCCESS = 153,
}

// Reason codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_reasoncode {
    WLAN_REASON_UNSPECIFIED = 1,
    WLAN_REASON_PREV_AUTH_NOT_VALID = 2,
    WLAN_REASON_DEAUTH_LEAVING = 3,
    WLAN_REASON_DISASSOC_DUE_TO_INACTIVITY = 4,
    WLAN_REASON_DISASSOC_AP_BUSY = 5,
    WLAN_REASON_CLASS2_FRAME_FROM_NONAUTH_STA = 6,
    WLAN_REASON_CLASS3_FRAME_FROM_NONASSOC_STA = 7,
    WLAN_REASON_DISASSOC_STA_HAS_LEFT = 8,
    WLAN_REASON_STA_REQ_ASSOC_WITHOUT_AUTH = 9,
// 802.11h
    WLAN_REASON_DISASSOC_BAD_POWER = 10,
    WLAN_REASON_DISASSOC_BAD_SUPP_CHAN = 11,
// 802.11i
    WLAN_REASON_INVALID_IE = 13,
    WLAN_REASON_MIC_FAILURE = 14,
    WLAN_REASON_4WAY_HANDSHAKE_TIMEOUT = 15,
    WLAN_REASON_GROUP_KEY_HANDSHAKE_TIMEOUT = 16,
    WLAN_REASON_IE_DIFFERENT = 17,
    WLAN_REASON_INVALID_GROUP_CIPHER = 18,
    WLAN_REASON_INVALID_PAIRWISE_CIPHER = 19,
    WLAN_REASON_INVALID_AKMP = 20,
    WLAN_REASON_UNSUPP_RSN_VERSION = 21,
    WLAN_REASON_INVALID_RSN_IE_CAP = 22,
    WLAN_REASON_IEEE8021X_FAILED = 23,
    WLAN_REASON_CIPHER_SUITE_REJECTED = 24,
// TDLS (802.11z)
    WLAN_REASON_TDLS_TEARDOWN_UNREACHABLE = 25,
    WLAN_REASON_TDLS_TEARDOWN_UNSPECIFIED = 26,
// 802.11e
    WLAN_REASON_DISASSOC_UNSPECIFIED_QOS = 32,
    WLAN_REASON_DISASSOC_QAP_NO_BANDWIDTH = 33,
    WLAN_REASON_DISASSOC_LOW_ACK = 34,
    WLAN_REASON_DISASSOC_QAP_EXCEED_TXOP = 35,
    WLAN_REASON_QSTA_LEAVE_QBSS = 36,
    WLAN_REASON_QSTA_NOT_USE = 37,
    WLAN_REASON_QSTA_REQUIRE_SETUP = 38,
    WLAN_REASON_QSTA_TIMEOUT = 39,
    WLAN_REASON_QSTA_CIPHER_NOT_SUPP = 45,
// 802.11s
    WLAN_REASON_MESH_PEER_CANCELED = 52,
    WLAN_REASON_MESH_MAX_PEERS = 53,
    WLAN_REASON_MESH_CONFIG = 54,
    WLAN_REASON_MESH_CLOSE = 55,
    WLAN_REASON_MESH_MAX_RETRIES = 56,
    WLAN_REASON_MESH_CONFIRM_TIMEOUT = 57,
    WLAN_REASON_MESH_INVALID_GTK = 58,
    WLAN_REASON_MESH_INCONSISTENT_PARAM = 59,
    WLAN_REASON_MESH_INVALID_SECURITY = 60,
    WLAN_REASON_MESH_PATH_ERROR = 61,
    WLAN_REASON_MESH_PATH_NOFORWARD = 62,
    WLAN_REASON_MESH_PATH_DEST_UNREACHABLE = 63,
    WLAN_REASON_MAC_EXISTS_IN_MBSS = 64,
    WLAN_REASON_MESH_CHAN_REGULATORY = 65,
    WLAN_REASON_MESH_CHAN = 66,
}

// Information Element IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_eid {
    WLAN_EID_SSID = 0,
    WLAN_EID_SUPP_RATES = 1,
    WLAN_EID_FH_PARAMS = 2, /* reserved now */
    WLAN_EID_DS_PARAMS = 3,
    WLAN_EID_CF_PARAMS = 4,
    WLAN_EID_TIM = 5,
    WLAN_EID_IBSS_PARAMS = 6,
    WLAN_EID_COUNTRY = 7,
// 8, 9 reserved
    WLAN_EID_REQUEST = 10,
    WLAN_EID_QBSS_LOAD = 11,
    WLAN_EID_EDCA_PARAM_SET = 12,
    WLAN_EID_TSPEC = 13,
    WLAN_EID_TCLAS = 14,
    WLAN_EID_SCHEDULE = 15,
    WLAN_EID_CHALLENGE = 16,
// 17-31 reserved for challenge text extension
    WLAN_EID_PWR_CONSTRAINT = 32,
    WLAN_EID_PWR_CAPABILITY = 33,
    WLAN_EID_TPC_REQUEST = 34,
    WLAN_EID_TPC_REPORT = 35,
    WLAN_EID_SUPPORTED_CHANNELS = 36,
    WLAN_EID_CHANNEL_SWITCH = 37,
    WLAN_EID_MEASURE_REQUEST = 38,
    WLAN_EID_MEASURE_REPORT = 39,
    WLAN_EID_QUIET = 40,
    WLAN_EID_IBSS_DFS = 41,
    WLAN_EID_ERP_INFO = 42,
    WLAN_EID_TS_DELAY = 43,
    WLAN_EID_TCLAS_PROCESSING = 44,
    WLAN_EID_HT_CAPABILITY = 45,
    WLAN_EID_QOS_CAPA = 46,
// 47 reserved for Broadcom
    WLAN_EID_RSN = 48,
    WLAN_EID_802_15_COEX = 49,
    WLAN_EID_EXT_SUPP_RATES = 50,
    WLAN_EID_AP_CHAN_REPORT = 51,
    WLAN_EID_NEIGHBOR_REPORT = 52,
    WLAN_EID_RCPI = 53,
    WLAN_EID_MOBILITY_DOMAIN = 54,
    WLAN_EID_FAST_BSS_TRANSITION = 55,
    WLAN_EID_TIMEOUT_INTERVAL = 56,
    WLAN_EID_RIC_DATA = 57,
    WLAN_EID_DSE_REGISTERED_LOCATION = 58,
    WLAN_EID_SUPPORTED_REGULATORY_CLASSES = 59,
    WLAN_EID_EXT_CHANSWITCH_ANN = 60,
    WLAN_EID_HT_OPERATION = 61,
    WLAN_EID_SECONDARY_CHANNEL_OFFSET = 62,
    WLAN_EID_BSS_AVG_ACCESS_DELAY = 63,
    WLAN_EID_ANTENNA_INFO = 64,
    WLAN_EID_RSNI = 65,
    WLAN_EID_MEASUREMENT_PILOT_TX_INFO = 66,
    WLAN_EID_BSS_AVAILABLE_CAPACITY = 67,
    WLAN_EID_BSS_AC_ACCESS_DELAY = 68,
    WLAN_EID_TIME_ADVERTISEMENT = 69,
    WLAN_EID_RRM_ENABLED_CAPABILITIES = 70,
    WLAN_EID_MULTIPLE_BSSID = 71,
    WLAN_EID_BSS_COEX_2040 = 72,
    WLAN_EID_BSS_INTOLERANT_CHL_REPORT = 73,
    WLAN_EID_OVERLAP_BSS_SCAN_PARAM = 74,
    WLAN_EID_RIC_DESCRIPTOR = 75,
    WLAN_EID_MMIE = 76,
    WLAN_EID_ASSOC_COMEBACK_TIME = 77,
    WLAN_EID_EVENT_REQUEST = 78,
    WLAN_EID_EVENT_REPORT = 79,
    WLAN_EID_DIAGNOSTIC_REQUEST = 80,
    WLAN_EID_DIAGNOSTIC_REPORT = 81,
    WLAN_EID_LOCATION_PARAMS = 82,
    WLAN_EID_NON_TX_BSSID_CAP =  83,
    WLAN_EID_SSID_LIST = 84,
    WLAN_EID_MULTI_BSSID_IDX = 85,
    WLAN_EID_FMS_DESCRIPTOR = 86,
    WLAN_EID_FMS_REQUEST = 87,
    WLAN_EID_FMS_RESPONSE = 88,
    WLAN_EID_QOS_TRAFFIC_CAPA = 89,
    WLAN_EID_BSS_MAX_IDLE_PERIOD = 90,
    WLAN_EID_TSF_REQUEST = 91,
    WLAN_EID_TSF_RESPOSNE = 92,
    WLAN_EID_WNM_SLEEP_MODE = 93,
    WLAN_EID_TIM_BCAST_REQ = 94,
    WLAN_EID_TIM_BCAST_RESP = 95,
    WLAN_EID_COLL_IF_REPORT = 96,
    WLAN_EID_CHANNEL_USAGE = 97,
    WLAN_EID_TIME_ZONE = 98,
    WLAN_EID_DMS_REQUEST = 99,
    WLAN_EID_DMS_RESPONSE = 100,
    WLAN_EID_LINK_ID = 101,
    WLAN_EID_WAKEUP_SCHEDUL = 102,
// 103 reserved
    WLAN_EID_CHAN_SWITCH_TIMING = 104,
    WLAN_EID_PTI_CONTROL = 105,
    WLAN_EID_PU_BUFFER_STATUS = 106,
    WLAN_EID_INTERWORKING = 107,
    WLAN_EID_ADVERTISEMENT_PROTOCOL = 108,
    WLAN_EID_EXPEDITED_BW_REQ = 109,
    WLAN_EID_QOS_MAP_SET = 110,
    WLAN_EID_ROAMING_CONSORTIUM = 111,
    WLAN_EID_EMERGENCY_ALERT = 112,
    WLAN_EID_MESH_CONFIG = 113,
    WLAN_EID_MESH_ID = 114,
    WLAN_EID_LINK_METRIC_REPORT = 115,
    WLAN_EID_CONGESTION_NOTIFICATION = 116,
    WLAN_EID_PEER_MGMT = 117,
    WLAN_EID_CHAN_SWITCH_PARAM = 118,
    WLAN_EID_MESH_AWAKE_WINDOW = 119,
    WLAN_EID_BEACON_TIMING = 120,
    WLAN_EID_MCCAOP_SETUP_REQ = 121,
    WLAN_EID_MCCAOP_SETUP_RESP = 122,
    WLAN_EID_MCCAOP_ADVERT = 123,
    WLAN_EID_MCCAOP_TEARDOWN = 124,
    WLAN_EID_GANN = 125,
    WLAN_EID_RANN = 126,
    WLAN_EID_EXT_CAPABILITY = 127,
// 128, 129 reserved for Agere
    WLAN_EID_PREQ = 130,
    WLAN_EID_PREP = 131,
    WLAN_EID_PERR = 132,
// 133-136 reserved for Cisco
    WLAN_EID_PXU = 137,
    WLAN_EID_PXUC = 138,
    WLAN_EID_AUTH_MESH_PEER_EXCH = 139,
    WLAN_EID_MIC = 140,
    WLAN_EID_DESTINATION_URI = 141,
    WLAN_EID_UAPSD_COEX = 142,
    WLAN_EID_WAKEUP_SCHEDULE = 143,
    WLAN_EID_EXT_SCHEDULE = 144,
    WLAN_EID_STA_AVAILABILITY = 145,
    WLAN_EID_DMG_TSPEC = 146,
    WLAN_EID_DMG_AT = 147,
    WLAN_EID_DMG_CAP = 148,
// 149 reserved for Cisco
    WLAN_EID_CISCO_VENDOR_SPECIFIC = 150,
    WLAN_EID_DMG_OPERATION = 151,
    WLAN_EID_DMG_BSS_PARAM_CHANGE = 152,
    WLAN_EID_DMG_BEAM_REFINEMENT = 153,
    WLAN_EID_CHANNEL_MEASURE_FEEDBACK = 154,
// 155-156 reserved for Cisco
    WLAN_EID_AWAKE_WINDOW = 157,
    WLAN_EID_MULTI_BAND = 158,
    WLAN_EID_ADDBA_EXT = 159,
    WLAN_EID_NEXT_PCP_LIST = 160,
    WLAN_EID_PCP_HANDOVER = 161,
    WLAN_EID_DMG_LINK_MARGIN = 162,
    WLAN_EID_SWITCHING_STREAM = 163,
    WLAN_EID_SESSION_TRANSITION = 164,
    WLAN_EID_DYN_TONE_PAIRING_REPORT = 165,
    WLAN_EID_CLUSTER_REPORT = 166,
    WLAN_EID_RELAY_CAP = 167,
    WLAN_EID_RELAY_XFER_PARAM_SET = 168,
    WLAN_EID_BEAM_LINK_MAINT = 169,
    WLAN_EID_MULTIPLE_MAC_ADDR = 170,
    WLAN_EID_U_PID = 171,
    WLAN_EID_DMG_LINK_ADAPT_ACK = 172,
// 173 reserved for Symbol
    WLAN_EID_MCCAOP_ADV_OVERVIEW = 174,
    WLAN_EID_QUIET_PERIOD_REQ = 175,
// 176 reserved for Symbol
    WLAN_EID_QUIET_PERIOD_RESP = 177,
// 178-179 reserved for Symbol
// 180 reserved for ISO/IEC 20011
    WLAN_EID_EPAC_POLICY = 182,
    WLAN_EID_CLISTER_TIME_OFF = 183,
    WLAN_EID_INTER_AC_PRIO = 184,
    WLAN_EID_SCS_DESCRIPTOR = 185,
    WLAN_EID_QLOAD_REPORT = 186,
    WLAN_EID_HCCA_TXOP_UPDATE_COUNT = 187,
    WLAN_EID_HL_STREAM_ID = 188,
    WLAN_EID_GCR_GROUP_ADDR = 189,
    WLAN_EID_ANTENNA_SECTOR_ID_PATTERN = 190,
    WLAN_EID_VHT_CAPABILITY = 191,
    WLAN_EID_VHT_OPERATION = 192,
    WLAN_EID_EXTENDED_BSS_LOAD = 193,
    WLAN_EID_WIDE_BW_CHANNEL_SWITCH = 194,
    WLAN_EID_TX_POWER_ENVELOPE = 195,
    WLAN_EID_CHANNEL_SWITCH_WRAPPER = 196,
    WLAN_EID_AID = 197,
    WLAN_EID_QUIET_CHANNEL = 198,
    WLAN_EID_OPMODE_NOTIF = 199,

    WLAN_EID_REDUCED_NEIGHBOR_REPORT = 201,

    WLAN_EID_AID_REQUEST = 210,
    WLAN_EID_AID_RESPONSE = 211,
    WLAN_EID_S1G_BCN_COMPAT = 213,
    WLAN_EID_S1G_SHORT_BCN_INTERVAL = 214,
    WLAN_EID_S1G_TWT = 216,
    WLAN_EID_S1G_CAPABILITIES = 217,
    WLAN_EID_VENDOR_SPECIFIC = 221,
    WLAN_EID_QOS_PARAMETER = 222,
    WLAN_EID_S1G_OPERATION = 232,
    WLAN_EID_CAG_NUMBER = 237,
    WLAN_EID_AP_CSN = 239,
    WLAN_EID_FILS_INDICATION = 240,
    WLAN_EID_DILS = 241,
    WLAN_EID_FRAGMENT = 242,
    WLAN_EID_RSNX = 244,
    WLAN_EID_EXTENSION = 255
}

// Element ID Extensions for Element ID 255
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_eid_ext {
    WLAN_EID_EXT_ASSOC_DELAY_INFO = 1,
    WLAN_EID_EXT_FILS_REQ_PARAMS = 2,
    WLAN_EID_EXT_FILS_KEY_CONFIRM = 3,
    WLAN_EID_EXT_FILS_SESSION = 4,
    WLAN_EID_EXT_FILS_HLP_CONTAINER = 5,
    WLAN_EID_EXT_FILS_IP_ADDR_ASSIGN = 6,
    WLAN_EID_EXT_KEY_DELIVERY = 7,
    WLAN_EID_EXT_FILS_WRAPPED_DATA = 8,
    WLAN_EID_EXT_FILS_PUBLIC_KEY = 12,
    WLAN_EID_EXT_FILS_NONCE = 13,
    WLAN_EID_EXT_FUTURE_CHAN_GUIDANCE = 14,
    WLAN_EID_EXT_DH_PARAMETER = 32,
    WLAN_EID_EXT_HE_CAPABILITY = 35,
    WLAN_EID_EXT_HE_OPERATION = 36,
    WLAN_EID_EXT_UORA = 37,
    WLAN_EID_EXT_HE_MU_EDCA = 38,
    WLAN_EID_EXT_HE_SPR = 39,
    WLAN_EID_EXT_NDP_FEEDBACK_REPORT_PARAMSET = 41,
    WLAN_EID_EXT_BSS_COLOR_CHG_ANN = 42,
    WLAN_EID_EXT_QUIET_TIME_PERIOD_SETUP = 43,
    WLAN_EID_EXT_ESS_REPORT = 45,
    WLAN_EID_EXT_OPS = 46,
    WLAN_EID_EXT_HE_BSS_LOAD = 47,
    WLAN_EID_EXT_MAX_CHANNEL_SWITCH_TIME = 52,
    WLAN_EID_EXT_MULTIPLE_BSSID_CONFIGURATION = 55,
    WLAN_EID_EXT_NON_INHERITANCE = 56,
    WLAN_EID_EXT_KNOWN_BSSID = 57,
    WLAN_EID_EXT_SHORT_SSID_LIST = 58,
    WLAN_EID_EXT_HE_6GHZ_CAPA = 59,
    WLAN_EID_EXT_UL_MU_POWER_CAPA = 60,
    WLAN_EID_EXT_EHT_OPERATION = 106,
    WLAN_EID_EXT_EHT_MULTI_LINK = 107,
    WLAN_EID_EXT_EHT_CAPABILITY = 108,
    WLAN_EID_EXT_TID_TO_LINK_MAPPING = 109,
    WLAN_EID_EXT_BANDWIDTH_INDICATION = 135,
    WLAN_EID_EXT_KNOWN_STA_IDENTIFCATION = 136,
    WLAN_EID_EXT_NON_AP_STA_REG_CON = 137,
    WLAN_EID_EXT_UHR_OPER = 151,
    WLAN_EID_EXT_UHR_CAPA = 152,
    WLAN_EID_EXT_MACP = 153,
    WLAN_EID_EXT_SMD = 154,
    WLAN_EID_EXT_BSS_SMD_TRANS_PARAMS = 155,
    WLAN_EID_EXT_CHAN_USAGE = 156,
    WLAN_EID_EXT_UHR_MODE_CHG = 157,
    WLAN_EID_EXT_UHR_PARAM_UPD = 158,
    WLAN_EID_EXT_TXPI = 159,
}

// Action category code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_category {
    WLAN_CATEGORY_SPECTRUM_MGMT = 0,
    WLAN_CATEGORY_QOS = 1,
    WLAN_CATEGORY_DLS = 2,
    WLAN_CATEGORY_BACK = 3,
    WLAN_CATEGORY_PUBLIC = 4,
    WLAN_CATEGORY_RADIO_MEASUREMENT = 5,
    WLAN_CATEGORY_FAST_BBS_TRANSITION = 6,
    WLAN_CATEGORY_HT = 7,
    WLAN_CATEGORY_SA_QUERY = 8,
    WLAN_CATEGORY_PROTECTED_DUAL_OF_ACTION = 9,
    WLAN_CATEGORY_WNM = 10,
    WLAN_CATEGORY_WNM_UNPROTECTED = 11,
    WLAN_CATEGORY_TDLS = 12,
    WLAN_CATEGORY_MESH_ACTION = 13,
    WLAN_CATEGORY_MULTIHOP_ACTION = 14,
    WLAN_CATEGORY_SELF_PROTECTED = 15,
    WLAN_CATEGORY_DMG = 16,
    WLAN_CATEGORY_WMM = 17,
    WLAN_CATEGORY_FST = 18,
    WLAN_CATEGORY_UNPROT_DMG = 20,
    WLAN_CATEGORY_VHT = 21,
    WLAN_CATEGORY_S1G = 22,
    WLAN_CATEGORY_PROTECTED_EHT = 37,
    WLAN_CATEGORY_PROTECTED_UHR = 43,
    WLAN_CATEGORY_VENDOR_SPECIFIC_PROTECTED = 126,
    WLAN_CATEGORY_VENDOR_SPECIFIC = 127,
}

// SPECTRUM_MGMT action code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_spectrum_mgmt_actioncode {
    WLAN_ACTION_SPCT_MSR_REQ = 0,
    WLAN_ACTION_SPCT_MSR_RPRT = 1,
    WLAN_ACTION_SPCT_TPC_REQ = 2,
    WLAN_ACTION_SPCT_TPC_RPRT = 3,
    WLAN_ACTION_SPCT_CHL_SWITCH = 4,
}

// Self Protected Action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_self_protected_actioncode {
    WLAN_SP_RESERVED = 0,
    WLAN_SP_MESH_PEERING_OPEN = 1,
    WLAN_SP_MESH_PEERING_CONFIRM = 2,
    WLAN_SP_MESH_PEERING_CLOSE = 3,
    WLAN_SP_MGK_INFORM = 4,
    WLAN_SP_MGK_ACK = 5,
}

// Unprotected WNM action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_unprotected_wnm_actioncode {
    WLAN_UNPROTECTED_WNM_ACTION_TIM = 0,
    WLAN_UNPROTECTED_WNM_ACTION_TIMING_MEASUREMENT_RESPONSE = 1,
}

// Security key length
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_key_len {
    WLAN_KEY_LEN_WEP40 = 5,
    WLAN_KEY_LEN_WEP104 = 13,
    WLAN_KEY_LEN_CCMP = 16,
    WLAN_KEY_LEN_CCMP_256 = 32,
    WLAN_KEY_LEN_TKIP = 32,
    WLAN_KEY_LEN_AES_CMAC = 16,
    WLAN_KEY_LEN_SMS4 = 32,
    WLAN_KEY_LEN_GCMP = 16,
    WLAN_KEY_LEN_GCMP_256 = 32,
    WLAN_KEY_LEN_BIP_CMAC_256 = 32,
    WLAN_KEY_LEN_BIP_GMAC_128 = 16,
    WLAN_KEY_LEN_BIP_GMAC_256 = 32,
}

// Radio measurement action codes as defined in IEEE 802.11-2024 - Table 9-470
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_radio_measurement_actioncode {
    WLAN_RM_ACTION_RADIO_MEASUREMENT_REQUEST = 0,
    WLAN_RM_ACTION_RADIO_MEASUREMENT_REPORT  = 1,
    WLAN_RM_ACTION_LINK_MEASUREMENT_REQUEST  = 2,
    WLAN_RM_ACTION_LINK_MEASUREMENT_REPORT   = 3,
    WLAN_RM_ACTION_NEIGHBOR_REPORT_REQUEST   = 4,
    WLAN_RM_ACTION_NEIGHBOR_REPORT_RESPONSE  = 5,
}

pub const IEEE80211_WEP_IV_LEN: c_int = 4;
pub const IEEE80211_WEP_ICV_LEN: c_int = 4;
pub const IEEE80211_CCMP_HDR_LEN: c_int = 8;
pub const IEEE80211_CCMP_MIC_LEN: c_int = 8;
pub const IEEE80211_CCMP_PN_LEN: c_int = 6;
pub const IEEE80211_CCMP_256_HDR_LEN: c_int = 8;
pub const IEEE80211_CCMP_256_MIC_LEN: c_int = 16;
pub const IEEE80211_CCMP_256_PN_LEN: c_int = 6;
pub const IEEE80211_TKIP_IV_LEN: c_int = 8;
pub const IEEE80211_TKIP_ICV_LEN: c_int = 4;
pub const IEEE80211_CMAC_PN_LEN: c_int = 6;
pub const IEEE80211_GMAC_PN_LEN: c_int = 6;
pub const IEEE80211_GCMP_HDR_LEN: c_int = 8;
pub const IEEE80211_GCMP_MIC_LEN: c_int = 16;
pub const IEEE80211_GCMP_PN_LEN: c_int = 6;
pub const IEEE80211_CMAC_128_MIC_LEN: c_int = 8;
pub const IEEE80211_CMAC_256_MIC_LEN: c_int = 16;
pub const IEEE80211_GMAC_MIC_LEN: c_int = 16;
pub const FILS_NONCE_LEN: c_int = 16;
pub const FILS_MAX_KEK_LEN: c_int = 64;
pub const FILS_ERP_MAX_USERNAME_LEN: c_int = 16;
pub const FILS_ERP_MAX_REALM_LEN: c_int = 253;
pub const FILS_ERP_MAX_RRK_LEN: c_int = 64;
pub const PMK_MAX_LEN: c_int = 64;
pub const SAE_PASSWORD_MAX_LEN: c_int = 128;
pub const MICHAEL_MIC_LEN: c_int = 8;
// Public action codes (IEEE Std 802.11-2016, 9.6.8.1, Table 9-307)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_pub_actioncode {
    WLAN_PUB_ACTION_20_40_BSS_COEX = 0,
    WLAN_PUB_ACTION_DSE_ENABLEMENT = 1,
    WLAN_PUB_ACTION_DSE_DEENABLEMENT = 2,
    WLAN_PUB_ACTION_DSE_REG_LOC_ANN = 3,
    WLAN_PUB_ACTION_EXT_CHANSW_ANN = 4,
    WLAN_PUB_ACTION_DSE_MSMT_REQ = 5,
    WLAN_PUB_ACTION_DSE_MSMT_RESP = 6,
    WLAN_PUB_ACTION_MSMT_PILOT = 7,
    WLAN_PUB_ACTION_DSE_PC = 8,
    WLAN_PUB_ACTION_VENDOR_SPECIFIC = 9,
    WLAN_PUB_ACTION_GAS_INITIAL_REQ = 10,
    WLAN_PUB_ACTION_GAS_INITIAL_RESP = 11,
    WLAN_PUB_ACTION_GAS_COMEBACK_REQ = 12,
    WLAN_PUB_ACTION_GAS_COMEBACK_RESP = 13,
    WLAN_PUB_ACTION_TDLS_DISCOVER_RES = 14,
    WLAN_PUB_ACTION_LOC_TRACK_NOTI = 15,
    WLAN_PUB_ACTION_QAB_REQUEST_FRAME = 16,
    WLAN_PUB_ACTION_QAB_RESPONSE_FRAME = 17,
    WLAN_PUB_ACTION_QMF_POLICY = 18,
    WLAN_PUB_ACTION_QMF_POLICY_CHANGE = 19,
    WLAN_PUB_ACTION_QLOAD_REQUEST = 20,
    WLAN_PUB_ACTION_QLOAD_REPORT = 21,
    WLAN_PUB_ACTION_HCCA_TXOP_ADVERT = 22,
    WLAN_PUB_ACTION_HCCA_TXOP_RESPONSE = 23,
    WLAN_PUB_ACTION_PUBLIC_KEY = 24,
    WLAN_PUB_ACTION_CHANNEL_AVAIL_QUERY = 25,
    WLAN_PUB_ACTION_CHANNEL_SCHEDULE_MGMT = 26,
    WLAN_PUB_ACTION_CONTACT_VERI_SIGNAL = 27,
    WLAN_PUB_ACTION_GDD_ENABLEMENT_REQ = 28,
    WLAN_PUB_ACTION_GDD_ENABLEMENT_RESP = 29,
    WLAN_PUB_ACTION_NETWORK_CHANNEL_CONTROL = 30,
    WLAN_PUB_ACTION_WHITE_SPACE_MAP_ANN = 31,
    WLAN_PUB_ACTION_FTM_REQUEST = 32,
    WLAN_PUB_ACTION_FTM_RESPONSE = 33,
    WLAN_PUB_ACTION_FILS_DISCOVERY = 34,
}

// TDLS action codes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_tdls_actioncode {
    WLAN_TDLS_SETUP_REQUEST = 0,
    WLAN_TDLS_SETUP_RESPONSE = 1,
    WLAN_TDLS_SETUP_CONFIRM = 2,
    WLAN_TDLS_TEARDOWN = 3,
    WLAN_TDLS_PEER_TRAFFIC_INDICATION = 4,
    WLAN_TDLS_CHANNEL_SWITCH_REQUEST = 5,
    WLAN_TDLS_CHANNEL_SWITCH_RESPONSE = 6,
    WLAN_TDLS_PEER_PSM_REQUEST = 7,
    WLAN_TDLS_PEER_PSM_RESPONSE = 8,
    WLAN_TDLS_PEER_TRAFFIC_RESPONSE = 9,
    WLAN_TDLS_DISCOVERY_REQUEST = 10,
}

// Extended Channel Switching capability to be set in the 1st byte of
// the @WLAN_EID_EXT_CAPABILITY information element
//

// Multiple BSSID capability is set in the 6th bit of 3rd byte of the
// @WLAN_EID_EXT_CAPABILITY information element
//

// Timing Measurement protocol for time sync is set in the 7th bit of 3rd byte
// of the @WLAN_EID_EXT_CAPABILITY information element
//

// TDLS capabilities in the 4th byte of @WLAN_EID_EXT_CAPABILITY

// Interworking capabilities are set in 7th bit of 4th byte of the
// @WLAN_EID_EXT_CAPABILITY information element
//

//
// TDLS capabililites to be enabled in the 5th byte of the
// @WLAN_EID_EXT_CAPABILITY information element
//

// Defines the maximal number of MSDUs in an A-MSDU.

//
// Fine Timing Measurement Initiator - bit 71 of @WLAN_EID_EXT_CAPABILITY
// information element
//

// Defines support for TWT Requester and TWT Responder

//
// When set, indicates that the AP is able to tolerate 26-tone RU UL
// OFDMA transmissions using HE TB PPDU from OBSS (not falsely classify the
// 26-tone RU UL OFDMA transmissions as radar pulses).
//

// Defines support for enhanced multi-bssid advertisement

// Enable Beacon Protection

// TDLS specific payload type in the LLC/SNAP header
pub const WLAN_TDLS_SNAP_RFTYPE: c_uint = 0x2;
// BSS Coex IE information field bits

//
// IEEE 802.11-2007 7.3.2.9 Country information element
//
// Minimum length is 8 octets, ie len must be evenly
// divisible by 2
//
// Although the spec says 8 I'm seeing 6 in practice
pub const IEEE80211_COUNTRY_IE_MIN_LEN: c_int = 6;
// The Country String field of the element shall be 3 octets in length
pub const IEEE80211_COUNTRY_STRING_LEN: c_int = 3;
//
// For regulatory extension stuff see IEEE 802.11-2007
// Annex I (page 1141) and Annex J (page 1147). Also
// review 7.3.2.9.
//
// When dot11RegulatoryClassesRequired is true and the
// first_channel/reg_extension_id is >= 201 then the IE
// compromises of the 'ext' struct represented below:
//
// - Regulatory extension ID - when generating IE this just needs
// to be monotonically increasing for each triplet passed in
// the IE
// - Regulatory class - index into set of rules
// - Coverage class - index into air propagation time (Table 7-27),
// in microseconds, you can compute the air propagation time from
// the index by multiplying by 3, so index 10 yields a propagation
// of 10 us. Valid values are 0-31, values 32-255 are not defined
// yet. A value of 0 inicates air propagation of <= 1 us.
//
// See also Table I.2 for Emission limit sets and table
// I.3 for Behavior limit sets. Table J.1 indicates how to map
// a reg_class to an emission limit set and behavior limit set.
//
pub const IEEE80211_COUNTRY_EXTENSION_ID: c_int = 201;
//
// Channels numbers in the IE must be monotonically increasing
// if dot11RegulatoryClassesRequired is not true.
//
// If dot11RegulatoryClassesRequired is true consecutive
// subband triplets following a regulatory triplet shall
// have monotonically increasing first_channel number fields.
//
// Channel numbers shall not overlap.
//
// Note that max_power is signed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_country_ie_triplet {
    pub first_channel: u8,
    pub num_channels: u8,
    pub max_power: i8,
    pub chans: } __packed,
    pub reg_extension_id: u8,
    pub reg_class: u8,
    pub coverage_class: u8,
    pub ext: } __packed,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_timeout_interval_type {
    WLAN_TIMEOUT_REASSOC_DEADLINE = 1 /* 802.11r */,
    WLAN_TIMEOUT_KEY_LIFETIME = 2 /* 802.11r */,
    WLAN_TIMEOUT_ASSOC_COMEBACK = 3 /* 802.11w */,
}

//
// struct ieee80211_timeout_interval_ie - Timeout Interval element
// @type: type, see &enum ieee80211_timeout_interval_type
// @value: timeout interval value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_timeout_interval_ie {
    pub type: u8,
    pub value: __le32,
    pub __packed: },
//
// enum ieee80211_idle_options - BSS idle options
// @WLAN_IDLE_OPTIONS_PROTECTED_KEEP_ALIVE: the station should send an RSN
// protected frame to the AP to reset the idle timer at the AP for
// the station.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_idle_options {
    WLAN_IDLE_OPTIONS_PROTECTED_KEEP_ALIVE = BIT(0),
}

//
// struct ieee80211_bss_max_idle_period_ie - BSS max idle period element struct
//
// This structure refers to "BSS Max idle period element"
//
// @max_idle_period: indicates the time period during which a station can
// refrain from transmitting frames to its associated AP without being
// disassociated. In units of 1000 TUs.
// @idle_options: indicates the options associated with the BSS idle capability
// as specified in &enum ieee80211_idle_options.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bss_max_idle_period_ie {
    pub max_idle_period: __le16,
    pub idle_options: u8,
    pub __packed: },
// SA Query action
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_sa_query_action {
    WLAN_ACTION_SA_QUERY_REQUEST = 0,
    WLAN_ACTION_SA_QUERY_RESPONSE = 1,
}

//
// struct ieee80211_bssid_index - multiple BSSID index element structure
//
// This structure refers to "Multiple BSSID-index element"
//
// @bssid_index: BSSID index
// @dtim_period: optional, overrides transmitted BSS dtim period
// @dtim_count: optional, overrides transmitted BSS dtim count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_bssid_index {
    pub bssid_index: u8,
    pub dtim_period: u8,
    pub dtim_count: u8,
}

//
// struct ieee80211_multiple_bssid_configuration - multiple BSSID configuration
// element structure
//
// This structure refers to "Multiple BSSID Configuration element"
//
// @bssid_count: total number of active BSSIDs in the set
// @profile_periodicity: the least number of beacon frames need to be received
// in order to discover all the nontransmitted BSSIDs in the set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_multiple_bssid_configuration {
    pub bssid_count: u8,
    pub profile_periodicity: u8,
}

// cipher suite selectors

// reserved: 				SUITE(0x000FAC, 3)

// AKM suite selectors

pub const WLAN_MAX_KEY_LEN: c_int = 32;
pub const WLAN_MAX_SECURE_LTF_KEYSEED_LEN: c_int = 48;
pub const WLAN_PMK_NAME_LEN: c_int = 16;
pub const WLAN_PMKID_LEN: c_int = 16;
pub const WLAN_PMK_LEN_EAP_LEAP: c_int = 16;
pub const WLAN_PMK_LEN: c_int = 32;
pub const WLAN_PMK_LEN_SUITE_B_192: c_int = 48;
pub const WLAN_OUI_WFA: c_uint = 0x506f9a;
pub const WLAN_OUI_TYPE_WFA_P2P: c_int = 9;
pub const WLAN_OUI_TYPE_WFA_NAN: c_uint = 0x13;
pub const WLAN_OUI_TYPE_WFA_DPP: c_uint = 0x1A;
pub const WLAN_OUI_MICROSOFT: c_uint = 0x0050f2;
pub const WLAN_OUI_TYPE_MICROSOFT_WPA: c_int = 1;
pub const WLAN_OUI_TYPE_MICROSOFT_WMM: c_int = 2;
pub const WLAN_OUI_TYPE_MICROSOFT_WPS: c_int = 4;
pub const WLAN_OUI_TYPE_MICROSOFT_TPC: c_int = 8;
//
// WMM/802.11e Tspec Element
//
pub const IEEE80211_WMM_IE_TSPEC_TID_MASK: c_uint = 0x0F;
pub const IEEE80211_WMM_IE_TSPEC_TID_SHIFT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_tspec_status_code {
    IEEE80211_TSPEC_STATUS_ADMISS_ACCEPTED = 0,
    IEEE80211_TSPEC_STATUS_ADDTS_INVAL_PARAMS = 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tspec_ie {
    pub element_id: u8,
    pub len: u8,
    pub oui: [u8; 3],
    pub oui_type: u8,
    pub oui_subtype: u8,
    pub version: u8,
    pub tsinfo: __le16,
    pub tsinfo_resvd: u8,
    pub nominal_msdu: __le16,
    pub max_msdu: __le16,
    pub min_service_int: __le32,
    pub max_service_int: __le32,
    pub inactivity_int: __le32,
    pub suspension_int: __le32,
    pub service_start_time: __le32,
    pub min_data_rate: __le32,
    pub mean_data_rate: __le32,
    pub peak_data_rate: __le32,
    pub max_burst_size: __le32,
    pub delay_bound: __le32,
    pub min_phy_rate: __le32,
    pub sba: __le16,
    pub medium_time: __le16,
    pub __packed: },
//
// ieee80211_get_qos_ctl - get pointer to qos control bytes
// @hdr: the frame
// Return: a pointer to the QoS control field in the frame header
//
// The qos ctrl bytes come after the frame_control, duration, seq_num
// and 3 or 4 addresses of length ETH_ALEN. Checks frame_control to choose
// between struct ieee80211_qos_hdr_4addr and struct ieee80211_qos_hdr.
//
    pub addr3: ieee80211_qos_hdr,
    pub addr4: ieee80211_qos_hdr_4addr,
    pub qos: *mut },
    pub )hdr: *mut qos = (void,
    pub )&qos->addr4.qos_ctrl: *mut return (u8,
    pub )&qos->addr3.qos_ctrl: *mut return (u8,
//
// ieee80211_get_tid - get qos TID
// @hdr: the frame
// Return: the TID from the QoS control field
//
    pub ieee80211_get_qos_ctl(hdr): *mut *mut u8 qc =,
    pub IEEE80211_QOS_CTL_TID_MASK: return qc[0] &,
//
// ieee80211_get_SA - get pointer to SA
// @hdr: the frame
// Return: a pointer to the source address (SA)
//
// Given an 802.11 frame, this function returns the offset
// to the source address (SA). It does not verify that the
// header is long enough to contain the address, and the
// header must be long enough to contain the frame control
// field.
//
    pub hdr->addr4: return,
    pub hdr->addr3: return,
    pub hdr->addr2: return,
//
// ieee80211_get_DA - get pointer to DA
// @hdr: the frame
// Return: a pointer to the destination address (DA)
//
// Given an 802.11 frame, this function returns the offset
// to the destination address (DA). It does not verify that
// the header is long enough to contain the address, and the
// header must be long enough to contain the frame control
// field.
//
    pub hdr->addr3: return,
    pub hdr->addr1: return,
//
// ieee80211_is_bufferable_mmpdu - check if frame is bufferable MMPDU
// @skb: the skb to check, starting with the 802.11 header
// Return: whether or not the MMPDU is bufferable
//
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (void,
    pub mgmt->frame_control: __le16 fc =,
//
// IEEE 802.11 REVme D2.0 definition of bufferable MMPDU;
// note that this ignores the IBSS special case.
//
    pub false: return,
    pub true: return,
    pub false: return,
    pub true: return,
// action frame - additionally check for non-bufferable FTM
    pub true: return,
    pub false: return,
    pub true: return,
//
// _ieee80211_is_robust_mgmt_frame - check if frame is a robust management frame
// @hdr: the frame (buffer must include at least the first octet of payload)
// Return: whether or not the frame is a robust management frame
//
    pub true: return,
    pub category: *mut u8,
//
// Action frames, excluding Public Action frames, are Robust
// Management Frames. However, if we are looking at a Protected
// frame, skip the check since the data may be encrypted and
// the frame has already been found to be a Robust Management
// Frame (by the other end).
//
    pub true: return,
    pub 24: *mut *mut category = ((u8 ) hdr) +,
// category != WLAN_CATEGORY_HT &&
// category != WLAN_CATEGORY_WNM_UNPROTECTED &&
// category != WLAN_CATEGORY_SELF_PROTECTED &&
// category != WLAN_CATEGORY_UNPROT_DMG &&
// category != WLAN_CATEGORY_VHT &&
// category != WLAN_CATEGORY_S1G &&
// category != WLAN_CATEGORY_VENDOR_SPECIFIC;
    pub false: return,
//
// ieee80211_is_robust_mgmt_frame - check if skb contains a robust mgmt frame
// @skb: the skb containing the frame, length will be checked
// Return: whether or not the frame is a robust management frame
//
    pub false: return,
    pub )skb->data): *mut return _ieee80211_is_robust_mgmt_frame((void,
//
// ieee80211_is_public_action - check if frame is a public action frame
// @hdr: the frame
// @len: length of the frame
// Return: whether or not the frame is a public action frame
//
    pub )hdr: *mut *mut ieee80211_mgmt mgmt = (void,
    pub false: return,
    pub false: return,
    pub WLAN_CATEGORY_PUBLIC: return mgmt->u.action.category ==,
//
// ieee80211_is_protected_dual_of_public_action - check if skb contains a
// protected dual of public action management frame
// @skb: the skb containing the frame, length will be checked
//
// Return: true if the skb contains a protected dual of public action
// management frame, false otherwise.
//
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (void,
    pub action: u8,
    pub false: return,
    pub mgmt->u.action.action_code: action =,
    pub WLAN_PUB_ACTION_VENDOR_SPECIFIC: action !=,
//
// _ieee80211_is_group_privacy_action - check if frame is a group addressed
// privacy action frame
// @hdr: the frame
// Return: whether or not the frame is a group addressed privacy action frame
//
    pub )hdr: *mut *mut ieee80211_mgmt mgmt = (void,
    pub false: return,
    pub WLAN_CATEGORY_MULTIHOP_ACTION: mgmt->u.action.category ==,
//
// ieee80211_is_group_privacy_action - check if frame is a group addressed
// privacy action frame
// @skb: the skb containing the frame, length will be checked
// Return: whether or not the frame is a group addressed privacy action frame
//
    pub false: return,
    pub )skb->data): *mut return _ieee80211_is_group_privacy_action((void,
//
// ieee80211_tu_to_usec - convert time units (TU) to microseconds
// @tu: the TUs
// Return: the time value converted to microseconds
//
    pub tu: *mut *mut return 1024,
    pub mask: u8,
    pub indexn2: u8 index, indexn1,,
    pub false: return,
    pub 0x3fff: aid &=,
    pub 8: index = aid /,
    pub 7): mask = 1 << (aid &,
    pub 0xfe: indexn1 = tim->bitmap_ctrl &,
    pub 4: indexn2 = tim_len + indexn1 -,
    pub false: return,
    pub indexn1: index -=,
    pub mask): return !!(tim->virtual_map[index] &,
//
// ieee80211_get_tdls_action - get TDLS action code
// @skb: the skb containing the frame, length will not be checked
// Return: the TDLS action code, or -1 if it's not an encapsulated TDLS action
// frame
//
// This function assumes the frame is a data frame, and that the network header
// is in the correct place.
//
// Point to where the indication of TDLS should start
    pub 2: *const *const u8 tdls_data = skb_network_header(skb) -,
    pub tdls_data: [return; 4],
    pub -1: return,
// convert time units

// convert frequencies

// convert powers

//
// ieee80211_action_contains_tpc - checks if the frame contains TPC element
// @skb: the skb containing the frame, length will be checked
// Return: %true if the frame contains a TPC element, %false otherwise
//
// This function checks if it's either TPC report action frame or Link
// Measurement report action frame as defined in IEEE Std. 802.11-2012 8.5.2.5
// and 8.5.7.5 accordingly.
//
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (void,
    pub false: return,
    pub false: return,
//
// TPC report - check that:
// category = 0 (Spectrum Management) or 5 (Radio Measurement)
// spectrum management action = 3 (TPC/Link Measurement report)
// TPC report EID = 35
// TPC report element length = 2
//
// The spectrum management's tpc_report struct is used here both for
// parsing tpc_report and radio measurement's link measurement report
// frame, since the relevant part is identical in both frames.
//
    pub false: return,
// both spectrum mgmt and link measurement have same action code
    pub false: return,
    pub false: return,
    pub true: return,
//
// ieee80211_is_timing_measurement - check if frame is timing measurement response
// @skb: the SKB to check
// Return: whether or not the frame is a valid timing measurement response
//
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (void,
    pub false: return,
    pub false: return,
    pub true: return,
    pub false: return,
//
// ieee80211_is_ftm - check if frame is FTM response
// @skb: the SKB to check
// Return: whether or not the frame is a valid FTM response action frame
//
    pub )skb->data: *mut *mut ieee80211_mgmt mgmt = (void,
    pub false: return,
    pub false: return,
    pub WLAN_PUB_ACTION_FTM_RESPONSE: return mgmt->u.action.action_code ==,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct element {
    pub id: u8,
    pub datalen: u8,
    pub data: [u8; ],
    pub __packed: },
// element iteration helpers

    pub \: *const *const for (_elem = (struct element )(_data);,
    pub \: *mut *mut (int)sizeof(_elem) + _elem->datalen;,

//
// for_each_element_completed - determine if element parsing consumed all data
// @element: element pointer after for_each_element() or friends
// @data: same data pointer as passed to for_each_element() or friends
// @datalen: same data length as passed to for_each_element() or friends
// Return: %true if all elements were iterated, %false otherwise; see notes
//
// This function returns %true if all the data was parsed or considered
// while walking the elements. Only use this if your for_each_element()
// loop cannot be broken out of, otherwise it always returns %false.
//
// If some data was malformed, this returns %false since the last parsed
// element will not fill the whole remaining data.
//
    pub datalen: *const *const *const return (u8 )element == (u8 )data +,
//
// RSNX Capabilities:
// bits 0-3: Field length (n-1)
//

// EBPCC = Enhanced BSS Parameter Change Count
pub const IEEE80211_ENH_CRIT_UPD_EBPCC: c_uint = 0x0F;
pub const IEEE80211_ENH_CRIT_UPD_TYPE: c_uint = 0x70;
pub const IEEE80211_ENH_CRIT_UPD_TYPE_NO_UHR: c_int = 0;
pub const IEEE80211_ENH_CRIT_UPD_TYPE_UHR: c_int = 1;
pub const IEEE80211_ENH_CRIT_UPD_ALL: c_uint = 0x80;
//
// struct ieee80211_enh_crit_upd - enhanced critical update (UHR)
// @v: value of the enhanced critical update data,
// see %IEEE80211_ENH_CRIT_UPD_* to parse the bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_enh_crit_upd {
    pub v: u8,
    pub __packed: },
//
// reduced neighbor report, based on Draft P802.11ax_D6.1,
// section 9.4.2.170 and accepted contributions.
//
pub const IEEE80211_AP_INFO_TBTT_HDR_TYPE: c_uint = 0x03;
pub const IEEE80211_AP_INFO_TBTT_HDR_FILTERED: c_uint = 0x04;
pub const IEEE80211_AP_INFO_TBTT_HDR_COLOC: c_uint = 0x08;
pub const IEEE80211_AP_INFO_TBTT_HDR_COUNT: c_uint = 0xF0;
pub const IEEE80211_TBTT_INFO_TYPE_TBTT: c_int = 0;
pub const IEEE80211_TBTT_INFO_TYPE_MLD: c_int = 1;
pub const IEEE80211_RNR_TBTT_PARAMS_OCT_RECOMMENDED: c_uint = 0x01;
pub const IEEE80211_RNR_TBTT_PARAMS_SAME_SSID: c_uint = 0x02;
pub const IEEE80211_RNR_TBTT_PARAMS_MULTI_BSSID: c_uint = 0x04;
pub const IEEE80211_RNR_TBTT_PARAMS_TRANSMITTED_BSSID: c_uint = 0x08;
pub const IEEE80211_RNR_TBTT_PARAMS_COLOC_ESS: c_uint = 0x10;
pub const IEEE80211_RNR_TBTT_PARAMS_PROBE_ACTIVE: c_uint = 0x20;
pub const IEEE80211_RNR_TBTT_PARAMS_COLOC_AP: c_uint = 0x40;
pub const IEEE80211_RNR_TBTT_PARAMS_SAME_SMD: c_uint = 0x80;
pub const IEEE80211_RNR_TBTT_PARAMS_PSD_NO_LIMIT: c_int = 127;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_neighbor_ap_info {
    pub tbtt_info_hdr: u8,
    pub tbtt_info_len: u8,
    pub op_class: u8,
    pub channel: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_range_params_max_total_ltf {
    IEEE80211_RANGE_PARAMS_MAX_TOTAL_LTF_4 = 0,
    IEEE80211_RANGE_PARAMS_MAX_TOTAL_LTF_8,
    IEEE80211_RANGE_PARAMS_MAX_TOTAL_LTF_16,
    IEEE80211_RANGE_PARAMS_MAX_TOTAL_LTF_UNSPECIFIED,
}

//
// reduced neighbor report, based on Draft P802.11be_D3.0,
// section 9.4.2.170.2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_rnr_mld_params {
    pub mld_id: u8,
    pub params: __le16,
    pub __packed: },
pub const IEEE80211_RNR_MLD_PARAMS_LINK_ID: c_uint = 0x000F;
pub const IEEE80211_RNR_MLD_PARAMS_BSS_CHANGE_COUNT: c_uint = 0x0FF0;
pub const IEEE80211_RNR_MLD_PARAMS_UPDATES_INCLUDED: c_uint = 0x1000;
pub const IEEE80211_RNR_MLD_PARAMS_DISABLED_LINK: c_uint = 0x2000;
// Format of the TBTT information element if it has 7, 8 or 9 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tbtt_info_7_8_9 {
    pub tbtt_offset: u8,
    pub bssid: [u8; ETH_ALEN],
// The following element is optional, structure may not grow
    pub bss_params: u8,
    pub psd_20: i8,
    pub __packed: },
// Format of the TBTT information element if it has >= 11 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_tbtt_info_ge_11 {
    pub tbtt_offset: u8,
    pub bssid: [u8; ETH_ALEN],
    pub short_ssid: __le32,
// The following elements are optional, structure may grow
    pub bss_params: u8,
    pub psd_20: i8,
    pub mld_params: ieee80211_rnr_mld_params,
    pub enh_crit_upd: ieee80211_enh_crit_upd,
    pub __packed: },

//
// ieee80211_check_tim - check if AID bit is set in TIM
// @tim: the TIM IE
// @tim_len: length of the TIM IE
// @aid: the AID to look for
// @s1g: whether the TIM is from an S1G PPDU
// Return: whether or not traffic is indicated in the TIM for the given AID
//
    pub aid): __ieee80211_check_tim(tim, tim_len,,
