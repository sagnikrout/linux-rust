//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/microchip/ksz9477_reg.h
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
//
// Microchip KSZ9477 register definitions
//
// Copyright (C) 2017-2025 Microchip Technology Inc.
//
pub const KS_PRIO_M: c_uint = 0x7;
pub const KS_PRIO_S: c_int = 4;
// 0 - Operation
pub const REG_CHIP_ID0__1: c_uint = 0x0000;
pub const REG_CHIP_ID1__1: c_uint = 0x0001;
pub const FAMILY_ID: c_uint = 0x95;
pub const FAMILY_ID_94: c_uint = 0x94;
pub const FAMILY_ID_95: c_uint = 0x95;
pub const FAMILY_ID_85: c_uint = 0x85;
pub const FAMILY_ID_98: c_uint = 0x98;
pub const FAMILY_ID_88: c_uint = 0x88;
pub const REG_CHIP_ID2__1: c_uint = 0x0002;
pub const CHIP_ID_66: c_uint = 0x66;
pub const CHIP_ID_67: c_uint = 0x67;
pub const CHIP_ID_77: c_uint = 0x77;
pub const CHIP_ID_93: c_uint = 0x93;
pub const CHIP_ID_96: c_uint = 0x96;
pub const CHIP_ID_97: c_uint = 0x97;
pub const REG_CHIP_ID3__1: c_uint = 0x0003;
pub const SWITCH_REVISION_M: c_uint = 0x0F;
pub const SWITCH_REVISION_S: c_int = 4;
pub const SWITCH_RESET: c_uint = 0x01;
pub const REG_GLOBAL_OPTIONS: c_uint = 0x000F;

pub const SW_9567_RL_5_2: c_uint = 0xC;
pub const SW_9477_SL_5_2: c_uint = 0xD;
pub const SW_9896_GL_5_1: c_uint = 0xB;
pub const SW_9896_RL_5_1: c_uint = 0x8;
pub const SW_9896_SL_5_1: c_uint = 0x9;
pub const SW_9895_GL_4_1: c_uint = 0x7;
pub const SW_9895_RL_4_1: c_uint = 0x4;
pub const SW_9895_SL_4_1: c_uint = 0x5;
pub const SW_9896_RL_4_2: c_uint = 0x6;
pub const SW_9893_RL_2_1: c_uint = 0x0;
pub const SW_9893_SL_2_1: c_uint = 0x1;
pub const SW_9893_GL_2_1: c_uint = 0x3;

pub const SW_9893_RN_2_1: c_uint = 0xC;
pub const REG_SW_INT_STATUS__4: c_uint = 0x0010;
pub const REG_SW_INT_MASK__4: c_uint = 0x0014;

pub const REG_SW_PORT_INT_STATUS__4: c_uint = 0x0018;
pub const REG_SW_PORT_INT_MASK__4: c_uint = 0x001C;
pub const REG_SW_PHY_INT_STATUS: c_uint = 0x0020;
pub const REG_SW_PHY_INT_ENABLE: c_uint = 0x0024;
// 1 - Global
pub const REG_SW_GLOBAL_SERIAL_CTRL_0: c_uint = 0x0100;

pub const REG_SW_GLOBAL_OUTPUT_CTRL__1: c_uint = 0x0103;

pub const REG_SW_IBA__4: c_uint = 0x0104;

pub const SW_IBA_QID_M: c_uint = 0xF;
pub const SW_IBA_QID_S: c_int = 22;
pub const SW_IBA_PORT_M: c_uint = 0x2F;
pub const SW_IBA_PORT_S: c_int = 16;
pub const SW_IBA_FRAME_TPID_M: c_uint = 0xFFFF;
pub const REG_SW_APB_TIMEOUT_ADDR__4: c_uint = 0x0108;

pub const REG_SW_IBA_SYNC__1: c_uint = 0x010C;
pub const REG_SW_IBA_STATUS__4: c_uint = 0x0110;

pub const REG_SW_IBA_STATES__4: c_uint = 0x0114;
pub const SW_IBA_BUF_STATE_S: c_int = 30;
pub const SW_IBA_CMD_STATE_S: c_int = 28;
pub const SW_IBA_RESP_STATE_S: c_int = 26;
pub const SW_IBA_STATE_M: c_uint = 0x3;
pub const SW_IBA_PACKET_SIZE_M: c_uint = 0x7F;
pub const SW_IBA_PACKET_SIZE_S: c_int = 16;
pub const SW_IBA_FMT_ID_M: c_uint = 0xFFFF;
pub const REG_SW_IBA_RESULT__4: c_uint = 0x0118;
pub const SW_IBA_SIZE_S: c_int = 24;

// 2 - PHY
pub const REG_SW_POWER_MANAGEMENT_CTRL: c_uint = 0x0201;

pub const SW_POWER_DOWN_MODE: c_uint = 0x3;
pub const SW_ENERGY_DETECTION: c_int = 1;
pub const SW_SOFT_POWER_DOWN: c_int = 2;
pub const SW_POWER_SAVING: c_int = 3;
// 3 - Operation Control
pub const REG_SW_OPERATION: c_uint = 0x0300;

pub const REG_SW_MTU__2: c_uint = 0x0308;

pub const REG_SW_ISP_TPID__2: c_uint = 0x030A;
pub const REG_SW_HSR_TPID__2: c_uint = 0x030C;
pub const REG_AVB_STRATEGY__2: c_uint = 0x030E;

pub const REG_SW_LUE_CTRL_0: c_uint = 0x0310;

pub const SW_HASH_OPTION_M: c_uint = 0x03;
pub const SW_HASH_OPTION_CRC: c_int = 1;
pub const SW_HASH_OPTION_XOR: c_int = 2;
pub const SW_HASH_OPTION_DIRECT: c_int = 3;
pub const REG_SW_LUE_CTRL_1: c_uint = 0x0311;

pub const REG_SW_LUE_CTRL_2: c_uint = 0x0312;

pub const SW_FLUSH_OPTION_M: c_uint = 0x3;
pub const SW_FLUSH_OPTION_S: c_int = 2;
pub const SW_FLUSH_OPTION_DYN_MAC: c_int = 1;
pub const SW_FLUSH_OPTION_STA_MAC: c_int = 2;
pub const SW_FLUSH_OPTION_BOTH: c_int = 3;
pub const SW_PRIO_M: c_uint = 0x3;
pub const SW_PRIO_DA: c_int = 0;
pub const SW_PRIO_SA: c_int = 1;
pub const SW_PRIO_HIGHEST_DA_SA: c_int = 2;
pub const SW_PRIO_LOWEST_DA_SA: c_int = 3;
pub const REG_SW_LUE_CTRL_3: c_uint = 0x0313;

pub const REG_SW_LUE_INT_STATUS: c_uint = 0x0314;
pub const REG_SW_LUE_INT_ENABLE: c_uint = 0x0315;

