//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ieee80211-uhr.h
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
// IEEE 802.11 UHR definitions
//
// Copyright (c) 2025-2026 Intel Corporation
//

pub const IEEE80211_UHR_OPER_PARAMS_DPS_ENA: c_uint = 0x0001;
pub const IEEE80211_UHR_OPER_PARAMS_NPCA_ENA: c_uint = 0x0002;
pub const IEEE80211_UHR_OPER_PARAMS_PEDCA_ENA: c_uint = 0x0004;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_ENA: c_uint = 0x0008;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_BW: c_uint = 0x0070;
pub const IEEE80211_UHR_OPER_PARAMS_DUO_PRES: c_uint = 0x0080;
pub const IEEE80211_UHR_OPER_PARAMS_DPS_PRES: c_uint = 0x0100;
pub const IEEE80211_UHR_OPER_PARAMS_NPCA_PRES: c_uint = 0x0200;
pub const IEEE80211_UHR_OPER_PARAMS_PEDCA_PRES: c_uint = 0x0400;
pub const IEEE80211_UHR_OPER_PARAMS_DBE_PRES: c_uint = 0x0800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_operation {
    pub params: __le16,
    pub basic_mcs_nss_set: [u8; 4],
    pub variable: [u8; ],
    pub __packed: },
pub const IEEE80211_UHR_NPCA_PARAMS_PRIMARY_CHAN_OFFS: c_uint = 0x0000000F;
pub const IEEE80211_UHR_NPCA_PARAMS_MIN_DUR_THRESH: c_uint = 0x000000F0;
pub const IEEE80211_UHR_NPCA_PARAMS_SWITCH_DELAY: c_uint = 0x00003F00;
pub const IEEE80211_UHR_NPCA_PARAMS_SWITCH_BACK_DELAY: c_uint = 0x000FC000;
pub const IEEE80211_UHR_NPCA_PARAMS_INIT_QSRC: c_uint = 0x00300000;
pub const IEEE80211_UHR_NPCA_PARAMS_MOPLEN: c_uint = 0x00400000;
pub const IEEE80211_UHR_NPCA_PARAMS_DIS_SUBCH_BMAP_PRES: c_uint = 0x00800000;
//
// struct ieee80211_uhr_npca_info - npca operation information
//
// This structure is the "NPCA Operation Parameters field format" of "UHR
// Operation Element" fields as described in P802.11bn_D1.3
// subclause 9.4.2.353. See Figure 9-aa4.
//
// Refer to IEEE80211_UHR_NPCA
// @params:
// NPCA Primary Channel - NPCA primary channel
// NPCA_Min Duration Threshold - Minimum duration of inter-BSS activity
// NPCA Switching Delay -
// Time needed by an NPCA AP to switch from the
// BSS primary channel to the NPCA primary channel
// in the unit of 4 µs.
// NPCA Switching Back Delay -
// Time to switch from the NPCA primary channel
// to the BSS primary channel in the unit of 4 µs.
// NPCA Initial QSRC -
// Initialize the EDCAF QSRC[AC] variables
// when an NPCA STA in the BSS
// switches to NPCA operation.
// NPCA MOPLEN -
// Indicates which conditions can be used to
// initiate an NPCA operation,
// 1 -> both PHYLEN NPCA operation and MOPLEN
// NPCA operation are
// permitted in the BSS
// 0 -> only PHYLEN NPCA operation is allowed in the BSS.
// NPCA Disabled Subchannel Bitmap Present -
// Indicates whether the NPCA Disabled Subchannel
// Bitmap field is present. A 1 in this field indicates that
// the NPCA Disabled Subchannel Bitmap field is present
// @dis_subch_bmap:
// A bit in the bitmap that lies within the BSS bandwidth is set
// to 1 to indicate that the corresponding 20 MHz subchannel is
// punctured and is set to 0 to indicate that the corresponding
// 20 MHz subchannel is not punctured. A bit in the bitmap that
// falls outside of the BSS bandwidth is reserved. This field is
// present when the value of the NPCA Disabled Subchannel Bitmap
// Field Present field is equal to 1, and not present, otherwise
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_npca_info {
    pub params: __le32,
    pub dis_subch_bmap: [__le16; ],
    pub __packed: },
