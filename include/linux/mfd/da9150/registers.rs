//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9150/registers.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// DA9150 MFD Driver - Registers
//
// Copyright (c) 2014 Dialog Semiconductor
//
// Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
//

// Registers
pub const DA9150_PAGE_CON: c_uint = 0x000;
pub const DA9150_STATUS_A: c_uint = 0x068;
pub const DA9150_STATUS_B: c_uint = 0x069;
pub const DA9150_STATUS_C: c_uint = 0x06A;
pub const DA9150_STATUS_D: c_uint = 0x06B;
pub const DA9150_STATUS_E: c_uint = 0x06C;
pub const DA9150_STATUS_F: c_uint = 0x06D;
pub const DA9150_STATUS_G: c_uint = 0x06E;
pub const DA9150_STATUS_H: c_uint = 0x06F;
pub const DA9150_STATUS_I: c_uint = 0x070;
pub const DA9150_STATUS_J: c_uint = 0x071;
pub const DA9150_STATUS_K: c_uint = 0x072;
pub const DA9150_STATUS_L: c_uint = 0x073;
pub const DA9150_STATUS_N: c_uint = 0x074;
pub const DA9150_FAULT_LOG_A: c_uint = 0x076;
pub const DA9150_FAULT_LOG_B: c_uint = 0x077;
pub const DA9150_EVENT_E: c_uint = 0x078;
pub const DA9150_EVENT_F: c_uint = 0x079;
pub const DA9150_EVENT_G: c_uint = 0x07A;
pub const DA9150_EVENT_H: c_uint = 0x07B;
pub const DA9150_IRQ_MASK_E: c_uint = 0x07C;
pub const DA9150_IRQ_MASK_F: c_uint = 0x07D;
pub const DA9150_IRQ_MASK_G: c_uint = 0x07E;
pub const DA9150_IRQ_MASK_H: c_uint = 0x07F;
pub const DA9150_PAGE_CON_1: c_uint = 0x080;
pub const DA9150_CONFIG_A: c_uint = 0x0E0;
pub const DA9150_CONFIG_B: c_uint = 0x0E1;
pub const DA9150_CONFIG_C: c_uint = 0x0E2;
pub const DA9150_CONFIG_D: c_uint = 0x0E3;
pub const DA9150_CONFIG_E: c_uint = 0x0E4;
pub const DA9150_CONTROL_A: c_uint = 0x0E5;
pub const DA9150_CONTROL_B: c_uint = 0x0E6;
pub const DA9150_CONTROL_C: c_uint = 0x0E7;
pub const DA9150_GPIO_A_B: c_uint = 0x0E8;
pub const DA9150_GPIO_C_D: c_uint = 0x0E9;
pub const DA9150_GPIO_MODE_CONT: c_uint = 0x0EA;
pub const DA9150_GPIO_CTRL_B: c_uint = 0x0EB;
pub const DA9150_GPIO_CTRL_A: c_uint = 0x0EC;
pub const DA9150_GPIO_CTRL_C: c_uint = 0x0ED;
pub const DA9150_GPIO_CFG_A: c_uint = 0x0EE;
pub const DA9150_GPIO_CFG_B: c_uint = 0x0EF;
pub const DA9150_GPIO_CFG_C: c_uint = 0x0F0;
pub const DA9150_GPADC_MAN: c_uint = 0x0F2;
pub const DA9150_GPADC_RES_A: c_uint = 0x0F4;
pub const DA9150_GPADC_RES_B: c_uint = 0x0F5;
pub const DA9150_PAGE_CON_2: c_uint = 0x100;
pub const DA9150_OTP_CONT_SHARED: c_uint = 0x101;
pub const DA9150_INTERFACE_SHARED: c_uint = 0x105;
pub const DA9150_CONFIG_A_SHARED: c_uint = 0x106;
pub const DA9150_CONFIG_D_SHARED: c_uint = 0x109;
pub const DA9150_ADETVB_CFG_C: c_uint = 0x150;
pub const DA9150_ADETD_STAT: c_uint = 0x151;
pub const DA9150_ADET_CMPSTAT: c_uint = 0x152;
pub const DA9150_ADET_CTRL_A: c_uint = 0x153;
pub const DA9150_ADETVB_CFG_B: c_uint = 0x154;
pub const DA9150_ADETVB_CFG_A: c_uint = 0x155;
pub const DA9150_ADETAC_CFG_A: c_uint = 0x156;
pub const DA9150_ADDETAC_CFG_B: c_uint = 0x157;
pub const DA9150_ADETAC_CFG_C: c_uint = 0x158;
pub const DA9150_ADETAC_CFG_D: c_uint = 0x159;
pub const DA9150_ADETVB_CFG_D: c_uint = 0x15A;
pub const DA9150_ADETID_CFG_A: c_uint = 0x15B;
pub const DA9150_ADET_RID_PT_CHG_H: c_uint = 0x15C;
pub const DA9150_ADET_RID_PT_CHG_L: c_uint = 0x15D;
pub const DA9150_PPR_TCTR_B: c_uint = 0x160;
pub const DA9150_PPR_BKCTRL_A: c_uint = 0x163;
pub const DA9150_PPR_BKCFG_A: c_uint = 0x164;
pub const DA9150_PPR_BKCFG_B: c_uint = 0x165;
pub const DA9150_PPR_CHGCTRL_A: c_uint = 0x166;
pub const DA9150_PPR_CHGCTRL_B: c_uint = 0x167;
pub const DA9150_PPR_CHGCTRL_C: c_uint = 0x168;
pub const DA9150_PPR_TCTR_A: c_uint = 0x169;
pub const DA9150_PPR_CHGCTRL_D: c_uint = 0x16A;
pub const DA9150_PPR_CHGCTRL_E: c_uint = 0x16B;
pub const DA9150_PPR_CHGCTRL_F: c_uint = 0x16C;
pub const DA9150_PPR_CHGCTRL_G: c_uint = 0x16D;
pub const DA9150_PPR_CHGCTRL_H: c_uint = 0x16E;
pub const DA9150_PPR_CHGCTRL_I: c_uint = 0x16F;
pub const DA9150_PPR_CHGCTRL_J: c_uint = 0x170;
pub const DA9150_PPR_CHGCTRL_K: c_uint = 0x171;
pub const DA9150_PPR_CHGCTRL_L: c_uint = 0x172;
pub const DA9150_PPR_CHGCTRL_M: c_uint = 0x173;
pub const DA9150_PPR_THYST_A: c_uint = 0x174;
pub const DA9150_PPR_THYST_B: c_uint = 0x175;
pub const DA9150_PPR_THYST_C: c_uint = 0x176;
pub const DA9150_PPR_THYST_D: c_uint = 0x177;
pub const DA9150_PPR_THYST_E: c_uint = 0x178;
pub const DA9150_PPR_THYST_F: c_uint = 0x179;
pub const DA9150_PPR_THYST_G: c_uint = 0x17A;
pub const DA9150_PAGE_CON_3: c_uint = 0x180;
pub const DA9150_PAGE_CON_4: c_uint = 0x200;
pub const DA9150_PAGE_CON_5: c_uint = 0x280;
pub const DA9150_PAGE_CON_6: c_uint = 0x300;
pub const DA9150_COREBTLD_STAT_A: c_uint = 0x302;
pub const DA9150_COREBTLD_CTRL_A: c_uint = 0x303;
pub const DA9150_CORE_CONFIG_A: c_uint = 0x304;
pub const DA9150_CORE_CONFIG_C: c_uint = 0x305;
pub const DA9150_CORE_CONFIG_B: c_uint = 0x306;
pub const DA9150_CORE_CFG_DATA_A: c_uint = 0x307;
pub const DA9150_CORE_CFG_DATA_B: c_uint = 0x308;
pub const DA9150_CORE_CMD_A: c_uint = 0x309;
pub const DA9150_CORE_DATA_A: c_uint = 0x30A;
pub const DA9150_CORE_DATA_B: c_uint = 0x30B;
pub const DA9150_CORE_DATA_C: c_uint = 0x30C;
pub const DA9150_CORE_DATA_D: c_uint = 0x30D;
pub const DA9150_CORE2WIRE_STAT_A: c_uint = 0x310;
pub const DA9150_CORE2WIRE_CTRL_A: c_uint = 0x311;
pub const DA9150_FW_CTRL_A: c_uint = 0x312;
pub const DA9150_FW_CTRL_C: c_uint = 0x313;
pub const DA9150_FW_CTRL_D: c_uint = 0x314;
pub const DA9150_FG_CTRL_A: c_uint = 0x315;
pub const DA9150_FG_CTRL_B: c_uint = 0x316;
pub const DA9150_FW_CTRL_E: c_uint = 0x317;
pub const DA9150_FW_CTRL_B: c_uint = 0x318;
pub const DA9150_GPADC_CMAN: c_uint = 0x320;
pub const DA9150_GPADC_CRES_A: c_uint = 0x322;
pub const DA9150_GPADC_CRES_B: c_uint = 0x323;
pub const DA9150_CC_CFG_A: c_uint = 0x328;
pub const DA9150_CC_CFG_B: c_uint = 0x329;
pub const DA9150_CC_ICHG_RES_A: c_uint = 0x32A;
pub const DA9150_CC_ICHG_RES_B: c_uint = 0x32B;
pub const DA9150_CC_IAVG_RES_A: c_uint = 0x32C;
pub const DA9150_CC_IAVG_RES_B: c_uint = 0x32D;
pub const DA9150_TAUX_CTRL_A: c_uint = 0x330;
pub const DA9150_TAUX_RELOAD_H: c_uint = 0x332;
pub const DA9150_TAUX_RELOAD_L: c_uint = 0x333;
pub const DA9150_TAUX_VALUE_H: c_uint = 0x334;
pub const DA9150_TAUX_VALUE_L: c_uint = 0x335;
pub const DA9150_AUX_DATA_0: c_uint = 0x338;
pub const DA9150_AUX_DATA_1: c_uint = 0x339;
pub const DA9150_AUX_DATA_2: c_uint = 0x33A;
pub const DA9150_AUX_DATA_3: c_uint = 0x33B;
pub const DA9150_BIF_CTRL: c_uint = 0x340;
pub const DA9150_TBAT_CTRL_A: c_uint = 0x342;
pub const DA9150_TBAT_CTRL_B: c_uint = 0x343;
pub const DA9150_TBAT_RES_A: c_uint = 0x344;
pub const DA9150_TBAT_RES_B: c_uint = 0x345;
// DA9150_PAGE_CON = 0x000
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_I2C_PAGE_SHIFT: c_int = 1;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_STATUS_A = 0x068
pub const DA9150_WKUP_STAT_SHIFT: c_int = 2;