pub const REG_SW_LUE_INDEX_0__2: c_uint = 0x0316;
pub const ENTRY_INDEX_M: c_uint = 0x0FFF;
pub const REG_SW_LUE_INDEX_1__2: c_uint = 0x0318;
pub const FAIL_INDEX_M: c_uint = 0x03FF;
pub const REG_SW_LUE_INDEX_2__2: c_uint = 0x031A;
pub const REG_SW_LUE_UNK_UCAST_CTRL__4: c_uint = 0x0320;

pub const REG_SW_LUE_UNK_MCAST_CTRL__4: c_uint = 0x0324;

pub const REG_SW_LUE_UNK_VID_CTRL__4: c_uint = 0x0328;

pub const REG_SW_MAC_CTRL_0: c_uint = 0x0330;

pub const REG_SW_MAC_CTRL_1: c_uint = 0x0331;

pub const SW_BACK_PRESSURE_COLLISION: c_int = 0;

pub const REG_SW_MAC_CTRL_2: c_uint = 0x0332;

pub const REG_SW_MAC_CTRL_3: c_uint = 0x0333;
pub const REG_SW_MAC_CTRL_4: c_uint = 0x0334;

pub const REG_SW_MAC_CTRL_5: c_uint = 0x0335;

pub const REG_SW_MAC_CTRL_6: c_uint = 0x0336;

pub const REG_SW_MAC_802_1P_MAP_0: c_uint = 0x0338;
pub const REG_SW_MAC_802_1P_MAP_1: c_uint = 0x0339;
pub const REG_SW_MAC_802_1P_MAP_2: c_uint = 0x033A;
pub const REG_SW_MAC_802_1P_MAP_3: c_uint = 0x033B;

pub const REG_SW_MAC_ISP_CTRL: c_uint = 0x033C;
pub const REG_SW_MAC_TOS_CTRL: c_uint = 0x033E;

pub const REG_SW_MAC_TOS_PRIO_0: c_uint = 0x0340;
pub const REG_SW_MAC_TOS_PRIO_1: c_uint = 0x0341;
pub const REG_SW_MAC_TOS_PRIO_2: c_uint = 0x0342;
pub const REG_SW_MAC_TOS_PRIO_3: c_uint = 0x0343;
pub const REG_SW_MAC_TOS_PRIO_4: c_uint = 0x0344;
pub const REG_SW_MAC_TOS_PRIO_5: c_uint = 0x0345;
pub const REG_SW_MAC_TOS_PRIO_6: c_uint = 0x0346;
pub const REG_SW_MAC_TOS_PRIO_7: c_uint = 0x0347;
pub const REG_SW_MAC_TOS_PRIO_8: c_uint = 0x0348;
pub const REG_SW_MAC_TOS_PRIO_9: c_uint = 0x0349;
pub const REG_SW_MAC_TOS_PRIO_10: c_uint = 0x034A;
pub const REG_SW_MAC_TOS_PRIO_11: c_uint = 0x034B;
pub const REG_SW_MAC_TOS_PRIO_12: c_uint = 0x034C;
pub const REG_SW_MAC_TOS_PRIO_13: c_uint = 0x034D;
pub const REG_SW_MAC_TOS_PRIO_14: c_uint = 0x034E;
pub const REG_SW_MAC_TOS_PRIO_15: c_uint = 0x034F;
pub const REG_SW_MAC_TOS_PRIO_16: c_uint = 0x0350;
pub const REG_SW_MAC_TOS_PRIO_17: c_uint = 0x0351;
pub const REG_SW_MAC_TOS_PRIO_18: c_uint = 0x0352;
pub const REG_SW_MAC_TOS_PRIO_19: c_uint = 0x0353;
pub const REG_SW_MAC_TOS_PRIO_20: c_uint = 0x0354;
pub const REG_SW_MAC_TOS_PRIO_21: c_uint = 0x0355;
pub const REG_SW_MAC_TOS_PRIO_22: c_uint = 0x0356;
pub const REG_SW_MAC_TOS_PRIO_23: c_uint = 0x0357;
pub const REG_SW_MAC_TOS_PRIO_24: c_uint = 0x0358;
pub const REG_SW_MAC_TOS_PRIO_25: c_uint = 0x0359;
pub const REG_SW_MAC_TOS_PRIO_26: c_uint = 0x035A;
pub const REG_SW_MAC_TOS_PRIO_27: c_uint = 0x035B;
pub const REG_SW_MAC_TOS_PRIO_28: c_uint = 0x035C;
pub const REG_SW_MAC_TOS_PRIO_29: c_uint = 0x035D;
pub const REG_SW_MAC_TOS_PRIO_30: c_uint = 0x035E;
pub const REG_SW_MAC_TOS_PRIO_31: c_uint = 0x035F;
pub const REG_SW_MRI_CTRL_0: c_uint = 0x0370;

pub const REG_SW_CLASS_D_IP_CTRL__4: c_uint = 0x0374;

pub const REG_SW_MRI_CTRL_8: c_uint = 0x0378;
pub const SW_NO_COLOR_S: c_int = 6;
pub const SW_RED_COLOR_S: c_int = 4;
pub const SW_YELLOW_COLOR_S: c_int = 2;
pub const SW_GREEN_COLOR_S: c_int = 0;
pub const SW_COLOR_M: c_uint = 0x3;
pub const REG_SW_QM_CTRL__4: c_uint = 0x0390;

pub const PRIO_SCHEME_SELECT_S: c_int = 6;
pub const PRIO_MAP_3_HI: c_int = 0;
pub const PRIO_MAP_2_HI: c_int = 2;
pub const PRIO_MAP_0_LO: c_int = 3;

pub const REG_SW_EEE_QM_CTRL__2: c_uint = 0x03C0;
pub const REG_SW_EEE_TXQ_WAIT_TIME__2: c_uint = 0x03C2;
// 4 -
pub const REG_SW_VLAN_ENTRY__4: c_uint = 0x0400;

pub const VLAN_PRIO_S: c_int = 24;
pub const VLAN_MSTP_M: c_uint = 0x7;
pub const VLAN_MSTP_S: c_int = 12;
pub const VLAN_FID_M: c_uint = 0x7F;
pub const REG_SW_VLAN_ENTRY_UNTAG__4: c_uint = 0x0404;
pub const REG_SW_VLAN_ENTRY_PORTS__4: c_uint = 0x0408;
pub const REG_SW_VLAN_ENTRY_INDEX__2: c_uint = 0x040C;
pub const VLAN_INDEX_M: c_uint = 0x0FFF;
pub const REG_SW_VLAN_CTRL: c_uint = 0x040E;