pub const IEEE80211_UHR_DPS_PADDING_DELAY: c_uint = 0x0000003F;
pub const IEEE80211_UHR_DPS_TRANSITION_DELAY: c_uint = 0x00003F00;
pub const IEEE80211_UHR_DPS_ICF_REQUIRED: c_uint = 0x00010000;
pub const IEEE80211_UHR_DPS_PARAMETERIZED_FLAG: c_uint = 0x00020000;
pub const IEEE80211_UHR_DPS_LC_MODE_BW: c_uint = 0x001C0000;
pub const IEEE80211_UHR_DPS_LC_MODE_NSS: c_uint = 0x01E00000;
pub const IEEE80211_UHR_DPS_LC_MODE_MCS: c_uint = 0x1E000000;
pub const IEEE80211_UHR_DPS_MOBILE_AP_DPS_STATIC_HCM: c_uint = 0x20000000;
//
// struct ieee80211_uhr_dps_info - DPS operation information
//
// This structure is the "DPS Operation Parameter field" of "UHR
// Operation Element" fields as described in P802.11bn_D1.3
// subclause 9.4.1.87. See Figure 9-207u.
//
// Refer to IEEE80211_UHR_DPS
// @params:
// DPS Padding Delay -
// Indicates the minimum MAC padding
// duration that is required by a DPS STA
// in an ICF to cause the STA to transition
// from the lower capability mode to the
// higher capability mode. The DPS Padding
// Delay field is in units of 4 µs.
// DPS Transition Delay -
// Indicates the amount of time required by a
// DPS STA to transition from the higher
// capability mode to the lower capability
// mode. The DPS Transition Delay field is in
// units of 4 µs.
// ICF Required -
// Indicates when the DPS assisting STA needs
// to transmit an ICF frame to the peer DPS STA
// before performing the frame exchanges with
// the peer DPS STA in a TXOP.
// 1 -> indicates that the transmission of the
// ICF frame to the peer DPS STA prior to
// any frame exchange is needed.
// 0 -> ICF transmission before the frame
// exchanges with the peer DPS STA is only
// needed if the frame exchange is performed
// in the HC mode.
// Parameterized Flag -
// 0 -> indicates that only 20 MHz, 1 SS,
// non-HT PPDU format with the data
// rate of 6, 12, and 24 Mb/s as the
// default mode are supported by the
// DPS STA in the LC mode
// 1 -> indicates that a bandwidth up to the
// bandwidth indicated in the LC Mode
// Bandwidth field, a number of spatial
// streams up to the NSS indicated in
// the LC Mode Nss field, and an MCS up
// to the MCS indicated in the LC Mode
// MCS fields are supported by the DPS
// STA in the LC mode as the
// parameterized mode.
// LC Mode Bandwidth -
// Indicates the maximum bandwidth supported
// by the STA in the LC mode.
// LC Mode NSS -
// Indicates the maximum number of the spatial
// streams supported by the STA in the LC mode.
// LC Mode MCS -
// Indicates the highest MCS supported by the STA
// in the LC mode.
// Mobile AP DPS Static HCM -
// 1 -> indicates that it will remain in the DPS high
// capability mode until the next TBTT on that
// link.
// 0 -> otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_dps_info {
    pub params: __le32,
    pub __packed: },
pub const IEEE80211_UHR_DBE_OPER_BANDWIDTH: c_uint = 0x07;
pub const IEEE80211_UHR_DBE_OPER_DIS_SUBCHANNEL_BITMAP_PRES: c_uint = 0x08;
//
// enum ieee80211_uhr_dbe_oper_bw - DBE Operational Bandwidth
//
// Encoding for the DBE Operational Bandwidth field in the UHR Operation
// element (DBE Operation Parameters).
//
// @IEEE80211_UHR_DBE_OPER_BW_40: 40 MHz operational DBE bandwidth
// @IEEE80211_UHR_DBE_OPER_BW_80: 80 MHz operational DBE bandwidth
// @IEEE80211_UHR_DBE_OPER_BW_160: 160 MHz operational DBE bandwidth
// @IEEE80211_UHR_DBE_OPER_BW_320_1: 320-1 MHz operational DBE bandwidth
// @IEEE80211_UHR_DBE_OPER_BW_320_2: 320-2 MHz operational DBE bandwidth
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_dbe_oper_bw {
    IEEE80211_UHR_DBE_OPER_BW_40 = 1,
    IEEE80211_UHR_DBE_OPER_BW_80 = 2,
    IEEE80211_UHR_DBE_OPER_BW_160 = 3,
    IEEE80211_UHR_DBE_OPER_BW_320_1 = 4,
    IEEE80211_UHR_DBE_OPER_BW_320_2 = 5,
}