pub const DA9150_SLEEP_STAT_SHIFT: c_int = 6;

// DA9150_STATUS_B = 0x069
pub const DA9150_VFAULT_STAT_SHIFT: c_int = 0;

pub const DA9150_TFAULT_STAT_SHIFT: c_int = 1;

// DA9150_STATUS_C = 0x06A
pub const DA9150_VDD33_STAT_SHIFT: c_int = 0;

pub const DA9150_VDD33_SLEEP_SHIFT: c_int = 1;

pub const DA9150_LFOSC_STAT_SHIFT: c_int = 7;

// DA9150_STATUS_D = 0x06B
pub const DA9150_GPIOA_STAT_SHIFT: c_int = 0;

pub const DA9150_GPIOB_STAT_SHIFT: c_int = 1;

pub const DA9150_GPIOC_STAT_SHIFT: c_int = 2;

pub const DA9150_GPIOD_STAT_SHIFT: c_int = 3;

// DA9150_STATUS_E = 0x06C
pub const DA9150_DTYPE_SHIFT: c_int = 0;

// DA9150_STATUS_F = 0x06D
pub const DA9150_SESS_VLD_SHIFT: c_int = 0;

pub const DA9150_ID_ERR_SHIFT: c_int = 1;

pub const DA9150_PT_CHG_SHIFT: c_int = 2;

