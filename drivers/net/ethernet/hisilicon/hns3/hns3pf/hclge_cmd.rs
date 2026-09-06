//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_cmd.h
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2016-2017 Hisilicon Limited.

pub const HCLGE_CMDQ_RX_INVLD_B: c_int = 0;
pub const HCLGE_CMDQ_RX_OUTVLD_B: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_misc_vector {
    pub addr: *mut u8 __iomem,
    pub vector_irq: c_int,
    pub name: [c_char; HNAE3_INT_NAME_LEN],
}

pub const HCLGE_TQP_REG_OFFSET: c_uint = 0x80000;
pub const HCLGE_TQP_REG_SIZE: c_uint = 0x200;
pub const HCLGE_FD_COUNTER_MAX_SIZE_DEV_V2: c_int = 128;
pub const HCLGE_TQP_MAX_SIZE_DEV_V2: c_int = 1024;
pub const HCLGE_TQP_EXT_REG_OFFSET: c_uint = 0x100;
pub const HCLGE_RCB_INIT_QUERY_TIMEOUT: c_int = 10;
pub const HCLGE_RCB_INIT_FLAG_EN_B: c_int = 0;
pub const HCLGE_RCB_INIT_FLAG_FINI_B: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_rcb_init_cmd {
    pub rcb_init_flag: __le16,
    pub rsv: [u8; 22],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tqp_map_cmd {
    pub /: *mut *mut __le16 tqp_id; / Absolute tqp id for in this pf,
    pub /: *mut *mut u8 tqp_vf; / VF id,
pub const HCLGE_TQP_MAP_TYPE_PF: c_int = 0;
pub const HCLGE_TQP_MAP_TYPE_VF: c_int = 1;
pub const HCLGE_TQP_MAP_TYPE_B: c_int = 0;
pub const HCLGE_TQP_MAP_EN_B: c_int = 1;
    pub /: *mut *mut u8 tqp_flag; / Indicate it's pf or vf tqp,
    pub /: *mut *mut __le16 tqp_vid; / Virtual id in this pf/vf,
    pub rsv: [u8; 18],
}

pub const HCLGE_VECTOR_ELEMENTS_PER_CMD: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_int_type {
    HCLGE_INT_TX,
    HCLGE_INT_RX,
    HCLGE_INT_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_ctrl_vector_chain_cmd {
pub const HCLGE_VECTOR_ID_L_S: c_int = 0;

    pub int_vector_id_l: u8,
    pub int_cause_num: u8,
pub const HCLGE_INT_TYPE_S: c_int = 0;

pub const HCLGE_TQP_ID_S: c_int = 2;

pub const HCLGE_INT_GL_IDX_S: c_int = 13;
    pub tqp_type_and_id: [__le16; HCLGE_VECTOR_ELEMENTS_PER_CMD],
    pub vfid: u8,
pub const HCLGE_VECTOR_ID_H_S: c_int = 8;

    pub int_vector_id_h: u8,
}

pub const HCLGE_MAX_TC_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tx_buff_alloc_cmd {
    pub tx_pkt_buff: [__le16; HCLGE_MAX_TC_NUM],
    pub tx_buff_rsv: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_priv_buff_cmd {
    pub buf_num: [__le16; HCLGE_MAX_TC_NUM],
    pub shared_buf: __le16,
    pub rsv: [u8; 6],
}

pub const HCLGE_RX_PRIV_EN_B: c_int = 15;
pub const HCLGE_TC_NUM_ONE_DESC: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_priv_wl {
    pub high: __le16,
    pub low: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_priv_wl_buf {
    pub tc_wl: [hclge_priv_wl; HCLGE_TC_NUM_ONE_DESC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_com_thrd {
    pub com_thrd: [hclge_priv_wl; HCLGE_TC_NUM_ONE_DESC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_com_wl {
    pub com_wl: hclge_priv_wl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_waterline {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tc_thrd {
    pub low: u32,
    pub high: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_priv_buf {
    pub /: *mut *mut hclge_waterline wl; / Waterline for low and high,
    pub /: *mut *mut u32 buf_size; / TC private buffer size,
    pub tx_buf_size: u32,
    pub /: *mut *mut u32 enable; / Enable TC private buffer or not,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_shared_buf {
    pub self: hclge_waterline,
    pub tc_thrd: [hclge_tc_thrd; HCLGE_MAX_TC_NUM],
    pub buf_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pkt_buf_alloc {
    pub priv_buf: [hclge_priv_buf; HCLGE_MAX_TC_NUM],
    pub s_buf: hclge_shared_buf,
}

pub const HCLGE_RX_COM_WL_EN_B: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_com_wl_buf_cmd {
    pub high_wl: __le16,
    pub low_wl: __le16,
    pub rsv: [u8; 20],
}

pub const HCLGE_RX_PKT_EN_B: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_pkt_buf_cmd {
    pub high_pkt: __le16,
    pub low_pkt: __le16,
    pub rsv: [u8; 20],
}

pub const HCLGE_PF_STATE_DONE_B: c_int = 0;
pub const HCLGE_PF_STATE_MAIN_B: c_int = 1;
pub const HCLGE_PF_STATE_BOND_B: c_int = 2;
pub const HCLGE_PF_STATE_MAC_N_B: c_int = 6;
pub const HCLGE_PF_MAC_NUM_MASK: c_uint = 0x3;

pub const HCLGE_VF_RST_STATUS_CMD: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_func_status_cmd {
    pub vf_rst_state: [__le32; HCLGE_VF_RST_STATUS_CMD],
    pub pf_state: u8,
    pub mac_id: u8,
    pub rsv1: u8,
    pub pf_cnt_in_mac: u8,
    pub pf_num: u8,
    pub vf_num: u8,
    pub rsv: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pf_res_cmd {
    pub tqp_num: __le16,
    pub buf_size: __le16,
    pub msixcap_localid_ba_nic: __le16,
    pub msixcap_localid_number_nic: __le16,
    pub pf_intr_vector_number_roce: __le16,
    pub pf_own_fun_number: __le16,
    pub tx_buf_size: __le16,
    pub dv_buf_size: __le16,
    pub ext_tqp_num: __le16,
    pub rsv: [u8; 6],
}

pub const HCLGE_CFG_OFFSET_S: c_int = 0;

pub const HCLGE_CFG_RD_LEN_S: c_int = 24;

pub const HCLGE_CFG_RD_LEN_BYTES: c_int = 16;
pub const HCLGE_CFG_RD_LEN_UNIT: c_int = 4;
pub const HCLGE_CFG_TC_NUM_S: c_int = 8;

pub const HCLGE_CFG_TQP_DESC_N_S: c_int = 16;

pub const HCLGE_CFG_PHY_ADDR_S: c_int = 0;

pub const HCLGE_CFG_MEDIA_TP_S: c_int = 8;

pub const HCLGE_CFG_RX_BUF_LEN_S: c_int = 16;

pub const HCLGE_CFG_MAC_ADDR_H_S: c_int = 0;

pub const HCLGE_CFG_DEFAULT_SPEED_S: c_int = 16;

pub const HCLGE_CFG_RSS_SIZE_S: c_int = 24;

pub const HCLGE_CFG_SPEED_ABILITY_S: c_int = 0;

pub const HCLGE_CFG_SPEED_ABILITY_EXT_S: c_int = 10;

pub const HCLGE_CFG_VLAN_FLTR_CAP_S: c_int = 8;

pub const HCLGE_CFG_UMV_TBL_SPACE_S: c_int = 16;

pub const HCLGE_CFG_PF_RSS_SIZE_S: c_int = 0;

pub const HCLGE_CFG_TX_SPARE_BUF_SIZE_S: c_int = 4;

pub const HCLGE_CFG_CMD_CNT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_param_cmd {
    pub offset: __le32,
    pub rsv: __le32,
    pub param: [__le32; HCLGE_CFG_CMD_CNT],
}

pub const HCLGE_MAC_MODE: c_uint = 0x0;
pub const HCLGE_DESC_NUM: c_uint = 0x40;
pub const HCLGE_ALLOC_VALID_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vf_num_cmd {
    pub alloc_valid: u8,
    pub rsv: [u8; 23],
}

pub const HCLGE_RSS_DEFAULT_OUTPORT_B: c_int = 4;
pub const HCLGE_RSS_CFG_TBL_SIZE_H: c_int = 4;

pub const HCLGE_RSS_TC_OFFSET_S: c_int = 0;

pub const HCLGE_RSS_TC_SIZE_MSB_B: c_int = 11;
pub const HCLGE_RSS_TC_SIZE_S: c_int = 12;

pub const HCLGE_RSS_TC_SIZE_MSB_OFFSET: c_int = 3;
pub const HCLGE_RSS_TC_VALID_B: c_int = 15;
pub const HCLGE_LINK_STATUS_UP_B: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_link_status_cmd {
    pub status: u8,
    pub rsv: [u8; 23],
}

// for DEVICE_VERSION_V1/2, reference to promisc cmd byte8
pub const HCLGE_PROMISC_EN_UC: c_int = 1;
pub const HCLGE_PROMISC_EN_MC: c_int = 2;
pub const HCLGE_PROMISC_EN_BC: c_int = 3;
pub const HCLGE_PROMISC_TX_EN: c_int = 4;
pub const HCLGE_PROMISC_RX_EN: c_int = 5;
// for DEVICE_VERSION_V3, reference to promisc cmd byte10
pub const HCLGE_PROMISC_UC_RX_EN: c_int = 2;
pub const HCLGE_PROMISC_MC_RX_EN: c_int = 3;
pub const HCLGE_PROMISC_BC_RX_EN: c_int = 4;
pub const HCLGE_PROMISC_UC_TX_EN: c_int = 5;
pub const HCLGE_PROMISC_MC_TX_EN: c_int = 6;
pub const HCLGE_PROMISC_BC_TX_EN: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_promisc_cfg_cmd {
    pub promisc: u8,
    pub vf_id: u8,
    pub extend_promisc: u8,
    pub rsv0: [u8; 21],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_promisc_type {
    HCLGE_UNICAST	= 1,
    HCLGE_MULTICAST	= 2,
    HCLGE_BROADCAST	= 3,
}

pub const HCLGE_MAC_TX_EN_B: c_int = 6;
pub const HCLGE_MAC_RX_EN_B: c_int = 7;
pub const HCLGE_MAC_PAD_TX_B: c_int = 11;
pub const HCLGE_MAC_PAD_RX_B: c_int = 12;
pub const HCLGE_MAC_1588_TX_B: c_int = 13;
pub const HCLGE_MAC_1588_RX_B: c_int = 14;
pub const HCLGE_MAC_APP_LP_B: c_int = 15;
pub const HCLGE_MAC_LINE_LP_B: c_int = 16;
pub const HCLGE_MAC_FCS_TX_B: c_int = 17;
pub const HCLGE_MAC_RX_OVERSIZE_TRUNCATE_B: c_int = 18;
pub const HCLGE_MAC_RX_FCS_STRIP_B: c_int = 19;
pub const HCLGE_MAC_RX_FCS_B: c_int = 20;
pub const HCLGE_MAC_TX_UNDER_MIN_ERR_B: c_int = 21;
pub const HCLGE_MAC_TX_OVERSIZE_TRUNCATE_B: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_mac_mode_cmd {
    pub txrx_pad_fcs_loop_en: __le32,
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pf_rst_sync_cmd {
pub const HCLGE_PF_RST_ALL_VF_RDY_B: c_int = 0;
    pub all_vf_ready: u8,
    pub rsv: [u8; 23],
}

pub const HCLGE_CFG_SPEED_S: c_int = 0;

pub const HCLGE_CFG_DUPLEX_B: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_mac_speed_dup_cmd {
    pub speed_dup: u8,
pub const HCLGE_CFG_MAC_SPEED_CHANGE_EN_B: c_int = 0;
    pub mac_change_fec_en: u8,
    pub rsv: [u8; 4],
    pub lane_num: u8,
    pub rsv1: [u8; 17],
}

pub const HCLGE_TQP_ENABLE_B: c_int = 0;
pub const HCLGE_MAC_CFG_AN_EN_B: c_int = 0;
pub const HCLGE_MAC_CFG_AN_INT_EN_B: c_int = 1;
pub const HCLGE_MAC_CFG_AN_INT_MSK_B: c_int = 2;
pub const HCLGE_MAC_CFG_AN_INT_CLR_B: c_int = 3;
pub const HCLGE_MAC_CFG_AN_RST_B: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_auto_neg_cmd {
    pub cfg_an_cmd_flag: __le32,
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_sfp_info_cmd {
    pub speed: __le32,
    pub /: *mut *mut u8 query_type; / 0: sfp speed, 1: active speed,
    pub active_fec: u8,
    pub /: *mut *mut u8 autoneg; / autoneg state,
    pub /: *mut *mut u8 autoneg_ability; / whether support autoneg,
    pub /: *mut *mut __le32 speed_ability; / speed ability for current media,
    pub module_type: __le32,
    pub fec_ability: u8,
    pub lane_num: u8,
    pub rsv: [u8; 6],
}

pub const HCLGE_MAC_CFG_FEC_AUTO_EN_B: c_int = 0;
pub const HCLGE_MAC_CFG_FEC_MODE_S: c_int = 1;

pub const HCLGE_MAC_CFG_FEC_SET_DEF_B: c_int = 0;
pub const HCLGE_MAC_CFG_FEC_CLR_DEF_B: c_int = 1;
pub const HCLGE_MAC_FEC_OFF: c_int = 0;
pub const HCLGE_MAC_FEC_BASER: c_int = 1;
pub const HCLGE_MAC_FEC_RS: c_int = 2;
pub const HCLGE_MAC_FEC_LLRS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_fec_cmd {
    pub fec_mode: u8,
    pub default_config: u8,
    pub rsv: [u8; 22],
}

pub const HCLGE_FEC_STATS_CMD_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_query_fec_stats_cmd {
// fec rs mode total stats
    pub rs_fec_corr_blocks: __le32,
    pub rs_fec_uncorr_blocks: __le32,
    pub rs_fec_error_blocks: __le32,
// fec base-r mode per lanes stats
    pub base_r_lane_num: u8,
    pub rsv: [u8; 3],
    pub base_r_fec_corr_blocks: __le32,
    pub base_r_fec_uncorr_blocks: __le32,
}

pub const HCLGE_MAC_UPLINK_PORT: c_uint = 0x100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_config_max_frm_size_cmd {
    pub max_frm_size: __le16,
    pub min_frm_size: u8,
    pub rsv: [u8; 21],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mac_vlan_tbl_opcode {
    HCLGE_MAC_VLAN_ADD,	/* Add new or modify mac_vlan */
    HCLGE_MAC_VLAN_UPDATE,  /* Modify other fields of this table */
    HCLGE_MAC_VLAN_REMOVE,  /* Remove a entry through mac_vlan key */
    HCLGE_MAC_VLAN_LKUP,    /* Lookup a entry through mac_vlan key */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mac_vlan_add_resp_code {
    HCLGE_ADD_UC_OVERFLOW = 2,	/* ADD failed for UC overflow */
    HCLGE_ADD_MC_OVERFLOW,		/* ADD failed for MC overflow */
}

pub const HCLGE_MAC_VLAN_BIT0_EN_B: c_int = 0;
pub const HCLGE_MAC_VLAN_BIT1_EN_B: c_int = 1;
pub const HCLGE_MAC_EPORT_SW_EN_B: c_int = 12;
pub const HCLGE_MAC_EPORT_TYPE_B: c_int = 11;
pub const HCLGE_MAC_EPORT_VFID_S: c_int = 3;

pub const HCLGE_MAC_EPORT_PFID_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_vlan_tbl_entry_cmd {
    pub flags: u8,
    pub resp_code: u8,
    pub vlan_tag: __le16,
    pub mac_addr_hi32: __le32,
    pub mac_addr_lo16: __le16,
    pub rsv1: __le16,
    pub entry_type: u8,
    pub mc_mac_en: u8,
    pub egress_port: __le16,
    pub egress_queue: __le16,
    pub rsv2: [u8; 6],
}

pub const HCLGE_UMV_SPC_ALC_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_umv_spc_alc_cmd {
    pub allocate: u8,
    pub rsv1: [u8; 3],
    pub space_size: __le32,
    pub rsv2: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_mgr_tbl_entry_cmd {
    pub flags: u8,
    pub resp_code: u8,
    pub vlan_tag: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub rsv1: __le16,
    pub ethter_type: __le16,
    pub egress_port: __le16,
    pub egress_queue: __le16,
    pub sw_port_id_aware: u8,
    pub rsv2: u8,
    pub i_port_bitmap: u8,
    pub i_port_direction: u8,
    pub rsv3: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vlan_filter_ctrl_cmd {
    pub vlan_type: u8,
    pub vlan_fe: u8,
    pub rsv1: [u8; 2],
    pub vf_id: u8,
    pub rsv2: [u8; 19],
}

pub const HCLGE_VLAN_ID_OFFSET_STEP: c_int = 160;
pub const HCLGE_VLAN_BYTE_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vlan_filter_pf_cfg_cmd {
    pub vlan_offset: u8,
    pub vlan_cfg: u8,
    pub rsv: [u8; 2],
    pub vlan_offset_bitmap: [u8; HCLGE_VLAN_OFFSET_BITMAP],
}

pub const HCLGE_MAX_VF_BYTES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vlan_filter_vf_cfg_cmd {
    pub vlan_id: __le16,
    pub resp_code: u8,
    pub rsv: u8,
    pub vlan_cfg: u8,
    pub rsv1: [u8; 3],
    pub vf_bitmap: [u8; HCLGE_MAX_VF_BYTES],
}

pub const HCLGE_INGRESS_BYPASS_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_port_vlan_filter_bypass_cmd {
    pub bypass_state: u8,
    pub rsv1: [u8; 3],
    pub vf_id: u8,
    pub rsv2: [u8; 19],
}

pub const HCLGE_SWITCH_NO_MASK: c_uint = 0x0;
pub const HCLGE_SWITCH_ANTI_SPOOF_MASK: c_uint = 0xFE;
pub const HCLGE_SWITCH_ALW_LPBK_MASK: c_uint = 0xFD;
pub const HCLGE_SWITCH_ALW_LCL_LPBK_MASK: c_uint = 0xFB;
pub const HCLGE_SWITCH_LW_DST_OVRD_MASK: c_uint = 0xF7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_vlan_switch_cmd {
    pub roce_sel: u8,
    pub rsv1: [u8; 3],
    pub func_id: __le32,
    pub switch_param: u8,
    pub rsv2: [u8; 3],
    pub param_mask: u8,
    pub rsv3: [u8; 11],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_mac_vlan_cfg_sel {
    HCLGE_MAC_VLAN_NIC_SEL = 0,
    HCLGE_MAC_VLAN_ROCE_SEL,
}

pub const HCLGE_ACCEPT_TAG1_B: c_int = 0;
pub const HCLGE_ACCEPT_UNTAG1_B: c_int = 1;
pub const HCLGE_PORT_INS_TAG1_EN_B: c_int = 2;
pub const HCLGE_PORT_INS_TAG2_EN_B: c_int = 3;
pub const HCLGE_CFG_NIC_ROCE_SEL_B: c_int = 4;
pub const HCLGE_ACCEPT_TAG2_B: c_int = 5;
pub const HCLGE_ACCEPT_UNTAG2_B: c_int = 6;
pub const HCLGE_TAG_SHIFT_MODE_EN_B: c_int = 7;
pub const HCLGE_VF_NUM_PER_BYTE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vport_vtag_tx_cfg_cmd {
    pub vport_vlan_cfg: u8,
    pub vf_offset: u8,
    pub rsv1: [u8; 2],
    pub def_vlan_tag1: __le16,
    pub def_vlan_tag2: __le16,
    pub vf_bitmap: [u8; HCLGE_VF_NUM_PER_BYTE],
    pub rsv2: [u8; 8],
}

pub const HCLGE_REM_TAG1_EN_B: c_int = 0;
pub const HCLGE_REM_TAG2_EN_B: c_int = 1;
pub const HCLGE_SHOW_TAG1_EN_B: c_int = 2;
pub const HCLGE_SHOW_TAG2_EN_B: c_int = 3;
pub const HCLGE_DISCARD_TAG1_EN_B: c_int = 5;
pub const HCLGE_DISCARD_TAG2_EN_B: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vport_vtag_rx_cfg_cmd {
    pub vport_vlan_cfg: u8,
    pub vf_offset: u8,
    pub rsv1: [u8; 6],
    pub vf_bitmap: [u8; HCLGE_VF_NUM_PER_BYTE],
    pub rsv2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tx_vlan_type_cfg_cmd {
    pub ot_vlan_type: __le16,
    pub in_vlan_type: __le16,
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_vlan_type_cfg_cmd {
    pub ot_fst_vlan_type: __le16,
    pub ot_sec_vlan_type: __le16,
    pub in_fst_vlan_type: __le16,
    pub in_sec_vlan_type: __le16,
    pub rsv: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_com_tqp_queue_cmd {
    pub tqp_id: __le16,
    pub stream_id: __le16,
    pub enable: u8,
    pub rsv: [u8; 19],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_tx_queue_pointer_cmd {
    pub tqp_id: __le16,
    pub tx_tail: __le16,
    pub tx_head: __le16,
    pub fbd_num: __le16,
    pub ring_offset: __le16,
    pub rsv: [u8; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_ethertype_idx_rd_cmd {
    pub flags: u8,
    pub resp_code: u8,
    pub vlan_tag: __le16,
    pub mac_addr: [u8; ETH_ALEN],
    pub index: __le16,
    pub ethter_type: __le16,
    pub egress_port: __le16,
    pub egress_queue: __le16,
    pub rev0: __le16,
    pub i_port_bitmap: u8,
    pub i_port_direction: u8,
    pub rev1: [u8; 2],
}

pub const HCLGE_TSO_MSS_MIN_S: c_int = 0;

pub const HCLGE_TSO_MSS_MAX_S: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_tso_status_cmd {
    pub tso_mss_min: __le16,
    pub tso_mss_max: __le16,
    pub rsv: [u8; 20],
}

pub const HCLGE_GRO_EN_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg_gro_status_cmd {
    pub gro_en: u8,
    pub rsv: [u8; 23],
}

pub const HCLGE_TSO_MSS_MIN: c_int = 256;
pub const HCLGE_TSO_MSS_MAX: c_int = 9668;
pub const HCLGE_TQP_RESET_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_reset_tqp_queue_cmd {
    pub tqp_id: __le16,
    pub reset_req: u8,
    pub ready_to_reset: u8,
    pub rsv: [u8; 20],
}

pub const HCLGE_CFG_RESET_MAC_B: c_int = 3;
pub const HCLGE_CFG_RESET_FUNC_B: c_int = 7;
pub const HCLGE_CFG_RESET_RCB_B: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_reset_cmd {
    pub mac_func_reset: u8,
    pub fun_reset_vfid: u8,
    pub fun_reset_rcb: u8,
    pub rsv: u8,
    pub fun_reset_rcb_vqid_start: __le16,
    pub fun_reset_rcb_vqid_num: __le16,
    pub fun_reset_rcb_return_status: u8,
    pub rsv1: [u8; 15],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pf_rst_done_cmd {
    pub pf_rst_done: u8,
    pub rsv: [u8; 23],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_common_lb_cmd {
    pub mask: u8,
    pub enable: u8,
    pub result: u8,
    pub rsv: [u8; 21],
}

pub const HCLGE_DEFAULT_TX_BUF: c_uint = 0x4000	 /* 16k  bytes */;
pub const HCLGE_TOTAL_PKT_BUF: c_uint = 0x108000 /* 1.03125M bytes */;
pub const HCLGE_DEFAULT_DV: c_uint = 0xA000	 /* 40k byte */;
pub const HCLGE_DEFAULT_NON_DCB_DV: c_uint = 0x7800	/* 30K byte */;
pub const HCLGE_NON_DCB_ADDITIONAL_BUF: c_uint = 0x1400	/* 5120 byte */;
pub const HCLGE_LED_LOCATE_STATE_S: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_set_led_state_cmd {
    pub rsv1: [u8; 3],
    pub locate_led_config: u8,
    pub rsv2: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_get_fd_mode_cmd {
    pub mode: u8,
    pub enable: u8,
    pub rsv: [u8; 22],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_get_fd_allocation_cmd {
    pub stage1_entry_num: __le32,
    pub stage2_entry_num: __le32,
    pub stage1_counter_num: __le16,
    pub stage2_counter_num: __le16,
    pub rsv: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_set_fd_key_config_cmd {
    pub stage: u8,
    pub key_select: u8,
    pub inner_sipv6_word_en: u8,
    pub inner_dipv6_word_en: u8,
    pub outer_sipv6_word_en: u8,
    pub outer_dipv6_word_en: u8,
    pub rsv1: [u8; 2],
    pub tuple_mask: __le32,
    pub meta_data_mask: __le32,
    pub rsv2: [u8; 8],
}

pub const HCLGE_FD_EPORT_SW_EN_B: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_tcam_config_1_cmd {
    pub stage: u8,
    pub xy_sel: u8,
    pub port_info: u8,
    pub rsv1: [u8; 1],
    pub index: __le32,
    pub entry_vld: u8,
    pub rsv2: [u8; 7],
    pub tcam_data: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_tcam_config_2_cmd {
    pub tcam_data: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_tcam_config_3_cmd {
    pub tcam_data: [u8; 20],
    pub rsv: [u8; 4],
}

pub const HCLGE_FD_AD_DROP_B: c_int = 0;
pub const HCLGE_FD_AD_DIRECT_QID_B: c_int = 1;
pub const HCLGE_FD_AD_QID_L_S: c_int = 2;

pub const HCLGE_FD_AD_USE_COUNTER_B: c_int = 12;
pub const HCLGE_FD_AD_COUNTER_NUM_L_S: c_int = 13;

pub const HCLGE_FD_AD_NXT_STEP_B: c_int = 20;
pub const HCLGE_FD_AD_NXT_KEY_S: c_int = 21;

pub const HCLGE_FD_AD_WR_RULE_ID_B: c_int = 0;
pub const HCLGE_FD_AD_RULE_ID_S: c_int = 1;

pub const HCLGE_FD_AD_TC_OVRD_B: c_int = 16;
pub const HCLGE_FD_AD_TC_SIZE_S: c_int = 17;

pub const HCLGE_FD_AD_QID_H_B: c_int = 21;
pub const HCLGE_FD_AD_COUNTER_NUM_H_B: c_int = 26;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_ad_config_cmd {
    pub stage: u8,
    pub rsv1: [u8; 3],
    pub index: __le32,
    pub ad_data: __le64,
    pub rsv2: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_ad_cnt_read_cmd {
    pub rsv0: [u8; 4],
    pub index: __le16,
    pub rsv1: [u8; 2],
    pub cnt: __le64,
    pub rsv2: [u8; 8],
}

pub const HCLGE_FD_USER_DEF_OFT_S: c_int = 0;

pub const HCLGE_FD_USER_DEF_EN_B: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_user_def_cfg_cmd {
    pub ol2_cfg: __le16,
    pub l2_cfg: __le16,
    pub ol3_cfg: __le16,
    pub l3_cfg: __le16,
    pub ol4_cfg: __le16,
    pub l4_cfg: __le16,
    pub rsv: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_get_imp_bd_cmd {
    pub bd_num: __le32,
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_query_ppu_pf_other_int_dfx_cmd {
    pub over_8bd_no_fe_qid: __le16,
    pub over_8bd_no_fe_vf_id: __le16,
    pub tso_mss_cmp_min_err_qid: __le16,
    pub tso_mss_cmp_min_err_vf_id: __le16,
    pub tso_mss_cmp_max_err_qid: __le16,
    pub tso_mss_cmp_max_err_vf_id: __le16,
    pub tx_rd_fbd_poison_qid: __le16,
    pub tx_rd_fbd_poison_vf_id: __le16,
    pub rx_rd_fbd_poison_qid: __le16,
    pub rx_rd_fbd_poison_vf_id: __le16,
    pub rsv: [u8; 4],
}

pub const HCLGE_SFP_INFO_CMD_NUM: c_int = 6;
pub const HCLGE_SFP_INFO_BD0_LEN: c_int = 20;
pub const HCLGE_SFP_INFO_BDX_LEN: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_sfp_info_bd0_cmd {
    pub offset: __le16,
    pub read_len: __le16,
    pub data: [u8; HCLGE_SFP_INFO_BD0_LEN],
}

pub const HCLGE_QUERY_DEV_SPECS_BD_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dev_specs_0_cmd {
    pub rsv0: __le32,
    pub mac_entry_num: __le32,
    pub mng_entry_num: __le32,
    pub rss_ind_tbl_size: __le16,
    pub rss_key_size: __le16,
    pub int_ql_max: __le16,
    pub max_non_tso_bd_num: u8,
    pub rsv1: u8,
    pub max_tm_rate: __le32,
}

pub const HCLGE_DEF_MAX_INT_GL: c_uint = 0x1FE0U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dev_specs_1_cmd {
    pub max_frm_size: __le16,
    pub max_qset_num: __le16,
    pub max_int_gl: __le16,
    pub rsv0: [u8; 2],
    pub umv_size: __le16,
    pub mc_mac_size: __le16,
    pub rsv1: [u8; 6],
    pub tnl_num: u8,
    pub hilink_version: u8,
    pub rsv2: [u8; 4],
}

// mac speed type defined in firmware command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FIRMWARE_MAC_SPEED {
    HCLGE_FW_MAC_SPEED_1G,
    HCLGE_FW_MAC_SPEED_10G,
    HCLGE_FW_MAC_SPEED_25G,
    HCLGE_FW_MAC_SPEED_40G,
    HCLGE_FW_MAC_SPEED_50G,
    HCLGE_FW_MAC_SPEED_100G,
    HCLGE_FW_MAC_SPEED_10M,
    HCLGE_FW_MAC_SPEED_100M,
    HCLGE_FW_MAC_SPEED_200G,
}

pub const HCLGE_PHY_LINK_SETTING_BD_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_phy_link_ksetting_0_cmd {
    pub speed: __le32,
    pub duplex: u8,
    pub autoneg: u8,
    pub eth_tp_mdix: u8,
    pub eth_tp_mdix_ctrl: u8,
    pub port: u8,
    pub transceiver: u8,
    pub phy_address: u8,
    pub rsv: u8,
    pub supported: __le32,
    pub advertising: __le32,
    pub lp_advertising: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_phy_link_ksetting_1_cmd {
    pub master_slave_cfg: u8,
    pub master_slave_state: u8,
    pub rsv: [u8; 22],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_phy_reg_cmd {
    pub reg_addr: __le16,
    pub rsv0: [u8; 2],
    pub reg_val: __le16,
    pub rsv1: [u8; 18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_wol_cfg_cmd {
    pub wake_on_lan_mode: __le32,
    pub sopass: [u8; SOPASS_MAX],
    pub sopass_size: u8,
    pub rsv: [u8; 13],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_query_wol_supported_cmd {
    pub supported_wake_mode: __le32,
    pub rsv: [u8; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pfc_storm_para_cmd {
    pub dir: __le32,
    pub enable: __le32,
    pub period_ms: __le32,
    pub times: __le32,
    pub recovery_period_ms: __le32,
    pub rsv: __le32,
}

extern "C" {
    pub fn hclge_cmd_send(hw: *mut hclge_hw, desc: *mut hclge_desc, num: c_int) -> c_int;
}
