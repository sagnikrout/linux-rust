//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76_connac_mcu.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2020 MediaTek Inc.

pub const PATCH_SEC_TYPE_INFO: c_uint = 0x2;

pub const PATCH_SEC_ENC_TYPE_PLAIN: c_uint = 0x00;
pub const PATCH_SEC_ENC_TYPE_AES: c_uint = 0x01;
pub const PATCH_SEC_ENC_TYPE_SCRAMBLE: c_uint = 0x02;

pub const SEC_TYPE_SUBSYS_SEC_IMG_SIGN: c_uint = 0x05;

pub const MCU_PKT_ID: c_uint = 0xa0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_mcu_txd {
    pub txd: [__le32; 8],
    pub len: __le16,
    pub pq_id: __le16,
    pub cid: u8,
    pub pkt_type: u8,
    pub /: *mut *mut u8 set_query; / FW don't care,
    pub seq: u8,
    pub uc_d2b0_rev: u8,
    pub ext_cid: u8,
    pub s2d_index: u8,
    pub ext_cid_ack: u8,
    pub rsv: [u32; 5],
    pub __aligned(4): } __packed,
//
// struct mt76_connac2_mcu_uni_txd - mcu command descriptor for connac2 and connac3
// @txd: hardware descriptor
// @len: total length not including txd
// @cid: command identifier
// @pkt_type: must be 0xa0 (cmd packet by long format)
// @frag_n: fragment number
// @seq: sequence number
// @checksum: 0 mean there is no checksum
// @s2d_index: index for command source and destination
// Definition              | value | note
// CMD_S2D_IDX_H2N         | 0x00  | command from HOST to WM
// CMD_S2D_IDX_C2N         | 0x01  | command from WA to WM
// CMD_S2D_IDX_H2C         | 0x02  | command from HOST to WA
// CMD_S2D_IDX_H2N_AND_H2C | 0x03  | command from HOST to WA and WM
//
// @option: command option
// BIT[0]: UNI_CMD_OPT_BIT_ACK
// set to 1 to request a fw reply
// if UNI_CMD_OPT_BIT_0_ACK is set and UNI_CMD_OPT_BIT_2_SET_QUERY
// is set, mcu firmware will send response event EID = 0x01
// (UNI_EVENT_ID_CMD_RESULT) to the host.
// BIT[1]: UNI_CMD_OPT_BIT_UNI_CMD
// 0: original command
// 1: unified command
// BIT[2]: UNI_CMD_OPT_BIT_SET_QUERY
// 0: QUERY command
// 1: SET command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_mcu_uni_txd {
    pub txd: [__le32; 8],
// DW1
    pub len: __le16,
    pub cid: __le16,
// DW2
    pub rsv: u8,
    pub pkt_type: u8,
    pub frag_n: u8,
    pub seq: u8,
// DW3
    pub checksum: __le16,
    pub s2d_index: u8,
    pub option: u8,