// DA9150_STATUS_G = 0x06E
pub const DA9150_RID_SHIFT: c_int = 0;

// DA9150_STATUS_H = 0x06F
pub const DA9150_VBUS_STAT_SHIFT: c_int = 0;

pub const DA9150_VBUS_TRED_SHIFT: c_int = 3;

pub const DA9150_VBUS_DROP_STAT_SHIFT: c_int = 4;

// DA9150_STATUS_I = 0x070
pub const DA9150_VBUS_ISET_STAT_SHIFT: c_int = 0;

pub const DA9150_VBUS_OT_SHIFT: c_int = 7;

// DA9150_STATUS_J = 0x071
pub const DA9150_CHG_STAT_SHIFT: c_int = 0;

pub const DA9150_CHG_TEMP_SHIFT: c_int = 4;

pub const DA9150_CHG_IEND_STAT_SHIFT: c_int = 7;

// DA9150_STATUS_K = 0x072
pub const DA9150_CHG_IAV_H_SHIFT: c_int = 0;

// DA9150_STATUS_L = 0x073
pub const DA9150_CHG_IAV_L_SHIFT: c_int = 5;

// DA9150_STATUS_N = 0x074
pub const DA9150_CHG_TIME_SHIFT: c_int = 1;

pub const DA9150_CHG_TRED_SHIFT: c_int = 2;

pub const DA9150_CHG_TJUNC_CLASS_SHIFT: c_int = 3;

pub const DA9150_EBS_STAT_SHIFT: c_int = 6;

pub const DA9150_CHG_BAT_REMOVED_SHIFT: c_int = 7;

// DA9150_FAULT_LOG_A = 0x076
pub const DA9150_TEMP_FAULT_SHIFT: c_int = 0;

pub const DA9150_VSYS_FAULT_SHIFT: c_int = 1;

pub const DA9150_START_FAULT_SHIFT: c_int = 2;

pub const DA9150_EXT_FAULT_SHIFT: c_int = 3;

pub const DA9150_POR_FAULT_SHIFT: c_int = 4;

// DA9150_FAULT_LOG_B = 0x077
pub const DA9150_VBUS_FAULT_SHIFT: c_int = 0;

pub const DA9150_OTG_FAULT_SHIFT: c_int = 1;

// DA9150_EVENT_E = 0x078
pub const DA9150_E_VBUS_SHIFT: c_int = 0;

pub const DA9150_E_CHG_SHIFT: c_int = 1;

pub const DA9150_E_TCLASS_SHIFT: c_int = 2;

pub const DA9150_E_TJUNC_SHIFT: c_int = 3;

pub const DA9150_E_VFAULT_SHIFT: c_int = 4;

pub const DA9150_EVENTS_H_SHIFT: c_int = 5;

pub const DA9150_EVENTS_G_SHIFT: c_int = 6;

pub const DA9150_EVENTS_F_SHIFT: c_int = 7;

// DA9150_EVENT_F = 0x079
pub const DA9150_E_CONF_SHIFT: c_int = 0;

pub const DA9150_E_DAT_SHIFT: c_int = 1;

pub const DA9150_E_DTYPE_SHIFT: c_int = 3;

pub const DA9150_E_ID_SHIFT: c_int = 4;

pub const DA9150_E_ADP_SHIFT: c_int = 5;

pub const DA9150_E_SESS_END_SHIFT: c_int = 6;

pub const DA9150_E_SESS_VLD_SHIFT: c_int = 7;

// DA9150_EVENT_G = 0x07A
pub const DA9150_E_FG_SHIFT: c_int = 0;

pub const DA9150_E_GP_SHIFT: c_int = 1;

pub const DA9150_E_TBAT_SHIFT: c_int = 2;

pub const DA9150_E_GPIOA_SHIFT: c_int = 3;

pub const DA9150_E_GPIOB_SHIFT: c_int = 4;

pub const DA9150_E_GPIOC_SHIFT: c_int = 5;

pub const DA9150_E_GPIOD_SHIFT: c_int = 6;

