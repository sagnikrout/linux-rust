//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_mfw_hsi.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2019-2021 Marvell International Ltd.
//
pub const MFW_TRACE_SIGNATURE: c_uint = 0x25071946;
// The trace in the buffer
pub const MFW_TRACE_EVENTID_MASK: c_uint = 0x00ffff;
pub const MFW_TRACE_PRM_SIZE_MASK: c_uint = 0x0f0000;
pub const MFW_TRACE_PRM_SIZE_OFFSET: c_int = 16;
pub const MFW_TRACE_ENTRY_SIZE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_trace {
    pub /: *mut *mut u32 signature; / Help to identify that the trace is valid,
    pub /: *mut *mut u32 size; / the size of the trace buffer in bytes,
    pub buffer: *mut *mut u32 curr_level; / 2 - all will be written to the,
// 1 - debug trace will not be written
// 0 - just errors will be written to the buffer
//
    pub means: *mut *mut u32 modules_mask[2]; / a bit per module, 1 means write it, 0,
// mask it.
//
// Warning: the following pointers are assumed to be 32bits as they are
// used only in the MFW.
//
    pub /: *mut *mut u32 trace_prod; / The next trace will be written to this offset,
    pub offset: *mut *mut u32 trace_oldest; / The oldest valid trace starts at this,
// (usually very close after the current producer).
//
}

pub const VF_MAX_STATIC: c_int = 192;

pub const EXT_VF_MAX_STATIC: c_int = 240;

pub const ADDED_VF_BITMAP_SIZE: c_int = 2;
pub const MCP_GLOB_PATH_MAX: c_int = 2;
pub const MCP_PORT_MAX: c_int = 2;
pub const MCP_GLOB_PORT_MAX: c_int = 4;
pub const MCP_GLOB_FUNC_MAX: c_int = 16;
// Offset from the beginning of the MCP scratchpad
pub const OFFSIZE_OFFSET_SHIFT: c_int = 0;
pub const OFFSIZE_OFFSET_MASK: c_uint = 0x0000ffff;
// Size of specific element (not the whole array if any)
pub const OFFSIZE_SIZE_SHIFT: c_int = 16;
pub const OFFSIZE_SIZE_MASK: c_uint = 0xffff0000;

// PHY configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_phy_cfg {
    pub speed: u32,
pub const ETH_SPEED_AUTONEG: c_uint = 0x0;
pub const ETH_SPEED_SMARTLINQ: c_uint = 0x8;
    pub pause: u32,
pub const ETH_PAUSE_NONE: c_uint = 0x0;
pub const ETH_PAUSE_AUTONEG: c_uint = 0x1;
pub const ETH_PAUSE_RX: c_uint = 0x2;
pub const ETH_PAUSE_TX: c_uint = 0x4;
    pub adv_speed: u32,
    pub loopback_mode: u32,
pub const ETH_LOOPBACK_NONE: c_uint = 0x0;
pub const ETH_LOOPBACK_INT_PHY: c_uint = 0x1;
pub const ETH_LOOPBACK_EXT_PHY: c_uint = 0x2;
pub const ETH_LOOPBACK_EXT: c_uint = 0x3;
pub const ETH_LOOPBACK_MAC: c_uint = 0x4;
pub const ETH_LOOPBACK_CNIG_AH_ONLY_0123: c_uint = 0x5;
pub const ETH_LOOPBACK_CNIG_AH_ONLY_2301: c_uint = 0x6;
pub const ETH_LOOPBACK_PCS_AH_ONLY: c_uint = 0x7;
pub const ETH_LOOPBACK_REVERSE_MAC_AH_ONLY: c_uint = 0x8;
pub const ETH_LOOPBACK_INT_PHY_FEA_AH_ONLY: c_uint = 0x9;
    pub eee_cfg: u32,

pub const EEE_TX_TIMER_USEC_MASK: c_uint = 0xfffffff0;
pub const EEE_TX_TIMER_USEC_OFFSET: c_int = 4;
pub const EEE_TX_TIMER_USEC_BALANCED_TIME: c_uint = 0xa00;
pub const EEE_TX_TIMER_USEC_AGGRESSIVE_TIME: c_uint = 0x100;
pub const EEE_TX_TIMER_USEC_LATENCY_TIME: c_uint = 0x6000;
    pub link_modes: u32,
    pub fec_mode: u32,
pub const FEC_FORCE_MODE_MASK: c_uint = 0x000000ff;
pub const FEC_FORCE_MODE_OFFSET: c_int = 0;
pub const FEC_FORCE_MODE_NONE: c_uint = 0x00;
pub const FEC_FORCE_MODE_FIRECODE: c_uint = 0x01;
pub const FEC_FORCE_MODE_RS: c_uint = 0x02;
pub const FEC_FORCE_MODE_AUTO: c_uint = 0x07;
pub const FEC_EXTENDED_MODE_MASK: c_uint = 0xffffff00;
pub const FEC_EXTENDED_MODE_OFFSET: c_int = 8;
pub const ETH_EXT_FEC_NONE: c_uint = 0x00000000;
pub const ETH_EXT_FEC_10G_NONE: c_uint = 0x00000100;
pub const ETH_EXT_FEC_10G_BASE_R: c_uint = 0x00000200;
pub const ETH_EXT_FEC_25G_NONE: c_uint = 0x00000400;
pub const ETH_EXT_FEC_25G_BASE_R: c_uint = 0x00000800;
pub const ETH_EXT_FEC_25G_RS528: c_uint = 0x00001000;
pub const ETH_EXT_FEC_40G_NONE: c_uint = 0x00002000;
pub const ETH_EXT_FEC_40G_BASE_R: c_uint = 0x00004000;
pub const ETH_EXT_FEC_50G_NONE: c_uint = 0x00008000;
pub const ETH_EXT_FEC_50G_BASE_R: c_uint = 0x00010000;
pub const ETH_EXT_FEC_50G_RS528: c_uint = 0x00020000;
pub const ETH_EXT_FEC_50G_RS544: c_uint = 0x00040000;
pub const ETH_EXT_FEC_100G_NONE: c_uint = 0x00080000;
pub const ETH_EXT_FEC_100G_BASE_R: c_uint = 0x00100000;
pub const ETH_EXT_FEC_100G_RS528: c_uint = 0x00200000;
pub const ETH_EXT_FEC_100G_RS544: c_uint = 0x00400000;
    pub extended_speed: u32,