// DW4
    pub rsv1: [u8; 4],
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_mcu_rxd {
// New members MUST be added within the struct_group() macro below.
    pub rxd: [__le32; 6],
    pub len: __le16,
    pub pkt_type_id: __le16,
    pub eid: u8,
    pub seq: u8,
    pub option: u8,
    pub rsv: u8,
    pub ext_eid: u8,
    pub rsv1: [u8; 2],
    pub s2d_index: u8,
    pub tlv: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_patch_hdr {
    pub build_date: [c_char; 16],
    pub platform: [c_char; 4],
    pub hw_sw_ver: __be32,
    pub patch_ver: __be32,
    pub checksum: __be16,
    pub rsv: u16,
    pub patch_ver: __be32,
    pub subsys: __be32,
    pub feature: __be32,
    pub n_region: __be32,
    pub crc: __be32,
    pub rsv: [u32; 11],
    pub desc: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_patch_sec {
    pub type: __be32,
    pub offs: __be32,
    pub size: __be32,
    pub spec: [__be32; 13],
    pub addr: __be32,
    pub len: __be32,
    pub sec_key_idx: __be32,
    pub align_len: __be32,
    pub rsv: [u32; 9],
    pub info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_fw_trailer {
    pub chip_id: u8,
    pub eco_code: u8,
    pub n_region: u8,
    pub format_ver: u8,
    pub format_flag: u8,
    pub rsv: [u8; 2],
    pub fw_ver: [c_char; 10],
    pub build_date: [c_char; 15],
    pub crc: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac2_fw_region {
    pub decomp_crc: __le32,
    pub decomp_len: __le32,
    pub decomp_blk_sz: __le32,
    pub rsv: [u8; 4],
    pub addr: __le32,
    pub len: __le32,
    pub feature_set: u8,
    pub type: u8,
    pub rsv1: [u8; 14],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac3_multi_header_v2_sec_raw_format {
    pub type: __le32,
    pub offset: __le32,
    pub size: __le32,
    pub spec: [u8; 52],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac3_multi_header_v2_raw_format {
    pub pack_time: [u8; 20],
    pub chip_id_eco_ver: [u8; 8],
    pub patch_ver: __le32,
    pub global_desc_head: [u8; 4],
    pub global_subsys: __le32,
    pub rsv2: [u8; 4],
    pub sec_num: __le32,
    pub rsv3: [u8; 48],
    pub sects: [mt76_connac3_multi_header_v2_sec_raw_format; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlv {
    pub tag: __le16,
    pub len: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_omac {
    pub tag: __le16,
    pub len: __le16,
    pub hw_bss_idx: u8,
    pub omac_idx: u8,
    pub band_idx: u8,
    pub rsv0: u8,
    pub conn_type: __le32,
    pub rsv1: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_basic {
    pub tag: __le16,
    pub len: __le16,
    pub network_type: __le32,
    pub active: u8,
    pub rsv0: u8,
    pub bcn_interval: __le16,
    pub bssid: [u8; ETH_ALEN],
    pub wmm_idx: u8,
    pub dtim_period: u8,
    pub bmc_wcid_lo: u8,
    pub cipher: u8,
    pub phy_mode: u8,
    pub /: *mut *mut u8 max_bssid; / max BSSID. range: 1 ~ 8, 0: MBSSID disabled,
    pub /: *mut *mut u8 non_tx_bssid;/ non-transmitted BSSID, 0: transmitted BSSID,
    pub /: *mut *mut u8 bmc_wcid_hi; / high Byte and version,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_rf_ch {
    pub tag: __le16,
    pub len: __le16,
    pub pri_ch: u8,
    pub center_ch0: u8,
    pub center_ch1: u8,
    pub bw: u8,
    pub /: *mut *mut u8 he_ru26_block; / 1: don't send HETB in RU26, 0: allow,
    pub /: *mut *mut u8 he_all_disable; / 1: disallow all HETB, 0: allow,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_ext_bss {
    pub tag: __le16,
    pub len: __le16,
    pub /: *mut *mut __le32 mbss_tsf_offset; / in unit of us,
    pub rsv: [u8; 8],
    pub __packed: },
}

// sta_rec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_ntlv_hdr {
    pub rsv: [u8; 2],
    pub tlv_num: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_req_hdr {
    pub bss_idx: u8,
    pub wlan_idx_lo: u8,
    pub tlv_num: __le16,
    pub is_tlv_append: u8,
    pub muar_idx: u8,
    pub wlan_idx_hi: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_basic {
    pub tag: __le16,
    pub len: __le16,
    pub conn_type: __le32,
    pub conn_state: u8,
    pub qos: u8,
    pub aid: __le16,
    pub peer_addr: [u8; ETH_ALEN],
    pub extra_info: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ht {
    pub tag: __le16,
    pub len: __le16,
    pub ht_cap: __le16,
    pub rsv: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_vht {
    pub tag: __le16,
    pub len: __le16,
    pub vht_cap: __le32,
    pub vht_rx_mcs_map: __le16,
    pub vht_tx_mcs_map: __le16,
// mt7915 - mt7921
    pub rts_bw_sig: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_uapsd {
    pub tag: __le16,
    pub len: __le16,
    pub dac_map: u8,
    pub tac_map: u8,
    pub max_sp: u8,
    pub rsv0: u8,
    pub listen_interval: __le16,
    pub rsv1: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ba {
    pub tag: __le16,
    pub len: __le16,
    pub tid: u8,
    pub ba_type: u8,
    pub amsdu: u8,
    pub ba_en: u8,
    pub ssn: __le16,
    pub winsize: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_he {
    pub tag: __le16,
    pub len: __le16,
    pub he_cap: __le32,
    pub t_frame_dur: u8,
    pub max_ampdu_exp: u8,
    pub bw_set: u8,
    pub device_class: u8,
    pub dcm_tx_mode: u8,
    pub dcm_tx_max_nss: u8,
    pub dcm_rx_mode: u8,
    pub dcm_rx_max_nss: u8,
    pub dcm_max_ru: u8,
    pub punc_pream_rx: u8,
    pub pkt_ext: u8,
    pub rsv1: u8,
    pub max_nss_mcs: [__le16; CMD_HE_MCS_BW_NUM],
    pub rsv2: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_he_v2 {
    pub tag: __le16,
    pub len: __le16,
    pub he_mac_cap: [u8; 6],
    pub he_phy_cap: [u8; 11],
    pub pkt_ext: u8,
// 0: BW80, 1: BW160, 2: BW8080
    pub max_nss_mcs: [__le16; CMD_HE_MCS_BW_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_amsdu {
    pub tag: __le16,
    pub len: __le16,
    pub max_amsdu_num: u8,
    pub max_mpdu_size: u8,
    pub amsdu_en: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_state {
    pub tag: __le16,
    pub len: __le16,
    pub flags: __le32,
    pub state: u8,
    pub vht_opmode: u8,
    pub action: u8,
    pub rsv: [u8; 1],
    pub __packed: },

pub const HT_MCS_MASK_NUM: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ra_info {
    pub tag: __le16,
    pub len: __le16,
    pub legacy: __le16,
    pub rx_mcs_bitmask: [u8; HT_MCS_MASK_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_phy {
    pub tag: __le16,
    pub len: __le16,
    pub basic_rate: __le16,
    pub phy_type: u8,
    pub ampdu: u8,
    pub rts_policy: u8,
    pub rcpi: u8,
    pub /: *mut *mut u8 max_ampdu_len; / connac3,
    pub rsv: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_he_6g_capa {
    pub tag: __le16,
    pub len: __le16,
    pub capa: __le16,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_pn_info {
    pub tag: __le16,
    pub len: __le16,
    pub pn: [u8; 6],
    pub tsc_type: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_key {
    pub cipher_id: u8,
    pub cipher_len: u8,
    pub key_id: u8,
    pub key_len: u8,
    pub key: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_sec {
    pub tag: __le16,
    pub len: __le16,
    pub add: u8,
    pub n_cipher: u8,
    pub rsv: [u8; 2],
    pub key: [sec_key; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_bf {
    pub tag: __le16,
    pub len: __le16,
    pub /: *mut *mut __le16 pfmu; / 0xffff: no access right for PFMU,
    pub /: *mut *mut bool su_mu; / 0: SU, 1: MU,
    pub /: *mut *mut u8 bf_cap; / 0: iBF, 1: eBF,
    pub /: *mut *mut u8 sounding_phy; / 0: legacy, 1: OFDM, 2: HT, 4: VHT,
    pub ndpa_rate: u8,
    pub ndp_rate: u8,
    pub rept_poll_rate: u8,
    pub /: *mut *mut u8 tx_mode; / 0: legacy, 1: OFDM, 2: HT, 4: VHT ...,
    pub ncol: u8,
    pub nrow: u8,
    pub /: *mut *mut u8 bw; / 0: 20M, 1: 40M, 2: 80M, 3: 160M,
    pub mem_total: u8,
    pub mem_20m: u8,
    pub row: u8,
    pub 2: u8 col: 6, row_msb:,
    pub mem: [}; 4],
    pub smart_ant: __le16,
    pub se_idx: u8,
    pub indicator: *mut *mut u8 auto_sounding; / b7: low traffic,
// b6: Stop sounding for this entry
// b5 ~ b0: postpone sounding
//
    pub ibf_timeout: u8,
    pub ibf_dbw: u8,
    pub ibf_ncol: u8,
    pub ibf_nrow: u8,
    pub nrow_gt_bw80: u8,
    pub ncol_gt_bw80: u8,
    pub ru_start_idx: u8,
    pub ru_end_idx: u8,
    pub trigger_su: bool,
    pub trigger_mu: bool,
    pub ng16_su: bool,
    pub ng16_mu: bool,
    pub codebook42_su: bool,
    pub codebook75_mu: bool,
    pub he_ltf: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_bfee {
    pub tag: __le16,
    pub len: __le16,
    pub /: *mut *mut bool fb_identity_matrix; / 1: feedback identity matrix,
    pub /: *mut *mut bool ignore_feedback; / 1: ignore,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_muru {
    pub tag: __le16,
    pub len: __le16,
    pub ofdma_dl_en: bool,
    pub ofdma_ul_en: bool,
    pub mimo_dl_en: bool,
    pub mimo_ul_en: bool,
    pub rsv: [u8; 4],
    pub cfg: },
    pub punc_pream_rx: u8,
    pub he_20m_in_40m_2g: bool,
    pub he_20m_in_160m: bool,
    pub he_80m_in_160m: bool,
    pub lt16_sigb: bool,
    pub rx_su_comp_sigb: bool,
    pub rx_su_non_comp_sigb: bool,
    pub rsv: u8,
    pub ofdma_dl: },
    pub t_frame_dur: u8,
    pub mu_cascading: u8,
    pub uo_ra: u8,
    pub he_2x996_tone: u8,
    pub rx_t_frame_11ac: u8,
    pub rx_ctrl_frame_to_mbss: u8,
    pub rsv: [u8; 2],
    pub ofdma_ul: },
    pub vht_mu_bfee: bool,
    pub partial_bw_dl_mimo: bool,
    pub rsv: [u8; 2],
    pub mimo_dl: },
    pub full_ul_mimo: bool,
    pub partial_ul_mimo: bool,
    pub rsv: [u8; 2],
    pub mimo_ul: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_remove {
    pub tag: __le16,
    pub len: __le16,
    pub action: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_phy {
    pub type: u8,
    pub flag: u8,
    pub stbc: u8,
    pub sgi: u8,
    pub bw: u8,
    pub ldpc: u8,
    pub mcs: u8,
    pub nss: u8,
    pub he_ltf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ra {
    pub tag: __le16,
    pub len: __le16,
    pub valid: u8,
    pub auto_rate: u8,
    pub phy_mode: u8,
    pub channel: u8,
    pub bw: u8,
    pub disable_cck: u8,
    pub ht_mcs32: u8,
    pub ht_gf: u8,
    pub ht_mcs: [u8; 4],
    pub mmps_mode: u8,
    pub gband_256: u8,
    pub af: u8,
    pub auth_wapi_mode: u8,
    pub rate_len: u8,
    pub supp_mode: u8,
    pub supp_cck_rate: u8,
    pub supp_ofdm_rate: u8,
    pub supp_ht_mcs: __le32,
    pub supp_vht_mcs: [__le16; 4],
    pub op_mode: u8,
    pub op_vht_chan_width: u8,
    pub op_vht_rx_nss: u8,
    pub op_vht_rx_nss_type: u8,
    pub sta_cap: __le32,
    pub phy: sta_phy,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_ra_fixed {
    pub tag: __le16,
    pub len: __le16,
    pub field: __le32,
    pub op_mode: u8,
    pub op_vht_chan_width: u8,
    pub op_vht_rx_nss: u8,
    pub op_vht_rx_nss_type: u8,
    pub phy: sta_phy,
    pub spe_idx: u8,
    pub short_preamble: u8,
    pub is_5g: u8,
    pub mmps_mode: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_tx_proc {
    pub tag: __le16,
    pub len: __le16,
    pub flag: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_rec_eml_op {
    pub tag: __le16,
    pub len: __le16,
    pub link_bitmap: u8,
    pub link_ant_num: [u8; 3],
    pub __packed: },
// wtbl_rec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_req_hdr {
    pub wlan_idx_lo: u8,
    pub operation: u8,
    pub tlv_num: __le16,
    pub wlan_idx_hi: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_generic {
    pub tag: __le16,
    pub len: __le16,
    pub peer_addr: [u8; ETH_ALEN],
    pub muar_idx: u8,
    pub skip_tx: u8,
    pub cf_ack: u8,
    pub qos: u8,
    pub mesh: u8,
    pub adm: u8,
    pub partial_aid: __le16,
    pub baf_en: u8,
    pub aad_om: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_rx {
    pub tag: __le16,
    pub len: __le16,
    pub rcid: u8,
    pub rca1: u8,
    pub rca2: u8,
    pub rv: u8,
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_ht {
    pub tag: __le16,
    pub len: __le16,
    pub ht: u8,
    pub ldpc: u8,
    pub af: u8,
    pub mm: u8,
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_vht {
    pub tag: __le16,
    pub len: __le16,
    pub ldpc: u8,
    pub dyn_bw: u8,
    pub vht: u8,
    pub txop_ps: u8,
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_tx_ps {
    pub tag: __le16,
    pub len: __le16,
    pub txps: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_hdr_trans {
    pub tag: __le16,
    pub len: __le16,
    pub to_ds: u8,
    pub from_ds: u8,
    pub no_rx_trans: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_ba {
    pub tag: __le16,
    pub len: __le16,
// common
    pub tid: u8,
    pub ba_type: u8,
    pub rsv0: [u8; 2],
// originator only
    pub sn: __le16,
    pub ba_en: u8,
    pub ba_winsize_idx: u8,
// originator & recipient
    pub ba_winsize: __le16,
// recipient only
    pub peer_addr: [u8; ETH_ALEN],
    pub rst_ba_tid: u8,
    pub rst_ba_sel: u8,
    pub rst_ba_sb: u8,
    pub band_idx: u8,
    pub rsv1: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_smps {
    pub tag: __le16,
    pub len: __le16,
    pub smps: u8,
    pub rsv: [u8; 3],
    pub __packed: },
// mt7615 only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_bf {
    pub tag: __le16,
    pub len: __le16,
    pub ibf: u8,
    pub ebf: u8,
    pub ibf_vht: u8,
    pub ebf_vht: u8,
    pub gid: u8,
    pub pfmu_idx: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_pn {
    pub tag: __le16,
    pub len: __le16,
    pub pn: [u8; 6],
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_spe {
    pub tag: __le16,
    pub len: __le16,
    pub spe_idx: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wtbl_raw {
    pub tag: __le16,
    pub len: __le16,
    pub wtbl_idx: u8,
    pub dw: u8,
    pub rsv: [u8; 2],
    pub msk: __le32,
    pub val: __le32,
    pub __packed: },

}

pub const CONN_STATE_DISCONNECT: c_int = 0;
pub const CONN_STATE_CONNECT: c_int = 1;
pub const CONN_STATE_PORT_SECURE: c_int = 2;
// HE MAC

// HE PHY

// STBC

// GI

// 242 TONE

// event table
// ext event table
// unified event table

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_cipher_type {
    MCU_CIPHER_NONE = 0,
    MCU_CIPHER_WEP40,
    MCU_CIPHER_WEP104,
    MCU_CIPHER_WEP128,
    MCU_CIPHER_TKIP,
    MCU_CIPHER_AES_CCMP,
    MCU_CIPHER_CCMP_256,
    MCU_CIPHER_GCMP,
    MCU_CIPHER_GCMP_256,
    MCU_CIPHER_WAPI,
    MCU_CIPHER_BIP_CMAC_128,
    MCU_CIPHER_BIP_CMAC_256,
    MCU_CIPHER_BCN_PROT_CMAC_128,
    MCU_CIPHER_BCN_PROT_CMAC_256,
    MCU_CIPHER_BCN_PROT_GMAC_128,
    MCU_CIPHER_BCN_PROT_GMAC_256,
    MCU_CIPHER_BIP_GMAC_128,
    MCU_CIPHER_BIP_GMAC_256,
}

// offload mcu commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UNI_ALL_STA_INFO_TAG {
    UNI_ALL_STA_TXRX_RATE,
    UNI_ALL_STA_TX_STAT,
    UNI_ALL_STA_TXRX_ADM_STAT,
    UNI_ALL_STA_TXRX_AIR_TIME,
    UNI_ALL_STA_DATA_TX_RETRY_COUNT,
    UNI_ALL_STA_GI_MODE,
    UNI_ALL_STA_TXRX_MSDU_COUNT,
    UNI_ALL_STA_MAX_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_bss_basic_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub active: u8,
    pub omac_idx: u8,
    pub hw_bss_idx: u8,
    pub band_idx: u8,
    pub conn_type: __le32,
    pub conn_state: u8,
    pub wmm_idx: u8,
    pub bssid: [u8; ETH_ALEN],
    pub bmc_tx_wlan_idx: __le16,
    pub bcn_interval: __le16,
    pub dtim_period: u8,
    pub A: *mut *mut u8 phymode; / bit(0):,
// bit(1): B
// bit(2): G
// bit(3): GN
// bit(4): AN
// bit(5): AC
// bit(6): AX2
// bit(7): AX5
// bit(8): AX6
//
    pub sta_idx: __le16,
    pub nonht_basic_phy: __le16,
    pub /: *mut *mut u8 phymode_ext; / bit(0) AX_6G,
    pub link_idx: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_bss_qos_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub qos: u8,
    pub pad: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_beacon_loss_event {
    pub bss_idx: u8,
    pub reason: u8,
    pub pad: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_rssi_notify_event {
    pub rssi: [__le32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_bss_event {
    pub bss_idx: u8,
    pub is_absent: u8,
    pub free_quota: u8,
    pub pad: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_scan_ssid {
    pub ssid_len: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_scan_channel {
    pub 2.4GHz: *mut *mut u8 band; / 1:,
// 2: 5.0GHz
// Others: Reserved
//
    pub channel_num: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_scan_match {
    pub rssi_th: __le32,
    pub ssid: [u8; IEEE80211_MAX_SSID_LEN],
    pub ssid_len: u8,
    pub rsv: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_hw_scan_req {
    pub seq_num: u8,
    pub bss_idx: u8,
    pub SCAN: *mut *mut u8 scan_type; / 0: PASSIVE,
// 1: ACTIVE SCAN
//
    pub SSID: *mut *mut u8 ssid_type; / BIT(0) wildcard,
// BIT(1) P2P wildcard SSID
// BIT(2) specified SSID + wildcard SSID
// BIT(2) + ssid_type_ext BIT(0) specified SSID only
//
    pub ssids_num: u8,
    pub /: *mut *mut u8 probe_req_num; / Number of probe request for each SSID,
    pub scan: *mut *mut u8 scan_func; / BIT(0) Enable random MAC,
// BIT(1) Disable DBDC scan type 1~3.
// BIT(2) Use DBDC scan type 3 (dedicated one RF to scan).
//
    pub ies.: *mut *mut u8 version; / 0: Not support fields after,
// 1: Support fields after ies.
//
    pub ssids: [mt76_connac_mcu_scan_ssid; 4],
    pub probe_delay_time: __le16,
    pub /: *mut *mut __le16 channel_dwell_time; / channel Dwell interval,
    pub timeout_value: __le16,
    pub channels: *mut *mut u8 channel_type; / 0: Full,
// 1: Only 2.4GHz channels
// 2: Only 5GHz channels
// 3: P2P social channel only (channel #1, #6 and #11)
// 4: Specified channels
// Others: Reserved
//
    pub /: *mut *mut u8 channels_num; / valid when channel_type is 4,
// valid when channels_num is set
    pub channels: [mt76_connac_mcu_scan_channel; 32],
    pub ies_len: __le16,
    pub ies: [u8; MT76_CONNAC_SCAN_IE_LEN],
// following fields are valid if version > 0
    pub ext_channels_num: u8,
    pub ext_ssids_num: u8,
    pub channel_min_dwell_time: __le16,
    pub ext_channels: [mt76_connac_mcu_scan_channel; 32],
    pub ext_ssids: [mt76_connac_mcu_scan_ssid; 6],
    pub bssid: [u8; ETH_ALEN],
    pub /: *mut *mut u8 random_mac[ETH_ALEN]; / valid when BIT(1) in scan_func is set.,
    pub pad: [u8; 63],
    pub ssid_type_ext: u8,
    pub __packed: },
pub const MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_hw_scan_done {
    pub seq_num: u8,
    pub sparse_channel_num: u8,
    pub sparse_channel: mt76_connac_mcu_scan_channel,
    pub complete_channel_num: u8,
    pub current_state: u8,
    pub version: u8,
    pub pad: u8,
    pub beacon_scan_num: __le32,
    pub pno_enabled: u8,
    pub pad2: [u8; 3],
    pub sparse_channel_valid_num: u8,
    pub alpha2: [u8; 3],
    pub channel_num: [u8; MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM],
// idle format for channel_idle_time
// 0: first bytes: idle time(ms) 2nd byte: dwell time(ms)
// 1: first bytes: idle time(8ms) 2nd byte: dwell time(8ms)
// 2: dwell time (16us)
//
    pub channel_idle_time: [__le16; MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM],
// beacon and probe response count
    pub beacon_probe_num: [u8; MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM],
    pub mdrdy_count: [u8; MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM],
    pub beacon_2g_num: __le32,
    pub beacon_5g_num: __le32,
    pub channel_scan_time: [__le16; MT76_CONNAC_SCAN_DONE_EVENT_MAX_CHANNEL_NUM],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_sched_scan_req {
    pub version: u8,
    pub seq_num: u8,
    pub stop_on_match: u8,
    pub ssids_num: u8,
    pub match_num: u8,
    pub pad: u8,
    pub ie_len: __le16,
    pub ssids: [mt76_connac_mcu_scan_ssid; MT76_CONNAC_MAX_SCHED_SCAN_SSID],
    pub match: [mt76_connac_mcu_scan_match; MT76_CONNAC_MAX_SCAN_MATCH],
    pub channel_type: u8,
    pub channels_num: u8,
    pub intervals_num: u8,
    pub /: *mut *mut u8 scan_func; / MT7663: BIT(0) eable random mac address,
    pub channels: [mt76_connac_mcu_scan_channel; 64],
    pub intervals: [__le16; MT76_CONNAC_MAX_NUM_SCHED_SCAN_INTERVAL],
    pub random_mac: [u8; ETH_ALEN],
    pub pad2: [u8; 58],
    pub mt7663: },
    pub bss_idx: u8,
    pub pad1: [u8; 3],
    pub delay: __le32,
    pub pad2: [u8; 12],
    pub random_mac: [u8; ETH_ALEN],
    pub pad3: [u8; 38],
    pub mt7921: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_sched_scan_done {
    pub seq_num: u8,
    pub /: *mut *mut u8 status; / 0: ssid found,
    pub pad: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_uni_bss_color {
    pub tag: __le16,
    pub len: __le16,
    pub enable: u8,
    pub bss_color: u8,
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_uni_he {
    pub tag: __le16,
    pub len: __le16,
    pub he_rts_thres: __le16,
    pub he_pe_duration: u8,
    pub su_disable: u8,
    pub max_nss_mcs: [__le16; CMD_HE_MCS_BW_NUM],
    pub rsv: [u8; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bss_info_uni_mbssid {
    pub tag: __le16,
    pub len: __le16,
    pub max_indicator: u8,
    pub mbss_idx: u8,
    pub tx_bss_omac_idx: u8,
    pub rsv: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_gtk_rekey_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub kek: [u8; NL80211_KEK_LEN],
    pub kck: [u8; NL80211_KCK_LEN],
    pub replay_ctr: [u8; NL80211_REPLAY_CTR_LEN],
    pub enable: *mut *mut u8 rekey_mode; / 0: rekey offload,
// 1: rekey offload disable
// 2: rekey update
//
    pub keyid: u8,
    pub /: *mut *mut u8 option; / 1: rekey data update without enabling offload,
    pub pad: [u8; 1],
    pub /: *mut *mut __le32 proto; / WPA-RSN-WAPI-OPSN,
    pub pairwise_cipher: __le32,
    pub group_cipher: __le32,
    pub /: *mut *mut __le32 key_mgmt; / NONE-PSK-IEEE802.1X,
    pub mgmt_group_cipher: __le32,
    pub reserverd: [u8; 4],
    pub __packed: },
pub const MT76_CONNAC_WOW_MASK_MAX_LEN: c_int = 16;
pub const MT76_CONNAC_WOW_PATTEN_MAX_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_wow_pattern_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub /: *mut *mut u8 index; / pattern index,
    pub disable: *mut *mut u8 enable; / 0:,
// 1: enable
//
    pub /: *mut *mut u8 data_len; / pattern length,
    pub pad: u8,
    pub mask: [u8; MT76_CONNAC_WOW_MASK_MAX_LEN],
    pub pattern: [u8; MT76_CONNAC_WOW_PATTEN_MAX_LEN],
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_wow_ctrl_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub PM_WOWLAN_REQ_START: *mut *mut u8 cmd; / 0x1:,
// 0x2: PM_WOWLAN_REQ_STOP
// 0x3: PM_WOWLAN_PARAM_CLEAR
//
    pub NONE: *mut *mut u8 trigger; / 0:,
// BIT(0): NL80211_WOWLAN_TRIG_MAGIC_PKT
// BIT(1): NL80211_WOWLAN_TRIG_ANY
// BIT(2): NL80211_WOWLAN_TRIG_DISCONNECT
// BIT(3): NL80211_WOWLAN_TRIG_GTK_REKEY_FAILURE
// BIT(4): BEACON_LOST
// BIT(5): NL80211_WOWLAN_TRIG_NET_DETECT
//
    pub HIF_SDIO: *mut *mut u8 wakeup_hif; / 0x0:,
// 0x1: HIF_USB
// 0x2: HIF_PCIE
// 0x3: HIF_GPIO
//
    pub pad: u8,
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_wow_gpio_param_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub gpio_pin: u8,
    pub trigger_lvl: u8,
    pub pad: [u8; 2],
    pub gpio_interval: __le32,
    pub rsv: [u8; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_arpns_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub mode: u8,
    pub ips_num: u8,
    pub option: u8,
    pub pad: [u8; 1],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_suspend_tlv {
    pub tag: __le16,
    pub len: __le16,
    pub disabled: *mut *mut u8 enable; / 0: suspend mode,
// 1: suspend mode enabled
//
    pub /: *mut *mut u8 mdtim; / LP parameter,
    pub policy: *mut *mut u8 wow_suspend; / 0: update by origin,
// 1: update by wow dtim
//
    pub pad: [u8; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76_sta_info_state {
    MT76_STA_INFO_STATE_NONE,
    MT76_STA_INFO_STATE_AUTH,
    MT76_STA_INFO_STATE_ASSOC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_sta_cmd_info {
    pub sta: *mut ieee80211_sta,
    pub link_sta: *mut ieee80211_link_sta,
}

pub const MT_SKU_POWER_LIMIT: c_int = 161;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_sku_tlv {
    pub channel: u8,
    pub pwr_limit: [i8; MT_SKU_POWER_LIMIT],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_tx_power_limit_tlv {
// DW0 - common info
    pub ver: u8,
    pub pad0: u8,
    pub len: __le16,
// DW1 - cmd hint
    pub /: *mut *mut u8 n_chan; / # channel,
    pub /: *mut *mut u8 band; / 2.4GHz - 5GHz - 6GHz,
    pub last_msg: u8,
    pub pad1: u8,
// DW3
    pub /: *mut *mut u8 alpha2[4]; / regulatory_request.alpha2,
    pub pad2: [u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_config {
    pub id: __le16,
    pub type: u8,
    pub resp_type: u8,
    pub data_size: __le16,
    pub resv: __le16,
    pub data: [u8; 320],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_uni_event {
    pub cid: u8,
    pub pad: [u8; 3],
    pub /: *mut *mut __le32 status; / 0: success, others: fail,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76_connac_mcu_reg_event {
    pub reg: __le32,
    pub val: __le32,
    pub __packed: },
    pub MCU_CIPHER_WEP40: return,
    pub MCU_CIPHER_WEP104: return,
    pub MCU_CIPHER_TKIP: return,
    pub MCU_CIPHER_BIP_CMAC_128: return,
    pub MCU_CIPHER_AES_CCMP: return,
    pub MCU_CIPHER_CCMP_256: return,
    pub MCU_CIPHER_GCMP: return,
    pub MCU_CIPHER_GCMP_256: return,
    pub MCU_CIPHER_BIP_GMAC_128: return,
    pub MCU_CIPHER_BIP_GMAC_256: return,
    pub MCU_CIPHER_BIP_CMAC_256: return,
    pub MCU_CIPHER_WAPI: return,
    pub MCU_CIPHER_NONE: return,
    pub 0: u32 ret =,
    pub 0: DL_MODE_ENCRYPT | DL_MODE_RESET_SEC_IV :,
    pub 0: DL_CONFIG_ENCRY_MODE_SEL :,
    pub feature_set)): FIELD_GET(FW_FEATURE_SET_KEY_IDX,,
    pub DL_MODE_NEED_RSP: ret |=,
    pub 0: ret |= is_wa ? DL_MODE_WORKING_PDA_CR4 :,
    pub ret: return,

// wlan_idx_hi = 0;
// wlan_idx_lo = wcid ? to_wcid_lo(wcid->idx) : 0;
// wlan_idx_hi = wcid ? to_wcid_hi(wcid->idx) : 0;
// wlan_idx_lo = wcid ? wcid->idx : 0;
pub const MT76_CONNAC_MCU_STATUS_WLAN_FAILURE: c_uint = 0xc0000001;
    pub err: return,
// Ignore wlan_failure state false alarm when deactivating an
// inactive network. It does not harm the firmware state.
//
    pub deactivated\n"): "ignore wlan_failure when bss is,
    pub 0: return,
    pub activated\n"): "wlan_failure when bss is,
    pub err: return,
    pub len): *mut *mut mt76_wcid wcid, int,
    pub skb): *mut *mut int cmd, void sta_wtbl, struct sk_buff,
    pub sta_wtbl): *mut c_void,
    pub NULL): return mt76_connac_mcu_add_nested_tlv(skb, tag, len, skb->data,,
    pub phy): *mut int mt76_connac_mcu_set_channel_domain(struct mt76_phy,
    pub vif): *mut *mut int mt76_connac_mcu_set_vif_ps(struct mt76_dev dev, struct ieee80211_vif,
    pub newly): int state, bool,
    pub wtbl_tlv): *mut c_void,
    pub wtbl_tlv): *mut *mut void sta_wtbl, void,
    pub cmd): *mut *mut mt76_wcid wcid, int,
    pub sta): *mut *mut void mt76_connac_mcu_sta_he_tlv_v2(struct sk_buff skb, struct ieee80211_sta,
    pub link_sta): *mut ieee80211_link_sta,
    pub sta): *mut ieee80211_sta,
    pub state): u8 rcpi, u8,
    pub vht_ldpc): *mut *mut void wtbl_tlv, bool ht_ldpc, bool,
    pub wtbl_tlv): *mut c_void,
    pub tx): bool enable, bool,
    pub enable): bool,
    pub tx): int cmd, bool enable, bool,
    pub ctx): *mut ieee80211_chanctx_conf,
    pub ctx): *mut ieee80211_chanctx_conf,
    pub info): *mut mt76_sta_cmd_info,
    pub vif): *mut ieee80211_vif,
    pub band): *mut *mut int mt76_connac_mcu_set_rts_thresh(struct mt76_dev dev, u32 val, u8,
    pub hdr_trans): bool,
    pub mode): u32,
    pub dev): *mut int mt76_connac_mcu_start_patch(struct mt76_dev,
    pub get): *mut *mut int mt76_connac_mcu_patch_sem_ctrl(struct mt76_dev dev, bool,
    pub len): *mut *mut int mt76_connac_cb_mcu_init_download(struct mt76_dev dev, u32,
    pub dev): *mut int mt76_connac_cb_mcu_start_patch(struct mt76_dev,
    pub get): *mut *mut int mt76_connac_cb_mcu_patch_sem_ctrl(struct mt76_dev dev, bool,
    pub option): *mut *mut int mt76_connac_mcu_start_firmware(struct mt76_dev dev, u32 addr, u32,
    pub sreq): *mut cfg80211_scan_request,
    pub scan_req): *mut ieee80211_scan_request,
    pub vif): *mut ieee80211_vif,
    pub sreq): *mut cfg80211_sched_scan_request,
    pub enable): bool,
    pub info): *mut ieee80211_bss_conf,
    pub suspend): bool,
    pub wowlan): *mut bool suspend, struct cfg80211_wowlan,
    pub key): *mut cfg80211_gtk_rekey_data,
    pub wow_suspend): bool,
    pub wait_resp): *mut *mut int mt76_connac_mcu_set_hif_suspend(struct mt76_dev dev, bool suspend, bool,
    pub vif): *mut ieee80211_vif,
    pub new_state): ieee80211_sta_state,
    pub dev): *mut int mt76_connac_mcu_chip_config(struct mt76_dev,
    pub enable): *mut *mut int mt76_connac_mcu_set_deep_sleep(struct mt76_dev dev, bool,
    pub coredump): *mut mt76_connac_coredump,
    pub target_power): i8,
    pub phy): *mut int mt76_connac_mcu_set_rate_txpower(struct mt76_phy,
    pub vif): *mut ieee80211_vif,
    pub offset): *mut *mut u32 mt76_connac_mcu_reg_rr(struct mt76_dev dev, u32,
    pub val): *mut *mut void mt76_connac_mcu_reg_wr(struct mt76_dev dev, u32 offset, u32,
    pub vif): *mut *mut mt76_connac_get_he_phy_cap(struct mt76_phy phy, struct ieee80211_vif,
    pub vif): *mut *mut mt76_connac_get_eht_phy_cap(struct mt76_phy phy, struct ieee80211_vif,
    pub sta): *mut ieee80211_link_sta,
    pub band): nl80211_band,
    pub cmd): *mut *mut mt76_wcid wcid, enum set_key_cmd,
    pub mvif): *mut *mut void mt76_connac_mcu_bss_ext_tlv(struct sk_buff skb, struct mt76_vif_link,
    pub vif): *mut ieee80211_vif,
    pub enable): bool,
    pub sta): *mut ieee80211_sta,
    pub wtbl_tlv): *mut *mut void sta_wtbl, void,
    pub enter): *mut *mut int mt76_connac_mcu_set_pm(struct mt76_dev dev, int band, int,
    pub dev): *mut int mt76_connac_mcu_restart(struct mt76_dev,
    pub dev): *mut int mt76_connac_mcu_del_wtbl_all(struct mt76_dev,
    pub val): u8 rx_sel, u8,
    pub skb): *mut *mut int mt76_connac_mcu_sta_wed_update(struct mt76_dev dev, struct sk_buff,
    pub fw_wa): *const c_char,
    pub fw_name): *const *const int mt76_connac3_load_phy_ram(struct mt76_dev dev, char,
    pub fw_name): *const *const int mt76_connac2_load_patch(struct mt76_dev dev, char,
    pub fw_name): *const *const int mt76_connac3_load_cb_patch(struct mt76_dev dev, char,
    pub wait_seq): *mut int cmd, int,