pub const DA9150_E_GPADC_SHIFT: c_int = 7;

// DA9150_EVENT_H = 0x07B
pub const DA9150_E_WKUP_SHIFT: c_int = 0;

// DA9150_IRQ_MASK_E = 0x07C
pub const DA9150_M_VBUS_SHIFT: c_int = 0;

pub const DA9150_M_CHG_SHIFT: c_int = 1;

pub const DA9150_M_TJUNC_SHIFT: c_int = 3;

pub const DA9150_M_VFAULT_SHIFT: c_int = 4;

// DA9150_IRQ_MASK_F = 0x07D
pub const DA9150_M_CONF_SHIFT: c_int = 0;

pub const DA9150_M_DAT_SHIFT: c_int = 1;

pub const DA9150_M_DTYPE_SHIFT: c_int = 3;

pub const DA9150_M_ID_SHIFT: c_int = 4;

pub const DA9150_M_ADP_SHIFT: c_int = 5;

pub const DA9150_M_SESS_END_SHIFT: c_int = 6;

pub const DA9150_M_SESS_VLD_SHIFT: c_int = 7;

// DA9150_IRQ_MASK_G = 0x07E
pub const DA9150_M_FG_SHIFT: c_int = 0;

pub const DA9150_M_GP_SHIFT: c_int = 1;

pub const DA9150_M_TBAT_SHIFT: c_int = 2;

pub const DA9150_M_GPIOA_SHIFT: c_int = 3;

pub const DA9150_M_GPIOB_SHIFT: c_int = 4;

pub const DA9150_M_GPIOC_SHIFT: c_int = 5;

pub const DA9150_M_GPIOD_SHIFT: c_int = 6;

pub const DA9150_M_GPADC_SHIFT: c_int = 7;

// DA9150_IRQ_MASK_H = 0x07F
pub const DA9150_M_WKUP_SHIFT: c_int = 0;

// DA9150_PAGE_CON_1 = 0x080
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_CONFIG_A = 0x0E0
pub const DA9150_RESET_DUR_SHIFT: c_int = 0;

pub const DA9150_RESET_EXT_SHIFT: c_int = 2;

pub const DA9150_START_MAX_SHIFT: c_int = 4;

pub const DA9150_PS_WAIT_EN_SHIFT: c_int = 6;

pub const DA9150_PS_DISABLE_DIRECT_SHIFT: c_int = 7;

// DA9150_CONFIG_B = 0x0E1
pub const DA9150_VFAULT_ADJ_SHIFT: c_int = 0;

pub const DA9150_VFAULT_HYST_SHIFT: c_int = 4;

pub const DA9150_VFAULT_EN_SHIFT: c_int = 7;

// DA9150_CONFIG_C = 0x0E2
pub const DA9150_VSYS_MIN_SHIFT: c_int = 3;

// DA9150_CONFIG_D = 0x0E3
pub const DA9150_LFOSC_EXT_SHIFT: c_int = 0;

pub const DA9150_VDD33_DWN_SHIFT: c_int = 1;

pub const DA9150_WKUP_PM_EN_SHIFT: c_int = 2;

pub const DA9150_WKUP_CE_SEL_SHIFT: c_int = 3;

pub const DA9150_WKUP_CLK32K_EN_SHIFT: c_int = 5;

pub const DA9150_DISABLE_DEL_SHIFT: c_int = 7;

// DA9150_CONFIG_E = 0x0E4
pub const DA9150_PM_SPKSUP_DIS_SHIFT: c_int = 0;

pub const DA9150_PM_MERGE_SHIFT: c_int = 1;

pub const DA9150_PM_SR_OFF_SHIFT: c_int = 2;

pub const DA9150_PM_TIMEOUT_EN_SHIFT: c_int = 3;

pub const DA9150_PM_DLY_SEL_SHIFT: c_int = 4;

pub const DA9150_PM_OUT_DLY_SEL_SHIFT: c_int = 7;

// DA9150_CONTROL_A = 0x0E5
pub const DA9150_VDD33_SL_SHIFT: c_int = 0;

pub const DA9150_VDD33_LPM_SHIFT: c_int = 1;

pub const DA9150_VDD33_EN_SHIFT: c_int = 3;

pub const DA9150_GPI_LPM_SHIFT: c_int = 6;

pub const DA9150_PM_IF_LPM_SHIFT: c_int = 7;

// DA9150_CONTROL_B = 0x0E6
pub const DA9150_LPM_SHIFT: c_int = 0;

pub const DA9150_RESET_SHIFT: c_int = 1;

pub const DA9150_RESET_USRCONF_EN_SHIFT: c_int = 2;

// DA9150_CONTROL_C = 0x0E7
pub const DA9150_DISABLE_SHIFT: c_int = 0;

// DA9150_GPIO_A_B = 0x0E8
pub const DA9150_GPIOA_PIN_SHIFT: c_int = 0;

pub const DA9150_GPIOA_TYPE_SHIFT: c_int = 3;

pub const DA9150_GPIOB_PIN_SHIFT: c_int = 4;

pub const DA9150_GPIOB_TYPE_SHIFT: c_int = 7;

// DA9150_GPIO_C_D = 0x0E9
pub const DA9150_GPIOC_PIN_SHIFT: c_int = 0;

pub const DA9150_GPIOC_TYPE_SHIFT: c_int = 3;

