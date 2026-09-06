//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_reg_addr.h
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
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

pub const NIG_REG_LLH_FUNC_TAG_EN: c_uint = 0x5019b0UL;
pub const NIG_REG_LLH_FUNC_TAG_VALUE: c_uint = 0x5019d0UL;

pub const IGU_REG_PRODUCER_MEMORY: c_uint = 0x182000UL;
pub const IGU_REG_CONSUMER_MEM: c_uint = 0x183000UL;

pub const MCP_REG_CPU_PROGRAM_COUNTER: c_uint = 0xe0501cUL;

pub const PGLUE_B_REG_VF_BAR1_SIZE: c_uint = 0x2aae68UL;
pub const PRS_REG_ENCAPSULATION_TYPE_EN: c_uint = 0x1f0730UL;
pub const PRS_REG_GRE_PROTOCOL: c_uint = 0x1f0734UL;
pub const PRS_REG_VXLAN_PORT: c_uint = 0x1f0738UL;
pub const PRS_REG_OUTPUT_FORMAT_4_0: c_uint = 0x1f099cUL;
pub const NIG_REG_ENC_TYPE_ENABLE: c_uint = 0x501058UL;

pub const NIG_REG_ENC_TYPE_ENABLE_ETH_OVER_GRE_ENABLE_SHIFT: c_int = 0;

pub const NIG_REG_ENC_TYPE_ENABLE_IP_OVER_GRE_ENABLE_SHIFT: c_int = 1;

pub const NIG_REG_ENC_TYPE_ENABLE_VXLAN_ENABLE_SHIFT: c_int = 2;
pub const NIG_REG_VXLAN_CTRL: c_uint = 0x50105cUL;
pub const PBF_REG_VXLAN_PORT: c_uint = 0xd80518UL;
pub const PBF_REG_NGE_PORT: c_uint = 0xd8051cUL;
pub const PRS_REG_NGE_PORT: c_uint = 0x1f086cUL;
pub const NIG_REG_NGE_PORT: c_uint = 0x508b38UL;
pub const DORQ_REG_L2_EDPM_TUNNEL_GRE_ETH_EN: c_uint = 0x10090cUL;
pub const DORQ_REG_L2_EDPM_TUNNEL_GRE_IP_EN: c_uint = 0x100910UL;
pub const DORQ_REG_L2_EDPM_TUNNEL_VXLAN_EN: c_uint = 0x100914UL;
pub const DORQ_REG_L2_EDPM_TUNNEL_NGE_IP_EN_K2: c_uint = 0x10092cUL;
pub const DORQ_REG_L2_EDPM_TUNNEL_NGE_ETH_EN_K2: c_uint = 0x100930UL;
pub const NIG_REG_NGE_IP_ENABLE: c_uint = 0x508b28UL;
pub const NIG_REG_NGE_ETH_ENABLE: c_uint = 0x508b2cUL;
pub const NIG_REG_NGE_COMP_VER: c_uint = 0x508b30UL;
pub const PBF_REG_NGE_COMP_VER: c_uint = 0xd80524UL;
pub const PRS_REG_NGE_COMP_VER: c_uint = 0x1f0878UL;
pub const QM_REG_WFQPFWEIGHT: c_uint = 0x2f4e80UL;
pub const QM_REG_WFQVPWEIGHT: c_uint = 0x2fa000UL;

// Macro flag: #define DORQ_REG_INT_STS_DORQ_FIFO_AFULL\

