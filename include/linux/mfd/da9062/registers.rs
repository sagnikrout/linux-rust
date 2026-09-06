//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9062/registers.h
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
// Copyright (C) 2015-2017  Dialog Semiconductor
//
pub const DA9062_PMIC_DEVICE_ID: c_uint = 0x62;
pub const DA9062_PMIC_VARIANT_MRC_AA: c_uint = 0x01;
pub const DA9062_PMIC_VARIANT_VRC_DA9061: c_uint = 0x01;
pub const DA9062_PMIC_VARIANT_VRC_DA9062: c_uint = 0x02;
pub const DA9062_I2C_PAGE_SEL_SHIFT: c_int = 1;
//
// Registers
//
pub const DA9062AA_PAGE_CON: c_uint = 0x000;
pub const DA9062AA_STATUS_A: c_uint = 0x001;
pub const DA9062AA_STATUS_B: c_uint = 0x002;
pub const DA9062AA_STATUS_D: c_uint = 0x004;
pub const DA9062AA_FAULT_LOG: c_uint = 0x005;
pub const DA9062AA_EVENT_A: c_uint = 0x006;
pub const DA9062AA_EVENT_B: c_uint = 0x007;
pub const DA9062AA_EVENT_C: c_uint = 0x008;
pub const DA9062AA_IRQ_MASK_A: c_uint = 0x00A;
pub const DA9062AA_IRQ_MASK_B: c_uint = 0x00B;
pub const DA9062AA_IRQ_MASK_C: c_uint = 0x00C;
pub const DA9062AA_CONTROL_A: c_uint = 0x00E;
pub const DA9062AA_CONTROL_B: c_uint = 0x00F;
pub const DA9062AA_CONTROL_C: c_uint = 0x010;
pub const DA9062AA_CONTROL_D: c_uint = 0x011;
pub const DA9062AA_CONTROL_E: c_uint = 0x012;
pub const DA9062AA_CONTROL_F: c_uint = 0x013;
pub const DA9062AA_PD_DIS: c_uint = 0x014;
pub const DA9062AA_GPIO_0_1: c_uint = 0x015;
pub const DA9062AA_GPIO_2_3: c_uint = 0x016;
pub const DA9062AA_GPIO_4: c_uint = 0x017;
pub const DA9062AA_GPIO_WKUP_MODE: c_uint = 0x01C;
pub const DA9062AA_GPIO_MODE0_4: c_uint = 0x01D;
pub const DA9062AA_GPIO_OUT0_2: c_uint = 0x01E;
pub const DA9062AA_GPIO_OUT3_4: c_uint = 0x01F;
pub const DA9062AA_BUCK2_CONT: c_uint = 0x020;
pub const DA9062AA_BUCK1_CONT: c_uint = 0x021;
pub const DA9062AA_BUCK4_CONT: c_uint = 0x022;
pub const DA9062AA_BUCK3_CONT: c_uint = 0x024;
pub const DA9062AA_LDO1_CONT: c_uint = 0x026;
pub const DA9062AA_LDO2_CONT: c_uint = 0x027;
pub const DA9062AA_LDO3_CONT: c_uint = 0x028;
pub const DA9062AA_LDO4_CONT: c_uint = 0x029;
pub const DA9062AA_DVC_1: c_uint = 0x032;
pub const DA9062AA_COUNT_S: c_uint = 0x040;
pub const DA9062AA_COUNT_MI: c_uint = 0x041;
pub const DA9062AA_COUNT_H: c_uint = 0x042;
pub const DA9062AA_COUNT_D: c_uint = 0x043;
pub const DA9062AA_COUNT_MO: c_uint = 0x044;
pub const DA9062AA_COUNT_Y: c_uint = 0x045;
pub const DA9062AA_ALARM_S: c_uint = 0x046;
pub const DA9062AA_ALARM_MI: c_uint = 0x047;
pub const DA9062AA_ALARM_H: c_uint = 0x048;
pub const DA9062AA_ALARM_D: c_uint = 0x049;
pub const DA9062AA_ALARM_MO: c_uint = 0x04A;
pub const DA9062AA_ALARM_Y: c_uint = 0x04B;
pub const DA9062AA_SECOND_A: c_uint = 0x04C;
pub const DA9062AA_SECOND_B: c_uint = 0x04D;
pub const DA9062AA_SECOND_C: c_uint = 0x04E;
pub const DA9062AA_SECOND_D: c_uint = 0x04F;
pub const DA9062AA_SEQ: c_uint = 0x081;
pub const DA9062AA_SEQ_TIMER: c_uint = 0x082;
pub const DA9062AA_ID_2_1: c_uint = 0x083;
pub const DA9062AA_ID_4_3: c_uint = 0x084;
pub const DA9062AA_ID_12_11: c_uint = 0x088;
pub const DA9062AA_ID_14_13: c_uint = 0x089;
pub const DA9062AA_ID_16_15: c_uint = 0x08A;
pub const DA9062AA_ID_22_21: c_uint = 0x08D;
pub const DA9062AA_ID_24_23: c_uint = 0x08E;
pub const DA9062AA_ID_26_25: c_uint = 0x08F;
pub const DA9062AA_ID_28_27: c_uint = 0x090;
pub const DA9062AA_ID_30_29: c_uint = 0x091;
pub const DA9062AA_ID_32_31: c_uint = 0x092;
pub const DA9062AA_SEQ_A: c_uint = 0x095;
pub const DA9062AA_SEQ_B: c_uint = 0x096;
pub const DA9062AA_WAIT: c_uint = 0x097;
pub const DA9062AA_EN_32K: c_uint = 0x098;
pub const DA9062AA_RESET: c_uint = 0x099;
pub const DA9062AA_BUCK_ILIM_A: c_uint = 0x09A;
pub const DA9062AA_BUCK_ILIM_B: c_uint = 0x09B;
pub const DA9062AA_BUCK_ILIM_C: c_uint = 0x09C;
pub const DA9062AA_BUCK2_CFG: c_uint = 0x09D;
pub const DA9062AA_BUCK1_CFG: c_uint = 0x09E;
pub const DA9062AA_BUCK4_CFG: c_uint = 0x09F;
pub const DA9062AA_BUCK3_CFG: c_uint = 0x0A0;
pub const DA9062AA_VBUCK2_A: c_uint = 0x0A3;
pub const DA9062AA_VBUCK1_A: c_uint = 0x0A4;
pub const DA9062AA_VBUCK4_A: c_uint = 0x0A5;
pub const DA9062AA_VBUCK3_A: c_uint = 0x0A7;
pub const DA9062AA_VLDO1_A: c_uint = 0x0A9;
pub const DA9062AA_VLDO2_A: c_uint = 0x0AA;
pub const DA9062AA_VLDO3_A: c_uint = 0x0AB;
pub const DA9062AA_VLDO4_A: c_uint = 0x0AC;
pub const DA9062AA_VBUCK2_B: c_uint = 0x0B4;
pub const DA9062AA_VBUCK1_B: c_uint = 0x0B5;
pub const DA9062AA_VBUCK4_B: c_uint = 0x0B6;
pub const DA9062AA_VBUCK3_B: c_uint = 0x0B8;
pub const DA9062AA_VLDO1_B: c_uint = 0x0BA;
pub const DA9062AA_VLDO2_B: c_uint = 0x0BB;
pub const DA9062AA_VLDO3_B: c_uint = 0x0BC;
pub const DA9062AA_VLDO4_B: c_uint = 0x0BD;
pub const DA9062AA_BBAT_CONT: c_uint = 0x0C5;
pub const DA9062AA_INTERFACE: c_uint = 0x105;
pub const DA9062AA_CONFIG_A: c_uint = 0x106;
pub const DA9062AA_CONFIG_B: c_uint = 0x107;
pub const DA9062AA_CONFIG_C: c_uint = 0x108;
pub const DA9062AA_CONFIG_D: c_uint = 0x109;
pub const DA9062AA_CONFIG_E: c_uint = 0x10A;
pub const DA9062AA_CONFIG_G: c_uint = 0x10C;
pub const DA9062AA_CONFIG_H: c_uint = 0x10D;
pub const DA9062AA_CONFIG_I: c_uint = 0x10E;
pub const DA9062AA_CONFIG_J: c_uint = 0x10F;
pub const DA9062AA_CONFIG_K: c_uint = 0x110;
pub const DA9062AA_CONFIG_M: c_uint = 0x112;
pub const DA9062AA_TRIM_CLDR: c_uint = 0x120;
pub const DA9062AA_GP_ID_0: c_uint = 0x121;
pub const DA9062AA_GP_ID_1: c_uint = 0x122;
pub const DA9062AA_GP_ID_2: c_uint = 0x123;
pub const DA9062AA_GP_ID_3: c_uint = 0x124;
pub const DA9062AA_GP_ID_4: c_uint = 0x125;
pub const DA9062AA_GP_ID_5: c_uint = 0x126;
pub const DA9062AA_GP_ID_6: c_uint = 0x127;
pub const DA9062AA_GP_ID_7: c_uint = 0x128;
pub const DA9062AA_GP_ID_8: c_uint = 0x129;
pub const DA9062AA_GP_ID_9: c_uint = 0x12A;
pub const DA9062AA_GP_ID_10: c_uint = 0x12B;
pub const DA9062AA_GP_ID_11: c_uint = 0x12C;
pub const DA9062AA_GP_ID_12: c_uint = 0x12D;
pub const DA9062AA_GP_ID_13: c_uint = 0x12E;
pub const DA9062AA_GP_ID_14: c_uint = 0x12F;
pub const DA9062AA_GP_ID_15: c_uint = 0x130;
pub const DA9062AA_GP_ID_16: c_uint = 0x131;
pub const DA9062AA_GP_ID_17: c_uint = 0x132;
pub const DA9062AA_GP_ID_18: c_uint = 0x133;
pub const DA9062AA_GP_ID_19: c_uint = 0x134;
pub const DA9062AA_DEVICE_ID: c_uint = 0x181;
pub const DA9062AA_VARIANT_ID: c_uint = 0x182;
pub const DA9062AA_CUSTOMER_ID: c_uint = 0x183;
pub const DA9062AA_CONFIG_ID: c_uint = 0x184;
//
// Bit fields
//
// DA9062AA_PAGE_CON = 0x000
pub const DA9062AA_PAGE_SHIFT: c_int = 0;
pub const DA9062AA_PAGE_MASK: c_uint = 0x3f;
pub const DA9062AA_WRITE_MODE_SHIFT: c_int = 6;

