//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/aquantia/atlantic/hw_atl2/hw_atl2_llh_internal.h
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
// Atlantic Network Driver
// Copyright (C) 2020 Marvell International Ltd.
//
// RX timestamp_req_desc{D} [1:0] Bitfield Definitions
//

pub const HW_ATL2_RPF_TIMESTAMP_REQ_DESCD_MSK: c_uint = 0x00030000;
pub const HW_ATL2_RPF_TIMESTAMP_REQ_DESCD_SHIFT: c_int = 16;
// RX pif_rpf_redir_2_en_i Bitfield Definitions
// PORT="pif_rpf_redir_2_en_i"
//
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_ADR: c_uint = 0x000054C8;
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_MSK: c_uint = 0x00001000;
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_MSKN: c_uint = 0xFFFFEFFF;
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_SHIFT: c_int = 12;
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_WIDTH: c_int = 1;
pub const HW_ATL2_RPF_PIF_RPF_REDIR2_ENI_DEFAULT: c_uint = 0x0;
// RX pif_rpf_rss_hash_type_i Bitfield Definitions
//
pub const HW_ATL2_RPF_PIF_RPF_RSS_HASH_TYPEI_ADR: c_uint = 0x000054C8;
pub const HW_ATL2_RPF_PIF_RPF_RSS_HASH_TYPEI_MSK: c_uint = 0x000001FF;
pub const HW_ATL2_RPF_PIF_RPF_RSS_HASH_TYPEI_MSKN: c_uint = 0xFFFFFE00;
pub const HW_ATL2_RPF_PIF_RPF_RSS_HASH_TYPEI_SHIFT: c_int = 0;
pub const HW_ATL2_RPF_PIF_RPF_RSS_HASH_TYPEI_WIDTH: c_int = 9;
// rx rpf_new_rpf_en bitfield definitions
// preprocessor definitions for the bitfield "rpf_new_rpf_en_i".
// port="pif_rpf_new_rpf_en_i
//
// register address for bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_ADR: c_uint = 0x00005104;
// bitmask for bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_MSK: c_uint = 0x00000800;
// inverted bitmask for bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_MSKN: c_uint = 0xfffff7ff;
// lower bit position of bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_SHIFT: c_int = 11;
// width of bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_WIDTH: c_int = 1;
// default value of bitfield rpf_new_rpf_en
pub const HW_ATL2_RPF_NEW_EN_DEFAULT: c_uint = 0x0;
// rx l2_uc_req_tag0{f}[5:0] bitfield definitions
// preprocessor definitions for the bitfield "l2_uc_req_tag0{f}[7:0]".
// parameter: filter {f} | stride size 0x8 | range [0, 37]
// port="pif_rpf_l2_uc_req_tag0[5:0]"
//
// register address for bitfield l2_uc_req_tag0{f}[2:0]

// bitmask for bitfield l2_uc_req_tag0{f}[2:0]
pub const HW_ATL2_RPFL2UC_TAG_MSK: c_uint = 0x0FC00000;
// inverted bitmask for bitfield l2_uc_req_tag0{f}[2:0]
pub const HW_ATL2_RPFL2UC_TAG_MSKN: c_uint = 0xF03FFFFF;
// lower bit position of bitfield l2_uc_req_tag0{f}[2:0]
pub const HW_ATL2_RPFL2UC_TAG_SHIFT: c_int = 22;
// width of bitfield l2_uc_req_tag0{f}[2:0]
pub const HW_ATL2_RPFL2UC_TAG_WIDTH: c_int = 6;
// default value of bitfield l2_uc_req_tag0{f}[2:0]
pub const HW_ATL2_RPFL2UC_TAG_DEFAULT: c_uint = 0x0;
// rpf_l2_bc_req_tag[5:0] bitfield definitions
// preprocessor definitions for the bitfield "rpf_l2_bc_req_tag[5:0]".
// port="pifrpf_l2_bc_req_tag_i[5:0]"
//
// register address for bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_ADR: c_uint = 0x000050F0;
// bitmask for bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_MSK: c_uint = 0x0000003F;
// inverted bitmask for bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_MSKN: c_uint = 0xffffffc0;
// lower bit position of bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_SHIFT: c_int = 0;
// width of bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_WIDTH: c_int = 6;
// default value of bitfield rpf_l2_bc_req_tag
pub const HW_ATL2_RPF_L2_BC_TAG_DEFAULT: c_uint = 0x0;
// rx rpf_rss_red1_data_[4:0] bitfield definitions
// preprocessor definitions for the bitfield "rpf_rss_red1_data[4:0]".
// port="pif_rpf_rss_red1_data_i[4:0]"
//
// register address for bitfield rpf_rss_red1_data[4:0]

