//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9055/reg.h
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
// DA9055 declarations for DA9055 PMICs.
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//
// PMIC registers
//
// PAGE0
pub const DA9055_REG_PAGE_CON: c_uint = 0x00;
// System Control and Event Registers
pub const DA9055_REG_STATUS_A: c_uint = 0x01;
pub const DA9055_REG_STATUS_B: c_uint = 0x02;
pub const DA9055_REG_FAULT_LOG: c_uint = 0x03;
pub const DA9055_REG_EVENT_A: c_uint = 0x04;
pub const DA9055_REG_EVENT_B: c_uint = 0x05;
pub const DA9055_REG_EVENT_C: c_uint = 0x06;
pub const DA9055_REG_IRQ_MASK_A: c_uint = 0x07;
pub const DA9055_REG_IRQ_MASK_B: c_uint = 0x08;
pub const DA9055_REG_IRQ_MASK_C: c_uint = 0x09;
pub const DA9055_REG_CONTROL_A: c_uint = 0x0A;
pub const DA9055_REG_CONTROL_B: c_uint = 0x0B;
pub const DA9055_REG_CONTROL_C: c_uint = 0x0C;
pub const DA9055_REG_CONTROL_D: c_uint = 0x0D;
pub const DA9055_REG_CONTROL_E: c_uint = 0x0E;
pub const DA9055_REG_PD_DIS: c_uint = 0x0F;
// GPIO Control Registers
pub const DA9055_REG_GPIO0_1: c_uint = 0x10;
pub const DA9055_REG_GPIO2: c_uint = 0x11;
pub const DA9055_REG_GPIO_MODE0_2: c_uint = 0x12;
// Regulator Control Registers
pub const DA9055_REG_BCORE_CONT: c_uint = 0x13;
pub const DA9055_REG_BMEM_CONT: c_uint = 0x14;
pub const DA9055_REG_LDO1_CONT: c_uint = 0x15;
pub const DA9055_REG_LDO2_CONT: c_uint = 0x16;
pub const DA9055_REG_LDO3_CONT: c_uint = 0x17;
pub const DA9055_REG_LDO4_CONT: c_uint = 0x18;
pub const DA9055_REG_LDO5_CONT: c_uint = 0x19;
pub const DA9055_REG_LDO6_CONT: c_uint = 0x1A;
// GP-ADC Control Registers
pub const DA9055_REG_ADC_MAN: c_uint = 0x1B;
pub const DA9055_REG_ADC_CONT: c_uint = 0x1C;
pub const DA9055_REG_VSYS_MON: c_uint = 0x1D;
pub const DA9055_REG_ADC_RES_L: c_uint = 0x1E;
pub const DA9055_REG_ADC_RES_H: c_uint = 0x1F;
pub const DA9055_REG_VSYS_RES: c_uint = 0x20;
pub const DA9055_REG_ADCIN1_RES: c_uint = 0x21;
pub const DA9055_REG_ADCIN2_RES: c_uint = 0x22;
pub const DA9055_REG_ADCIN3_RES: c_uint = 0x23;
// Sequencer Control Registers
pub const DA9055_REG_EN_32K: c_uint = 0x35;
// Regulator Setting Registers
pub const DA9055_REG_BUCK_LIM: c_uint = 0x37;
pub const DA9055_REG_BCORE_MODE: c_uint = 0x38;
pub const DA9055_REG_VBCORE_A: c_uint = 0x39;
pub const DA9055_REG_VBMEM_A: c_uint = 0x3A;
pub const DA9055_REG_VLDO1_A: c_uint = 0x3B;
pub const DA9055_REG_VLDO2_A: c_uint = 0x3C;
pub const DA9055_REG_VLDO3_A: c_uint = 0x3D;
pub const DA9055_REG_VLDO4_A: c_uint = 0x3E;
pub const DA9055_REG_VLDO5_A: c_uint = 0x3F;
pub const DA9055_REG_VLDO6_A: c_uint = 0x40;
pub const DA9055_REG_VBCORE_B: c_uint = 0x41;
pub const DA9055_REG_VBMEM_B: c_uint = 0x42;
pub const DA9055_REG_VLDO1_B: c_uint = 0x43;
pub const DA9055_REG_VLDO2_B: c_uint = 0x44;
pub const DA9055_REG_VLDO3_B: c_uint = 0x45;
pub const DA9055_REG_VLDO4_B: c_uint = 0x46;
pub const DA9055_REG_VLDO5_B: c_uint = 0x47;
pub const DA9055_REG_VLDO6_B: c_uint = 0x48;
// GP-ADC Threshold Registers
pub const DA9055_REG_AUTO1_HIGH: c_uint = 0x49;
pub const DA9055_REG_AUTO1_LOW: c_uint = 0x4A;
pub const DA9055_REG_AUTO2_HIGH: c_uint = 0x4B;
pub const DA9055_REG_AUTO2_LOW: c_uint = 0x4C;
pub const DA9055_REG_AUTO3_HIGH: c_uint = 0x4D;
pub const DA9055_REG_AUTO3_LOW: c_uint = 0x4E;
// OTP
pub const DA9055_REG_OPT_COUNT: c_uint = 0x50;
pub const DA9055_REG_OPT_ADDR: c_uint = 0x51;
pub const DA9055_REG_OPT_DATA: c_uint = 0x52;
// RTC Calendar and Alarm Registers
pub const DA9055_REG_COUNT_S: c_uint = 0x53;
pub const DA9055_REG_COUNT_MI: c_uint = 0x54;
pub const DA9055_REG_COUNT_H: c_uint = 0x55;
pub const DA9055_REG_COUNT_D: c_uint = 0x56;
pub const DA9055_REG_COUNT_MO: c_uint = 0x57;
pub const DA9055_REG_COUNT_Y: c_uint = 0x58;
pub const DA9055_REG_ALARM_MI: c_uint = 0x59;
pub const DA9055_REG_ALARM_H: c_uint = 0x5A;
pub const DA9055_REG_ALARM_D: c_uint = 0x5B;
pub const DA9055_REG_ALARM_MO: c_uint = 0x5C;
pub const DA9055_REG_ALARM_Y: c_uint = 0x5D;
pub const DA9055_REG_SECOND_A: c_uint = 0x5E;
pub const DA9055_REG_SECOND_B: c_uint = 0x5F;
pub const DA9055_REG_SECOND_C: c_uint = 0x60;
pub const DA9055_REG_SECOND_D: c_uint = 0x61;
// Customer Trim and Configuration
pub const DA9055_REG_T_OFFSET: c_uint = 0x63;
pub const DA9055_REG_INTERFACE: c_uint = 0x64;
pub const DA9055_REG_CONFIG_A: c_uint = 0x65;
pub const DA9055_REG_CONFIG_B: c_uint = 0x66;
pub const DA9055_REG_CONFIG_C: c_uint = 0x67;
pub const DA9055_REG_CONFIG_D: c_uint = 0x68;
pub const DA9055_REG_CONFIG_E: c_uint = 0x69;
pub const DA9055_REG_TRIM_CLDR: c_uint = 0x6F;
// General Purpose Registers
pub const DA9055_REG_GP_ID_0: c_uint = 0x70;
pub const DA9055_REG_GP_ID_1: c_uint = 0x71;
pub const DA9055_REG_GP_ID_2: c_uint = 0x72;
pub const DA9055_REG_GP_ID_3: c_uint = 0x73;
pub const DA9055_REG_GP_ID_4: c_uint = 0x74;
pub const DA9055_REG_GP_ID_5: c_uint = 0x75;
pub const DA9055_REG_GP_ID_6: c_uint = 0x76;
pub const DA9055_REG_GP_ID_7: c_uint = 0x77;
pub const DA9055_REG_GP_ID_8: c_uint = 0x78;
pub const DA9055_REG_GP_ID_9: c_uint = 0x79;
pub const DA9055_REG_GP_ID_10: c_uint = 0x7A;
pub const DA9055_REG_GP_ID_11: c_uint = 0x7B;
pub const DA9055_REG_GP_ID_12: c_uint = 0x7C;
pub const DA9055_REG_GP_ID_13: c_uint = 0x7D;
pub const DA9055_REG_GP_ID_14: c_uint = 0x7E;
pub const DA9055_REG_GP_ID_15: c_uint = 0x7F;
pub const DA9055_REG_GP_ID_16: c_uint = 0x80;
pub const DA9055_REG_GP_ID_17: c_uint = 0x81;
pub const DA9055_REG_GP_ID_18: c_uint = 0x82;
pub const DA9055_REG_GP_ID_19: c_uint = 0x83;