pub const DA9062AA_REVERT_SHIFT: c_int = 7;

// DA9062AA_STATUS_A = 0x001
pub const DA9062AA_NONKEY_SHIFT: c_int = 0;
pub const DA9062AA_NONKEY_MASK: c_uint = 0x01;
pub const DA9062AA_DVC_BUSY_SHIFT: c_int = 2;

// DA9062AA_STATUS_B = 0x002
pub const DA9062AA_GPI0_SHIFT: c_int = 0;
pub const DA9062AA_GPI0_MASK: c_uint = 0x01;
pub const DA9062AA_GPI1_SHIFT: c_int = 1;

pub const DA9062AA_GPI2_SHIFT: c_int = 2;

pub const DA9062AA_GPI3_SHIFT: c_int = 3;

pub const DA9062AA_GPI4_SHIFT: c_int = 4;

// DA9062AA_STATUS_D = 0x004
pub const DA9062AA_LDO1_ILIM_SHIFT: c_int = 0;
pub const DA9062AA_LDO1_ILIM_MASK: c_uint = 0x01;
pub const DA9062AA_LDO2_ILIM_SHIFT: c_int = 1;

pub const DA9062AA_LDO3_ILIM_SHIFT: c_int = 2;

pub const DA9062AA_LDO4_ILIM_SHIFT: c_int = 3;

// DA9062AA_FAULT_LOG = 0x005
pub const DA9062AA_TWD_ERROR_SHIFT: c_int = 0;
pub const DA9062AA_TWD_ERROR_MASK: c_uint = 0x01;
pub const DA9062AA_POR_SHIFT: c_int = 1;

pub const DA9062AA_VDD_FAULT_SHIFT: c_int = 2;

pub const DA9062AA_VDD_START_SHIFT: c_int = 3;

pub const DA9062AA_TEMP_CRIT_SHIFT: c_int = 4;

pub const DA9062AA_KEY_RESET_SHIFT: c_int = 5;

pub const DA9062AA_NSHUTDOWN_SHIFT: c_int = 6;

pub const DA9062AA_WAIT_SHUT_SHIFT: c_int = 7;