pub const VLAN_ACTION: c_uint = 0x3;
pub const VLAN_WRITE: c_int = 1;
pub const VLAN_READ: c_int = 2;
pub const VLAN_CLEAR: c_int = 3;
pub const REG_SW_ALU_INDEX_0: c_uint = 0x0410;
pub const ALU_FID_INDEX_S: c_int = 16;
pub const ALU_MAC_ADDR_HI: c_uint = 0xFFFF;
pub const REG_SW_ALU_INDEX_1: c_uint = 0x0414;

pub const REG_SW_ALU_CTRL__4: c_uint = 0x0418;

pub const ALU_VALID_CNT_S: c_int = 16;

pub const ALU_ACTION: c_uint = 0x3;
pub const ALU_WRITE: c_int = 1;
pub const ALU_READ: c_int = 2;
pub const ALU_SEARCH: c_int = 3;
pub const REG_SW_ALU_STAT_CTRL__4: c_uint = 0x041C;

pub const REG_SW_ALU_VAL_A: c_uint = 0x0420;

pub const ALU_V_PRIO_AGE_CNT_S: c_int = 26;
pub const ALU_V_MSTP_M: c_uint = 0x7;
pub const REG_SW_ALU_VAL_B: c_uint = 0x0424;

pub const REG_SW_ALU_VAL_C: c_uint = 0x0428;

pub const ALU_V_FID_S: c_int = 16;
pub const ALU_V_MAC_ADDR_HI: c_uint = 0xFFFF;
pub const REG_SW_ALU_VAL_D: c_uint = 0x042C;
pub const REG_HSR_ALU_INDEX_0: c_uint = 0x0440;
pub const REG_HSR_ALU_INDEX_1: c_uint = 0x0444;
pub const HSR_DST_MAC_INDEX_LO_S: c_int = 16;
pub const HSR_SRC_MAC_INDEX_HI: c_uint = 0xFFFF;
pub const REG_HSR_ALU_INDEX_2: c_uint = 0x0448;

pub const REG_HSR_ALU_INDEX_3: c_uint = 0x044C;

pub const REG_HSR_ALU_CTRL__4: c_uint = 0x0450;

pub const HSR_VALID_CNT_S: c_int = 16;

pub const HSR_ACTION: c_uint = 0x3;
pub const HSR_WRITE: c_int = 1;
pub const HSR_READ: c_int = 2;
pub const HSR_SEARCH: c_int = 3;
pub const REG_HSR_ALU_VAL_A: c_uint = 0x0454;

pub const HSR_V_AGE_CNT_S: c_int = 26;

pub const REG_HSR_ALU_VAL_B: c_uint = 0x0458;
pub const REG_HSR_ALU_VAL_C: c_uint = 0x045C;
pub const HSR_V_DST_MAC_ADDR_LO_S: c_int = 16;
pub const HSR_V_SRC_MAC_ADDR_HI: c_uint = 0xFFFF;
pub const REG_HSR_ALU_VAL_D: c_uint = 0x0460;
pub const REG_HSR_ALU_VAL_E: c_uint = 0x0464;
pub const HSR_V_START_SEQ_1_S: c_int = 16;
pub const HSR_V_START_SEQ_2_S: c_int = 0;
pub const REG_HSR_ALU_VAL_F: c_uint = 0x0468;
pub const HSR_V_EXP_SEQ_1_S: c_int = 16;
pub const HSR_V_EXP_SEQ_2_S: c_int = 0;
pub const REG_HSR_ALU_VAL_G: c_uint = 0x046C;
pub const HSR_V_SEQ_CNT_1_S: c_int = 16;
pub const HSR_V_SEQ_CNT_2_S: c_int = 0;

// 5 - PTP Clock
pub const REG_PTP_CLK_CTRL: c_uint = 0x0500;

pub const REG_PTP_RTC_SUB_NANOSEC__2: c_uint = 0x0502;
pub const PTP_RTC_SUB_NANOSEC_M: c_uint = 0x0007;
pub const REG_PTP_RTC_NANOSEC: c_uint = 0x0504;
pub const REG_PTP_RTC_NANOSEC_H: c_uint = 0x0504;
pub const REG_PTP_RTC_NANOSEC_L: c_uint = 0x0506;
pub const REG_PTP_RTC_SEC: c_uint = 0x0508;
pub const REG_PTP_RTC_SEC_H: c_uint = 0x0508;
pub const REG_PTP_RTC_SEC_L: c_uint = 0x050A;
pub const REG_PTP_SUBNANOSEC_RATE: c_uint = 0x050C;
pub const REG_PTP_SUBNANOSEC_RATE_H: c_uint = 0x050C;

pub const REG_PTP_SUBNANOSEC_RATE_L: c_uint = 0x050E;
pub const REG_PTP_RATE_DURATION: c_uint = 0x0510;
pub const REG_PTP_RATE_DURATION_H: c_uint = 0x0510;
pub const REG_PTP_RATE_DURATION_L: c_uint = 0x0512;
pub const REG_PTP_MSG_CONF1: c_uint = 0x0514;

pub const REG_PTP_MSG_CONF2: c_uint = 0x0516;

pub const REG_PTP_DOMAIN_VERSION: c_uint = 0x0518;
pub const PTP_VERSION_M: c_uint = 0xFF00;
pub const PTP_DOMAIN_M: c_uint = 0x00FF;
pub const REG_PTP_UNIT_INDEX__4: c_uint = 0x0520;
pub const PTP_UNIT_M: c_uint = 0xF;
pub const PTP_GPIO_INDEX_S: c_int = 16;
pub const PTP_TSI_INDEX_S: c_int = 8;
pub const PTP_TOU_INDEX_S: c_int = 0;
pub const REG_PTP_TRIG_STATUS__4: c_uint = 0x0524;
pub const TRIG_ERROR_S: c_int = 16;
pub const TRIG_DONE_S: c_int = 0;
pub const REG_PTP_INT_STATUS__4: c_uint = 0x0528;
pub const TRIG_INT_S: c_int = 16;
pub const TS_INT_S: c_int = 0;
pub const TRIG_UNIT_M: c_uint = 0x7;
pub const TS_UNIT_M: c_uint = 0x3;
pub const REG_PTP_CTRL_STAT__4: c_uint = 0x052C;

pub const REG_TRIG_TARGET_NANOSEC: c_uint = 0x0530;
pub const REG_TRIG_TARGET_SEC: c_uint = 0x0534;
pub const REG_TRIG_CTRL__4: c_uint = 0x0538;

pub const TRIG_CASCADE_UPS_M: c_uint = 0xF;
pub const TRIG_CASCADE_UPS_S: c_int = 26;

