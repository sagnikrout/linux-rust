//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/qcom,ipq5210-gcc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const GCC_ADSS_BCR: c_int = 0;
pub const GCC_ADSS_PWM_ARES: c_int = 1;
pub const GCC_APC0_VOLTAGE_DROOP_DETECTOR_BCR: c_int = 2;
pub const GCC_APC0_VOLTAGE_DROOP_DETECTOR_GPLL0_ARES: c_int = 3;
pub const GCC_APSS_AHB_ARES: c_int = 4;
pub const GCC_APSS_ATB_ARES: c_int = 5;
pub const GCC_APSS_AXI_ARES: c_int = 6;
pub const GCC_APSS_TS_ARES: c_int = 7;
pub const GCC_BOOT_ROM_AHB_ARES: c_int = 8;
pub const GCC_BOOT_ROM_BCR: c_int = 9;
pub const GCC_GEPHY_BCR: c_int = 10;
pub const GCC_GEPHY_SYS_ARES: c_int = 11;
pub const GCC_GP1_ARES: c_int = 12;
pub const GCC_GP2_ARES: c_int = 13;
pub const GCC_GP3_ARES: c_int = 14;
pub const GCC_MDIO_AHB_ARES: c_int = 15;
pub const GCC_MDIO_BCR: c_int = 16;
pub const GCC_MDIO_GEPHY_AHB_ARES: c_int = 17;
pub const GCC_NSS_BCR: c_int = 18;
pub const GCC_NSS_TS_ARES: c_int = 19;
pub const GCC_NSSCC_ARES: c_int = 20;
pub const GCC_NSSCFG_ARES: c_int = 21;
pub const GCC_NSSNOC_ATB_ARES: c_int = 22;
pub const GCC_NSSNOC_MEMNOC_1_ARES: c_int = 23;
pub const GCC_NSSNOC_MEMNOC_ARES: c_int = 24;
pub const GCC_NSSNOC_NSSCC_ARES: c_int = 25;
pub const GCC_NSSNOC_PCNOC_1_ARES: c_int = 26;
pub const GCC_NSSNOC_QOSGEN_REF_ARES: c_int = 27;
pub const GCC_NSSNOC_SNOC_1_ARES: c_int = 28;
pub const GCC_NSSNOC_SNOC_ARES: c_int = 29;
pub const GCC_NSSNOC_TIMEOUT_REF_ARES: c_int = 30;
pub const GCC_NSSNOC_XO_DCD_ARES: c_int = 31;
pub const GCC_PCIE0_AHB_ARES: c_int = 32;
pub const GCC_PCIE0_AUX_ARES: c_int = 33;
pub const GCC_PCIE0_AXI_M_ARES: c_int = 34;
pub const GCC_PCIE0_AXI_S_BRIDGE_ARES: c_int = 35;
pub const GCC_PCIE0_AXI_S_ARES: c_int = 36;
pub const GCC_PCIE0_BCR: c_int = 37;
pub const GCC_PCIE0_LINK_DOWN_BCR: c_int = 38;
pub const GCC_PCIE0_PHY_BCR: c_int = 39;
pub const GCC_PCIE0_PIPE_ARES: c_int = 40;
pub const GCC_PCIE0PHY_PHY_BCR: c_int = 41;
pub const GCC_PCIE1_AHB_ARES: c_int = 42;
pub const GCC_PCIE1_AUX_ARES: c_int = 43;
pub const GCC_PCIE1_AXI_M_ARES: c_int = 44;
pub const GCC_PCIE1_AXI_S_BRIDGE_ARES: c_int = 45;
pub const GCC_PCIE1_AXI_S_ARES: c_int = 46;
pub const GCC_PCIE1_BCR: c_int = 47;
pub const GCC_PCIE1_LINK_DOWN_BCR: c_int = 48;
pub const GCC_PCIE1_PHY_BCR: c_int = 49;
pub const GCC_PCIE1_PIPE_ARES: c_int = 50;
pub const GCC_PCIE1PHY_PHY_BCR: c_int = 51;
pub const GCC_QRNG_AHB_ARES: c_int = 52;
pub const GCC_QRNG_BCR: c_int = 53;
pub const GCC_QUPV3_2X_CORE_ARES: c_int = 54;
pub const GCC_QUPV3_AHB_MST_ARES: c_int = 55;
pub const GCC_QUPV3_AHB_SLV_ARES: c_int = 56;
pub const GCC_QUPV3_BCR: c_int = 57;
pub const GCC_QUPV3_CORE_ARES: c_int = 58;
pub const GCC_QUPV3_WRAP_SE0_ARES: c_int = 59;
pub const GCC_QUPV3_WRAP_SE0_BCR: c_int = 60;
pub const GCC_QUPV3_WRAP_SE1_ARES: c_int = 61;
pub const GCC_QUPV3_WRAP_SE1_BCR: c_int = 62;
pub const GCC_QUPV3_WRAP_SE2_ARES: c_int = 63;
pub const GCC_QUPV3_WRAP_SE2_BCR: c_int = 64;
pub const GCC_QUPV3_WRAP_SE3_ARES: c_int = 65;
pub const GCC_QUPV3_WRAP_SE3_BCR: c_int = 66;
pub const GCC_QUPV3_WRAP_SE4_ARES: c_int = 67;
pub const GCC_QUPV3_WRAP_SE4_BCR: c_int = 68;
pub const GCC_QUPV3_WRAP_SE5_ARES: c_int = 69;
pub const GCC_QUPV3_WRAP_SE5_BCR: c_int = 70;
pub const GCC_QUSB2_0_PHY_BCR: c_int = 71;
pub const GCC_SDCC1_AHB_ARES: c_int = 72;
pub const GCC_SDCC1_APPS_ARES: c_int = 73;
pub const GCC_SDCC1_ICE_CORE_ARES: c_int = 74;
pub const GCC_SDCC_BCR: c_int = 75;
pub const GCC_TLMM_AHB_ARES: c_int = 76;
pub const GCC_TLMM_ARES: c_int = 77;
pub const GCC_TLMM_BCR: c_int = 78;
pub const GCC_UNIPHY0_AHB_ARES: c_int = 79;
pub const GCC_UNIPHY0_BCR: c_int = 80;
pub const GCC_UNIPHY0_SYS_ARES: c_int = 81;
pub const GCC_UNIPHY1_AHB_ARES: c_int = 82;
pub const GCC_UNIPHY1_BCR: c_int = 83;
pub const GCC_UNIPHY1_SYS_ARES: c_int = 84;
pub const GCC_UNIPHY2_AHB_ARES: c_int = 85;
pub const GCC_UNIPHY2_BCR: c_int = 86;
pub const GCC_UNIPHY2_SYS_ARES: c_int = 87;
pub const GCC_USB0_AUX_ARES: c_int = 88;
pub const GCC_USB0_MASTER_ARES: c_int = 89;
pub const GCC_USB0_MOCK_UTMI_ARES: c_int = 90;
pub const GCC_USB0_PHY_BCR: c_int = 91;
pub const GCC_USB0_PHY_CFG_AHB_ARES: c_int = 92;
pub const GCC_USB0_PIPE_ARES: c_int = 93;
pub const GCC_USB0_SLEEP_ARES: c_int = 94;
pub const GCC_USB3PHY_0_PHY_BCR: c_int = 95;
pub const GCC_USB_BCR: c_int = 96;
pub const GCC_PCIE0_PIPE_RESET: c_int = 97;
pub const GCC_PCIE0_CORE_STICKY_RESET: c_int = 98;
pub const GCC_PCIE0_AXI_S_STICKY_RESET: c_int = 99;
pub const GCC_PCIE0_AXI_S_RESET: c_int = 100;
pub const GCC_PCIE0_AXI_M_STICKY_RESET: c_int = 101;
pub const GCC_PCIE0_AXI_M_RESET: c_int = 102;
pub const GCC_PCIE0_AUX_RESET: c_int = 103;
pub const GCC_PCIE0_AHB_RESET: c_int = 104;
pub const GCC_PCIE1_PIPE_RESET: c_int = 105;
pub const GCC_PCIE1_CORE_STICKY_RESET: c_int = 106;
pub const GCC_PCIE1_AXI_S_STICKY_RESET: c_int = 107;
pub const GCC_PCIE1_AXI_S_RESET: c_int = 108;
pub const GCC_PCIE1_AXI_M_STICKY_RESET: c_int = 109;
pub const GCC_PCIE1_AXI_M_RESET: c_int = 110;
pub const GCC_PCIE1_AUX_RESET: c_int = 111;
pub const GCC_PCIE1_AHB_RESET: c_int = 112;
pub const GCC_UNIPHY0_XPCS_ARES: c_int = 113;
pub const GCC_UNIPHY1_XPCS_ARES: c_int = 114;
pub const GCC_UNIPHY2_XPCS_ARES: c_int = 115;
pub const GCC_QDSS_BCR: c_int = 116;