pub const NIG_REG_ROCE_DUPLICATE_TO_HOST: c_uint = 0x5088f0UL;
pub const NIG_REG_PPF_TO_ENGINE_SEL: c_uint = 0x508900UL;
pub const NIG_REG_PPF_TO_ENGINE_SEL_SIZE: c_int = 8;
pub const PRS_REG_LIGHT_L2_ETHERTYPE_EN: c_uint = 0x1f0968UL;
pub const NIG_REG_LLH_ENG_CLS_ENG_ID_TBL: c_uint = 0x501b90UL;
pub const DORQ_REG_PF_DPM_ENABLE: c_uint = 0x100510UL;
pub const DORQ_REG_PF_ICID_BIT_SHIFT_NORM: c_uint = 0x100448UL;
pub const DORQ_REG_PF_MIN_ADDR_REG1: c_uint = 0x100400UL;
pub const DORQ_REG_PF_DPI_BIT_SHIFT: c_uint = 0x100450UL;
pub const NIG_REG_RX_PTP_EN: c_uint = 0x501900UL;
pub const NIG_REG_TX_PTP_EN: c_uint = 0x501904UL;
pub const NIG_REG_LLH_PTP_TO_HOST: c_uint = 0x501908UL;
pub const NIG_REG_LLH_PTP_TO_MCP: c_uint = 0x50190cUL;
pub const NIG_REG_PTP_SW_TXTSEN: c_uint = 0x501910UL;
pub const NIG_REG_LLH_PTP_ETHERTYPE_1: c_uint = 0x501914UL;
pub const NIG_REG_LLH_PTP_MAC_DA_2_LSB: c_uint = 0x501918UL;
pub const NIG_REG_LLH_PTP_MAC_DA_2_MSB: c_uint = 0x50191cUL;
pub const NIG_REG_LLH_PTP_PARAM_MASK: c_uint = 0x501920UL;
pub const NIG_REG_LLH_PTP_RULE_MASK: c_uint = 0x501924UL;
pub const NIG_REG_TX_LLH_PTP_PARAM_MASK: c_uint = 0x501928UL;
pub const NIG_REG_TX_LLH_PTP_RULE_MASK: c_uint = 0x50192cUL;
pub const NIG_REG_LLH_PTP_HOST_BUF_SEQID: c_uint = 0x501930UL;
pub const NIG_REG_LLH_PTP_HOST_BUF_TS_LSB: c_uint = 0x501934UL;
pub const NIG_REG_LLH_PTP_HOST_BUF_TS_MSB: c_uint = 0x501938UL;
pub const NIG_REG_LLH_PTP_MCP_BUF_SEQID: c_uint = 0x50193cUL;
pub const NIG_REG_LLH_PTP_MCP_BUF_TS_LSB: c_uint = 0x501940UL;
pub const NIG_REG_LLH_PTP_MCP_BUF_TS_MSB: c_uint = 0x501944UL;
pub const NIG_REG_TX_LLH_PTP_BUF_SEQID: c_uint = 0x501948UL;
pub const NIG_REG_TX_LLH_PTP_BUF_TS_LSB: c_uint = 0x50194cUL;
pub const NIG_REG_TX_LLH_PTP_BUF_TS_MSB: c_uint = 0x501950UL;
pub const NIG_REG_RX_PTP_TS_MSB_ERR: c_uint = 0x501954UL;
pub const NIG_REG_TX_PTP_TS_MSB_ERR: c_uint = 0x501958UL;
pub const NIG_REG_TSGEN_SYNC_TIME_LSB: c_uint = 0x5088c0UL;
pub const NIG_REG_TSGEN_SYNC_TIME_MSB: c_uint = 0x5088c4UL;
pub const NIG_REG_TSGEN_RST_DRIFT_CNTR: c_uint = 0x5088d8UL;
pub const NIG_REG_TSGEN_DRIFT_CNTR_CONF: c_uint = 0x5088dcUL;
pub const NIG_REG_TS_OUTPUT_ENABLE_PDA: c_uint = 0x508870UL;
pub const NIG_REG_TIMESYNC_GEN_REG_BB: c_uint = 0x500d00UL;
pub const NIG_REG_TSGEN_FREE_CNT_VALUE_LSB: c_uint = 0x5088a8UL;
pub const NIG_REG_TSGEN_FREE_CNT_VALUE_MSB: c_uint = 0x5088acUL;
pub const NIG_REG_PTP_LATCH_OSTS_PKT_TIME: c_uint = 0x509040UL;
pub const PSWRQ2_REG_WR_MBS0: c_uint = 0x240400UL;
pub const PGLUE_B_REG_PGL_ADDR_E8_F0_K2: c_uint = 0x2aaf98UL;
pub const PGLUE_B_REG_PGL_ADDR_EC_F0_K2: c_uint = 0x2aaf9cUL;
pub const PGLUE_B_REG_PGL_ADDR_F0_F0_K2: c_uint = 0x2aafa0UL;
pub const PGLUE_B_REG_PGL_ADDR_F4_F0_K2: c_uint = 0x2aafa4UL;
pub const PGLUE_B_REG_MASTER_WRITE_PAD_ENABLE: c_uint = 0x2aae30UL;
pub const NIG_REG_TSGEN_FREECNT_UPDATE_K2: c_uint = 0x509008UL;
pub const CNIG_REG_NIG_PORT0_CONF_K2: c_uint = 0x218200UL;
pub const NIG_REG_TX_EDPM_CTRL: c_uint = 0x501f0cUL;

pub const NIG_REG_TX_EDPM_CTRL_TX_EDPM_EN_SHIFT: c_int = 0;

pub const NIG_REG_TX_EDPM_CTRL_TX_EDPM_TC_EN_SHIFT: c_int = 1;
pub const PRS_REG_SEARCH_GFT: c_uint = 0x1f11bcUL;
pub const PRS_REG_SEARCH_NON_IP_AS_GFT: c_uint = 0x1f11c0UL;
pub const PRS_REG_CM_HDR_GFT: c_uint = 0x1f11c8UL;
pub const PRS_REG_GFT_CAM: c_uint = 0x1f1100UL;
pub const PRS_REG_GFT_PROFILE_MASK_RAM: c_uint = 0x1f1000UL;
pub const PRS_REG_CM_HDR_GFT_EVENT_ID_SHIFT: c_int = 0;
pub const PRS_REG_CM_HDR_GFT_CM_HDR_SHIFT: c_int = 8;
pub const PRS_REG_LOAD_L2_FILTER: c_uint = 0x1f0198UL;
