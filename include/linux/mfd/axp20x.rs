//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/axp20x.h
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
//
// Functions and registers to access AXP20X power management chip.
//
// Copyright (C) 2013, Carlo Caione <carlo@caione.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp20x_variants {
    AXP152_ID = 0,
    AXP192_ID,
    AXP202_ID,
    AXP209_ID,
    AXP221_ID,
    AXP223_ID,
    AXP288_ID,
    AXP313A_ID,
    AXP323_ID,
    AXP717_ID,
    AXP803_ID,
    AXP806_ID,
    AXP809_ID,
    AXP813_ID,
    AXP15060_ID,
    NR_AXP20X_VARIANTS,
}

// Power supply
pub const AXP152_PWR_OP_MODE: c_uint = 0x01;
pub const AXP152_LDO3456_DC1234_CTRL: c_uint = 0x12;
pub const AXP152_ALDO_OP_MODE: c_uint = 0x13;
pub const AXP152_LDO0_CTRL: c_uint = 0x15;
pub const AXP152_DCDC2_V_OUT: c_uint = 0x23;
pub const AXP152_DCDC2_V_RAMP: c_uint = 0x25;
pub const AXP152_DCDC1_V_OUT: c_uint = 0x26;
pub const AXP152_DCDC3_V_OUT: c_uint = 0x27;
pub const AXP152_ALDO12_V_OUT: c_uint = 0x28;
pub const AXP152_DLDO1_V_OUT: c_uint = 0x29;
pub const AXP152_DLDO2_V_OUT: c_uint = 0x2a;
pub const AXP152_DCDC4_V_OUT: c_uint = 0x2b;
pub const AXP152_V_OFF: c_uint = 0x31;
pub const AXP152_OFF_CTRL: c_uint = 0x32;
pub const AXP152_PEK_KEY: c_uint = 0x36;
pub const AXP152_DCDC_FREQ: c_uint = 0x37;
pub const AXP152_DCDC_MODE: c_uint = 0x80;
pub const AXP192_USB_OTG_STATUS: c_uint = 0x04;
pub const AXP192_PWR_OUT_CTRL: c_uint = 0x12;
pub const AXP192_DCDC2_V_OUT: c_uint = 0x23;
pub const AXP192_DCDC1_V_OUT: c_uint = 0x26;
pub const AXP192_DCDC3_V_OUT: c_uint = 0x27;
pub const AXP192_LDO2_3_V_OUT: c_uint = 0x28;
pub const AXP20X_PWR_INPUT_STATUS: c_uint = 0x00;
pub const AXP20X_PWR_OP_MODE: c_uint = 0x01;
pub const AXP20X_USB_OTG_STATUS: c_uint = 0x02;
pub const AXP20X_PWR_OUT_CTRL: c_uint = 0x12;
pub const AXP20X_DCDC2_V_OUT: c_uint = 0x23;
pub const AXP20X_DCDC2_LDO3_V_RAMP: c_uint = 0x25;
pub const AXP20X_DCDC3_V_OUT: c_uint = 0x27;
pub const AXP20X_LDO24_V_OUT: c_uint = 0x28;
pub const AXP20X_LDO3_V_OUT: c_uint = 0x29;
pub const AXP20X_VBUS_IPSOUT_MGMT: c_uint = 0x30;
pub const AXP20X_V_OFF: c_uint = 0x31;
pub const AXP20X_OFF_CTRL: c_uint = 0x32;
pub const AXP20X_CHRG_CTRL1: c_uint = 0x33;
pub const AXP20X_CHRG_CTRL2: c_uint = 0x34;
pub const AXP20X_CHRG_BAK_CTRL: c_uint = 0x35;
pub const AXP20X_PEK_KEY: c_uint = 0x36;
pub const AXP20X_DCDC_FREQ: c_uint = 0x37;
pub const AXP20X_V_LTF_CHRG: c_uint = 0x38;
pub const AXP20X_V_HTF_CHRG: c_uint = 0x39;
pub const AXP20X_APS_WARN_L1: c_uint = 0x3a;
pub const AXP20X_APS_WARN_L2: c_uint = 0x3b;
pub const AXP20X_V_LTF_DISCHRG: c_uint = 0x3c;
pub const AXP20X_V_HTF_DISCHRG: c_uint = 0x3d;
pub const AXP22X_PWR_OUT_CTRL1: c_uint = 0x10;
pub const AXP22X_PWR_OUT_CTRL2: c_uint = 0x12;
pub const AXP22X_PWR_OUT_CTRL3: c_uint = 0x13;
pub const AXP22X_DLDO1_V_OUT: c_uint = 0x15;
pub const AXP22X_DLDO2_V_OUT: c_uint = 0x16;
pub const AXP22X_DLDO3_V_OUT: c_uint = 0x17;
pub const AXP22X_DLDO4_V_OUT: c_uint = 0x18;
pub const AXP22X_ELDO1_V_OUT: c_uint = 0x19;
pub const AXP22X_ELDO2_V_OUT: c_uint = 0x1a;
pub const AXP22X_ELDO3_V_OUT: c_uint = 0x1b;
pub const AXP22X_DC5LDO_V_OUT: c_uint = 0x1c;
pub const AXP22X_DCDC1_V_OUT: c_uint = 0x21;
pub const AXP22X_DCDC2_V_OUT: c_uint = 0x22;
pub const AXP22X_DCDC3_V_OUT: c_uint = 0x23;
pub const AXP22X_DCDC4_V_OUT: c_uint = 0x24;
pub const AXP22X_DCDC5_V_OUT: c_uint = 0x25;
pub const AXP22X_DCDC23_V_RAMP_CTRL: c_uint = 0x27;
pub const AXP22X_ALDO1_V_OUT: c_uint = 0x28;
pub const AXP22X_ALDO2_V_OUT: c_uint = 0x29;
pub const AXP22X_ALDO3_V_OUT: c_uint = 0x2a;
pub const AXP22X_CHRG_CTRL3: c_uint = 0x35;
pub const AXP313A_ON_INDICATE: c_uint = 0x00;
pub const AXP313A_OUTPUT_CONTROL: c_uint = 0x10;
pub const AXP313A_DCDC1_CONTROL: c_uint = 0x13;
pub const AXP313A_DCDC2_CONTROL: c_uint = 0x14;
pub const AXP313A_DCDC3_CONTROL: c_uint = 0x15;
pub const AXP313A_ALDO1_CONTROL: c_uint = 0x16;
pub const AXP313A_DLDO1_CONTROL: c_uint = 0x17;
pub const AXP313A_SHUTDOWN_CTRL: c_uint = 0x1a;
pub const AXP313A_IRQ_EN: c_uint = 0x20;
pub const AXP313A_IRQ_STATE: c_uint = 0x21;
pub const AXP323_DCDC_MODE_CTRL2: c_uint = 0x22;
pub const AXP717_ON_INDICATE: c_uint = 0x00;
pub const AXP717_PMU_STATUS_2: c_uint = 0x01;
pub const AXP717_BC_DETECT: c_uint = 0x05;
pub const AXP717_PMU_FAULT: c_uint = 0x08;
pub const AXP717_MODULE_EN_CONTROL_1: c_uint = 0x0b;
pub const AXP717_MIN_SYS_V_CONTROL: c_uint = 0x15;
pub const AXP717_INPUT_VOL_LIMIT_CTRL: c_uint = 0x16;
pub const AXP717_INPUT_CUR_LIMIT_CTRL: c_uint = 0x17;
pub const AXP717_MODULE_EN_CONTROL_2: c_uint = 0x19;
pub const AXP717_BOOST_CONTROL: c_uint = 0x1e;
pub const AXP717_VSYS_V_POWEROFF: c_uint = 0x24;
pub const AXP717_IRQ0_EN: c_uint = 0x40;
pub const AXP717_IRQ1_EN: c_uint = 0x41;
pub const AXP717_IRQ2_EN: c_uint = 0x42;
pub const AXP717_IRQ3_EN: c_uint = 0x43;
pub const AXP717_IRQ4_EN: c_uint = 0x44;
pub const AXP717_IRQ0_STATE: c_uint = 0x48;
pub const AXP717_IRQ1_STATE: c_uint = 0x49;
pub const AXP717_IRQ2_STATE: c_uint = 0x4a;
pub const AXP717_IRQ3_STATE: c_uint = 0x4b;
pub const AXP717_IRQ4_STATE: c_uint = 0x4c;
pub const AXP717_TS_PIN_CFG: c_uint = 0x50;
pub const AXP717_ICC_CHG_SET: c_uint = 0x62;
pub const AXP717_ITERM_CHG_SET: c_uint = 0x63;
pub const AXP717_CV_CHG_SET: c_uint = 0x64;
pub const AXP717_DCDC_OUTPUT_CONTROL: c_uint = 0x80;
pub const AXP717_DCDC1_CONTROL: c_uint = 0x83;
pub const AXP717_DCDC2_CONTROL: c_uint = 0x84;
pub const AXP717_DCDC3_CONTROL: c_uint = 0x85;
pub const AXP717_DCDC4_CONTROL: c_uint = 0x86;
pub const AXP717_LDO0_OUTPUT_CONTROL: c_uint = 0x90;
pub const AXP717_LDO1_OUTPUT_CONTROL: c_uint = 0x91;
pub const AXP717_ALDO1_CONTROL: c_uint = 0x93;
pub const AXP717_ALDO2_CONTROL: c_uint = 0x94;
pub const AXP717_ALDO3_CONTROL: c_uint = 0x95;
pub const AXP717_ALDO4_CONTROL: c_uint = 0x96;
pub const AXP717_BLDO1_CONTROL: c_uint = 0x97;
pub const AXP717_BLDO2_CONTROL: c_uint = 0x98;
pub const AXP717_BLDO3_CONTROL: c_uint = 0x99;
pub const AXP717_BLDO4_CONTROL: c_uint = 0x9a;
pub const AXP717_CLDO1_CONTROL: c_uint = 0x9b;
pub const AXP717_CLDO2_CONTROL: c_uint = 0x9c;
pub const AXP717_CLDO3_CONTROL: c_uint = 0x9d;
pub const AXP717_CLDO4_CONTROL: c_uint = 0x9e;
pub const AXP717_CPUSLDO_CONTROL: c_uint = 0x9f;
pub const AXP717_BATT_PERCENT_DATA: c_uint = 0xa4;
pub const AXP717_ADC_CH_EN_CONTROL: c_uint = 0xc0;
pub const AXP717_BATT_V_H: c_uint = 0xc4;
pub const AXP717_BATT_V_L: c_uint = 0xc5;
pub const AXP717_VBUS_V_H: c_uint = 0xc6;
pub const AXP717_VBUS_V_L: c_uint = 0xc7;
pub const AXP717_VSYS_V_H: c_uint = 0xc8;
pub const AXP717_VSYS_V_L: c_uint = 0xc9;
pub const AXP717_BATT_CHRG_I_H: c_uint = 0xca;
pub const AXP717_BATT_CHRG_I_L: c_uint = 0xcb;
pub const AXP717_ADC_DATA_SEL: c_uint = 0xcd;
pub const AXP717_ADC_DATA_H: c_uint = 0xce;
pub const AXP717_ADC_DATA_L: c_uint = 0xcf;
pub const AXP717_TYPEC_CC_AA_EN: c_uint = 0xe1;
pub const AXP717_TYPEC_CC_MODE_CONTROL: c_uint = 0xe3;
pub const AXP717_TYPEC_CC_STATUS: c_uint = 0xe7;
pub const AXP806_STARTUP_SRC: c_uint = 0x00;
pub const AXP806_CHIP_ID: c_uint = 0x03;
pub const AXP806_PWR_OUT_CTRL1: c_uint = 0x10;
pub const AXP806_PWR_OUT_CTRL2: c_uint = 0x11;
pub const AXP806_DCDCA_V_CTRL: c_uint = 0x12;
pub const AXP806_DCDCB_V_CTRL: c_uint = 0x13;
pub const AXP806_DCDCC_V_CTRL: c_uint = 0x14;
pub const AXP806_DCDCD_V_CTRL: c_uint = 0x15;
pub const AXP806_DCDCE_V_CTRL: c_uint = 0x16;
pub const AXP806_ALDO1_V_CTRL: c_uint = 0x17;
pub const AXP806_ALDO2_V_CTRL: c_uint = 0x18;
pub const AXP806_ALDO3_V_CTRL: c_uint = 0x19;
pub const AXP806_DCDC_MODE_CTRL1: c_uint = 0x1a;
pub const AXP806_DCDC_MODE_CTRL2: c_uint = 0x1b;
pub const AXP806_DCDC_FREQ_CTRL: c_uint = 0x1c;
pub const AXP806_BLDO1_V_CTRL: c_uint = 0x20;
pub const AXP806_BLDO2_V_CTRL: c_uint = 0x21;
pub const AXP806_BLDO3_V_CTRL: c_uint = 0x22;
pub const AXP806_BLDO4_V_CTRL: c_uint = 0x23;
pub const AXP806_CLDO1_V_CTRL: c_uint = 0x24;
pub const AXP806_CLDO2_V_CTRL: c_uint = 0x25;
pub const AXP806_CLDO3_V_CTRL: c_uint = 0x26;
pub const AXP806_VREF_TEMP_WARN_L: c_uint = 0xf3;
pub const AXP806_BUS_ADDR_EXT: c_uint = 0xfe;
pub const AXP806_REG_ADDR_EXT: c_uint = 0xff;
pub const AXP803_POLYPHASE_CTRL: c_uint = 0x14;
pub const AXP803_FLDO1_V_OUT: c_uint = 0x1c;
pub const AXP803_FLDO2_V_OUT: c_uint = 0x1d;
pub const AXP803_DCDC1_V_OUT: c_uint = 0x20;
pub const AXP803_DCDC2_V_OUT: c_uint = 0x21;
pub const AXP803_DCDC3_V_OUT: c_uint = 0x22;
pub const AXP803_DCDC4_V_OUT: c_uint = 0x23;
pub const AXP803_DCDC5_V_OUT: c_uint = 0x24;
pub const AXP803_DCDC6_V_OUT: c_uint = 0x25;
pub const AXP803_DCDC_FREQ_CTRL: c_uint = 0x3b;
// Other DCDC regulator control registers are the same as AXP803
pub const AXP813_DCDC7_V_OUT: c_uint = 0x26;
pub const AXP15060_STARTUP_SRC: c_uint = 0x00;
pub const AXP15060_PWR_OUT_CTRL1: c_uint = 0x10;
pub const AXP15060_PWR_OUT_CTRL2: c_uint = 0x11;
pub const AXP15060_PWR_OUT_CTRL3: c_uint = 0x12;
pub const AXP15060_DCDC1_V_CTRL: c_uint = 0x13;
pub const AXP15060_DCDC2_V_CTRL: c_uint = 0x14;
pub const AXP15060_DCDC3_V_CTRL: c_uint = 0x15;
pub const AXP15060_DCDC4_V_CTRL: c_uint = 0x16;
pub const AXP15060_DCDC5_V_CTRL: c_uint = 0x17;
pub const AXP15060_DCDC6_V_CTRL: c_uint = 0x18;
pub const AXP15060_ALDO1_V_CTRL: c_uint = 0x19;
pub const AXP15060_DCDC_MODE_CTRL1: c_uint = 0x1a;
pub const AXP15060_DCDC_MODE_CTRL2: c_uint = 0x1b;
pub const AXP15060_OUTPUT_MONITOR_DISCHARGE: c_uint = 0x1e;
pub const AXP15060_IRQ_PWROK_VOFF: c_uint = 0x1f;
pub const AXP15060_ALDO2_V_CTRL: c_uint = 0x20;
pub const AXP15060_ALDO3_V_CTRL: c_uint = 0x21;
pub const AXP15060_ALDO4_V_CTRL: c_uint = 0x22;
pub const AXP15060_ALDO5_V_CTRL: c_uint = 0x23;
pub const AXP15060_BLDO1_V_CTRL: c_uint = 0x24;
pub const AXP15060_BLDO2_V_CTRL: c_uint = 0x25;
pub const AXP15060_BLDO3_V_CTRL: c_uint = 0x26;
pub const AXP15060_BLDO4_V_CTRL: c_uint = 0x27;
pub const AXP15060_BLDO5_V_CTRL: c_uint = 0x28;
pub const AXP15060_CLDO1_V_CTRL: c_uint = 0x29;
pub const AXP15060_CLDO2_V_CTRL: c_uint = 0x2a;
pub const AXP15060_CLDO3_V_CTRL: c_uint = 0x2b;
pub const AXP15060_CLDO4_V_CTRL: c_uint = 0x2d;
pub const AXP15060_CPUSLDO_V_CTRL: c_uint = 0x2e;
pub const AXP15060_PWR_WAKEUP_CTRL: c_uint = 0x31;
pub const AXP15060_PWR_DISABLE_DOWN_SEQ: c_uint = 0x32;
pub const AXP15060_PEK_KEY: c_uint = 0x36;
// Interrupt
pub const AXP152_IRQ1_EN: c_uint = 0x40;
pub const AXP152_IRQ2_EN: c_uint = 0x41;
pub const AXP152_IRQ3_EN: c_uint = 0x42;
pub const AXP152_IRQ1_STATE: c_uint = 0x48;
pub const AXP152_IRQ2_STATE: c_uint = 0x49;
pub const AXP152_IRQ3_STATE: c_uint = 0x4a;
pub const AXP192_IRQ1_EN: c_uint = 0x40;
pub const AXP192_IRQ2_EN: c_uint = 0x41;
pub const AXP192_IRQ3_EN: c_uint = 0x42;
pub const AXP192_IRQ4_EN: c_uint = 0x43;
pub const AXP192_IRQ1_STATE: c_uint = 0x44;
pub const AXP192_IRQ2_STATE: c_uint = 0x45;
pub const AXP192_IRQ3_STATE: c_uint = 0x46;
pub const AXP192_IRQ4_STATE: c_uint = 0x47;
pub const AXP192_IRQ5_EN: c_uint = 0x4a;
pub const AXP192_IRQ5_STATE: c_uint = 0x4d;
pub const AXP20X_IRQ1_EN: c_uint = 0x40;
pub const AXP20X_IRQ2_EN: c_uint = 0x41;
pub const AXP20X_IRQ3_EN: c_uint = 0x42;
pub const AXP20X_IRQ4_EN: c_uint = 0x43;
pub const AXP20X_IRQ5_EN: c_uint = 0x44;
pub const AXP20X_IRQ6_EN: c_uint = 0x45;
pub const AXP20X_IRQ1_STATE: c_uint = 0x48;
pub const AXP20X_IRQ2_STATE: c_uint = 0x49;
pub const AXP20X_IRQ3_STATE: c_uint = 0x4a;
pub const AXP20X_IRQ4_STATE: c_uint = 0x4b;
pub const AXP20X_IRQ5_STATE: c_uint = 0x4c;
pub const AXP20X_IRQ6_STATE: c_uint = 0x4d;
pub const AXP15060_IRQ1_EN: c_uint = 0x40;
pub const AXP15060_IRQ2_EN: c_uint = 0x41;
pub const AXP15060_IRQ1_STATE: c_uint = 0x48;
pub const AXP15060_IRQ2_STATE: c_uint = 0x49;
// ADC
pub const AXP192_GPIO2_V_ADC_H: c_uint = 0x68;
pub const AXP192_GPIO2_V_ADC_L: c_uint = 0x69;
pub const AXP192_GPIO3_V_ADC_H: c_uint = 0x6a;
pub const AXP192_GPIO3_V_ADC_L: c_uint = 0x6b;
pub const AXP20X_ACIN_V_ADC_H: c_uint = 0x56;
pub const AXP20X_ACIN_V_ADC_L: c_uint = 0x57;
pub const AXP20X_ACIN_I_ADC_H: c_uint = 0x58;
pub const AXP20X_ACIN_I_ADC_L: c_uint = 0x59;
pub const AXP20X_VBUS_V_ADC_H: c_uint = 0x5a;
pub const AXP20X_VBUS_V_ADC_L: c_uint = 0x5b;
pub const AXP20X_VBUS_I_ADC_H: c_uint = 0x5c;
pub const AXP20X_VBUS_I_ADC_L: c_uint = 0x5d;
pub const AXP20X_TEMP_ADC_H: c_uint = 0x5e;
pub const AXP20X_TEMP_ADC_L: c_uint = 0x5f;
pub const AXP20X_TS_IN_H: c_uint = 0x62;
pub const AXP20X_TS_IN_L: c_uint = 0x63;
pub const AXP20X_GPIO0_V_ADC_H: c_uint = 0x64;
pub const AXP20X_GPIO0_V_ADC_L: c_uint = 0x65;
pub const AXP20X_GPIO1_V_ADC_H: c_uint = 0x66;
pub const AXP20X_GPIO1_V_ADC_L: c_uint = 0x67;
pub const AXP20X_PWR_BATT_H: c_uint = 0x70;
pub const AXP20X_PWR_BATT_M: c_uint = 0x71;
pub const AXP20X_PWR_BATT_L: c_uint = 0x72;
pub const AXP20X_BATT_V_H: c_uint = 0x78;
pub const AXP20X_BATT_V_L: c_uint = 0x79;
pub const AXP20X_BATT_CHRG_I_H: c_uint = 0x7a;
pub const AXP20X_BATT_CHRG_I_L: c_uint = 0x7b;
pub const AXP20X_BATT_DISCHRG_I_H: c_uint = 0x7c;
pub const AXP20X_BATT_DISCHRG_I_L: c_uint = 0x7d;
pub const AXP20X_IPSOUT_V_HIGH_H: c_uint = 0x7e;
pub const AXP20X_IPSOUT_V_HIGH_L: c_uint = 0x7f;
// Power supply
pub const AXP192_GPIO30_IN_RANGE: c_uint = 0x85;
pub const AXP20X_DCDC_MODE: c_uint = 0x80;
pub const AXP20X_ADC_EN1: c_uint = 0x82;
pub const AXP20X_ADC_EN2: c_uint = 0x83;
pub const AXP20X_ADC_RATE: c_uint = 0x84;
pub const AXP20X_GPIO10_IN_RANGE: c_uint = 0x85;
pub const AXP20X_GPIO1_ADC_IRQ_RIS: c_uint = 0x86;
pub const AXP20X_GPIO1_ADC_IRQ_FAL: c_uint = 0x87;
pub const AXP20X_TIMER_CTRL: c_uint = 0x8a;
pub const AXP20X_VBUS_MON: c_uint = 0x8b;
pub const AXP20X_OVER_TMP: c_uint = 0x8f;
pub const AXP22X_PWREN_CTRL1: c_uint = 0x8c;
pub const AXP22X_PWREN_CTRL2: c_uint = 0x8d;
// GPIO
pub const AXP152_GPIO0_CTRL: c_uint = 0x90;
pub const AXP152_GPIO1_CTRL: c_uint = 0x91;
pub const AXP152_GPIO2_CTRL: c_uint = 0x92;
pub const AXP152_GPIO3_CTRL: c_uint = 0x93;
pub const AXP152_LDOGPIO2_V_OUT: c_uint = 0x96;
pub const AXP152_GPIO_INPUT: c_uint = 0x97;
pub const AXP152_PWM0_FREQ_X: c_uint = 0x98;
pub const AXP152_PWM0_FREQ_Y: c_uint = 0x99;
pub const AXP152_PWM0_DUTY_CYCLE: c_uint = 0x9a;
pub const AXP152_PWM1_FREQ_X: c_uint = 0x9b;
pub const AXP152_PWM1_FREQ_Y: c_uint = 0x9c;
pub const AXP152_PWM1_DUTY_CYCLE: c_uint = 0x9d;
pub const AXP192_GPIO0_CTRL: c_uint = 0x90;
pub const AXP192_LDO_IO0_V_OUT: c_uint = 0x91;
pub const AXP192_GPIO1_CTRL: c_uint = 0x92;
pub const AXP192_GPIO2_CTRL: c_uint = 0x93;
pub const AXP192_GPIO2_0_STATE: c_uint = 0x94;
pub const AXP192_GPIO4_3_CTRL: c_uint = 0x95;
pub const AXP192_GPIO4_3_STATE: c_uint = 0x96;
pub const AXP192_GPIO2_0_PULL: c_uint = 0x97;
pub const AXP192_N_RSTO_CTRL: c_uint = 0x9e;
pub const AXP20X_GPIO0_CTRL: c_uint = 0x90;
pub const AXP20X_LDO5_V_OUT: c_uint = 0x91;
pub const AXP20X_GPIO1_CTRL: c_uint = 0x92;
pub const AXP20X_GPIO2_CTRL: c_uint = 0x93;
pub const AXP20X_GPIO20_SS: c_uint = 0x94;
pub const AXP20X_GPIO3_CTRL: c_uint = 0x95;
pub const AXP22X_LDO_IO0_V_OUT: c_uint = 0x91;
pub const AXP22X_LDO_IO1_V_OUT: c_uint = 0x93;
pub const AXP22X_GPIO_STATE: c_uint = 0x94;
pub const AXP22X_GPIO_PULL_DOWN: c_uint = 0x95;
pub const AXP15060_CLDO4_GPIO2_MODESET: c_uint = 0x2c;
// Battery
pub const AXP20X_CHRG_CC_31_24: c_uint = 0xb0;
pub const AXP20X_CHRG_CC_23_16: c_uint = 0xb1;
pub const AXP20X_CHRG_CC_15_8: c_uint = 0xb2;
pub const AXP20X_CHRG_CC_7_0: c_uint = 0xb3;
pub const AXP20X_DISCHRG_CC_31_24: c_uint = 0xb4;
pub const AXP20X_DISCHRG_CC_23_16: c_uint = 0xb5;
pub const AXP20X_DISCHRG_CC_15_8: c_uint = 0xb6;
pub const AXP20X_DISCHRG_CC_7_0: c_uint = 0xb7;
pub const AXP20X_CC_CTRL: c_uint = 0xb8;
pub const AXP20X_FG_RES: c_uint = 0xb9;
// OCV
pub const AXP20X_RDC_H: c_uint = 0xba;
pub const AXP20X_RDC_L: c_uint = 0xbb;