pub const TRIG_PATTERN_S: c_int = 20;
pub const TRIG_PATTERN_M: c_uint = 0x7;
pub const TRIG_NEG_EDGE: c_int = 0;
pub const TRIG_POS_EDGE: c_int = 1;
pub const TRIG_NEG_PULSE: c_int = 2;
pub const TRIG_POS_PULSE: c_int = 3;
pub const TRIG_NEG_PERIOD: c_int = 4;
pub const TRIG_POS_PERIOD: c_int = 5;
pub const TRIG_REG_OUTPUT: c_int = 6;
pub const TRIG_GPO_S: c_int = 16;
pub const TRIG_GPO_M: c_uint = 0xF;
pub const TRIG_CASCADE_ITERATE_CNT_M: c_uint = 0xFFFF;
pub const REG_TRIG_CYCLE_WIDTH: c_uint = 0x053C;
pub const REG_TRIG_CYCLE_CNT: c_uint = 0x0540;
pub const TRIG_CYCLE_CNT_M: c_uint = 0xFFFF;
pub const TRIG_CYCLE_CNT_S: c_int = 16;
pub const TRIG_BIT_PATTERN_M: c_uint = 0xFFFF;
pub const REG_TRIG_ITERATE_TIME: c_uint = 0x0544;
pub const REG_TRIG_PULSE_WIDTH__4: c_uint = 0x0548;
pub const TRIG_PULSE_WIDTH_M: c_uint = 0x00FFFFFF;
pub const REG_TS_CTRL_STAT__4: c_uint = 0x0550;
pub const TS_EVENT_DETECT_M: c_uint = 0xF;
pub const TS_EVENT_DETECT_S: c_int = 17;

pub const TS_GPI_M: c_uint = 0xF;
pub const TS_GPI_S: c_int = 8;

pub const TS_DETECT_S: c_int = 6;

pub const TS_CASCADE_UPS_M: c_uint = 0xF;
pub const TS_CASCADE_UPS_S: c_int = 1;

pub const REG_TS_EVENT_0_NANOSEC: c_uint = 0x0554;
pub const REG_TS_EVENT_0_SEC: c_uint = 0x0558;
pub const REG_TS_EVENT_0_SUB_NANOSEC: c_uint = 0x055C;
pub const REG_TS_EVENT_1_NANOSEC: c_uint = 0x0560;
pub const REG_TS_EVENT_1_SEC: c_uint = 0x0564;
pub const REG_TS_EVENT_1_SUB_NANOSEC: c_uint = 0x0568;
pub const REG_TS_EVENT_2_NANOSEC: c_uint = 0x056C;
pub const REG_TS_EVENT_2_SEC: c_uint = 0x0570;
pub const REG_TS_EVENT_2_SUB_NANOSEC: c_uint = 0x0574;
pub const REG_TS_EVENT_3_NANOSEC: c_uint = 0x0578;
pub const REG_TS_EVENT_3_SEC: c_uint = 0x057C;
pub const REG_TS_EVENT_3_SUB_NANOSEC: c_uint = 0x0580;
pub const REG_TS_EVENT_4_NANOSEC: c_uint = 0x0584;
pub const REG_TS_EVENT_4_SEC: c_uint = 0x0588;
pub const REG_TS_EVENT_4_SUB_NANOSEC: c_uint = 0x058C;
pub const REG_TS_EVENT_5_NANOSEC: c_uint = 0x0590;
pub const REG_TS_EVENT_5_SEC: c_uint = 0x0594;
pub const REG_TS_EVENT_5_SUB_NANOSEC: c_uint = 0x0598;
pub const REG_TS_EVENT_6_NANOSEC: c_uint = 0x059C;
pub const REG_TS_EVENT_6_SEC: c_uint = 0x05A0;
pub const REG_TS_EVENT_6_SUB_NANOSEC: c_uint = 0x05A4;
pub const REG_TS_EVENT_7_NANOSEC: c_uint = 0x05A8;
pub const REG_TS_EVENT_7_SEC: c_uint = 0x05AC;
pub const REG_TS_EVENT_7_SUB_NANOSEC: c_uint = 0x05B0;
pub const TS_EVENT_EDGE_M: c_uint = 0x1;
pub const TS_EVENT_EDGE_S: c_int = 30;

pub const TS_EVENT_SUB_NANOSEC_M: c_uint = 0x7;

pub const REG_GLOBAL_RR_INDEX__1: c_uint = 0x0600;
// DLR
pub const REG_DLR_SRC_PORT__4: c_uint = 0x0604;

pub const DLR_SRC_PORT_M: c_uint = 0x3;
pub const DLR_SRC_PORT_BOTH: c_int = 0;
pub const DLR_SRC_PORT_EACH: c_int = 1;
pub const REG_DLR_IP_ADDR__4: c_uint = 0x0608;
pub const REG_DLR_CTRL__1: c_uint = 0x0610;

pub const REG_DLR_STATE__1: c_uint = 0x0611;
pub const DLR_NODE_STATE_M: c_uint = 0x3;
pub const DLR_NODE_STATE_S: c_int = 1;
pub const DLR_NODE_STATE_IDLE: c_int = 0;
pub const DLR_NODE_STATE_FAULT: c_int = 1;
pub const DLR_NODE_STATE_NORMAL: c_int = 2;
pub const DLR_RING_STATE_FAULT: c_int = 0;
pub const DLR_RING_STATE_NORMAL: c_int = 1;
pub const REG_DLR_PRECEDENCE__1: c_uint = 0x0612;
pub const REG_DLR_BEACON_INTERVAL__4: c_uint = 0x0614;
pub const REG_DLR_BEACON_TIMEOUT__4: c_uint = 0x0618;
pub const REG_DLR_TIMEOUT_WINDOW__4: c_uint = 0x061C;

pub const REG_DLR_VLAN_ID__2: c_uint = 0x0620;

pub const REG_DLR_DEST_ADDR_0: c_uint = 0x0622;
pub const REG_DLR_DEST_ADDR_1: c_uint = 0x0623;
pub const REG_DLR_DEST_ADDR_2: c_uint = 0x0624;
pub const REG_DLR_DEST_ADDR_3: c_uint = 0x0625;
pub const REG_DLR_DEST_ADDR_4: c_uint = 0x0626;
pub const REG_DLR_DEST_ADDR_5: c_uint = 0x0627;
pub const REG_DLR_PORT_MAP__4: c_uint = 0x0628;
pub const REG_DLR_CLASS__1: c_uint = 0x062C;
pub const DLR_FRAME_QID_M: c_uint = 0x3;
// HSR
pub const REG_HSR_PORT_MAP__4: c_uint = 0x0640;
pub const REG_HSR_ALU_CTRL_0__1: c_uint = 0x0644;

pub const HSR_AGE_CNT_DEFAULT_M: c_uint = 0x7;
pub const HSR_AGE_CNT_DEFAULT_S: c_int = 3;