//
// ieee80211_uhr_dbe_bw_mhz - get bandwidth in MHz from UHR DBE bandwidth
// @bw: UHR DBE bandwidth
//
// Return: the bandwidth in MHz, or -1 for invalid values
//
    pub 40: return,
    pub 80: return,
    pub 160: return,
    pub 320: return,
    pub -1: return,
//
// struct ieee80211_uhr_dbe_info - DBE operation information
//
// This structure is the "DBE Operation Parameters field" of
// "UHR Operation Element" fields as described in P802.11bn_D1.3
// subclause 9.4.2.353. See Figure 9-aa6.
//
// Refer to IEEE80211_UHR_DBE_OPER
// @params:
// B0-B2 - DBE Operational Bandwidth field, see
// "enum ieee80211_uhr_dbe_oper_bw" for values.
// Value 0 is reserved.
// Value 1 indicates 40 MHz operational DBE bandwidth.
// Value 2 indicates 80 MHz operational DBE bandwidth.
// Value 3 indicates 160 MHz operational DBE bandwidth.
// Value 4 indicates 320-1 MHz operational DBE bandwidth.
// Value 5 indicates 320-2 MHz operational DBE bandwidth.
// Values 6 to 7 are reserved.
// B3 - DBE Disabled Subchannel Bitmap Present.
// @dis_subch_bmap: DBE Disabled Subchannel Bitmap field is set to indicate
// disabled 20 MHz subchannels within the DBE Bandwidth.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_dbe_info {
    pub params: u8,
    pub dis_subch_bmap: [__le16; ],
    pub __packed: },