//
// PMIC registers bits
//
// DA9055_REG_PAGE_CON (addr=0x00)

// DA9055_REG_STATUS_A (addr=0x01)
pub const DA9055_NOKEY_STS: c_uint = 0x01;
pub const DA9055_WAKE_STS: c_uint = 0x02;
pub const DA9055_DVC_BUSY_STS: c_uint = 0x04;
pub const DA9055_COMP1V2_STS: c_uint = 0x08;
pub const DA9055_NJIG_STS: c_uint = 0x10;
pub const DA9055_LDO5_LIM_STS: c_uint = 0x20;
pub const DA9055_LDO6_LIM_STS: c_uint = 0x40;
// DA9055_REG_STATUS_B (addr=0x02)
pub const DA9055_GPI0_STS: c_uint = 0x01;
pub const DA9055_GPI1_STS: c_uint = 0x02;
pub const DA9055_GPI2_STS: c_uint = 0x04;
// DA9055_REG_FAULT_LOG (addr=0x03)
pub const DA9055_TWD_ERROR_FLG: c_uint = 0x01;
pub const DA9055_POR_FLG: c_uint = 0x02;
pub const DA9055_VDD_FAULT_FLG: c_uint = 0x04;
pub const DA9055_VDD_START_FLG: c_uint = 0x08;
pub const DA9055_TEMP_CRIT_FLG: c_uint = 0x10;
pub const DA9055_KEY_RESET_FLG: c_uint = 0x20;
pub const DA9055_WAIT_SHUT_FLG: c_uint = 0x80;
// DA9055_REG_EVENT_A (addr=0x04)
pub const DA9055_NOKEY_EINT: c_uint = 0x01;
pub const DA9055_ALARM_EINT: c_uint = 0x02;
pub const DA9055_TICK_EINT: c_uint = 0x04;
pub const DA9055_ADC_RDY_EINT: c_uint = 0x08;
pub const DA9055_SEQ_RDY_EINT: c_uint = 0x10;
pub const DA9055_EVENTS_B_EINT: c_uint = 0x20;
pub const DA9055_EVENTS_C_EINT: c_uint = 0x40;
// DA9055_REG_EVENT_B (addr=0x05)
pub const DA9055_E_WAKE_EINT: c_uint = 0x01;
pub const DA9055_E_TEMP_EINT: c_uint = 0x02;
pub const DA9055_E_COMP1V2_EINT: c_uint = 0x04;
pub const DA9055_E_LDO_LIM_EINT: c_uint = 0x08;
pub const DA9055_E_NJIG_EINT: c_uint = 0x20;
pub const DA9055_E_VDD_MON_EINT: c_uint = 0x40;
pub const DA9055_E_VDD_WARN_EINT: c_uint = 0x80;
// DA9055_REG_EVENT_C (addr=0x06)
pub const DA9055_E_GPI0_EINT: c_uint = 0x01;
pub const DA9055_E_GPI1_EINT: c_uint = 0x02;
pub const DA9055_E_GPI2_EINT: c_uint = 0x04;
// DA9055_REG_IRQ_MASK_A (addr=0x07)
pub const DA9055_M_NONKEY_EINT: c_uint = 0x01;
pub const DA9055_M_ALARM_EINT: c_uint = 0x02;
pub const DA9055_M_TICK_EINT: c_uint = 0x04;
pub const DA9055_M_ADC_RDY_EINT: c_uint = 0x08;
pub const DA9055_M_SEQ_RDY_EINT: c_uint = 0x10;
// DA9055_REG_IRQ_MASK_B (addr=0x08)
pub const DA9055_M_WAKE_EINT: c_uint = 0x01;
pub const DA9055_M_TEMP_EINT: c_uint = 0x02;
pub const DA9055_M_COMP_1V2_EINT: c_uint = 0x04;
pub const DA9055_M_LDO_LIM_EINT: c_uint = 0x08;
pub const DA9055_M_NJIG_EINT: c_uint = 0x20;
pub const DA9055_M_VDD_MON_EINT: c_uint = 0x40;
pub const DA9055_M_VDD_WARN_EINT: c_uint = 0x80;
// DA9055_REG_IRQ_MASK_C (addr=0x09)
pub const DA9055_M_GPI0_EINT: c_uint = 0x01;
pub const DA9055_M_GPI1_EINT: c_uint = 0x02;
pub const DA9055_M_GPI2_EINT: c_uint = 0x04;
// DA9055_REG_CONTROL_A (addr=0xA)
pub const DA9055_DEBOUNCING_SHIFT: c_uint = 0x00;
pub const DA9055_DEBOUNCING_MASK: c_uint = 0x07;
pub const DA9055_NRES_MODE_SHIFT: c_uint = 0x03;
pub const DA9055_NRES_MODE_MASK: c_uint = 0x08;
pub const DA9055_SLEW_RATE_SHIFT: c_uint = 0x04;
pub const DA9055_SLEW_RATE_MASK: c_uint = 0x30;
pub const DA9055_NOKEY_LOCK_SHIFT: c_uint = 0x06;
pub const DA9055_NOKEY_LOCK_MASK: c_uint = 0x40;
// DA9055_REG_CONTROL_B (addr=0xB)
pub const DA9055_RTC_MODE_PD: c_uint = 0x01;
pub const DA9055_RTC_MODE_SD_SHIFT: c_uint = 0x01;
pub const DA9055_RTC_MODE_SD: c_uint = 0x02;
pub const DA9055_RTC_EN: c_uint = 0x04;
pub const DA9055_ECO_MODE_SHIFT: c_uint = 0x03;
pub const DA9055_ECO_MODE_MASK: c_uint = 0x08;
pub const DA9055_TWDSCALE_SHIFT: c_int = 4;
pub const DA9055_TWDSCALE_MASK: c_uint = 0x70;
pub const DA9055_V_LOCK_SHIFT: c_uint = 0x07;
pub const DA9055_V_LOCK_MASK: c_uint = 0x80;
// DA9055_REG_CONTROL_C (addr=0xC)
pub const DA9055_SYSTEM_EN_SHIFT: c_uint = 0x00;
pub const DA9055_SYSTEM_EN_MASK: c_uint = 0x01;
pub const DA9055_POWERN_EN_SHIFT: c_uint = 0x01;
pub const DA9055_POWERN_EN_MASK: c_uint = 0x02;
pub const DA9055_POWER1_EN_SHIFT: c_uint = 0x02;
pub const DA9055_POWER1_EN_MASK: c_uint = 0x04;
// DA9055_REG_CONTROL_D (addr=0xD)
pub const DA9055_STANDBY_SHIFT: c_uint = 0x02;
pub const DA9055_STANDBY_MASK: c_uint = 0x08;
pub const DA9055_AUTO_BOOT_SHIFT: c_uint = 0x03;
pub const DA9055_AUTO_BOOT_MASK: c_uint = 0x04;
// DA9055_REG_CONTROL_E (addr=0xE)
pub const DA9055_WATCHDOG_SHIFT: c_uint = 0x00;
pub const DA9055_WATCHDOG_MASK: c_uint = 0x01;
pub const DA9055_SHUTDOWN_SHIFT: c_uint = 0x01;
pub const DA9055_SHUTDOWN_MASK: c_uint = 0x02;
pub const DA9055_WAKE_UP_SHIFT: c_uint = 0x02;
pub const DA9055_WAKE_UP_MASK: c_uint = 0x04;
// DA9055_REG_GPIO (addr=0x10/0x11)
pub const DA9055_GPIO0_PIN_SHIFT: c_uint = 0x00;
pub const DA9055_GPIO0_PIN_MASK: c_uint = 0x03;
pub const DA9055_GPIO0_TYPE_SHIFT: c_uint = 0x02;
pub const DA9055_GPIO0_TYPE_MASK: c_uint = 0x04;
pub const DA9055_GPIO0_WEN_SHIFT: c_uint = 0x03;
pub const DA9055_GPIO0_WEN_MASK: c_uint = 0x08;
pub const DA9055_GPIO1_PIN_SHIFT: c_uint = 0x04;
pub const DA9055_GPIO1_PIN_MASK: c_uint = 0x30;
pub const DA9055_GPIO1_TYPE_SHIFT: c_uint = 0x06;
pub const DA9055_GPIO1_TYPE_MASK: c_uint = 0x40;
pub const DA9055_GPIO1_WEN_SHIFT: c_uint = 0x07;
pub const DA9055_GPIO1_WEN_MASK: c_uint = 0x80;
pub const DA9055_GPIO2_PIN_SHIFT: c_uint = 0x00;
pub const DA9055_GPIO2_PIN_MASK: c_uint = 0x30;
pub const DA9055_GPIO2_TYPE_SHIFT: c_uint = 0x02;
pub const DA9055_GPIO2_TYPE_MASK: c_uint = 0x04;
pub const DA9055_GPIO2_WEN_SHIFT: c_uint = 0x03;
pub const DA9055_GPIO2_WEN_MASK: c_uint = 0x08;
// DA9055_REG_GPIO_MODE (addr=0x12)
pub const DA9055_GPIO0_MODE_SHIFT: c_uint = 0x00;
pub const DA9055_GPIO0_MODE_MASK: c_uint = 0x01;
pub const DA9055_GPIO1_MODE_SHIFT: c_uint = 0x01;
pub const DA9055_GPIO1_MODE_MASK: c_uint = 0x02;
pub const DA9055_GPIO2_MODE_SHIFT: c_uint = 0x02;
pub const DA9055_GPIO2_MODE_MASK: c_uint = 0x04;
// DA9055_REG_BCORE_CONT (addr=0x13)
pub const DA9055_BCORE_EN_SHIFT: c_uint = 0x00;
pub const DA9055_BCORE_EN_MASK: c_uint = 0x01;
pub const DA9055_BCORE_GPI_SHIFT: c_uint = 0x01;
pub const DA9055_BCORE_GPI_MASK: c_uint = 0x02;
pub const DA9055_BCORE_PD_DIS_SHIFT: c_uint = 0x03;
pub const DA9055_BCORE_PD_DIS_MASK: c_uint = 0x04;
pub const DA9055_VBCORE_SEL_SHIFT: c_uint = 0x04;
pub const DA9055_SEL_REG_A: c_uint = 0x0;
pub const DA9055_SEL_REG_B: c_uint = 0x10;
pub const DA9055_VBCORE_SEL_MASK: c_uint = 0x10;
pub const DA9055_V_GPI_MASK: c_uint = 0x60;
pub const DA9055_V_GPI_SHIFT: c_uint = 0x05;
pub const DA9055_E_GPI_MASK: c_uint = 0x06;
pub const DA9055_E_GPI_SHIFT: c_uint = 0x01;
pub const DA9055_VBCORE_GPI_SHIFT: c_uint = 0x05;
pub const DA9055_VBCORE_GPI_MASK: c_uint = 0x60;
pub const DA9055_BCORE_CONF_SHIFT: c_uint = 0x07;
pub const DA9055_BCORE_CONF_MASK: c_uint = 0x80;
// DA9055_REG_BMEM_CONT (addr=0x14)
pub const DA9055_BMEM_EN_SHIFT: c_uint = 0x00;
pub const DA9055_BMEM_EN_MASK: c_uint = 0x01;
pub const DA9055_BMEM_GPI_SHIFT: c_uint = 0x01;
pub const DA9055_BMEM_GPI_MASK: c_uint = 0x06;
pub const DA9055_BMEM_PD_DIS_SHIFT: c_uint = 0x03;
pub const DA9055_BMEM_PD_DIS_MASK: c_uint = 0x08;
pub const DA9055_VBMEM_SEL_SHIT: c_uint = 0x04;