pub const DA9150_GPIOD_PIN_SHIFT: c_int = 4;

pub const DA9150_GPIOD_TYPE_SHIFT: c_int = 7;

// DA9150_GPIO_MODE_CONT = 0x0EA
pub const DA9150_GPIOA_MODE_SHIFT: c_int = 0;

pub const DA9150_GPIOB_MODE_SHIFT: c_int = 1;

pub const DA9150_GPIOC_MODE_SHIFT: c_int = 2;

pub const DA9150_GPIOD_MODE_SHIFT: c_int = 3;

pub const DA9150_GPIOA_CONT_SHIFT: c_int = 4;

pub const DA9150_GPIOB_CONT_SHIFT: c_int = 5;

pub const DA9150_GPIOC_CONT_SHIFT: c_int = 6;

pub const DA9150_GPIOD_CONT_SHIFT: c_int = 7;

// DA9150_GPIO_CTRL_B = 0x0EB
pub const DA9150_WAKE_PIN_SHIFT: c_int = 0;

pub const DA9150_WAKE_MODE_SHIFT: c_int = 2;

pub const DA9150_WAKE_CONT_SHIFT: c_int = 3;

pub const DA9150_WAKE_DLY_SHIFT: c_int = 4;

// DA9150_GPIO_CTRL_A = 0x0EC
pub const DA9150_GPIOA_ANAEN_SHIFT: c_int = 0;

pub const DA9150_GPIOB_ANAEN_SHIFT: c_int = 1;

pub const DA9150_GPIOC_ANAEN_SHIFT: c_int = 2;

pub const DA9150_GPIOD_ANAEN_SHIFT: c_int = 3;

pub const DA9150_GPIO_ANAEN: c_uint = 0x01;
pub const DA9150_GPIO_ANAEN_MASK: c_uint = 0x0F;
pub const DA9150_CHGLED_PIN_SHIFT: c_int = 5;

// DA9150_GPIO_CTRL_C = 0x0ED
pub const DA9150_CHGBL_DUR_SHIFT: c_int = 0;

pub const DA9150_CHGBL_DBL_SHIFT: c_int = 2;

pub const DA9150_CHGBL_FRQ_SHIFT: c_int = 3;

pub const DA9150_CHGBL_FLKR_SHIFT: c_int = 5;

// DA9150_GPIO_CFG_A = 0x0EE
pub const DA9150_CE_LPM_DEB_SHIFT: c_int = 0;

// DA9150_GPIO_CFG_B = 0x0EF
pub const DA9150_GPIOA_PUPD_SHIFT: c_int = 0;

pub const DA9150_GPIOB_PUPD_SHIFT: c_int = 1;

pub const DA9150_GPIOC_PUPD_SHIFT: c_int = 2;

pub const DA9150_GPIOD_PUPD_SHIFT: c_int = 3;

pub const DA9150_GPI_DEB_SHIFT: c_int = 4;

pub const DA9150_LPM_EN_SHIFT: c_int = 7;

// DA9150_GPIO_CFG_C = 0x0F0
pub const DA9150_GPI_V_SHIFT: c_int = 0;

pub const DA9150_VDDIO_INT_SHIFT: c_int = 1;

pub const DA9150_FAULT_PIN_SHIFT: c_int = 3;

pub const DA9150_FAULT_TYPE_SHIFT: c_int = 6;

pub const DA9150_NIRQ_PUPD_SHIFT: c_int = 7;

// DA9150_GPADC_MAN = 0x0F2
pub const DA9150_GPADC_EN_SHIFT: c_int = 0;

pub const DA9150_GPADC_MUX_SHIFT: c_int = 1;

// DA9150_GPADC_RES_A = 0x0F4
pub const DA9150_GPADC_RES_H_SHIFT: c_int = 0;

// DA9150_GPADC_RES_B = 0x0F5
pub const DA9150_GPADC_RUN_SHIFT: c_int = 0;

pub const DA9150_GPADC_RES_L_SHIFT: c_int = 6;

pub const DA9150_GPADC_RES_L_BITS: c_int = 2;
// DA9150_PAGE_CON_2 = 0x100
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_OTP_CONT_SHARED = 0x101
pub const DA9150_PC_DONE_SHIFT: c_int = 3;

// DA9150_INTERFACE_SHARED = 0x105
pub const DA9150_IF_BASE_ADDR_SHIFT: c_int = 4;

// DA9150_CONFIG_A_SHARED = 0x106
pub const DA9150_NIRQ_VDD_SHIFT: c_int = 1;

pub const DA9150_NIRQ_PIN_SHIFT: c_int = 2;

pub const DA9150_NIRQ_TYPE_SHIFT: c_int = 3;

pub const DA9150_PM_IF_V_SHIFT: c_int = 4;

pub const DA9150_PM_IF_FMP_SHIFT: c_int = 5;

pub const DA9150_PM_IF_HSM_SHIFT: c_int = 6;

// DA9150_CONFIG_D_SHARED = 0x109
pub const DA9150_NIRQ_MODE_SHIFT: c_int = 1;

// DA9150_ADETVB_CFG_C = 0x150
pub const DA9150_TADP_RISE_SHIFT: c_int = 0;

// DA9150_ADETD_STAT = 0x151
pub const DA9150_DCD_STAT_SHIFT: c_int = 0;

pub const DA9150_PCD_STAT_SHIFT: c_int = 1;