// bitmask for bitfield rpf_rss_red1_data[4:0]

// lower bit position of bitfield rpf_rss_red1_data[4:0]

// width of bitfield rpf_rss_red1_data[4:0]
pub const HW_ATL2_RPF_RSS_REDIR_WIDTH: c_int = 5;
// default value of bitfield rpf_rss_red1_data[4:0]
pub const HW_ATL2_RPF_RSS_REDIR_DEFAULT: c_uint = 0x0;
// rx vlan_req_tag0{f}[3:0] bitfield definitions
// preprocessor definitions for the bitfield "vlan_req_tag0{f}[3:0]".
// parameter: filter {f} | stride size 0x4 | range [0, 15]
// port="pif_rpf_vlan_req_tag0[3:0]"
//
// register address for bitfield vlan_req_tag0{f}[3:0]

// bitmask for bitfield vlan_req_tag0{f}[3:0]
pub const HW_ATL2_RPF_VL_TAG_MSK: c_uint = 0x0000F000;
// inverted bitmask for bitfield vlan_req_tag0{f}[3:0]
pub const HW_ATL2_RPF_VL_TAG_MSKN: c_uint = 0xFFFF0FFF;
// lower bit position of bitfield vlan_req_tag0{f}[3:0]
pub const HW_ATL2_RPF_VL_TAG_SHIFT: c_int = 12;
// width of bitfield vlan_req_tag0{f}[3:0]
pub const HW_ATL2_RPF_VL_TAG_WIDTH: c_int = 4;
// default value of bitfield vlan_req_tag0{f}[3:0]
pub const HW_ATL2_RPF_VL_TAG_DEFAULT: c_uint = 0x0;
// register address for bitfield etype_req_tag0{f}[2:0]

// bitmask for bitfield etype_req_tag0{f}[2:0]
pub const HW_ATL2_RPF_ET_TAG_MSK: c_uint = 0x00000007;
// lower bit position of bitfield etype_req_tag0{f}[2:0]
pub const HW_ATL2_RPF_ET_TAG_SHIFT: c_int = 0;
// Lower bit position of bitfield l3_l4_act{F}[2:0]
pub const HW_ATL2_RPF_L3_L4_ACTF_SHIFT: c_int = 16;
// Bitmask for bitfield l3_l4_rxq{F}[4:0]
pub const HW_ATL2_RPF_L3_L4_RXQF_MSK: c_uint = 0x00001F00u;
// Lower bit position of bitfield l3_l4_rxq{F}[4:0]
pub const HW_ATL2_RPF_L3_L4_RXQF_SHIFT: c_int = 8;
// Register address for bitfield rpf_l3_v6_sa{F}_dw{D}[1F:0]

// Register address for bitfield rpf_l3_v6_da{F}_dw{D}[1F:0]

// Register address for bitfield rpf_l3_cmd{F}[1F:0]

// Bitmask for bitfield rpf_l3_cmd{F}[F:0]
pub const HW_ATL2_RPF_L3_V4_CMD_MSK: c_uint = 0x0000FFFFu;
// Lower bit position of bitfield rpf_l3_cmd{F}[1F:0]
pub const HW_ATL2_RPF_L3_V4_CMD_SHIFT: c_int = 0;
// Register address for bitfield rpf_l3_v6_cmd{F}[1F:0]

// Bitmask for bitfield rpf_l3_v6_cmd{F}[F:0]
pub const HW_ATL2_RPF_L3_V6_CMD_MSK: c_uint = 0xFF7F0000u;
// Lower bit position of bitfield rpf_l3_v6_cmd{F}[1F:0]
pub const HW_ATL2_RPF_L3_V6_CMD_SHIFT: c_int = 0;
// Register address for bitfield rpf_l3_v6_cmd{F}[F:0]
pub const HW_ATL2_RPF_L3_V6_V4_SELECT_ADR: c_uint = 0x00006500u;
// Bitmask for bitfield pif_rpf_l3_v6_v4_select
pub const HW_ATL2_RPF_L3_V6_V4_SELECT_MSK: c_uint = 0x00800000u;
// Lower bit position of bitfield pif_rpf_l3_v6_v4_select
pub const HW_ATL2_RPF_L3_V6_V4_SELECT_SHIFT: c_int = 23;
// Register address for bitfield rpf_l3_v4_req_tag{F}[2:0]

