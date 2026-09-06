//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/amlogic/amlogic-t7-reset.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (c) 2024 Amlogic, Inc. All rights reserved.
//
// RESET0
// 0-3
pub const RESET_USB: c_int = 4;
pub const RESET_U2DRD: c_int = 5;
pub const RESET_U3DRD: c_int = 6;
pub const RESET_U3DRD_PIPE0: c_int = 7;
pub const RESET_U2PHY20: c_int = 8;
pub const RESET_U2PHY21: c_int = 9;
pub const RESET_GDC: c_int = 10;
pub const RESET_HDMI20_AES: c_int = 11;
pub const RESET_HDMIRX: c_int = 12;
pub const RESET_HDMIRX_APB: c_int = 13;
pub const RESET_DEWARP: c_int = 14;
// 15
pub const RESET_HDMITX_CAPB3: c_int = 16;
pub const RESET_BRG_VCBUG_DEC: c_int = 17;
pub const RESET_VCBUS: c_int = 18;
pub const RESET_VID_PLL_DIV: c_int = 19;
pub const RESET_VDI6: c_int = 20;
pub const RESET_GE2D: c_int = 21;
pub const RESET_HDMITXPHY: c_int = 22;
pub const RESET_VID_LOCK: c_int = 23;
pub const RESET_VENC0: c_int = 24;
pub const RESET_VDAC: c_int = 25;
pub const RESET_VENC2: c_int = 26;
pub const RESET_VENC1: c_int = 27;
pub const RESET_RDMA: c_int = 28;
pub const RESET_HDMITX: c_int = 29;
pub const RESET_VIU: c_int = 30;
pub const RESET_VENC: c_int = 31;
// RESET1
pub const RESET_AUDIO: c_int = 32;
pub const RESET_MALI_CAPB3: c_int = 33;
pub const RESET_MALI: c_int = 34;
pub const RESET_DDR_APB: c_int = 35;
pub const RESET_DDR: c_int = 36;
pub const RESET_DOS_CAPB3: c_int = 37;
pub const RESET_DOS: c_int = 38;
pub const RESET_COMBO_DPHY_CHAN2: c_int = 39;
pub const RESET_DEBUG_B: c_int = 40;
pub const RESET_DEBUG_A: c_int = 41;
pub const RESET_DSP_B: c_int = 42;
pub const RESET_DSP_A: c_int = 43;
pub const RESET_PCIE_A: c_int = 44;
pub const RESET_PCIE_PHY: c_int = 45;
pub const RESET_PCIE_APB: c_int = 46;
pub const RESET_ANAKIN: c_int = 47;
pub const RESET_ETH: c_int = 48;
pub const RESET_EDP0_CTRL: c_int = 49;
pub const RESET_EDP1_CTRL: c_int = 50;
pub const RESET_COMBO_DPHY_CHAN0: c_int = 51;
pub const RESET_COMBO_DPHY_CHAN1: c_int = 52;
pub const RESET_DSI_LVDS_EDP_TOP: c_int = 53;
pub const RESET_PCIE1_PHY: c_int = 54;
pub const RESET_PCIE1_APB: c_int = 55;
pub const RESET_DDR_1: c_int = 56;
// 57
pub const RESET_EDP1_PIPELINE: c_int = 58;
pub const RESET_EDP0_PIPELINE: c_int = 59;
pub const RESET_MIPI_DSI1_PHY: c_int = 60;
pub const RESET_MIPI_DSI0_PHY: c_int = 61;
pub const RESET_MIPI_DSI_A_HOST: c_int = 62;
pub const RESET_MIPI_DSI_B_HOST: c_int = 63;
// RESET2
pub const RESET_DEVICE_MMC_ARB: c_int = 64;
pub const RESET_IR_CTRL: c_int = 65;
pub const RESET_TS_A73: c_int = 66;
pub const RESET_TS_A53: c_int = 67;
pub const RESET_SPICC_2: c_int = 68;
pub const RESET_SPICC_3: c_int = 69;
pub const RESET_SPICC_4: c_int = 70;
pub const RESET_SPICC_5: c_int = 71;
pub const RESET_SMART_CARD: c_int = 72;
pub const RESET_SPICC_0: c_int = 73;
pub const RESET_SPICC_1: c_int = 74;
pub const RESET_RSA: c_int = 75;
// 76-79
pub const RESET_MSR_CLK: c_int = 80;
pub const RESET_SPIFC: c_int = 81;
pub const RESET_SAR_ADC: c_int = 82;
pub const RESET_BT: c_int = 83;
// 84-87
pub const RESET_ACODEC: c_int = 88;
pub const RESET_CEC: c_int = 89;
pub const RESET_AFIFO: c_int = 90;
pub const RESET_WATCHDOG: c_int = 91;
// 92-95
// RESET3
pub const RESET_BRG_NIC1_GPV: c_int = 96;
pub const RESET_BRG_NIC2_GPV: c_int = 97;
pub const RESET_BRG_NIC3_GPV: c_int = 98;
pub const RESET_BRG_NIC4_GPV: c_int = 99;
pub const RESET_BRG_NIC5_GPV: c_int = 100;
// 101-121
pub const RESET_MIPI_ISP: c_int = 122;
pub const RESET_BRG_ADB_MALI_1: c_int = 123;
pub const RESET_BRG_ADB_MALI_0: c_int = 124;
pub const RESET_BRG_ADB_A73: c_int = 125;
pub const RESET_BRG_ADB_A53: c_int = 126;
pub const RESET_BRG_CCI: c_int = 127;
// RESET4
pub const RESET_PWM_AO_AB: c_int = 128;
pub const RESET_PWM_AO_CD: c_int = 129;
pub const RESET_PWM_AO_EF: c_int = 130;
pub const RESET_PWM_AO_GH: c_int = 131;
pub const RESET_PWM_AB: c_int = 132;
pub const RESET_PWM_CD: c_int = 133;
pub const RESET_PWM_EF: c_int = 134;
// 135-137
pub const RESET_UART_A: c_int = 138;
pub const RESET_UART_B: c_int = 139;
pub const RESET_UART_C: c_int = 140;
pub const RESET_UART_D: c_int = 141;
pub const RESET_UART_E: c_int = 142;
pub const RESET_UART_F: c_int = 143;
pub const RESET_I2C_S_A: c_int = 144;
pub const RESET_I2C_M_A: c_int = 145;
pub const RESET_I2C_M_B: c_int = 146;
pub const RESET_I2C_M_C: c_int = 147;
pub const RESET_I2C_M_D: c_int = 148;
pub const RESET_I2C_M_E: c_int = 149;
pub const RESET_I2C_M_F: c_int = 150;
pub const RESET_I2C_M_AO_A: c_int = 151;
pub const RESET_SD_EMMC_A: c_int = 152;
pub const RESET_SD_EMMC_B: c_int = 153;
pub const RESET_SD_EMMC_C: c_int = 154;
pub const RESET_I2C_M_AO_B: c_int = 155;
pub const RESET_TS_GPU: c_int = 156;
pub const RESET_TS_NNA: c_int = 157;
pub const RESET_TS_VPN: c_int = 158;
pub const RESET_TS_HEVC: c_int = 159;
// RESET5
pub const RESET_BRG_NOC_DDR_1: c_int = 160;
pub const RESET_BRG_NOC_DDR_0: c_int = 161;
pub const RESET_BRG_NOC_MAIN: c_int = 162;
pub const RESET_BRG_NOC_ALL: c_int = 163;
// 164-167
pub const RESET_BRG_NIC2_SYS: c_int = 168;
pub const RESET_BRG_NIC2_MAIN: c_int = 169;
pub const RESET_BRG_NIC2_HDMI: c_int = 170;
pub const RESET_BRG_NIC2_ALL: c_int = 171;
pub const RESET_BRG_NIC3_WAVE: c_int = 172;
pub const RESET_BRG_NIC3_VDEC: c_int = 173;
pub const RESET_BRG_NIC3_HEVCF: c_int = 174;
pub const RESET_BRG_NIC3_HEVCB: c_int = 175;
pub const RESET_BRG_NIC3_HCODEC: c_int = 176;
pub const RESET_BRG_NIC3_GE2D: c_int = 177;
pub const RESET_BRG_NIC3_GDC: c_int = 178;
pub const RESET_BRG_NIC3_AMLOGIC: c_int = 179;
pub const RESET_BRG_NIC3_MAIN: c_int = 180;
pub const RESET_BRG_NIC3_ALL: c_int = 181;
pub const RESET_BRG_NIC5_VPU: c_int = 182;
// 183-185
pub const RESET_BRG_NIC4_DSPB: c_int = 186;
pub const RESET_BRG_NIC4_DSPA: c_int = 187;
pub const RESET_BRG_NIC4_VAPB: c_int = 188;
pub const RESET_BRG_NIC4_CLK81: c_int = 189;
pub const RESET_BRG_NIC4_MAIN: c_int = 190;
pub const RESET_BRG_NIC4_ALL: c_int = 191;
// RESET6
pub const RESET_BRG_VDEC_PIPEL: c_int = 192;
pub const RESET_BRG_HEVCF_DMC_PIPEL: c_int = 193;
pub const RESET_BRG_NIC2TONIC4_PIPEL: c_int = 194;
pub const RESET_BRG_HDMIRXTONIC2_PIPEL: c_int = 195;
pub const RESET_BRG_SECTONIC4_PIPEL: c_int = 196;
pub const RESET_BRG_VPUTONOC_PIPEL: c_int = 197;
pub const RESET_BRG_NIC4TONOC_PIPEL: c_int = 198;
pub const RESET_BRG_NIC3TONOC_PIPEL: c_int = 199;
pub const RESET_BRG_NIC2TONOC_PIPEL: c_int = 200;
pub const RESET_BRG_NNATONOC_PIPEL: c_int = 201;
pub const RESET_BRG_FRISP3_PIPEL: c_int = 202;
pub const RESET_BRG_FRISP2_PIPEL: c_int = 203;
pub const RESET_BRG_FRISP1_PIPEL: c_int = 204;
pub const RESET_BRG_FRISP0_PIPEL: c_int = 205;
// 206-217
pub const RESET_BRG_AMPIPE_NAND: c_int = 218;
pub const RESET_BRG_AMPIPE_ETH: c_int = 219;
// 220
pub const RESET_BRG_AM2AXI0: c_int = 221;
pub const RESET_BRG_AM2AXI1: c_int = 222;
pub const RESET_BRG_AM2AXI2: c_int = 223;
