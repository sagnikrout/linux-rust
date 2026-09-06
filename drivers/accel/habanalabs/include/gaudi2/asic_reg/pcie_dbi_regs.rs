//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_dbi_regs.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PCIE_DBI
// (Prototype: PCIE_DBI)
//
pub const mmPCIE_DBI_DEVICE_ID_VENDOR_ID_REG: c_uint = 0x4C02000;
pub const mmPCIE_DBI_STATUS_COMMAND_REG: c_uint = 0x4C02004;
pub const mmPCIE_DBI_CLASS_CODE_REVISION_ID: c_uint = 0x4C02008;
pub const mmPCIE_DBI_BIST_HEADER_TYPE_LATENCY_CACHE_LINE_SIZE_REG: c_uint = 0x4C0200C;
pub const mmPCIE_DBI_BAR0_REG: c_uint = 0x4C02010;
pub const mmPCIE_DBI_BAR1_REG: c_uint = 0x4C02014;
pub const mmPCIE_DBI_BAR2_REG: c_uint = 0x4C02018;
pub const mmPCIE_DBI_BAR3_REG: c_uint = 0x4C0201C;
pub const mmPCIE_DBI_BAR4_REG: c_uint = 0x4C02020;
pub const mmPCIE_DBI_BAR5_REG: c_uint = 0x4C02024;
pub const mmPCIE_DBI_CARDBUS_CIS_PTR_REG: c_uint = 0x4C02028;
pub const mmPCIE_DBI_SUBSYSTEM_ID_SUBSYSTEM_VENDOR_ID_REG: c_uint = 0x4C0202C;
pub const mmPCIE_DBI_EXP_ROM_BASE_ADDR_REG: c_uint = 0x4C02030;
pub const mmPCIE_DBI_PCI_CAP_PTR_REG: c_uint = 0x4C02034;
pub const mmPCIE_DBI_MAX_LATENCY_MIN_GRANT_INTERRUPT_PIN_INTERRUPT_LINE_REG: c_uint = 0x4C0203C;
pub const mmPCIE_DBI_CAP_ID_NXT_PTR_REG: c_uint = 0x4C02040;
pub const mmPCIE_DBI_CON_STATUS_REG: c_uint = 0x4C02044;
pub const mmPCIE_DBI_PCI_MSI_CAP_ID_NEXT_CTRL_REG: c_uint = 0x4C02050;
pub const mmPCIE_DBI_MSI_CAP_OFF_04H_REG: c_uint = 0x4C02054;
pub const mmPCIE_DBI_MSI_CAP_OFF_08H_REG: c_uint = 0x4C02058;
pub const mmPCIE_DBI_MSI_CAP_OFF_0CH_REG: c_uint = 0x4C0205C;
pub const mmPCIE_DBI_MSI_CAP_OFF_10H_REG: c_uint = 0x4C02060;
pub const mmPCIE_DBI_MSI_CAP_OFF_14H_REG: c_uint = 0x4C02064;
pub const mmPCIE_DBI_PCIE_CAP_ID_PCIE_NEXT_CAP_PTR_PCIE_CAP_REG: c_uint = 0x4C02070;
pub const mmPCIE_DBI_DEVICE_CAPABILITIES_REG: c_uint = 0x4C02074;
pub const mmPCIE_DBI_DEVICE_CONTROL_DEVICE_STATUS: c_uint = 0x4C02078;
pub const mmPCIE_DBI_LINK_CAPABILITIES_REG: c_uint = 0x4C0207C;
pub const mmPCIE_DBI_LINK_CONTROL_LINK_STATUS_REG: c_uint = 0x4C02080;
pub const mmPCIE_DBI_DEVICE_CAPABILITIES2_REG: c_uint = 0x4C02094;
pub const mmPCIE_DBI_DEVICE_CONTROL2_DEVICE_STATUS2_REG: c_uint = 0x4C02098;
pub const mmPCIE_DBI_LINK_CAPABILITIES2_REG: c_uint = 0x4C0209C;
pub const mmPCIE_DBI_LINK_CONTROL2_LINK_STATUS2_REG: c_uint = 0x4C020A0;
pub const mmPCIE_DBI_PCI_MSIX_CAP_ID_NEXT_CTRL_REG: c_uint = 0x4C020B0;
pub const mmPCIE_DBI_MSIX_TABLE_OFFSET_REG: c_uint = 0x4C020B4;
pub const mmPCIE_DBI_MSIX_PBA_OFFSET_REG: c_uint = 0x4C020B8;
pub const mmPCIE_DBI_AER_EXT_CAP_HDR_OFF: c_uint = 0x4C02100;
pub const mmPCIE_DBI_UNCORR_ERR_STATUS_OFF: c_uint = 0x4C02104;
pub const mmPCIE_DBI_UNCORR_ERR_MASK_OFF: c_uint = 0x4C02108;
pub const mmPCIE_DBI_UNCORR_ERR_SEV_OFF: c_uint = 0x4C0210C;
pub const mmPCIE_DBI_CORR_ERR_STATUS_OFF: c_uint = 0x4C02110;
pub const mmPCIE_DBI_CORR_ERR_MASK_OFF: c_uint = 0x4C02114;
pub const mmPCIE_DBI_ADV_ERR_CAP_CTRL_OFF: c_uint = 0x4C02118;
pub const mmPCIE_DBI_HDR_LOG_0_OFF: c_uint = 0x4C0211C;
pub const mmPCIE_DBI_HDR_LOG_1_OFF: c_uint = 0x4C02120;
pub const mmPCIE_DBI_HDR_LOG_2_OFF: c_uint = 0x4C02124;
pub const mmPCIE_DBI_HDR_LOG_3_OFF: c_uint = 0x4C02128;
pub const mmPCIE_DBI_TLP_PREFIX_LOG_1_OFF: c_uint = 0x4C02138;
pub const mmPCIE_DBI_TLP_PREFIX_LOG_2_OFF: c_uint = 0x4C0213C;
pub const mmPCIE_DBI_TLP_PREFIX_LOG_3_OFF: c_uint = 0x4C02140;
pub const mmPCIE_DBI_TLP_PREFIX_LOG_4_OFF: c_uint = 0x4C02144;
pub const mmPCIE_DBI_SPCIE_CAP_HEADER_REG: c_uint = 0x4C02148;
pub const mmPCIE_DBI_LINK_CONTROL3_REG: c_uint = 0x4C0214C;
pub const mmPCIE_DBI_LANE_ERR_STATUS_REG: c_uint = 0x4C02150;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_0CH_REG: c_uint = 0x4C02154;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_10H_REG: c_uint = 0x4C02158;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_14H_REG: c_uint = 0x4C0215C;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_18H_REG: c_uint = 0x4C02160;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_1CH_REG: c_uint = 0x4C02164;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_20H_REG: c_uint = 0x4C02168;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_24H_REG: c_uint = 0x4C0216C;
pub const mmPCIE_DBI_SPCIE_CAP_OFF_28H_REG: c_uint = 0x4C02170;
pub const mmPCIE_DBI_PL16G_EXT_CAP_HDR_REG: c_uint = 0x4C02178;
pub const mmPCIE_DBI_PL16G_CAPABILITY_REG: c_uint = 0x4C0217C;
pub const mmPCIE_DBI_PL16G_CONTROL_REG: c_uint = 0x4C02180;
pub const mmPCIE_DBI_PL16G_STATUS_REG: c_uint = 0x4C02184;
pub const mmPCIE_DBI_PL16G_LC_DPAR_STATUS_REG: c_uint = 0x4C02188;
pub const mmPCIE_DBI_PL16G_FIRST_RETIMER_DPAR_STATUS_REG: c_uint = 0x4C0218C;
pub const mmPCIE_DBI_PL16G_SECOND_RETIMER_DPAR_STATUS_REG: c_uint = 0x4C02190;
pub const mmPCIE_DBI_PL16G_CAP_OFF_20H_REG: c_uint = 0x4C02198;
pub const mmPCIE_DBI_PL16G_CAP_OFF_24H_REG: c_uint = 0x4C0219C;
pub const mmPCIE_DBI_PL16G_CAP_OFF_28H_REG: c_uint = 0x4C021A0;
pub const mmPCIE_DBI_PL16G_CAP_OFF_2CH_REG: c_uint = 0x4C021A4;
pub const mmPCIE_DBI_MARGIN_EXT_CAP_HDR_REG: c_uint = 0x4C021A8;
pub const mmPCIE_DBI_MARGIN_PORT_CAPABILITIES_STATUS_REG: c_uint = 0x4C021AC;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS0_REG: c_uint = 0x4C021B0;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS1_REG: c_uint = 0x4C021B4;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS2_REG: c_uint = 0x4C021B8;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS3_REG: c_uint = 0x4C021BC;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS4_REG: c_uint = 0x4C021C0;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS5_REG: c_uint = 0x4C021C4;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS6_REG: c_uint = 0x4C021C8;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS7_REG: c_uint = 0x4C021CC;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS8_REG: c_uint = 0x4C021D0;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS9_REG: c_uint = 0x4C021D4;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS10_REG: c_uint = 0x4C021D8;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS11_REG: c_uint = 0x4C021DC;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS12_REG: c_uint = 0x4C021E0;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS13_REG: c_uint = 0x4C021E4;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS14_REG: c_uint = 0x4C021E8;
pub const mmPCIE_DBI_MARGIN_LANE_CNTRL_STATUS15_REG: c_uint = 0x4C021EC;
pub const mmPCIE_DBI_LTR_CAP_HDR_REG: c_uint = 0x4C021F0;
pub const mmPCIE_DBI_LTR_LATENCY_REG: c_uint = 0x4C021F4;
pub const mmPCIE_DBI_RAS_DES_CAP_HEADER_REG: c_uint = 0x4C021F8;
pub const mmPCIE_DBI_VENDOR_SPECIFIC_HEADER_REG: c_uint = 0x4C021FC;
pub const mmPCIE_DBI_EVENT_COUNTER_CONTROL_REG: c_uint = 0x4C02200;
pub const mmPCIE_DBI_EVENT_COUNTER_DATA_REG: c_uint = 0x4C02204;
pub const mmPCIE_DBI_TIME_BASED_ANALYSIS_CONTROL_REG: c_uint = 0x4C02208;
pub const mmPCIE_DBI_TIME_BASED_ANALYSIS_DATA_REG: c_uint = 0x4C0220C;
pub const mmPCIE_DBI_TIME_BASED_ANALYSIS_DATA_63_32_REG: c_uint = 0x4C02210;
pub const mmPCIE_DBI_EINJ_ENABLE_REG: c_uint = 0x4C02228;
pub const mmPCIE_DBI_EINJ0_CRC_REG: c_uint = 0x4C0222C;
pub const mmPCIE_DBI_EINJ1_SEQNUM_REG: c_uint = 0x4C02230;
pub const mmPCIE_DBI_EINJ2_DLLP_REG: c_uint = 0x4C02234;
pub const mmPCIE_DBI_EINJ3_SYMBOL_REG: c_uint = 0x4C02238;
pub const mmPCIE_DBI_EINJ4_FC_REG: c_uint = 0x4C0223C;
pub const mmPCIE_DBI_EINJ5_SP_TLP_REG: c_uint = 0x4C02240;
pub const mmPCIE_DBI_EINJ6_COMPARE_POINT_H0_REG: c_uint = 0x4C02244;
pub const mmPCIE_DBI_EINJ6_COMPARE_POINT_H1_REG: c_uint = 0x4C02248;
pub const mmPCIE_DBI_EINJ6_COMPARE_POINT_H2_REG: c_uint = 0x4C0224C;
pub const mmPCIE_DBI_EINJ6_COMPARE_POINT_H3_REG: c_uint = 0x4C02250;
pub const mmPCIE_DBI_EINJ6_COMPARE_VALUE_H0_REG: c_uint = 0x4C02254;
pub const mmPCIE_DBI_EINJ6_COMPARE_VALUE_H1_REG: c_uint = 0x4C02258;
pub const mmPCIE_DBI_EINJ6_COMPARE_VALUE_H2_REG: c_uint = 0x4C0225C;
pub const mmPCIE_DBI_EINJ6_COMPARE_VALUE_H3_REG: c_uint = 0x4C02260;
pub const mmPCIE_DBI_EINJ6_CHANGE_POINT_H0_REG: c_uint = 0x4C02264;
pub const mmPCIE_DBI_EINJ6_CHANGE_POINT_H1_REG: c_uint = 0x4C02268;
pub const mmPCIE_DBI_EINJ6_CHANGE_POINT_H2_REG: c_uint = 0x4C0226C;
pub const mmPCIE_DBI_EINJ6_CHANGE_POINT_H3_REG: c_uint = 0x4C02270;
pub const mmPCIE_DBI_EINJ6_CHANGE_VALUE_H0_REG: c_uint = 0x4C02274;
pub const mmPCIE_DBI_EINJ6_CHANGE_VALUE_H1_REG: c_uint = 0x4C02278;
pub const mmPCIE_DBI_EINJ6_CHANGE_VALUE_H2_REG: c_uint = 0x4C0227C;
pub const mmPCIE_DBI_EINJ6_CHANGE_VALUE_H3_REG: c_uint = 0x4C02280;
pub const mmPCIE_DBI_EINJ6_TLP_REG: c_uint = 0x4C02284;
pub const mmPCIE_DBI_SD_CONTROL1_REG: c_uint = 0x4C02298;
pub const mmPCIE_DBI_SD_CONTROL2_REG: c_uint = 0x4C0229C;
pub const mmPCIE_DBI_SD_STATUS_L1LANE_REG: c_uint = 0x4C022A8;
pub const mmPCIE_DBI_SD_STATUS_L1LTSSM_REG: c_uint = 0x4C022AC;
pub const mmPCIE_DBI_SD_STATUS_PM_REG: c_uint = 0x4C022B0;
pub const mmPCIE_DBI_SD_STATUS_L2_REG: c_uint = 0x4C022B4;
pub const mmPCIE_DBI_SD_STATUS_L3FC_REG: c_uint = 0x4C022B8;
pub const mmPCIE_DBI_SD_STATUS_L3_REG: c_uint = 0x4C022BC;
pub const mmPCIE_DBI_SD_EQ_CONTROL1_REG: c_uint = 0x4C022C8;
pub const mmPCIE_DBI_SD_EQ_CONTROL2_REG: c_uint = 0x4C022CC;
pub const mmPCIE_DBI_SD_EQ_CONTROL3_REG: c_uint = 0x4C022D0;
pub const mmPCIE_DBI_SD_EQ_STATUS1_REG: c_uint = 0x4C022D8;
pub const mmPCIE_DBI_SD_EQ_STATUS2_REG: c_uint = 0x4C022DC;
pub const mmPCIE_DBI_SD_EQ_STATUS3_REG: c_uint = 0x4C022E0;
pub const mmPCIE_DBI_DATA_LINK_FEATURE_EXT_HDR_OFF: c_uint = 0x4C022F8;
pub const mmPCIE_DBI_DATA_LINK_FEATURE_CAP_OFF: c_uint = 0x4C022FC;
pub const mmPCIE_DBI_DATA_LINK_FEATURE_STATUS_OFF: c_uint = 0x4C02300;
pub const mmPCIE_DBI_ACK_LATENCY_TIMER_OFF: c_uint = 0x4C02700;
pub const mmPCIE_DBI_VENDOR_SPEC_DLLP_OFF: c_uint = 0x4C02704;
pub const mmPCIE_DBI_PORT_FORCE_OFF: c_uint = 0x4C02708;
pub const mmPCIE_DBI_ACK_F_ASPM_CTRL_OFF: c_uint = 0x4C0270C;
pub const mmPCIE_DBI_PORT_LINK_CTRL_OFF: c_uint = 0x4C02710;
pub const mmPCIE_DBI_LANE_SKEW_OFF: c_uint = 0x4C02714;
pub const mmPCIE_DBI_TIMER_CTRL_MAX_FUNC_NUM_OFF: c_uint = 0x4C02718;
pub const mmPCIE_DBI_SYMBOL_TIMER_FILTER_1_OFF: c_uint = 0x4C0271C;
pub const mmPCIE_DBI_FILTER_MASK_2_OFF: c_uint = 0x4C02720;
pub const mmPCIE_DBI_AMBA_MUL_OB_DECOMP_NP_SUB_REQ_CTRL_OFF: c_uint = 0x4C02724;
pub const mmPCIE_DBI_PL_DEBUG0_OFF: c_uint = 0x4C02728;
pub const mmPCIE_DBI_PL_DEBUG1_OFF: c_uint = 0x4C0272C;
pub const mmPCIE_DBI_TX_P_FC_CREDIT_STATUS_OFF: c_uint = 0x4C02730;
pub const mmPCIE_DBI_TX_NP_FC_CREDIT_STATUS_OFF: c_uint = 0x4C02734;
pub const mmPCIE_DBI_TX_CPL_FC_CREDIT_STATUS_OFF: c_uint = 0x4C02738;
pub const mmPCIE_DBI_QUEUE_STATUS_OFF: c_uint = 0x4C0273C;
pub const mmPCIE_DBI_VC_TX_ARBI_1_OFF: c_uint = 0x4C02740;
pub const mmPCIE_DBI_VC_TX_ARBI_2_OFF: c_uint = 0x4C02744;
pub const mmPCIE_DBI_VC0_P_RX_Q_CTRL_OFF: c_uint = 0x4C02748;
pub const mmPCIE_DBI_VC0_NP_RX_Q_CTRL_OFF: c_uint = 0x4C0274C;
pub const mmPCIE_DBI_VC0_CPL_RX_Q_CTRL_OFF: c_uint = 0x4C02750;
pub const mmPCIE_DBI_GEN2_CTRL_OFF: c_uint = 0x4C0280C;
pub const mmPCIE_DBI_PHY_STATUS_OFF: c_uint = 0x4C02810;
pub const mmPCIE_DBI_PHY_CONTROL_OFF: c_uint = 0x4C02814;
pub const mmPCIE_DBI_TRGT_MAP_CTRL_OFF: c_uint = 0x4C0281C;
pub const mmPCIE_DBI_CLOCK_GATING_CTRL_OFF: c_uint = 0x4C0288C;
pub const mmPCIE_DBI_GEN3_RELATED_OFF: c_uint = 0x4C02890;
pub const mmPCIE_DBI_GEN3_EQ_CONTROL_OFF: c_uint = 0x4C028A8;
pub const mmPCIE_DBI_GEN3_EQ_FB_MODE_DIR_CHANGE_OFF: c_uint = 0x4C028AC;
pub const mmPCIE_DBI_ORDER_RULE_CTRL_OFF: c_uint = 0x4C028B4;
pub const mmPCIE_DBI_PIPE_LOOPBACK_CONTROL_OFF: c_uint = 0x4C028B8;
pub const mmPCIE_DBI_MISC_CONTROL_1_OFF: c_uint = 0x4C028BC;
pub const mmPCIE_DBI_MULTI_LANE_CONTROL_OFF: c_uint = 0x4C028C0;
pub const mmPCIE_DBI_PHY_INTEROP_CTRL_OFF: c_uint = 0x4C028C4;
pub const mmPCIE_DBI_TRGT_CPL_LUT_DELETE_ENTRY_OFF: c_uint = 0x4C028C8;
pub const mmPCIE_DBI_LINK_FLUSH_CONTROL_OFF: c_uint = 0x4C028CC;
pub const mmPCIE_DBI_AMBA_ERROR_RESPONSE_DEFAULT_OFF: c_uint = 0x4C028D0;
pub const mmPCIE_DBI_AMBA_LINK_TIMEOUT_OFF: c_uint = 0x4C028D4;
pub const mmPCIE_DBI_AMBA_ORDERING_CTRL_OFF: c_uint = 0x4C028D8;
pub const mmPCIE_DBI_COHERENCY_CONTROL_1_OFF: c_uint = 0x4C028E0;
pub const mmPCIE_DBI_COHERENCY_CONTROL_2_OFF: c_uint = 0x4C028E4;
pub const mmPCIE_DBI_COHERENCY_CONTROL_3_OFF: c_uint = 0x4C028E8;
pub const mmPCIE_DBI_AXI_MSTR_MSG_ADDR_LOW_OFF: c_uint = 0x4C028F0;
pub const mmPCIE_DBI_AXI_MSTR_MSG_ADDR_HIGH_OFF: c_uint = 0x4C028F4;
pub const mmPCIE_DBI_PCIE_VERSION_NUMBER_OFF: c_uint = 0x4C028F8;
pub const mmPCIE_DBI_PCIE_VERSION_TYPE_OFF: c_uint = 0x4C028FC;
pub const mmPCIE_DBI_MSIX_ADDRESS_MATCH_LOW_OFF: c_uint = 0x4C02940;
pub const mmPCIE_DBI_MSIX_ADDRESS_MATCH_HIGH_OFF: c_uint = 0x4C02944;
pub const mmPCIE_DBI_MSIX_DOORBELL_OFF: c_uint = 0x4C02948;
pub const mmPCIE_DBI_MSIX_RAM_CTRL_OFF: c_uint = 0x4C0294C;
pub const mmPCIE_DBI_PL_LTR_LATENCY_OFF: c_uint = 0x4C02B30;
pub const mmPCIE_DBI_AUX_CLK_FREQ_OFF: c_uint = 0x4C02B40;
pub const mmPCIE_DBI_POWERDOWN_CTRL_STATUS_OFF: c_uint = 0x4C02B48;
pub const mmPCIE_DBI_PHY_VIEWPORT_CTLSTS_OFF: c_uint = 0x4C02B70;
pub const mmPCIE_DBI_PHY_VIEWPORT_DATA_OFF: c_uint = 0x4C02B74;
pub const mmPCIE_DBI_GEN4_LANE_MARGINING_1_OFF: c_uint = 0x4C02B80;
pub const mmPCIE_DBI_GEN4_LANE_MARGINING_2_OFF: c_uint = 0x4C02B84;
pub const mmPCIE_DBI_PIPE_RELATED_OFF: c_uint = 0x4C02B90;
pub const mmPCIE_DBI_RX_SERIALIZATION_Q_CTRL_OFF: c_uint = 0x4C02C00;