// Bitmask for bitfield rpf_l3_v4_req_tag{F}[2:0]
pub const HW_ATL2_RPF_L3_V4_TAG_MSK: c_uint = 0x00000070u;
// Lower bit position of bitfield rpf_l3_v4_req_tag{F}[2:0]
pub const HW_ATL2_RPF_L3_V4_TAG_SHIFT: c_int = 4;
// Register address for bitfield rpf_l3_v6_req_tag{F}[2:0]

// Bitmask for bitfield rpf_l3_v6_req_tag{F}[2:0]
pub const HW_ATL2_RPF_L3_V6_TAG_MSK: c_uint = 0x00700000;
// Lower bit position of bitfield rpf_l3_v6_req_tag{F}[2:0]
pub const HW_ATL2_RPF_L3_V6_TAG_SHIFT: c_int = 20;
// Register address for bitfield rpf_l4_cmd{F}[2:0]

// Bitmask for bitfield rpf_l4_cmd{F}[2:0]
pub const HW_ATL2_RPF_L4_CMD_MSK: c_uint = 0x00000007u;
// Lower bit position of bitfield rpf_l4_cmd{F}[2:0]
pub const HW_ATL2_RPF_L4_CMD_SHIFT: c_int = 0;
// Register address for bitfield rpf_l4_tag{F}[2:0]

// Bitmask for bitfield rpf_l4_tag{F}[2:0]
pub const HW_ATL2_RPF_L4_TAG_MSK: c_uint = 0x00000070u;
// Lower bit position of bitfield rpf_l4_tag{F}[2:0]
pub const HW_ATL2_RPF_L4_TAG_SHIFT: c_int = 4;
// RX rx_q{Q}_tc_map[2:0] Bitfield Definitions
// Preprocessor definitions for the bitfield "rx_q{Q}_tc_map[2:0]".
// Parameter: Queue {Q} | bit-level stride | range [0, 31]
// PORT="pif_rx_q0_tc_map_i[2:0]"
//
// Register address for bitfield rx_q{Q}_tc_map[2:0]

// Lower bit position of bitfield rx_q{Q}_tc_map[2:0]

// Width of bitfield rx_q{Q}_tc_map[2:0]
pub const HW_ATL2_RX_Q_TC_MAP_WIDTH: c_int = 3;
// Default value of bitfield rx_q{Q}_tc_map[2:0]
pub const HW_ATL2_RX_Q_TC_MAP_DEFAULT: c_uint = 0x0;
// TX desc{D}_ts_wrb_en Bitfield Definitions
//

pub const HW_ATL2_TDM_DESCD_TS_WRB_EN_MSK: c_uint = 0x00040000;
pub const HW_ATL2_TDM_DESCD_TS_WRB_EN_SHIFT: c_int = 18;
// TX desc{D}_ts_en Bitfield Definitions
//

pub const HW_ATL2_TDM_DESCD_TS_EN_MSK: c_uint = 0x00020000;
pub const HW_ATL2_TDM_DESCD_TS_EN_SHIFT: c_int = 17;
// TX desc{D}_avb_en Bitfield Definitions
//

pub const HW_ATL2_TDM_DESCD_AVB_EN_MSK: c_uint = 0x00010000;
pub const HW_ATL2_TDM_DESCD_AVB_EN_SHIFT: c_int = 16;
// tx tx_tc_q_rand_map_en bitfield definitions
// preprocessor definitions for the bitfield "tx_tc_q_rand_map_en".
// port="pif_tpb_tx_tc_q_rand_map_en_i"
//
// register address for bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_ADR: c_uint = 0x00007900;
// bitmask for bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_MSK: c_uint = 0x00000200;
// inverted bitmask for bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_MSKN: c_uint = 0xFFFFFDFF;
// lower bit position of bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_SHIFT: c_int = 9;
// width of bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_WIDTH: c_int = 1;
// default value of bitfield tx_tc_q_rand_map_en
pub const HW_ATL2_TPB_TX_TC_Q_RAND_MAP_EN_DEFAULT: c_uint = 0x0;
// tx tx_buffer_clk_gate_en bitfield definitions
// preprocessor definitions for the bitfield "tx_buffer_clk_gate_en".
// port="pif_tpb_tx_buffer_clk_gate_en_i"
//
// register address for bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_ADR: c_uint = 0x00007900;
// bitmask for bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_MSK: c_uint = 0x00000020;
// inverted bitmask for bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_MSKN: c_uint = 0xffffffdf;
// lower bit position of bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_SHIFT: c_int = 5;
// width of bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_WIDTH: c_int = 1;
// default value of bitfield tx_buffer_clk_gate_en
pub const HW_ATL2_TPB_TX_BUF_CLK_GATE_EN_DEFAULT: c_uint = 0x0;
// tx tx_q_tc_map{q} bitfield definitions
// preprocessor definitions for the bitfield "tx_q_tc_map{q}".
// parameter: queue {q} | bit-level stride | range [0, 31]
// port="pif_tpb_tx_q_tc_map0_i[2:0]"
//
// register address for bitfield tx_q_tc_map{q}