// DA9062AA_EVENT_A = 0x006
pub const DA9062AA_E_NONKEY_SHIFT: c_int = 0;
pub const DA9062AA_E_NONKEY_MASK: c_uint = 0x01;
pub const DA9062AA_E_ALARM_SHIFT: c_int = 1;

pub const DA9062AA_E_TICK_SHIFT: c_int = 2;

pub const DA9062AA_E_WDG_WARN_SHIFT: c_int = 3;

pub const DA9062AA_E_SEQ_RDY_SHIFT: c_int = 4;

pub const DA9062AA_EVENTS_B_SHIFT: c_int = 5;

pub const DA9062AA_EVENTS_C_SHIFT: c_int = 6;

// DA9062AA_EVENT_B = 0x007
pub const DA9062AA_E_TEMP_SHIFT: c_int = 1;

pub const DA9062AA_E_LDO_LIM_SHIFT: c_int = 3;

pub const DA9062AA_E_DVC_RDY_SHIFT: c_int = 5;

pub const DA9062AA_E_VDD_WARN_SHIFT: c_int = 7;

// DA9062AA_EVENT_C = 0x008
pub const DA9062AA_E_GPI0_SHIFT: c_int = 0;
pub const DA9062AA_E_GPI0_MASK: c_uint = 0x01;
pub const DA9062AA_E_GPI1_SHIFT: c_int = 1;

pub const DA9062AA_E_GPI2_SHIFT: c_int = 2;

pub const DA9062AA_E_GPI3_SHIFT: c_int = 3;

pub const DA9062AA_E_GPI4_SHIFT: c_int = 4;

// DA9062AA_IRQ_MASK_A = 0x00A
pub const DA9062AA_M_NONKEY_SHIFT: c_int = 0;
pub const DA9062AA_M_NONKEY_MASK: c_uint = 0x01;
pub const DA9062AA_M_ALARM_SHIFT: c_int = 1;

pub const DA9062AA_M_TICK_SHIFT: c_int = 2;

pub const DA9062AA_M_WDG_WARN_SHIFT: c_int = 3;

pub const DA9062AA_M_SEQ_RDY_SHIFT: c_int = 4;

// DA9062AA_IRQ_MASK_B = 0x00B
pub const DA9062AA_M_TEMP_SHIFT: c_int = 1;

pub const DA9062AA_M_LDO_LIM_SHIFT: c_int = 3;

pub const DA9062AA_M_DVC_RDY_SHIFT: c_int = 5;

pub const DA9062AA_M_VDD_WARN_SHIFT: c_int = 7;

// DA9062AA_IRQ_MASK_C = 0x00C
pub const DA9062AA_M_GPI0_SHIFT: c_int = 0;
pub const DA9062AA_M_GPI0_MASK: c_uint = 0x01;
pub const DA9062AA_M_GPI1_SHIFT: c_int = 1;

pub const DA9062AA_M_GPI2_SHIFT: c_int = 2;

pub const DA9062AA_M_GPI3_SHIFT: c_int = 3;

pub const DA9062AA_M_GPI4_SHIFT: c_int = 4;

// DA9062AA_CONTROL_A = 0x00E
pub const DA9062AA_SYSTEM_EN_SHIFT: c_int = 0;
pub const DA9062AA_SYSTEM_EN_MASK: c_uint = 0x01;
pub const DA9062AA_POWER_EN_SHIFT: c_int = 1;

pub const DA9062AA_POWER1_EN_SHIFT: c_int = 2;

pub const DA9062AA_STANDBY_SHIFT: c_int = 3;

pub const DA9062AA_M_SYSTEM_EN_SHIFT: c_int = 4;

pub const DA9062AA_M_POWER_EN_SHIFT: c_int = 5;

pub const DA9062AA_M_POWER1_EN_SHIFT: c_int = 6;

// DA9062AA_CONTROL_B = 0x00F
pub const DA9062AA_WATCHDOG_PD_SHIFT: c_int = 1;

pub const DA9062AA_FREEZE_EN_SHIFT: c_int = 2;

pub const DA9062AA_NRES_MODE_SHIFT: c_int = 3;

pub const DA9062AA_NONKEY_LOCK_SHIFT: c_int = 4;

pub const DA9062AA_NFREEZE_SHIFT: c_int = 5;

pub const DA9062AA_BUCK_SLOWSTART_SHIFT: c_int = 7;

// DA9062AA_CONTROL_C = 0x010
pub const DA9062AA_DEBOUNCING_SHIFT: c_int = 0;
pub const DA9062AA_DEBOUNCING_MASK: c_uint = 0x07;
pub const DA9062AA_AUTO_BOOT_SHIFT: c_int = 3;

pub const DA9062AA_OTPREAD_EN_SHIFT: c_int = 4;

pub const DA9062AA_SLEW_RATE_SHIFT: c_int = 5;

pub const DA9062AA_DEF_SUPPLY_SHIFT: c_int = 7;

// DA9062AA_CONTROL_D = 0x011
pub const DA9062AA_TWDSCALE_SHIFT: c_int = 0;
pub const DA9062AA_TWDSCALE_MASK: c_uint = 0x07;
// DA9062AA_CONTROL_E = 0x012
pub const DA9062AA_RTC_MODE_PD_SHIFT: c_int = 0;
pub const DA9062AA_RTC_MODE_PD_MASK: c_uint = 0x01;
pub const DA9062AA_RTC_MODE_SD_SHIFT: c_int = 1;

pub const DA9062AA_RTC_EN_SHIFT: c_int = 2;

pub const DA9062AA_V_LOCK_SHIFT: c_int = 7;

// DA9062AA_CONTROL_F = 0x013
pub const DA9062AA_WATCHDOG_SHIFT: c_int = 0;
pub const DA9062AA_WATCHDOG_MASK: c_uint = 0x01;
pub const DA9062AA_SHUTDOWN_SHIFT: c_int = 1;

pub const DA9062AA_WAKE_UP_SHIFT: c_int = 2;

// DA9062AA_PD_DIS = 0x014
pub const DA9062AA_GPI_DIS_SHIFT: c_int = 0;
pub const DA9062AA_GPI_DIS_MASK: c_uint = 0x01;
pub const DA9062AA_PMIF_DIS_SHIFT: c_int = 2;

pub const DA9062AA_CLDR_PAUSE_SHIFT: c_int = 4;

pub const DA9062AA_BBAT_DIS_SHIFT: c_int = 5;

pub const DA9062AA_OUT32K_PAUSE_SHIFT: c_int = 6;

