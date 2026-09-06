//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm8350/core.h
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
// core.h  --  Core Driver for Wolfson WM8350 PMIC
//
// Copyright 2007 Wolfson Microelectronics PLC
//

//
// Register values.
//
pub const WM8350_RESET_ID: c_uint = 0x00;
pub const WM8350_ID: c_uint = 0x01;
pub const WM8350_REVISION: c_uint = 0x02;
pub const WM8350_SYSTEM_CONTROL_1: c_uint = 0x03;
pub const WM8350_SYSTEM_CONTROL_2: c_uint = 0x04;
pub const WM8350_SYSTEM_HIBERNATE: c_uint = 0x05;
pub const WM8350_INTERFACE_CONTROL: c_uint = 0x06;
pub const WM8350_POWER_MGMT_1: c_uint = 0x08;
pub const WM8350_POWER_MGMT_2: c_uint = 0x09;
pub const WM8350_POWER_MGMT_3: c_uint = 0x0A;
pub const WM8350_POWER_MGMT_4: c_uint = 0x0B;
pub const WM8350_POWER_MGMT_5: c_uint = 0x0C;
pub const WM8350_POWER_MGMT_6: c_uint = 0x0D;
pub const WM8350_POWER_MGMT_7: c_uint = 0x0E;
pub const WM8350_SYSTEM_INTERRUPTS: c_uint = 0x18;
pub const WM8350_INT_STATUS_1: c_uint = 0x19;
pub const WM8350_INT_STATUS_2: c_uint = 0x1A;
pub const WM8350_POWER_UP_INT_STATUS: c_uint = 0x1B;
pub const WM8350_UNDER_VOLTAGE_INT_STATUS: c_uint = 0x1C;
pub const WM8350_OVER_CURRENT_INT_STATUS: c_uint = 0x1D;
pub const WM8350_GPIO_INT_STATUS: c_uint = 0x1E;
pub const WM8350_COMPARATOR_INT_STATUS: c_uint = 0x1F;
pub const WM8350_SYSTEM_INTERRUPTS_MASK: c_uint = 0x20;
pub const WM8350_INT_STATUS_1_MASK: c_uint = 0x21;
pub const WM8350_INT_STATUS_2_MASK: c_uint = 0x22;
pub const WM8350_POWER_UP_INT_STATUS_MASK: c_uint = 0x23;
pub const WM8350_UNDER_VOLTAGE_INT_STATUS_MASK: c_uint = 0x24;
pub const WM8350_OVER_CURRENT_INT_STATUS_MASK: c_uint = 0x25;
pub const WM8350_GPIO_INT_STATUS_MASK: c_uint = 0x26;
pub const WM8350_COMPARATOR_INT_STATUS_MASK: c_uint = 0x27;
pub const WM8350_CHARGER_OVERRIDES: c_uint = 0xE2;
pub const WM8350_MISC_OVERRIDES: c_uint = 0xE3;
pub const WM8350_COMPARATOR_OVERRIDES: c_uint = 0xE7;
pub const WM8350_STATE_MACHINE_STATUS: c_uint = 0xE9;
pub const WM8350_MAX_REGISTER: c_uint = 0xFF;
pub const WM8350_UNLOCK_KEY: c_uint = 0x0013;
pub const WM8350_LOCK_KEY: c_uint = 0x0000;
//
// Field Definitions.
//
// R0 (0x00) - Reset/ID
//
pub const WM8350_SW_RESET_CHIP_ID_MASK: c_uint = 0xFFFF;
//
// R1 (0x01) - ID
//
pub const WM8350_CHIP_REV_MASK: c_uint = 0x7000;
pub const WM8350_CONF_STS_MASK: c_uint = 0x0C00;
pub const WM8350_CUST_ID_MASK: c_uint = 0x00FF;
//
// R2 (0x02) - Revision
//
pub const WM8350_MASK_REV_MASK: c_uint = 0x00FF;
//
// R3 (0x03) - System Control 1
//
pub const WM8350_CHIP_ON: c_uint = 0x8000;
pub const WM8350_POWERCYCLE: c_uint = 0x2000;
pub const WM8350_VCC_FAULT_OV: c_uint = 0x1000;
pub const WM8350_REG_RSTB_TIME_MASK: c_uint = 0x0C00;
pub const WM8350_BG_SLEEP: c_uint = 0x0200;
pub const WM8350_MEM_VALID: c_uint = 0x0020;
pub const WM8350_CHIP_SET_UP: c_uint = 0x0010;
pub const WM8350_ON_DEB_T: c_uint = 0x0008;
pub const WM8350_ON_POL: c_uint = 0x0002;
pub const WM8350_IRQ_POL: c_uint = 0x0001;
//
// R4 (0x04) - System Control 2
//
pub const WM8350_USB_SUSPEND_8MA: c_uint = 0x8000;
pub const WM8350_USB_SUSPEND: c_uint = 0x4000;
pub const WM8350_USB_MSTR: c_uint = 0x2000;
pub const WM8350_USB_MSTR_SRC: c_uint = 0x1000;
pub const WM8350_USB_500MA: c_uint = 0x0800;
pub const WM8350_USB_NOLIM: c_uint = 0x0400;
//
// R5 (0x05) - System Hibernate
//
pub const WM8350_HIBERNATE: c_uint = 0x8000;
pub const WM8350_WDOG_HIB_MODE: c_uint = 0x0080;
pub const WM8350_REG_HIB_STARTUP_SEQ: c_uint = 0x0040;
pub const WM8350_REG_RESET_HIB_MODE: c_uint = 0x0020;
pub const WM8350_RST_HIB_MODE: c_uint = 0x0010;
pub const WM8350_IRQ_HIB_MODE: c_uint = 0x0008;
pub const WM8350_MEMRST_HIB_MODE: c_uint = 0x0004;
pub const WM8350_PCCOMP_HIB_MODE: c_uint = 0x0002;
pub const WM8350_TEMPMON_HIB_MODE: c_uint = 0x0001;
//
// R6 (0x06) - Interface Control
//
pub const WM8350_USE_DEV_PINS: c_uint = 0x8000;
pub const WM8350_USE_DEV_PINS_MASK: c_uint = 0x8000;
pub const WM8350_USE_DEV_PINS_SHIFT: c_int = 15;
pub const WM8350_DEV_ADDR_MASK: c_uint = 0x6000;
pub const WM8350_DEV_ADDR_SHIFT: c_int = 13;
pub const WM8350_CONFIG_DONE: c_uint = 0x1000;
pub const WM8350_CONFIG_DONE_MASK: c_uint = 0x1000;
pub const WM8350_CONFIG_DONE_SHIFT: c_int = 12;
pub const WM8350_RECONFIG_AT_ON: c_uint = 0x0800;
pub const WM8350_RECONFIG_AT_ON_MASK: c_uint = 0x0800;
pub const WM8350_RECONFIG_AT_ON_SHIFT: c_int = 11;
pub const WM8350_AUTOINC: c_uint = 0x0200;
pub const WM8350_AUTOINC_MASK: c_uint = 0x0200;
pub const WM8350_AUTOINC_SHIFT: c_int = 9;
pub const WM8350_ARA: c_uint = 0x0100;
pub const WM8350_ARA_MASK: c_uint = 0x0100;
pub const WM8350_ARA_SHIFT: c_int = 8;
pub const WM8350_SPI_CFG: c_uint = 0x0008;
pub const WM8350_SPI_CFG_MASK: c_uint = 0x0008;
pub const WM8350_SPI_CFG_SHIFT: c_int = 3;
pub const WM8350_SPI_4WIRE: c_uint = 0x0004;
pub const WM8350_SPI_4WIRE_MASK: c_uint = 0x0004;
pub const WM8350_SPI_4WIRE_SHIFT: c_int = 2;
pub const WM8350_SPI_3WIRE: c_uint = 0x0002;
pub const WM8350_SPI_3WIRE_MASK: c_uint = 0x0002;
pub const WM8350_SPI_3WIRE_SHIFT: c_int = 1;
// Bit values for R06 (0x06)
pub const WM8350_USE_DEV_PINS_PRIMARY: c_int = 0;
pub const WM8350_USE_DEV_PINS_DEV: c_int = 1;
pub const WM8350_DEV_ADDR_34: c_int = 0;
pub const WM8350_DEV_ADDR_36: c_int = 1;
pub const WM8350_DEV_ADDR_3C: c_int = 2;
pub const WM8350_DEV_ADDR_3E: c_int = 3;
pub const WM8350_CONFIG_DONE_OFF: c_int = 0;
pub const WM8350_CONFIG_DONE_DONE: c_int = 1;
pub const WM8350_RECONFIG_AT_ON_OFF: c_int = 0;
pub const WM8350_RECONFIG_AT_ON_ON: c_int = 1;
pub const WM8350_AUTOINC_OFF: c_int = 0;
pub const WM8350_AUTOINC_ON: c_int = 1;
pub const WM8350_ARA_OFF: c_int = 0;
pub const WM8350_ARA_ON: c_int = 1;
pub const WM8350_SPI_CFG_CMOS: c_int = 0;
pub const WM8350_SPI_CFG_OD: c_int = 1;
pub const WM8350_SPI_4WIRE_3WIRE: c_int = 0;
pub const WM8350_SPI_4WIRE_4WIRE: c_int = 1;
pub const WM8350_SPI_3WIRE_I2C: c_int = 0;
pub const WM8350_SPI_3WIRE_SPI: c_int = 1;
//
// R8 (0x08) - Power mgmt (1)
//
pub const WM8350_CODEC_ISEL_MASK: c_uint = 0xC000;
pub const WM8350_VBUFEN: c_uint = 0x2000;
pub const WM8350_OUTPUT_DRAIN_EN: c_uint = 0x0400;
pub const WM8350_MIC_DET_ENA: c_uint = 0x0100;
pub const WM8350_BIASEN: c_uint = 0x0020;
pub const WM8350_MICBEN: c_uint = 0x0010;
pub const WM8350_VMIDEN: c_uint = 0x0004;
pub const WM8350_VMID_MASK: c_uint = 0x0003;
pub const WM8350_VMID_SHIFT: c_int = 0;
//
// R9 (0x09) - Power mgmt (2)
//
pub const WM8350_IN3R_ENA: c_uint = 0x0800;
pub const WM8350_IN3L_ENA: c_uint = 0x0400;
pub const WM8350_INR_ENA: c_uint = 0x0200;
pub const WM8350_INL_ENA: c_uint = 0x0100;
pub const WM8350_MIXINR_ENA: c_uint = 0x0080;
pub const WM8350_MIXINL_ENA: c_uint = 0x0040;
pub const WM8350_OUT4_ENA: c_uint = 0x0020;
pub const WM8350_OUT3_ENA: c_uint = 0x0010;
pub const WM8350_MIXOUTR_ENA: c_uint = 0x0002;
pub const WM8350_MIXOUTL_ENA: c_uint = 0x0001;
//
// R10 (0x0A) - Power mgmt (3)
//
pub const WM8350_IN3R_TO_OUT2R: c_uint = 0x0080;
pub const WM8350_OUT2R_ENA: c_uint = 0x0008;
pub const WM8350_OUT2L_ENA: c_uint = 0x0004;
pub const WM8350_OUT1R_ENA: c_uint = 0x0002;
pub const WM8350_OUT1L_ENA: c_uint = 0x0001;
//
// R11 (0x0B) - Power mgmt (4)
//
pub const WM8350_SYSCLK_ENA: c_uint = 0x4000;
pub const WM8350_ADC_HPF_ENA: c_uint = 0x2000;
pub const WM8350_FLL_ENA: c_uint = 0x0800;
pub const WM8350_FLL_OSC_ENA: c_uint = 0x0400;
pub const WM8350_TOCLK_ENA: c_uint = 0x0100;
pub const WM8350_DACR_ENA: c_uint = 0x0020;
pub const WM8350_DACL_ENA: c_uint = 0x0010;
pub const WM8350_ADCR_ENA: c_uint = 0x0008;
pub const WM8350_ADCL_ENA: c_uint = 0x0004;
//
// R12 (0x0C) - Power mgmt (5)
//
pub const WM8350_CODEC_ENA: c_uint = 0x1000;
pub const WM8350_RTC_TICK_ENA: c_uint = 0x0800;
pub const WM8350_OSC32K_ENA: c_uint = 0x0400;
pub const WM8350_CHG_ENA: c_uint = 0x0200;
pub const WM8350_ACC_DET_ENA: c_uint = 0x0100;
pub const WM8350_AUXADC_ENA: c_uint = 0x0080;
pub const WM8350_DCMP4_ENA: c_uint = 0x0008;
pub const WM8350_DCMP3_ENA: c_uint = 0x0004;
pub const WM8350_DCMP2_ENA: c_uint = 0x0002;
pub const WM8350_DCMP1_ENA: c_uint = 0x0001;
//
// R13 (0x0D) - Power mgmt (6)
//
pub const WM8350_LS_ENA: c_uint = 0x8000;
pub const WM8350_LDO4_ENA: c_uint = 0x0800;
pub const WM8350_LDO3_ENA: c_uint = 0x0400;
pub const WM8350_LDO2_ENA: c_uint = 0x0200;
pub const WM8350_LDO1_ENA: c_uint = 0x0100;
pub const WM8350_DC6_ENA: c_uint = 0x0020;
pub const WM8350_DC5_ENA: c_uint = 0x0010;
pub const WM8350_DC4_ENA: c_uint = 0x0008;
pub const WM8350_DC3_ENA: c_uint = 0x0004;
pub const WM8350_DC2_ENA: c_uint = 0x0002;
pub const WM8350_DC1_ENA: c_uint = 0x0001;
//
// R14 (0x0E) - Power mgmt (7)
//
pub const WM8350_CS2_ENA: c_uint = 0x0002;
pub const WM8350_CS1_ENA: c_uint = 0x0001;
//
// R24 (0x18) - System Interrupts
//
pub const WM8350_OC_INT: c_uint = 0x2000;
pub const WM8350_UV_INT: c_uint = 0x1000;
pub const WM8350_PUTO_INT: c_uint = 0x0800;
pub const WM8350_CS_INT: c_uint = 0x0200;
pub const WM8350_EXT_INT: c_uint = 0x0100;
pub const WM8350_CODEC_INT: c_uint = 0x0080;
pub const WM8350_GP_INT: c_uint = 0x0040;
pub const WM8350_AUXADC_INT: c_uint = 0x0020;
pub const WM8350_RTC_INT: c_uint = 0x0010;
pub const WM8350_SYS_INT: c_uint = 0x0008;
pub const WM8350_CHG_INT: c_uint = 0x0004;
pub const WM8350_USB_INT: c_uint = 0x0002;
pub const WM8350_WKUP_INT: c_uint = 0x0001;
//
// R25 (0x19) - Interrupt Status 1
//
pub const WM8350_CHG_BAT_HOT_EINT: c_uint = 0x8000;
pub const WM8350_CHG_BAT_COLD_EINT: c_uint = 0x4000;
pub const WM8350_CHG_BAT_FAIL_EINT: c_uint = 0x2000;
pub const WM8350_CHG_TO_EINT: c_uint = 0x1000;
pub const WM8350_CHG_END_EINT: c_uint = 0x0800;
pub const WM8350_CHG_START_EINT: c_uint = 0x0400;
pub const WM8350_CHG_FAST_RDY_EINT: c_uint = 0x0200;
pub const WM8350_RTC_PER_EINT: c_uint = 0x0080;
pub const WM8350_RTC_SEC_EINT: c_uint = 0x0040;
pub const WM8350_RTC_ALM_EINT: c_uint = 0x0020;
pub const WM8350_CHG_VBATT_LT_3P9_EINT: c_uint = 0x0004;
pub const WM8350_CHG_VBATT_LT_3P1_EINT: c_uint = 0x0002;
pub const WM8350_CHG_VBATT_LT_2P85_EINT: c_uint = 0x0001;
//
// R26 (0x1A) - Interrupt Status 2
//
pub const WM8350_CS1_EINT: c_uint = 0x2000;
pub const WM8350_CS2_EINT: c_uint = 0x1000;
pub const WM8350_USB_LIMIT_EINT: c_uint = 0x0400;
pub const WM8350_AUXADC_DATARDY_EINT: c_uint = 0x0100;
pub const WM8350_AUXADC_DCOMP4_EINT: c_uint = 0x0080;
pub const WM8350_AUXADC_DCOMP3_EINT: c_uint = 0x0040;
pub const WM8350_AUXADC_DCOMP2_EINT: c_uint = 0x0020;
pub const WM8350_AUXADC_DCOMP1_EINT: c_uint = 0x0010;
pub const WM8350_SYS_HYST_COMP_FAIL_EINT: c_uint = 0x0008;
pub const WM8350_SYS_CHIP_GT115_EINT: c_uint = 0x0004;
pub const WM8350_SYS_CHIP_GT140_EINT: c_uint = 0x0002;
pub const WM8350_SYS_WDOG_TO_EINT: c_uint = 0x0001;
//
// R27 (0x1B) - Power Up Interrupt Status
//
pub const WM8350_PUTO_LDO4_EINT: c_uint = 0x0800;
pub const WM8350_PUTO_LDO3_EINT: c_uint = 0x0400;
pub const WM8350_PUTO_LDO2_EINT: c_uint = 0x0200;
pub const WM8350_PUTO_LDO1_EINT: c_uint = 0x0100;
pub const WM8350_PUTO_DC6_EINT: c_uint = 0x0020;
pub const WM8350_PUTO_DC5_EINT: c_uint = 0x0010;
pub const WM8350_PUTO_DC4_EINT: c_uint = 0x0008;
pub const WM8350_PUTO_DC3_EINT: c_uint = 0x0004;
pub const WM8350_PUTO_DC2_EINT: c_uint = 0x0002;
pub const WM8350_PUTO_DC1_EINT: c_uint = 0x0001;
//
// R28 (0x1C) - Under Voltage Interrupt status
//
pub const WM8350_UV_LDO4_EINT: c_uint = 0x0800;
pub const WM8350_UV_LDO3_EINT: c_uint = 0x0400;
pub const WM8350_UV_LDO2_EINT: c_uint = 0x0200;
pub const WM8350_UV_LDO1_EINT: c_uint = 0x0100;
pub const WM8350_UV_DC6_EINT: c_uint = 0x0020;
pub const WM8350_UV_DC5_EINT: c_uint = 0x0010;
pub const WM8350_UV_DC4_EINT: c_uint = 0x0008;
pub const WM8350_UV_DC3_EINT: c_uint = 0x0004;
pub const WM8350_UV_DC2_EINT: c_uint = 0x0002;
pub const WM8350_UV_DC1_EINT: c_uint = 0x0001;
//
// R29 (0x1D) - Over Current Interrupt status
//
pub const WM8350_OC_LS_EINT: c_uint = 0x8000;
//
// R30 (0x1E) - GPIO Interrupt Status
//
pub const WM8350_GP12_EINT: c_uint = 0x1000;
pub const WM8350_GP11_EINT: c_uint = 0x0800;
pub const WM8350_GP10_EINT: c_uint = 0x0400;
pub const WM8350_GP9_EINT: c_uint = 0x0200;
pub const WM8350_GP8_EINT: c_uint = 0x0100;
pub const WM8350_GP7_EINT: c_uint = 0x0080;
pub const WM8350_GP6_EINT: c_uint = 0x0040;
pub const WM8350_GP5_EINT: c_uint = 0x0020;
pub const WM8350_GP4_EINT: c_uint = 0x0010;
pub const WM8350_GP3_EINT: c_uint = 0x0008;
pub const WM8350_GP2_EINT: c_uint = 0x0004;
pub const WM8350_GP1_EINT: c_uint = 0x0002;
pub const WM8350_GP0_EINT: c_uint = 0x0001;
//
// R31 (0x1F) - Comparator Interrupt Status
//
pub const WM8350_EXT_USB_FB_EINT: c_uint = 0x8000;
pub const WM8350_EXT_WALL_FB_EINT: c_uint = 0x4000;
pub const WM8350_EXT_BAT_FB_EINT: c_uint = 0x2000;
pub const WM8350_CODEC_JCK_DET_L_EINT: c_uint = 0x0800;
pub const WM8350_CODEC_JCK_DET_R_EINT: c_uint = 0x0400;
pub const WM8350_CODEC_MICSCD_EINT: c_uint = 0x0200;
pub const WM8350_CODEC_MICD_EINT: c_uint = 0x0100;
pub const WM8350_WKUP_OFF_STATE_EINT: c_uint = 0x0040;
pub const WM8350_WKUP_HIB_STATE_EINT: c_uint = 0x0020;
pub const WM8350_WKUP_CONV_FAULT_EINT: c_uint = 0x0010;
pub const WM8350_WKUP_WDOG_RST_EINT: c_uint = 0x0008;
pub const WM8350_WKUP_GP_PWR_ON_EINT: c_uint = 0x0004;
pub const WM8350_WKUP_ONKEY_EINT: c_uint = 0x0002;
pub const WM8350_WKUP_GP_WAKEUP_EINT: c_uint = 0x0001;
//
// R32 (0x20) - System Interrupts Mask
//
pub const WM8350_IM_OC_INT: c_uint = 0x2000;
pub const WM8350_IM_UV_INT: c_uint = 0x1000;
pub const WM8350_IM_PUTO_INT: c_uint = 0x0800;
pub const WM8350_IM_SPARE_INT: c_uint = 0x0400;
pub const WM8350_IM_CS_INT: c_uint = 0x0200;
pub const WM8350_IM_EXT_INT: c_uint = 0x0100;
pub const WM8350_IM_CODEC_INT: c_uint = 0x0080;
pub const WM8350_IM_GP_INT: c_uint = 0x0040;
pub const WM8350_IM_AUXADC_INT: c_uint = 0x0020;
pub const WM8350_IM_RTC_INT: c_uint = 0x0010;
pub const WM8350_IM_SYS_INT: c_uint = 0x0008;
pub const WM8350_IM_CHG_INT: c_uint = 0x0004;
pub const WM8350_IM_USB_INT: c_uint = 0x0002;
pub const WM8350_IM_WKUP_INT: c_uint = 0x0001;
//
// R33 (0x21) - Interrupt Status 1 Mask
//
pub const WM8350_IM_CHG_BAT_HOT_EINT: c_uint = 0x8000;
pub const WM8350_IM_CHG_BAT_COLD_EINT: c_uint = 0x4000;
pub const WM8350_IM_CHG_BAT_FAIL_EINT: c_uint = 0x2000;
pub const WM8350_IM_CHG_TO_EINT: c_uint = 0x1000;
pub const WM8350_IM_CHG_END_EINT: c_uint = 0x0800;
pub const WM8350_IM_CHG_START_EINT: c_uint = 0x0400;
pub const WM8350_IM_CHG_FAST_RDY_EINT: c_uint = 0x0200;
pub const WM8350_IM_RTC_PER_EINT: c_uint = 0x0080;
pub const WM8350_IM_RTC_SEC_EINT: c_uint = 0x0040;
pub const WM8350_IM_RTC_ALM_EINT: c_uint = 0x0020;
pub const WM8350_IM_CHG_VBATT_LT_3P9_EINT: c_uint = 0x0004;
pub const WM8350_IM_CHG_VBATT_LT_3P1_EINT: c_uint = 0x0002;
pub const WM8350_IM_CHG_VBATT_LT_2P85_EINT: c_uint = 0x0001;
//
// R34 (0x22) - Interrupt Status 2 Mask
//
pub const WM8350_IM_SPARE2_EINT: c_uint = 0x8000;
pub const WM8350_IM_SPARE1_EINT: c_uint = 0x4000;
pub const WM8350_IM_CS1_EINT: c_uint = 0x2000;
pub const WM8350_IM_CS2_EINT: c_uint = 0x1000;
pub const WM8350_IM_USB_LIMIT_EINT: c_uint = 0x0400;
pub const WM8350_IM_AUXADC_DATARDY_EINT: c_uint = 0x0100;
pub const WM8350_IM_AUXADC_DCOMP4_EINT: c_uint = 0x0080;
pub const WM8350_IM_AUXADC_DCOMP3_EINT: c_uint = 0x0040;
pub const WM8350_IM_AUXADC_DCOMP2_EINT: c_uint = 0x0020;
pub const WM8350_IM_AUXADC_DCOMP1_EINT: c_uint = 0x0010;
pub const WM8350_IM_SYS_HYST_COMP_FAIL_EINT: c_uint = 0x0008;
pub const WM8350_IM_SYS_CHIP_GT115_EINT: c_uint = 0x0004;
pub const WM8350_IM_SYS_CHIP_GT140_EINT: c_uint = 0x0002;
pub const WM8350_IM_SYS_WDOG_TO_EINT: c_uint = 0x0001;
//
// R35 (0x23) - Power Up Interrupt Status Mask
//
pub const WM8350_IM_PUTO_LDO4_EINT: c_uint = 0x0800;
pub const WM8350_IM_PUTO_LDO3_EINT: c_uint = 0x0400;
pub const WM8350_IM_PUTO_LDO2_EINT: c_uint = 0x0200;
pub const WM8350_IM_PUTO_LDO1_EINT: c_uint = 0x0100;
pub const WM8350_IM_PUTO_DC6_EINT: c_uint = 0x0020;
pub const WM8350_IM_PUTO_DC5_EINT: c_uint = 0x0010;
pub const WM8350_IM_PUTO_DC4_EINT: c_uint = 0x0008;
pub const WM8350_IM_PUTO_DC3_EINT: c_uint = 0x0004;
pub const WM8350_IM_PUTO_DC2_EINT: c_uint = 0x0002;
pub const WM8350_IM_PUTO_DC1_EINT: c_uint = 0x0001;
//
// R36 (0x24) - Under Voltage Interrupt status Mask
//
pub const WM8350_IM_UV_LDO4_EINT: c_uint = 0x0800;
pub const WM8350_IM_UV_LDO3_EINT: c_uint = 0x0400;
pub const WM8350_IM_UV_LDO2_EINT: c_uint = 0x0200;
pub const WM8350_IM_UV_LDO1_EINT: c_uint = 0x0100;
pub const WM8350_IM_UV_DC6_EINT: c_uint = 0x0020;
pub const WM8350_IM_UV_DC5_EINT: c_uint = 0x0010;
pub const WM8350_IM_UV_DC4_EINT: c_uint = 0x0008;
pub const WM8350_IM_UV_DC3_EINT: c_uint = 0x0004;
pub const WM8350_IM_UV_DC2_EINT: c_uint = 0x0002;
pub const WM8350_IM_UV_DC1_EINT: c_uint = 0x0001;
//
// R37 (0x25) - Over Current Interrupt status Mask
//
pub const WM8350_IM_OC_LS_EINT: c_uint = 0x8000;
//
// R38 (0x26) - GPIO Interrupt Status Mask
//
pub const WM8350_IM_GP12_EINT: c_uint = 0x1000;
pub const WM8350_IM_GP11_EINT: c_uint = 0x0800;
pub const WM8350_IM_GP10_EINT: c_uint = 0x0400;
pub const WM8350_IM_GP9_EINT: c_uint = 0x0200;
pub const WM8350_IM_GP8_EINT: c_uint = 0x0100;
pub const WM8350_IM_GP7_EINT: c_uint = 0x0080;
pub const WM8350_IM_GP6_EINT: c_uint = 0x0040;
pub const WM8350_IM_GP5_EINT: c_uint = 0x0020;
pub const WM8350_IM_GP4_EINT: c_uint = 0x0010;
pub const WM8350_IM_GP3_EINT: c_uint = 0x0008;
pub const WM8350_IM_GP2_EINT: c_uint = 0x0004;
pub const WM8350_IM_GP1_EINT: c_uint = 0x0002;
pub const WM8350_IM_GP0_EINT: c_uint = 0x0001;
//
// R39 (0x27) - Comparator Interrupt Status Mask
//
pub const WM8350_IM_EXT_USB_FB_EINT: c_uint = 0x8000;
pub const WM8350_IM_EXT_WALL_FB_EINT: c_uint = 0x4000;
pub const WM8350_IM_EXT_BAT_FB_EINT: c_uint = 0x2000;
pub const WM8350_IM_CODEC_JCK_DET_L_EINT: c_uint = 0x0800;
pub const WM8350_IM_CODEC_JCK_DET_R_EINT: c_uint = 0x0400;
pub const WM8350_IM_CODEC_MICSCD_EINT: c_uint = 0x0200;
pub const WM8350_IM_CODEC_MICD_EINT: c_uint = 0x0100;
pub const WM8350_IM_WKUP_OFF_STATE_EINT: c_uint = 0x0040;
pub const WM8350_IM_WKUP_HIB_STATE_EINT: c_uint = 0x0020;
pub const WM8350_IM_WKUP_CONV_FAULT_EINT: c_uint = 0x0010;
pub const WM8350_IM_WKUP_WDOG_RST_EINT: c_uint = 0x0008;
pub const WM8350_IM_WKUP_GP_PWR_ON_EINT: c_uint = 0x0004;
pub const WM8350_IM_WKUP_ONKEY_EINT: c_uint = 0x0002;
pub const WM8350_IM_WKUP_GP_WAKEUP_EINT: c_uint = 0x0001;
//
// R220 (0xDC) - RAM BIST 1
//
pub const WM8350_READ_STATUS: c_uint = 0x0800;
pub const WM8350_TSTRAM_CLK: c_uint = 0x0100;
pub const WM8350_TSTRAM_CLK_ENA: c_uint = 0x0080;
pub const WM8350_STARTSEQ: c_uint = 0x0040;
pub const WM8350_READ_SRC: c_uint = 0x0020;
pub const WM8350_COUNT_DIR: c_uint = 0x0010;
pub const WM8350_TSTRAM_MODE_MASK: c_uint = 0x000E;
pub const WM8350_TSTRAM_ENA: c_uint = 0x0001;
//
// R225 (0xE1) - DCDC/LDO status
//
pub const WM8350_LS_STS: c_uint = 0x8000;
pub const WM8350_LDO4_STS: c_uint = 0x0800;
pub const WM8350_LDO3_STS: c_uint = 0x0400;
pub const WM8350_LDO2_STS: c_uint = 0x0200;
pub const WM8350_LDO1_STS: c_uint = 0x0100;
pub const WM8350_DC6_STS: c_uint = 0x0020;
pub const WM8350_DC5_STS: c_uint = 0x0010;
pub const WM8350_DC4_STS: c_uint = 0x0008;
pub const WM8350_DC3_STS: c_uint = 0x0004;
pub const WM8350_DC2_STS: c_uint = 0x0002;
pub const WM8350_DC1_STS: c_uint = 0x0001;
//
// R226 (0xE2) - Charger status
//
pub const WM8350_CHG_BATT_HOT_OVRDE: c_uint = 0x8000;
pub const WM8350_CHG_BATT_COLD_OVRDE: c_uint = 0x4000;
//
// R227 (0xE3) - Misc Overrides
//
pub const WM8350_USB_LIMIT_OVRDE: c_uint = 0x0400;
//
// R227 (0xE7) - Comparator Overrides
//
pub const WM8350_USB_FB_OVRDE: c_uint = 0x8000;
pub const WM8350_WALL_FB_OVRDE: c_uint = 0x4000;
pub const WM8350_BATT_FB_OVRDE: c_uint = 0x2000;
//
// R233 (0xE9) - State Machinine Status
//
pub const WM8350_USB_SM_MASK: c_uint = 0x0700;
pub const WM8350_USB_SM_SHIFT: c_int = 8;
pub const WM8350_USB_SM_100_SLV: c_int = 1;
pub const WM8350_USB_SM_500_SLV: c_int = 5;
pub const WM8350_USB_SM_STDBY_SLV: c_int = 7;
// WM8350 wake up conditions
pub const WM8350_IRQ_WKUP_OFF_STATE: c_int = 43;
pub const WM8350_IRQ_WKUP_HIB_STATE: c_int = 44;
pub const WM8350_IRQ_WKUP_CONV_FAULT: c_int = 45;
pub const WM8350_IRQ_WKUP_WDOG_RST: c_int = 46;
pub const WM8350_IRQ_WKUP_GP_PWR_ON: c_int = 47;
pub const WM8350_IRQ_WKUP_ONKEY: c_int = 48;
pub const WM8350_IRQ_WKUP_GP_WAKEUP: c_int = 49;
// wm8350 chip revisions
pub const WM8350_REV_E: c_uint = 0x4;
pub const WM8350_REV_F: c_uint = 0x5;
pub const WM8350_REV_G: c_uint = 0x6;
pub const WM8350_REV_H: c_uint = 0x7;
pub const WM8350_NUM_IRQ: c_int = 63;
pub const WM8350_NUM_IRQ_REGS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_hwmon {
    pub pdev: *mut platform_device,
    pub classdev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350 {
    pub dev: *mut device,
// device IO
    pub regmap: *mut regmap,
    pub unlocked: bool,
    pub auxadc_mutex: mutex,
    pub auxadc_done: completion,
// Interrupt handling
    pub irq_lock: mutex,
    pub chip_irq: c_int,
    pub irq_base: c_int,
    pub irq_masks: [u16; WM8350_NUM_IRQ_REGS],
// Client devices
    pub codec: wm8350_codec,
    pub gpio: wm8350_gpio,
    pub hwmon: wm8350_hwmon,
    pub pmic: wm8350_pmic,
    pub power: wm8350_power,
    pub rtc: wm8350_rtc,
    pub wdt: wm8350_wdt,
}

//
// Data to be supplied by the platform to initialise the WM8350.
//
// @init: Function called during driver initialisation.  Should be
// used by the platform to configure GPIO functions and similar.
// @irq_high: Set if WM8350 IRQ is active high.
// @irq_base: Base IRQ for genirq (not currently used).
// @gpio_base: Base for gpiolib.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8350_platform_data {
    pub wm8350): *mut *mut int (init)(struct wm8350,
    pub irq_high: c_int,
    pub irq_base: c_int,
    pub gpio_base: c_int,
}

