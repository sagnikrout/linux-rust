//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/hisilicon/hns3/hns3pf/hclge_main.h
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

pub const HCLGE_MAX_PF_NUM: c_int = 8;
pub const HCLGE_VF_VPORT_START_NUM: c_int = 1;
pub const HCLGE_RD_FIRST_STATS_NUM: c_int = 2;
pub const HCLGE_RD_OTHER_STATS_NUM: c_int = 4;
pub const HCLGE_INVALID_VPORT: c_uint = 0xffff;
pub const HCLGE_PF_CFG_BLOCK_SIZE: c_int = 32;

pub const HCLGE_VECTOR_REG_BASE: c_uint = 0x20000;
pub const HCLGE_VECTOR_EXT_REG_BASE: c_uint = 0x30000;
pub const HCLGE_MISC_VECTOR_REG_BASE: c_uint = 0x20400;
pub const HCLGE_VECTOR_REG_OFFSET: c_uint = 0x4;
pub const HCLGE_VECTOR_REG_OFFSET_H: c_uint = 0x1000;
pub const HCLGE_VECTOR_VF_OFFSET: c_uint = 0x100000;
pub const HCLGE_NIC_CSQ_DEPTH_REG: c_uint = 0x27008;
// bar registers for common func
pub const HCLGE_GRO_EN_REG: c_uint = 0x28000;
pub const HCLGE_RXD_ADV_LAYOUT_EN_REG: c_uint = 0x28008;
// bar registers for rcb
pub const HCLGE_RING_RX_ADDR_L_REG: c_uint = 0x80000;
pub const HCLGE_RING_RX_ADDR_H_REG: c_uint = 0x80004;
pub const HCLGE_RING_RX_BD_NUM_REG: c_uint = 0x80008;
pub const HCLGE_RING_RX_BD_LENGTH_REG: c_uint = 0x8000C;
pub const HCLGE_RING_RX_MERGE_EN_REG: c_uint = 0x80014;
pub const HCLGE_RING_RX_TAIL_REG: c_uint = 0x80018;
pub const HCLGE_RING_RX_HEAD_REG: c_uint = 0x8001C;
pub const HCLGE_RING_RX_FBD_NUM_REG: c_uint = 0x80020;
pub const HCLGE_RING_RX_OFFSET_REG: c_uint = 0x80024;
pub const HCLGE_RING_RX_FBD_OFFSET_REG: c_uint = 0x80028;
pub const HCLGE_RING_RX_STASH_REG: c_uint = 0x80030;
pub const HCLGE_RING_RX_BD_ERR_REG: c_uint = 0x80034;
pub const HCLGE_RING_TX_ADDR_L_REG: c_uint = 0x80040;
pub const HCLGE_RING_TX_ADDR_H_REG: c_uint = 0x80044;
pub const HCLGE_RING_TX_BD_NUM_REG: c_uint = 0x80048;
pub const HCLGE_RING_TX_PRIORITY_REG: c_uint = 0x8004C;
pub const HCLGE_RING_TX_TC_REG: c_uint = 0x80050;
pub const HCLGE_RING_TX_MERGE_EN_REG: c_uint = 0x80054;
pub const HCLGE_RING_TX_TAIL_REG: c_uint = 0x80058;
pub const HCLGE_RING_TX_HEAD_REG: c_uint = 0x8005C;
pub const HCLGE_RING_TX_FBD_NUM_REG: c_uint = 0x80060;
pub const HCLGE_RING_TX_OFFSET_REG: c_uint = 0x80064;
pub const HCLGE_RING_TX_EBD_NUM_REG: c_uint = 0x80068;
pub const HCLGE_RING_TX_EBD_OFFSET_REG: c_uint = 0x80070;
pub const HCLGE_RING_TX_BD_ERR_REG: c_uint = 0x80074;
pub const HCLGE_RING_EN_REG: c_uint = 0x80090;
// bar registers for tqp interrupt
pub const HCLGE_TQP_INTR_CTRL_REG: c_uint = 0x20000;
pub const HCLGE_TQP_INTR_GL0_REG: c_uint = 0x20100;
pub const HCLGE_TQP_INTR_GL1_REG: c_uint = 0x20200;
pub const HCLGE_TQP_INTR_GL2_REG: c_uint = 0x20300;
pub const HCLGE_TQP_INTR_RL_REG: c_uint = 0x20900;
pub const HCLGE_RSS_IND_TBL_SIZE: c_int = 512;
pub const HCLGE_RSS_TC_SIZE_0: c_int = 1;
pub const HCLGE_RSS_TC_SIZE_1: c_int = 2;
pub const HCLGE_RSS_TC_SIZE_2: c_int = 4;
pub const HCLGE_RSS_TC_SIZE_3: c_int = 8;
pub const HCLGE_RSS_TC_SIZE_4: c_int = 16;
pub const HCLGE_RSS_TC_SIZE_5: c_int = 32;
pub const HCLGE_RSS_TC_SIZE_6: c_int = 64;
pub const HCLGE_RSS_TC_SIZE_7: c_int = 128;
pub const HCLGE_UMV_TBL_SIZE: c_int = 3072;

pub const HCLGE_TQP_RESET_TRY_TIMES: c_int = 200;
pub const HCLGE_PHY_PAGE_MDIX: c_int = 0;
pub const HCLGE_PHY_PAGE_COPPER: c_int = 0;
// Page Selection Reg.
pub const HCLGE_PHY_PAGE_REG: c_int = 22;
// Copper Specific Control Register
pub const HCLGE_PHY_CSC_REG: c_int = 16;
// Copper Specific Status Register
pub const HCLGE_PHY_CSS_REG: c_int = 17;
pub const HCLGE_PHY_MDIX_CTRL_S: c_int = 5;