pub const DA9062AA_PMCONT_DIS_SHIFT: c_int = 7;

// DA9062AA_GPIO_0_1 = 0x015
pub const DA9062AA_GPIO0_PIN_SHIFT: c_int = 0;
pub const DA9062AA_GPIO0_PIN_MASK: c_uint = 0x03;
pub const DA9062AA_GPIO0_TYPE_SHIFT: c_int = 2;

pub const DA9062AA_GPIO0_WEN_SHIFT: c_int = 3;

pub const DA9062AA_GPIO1_PIN_SHIFT: c_int = 4;

pub const DA9062AA_GPIO1_TYPE_SHIFT: c_int = 6;

pub const DA9062AA_GPIO1_WEN_SHIFT: c_int = 7;

// DA9062AA_GPIO_2_3 = 0x016
pub const DA9062AA_GPIO2_PIN_SHIFT: c_int = 0;
pub const DA9062AA_GPIO2_PIN_MASK: c_uint = 0x03;
pub const DA9062AA_GPIO2_TYPE_SHIFT: c_int = 2;

pub const DA9062AA_GPIO2_WEN_SHIFT: c_int = 3;

pub const DA9062AA_GPIO3_PIN_SHIFT: c_int = 4;

pub const DA9062AA_GPIO3_TYPE_SHIFT: c_int = 6;

pub const DA9062AA_GPIO3_WEN_SHIFT: c_int = 7;

// DA9062AA_GPIO_4 = 0x017
pub const DA9062AA_GPIO4_PIN_SHIFT: c_int = 0;
pub const DA9062AA_GPIO4_PIN_MASK: c_uint = 0x03;
pub const DA9062AA_GPIO4_TYPE_SHIFT: c_int = 2;

pub const DA9062AA_GPIO4_WEN_SHIFT: c_int = 3;

// DA9062AA_GPIO_WKUP_MODE = 0x01C
pub const DA9062AA_GPIO0_WKUP_MODE_SHIFT: c_int = 0;
pub const DA9062AA_GPIO0_WKUP_MODE_MASK: c_uint = 0x01;
pub const DA9062AA_GPIO1_WKUP_MODE_SHIFT: c_int = 1;

pub const DA9062AA_GPIO2_WKUP_MODE_SHIFT: c_int = 2;

pub const DA9062AA_GPIO3_WKUP_MODE_SHIFT: c_int = 3;

pub const DA9062AA_GPIO4_WKUP_MODE_SHIFT: c_int = 4;

// DA9062AA_GPIO_MODE0_4 = 0x01D
pub const DA9062AA_GPIO0_MODE_SHIFT: c_int = 0;
pub const DA9062AA_GPIO0_MODE_MASK: c_uint = 0x01;
pub const DA9062AA_GPIO1_MODE_SHIFT: c_int = 1;

pub const DA9062AA_GPIO2_MODE_SHIFT: c_int = 2;

pub const DA9062AA_GPIO3_MODE_SHIFT: c_int = 3;

pub const DA9062AA_GPIO4_MODE_SHIFT: c_int = 4;

// DA9062AA_GPIO_OUT0_2 = 0x01E
pub const DA9062AA_GPIO0_OUT_SHIFT: c_int = 0;
pub const DA9062AA_GPIO0_OUT_MASK: c_uint = 0x07;
pub const DA9062AA_GPIO1_OUT_SHIFT: c_int = 3;

pub const DA9062AA_GPIO2_OUT_SHIFT: c_int = 6;

// DA9062AA_GPIO_OUT3_4 = 0x01F
pub const DA9062AA_GPIO3_OUT_SHIFT: c_int = 0;
pub const DA9062AA_GPIO3_OUT_MASK: c_uint = 0x07;
pub const DA9062AA_GPIO4_OUT_SHIFT: c_int = 3;

// DA9062AA_BUCK2_CONT = 0x020
pub const DA9062AA_BUCK2_EN_SHIFT: c_int = 0;
pub const DA9062AA_BUCK2_EN_MASK: c_uint = 0x01;
pub const DA9062AA_BUCK2_GPI_SHIFT: c_int = 1;

pub const DA9062AA_BUCK2_CONF_SHIFT: c_int = 3;

pub const DA9062AA_VBUCK2_GPI_SHIFT: c_int = 5;

// DA9062AA_BUCK1_CONT = 0x021
pub const DA9062AA_BUCK1_EN_SHIFT: c_int = 0;
pub const DA9062AA_BUCK1_EN_MASK: c_uint = 0x01;
pub const DA9062AA_BUCK1_GPI_SHIFT: c_int = 1;

pub const DA9062AA_BUCK1_CONF_SHIFT: c_int = 3;

pub const DA9062AA_VBUCK1_GPI_SHIFT: c_int = 5;

// DA9062AA_BUCK4_CONT = 0x022
pub const DA9062AA_BUCK4_EN_SHIFT: c_int = 0;
pub const DA9062AA_BUCK4_EN_MASK: c_uint = 0x01;
pub const DA9062AA_BUCK4_GPI_SHIFT: c_int = 1;

pub const DA9062AA_BUCK4_CONF_SHIFT: c_int = 3;

pub const DA9062AA_VBUCK4_GPI_SHIFT: c_int = 5;

// DA9062AA_BUCK3_CONT = 0x024
pub const DA9062AA_BUCK3_EN_SHIFT: c_int = 0;
pub const DA9062AA_BUCK3_EN_MASK: c_uint = 0x01;
pub const DA9062AA_BUCK3_GPI_SHIFT: c_int = 1;

pub const DA9062AA_BUCK3_CONF_SHIFT: c_int = 3;

pub const DA9062AA_VBUCK3_GPI_SHIFT: c_int = 5;

// DA9062AA_LDO1_CONT = 0x026
pub const DA9062AA_LDO1_EN_SHIFT: c_int = 0;
pub const DA9062AA_LDO1_EN_MASK: c_uint = 0x01;
pub const DA9062AA_LDO1_GPI_SHIFT: c_int = 1;

pub const DA9062AA_LDO1_PD_DIS_SHIFT: c_int = 3;

pub const DA9062AA_VLDO1_GPI_SHIFT: c_int = 5;

pub const DA9062AA_LDO1_CONF_SHIFT: c_int = 7;

// DA9062AA_LDO2_CONT = 0x027
pub const DA9062AA_LDO2_EN_SHIFT: c_int = 0;
pub const DA9062AA_LDO2_EN_MASK: c_uint = 0x01;
pub const DA9062AA_LDO2_GPI_SHIFT: c_int = 1;