pub const IEEE80211_UHR_P_EDCA_ECWMIN: c_uint = 0x0F;
pub const IEEE80211_UHR_P_EDCA_ECWMAX: c_uint = 0xF0;
pub const IEEE80211_UHR_P_EDCA_AIFSN: c_uint = 0x000F;
pub const IEEE80211_UHR_P_EDCA_CW_DS: c_uint = 0x0030;
pub const IEEE80211_UHR_P_EDCA_PSRC_THRESHOLD: c_uint = 0x01C0;
pub const IEEE80211_UHR_P_EDCA_QSRC_THRESHOLD: c_uint = 0x0600;
//
// struct ieee80211_uhr_p_edca_info - P-EDCA operation information
//
// This structure is the "P-EDCA Operation Parameters field" of
// "UHR Operation Element" fields as described in P802.11bn_D1.3
// subclause 9.4.2.353. See Figure 9-aa5.
//
// Refer to IEEE80211_UHR_P_EDCA
// @p_edca_ec: P-EDCA ECWmin and ECWmax.
// These fields indicate the CWmin and CWmax values used by a
// P-EDCA STA during P-EDCA contention.
// @params: AIFSN, CW DS, PSRC threshold, and QSRC threshold.
// - The AIFSN field indicates the AIFSN value used by a P-EDCA STA
// during P-EDCA contention.
// - The CW DS field indicates the value used for randomization of the
// transmission slot of the DS-CTS frame. The value 3 is reserved.
// The value 0 indicates that randomization is not enabled.
// - The P-EDCA PSRC threshold field indicates the maximum number of
// allowed consecutive DS-CTS transmissions. The value 0 and values
// greater than 4 are reserved.
// - The P-EDCA QSRC threshold field indicates the value of the
// QSRC[AC_VO] counter required to start P-EDCA contention. The
// value 0 is reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_p_edca_info {
    pub p_edca_ec: u8,
    pub params: __le16,
    pub __packed: },
    pub )data: *const *const ieee80211_uhr_operation oper = (void,
    pub sizeof(*oper): *mut u8 needed =,
    pub false: return,
// DPS Operation Parameters (fixed 4 bytes)
    pub ieee80211_uhr_dps_info): needed += sizeof(struct,
    pub false: return,
// NPCA Operation Parameters (fixed 4 bytes + optional 2 bytes)
    pub needed): *const *const (void )(data +,
    pub sizeof(*npca): *mut needed +=,
    pub false: return,
    pub sizeof(npca->dis_subch_bmap[0]): needed +=,
    pub false: return,
// P-EDCA Operation Parameters (fixed 3 bytes)
    pub ieee80211_uhr_p_edca_info): needed += sizeof(struct,
    pub false: return,
// DBE Operation Parameters (fixed 1 byte + optional 2 bytes)
    pub needed): *const *const (void )(data +,
    pub sizeof(*dbe): *mut needed +=,
    pub false: return,
    pub sizeof(dbe->dis_subch_bmap[0]): needed +=,
    pub false: return,
    pub needed: return len >=,
// Note: must ensure ieee80211_uhr_oper_size_ok(...) first
    pub oper->variable: *const *const u8 pos =,
    pub NULL: return,
    pub NULL: return,
    pub ieee80211_uhr_dps_info): pos += sizeof(struct,
    pub )pos: *const return (void,
    pub npca: *const ieee80211_uhr_npca_info,
    pub ieee80211_uhr_npca_info(oper): npca =,
    pub NULL: return,
    pub NULL: return,
    pub npca->dis_subch_bmap: return,
// Note: must ensure ieee80211_uhr_oper_size_ok(...) first
    pub oper->variable: *const *const u8 pos =,
    pub NULL: return,
    pub NULL: return,
    pub ieee80211_uhr_dps_info): pos += sizeof(struct,
    pub )pos: *const *const ieee80211_uhr_npca_info npca = (void,
    pub sizeof(*npca): *mut pos +=,
    pub sizeof(npca->dis_subch_bmap[0]): pos +=,
    pub ieee80211_uhr_p_edca_info): pos += sizeof(struct,
    pub )pos: *const return (void,
pub const IEEE80211_UHR_MAC_CAP0_DPS_SUPP: c_uint = 0x01;
pub const IEEE80211_UHR_MAC_CAP0_DPS_ASSIST_SUPP: c_uint = 0x02;
pub const IEEE80211_UHR_MAC_CAP0_DPS_AP_STATIC_HCM_SUPP: c_uint = 0x04;
pub const IEEE80211_UHR_MAC_CAP0_NPCA_SUPP: c_uint = 0x10;
pub const IEEE80211_UHR_MAC_CAP0_ENH_BSR_SUPP: c_uint = 0x20;
pub const IEEE80211_UHR_MAC_CAP0_ADD_MAP_TID_SUPP: c_uint = 0x40;
pub const IEEE80211_UHR_MAC_CAP0_EOTSP_SUPP: c_uint = 0x80;
pub const IEEE80211_UHR_MAC_CAP1_DSO_SUPP: c_uint = 0x01;
pub const IEEE80211_UHR_MAC_CAP1_PEDCA_SUPP: c_uint = 0x02;
pub const IEEE80211_UHR_MAC_CAP1_DBE_SUPP: c_uint = 0x04;
pub const IEEE80211_UHR_MAC_CAP1_UL_LLI_SUPP: c_uint = 0x08;
pub const IEEE80211_UHR_MAC_CAP1_P2P_LLI_SUPP: c_uint = 0x10;
pub const IEEE80211_UHR_MAC_CAP1_PUO_SUPP: c_uint = 0x20;
pub const IEEE80211_UHR_MAC_CAP1_AP_PUO_SUPP: c_uint = 0x40;
pub const IEEE80211_UHR_MAC_CAP1_DUO_SUPP: c_uint = 0x80;
pub const IEEE80211_UHR_MAC_CAP2_OMC_UL_MU_DIS_RX_SUPP: c_uint = 0x01;
pub const IEEE80211_UHR_MAC_CAP2_AOM_SUPP: c_uint = 0x02;
pub const IEEE80211_UHR_MAC_CAP2_IFCS_LOC_SUPP: c_uint = 0x04;
pub const IEEE80211_UHR_MAC_CAP2_UHR_TRS_SUPP: c_uint = 0x08;
pub const IEEE80211_UHR_MAC_CAP2_TXSPG_SUPP: c_uint = 0x10;
pub const IEEE80211_UHR_MAC_CAP2_TXOP_RET_IN_TXSPG: c_uint = 0x20;
pub const IEEE80211_UHR_MAC_CAP2_UHR_OM_PU_TO_LOW: c_uint = 0xC0;
pub const IEEE80211_UHR_MAC_CAP3_UHR_OM_PU_TO_HIGH: c_uint = 0x03;
pub const IEEE80211_UHR_MAC_CAP3_PARAM_UPD_ADV_NOTIF_INTV: c_uint = 0x1C;
pub const IEEE80211_UHR_MAC_CAP3_UPD_IND_TIM_INTV_LOW: c_uint = 0xE0;
pub const IEEE80211_UHR_MAC_CAP4_UPD_IND_TIM_INTV_HIGH: c_uint = 0x03;
pub const IEEE80211_UHR_MAC_CAP4_BOUNDED_ESS: c_uint = 0x04;
pub const IEEE80211_UHR_MAC_CAP4_BTM_ASSURANCE: c_uint = 0x08;
pub const IEEE80211_UHR_MAC_CAP4_CO_BF_SUPP: c_uint = 0x10;
pub const IEEE80211_UHR_MAC_CAP_DBE_MAX_BW: c_uint = 0x07;
pub const IEEE80211_UHR_MAC_CAP_DBE_EHT_MCS_MAP_160_PRES: c_uint = 0x08;
pub const IEEE80211_UHR_MAC_CAP_DBE_EHT_MCS_MAP_320_PRES: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_cap_dbe {
    pub cap: u8,
// present 0, 1 or 2 times depending on _PRES bits
    pub eht_mcs_map: [ieee80211_eht_mcs_nss_supp_bw; ],
    pub __packed: },
//
// enum ieee80211_uhr_dbe_max_supported_bw - DBE Maximum Supported Bandwidth
//
// As per spec P802.11bn_D1.3 "Table 9-bb5—Encoding of the DBE Maximum
// Supported Bandwidth field".
//
// @IEEE80211_UHR_DBE_MAX_BW_40: Indicates 40 MHz DBE max supported bw
// @IEEE80211_UHR_DBE_MAX_BW_80: Indicates 80 MHz DBE max supported bw
// @IEEE80211_UHR_DBE_MAX_BW_160: Indicates 160 MHz DBE max supported bw
// @IEEE80211_UHR_DBE_MAX_BW_320: Indicates 320 MHz DBE max supported bw
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_dbe_max_supported_bw {
    IEEE80211_UHR_DBE_MAX_BW_40 = 1,
    IEEE80211_UHR_DBE_MAX_BW_80 = 2,
    IEEE80211_UHR_DBE_MAX_BW_160 = 3,
    IEEE80211_UHR_DBE_MAX_BW_320 = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_cap_mac {
    pub mac_cap: [u8; 6],
    pub __packed: },
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_LE80: c_uint = 0x00000001;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_LE80: c_uint = 0x00000002;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_160: c_uint = 0x00000004;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_160: c_uint = 0x00000008;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_SND_NDP_320: c_uint = 0x00000010;
pub const IEEE80211_UHR_PHY_CAP_MAX_NSS_RX_DL_MU_320: c_uint = 0x00000020;
pub const IEEE80211_UHR_PHY_CAP_ELR_TX: c_uint = 0x00000040;
pub const IEEE80211_UHR_PHY_CAP_ELR_RX: c_uint = 0x00000080;
pub const IEEE80211_UHR_PHY_CAP_PART_BW_DL_MUMIMO: c_uint = 0x00000100;
pub const IEEE80211_UHR_PHY_CAP_PART_BW_UL_MUMIMO: c_uint = 0x00000200;
pub const IEEE80211_UHR_PHY_CAP_MCS15: c_uint = 0x00000400;
pub const IEEE80211_UHR_PHY_CAP_2XLDPC_TX: c_uint = 0x00000800;
pub const IEEE80211_UHR_PHY_CAP_2XLDPC_RX: c_uint = 0x00001000;
pub const IEEE80211_UHR_PHY_CAP_UEQM_TX_MAX_NSS: c_uint = 0x00006000;
pub const IEEE80211_UHR_PHY_CAP_UEQM_RX_MAX_NSS: c_uint = 0x00018000;
pub const IEEE80211_UHR_PHY_CAP_CO_BF_JOINT_SOUNDING: c_uint = 0x00040000;
pub const IEEE80211_UHR_PHY_CAP_IM_TX: c_uint = 0x00080000;
pub const IEEE80211_UHR_PHY_CAP_IM_RX: c_uint = 0x00100000;
pub const IEEE80211_UHR_PHY_CAP_CO_SR_MODE_1: c_uint = 0x00200000;
pub const IEEE80211_UHR_PHY_CAP_CO_SR_MODE_2: c_uint = 0x00400000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_20_IN_PBW_20: c_uint = 0x00800000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_40_IN_PBW_40: c_uint = 0x01000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_80_IN_PBW_80: c_uint = 0x02000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_80_IN_PBW_160: c_uint = 0x04000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_80_IN_PBW_320: c_uint = 0x08000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_20_IN_PBW_GE80: c_uint = 0x10000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_40_IN_PBW_GE80: c_uint = 0x20000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_DBW_60_IN_PBW_GE80: c_uint = 0x40000000;
pub const IEEE80211_UHR_PHY_CAP_DRU_RRU_HYBRID_MODE: c_uint = 0x80000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_cap_phy {
    pub cap: __le32,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_cap {
    pub mac: ieee80211_uhr_cap_mac,
    pub phy: ieee80211_uhr_cap_phy,
// optional DBE capabilities
    pub variable: [u8; ],
    pub __packed: },
    pub )data: *const *const ieee80211_uhr_cap cap = (void,
    pub sizeof(*cap): *mut size_t needed =,
    pub false: return,
//
// A non-AP STA does not include the DBE Capability Parameters field
// in the UHR MAC Capabilities Information field.
//
    pub dbe: *const ieee80211_uhr_cap_dbe,
    pub ieee80211_uhr_cap_dbe): needed += sizeof(struct,
    pub false: return,
    pub )cap->variable: *const dbe = (void,
    pub sizeof(dbe->eht_mcs_map[0]): needed +=,
    pub sizeof(dbe->eht_mcs_map[0]): needed +=,
    pub needed: return len >=,
pub const IEEE80211_UHR_OM_PU_TO_128TU: c_int = 11;
//
// ieee80211_uhr_capa_get_om_pu_to_us - get OM parameter update timeout in usec
// @cap: the UHR capability element, size must be validated
//
// Return: the OM parameter update timeout in usec, or -1 if it's not valid
//
    pub timeout: u8,
    pub 2: timeout <<=,
    pub -1: return,
    pub 0: return,
    pub 1): return 128 << (timeout -,
// only valid from AP, must check ieee80211_uhr_capa_size_ok(..., true)
    pub NULL: return,
    pub )cap->variable: *const return (void,
pub const IEEE80211_SMD_INFO_CAPA_DL_DATA_FWD: c_uint = 0x01;
pub const IEEE80211_SMD_INFO_CAPA_MAX_NUM_PREP: c_uint = 0x0E;
pub const IEEE80211_SMD_INFO_CAPA_TYPE: c_uint = 0x10;
pub const IEEE80211_SMD_INFO_CAPA_PTK_PER_AP_MLD: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_smd_info {
    pub id: [u8; ETH_ALEN],
    pub capa: u8,
    pub timeout: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_protected_uhr_action {
    IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_REQUEST	= 0,
    IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_RESPONSE	= 1,
    IEEE80211_PROTECTED_UHR_ACTION_LINK_RECONFIG_NOTIFY	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_link_reconfig_request_type {
    IEEE80211_UHR_LINK_RECONFIG_REQUEST_ST_PREP		= 0,
    IEEE80211_UHR_LINK_RECONFIG_REQUEST_ST_EXEC		= 1,
    IEEE80211_UHR_LINK_RECONFIG_REQUEST_OMP_REQUEST		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_link_reconfig_response_type {
    IEEE80211_UHR_LINK_RECONFIG_RESPONSE_ST_PREP		= 0,
    IEEE80211_UHR_LINK_RECONFIG_RESPONSE_ST_EXEC		= 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_link_reconfig_notify_type {
    IEEE80211_UHR_LINK_RECONFIG_NOTIFY_DL_DRAINED		= 2,
    IEEE80211_UHR_LINK_RECONFIG_NOTIFY_OMP_RESPONSE		= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_mode_change_control {
    IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ID		= 0x003f,
    IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_ENABLE		= 0x0040,
    IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_UPDATE		= 0x0080,
    IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_LENGTH		= 0x0f00,
    IEEE80211_UHR_MODE_CHANGE_CONTROL_MODE_SPECIFIC		= 0xf000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ieee80211_uhr_mode_change_mode_id {
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_DPS			= 0,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_NPCA			= 1,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_DUO			= 2,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_DSO			= 3,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_P_EDCA		= 4,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_ELR_RX		= 5,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_AOM			= 6,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_LLI			= 7,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_CO_BF			= 8,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_CO_SR			= 9,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_EMLSR			= 10,
    IEEE80211_UHR_MODE_CHANGE_MODE_ID_DBE			= 11,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ieee80211_uhr_mode_change_tuple {
    pub control: __le16,
    pub variable: [u8; ],
    pub __packed: },

    pub \: *const *const for (tuple = (void )(data);,
    pub \: ieee80211_uhr_mode_change_tuple_size(tuple);,