// lower bit position of bitfield tx_q_tc_map{q}

// width of bitfield tx_q_tc_map{q}
pub const HW_ATL2_TX_Q_TC_MAP_WIDTH: c_int = 3;
// default value of bitfield tx_q_tc_map{q}
pub const HW_ATL2_TX_Q_TC_MAP_DEFAULT: c_uint = 0x0;
// tx data_tc_arb_mode bitfield definitions
// preprocessor definitions for the bitfield "data_tc_arb_mode".
// port="pif_tps_data_tc_arb_mode_i"
//
// register address for bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_ADR: c_uint = 0x00007100;
// bitmask for bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_MSK: c_uint = 0x00000003;
// inverted bitmask for bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_MSKN: c_uint = 0xfffffffc;
// lower bit position of bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_SHIFT: c_int = 0;
// width of bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_WIDTH: c_int = 2;
// default value of bitfield data_tc_arb_mode
pub const HW_ATL2_TPS_DATA_TC_ARB_MODE_DEFAULT: c_uint = 0x0;
// tx data_tc{t}_credit_max[f:0] bitfield definitions
// preprocessor definitions for the bitfield "data_tc{t}_credit_max[f:0]".
// parameter: tc {t} | stride size 0x4 | range [0, 7]
// port="pif_tps_data_tc0_credit_max_i[15:0]"
//
// register address for bitfield data_tc{t}_credit_max[f:0]

// bitmask for bitfield data_tc{t}_credit_max[f:0]
pub const HW_ATL2_TPS_DATA_TCTCREDIT_MAX_MSK: c_uint = 0xffff0000;
// inverted bitmask for bitfield data_tc{t}_credit_max[f:0]
pub const HW_ATL2_TPS_DATA_TCTCREDIT_MAX_MSKN: c_uint = 0x0000ffff;
// lower bit position of bitfield data_tc{t}_credit_max[f:0]
pub const HW_ATL2_TPS_DATA_TCTCREDIT_MAX_SHIFT: c_int = 16;
// width of bitfield data_tc{t}_credit_max[f:0]
pub const HW_ATL2_TPS_DATA_TCTCREDIT_MAX_WIDTH: c_int = 16;
// default value of bitfield data_tc{t}_credit_max[f:0]
pub const HW_ATL2_TPS_DATA_TCTCREDIT_MAX_DEFAULT: c_uint = 0x0;
// register address for bitfield pif_tpb_highest_prio_tc_en
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_EN_ADR: c_uint = 0x00007180;
// bitmask for bitfield pif_tpb_highest_prio_tc_en
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_EN_MSK: c_uint = 0x00000100;
// lower bit position of bitfield pif_tpb_highest_prio_tc_en
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_EN_SHIFT: c_int = 8;
// register address for bitfield pif_tpb_highest_prio_tc
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_ADR: c_uint = 0x00007180;
// bitmask for bitfield pif_tpb_highest_prio_tc
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_MSK: c_uint = 0x00000007;
// lower bit position of bitfield pif_tpb_highest_prio_tc
pub const HW_ATL2_TPB_HIGHEST_PRIO_TC_SHIFT: c_int = 0;
// tx data_tc{t}_weight[e:0] bitfield definitions
// preprocessor definitions for the bitfield "data_tc{t}_weight[e:0]".
// parameter: tc {t} | stride size 0x4 | range [0, 7]
// port="pif_tps_data_tc0_weight_i[14:0]"
//
// register address for bitfield data_tc{t}_weight[e:0]