pub const HCLGE_PHY_MDIX_STATUS_B: c_int = 6;
pub const HCLGE_PHY_SPEED_DUP_RESOLVE_B: c_int = 11;
pub const HCLGE_GET_DFX_REG_TYPE_CNT: c_int = 4;
// Factor used to calculate offset and bitmap of VF num
pub const HCLGE_VF_NUM_PER_CMD: c_int = 64;
pub const HCLGE_MAX_QSET_NUM: c_int = 1024;
pub const HCLGE_DBG_RESET_INFO_LEN: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HLCGE_PORT_TYPE {
    HOST_PORT,
    NETWORK_PORT
}

pub const PF_VPORT_ID: c_int = 0;
pub const HCLGE_PF_ID_S: c_int = 0;

pub const HCLGE_VF_ID_S: c_int = 3;

pub const HCLGE_PORT_TYPE_B: c_int = 11;
pub const HCLGE_NETWORK_PORT_ID_S: c_int = 0;

// Reset related Registers
pub const HCLGE_PF_OTHER_INT_REG: c_uint = 0x20600;
pub const HCLGE_MISC_RESET_STS_REG: c_uint = 0x20700;
pub const HCLGE_MISC_VECTOR_INT_STS: c_uint = 0x20800;
pub const HCLGE_GLOBAL_RESET_REG: c_uint = 0x20A00;
pub const HCLGE_GLOBAL_RESET_BIT: c_int = 0;
pub const HCLGE_CORE_RESET_BIT: c_int = 1;
pub const HCLGE_IMP_RESET_BIT: c_int = 2;

pub const HCLGE_FUN_RST_ING: c_uint = 0x20C00;
pub const HCLGE_FUN_RST_ING_B: c_int = 0;
// Vector0 register bits define
pub const HCLGE_VECTOR0_REG_PTP_INT_B: c_int = 0;
pub const HCLGE_VECTOR0_GLOBALRESET_INT_B: c_int = 5;
pub const HCLGE_VECTOR0_CORERESET_INT_B: c_int = 6;
pub const HCLGE_VECTOR0_IMPRESET_INT_B: c_int = 7;
// Vector0 interrupt CMDQ event source register(RW)
pub const HCLGE_VECTOR0_CMDQ_SRC_REG: c_uint = 0x27100;
// CMDQ register bits for RX event(=MBX event)
pub const HCLGE_VECTOR0_RX_CMDQ_INT_B: c_int = 1;
pub const HCLGE_VECTOR0_IMP_RESET_INT_B: c_int = 1;

pub const HCLGE_TQP_MEM_SIZE: c_uint = 0x10000;
pub const HCLGE_MEM_BAR: c_int = 4;
// in the bar4, the first half is for roce, and the second half is for nic

pub const HCLGE_MAC_MIN_FRAME: c_int = 64;
pub const HCLGE_MAC_MAX_FRAME: c_int = 9728;