pub const AXP20X_OCV_MAX: c_uint = 0xf;
// AXP22X specific registers
pub const AXP22X_PMIC_TEMP_H: c_uint = 0x56;
pub const AXP22X_PMIC_TEMP_L: c_uint = 0x57;
pub const AXP22X_TS_ADC_H: c_uint = 0x58;
pub const AXP22X_TS_ADC_L: c_uint = 0x59;
pub const AXP22X_BATLOW_THRES1: c_uint = 0xe6;
// AXP288/AXP803 specific registers
pub const AXP288_POWER_REASON: c_uint = 0x02;
pub const AXP288_BC_GLOBAL: c_uint = 0x2c;
pub const AXP288_BC_VBUS_CNTL: c_uint = 0x2d;
pub const AXP288_BC_USB_STAT: c_uint = 0x2e;
pub const AXP288_BC_DET_STAT: c_uint = 0x2f;
pub const AXP288_PMIC_ADC_H: c_uint = 0x56;
pub const AXP288_PMIC_ADC_L: c_uint = 0x57;
pub const AXP288_TS_ADC_H: c_uint = 0x58;
pub const AXP288_TS_ADC_L: c_uint = 0x59;
pub const AXP288_GP_ADC_H: c_uint = 0x5a;
pub const AXP288_GP_ADC_L: c_uint = 0x5b;
pub const AXP288_ADC_TS_PIN_CTRL: c_uint = 0x84;
pub const AXP288_RT_BATT_V_H: c_uint = 0xa0;
pub const AXP288_RT_BATT_V_L: c_uint = 0xa1;
pub const AXP813_ACIN_PATH_CTRL: c_uint = 0x3a;
pub const AXP813_ADC_RATE: c_uint = 0x85;
// Fuel Gauge
pub const AXP288_FG_RDC1_REG: c_uint = 0xba;
pub const AXP288_FG_RDC0_REG: c_uint = 0xbb;
pub const AXP288_FG_OCVH_REG: c_uint = 0xbc;
pub const AXP288_FG_OCVL_REG: c_uint = 0xbd;
pub const AXP288_FG_OCV_CURVE_REG: c_uint = 0xc0;
pub const AXP288_FG_DES_CAP1_REG: c_uint = 0xe0;
pub const AXP288_FG_DES_CAP0_REG: c_uint = 0xe1;
pub const AXP288_FG_CC_MTR1_REG: c_uint = 0xe2;
pub const AXP288_FG_CC_MTR0_REG: c_uint = 0xe3;
pub const AXP288_FG_OCV_CAP_REG: c_uint = 0xe4;
pub const AXP288_FG_CC_CAP_REG: c_uint = 0xe5;
pub const AXP288_FG_LOW_CAP_REG: c_uint = 0xe6;
pub const AXP288_FG_TUNE0: c_uint = 0xe8;
pub const AXP288_FG_TUNE1: c_uint = 0xe9;
pub const AXP288_FG_TUNE2: c_uint = 0xea;
pub const AXP288_FG_TUNE3: c_uint = 0xeb;
pub const AXP288_FG_TUNE4: c_uint = 0xec;
pub const AXP288_FG_TUNE5: c_uint = 0xed;
// Regulators IDs
// IRQs
// out of bit order to make sure the press event is handled first
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp192_irqs {
    AXP192_IRQ_ACIN_OVER_V = 1,
    AXP192_IRQ_ACIN_PLUGIN,
    AXP192_IRQ_ACIN_REMOVAL,
    AXP192_IRQ_VBUS_OVER_V,
    AXP192_IRQ_VBUS_PLUGIN,
    AXP192_IRQ_VBUS_REMOVAL,
    AXP192_IRQ_VBUS_V_LOW,
    AXP192_IRQ_BATT_PLUGIN,
    AXP192_IRQ_BATT_REMOVAL,
    AXP192_IRQ_BATT_ENT_ACT_MODE,
    AXP192_IRQ_BATT_EXIT_ACT_MODE,
    AXP192_IRQ_CHARG,
    AXP192_IRQ_CHARG_DONE,
    AXP192_IRQ_BATT_TEMP_HIGH,
    AXP192_IRQ_BATT_TEMP_LOW,
    AXP192_IRQ_DIE_TEMP_HIGH,
    AXP192_IRQ_CHARG_I_LOW,
    AXP192_IRQ_DCDC1_V_LONG,
    AXP192_IRQ_DCDC2_V_LONG,
    AXP192_IRQ_DCDC3_V_LONG,
    AXP192_IRQ_PEK_SHORT = 22,
    AXP192_IRQ_PEK_LONG,
    AXP192_IRQ_N_OE_PWR_ON,
    AXP192_IRQ_N_OE_PWR_OFF,
    AXP192_IRQ_VBUS_VALID,
    AXP192_IRQ_VBUS_NOT_VALID,
    AXP192_IRQ_VBUS_SESS_VALID,
    AXP192_IRQ_VBUS_SESS_END,
    AXP192_IRQ_LOW_PWR_LVL = 31,
    AXP192_IRQ_TIMER,
    AXP192_IRQ_GPIO2_INPUT = 37,
    AXP192_IRQ_GPIO1_INPUT,
    AXP192_IRQ_GPIO0_INPUT,
}