pub const DA9150_SCD_STAT_SHIFT: c_int = 3;

pub const DA9150_DP_STAT_SHIFT: c_int = 5;

pub const DA9150_DM_STAT_SHIFT: c_int = 6;

// DA9150_ADET_CMPSTAT = 0x152
pub const DA9150_DP_COMP_SHIFT: c_int = 1;

pub const DA9150_DM_COMP_SHIFT: c_int = 2;

pub const DA9150_ADP_SNS_COMP_SHIFT: c_int = 3;

pub const DA9150_ADP_PRB_COMP_SHIFT: c_int = 4;

pub const DA9150_ID_COMP_SHIFT: c_int = 5;

// DA9150_ADET_CTRL_A = 0x153
pub const DA9150_AID_DAT_SHIFT: c_int = 0;

pub const DA9150_AID_ID_SHIFT: c_int = 1;

pub const DA9150_AID_TRIG_SHIFT: c_int = 2;

// DA9150_ADETVB_CFG_B = 0x154
pub const DA9150_VB_MODE_SHIFT: c_int = 0;

pub const DA9150_TADP_PRB_SHIFT: c_int = 2;

pub const DA9150_DAT_RPD_EXT_SHIFT: c_int = 5;

pub const DA9150_CONF_RPD_SHIFT: c_int = 6;

pub const DA9150_CONF_SRP_SHIFT: c_int = 7;

// DA9150_ADETVB_CFG_A = 0x155
pub const DA9150_AID_MODE_SHIFT: c_int = 0;

pub const DA9150_AID_EXT_POL_SHIFT: c_int = 2;

// DA9150_ADETAC_CFG_A = 0x156
pub const DA9150_ISET_CDP_SHIFT: c_int = 0;

pub const DA9150_CONF_DBP_SHIFT: c_int = 5;

// DA9150_ADDETAC_CFG_B = 0x157
pub const DA9150_ISET_DCHG_SHIFT: c_int = 0;

pub const DA9150_CONF_GPIOA_SHIFT: c_int = 5;

pub const DA9150_CONF_GPIOB_SHIFT: c_int = 6;

pub const DA9150_AID_VB_SHIFT: c_int = 7;

// DA9150_ADETAC_CFG_C = 0x158
pub const DA9150_ISET_DEF_SHIFT: c_int = 0;

pub const DA9150_CONF_MODE_SHIFT: c_int = 5;

pub const DA9150_AID_CR_DIS_SHIFT: c_int = 7;

// DA9150_ADETAC_CFG_D = 0x159
pub const DA9150_ISET_UNIT_SHIFT: c_int = 0;

pub const DA9150_AID_UNCLAMP_SHIFT: c_int = 5;

// DA9150_ADETVB_CFG_D = 0x15A
pub const DA9150_ID_MODE_SHIFT: c_int = 0;

pub const DA9150_DAT_MODE_SHIFT: c_int = 2;

pub const DA9150_DAT_SWP_SHIFT: c_int = 6;

pub const DA9150_DAT_CLAMP_EXT_SHIFT: c_int = 7;

// DA9150_ADETID_CFG_A = 0x15B
pub const DA9150_TID_POLL_SHIFT: c_int = 0;

pub const DA9150_RID_CONV_SHIFT: c_int = 3;

// DA9150_ADET_RID_PT_CHG_H = 0x15C
pub const DA9150_RID_PT_CHG_H_SHIFT: c_int = 0;

// DA9150_ADET_RID_PT_CHG_L = 0x15D
pub const DA9150_RID_PT_CHG_L_SHIFT: c_int = 6;

// DA9150_PPR_TCTR_B = 0x160
pub const DA9150_CHG_TCTR_VAL_SHIFT: c_int = 0;

// DA9150_PPR_BKCTRL_A = 0x163
pub const DA9150_VBUS_MODE_SHIFT: c_int = 0;

pub const DA9150_VBUS_LPM_SHIFT: c_int = 2;

pub const DA9150_VBUS_SUSP_SHIFT: c_int = 4;

pub const DA9150_VBUS_PWM_SHIFT: c_int = 5;

pub const DA9150_VBUS_ISO_SHIFT: c_int = 6;

pub const DA9150_VBUS_LDO_SHIFT: c_int = 7;

// DA9150_PPR_BKCFG_A = 0x164
pub const DA9150_VBUS_ISET_SHIFT: c_int = 0;

pub const DA9150_VBUS_IMAX_SHIFT: c_int = 5;

pub const DA9150_VBUS_IOTG_SHIFT: c_int = 6;

// DA9150_PPR_BKCFG_B = 0x165
pub const DA9150_VBUS_DROP_SHIFT: c_int = 0;

pub const DA9150_VBUS_FAULT_DIS_SHIFT: c_int = 6;

pub const DA9150_OTG_FAULT_DIS_SHIFT: c_int = 7;

// DA9150_PPR_CHGCTRL_A = 0x166
pub const DA9150_CHG_EN_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_B = 0x167
pub const DA9150_CHG_VBAT_SHIFT: c_int = 0;

pub const DA9150_CHG_VDROP_SHIFT: c_int = 6;

// DA9150_PPR_CHGCTRL_C = 0x168
pub const DA9150_CHG_VFAULT_SHIFT: c_int = 0;

pub const DA9150_CHG_IPRE_SHIFT: c_int = 4;

// DA9150_PPR_TCTR_A = 0x169
pub const DA9150_CHG_TCTR_SHIFT: c_int = 0;

