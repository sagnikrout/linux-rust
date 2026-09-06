//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos5250.h
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Author: Andrzej Hajda <a.hajda@samsung.com>
//
// Device Tree binding constants for Exynos5250 clock controller.
//
// core clocks
pub const CLK_FIN_PLL: c_int = 1;
pub const CLK_FOUT_APLL: c_int = 2;
pub const CLK_FOUT_MPLL: c_int = 3;
pub const CLK_FOUT_BPLL: c_int = 4;
pub const CLK_FOUT_GPLL: c_int = 5;
pub const CLK_FOUT_CPLL: c_int = 6;
pub const CLK_FOUT_EPLL: c_int = 7;
pub const CLK_FOUT_VPLL: c_int = 8;
pub const CLK_ARM_CLK: c_int = 9;
pub const CLK_DIV_ARM2: c_int = 10;
// gate for special clocks (sclk)
pub const CLK_SCLK_CAM_BAYER: c_int = 128;
pub const CLK_SCLK_CAM0: c_int = 129;
pub const CLK_SCLK_CAM1: c_int = 130;
pub const CLK_SCLK_GSCL_WA: c_int = 131;
pub const CLK_SCLK_GSCL_WB: c_int = 132;
pub const CLK_SCLK_FIMD1: c_int = 133;
pub const CLK_SCLK_MIPI1: c_int = 134;
pub const CLK_SCLK_DP: c_int = 135;
pub const CLK_SCLK_HDMI: c_int = 136;
pub const CLK_SCLK_PIXEL: c_int = 137;
pub const CLK_SCLK_AUDIO0: c_int = 138;
pub const CLK_SCLK_MMC0: c_int = 139;
pub const CLK_SCLK_MMC1: c_int = 140;
pub const CLK_SCLK_MMC2: c_int = 141;
pub const CLK_SCLK_MMC3: c_int = 142;
pub const CLK_SCLK_SATA: c_int = 143;
pub const CLK_SCLK_USB3: c_int = 144;
pub const CLK_SCLK_JPEG: c_int = 145;
pub const CLK_SCLK_UART0: c_int = 146;
pub const CLK_SCLK_UART1: c_int = 147;
pub const CLK_SCLK_UART2: c_int = 148;
pub const CLK_SCLK_UART3: c_int = 149;
pub const CLK_SCLK_PWM: c_int = 150;
pub const CLK_SCLK_AUDIO1: c_int = 151;
pub const CLK_SCLK_AUDIO2: c_int = 152;
pub const CLK_SCLK_SPDIF: c_int = 153;
pub const CLK_SCLK_SPI0: c_int = 154;
pub const CLK_SCLK_SPI1: c_int = 155;
pub const CLK_SCLK_SPI2: c_int = 156;
pub const CLK_DIV_I2S1: c_int = 157;
pub const CLK_DIV_I2S2: c_int = 158;
pub const CLK_SCLK_HDMIPHY: c_int = 159;
pub const CLK_DIV_PCM0: c_int = 160;
// gate clocks
pub const CLK_GSCL0: c_int = 256;
pub const CLK_GSCL1: c_int = 257;
pub const CLK_GSCL2: c_int = 258;
pub const CLK_GSCL3: c_int = 259;
pub const CLK_GSCL_WA: c_int = 260;
pub const CLK_GSCL_WB: c_int = 261;
pub const CLK_SMMU_GSCL0: c_int = 262;
pub const CLK_SMMU_GSCL1: c_int = 263;
pub const CLK_SMMU_GSCL2: c_int = 264;
pub const CLK_SMMU_GSCL3: c_int = 265;
pub const CLK_MFC: c_int = 266;
pub const CLK_SMMU_MFCL: c_int = 267;
pub const CLK_SMMU_MFCR: c_int = 268;
pub const CLK_ROTATOR: c_int = 269;
pub const CLK_JPEG: c_int = 270;
pub const CLK_MDMA1: c_int = 271;
pub const CLK_SMMU_ROTATOR: c_int = 272;
pub const CLK_SMMU_JPEG: c_int = 273;
pub const CLK_SMMU_MDMA1: c_int = 274;
pub const CLK_PDMA0: c_int = 275;
pub const CLK_PDMA1: c_int = 276;
pub const CLK_SATA: c_int = 277;
pub const CLK_USBOTG: c_int = 278;
pub const CLK_MIPI_HSI: c_int = 279;
pub const CLK_SDMMC0: c_int = 280;
pub const CLK_SDMMC1: c_int = 281;
pub const CLK_SDMMC2: c_int = 282;
pub const CLK_SDMMC3: c_int = 283;
pub const CLK_SROMC: c_int = 284;
pub const CLK_USB2: c_int = 285;
pub const CLK_USB3: c_int = 286;
pub const CLK_SATA_PHYCTRL: c_int = 287;
pub const CLK_SATA_PHYI2C: c_int = 288;
pub const CLK_UART0: c_int = 289;
pub const CLK_UART1: c_int = 290;
pub const CLK_UART2: c_int = 291;
pub const CLK_UART3: c_int = 292;
pub const CLK_UART4: c_int = 293;
pub const CLK_I2C0: c_int = 294;
pub const CLK_I2C1: c_int = 295;
pub const CLK_I2C2: c_int = 296;
pub const CLK_I2C3: c_int = 297;
pub const CLK_I2C4: c_int = 298;
pub const CLK_I2C5: c_int = 299;
pub const CLK_I2C6: c_int = 300;
pub const CLK_I2C7: c_int = 301;
pub const CLK_I2C_HDMI: c_int = 302;
pub const CLK_ADC: c_int = 303;
pub const CLK_SPI0: c_int = 304;
pub const CLK_SPI1: c_int = 305;
pub const CLK_SPI2: c_int = 306;
pub const CLK_I2S1: c_int = 307;
pub const CLK_I2S2: c_int = 308;
pub const CLK_PCM1: c_int = 309;
pub const CLK_PCM2: c_int = 310;
pub const CLK_PWM: c_int = 311;
pub const CLK_SPDIF: c_int = 312;
pub const CLK_AC97: c_int = 313;
pub const CLK_HSI2C0: c_int = 314;
pub const CLK_HSI2C1: c_int = 315;
pub const CLK_HSI2C2: c_int = 316;
pub const CLK_HSI2C3: c_int = 317;
pub const CLK_CHIPID: c_int = 318;
pub const CLK_SYSREG: c_int = 319;
pub const CLK_PMU: c_int = 320;
pub const CLK_CMU_TOP: c_int = 321;
pub const CLK_CMU_CORE: c_int = 322;
pub const CLK_CMU_MEM: c_int = 323;
pub const CLK_TZPC0: c_int = 324;
pub const CLK_TZPC1: c_int = 325;
pub const CLK_TZPC2: c_int = 326;
pub const CLK_TZPC3: c_int = 327;
pub const CLK_TZPC4: c_int = 328;
pub const CLK_TZPC5: c_int = 329;
pub const CLK_TZPC6: c_int = 330;
pub const CLK_TZPC7: c_int = 331;
pub const CLK_TZPC8: c_int = 332;
pub const CLK_TZPC9: c_int = 333;
pub const CLK_HDMI_CEC: c_int = 334;
pub const CLK_MCT: c_int = 335;
pub const CLK_WDT: c_int = 336;
pub const CLK_RTC: c_int = 337;
pub const CLK_TMU: c_int = 338;
pub const CLK_FIMD1: c_int = 339;
pub const CLK_MIE1: c_int = 340;
pub const CLK_DSIM0: c_int = 341;
pub const CLK_DP: c_int = 342;
pub const CLK_MIXER: c_int = 343;
pub const CLK_HDMI: c_int = 344;
pub const CLK_G2D: c_int = 345;
pub const CLK_MDMA0: c_int = 346;
pub const CLK_SMMU_MDMA0: c_int = 347;
pub const CLK_SSS: c_int = 348;
pub const CLK_G3D: c_int = 349;
pub const CLK_SMMU_TV: c_int = 350;
pub const CLK_SMMU_FIMD1: c_int = 351;
pub const CLK_SMMU_2D: c_int = 352;
pub const CLK_SMMU_FIMC_ISP: c_int = 353;
pub const CLK_SMMU_FIMC_DRC: c_int = 354;
pub const CLK_SMMU_FIMC_SCC: c_int = 355;
pub const CLK_SMMU_FIMC_SCP: c_int = 356;
pub const CLK_SMMU_FIMC_FD: c_int = 357;
pub const CLK_SMMU_FIMC_MCU: c_int = 358;
pub const CLK_SMMU_FIMC_ODC: c_int = 359;
pub const CLK_SMMU_FIMC_DIS0: c_int = 360;
pub const CLK_SMMU_FIMC_DIS1: c_int = 361;
pub const CLK_SMMU_FIMC_3DNR: c_int = 362;
pub const CLK_SMMU_FIMC_LITE0: c_int = 363;
pub const CLK_SMMU_FIMC_LITE1: c_int = 364;
pub const CLK_CAMIF_TOP: c_int = 365;
// mux clocks
pub const CLK_MOUT_HDMI: c_int = 1024;
pub const CLK_MOUT_GPLL: c_int = 1025;
pub const CLK_MOUT_ACLK200_DISP1_SUB: c_int = 1026;
pub const CLK_MOUT_ACLK300_DISP1_SUB: c_int = 1027;
pub const CLK_MOUT_APLL: c_int = 1028;
pub const CLK_MOUT_MPLL: c_int = 1029;
pub const CLK_MOUT_VPLLSRC: c_int = 1030;