pub const DA9062AA_LDO2_PD_DIS_SHIFT: c_int = 3;

pub const DA9062AA_VLDO2_GPI_SHIFT: c_int = 5;

pub const DA9062AA_LDO2_CONF_SHIFT: c_int = 7;

// DA9062AA_LDO3_CONT = 0x028
pub const DA9062AA_LDO3_EN_SHIFT: c_int = 0;
pub const DA9062AA_LDO3_EN_MASK: c_uint = 0x01;
pub const DA9062AA_LDO3_GPI_SHIFT: c_int = 1;

pub const DA9062AA_LDO3_PD_DIS_SHIFT: c_int = 3;

pub const DA9062AA_VLDO3_GPI_SHIFT: c_int = 5;

pub const DA9062AA_LDO3_CONF_SHIFT: c_int = 7;

// DA9062AA_LDO4_CONT = 0x029
pub const DA9062AA_LDO4_EN_SHIFT: c_int = 0;
pub const DA9062AA_LDO4_EN_MASK: c_uint = 0x01;
pub const DA9062AA_LDO4_GPI_SHIFT: c_int = 1;

pub const DA9062AA_LDO4_PD_DIS_SHIFT: c_int = 3;

pub const DA9062AA_VLDO4_GPI_SHIFT: c_int = 5;

pub const DA9062AA_LDO4_CONF_SHIFT: c_int = 7;

// DA9062AA_DVC_1 = 0x032
pub const DA9062AA_VBUCK1_SEL_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK1_SEL_MASK: c_uint = 0x01;
pub const DA9062AA_VBUCK2_SEL_SHIFT: c_int = 1;

pub const DA9062AA_VBUCK4_SEL_SHIFT: c_int = 2;

pub const DA9062AA_VBUCK3_SEL_SHIFT: c_int = 3;

pub const DA9062AA_VLDO1_SEL_SHIFT: c_int = 4;

pub const DA9062AA_VLDO2_SEL_SHIFT: c_int = 5;

pub const DA9062AA_VLDO3_SEL_SHIFT: c_int = 6;

pub const DA9062AA_VLDO4_SEL_SHIFT: c_int = 7;

// DA9062AA_COUNT_S = 0x040
pub const DA9062AA_COUNT_SEC_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_SEC_MASK: c_uint = 0x3f;
pub const DA9062AA_RTC_READ_SHIFT: c_int = 7;

// DA9062AA_COUNT_MI = 0x041
pub const DA9062AA_COUNT_MIN_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_MIN_MASK: c_uint = 0x3f;
// DA9062AA_COUNT_H = 0x042
pub const DA9062AA_COUNT_HOUR_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_HOUR_MASK: c_uint = 0x1f;
// DA9062AA_COUNT_D = 0x043
pub const DA9062AA_COUNT_DAY_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_DAY_MASK: c_uint = 0x1f;
// DA9062AA_COUNT_MO = 0x044
pub const DA9062AA_COUNT_MONTH_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_MONTH_MASK: c_uint = 0x0f;
// DA9062AA_COUNT_Y = 0x045
pub const DA9062AA_COUNT_YEAR_SHIFT: c_int = 0;
pub const DA9062AA_COUNT_YEAR_MASK: c_uint = 0x3f;
pub const DA9062AA_MONITOR_SHIFT: c_int = 6;

// DA9062AA_ALARM_S = 0x046
pub const DA9062AA_ALARM_SEC_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_SEC_MASK: c_uint = 0x3f;
pub const DA9062AA_ALARM_STATUS_SHIFT: c_int = 6;

// DA9062AA_ALARM_MI = 0x047
pub const DA9062AA_ALARM_MIN_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_MIN_MASK: c_uint = 0x3f;
// DA9062AA_ALARM_H = 0x048
pub const DA9062AA_ALARM_HOUR_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_HOUR_MASK: c_uint = 0x1f;
// DA9062AA_ALARM_D = 0x049
pub const DA9062AA_ALARM_DAY_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_DAY_MASK: c_uint = 0x1f;
// DA9062AA_ALARM_MO = 0x04A
pub const DA9062AA_ALARM_MONTH_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_MONTH_MASK: c_uint = 0x0f;
pub const DA9062AA_TICK_TYPE_SHIFT: c_int = 4;

pub const DA9062AA_TICK_WAKE_SHIFT: c_int = 5;

// DA9062AA_ALARM_Y = 0x04B
pub const DA9062AA_ALARM_YEAR_SHIFT: c_int = 0;
pub const DA9062AA_ALARM_YEAR_MASK: c_uint = 0x3f;
pub const DA9062AA_ALARM_ON_SHIFT: c_int = 6;

pub const DA9062AA_TICK_ON_SHIFT: c_int = 7;

// DA9062AA_SECOND_A = 0x04C
pub const DA9062AA_SECONDS_A_SHIFT: c_int = 0;
pub const DA9062AA_SECONDS_A_MASK: c_uint = 0xff;
// DA9062AA_SECOND_B = 0x04D
pub const DA9062AA_SECONDS_B_SHIFT: c_int = 0;
pub const DA9062AA_SECONDS_B_MASK: c_uint = 0xff;
// DA9062AA_SECOND_C = 0x04E
pub const DA9062AA_SECONDS_C_SHIFT: c_int = 0;
pub const DA9062AA_SECONDS_C_MASK: c_uint = 0xff;
// DA9062AA_SECOND_D = 0x04F
pub const DA9062AA_SECONDS_D_SHIFT: c_int = 0;
pub const DA9062AA_SECONDS_D_MASK: c_uint = 0xff;
// DA9062AA_SEQ = 0x081
pub const DA9062AA_SEQ_POINTER_SHIFT: c_int = 0;
pub const DA9062AA_SEQ_POINTER_MASK: c_uint = 0x0f;
pub const DA9062AA_NXT_SEQ_START_SHIFT: c_int = 4;

// DA9062AA_SEQ_TIMER = 0x082
pub const DA9062AA_SEQ_TIME_SHIFT: c_int = 0;
pub const DA9062AA_SEQ_TIME_MASK: c_uint = 0x0f;
pub const DA9062AA_SEQ_DUMMY_SHIFT: c_int = 4;