pub const DA9150_CHG_TCTR_MODE_SHIFT: c_int = 4;

// DA9150_PPR_CHGCTRL_D = 0x16A
pub const DA9150_CHG_IBAT_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_E = 0x16B
pub const DA9150_CHG_IEND_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_F = 0x16C
pub const DA9150_CHG_VCOLD_SHIFT: c_int = 0;

pub const DA9150_TBAT_TQA_EN_SHIFT: c_int = 6;

pub const DA9150_TBAT_TDP_EN_SHIFT: c_int = 7;

// DA9150_PPR_CHGCTRL_G = 0x16D
pub const DA9150_CHG_VWARM_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_H = 0x16E
pub const DA9150_CHG_VHOT_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_I = 0x16F
pub const DA9150_CHG_ICOLD_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_J = 0x170
pub const DA9150_CHG_IWARM_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_K = 0x171
pub const DA9150_CHG_IHOT_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_L = 0x172
pub const DA9150_CHG_IBAT_TRED_SHIFT: c_int = 0;

// DA9150_PPR_CHGCTRL_M = 0x173
pub const DA9150_CHG_VFLOAT_SHIFT: c_int = 0;

pub const DA9150_CHG_LPM_SHIFT: c_int = 5;

pub const DA9150_CHG_NBLO_SHIFT: c_int = 6;

pub const DA9150_EBS_EN_SHIFT: c_int = 7;

// DA9150_PPR_THYST_A = 0x174
pub const DA9150_TBAT_T1_SHIFT: c_int = 0;

// DA9150_PPR_THYST_B = 0x175
pub const DA9150_TBAT_T2_SHIFT: c_int = 0;

// DA9150_PPR_THYST_C = 0x176
pub const DA9150_TBAT_T3_SHIFT: c_int = 0;

// DA9150_PPR_THYST_D = 0x177
pub const DA9150_TBAT_T4_SHIFT: c_int = 0;

// DA9150_PPR_THYST_E = 0x178
pub const DA9150_TBAT_T5_SHIFT: c_int = 0;

// DA9150_PPR_THYST_F = 0x179
pub const DA9150_TBAT_H1_SHIFT: c_int = 0;

// DA9150_PPR_THYST_G = 0x17A
pub const DA9150_TBAT_H5_SHIFT: c_int = 0;

// DA9150_PAGE_CON_3 = 0x180
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_PAGE_CON_4 = 0x200
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_PAGE_CON_5 = 0x280
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_PAGE_CON_6 = 0x300
pub const DA9150_PAGE_SHIFT: c_int = 0;

pub const DA9150_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9150_REVERT_SHIFT: c_int = 7;

// DA9150_COREBTLD_STAT_A = 0x302
pub const DA9150_BOOTLD_STAT_SHIFT: c_int = 0;

pub const DA9150_CORE_LOCKUP_SHIFT: c_int = 2;

// DA9150_COREBTLD_CTRL_A = 0x303
pub const DA9150_CORE_RESET_SHIFT: c_int = 0;

pub const DA9150_CORE_STOP_SHIFT: c_int = 1;

// DA9150_CORE_CONFIG_A = 0x304
pub const DA9150_CORE_MEMMUX_SHIFT: c_int = 0;

pub const DA9150_WDT_AUTO_START_SHIFT: c_int = 2;

pub const DA9150_WDT_AUTO_LOCK_SHIFT: c_int = 3;

pub const DA9150_WDT_HLT_NO_CLK_SHIFT: c_int = 4;

// DA9150_CORE_CONFIG_C = 0x305
pub const DA9150_CORE_SW_SIZE_SHIFT: c_int = 0;

// DA9150_CORE_CONFIG_B = 0x306
pub const DA9150_BOOTLD_EN_SHIFT: c_int = 0;

pub const DA9150_CORE_EN_SHIFT: c_int = 2;

pub const DA9150_CORE_SW_SRC_SHIFT: c_int = 3;

pub const DA9150_DEEP_SLEEP_EN_SHIFT: c_int = 7;

// DA9150_CORE_CFG_DATA_A = 0x307
pub const DA9150_CORE_CFG_DT_A_SHIFT: c_int = 0;

// DA9150_CORE_CFG_DATA_B = 0x308
pub const DA9150_CORE_CFG_DT_B_SHIFT: c_int = 0;

// DA9150_CORE_CMD_A = 0x309
pub const DA9150_CORE_CMD_SHIFT: c_int = 0;

// DA9150_CORE_DATA_A = 0x30A
pub const DA9150_CORE_DATA_0_SHIFT: c_int = 0;

// DA9150_CORE_DATA_B = 0x30B
pub const DA9150_CORE_DATA_1_SHIFT: c_int = 0;

// DA9150_CORE_DATA_C = 0x30C
pub const DA9150_CORE_DATA_2_SHIFT: c_int = 0;

// DA9150_CORE_DATA_D = 0x30D
pub const DA9150_CORE_DATA_3_SHIFT: c_int = 0;

// DA9150_CORE2WIRE_STAT_A = 0x310
pub const DA9150_FW_FWDL_ERR_SHIFT: c_int = 7;

// DA9150_CORE2WIRE_CTRL_A = 0x311
pub const DA9150_FW_FWDL_EN_SHIFT: c_int = 0;

pub const DA9150_FG_QIF_EN_SHIFT: c_int = 1;

