//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sophgo,cv1800.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
//
// Copyright (C) 2023 Sophgo Ltd.
//
pub const CLK_MPLL: c_int = 0;
pub const CLK_TPLL: c_int = 1;
pub const CLK_FPLL: c_int = 2;
pub const CLK_MIPIMPLL: c_int = 3;
pub const CLK_A0PLL: c_int = 4;
pub const CLK_DISPPLL: c_int = 5;
pub const CLK_CAM0PLL: c_int = 6;
pub const CLK_CAM1PLL: c_int = 7;
pub const CLK_MIPIMPLL_D3: c_int = 8;
pub const CLK_CAM0PLL_D2: c_int = 9;
pub const CLK_CAM0PLL_D3: c_int = 10;
pub const CLK_TPU: c_int = 11;
pub const CLK_TPU_FAB: c_int = 12;
pub const CLK_AHB_ROM: c_int = 13;
pub const CLK_DDR_AXI_REG: c_int = 14;
pub const CLK_RTC_25M: c_int = 15;
pub const CLK_SRC_RTC_SYS_0: c_int = 16;
pub const CLK_TEMPSEN: c_int = 17;
pub const CLK_SARADC: c_int = 18;
pub const CLK_EFUSE: c_int = 19;
pub const CLK_APB_EFUSE: c_int = 20;
pub const CLK_DEBUG: c_int = 21;
pub const CLK_AP_DEBUG: c_int = 22;
pub const CLK_XTAL_MISC: c_int = 23;
pub const CLK_AXI4_EMMC: c_int = 24;
pub const CLK_EMMC: c_int = 25;
pub const CLK_EMMC_100K: c_int = 26;
pub const CLK_AXI4_SD0: c_int = 27;
pub const CLK_SD0: c_int = 28;
pub const CLK_SD0_100K: c_int = 29;
pub const CLK_AXI4_SD1: c_int = 30;
pub const CLK_SD1: c_int = 31;
pub const CLK_SD1_100K: c_int = 32;
pub const CLK_SPI_NAND: c_int = 33;
pub const CLK_ETH0_500M: c_int = 34;
pub const CLK_AXI4_ETH0: c_int = 35;
pub const CLK_ETH1_500M: c_int = 36;
pub const CLK_AXI4_ETH1: c_int = 37;
pub const CLK_APB_GPIO: c_int = 38;
pub const CLK_APB_GPIO_INTR: c_int = 39;
pub const CLK_GPIO_DB: c_int = 40;
pub const CLK_AHB_SF: c_int = 41;
pub const CLK_AHB_SF1: c_int = 42;
pub const CLK_A24M: c_int = 43;
pub const CLK_AUDSRC: c_int = 44;
pub const CLK_APB_AUDSRC: c_int = 45;
pub const CLK_SDMA_AXI: c_int = 46;
pub const CLK_SDMA_AUD0: c_int = 47;
pub const CLK_SDMA_AUD1: c_int = 48;
pub const CLK_SDMA_AUD2: c_int = 49;
pub const CLK_SDMA_AUD3: c_int = 50;
pub const CLK_I2C: c_int = 51;
pub const CLK_APB_I2C: c_int = 52;
pub const CLK_APB_I2C0: c_int = 53;
pub const CLK_APB_I2C1: c_int = 54;
pub const CLK_APB_I2C2: c_int = 55;
pub const CLK_APB_I2C3: c_int = 56;
pub const CLK_APB_I2C4: c_int = 57;
pub const CLK_APB_WDT: c_int = 58;
pub const CLK_PWM_SRC: c_int = 59;
pub const CLK_PWM: c_int = 60;
pub const CLK_SPI: c_int = 61;
pub const CLK_APB_SPI0: c_int = 62;
pub const CLK_APB_SPI1: c_int = 63;
pub const CLK_APB_SPI2: c_int = 64;
pub const CLK_APB_SPI3: c_int = 65;
pub const CLK_1M: c_int = 66;
pub const CLK_CAM0_200: c_int = 67;
pub const CLK_PM: c_int = 68;
pub const CLK_TIMER0: c_int = 69;
pub const CLK_TIMER1: c_int = 70;
pub const CLK_TIMER2: c_int = 71;
pub const CLK_TIMER3: c_int = 72;
pub const CLK_TIMER4: c_int = 73;
pub const CLK_TIMER5: c_int = 74;
pub const CLK_TIMER6: c_int = 75;
pub const CLK_TIMER7: c_int = 76;
pub const CLK_UART0: c_int = 77;
pub const CLK_APB_UART0: c_int = 78;
pub const CLK_UART1: c_int = 79;
pub const CLK_APB_UART1: c_int = 80;
pub const CLK_UART2: c_int = 81;
pub const CLK_APB_UART2: c_int = 82;
pub const CLK_UART3: c_int = 83;
pub const CLK_APB_UART3: c_int = 84;
pub const CLK_UART4: c_int = 85;
pub const CLK_APB_UART4: c_int = 86;
pub const CLK_APB_I2S0: c_int = 87;
pub const CLK_APB_I2S1: c_int = 88;
pub const CLK_APB_I2S2: c_int = 89;
pub const CLK_APB_I2S3: c_int = 90;
pub const CLK_AXI4_USB: c_int = 91;
pub const CLK_APB_USB: c_int = 92;
pub const CLK_USB_125M: c_int = 93;
pub const CLK_USB_33K: c_int = 94;
pub const CLK_USB_12M: c_int = 95;
pub const CLK_AXI4: c_int = 96;
pub const CLK_AXI6: c_int = 97;
pub const CLK_DSI_ESC: c_int = 98;
pub const CLK_AXI_VIP: c_int = 99;
pub const CLK_SRC_VIP_SYS_0: c_int = 100;
pub const CLK_SRC_VIP_SYS_1: c_int = 101;
pub const CLK_SRC_VIP_SYS_2: c_int = 102;
pub const CLK_SRC_VIP_SYS_3: c_int = 103;
pub const CLK_SRC_VIP_SYS_4: c_int = 104;
pub const CLK_CSI_BE_VIP: c_int = 105;
pub const CLK_CSI_MAC0_VIP: c_int = 106;
pub const CLK_CSI_MAC1_VIP: c_int = 107;
pub const CLK_CSI_MAC2_VIP: c_int = 108;
pub const CLK_CSI0_RX_VIP: c_int = 109;
pub const CLK_CSI1_RX_VIP: c_int = 110;
pub const CLK_ISP_TOP_VIP: c_int = 111;
pub const CLK_IMG_D_VIP: c_int = 112;
pub const CLK_IMG_V_VIP: c_int = 113;
pub const CLK_SC_TOP_VIP: c_int = 114;
pub const CLK_SC_D_VIP: c_int = 115;
pub const CLK_SC_V1_VIP: c_int = 116;
pub const CLK_SC_V2_VIP: c_int = 117;
pub const CLK_SC_V3_VIP: c_int = 118;
pub const CLK_DWA_VIP: c_int = 119;
pub const CLK_BT_VIP: c_int = 120;
pub const CLK_DISP_VIP: c_int = 121;
pub const CLK_DSI_MAC_VIP: c_int = 122;
pub const CLK_LVDS0_VIP: c_int = 123;
pub const CLK_LVDS1_VIP: c_int = 124;
pub const CLK_PAD_VI_VIP: c_int = 125;
pub const CLK_PAD_VI1_VIP: c_int = 126;
pub const CLK_PAD_VI2_VIP: c_int = 127;
pub const CLK_CFG_REG_VIP: c_int = 128;
pub const CLK_VIP_IP0: c_int = 129;
pub const CLK_VIP_IP1: c_int = 130;
pub const CLK_VIP_IP2: c_int = 131;
pub const CLK_VIP_IP3: c_int = 132;
pub const CLK_IVE_VIP: c_int = 133;
pub const CLK_RAW_VIP: c_int = 134;
pub const CLK_OSDC_VIP: c_int = 135;
pub const CLK_CAM0_VIP: c_int = 136;
pub const CLK_AXI_VIDEO_CODEC: c_int = 137;
pub const CLK_VC_SRC0: c_int = 138;
pub const CLK_VC_SRC1: c_int = 139;
pub const CLK_VC_SRC2: c_int = 140;
pub const CLK_H264C: c_int = 141;
pub const CLK_APB_H264C: c_int = 142;
pub const CLK_H265C: c_int = 143;
pub const CLK_APB_H265C: c_int = 144;
pub const CLK_JPEG: c_int = 145;
pub const CLK_APB_JPEG: c_int = 146;
pub const CLK_CAM0: c_int = 147;
pub const CLK_CAM1: c_int = 148;
pub const CLK_WGN: c_int = 149;
pub const CLK_WGN0: c_int = 150;
pub const CLK_WGN1: c_int = 151;
pub const CLK_WGN2: c_int = 152;
pub const CLK_KEYSCAN: c_int = 153;
pub const CLK_CFG_REG_VC: c_int = 154;
pub const CLK_C906_0: c_int = 155;
pub const CLK_C906_1: c_int = 156;
pub const CLK_A53: c_int = 157;
pub const CLK_CPU_AXI0: c_int = 158;
pub const CLK_CPU_GIC: c_int = 159;
pub const CLK_XTAL_AP: c_int = 160;
// Only for CV181x
pub const CLK_DISP_SRC_VIP: c_int = 161;