pub const ETH_EXT_SPEED_MASK: c_uint = 0x0000ffff;
pub const ETH_EXT_SPEED_OFFSET: c_int = 0;
pub const ETH_EXT_SPEED_NONE: c_uint = 0x00000001;
pub const ETH_EXT_SPEED_1G: c_uint = 0x00000002;
pub const ETH_EXT_SPEED_10G: c_uint = 0x00000004;
pub const ETH_EXT_SPEED_25G: c_uint = 0x00000008;
pub const ETH_EXT_SPEED_40G: c_uint = 0x00000010;
pub const ETH_EXT_SPEED_50G_BASE_R: c_uint = 0x00000020;
pub const ETH_EXT_SPEED_50G_BASE_R2: c_uint = 0x00000040;
pub const ETH_EXT_SPEED_100G_BASE_R2: c_uint = 0x00000080;
pub const ETH_EXT_SPEED_100G_BASE_R4: c_uint = 0x00000100;
pub const ETH_EXT_SPEED_100G_BASE_P4: c_uint = 0x00000200;
pub const ETH_EXT_ADV_SPEED_MASK: c_uint = 0xFFFF0000;
pub const ETH_EXT_ADV_SPEED_OFFSET: c_int = 16;
pub const ETH_EXT_ADV_SPEED_1G: c_uint = 0x00010000;
pub const ETH_EXT_ADV_SPEED_10G: c_uint = 0x00020000;
pub const ETH_EXT_ADV_SPEED_25G: c_uint = 0x00040000;
pub const ETH_EXT_ADV_SPEED_40G: c_uint = 0x00080000;
pub const ETH_EXT_ADV_SPEED_50G_BASE_R: c_uint = 0x00100000;
pub const ETH_EXT_ADV_SPEED_50G_BASE_R2: c_uint = 0x00200000;
pub const ETH_EXT_ADV_SPEED_100G_BASE_R2: c_uint = 0x00400000;
pub const ETH_EXT_ADV_SPEED_100G_BASE_R4: c_uint = 0x00800000;
pub const ETH_EXT_ADV_SPEED_100G_BASE_P4: c_uint = 0x01000000;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_mf_cfg {
    pub dynamic_cfg: u32,
pub const PORT_MF_CFG_OV_TAG_MASK: c_uint = 0x0000ffff;
pub const PORT_MF_CFG_OV_TAG_SHIFT: c_int = 0;
    pub reserved: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth_stats {
    pub r64: u64,
    pub r127: u64,
    pub r255: u64,
    pub r511: u64,
    pub r1023: u64,
    pub r1518: u64,
    pub r1522: u64,
    pub r2047: u64,
    pub r4095: u64,
    pub r9216: u64,
    pub r16383: u64,
    pub bb0: },
    pub unused1: u64,
    pub r1519_to_max: u64,
    pub unused2: u64,
    pub unused3: u64,
    pub unused4: u64,
    pub ah0: },
    pub u0: },
    pub rfcs: u64,
    pub rxcf: u64,
    pub rxpf: u64,
    pub rxpp: u64,
    pub raln: u64,
    pub rfcr: u64,
    pub rovr: u64,
    pub rjbr: u64,
    pub rund: u64,
    pub rfrg: u64,
    pub t64: u64,
    pub t127: u64,
    pub t255: u64,
    pub t511: u64,
    pub t1023: u64,
    pub t1518: u64,
    pub t2047: u64,
    pub t4095: u64,
    pub t9216: u64,
    pub t16383: u64,
    pub bb1: },
    pub t1519_to_max: u64,
    pub unused6: u64,
    pub unused7: u64,
    pub unused8: u64,
    pub ah1: },
    pub u1: },
    pub txpf: u64,
    pub txpp: u64,
    pub tlpiec: u64,
    pub tncl: u64,
    pub bb2: },
    pub unused9: u64,
    pub unused10: u64,
    pub ah2: },
    pub u2: },
    pub rbyte: u64,
    pub rxuca: u64,
    pub rxmca: u64,
    pub rxbca: u64,
    pub rxpok: u64,
    pub tbyte: u64,
    pub txuca: u64,
    pub txmca: u64,
    pub txbca: u64,
    pub txcf: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_type_cnt {
    pub tc_tx_pkt_cnt: [u64; 8],
    pub tc_tx_oct_cnt: [u64; 8],
    pub priority_rx_pkt_cnt: [u64; 8],
    pub priority_rx_oct_cnt: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brb_stats {
    pub brb_truncate: [u64; 8],
    pub brb_discard: [u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct port_stats {
    pub brb: brb_stats,
    pub eth: eth_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct couple_mode_teaming {
    pub port_cmt: [u8; MCP_GLOB_PORT_MAX],
}

pub const LLDP_CHASSIS_ID_STAT_LEN: c_int = 4;
pub const LLDP_PORT_ID_STAT_LEN: c_int = 4;
pub const DCBX_MAX_APP_PROTOCOL: c_int = 32;
pub const MAX_SYSTEM_LLDP_TLV_DATA: c_int = 32;
pub const MAX_TLV_BUFFER: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _lldp_agent {
    LLDP_NEAREST_BRIDGE = 0,
    LLDP_NEAREST_NON_TPMR_BRIDGE,
    LLDP_NEAREST_CUSTOMER_BRIDGE,
    LLDP_MAX_LLDP_AGENTS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_config_params_s {
    pub config: u32,
pub const LLDP_CONFIG_TX_INTERVAL_MASK: c_uint = 0x000000ff;
pub const LLDP_CONFIG_TX_INTERVAL_SHIFT: c_int = 0;
pub const LLDP_CONFIG_HOLD_MASK: c_uint = 0x00000f00;
pub const LLDP_CONFIG_HOLD_SHIFT: c_int = 8;
pub const LLDP_CONFIG_MAX_CREDIT_MASK: c_uint = 0x0000f000;
pub const LLDP_CONFIG_MAX_CREDIT_SHIFT: c_int = 12;
pub const LLDP_CONFIG_ENABLE_RX_MASK: c_uint = 0x40000000;
pub const LLDP_CONFIG_ENABLE_RX_SHIFT: c_int = 30;
pub const LLDP_CONFIG_ENABLE_TX_MASK: c_uint = 0x80000000;
pub const LLDP_CONFIG_ENABLE_TX_SHIFT: c_int = 31;
    pub local_chassis_id: [u32; LLDP_CHASSIS_ID_STAT_LEN],
    pub local_port_id: [u32; LLDP_PORT_ID_STAT_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_status_params_s {
    pub prefix_seq_num: u32,
    pub status: u32,
    pub peer_chassis_id: [u32; LLDP_CHASSIS_ID_STAT_LEN],
    pub peer_port_id: [u32; LLDP_PORT_ID_STAT_LEN],
    pub suffix_seq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_ets_feature {
    pub flags: u32,
pub const DCBX_ETS_ENABLED_MASK: c_uint = 0x00000001;
pub const DCBX_ETS_ENABLED_SHIFT: c_int = 0;
pub const DCBX_ETS_WILLING_MASK: c_uint = 0x00000002;
pub const DCBX_ETS_WILLING_SHIFT: c_int = 1;
pub const DCBX_ETS_ERROR_MASK: c_uint = 0x00000004;
pub const DCBX_ETS_ERROR_SHIFT: c_int = 2;
pub const DCBX_ETS_CBS_MASK: c_uint = 0x00000008;
pub const DCBX_ETS_CBS_SHIFT: c_int = 3;
pub const DCBX_ETS_MAX_TCS_MASK: c_uint = 0x000000f0;
pub const DCBX_ETS_MAX_TCS_SHIFT: c_int = 4;
pub const DCBX_OOO_TC_MASK: c_uint = 0x00000f00;
pub const DCBX_OOO_TC_SHIFT: c_int = 8;
    pub pri_tc_tbl: [u32; 1],
pub const DCBX_CEE_STRICT_PRIORITY: c_uint = 0xf;
    pub tc_bw_tbl: [u32; 2],
    pub tc_tsa_tbl: [u32; 2],
pub const DCBX_ETS_TSA_STRICT: c_int = 0;
pub const DCBX_ETS_TSA_CBS: c_int = 1;
pub const DCBX_ETS_TSA_ETS: c_int = 2;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_app_priority_entry {
    pub entry: u32,
pub const DCBX_APP_PRI_MAP_MASK: c_uint = 0x000000ff;
pub const DCBX_APP_PRI_MAP_SHIFT: c_int = 0;
pub const DCBX_APP_PRI_0: c_uint = 0x01;
pub const DCBX_APP_PRI_1: c_uint = 0x02;
pub const DCBX_APP_PRI_2: c_uint = 0x04;
pub const DCBX_APP_PRI_3: c_uint = 0x08;
pub const DCBX_APP_PRI_4: c_uint = 0x10;
pub const DCBX_APP_PRI_5: c_uint = 0x20;
pub const DCBX_APP_PRI_6: c_uint = 0x40;
pub const DCBX_APP_PRI_7: c_uint = 0x80;
pub const DCBX_APP_SF_MASK: c_uint = 0x00000300;
pub const DCBX_APP_SF_SHIFT: c_int = 8;
pub const DCBX_APP_SF_ETHTYPE: c_int = 0;
pub const DCBX_APP_SF_PORT: c_int = 1;
pub const DCBX_APP_SF_IEEE_MASK: c_uint = 0x0000f000;
pub const DCBX_APP_SF_IEEE_SHIFT: c_int = 12;
pub const DCBX_APP_SF_IEEE_RESERVED: c_int = 0;
pub const DCBX_APP_SF_IEEE_ETHTYPE: c_int = 1;
pub const DCBX_APP_SF_IEEE_TCP_PORT: c_int = 2;
pub const DCBX_APP_SF_IEEE_UDP_PORT: c_int = 3;
pub const DCBX_APP_SF_IEEE_TCP_UDP_PORT: c_int = 4;
pub const DCBX_APP_PROTOCOL_ID_MASK: c_uint = 0xffff0000;
pub const DCBX_APP_PROTOCOL_ID_SHIFT: c_int = 16;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_app_priority_feature {
    pub flags: u32,
pub const DCBX_APP_ENABLED_MASK: c_uint = 0x00000001;
pub const DCBX_APP_ENABLED_SHIFT: c_int = 0;
pub const DCBX_APP_WILLING_MASK: c_uint = 0x00000002;
pub const DCBX_APP_WILLING_SHIFT: c_int = 1;
pub const DCBX_APP_ERROR_MASK: c_uint = 0x00000004;
pub const DCBX_APP_ERROR_SHIFT: c_int = 2;
pub const DCBX_APP_MAX_TCS_MASK: c_uint = 0x0000f000;
pub const DCBX_APP_MAX_TCS_SHIFT: c_int = 12;
pub const DCBX_APP_NUM_ENTRIES_MASK: c_uint = 0x00ff0000;
pub const DCBX_APP_NUM_ENTRIES_SHIFT: c_int = 16;
    pub app_pri_tbl: [dcbx_app_priority_entry; DCBX_MAX_APP_PROTOCOL],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_features {
    pub ets: dcbx_ets_feature,
    pub pfc: u32,
pub const DCBX_PFC_PRI_EN_BITMAP_MASK: c_uint = 0x000000ff;
pub const DCBX_PFC_PRI_EN_BITMAP_SHIFT: c_int = 0;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_0: c_uint = 0x01;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_1: c_uint = 0x02;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_2: c_uint = 0x04;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_3: c_uint = 0x08;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_4: c_uint = 0x10;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_5: c_uint = 0x20;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_6: c_uint = 0x40;
pub const DCBX_PFC_PRI_EN_BITMAP_PRI_7: c_uint = 0x80;
pub const DCBX_PFC_FLAGS_MASK: c_uint = 0x0000ff00;
pub const DCBX_PFC_FLAGS_SHIFT: c_int = 8;
pub const DCBX_PFC_CAPS_MASK: c_uint = 0x00000f00;
pub const DCBX_PFC_CAPS_SHIFT: c_int = 8;
pub const DCBX_PFC_MBC_MASK: c_uint = 0x00004000;
pub const DCBX_PFC_MBC_SHIFT: c_int = 14;
pub const DCBX_PFC_WILLING_MASK: c_uint = 0x00008000;
pub const DCBX_PFC_WILLING_SHIFT: c_int = 15;
pub const DCBX_PFC_ENABLED_MASK: c_uint = 0x00010000;
pub const DCBX_PFC_ENABLED_SHIFT: c_int = 16;
pub const DCBX_PFC_ERROR_MASK: c_uint = 0x00020000;
pub const DCBX_PFC_ERROR_SHIFT: c_int = 17;
    pub app: dcbx_app_priority_feature,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_local_params {
    pub config: u32,
pub const DCBX_CONFIG_VERSION_MASK: c_uint = 0x00000007;
pub const DCBX_CONFIG_VERSION_SHIFT: c_int = 0;
pub const DCBX_CONFIG_VERSION_DISABLED: c_int = 0;
pub const DCBX_CONFIG_VERSION_IEEE: c_int = 1;
pub const DCBX_CONFIG_VERSION_CEE: c_int = 2;
pub const DCBX_CONFIG_VERSION_STATIC: c_int = 4;
    pub flags: u32,
    pub features: dcbx_features,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcbx_mib {
    pub prefix_seq_num: u32,
    pub flags: u32,
    pub features: dcbx_features,
    pub suffix_seq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_system_tlvs_buffer_s {
    pub flags: u32,
pub const LLDP_SYSTEM_TLV_VALID_MASK: c_uint = 0x1;
pub const LLDP_SYSTEM_TLV_VALID_OFFSET: c_int = 0;
pub const LLDP_SYSTEM_TLV_MANDATORY_MASK: c_uint = 0x2;
pub const LLDP_SYSTEM_TLV_MANDATORY_SHIFT: c_int = 1;
pub const LLDP_SYSTEM_TLV_LENGTH_MASK: c_uint = 0xffff0000;
pub const LLDP_SYSTEM_TLV_LENGTH_SHIFT: c_int = 16;
    pub data: [u32; MAX_SYSTEM_LLDP_TLV_DATA],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_received_tlvs_s {
    pub prefix_seq_num: u32,
    pub length: u32,
    pub tlvs_buffer: [u32; MAX_TLV_BUFFER],
    pub suffix_seq_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcb_dscp_map {
    pub flags: u32,
pub const DCB_DSCP_ENABLE_MASK: c_uint = 0x1;
pub const DCB_DSCP_ENABLE_SHIFT: c_int = 0;
pub const DCB_DSCP_ENABLE: c_int = 1;
    pub dscp_pri_map: [u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_val64 {
    pub lo: u32,
    pub hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct generic_idc_msg_s {
    pub source_pf: u32,
    pub msg: mcp_val64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcie_stats_stc {
    pub sr_cnt_wr_byte_msb: u32,
    pub sr_cnt_wr_byte_lsb: u32,
    pub sr_cnt_wr_cnt: u32,
    pub sr_cnt_rd_byte_msb: u32,
    pub sr_cnt_rd_byte_lsb: u32,
    pub sr_cnt_rd_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _attribute_commands_e {
    ATTRIBUTE_CMD_READ = 0,
    ATTRIBUTE_CMD_WRITE,
    ATTRIBUTE_CMD_READ_CLEAR,
    ATTRIBUTE_CMD_CLEAR,
    ATTRIBUTE_NUM_OF_COMMANDS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_global {
    pub max_path: u32,
    pub max_ports: u32,
pub const MODE_1P: c_int = 1;
pub const MODE_2P: c_int = 2;
pub const MODE_3P: c_int = 3;
pub const MODE_4P: c_int = 4;
    pub debug_mb_offset: u32,
    pub phymod_dbg_mb_offset: u32,
    pub cmt: couple_mode_teaming,
    pub internal_temperature: i32,
    pub mfw_ver: u32,
    pub running_bundle_id: u32,
    pub external_temperature: i32,
    pub mdump_reason: u32,
    pub ext_phy_upgrade_fw: u32,
    pub runtime_port_swap_map: [u8; MODE_4P],
    pub data_ptr: u32,
    pub data_size: u32,
    pub bmb_error_status_cnt: u32,
    pub bmb_jumbo_frame_cnt: u32,
    pub sent_to_bmc_cnt: u32,
    pub handled_by_mfw: u32,
    pub sent_to_nw_cnt: u32,
    pub to_bmc_kb_per_second: u32,
    pub bcast_dropped_to_bmc_cnt: u32,
    pub mcast_dropped_to_bmc_cnt: u32,
    pub ucast_dropped_to_bmc_cnt: u32,
    pub ncsi_response_failure_cnt: u32,
    pub device_attr: u32,
    pub vpd_warning: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_flr_mb {
    pub aggint: u32,
    pub opgen_addr: u32,
    pub accum_ack: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_path {
    pub flr_mb: fw_flr_mb,
    pub 32]: u32 mcp_vf_disabled[VF_MAX_STATIC /,
    pub process_kill: u32,
pub const PROCESS_KILL_COUNTER_MASK: c_uint = 0x0000ffff;
pub const PROCESS_KILL_COUNTER_SHIFT: c_int = 0;
pub const PROCESS_KILL_GLOB_AEU_BIT_MASK: c_uint = 0xffff0000;
pub const PROCESS_KILL_GLOB_AEU_BIT_SHIFT: c_int = 16;

}

pub const FC_NPIV_WWPN_SIZE: c_int = 8;
pub const FC_NPIV_WWNN_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dci_npiv_settings {
    pub npiv_wwpn: [u8; FC_NPIV_WWPN_SIZE],
    pub npiv_wwnn: [u8; FC_NPIV_WWNN_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dci_fc_npiv_cfg {
// hdr used internally by the MFW
    pub hdr: u32,
    pub num_of_npiv: u32,
}

pub const MAX_NUMBER_NPIV: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dci_fc_npiv_tbl {
    pub fc_npiv_cfg: dci_fc_npiv_cfg,
    pub settings: [dci_npiv_settings; MAX_NUMBER_NPIV],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pause_flood_monitor {
    pub period_cnt: u8,
    pub any_brb_prs_packet_hist: u8,
    pub any_brb_block_is_full_hist: u8,
    pub flags: u8,
    pub num_of_state_changes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_port {
    pub validity_map: u32,
    pub link_status: u32,
pub const LINK_STATUS_LINK_UP: c_uint = 0x00000001;
pub const LINK_STATUS_SPEED_AND_DUPLEX_MASK: c_uint = 0x0000001e;

pub const LINK_STATUS_AUTO_NEGOTIATE_ENABLED: c_uint = 0x00000020;
pub const LINK_STATUS_AUTO_NEGOTIATE_COMPLETE: c_uint = 0x00000040;
pub const LINK_STATUS_PARALLEL_DETECTION_USED: c_uint = 0x00000080;
pub const LINK_STATUS_PFC_ENABLED: c_uint = 0x00000100;
pub const LINK_STATUS_LINK_PARTNER_1000TFD_CAPABLE: c_uint = 0x00000200;
pub const LINK_STATUS_LINK_PARTNER_1000THD_CAPABLE: c_uint = 0x00000400;
pub const LINK_STATUS_LINK_PARTNER_10G_CAPABLE: c_uint = 0x00000800;
pub const LINK_STATUS_LINK_PARTNER_20G_CAPABLE: c_uint = 0x00001000;
pub const LINK_STATUS_LINK_PARTNER_40G_CAPABLE: c_uint = 0x00002000;
pub const LINK_STATUS_LINK_PARTNER_50G_CAPABLE: c_uint = 0x00004000;
pub const LINK_STATUS_LINK_PARTNER_100G_CAPABLE: c_uint = 0x00008000;
pub const LINK_STATUS_LINK_PARTNER_25G_CAPABLE: c_uint = 0x00010000;
pub const LINK_STATUS_LINK_PARTNER_FLOW_CONTROL_MASK: c_uint = 0x000c0000;

pub const LINK_STATUS_SFP_TX_FAULT: c_uint = 0x00100000;
pub const LINK_STATUS_TX_FLOW_CONTROL_ENABLED: c_uint = 0x00200000;
pub const LINK_STATUS_RX_FLOW_CONTROL_ENABLED: c_uint = 0x00400000;
pub const LINK_STATUS_RX_SIGNAL_PRESENT: c_uint = 0x00800000;
pub const LINK_STATUS_MAC_LOCAL_FAULT: c_uint = 0x01000000;
pub const LINK_STATUS_MAC_REMOTE_FAULT: c_uint = 0x02000000;
pub const LINK_STATUS_UNSUPPORTED_SPD_REQ: c_uint = 0x04000000;
pub const LINK_STATUS_FEC_MODE_MASK: c_uint = 0x38000000;

    pub link_status1: u32,
    pub ext_phy_fw_version: u32,
    pub drv_phy_cfg_addr: u32,
    pub port_stx: u32,
    pub stat_nig_timer: u32,
    pub port_mf_config: port_mf_cfg,
    pub stats: port_stats,
    pub media_type: u32,
pub const MEDIA_UNSPECIFIED: c_uint = 0x0;
pub const MEDIA_SFPP_10G_FIBER: c_uint = 0x1;
pub const MEDIA_XFP_FIBER: c_uint = 0x2;
pub const MEDIA_DA_TWINAX: c_uint = 0x3;
pub const MEDIA_BASE_T: c_uint = 0x4;
pub const MEDIA_SFP_1G_FIBER: c_uint = 0x5;
pub const MEDIA_MODULE_FIBER: c_uint = 0x6;
pub const MEDIA_KR: c_uint = 0xf0;
pub const MEDIA_NOT_PRESENT: c_uint = 0xff;
    pub lfa_status: u32,
    pub link_change_count: u32,
    pub lldp_config_params: [lldp_config_params_s; LLDP_MAX_LLDP_AGENTS],
    pub lldp_status_params: [lldp_status_params_s; LLDP_MAX_LLDP_AGENTS],
    pub system_lldp_tlvs_buf: lldp_system_tlvs_buffer_s,
// DCBX related MIB
    pub local_admin_dcbx_mib: dcbx_local_params,
    pub remote_dcbx_mib: dcbx_mib,
    pub operational_dcbx_mib: dcbx_mib,
    pub fc_npiv_nvram_tbl_addr: u32,
    pub fc_npiv_nvram_tbl_size: u32,
    pub transceiver_data: u32,
pub const ETH_TRANSCEIVER_STATE_MASK: c_uint = 0x000000ff;
pub const ETH_TRANSCEIVER_STATE_SHIFT: c_uint = 0x00000000;
pub const ETH_TRANSCEIVER_STATE_OFFSET: c_uint = 0x00000000;
pub const ETH_TRANSCEIVER_STATE_UNPLUGGED: c_uint = 0x00000000;
pub const ETH_TRANSCEIVER_STATE_PRESENT: c_uint = 0x00000001;
pub const ETH_TRANSCEIVER_STATE_VALID: c_uint = 0x00000003;
pub const ETH_TRANSCEIVER_STATE_UPDATING: c_uint = 0x00000008;
pub const ETH_TRANSCEIVER_STATE_IN_SETUP: c_uint = 0x10;
pub const ETH_TRANSCEIVER_TYPE_MASK: c_uint = 0x0000ff00;
pub const ETH_TRANSCEIVER_TYPE_OFFSET: c_uint = 0x8;
pub const ETH_TRANSCEIVER_TYPE_NONE: c_uint = 0x00;
pub const ETH_TRANSCEIVER_TYPE_UNKNOWN: c_uint = 0xff;
pub const ETH_TRANSCEIVER_TYPE_1G_PCC: c_uint = 0x01;
pub const ETH_TRANSCEIVER_TYPE_1G_ACC: c_uint = 0x02;
pub const ETH_TRANSCEIVER_TYPE_1G_LX: c_uint = 0x03;
pub const ETH_TRANSCEIVER_TYPE_1G_SX: c_uint = 0x04;
pub const ETH_TRANSCEIVER_TYPE_10G_SR: c_uint = 0x05;
pub const ETH_TRANSCEIVER_TYPE_10G_LR: c_uint = 0x06;
pub const ETH_TRANSCEIVER_TYPE_10G_LRM: c_uint = 0x07;
pub const ETH_TRANSCEIVER_TYPE_10G_ER: c_uint = 0x08;
pub const ETH_TRANSCEIVER_TYPE_10G_PCC: c_uint = 0x09;
pub const ETH_TRANSCEIVER_TYPE_10G_ACC: c_uint = 0x0a;
pub const ETH_TRANSCEIVER_TYPE_XLPPI: c_uint = 0x0b;
pub const ETH_TRANSCEIVER_TYPE_40G_LR4: c_uint = 0x0c;
pub const ETH_TRANSCEIVER_TYPE_40G_SR4: c_uint = 0x0d;
pub const ETH_TRANSCEIVER_TYPE_40G_CR4: c_uint = 0x0e;
pub const ETH_TRANSCEIVER_TYPE_100G_AOC: c_uint = 0x0f;
pub const ETH_TRANSCEIVER_TYPE_100G_SR4: c_uint = 0x10;
pub const ETH_TRANSCEIVER_TYPE_100G_LR4: c_uint = 0x11;
pub const ETH_TRANSCEIVER_TYPE_100G_ER4: c_uint = 0x12;
pub const ETH_TRANSCEIVER_TYPE_100G_ACC: c_uint = 0x13;
pub const ETH_TRANSCEIVER_TYPE_100G_CR4: c_uint = 0x14;
pub const ETH_TRANSCEIVER_TYPE_4x10G_SR: c_uint = 0x15;
pub const ETH_TRANSCEIVER_TYPE_25G_CA_N: c_uint = 0x16;
pub const ETH_TRANSCEIVER_TYPE_25G_ACC_S: c_uint = 0x17;
pub const ETH_TRANSCEIVER_TYPE_25G_CA_S: c_uint = 0x18;
pub const ETH_TRANSCEIVER_TYPE_25G_ACC_M: c_uint = 0x19;
pub const ETH_TRANSCEIVER_TYPE_25G_CA_L: c_uint = 0x1a;
pub const ETH_TRANSCEIVER_TYPE_25G_ACC_L: c_uint = 0x1b;
pub const ETH_TRANSCEIVER_TYPE_25G_SR: c_uint = 0x1c;
pub const ETH_TRANSCEIVER_TYPE_25G_LR: c_uint = 0x1d;
pub const ETH_TRANSCEIVER_TYPE_25G_AOC: c_uint = 0x1e;
pub const ETH_TRANSCEIVER_TYPE_4x10G: c_uint = 0x1f;
pub const ETH_TRANSCEIVER_TYPE_4x25G_CR: c_uint = 0x20;
pub const ETH_TRANSCEIVER_TYPE_1000BASET: c_uint = 0x21;
pub const ETH_TRANSCEIVER_TYPE_10G_BASET: c_uint = 0x22;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_10G_40G_SR: c_uint = 0x30;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_10G_40G_CR: c_uint = 0x31;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_10G_40G_LR: c_uint = 0x32;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_40G_100G_SR: c_uint = 0x33;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_40G_100G_CR: c_uint = 0x34;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_40G_100G_LR: c_uint = 0x35;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_40G_100G_AOC: c_uint = 0x36;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_10G_25G_SR: c_uint = 0x37;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_10G_25G_LR: c_uint = 0x38;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_1G_10G_SR: c_uint = 0x39;
pub const ETH_TRANSCEIVER_TYPE_MULTI_RATE_1G_10G_LR: c_uint = 0x3a;
    pub wol_info: u32,
    pub wol_pkt_len: u32,
    pub wol_pkt_details: u32,
    pub dcb_dscp_map: dcb_dscp_map,
    pub eee_status: u32,

pub const EEE_LD_ADV_STATUS_MASK: c_uint = 0x000000f0;
pub const EEE_LD_ADV_STATUS_OFFSET: c_int = 4;

pub const EEE_LP_ADV_STATUS_MASK: c_uint = 0x00000f00;
pub const EEE_LP_ADV_STATUS_OFFSET: c_int = 8;
pub const EEE_SUPPORTED_SPEED_MASK: c_uint = 0x0000f000;
pub const EEE_SUPPORTED_SPEED_OFFSET: c_int = 12;

    pub eee_remote: u32,
pub const EEE_REMOTE_TW_TX_MASK: c_uint = 0x0000ffff;
pub const EEE_REMOTE_TW_TX_OFFSET: c_int = 0;
pub const EEE_REMOTE_TW_RX_MASK: c_uint = 0xffff0000;
pub const EEE_REMOTE_TW_RX_OFFSET: c_int = 16;
    pub module_info: u32,
    pub oem_cfg_port: u32,
pub const OEM_CFG_CHANNEL_TYPE_MASK: c_uint = 0x00000003;
pub const OEM_CFG_CHANNEL_TYPE_OFFSET: c_int = 0;
pub const OEM_CFG_CHANNEL_TYPE_VLAN_PARTITION: c_uint = 0x1;
pub const OEM_CFG_CHANNEL_TYPE_STAGGED: c_uint = 0x2;
pub const OEM_CFG_SCHED_TYPE_MASK: c_uint = 0x0000000C;
pub const OEM_CFG_SCHED_TYPE_OFFSET: c_int = 2;
pub const OEM_CFG_SCHED_TYPE_ETS: c_uint = 0x1;
pub const OEM_CFG_SCHED_TYPE_VNIC_BW: c_uint = 0x2;
    pub lldp_received_tlvs: [lldp_received_tlvs_s; LLDP_MAX_LLDP_AGENTS],
    pub system_lldp_tlvs_buf2: [u32; MAX_SYSTEM_LLDP_TLV_DATA],
    pub phy_module_temperature: u32,
    pub nig_reg_stat_rx_bmb_packet: u32,
    pub nig_reg_rx_llh_ncsi_mcp_mask: u32,
    pub nig_reg_rx_llh_ncsi_mcp_mask_2: u32,
    pub pause_flood_monitor: pause_flood_monitor,
    pub nig_drain_cnt: u32,
    pub pkt_tc_priority_cnt: pkt_type_cnt,
}

pub const MCP_DRV_VER_STR_SIZE: c_int = 16;

pub const MCP_DRV_NVM_BUF_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_version_stc {
    pub version: u32,
    pub 4]: u8 name[MCP_DRV_VER_STR_SIZE -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_func {
    pub iscsi_boot_signature: u32,
    pub iscsi_boot_block_offset: u32,
    pub mtu_size: u32,
    pub c2s_pcp_map_lower: u32,
    pub c2s_pcp_map_upper: u32,
    pub c2s_pcp_map_default: u32,
    pub generic_idc_msg: generic_idc_msg_s,
    pub num_of_msix: u32,
    pub config: u32,
pub const FUNC_MF_CFG_FUNC_HIDE: c_uint = 0x00000001;
pub const FUNC_MF_CFG_PAUSE_ON_HOST_RING: c_uint = 0x00000002;
pub const FUNC_MF_CFG_PAUSE_ON_HOST_RING_SHIFT: c_uint = 0x00000001;
pub const FUNC_MF_CFG_PROTOCOL_MASK: c_uint = 0x000000f0;
pub const FUNC_MF_CFG_PROTOCOL_SHIFT: c_int = 4;
pub const FUNC_MF_CFG_PROTOCOL_ETHERNET: c_uint = 0x00000000;
pub const FUNC_MF_CFG_PROTOCOL_ISCSI: c_uint = 0x00000010;
pub const FUNC_MF_CFG_PROTOCOL_FCOE: c_uint = 0x00000020;
pub const FUNC_MF_CFG_PROTOCOL_ROCE: c_uint = 0x00000030;
pub const FUNC_MF_CFG_PROTOCOL_MAX: c_uint = 0x00000030;
pub const FUNC_MF_CFG_MIN_BW_MASK: c_uint = 0x0000ff00;
pub const FUNC_MF_CFG_MIN_BW_SHIFT: c_int = 8;
pub const FUNC_MF_CFG_MIN_BW_DEFAULT: c_uint = 0x00000000;
pub const FUNC_MF_CFG_MAX_BW_MASK: c_uint = 0x00ff0000;
pub const FUNC_MF_CFG_MAX_BW_SHIFT: c_int = 16;
pub const FUNC_MF_CFG_MAX_BW_DEFAULT: c_uint = 0x00640000;
    pub status: u32,
pub const FUNC_STATUS_VIRTUAL_LINK_UP: c_uint = 0x00000001;
    pub mac_upper: u32,
pub const FUNC_MF_CFG_UPPERMAC_MASK: c_uint = 0x0000ffff;
pub const FUNC_MF_CFG_UPPERMAC_SHIFT: c_int = 0;

    pub mac_lower: u32,
pub const FUNC_MF_CFG_LOWERMAC_DEFAULT: c_uint = 0xffffffff;
    pub fcoe_wwn_port_name_upper: u32,
    pub fcoe_wwn_port_name_lower: u32,
    pub fcoe_wwn_node_name_upper: u32,
    pub fcoe_wwn_node_name_lower: u32,
    pub ovlan_stag: u32,
pub const FUNC_MF_CFG_OV_STAG_MASK: c_uint = 0x0000ffff;
pub const FUNC_MF_CFG_OV_STAG_SHIFT: c_int = 0;

    pub pf_allocation: u32,
    pub preserve_data: u32,
    pub driver_last_activity_ts: u32,
    pub 32]: u32 drv_ack_vf_disabled[VF_MAX_STATIC /,
    pub drv_id: u32,
pub const DRV_ID_PDA_COMP_VER_MASK: c_uint = 0x0000ffff;
pub const DRV_ID_PDA_COMP_VER_SHIFT: c_int = 0;
pub const LOAD_REQ_HSI_VERSION: c_int = 2;
pub const DRV_ID_MCP_HSI_VER_MASK: c_uint = 0x00ff0000;
pub const DRV_ID_MCP_HSI_VER_SHIFT: c_int = 16;

pub const DRV_ID_DRV_TYPE_MASK: c_uint = 0x7f000000;
pub const DRV_ID_DRV_TYPE_SHIFT: c_int = 24;

pub const DRV_ID_DRV_INIT_HW_MASK: c_uint = 0x80000000;
pub const DRV_ID_DRV_INIT_HW_SHIFT: c_int = 31;

    pub oem_cfg_func: u32,
pub const OEM_CFG_FUNC_TC_MASK: c_uint = 0x0000000F;
pub const OEM_CFG_FUNC_TC_OFFSET: c_int = 0;
pub const OEM_CFG_FUNC_TC_0: c_uint = 0x0;
pub const OEM_CFG_FUNC_TC_1: c_uint = 0x1;
pub const OEM_CFG_FUNC_TC_2: c_uint = 0x2;
pub const OEM_CFG_FUNC_TC_3: c_uint = 0x3;
pub const OEM_CFG_FUNC_TC_4: c_uint = 0x4;
pub const OEM_CFG_FUNC_TC_5: c_uint = 0x5;
pub const OEM_CFG_FUNC_TC_6: c_uint = 0x6;
pub const OEM_CFG_FUNC_TC_7: c_uint = 0x7;
pub const OEM_CFG_FUNC_HOST_PRI_CTRL_MASK: c_uint = 0x00000030;
pub const OEM_CFG_FUNC_HOST_PRI_CTRL_OFFSET: c_int = 4;
pub const OEM_CFG_FUNC_HOST_PRI_CTRL_VNIC: c_uint = 0x1;
pub const OEM_CFG_FUNC_HOST_PRI_CTRL_OS: c_uint = 0x2;
    pub drv_ver: drv_version_stc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_mac {
    pub mac_upper: u32,
    pub mac_lower: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_file_att {
    pub nvm_start_addr: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bist_nvm_image_att {
    pub return_code: u32,
    pub image_type: u32,
    pub nvm_start_addr: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan_stats_stc {
    pub ucast_rx_pkts: u64,
    pub ucast_tx_pkts: u64,
    pub fcs_err: u32,
    pub rserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_stats_stc {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub fcs_err: u32,
    pub login_failure: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iscsi_stats_stc {
    pub rx_pdus: u64,
    pub tx_pdus: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_stats_stc {
    pub rx_pkts: u64,
    pub tx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocbb_data_stc {
    pub ocbb_host_addr: u32,
    pub ocsd_host_addr: u32,
    pub ocsd_req_update_interval: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_cap_stc {
    pub max_ios: u32,
    pub max_log: u32,
    pub max_exch: u32,
    pub max_npiv: u32,
    pub max_tgt: u32,
    pub max_outstnd: u32,
}

pub const MAX_NUM_OF_SENSORS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct temperature_status_stc {
    pub num_of_sensors: u32,
    pub sensor: [u32; MAX_NUM_OF_SENSORS],
}

// crash dump configuration header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdump_config_stc {
    pub version: u32,
    pub config: u32,
    pub epoc: u32,
    pub num_of_logs: u32,
    pub valid_logs: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum resource_id_enum {
    RESOURCE_NUM_SB_E = 0,
    RESOURCE_NUM_L2_QUEUE_E = 1,
    RESOURCE_NUM_VPORT_E = 2,
    RESOURCE_NUM_VMQ_E = 3,
    RESOURCE_FACTOR_NUM_RSS_PF_E = 4,
    RESOURCE_FACTOR_RSS_PER_VF_E = 5,
    RESOURCE_NUM_RL_E = 6,
    RESOURCE_NUM_PQ_E = 7,
    RESOURCE_NUM_VF_E = 8,
    RESOURCE_VFC_FILTER_E = 9,
    RESOURCE_ILT_E = 10,
    RESOURCE_CQS_E = 11,
    RESOURCE_GFT_PROFILES_E = 12,
    RESOURCE_NUM_TC_E = 13,
    RESOURCE_NUM_RSS_ENGINES_E = 14,
    RESOURCE_LL2_QUEUE_E = 15,
    RESOURCE_RDMA_STATS_QUEUE_E = 16,
    RESOURCE_BDQ_E = 17,
    RESOURCE_QCN_E = 18,
    RESOURCE_LLH_FILTER_E = 19,
    RESOURCE_VF_MAC_ADDR = 20,
    RESOURCE_LL2_CQS_E = 21,
    RESOURCE_VF_CNQS = 22,
    RESOURCE_MAX_NUM,
    RESOURCE_NUM_INVALID = 0xFFFFFFFF
}

// Resource ID is to be filled by the driver in the MB request
// Size, offset & flags to be filled by the MFW in the MB response
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_info {
    pub res_id: resource_id_enum,
    pub /: *mut *mut u32 size; / number of allocated resources,
    pub /: *mut *mut u32 offset; / Offset of the 1st resource,
    pub vf_size: u32,
    pub vf_offset: u32,
    pub flags: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_wwn {
    pub wwn_upper: u32,
    pub wwn_lower: u32,
}

pub const DRV_ROLE_NONE: c_int = 0;
pub const DRV_ROLE_PREBOOT: c_int = 1;
pub const DRV_ROLE_OS: c_int = 2;
pub const DRV_ROLE_KDUMP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct load_req_stc {
    pub drv_ver_0: u32,
    pub drv_ver_1: u32,
    pub fw_ver: u32,
    pub misc0: u32,
pub const LOAD_REQ_ROLE_MASK: c_uint = 0x000000FF;
pub const LOAD_REQ_ROLE_SHIFT: c_int = 0;
pub const LOAD_REQ_LOCK_TO_MASK: c_uint = 0x0000FF00;
pub const LOAD_REQ_LOCK_TO_SHIFT: c_int = 8;
pub const LOAD_REQ_LOCK_TO_DEFAULT: c_int = 0;
pub const LOAD_REQ_LOCK_TO_NONE: c_int = 255;
pub const LOAD_REQ_FORCE_MASK: c_uint = 0x000F0000;
pub const LOAD_REQ_FORCE_SHIFT: c_int = 16;
pub const LOAD_REQ_FORCE_NONE: c_int = 0;
pub const LOAD_REQ_FORCE_PF: c_int = 1;
pub const LOAD_REQ_FORCE_ALL: c_int = 2;
pub const LOAD_REQ_FLAGS0_MASK: c_uint = 0x00F00000;
pub const LOAD_REQ_FLAGS0_SHIFT: c_int = 20;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct load_rsp_stc {
    pub drv_ver_0: u32,
    pub drv_ver_1: u32,
    pub fw_ver: u32,
    pub misc0: u32,
pub const LOAD_RSP_ROLE_MASK: c_uint = 0x000000FF;
pub const LOAD_RSP_ROLE_SHIFT: c_int = 0;
pub const LOAD_RSP_HSI_MASK: c_uint = 0x0000FF00;
pub const LOAD_RSP_HSI_SHIFT: c_int = 8;
pub const LOAD_RSP_FLAGS0_MASK: c_uint = 0x000F0000;
pub const LOAD_RSP_FLAGS0_SHIFT: c_int = 16;

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdump_retain_data_stc {
    pub valid: u32,
    pub epoch: u32,
    pub pf: u32,
    pub status: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute_cmd_write_stc {
    pub val: u32,
    pub mask: u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lldp_stats_stc {
    pub tx_frames_total: u32,
    pub rx_frames_total: u32,
    pub rx_frames_discarded: u32,
    pub rx_age_outs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_att_ctrl_stc {
    pub disabled_attns: u32,
    pub controllable_attns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_filter_stc {
    pub level: u32,
    pub modules: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union drv_union_data {
    pub wol_mac: mcp_mac,
    pub drv_phy_cfg: eth_phy_cfg,
    pub val64: mcp_val64,
    pub raw_data: [u8; MCP_DRV_NVM_BUF_LEN],
    pub file_att: mcp_file_att,
    pub ack_vf_disabled: [u32; EXT_VF_BITMAP_SIZE_IN_DWORDS],
    pub drv_version: drv_version_stc,
    pub lan_stats: lan_stats_stc,
    pub fcoe_stats: fcoe_stats_stc,
    pub iscsi_stats: iscsi_stats_stc,
    pub rdma_stats: rdma_stats_stc,
    pub ocbb_info: ocbb_data_stc,
    pub temp_info: temperature_status_stc,
    pub resource: resource_info,
    pub nvm_image_att: bist_nvm_image_att,
    pub mdump_config: mdump_config_stc,
    pub lldp_mac: mcp_mac,
    pub fcoe_fabric_name: mcp_wwn,
    pub dword: u32,
    pub load_req: load_req_stc,
    pub load_rsp: load_rsp_stc,
    pub mdump_retain: mdump_retain_data_stc,
    pub attribute_cmd_write: attribute_cmd_write_stc,
    pub lldp_stats: lldp_stats_stc,
    pub pcie_stats: pcie_stats_stc,
    pub get_att_ctrl: get_att_ctrl_stc,
    pub fcoe_cap: fcoe_cap_stc,
    pub trace_filter: trace_filter_stc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_drv_mb {
    pub drv_mb_header: u32,
pub const DRV_MSG_SEQ_NUMBER_MASK: c_uint = 0x0000ffff;
pub const DRV_MSG_SEQ_NUMBER_OFFSET: c_int = 0;
pub const DRV_MSG_CODE_MASK: c_uint = 0xffff0000;
pub const DRV_MSG_CODE_OFFSET: c_int = 16;
    pub drv_mb_param: u32,
    pub fw_mb_header: u32,
pub const FW_MSG_SEQ_NUMBER_MASK: c_uint = 0x0000ffff;
pub const FW_MSG_SEQ_NUMBER_OFFSET: c_int = 0;
pub const FW_MSG_CODE_MASK: c_uint = 0xffff0000;
pub const FW_MSG_CODE_OFFSET: c_int = 16;
    pub fw_mb_param: u32,
    pub drv_pulse_mb: u32,
pub const DRV_PULSE_SEQ_MASK: c_uint = 0x00007fff;
pub const DRV_PULSE_SYSTEM_TIME_MASK: c_uint = 0xffff0000;
pub const DRV_PULSE_ALWAYS_ALIVE: c_uint = 0x00008000;
    pub mcp_pulse_mb: u32,
pub const MCP_PULSE_SEQ_MASK: c_uint = 0x00007fff;
pub const MCP_PULSE_ALWAYS_ALIVE: c_uint = 0x00008000;
pub const MCP_EVENT_MASK: c_uint = 0xffff0000;
pub const MCP_EVENT_OTHER_DRIVER_RESET_REQ: c_uint = 0x00010000;
    pub union_data: drv_union_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drv_msg_code_enum {
    DRV_MSG_CODE_NVM_PUT_FILE_BEGIN = DRV_MSG_CODE(0x0001),
    DRV_MSG_CODE_NVM_PUT_FILE_DATA = DRV_MSG_CODE(0x0002),
    DRV_MSG_CODE_NVM_GET_FILE_ATT = DRV_MSG_CODE(0x0003),
    DRV_MSG_CODE_NVM_READ_NVRAM = DRV_MSG_CODE(0x0005),
    DRV_MSG_CODE_NVM_WRITE_NVRAM = DRV_MSG_CODE(0x0006),
    DRV_MSG_CODE_MCP_RESET = DRV_MSG_CODE(0x0009),
    DRV_MSG_CODE_SET_VERSION = DRV_MSG_CODE(0x000f),
    DRV_MSG_CODE_MCP_HALT = DRV_MSG_CODE(0x0010),
    DRV_MSG_CODE_SET_VMAC = DRV_MSG_CODE(0x0011),
    DRV_MSG_CODE_GET_VMAC = DRV_MSG_CODE(0x0012),
    DRV_MSG_CODE_GET_STATS = DRV_MSG_CODE(0x0013),
    DRV_MSG_CODE_TRANSCEIVER_READ = DRV_MSG_CODE(0x0016),
    DRV_MSG_CODE_MASK_PARITIES = DRV_MSG_CODE(0x001a),
    DRV_MSG_CODE_BIST_TEST = DRV_MSG_CODE(0x001e),
    DRV_MSG_CODE_SET_LED_MODE = DRV_MSG_CODE(0x0020),
    DRV_MSG_CODE_RESOURCE_CMD = DRV_MSG_CODE(0x0023),
    DRV_MSG_CODE_MDUMP_CMD = DRV_MSG_CODE(0x0025),
    DRV_MSG_CODE_GET_PF_RDMA_PROTOCOL = DRV_MSG_CODE(0x002b),
    DRV_MSG_CODE_OS_WOL = DRV_MSG_CODE(0x002e),
    DRV_MSG_CODE_GET_TLV_DONE = DRV_MSG_CODE(0x002f),
    DRV_MSG_CODE_FEATURE_SUPPORT = DRV_MSG_CODE(0x0030),
    DRV_MSG_CODE_GET_MFW_FEATURE_SUPPORT = DRV_MSG_CODE(0x0031),
    DRV_MSG_CODE_GET_ENGINE_CONFIG = DRV_MSG_CODE(0x0037),
    DRV_MSG_CODE_GET_NVM_CFG_OPTION = DRV_MSG_CODE(0x003e),
    DRV_MSG_CODE_SET_NVM_CFG_OPTION = DRV_MSG_CODE(0x003f),
    DRV_MSG_CODE_INITIATE_PF_FLR = DRV_MSG_CODE(0x0201),
    DRV_MSG_CODE_LOAD_REQ = DRV_MSG_CODE(0x1000),
    DRV_MSG_CODE_LOAD_DONE = DRV_MSG_CODE(0x1100),
    DRV_MSG_CODE_INIT_HW = DRV_MSG_CODE(0x1200),
    DRV_MSG_CODE_CANCEL_LOAD_REQ = DRV_MSG_CODE(0x1300),
    DRV_MSG_CODE_UNLOAD_REQ = DRV_MSG_CODE(0x2000),
    DRV_MSG_CODE_UNLOAD_DONE = DRV_MSG_CODE(0x2100),
    DRV_MSG_CODE_INIT_PHY = DRV_MSG_CODE(0x2200),
    DRV_MSG_CODE_LINK_RESET = DRV_MSG_CODE(0x2300),
    DRV_MSG_CODE_SET_DCBX = DRV_MSG_CODE(0x2500),
    DRV_MSG_CODE_OV_UPDATE_CURR_CFG = DRV_MSG_CODE(0x2600),
    DRV_MSG_CODE_OV_UPDATE_BUS_NUM = DRV_MSG_CODE(0x2700),
    DRV_MSG_CODE_OV_UPDATE_BOOT_PROGRESS = DRV_MSG_CODE(0x2800),
    DRV_MSG_CODE_OV_UPDATE_STORM_FW_VER = DRV_MSG_CODE(0x2900),
    DRV_MSG_CODE_NIG_DRAIN = DRV_MSG_CODE(0x3000),
    DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE = DRV_MSG_CODE(0x3100),
    DRV_MSG_CODE_BW_UPDATE_ACK = DRV_MSG_CODE(0x3200),
    DRV_MSG_CODE_OV_UPDATE_MTU = DRV_MSG_CODE(0x3300),
    DRV_MSG_GET_RESOURCE_ALLOC_MSG = DRV_MSG_CODE(0x3400),
    DRV_MSG_SET_RESOURCE_VALUE_MSG = DRV_MSG_CODE(0x3500),
    DRV_MSG_CODE_OV_UPDATE_WOL = DRV_MSG_CODE(0x3800),
    DRV_MSG_CODE_OV_UPDATE_ESWITCH_MODE = DRV_MSG_CODE(0x3900),
    DRV_MSG_CODE_S_TAG_UPDATE_ACK = DRV_MSG_CODE(0x3b00),
    DRV_MSG_CODE_GET_OEM_UPDATES = DRV_MSG_CODE(0x4100),
    DRV_MSG_CODE_GET_PPFID_BITMAP = DRV_MSG_CODE(0x4300),
    DRV_MSG_CODE_VF_DISABLED_DONE = DRV_MSG_CODE(0xc000),
    DRV_MSG_CODE_CFG_VF_MSIX = DRV_MSG_CODE(0xc001),
    DRV_MSG_CODE_CFG_PF_VFS_MSIX = DRV_MSG_CODE(0xc002),
    DRV_MSG_CODE_DEBUG_DATA_SEND = DRV_MSG_CODE(0xc004),
    DRV_MSG_CODE_GET_MANAGEMENT_STATUS = DRV_MSG_CODE(0xc007),
}

pub const DRV_MSG_CODE_VMAC_TYPE_SHIFT: c_int = 4;
pub const DRV_MSG_CODE_VMAC_TYPE_MASK: c_uint = 0x30;
pub const DRV_MSG_CODE_VMAC_TYPE_MAC: c_int = 1;
pub const DRV_MSG_CODE_VMAC_TYPE_WWNN: c_int = 2;
pub const DRV_MSG_CODE_VMAC_TYPE_WWPN: c_int = 3;
// DRV_MSG_CODE_RETAIN_VMAC parameters
pub const DRV_MSG_CODE_RETAIN_VMAC_FUNC_SHIFT: c_int = 0;
pub const DRV_MSG_CODE_RETAIN_VMAC_FUNC_MASK: c_uint = 0xf;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_SHIFT: c_int = 4;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_MASK: c_uint = 0x70;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_L2: c_int = 0;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_ISCSI: c_int = 1;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_FCOE: c_int = 2;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_WWNN: c_int = 3;
pub const DRV_MSG_CODE_RETAIN_VMAC_TYPE_WWPN: c_int = 4;
pub const DRV_MSG_CODE_MCP_RESET_FORCE: c_uint = 0xf04ce;
pub const DRV_MSG_CODE_STATS_TYPE_LAN: c_int = 1;
pub const DRV_MSG_CODE_STATS_TYPE_FCOE: c_int = 2;
pub const DRV_MSG_CODE_STATS_TYPE_ISCSI: c_int = 3;
pub const DRV_MSG_CODE_STATS_TYPE_RDMA: c_int = 4;
pub const BW_MAX_MASK: c_uint = 0x000000ff;
pub const BW_MAX_OFFSET: c_int = 0;
pub const BW_MIN_MASK: c_uint = 0x0000ff00;
pub const BW_MIN_OFFSET: c_int = 8;

pub const RESOURCE_CMD_REQ_RESC_MASK: c_uint = 0x0000001F;
pub const RESOURCE_CMD_REQ_RESC_SHIFT: c_int = 0;
pub const RESOURCE_CMD_REQ_OPCODE_MASK: c_uint = 0x000000E0;
pub const RESOURCE_CMD_REQ_OPCODE_SHIFT: c_int = 5;
pub const RESOURCE_OPCODE_REQ: c_int = 1;
pub const RESOURCE_OPCODE_REQ_WO_AGING: c_int = 2;
pub const RESOURCE_OPCODE_REQ_W_AGING: c_int = 3;
pub const RESOURCE_OPCODE_RELEASE: c_int = 4;
pub const RESOURCE_OPCODE_FORCE_RELEASE: c_int = 5;
pub const RESOURCE_CMD_REQ_AGE_MASK: c_uint = 0x0000FF00;
pub const RESOURCE_CMD_REQ_AGE_SHIFT: c_int = 8;
pub const RESOURCE_CMD_RSP_OWNER_MASK: c_uint = 0x000000FF;
pub const RESOURCE_CMD_RSP_OWNER_SHIFT: c_int = 0;
pub const RESOURCE_CMD_RSP_OPCODE_MASK: c_uint = 0x00000700;
pub const RESOURCE_CMD_RSP_OPCODE_SHIFT: c_int = 8;
pub const RESOURCE_OPCODE_GNT: c_int = 1;
pub const RESOURCE_OPCODE_BUSY: c_int = 2;
pub const RESOURCE_OPCODE_RELEASED: c_int = 3;
pub const RESOURCE_OPCODE_RELEASED_PREVIOUS: c_int = 4;
pub const RESOURCE_OPCODE_WRONG_OWNER: c_int = 5;
pub const RESOURCE_OPCODE_UNKNOWN_CMD: c_int = 255;
pub const RESOURCE_DUMP: c_int = 0;
// DRV_MSG_CODE_MDUMP_CMD parameters
pub const MDUMP_DRV_PARAM_OPCODE_MASK: c_uint = 0x000000ff;
pub const DRV_MSG_CODE_MDUMP_ACK: c_uint = 0x01;
pub const DRV_MSG_CODE_MDUMP_SET_VALUES: c_uint = 0x02;
pub const DRV_MSG_CODE_MDUMP_TRIGGER: c_uint = 0x03;
pub const DRV_MSG_CODE_MDUMP_GET_CONFIG: c_uint = 0x04;
pub const DRV_MSG_CODE_MDUMP_SET_ENABLE: c_uint = 0x05;
pub const DRV_MSG_CODE_MDUMP_CLEAR_LOGS: c_uint = 0x06;
pub const DRV_MSG_CODE_MDUMP_GET_RETAIN: c_uint = 0x07;
pub const DRV_MSG_CODE_MDUMP_CLR_RETAIN: c_uint = 0x08;
pub const DRV_MSG_CODE_HW_DUMP_TRIGGER: c_uint = 0x0a;
pub const DRV_MSG_CODE_MDUMP_FREE_DRIVER_BUF: c_uint = 0x0b;
pub const DRV_MSG_CODE_MDUMP_GEN_LINK_DUMP: c_uint = 0x0c;
pub const DRV_MSG_CODE_MDUMP_GEN_IDLE_CHK: c_uint = 0x0d;
// DRV_MSG_CODE_MDUMP_CMD options
pub const MDUMP_DRV_PARAM_OPTION_MASK: c_uint = 0x00000f00;
pub const DRV_MSG_CODE_MDUMP_USE_DRIVER_BUF_OFFSET: c_int = 8;
pub const DRV_MSG_CODE_MDUMP_USE_DRIVER_BUF_MASK: c_uint = 0x100;
// DRV_MSG_CODE_EXT_PHY_READ/DRV_MSG_CODE_EXT_PHY_WRITE parameters
pub const DRV_MB_PARAM_ADDR_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_ADDR_MASK: c_uint = 0x0000FFFF;
pub const DRV_MB_PARAM_DEVAD_SHIFT: c_int = 16;
pub const DRV_MB_PARAM_DEVAD_MASK: c_uint = 0x001F0000;
pub const DRV_MB_PARAM_PORT_SHIFT: c_int = 21;
pub const DRV_MB_PARAM_PORT_MASK: c_uint = 0x00600000;
// DRV_MSG_CODE_PMBUS_READ/DRV_MSG_CODE_PMBUS_WRITE parameters
pub const DRV_MB_PARAM_PMBUS_CMD_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_PMBUS_CMD_MASK: c_uint = 0xFF;
pub const DRV_MB_PARAM_PMBUS_LEN_SHIFT: c_int = 8;
pub const DRV_MB_PARAM_PMBUS_LEN_MASK: c_uint = 0x300;
pub const DRV_MB_PARAM_PMBUS_DATA_SHIFT: c_int = 16;
pub const DRV_MB_PARAM_PMBUS_DATA_MASK: c_uint = 0xFFFF0000;
// UNLOAD_REQ params
pub const DRV_MB_PARAM_UNLOAD_WOL_UNKNOWN: c_uint = 0x00000000;
pub const DRV_MB_PARAM_UNLOAD_WOL_MCP: c_uint = 0x00000001;
pub const DRV_MB_PARAM_UNLOAD_WOL_DISABLED: c_uint = 0x00000002;
pub const DRV_MB_PARAM_UNLOAD_WOL_ENABLED: c_uint = 0x00000003;
// UNLOAD_DONE_params
pub const DRV_MB_PARAM_UNLOAD_NON_D3_POWER: c_uint = 0x00000001;
// INIT_PHY params
pub const DRV_MB_PARAM_INIT_PHY_FORCE: c_uint = 0x00000001;
pub const DRV_MB_PARAM_INIT_PHY_DONT_CARE: c_uint = 0x00000002;
// LLDP / DCBX params
pub const DRV_MB_PARAM_LLDP_SEND_MASK: c_uint = 0x00000001;
pub const DRV_MB_PARAM_LLDP_SEND_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_LLDP_AGENT_MASK: c_uint = 0x00000006;
pub const DRV_MB_PARAM_LLDP_AGENT_SHIFT: c_int = 1;
pub const DRV_MB_PARAM_LLDP_TLV_RX_VALID_MASK: c_uint = 0x00000001;
pub const DRV_MB_PARAM_LLDP_TLV_RX_VALID_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_LLDP_TLV_RX_TYPE_MASK: c_uint = 0x000007f0;
pub const DRV_MB_PARAM_LLDP_TLV_RX_TYPE_SHIFT: c_int = 4;
pub const DRV_MB_PARAM_DCBX_NOTIFY_MASK: c_uint = 0x00000008;
pub const DRV_MB_PARAM_DCBX_NOTIFY_SHIFT: c_int = 3;
pub const DRV_MB_PARAM_DCBX_ADMIN_CFG_NOTIFY_MASK: c_uint = 0x00000010;
pub const DRV_MB_PARAM_DCBX_ADMIN_CFG_NOTIFY_SHIFT: c_int = 4;
pub const DRV_MB_PARAM_NIG_DRAIN_PERIOD_MS_MASK: c_uint = 0x000000FF;
pub const DRV_MB_PARAM_NIG_DRAIN_PERIOD_MS_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_NVM_PUT_FILE_TYPE_MASK: c_uint = 0x000000ff;
pub const DRV_MB_PARAM_NVM_PUT_FILE_TYPE_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_NVM_PUT_FILE_BEGIN_MFW: c_uint = 0x1;
pub const DRV_MB_PARAM_NVM_PUT_FILE_BEGIN_IMAGE: c_uint = 0x2;
pub const DRV_MB_PARAM_NVM_PUT_FILE_BEGIN_MBI: c_uint = 0x3;
pub const DRV_MB_PARAM_NVM_OFFSET_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_NVM_OFFSET_MASK: c_uint = 0x00FFFFFF;
pub const DRV_MB_PARAM_NVM_LEN_OFFSET: c_int = 24;
pub const DRV_MB_PARAM_NVM_LEN_MASK: c_uint = 0xFF000000;
pub const DRV_MB_PARAM_CFG_VF_MSIX_VF_ID_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_CFG_VF_MSIX_VF_ID_MASK: c_uint = 0x000000FF;
pub const DRV_MB_PARAM_CFG_VF_MSIX_SB_NUM_SHIFT: c_int = 8;
pub const DRV_MB_PARAM_CFG_VF_MSIX_SB_NUM_MASK: c_uint = 0x0000FF00;
pub const DRV_MB_PARAM_OV_CURR_CFG_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_OV_CURR_CFG_MASK: c_uint = 0x0000000F;
pub const DRV_MB_PARAM_OV_CURR_CFG_NONE: c_int = 0;
pub const DRV_MB_PARAM_OV_CURR_CFG_OS: c_int = 1;
pub const DRV_MB_PARAM_OV_CURR_CFG_VENDOR_SPEC: c_int = 2;
pub const DRV_MB_PARAM_OV_CURR_CFG_OTHER: c_int = 3;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_MASK: c_uint = 0xFFFFFFFF;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_MAJOR_MASK: c_uint = 0xFF000000;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_MINOR_MASK: c_uint = 0x00FF0000;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_BUILD_MASK: c_uint = 0x0000FF00;
pub const DRV_MB_PARAM_OV_STORM_FW_VER_DROP_MASK: c_uint = 0x000000FF;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_SHIFT: c_int = 0;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_MASK: c_uint = 0xF;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_UNKNOWN: c_uint = 0x1;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_NOT_LOADED: c_uint = 0x2;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_LOADING: c_uint = 0x3;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_DISABLED: c_uint = 0x4;
pub const DRV_MSG_CODE_OV_UPDATE_DRIVER_STATE_ACTIVE: c_uint = 0x5;
pub const DRV_MB_PARAM_OV_MTU_SIZE_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_OV_MTU_SIZE_MASK: c_uint = 0xFFFFFFFF;

pub const DRV_MB_PARAM_ESWITCH_MODE_NONE: c_uint = 0x0;
pub const DRV_MB_PARAM_ESWITCH_MODE_VEB: c_uint = 0x1;
pub const DRV_MB_PARAM_ESWITCH_MODE_VEPA: c_uint = 0x2;
pub const DRV_MB_PARAM_DUMMY_OEM_UPDATES_MASK: c_uint = 0x1;
pub const DRV_MB_PARAM_DUMMY_OEM_UPDATES_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_SET_LED_MODE_OPER: c_uint = 0x0;
pub const DRV_MB_PARAM_SET_LED_MODE_ON: c_uint = 0x1;
pub const DRV_MB_PARAM_SET_LED_MODE_OFF: c_uint = 0x2;
pub const DRV_MB_PARAM_TRANSCEIVER_PORT_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_TRANSCEIVER_PORT_MASK: c_uint = 0x00000003;
pub const DRV_MB_PARAM_TRANSCEIVER_SIZE_OFFSET: c_int = 2;
pub const DRV_MB_PARAM_TRANSCEIVER_SIZE_MASK: c_uint = 0x000000fc;
pub const DRV_MB_PARAM_TRANSCEIVER_I2C_ADDRESS_OFFSET: c_int = 8;
pub const DRV_MB_PARAM_TRANSCEIVER_I2C_ADDRESS_MASK: c_uint = 0x0000ff00;
pub const DRV_MB_PARAM_TRANSCEIVER_OFFSET_OFFSET: c_int = 16;
pub const DRV_MB_PARAM_TRANSCEIVER_OFFSET_MASK: c_uint = 0xffff0000;
// Resource Allocation params - Driver version support
pub const DRV_MB_PARAM_RESOURCE_ALLOC_VERSION_MAJOR_MASK: c_uint = 0xffff0000;
pub const DRV_MB_PARAM_RESOURCE_ALLOC_VERSION_MAJOR_SHIFT: c_int = 16;
pub const DRV_MB_PARAM_RESOURCE_ALLOC_VERSION_MINOR_MASK: c_uint = 0x0000ffff;
pub const DRV_MB_PARAM_RESOURCE_ALLOC_VERSION_MINOR_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_BIST_UNKNOWN_TEST: c_int = 0;
pub const DRV_MB_PARAM_BIST_REGISTER_TEST: c_int = 1;
pub const DRV_MB_PARAM_BIST_CLOCK_TEST: c_int = 2;
pub const DRV_MB_PARAM_BIST_NVM_TEST_NUM_IMAGES: c_int = 3;
pub const DRV_MB_PARAM_BIST_NVM_TEST_IMAGE_BY_INDEX: c_int = 4;
pub const DRV_MB_PARAM_BIST_RC_UNKNOWN: c_int = 0;
pub const DRV_MB_PARAM_BIST_RC_PASSED: c_int = 1;
pub const DRV_MB_PARAM_BIST_RC_FAILED: c_int = 2;
pub const DRV_MB_PARAM_BIST_RC_INVALID_PARAMETER: c_int = 3;
pub const DRV_MB_PARAM_BIST_TEST_INDEX_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_BIST_TEST_INDEX_MASK: c_uint = 0x000000ff;
pub const DRV_MB_PARAM_BIST_TEST_IMAGE_INDEX_SHIFT: c_int = 8;
pub const DRV_MB_PARAM_BIST_TEST_IMAGE_INDEX_MASK: c_uint = 0x0000ff00;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_MASK: c_uint = 0x0000ffff;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_SMARTLINQ: c_uint = 0x00000001;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_EEE: c_uint = 0x00000002;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_FEC_CONTROL: c_uint = 0x00000004;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_PORT_EXT_SPEED_FEC_CONTROL: c_uint = 0x00000008;
pub const DRV_MB_PARAM_FEATURE_SUPPORT_FUNC_VLINK: c_uint = 0x00010000;
// DRV_MSG_CODE_DEBUG_DATA_SEND parameters
pub const DRV_MSG_CODE_DEBUG_DATA_SEND_SIZE_OFFSET: c_int = 0;
pub const DRV_MSG_CODE_DEBUG_DATA_SEND_SIZE_MASK: c_uint = 0xff;
// Driver attributes params
pub const DRV_MB_PARAM_ATTRIBUTE_KEY_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_ATTRIBUTE_KEY_MASK: c_uint = 0x00ffffff;
pub const DRV_MB_PARAM_ATTRIBUTE_CMD_OFFSET: c_int = 24;
pub const DRV_MB_PARAM_ATTRIBUTE_CMD_MASK: c_uint = 0xff000000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ID_OFFSET: c_int = 0;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ID_MASK: c_uint = 0x0000ffff;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ID_IGNORE: c_uint = 0x0000ffff;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ID_SHIFT: c_int = 0;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ALL_SHIFT: c_int = 16;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ALL_MASK: c_uint = 0x00010000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_INIT_SHIFT: c_int = 17;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_INIT_MASK: c_uint = 0x00020000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_COMMIT_SHIFT: c_int = 18;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_COMMIT_MASK: c_uint = 0x00040000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_FREE_SHIFT: c_int = 19;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_FREE_MASK: c_uint = 0x00080000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ENTITY_SEL_SHIFT: c_int = 20;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ENTITY_SEL_MASK: c_uint = 0x00100000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_DEFAULT_RESTORE_ALL_SHIFT: c_int = 21;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_DEFAULT_RESTORE_ALL_MASK: c_uint = 0x00200000;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ENTITY_ID_SHIFT: c_int = 24;
pub const DRV_MB_PARAM_NVM_CFG_OPTION_ENTITY_ID_MASK: c_uint = 0x0f000000;
// DRV_MSG_CODE_GET_PERM_MAC parametres
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_SHIFT: c_int = 0;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_MASK: c_uint = 0xF;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_PF: c_int = 0;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_BMC: c_int = 1;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_VF: c_int = 2;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_LLDP: c_int = 3;
pub const DRV_MSG_CODE_GET_PERM_MAC_TYPE_MAX: c_int = 4;
pub const DRV_MSG_CODE_GET_PERM_MAC_INDEX_SHIFT: c_int = 8;
pub const DRV_MSG_CODE_GET_PERM_MAC_INDEX_MASK: c_uint = 0xFFFF00;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_msg_code_enum {
    FW_MSG_CODE_UNSUPPORTED = FW_MSG_CODE(0x0000),
    FW_MSG_CODE_NVM_OK = FW_MSG_CODE(0x0001),
    FW_MSG_CODE_NVM_PUT_FILE_FINISH_OK = FW_MSG_CODE(0x0040),
    FW_MSG_CODE_PHY_OK = FW_MSG_CODE(0x0011),
    FW_MSG_CODE_OK = FW_MSG_CODE(0x0016),
    FW_MSG_CODE_ERROR = FW_MSG_CODE(0x0017),
    FW_MSG_CODE_TRANSCEIVER_DIAG_OK = FW_MSG_CODE(0x0016),
    FW_MSG_CODE_TRANSCEIVER_NOT_PRESENT = FW_MSG_CODE(0x0002),
    FW_MSG_CODE_MDUMP_INVALID_CMD = FW_MSG_CODE(0x0003),
    FW_MSG_CODE_OS_WOL_SUPPORTED = FW_MSG_CODE(0x0080),
    FW_MSG_CODE_DRV_CFG_PF_VFS_MSIX_DONE = FW_MSG_CODE(0x0087),
    FW_MSG_CODE_DRV_LOAD_ENGINE = FW_MSG_CODE(0x1010),
    FW_MSG_CODE_DRV_LOAD_PORT = FW_MSG_CODE(0x1011),
    FW_MSG_CODE_DRV_LOAD_FUNCTION = FW_MSG_CODE(0x1012),
    FW_MSG_CODE_DRV_LOAD_REFUSED_PDA = FW_MSG_CODE(0x1020),
    FW_MSG_CODE_DRV_LOAD_REFUSED_HSI_1 = FW_MSG_CODE(0x1021),
    FW_MSG_CODE_DRV_LOAD_REFUSED_DIAG = FW_MSG_CODE(0x1022),
    FW_MSG_CODE_DRV_LOAD_REFUSED_HSI = FW_MSG_CODE(0x1023),
    FW_MSG_CODE_DRV_LOAD_REFUSED_REQUIRES_FORCE = FW_MSG_CODE(0x1030),
    FW_MSG_CODE_DRV_LOAD_REFUSED_REJECT = FW_MSG_CODE(0x1031),
    FW_MSG_CODE_DRV_LOAD_DONE = FW_MSG_CODE(0x1110),
    FW_MSG_CODE_DRV_UNLOAD_ENGINE = FW_MSG_CODE(0x2011),
    FW_MSG_CODE_DRV_UNLOAD_PORT = FW_MSG_CODE(0x2012),
    FW_MSG_CODE_DRV_UNLOAD_FUNCTION = FW_MSG_CODE(0x2013),
    FW_MSG_CODE_DRV_UNLOAD_DONE = FW_MSG_CODE(0x2110),
    FW_MSG_CODE_RESOURCE_ALLOC_OK = FW_MSG_CODE(0x3400),
    FW_MSG_CODE_RESOURCE_ALLOC_UNKNOWN = FW_MSG_CODE(0x3500),
    FW_MSG_CODE_S_TAG_UPDATE_ACK_DONE = FW_MSG_CODE(0x3b00),
    FW_MSG_CODE_DRV_CFG_VF_MSIX_DONE = FW_MSG_CODE(0xb001),
    FW_MSG_CODE_DEBUG_NOT_ENABLED = FW_MSG_CODE(0xb00a),
    FW_MSG_CODE_DEBUG_DATA_SEND_OK = FW_MSG_CODE(0xb00b),
}

pub const FW_MB_PARAM_RESOURCE_ALLOC_VERSION_MAJOR_MASK: c_uint = 0xffff0000;
pub const FW_MB_PARAM_RESOURCE_ALLOC_VERSION_MAJOR_SHIFT: c_int = 16;
pub const FW_MB_PARAM_RESOURCE_ALLOC_VERSION_MINOR_MASK: c_uint = 0x0000ffff;
pub const FW_MB_PARAM_RESOURCE_ALLOC_VERSION_MINOR_SHIFT: c_int = 0;
// Get PF RDMA protocol command response
pub const FW_MB_PARAM_GET_PF_RDMA_NONE: c_uint = 0x0;
pub const FW_MB_PARAM_GET_PF_RDMA_ROCE: c_uint = 0x1;
pub const FW_MB_PARAM_GET_PF_RDMA_IWARP: c_uint = 0x2;
pub const FW_MB_PARAM_GET_PF_RDMA_BOTH: c_uint = 0x3;
// Get MFW feature support response

pub const FW_MB_PARAM_MANAGEMENT_STATUS_LOCKDOWN_ENABLED: c_uint = 0x00000001;

pub const FW_MB_PARAM_ENG_CFG_FIR_AFFIN_VALID_MASK: c_uint = 0x00000001;
pub const FW_MB_PARAM_ENG_CFG_FIR_AFFIN_VALID_SHIFT: c_int = 0;
pub const FW_MB_PARAM_ENG_CFG_FIR_AFFIN_VALUE_MASK: c_uint = 0x00000002;
pub const FW_MB_PARAM_ENG_CFG_FIR_AFFIN_VALUE_SHIFT: c_int = 1;
pub const FW_MB_PARAM_ENG_CFG_L2_AFFIN_VALID_MASK: c_uint = 0x00000004;
pub const FW_MB_PARAM_ENG_CFG_L2_AFFIN_VALID_SHIFT: c_int = 2;
pub const FW_MB_PARAM_ENG_CFG_L2_AFFIN_VALUE_MASK: c_uint = 0x00000008;
pub const FW_MB_PARAM_ENG_CFG_L2_AFFIN_VALUE_SHIFT: c_int = 3;
pub const FW_MB_PARAM_PPFID_BITMAP_MASK: c_uint = 0xff;
pub const FW_MB_PARAM_PPFID_BITMAP_SHIFT: c_int = 0;
pub const FW_MB_PARAM_NVM_PUT_FILE_REQ_OFFSET_MASK: c_uint = 0x00ffffff;
pub const FW_MB_PARAM_NVM_PUT_FILE_REQ_OFFSET_SHIFT: c_int = 0;
pub const FW_MB_PARAM_NVM_PUT_FILE_REQ_SIZE_MASK: c_uint = 0xff000000;
pub const FW_MB_PARAM_NVM_PUT_FILE_REQ_SIZE_SHIFT: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MFW_DRV_MSG_TYPE {
    MFW_DRV_MSG_LINK_CHANGE,
    MFW_DRV_MSG_FLR_FW_ACK_FAILED,
    MFW_DRV_MSG_VF_DISABLED,
    MFW_DRV_MSG_LLDP_DATA_UPDATED,
    MFW_DRV_MSG_DCBX_REMOTE_MIB_UPDATED,
    MFW_DRV_MSG_DCBX_OPERATIONAL_MIB_UPDATED,
    MFW_DRV_MSG_ERROR_RECOVERY,
    MFW_DRV_MSG_BW_UPDATE,
    MFW_DRV_MSG_S_TAG_UPDATE,
    MFW_DRV_MSG_GET_LAN_STATS,
    MFW_DRV_MSG_GET_FCOE_STATS,
    MFW_DRV_MSG_GET_ISCSI_STATS,
    MFW_DRV_MSG_GET_RDMA_STATS,
    MFW_DRV_MSG_FAILURE_DETECTED,
    MFW_DRV_MSG_TRANSCEIVER_STATE_CHANGE,
    MFW_DRV_MSG_CRITICAL_ERROR_OCCURRED,
    MFW_DRV_MSG_EEE_NEGOTIATION_COMPLETE,
    MFW_DRV_MSG_GET_TLV_REQ,
    MFW_DRV_MSG_OEM_CFG_UPDATE,
    MFW_DRV_MSG_LLDP_RECEIVED_TLVS_UPDATED,
    MFW_DRV_MSG_GENERIC_IDC,
    MFW_DRV_MSG_XCVR_TX_FAULT,
    MFW_DRV_MSG_XCVR_RX_LOS,
    MFW_DRV_MSG_GET_FCOE_CAP,
    MFW_DRV_MSG_GEN_LINK_DUMP,
    MFW_DRV_MSG_GEN_IDLE_CHK,
    MFW_DRV_MSG_DCBX_ADMIN_CFG_APPLIED,
    MFW_DRV_MSG_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct public_mfw_mb {
    pub sup_msgs: u32,
    pub msg: [u32; MFW_DRV_MSG_MAX_DWORDS(MFW_DRV_MSG_MAX)],
    pub ack: [u32; MFW_DRV_MSG_MAX_DWORDS(MFW_DRV_MSG_MAX)],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum public_sections {
    PUBLIC_DRV_MB,
    PUBLIC_MFW_MB,
    PUBLIC_GLOBAL,
    PUBLIC_PATH,
    PUBLIC_PORT,
    PUBLIC_FUNC,
    PUBLIC_MAX_SECTIONS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_ver_info_stc {
    pub ver: u32,
    pub name: [u8; 32],
}

// Runtime data needs about 1/2K. We use 2K to be on the safe side.
// Please make sure data does not exceed this size.
//
pub const NUM_RUNTIME_DWORDS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv_init_hw_stc {
    pub init_hw_bitmask: [u32; NUM_RUNTIME_DWORDS],
    pub 32]: *mut *mut u32 init_hw_data[NUM_RUNTIME_DWORDS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp_public_data {
    pub num_sections: u32,
    pub sections: [u32; PUBLIC_MAX_SECTIONS],
    pub drv_mb: [public_drv_mb; MCP_GLOB_FUNC_MAX],
    pub mfw_mb: [public_mfw_mb; MCP_GLOB_FUNC_MAX],
    pub global: public_global,
    pub path: [public_path; MCP_GLOB_PATH_MAX],
    pub port: [public_port; MCP_GLOB_PORT_MAX],
    pub func: [public_func; MCP_GLOB_FUNC_MAX],
}

pub const I2C_TRANSCEIVER_ADDR: c_uint = 0xa0;
pub const MAX_I2C_TRANSACTION_SIZE: c_int = 16;
pub const MAX_I2C_TRANSCEIVER_PAGE_SIZE: c_int = 256;
// OCBB definitions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tlvs {
// Category 1: Device Properties
    DRV_TLV_CLP_STR,
    DRV_TLV_CLP_STR_CTD,
// Category 6: Device Configuration
    DRV_TLV_SCSI_TO,
    DRV_TLV_R_T_TOV,
    DRV_TLV_R_A_TOV,
    DRV_TLV_E_D_TOV,
    DRV_TLV_CR_TOV,
    DRV_TLV_BOOT_TYPE,
// Category 8: Port Configuration
    DRV_TLV_NPIV_ENABLED,
// Category 10: Function Configuration
    DRV_TLV_FEATURE_FLAGS,
    DRV_TLV_LOCAL_ADMIN_ADDR,
    DRV_TLV_ADDITIONAL_MAC_ADDR_1,
    DRV_TLV_ADDITIONAL_MAC_ADDR_2,
    DRV_TLV_LSO_MAX_OFFLOAD_SIZE,
    DRV_TLV_LSO_MIN_SEGMENT_COUNT,
    DRV_TLV_PROMISCUOUS_MODE,
    DRV_TLV_TX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_RX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_NUM_OF_NET_QUEUE_VMQ_CFG,
    DRV_TLV_FLEX_NIC_OUTER_VLAN_ID,
    DRV_TLV_OS_DRIVER_STATES,
    DRV_TLV_PXE_BOOT_PROGRESS,
// Category 12: FC/FCoE Configuration
    DRV_TLV_NPIV_STATE,
    DRV_TLV_NUM_OF_NPIV_IDS,
    DRV_TLV_SWITCH_NAME,
    DRV_TLV_SWITCH_PORT_NUM,
    DRV_TLV_SWITCH_PORT_ID,
    DRV_TLV_VENDOR_NAME,
    DRV_TLV_SWITCH_MODEL,
    DRV_TLV_SWITCH_FW_VER,
    DRV_TLV_QOS_PRIORITY_PER_802_1P,
    DRV_TLV_PORT_ALIAS,
    DRV_TLV_PORT_STATE,
    DRV_TLV_FIP_TX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_FCOE_RX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_LINK_FAILURE_COUNT,
    DRV_TLV_FCOE_BOOT_PROGRESS,
// Category 13: iSCSI Configuration
    DRV_TLV_TARGET_LLMNR_ENABLED,
    DRV_TLV_HEADER_DIGEST_FLAG_ENABLED,
    DRV_TLV_DATA_DIGEST_FLAG_ENABLED,
    DRV_TLV_AUTHENTICATION_METHOD,
    DRV_TLV_ISCSI_BOOT_TARGET_PORTAL,
    DRV_TLV_MAX_FRAME_SIZE,
    DRV_TLV_PDU_TX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_PDU_RX_DESCRIPTORS_QUEUE_SIZE,
    DRV_TLV_ISCSI_BOOT_PROGRESS,
// Category 20: Device Data
    DRV_TLV_PCIE_BUS_RX_UTILIZATION,
    DRV_TLV_PCIE_BUS_TX_UTILIZATION,
    DRV_TLV_DEVICE_CPU_CORES_UTILIZATION,
    DRV_TLV_LAST_VALID_DCC_TLV_RECEIVED,
    DRV_TLV_NCSI_RX_BYTES_RECEIVED,
    DRV_TLV_NCSI_TX_BYTES_SENT,
// Category 22: Base Port Data
    DRV_TLV_RX_DISCARDS,
    DRV_TLV_RX_ERRORS,
    DRV_TLV_TX_ERRORS,
    DRV_TLV_TX_DISCARDS,
    DRV_TLV_RX_FRAMES_RECEIVED,
    DRV_TLV_TX_FRAMES_SENT,
// Category 23: FC/FCoE Port Data
    DRV_TLV_RX_BROADCAST_PACKETS,
    DRV_TLV_TX_BROADCAST_PACKETS,
// Category 28: Base Function Data
    DRV_TLV_NUM_OFFLOADED_CONNECTIONS_TCP_IPV4,
    DRV_TLV_NUM_OFFLOADED_CONNECTIONS_TCP_IPV6,
    DRV_TLV_TX_DESCRIPTOR_QUEUE_AVG_DEPTH,
    DRV_TLV_RX_DESCRIPTORS_QUEUE_AVG_DEPTH,
    DRV_TLV_PF_RX_FRAMES_RECEIVED,
    DRV_TLV_RX_BYTES_RECEIVED,
    DRV_TLV_PF_TX_FRAMES_SENT,
    DRV_TLV_TX_BYTES_SENT,
    DRV_TLV_IOV_OFFLOAD,
    DRV_TLV_PCI_ERRORS_CAP_ID,
    DRV_TLV_UNCORRECTABLE_ERROR_STATUS,
    DRV_TLV_UNCORRECTABLE_ERROR_MASK,
    DRV_TLV_CORRECTABLE_ERROR_STATUS,
    DRV_TLV_CORRECTABLE_ERROR_MASK,
    DRV_TLV_PCI_ERRORS_AECC_REGISTER,
    DRV_TLV_TX_QUEUES_EMPTY,
    DRV_TLV_RX_QUEUES_EMPTY,
    DRV_TLV_TX_QUEUES_FULL,
    DRV_TLV_RX_QUEUES_FULL,
// Category 29: FC/FCoE Function Data
    DRV_TLV_FCOE_TX_DESCRIPTOR_QUEUE_AVG_DEPTH,
    DRV_TLV_FCOE_RX_DESCRIPTORS_QUEUE_AVG_DEPTH,
    DRV_TLV_FCOE_RX_FRAMES_RECEIVED,
    DRV_TLV_FCOE_RX_BYTES_RECEIVED,
    DRV_TLV_FCOE_TX_FRAMES_SENT,
    DRV_TLV_FCOE_TX_BYTES_SENT,
    DRV_TLV_CRC_ERROR_COUNT,
    DRV_TLV_CRC_ERROR_1_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_CRC_ERROR_1_TIMESTAMP,
    DRV_TLV_CRC_ERROR_2_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_CRC_ERROR_2_TIMESTAMP,
    DRV_TLV_CRC_ERROR_3_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_CRC_ERROR_3_TIMESTAMP,
    DRV_TLV_CRC_ERROR_4_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_CRC_ERROR_4_TIMESTAMP,
    DRV_TLV_CRC_ERROR_5_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_CRC_ERROR_5_TIMESTAMP,
    DRV_TLV_LOSS_OF_SYNC_ERROR_COUNT,
    DRV_TLV_LOSS_OF_SIGNAL_ERRORS,
    DRV_TLV_PRIMITIVE_SEQUENCE_PROTOCOL_ERROR_COUNT,
    DRV_TLV_DISPARITY_ERROR_COUNT,
    DRV_TLV_CODE_VIOLATION_ERROR_COUNT,
    DRV_TLV_LAST_FLOGI_ISSUED_COMMON_PARAMETERS_WORD_1,
    DRV_TLV_LAST_FLOGI_ISSUED_COMMON_PARAMETERS_WORD_2,
    DRV_TLV_LAST_FLOGI_ISSUED_COMMON_PARAMETERS_WORD_3,
    DRV_TLV_LAST_FLOGI_ISSUED_COMMON_PARAMETERS_WORD_4,
    DRV_TLV_LAST_FLOGI_TIMESTAMP,
    DRV_TLV_LAST_FLOGI_ACC_COMMON_PARAMETERS_WORD_1,
    DRV_TLV_LAST_FLOGI_ACC_COMMON_PARAMETERS_WORD_2,
    DRV_TLV_LAST_FLOGI_ACC_COMMON_PARAMETERS_WORD_3,
    DRV_TLV_LAST_FLOGI_ACC_COMMON_PARAMETERS_WORD_4,
    DRV_TLV_LAST_FLOGI_ACC_TIMESTAMP,
    DRV_TLV_LAST_FLOGI_RJT,
    DRV_TLV_LAST_FLOGI_RJT_TIMESTAMP,
    DRV_TLV_FDISCS_SENT_COUNT,
    DRV_TLV_FDISC_ACCS_RECEIVED,
    DRV_TLV_FDISC_RJTS_RECEIVED,
    DRV_TLV_PLOGI_SENT_COUNT,
    DRV_TLV_PLOGI_ACCS_RECEIVED,
    DRV_TLV_PLOGI_RJTS_RECEIVED,
    DRV_TLV_PLOGI_1_SENT_DESTINATION_FC_ID,
    DRV_TLV_PLOGI_1_TIMESTAMP,
    DRV_TLV_PLOGI_2_SENT_DESTINATION_FC_ID,
    DRV_TLV_PLOGI_2_TIMESTAMP,
    DRV_TLV_PLOGI_3_SENT_DESTINATION_FC_ID,
    DRV_TLV_PLOGI_3_TIMESTAMP,
    DRV_TLV_PLOGI_4_SENT_DESTINATION_FC_ID,
    DRV_TLV_PLOGI_4_TIMESTAMP,
    DRV_TLV_PLOGI_5_SENT_DESTINATION_FC_ID,
    DRV_TLV_PLOGI_5_TIMESTAMP,
    DRV_TLV_PLOGI_1_ACC_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_PLOGI_1_ACC_TIMESTAMP,
    DRV_TLV_PLOGI_2_ACC_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_PLOGI_2_ACC_TIMESTAMP,
    DRV_TLV_PLOGI_3_ACC_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_PLOGI_3_ACC_TIMESTAMP,
    DRV_TLV_PLOGI_4_ACC_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_PLOGI_4_ACC_TIMESTAMP,
    DRV_TLV_PLOGI_5_ACC_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_PLOGI_5_ACC_TIMESTAMP,
    DRV_TLV_LOGOS_ISSUED,
    DRV_TLV_LOGO_ACCS_RECEIVED,
    DRV_TLV_LOGO_RJTS_RECEIVED,
    DRV_TLV_LOGO_1_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_LOGO_1_TIMESTAMP,
    DRV_TLV_LOGO_2_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_LOGO_2_TIMESTAMP,
    DRV_TLV_LOGO_3_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_LOGO_3_TIMESTAMP,
    DRV_TLV_LOGO_4_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_LOGO_4_TIMESTAMP,
    DRV_TLV_LOGO_5_RECEIVED_SOURCE_FC_ID,
    DRV_TLV_LOGO_5_TIMESTAMP,
    DRV_TLV_LOGOS_RECEIVED,
    DRV_TLV_ACCS_ISSUED,
    DRV_TLV_PRLIS_ISSUED,
    DRV_TLV_ACCS_RECEIVED,
    DRV_TLV_ABTS_SENT_COUNT,
    DRV_TLV_ABTS_ACCS_RECEIVED,
    DRV_TLV_ABTS_RJTS_RECEIVED,
    DRV_TLV_ABTS_1_SENT_DESTINATION_FC_ID,
    DRV_TLV_ABTS_1_TIMESTAMP,
    DRV_TLV_ABTS_2_SENT_DESTINATION_FC_ID,
    DRV_TLV_ABTS_2_TIMESTAMP,
    DRV_TLV_ABTS_3_SENT_DESTINATION_FC_ID,
    DRV_TLV_ABTS_3_TIMESTAMP,
    DRV_TLV_ABTS_4_SENT_DESTINATION_FC_ID,
    DRV_TLV_ABTS_4_TIMESTAMP,
    DRV_TLV_ABTS_5_SENT_DESTINATION_FC_ID,
    DRV_TLV_ABTS_5_TIMESTAMP,
    DRV_TLV_RSCNS_RECEIVED,
    DRV_TLV_LAST_RSCN_RECEIVED_N_PORT_1,
    DRV_TLV_LAST_RSCN_RECEIVED_N_PORT_2,
    DRV_TLV_LAST_RSCN_RECEIVED_N_PORT_3,
    DRV_TLV_LAST_RSCN_RECEIVED_N_PORT_4,
    DRV_TLV_LUN_RESETS_ISSUED,
    DRV_TLV_ABORT_TASK_SETS_ISSUED,
    DRV_TLV_TPRLOS_SENT,
    DRV_TLV_NOS_SENT_COUNT,
    DRV_TLV_NOS_RECEIVED_COUNT,
    DRV_TLV_OLS_COUNT,
    DRV_TLV_LR_COUNT,
    DRV_TLV_LRR_COUNT,
    DRV_TLV_LIP_SENT_COUNT,
    DRV_TLV_LIP_RECEIVED_COUNT,
    DRV_TLV_EOFA_COUNT,
    DRV_TLV_EOFNI_COUNT,
    DRV_TLV_SCSI_STATUS_CHECK_CONDITION_COUNT,
    DRV_TLV_SCSI_STATUS_CONDITION_MET_COUNT,
    DRV_TLV_SCSI_STATUS_BUSY_COUNT,
    DRV_TLV_SCSI_STATUS_INTERMEDIATE_COUNT,
    DRV_TLV_SCSI_STATUS_INTERMEDIATE_CONDITION_MET_COUNT,
    DRV_TLV_SCSI_STATUS_RESERVATION_CONFLICT_COUNT,
    DRV_TLV_SCSI_STATUS_TASK_SET_FULL_COUNT,
    DRV_TLV_SCSI_STATUS_ACA_ACTIVE_COUNT,
    DRV_TLV_SCSI_STATUS_TASK_ABORTED_COUNT,
    DRV_TLV_SCSI_CHECK_CONDITION_1_RECEIVED_SK_ASC_ASCQ,
    DRV_TLV_SCSI_CHECK_1_TIMESTAMP,
    DRV_TLV_SCSI_CHECK_CONDITION_2_RECEIVED_SK_ASC_ASCQ,
    DRV_TLV_SCSI_CHECK_2_TIMESTAMP,
    DRV_TLV_SCSI_CHECK_CONDITION_3_RECEIVED_SK_ASC_ASCQ,
    DRV_TLV_SCSI_CHECK_3_TIMESTAMP,
    DRV_TLV_SCSI_CHECK_CONDITION_4_RECEIVED_SK_ASC_ASCQ,
    DRV_TLV_SCSI_CHECK_4_TIMESTAMP,
    DRV_TLV_SCSI_CHECK_CONDITION_5_RECEIVED_SK_ASC_ASCQ,
    DRV_TLV_SCSI_CHECK_5_TIMESTAMP,
// Category 30: iSCSI Function Data
    DRV_TLV_PDU_TX_DESCRIPTOR_QUEUE_AVG_DEPTH,
    DRV_TLV_PDU_RX_DESCRIPTORS_QUEUE_AVG_DEPTH,
    DRV_TLV_ISCSI_PDU_RX_FRAMES_RECEIVED,
    DRV_TLV_ISCSI_PDU_RX_BYTES_RECEIVED,
    DRV_TLV_ISCSI_PDU_TX_FRAMES_SENT,
    DRV_TLV_ISCSI_PDU_TX_BYTES_SENT,
    DRV_TLV_RDMA_DRV_VERSION
}

pub const I2C_DEV_ADDR_A2: c_uint = 0xa2;
pub const SFP_EEPROM_A2_TEMPERATURE_ADDR: c_uint = 0x60;
pub const SFP_EEPROM_A2_TEMPERATURE_SIZE: c_int = 2;
pub const SFP_EEPROM_A2_VCC_ADDR: c_uint = 0x62;
pub const SFP_EEPROM_A2_VCC_SIZE: c_int = 2;
pub const SFP_EEPROM_A2_TX_BIAS_ADDR: c_uint = 0x64;
pub const SFP_EEPROM_A2_TX_BIAS_SIZE: c_int = 2;
pub const SFP_EEPROM_A2_TX_POWER_ADDR: c_uint = 0x66;
pub const SFP_EEPROM_A2_TX_POWER_SIZE: c_int = 2;
pub const SFP_EEPROM_A2_RX_POWER_ADDR: c_uint = 0x68;
pub const SFP_EEPROM_A2_RX_POWER_SIZE: c_int = 2;
pub const I2C_DEV_ADDR_A0: c_uint = 0xa0;
pub const QSFP_EEPROM_A0_TEMPERATURE_ADDR: c_uint = 0x16;
pub const QSFP_EEPROM_A0_TEMPERATURE_SIZE: c_int = 2;
pub const QSFP_EEPROM_A0_VCC_ADDR: c_uint = 0x1a;
pub const QSFP_EEPROM_A0_VCC_SIZE: c_int = 2;
pub const QSFP_EEPROM_A0_TX1_BIAS_ADDR: c_uint = 0x2a;
pub const QSFP_EEPROM_A0_TX1_BIAS_SIZE: c_int = 2;
pub const QSFP_EEPROM_A0_TX1_POWER_ADDR: c_uint = 0x32;
pub const QSFP_EEPROM_A0_TX1_POWER_SIZE: c_int = 2;
pub const QSFP_EEPROM_A0_RX1_POWER_ADDR: c_uint = 0x22;
pub const QSFP_EEPROM_A0_RX1_POWER_SIZE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg_mac_address {
    pub mac_addr_hi: u32,
pub const NVM_CFG_MAC_ADDRESS_HI_MASK: c_uint = 0x0000ffff;
pub const NVM_CFG_MAC_ADDRESS_HI_OFFSET: c_int = 0;
    pub mac_addr_lo: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg1_glob {
    pub generic_cont0: u32,
pub const NVM_CFG1_GLOB_MF_MODE_MASK: c_uint = 0x00000ff0;
pub const NVM_CFG1_GLOB_MF_MODE_OFFSET: c_int = 4;
pub const NVM_CFG1_GLOB_MF_MODE_MF_ALLOWED: c_uint = 0x0;
pub const NVM_CFG1_GLOB_MF_MODE_DEFAULT: c_uint = 0x1;
pub const NVM_CFG1_GLOB_MF_MODE_SPIO4: c_uint = 0x2;
pub const NVM_CFG1_GLOB_MF_MODE_NPAR1_0: c_uint = 0x3;
pub const NVM_CFG1_GLOB_MF_MODE_NPAR1_5: c_uint = 0x4;
pub const NVM_CFG1_GLOB_MF_MODE_NPAR2_0: c_uint = 0x5;
pub const NVM_CFG1_GLOB_MF_MODE_BD: c_uint = 0x6;
pub const NVM_CFG1_GLOB_MF_MODE_UFP: c_uint = 0x7;
    pub engineering_change: [u32; 3],
    pub manufacturing_id: u32,
    pub serial_number: [u32; 4],
    pub pcie_cfg: u32,
    pub mgmt_traffic: u32,
    pub core_cfg: u32,
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_MASK: c_uint = 0x000000ff;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_OFFSET: c_int = 0;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_BB_2X40G: c_uint = 0x0;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_2X50G: c_uint = 0x1;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_BB_1X100G: c_uint = 0x2;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_4X10G_F: c_uint = 0x3;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_BB_4X10G_E: c_uint = 0x4;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_BB_4X20G: c_uint = 0x5;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_1X40G: c_uint = 0xb;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_2X25G: c_uint = 0xc;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_1X25G: c_uint = 0xd;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_4X25G: c_uint = 0xe;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_2X10G: c_uint = 0xf;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_AHP_2X50G_R1: c_uint = 0x11;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_AHP_4X50G_R1: c_uint = 0x12;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_AHP_1X100G_R2: c_uint = 0x13;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_AHP_2X100G_R2: c_uint = 0x14;
pub const NVM_CFG1_GLOB_NETWORK_PORT_MODE_AHP_1X100G_R4: c_uint = 0x15;
    pub e_lane_cfg1: u32,
    pub e_lane_cfg2: u32,
    pub f_lane_cfg1: u32,
    pub f_lane_cfg2: u32,
    pub mps10_preemphasis: u32,
    pub mps10_driver_current: u32,
    pub mps25_preemphasis: u32,
    pub mps25_driver_current: u32,
    pub pci_id: u32,
    pub pci_subsys_id: u32,
    pub bar: u32,
    pub mps10_txfir_main: u32,
    pub mps10_txfir_post: u32,
    pub mps25_txfir_main: u32,
    pub mps25_txfir_post: u32,
    pub manufacture_ver: u32,
    pub manufacture_time: u32,
    pub led_global_settings: u32,
    pub generic_cont1: u32,
    pub mbi_version: u32,
pub const NVM_CFG1_GLOB_MBI_VERSION_0_MASK: c_uint = 0x000000ff;
pub const NVM_CFG1_GLOB_MBI_VERSION_0_OFFSET: c_int = 0;
pub const NVM_CFG1_GLOB_MBI_VERSION_1_MASK: c_uint = 0x0000ff00;
pub const NVM_CFG1_GLOB_MBI_VERSION_1_OFFSET: c_int = 8;
pub const NVM_CFG1_GLOB_MBI_VERSION_2_MASK: c_uint = 0x00ff0000;
pub const NVM_CFG1_GLOB_MBI_VERSION_2_OFFSET: c_int = 16;
    pub mbi_date: u32,
    pub misc_sig: u32,
    pub device_capabilities: u32,
pub const NVM_CFG1_GLOB_DEVICE_CAPABILITIES_ETHERNET: c_uint = 0x1;
pub const NVM_CFG1_GLOB_DEVICE_CAPABILITIES_FCOE: c_uint = 0x2;
pub const NVM_CFG1_GLOB_DEVICE_CAPABILITIES_ISCSI: c_uint = 0x4;
pub const NVM_CFG1_GLOB_DEVICE_CAPABILITIES_ROCE: c_uint = 0x8;
pub const NVM_CFG1_GLOB_DEVICE_CAPABILITIES_IWARP: c_uint = 0x10;
    pub power_dissipated: u32,
    pub power_consumed: u32,
    pub efi_version: u32,
    pub multi_network_modes_capability: u32,
    pub nvm_cfg_version: u32,
    pub nvm_cfg_new_option_seq: u32,
    pub nvm_cfg_removed_option_seq: u32,
    pub nvm_cfg_updated_value_seq: u32,
    pub extended_serial_number: [u32; 8],
    pub option_kit_pn: [u32; 8],
    pub spare_pn: [u32; 8],
    pub mps25_active_txfir_pre: u32,
    pub mps25_active_txfir_main: u32,
    pub mps25_active_txfir_post: u32,
    pub features: u32,
    pub tx_rx_eq_25g_hlpc: u32,
    pub tx_rx_eq_25g_llpc: u32,
    pub tx_rx_eq_25g_ac: u32,
    pub tx_rx_eq_10g_pc: u32,
    pub tx_rx_eq_10g_ac: u32,
    pub tx_rx_eq_1g: u32,
    pub tx_rx_eq_25g_bt: u32,
    pub tx_rx_eq_10g_bt: u32,
    pub generic_cont4: u32,
    pub preboot_debug_mode_std: u32,
    pub preboot_debug_mode_ext: u32,
    pub ext_phy_cfg1: u32,
    pub clocks: u32,
    pub pre2_generic_cont_1: u32,
    pub pre2_generic_cont_2: u32,
    pub pre2_generic_cont_3: u32,
    pub tx_rx_eq_50g_hlpc: u32,
    pub tx_rx_eq_50g_mlpc: u32,
    pub tx_rx_eq_50g_llpc: u32,
    pub tx_rx_eq_50g_ac: u32,
    pub trace_modules: u32,
    pub pcie_class_code_fcoe: u32,
    pub pcie_class_code_iscsi: u32,
    pub no_provisioned_mac: u32,
    pub lowest_mbi_version: u32,
    pub generic_cont5: u32,
    pub pre2_generic_cont_4: u32,
    pub reserved: [u32; 40],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg1_path {
    pub reserved: [u32; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg1_port {
    pub rel_to_opt123: u32,
    pub rel_to_opt124: u32,
    pub generic_cont0: u32,
pub const NVM_CFG1_PORT_DCBX_MODE_MASK: c_uint = 0x000f0000;
pub const NVM_CFG1_PORT_DCBX_MODE_OFFSET: c_int = 16;
pub const NVM_CFG1_PORT_DCBX_MODE_DISABLED: c_uint = 0x0;
pub const NVM_CFG1_PORT_DCBX_MODE_IEEE: c_uint = 0x1;
pub const NVM_CFG1_PORT_DCBX_MODE_CEE: c_uint = 0x2;
pub const NVM_CFG1_PORT_DCBX_MODE_DYNAMIC: c_uint = 0x3;
pub const NVM_CFG1_PORT_DEFAULT_ENABLED_PROTOCOLS_MASK: c_uint = 0x00f00000;
pub const NVM_CFG1_PORT_DEFAULT_ENABLED_PROTOCOLS_OFFSET: c_int = 20;
pub const NVM_CFG1_PORT_DEFAULT_ENABLED_PROTOCOLS_ETHERNET: c_uint = 0x1;
pub const NVM_CFG1_PORT_DEFAULT_ENABLED_PROTOCOLS_FCOE: c_uint = 0x2;
pub const NVM_CFG1_PORT_DEFAULT_ENABLED_PROTOCOLS_ISCSI: c_uint = 0x4;
    pub pcie_cfg: u32,
    pub features: u32,
    pub speed_cap_mask: u32,
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_MASK: c_uint = 0x0000ffff;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_OFFSET: c_int = 0;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_1G: c_uint = 0x1;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_10G: c_uint = 0x2;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_20G: c_uint = 0x4;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_25G: c_uint = 0x8;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_40G: c_uint = 0x10;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_50G: c_uint = 0x20;
pub const NVM_CFG1_PORT_DRV_SPEED_CAPABILITY_MASK_BB_100G: c_uint = 0x40;
    pub link_settings: u32,
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_MASK: c_uint = 0x0000000f;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_OFFSET: c_int = 0;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_AUTONEG: c_uint = 0x0;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_1G: c_uint = 0x1;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_10G: c_uint = 0x2;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_20G: c_uint = 0x3;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_25G: c_uint = 0x4;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_40G: c_uint = 0x5;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_50G: c_uint = 0x6;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_BB_100G: c_uint = 0x7;
pub const NVM_CFG1_PORT_DRV_LINK_SPEED_SMARTLINQ: c_uint = 0x8;
pub const NVM_CFG1_PORT_DRV_FLOW_CONTROL_MASK: c_uint = 0x00000070;
pub const NVM_CFG1_PORT_DRV_FLOW_CONTROL_OFFSET: c_int = 4;
pub const NVM_CFG1_PORT_DRV_FLOW_CONTROL_AUTONEG: c_uint = 0x1;
pub const NVM_CFG1_PORT_DRV_FLOW_CONTROL_RX: c_uint = 0x2;
pub const NVM_CFG1_PORT_DRV_FLOW_CONTROL_TX: c_uint = 0x4;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_MASK: c_uint = 0x000e0000;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_OFFSET: c_int = 17;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_NONE: c_uint = 0x0;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_FIRECODE: c_uint = 0x1;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_RS: c_uint = 0x2;
pub const NVM_CFG1_PORT_FEC_FORCE_MODE_AUTO: c_uint = 0x7;
    pub phy_cfg: u32,
    pub mgmt_traffic: u32,
    pub ext_phy: u32,
// EEE power saving mode
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_MASK: c_uint = 0x00ff0000;
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_OFFSET: c_int = 16;
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_DISABLED: c_uint = 0x0;
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_BALANCED: c_uint = 0x1;
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_AGGRESSIVE: c_uint = 0x2;
pub const NVM_CFG1_PORT_EEE_POWER_SAVING_MODE_LOW_LATENCY: c_uint = 0x3;
    pub mba_cfg1: u32,
    pub mba_cfg2: u32,
    pub vf_cfg: u32,
    pub lldp_mac_address: nvm_cfg_mac_address,
    pub led_port_settings: u32,
    pub transceiver_00: u32,
    pub device_ids: u32,
    pub board_cfg: u32,
pub const NVM_CFG1_PORT_PORT_TYPE_MASK: c_uint = 0x000000ff;
pub const NVM_CFG1_PORT_PORT_TYPE_OFFSET: c_int = 0;
pub const NVM_CFG1_PORT_PORT_TYPE_UNDEFINED: c_uint = 0x0;
pub const NVM_CFG1_PORT_PORT_TYPE_MODULE: c_uint = 0x1;
pub const NVM_CFG1_PORT_PORT_TYPE_BACKPLANE: c_uint = 0x2;
pub const NVM_CFG1_PORT_PORT_TYPE_EXT_PHY: c_uint = 0x3;
pub const NVM_CFG1_PORT_PORT_TYPE_MODULE_SLAVE: c_uint = 0x4;
    pub mnm_10g_cap: u32,
    pub mnm_10g_ctrl: u32,
    pub mnm_10g_misc: u32,
    pub mnm_25g_cap: u32,
    pub mnm_25g_ctrl: u32,
    pub mnm_25g_misc: u32,
    pub mnm_40g_cap: u32,
    pub mnm_40g_ctrl: u32,
    pub mnm_40g_misc: u32,
    pub mnm_50g_cap: u32,
    pub mnm_50g_ctrl: u32,
    pub mnm_50g_misc: u32,
    pub mnm_100g_cap: u32,
    pub mnm_100g_ctrl: u32,
    pub mnm_100g_misc: u32,
    pub temperature: u32,
    pub ext_phy_cfg1: u32,
    pub extended_speed: u32,
pub const NVM_CFG1_PORT_EXTENDED_SPEED_MASK: c_uint = 0x0000ffff;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_OFFSET: c_int = 0;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_AN: c_uint = 0x1;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_1G: c_uint = 0x2;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_10G: c_uint = 0x4;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_20G: c_uint = 0x8;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_25G: c_uint = 0x10;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_40G: c_uint = 0x20;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_50G_R: c_uint = 0x40;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_50G_R2: c_uint = 0x80;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_100G_R2: c_uint = 0x100;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_100G_R4: c_uint = 0x200;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_EXTND_SPD_100G_P4: c_uint = 0x400;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_MASK: c_uint = 0xffff0000;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_OFFSET: c_int = 16;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_RESERVED: c_uint = 0x1;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_1G: c_uint = 0x2;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_10G: c_uint = 0x4;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_20G: c_uint = 0x8;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_25G: c_uint = 0x10;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_40G: c_uint = 0x20;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_50G_R: c_uint = 0x40;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_50G_R2: c_uint = 0x80;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_100G_R2: c_uint = 0x100;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_100G_R4: c_uint = 0x200;
pub const NVM_CFG1_PORT_EXTENDED_SPEED_CAP_EXTND_SPD_100G_P4: c_uint = 0x400;
    pub extended_fec_mode: u32,
    pub port_generic_cont_01: u32,
    pub port_generic_cont_02: u32,
    pub phy_temp_monitor: u32,
    pub reserved: [u32; 109],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg1_func {
    pub mac_address: nvm_cfg_mac_address,
    pub rsrv1: u32,
    pub rsrv2: u32,
    pub device_id: u32,
    pub cmn_cfg: u32,
    pub pci_cfg: u32,
    pub fcoe_node_wwn_mac_addr: nvm_cfg_mac_address,
    pub fcoe_port_wwn_mac_addr: nvm_cfg_mac_address,
    pub preboot_generic_cfg: u32,
    pub features: u32,
    pub mf_mode_feature: u32,
    pub reserved: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg1 {
    pub glob: nvm_cfg1_glob,
    pub path: [nvm_cfg1_path; MCP_GLOB_PATH_MAX],
    pub port: [nvm_cfg1_port; MCP_GLOB_PORT_MAX],
    pub func: [nvm_cfg1_func; MCP_GLOB_FUNC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct board_info {
    pub vendor_id: u16,
    pub eth_did_suffix: u16,
    pub sub_vendor_id: u16,
    pub sub_device_id: u16,
    pub board_name: *mut c_char,
    pub friendly_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_module_info {
    pub module_name: *mut c_char,
}

pub const NUM_TRACE_MODULES: c_int = 25;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvm_cfg_sections {
    NVM_CFG_SECTION_NVM_CFG1,
    NVM_CFG_SECTION_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_cfg {
    pub num_sections: u32,
    pub sections_offset: [u32; NVM_CFG_SECTION_MAX],
    pub cfg1: nvm_cfg1,
}

pub const PORT_0: c_int = 0;
pub const PORT_1: c_int = 1;
pub const PORT_2: c_int = 2;
pub const PORT_3: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spad_layout {
    pub nvm_cfg: nvm_cfg,
    pub public_data: mcp_public_data,
}

pub const MCP_SPAD_SIZE: c_uint = 0x00028000	/* 160 KB */;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spad_sections {
    SPAD_SECTION_TRACE,
    SPAD_SECTION_NVM_CFG,
    SPAD_SECTION_PUBLIC,
    SPAD_SECTION_PRIVATE,
    SPAD_SECTION_MAX
}

// This section is located at a fixed location in the beginning of the
// scratchpad, to ensure that the MCP trace is not run over during MFW upgrade.
// All the rest of data has a floating location which differs from version to
// version, and is pointed by the mcp_meta_data below.
// Moreover, the spad_layout section is part of the MFW firmware, and is loaded
// with it from nvram in order to clear this portion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_init {
    pub num_sections: u32,
    pub sections: [offsize_t; SPAD_SECTION_MAX],    pub tim_hash: [u32; 8],    pub tpu_hash: [u32; 8],
    pub secure_pcie_fw_ver: u32,

    pub secure_running_mfw: u32,

    pub trace: mcp_trace,
}

pub const CRC_MAGIC_VALUE: c_uint = 0xDEBB20E3;
pub const CRC32_POLYNOMIAL: c_uint = 0xEDB88320;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvm_sw_arbitrator {
    NVM_SW_ARB_HOST,
    NVM_SW_ARB_MCP,
    NVM_SW_ARB_UART,
    NVM_SW_ARB_RESERVED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_bootstrap_region {
    pub magic_value: u32,
pub const NVM_MAGIC_VALUE: c_uint = 0x669955aa;
    pub sram_start_addr: u32,
    pub code_len: u32,
    pub code_start_addr: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_code_entry {
    pub image_type: u32,
    pub nvm_start_addr: u32,
    pub len: u32,
    pub sram_start_addr: u32,
    pub sram_run_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvm_image_type {
    NVM_TYPE_TIM1 = 0x01,
    NVM_TYPE_TIM2 = 0x02,
    NVM_TYPE_MIM1 = 0x03,
    NVM_TYPE_MIM2 = 0x04,
    NVM_TYPE_MBA = 0x05,
    NVM_TYPE_MODULES_PN = 0x06,
    NVM_TYPE_VPD = 0x07,
    NVM_TYPE_MFW_TRACE1 = 0x08,
    NVM_TYPE_MFW_TRACE2 = 0x09,
    NVM_TYPE_NVM_CFG1 = 0x0a,
    NVM_TYPE_L2B = 0x0b,
    NVM_TYPE_DIR1 = 0x0c,
    NVM_TYPE_EAGLE_FW1 = 0x0d,
    NVM_TYPE_FALCON_FW1 = 0x0e,
    NVM_TYPE_PCIE_FW1 = 0x0f,
    NVM_TYPE_HW_SET = 0x10,
    NVM_TYPE_LIM = 0x11,
    NVM_TYPE_AVS_FW1 = 0x12,
    NVM_TYPE_DIR2 = 0x13,
    NVM_TYPE_CCM = 0x14,
    NVM_TYPE_EAGLE_FW2 = 0x15,
    NVM_TYPE_FALCON_FW2 = 0x16,
    NVM_TYPE_PCIE_FW2 = 0x17,
    NVM_TYPE_AVS_FW2 = 0x18,
    NVM_TYPE_INIT_HW = 0x19,
    NVM_TYPE_DEFAULT_CFG = 0x1a,
    NVM_TYPE_MDUMP = 0x1b,
    NVM_TYPE_NVM_META = 0x1c,
    NVM_TYPE_ISCSI_CFG = 0x1d,
    NVM_TYPE_FCOE_CFG = 0x1f,
    NVM_TYPE_ETH_PHY_FW1 = 0x20,
    NVM_TYPE_ETH_PHY_FW2 = 0x21,
    NVM_TYPE_BDN = 0x22,
    NVM_TYPE_8485X_PHY_FW = 0x23,
    NVM_TYPE_PUB_KEY = 0x24,
    NVM_TYPE_RECOVERY = 0x25,
    NVM_TYPE_PLDM = 0x26,
    NVM_TYPE_UPK1 = 0x27,
    NVM_TYPE_UPK2 = 0x28,
    NVM_TYPE_MASTER_KC = 0x29,
    NVM_TYPE_BACKUP_KC = 0x2a,
    NVM_TYPE_HW_DUMP = 0x2b,
    NVM_TYPE_HW_DUMP_OUT = 0x2c,
    NVM_TYPE_BIN_NVM_META = 0x30,
    NVM_TYPE_ROM_TEST = 0xf0,
    NVM_TYPE_88X33X0_PHY_FW = 0x31,
    NVM_TYPE_88X33X0_PHY_SLAVE_FW = 0x32,
    NVM_TYPE_IDLE_CHK = 0x33,
    NVM_TYPE_MAX,
}

pub const MAX_NVM_DIR_ENTRIES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_dir_meta {
    pub dir_id: u32,
    pub nvm_dir_addr: u32,
    pub num_images: u32,
    pub next_mfw_to_run: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_dir {
    pub seq: i32,
pub const NVM_DIR_NEXT_MFW_MASK: c_uint = 0x00000001;
pub const NVM_DIR_SEQ_MASK: c_uint = 0xfffffffe;

    pub (swap_mfw))));\: (NVM_DIR_NEXT_MFW(_seq ^,

    pub num_images: u32,
    pub rsrv: u32,
    pub /: *mut *mut nvm_code_entry code[1]; / Up to MAX_NVM_DIR_ENTRIES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_vpd_image {
    pub format_revision: u32,
pub const VPD_IMAGE_VERSION: c_int = 1;
    pub vpd_data: [u8; 1],
}

pub const FLASH_PAGE_SIZE: c_uint = 0x1000;

#[repr(C)]
#[derive(Copy, Clone)]
pub union nvm_dir_union {
    pub dir: nvm_dir,
    pub page: [u8; FLASH_PAGE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_image {
    pub bootstrap: legacy_bootstrap_region,
    pub rsrv: [u8; NVM_RSV_SIZE],
    pub lim_image: [u8; LIM_MAX_SIZE],
    pub dir: [nvm_dir_union; MAX_MFW_BUNDLES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_set_info {
    pub reg_type: u32,
pub const GRC_REG_TYPE: c_int = 1;
pub const PHY_REG_TYPE: c_int = 2;
pub const PCI_REG_TYPE: c_int = 4;
    pub bank_num: u32,
    pub pf_num: u32,
    pub operation: u32,
pub const READ_OP: c_int = 1;
pub const WRITE_OP: c_int = 2;
pub const RMW_SET_OP: c_int = 3;
pub const RMW_CLR_OP: c_int = 4;
    pub reg_addr: u32,
    pub reg_data: u32,
    pub reset_type: u32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_set_image {
    pub format_version: u32,
pub const HW_SET_IMAGE_VERSION: c_int = 1;
    pub no_hw_sets: u32,
    pub hw_sets: [hw_set_info; 1],
}

pub const MAX_SUPPORTED_NVM_OPTIONS: c_int = 1000;
pub const NVM_META_BIN_OPTION_OFFSET_MASK: c_uint = 0x0000ffff;
pub const NVM_META_BIN_OPTION_OFFSET_SHIFT: c_int = 0;
pub const NVM_META_BIN_OPTION_LEN_MASK: c_uint = 0x00ff0000;
pub const NVM_META_BIN_OPTION_LEN_OFFSET: c_int = 16;
pub const NVM_META_BIN_OPTION_ENTITY_MASK: c_uint = 0x03000000;
pub const NVM_META_BIN_OPTION_ENTITY_SHIFT: c_int = 24;
pub const NVM_META_BIN_OPTION_ENTITY_GLOB: c_int = 0;
pub const NVM_META_BIN_OPTION_ENTITY_PORT: c_int = 1;
pub const NVM_META_BIN_OPTION_ENTITY_FUNC: c_int = 2;
pub const NVM_META_BIN_OPTION_CONFIG_TYPE_MASK: c_uint = 0x0c000000;
pub const NVM_META_BIN_OPTION_CONFIG_TYPE_SHIFT: c_int = 26;
pub const NVM_META_BIN_OPTION_CONFIG_TYPE_USER: c_int = 0;
pub const NVM_META_BIN_OPTION_CONFIG_TYPE_FIXED: c_int = 1;
pub const NVM_META_BIN_OPTION_CONFIG_TYPE_FORCED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvm_meta_bin_t {
    pub magic: u32,
pub const NVM_META_BIN_MAGIC: c_uint = 0x669955bb;
    pub version: u32,
pub const NVM_META_BIN_VERSION: c_int = 1;
    pub num_options: u32,
    pub options: [u32; ],
}