// to be compatible with exsit board

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_DEV_STATE {
    HCLGE_STATE_REINITING,
    HCLGE_STATE_DOWN,
    HCLGE_STATE_DISABLED,
    HCLGE_STATE_REMOVING,
    HCLGE_STATE_NIC_REGISTERED,
    HCLGE_STATE_ROCE_REGISTERED,
    HCLGE_STATE_SERVICE_INITED,
    HCLGE_STATE_RST_SERVICE_SCHED,
    HCLGE_STATE_RST_HANDLING,
    HCLGE_STATE_MBX_SERVICE_SCHED,
    HCLGE_STATE_MBX_HANDLING,
    HCLGE_STATE_ERR_SERVICE_SCHED,
    HCLGE_STATE_STATISTICS_UPDATING,
    HCLGE_STATE_LINK_UPDATING,
    HCLGE_STATE_RST_FAIL,
    HCLGE_STATE_FD_TBL_CHANGED,
    HCLGE_STATE_FD_CLEAR_ALL,
    HCLGE_STATE_FD_USER_DEF_CHANGED,
    HCLGE_STATE_PTP_EN,
    HCLGE_STATE_PTP_TX_HANDLING,
    HCLGE_STATE_FEC_STATS_UPDATING,
    HCLGE_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_evt_cause {
    HCLGE_VECTOR0_EVENT_RST,
    HCLGE_VECTOR0_EVENT_MBX,
    HCLGE_VECTOR0_EVENT_ERR,
    HCLGE_VECTOR0_EVENT_PTP,
    HCLGE_VECTOR0_EVENT_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_MAC_SPEED {
    HCLGE_MAC_SPEED_UNKNOWN = 0,		/* unknown */
    HCLGE_MAC_SPEED_10M	= 10,		/* 10 Mbps */
    HCLGE_MAC_SPEED_100M	= 100,		/* 100 Mbps */
    HCLGE_MAC_SPEED_1G	= 1000,		/* 1000 Mbps   = 1 Gbps */
    HCLGE_MAC_SPEED_10G	= 10000,	/* 10000 Mbps  = 10 Gbps */
    HCLGE_MAC_SPEED_25G	= 25000,	/* 25000 Mbps  = 25 Gbps */
    HCLGE_MAC_SPEED_40G	= 40000,	/* 40000 Mbps  = 40 Gbps */
    HCLGE_MAC_SPEED_50G	= 50000,	/* 50000 Mbps  = 50 Gbps */
    HCLGE_MAC_SPEED_100G	= 100000,	/* 100000 Mbps = 100 Gbps */
    HCLGE_MAC_SPEED_200G	= 200000	/* 200000 Mbps = 200 Gbps */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_MAC_DUPLEX {
    HCLGE_MAC_HALF,
    HCLGE_MAC_FULL
}

// hilink version
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_hilink_version {
    HCLGE_HILINK_H32 = 0,
    HCLGE_HILINK_H60 = 1,
}

pub const QUERY_SFP_SPEED: c_int = 0;
pub const QUERY_ACTIVE_SPEED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_wol_info {
    pub /: *mut *mut u32 wol_support_mode; / store the wake on lan info,
    pub wol_current_mode: u32,
    pub wol_sopass: [u8; SOPASS_MAX],
    pub wol_sopass_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac {
    pub mac_id: u8,
    pub phy_addr: u8,
    pub flag: u8,
    pub /: *mut *mut u8 media_type; / port media type, e.g. fibre/copper/backplane,
    pub mac_addr: [u8; ETH_ALEN],
    pub autoneg: u8,
    pub req_autoneg: u8,
    pub duplex: u8,
    pub req_duplex: u8,
    pub support_autoneg: u8,
    pub /: *mut *mut u8 speed_type; / 0: sfp speed, 1: active speed,
    pub lane_num: u8,
    pub req_lane_num: u8,
    pub speed: u32,
    pub req_speed: u32,
    pub max_speed: u32,
    pub /: *mut *mut u32 speed_ability; / speed ability supported by current media,
    pub /: *mut *mut u32 module_type; / sub media type, e.g. kr/cr/sr/lr,
    pub /: *mut *mut u32 fec_mode; / active fec mode,
    pub user_fec_mode: u32,
    pub fec_ability: u32,
    pub /: *mut *mut int link; / store the link status of mac & phy (if phy exists),
    pub wol: hclge_wol_info,
    pub phydev: *mut phy_device,
    pub mdio_bus: *mut mii_bus,
    pub phy_if: phy_interface_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_hw {
    pub hw: hclge_comm_hw,
    pub mac: hclge_mac,
    pub num_vec: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_fc_mode {
    HCLGE_FC_NONE,
    HCLGE_FC_RX_PAUSE,
    HCLGE_FC_TX_PAUSE,
    HCLGE_FC_FULL,
    HCLGE_FC_PFC,
    HCLGE_FC_DEFAULT
}

pub const HCLGE_FILTER_TYPE_VF: c_int = 0;
pub const HCLGE_FILTER_TYPE_PORT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_vlan_fltr_cap {
    HCLGE_VLAN_FLTR_DEF,
    HCLGE_VLAN_FLTR_CAN_MDF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hclge_link_fail_code {
    HCLGE_LF_NORMAL,
    HCLGE_LF_REF_CLOCK_LOST,
    HCLGE_LF_XSFP_TX_DISABLE,
    HCLGE_LF_XSFP_ABSENT,
}

pub const HCLGE_LINK_STATUS_DOWN: c_int = 0;
pub const HCLGE_LINK_STATUS_UP: c_int = 1;
pub const HCLGE_DIR_RX: c_int = 0;
pub const HCLGE_DIR_TX: c_int = 1;
pub const HCLGE_MAX_PFC_PREVENTION_TOUT: c_int = 2000;
pub const HCLGE_DEFAULT_PFC_PREVENTION_TOUT: c_int = 1000;
pub const HCLGE_PG_NUM: c_int = 4;
pub const HCLGE_SCH_MODE_SP: c_int = 0;
pub const HCLGE_SCH_MODE_DWRR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_pg_info {
    pub pg_id: u8,
    pub /: *mut *mut u8 pg_sch_mode; / 0: sp; 1: dwrr,
    pub tc_bit_map: u8,
    pub bw_limit: u32,
    pub tc_dwrr: [u8; HNAE3_MAX_TC],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tc_info {
    pub tc_id: u8,
    pub /: *mut *mut u8 tc_sch_mode; / 0: sp; 1: dwrr,
    pub pgid: u8,
    pub bw_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_cfg {
    pub tc_num: u8,
    pub vlan_fliter_cap: u8,
    pub tqp_desc_num: u16,
    pub rx_buf_len: u16,
    pub vf_rss_size_max: u16,
    pub pf_rss_size_max: u16,
    pub phy_addr: u8,
    pub media_type: u8,
    pub mac_addr: [u8; ETH_ALEN],
    pub default_speed: u8,
    pub numa_node_map: u32,
    pub tx_spare_buf_size: u32,
    pub speed_ability: u16,
    pub umv_space: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tm_info {
    pub num_tc: u8,
    pub /: *mut *mut u8 num_pg; / It must be 1 if vNET-Base schd,
    pub pg_dwrr: [u8; HCLGE_PG_NUM],
    pub prio_tc: [u8; HNAE3_MAX_USER_PRIO],
    pub pg_info: [hclge_pg_info; HCLGE_PG_NUM],
    pub tc_info: [hclge_tc_info; HNAE3_MAX_TC],
    pub fc_mode: hclge_fc_mode,
    pub /: *mut *mut u8 hw_pfc_map; / Allow for packet drop or not on this TC,
    pub /: *mut *mut u8 pfc_en; / PFC enabled or not for user priority,
}

// max number of mac statistics on each version
pub const HCLGE_MAC_STATS_MAX_NUM_V1: c_int = 87;
pub const HCLGE_MAC_STATS_MAX_NUM_V2: c_int = 105;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_comm_stats_str {
    pub desc: [c_char; ETH_GSTRING_LEN],
    pub stats_num: u32,
    pub offset: c_ulong,
}

// mac stats ,opcode id: 0x0032
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_stats {
    pub mac_tx_mac_pause_num: u64,
    pub mac_rx_mac_pause_num: u64,
    pub rsv0: u64,
    pub mac_tx_pfc_pri0_pkt_num: u64,
    pub mac_tx_pfc_pri1_pkt_num: u64,
    pub mac_tx_pfc_pri2_pkt_num: u64,
    pub mac_tx_pfc_pri3_pkt_num: u64,
    pub mac_tx_pfc_pri4_pkt_num: u64,
    pub mac_tx_pfc_pri5_pkt_num: u64,
    pub mac_tx_pfc_pri6_pkt_num: u64,
    pub mac_tx_pfc_pri7_pkt_num: u64,
    pub mac_rx_pfc_pri0_pkt_num: u64,
    pub mac_rx_pfc_pri1_pkt_num: u64,
    pub mac_rx_pfc_pri2_pkt_num: u64,
    pub mac_rx_pfc_pri3_pkt_num: u64,
    pub mac_rx_pfc_pri4_pkt_num: u64,
    pub mac_rx_pfc_pri5_pkt_num: u64,
    pub mac_rx_pfc_pri6_pkt_num: u64,
    pub mac_rx_pfc_pri7_pkt_num: u64,
    pub mac_tx_total_pkt_num: u64,
    pub mac_tx_total_oct_num: u64,
    pub mac_tx_good_pkt_num: u64,
    pub mac_tx_bad_pkt_num: u64,
    pub mac_tx_good_oct_num: u64,
    pub mac_tx_bad_oct_num: u64,
    pub mac_tx_uni_pkt_num: u64,
    pub mac_tx_multi_pkt_num: u64,
    pub mac_tx_broad_pkt_num: u64,
    pub mac_tx_undersize_pkt_num: u64,
    pub mac_tx_oversize_pkt_num: u64,
    pub mac_tx_64_oct_pkt_num: u64,
    pub mac_tx_65_127_oct_pkt_num: u64,
    pub mac_tx_128_255_oct_pkt_num: u64,
    pub mac_tx_256_511_oct_pkt_num: u64,
    pub mac_tx_512_1023_oct_pkt_num: u64,
    pub mac_tx_1024_1518_oct_pkt_num: u64,
    pub mac_tx_1519_2047_oct_pkt_num: u64,
    pub mac_tx_2048_4095_oct_pkt_num: u64,
    pub mac_tx_4096_8191_oct_pkt_num: u64,
    pub rsv1: u64,
    pub mac_tx_8192_9216_oct_pkt_num: u64,
    pub mac_tx_9217_12287_oct_pkt_num: u64,
    pub mac_tx_12288_16383_oct_pkt_num: u64,
    pub mac_tx_1519_max_good_oct_pkt_num: u64,
    pub mac_tx_1519_max_bad_oct_pkt_num: u64,
    pub mac_rx_total_pkt_num: u64,
    pub mac_rx_total_oct_num: u64,
    pub mac_rx_good_pkt_num: u64,
    pub mac_rx_bad_pkt_num: u64,
    pub mac_rx_good_oct_num: u64,
    pub mac_rx_bad_oct_num: u64,
    pub mac_rx_uni_pkt_num: u64,
    pub mac_rx_multi_pkt_num: u64,
    pub mac_rx_broad_pkt_num: u64,
    pub mac_rx_undersize_pkt_num: u64,
    pub mac_rx_oversize_pkt_num: u64,
    pub mac_rx_64_oct_pkt_num: u64,
    pub mac_rx_65_127_oct_pkt_num: u64,
    pub mac_rx_128_255_oct_pkt_num: u64,
    pub mac_rx_256_511_oct_pkt_num: u64,
    pub mac_rx_512_1023_oct_pkt_num: u64,
    pub mac_rx_1024_1518_oct_pkt_num: u64,
    pub mac_rx_1519_2047_oct_pkt_num: u64,
    pub mac_rx_2048_4095_oct_pkt_num: u64,
    pub mac_rx_4096_8191_oct_pkt_num: u64,
    pub rsv2: u64,
    pub mac_rx_8192_9216_oct_pkt_num: u64,
    pub mac_rx_9217_12287_oct_pkt_num: u64,
    pub mac_rx_12288_16383_oct_pkt_num: u64,
    pub mac_rx_1519_max_good_oct_pkt_num: u64,
    pub mac_rx_1519_max_bad_oct_pkt_num: u64,
    pub mac_tx_fragment_pkt_num: u64,
    pub mac_tx_undermin_pkt_num: u64,
    pub mac_tx_jabber_pkt_num: u64,
    pub mac_tx_err_all_pkt_num: u64,
    pub mac_tx_from_app_good_pkt_num: u64,
    pub mac_tx_from_app_bad_pkt_num: u64,
    pub mac_rx_fragment_pkt_num: u64,
    pub mac_rx_undermin_pkt_num: u64,
    pub mac_rx_jabber_pkt_num: u64,
    pub mac_rx_fcs_err_pkt_num: u64,
    pub mac_rx_send_app_good_pkt_num: u64,
    pub mac_rx_send_app_bad_pkt_num: u64,
    pub mac_tx_pfc_pause_pkt_num: u64,
    pub mac_rx_pfc_pause_pkt_num: u64,
    pub mac_tx_ctrl_pkt_num: u64,
    pub mac_rx_ctrl_pkt_num: u64,
// duration of pfc
    pub mac_tx_pfc_pri0_xoff_time: u64,
    pub mac_tx_pfc_pri1_xoff_time: u64,
    pub mac_tx_pfc_pri2_xoff_time: u64,
    pub mac_tx_pfc_pri3_xoff_time: u64,
    pub mac_tx_pfc_pri4_xoff_time: u64,
    pub mac_tx_pfc_pri5_xoff_time: u64,
    pub mac_tx_pfc_pri6_xoff_time: u64,
    pub mac_tx_pfc_pri7_xoff_time: u64,
    pub mac_rx_pfc_pri0_xoff_time: u64,
    pub mac_rx_pfc_pri1_xoff_time: u64,
    pub mac_rx_pfc_pri2_xoff_time: u64,
    pub mac_rx_pfc_pri3_xoff_time: u64,
    pub mac_rx_pfc_pri4_xoff_time: u64,
    pub mac_rx_pfc_pri5_xoff_time: u64,
    pub mac_rx_pfc_pri6_xoff_time: u64,
    pub mac_rx_pfc_pri7_xoff_time: u64,
// duration of pause
    pub mac_tx_pause_xoff_time: u64,
    pub mac_rx_pause_xoff_time: u64,
}

// fec stats ,opcode id: 0x0316
pub const HCLGE_FEC_STATS_MAX_LANES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fec_stats {
// fec rs mode total stats
    pub rs_corr_blocks: u64,
    pub rs_uncorr_blocks: u64,
    pub rs_error_blocks: u64,
// fec base-r mode per lanes stats
    pub base_r_lane_num: u64,
    pub base_r_corr_blocks: u64,
    pub base_r_uncorr_blocks: u64,
    pub base_r_corr_per_lanes: [u64; HCLGE_FEC_STATS_MAX_LANES],
    pub base_r_uncorr_per_lanes: [u64; HCLGE_FEC_STATS_MAX_LANES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vlan_type_cfg {
    pub rx_ot_fst_vlan_type: u16,
    pub rx_ot_sec_vlan_type: u16,
    pub rx_in_fst_vlan_type: u16,
    pub rx_in_sec_vlan_type: u16,
    pub tx_ot_vlan_type: u16,
    pub tx_in_vlan_type: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_MODE {
    HCLGE_FD_MODE_DEPTH_2K_WIDTH_400B_STAGE_1,
    HCLGE_FD_MODE_DEPTH_1K_WIDTH_400B_STAGE_2,
    HCLGE_FD_MODE_DEPTH_4K_WIDTH_200B_STAGE_1,
    HCLGE_FD_MODE_DEPTH_2K_WIDTH_200B_STAGE_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_KEY_TYPE {
    HCLGE_FD_KEY_BASE_ON_PTYPE,
    HCLGE_FD_KEY_BASE_ON_TUPLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_STAGE {
    HCLGE_FD_STAGE_1,
    HCLGE_FD_STAGE_2,
    MAX_STAGE_NUM,
}

// OUTER_XXX indicates tuples in tunnel header of tunnel packet
// INNER_XXX indicate tuples in tunneled header of tunnel packet or
// tuples of non-tunnel packet
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_TUPLE {
    OUTER_DST_MAC,
    OUTER_SRC_MAC,
    OUTER_VLAN_TAG_FST,
    OUTER_VLAN_TAG_SEC,
    OUTER_ETH_TYPE,
    OUTER_L2_RSV,
    OUTER_IP_TOS,
    OUTER_IP_PROTO,
    OUTER_SRC_IP,
    OUTER_DST_IP,
    OUTER_L3_RSV,
    OUTER_SRC_PORT,
    OUTER_DST_PORT,
    OUTER_L4_RSV,
    OUTER_TUN_VNI,
    OUTER_TUN_FLOW_ID,
    INNER_DST_MAC,
    INNER_SRC_MAC,
    INNER_VLAN_TAG_FST,
    INNER_VLAN_TAG_SEC,
    INNER_ETH_TYPE,
    INNER_L2_RSV,
    INNER_IP_TOS,
    INNER_IP_PROTO,
    INNER_SRC_IP,
    INNER_DST_IP,
    INNER_L3_RSV,
    INNER_SRC_PORT,
    INNER_DST_PORT,
    INNER_L4_RSV,
    MAX_TUPLE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_META_DATA {
    PACKET_TYPE_ID,
    IP_FRAGEMENT,
    ROCE_TYPE,
    NEXT_KEY,
    VLAN_NUMBER,
    SRC_VPORT,
    DST_VPORT,
    TUNNEL_PACKET,
    MAX_META_DATA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_KEY_OPT {
    KEY_OPT_U8,
    KEY_OPT_LE16,
    KEY_OPT_LE32,
    KEY_OPT_MAC,
    KEY_OPT_IP,
    KEY_OPT_VNI,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_info {
    pub key_type: u8,
    pub /: *mut *mut u8 key_length; / use bit as unit,
    pub key_opt: HCLGE_FD_KEY_OPT,
    pub offset: c_int,
    pub moffset: c_int,
}

pub const MAX_KEY_LENGTH: c_int = 400;

pub const MAX_META_DATA_LENGTH: c_int = 32;
pub const HCLGE_FD_MAX_USER_DEF_OFFSET: c_int = 9000;

pub const HCLGE_VNI_LENGTH: c_int = 3;
// assigned by firmware, the real filter number for each pf may be less
pub const MAX_FD_FILTER_NUM: c_int = 4096;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_ACTIVE_RULE_TYPE {
    HCLGE_FD_RULE_NONE,
    HCLGE_FD_ARFS_ACTIVE,
    HCLGE_FD_EP_ACTIVE,
    HCLGE_FD_TC_FLOWER_ACTIVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_PACKET_TYPE {
    NIC_PACKET,
    ROCE_PACKET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_ACTION {
    HCLGE_FD_ACTION_SELECT_QUEUE,
    HCLGE_FD_ACTION_DROP_PACKET,
    HCLGE_FD_ACTION_SELECT_TC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_NODE_STATE {
    HCLGE_FD_TO_ADD,
    HCLGE_FD_TO_DEL,
    HCLGE_FD_ACTIVE,
    HCLGE_FD_DELETED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_FD_USER_DEF_LAYER {
    HCLGE_FD_USER_DEF_NONE,
    HCLGE_FD_USER_DEF_L2,
    HCLGE_FD_USER_DEF_L3,
    HCLGE_FD_USER_DEF_L4,
}

pub const HCLGE_FD_USER_DEF_LAYER_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_user_def_cfg {
    pub ref_cnt: u16,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_user_def_info {
    pub layer: HCLGE_FD_USER_DEF_LAYER,
    pub data: u16,
    pub data_mask: u16,
    pub offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_key_cfg {
    pub key_sel: u8,
    pub inner_sipv6_word_en: u8,
    pub inner_dipv6_word_en: u8,
    pub outer_sipv6_word_en: u8,
    pub outer_dipv6_word_en: u8,
    pub tuple_active: u32,
    pub meta_data_active: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_cfg {
    pub fd_mode: u8,
    pub /: *mut *mut u16 max_key_length; / use bit as unit,
    pub /: *mut *mut u32 rule_num[MAX_STAGE_NUM]; / rule entry number,
    pub /: *mut *mut u16 cnt_num[MAX_STAGE_NUM]; / rule hit counter number,
    pub key_cfg: [hclge_fd_key_cfg; MAX_STAGE_NUM],
    pub user_def_cfg: [hclge_fd_user_def_cfg; HCLGE_FD_USER_DEF_LAYER_NUM],
}

pub const IPV4_INDEX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_rule_tuples {
    pub src_mac: [u8; ETH_ALEN],
    pub dst_mac: [u8; ETH_ALEN],
// Be compatible for ip address of both ipv4 and ipv6.
// For ipv4 address, we store it in src/dst_ip[3].
//
    pub src_ip: [u32; IPV6_ADDR_WORDS],
    pub dst_ip: [u32; IPV6_ADDR_WORDS],
    pub src_port: u16,
    pub dst_port: u16,
    pub vlan_tag1: u16,
    pub ether_proto: u16,
    pub l2_user_def: u16,
    pub l3_user_def: u16,
    pub l4_user_def: u32,
    pub ip_tos: u8,
    pub ip_proto: u8,
    pub outer_tun_vni: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_rule {
    pub rule_node: hlist_node,
    pub tuples: hclge_fd_rule_tuples,
    pub tuples_mask: hclge_fd_rule_tuples,
    pub unused_tuple: u32,
    pub flow_type: u32,
    pub cookie: c_ulong,
    pub tc: u8,
    pub cls_flower: },
    pub /: *mut *mut u16 flow_id; / only used for arfs,
    pub arfs: },
    pub user_def: hclge_fd_user_def_info,
    pub ep: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_fd_ad_data {
    pub ad_id: u16,
    pub drop_packet: u8,
    pub forward_to_direct_queue: u8,
    pub queue_id: u16,
    pub use_counter: u8,
    pub counter_id: u8,
    pub use_next_stage: u8,
    pub write_rule_id_to_bd: u8,
    pub next_input_key: u8,
    pub rule_id: u16,
    pub tc_size: u16,
    pub override_tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_MAC_NODE_STATE {
    HCLGE_MAC_TO_ADD,
    HCLGE_MAC_TO_DEL,
    HCLGE_MAC_ACTIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_node {
    pub node: list_head,
    pub state: HCLGE_MAC_NODE_STATE,
    pub mac_addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_MAC_ADDR_TYPE {
    HCLGE_MAC_ADDR_UC,
    HCLGE_MAC_ADDR_MC
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vport_vlan_cfg {
    pub node: list_head,
    pub hd_tbl_status: c_int,
    pub vlan_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rst_stats {
    pub /: *mut *mut u32 reset_done_cnt; / the number of reset has completed,
    pub /: *mut *mut u32 hw_reset_done_cnt; / the number of HW reset has completed,
    pub /: *mut *mut u32 pf_rst_cnt; / the number of PF reset,
    pub /: *mut *mut u32 flr_rst_cnt; / the number of FLR,
    pub /: *mut *mut u32 global_rst_cnt; / the number of GLOBAL,
    pub /: *mut *mut u32 imp_rst_cnt; / the number of IMP reset,
    pub /: *mut *mut u32 reset_cnt; / the number of reset,
    pub /: *mut *mut u32 reset_fail_cnt; / the number of reset fail,
}

// time and register status when mac tunnel interruption occur
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_tnl_stats {
    pub time: u64,
    pub status: u32,
}

pub const HCLGE_WAIT_RESET_DONE: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vf_vlan_cfg {
    pub mbx_cmd: u8,
    pub subcode: u8,
    pub is_kill: u8,
    pub vlan: __le16,
    pub proto: __le16,
}

// For each bit of TCAM entry, it uses a pair of 'x' and
// 'y' to indicate which value to match, like below:
// ----------------------------------
// | bit x | bit y |  search value  |
// ----------------------------------
// |   0   |   0   |   always hit   |
// ----------------------------------
// |   1   |   0   |   match '0'    |
// ----------------------------------
// |   0   |   1   |   match '1'    |
// ----------------------------------
// |   1   |   1   |   invalid      |
// ----------------------------------
// Then for input key(k) and mask(v), we can calculate the value by
// the formulae:
// x = (~k) & v
// y = k & v
//

pub const HCLGE_MAC_TNL_LOG_SIZE: c_int = 8;
pub const HCLGE_VPORT_NUM: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_dev {
    pub pdev: *mut pci_dev,
    pub ae_dev: *mut hnae3_ae_dev,
    pub hw: hclge_hw,
    pub misc_vector: hclge_misc_vector,
    pub mac_stats: hclge_mac_stats,
    pub fec_stats: hclge_fec_stats,
    pub state: c_ulong,
    pub flr_state: c_ulong,
    pub last_reset_time: c_ulong,
    pub reset_type: hnae3_reset_type,
    pub reset_level: hnae3_reset_type,
    pub default_reset_request: c_ulong,
    pub /: *mut *mut unsigned long reset_request; / reset has been requested,
    pub /: *mut *mut unsigned long reset_pending; / client rst is pending to be served,
    pub rst_stats: hclge_rst_stats,
    pub /: *mut *mut semaphore reset_sem; / protect reset process,
    pub fw_version: u32,
    pub /: *mut *mut u16 num_tqps; / Num task queue pairs of this PF,
    pub /: *mut *mut u16 num_req_vfs; / Num VFs requested for this PF,
    pub /: *mut *mut u16 base_tqp_pid; / Base task tqp physical id of this PF,
    pub /: *mut *mut u16 alloc_rss_size; / Allocated RSS task queue,
    pub /: *mut *mut u16 vf_rss_size_max; / HW defined VF max RSS task queue,
    pub /: *mut *mut u16 pf_rss_size_max; / HW defined PF max RSS task queue,
    pub /: *mut *mut u32 tx_spare_buf_size; / HW defined TX spare buffer size,
    pub /: *mut *mut u16 pfc_prevention_tout; / User config, restored after reset,
    pub /: *mut *mut u16 pfc_prevention_tout_default; / HW default, to avoid stale state,
    pub /: *mut *mut u16 fdir_pf_filter_count; / Num of guaranteed filters for this PF,
    pub /: *mut *mut u16 num_alloc_vport; / Num vports this driver supports,
    pub numa_node_mask: nodemask_t,
    pub rx_buf_len: u16,
    pub /: *mut *mut u16 num_tx_desc; / desc num of per tx queue,
    pub /: *mut *mut u16 num_rx_desc; / desc num of per rx queue,
    pub hw_tc_map: u8,
    pub fc_mode_last_time: hclge_fc_mode,
    pub support_sfp_query: u8,
pub const HCLGE_FLAG_TC_BASE_SCH_MODE: c_int = 1;
pub const HCLGE_FLAG_VNET_BASE_SCH_MODE: c_int = 2;
    pub tx_sch_mode: u8,
    pub tc_max: u8,
    pub pfc_max: u8,
    pub default_up: u8,
    pub dcbx_cap: u8,
    pub tm_info: hclge_tm_info,
    pub num_msi: u16,
    pub num_msi_left: u16,
    pub num_msi_used: u16,
    pub vector_status: *mut u16,
    pub vector_irq: *mut c_int,
    pub /: *mut *mut u16 num_nic_msi; / Num of nic vectors for this PF,
    pub /: *mut *mut u16 num_roce_msi; / Num of roce vectors for this PF,
    pub service_timer_period: c_ulong,
    pub service_timer_previous: c_ulong,
    pub reset_timer: timer_list,
    pub service_task: delayed_work,
    pub cur_promisc: bool,
    pub /: *mut *mut int num_alloc_vfs; / Actual number of VFs allocated,
    pub htqp: *mut hclge_comm_tqp,
    pub vport: *mut hclge_vport,
    pub hclge_dbgfs: *mut dentry,
    pub nic_client: *mut hnae3_client,
    pub roce_client: *mut hnae3_client,

    pub flag: u32,
    pub /: *mut *mut u32 pkt_buf_size; / Total pf buf size for tx/rx,
    pub /: *mut *mut u32 tx_buf_size; / Tx buffer size for each TC,
    pub /: *mut *mut u32 dv_buf_size; / Dv buffer size for each TC,
    pub /: *mut *mut u32 mps; / Max packet size,
// vport_lock protect resource shared by vports
    pub vport_lock: mutex,
    pub vlan_type_cfg: hclge_vlan_type_cfg,
    pub vlan_table: [c_ulong; VLAN_N_VID][BITS_TO_LONGS(HCLGE_VPORT_NUM)],
    pub vf_vlan_full: [c_ulong; BITS_TO_LONGS(HCLGE_VPORT_NUM)],
    pub vport_config_block: [c_ulong; BITS_TO_LONGS(HCLGE_VPORT_NUM)],
    pub fd_cfg: hclge_fd_cfg,
    pub fd_rule_list: hlist_head,
    pub /: *mut *mut spinlock_t fd_rule_lock; / protect fd_rule_list and fd_bmap,
    pub hclge_fd_rule_num: u16,
    pub serv_processed_cnt: c_ulong,
    pub last_serv_processed: c_ulong,
    pub last_rst_scheduled: c_ulong,
    pub last_mbx_scheduled: c_ulong,
    pub fd_bmap: [c_ulong; BITS_TO_LONGS(MAX_FD_FILTER_NUM)],
    pub fd_active_type: HCLGE_FD_ACTIVE_RULE_TYPE,
    pub fd_en: u8,
    pub gro_en: bool,
    pub wanted_umv_size: u16,
// max available unicast mac vlan space
    pub max_umv_size: u16,
// private unicast mac vlan space, it's same for PF and its VFs
    pub priv_umv_size: u16,
// unicast mac vlan space shared by PF and its VFs
    pub share_umv_size: u16,
// multicast mac address number used by PF and its VFs
    pub used_mc_mac_num: u16,
    pub ptp: *mut hclge_ptp,
    pub devlink: *mut devlink,
    pub rss_cfg: hclge_comm_rss_cfg,
}

// VPort level vlan tag configuration for TX direction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_tx_vtag_cfg {
    pub /: *mut *mut bool accept_tag1; / Whether accept tag1 packet from host,
    pub /: *mut *mut bool accept_untag1; / Whether accept untag1 packet from host,
    pub accept_tag2: bool,
    pub accept_untag2: bool,
    pub /: *mut *mut bool insert_tag1_en; / Whether insert inner vlan tag,
    pub /: *mut *mut bool insert_tag2_en; / Whether insert outer vlan tag,
    pub /: *mut *mut u16 default_tag1; / The default inner vlan tag to insert,
    pub /: *mut *mut u16 default_tag2; / The default outer vlan tag to insert,
    pub tag_shift_mode_en: bool,
}

// VPort level vlan tag configuration for RX direction
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_rx_vtag_cfg {
    pub /: *mut *mut bool rx_vlan_offload_en; / Whether enable rx vlan offload,
    pub /: *mut *mut bool strip_tag1_en; / Whether strip inner vlan tag,
    pub /: *mut *mut bool strip_tag2_en; / Whether strip outer vlan tag,
    pub /: *mut *mut bool vlan1_vlan_prionly; / Inner vlan tag up to descriptor enable,
    pub /: *mut *mut bool vlan2_vlan_prionly; / Outer vlan tag up to descriptor enable,
    pub /: *mut *mut bool strip_tag1_discard_en; / Inner vlan tag discard for BD enable,
    pub /: *mut *mut bool strip_tag2_discard_en; / Outer vlan tag discard for BD enable,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_VPORT_STATE {
    HCLGE_VPORT_STATE_ALIVE,
    HCLGE_VPORT_STATE_MAC_TBL_CHANGE,
    HCLGE_VPORT_STATE_PROMISC_CHANGE,
    HCLGE_VPORT_STATE_VLAN_FLTR_CHANGE,
    HCLGE_VPORT_STATE_INITED,
    HCLGE_VPORT_STATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HCLGE_VPORT_NEED_NOTIFY {
    HCLGE_VPORT_NEED_NOTIFY_RESET,
    HCLGE_VPORT_NEED_NOTIFY_VF_VLAN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vlan_info {
    pub /: *mut *mut u16 vlan_proto; / so far support 802.1Q only,
    pub qos: u16,
    pub vlan_tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_port_base_vlan_config {
    pub state: u16,
    pub tbl_sta: bool,
    pub vlan_info: hclge_vlan_info,
    pub old_vlan_info: hclge_vlan_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vf_info {
    pub link_state: c_int,
    pub mac: [u8; ETH_ALEN],
    pub spoofchk: u32,
    pub max_tx_rate: u32,
    pub trusted: u32,
    pub request_uc_en: u8,
    pub request_mc_en: u8,
    pub request_bc_en: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_vport {
    pub /: *mut *mut u16 alloc_tqps; / Allocated Tx/Rx queues,
    pub qs_offset: u16,
    pub /: *mut *mut u32 bw_limit; / VSI BW Limit (0 = disabled),
    pub dwrr: u8,
    pub req_vlan_fltr_en: bool,
    pub cur_vlan_fltr_en: bool,
    pub vlan_del_fail_bmap: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub port_base_vlan_cfg: hclge_port_base_vlan_config,
    pub txvlan_cfg: hclge_tx_vtag_cfg,
    pub rxvlan_cfg: hclge_rx_vtag_cfg,
    pub used_umv_num: u16,
    pub vport_id: u16,
    pub /: *mut *mut *mut hclge_dev back; / Back reference to associated dev,
    pub nic: hnae3_handle,
    pub roce: hnae3_handle,
    pub state: c_ulong,
    pub need_notify: c_ulong,
    pub last_active_jiffies: c_ulong,
    pub /: *mut *mut u32 mps; / Max packet size,
    pub vf_info: hclge_vf_info,
    pub overflow_promisc_flags: u8,
    pub last_promisc_flags: u8,
    pub /: *mut *mut spinlock_t mac_list_lock; / protect mac address need to add/detele,
    pub /: *mut *mut list_head uc_mac_list; / Store VF unicast table,
    pub /: *mut *mut list_head mc_mac_list; / Store VF multicast table,
    pub /: *mut *mut list_head vlan_list; / Store VF vlan table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_speed_bit_map {
    pub speed: u32,
    pub speed_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_mac_speed_map {
    pub /: *mut *mut u32 speed_drv; / speed defined in driver,
    pub /: *mut *mut u32 speed_fw; / speed defined in firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hclge_link_mode_bmap {
    pub support_bit: u16,
    pub link_mode: ethtool_link_mode_bit_indices,
}

extern "C" {
    pub fn hclge_inform_reset_assert_to_vf(vport: *mut hclge_vport) -> c_int;
}
extern "C" {
    pub fn hclge_cfg_mac_speed_dup(hdev: *mut hclge_dev, speed: c_int, duplex: u8, lane_num: u8) -> c_int;
}
extern "C" {
    pub fn hclge_en_hw_strip_rxvtag(handle: *mut hnae3_handle, enable: bool) -> c_int;
}
extern "C" {
    pub fn hclge_buffer_alloc(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_rss_init_hw(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_mbx_handler(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_reset_tqp(handle: *mut hnae3_handle) -> c_int;
}
extern "C" {
    pub fn hclge_cfg_flowctrl(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_func_reset_cmd(hdev: *mut hclge_dev, func_id: c_int) -> c_int;
}
extern "C" {
    pub fn hclge_vport_start(vport: *mut hclge_vport) -> c_int;
}
extern "C" {
    pub fn hclge_vport_stop(vport: *mut hclge_vport);
}
extern "C" {
    pub fn hclge_set_vport_mtu(vport: *mut hclge_vport, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn hclge_covert_handle_qid_global(handle: *mut hnae3_handle, queue_id: u16) -> u16;
}
extern "C" {
    pub fn hclge_rm_vport_all_vlan_table(vport: *mut hclge_vport, is_del_list: bool);
}
extern "C" {
    pub fn hclge_uninit_vport_vlan_table(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_restore_mac_table_common(vport: *mut hclge_vport);
}
extern "C" {
    pub fn hclge_restore_vport_port_base_vlan_config(hdev: *mut hclge_dev);
}
extern "C" {
    pub fn hclge_restore_vport_vlan_table(vport: *mut hclge_vport);
}
extern "C" {
    pub fn hclge_task_schedule(hdev: *mut hclge_dev, delay_time: c_ulong);
}
extern "C" {
    pub fn hclge_dbg_dump_rst_info(hdev: *mut hclge_dev, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn hclge_push_vf_link_status(vport: *mut hclge_vport) -> c_int;
}
extern "C" {
    pub fn hclge_enable_vport_vlan_filter(vport: *mut hclge_vport, request_en: bool) -> c_int;
}
extern "C" {
    pub fn hclge_mac_update_stats(hdev: *mut hclge_dev) -> c_int;
}
extern "C" {
    pub fn hclge_inform_vf_reset(vport: *mut hclge_vport, reset_type: u16) -> c_int;
}
extern "C" {
    pub fn hclge_query_scc_version(hdev: *mut hclge_dev, scc_version: *mut u32) -> c_int;
}