// DA9062AA_ID_2_1 = 0x083
pub const DA9062AA_LDO1_STEP_SHIFT: c_int = 0;
pub const DA9062AA_LDO1_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_LDO2_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_4_3 = 0x084
pub const DA9062AA_LDO3_STEP_SHIFT: c_int = 0;
pub const DA9062AA_LDO3_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_LDO4_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_12_11 = 0x088
pub const DA9062AA_PD_DIS_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_14_13 = 0x089
pub const DA9062AA_BUCK1_STEP_SHIFT: c_int = 0;
pub const DA9062AA_BUCK1_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_BUCK2_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_16_15 = 0x08A
pub const DA9062AA_BUCK4_STEP_SHIFT: c_int = 0;
pub const DA9062AA_BUCK4_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_BUCK3_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_22_21 = 0x08D
pub const DA9062AA_GP_RISE1_STEP_SHIFT: c_int = 0;
pub const DA9062AA_GP_RISE1_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_GP_FALL1_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_24_23 = 0x08E
pub const DA9062AA_GP_RISE2_STEP_SHIFT: c_int = 0;
pub const DA9062AA_GP_RISE2_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_GP_FALL2_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_26_25 = 0x08F
pub const DA9062AA_GP_RISE3_STEP_SHIFT: c_int = 0;
pub const DA9062AA_GP_RISE3_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_GP_FALL3_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_28_27 = 0x090
pub const DA9062AA_GP_RISE4_STEP_SHIFT: c_int = 0;
pub const DA9062AA_GP_RISE4_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_GP_FALL4_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_30_29 = 0x091
pub const DA9062AA_GP_RISE5_STEP_SHIFT: c_int = 0;
pub const DA9062AA_GP_RISE5_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_GP_FALL5_STEP_SHIFT: c_int = 4;

// DA9062AA_ID_32_31 = 0x092
pub const DA9062AA_WAIT_STEP_SHIFT: c_int = 0;
pub const DA9062AA_WAIT_STEP_MASK: c_uint = 0x0f;
pub const DA9062AA_EN32K_STEP_SHIFT: c_int = 4;

// DA9062AA_SEQ_A = 0x095
pub const DA9062AA_SYSTEM_END_SHIFT: c_int = 0;
pub const DA9062AA_SYSTEM_END_MASK: c_uint = 0x0f;
pub const DA9062AA_POWER_END_SHIFT: c_int = 4;

// DA9062AA_SEQ_B = 0x096
pub const DA9062AA_MAX_COUNT_SHIFT: c_int = 0;
pub const DA9062AA_MAX_COUNT_MASK: c_uint = 0x0f;
pub const DA9062AA_PART_DOWN_SHIFT: c_int = 4;

// DA9062AA_WAIT = 0x097
pub const DA9062AA_WAIT_TIME_SHIFT: c_int = 0;
pub const DA9062AA_WAIT_TIME_MASK: c_uint = 0x0f;
pub const DA9062AA_WAIT_MODE_SHIFT: c_int = 4;

pub const DA9062AA_TIME_OUT_SHIFT: c_int = 5;

pub const DA9062AA_WAIT_DIR_SHIFT: c_int = 6;

// DA9062AA_EN_32K = 0x098
pub const DA9062AA_STABILISATION_TIME_SHIFT: c_int = 0;
pub const DA9062AA_STABILISATION_TIME_MASK: c_uint = 0x07;
pub const DA9062AA_CRYSTAL_SHIFT: c_int = 3;

pub const DA9062AA_DELAY_MODE_SHIFT: c_int = 4;

pub const DA9062AA_OUT_CLOCK_SHIFT: c_int = 5;

pub const DA9062AA_RTC_CLOCK_SHIFT: c_int = 6;

pub const DA9062AA_EN_32KOUT_SHIFT: c_int = 7;

// DA9062AA_RESET = 0x099
pub const DA9062AA_RESET_TIMER_SHIFT: c_int = 0;
pub const DA9062AA_RESET_TIMER_MASK: c_uint = 0x3f;
pub const DA9062AA_RESET_EVENT_SHIFT: c_int = 6;

// DA9062AA_BUCK_ILIM_A = 0x09A
pub const DA9062AA_BUCK3_ILIM_SHIFT: c_int = 0;
pub const DA9062AA_BUCK3_ILIM_MASK: c_uint = 0x0f;
// DA9062AA_BUCK_ILIM_B = 0x09B
pub const DA9062AA_BUCK4_ILIM_SHIFT: c_int = 0;
pub const DA9062AA_BUCK4_ILIM_MASK: c_uint = 0x0f;
// DA9062AA_BUCK_ILIM_C = 0x09C
pub const DA9062AA_BUCK1_ILIM_SHIFT: c_int = 0;
pub const DA9062AA_BUCK1_ILIM_MASK: c_uint = 0x0f;
pub const DA9062AA_BUCK2_ILIM_SHIFT: c_int = 4;

// DA9062AA_BUCK2_CFG = 0x09D
pub const DA9062AA_BUCK2_PD_DIS_SHIFT: c_int = 5;

pub const DA9062AA_BUCK2_MODE_SHIFT: c_int = 6;

// DA9062AA_BUCK1_CFG = 0x09E
pub const DA9062AA_BUCK1_PD_DIS_SHIFT: c_int = 5;

pub const DA9062AA_BUCK1_MODE_SHIFT: c_int = 6;

// DA9062AA_BUCK4_CFG = 0x09F
pub const DA9062AA_BUCK4_VTTR_EN_SHIFT: c_int = 3;

pub const DA9062AA_BUCK4_VTT_EN_SHIFT: c_int = 4;

pub const DA9062AA_BUCK4_PD_DIS_SHIFT: c_int = 5;

pub const DA9062AA_BUCK4_MODE_SHIFT: c_int = 6;

// DA9062AA_BUCK3_CFG = 0x0A0
pub const DA9062AA_BUCK3_PD_DIS_SHIFT: c_int = 5;

pub const DA9062AA_BUCK3_MODE_SHIFT: c_int = 6;

// DA9062AA_VBUCK2_A = 0x0A3
pub const DA9062AA_VBUCK2_A_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK2_A_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK2_SL_A_SHIFT: c_int = 7;

// DA9062AA_VBUCK1_A = 0x0A4
pub const DA9062AA_VBUCK1_A_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK1_A_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK1_SL_A_SHIFT: c_int = 7;

// DA9062AA_VBUCK4_A = 0x0A5
pub const DA9062AA_VBUCK4_A_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK4_A_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK4_SL_A_SHIFT: c_int = 7;