pub const HSR_HASH_OPTION_M: c_uint = 0x3;
pub const HSR_HASH_DISABLE: c_int = 0;
pub const HSR_HASH_UPPER_BITS: c_int = 1;
pub const HSR_HASH_LOWER_BITS: c_int = 2;
pub const HSR_HASH_XOR_BOTH_BITS: c_int = 3;
pub const REG_HSR_ALU_CTRL_1__1: c_uint = 0x0645;

pub const REG_HSR_ALU_CTRL_2__2: c_uint = 0x0646;
pub const REG_HSR_ALU_AGE_PERIOD__4: c_uint = 0x0648;
pub const REG_HSR_ALU_INT_STATUS__1: c_uint = 0x064C;
pub const REG_HSR_ALU_INT_MASK__1: c_uint = 0x064D;

pub const REG_HSR_ALU_ENTRY_0__2: c_uint = 0x0650;

pub const REG_HSR_ALU_ENTRY_1__2: c_uint = 0x0652;

pub const REG_HSR_ALU_ENTRY_3__2: c_uint = 0x0654;

// 0 - Operation
pub const REG_PORT_DEFAULT_VID: c_uint = 0x0000;
pub const REG_PORT_CUSTOM_VID: c_uint = 0x0002;
pub const REG_PORT_AVB_SR_1_VID: c_uint = 0x0004;
pub const REG_PORT_AVB_SR_2_VID: c_uint = 0x0006;
pub const REG_PORT_AVB_SR_1_TYPE: c_uint = 0x0008;
pub const REG_PORT_AVB_SR_2_TYPE: c_uint = 0x000A;
pub const REG_PORT_INT_STATUS: c_uint = 0x001B;
pub const REG_PORT_INT_MASK: c_uint = 0x001F;

pub const REG_PORT_CTRL_0: c_uint = 0x0020;

pub const PORT_EIGHT_QUEUE: c_uint = 0x3;
pub const PORT_FOUR_QUEUE: c_uint = 0x2;
pub const PORT_TWO_QUEUE: c_uint = 0x1;
pub const PORT_SINGLE_QUEUE: c_uint = 0x0;
pub const REG_PORT_CTRL_1: c_uint = 0x0021;
pub const PORT_SRP_ENABLE: c_uint = 0x3;
pub const REG_PORT_STATUS_0: c_uint = 0x0030;

pub const REG_PORT_STATUS_1: c_uint = 0x0034;
// 1 - PHY
pub const REG_PORT_PHY_CTRL: c_uint = 0x0100;

pub const REG_PORT_PHY_STATUS: c_uint = 0x0102;

pub const REG_PORT_PHY_ID_HI: c_uint = 0x0104;
pub const REG_PORT_PHY_ID_LO: c_uint = 0x0106;
pub const KSZ9477_ID_HI: c_uint = 0x0022;
pub const KSZ9477_ID_LO: c_uint = 0x1622;
pub const REG_PORT_PHY_AUTO_NEGOTIATION: c_uint = 0x0108;

pub const PORT_AUTO_NEG_SELECTOR: c_uint = 0x001F;
pub const PORT_AUTO_NEG_802_3: c_uint = 0x0001;

pub const REG_PORT_PHY_REMOTE_CAPABILITY: c_uint = 0x010A;

pub const REG_PORT_PHY_1000_CTRL: c_uint = 0x0112;

pub const REG_PORT_PHY_1000_STATUS: c_uint = 0x0114;

pub const PORT_REMOTE_IDLE_CNT_M: c_uint = 0x0F;

pub const REG_PORT_PHY_MMD_SETUP: c_uint = 0x011A;
pub const PORT_MMD_OP_MODE_M: c_uint = 0x3;
pub const PORT_MMD_OP_MODE_S: c_int = 14;
pub const PORT_MMD_OP_INDEX: c_int = 0;
pub const PORT_MMD_OP_DATA_NO_INCR: c_int = 1;
pub const PORT_MMD_OP_DATA_INCR_RW: c_int = 2;
pub const PORT_MMD_OP_DATA_INCR_W: c_int = 3;
pub const PORT_MMD_DEVICE_ID_M: c_uint = 0x1F;

pub const REG_PORT_PHY_MMD_INDEX_DATA: c_uint = 0x011C;
pub const MMD_DEVICE_ID_DSP: c_int = 1;
pub const MMD_DSP_SQI_CHAN_A: c_uint = 0xAC;
pub const MMD_DSP_SQI_CHAN_B: c_uint = 0xAD;
pub const MMD_DSP_SQI_CHAN_C: c_uint = 0xAE;
pub const MMD_DSP_SQI_CHAN_D: c_uint = 0xAF;

pub const DSP_SQI_AVG_ERR: c_uint = 0x7FFF;
pub const MMD_DEVICE_ID_COMMON: c_int = 2;
pub const MMD_DEVICE_ID_EEE_ADV: c_int = 7;
pub const MMD_EEE_ADV: c_uint = 0x3C;

pub const MMD_EEE_LP_ADV: c_uint = 0x3D;
pub const MMD_EEE_MSG_CODE: c_uint = 0x3F;
pub const MMD_DEVICE_ID_AFED: c_uint = 0x1C;
pub const REG_PORT_PHY_EXTENDED_STATUS: c_uint = 0x011E;

pub const REG_PORT_SGMII_ADDR__4: c_uint = 0x0200;

pub const PORT_SGMII_DEVICE_ID_M: c_uint = 0x1F;
pub const PORT_SGMII_DEVICE_ID_S: c_int = 16;

pub const REG_PORT_SGMII_DATA__4: c_uint = 0x0204;

pub const MMD_DEVICE_ID_PMA: c_uint = 0x01;
pub const MMD_DEVICE_ID_PCS: c_uint = 0x03;
pub const MMD_DEVICE_ID_PHY_XS: c_uint = 0x04;
pub const MMD_DEVICE_ID_DTE_XS: c_uint = 0x05;
pub const MMD_DEVICE_ID_AN: c_uint = 0x07;
pub const MMD_DEVICE_ID_VENDOR_CTRL: c_uint = 0x1E;
pub const MMD_DEVICE_ID_VENDOR_MII: c_uint = 0x1F;

pub const MMD_SR_MII_CTRL: c_uint = 0x0000;

pub const MMD_SR_MII_STATUS: c_uint = 0x0001;
pub const MMD_SR_MII_ID_1: c_uint = 0x0002;
pub const MMD_SR_MII_ID_2: c_uint = 0x0003;
pub const MMD_SR_MII_AUTO_NEGOTIATION: c_uint = 0x0004;