//
// WM8350 device initialisation and exit.
//
// WM8350 device IO
//
extern "C" {
    pub fn wm8350_clear_bits(wm8350: *mut wm8350, reg: u16, mask: u16) -> c_int;
}
extern "C" {
    pub fn wm8350_set_bits(wm8350: *mut wm8350, reg: u16, mask: u16) -> c_int;
}
extern "C" {
    pub fn wm8350_reg_read(wm8350: *mut wm8350, reg: c_int) -> u16;
}
extern "C" {
    pub fn wm8350_reg_write(wm8350: *mut wm8350, reg: c_int, val: u16) -> c_int;
}
extern "C" {
    pub fn wm8350_reg_lock(wm8350: *mut wm8350) -> c_int;
}
extern "C" {
    pub fn wm8350_reg_unlock(wm8350: *mut wm8350) -> c_int;
}
extern "C" {
    pub fn wm8350_block_read(wm8350: *mut wm8350, reg: c_int, size: c_int, dest: *mut u16) -> c_int;
}
extern "C" {
    pub fn wm8350_block_write(wm8350: *mut wm8350, reg: c_int, size: c_int, src: *mut u16) -> c_int;
}
//
// WM8350 internal interrupts
//
extern "C" {
    pub fn wm8350_irq_exit(wm8350: *mut wm8350) -> c_int;
}