// DA9062AA_VBUCK3_A = 0x0A7
pub const DA9062AA_VBUCK3_A_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK3_A_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK3_SL_A_SHIFT: c_int = 7;

// DA9062AA_VLDO[1-4]_A common
pub const DA9062AA_VLDO_A_MIN_SEL: c_int = 2;
// DA9062AA_VLDO1_A = 0x0A9
pub const DA9062AA_VLDO1_A_SHIFT: c_int = 0;
pub const DA9062AA_VLDO1_A_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO1_SL_A_SHIFT: c_int = 7;

// DA9062AA_VLDO2_A = 0x0AA
pub const DA9062AA_VLDO2_A_SHIFT: c_int = 0;
pub const DA9062AA_VLDO2_A_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO2_SL_A_SHIFT: c_int = 7;

// DA9062AA_VLDO3_A = 0x0AB
pub const DA9062AA_VLDO3_A_SHIFT: c_int = 0;
pub const DA9062AA_VLDO3_A_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO3_SL_A_SHIFT: c_int = 7;

// DA9062AA_VLDO4_A = 0x0AC
pub const DA9062AA_VLDO4_A_SHIFT: c_int = 0;
pub const DA9062AA_VLDO4_A_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO4_SL_A_SHIFT: c_int = 7;

// DA9062AA_VBUCK2_B = 0x0B4
pub const DA9062AA_VBUCK2_B_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK2_B_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK2_SL_B_SHIFT: c_int = 7;

// DA9062AA_VBUCK1_B = 0x0B5
pub const DA9062AA_VBUCK1_B_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK1_B_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK1_SL_B_SHIFT: c_int = 7;

// DA9062AA_VBUCK4_B = 0x0B6
pub const DA9062AA_VBUCK4_B_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK4_B_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK4_SL_B_SHIFT: c_int = 7;

// DA9062AA_VBUCK3_B = 0x0B8
pub const DA9062AA_VBUCK3_B_SHIFT: c_int = 0;
pub const DA9062AA_VBUCK3_B_MASK: c_uint = 0x7f;
pub const DA9062AA_BUCK3_SL_B_SHIFT: c_int = 7;

// DA9062AA_VLDO1_B = 0x0BA
pub const DA9062AA_VLDO1_B_SHIFT: c_int = 0;
pub const DA9062AA_VLDO1_B_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO1_SL_B_SHIFT: c_int = 7;

// DA9062AA_VLDO2_B = 0x0BB
pub const DA9062AA_VLDO2_B_SHIFT: c_int = 0;
pub const DA9062AA_VLDO2_B_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO2_SL_B_SHIFT: c_int = 7;

// DA9062AA_VLDO3_B = 0x0BC
pub const DA9062AA_VLDO3_B_SHIFT: c_int = 0;
pub const DA9062AA_VLDO3_B_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO3_SL_B_SHIFT: c_int = 7;

// DA9062AA_VLDO4_B = 0x0BD
pub const DA9062AA_VLDO4_B_SHIFT: c_int = 0;
pub const DA9062AA_VLDO4_B_MASK: c_uint = 0x3f;
pub const DA9062AA_LDO4_SL_B_SHIFT: c_int = 7;

// DA9062AA_BBAT_CONT = 0x0C5
pub const DA9062AA_BCHG_VSET_SHIFT: c_int = 0;
pub const DA9062AA_BCHG_VSET_MASK: c_uint = 0x0f;
pub const DA9062AA_BCHG_ISET_SHIFT: c_int = 4;

// DA9062AA_INTERFACE = 0x105
pub const DA9062AA_IF_BASE_ADDR_SHIFT: c_int = 4;

// DA9062AA_CONFIG_A = 0x106
pub const DA9062AA_PM_I_V_SHIFT: c_int = 0;
pub const DA9062AA_PM_I_V_MASK: c_uint = 0x01;
pub const DA9062AA_PM_O_TYPE_SHIFT: c_int = 2;

pub const DA9062AA_IRQ_TYPE_SHIFT: c_int = 3;

pub const DA9062AA_PM_IF_V_SHIFT: c_int = 4;

pub const DA9062AA_PM_IF_FMP_SHIFT: c_int = 5;

pub const DA9062AA_PM_IF_HSM_SHIFT: c_int = 6;

// DA9062AA_CONFIG_B = 0x107
pub const DA9062AA_VDD_FAULT_ADJ_SHIFT: c_int = 0;
pub const DA9062AA_VDD_FAULT_ADJ_MASK: c_uint = 0x0f;
pub const DA9062AA_VDD_HYST_ADJ_SHIFT: c_int = 4;

// DA9062AA_CONFIG_C = 0x108
pub const DA9062AA_BUCK_ACTV_DISCHRG_SHIFT: c_int = 2;

pub const DA9062AA_BUCK1_CLK_INV_SHIFT: c_int = 3;

pub const DA9062AA_BUCK4_CLK_INV_SHIFT: c_int = 4;

pub const DA9062AA_BUCK3_CLK_INV_SHIFT: c_int = 6;

// DA9062AA_CONFIG_D = 0x109
pub const DA9062AA_GPI_V_SHIFT: c_int = 0;
pub const DA9062AA_GPI_V_MASK: c_uint = 0x01;
pub const DA9062AA_NIRQ_MODE_SHIFT: c_int = 1;

pub const DA9062AA_SYSTEM_EN_RD_SHIFT: c_int = 2;

pub const DA9062AA_FORCE_RESET_SHIFT: c_int = 5;

// DA9062AA_CONFIG_E = 0x10A
pub const DA9062AA_BUCK1_AUTO_SHIFT: c_int = 0;
pub const DA9062AA_BUCK1_AUTO_MASK: c_uint = 0x01;
pub const DA9062AA_BUCK2_AUTO_SHIFT: c_int = 1;

pub const DA9062AA_BUCK4_AUTO_SHIFT: c_int = 2;

pub const DA9062AA_BUCK3_AUTO_SHIFT: c_int = 4;

// DA9062AA_CONFIG_G = 0x10C
pub const DA9062AA_LDO1_AUTO_SHIFT: c_int = 0;
pub const DA9062AA_LDO1_AUTO_MASK: c_uint = 0x01;
pub const DA9062AA_LDO2_AUTO_SHIFT: c_int = 1;

pub const DA9062AA_LDO3_AUTO_SHIFT: c_int = 2;

pub const DA9062AA_LDO4_AUTO_SHIFT: c_int = 3;