pub const SR_MII_AUTO_NEG_REMOTE_FAULT_M: c_uint = 0x3;
pub const SR_MII_AUTO_NEG_REMOTE_FAULT_S: c_int = 12;
pub const SR_MII_AUTO_NEG_NO_ERROR: c_int = 0;
pub const SR_MII_AUTO_NEG_OFFLINE: c_int = 1;
pub const SR_MII_AUTO_NEG_LINK_FAILURE: c_int = 2;
pub const SR_MII_AUTO_NEG_ERROR: c_int = 3;
pub const SR_MII_AUTO_NEG_PAUSE_M: c_uint = 0x3;
pub const SR_MII_AUTO_NEG_PAUSE_S: c_int = 7;
pub const SR_MII_AUTO_NEG_NO_PAUSE: c_int = 0;
pub const SR_MII_AUTO_NEG_ASYM_PAUSE_TX: c_int = 1;
pub const SR_MII_AUTO_NEG_SYM_PAUSE: c_int = 2;
pub const SR_MII_AUTO_NEG_ASYM_PAUSE_RX: c_int = 3;

pub const MMD_SR_MII_REMOTE_CAPABILITY: c_uint = 0x0005;
pub const MMD_SR_MII_AUTO_NEG_EXP: c_uint = 0x0006;
pub const MMD_SR_MII_AUTO_NEG_EXT: c_uint = 0x000F;
pub const MMD_SR_MII_DIGITAL_CTRL_1: c_uint = 0x8000;
pub const MMD_SR_MII_AUTO_NEG_CTRL: c_uint = 0x8001;

pub const SR_MII_PCS_MODE_M: c_uint = 0x3;
pub const SR_MII_PCS_MODE_S: c_int = 1;
pub const SR_MII_PCS_SGMII: c_int = 2;

pub const MMD_SR_MII_AUTO_NEG_STATUS: c_uint = 0x8002;

pub const SR_MII_STAT_M: c_uint = 0x3;
pub const SR_MII_STAT_S: c_int = 2;
pub const SR_MII_STAT_10_MBPS: c_int = 0;
pub const SR_MII_STAT_100_MBPS: c_int = 1;
pub const SR_MII_STAT_1000_MBPS: c_int = 2;

pub const MMD_SR_MII_PHY_CTRL: c_uint = 0x80A0;
pub const SR_MII_PHY_LANE_SEL_M: c_uint = 0xF;
pub const SR_MII_PHY_LANE_SEL_S: c_int = 8;

pub const MMD_SR_MII_PHY_ADDR: c_uint = 0x80A1;

pub const MMD_SR_MII_PHY_DATA: c_uint = 0x80A2;

pub const SR_MII_PHY_JTAG_CHIP_ID_HI: c_uint = 0x000C;
pub const SR_MII_PHY_JTAG_CHIP_ID_LO: c_uint = 0x000D;
pub const REG_PORT_PHY_REMOTE_LB_LED: c_uint = 0x0122;

pub const REG_PORT_PHY_LINK_MD: c_uint = 0x0124;

pub const PORT_CABLE_DIAG_PAIR_M: c_uint = 0x3;
pub const PORT_CABLE_DIAG_PAIR_S: c_int = 12;
pub const PORT_CABLE_DIAG_SELECT_M: c_uint = 0x3;
pub const PORT_CABLE_DIAG_SELECT_S: c_int = 10;
pub const PORT_CABLE_DIAG_RESULT_M: c_uint = 0x3;
pub const PORT_CABLE_DIAG_RESULT_S: c_int = 8;
pub const PORT_CABLE_STAT_NORMAL: c_int = 0;
pub const PORT_CABLE_STAT_OPEN: c_int = 1;
pub const PORT_CABLE_STAT_SHORT: c_int = 2;
pub const PORT_CABLE_STAT_FAILED: c_int = 3;
pub const PORT_CABLE_FAULT_COUNTER: c_uint = 0x00FF;
pub const REG_PORT_PHY_PMA_STATUS: c_uint = 0x0126;

pub const REG_PORT_PHY_DIGITAL_STATUS: c_uint = 0x0128;

pub const REG_PORT_PHY_RXER_COUNTER: c_uint = 0x012A;
pub const REG_PORT_PHY_INT_ENABLE: c_uint = 0x0136;
pub const REG_PORT_PHY_INT_STATUS: c_uint = 0x0137;

pub const REG_PORT_PHY_DIGITAL_DEBUG_1: c_uint = 0x0138;

// Same as PORT_PHY_LOOPBACK

pub const REG_PORT_PHY_DIGITAL_DEBUG_2: c_uint = 0x013A;
pub const REG_PORT_PHY_DIGITAL_DEBUG_3: c_uint = 0x013C;

pub const REG_PORT_PHY_PHY_CTRL: c_uint = 0x013E;

// Same as PORT_PHY_STAT_MASTER

// 3 - xMII

pub const REG_PMAVBC: c_uint = 0x03AC;

pub const PMAVBC_MIN: c_uint = 0x580;
// 4 - MAC
pub const REG_PORT_MAC_CTRL_0: c_uint = 0x0400;

pub const REG_PORT_MAC_CTRL_1: c_uint = 0x0401;

pub const REG_PORT_MAC_CTRL_2: c_uint = 0x0402;

pub const REG_PORT_MAC_IN_RATE_LIMIT: c_uint = 0x0403;
pub const PORT_IN_PORT_BASED_S: c_int = 6;
pub const PORT_RATE_PACKET_BASED_S: c_int = 5;
pub const PORT_IN_FLOW_CTRL_S: c_int = 4;
pub const PORT_COUNT_IFG_S: c_int = 1;
pub const PORT_COUNT_PREAMBLE_S: c_int = 0;

pub const PORT_IN_LIMIT_MODE_M: c_uint = 0x3;
pub const PORT_IN_LIMIT_MODE_S: c_int = 2;
pub const PORT_IN_ALL: c_int = 0;
pub const PORT_IN_UNICAST: c_int = 1;
pub const PORT_IN_MULTICAST: c_int = 2;
pub const PORT_IN_BROADCAST: c_int = 3;

pub const REG_PORT_IN_RATE_0: c_uint = 0x0410;
pub const REG_PORT_IN_RATE_1: c_uint = 0x0411;
pub const REG_PORT_IN_RATE_2: c_uint = 0x0412;
pub const REG_PORT_IN_RATE_3: c_uint = 0x0413;
pub const REG_PORT_IN_RATE_4: c_uint = 0x0414;
pub const REG_PORT_IN_RATE_5: c_uint = 0x0415;
pub const REG_PORT_IN_RATE_6: c_uint = 0x0416;
pub const REG_PORT_IN_RATE_7: c_uint = 0x0417;
pub const REG_PORT_OUT_RATE_0: c_uint = 0x0420;
pub const REG_PORT_OUT_RATE_1: c_uint = 0x0421;
pub const REG_PORT_OUT_RATE_2: c_uint = 0x0422;
pub const REG_PORT_OUT_RATE_3: c_uint = 0x0423;

// 5 - MIB Counters
pub const REG_PORT_MIB_CTRL_STAT__4: c_uint = 0x0500;