pub const DA9055_VBMEM_SEL_MASK: c_uint = 0x10;
pub const DA9055_VBMEM_GPI_SHIFT: c_uint = 0x05;
pub const DA9055_VBMEM_GPI_MASK: c_uint = 0x60;
pub const DA9055_BMEM_CONF_SHIFT: c_uint = 0x07;
pub const DA9055_BMEM_CONF_MASK: c_uint = 0x80;
// DA9055_REG_LDO_CONT (addr=0x15-0x1A)
pub const DA9055_LDO_EN_SHIFT: c_uint = 0x00;
pub const DA9055_LDO_EN_MASK: c_uint = 0x01;
pub const DA9055_LDO_GPI_SHIFT: c_uint = 0x01;
pub const DA9055_LDO_GPI_MASK: c_uint = 0x06;
pub const DA9055_LDO_PD_DIS_SHIFT: c_uint = 0x03;
pub const DA9055_LDO_PD_DIS_MASK: c_uint = 0x08;
pub const DA9055_VLDO_SEL_SHIFT: c_uint = 0x04;
pub const DA9055_VLDO_SEL_MASK: c_uint = 0x10;
pub const DA9055_VLDO_SEL_VLDO_A: c_uint = 0x00;
pub const DA9055_VLDO_SEL_VLDO_B: c_uint = 0x01;
pub const DA9055_VLDO_GPI_SHIFT: c_uint = 0x05;
pub const DA9055_VLDO_GPI_MASK: c_uint = 0x60;
pub const DA9055_LDO_CONF_SHIFT: c_uint = 0x07;
pub const DA9055_LDO_CONF_MASK: c_uint = 0x80;
pub const DA9055_REGUALTOR_SET_A: c_uint = 0x00;
pub const DA9055_REGUALTOR_SET_B: c_uint = 0x10;
// DA9055_REG_ADC_MAN (addr=0x1B)
pub const DA9055_ADC_MUX_SHIFT: c_int = 0;
pub const DA9055_ADC_MUX_MASK: c_uint = 0xF;
pub const DA9055_ADC_MUX_VSYS: c_uint = 0x0;
pub const DA9055_ADC_MUX_ADCIN1: c_uint = 0x01;
pub const DA9055_ADC_MUX_ADCIN2: c_uint = 0x02;
pub const DA9055_ADC_MUX_ADCIN3: c_uint = 0x03;
pub const DA9055_ADC_MUX_T_SENSE: c_uint = 0x04;
pub const DA9055_ADC_MAN_SHIFT: c_uint = 0x04;
pub const DA9055_ADC_MAN_CONV: c_uint = 0x10;