// DA9062AA_CONFIG_H = 0x10D
pub const DA9062AA_BUCK1_2_MERGE_SHIFT: c_int = 3;

pub const DA9062AA_BUCK2_OD_SHIFT: c_int = 5;

pub const DA9062AA_BUCK1_OD_SHIFT: c_int = 6;

// DA9062AA_CONFIG_I = 0x10E
pub const DA9062AA_NONKEY_PIN_SHIFT: c_int = 0;
pub const DA9062AA_NONKEY_PIN_MASK: c_uint = 0x03;
pub const DA9062AA_nONKEY_SD_SHIFT: c_int = 2;

pub const DA9062AA_WATCHDOG_SD_SHIFT: c_int = 3;

pub const DA9062AA_KEY_SD_MODE_SHIFT: c_int = 4;

pub const DA9062AA_HOST_SD_MODE_SHIFT: c_int = 5;

pub const DA9062AA_INT_SD_MODE_SHIFT: c_int = 6;

pub const DA9062AA_LDO_SD_SHIFT: c_int = 7;

// DA9062AA_CONFIG_J = 0x10F
pub const DA9062AA_KEY_DELAY_SHIFT: c_int = 0;
pub const DA9062AA_KEY_DELAY_MASK: c_uint = 0x03;
pub const DA9062AA_SHUT_DELAY_SHIFT: c_int = 2;

pub const DA9062AA_RESET_DURATION_SHIFT: c_int = 4;

pub const DA9062AA_TWOWIRE_TO_SHIFT: c_int = 6;

pub const DA9062AA_IF_RESET_SHIFT: c_int = 7;

// DA9062AA_CONFIG_K = 0x110
pub const DA9062AA_GPIO0_PUPD_SHIFT: c_int = 0;
pub const DA9062AA_GPIO0_PUPD_MASK: c_uint = 0x01;
pub const DA9062AA_GPIO1_PUPD_SHIFT: c_int = 1;

pub const DA9062AA_GPIO2_PUPD_SHIFT: c_int = 2;

pub const DA9062AA_GPIO3_PUPD_SHIFT: c_int = 3;

pub const DA9062AA_GPIO4_PUPD_SHIFT: c_int = 4;

// DA9062AA_CONFIG_M = 0x112
pub const DA9062AA_NSHUTDOWN_PU_SHIFT: c_int = 1;

pub const DA9062AA_WDG_MODE_SHIFT: c_int = 3;

pub const DA9062AA_OSC_FRQ_SHIFT: c_int = 4;

// DA9062AA_TRIM_CLDR = 0x120
pub const DA9062AA_TRIM_CLDR_SHIFT: c_int = 0;
pub const DA9062AA_TRIM_CLDR_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_0 = 0x121
pub const DA9062AA_GP_0_SHIFT: c_int = 0;
pub const DA9062AA_GP_0_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_1 = 0x122
pub const DA9062AA_GP_1_SHIFT: c_int = 0;
pub const DA9062AA_GP_1_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_2 = 0x123
pub const DA9062AA_GP_2_SHIFT: c_int = 0;
pub const DA9062AA_GP_2_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_3 = 0x124
pub const DA9062AA_GP_3_SHIFT: c_int = 0;
pub const DA9062AA_GP_3_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_4 = 0x125
pub const DA9062AA_GP_4_SHIFT: c_int = 0;
pub const DA9062AA_GP_4_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_5 = 0x126
pub const DA9062AA_GP_5_SHIFT: c_int = 0;
pub const DA9062AA_GP_5_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_6 = 0x127
pub const DA9062AA_GP_6_SHIFT: c_int = 0;
pub const DA9062AA_GP_6_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_7 = 0x128
pub const DA9062AA_GP_7_SHIFT: c_int = 0;
pub const DA9062AA_GP_7_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_8 = 0x129
pub const DA9062AA_GP_8_SHIFT: c_int = 0;
pub const DA9062AA_GP_8_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_9 = 0x12A
pub const DA9062AA_GP_9_SHIFT: c_int = 0;
pub const DA9062AA_GP_9_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_10 = 0x12B
pub const DA9062AA_GP_10_SHIFT: c_int = 0;
pub const DA9062AA_GP_10_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_11 = 0x12C
pub const DA9062AA_GP_11_SHIFT: c_int = 0;
pub const DA9062AA_GP_11_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_12 = 0x12D
pub const DA9062AA_GP_12_SHIFT: c_int = 0;
pub const DA9062AA_GP_12_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_13 = 0x12E
pub const DA9062AA_GP_13_SHIFT: c_int = 0;
pub const DA9062AA_GP_13_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_14 = 0x12F
pub const DA9062AA_GP_14_SHIFT: c_int = 0;
pub const DA9062AA_GP_14_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_15 = 0x130
pub const DA9062AA_GP_15_SHIFT: c_int = 0;
pub const DA9062AA_GP_15_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_16 = 0x131
pub const DA9062AA_GP_16_SHIFT: c_int = 0;
pub const DA9062AA_GP_16_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_17 = 0x132
pub const DA9062AA_GP_17_SHIFT: c_int = 0;
pub const DA9062AA_GP_17_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_18 = 0x133
pub const DA9062AA_GP_18_SHIFT: c_int = 0;
pub const DA9062AA_GP_18_MASK: c_uint = 0xff;
// DA9062AA_GP_ID_19 = 0x134
pub const DA9062AA_GP_19_SHIFT: c_int = 0;
pub const DA9062AA_GP_19_MASK: c_uint = 0xff;
// DA9062AA_DEVICE_ID = 0x181
pub const DA9062AA_DEV_ID_SHIFT: c_int = 0;
pub const DA9062AA_DEV_ID_MASK: c_uint = 0xff;
// DA9062AA_VARIANT_ID = 0x182
pub const DA9062AA_VRC_SHIFT: c_int = 0;
pub const DA9062AA_VRC_MASK: c_uint = 0x0f;
pub const DA9062AA_MRC_SHIFT: c_int = 4;

// DA9062AA_CUSTOMER_ID = 0x183
pub const DA9062AA_CUST_ID_SHIFT: c_int = 0;
pub const DA9062AA_CUST_ID_MASK: c_uint = 0xff;
// DA9062AA_CONFIG_ID = 0x184
pub const DA9062AA_CONFIG_REV_SHIFT: c_int = 0;
pub const DA9062AA_CONFIG_REV_MASK: c_uint = 0xff;
