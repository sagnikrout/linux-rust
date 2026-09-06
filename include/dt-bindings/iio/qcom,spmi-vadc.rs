//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/iio/qcom,spmi-vadc.h
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
// Copyright (c) 2012-2014,2018,2020 The Linux Foundation. All rights reserved.
//
// Voltage ADC channels
pub const VADC_USBIN: c_uint = 0x00;
pub const VADC_DCIN: c_uint = 0x01;
pub const VADC_VCHG_SNS: c_uint = 0x02;
pub const VADC_SPARE1_03: c_uint = 0x03;
pub const VADC_USB_ID_MV: c_uint = 0x04;
pub const VADC_VCOIN: c_uint = 0x05;
pub const VADC_VBAT_SNS: c_uint = 0x06;
pub const VADC_VSYS: c_uint = 0x07;
pub const VADC_DIE_TEMP: c_uint = 0x08;
pub const VADC_REF_625MV: c_uint = 0x09;
pub const VADC_REF_1250MV: c_uint = 0x0a;
pub const VADC_CHG_TEMP: c_uint = 0x0b;
pub const VADC_SPARE1: c_uint = 0x0c;
pub const VADC_SPARE2: c_uint = 0x0d;
pub const VADC_GND_REF: c_uint = 0x0e;
pub const VADC_VDD_VADC: c_uint = 0x0f;
pub const VADC_P_MUX1_1_1: c_uint = 0x10;
pub const VADC_P_MUX2_1_1: c_uint = 0x11;
pub const VADC_P_MUX3_1_1: c_uint = 0x12;
pub const VADC_P_MUX4_1_1: c_uint = 0x13;
pub const VADC_P_MUX5_1_1: c_uint = 0x14;
pub const VADC_P_MUX6_1_1: c_uint = 0x15;
pub const VADC_P_MUX7_1_1: c_uint = 0x16;
pub const VADC_P_MUX8_1_1: c_uint = 0x17;
pub const VADC_P_MUX9_1_1: c_uint = 0x18;
pub const VADC_P_MUX10_1_1: c_uint = 0x19;
pub const VADC_P_MUX11_1_1: c_uint = 0x1a;
pub const VADC_P_MUX12_1_1: c_uint = 0x1b;
pub const VADC_P_MUX13_1_1: c_uint = 0x1c;
pub const VADC_P_MUX14_1_1: c_uint = 0x1d;
pub const VADC_P_MUX15_1_1: c_uint = 0x1e;
pub const VADC_P_MUX16_1_1: c_uint = 0x1f;
pub const VADC_P_MUX1_1_3: c_uint = 0x20;
pub const VADC_P_MUX2_1_3: c_uint = 0x21;
pub const VADC_P_MUX3_1_3: c_uint = 0x22;
pub const VADC_P_MUX4_1_3: c_uint = 0x23;
pub const VADC_P_MUX5_1_3: c_uint = 0x24;
pub const VADC_P_MUX6_1_3: c_uint = 0x25;
pub const VADC_P_MUX7_1_3: c_uint = 0x26;
pub const VADC_P_MUX8_1_3: c_uint = 0x27;
pub const VADC_P_MUX9_1_3: c_uint = 0x28;
pub const VADC_P_MUX10_1_3: c_uint = 0x29;
pub const VADC_P_MUX11_1_3: c_uint = 0x2a;
pub const VADC_P_MUX12_1_3: c_uint = 0x2b;
pub const VADC_P_MUX13_1_3: c_uint = 0x2c;
pub const VADC_P_MUX14_1_3: c_uint = 0x2d;
pub const VADC_P_MUX15_1_3: c_uint = 0x2e;
pub const VADC_P_MUX16_1_3: c_uint = 0x2f;
pub const VADC_LR_MUX1_BAT_THERM: c_uint = 0x30;
pub const VADC_LR_MUX2_BAT_ID: c_uint = 0x31;
pub const VADC_LR_MUX3_XO_THERM: c_uint = 0x32;
pub const VADC_LR_MUX4_AMUX_THM1: c_uint = 0x33;
pub const VADC_LR_MUX5_AMUX_THM2: c_uint = 0x34;
pub const VADC_LR_MUX6_AMUX_THM3: c_uint = 0x35;
pub const VADC_LR_MUX7_HW_ID: c_uint = 0x36;
pub const VADC_LR_MUX8_AMUX_THM4: c_uint = 0x37;
pub const VADC_LR_MUX9_AMUX_THM5: c_uint = 0x38;
pub const VADC_LR_MUX10_USB_ID: c_uint = 0x39;
pub const VADC_AMUX_PU1: c_uint = 0x3a;
pub const VADC_AMUX_PU2: c_uint = 0x3b;
pub const VADC_LR_MUX3_BUF_XO_THERM: c_uint = 0x3c;
pub const VADC_LR_MUX1_PU1_BAT_THERM: c_uint = 0x70;
pub const VADC_LR_MUX2_PU1_BAT_ID: c_uint = 0x71;
pub const VADC_LR_MUX3_PU1_XO_THERM: c_uint = 0x72;
pub const VADC_LR_MUX4_PU1_AMUX_THM1: c_uint = 0x73;
pub const VADC_LR_MUX5_PU1_AMUX_THM2: c_uint = 0x74;
pub const VADC_LR_MUX6_PU1_AMUX_THM3: c_uint = 0x75;
pub const VADC_LR_MUX7_PU1_AMUX_HW_ID: c_uint = 0x76;
pub const VADC_LR_MUX8_PU1_AMUX_THM4: c_uint = 0x77;
pub const VADC_LR_MUX9_PU1_AMUX_THM5: c_uint = 0x78;
pub const VADC_LR_MUX10_PU1_AMUX_USB_ID: c_uint = 0x79;
pub const VADC_LR_MUX3_BUF_PU1_XO_THERM: c_uint = 0x7c;
pub const VADC_LR_MUX1_PU2_BAT_THERM: c_uint = 0xb0;
pub const VADC_LR_MUX2_PU2_BAT_ID: c_uint = 0xb1;
pub const VADC_LR_MUX3_PU2_XO_THERM: c_uint = 0xb2;
pub const VADC_LR_MUX4_PU2_AMUX_THM1: c_uint = 0xb3;
pub const VADC_LR_MUX5_PU2_AMUX_THM2: c_uint = 0xb4;
pub const VADC_LR_MUX6_PU2_AMUX_THM3: c_uint = 0xb5;
pub const VADC_LR_MUX7_PU2_AMUX_HW_ID: c_uint = 0xb6;
pub const VADC_LR_MUX8_PU2_AMUX_THM4: c_uint = 0xb7;
pub const VADC_LR_MUX9_PU2_AMUX_THM5: c_uint = 0xb8;
pub const VADC_LR_MUX10_PU2_AMUX_USB_ID: c_uint = 0xb9;
pub const VADC_LR_MUX3_BUF_PU2_XO_THERM: c_uint = 0xbc;
pub const VADC_LR_MUX1_PU1_PU2_BAT_THERM: c_uint = 0xf0;
pub const VADC_LR_MUX2_PU1_PU2_BAT_ID: c_uint = 0xf1;
pub const VADC_LR_MUX3_PU1_PU2_XO_THERM: c_uint = 0xf2;
pub const VADC_LR_MUX4_PU1_PU2_AMUX_THM1: c_uint = 0xf3;
pub const VADC_LR_MUX5_PU1_PU2_AMUX_THM2: c_uint = 0xf4;
pub const VADC_LR_MUX6_PU1_PU2_AMUX_THM3: c_uint = 0xf5;
pub const VADC_LR_MUX7_PU1_PU2_AMUX_HW_ID: c_uint = 0xf6;
pub const VADC_LR_MUX8_PU1_PU2_AMUX_THM4: c_uint = 0xf7;
pub const VADC_LR_MUX9_PU1_PU2_AMUX_THM5: c_uint = 0xf8;
pub const VADC_LR_MUX10_PU1_PU2_AMUX_USB_ID: c_uint = 0xf9;
pub const VADC_LR_MUX3_BUF_PU1_PU2_XO_THERM: c_uint = 0xfc;
// ADC channels for SPMI PMIC5
pub const ADC5_REF_GND: c_uint = 0x00;
pub const ADC5_1P25VREF: c_uint = 0x01;
pub const ADC5_VREF_VADC: c_uint = 0x02;
pub const ADC5_VREF_VADC5_DIV_3: c_uint = 0x82;
pub const ADC5_VPH_PWR: c_uint = 0x83;
pub const ADC5_VBAT_SNS: c_uint = 0x84;
pub const ADC5_VCOIN: c_uint = 0x85;
pub const ADC5_DIE_TEMP: c_uint = 0x06;
pub const ADC5_USB_IN_I: c_uint = 0x07;
pub const ADC5_USB_IN_V_16: c_uint = 0x08;
pub const ADC5_CHG_TEMP: c_uint = 0x09;
pub const ADC5_BAT_THERM: c_uint = 0x0a;
pub const ADC5_BAT_ID: c_uint = 0x0b;
pub const ADC5_XO_THERM: c_uint = 0x0c;
pub const ADC5_AMUX_THM1: c_uint = 0x0d;
pub const ADC5_AMUX_THM2: c_uint = 0x0e;
pub const ADC5_AMUX_THM3: c_uint = 0x0f;
pub const ADC5_AMUX_THM4: c_uint = 0x10;
pub const ADC5_AMUX_THM5: c_uint = 0x11;
pub const ADC5_GPIO1: c_uint = 0x12;
pub const ADC5_GPIO2: c_uint = 0x13;
pub const ADC5_GPIO3: c_uint = 0x14;
pub const ADC5_GPIO4: c_uint = 0x15;
pub const ADC5_GPIO5: c_uint = 0x16;
pub const ADC5_GPIO6: c_uint = 0x17;
pub const ADC5_GPIO7: c_uint = 0x18;
pub const ADC5_SBUx: c_uint = 0x99;
pub const ADC5_MID_CHG_DIV6: c_uint = 0x1e;
pub const ADC5_OFF: c_uint = 0xff;
// 30k pull-up1
pub const ADC5_BAT_THERM_30K_PU: c_uint = 0x2a;
pub const ADC5_BAT_ID_30K_PU: c_uint = 0x2b;
pub const ADC5_XO_THERM_30K_PU: c_uint = 0x2c;
pub const ADC5_AMUX_THM1_30K_PU: c_uint = 0x2d;
pub const ADC5_AMUX_THM2_30K_PU: c_uint = 0x2e;
pub const ADC5_AMUX_THM3_30K_PU: c_uint = 0x2f;
pub const ADC5_AMUX_THM4_30K_PU: c_uint = 0x30;
pub const ADC5_AMUX_THM5_30K_PU: c_uint = 0x31;
pub const ADC5_GPIO1_30K_PU: c_uint = 0x32;
pub const ADC5_GPIO2_30K_PU: c_uint = 0x33;
pub const ADC5_GPIO3_30K_PU: c_uint = 0x34;
pub const ADC5_GPIO4_30K_PU: c_uint = 0x35;
pub const ADC5_GPIO5_30K_PU: c_uint = 0x36;
pub const ADC5_GPIO6_30K_PU: c_uint = 0x37;
pub const ADC5_GPIO7_30K_PU: c_uint = 0x38;
pub const ADC5_SBUx_30K_PU: c_uint = 0x39;
// 100k pull-up2
pub const ADC5_BAT_THERM_100K_PU: c_uint = 0x4a;
pub const ADC5_BAT_ID_100K_PU: c_uint = 0x4b;
pub const ADC5_XO_THERM_100K_PU: c_uint = 0x4c;
pub const ADC5_AMUX_THM1_100K_PU: c_uint = 0x4d;
pub const ADC5_AMUX_THM2_100K_PU: c_uint = 0x4e;
pub const ADC5_AMUX_THM3_100K_PU: c_uint = 0x4f;
pub const ADC5_AMUX_THM4_100K_PU: c_uint = 0x50;
pub const ADC5_AMUX_THM5_100K_PU: c_uint = 0x51;
pub const ADC5_GPIO1_100K_PU: c_uint = 0x52;
pub const ADC5_GPIO2_100K_PU: c_uint = 0x53;
pub const ADC5_GPIO3_100K_PU: c_uint = 0x54;
pub const ADC5_GPIO4_100K_PU: c_uint = 0x55;
pub const ADC5_GPIO5_100K_PU: c_uint = 0x56;
pub const ADC5_GPIO6_100K_PU: c_uint = 0x57;
pub const ADC5_GPIO7_100K_PU: c_uint = 0x58;
pub const ADC5_SBUx_100K_PU: c_uint = 0x59;
// 400k pull-up3
pub const ADC5_BAT_THERM_400K_PU: c_uint = 0x6a;
pub const ADC5_BAT_ID_400K_PU: c_uint = 0x6b;
pub const ADC5_XO_THERM_400K_PU: c_uint = 0x6c;
pub const ADC5_AMUX_THM1_400K_PU: c_uint = 0x6d;
pub const ADC5_AMUX_THM2_400K_PU: c_uint = 0x6e;
pub const ADC5_AMUX_THM3_400K_PU: c_uint = 0x6f;
pub const ADC5_AMUX_THM4_400K_PU: c_uint = 0x70;
pub const ADC5_AMUX_THM5_400K_PU: c_uint = 0x71;
pub const ADC5_GPIO1_400K_PU: c_uint = 0x72;
pub const ADC5_GPIO2_400K_PU: c_uint = 0x73;
pub const ADC5_GPIO3_400K_PU: c_uint = 0x74;
pub const ADC5_GPIO4_400K_PU: c_uint = 0x75;
pub const ADC5_GPIO5_400K_PU: c_uint = 0x76;
pub const ADC5_GPIO6_400K_PU: c_uint = 0x77;
pub const ADC5_GPIO7_400K_PU: c_uint = 0x78;
pub const ADC5_SBUx_400K_PU: c_uint = 0x79;
// 1/3 Divider
pub const ADC5_GPIO1_DIV3: c_uint = 0x92;
pub const ADC5_GPIO2_DIV3: c_uint = 0x93;
pub const ADC5_GPIO3_DIV3: c_uint = 0x94;
pub const ADC5_GPIO4_DIV3: c_uint = 0x95;
pub const ADC5_GPIO5_DIV3: c_uint = 0x96;
pub const ADC5_GPIO6_DIV3: c_uint = 0x97;
pub const ADC5_GPIO7_DIV3: c_uint = 0x98;
pub const ADC5_SBUx_DIV3: c_uint = 0x99;
// Current and combined current/voltage channels
pub const ADC5_INT_EXT_ISENSE: c_uint = 0xa1;
pub const ADC5_PARALLEL_ISENSE: c_uint = 0xa5;
pub const ADC5_CUR_REPLICA_VDS: c_uint = 0xa7;
pub const ADC5_CUR_SENS_BATFET_VDS_OFFSET: c_uint = 0xa9;
pub const ADC5_CUR_SENS_REPLICA_VDS_OFFSET: c_uint = 0xab;
pub const ADC5_EXT_SENS_OFFSET: c_uint = 0xad;
pub const ADC5_INT_EXT_ISENSE_VBAT_VDATA: c_uint = 0xb0;
pub const ADC5_INT_EXT_ISENSE_VBAT_IDATA: c_uint = 0xb1;
pub const ADC5_EXT_ISENSE_VBAT_VDATA: c_uint = 0xb2;
pub const ADC5_EXT_ISENSE_VBAT_IDATA: c_uint = 0xb3;
pub const ADC5_PARALLEL_ISENSE_VBAT_VDATA: c_uint = 0xb4;
pub const ADC5_PARALLEL_ISENSE_VBAT_IDATA: c_uint = 0xb5;
pub const ADC5_MAX_CHANNEL: c_uint = 0xc0;
// ADC channels for ADC for PMIC7
pub const ADC7_REF_GND: c_uint = 0x00;
pub const ADC7_1P25VREF: c_uint = 0x01;
pub const ADC7_VREF_VADC: c_uint = 0x02;
pub const ADC7_DIE_TEMP: c_uint = 0x03;
pub const ADC7_AMUX_THM1: c_uint = 0x04;
pub const ADC7_AMUX_THM2: c_uint = 0x05;
pub const ADC7_AMUX_THM3: c_uint = 0x06;
pub const ADC7_AMUX_THM4: c_uint = 0x07;
pub const ADC7_AMUX_THM5: c_uint = 0x08;
pub const ADC7_AMUX_THM6: c_uint = 0x09;
pub const ADC7_GPIO1: c_uint = 0x0a;
pub const ADC7_GPIO2: c_uint = 0x0b;
pub const ADC7_GPIO3: c_uint = 0x0c;
pub const ADC7_GPIO4: c_uint = 0x0d;
pub const ADC7_SMB_TEMP: c_uint = 0x06;
pub const ADC7_CHG_TEMP: c_uint = 0x10;
pub const ADC7_USB_IN_V_16: c_uint = 0x11;
pub const ADC7_VDC_16: c_uint = 0x12;
pub const ADC7_CC1_ID: c_uint = 0x13;
pub const ADC7_VREF_BAT_THERM: c_uint = 0x15;
pub const ADC7_IIN_FB: c_uint = 0x17;
pub const ADC7_ICHG_SMB: c_uint = 0x18;
pub const ADC7_IIN_SMB: c_uint = 0x19;
// 30k pull-up1
pub const ADC7_AMUX_THM1_30K_PU: c_uint = 0x24;
pub const ADC7_AMUX_THM2_30K_PU: c_uint = 0x25;
pub const ADC7_AMUX_THM3_30K_PU: c_uint = 0x26;
pub const ADC7_AMUX_THM4_30K_PU: c_uint = 0x27;
pub const ADC7_AMUX_THM5_30K_PU: c_uint = 0x28;
pub const ADC7_AMUX_THM6_30K_PU: c_uint = 0x29;
pub const ADC7_GPIO1_30K_PU: c_uint = 0x2a;
pub const ADC7_GPIO2_30K_PU: c_uint = 0x2b;
pub const ADC7_GPIO3_30K_PU: c_uint = 0x2c;
pub const ADC7_GPIO4_30K_PU: c_uint = 0x2d;
pub const ADC7_CC1_ID_30K_PU: c_uint = 0x33;
// 100k pull-up2
pub const ADC7_AMUX_THM1_100K_PU: c_uint = 0x44;
pub const ADC7_AMUX_THM2_100K_PU: c_uint = 0x45;
pub const ADC7_AMUX_THM3_100K_PU: c_uint = 0x46;
pub const ADC7_AMUX_THM4_100K_PU: c_uint = 0x47;
pub const ADC7_AMUX_THM5_100K_PU: c_uint = 0x48;
pub const ADC7_AMUX_THM6_100K_PU: c_uint = 0x49;
pub const ADC7_GPIO1_100K_PU: c_uint = 0x4a;
pub const ADC7_GPIO2_100K_PU: c_uint = 0x4b;
pub const ADC7_GPIO3_100K_PU: c_uint = 0x4c;
pub const ADC7_GPIO4_100K_PU: c_uint = 0x4d;
pub const ADC7_CC1_ID_100K_PU: c_uint = 0x53;
// 400k pull-up3
pub const ADC7_AMUX_THM1_400K_PU: c_uint = 0x64;
pub const ADC7_AMUX_THM2_400K_PU: c_uint = 0x65;
pub const ADC7_AMUX_THM3_400K_PU: c_uint = 0x66;
pub const ADC7_AMUX_THM4_400K_PU: c_uint = 0x67;
pub const ADC7_AMUX_THM5_400K_PU: c_uint = 0x68;
pub const ADC7_AMUX_THM6_400K_PU: c_uint = 0x69;
pub const ADC7_GPIO1_400K_PU: c_uint = 0x6a;
pub const ADC7_GPIO2_400K_PU: c_uint = 0x6b;
pub const ADC7_GPIO3_400K_PU: c_uint = 0x6c;
pub const ADC7_GPIO4_400K_PU: c_uint = 0x6d;
pub const ADC7_CC1_ID_400K_PU: c_uint = 0x73;
// 1/3 Divider
pub const ADC7_GPIO1_DIV3: c_uint = 0x8a;
pub const ADC7_GPIO2_DIV3: c_uint = 0x8b;
pub const ADC7_GPIO3_DIV3: c_uint = 0x8c;
pub const ADC7_GPIO4_DIV3: c_uint = 0x8d;
pub const ADC7_VPH_PWR: c_uint = 0x8e;
pub const ADC7_VBAT_SNS: c_uint = 0x8f;
pub const ADC7_SBUx: c_uint = 0x94;
pub const ADC7_VBAT_2S_MID: c_uint = 0x96;
