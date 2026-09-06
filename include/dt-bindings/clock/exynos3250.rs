//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos3250.h
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Tomasz Figa <t.figa@samsung.com>
//
// Device Tree binding constants for Samsung Exynos3250 clock controllers.
//
// Let each exported clock get a unique index, which is used on DT-enabled
// platforms to lookup the clock from a clock specifier. These indices are
// therefore considered an ABI and so must not be changed. This implies
// that new clocks should be added either in free spaces between clock groups
// or at the end.
//
// Main CMU
//
pub const CLK_OSCSEL: c_int = 1;
pub const CLK_FIN_PLL: c_int = 2;
pub const CLK_FOUT_APLL: c_int = 3;
pub const CLK_FOUT_VPLL: c_int = 4;
pub const CLK_FOUT_UPLL: c_int = 5;
pub const CLK_FOUT_MPLL: c_int = 6;
pub const CLK_ARM_CLK: c_int = 7;
// Muxes
pub const CLK_MOUT_MPLL_USER_L: c_int = 16;
pub const CLK_MOUT_GDL: c_int = 17;
pub const CLK_MOUT_MPLL_USER_R: c_int = 18;
pub const CLK_MOUT_GDR: c_int = 19;
pub const CLK_MOUT_EBI: c_int = 20;
pub const CLK_MOUT_ACLK_200: c_int = 21;
pub const CLK_MOUT_ACLK_160: c_int = 22;
pub const CLK_MOUT_ACLK_100: c_int = 23;
pub const CLK_MOUT_ACLK_266_1: c_int = 24;
pub const CLK_MOUT_ACLK_266_0: c_int = 25;
pub const CLK_MOUT_ACLK_266: c_int = 26;
pub const CLK_MOUT_VPLL: c_int = 27;
pub const CLK_MOUT_EPLL_USER: c_int = 28;
pub const CLK_MOUT_EBI_1: c_int = 29;
pub const CLK_MOUT_UPLL: c_int = 30;
pub const CLK_MOUT_ACLK_400_MCUISP_SUB: c_int = 31;
pub const CLK_MOUT_MPLL: c_int = 32;
pub const CLK_MOUT_ACLK_400_MCUISP: c_int = 33;
pub const CLK_MOUT_VPLLSRC: c_int = 34;
pub const CLK_MOUT_CAM1: c_int = 35;
pub const CLK_MOUT_CAM_BLK: c_int = 36;
pub const CLK_MOUT_MFC: c_int = 37;
pub const CLK_MOUT_MFC_1: c_int = 38;
pub const CLK_MOUT_MFC_0: c_int = 39;
pub const CLK_MOUT_G3D: c_int = 40;
pub const CLK_MOUT_G3D_1: c_int = 41;
pub const CLK_MOUT_G3D_0: c_int = 42;
pub const CLK_MOUT_MIPI0: c_int = 43;
pub const CLK_MOUT_FIMD0: c_int = 44;
pub const CLK_MOUT_UART_ISP: c_int = 45;
pub const CLK_MOUT_SPI1_ISP: c_int = 46;
pub const CLK_MOUT_SPI0_ISP: c_int = 47;
pub const CLK_MOUT_TSADC: c_int = 48;
pub const CLK_MOUT_MMC1: c_int = 49;
pub const CLK_MOUT_MMC0: c_int = 50;
pub const CLK_MOUT_UART1: c_int = 51;
pub const CLK_MOUT_UART0: c_int = 52;
pub const CLK_MOUT_SPI1: c_int = 53;
pub const CLK_MOUT_SPI0: c_int = 54;
pub const CLK_MOUT_AUDIO: c_int = 55;
pub const CLK_MOUT_MPLL_USER_C: c_int = 56;
pub const CLK_MOUT_HPM: c_int = 57;
pub const CLK_MOUT_CORE: c_int = 58;
pub const CLK_MOUT_APLL: c_int = 59;
pub const CLK_MOUT_ACLK_266_SUB: c_int = 60;
pub const CLK_MOUT_UART2: c_int = 61;
pub const CLK_MOUT_MMC2: c_int = 62;
// Dividers
pub const CLK_DIV_GPL: c_int = 64;
pub const CLK_DIV_GDL: c_int = 65;
pub const CLK_DIV_GPR: c_int = 66;
pub const CLK_DIV_GDR: c_int = 67;
pub const CLK_DIV_MPLL_PRE: c_int = 68;
pub const CLK_DIV_ACLK_400_MCUISP: c_int = 69;
pub const CLK_DIV_EBI: c_int = 70;
pub const CLK_DIV_ACLK_200: c_int = 71;
pub const CLK_DIV_ACLK_160: c_int = 72;
pub const CLK_DIV_ACLK_100: c_int = 73;
pub const CLK_DIV_ACLK_266: c_int = 74;
pub const CLK_DIV_CAM1: c_int = 75;
pub const CLK_DIV_CAM_BLK: c_int = 76;
pub const CLK_DIV_MFC: c_int = 77;
pub const CLK_DIV_G3D: c_int = 78;
pub const CLK_DIV_MIPI0_PRE: c_int = 79;
pub const CLK_DIV_MIPI0: c_int = 80;
pub const CLK_DIV_FIMD0: c_int = 81;
pub const CLK_DIV_UART_ISP: c_int = 82;
pub const CLK_DIV_SPI1_ISP_PRE: c_int = 83;
pub const CLK_DIV_SPI1_ISP: c_int = 84;
pub const CLK_DIV_SPI0_ISP_PRE: c_int = 85;
pub const CLK_DIV_SPI0_ISP: c_int = 86;
pub const CLK_DIV_TSADC_PRE: c_int = 87;
pub const CLK_DIV_TSADC: c_int = 88;
pub const CLK_DIV_MMC1_PRE: c_int = 89;
pub const CLK_DIV_MMC1: c_int = 90;
pub const CLK_DIV_MMC0_PRE: c_int = 91;
pub const CLK_DIV_MMC0: c_int = 92;
pub const CLK_DIV_UART1: c_int = 93;
pub const CLK_DIV_UART0: c_int = 94;
pub const CLK_DIV_SPI1_PRE: c_int = 95;
pub const CLK_DIV_SPI1: c_int = 96;
pub const CLK_DIV_SPI0_PRE: c_int = 97;
pub const CLK_DIV_SPI0: c_int = 98;
pub const CLK_DIV_PCM: c_int = 99;
pub const CLK_DIV_AUDIO: c_int = 100;
pub const CLK_DIV_I2S: c_int = 101;
pub const CLK_DIV_CORE2: c_int = 102;
pub const CLK_DIV_APLL: c_int = 103;
pub const CLK_DIV_PCLK_DBG: c_int = 104;
pub const CLK_DIV_ATB: c_int = 105;
pub const CLK_DIV_COREM: c_int = 106;
pub const CLK_DIV_CORE: c_int = 107;
pub const CLK_DIV_HPM: c_int = 108;
pub const CLK_DIV_COPY: c_int = 109;
pub const CLK_DIV_UART2: c_int = 110;
pub const CLK_DIV_MMC2_PRE: c_int = 111;
pub const CLK_DIV_MMC2: c_int = 112;
// Gates
pub const CLK_ASYNC_G3D: c_int = 128;
pub const CLK_ASYNC_MFCL: c_int = 129;
pub const CLK_PPMULEFT: c_int = 130;
pub const CLK_GPIO_LEFT: c_int = 131;
pub const CLK_ASYNC_ISPMX: c_int = 132;
pub const CLK_ASYNC_FSYSD: c_int = 133;
pub const CLK_ASYNC_LCD0X: c_int = 134;
pub const CLK_ASYNC_CAMX: c_int = 135;
pub const CLK_PPMURIGHT: c_int = 136;
pub const CLK_GPIO_RIGHT: c_int = 137;
pub const CLK_MONOCNT: c_int = 138;
pub const CLK_TZPC6: c_int = 139;
pub const CLK_PROVISIONKEY1: c_int = 140;
pub const CLK_PROVISIONKEY0: c_int = 141;
pub const CLK_CMU_ISPPART: c_int = 142;
pub const CLK_TMU_APBIF: c_int = 143;
pub const CLK_KEYIF: c_int = 144;
pub const CLK_RTC: c_int = 145;
pub const CLK_WDT: c_int = 146;
pub const CLK_MCT: c_int = 147;
pub const CLK_SECKEY: c_int = 148;
pub const CLK_TZPC5: c_int = 149;
pub const CLK_TZPC4: c_int = 150;
pub const CLK_TZPC3: c_int = 151;
pub const CLK_TZPC2: c_int = 152;
pub const CLK_TZPC1: c_int = 153;
pub const CLK_TZPC0: c_int = 154;
pub const CLK_CMU_COREPART: c_int = 155;
pub const CLK_CMU_TOPPART: c_int = 156;
pub const CLK_PMU_APBIF: c_int = 157;
pub const CLK_SYSREG: c_int = 158;
pub const CLK_CHIP_ID: c_int = 159;
pub const CLK_QEJPEG: c_int = 160;
pub const CLK_PIXELASYNCM1: c_int = 161;
pub const CLK_PIXELASYNCM0: c_int = 162;
pub const CLK_PPMUCAMIF: c_int = 163;
pub const CLK_QEM2MSCALER: c_int = 164;
pub const CLK_QEGSCALER1: c_int = 165;
pub const CLK_QEGSCALER0: c_int = 166;
pub const CLK_SMMUJPEG: c_int = 167;
pub const CLK_SMMUM2M2SCALER: c_int = 168;
pub const CLK_SMMUGSCALER1: c_int = 169;
pub const CLK_SMMUGSCALER0: c_int = 170;
pub const CLK_JPEG: c_int = 171;
pub const CLK_M2MSCALER: c_int = 172;
pub const CLK_GSCALER1: c_int = 173;
pub const CLK_GSCALER0: c_int = 174;
pub const CLK_QEMFC: c_int = 175;
pub const CLK_PPMUMFC_L: c_int = 176;
pub const CLK_SMMUMFC_L: c_int = 177;
pub const CLK_MFC: c_int = 178;
pub const CLK_SMMUG3D: c_int = 179;
pub const CLK_QEG3D: c_int = 180;
pub const CLK_PPMUG3D: c_int = 181;
pub const CLK_G3D: c_int = 182;
pub const CLK_QE_CH1_LCD: c_int = 183;
pub const CLK_QE_CH0_LCD: c_int = 184;
pub const CLK_PPMULCD0: c_int = 185;
pub const CLK_SMMUFIMD0: c_int = 186;
pub const CLK_DSIM0: c_int = 187;
pub const CLK_FIMD0: c_int = 188;
pub const CLK_CAM1: c_int = 189;
pub const CLK_UART_ISP_TOP: c_int = 190;
pub const CLK_SPI1_ISP_TOP: c_int = 191;
pub const CLK_SPI0_ISP_TOP: c_int = 192;
pub const CLK_TSADC: c_int = 193;
pub const CLK_PPMUFILE: c_int = 194;
pub const CLK_USBOTG: c_int = 195;
pub const CLK_USBHOST: c_int = 196;
pub const CLK_SROMC: c_int = 197;
pub const CLK_SDMMC1: c_int = 198;
pub const CLK_SDMMC0: c_int = 199;
pub const CLK_PDMA1: c_int = 200;
pub const CLK_PDMA0: c_int = 201;
pub const CLK_PWM: c_int = 202;
pub const CLK_PCM: c_int = 203;
pub const CLK_I2S: c_int = 204;
pub const CLK_SPI1: c_int = 205;
pub const CLK_SPI0: c_int = 206;
pub const CLK_I2C7: c_int = 207;
pub const CLK_I2C6: c_int = 208;
pub const CLK_I2C5: c_int = 209;
pub const CLK_I2C4: c_int = 210;
pub const CLK_I2C3: c_int = 211;
pub const CLK_I2C2: c_int = 212;
pub const CLK_I2C1: c_int = 213;
pub const CLK_I2C0: c_int = 214;
pub const CLK_UART1: c_int = 215;
pub const CLK_UART0: c_int = 216;
pub const CLK_BLOCK_LCD: c_int = 217;
pub const CLK_BLOCK_G3D: c_int = 218;
pub const CLK_BLOCK_MFC: c_int = 219;
pub const CLK_BLOCK_CAM: c_int = 220;
pub const CLK_SMIES: c_int = 221;
pub const CLK_UART2: c_int = 222;
pub const CLK_SDMMC2: c_int = 223;
// Special clocks
pub const CLK_SCLK_JPEG: c_int = 224;
pub const CLK_SCLK_M2MSCALER: c_int = 225;
pub const CLK_SCLK_GSCALER1: c_int = 226;
pub const CLK_SCLK_GSCALER0: c_int = 227;
pub const CLK_SCLK_MFC: c_int = 228;
pub const CLK_SCLK_G3D: c_int = 229;
pub const CLK_SCLK_MIPIDPHY2L: c_int = 230;
pub const CLK_SCLK_MIPI0: c_int = 231;
pub const CLK_SCLK_FIMD0: c_int = 232;
pub const CLK_SCLK_CAM1: c_int = 233;
pub const CLK_SCLK_UART_ISP: c_int = 234;
pub const CLK_SCLK_SPI1_ISP: c_int = 235;
pub const CLK_SCLK_SPI0_ISP: c_int = 236;
pub const CLK_SCLK_UPLL: c_int = 237;
pub const CLK_SCLK_TSADC: c_int = 238;
pub const CLK_SCLK_EBI: c_int = 239;
pub const CLK_SCLK_MMC1: c_int = 240;
pub const CLK_SCLK_MMC0: c_int = 241;
pub const CLK_SCLK_I2S: c_int = 242;
pub const CLK_SCLK_PCM: c_int = 243;
pub const CLK_SCLK_SPI1: c_int = 244;
pub const CLK_SCLK_SPI0: c_int = 245;
pub const CLK_SCLK_UART1: c_int = 246;
pub const CLK_SCLK_UART0: c_int = 247;
pub const CLK_SCLK_UART2: c_int = 248;
pub const CLK_SCLK_MMC2: c_int = 249;
//
// CMU DMC
//
pub const CLK_FOUT_BPLL: c_int = 1;
pub const CLK_FOUT_EPLL: c_int = 2;
// Muxes
pub const CLK_MOUT_MPLL_MIF: c_int = 8;
pub const CLK_MOUT_BPLL: c_int = 9;
pub const CLK_MOUT_DPHY: c_int = 10;
pub const CLK_MOUT_DMC_BUS: c_int = 11;
pub const CLK_MOUT_EPLL: c_int = 12;
// Dividers
pub const CLK_DIV_DMC: c_int = 16;
pub const CLK_DIV_DPHY: c_int = 17;
pub const CLK_DIV_DMC_PRE: c_int = 18;
pub const CLK_DIV_DMCP: c_int = 19;
pub const CLK_DIV_DMCD: c_int = 20;
//
// CMU ISP
//
// Dividers
pub const CLK_DIV_ISP1: c_int = 1;
pub const CLK_DIV_ISP0: c_int = 2;
pub const CLK_DIV_MCUISP1: c_int = 3;
pub const CLK_DIV_MCUISP0: c_int = 4;
pub const CLK_DIV_MPWM: c_int = 5;
// Gates
pub const CLK_UART_ISP: c_int = 8;
pub const CLK_WDT_ISP: c_int = 9;
pub const CLK_PWM_ISP: c_int = 10;
pub const CLK_I2C1_ISP: c_int = 11;
pub const CLK_I2C0_ISP: c_int = 12;
pub const CLK_MPWM_ISP: c_int = 13;
pub const CLK_MCUCTL_ISP: c_int = 14;
pub const CLK_PPMUISPX: c_int = 15;
pub const CLK_PPMUISPMX: c_int = 16;
pub const CLK_QE_LITE1: c_int = 17;
pub const CLK_QE_LITE0: c_int = 18;
pub const CLK_QE_FD: c_int = 19;
pub const CLK_QE_DRC: c_int = 20;
pub const CLK_QE_ISP: c_int = 21;
pub const CLK_CSIS1: c_int = 22;
pub const CLK_SMMU_LITE1: c_int = 23;
pub const CLK_SMMU_LITE0: c_int = 24;
pub const CLK_SMMU_FD: c_int = 25;
pub const CLK_SMMU_DRC: c_int = 26;
pub const CLK_SMMU_ISP: c_int = 27;
pub const CLK_GICISP: c_int = 28;
pub const CLK_CSIS0: c_int = 29;
pub const CLK_MCUISP: c_int = 30;
pub const CLK_LITE1: c_int = 31;
pub const CLK_LITE0: c_int = 32;
pub const CLK_FD: c_int = 33;
pub const CLK_DRC: c_int = 34;
pub const CLK_ISP: c_int = 35;
pub const CLK_QE_ISPCX: c_int = 36;
pub const CLK_QE_SCALERP: c_int = 37;
pub const CLK_QE_SCALERC: c_int = 38;
pub const CLK_SMMU_SCALERP: c_int = 39;
pub const CLK_SMMU_SCALERC: c_int = 40;
pub const CLK_SCALERP: c_int = 41;
pub const CLK_SCALERC: c_int = 42;
pub const CLK_SPI1_ISP: c_int = 43;
pub const CLK_SPI0_ISP: c_int = 44;
pub const CLK_SMMU_ISPCX: c_int = 45;
pub const CLK_ASYNCAXIM: c_int = 46;
pub const CLK_SCLK_MPWM_ISP: c_int = 47;