// out of bit order to make sure the press event is handled first
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp22x_irqs {
    AXP22X_IRQ_ACIN_OVER_V = 1,
    AXP22X_IRQ_ACIN_PLUGIN,
    AXP22X_IRQ_ACIN_REMOVAL,
    AXP22X_IRQ_VBUS_OVER_V,
    AXP22X_IRQ_VBUS_PLUGIN,
    AXP22X_IRQ_VBUS_REMOVAL,
    AXP22X_IRQ_VBUS_V_LOW,
    AXP22X_IRQ_BATT_PLUGIN,
    AXP22X_IRQ_BATT_REMOVAL,
    AXP22X_IRQ_BATT_ENT_ACT_MODE,
    AXP22X_IRQ_BATT_EXIT_ACT_MODE,
    AXP22X_IRQ_CHARG,
    AXP22X_IRQ_CHARG_DONE,
    AXP22X_IRQ_BATT_TEMP_HIGH,
    AXP22X_IRQ_BATT_TEMP_LOW,
    AXP22X_IRQ_DIE_TEMP_HIGH,
    AXP22X_IRQ_PEK_SHORT,
    AXP22X_IRQ_PEK_LONG,
    AXP22X_IRQ_LOW_PWR_LVL1,
    AXP22X_IRQ_LOW_PWR_LVL2,
    AXP22X_IRQ_TIMER,
// out of bit order to make sure the press event is handled first
    AXP22X_IRQ_PEK_FAL_EDGE,
    AXP22X_IRQ_PEK_RIS_EDGE,
    AXP22X_IRQ_GPIO1_INPUT,
    AXP22X_IRQ_GPIO0_INPUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp288_irqs {
    AXP288_IRQ_VBUS_FALL     = 2,
    AXP288_IRQ_VBUS_RISE,
    AXP288_IRQ_OV,
    AXP288_IRQ_FALLING_ALT,
    AXP288_IRQ_RISING_ALT,
    AXP288_IRQ_OV_ALT,
    AXP288_IRQ_DONE          = 10,
    AXP288_IRQ_CHARGING,
    AXP288_IRQ_SAFE_QUIT,
    AXP288_IRQ_SAFE_ENTER,
    AXP288_IRQ_ABSENT,
    AXP288_IRQ_APPEND,
    AXP288_IRQ_QWBTU,
    AXP288_IRQ_WBTU,
    AXP288_IRQ_QWBTO,
    AXP288_IRQ_WBTO,
    AXP288_IRQ_QCBTU,
    AXP288_IRQ_CBTU,
    AXP288_IRQ_QCBTO,
    AXP288_IRQ_CBTO,
    AXP288_IRQ_WL2,
    AXP288_IRQ_WL1,
    AXP288_IRQ_GPADC,
    AXP288_IRQ_OT            = 31,
    AXP288_IRQ_GPIO0,
    AXP288_IRQ_GPIO1,
    AXP288_IRQ_POKO,
    AXP288_IRQ_POKL,
    AXP288_IRQ_POKS,
    AXP288_IRQ_POKN,
    AXP288_IRQ_POKP,
    AXP288_IRQ_TIMER,
    AXP288_IRQ_MV_CHNG,
    AXP288_IRQ_BC_USB_CHNG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp313a_irqs {
    AXP313A_IRQ_DIE_TEMP_HIGH,
    AXP313A_IRQ_DCDC2_V_LOW = 2,
    AXP313A_IRQ_DCDC3_V_LOW,
    AXP313A_IRQ_PEK_LONG,
    AXP313A_IRQ_PEK_SHORT,
    AXP313A_IRQ_PEK_FAL_EDGE,
    AXP313A_IRQ_PEK_RIS_EDGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp717_irqs {
    AXP717_IRQ_VBUS_FAULT,
    AXP717_IRQ_VBUS_OVER_V,
    AXP717_IRQ_BOOST_OVER_V,
    AXP717_IRQ_GAUGE_NEW_SOC = 4,
    AXP717_IRQ_SOC_DROP_LVL1 = 6,
    AXP717_IRQ_SOC_DROP_LVL2,
    AXP717_IRQ_PEK_RIS_EDGE,
    AXP717_IRQ_PEK_FAL_EDGE,
    AXP717_IRQ_PEK_LONG,
    AXP717_IRQ_PEK_SHORT,
    AXP717_IRQ_BATT_REMOVAL,
    AXP717_IRQ_BATT_PLUGIN,
    AXP717_IRQ_VBUS_REMOVAL,
    AXP717_IRQ_VBUS_PLUGIN,
    AXP717_IRQ_BATT_OVER_V,
    AXP717_IRQ_CHARG_TIMER,
    AXP717_IRQ_DIE_TEMP_HIGH,
    AXP717_IRQ_CHARG,
    AXP717_IRQ_CHARG_DONE,
    AXP717_IRQ_BATT_OVER_CURR,
    AXP717_IRQ_LDO_OVER_CURR,
    AXP717_IRQ_WDOG_EXPIRE,
    AXP717_IRQ_BATT_ACT_TEMP_LOW,
    AXP717_IRQ_BATT_ACT_TEMP_HIGH,
    AXP717_IRQ_BATT_CHG_TEMP_LOW,
    AXP717_IRQ_BATT_CHG_TEMP_HIGH,
    AXP717_IRQ_BATT_QUIT_TEMP_HIGH,
    AXP717_IRQ_BC_USB_CHNG = 30,
    AXP717_IRQ_BC_USB_DONE,
    AXP717_IRQ_TYPEC_PLUGIN = 37,
    AXP717_IRQ_TYPEC_REMOVE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp803_irqs {
    AXP803_IRQ_ACIN_OVER_V = 1,
    AXP803_IRQ_ACIN_PLUGIN,
    AXP803_IRQ_ACIN_REMOVAL,
    AXP803_IRQ_VBUS_OVER_V,
    AXP803_IRQ_VBUS_PLUGIN,
    AXP803_IRQ_VBUS_REMOVAL,
    AXP803_IRQ_BATT_PLUGIN,
    AXP803_IRQ_BATT_REMOVAL,
    AXP803_IRQ_BATT_ENT_ACT_MODE,
    AXP803_IRQ_BATT_EXIT_ACT_MODE,
    AXP803_IRQ_CHARG,
    AXP803_IRQ_CHARG_DONE,
    AXP803_IRQ_BATT_CHG_TEMP_HIGH,
    AXP803_IRQ_BATT_CHG_TEMP_HIGH_END,
    AXP803_IRQ_BATT_CHG_TEMP_LOW,
    AXP803_IRQ_BATT_CHG_TEMP_LOW_END,
    AXP803_IRQ_BATT_ACT_TEMP_HIGH,
    AXP803_IRQ_BATT_ACT_TEMP_HIGH_END,
    AXP803_IRQ_BATT_ACT_TEMP_LOW,
    AXP803_IRQ_BATT_ACT_TEMP_LOW_END,
    AXP803_IRQ_DIE_TEMP_HIGH,
    AXP803_IRQ_GPADC,
    AXP803_IRQ_LOW_PWR_LVL1,
    AXP803_IRQ_LOW_PWR_LVL2,
    AXP803_IRQ_TIMER,
// out of bit order to make sure the press event is handled first
    AXP803_IRQ_PEK_FAL_EDGE,
    AXP803_IRQ_PEK_RIS_EDGE,
    AXP803_IRQ_PEK_SHORT,
    AXP803_IRQ_PEK_LONG,
    AXP803_IRQ_PEK_OVER_OFF,
    AXP803_IRQ_GPIO1_INPUT,
    AXP803_IRQ_GPIO0_INPUT,
    AXP803_IRQ_BC_USB_CHNG,
    AXP803_IRQ_MV_CHNG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp806_irqs {
    AXP806_IRQ_DIE_TEMP_HIGH_LV1,
    AXP806_IRQ_DIE_TEMP_HIGH_LV2,
    AXP806_IRQ_DCDCA_V_LOW,
    AXP806_IRQ_DCDCB_V_LOW,
    AXP806_IRQ_DCDCC_V_LOW,
    AXP806_IRQ_DCDCD_V_LOW,
    AXP806_IRQ_DCDCE_V_LOW,
    AXP806_IRQ_POK_LONG,
    AXP806_IRQ_POK_SHORT,
    AXP806_IRQ_WAKEUP,
    AXP806_IRQ_POK_FALL,
    AXP806_IRQ_POK_RISE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp809_irqs {
    AXP809_IRQ_ACIN_OVER_V = 1,
    AXP809_IRQ_ACIN_PLUGIN,
    AXP809_IRQ_ACIN_REMOVAL,
    AXP809_IRQ_VBUS_OVER_V,
    AXP809_IRQ_VBUS_PLUGIN,
    AXP809_IRQ_VBUS_REMOVAL,
    AXP809_IRQ_VBUS_V_LOW,
    AXP809_IRQ_BATT_PLUGIN,
    AXP809_IRQ_BATT_REMOVAL,
    AXP809_IRQ_BATT_ENT_ACT_MODE,
    AXP809_IRQ_BATT_EXIT_ACT_MODE,
    AXP809_IRQ_CHARG,
    AXP809_IRQ_CHARG_DONE,
    AXP809_IRQ_BATT_CHG_TEMP_HIGH,
    AXP809_IRQ_BATT_CHG_TEMP_HIGH_END,
    AXP809_IRQ_BATT_CHG_TEMP_LOW,
    AXP809_IRQ_BATT_CHG_TEMP_LOW_END,
    AXP809_IRQ_BATT_ACT_TEMP_HIGH,
    AXP809_IRQ_BATT_ACT_TEMP_HIGH_END,
    AXP809_IRQ_BATT_ACT_TEMP_LOW,
    AXP809_IRQ_BATT_ACT_TEMP_LOW_END,
    AXP809_IRQ_DIE_TEMP_HIGH,
    AXP809_IRQ_LOW_PWR_LVL1,
    AXP809_IRQ_LOW_PWR_LVL2,
    AXP809_IRQ_TIMER,
// out of bit order to make sure the press event is handled first
    AXP809_IRQ_PEK_FAL_EDGE,
    AXP809_IRQ_PEK_RIS_EDGE,
    AXP809_IRQ_PEK_SHORT,
    AXP809_IRQ_PEK_LONG,
    AXP809_IRQ_PEK_OVER_OFF,
    AXP809_IRQ_GPIO1_INPUT,
    AXP809_IRQ_GPIO0_INPUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum axp15060_irqs {
    AXP15060_IRQ_DIE_TEMP_HIGH_LV1 = 1,
    AXP15060_IRQ_DIE_TEMP_HIGH_LV2,
    AXP15060_IRQ_DCDC1_V_LOW,
    AXP15060_IRQ_DCDC2_V_LOW,
    AXP15060_IRQ_DCDC3_V_LOW,
    AXP15060_IRQ_DCDC4_V_LOW,
    AXP15060_IRQ_DCDC5_V_LOW,
    AXP15060_IRQ_DCDC6_V_LOW,
    AXP15060_IRQ_PEK_LONG,
    AXP15060_IRQ_PEK_SHORT,
    AXP15060_IRQ_GPIO1_INPUT,
    AXP15060_IRQ_PEK_FAL_EDGE,
    AXP15060_IRQ_PEK_RIS_EDGE,
    AXP15060_IRQ_GPIO2_INPUT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct axp20x_dev {
    pub dev: *mut device,
    pub irq: c_int,
    pub irq_flags: c_ulong,
    pub regmap: *mut regmap,
    pub regmap_irqc: *mut regmap_irq_chip_data,
    pub variant: axp20x_variants,
    pub nr_cells: c_int,
    pub cells: *const mfd_cell,
    pub regmap_cfg: *const regmap_config,
    pub regmap_irq_chip: *const regmap_irq_chip,
}

// generic helper function for reading 9-16 bit wide regs
//
// axp20x_match_device(): Setup axp20x variant related fields
//
// @axp20x: axp20x device to setup (.dev field must be set)
// @dev: device associated with this axp20x device
//
// This lets the axp20x core configure the mfd cells and register maps
// for later use.
//
extern "C" {
    pub fn axp20x_match_device(axp20x: *mut axp20x_dev) -> c_int;
}
//
// axp20x_device_probe(): Probe a configured axp20x device
//
// @axp20x: axp20x device to probe (must be configured)
//
// This function lets the axp20x core register the axp20x mfd devices
// and irqchip. The axp20x device passed in must be fully configured
// with axp20x_match_device, its irq set, and regmap created.
//
extern "C" {
    pub fn axp20x_device_probe(axp20x: *mut axp20x_dev) -> c_int;
}
//
// axp20x_device_remove(): Remove a axp20x device
//
// @axp20x: axp20x device to remove
//
// This tells the axp20x core to remove the associated mfd devices
//
extern "C" {
    pub fn axp20x_device_remove(axp20x: *mut axp20x_dev);
}
