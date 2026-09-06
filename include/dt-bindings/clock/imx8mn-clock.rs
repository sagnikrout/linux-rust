//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/imx8mn-clock.h
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
// Copyright 2018-2019 NXP
//
pub const IMX8MN_CLK_DUMMY: c_int = 0;
pub const IMX8MN_CLK_32K: c_int = 1;
pub const IMX8MN_CLK_24M: c_int = 2;
pub const IMX8MN_OSC_HDMI_CLK: c_int = 3;
pub const IMX8MN_CLK_EXT1: c_int = 4;
pub const IMX8MN_CLK_EXT2: c_int = 5;
pub const IMX8MN_CLK_EXT3: c_int = 6;
pub const IMX8MN_CLK_EXT4: c_int = 7;
pub const IMX8MN_AUDIO_PLL1_REF_SEL: c_int = 8;
pub const IMX8MN_AUDIO_PLL2_REF_SEL: c_int = 9;
pub const IMX8MN_VIDEO_PLL_REF_SEL: c_int = 10;

pub const IMX8MN_DRAM_PLL_REF_SEL: c_int = 11;
pub const IMX8MN_GPU_PLL_REF_SEL: c_int = 12;
pub const IMX8MN_M7_ALT_PLL_REF_SEL: c_int = 13;

pub const IMX8MN_ARM_PLL_REF_SEL: c_int = 14;
pub const IMX8MN_SYS_PLL1_REF_SEL: c_int = 15;
pub const IMX8MN_SYS_PLL2_REF_SEL: c_int = 16;
pub const IMX8MN_SYS_PLL3_REF_SEL: c_int = 17;
pub const IMX8MN_AUDIO_PLL1: c_int = 18;
pub const IMX8MN_AUDIO_PLL2: c_int = 19;
pub const IMX8MN_VIDEO_PLL: c_int = 20;

pub const IMX8MN_DRAM_PLL: c_int = 21;
pub const IMX8MN_GPU_PLL: c_int = 22;
pub const IMX8MN_M7_ALT_PLL: c_int = 23;

pub const IMX8MN_ARM_PLL: c_int = 24;
pub const IMX8MN_SYS_PLL1: c_int = 25;
pub const IMX8MN_SYS_PLL2: c_int = 26;
pub const IMX8MN_SYS_PLL3: c_int = 27;
pub const IMX8MN_AUDIO_PLL1_BYPASS: c_int = 28;
pub const IMX8MN_AUDIO_PLL2_BYPASS: c_int = 29;
pub const IMX8MN_VIDEO_PLL_BYPASS: c_int = 30;

pub const IMX8MN_DRAM_PLL_BYPASS: c_int = 31;
pub const IMX8MN_GPU_PLL_BYPASS: c_int = 32;
pub const IMX8MN_M7_ALT_PLL_BYPASS: c_int = 33;

pub const IMX8MN_ARM_PLL_BYPASS: c_int = 34;
pub const IMX8MN_SYS_PLL1_BYPASS: c_int = 35;
pub const IMX8MN_SYS_PLL2_BYPASS: c_int = 36;
pub const IMX8MN_SYS_PLL3_BYPASS: c_int = 37;
pub const IMX8MN_AUDIO_PLL1_OUT: c_int = 38;
pub const IMX8MN_AUDIO_PLL2_OUT: c_int = 39;
pub const IMX8MN_VIDEO_PLL_OUT: c_int = 40;

pub const IMX8MN_DRAM_PLL_OUT: c_int = 41;
pub const IMX8MN_GPU_PLL_OUT: c_int = 42;
pub const IMX8MN_M7_ALT_PLL_OUT: c_int = 43;

