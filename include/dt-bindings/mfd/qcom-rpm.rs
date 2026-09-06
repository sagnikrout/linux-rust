//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/qcom-rpm.h
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
// This header provides constants for the Qualcomm RPM bindings.
//
// Constants use to identify individual resources in the RPM.
//
pub const QCOM_RPM_APPS_FABRIC_ARB: c_int = 1;
pub const QCOM_RPM_APPS_FABRIC_CLK: c_int = 2;
pub const QCOM_RPM_APPS_FABRIC_HALT: c_int = 3;
pub const QCOM_RPM_APPS_FABRIC_IOCTL: c_int = 4;
pub const QCOM_RPM_APPS_FABRIC_MODE: c_int = 5;
pub const QCOM_RPM_APPS_L2_CACHE_CTL: c_int = 6;
pub const QCOM_RPM_CFPB_CLK: c_int = 7;
pub const QCOM_RPM_CXO_BUFFERS: c_int = 8;
pub const QCOM_RPM_CXO_CLK: c_int = 9;
pub const QCOM_RPM_DAYTONA_FABRIC_CLK: c_int = 10;
pub const QCOM_RPM_DDR_DMM: c_int = 11;
pub const QCOM_RPM_EBI1_CLK: c_int = 12;
pub const QCOM_RPM_HDMI_SWITCH: c_int = 13;
pub const QCOM_RPM_MMFPB_CLK: c_int = 14;
pub const QCOM_RPM_MM_FABRIC_ARB: c_int = 15;
pub const QCOM_RPM_MM_FABRIC_CLK: c_int = 16;
pub const QCOM_RPM_MM_FABRIC_HALT: c_int = 17;
pub const QCOM_RPM_MM_FABRIC_IOCTL: c_int = 18;
pub const QCOM_RPM_MM_FABRIC_MODE: c_int = 19;
pub const QCOM_RPM_PLL_4: c_int = 20;
pub const QCOM_RPM_PM8058_LDO0: c_int = 21;
pub const QCOM_RPM_PM8058_LDO1: c_int = 22;
pub const QCOM_RPM_PM8058_LDO2: c_int = 23;
pub const QCOM_RPM_PM8058_LDO3: c_int = 24;
pub const QCOM_RPM_PM8058_LDO4: c_int = 25;
pub const QCOM_RPM_PM8058_LDO5: c_int = 26;
pub const QCOM_RPM_PM8058_LDO6: c_int = 27;
pub const QCOM_RPM_PM8058_LDO7: c_int = 28;
pub const QCOM_RPM_PM8058_LDO8: c_int = 29;
pub const QCOM_RPM_PM8058_LDO9: c_int = 30;
pub const QCOM_RPM_PM8058_LDO10: c_int = 31;
pub const QCOM_RPM_PM8058_LDO11: c_int = 32;
pub const QCOM_RPM_PM8058_LDO12: c_int = 33;
pub const QCOM_RPM_PM8058_LDO13: c_int = 34;
pub const QCOM_RPM_PM8058_LDO14: c_int = 35;
pub const QCOM_RPM_PM8058_LDO15: c_int = 36;
pub const QCOM_RPM_PM8058_LDO16: c_int = 37;
pub const QCOM_RPM_PM8058_LDO17: c_int = 38;
pub const QCOM_RPM_PM8058_LDO18: c_int = 39;
pub const QCOM_RPM_PM8058_LDO19: c_int = 40;
pub const QCOM_RPM_PM8058_LDO20: c_int = 41;
pub const QCOM_RPM_PM8058_LDO21: c_int = 42;
pub const QCOM_RPM_PM8058_LDO22: c_int = 43;
pub const QCOM_RPM_PM8058_LDO23: c_int = 44;
pub const QCOM_RPM_PM8058_LDO24: c_int = 45;
pub const QCOM_RPM_PM8058_LDO25: c_int = 46;
pub const QCOM_RPM_PM8058_LVS0: c_int = 47;
pub const QCOM_RPM_PM8058_LVS1: c_int = 48;
pub const QCOM_RPM_PM8058_NCP: c_int = 49;
pub const QCOM_RPM_PM8058_SMPS0: c_int = 50;
pub const QCOM_RPM_PM8058_SMPS1: c_int = 51;
pub const QCOM_RPM_PM8058_SMPS2: c_int = 52;
pub const QCOM_RPM_PM8058_SMPS3: c_int = 53;
pub const QCOM_RPM_PM8058_SMPS4: c_int = 54;
pub const QCOM_RPM_PM8821_LDO1: c_int = 55;
pub const QCOM_RPM_PM8821_SMPS1: c_int = 56;
pub const QCOM_RPM_PM8821_SMPS2: c_int = 57;
pub const QCOM_RPM_PM8901_LDO0: c_int = 58;
pub const QCOM_RPM_PM8901_LDO1: c_int = 59;
pub const QCOM_RPM_PM8901_LDO2: c_int = 60;
pub const QCOM_RPM_PM8901_LDO3: c_int = 61;
pub const QCOM_RPM_PM8901_LDO4: c_int = 62;
pub const QCOM_RPM_PM8901_LDO5: c_int = 63;
pub const QCOM_RPM_PM8901_LDO6: c_int = 64;
pub const QCOM_RPM_PM8901_LVS0: c_int = 65;
pub const QCOM_RPM_PM8901_LVS1: c_int = 66;
pub const QCOM_RPM_PM8901_LVS2: c_int = 67;
pub const QCOM_RPM_PM8901_LVS3: c_int = 68;
pub const QCOM_RPM_PM8901_MVS: c_int = 69;
pub const QCOM_RPM_PM8901_SMPS0: c_int = 70;
pub const QCOM_RPM_PM8901_SMPS1: c_int = 71;
pub const QCOM_RPM_PM8901_SMPS2: c_int = 72;
pub const QCOM_RPM_PM8901_SMPS3: c_int = 73;
pub const QCOM_RPM_PM8901_SMPS4: c_int = 74;
pub const QCOM_RPM_PM8921_CLK1: c_int = 75;
pub const QCOM_RPM_PM8921_CLK2: c_int = 76;
pub const QCOM_RPM_PM8921_LDO1: c_int = 77;
pub const QCOM_RPM_PM8921_LDO2: c_int = 78;
pub const QCOM_RPM_PM8921_LDO3: c_int = 79;
pub const QCOM_RPM_PM8921_LDO4: c_int = 80;
pub const QCOM_RPM_PM8921_LDO5: c_int = 81;
pub const QCOM_RPM_PM8921_LDO6: c_int = 82;
pub const QCOM_RPM_PM8921_LDO7: c_int = 83;
pub const QCOM_RPM_PM8921_LDO8: c_int = 84;
pub const QCOM_RPM_PM8921_LDO9: c_int = 85;
pub const QCOM_RPM_PM8921_LDO10: c_int = 86;
pub const QCOM_RPM_PM8921_LDO11: c_int = 87;
pub const QCOM_RPM_PM8921_LDO12: c_int = 88;
pub const QCOM_RPM_PM8921_LDO13: c_int = 89;
pub const QCOM_RPM_PM8921_LDO14: c_int = 90;
pub const QCOM_RPM_PM8921_LDO15: c_int = 91;
pub const QCOM_RPM_PM8921_LDO16: c_int = 92;
pub const QCOM_RPM_PM8921_LDO17: c_int = 93;
pub const QCOM_RPM_PM8921_LDO18: c_int = 94;
pub const QCOM_RPM_PM8921_LDO19: c_int = 95;
pub const QCOM_RPM_PM8921_LDO20: c_int = 96;
pub const QCOM_RPM_PM8921_LDO21: c_int = 97;
pub const QCOM_RPM_PM8921_LDO22: c_int = 98;
pub const QCOM_RPM_PM8921_LDO23: c_int = 99;
pub const QCOM_RPM_PM8921_LDO24: c_int = 100;
pub const QCOM_RPM_PM8921_LDO25: c_int = 101;
pub const QCOM_RPM_PM8921_LDO26: c_int = 102;
pub const QCOM_RPM_PM8921_LDO27: c_int = 103;
pub const QCOM_RPM_PM8921_LDO28: c_int = 104;
pub const QCOM_RPM_PM8921_LDO29: c_int = 105;
pub const QCOM_RPM_PM8921_LVS1: c_int = 106;
pub const QCOM_RPM_PM8921_LVS2: c_int = 107;
pub const QCOM_RPM_PM8921_LVS3: c_int = 108;
pub const QCOM_RPM_PM8921_LVS4: c_int = 109;
pub const QCOM_RPM_PM8921_LVS5: c_int = 110;
pub const QCOM_RPM_PM8921_LVS6: c_int = 111;
pub const QCOM_RPM_PM8921_LVS7: c_int = 112;
pub const QCOM_RPM_PM8921_MVS: c_int = 113;
pub const QCOM_RPM_PM8921_NCP: c_int = 114;
pub const QCOM_RPM_PM8921_SMPS1: c_int = 115;
pub const QCOM_RPM_PM8921_SMPS2: c_int = 116;
pub const QCOM_RPM_PM8921_SMPS3: c_int = 117;
pub const QCOM_RPM_PM8921_SMPS4: c_int = 118;
pub const QCOM_RPM_PM8921_SMPS5: c_int = 119;
pub const QCOM_RPM_PM8921_SMPS6: c_int = 120;
pub const QCOM_RPM_PM8921_SMPS7: c_int = 121;
pub const QCOM_RPM_PM8921_SMPS8: c_int = 122;
pub const QCOM_RPM_PXO_CLK: c_int = 123;
pub const QCOM_RPM_QDSS_CLK: c_int = 124;
pub const QCOM_RPM_SFPB_CLK: c_int = 125;
pub const QCOM_RPM_SMI_CLK: c_int = 126;
pub const QCOM_RPM_SYS_FABRIC_ARB: c_int = 127;
pub const QCOM_RPM_SYS_FABRIC_CLK: c_int = 128;
pub const QCOM_RPM_SYS_FABRIC_HALT: c_int = 129;
pub const QCOM_RPM_SYS_FABRIC_IOCTL: c_int = 130;
pub const QCOM_RPM_SYS_FABRIC_MODE: c_int = 131;
pub const QCOM_RPM_USB_OTG_SWITCH: c_int = 132;
pub const QCOM_RPM_VDDMIN_GPIO: c_int = 133;
pub const QCOM_RPM_NSS_FABRIC_0_CLK: c_int = 134;
pub const QCOM_RPM_NSS_FABRIC_1_CLK: c_int = 135;
pub const QCOM_RPM_SMB208_S1a: c_int = 136;
pub const QCOM_RPM_SMB208_S1b: c_int = 137;
pub const QCOM_RPM_SMB208_S2a: c_int = 138;
pub const QCOM_RPM_SMB208_S2b: c_int = 139;
pub const QCOM_RPM_PM8018_SMPS1: c_int = 140;
pub const QCOM_RPM_PM8018_SMPS2: c_int = 141;
pub const QCOM_RPM_PM8018_SMPS3: c_int = 142;
pub const QCOM_RPM_PM8018_SMPS4: c_int = 143;
pub const QCOM_RPM_PM8018_SMPS5: c_int = 144;
pub const QCOM_RPM_PM8018_LDO1: c_int = 145;
pub const QCOM_RPM_PM8018_LDO2: c_int = 146;
pub const QCOM_RPM_PM8018_LDO3: c_int = 147;
pub const QCOM_RPM_PM8018_LDO4: c_int = 148;
pub const QCOM_RPM_PM8018_LDO5: c_int = 149;
pub const QCOM_RPM_PM8018_LDO6: c_int = 150;
pub const QCOM_RPM_PM8018_LDO7: c_int = 151;
pub const QCOM_RPM_PM8018_LDO8: c_int = 152;
pub const QCOM_RPM_PM8018_LDO9: c_int = 153;
pub const QCOM_RPM_PM8018_LDO10: c_int = 154;
pub const QCOM_RPM_PM8018_LDO11: c_int = 155;
pub const QCOM_RPM_PM8018_LDO12: c_int = 156;
pub const QCOM_RPM_PM8018_LDO13: c_int = 157;
pub const QCOM_RPM_PM8018_LDO14: c_int = 158;
pub const QCOM_RPM_PM8018_LVS1: c_int = 159;
pub const QCOM_RPM_PM8018_NCP: c_int = 160;
pub const QCOM_RPM_VOLTAGE_CORNER: c_int = 161;
//
// Constants used to select force mode for regulators.
//
pub const QCOM_RPM_FORCE_MODE_NONE: c_int = 0;
pub const QCOM_RPM_FORCE_MODE_LPM: c_int = 1;
pub const QCOM_RPM_FORCE_MODE_HPM: c_int = 2;
pub const QCOM_RPM_FORCE_MODE_AUTO: c_int = 3;
pub const QCOM_RPM_FORCE_MODE_BYPASS: c_int = 4;
