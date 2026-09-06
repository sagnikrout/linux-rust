//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/gaudi2/asic_reg/pcie_aux_regs.h
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
// PCIE_AUX
// (Prototype: PCIE_AUX)
//
pub const mmPCIE_AUX_APB_TIMEOUT: c_uint = 0x4C07004;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_0: c_uint = 0x4C07008;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_1: c_uint = 0x4C0700C;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_2: c_uint = 0x4C07010;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_3: c_uint = 0x4C07014;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_4: c_uint = 0x4C07018;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_5: c_uint = 0x4C0701C;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_6: c_uint = 0x4C07020;
pub const mmPCIE_AUX_SW_GENERAL_PURPOSE_7: c_uint = 0x4C07024;
pub const mmPCIE_AUX_PHY_INIT: c_uint = 0x4C07100;
pub const mmPCIE_AUX_LTR_MAX_LATENCY: c_uint = 0x4C07138;
pub const mmPCIE_AUX_BAR0_START_L: c_uint = 0x4C07160;
pub const mmPCIE_AUX_BAR0_START_H: c_uint = 0x4C07164;
pub const mmPCIE_AUX_BAR1_START: c_uint = 0x4C07168;
pub const mmPCIE_AUX_BAR2_START_L: c_uint = 0x4C0716C;
pub const mmPCIE_AUX_BAR2_START_H: c_uint = 0x4C07170;
pub const mmPCIE_AUX_BAR3_START: c_uint = 0x4C07174;
pub const mmPCIE_AUX_BAR4_START_L: c_uint = 0x4C07178;
pub const mmPCIE_AUX_BAR4_START_H: c_uint = 0x4C0717C;
pub const mmPCIE_AUX_BAR5_START: c_uint = 0x4C07180;
pub const mmPCIE_AUX_BAR0_LIMIT_L: c_uint = 0x4C07184;
pub const mmPCIE_AUX_BAR0_LIMIT_H: c_uint = 0x4C07188;
pub const mmPCIE_AUX_BAR1_LIMIT: c_uint = 0x4C0718C;
pub const mmPCIE_AUX_BAR2_LIMIT_L: c_uint = 0x4C07190;
pub const mmPCIE_AUX_BAR2_LIMIT_H: c_uint = 0x4C07194;
pub const mmPCIE_AUX_BAR3_LIMIT: c_uint = 0x4C07198;
pub const mmPCIE_AUX_BAR4_LIMIT_L: c_uint = 0x4C0719C;
pub const mmPCIE_AUX_BAR4_LIMIT_H: c_uint = 0x4C07200;
pub const mmPCIE_AUX_BAR5_LIMIT: c_uint = 0x4C07204;
pub const mmPCIE_AUX_BUS_MASTER_EN: c_uint = 0x4C07208;
pub const mmPCIE_AUX_MEM_SPACE_EN: c_uint = 0x4C0720C;
pub const mmPCIE_AUX_MAX_RD_REQ_SIZE: c_uint = 0x4C07210;
pub const mmPCIE_AUX_MAX_PAYLOAD_SIZE: c_uint = 0x4C07214;
pub const mmPCIE_AUX_EXT_TAG_EN: c_uint = 0x4C07218;
pub const mmPCIE_AUX_RCB: c_uint = 0x4C0721C;
pub const mmPCIE_AUX_PM_NO_SOFT_RST: c_uint = 0x4C07220;
pub const mmPCIE_AUX_PBUS_NUM: c_uint = 0x4C07224;
pub const mmPCIE_AUX_PBUS_DEV_NUM: c_uint = 0x4C07228;
pub const mmPCIE_AUX_NO_SNOOP_EN: c_uint = 0x4C0722C;
pub const mmPCIE_AUX_RELAX_ORDER_EN: c_uint = 0x4C07230;
pub const mmPCIE_AUX_HP_SLOT_CTRL_ACCESS: c_uint = 0x4C07234;
pub const mmPCIE_AUX_DLL_STATE_CHGED_EN: c_uint = 0x4C07238;
pub const mmPCIE_AUX_CMP_CPLED_INT_EN: c_uint = 0x4C0723C;
pub const mmPCIE_AUX_HP_INT_EN: c_uint = 0x4C07340;
pub const mmPCIE_AUX_PRE_DET_CHGEN_EN: c_uint = 0x4C07344;
pub const mmPCIE_AUX_MRL_SENSOR_CHGED_EN: c_uint = 0x4C07348;
pub const mmPCIE_AUX_PWR_FAULT_DET_EN: c_uint = 0x4C0734C;
pub const mmPCIE_AUX_ATTEN_BUTTON_PRESSED_EN: c_uint = 0x4C07350;
pub const mmPCIE_AUX_PF_FLR_ACTIVE: c_uint = 0x4C07360;
pub const mmPCIE_AUX_PF_FLR_DONE: c_uint = 0x4C07364;
pub const mmPCIE_AUX_FLR_INT: c_uint = 0x4C07390;
pub const mmPCIE_AUX_FLR_CTRL: c_uint = 0x4C07394;
pub const mmPCIE_AUX_LTR_M_EN: c_uint = 0x4C073B0;
pub const mmPCIE_AUX_LTSSM_EN: c_uint = 0x4C07428;
pub const mmPCIE_AUX_SYS_INTR: c_uint = 0x4C07440;
pub const mmPCIE_AUX_INT_DISABLE: c_uint = 0x4C07444;
pub const mmPCIE_AUX_SMLH_LINK_UP: c_uint = 0x4C07448;
pub const mmPCIE_AUX_PM_CURR_STATE: c_uint = 0x4C07450;
pub const mmPCIE_AUX_RDLH_LINK_UP: c_uint = 0x4C07458;
pub const mmPCIE_AUX_BRDG_SLV_XFER_PENDING: c_uint = 0x4C0745C;
pub const mmPCIE_AUX_BRDG_DBI_XFER_PENDING: c_uint = 0x4C07460;
pub const mmPCIE_AUX_AUTO_SP_DIS: c_uint = 0x4C07478;
pub const mmPCIE_AUX_DBI: c_uint = 0x4C07490;
pub const mmPCIE_AUX_DBI_32: c_uint = 0x4C07494;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_0: c_uint = 0x4C074A4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_1: c_uint = 0x4C074A8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_2: c_uint = 0x4C074AC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_3: c_uint = 0x4C074B0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_4: c_uint = 0x4C074B4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_5: c_uint = 0x4C074B8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_6: c_uint = 0x4C074BC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_7: c_uint = 0x4C074C0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_8: c_uint = 0x4C074C4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_9: c_uint = 0x4C074C8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_10: c_uint = 0x4C074CC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_11: c_uint = 0x4C074D0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_12: c_uint = 0x4C074D4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_13: c_uint = 0x4C074D8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_14: c_uint = 0x4C074DC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_15: c_uint = 0x4C074E0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_16: c_uint = 0x4C074E4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_17: c_uint = 0x4C074E8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_18: c_uint = 0x4C074EC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_19: c_uint = 0x4C074F0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_20: c_uint = 0x4C074F4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_21: c_uint = 0x4C074F8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_22: c_uint = 0x4C074FC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_23: c_uint = 0x4C07500;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_24: c_uint = 0x4C07504;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_25: c_uint = 0x4C07508;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_26: c_uint = 0x4C0750C;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_27: c_uint = 0x4C07510;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_28: c_uint = 0x4C07514;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_0: c_uint = 0x4C07640;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_1: c_uint = 0x4C07644;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_2: c_uint = 0x4C07648;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_3: c_uint = 0x4C0764C;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_4: c_uint = 0x4C07650;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_5: c_uint = 0x4C07654;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_6: c_uint = 0x4C07658;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_7: c_uint = 0x4C0765C;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_8: c_uint = 0x4C07660;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_9: c_uint = 0x4C07664;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_10: c_uint = 0x4C07668;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_11: c_uint = 0x4C0766C;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_12: c_uint = 0x4C07670;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_13: c_uint = 0x4C07674;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_14: c_uint = 0x4C07678;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_15: c_uint = 0x4C0767C;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_0: c_uint = 0x4C07744;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_1: c_uint = 0x4C07748;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_2: c_uint = 0x4C0774C;
pub const mmPCIE_AUX_APP_RAS_DES_TBA_CTRL: c_uint = 0x4C07774;
pub const mmPCIE_AUX_PM_MASTER_STATE: c_uint = 0x4C07838;
pub const mmPCIE_AUX_PM_SLAVE_STATE: c_uint = 0x4C0783C;
pub const mmPCIE_AUX_PM_DSTATE: c_uint = 0x4C07840;
pub const mmPCIE_AUX_PM_PME_EN: c_uint = 0x4C07844;
pub const mmPCIE_AUX_PM_LINKST_IN_L0S: c_uint = 0x4C07848;
pub const mmPCIE_AUX_PM_LINKST_IN_L1: c_uint = 0x4C0784C;
pub const mmPCIE_AUX_PM_LINKST_IN_L2: c_uint = 0x4C07850;
pub const mmPCIE_AUX_PM_LINKST_L2_EXIT: c_uint = 0x4C07854;
pub const mmPCIE_AUX_PM_STATUS: c_uint = 0x4C07858;
pub const mmPCIE_AUX_APP_READY_ENTER_L23: c_uint = 0x4C0785C;
pub const mmPCIE_AUX_APP_XFER_PENDING: c_uint = 0x4C07860;
pub const mmPCIE_AUX_APP_REQ_L1: c_uint = 0x4C07930;
pub const mmPCIE_AUX_AUX_PM_EN: c_uint = 0x4C07934;
pub const mmPCIE_AUX_APPS_PM_XMT_PME: c_uint = 0x4C07938;
pub const mmPCIE_AUX_OUTBAND_PWRUP_CMD: c_uint = 0x4C07940;
pub const mmPCIE_AUX_PERST: c_uint = 0x4C079B8;
pub const mmPCIE_AUX_DBI_RO_WR_DISABLE: c_uint = 0x4C079BC;
pub const mmPCIE_AUX_HOLD_PHY_RST: c_uint = 0x4C079C0;
pub const mmPCIE_AUX_TLP_INTERNAL_ERR_REP: c_uint = 0x4C079C4;
pub const mmPCIE_AUX_APP_SRIS_MODE: c_uint = 0x4C079C8;
pub const mmPCIE_AUX_BUS_MSTR_EN_CLR_INTR: c_uint = 0x4C079CC;
pub const mmPCIE_AUX_BUS_MSTR_EN_CLR_INTR_MASK: c_uint = 0x4C079D0;