pub const IMX8MN_ARM_PLL_OUT: c_int = 44;
pub const IMX8MN_SYS_PLL1_OUT: c_int = 45;
pub const IMX8MN_SYS_PLL2_OUT: c_int = 46;
pub const IMX8MN_SYS_PLL3_OUT: c_int = 47;
pub const IMX8MN_SYS_PLL1_40M: c_int = 48;
pub const IMX8MN_SYS_PLL1_80M: c_int = 49;
pub const IMX8MN_SYS_PLL1_100M: c_int = 50;
pub const IMX8MN_SYS_PLL1_133M: c_int = 51;
pub const IMX8MN_SYS_PLL1_160M: c_int = 52;
pub const IMX8MN_SYS_PLL1_200M: c_int = 53;
pub const IMX8MN_SYS_PLL1_266M: c_int = 54;
pub const IMX8MN_SYS_PLL1_400M: c_int = 55;
pub const IMX8MN_SYS_PLL1_800M: c_int = 56;
pub const IMX8MN_SYS_PLL2_50M: c_int = 57;
pub const IMX8MN_SYS_PLL2_100M: c_int = 58;
pub const IMX8MN_SYS_PLL2_125M: c_int = 59;
pub const IMX8MN_SYS_PLL2_166M: c_int = 60;
pub const IMX8MN_SYS_PLL2_200M: c_int = 61;
pub const IMX8MN_SYS_PLL2_250M: c_int = 62;
pub const IMX8MN_SYS_PLL2_333M: c_int = 63;
pub const IMX8MN_SYS_PLL2_500M: c_int = 64;
pub const IMX8MN_SYS_PLL2_1000M: c_int = 65;
// CORE CLOCK ROOT
pub const IMX8MN_CLK_A53_SRC: c_int = 66;
pub const IMX8MN_CLK_GPU_CORE_SRC: c_int = 67;
pub const IMX8MN_CLK_GPU_SHADER_SRC: c_int = 68;
pub const IMX8MN_CLK_A53_CG: c_int = 69;
pub const IMX8MN_CLK_GPU_CORE_CG: c_int = 70;
pub const IMX8MN_CLK_GPU_SHADER_CG: c_int = 71;
pub const IMX8MN_CLK_A53_DIV: c_int = 72;
pub const IMX8MN_CLK_GPU_CORE_DIV: c_int = 73;
pub const IMX8MN_CLK_GPU_SHADER_DIV: c_int = 74;
// BUS CLOCK ROOT
pub const IMX8MN_CLK_MAIN_AXI: c_int = 75;
pub const IMX8MN_CLK_ENET_AXI: c_int = 76;
pub const IMX8MN_CLK_NAND_USDHC_BUS: c_int = 77;
pub const IMX8MN_CLK_DISP_AXI: c_int = 78;
pub const IMX8MN_CLK_DISP_APB: c_int = 79;
pub const IMX8MN_CLK_USB_BUS: c_int = 80;
pub const IMX8MN_CLK_GPU_AXI: c_int = 81;
pub const IMX8MN_CLK_GPU_AHB: c_int = 82;
pub const IMX8MN_CLK_NOC: c_int = 83;
pub const IMX8MN_CLK_AHB: c_int = 84;
pub const IMX8MN_CLK_AUDIO_AHB: c_int = 85;
// IPG CLOCK ROOT
pub const IMX8MN_CLK_IPG_ROOT: c_int = 86;
pub const IMX8MN_CLK_IPG_AUDIO_ROOT: c_int = 87;
// IP
pub const IMX8MN_CLK_DRAM_CORE: c_int = 88;
pub const IMX8MN_CLK_DRAM_ALT: c_int = 89;
pub const IMX8MN_CLK_DRAM_APB: c_int = 90;
pub const IMX8MN_CLK_DRAM_ALT_ROOT: c_int = 91;
pub const IMX8MN_CLK_DISP_PIXEL: c_int = 92;
pub const IMX8MN_CLK_SAI2: c_int = 93;
pub const IMX8MN_CLK_SAI3: c_int = 94;
pub const IMX8MN_CLK_SAI5: c_int = 95;
pub const IMX8MN_CLK_SAI6: c_int = 96;
pub const IMX8MN_CLK_SPDIF1: c_int = 97;
pub const IMX8MN_CLK_ENET_REF: c_int = 98;
pub const IMX8MN_CLK_ENET_TIMER: c_int = 99;
pub const IMX8MN_CLK_ENET_PHY_REF: c_int = 100;
pub const IMX8MN_CLK_NAND: c_int = 101;
pub const IMX8MN_CLK_QSPI: c_int = 102;
pub const IMX8MN_CLK_USDHC1: c_int = 103;
pub const IMX8MN_CLK_USDHC2: c_int = 104;
pub const IMX8MN_CLK_I2C1: c_int = 105;
pub const IMX8MN_CLK_I2C2: c_int = 106;
pub const IMX8MN_CLK_I2C3: c_int = 107;
pub const IMX8MN_CLK_I2C4: c_int = 108;
pub const IMX8MN_CLK_UART1: c_int = 109;
pub const IMX8MN_CLK_UART2: c_int = 110;
pub const IMX8MN_CLK_UART3: c_int = 111;
pub const IMX8MN_CLK_UART4: c_int = 112;
pub const IMX8MN_CLK_USB_CORE_REF: c_int = 113;
pub const IMX8MN_CLK_USB_PHY_REF: c_int = 114;
pub const IMX8MN_CLK_ECSPI1: c_int = 115;
pub const IMX8MN_CLK_ECSPI2: c_int = 116;
pub const IMX8MN_CLK_PWM1: c_int = 117;
pub const IMX8MN_CLK_PWM2: c_int = 118;
pub const IMX8MN_CLK_PWM3: c_int = 119;
pub const IMX8MN_CLK_PWM4: c_int = 120;
pub const IMX8MN_CLK_WDOG: c_int = 121;
pub const IMX8MN_CLK_WRCLK: c_int = 122;
pub const IMX8MN_CLK_CLKO1: c_int = 123;
pub const IMX8MN_CLK_CLKO2: c_int = 124;
pub const IMX8MN_CLK_DSI_CORE: c_int = 125;
pub const IMX8MN_CLK_DSI_PHY_REF: c_int = 126;
pub const IMX8MN_CLK_DSI_DBI: c_int = 127;
pub const IMX8MN_CLK_USDHC3: c_int = 128;
pub const IMX8MN_CLK_CAMERA_PIXEL: c_int = 129;
pub const IMX8MN_CLK_CSI1_PHY_REF: c_int = 130;
pub const IMX8MN_CLK_CSI2_PHY_REF: c_int = 131;
pub const IMX8MN_CLK_CSI2_ESC: c_int = 132;
pub const IMX8MN_CLK_ECSPI3: c_int = 133;
pub const IMX8MN_CLK_PDM: c_int = 134;
pub const IMX8MN_CLK_SAI7: c_int = 135;
pub const IMX8MN_CLK_ECSPI1_ROOT: c_int = 136;
pub const IMX8MN_CLK_ECSPI2_ROOT: c_int = 137;
pub const IMX8MN_CLK_ECSPI3_ROOT: c_int = 138;
pub const IMX8MN_CLK_ENET1_ROOT: c_int = 139;
pub const IMX8MN_CLK_GPIO1_ROOT: c_int = 140;
pub const IMX8MN_CLK_GPIO2_ROOT: c_int = 141;
pub const IMX8MN_CLK_GPIO3_ROOT: c_int = 142;
pub const IMX8MN_CLK_GPIO4_ROOT: c_int = 143;
pub const IMX8MN_CLK_GPIO5_ROOT: c_int = 144;
pub const IMX8MN_CLK_I2C1_ROOT: c_int = 145;
pub const IMX8MN_CLK_I2C2_ROOT: c_int = 146;
pub const IMX8MN_CLK_I2C3_ROOT: c_int = 147;
pub const IMX8MN_CLK_I2C4_ROOT: c_int = 148;
pub const IMX8MN_CLK_MU_ROOT: c_int = 149;
pub const IMX8MN_CLK_OCOTP_ROOT: c_int = 150;
pub const IMX8MN_CLK_PWM1_ROOT: c_int = 151;
pub const IMX8MN_CLK_PWM2_ROOT: c_int = 152;
pub const IMX8MN_CLK_PWM3_ROOT: c_int = 153;
pub const IMX8MN_CLK_PWM4_ROOT: c_int = 154;
pub const IMX8MN_CLK_QSPI_ROOT: c_int = 155;
pub const IMX8MN_CLK_NAND_ROOT: c_int = 156;
pub const IMX8MN_CLK_SAI2_ROOT: c_int = 157;
pub const IMX8MN_CLK_SAI2_IPG: c_int = 158;
pub const IMX8MN_CLK_SAI3_ROOT: c_int = 159;
pub const IMX8MN_CLK_SAI3_IPG: c_int = 160;
pub const IMX8MN_CLK_SAI5_ROOT: c_int = 161;
pub const IMX8MN_CLK_SAI5_IPG: c_int = 162;
pub const IMX8MN_CLK_SAI6_ROOT: c_int = 163;
pub const IMX8MN_CLK_SAI6_IPG: c_int = 164;
pub const IMX8MN_CLK_SAI7_ROOT: c_int = 165;
pub const IMX8MN_CLK_SAI7_IPG: c_int = 166;
pub const IMX8MN_CLK_SDMA1_ROOT: c_int = 167;
pub const IMX8MN_CLK_SDMA2_ROOT: c_int = 168;
pub const IMX8MN_CLK_UART1_ROOT: c_int = 169;
pub const IMX8MN_CLK_UART2_ROOT: c_int = 170;
pub const IMX8MN_CLK_UART3_ROOT: c_int = 171;
pub const IMX8MN_CLK_UART4_ROOT: c_int = 172;
pub const IMX8MN_CLK_USB1_CTRL_ROOT: c_int = 173;
pub const IMX8MN_CLK_USDHC1_ROOT: c_int = 174;
pub const IMX8MN_CLK_USDHC2_ROOT: c_int = 175;
pub const IMX8MN_CLK_WDOG1_ROOT: c_int = 176;
pub const IMX8MN_CLK_WDOG2_ROOT: c_int = 177;
pub const IMX8MN_CLK_WDOG3_ROOT: c_int = 178;
pub const IMX8MN_CLK_GPU_BUS_ROOT: c_int = 179;
pub const IMX8MN_CLK_ASRC_ROOT: c_int = 180;
pub const IMX8MN_CLK_GPU3D_ROOT: c_int = 181;
pub const IMX8MN_CLK_PDM_ROOT: c_int = 182;
pub const IMX8MN_CLK_PDM_IPG: c_int = 183;
pub const IMX8MN_CLK_DISP_AXI_ROOT: c_int = 184;
pub const IMX8MN_CLK_DISP_APB_ROOT: c_int = 185;
pub const IMX8MN_CLK_DISP_PIXEL_ROOT: c_int = 186;
pub const IMX8MN_CLK_CAMERA_PIXEL_ROOT: c_int = 187;
pub const IMX8MN_CLK_USDHC3_ROOT: c_int = 188;
pub const IMX8MN_CLK_SDMA3_ROOT: c_int = 189;
pub const IMX8MN_CLK_TMU_ROOT: c_int = 190;
pub const IMX8MN_CLK_ARM: c_int = 191;
pub const IMX8MN_CLK_NAND_USDHC_BUS_RAWNAND_CLK: c_int = 192;
pub const IMX8MN_CLK_GPU_CORE_ROOT: c_int = 193;
pub const IMX8MN_CLK_GIC: c_int = 194;
pub const IMX8MN_SYS_PLL1_40M_CG: c_int = 195;
pub const IMX8MN_SYS_PLL1_80M_CG: c_int = 196;
pub const IMX8MN_SYS_PLL1_100M_CG: c_int = 197;
pub const IMX8MN_SYS_PLL1_133M_CG: c_int = 198;
pub const IMX8MN_SYS_PLL1_160M_CG: c_int = 199;
pub const IMX8MN_SYS_PLL1_200M_CG: c_int = 200;
pub const IMX8MN_SYS_PLL1_266M_CG: c_int = 201;
pub const IMX8MN_SYS_PLL1_400M_CG: c_int = 202;
pub const IMX8MN_SYS_PLL2_50M_CG: c_int = 203;
pub const IMX8MN_SYS_PLL2_100M_CG: c_int = 204;
pub const IMX8MN_SYS_PLL2_125M_CG: c_int = 205;
pub const IMX8MN_SYS_PLL2_166M_CG: c_int = 206;
pub const IMX8MN_SYS_PLL2_200M_CG: c_int = 207;
pub const IMX8MN_SYS_PLL2_250M_CG: c_int = 208;
pub const IMX8MN_SYS_PLL2_333M_CG: c_int = 209;
pub const IMX8MN_SYS_PLL2_500M_CG: c_int = 210;
pub const IMX8MN_CLK_SNVS_ROOT: c_int = 211;
pub const IMX8MN_CLK_GPU_CORE: c_int = 212;
pub const IMX8MN_CLK_GPU_SHADER: c_int = 213;
pub const IMX8MN_CLK_A53_CORE: c_int = 214;
pub const IMX8MN_CLK_CLKOUT1_SEL: c_int = 215;
pub const IMX8MN_CLK_CLKOUT1_DIV: c_int = 216;
pub const IMX8MN_CLK_CLKOUT1: c_int = 217;
pub const IMX8MN_CLK_CLKOUT2_SEL: c_int = 218;
pub const IMX8MN_CLK_CLKOUT2_DIV: c_int = 219;
pub const IMX8MN_CLK_CLKOUT2: c_int = 220;
pub const IMX8MN_CLK_M7_CORE: c_int = 221;
pub const IMX8MN_CLK_GPT_3M: c_int = 222;
pub const IMX8MN_CLK_GPT1: c_int = 223;
pub const IMX8MN_CLK_GPT1_ROOT: c_int = 224;
pub const IMX8MN_CLK_GPT2: c_int = 225;
pub const IMX8MN_CLK_GPT2_ROOT: c_int = 226;
pub const IMX8MN_CLK_GPT3: c_int = 227;
pub const IMX8MN_CLK_GPT3_ROOT: c_int = 228;
pub const IMX8MN_CLK_GPT4: c_int = 229;
pub const IMX8MN_CLK_GPT4_ROOT: c_int = 230;
pub const IMX8MN_CLK_GPT5: c_int = 231;
pub const IMX8MN_CLK_GPT5_ROOT: c_int = 232;
pub const IMX8MN_CLK_GPT6: c_int = 233;
pub const IMX8MN_CLK_GPT6_ROOT: c_int = 234;
pub const IMX8MN_CLK_END: c_int = 235;