pub const DA9055_ADC_MODE_MASK: c_uint = 0x20;
pub const DA9055_ADC_MODE_SHIFT: c_int = 5;

pub const DA9055_COMP1V2_EN_SHIFT: c_int = 7;
// DA9055_REG_ADC_CONT (addr=0x1C)
pub const DA9055_ADC_AUTO_VSYS_EN_SHIFT: c_int = 0;
pub const DA9055_ADC_AUTO_AD1_EN_SHIFT: c_int = 1;
pub const DA9055_ADC_AUTO_AD2_EN_SHIFT: c_int = 2;
pub const DA9055_ADC_AUTO_AD3_EN_SHIFT: c_int = 3;
pub const DA9055_ADC_ISRC_EN_SHIFT: c_int = 4;
pub const DA9055_ADC_ADCIN1_DEB_SHIFT: c_int = 5;
pub const DA9055_ADC_ADCIN2_DEB_SHIFT: c_int = 6;
pub const DA9055_ADC_ADCIN3_DEB_SHIFT: c_int = 7;
pub const DA9055_AD1_ISRC_MASK: c_uint = 0x10;
pub const DA9055_AD1_ISRC_SHIFT: c_int = 4;
// DA9055_REG_VSYS_MON (addr=0x1D)
pub const DA9055_VSYS_VAL_SHIFT: c_int = 0;
pub const DA9055_VSYS_VAL_MASK: c_uint = 0xFF;
pub const DA9055_VSYS_VAL_BASE: c_uint = 0x00;