// bitmask for bitfield data_tc{t}_weight[e:0]
pub const HW_ATL2_TPS_DATA_TCTWEIGHT_MSK: c_uint = 0x00007fff;
// inverted bitmask for bitfield data_tc{t}_weight[e:0]
pub const HW_ATL2_TPS_DATA_TCTWEIGHT_MSKN: c_uint = 0xffff8000;
// lower bit position of bitfield data_tc{t}_weight[e:0]
pub const HW_ATL2_TPS_DATA_TCTWEIGHT_SHIFT: c_int = 0;
// width of bitfield data_tc{t}_weight[e:0]
pub const HW_ATL2_TPS_DATA_TCTWEIGHT_WIDTH: c_int = 15;
// default value of bitfield data_tc{t}_weight[e:0]
pub const HW_ATL2_TPS_DATA_TCTWEIGHT_DEFAULT: c_uint = 0x0;
// tx interrupt moderation control register definitions
// Preprocessor definitions for TX Interrupt Moderation Control Register
// Base Address: 0x00007c28
// Parameter: queue {Q} | stride size 0x4 | range [0, 31]
//

// TX tx_data_rd_req_limit[7:0] Bitfield Definitions
//
pub const HW_ATL2_TDM_TX_DATA_RD_REQ_LIMIT_ADR: c_uint = 0x00007B04;
pub const HW_ATL2_TDM_TX_DATA_RD_REQ_LIMIT_MSK: c_uint = 0x0000FF00;
pub const HW_ATL2_TDM_TX_DATA_RD_REQ_LIMIT_SHIFT: c_int = 8;
// TX tx_desc_rd_req_limit[4:0] Bitfield Definitions
//
pub const HW_ATL2_TDM_TX_DESC_RD_REQ_LIMIT_ADR: c_uint = 0x00007B04;
pub const HW_ATL2_TDM_TX_DESC_RD_REQ_LIMIT_MSK: c_uint = 0x0000001F;
pub const HW_ATL2_TDM_TX_DESC_RD_REQ_LIMIT_SHIFT: c_int = 0;
// register address for bitfield uP Force Interrupt
pub const HW_ATL2_GLB_CONTROL_2_ADR: c_uint = 0x00000404;
pub const HW_ATL2_MIF_INTERRUPT_2_TO_ITR_MSK: c_uint = 0x00000100;
// lower bit position of bitfield MIF Interrupt to ITR
pub const HW_ATL2_MIF_INTERRUPT_TO_ITR_SHIFT: c_int = 6;
pub const HW_ATL2_EN_INTERRUPT_MIF2_TO_ITR_MSK: c_uint = 0x00001000;
// lower bit position of bitfield Enable MIF Interrupt to ITR
pub const HW_ATL2_EN_INTERRUPT_TO_ITR_SHIFT: c_uint = 0xA;
pub const HW_ATL2_GLOBAL_INTERNAL_ALARMS_1_ADR: c_uint = 0x00000924;
pub const HW_ATL2_GLOBAL_HIGH_PRIO_INTERRUPT_1_MASK_ADR: c_uint = 0x00000964;
// bitmask for bitfield TSG PTM GPIO interrupt
pub const HW_ATL2_TSG_TSG1_GPIO_INTERRUPT_MSK: c_uint = 0x00000200;
// lower bit position of bitfield TSG PTM GPIO interrupt
pub const HW_ATL2_TSG_TSG1_GPIO_INTERRUPT_SHIFT: c_int = 9;
// bitmask for bitfield TSG0 GPIO interrupt
pub const HW_ATL2_TSG_TSG0_GPIO_INTERRUPT_MSK: c_uint = 0x00000020;
// lower bit position of bitfield TSG0 GPIO interrupt
pub const HW_ATL2_TSG_TSG0_GPIO_INTERRUPT_SHIFT: c_int = 5;
// TSG registers