pub const DA9150_CORE_BASE_ADDR_SHIFT: c_int = 4;

// DA9150_FW_CTRL_A = 0x312
pub const DA9150_FW_SEAL_SHIFT: c_int = 0;

// DA9150_FW_CTRL_C = 0x313
pub const DA9150_FW_FWDL_CRC_SHIFT: c_int = 0;

// DA9150_FW_CTRL_D = 0x314
pub const DA9150_FW_FWDL_BASE_SHIFT: c_int = 0;

// DA9150_FG_CTRL_A = 0x315
pub const DA9150_FG_QIF_CODE_SHIFT: c_int = 0;

// DA9150_FG_CTRL_B = 0x316
pub const DA9150_FG_QIF_VALUE_SHIFT: c_int = 0;

// DA9150_FW_CTRL_E = 0x317
pub const DA9150_FW_FWDL_SEG_SHIFT: c_int = 0;

// DA9150_FW_CTRL_B = 0x318
pub const DA9150_FW_FWDL_VALUE_SHIFT: c_int = 0;

// DA9150_GPADC_CMAN = 0x320
pub const DA9150_GPADC_CEN_SHIFT: c_int = 0;

pub const DA9150_GPADC_CMUX_SHIFT: c_int = 1;

// DA9150_GPADC_CRES_A = 0x322
pub const DA9150_GPADC_CRES_H_SHIFT: c_int = 0;

// DA9150_GPADC_CRES_B = 0x323
pub const DA9150_GPADC_CRUN_SHIFT: c_int = 0;

pub const DA9150_GPADC_CRES_L_SHIFT: c_int = 6;

// DA9150_CC_CFG_A = 0x328
pub const DA9150_CC_EN_SHIFT: c_int = 0;

pub const DA9150_CC_TIMEBASE_SHIFT: c_int = 1;

pub const DA9150_CC_CFG_SHIFT: c_int = 5;

pub const DA9150_CC_ENDLESS_MODE_SHIFT: c_int = 7;

// DA9150_CC_CFG_B = 0x329
pub const DA9150_CC_OPT_SHIFT: c_int = 0;

pub const DA9150_CC_PREAMP_SHIFT: c_int = 2;

// DA9150_CC_ICHG_RES_A = 0x32A
pub const DA9150_CC_ICHG_RES_H_SHIFT: c_int = 0;

// DA9150_CC_ICHG_RES_B = 0x32B
pub const DA9150_CC_ICHG_RES_L_SHIFT: c_int = 3;

// DA9150_CC_IAVG_RES_A = 0x32C
pub const DA9150_CC_IAVG_RES_H_SHIFT: c_int = 0;

// DA9150_CC_IAVG_RES_B = 0x32D
pub const DA9150_CC_IAVG_RES_L_SHIFT: c_int = 0;

// DA9150_TAUX_CTRL_A = 0x330
pub const DA9150_TAUX_EN_SHIFT: c_int = 0;

pub const DA9150_TAUX_MOD_SHIFT: c_int = 1;

pub const DA9150_TAUX_UPDATE_SHIFT: c_int = 2;

// DA9150_TAUX_RELOAD_H = 0x332
pub const DA9150_TAUX_RLD_H_SHIFT: c_int = 0;

// DA9150_TAUX_RELOAD_L = 0x333
pub const DA9150_TAUX_RLD_L_SHIFT: c_int = 3;

// DA9150_TAUX_VALUE_H = 0x334
pub const DA9150_TAUX_VAL_H_SHIFT: c_int = 0;

// DA9150_TAUX_VALUE_L = 0x335
pub const DA9150_TAUX_VAL_L_SHIFT: c_int = 3;

// DA9150_AUX_DATA_0 = 0x338
pub const DA9150_AUX_DAT_0_SHIFT: c_int = 0;

// DA9150_AUX_DATA_1 = 0x339
pub const DA9150_AUX_DAT_1_SHIFT: c_int = 0;

// DA9150_AUX_DATA_2 = 0x33A
pub const DA9150_AUX_DAT_2_SHIFT: c_int = 0;

// DA9150_AUX_DATA_3 = 0x33B
pub const DA9150_AUX_DAT_3_SHIFT: c_int = 0;

// DA9150_BIF_CTRL = 0x340
pub const DA9150_BIF_ISRC_EN_SHIFT: c_int = 0;

// DA9150_TBAT_CTRL_A = 0x342
pub const DA9150_TBAT_EN_SHIFT: c_int = 0;

pub const DA9150_TBAT_SW1_SHIFT: c_int = 1;

pub const DA9150_TBAT_SW2_SHIFT: c_int = 2;

// DA9150_TBAT_CTRL_B = 0x343
pub const DA9150_TBAT_SW_FRC_SHIFT: c_int = 0;

pub const DA9150_TBAT_STAT_SW1_SHIFT: c_int = 1;

pub const DA9150_TBAT_STAT_SW2_SHIFT: c_int = 2;

pub const DA9150_TBAT_HIGH_CURR_SHIFT: c_int = 3;

// DA9150_TBAT_RES_A = 0x344
pub const DA9150_TBAT_RES_H_SHIFT: c_int = 0;

// DA9150_TBAT_RES_B = 0x345
pub const DA9150_TBAT_RES_DIS_SHIFT: c_int = 0;

pub const DA9150_TBAT_RES_L_SHIFT: c_int = 6;