pub const DA9055_VSYS_VOLT_BASE: c_int = 2500;
pub const DA9055_VSYS_VOLT_INC: c_int = 10;
pub const DA9055_VSYS_STEPS: c_int = 255;
pub const DA9055_VSYS_VOLT_MIN: c_int = 2500;
// DA9044_REG_XXX_RES (addr=0x20-0x23)
pub const DA9055_ADC_VAL_SHIFT: c_int = 0;
pub const DA9055_ADC_VAL_MASK: c_uint = 0xFF;
pub const DA9055_ADC_VAL_BASE: c_uint = 0x00;

pub const DA9055_ADC_VOLT_BASE: c_int = 0;
pub const DA9055_ADC_VSYS_VOLT_BASE: c_int = 2500;
pub const DA9055_ADC_VOLT_INC: c_int = 10;
pub const DA9055_ADC_VSYS_VOLT_INC: c_int = 12;
pub const DA9055_ADC_STEPS: c_int = 255;
// DA9055_REG_EN_32K  (addr=0x35)
pub const DA9055_STARTUP_TIME_MASK: c_uint = 0x07;
pub const DA9055_STARTUP_TIME_0S: c_uint = 0x0;
pub const DA9055_STARTUP_TIME_0_52S: c_uint = 0x1;
pub const DA9055_STARTUP_TIME_1S: c_uint = 0x2;
pub const DA9055_CRYSTAL_EN: c_uint = 0x08;
pub const DA9055_DELAY_MODE_EN: c_uint = 0x10;
pub const DA9055_OUT_CLCK_GATED: c_uint = 0x20;
pub const DA9055_RTC_CLOCK_GATED: c_uint = 0x40;
pub const DA9055_EN_32KOUT_BUF: c_uint = 0x80;
// DA9055_REG_RESET (addr=0x36)
// Timer up to 31.744 ms
pub const DA9055_RESET_TIMER_VAL_SHIFT: c_int = 0;
pub const DA9055_RESET_LOW_VAL_MASK: c_uint = 0x3F;
pub const DA9055_RESET_LOW_VAL_BASE: c_int = 0;

