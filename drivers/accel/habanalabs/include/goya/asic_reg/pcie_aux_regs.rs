//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/goya/asic_reg/pcie_aux_regs.h
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
// Copyright 2016-2018 HabanaLabs, Ltd.
// All Rights Reserved.
//
// This is an auto-generated file
// DO NOT EDIT BELOW
//
// PCIE_AUX (Prototype: PCIE_AUX)
//
pub const mmPCIE_AUX_APB_TIMEOUT: c_uint = 0xC07004;
pub const mmPCIE_AUX_PHY_INIT: c_uint = 0xC07100;
pub const mmPCIE_AUX_LTR_MAX_LATENCY: c_uint = 0xC07138;
pub const mmPCIE_AUX_BAR0_START_L: c_uint = 0xC07160;
pub const mmPCIE_AUX_BAR0_START_H: c_uint = 0xC07164;
pub const mmPCIE_AUX_BAR1_START: c_uint = 0xC07168;
pub const mmPCIE_AUX_BAR2_START_L: c_uint = 0xC0716C;
pub const mmPCIE_AUX_BAR2_START_H: c_uint = 0xC07170;
pub const mmPCIE_AUX_BAR3_START: c_uint = 0xC07174;
pub const mmPCIE_AUX_BAR4_START_L: c_uint = 0xC07178;
pub const mmPCIE_AUX_BAR4_START_H: c_uint = 0xC0717C;
pub const mmPCIE_AUX_BAR5_START: c_uint = 0xC07180;
pub const mmPCIE_AUX_BAR0_LIMIT_L: c_uint = 0xC07184;
pub const mmPCIE_AUX_BAR0_LIMIT_H: c_uint = 0xC07188;
pub const mmPCIE_AUX_BAR1_LIMIT: c_uint = 0xC0718C;
pub const mmPCIE_AUX_BAR2_LIMIT_L: c_uint = 0xC07190;
pub const mmPCIE_AUX_BAR2_LIMIT_H: c_uint = 0xC07194;
pub const mmPCIE_AUX_BAR3_LIMIT: c_uint = 0xC07198;
pub const mmPCIE_AUX_BAR4_LIMIT_L: c_uint = 0xC0719C;
pub const mmPCIE_AUX_BAR4_LIMIT_H: c_uint = 0xC07200;
pub const mmPCIE_AUX_BAR5_LIMIT: c_uint = 0xC07204;
pub const mmPCIE_AUX_BUS_MASTER_EN: c_uint = 0xC07208;
pub const mmPCIE_AUX_MEM_SPACE_EN: c_uint = 0xC0720C;
pub const mmPCIE_AUX_MAX_RD_REQ_SIZE: c_uint = 0xC07210;
pub const mmPCIE_AUX_MAX_PAYLOAD_SIZE: c_uint = 0xC07214;
pub const mmPCIE_AUX_EXT_TAG_EN: c_uint = 0xC07218;
pub const mmPCIE_AUX_RCB: c_uint = 0xC0721C;
pub const mmPCIE_AUX_PM_NO_SOFT_RST: c_uint = 0xC07220;
pub const mmPCIE_AUX_PBUS_NUM: c_uint = 0xC07224;
pub const mmPCIE_AUX_PBUS_DEV_NUM: c_uint = 0xC07228;
pub const mmPCIE_AUX_NO_SNOOP_EN: c_uint = 0xC0722C;
pub const mmPCIE_AUX_RELAX_ORDER_EN: c_uint = 0xC07230;
pub const mmPCIE_AUX_HP_SLOT_CTRL_ACCESS: c_uint = 0xC07234;
pub const mmPCIE_AUX_DLL_STATE_CHGED_EN: c_uint = 0xC07238;
pub const mmPCIE_AUX_CMP_CPLED_INT_EN: c_uint = 0xC0723C;
pub const mmPCIE_AUX_HP_INT_EN: c_uint = 0xC07340;
pub const mmPCIE_AUX_PRE_DET_CHGEN_EN: c_uint = 0xC07344;
pub const mmPCIE_AUX_MRL_SENSOR_CHGED_EN: c_uint = 0xC07348;
pub const mmPCIE_AUX_PWR_FAULT_DET_EN: c_uint = 0xC0734C;
pub const mmPCIE_AUX_ATTEN_BUTTON_PRESSED_EN: c_uint = 0xC07350;
pub const mmPCIE_AUX_PF_FLR_ACTIVE: c_uint = 0xC07360;
pub const mmPCIE_AUX_PF_FLR_DONE: c_uint = 0xC07364;
pub const mmPCIE_AUX_FLR_INT: c_uint = 0xC07390;
pub const mmPCIE_AUX_LTR_M_EN: c_uint = 0xC073B0;
pub const mmPCIE_AUX_LTSSM_EN: c_uint = 0xC07428;
pub const mmPCIE_AUX_SYS_INTR: c_uint = 0xC07440;
pub const mmPCIE_AUX_INT_DISABLE: c_uint = 0xC07444;
pub const mmPCIE_AUX_SMLH_LINK_UP: c_uint = 0xC07448;
pub const mmPCIE_AUX_PM_CURR_STATE: c_uint = 0xC07450;
pub const mmPCIE_AUX_RDLH_LINK_UP: c_uint = 0xC07458;
pub const mmPCIE_AUX_BRDG_SLV_XFER_PENDING: c_uint = 0xC0745C;
pub const mmPCIE_AUX_BRDG_DBI_XFER_PENDING: c_uint = 0xC07460;
pub const mmPCIE_AUX_AUTO_SP_DIS: c_uint = 0xC07478;
pub const mmPCIE_AUX_DBI: c_uint = 0xC07490;
pub const mmPCIE_AUX_DBI_32: c_uint = 0xC07494;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_0: c_uint = 0xC074A4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_1: c_uint = 0xC074A8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_2: c_uint = 0xC074AC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_3: c_uint = 0xC074B0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_4: c_uint = 0xC074B4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_5: c_uint = 0xC074B8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_6: c_uint = 0xC074BC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_7: c_uint = 0xC074C0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_8: c_uint = 0xC074C4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_9: c_uint = 0xC074C8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_10: c_uint = 0xC074CC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_11: c_uint = 0xC074D0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_12: c_uint = 0xC074D4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_13: c_uint = 0xC074D8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_14: c_uint = 0xC074DC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_15: c_uint = 0xC074E0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_16: c_uint = 0xC074E4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_17: c_uint = 0xC074E8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_18: c_uint = 0xC074EC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_19: c_uint = 0xC074F0;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_20: c_uint = 0xC074F4;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_21: c_uint = 0xC074F8;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_22: c_uint = 0xC074FC;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_23: c_uint = 0xC07500;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_24: c_uint = 0xC07504;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_25: c_uint = 0xC07508;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_26: c_uint = 0xC0750C;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_27: c_uint = 0xC07510;
pub const mmPCIE_AUX_DIAG_STATUS_BUS_28: c_uint = 0xC07514;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_0: c_uint = 0xC07640;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_1: c_uint = 0xC07644;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_2: c_uint = 0xC07648;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_3: c_uint = 0xC0764C;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_4: c_uint = 0xC07650;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_5: c_uint = 0xC07654;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_6: c_uint = 0xC07658;
pub const mmPCIE_AUX_CDM_RAS_DES_EC_INFO_7: c_uint = 0xC0765C;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_0: c_uint = 0xC07744;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_1: c_uint = 0xC07748;
pub const mmPCIE_AUX_CDM_RAS_DES_SD_COMMON_2: c_uint = 0xC0774C;
pub const mmPCIE_AUX_APP_RAS_DES_TBA_CTRL: c_uint = 0xC07774;
pub const mmPCIE_AUX_PM_DSTATE: c_uint = 0xC07840;
pub const mmPCIE_AUX_PM_PME_EN: c_uint = 0xC07844;
pub const mmPCIE_AUX_PM_LINKST_IN_L0S: c_uint = 0xC07848;
pub const mmPCIE_AUX_PM_LINKST_IN_L1: c_uint = 0xC0784C;
pub const mmPCIE_AUX_PM_LINKST_IN_L2: c_uint = 0xC07850;
pub const mmPCIE_AUX_PM_LINKST_L2_EXIT: c_uint = 0xC07854;
pub const mmPCIE_AUX_PM_STATUS: c_uint = 0xC07858;
pub const mmPCIE_AUX_APP_READY_ENTER_L23: c_uint = 0xC0785C;
pub const mmPCIE_AUX_APP_XFER_PENDING: c_uint = 0xC07860;
pub const mmPCIE_AUX_APP_REQ_L1: c_uint = 0xC07930;
pub const mmPCIE_AUX_AUX_PM_EN: c_uint = 0xC07934;
pub const mmPCIE_AUX_APPS_PM_XMT_PME: c_uint = 0xC07938;
pub const mmPCIE_AUX_OUTBAND_PWRUP_CMD: c_uint = 0xC07940;
pub const mmPCIE_AUX_PERST: c_uint = 0xC079B8;