pub const MIB_COUNTER_INDEX_S: c_int = 16;
pub const MIB_COUNTER_DATA_HI_M: c_uint = 0xF;
pub const REG_PORT_MIB_DATA: c_uint = 0x0504;
// 6 - ACL
pub const REG_PORT_ACL_0: c_uint = 0x0600;
pub const ACL_FIRST_RULE_M: c_uint = 0xF;
pub const REG_PORT_ACL_1: c_uint = 0x0601;
pub const ACL_MODE_M: c_uint = 0x3;
pub const ACL_MODE_S: c_int = 4;
pub const ACL_MODE_DISABLE: c_int = 0;
pub const ACL_MODE_LAYER_2: c_int = 1;
pub const ACL_MODE_LAYER_3: c_int = 2;
pub const ACL_MODE_LAYER_4: c_int = 3;
pub const ACL_ENABLE_M: c_uint = 0x3;
pub const ACL_ENABLE_S: c_int = 2;
pub const ACL_ENABLE_2_COUNT: c_int = 0;
pub const ACL_ENABLE_2_TYPE: c_int = 1;
pub const ACL_ENABLE_2_MAC: c_int = 2;
pub const ACL_ENABLE_2_BOTH: c_int = 3;
pub const ACL_ENABLE_3_IP: c_int = 1;
pub const ACL_ENABLE_3_SRC_DST_COMP: c_int = 2;
pub const ACL_ENABLE_4_PROTOCOL: c_int = 0;
pub const ACL_ENABLE_4_TCP_PORT_COMP: c_int = 1;
pub const ACL_ENABLE_4_UDP_PORT_COMP: c_int = 2;
pub const ACL_ENABLE_4_TCP_SEQN_COMP: c_int = 3;

pub const REG_PORT_ACL_2: c_uint = 0x0602;
pub const REG_PORT_ACL_3: c_uint = 0x0603;
pub const ACL_MAX_PORT: c_uint = 0xFFFF;
pub const REG_PORT_ACL_4: c_uint = 0x0604;
pub const REG_PORT_ACL_5: c_uint = 0x0605;
pub const ACL_MIN_PORT: c_uint = 0xFFFF;
pub const ACL_IP_ADDR: c_uint = 0xFFFFFFFF;
pub const ACL_TCP_SEQNUM: c_uint = 0xFFFFFFFF;
pub const REG_PORT_ACL_6: c_uint = 0x0606;
pub const ACL_RESERVED: c_uint = 0xF8;
pub const ACL_PORT_MODE_M: c_uint = 0x3;
pub const ACL_PORT_MODE_S: c_int = 1;
pub const ACL_PORT_MODE_DISABLE: c_int = 0;
pub const ACL_PORT_MODE_EITHER: c_int = 1;
pub const ACL_PORT_MODE_IN_RANGE: c_int = 2;
pub const ACL_PORT_MODE_OUT_OF_RANGE: c_int = 3;
pub const REG_PORT_ACL_7: c_uint = 0x0607;

pub const REG_PORT_ACL_8: c_uint = 0x0608;
pub const ACL_TCP_FLAG_M: c_uint = 0xFF;
pub const REG_PORT_ACL_9: c_uint = 0x0609;
pub const ACL_TCP_FLAG: c_uint = 0xFF;
pub const ACL_ETH_TYPE: c_uint = 0xFFFF;
pub const ACL_IP_M: c_uint = 0xFFFFFFFF;
pub const REG_PORT_ACL_A: c_uint = 0x060A;
pub const ACL_PRIO_MODE_M: c_uint = 0x3;
pub const ACL_PRIO_MODE_S: c_int = 6;
pub const ACL_PRIO_MODE_DISABLE: c_int = 0;
pub const ACL_PRIO_MODE_HIGHER: c_int = 1;
pub const ACL_PRIO_MODE_LOWER: c_int = 2;
pub const ACL_PRIO_MODE_REPLACE: c_int = 3;

pub const ACL_PRIO_S: c_int = 3;

pub const ACL_VLAN_PRIO_HI_M: c_uint = 0x3;
pub const REG_PORT_ACL_B: c_uint = 0x060B;
pub const ACL_VLAN_PRIO_LO_M: c_uint = 0x8;
pub const ACL_VLAN_PRIO_S: c_int = 7;
pub const ACL_MAP_MODE_M: c_uint = 0x3;
pub const ACL_MAP_MODE_S: c_int = 5;
pub const ACL_MAP_MODE_DISABLE: c_int = 0;
pub const ACL_MAP_MODE_OR: c_int = 1;
pub const ACL_MAP_MODE_AND: c_int = 2;
pub const ACL_MAP_MODE_REPLACE: c_int = 3;

pub const ACL_CNT_S: c_int = 5;
pub const REG_PORT_ACL_C: c_uint = 0x060C;
pub const REG_PORT_ACL_D: c_uint = 0x060D;

pub const ACL_PORT_MAP: c_uint = 0x7F;
pub const REG_PORT_ACL_E: c_uint = 0x060E;
pub const REG_PORT_ACL_F: c_uint = 0x060F;
pub const REG_PORT_ACL_BYTE_EN_MSB: c_uint = 0x0610;
pub const REG_PORT_ACL_BYTE_EN_LSB: c_uint = 0x0611;
pub const ACL_ACTION_START: c_uint = 0xA;
pub const ACL_ACTION_LEN: c_int = 4;
pub const ACL_INTR_CNT_START: c_uint = 0xD;
pub const ACL_RULESET_START: c_uint = 0xE;
pub const ACL_RULESET_LEN: c_int = 2;
pub const ACL_TABLE_LEN: c_int = 16;
pub const ACL_ACTION_ENABLE: c_uint = 0x003C;
pub const ACL_MATCH_ENABLE: c_uint = 0x7FC3;
pub const ACL_RULESET_ENABLE: c_uint = 0x8003;
pub const ACL_BYTE_ENABLE: c_uint = 0xFFFF;
pub const REG_PORT_ACL_CTRL_0: c_uint = 0x0612;

pub const PORT_ACL_INDEX_M: c_uint = 0xF;
pub const REG_PORT_ACL_CTRL_1: c_uint = 0x0613;
// 8 - Classification and Policing
pub const REG_PORT_MRI_MIRROR_CTRL: c_uint = 0x0800;

pub const REG_PORT_MRI_PRIO_CTRL: c_uint = 0x0801;

pub const REG_PORT_MRI_MAC_CTRL: c_uint = 0x0802;

pub const PORT_BASED_PRIO_S: c_int = 0;
pub const REG_PORT_MRI_AUTHEN_CTRL: c_uint = 0x0803;