pub const DA9055_RESET_US_LOW_STEP: c_int = 30;
// Timer up to 1048.576ms
pub const DA9055_RESET_HIGH_VAL_MASK: c_uint = 0x3F;
pub const DA9055_RESET_HIGH_VAL_BASE: c_int = 0;

pub const DA9055_RESET_US_HIGH_STEP: c_int = 31;
// DA9055_REG_BUCK_ILIM (addr=0x37)
pub const DA9055_BMEM_ILIM_SHIFT: c_int = 0;
pub const DA9055_ILIM_MASK: c_uint = 0x3;
pub const DA9055_ILIM_500MA: c_uint = 0x0;
pub const DA9055_ILIM_600MA: c_uint = 0x1;
pub const DA9055_ILIM_700MA: c_uint = 0x2;
pub const DA9055_ILIM_800MA: c_uint = 0x3;
pub const DA9055_BCORE_ILIM_SHIFT: c_int = 2;
// DA9055_REG_BCORE_MODE (addr=0x38)
pub const DA9055_BMEM_MODE_SHIFT: c_int = 0;
pub const DA9055_MODE_MASK: c_uint = 0x3;
pub const DA9055_MODE_AB: c_uint = 0x0;
pub const DA9055_MODE_SLEEP: c_uint = 0x1;
pub const DA9055_MODE_SYNCHRO: c_uint = 0x2;
pub const DA9055_MODE_AUTO: c_uint = 0x3;
pub const DA9055_BCORE_MODE_SHIFT: c_int = 2;
// DA9055_REG_VBCORE_A/B (addr=0x39/0x41)
pub const DA9055_VBCORE_VAL_SHIFT: c_int = 0;
pub const DA9055_VBCORE_VAL_MASK: c_uint = 0x3F;
pub const DA9055_VBCORE_VAL_BASE: c_uint = 0x09;

pub const DA9055_VBCORE_VOLT_BASE: c_int = 750;
pub const DA9055_VBCORE_VOLT_INC: c_int = 25;
pub const DA9055_VBCORE_STEPS: c_int = 53;

// DA9055_REG_VBMEM_A/B (addr=0x3A/0x42)
pub const DA9055_VBMEM_VAL_SHIFT: c_int = 0;
pub const DA9055_VBMEM_VAL_MASK: c_uint = 0x3F;
pub const DA9055_VBMEM_VAL_BASE: c_uint = 0x00;

pub const DA9055_VBMEM_VOLT_BASE: c_int = 925;
pub const DA9055_VBMEM_VOLT_INC: c_int = 25;
pub const DA9055_VBMEM_STEPS: c_int = 63;

// DA9055_REG_VLDO (addr=0x3B-0x40/0x43-0x48)
pub const DA9055_VLDO_VAL_SHIFT: c_int = 0;
pub const DA9055_VLDO_VAL_MASK: c_uint = 0x3F;
pub const DA9055_VLDO6_VAL_MASK: c_uint = 0x7F;
pub const DA9055_VLDO_VAL_BASE: c_uint = 0x02;
pub const DA9055_VLDO2_VAL_BASE: c_uint = 0x03;
pub const DA9055_VLDO6_VAL_BASE: c_uint = 0x00;

pub const DA9055_VLDO_VOLT_BASE: c_int = 900;
pub const DA9055_VLDO_VOLT_INC: c_int = 50;
pub const DA9055_VLDO6_VOLT_INC: c_int = 20;
pub const DA9055_VLDO_STEPS: c_int = 48;
pub const DA9055_VLDO5_STEPS: c_int = 37;
pub const DA9055_VLDO6_STEPS: c_int = 120;

pub const DA9055_LDO_MODE_SHIFT: c_int = 7;
pub const DA9055_LDO_SL_NORMAL: c_int = 0;
pub const DA9055_LDO_SL_SLEEP: c_int = 1;
// DA9055_REG_OTP_CONT (addr=0x50)