pub const HW_ATL2_CLK0_CLOCK_CFG_ADR: c_uint = 0x00000CA0u;
pub const HW_ATL2_CLK1_CLOCK_CFG_ADR: c_uint = 0x00000D50u;
pub const HW_ATL2_TSG_SYNC_RESET_MSK: c_uint = 0x00000001;
pub const HW_ATL2_TSG_SYNC_RESET_SHIFT: c_uint = 0x00000000;
pub const HW_ATL2_TSG_CLOCK_EN_MSK: c_uint = 0x00000002;
pub const HW_ATL2_TSG_CLOCK_EN_SHIFT: c_uint = 0x00000001;
pub const HW_ATL2_CLK0_CLOCK_MODIF_CTRL_ADR: c_uint = 0x00000CA4u;
pub const HW_ATL2_CLK1_CLOCK_MODIF_CTRL_ADR: c_uint = 0x00000D54u;
pub const HW_ATL2_TSG_SUBTRACT_COUNTER_MSK: c_uint = 0x00000002;
pub const HW_ATL2_TSG_ADD_COUNTER_MSK: c_uint = 0x00000004;
pub const HW_ATL2_TSG_LOAD_INC_CFG_MSK: c_uint = 0x00000008;
pub const HW_ATL2_CLK0_CLOCK_MODIF_VAL_LSW_ADR: c_uint = 0x00000CA8u;
pub const HW_ATL2_CLK1_CLOCK_MODIF_VAL_LSW_ADR: c_uint = 0x00000D58u;
pub const HW_ATL2_CLK0_CLOCK_INC_CFG_ADR: c_uint = 0x00000CB0u;
pub const HW_ATL2_CLK1_CLOCK_INC_CFG_ADR: c_uint = 0x00000D60u;
pub const HW_ATL2_CLK0_READ_CUR_NS_LSW_ADR: c_uint = 0x00000CB8u;
pub const HW_ATL2_CLK1_READ_CUR_NS_LSW_ADR: c_uint = 0x00000D68u;
pub const HW_ATL2_CLK0_GPIO_CFG_ADR: c_uint = 0x00000CC4u;
pub const HW_ATL2_CLK1_GPIO_CFG_ADR: c_uint = 0x00000D74u;
pub const HW_ATL2_TSG_GPIO_IN_MONITOR_EN_SHIFT: c_uint = 0x00000000;
pub const HW_ATL2_TSG_GPIO_IN_MONITOR_EN_MSK: c_uint = 0x00000001;
pub const HW_ATL2_TSG_GPIO_IN_MODE_SHIFT: c_uint = 0x00000001;
pub const HW_ATL2_TSG_GPIO_IN_MODE_MSK: c_uint = 0x00000006;
pub const HW_ATL2_TSG_GPIO_IN_MODE_POSEDGE: c_uint = 0x00000000;
pub const HW_ATL2_CLK0_EXT_CLK_COUNT_ADR: c_uint = 0x00000CCCu;
pub const HW_ATL2_CLK1_EXT_CLK_COUNT_ADR: c_uint = 0x00000D7Cu;
pub const HW_ATL2_CLK0_GPIO_EVENT_TS_LSW_ADR: c_uint = 0x00000CD0u;
pub const HW_ATL2_CLK1_GPIO_EVENT_TS_LSW_ADR: c_uint = 0x00000D80u;
pub const HW_ATL2_CLK0_GPIO_EVENT_GEN_TS_LSW_ADR: c_uint = 0x00000CE0u;
pub const HW_ATL2_CLK1_GPIO_EVENT_GEN_TS_LSW_ADR: c_uint = 0x00000D90u;
pub const HW_ATL2_CLK0_GPIO_EVENT_GEN_CFG_ADR: c_uint = 0x00000CE8u;
pub const HW_ATL2_CLK1_GPIO_EVENT_GEN_CFG_ADR: c_uint = 0x00000D98u;
pub const HW_ATL2_TSG_GPIO_OUTPUT_EN_SHIFT: c_uint = 0x00000000;
pub const HW_ATL2_TSG_GPIO_OUTPUT_EN_MSK: c_uint = 0x00000001;
pub const HW_ATL2_TSG_GPIO_EVENT_MODE_SHIFT: c_uint = 0x00000001;
pub const HW_ATL2_TSG_GPIO_EVENT_MODE_MSK: c_uint = 0x00000006;
pub const HW_ATL2_TSG_GPIO_EVENT_MODE_SET_ON_TIME: c_uint = 0x00000003;
pub const HW_ATL2_TSG_GPIO_GEN_OUTPUT_EN_MSK: c_uint = 0x00000008;
pub const HW_ATL2_CLK0_GPIO_EVENT_HIGH_TIME_LSW_ADR: c_uint = 0x00000CF0u;
pub const HW_ATL2_CLK1_GPIO_EVENT_HIGH_TIME_LSW_ADR: c_uint = 0x00000DA0u;
pub const HW_ATL2_CLK0_GPIO_EVENT_LOW_TIME_LSW_ADR: c_uint = 0x00000CF8u;
pub const HW_ATL2_CLK1_GPIO_EVENT_LOW_TIME_LSW_ADR: c_uint = 0x00000DA8u;
// PCIE Extended tag enable Bitfield Definitions
//
pub const HW_ATL2_PHI_EXT_TAG_EN_ADR: c_uint = 0x00001000;
pub const HW_ATL2_PHI_EXT_TAG_EN_MSK: c_uint = 0x00000020;
pub const HW_ATL2_PHI_EXT_TAG_EN_SHIFT: c_int = 5;
// Launch time control register
pub const HW_ATL2_LT_CTRL_ADR: c_uint = 0x00007a1c;
pub const HW_ATL2_LT_CTRL_AVB_LEN_CMP_TRSHLD_MSK: c_uint = 0xFFFF0000;
pub const HW_ATL2_LT_CTRL_AVB_LEN_CMP_TRSHLD_SHIFT: c_int = 16;
pub const HW_ATL2_LT_CTRL_CLK_RATIO_MSK: c_uint = 0x0000FF00;
pub const HW_ATL2_LT_CTRL_CLK_RATIO_SHIFT: c_int = 8;
pub const HW_ATL2_LT_CTRL_CLK_RATIO_QUATER_SPEED: c_int = 4;
pub const HW_ATL2_LT_CTRL_CLK_RATIO_HALF_SPEED: c_int = 2;
pub const HW_ATL2_LT_CTRL_CLK_RATIO_FULL_SPEED: c_int = 1;
pub const HW_ATL2_LT_CTRL_25G_MODE_SUPPORT_MSK: c_uint = 0x00000008;
pub const HW_ATL2_LT_CTRL_25G_MODE_SUPPORT_SHIFT: c_int = 3;
pub const HW_ATL2_LT_CTRL_LINK_SPEED_MSK: c_uint = 0x00000007;
pub const HW_ATL2_LT_CTRL_LINK_SPEED_SHIFT: c_int = 0;
// FPGA VER register
pub const HW_ATL2_FPGA_VER_ADR: c_uint = 0x000000f4;