pub const PORT_AUTHEN_MODE: c_uint = 0x3;
pub const PORT_AUTHEN_PASS: c_int = 0;
pub const PORT_AUTHEN_BLOCK: c_int = 1;
pub const PORT_AUTHEN_TRAP: c_int = 2;
pub const REG_PORT_MRI_INDEX__4: c_uint = 0x0804;
pub const MRI_INDEX_P_M: c_uint = 0x7;
pub const MRI_INDEX_P_S: c_int = 16;
pub const MRI_INDEX_Q_M: c_uint = 0x3;
pub const MRI_INDEX_Q_S: c_int = 0;
pub const REG_PORT_MRI_TC_MAP__4: c_uint = 0x0808;
pub const PORT_TC_MAP_M: c_uint = 0xf;
pub const PORT_TC_MAP_S: c_int = 4;
pub const REG_PORT_MRI_POLICE_CTRL__4: c_uint = 0x080C;

pub const POLICE_PACKET_TYPE_M: c_uint = 0x3;
pub const POLICE_PACKET_TYPE_S: c_int = 8;
pub const POLICE_PACKET_DROPPED: c_int = 0;
pub const POLICE_PACKET_GREEN: c_int = 1;
pub const POLICE_PACKET_YELLOW: c_int = 2;
pub const POLICE_PACKET_RED: c_int = 3;

pub const NON_DSCP_COLOR_M: c_uint = 0x3;
pub const NON_DSCP_COLOR_S: c_int = 5;

pub const REG_PORT_POLICE_COLOR_0__4: c_uint = 0x0810;
pub const REG_PORT_POLICE_COLOR_1__4: c_uint = 0x0814;
pub const REG_PORT_POLICE_COLOR_2__4: c_uint = 0x0818;
pub const REG_PORT_POLICE_COLOR_3__4: c_uint = 0x081C;
pub const POLICE_COLOR_MAP_S: c_int = 2;

pub const REG_PORT_POLICE_RATE__4: c_uint = 0x0820;
pub const POLICE_CIR_S: c_int = 16;
pub const POLICE_PIR_S: c_int = 0;
pub const REG_PORT_POLICE_BURST_SIZE__4: c_uint = 0x0824;
pub const POLICE_BURST_SIZE_M: c_uint = 0x3FFF;
pub const POLICE_CBS_S: c_int = 16;
pub const POLICE_PBS_S: c_int = 0;
pub const REG_PORT_WRED_PM_CTRL_0__4: c_uint = 0x0830;

pub const WRED_PM_MAX_THRESHOLD_S: c_int = 16;
pub const WRED_PM_MIN_THRESHOLD_S: c_int = 0;
pub const REG_PORT_WRED_PM_CTRL_1__4: c_uint = 0x0834;
pub const WRED_PM_MULTIPLIER_S: c_int = 16;
pub const WRED_PM_AVG_QUEUE_SIZE_S: c_int = 0;
pub const REG_PORT_WRED_QUEUE_CTRL_0__4: c_uint = 0x0840;
pub const REG_PORT_WRED_QUEUE_CTRL_1__4: c_uint = 0x0844;
pub const REG_PORT_WRED_QUEUE_PMON__4: c_uint = 0x0848;

// 9 - Shaping
pub const REG_PORT_MTI_QUEUE_CTRL_0__4: c_uint = 0x0904;

pub const REG_PORT_MTI_CREDIT_INCREMENT: c_uint = 0x091A;
// A - QM
pub const REG_PORT_QM_CTRL__4: c_uint = 0x0A00;
pub const PORT_QM_DROP_PRIO_M: c_uint = 0x3;
pub const REG_PORT_VLAN_MEMBERSHIP__4: c_uint = 0x0A04;
pub const REG_PORT_QM_QUEUE_INDEX__4: c_uint = 0x0A08;
pub const PORT_QM_QUEUE_INDEX_S: c_int = 24;
pub const PORT_QM_BURST_SIZE_S: c_int = 16;

pub const REG_PORT_QM_WATER_MARK__4: c_uint = 0x0A0C;
pub const PORT_QM_HI_WATER_MARK_S: c_int = 16;
pub const PORT_QM_LO_WATER_MARK_S: c_int = 0;

pub const REG_PORT_QM_TX_CNT_0__4: c_uint = 0x0A10;
pub const PORT_QM_TX_CNT_USED_S: c_int = 0;

pub const PORT_QM_TX_CNT_MAX: c_uint = 0x200;
pub const REG_PORT_QM_TX_CNT_1__4: c_uint = 0x0A14;
pub const PORT_QM_TX_CNT_CALCULATED_S: c_int = 16;
pub const PORT_QM_TX_CNT_AVAIL_S: c_int = 0;
// B - LUE
pub const REG_PORT_LUE_CTRL: c_uint = 0x0B00;

pub const REG_PORT_LUE_MSTP_INDEX: c_uint = 0x0B01;
pub const REG_PORT_LUE_MSTP_STATE: c_uint = 0x0B04;
// C - PTP
pub const REG_PTP_PORT_RX_DELAY__2: c_uint = 0x0C00;
pub const REG_PTP_PORT_TX_DELAY__2: c_uint = 0x0C02;
pub const REG_PTP_PORT_ASYM_DELAY__2: c_uint = 0x0C04;
pub const REG_PTP_PORT_XDELAY_TS: c_uint = 0x0C08;
pub const REG_PTP_PORT_XDELAY_TS_H: c_uint = 0x0C08;
pub const REG_PTP_PORT_XDELAY_TS_L: c_uint = 0x0C0A;
pub const REG_PTP_PORT_SYNC_TS: c_uint = 0x0C0C;
pub const REG_PTP_PORT_SYNC_TS_H: c_uint = 0x0C0C;
pub const REG_PTP_PORT_SYNC_TS_L: c_uint = 0x0C0E;
pub const REG_PTP_PORT_PDRESP_TS: c_uint = 0x0C10;
pub const REG_PTP_PORT_PDRESP_TS_H: c_uint = 0x0C10;
pub const REG_PTP_PORT_PDRESP_TS_L: c_uint = 0x0C12;
pub const REG_PTP_PORT_TX_INT_STATUS__2: c_uint = 0x0C14;
pub const REG_PTP_PORT_TX_INT_ENABLE__2: c_uint = 0x0C16;

pub const REG_PTP_PORT_LINK_DELAY__4: c_uint = 0x0C18;
pub const PRIO_QUEUES: c_int = 4;
pub const RX_PRIO_QUEUES: c_int = 8;
pub const KS_PRIO_IN_REG: c_int = 2;
pub const TOTAL_PORT_NUM: c_int = 7;
pub const KSZ9477_COUNTER_NUM: c_uint = 0x20;

pub const MAX_TIMESTAMP_UNIT: c_int = 2;
pub const MAX_TRIG_UNIT: c_int = 3;
pub const MAX_TIMESTAMP_EVENT_UNIT: c_int = 8;
pub const MAX_GPIO: c_int = 4;