pub const DA9055_OTP_GP_RD_SHIFT: c_int = 1;
pub const DA9055_OTP_APPS_RD_SHIFT: c_int = 2;
pub const DA9055_PC_DONE_SHIFT: c_int = 3;
pub const DA9055_OTP_GP_LOCK_SHIFT: c_int = 4;
pub const DA9055_OTP_APPS_LOCK_SHIFT: c_int = 5;
pub const DA9055_OTP_CONF_LOCK_SHIFT: c_int = 6;
pub const DA9055_OTP_WRITE_DIS_SHIFT: c_int = 7;
// DA9055_REG_COUNT_S (addr=0x53)
pub const DA9055_RTC_SEC: c_uint = 0x3F;
pub const DA9055_RTC_MONITOR_EN: c_uint = 0x40;
pub const DA9055_RTC_READ: c_uint = 0x80;
// DA9055_REG_COUNT_MI (addr=0x54)
pub const DA9055_RTC_MIN: c_uint = 0x3F;
// DA9055_REG_COUNT_H (addr=0x55)
pub const DA9055_RTC_HOUR: c_uint = 0x1F;
// DA9055_REG_COUNT_D (addr=0x56)
pub const DA9055_RTC_DAY: c_uint = 0x1F;
// DA9055_REG_COUNT_MO (addr=0x57)
pub const DA9055_RTC_MONTH: c_uint = 0x0F;
// DA9055_REG_COUNT_Y (addr=0x58)
pub const DA9055_RTC_YEAR: c_uint = 0x3F;
pub const DA9055_RTC_YEAR_BASE: c_int = 2000;
// DA9055_REG_ALARM_MI (addr=0x59)
pub const DA9055_RTC_ALM_MIN: c_uint = 0x3F;
pub const DA9055_ALARM_STATUS_SHIFT: c_int = 6;
pub const DA9055_ALARM_STATUS_MASK: c_uint = 0x3;
pub const DA9055_ALARM_STATUS_NO_ALARM: c_uint = 0x0;
pub const DA9055_ALARM_STATUS_TICK: c_uint = 0x1;
pub const DA9055_ALARM_STATUS_TIMER_ALARM: c_uint = 0x2;
pub const DA9055_ALARM_STATUS_BOTH: c_uint = 0x3;
// DA9055_REG_ALARM_H (addr=0x5A)
pub const DA9055_RTC_ALM_HOUR: c_uint = 0x1F;
// DA9055_REG_ALARM_D (addr=0x5B)
pub const DA9055_RTC_ALM_DAY: c_uint = 0x1F;
// DA9055_REG_ALARM_MO (addr=0x5C)
pub const DA9055_RTC_ALM_MONTH: c_uint = 0x0F;
pub const DA9055_RTC_TICK_WAKE_MASK: c_uint = 0x20;
pub const DA9055_RTC_TICK_WAKE_SHIFT: c_int = 5;
pub const DA9055_RTC_TICK_TYPE: c_uint = 0x10;
pub const DA9055_RTC_TICK_TYPE_SHIFT: c_uint = 0x4;
pub const DA9055_RTC_TICK_SEC: c_uint = 0x0;
pub const DA9055_RTC_TICK_MIN: c_uint = 0x1;
pub const DA9055_ALARAM_TICK_WAKE: c_uint = 0x20;
// DA9055_REG_ALARM_Y (addr=0x5D)
pub const DA9055_RTC_TICK_EN: c_uint = 0x80;
pub const DA9055_RTC_ALM_EN: c_uint = 0x40;
pub const DA9055_RTC_TICK_ALM_MASK: c_uint = 0xC0;
pub const DA9055_RTC_ALM_YEAR: c_uint = 0x3F;
// DA9055_REG_TRIM_CLDR (addr=0x62)
pub const DA9055_TRIM_32K_SHIFT: c_int = 0;
pub const DA9055_TRIM_32K_MASK: c_uint = 0x7F;

pub const DA9055_TRIM_VAL_BASE: c_uint = 0x0;
pub const DA9055_TRIM_PPM_BASE: c_uint = 0x0 /* min val in units of 0.1PPM */;

pub const DA9055_TRIM_STEPS: c_int = 127;
// DA9055_REG_CONFIG_A (addr=0x65)

// DA9055_REG_CONFIG_B (addr=0x66)
pub const DA9055_VDD_FAULT_VAL_SHIFT: c_int = 0;
pub const DA9055_VDD_FAULT_VAL_MASK: c_uint = 0xF;
pub const DA9055_VDD_FAULT_VAL_BASE: c_uint = 0x0;

pub const DA9055_VDD_FAULT_VOLT_BASE: c_int = 2500;
pub const DA9055_VDD_FAULT_VOLT_INC: c_int = 50;
pub const DA9055_VDD_FAULT_STEPS: c_int = 15;
pub const DA9055_VDD_HYST_VAL_SHIFT: c_int = 4;
pub const DA9055_VDD_HYST_VAL_MASK: c_uint = 0x7;
pub const DA9055_VDD_HYST_VAL_BASE: c_uint = 0x0;

pub const DA9055_VDD_HYST_VOLT_BASE: c_int = 100;
pub const DA9055_VDD_HYST_VOLT_INC: c_int = 50;
pub const DA9055_VDD_HYST_STEPS: c_int = 7;