// ahb_mem_addr{f}[31:0] Bitfield Definitions
// Preprocessor definitions for the bitfield "ahb_mem_addr{f}[31:0]".
// Parameter: filter {f} | stride size 0x10 | range [0, 127]
// PORT="ahb_mem_addr{f}[31:0]"
//
// Register address for bitfield ahb_mem_addr{f}[31:0]

// Bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_REQ_TAG_MSK: c_uint = 0xFFFFFFFFu;
// Inverted bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_REQ_TAG_MSKN: c_uint = 0x00000000u;
// Lower bit position of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_REQ_TAG_SHIFT: c_int = 0;
// Width of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_REQ_TAG_WIDTH: c_int = 31;
// Default value of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_REQ_TAG_DEFAULT: c_uint = 0x0;
// Register address for bitfield ahb_mem_addr{f}[31:0]

// Bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_TAG_MASK_MSK: c_uint = 0xFFFFFFFFu;
// Inverted bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_TAG_MASK_MSKN: c_uint = 0x00000000u;
// Lower bit position of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_TAG_MASK_SHIFT: c_int = 0;
// Width of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_TAG_MASK_WIDTH: c_int = 31;
// Default value of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_TAG_MASK_DEFAULT: c_uint = 0x0;
// Register address for bitfield ahb_mem_addr{f}[31:0]

// Bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_ACTN_MSK: c_uint = 0x000007FFu;
// Inverted bitmask for bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_ACTN_MSKN: c_uint = 0xFFFFF800u;
// Lower bit position of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_ACTN_SHIFT: c_int = 0;
// Width of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_ACTN_WIDTH: c_int = 10;
// Default value of bitfield ahb_mem_addr{f}[31:0]
pub const HW_ATL2_RPF_ACT_RSLVR_ACTN_DEFAULT: c_uint = 0x0;
// rpf_rec_tab_en[15:0] Bitfield Definitions
// Preprocessor definitions for the bitfield "rpf_rec_tab_en[15:0]".
// PORT="pif_rpf_rec_tab_en[15:0]"
//
// Register address for bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_ADR: c_uint = 0x00006ff0u;
// Bitmask for bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_MSK: c_uint = 0x0000FFFFu;
// Inverted bitmask for bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_MSKN: c_uint = 0xFFFF0000u;
// Lower bit position of bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_SHIFT: c_int = 0;
// Width of bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_WIDTH: c_int = 16;
// Default value of bitfield rpf_rec_tab_en[15:0]
pub const HW_ATL2_RPF_REC_TAB_EN_DEFAULT: c_uint = 0x0;
// Register address for firmware shared input buffer