pub const DA9055_VDD_FAULT_EN_SHIFT: c_int = 7;
// DA9055_REG_CONFIG_C (addr=0x67)
pub const DA9055_BCORE_CLK_INV_SHIFT: c_int = 0;
pub const DA9055_BMEM_CLK_INV_SHIFT: c_int = 1;
pub const DA9055_NFAULT_CONF_SHIFT: c_int = 2;
pub const DA9055_LDO_SD_SHIFT: c_int = 4;
pub const DA9055_LDO5_BYP_SHIFT: c_int = 6;
pub const DA9055_LDO6_BYP_SHIFT: c_int = 7;
// DA9055_REG_CONFIG_D (addr=0x68)
pub const DA9055_NONKEY_PIN_SHIFT: c_int = 0;
pub const DA9055_NONKEY_PIN_MASK: c_uint = 0x3;
pub const DA9055_NONKEY_PIN_PORT_MODE: c_uint = 0x0;
pub const DA9055_NONKEY_PIN_KEY_MODE: c_uint = 0x1;
pub const DA9055_NONKEY_PIN_MULTI_FUNC: c_uint = 0x2;
pub const DA9055_NONKEY_PIN_DEDICT: c_uint = 0x3;
pub const DA9055_NONKEY_SD_SHIFT: c_int = 2;
pub const DA9055_KEY_DELAY_SHIFT: c_int = 3;
pub const DA9055_KEY_DELAY_MASK: c_uint = 0x3;
pub const DA9055_KEY_DELAY_4S: c_uint = 0x0;
pub const DA9055_KEY_DELAY_6S: c_uint = 0x1;
pub const DA9055_KEY_DELAY_8S: c_uint = 0x2;
pub const DA9055_KEY_DELAY_10S: c_uint = 0x3;
// DA9055_REG_CONFIG_E (addr=0x69)
pub const DA9055_GPIO_PUPD_PULL_UP: c_uint = 0x0;
pub const DA9055_GPIO_PUPD_OPEN_DRAIN: c_uint = 0x1;
pub const DA9055_GPIO0_PUPD_SHIFT: c_int = 0;
pub const DA9055_GPIO1_PUPD_SHIFT: c_int = 1;
pub const DA9055_GPIO2_PUPD_SHIFT: c_int = 2;
pub const DA9055_UVOV_DELAY_SHIFT: c_int = 4;
pub const DA9055_UVOV_DELAY_MASK: c_uint = 0x3;
pub const DA9055_RESET_DURATION_SHIFT: c_int = 6;
pub const DA9055_RESET_DURATION_MASK: c_uint = 0x3;
pub const DA9055_RESET_DURATION_0MS: c_uint = 0x0;
pub const DA9055_RESET_DURATION_100MS: c_uint = 0x1;
pub const DA9055_RESET_DURATION_500MS: c_uint = 0x2;
pub const DA9055_RESET_DURATION_1000MS: c_uint = 0x3;
// DA9055_REG_MON_REG_1 (addr=0x6A)
pub const DA9055_MON_THRES_SHIFT: c_int = 0;
pub const DA9055_MON_THRES_MASK: c_uint = 0x3;
pub const DA9055_MON_RES_SHIFT: c_int = 2;
pub const DA9055_MON_DEB_SHIFT: c_int = 3;
pub const DA9055_MON_MODE_SHIFT: c_int = 4;
pub const DA9055_MON_MODE_MASK: c_uint = 0x3;
pub const DA9055_START_MAX_SHIFT: c_int = 6;
pub const DA9055_START_MAX_MASK: c_uint = 0x3;
// DA9055_REG_MON_REG_2 (addr=0x6B)
pub const DA9055_LDO1_MON_EN_SHIFT: c_int = 0;
pub const DA9055_LDO2_MON_EN_SHIFT: c_int = 1;
pub const DA9055_LDO3_MON_EN_SHIFT: c_int = 2;
pub const DA9055_LDO4_MON_EN_SHIFT: c_int = 3;
pub const DA9055_LDO5_MON_EN_SHIFT: c_int = 4;
pub const DA9055_LDO6_MON_EN_SHIFT: c_int = 5;
pub const DA9055_BCORE_MON_EN_SHIFT: c_int = 6;
pub const DA9055_BMEM_MON_EN_SHIFT: c_int = 7;
// DA9055_REG_CONFIG_F (addr=0x6C)
pub const DA9055_LDO1_DEF_SHIFT: c_int = 0;
pub const DA9055_LDO2_DEF_SHIFT: c_int = 1;
pub const DA9055_LDO3_DEF_SHIFT: c_int = 2;
pub const DA9055_LDO4_DEF_SHIFT: c_int = 3;
pub const DA9055_LDO5_DEF_SHIFT: c_int = 4;
pub const DA9055_LDO6_DEF_SHIFT: c_int = 5;
pub const DA9055_BCORE_DEF_SHIFT: c_int = 6;
pub const DA9055_BMEM_DEF_SHIFT: c_int = 7;
// DA9055_REG_MON_REG_4 (addr=0x6D)
pub const DA9055_MON_A8_IDX_SHIFT: c_int = 0;
pub const DA9055_MON_A89_IDX_MASK: c_uint = 0x3;
pub const DA9055_MON_A89_IDX_NONE: c_uint = 0x0;
pub const DA9055_MON_A89_IDX_BUCKCORE: c_uint = 0x1;
pub const DA9055_MON_A89_IDX_LDO3: c_uint = 0x2;
pub const DA9055_MON_A9_IDX_SHIFT: c_int = 5;
// DA9055_REG_MON_REG_5 (addr=0x6E)
pub const DA9055_MON_A10_IDX_SHIFT: c_int = 0;
pub const DA9055_MON_A10_IDX_MASK: c_uint = 0x3;
pub const DA9055_MON_A10_IDX_NONE: c_uint = 0x0;
pub const DA9055_MON_A10_IDX_LDO1: c_uint = 0x1;
pub const DA9055_MON_A10_IDX_LDO2: c_uint = 0x2;
pub const DA9055_MON_A10_IDX_LDO5: c_uint = 0x3;
pub const DA9055_MON_A10_IDX_LDO6: c_uint = 0x4;