// Register address for firmware shared output buffer

// pif_host_finished_buf_wr_i Bitfield Definitions
// Preprocessor definitions for the bitfield "pif_host_finished_buf_wr_i".
// PORT="pif_host_finished_buf_wr_i"
//
// Register address for bitfield rpif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_ADR: c_uint = 0x00000e00u;
// Bitmask for bitfield pif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_MSK: c_uint = 0x00000001u;
// Inverted bitmask for bitfield pif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_MSKN: c_uint = 0xFFFFFFFEu;
// Lower bit position of bitfield pif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_SHIFT: c_int = 0;
// Width of bitfield pif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_WIDTH: c_int = 1;
// Default value of bitfield pif_host_finished_buf_wr_i
pub const HW_ATL2_MIF_HOST_FINISHED_WRITE_DEFAULT: c_uint = 0x0;
// pif_mcp_finished_buf_rd_i Bitfield Definitions
// Preprocessor definitions for the bitfield "pif_mcp_finished_buf_rd_i".
// PORT="pif_mcp_finished_buf_rd_i"
//
// Register address for bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_ADR: c_uint = 0x00000e04u;
// Bitmask for bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_MSK: c_uint = 0x00000001u;
// Inverted bitmask for bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_MSKN: c_uint = 0xFFFFFFFEu;
// Lower bit position of bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_SHIFT: c_int = 0;
// Width of bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_WIDTH: c_int = 1;
// Default value of bitfield pif_mcp_finished_buf_rd_i
pub const HW_ATL2_MIF_MCP_FINISHED_READ_DEFAULT: c_uint = 0x0;
// Register address for bitfield pif_mcp_boot_reg
pub const HW_ATL2_MIF_BOOT_REG_ADR: c_uint = 0x00003040u;

pub const HW_ATL2_MCP_HOST_REQ_INT_ADR: c_uint = 0x00000F00u;
pub const HW_ATL2_MCP_HOST_REQ_INT_SET_ADR: c_uint = 0x00000F04u;
pub const HW_ATL2_MCP_HOST_REQ_INT_CLR_ADR: c_uint = 0x00000F08u;
// Register address for bitfield PTP EXT GPIO TS SEL
pub const HW_ATL2_TSG0_EXT_GPIO_TS_INPUT_SEL_ADR: c_uint = 0x00003664;
// Bitmask for bitfield PTP EXT GPIO TS SEL
pub const HW_ATL2_TSG0_EXT_GPIO_TS_INPUT_SEL_MSK: c_uint = 0x00001F00;
// Lower bit position of bitfield PTP EXT GPIO TS SEL
pub const HW_ATL2_TSG0_EXT_GPIO_TS_INPUT_SEL_SHIFT: c_int = 8;
// Register address for bitfield TSG EXT GPIO TS SEL
pub const HW_ATL2_TSG1_EXT_GPIO_TS_INPUT_SEL_ADR: c_uint = 0x00003660;
// Bitmask for bitfield TSG EXT GPIO TS SEL
pub const HW_ATL2_TSG1_EXT_GPIO_TS_INPUT_SEL_MSK: c_uint = 0x00001F00;
// Lower bit position of bitfield TSG EXT GPIO TS SEL
pub const HW_ATL2_TSG1_EXT_GPIO_TS_INPUT_SEL_SHIFT: c_int = 8;
// Register address for bitfield GPIO{P} Special Mode

// Bitmask for bitfield GPIO{P} Special Mode
pub const HW_ATL2_GPIO_PIN_SPEC_MODE_MSK: c_uint = 0x0000000C;
// Lower bit position of bitfield GPIO{P} Special Mode
pub const HW_ATL2_GPIO_PIN_SPEC_MODE_SHIFT: c_int = 2;
pub const HW_ATL2_GPIO_PIN_SPEC_MODE_TSG1_EVENT_OUTPUT: c_int = 0;
pub const HW_ATL2_GPIO_PIN_SPEC_MODE_TSG0_EVENT_OUTPUT: c_int = 2;
pub const HW_ATL2_GPIO_PIN_SPEC_MODE_GPIO: c_int = 3;
