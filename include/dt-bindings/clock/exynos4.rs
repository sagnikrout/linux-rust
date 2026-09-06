//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/exynos4.h
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
// Device Tree binding constants for Exynos4 clock controller.
//
// core clocks
pub const CLK_XXTI: c_int = 1;
pub const CLK_XUSBXTI: c_int = 2;
pub const CLK_FIN_PLL: c_int = 3;
pub const CLK_FOUT_APLL: c_int = 4;
pub const CLK_FOUT_MPLL: c_int = 5;
pub const CLK_FOUT_EPLL: c_int = 6;
pub const CLK_FOUT_VPLL: c_int = 7;
pub const CLK_SCLK_APLL: c_int = 8;
pub const CLK_SCLK_MPLL: c_int = 9;
pub const CLK_SCLK_EPLL: c_int = 10;
pub const CLK_SCLK_VPLL: c_int = 11;
pub const CLK_ARM_CLK: c_int = 12;
pub const CLK_ACLK200: c_int = 13;
pub const CLK_ACLK100: c_int = 14;
pub const CLK_ACLK160: c_int = 15;
pub const CLK_ACLK133: c_int = 16;

pub const CLK_MOUT_CORE: c_int = 19;
pub const CLK_MOUT_APLL: c_int = 20;
pub const CLK_SCLK_HDMIPHY: c_int = 22;
pub const CLK_OUT_DMC: c_int = 23;
pub const CLK_OUT_TOP: c_int = 24;
pub const CLK_OUT_LEFTBUS: c_int = 25;
pub const CLK_OUT_RIGHTBUS: c_int = 26;
pub const CLK_OUT_CPU: c_int = 27;
// gate for special clocks (sclk)
pub const CLK_SCLK_FIMC0: c_int = 128;
pub const CLK_SCLK_FIMC1: c_int = 129;
pub const CLK_SCLK_FIMC2: c_int = 130;
pub const CLK_SCLK_FIMC3: c_int = 131;
pub const CLK_SCLK_CAM0: c_int = 132;
pub const CLK_SCLK_CAM1: c_int = 133;
pub const CLK_SCLK_CSIS0: c_int = 134;
pub const CLK_SCLK_CSIS1: c_int = 135;
pub const CLK_SCLK_HDMI: c_int = 136;
pub const CLK_SCLK_MIXER: c_int = 137;
pub const CLK_SCLK_DAC: c_int = 138;
pub const CLK_SCLK_PIXEL: c_int = 139;
pub const CLK_SCLK_FIMD0: c_int = 140;

pub const CLK_SCLK_MDNIE_PWM0: c_int = 142;
pub const CLK_SCLK_MIPI0: c_int = 143;
pub const CLK_SCLK_AUDIO0: c_int = 144;
pub const CLK_SCLK_MMC0: c_int = 145;
pub const CLK_SCLK_MMC1: c_int = 146;
pub const CLK_SCLK_MMC2: c_int = 147;
pub const CLK_SCLK_MMC3: c_int = 148;
pub const CLK_SCLK_MMC4: c_int = 149;

pub const CLK_SCLK_UART0: c_int = 151;
pub const CLK_SCLK_UART1: c_int = 152;
pub const CLK_SCLK_UART2: c_int = 153;
pub const CLK_SCLK_UART3: c_int = 154;
pub const CLK_SCLK_UART4: c_int = 155;
pub const CLK_SCLK_AUDIO1: c_int = 156;
pub const CLK_SCLK_AUDIO2: c_int = 157;
pub const CLK_SCLK_SPDIF: c_int = 158;
pub const CLK_SCLK_SPI0: c_int = 159;
pub const CLK_SCLK_SPI1: c_int = 160;
pub const CLK_SCLK_SPI2: c_int = 161;
pub const CLK_SCLK_SLIMBUS: c_int = 162;

pub const CLK_SCLK_PCM1: c_int = 165;
pub const CLK_SCLK_PCM2: c_int = 166;
pub const CLK_SCLK_I2S1: c_int = 167;
pub const CLK_SCLK_I2S2: c_int = 168;

pub const CLK_SCLK_MFC: c_int = 170;
pub const CLK_SCLK_PCM0: c_int = 171;
pub const CLK_SCLK_G3D: c_int = 172;

pub const CLK_SCLK_FIMG2D: c_int = 177;
// gate clocks
pub const CLK_SSS: c_int = 255;
pub const CLK_FIMC0: c_int = 256;
pub const CLK_FIMC1: c_int = 257;
pub const CLK_FIMC2: c_int = 258;
pub const CLK_FIMC3: c_int = 259;
pub const CLK_CSIS0: c_int = 260;
pub const CLK_CSIS1: c_int = 261;
pub const CLK_JPEG: c_int = 262;
pub const CLK_SMMU_FIMC0: c_int = 263;
pub const CLK_SMMU_FIMC1: c_int = 264;
pub const CLK_SMMU_FIMC2: c_int = 265;
pub const CLK_SMMU_FIMC3: c_int = 266;
pub const CLK_SMMU_JPEG: c_int = 267;
pub const CLK_VP: c_int = 268;
pub const CLK_MIXER: c_int = 269;

pub const CLK_HDMI: c_int = 271;
pub const CLK_SMMU_TV: c_int = 272;
pub const CLK_MFC: c_int = 273;
pub const CLK_SMMU_MFCL: c_int = 274;
pub const CLK_SMMU_MFCR: c_int = 275;
pub const CLK_G3D: c_int = 276;
pub const CLK_G2D: c_int = 277;
pub const CLK_ROTATOR: c_int = 278;
pub const CLK_MDMA: c_int = 279;
pub const CLK_SMMU_G2D: c_int = 280;
pub const CLK_SMMU_ROTATOR: c_int = 281;
pub const CLK_SMMU_MDMA: c_int = 282;
pub const CLK_FIMD0: c_int = 283;
pub const CLK_MIE0: c_int = 284;

pub const CLK_DSIM0: c_int = 286;
pub const CLK_SMMU_FIMD0: c_int = 287;

pub const CLK_PDMA0: c_int = 292;
pub const CLK_PDMA1: c_int = 293;
pub const CLK_PCIE_PHY: c_int = 294;

pub const CLK_TSI: c_int = 296;
pub const CLK_SDMMC0: c_int = 297;
pub const CLK_SDMMC1: c_int = 298;
pub const CLK_SDMMC2: c_int = 299;
pub const CLK_SDMMC3: c_int = 300;
pub const CLK_SDMMC4: c_int = 301;

pub const CLK_SROMC: c_int = 303;
pub const CLK_USB_HOST: c_int = 304;
pub const CLK_USB_DEVICE: c_int = 305;
pub const CLK_PCIE: c_int = 306;
pub const CLK_ONENAND: c_int = 307;
pub const CLK_NFCON: c_int = 308;
pub const CLK_SMMU_PCIE: c_int = 309;
pub const CLK_GPS: c_int = 310;
pub const CLK_SMMU_GPS: c_int = 311;
pub const CLK_UART0: c_int = 312;
pub const CLK_UART1: c_int = 313;
pub const CLK_UART2: c_int = 314;
pub const CLK_UART3: c_int = 315;
pub const CLK_UART4: c_int = 316;
pub const CLK_I2C0: c_int = 317;
pub const CLK_I2C1: c_int = 318;
pub const CLK_I2C2: c_int = 319;
pub const CLK_I2C3: c_int = 320;
pub const CLK_I2C4: c_int = 321;
pub const CLK_I2C5: c_int = 322;
pub const CLK_I2C6: c_int = 323;
pub const CLK_I2C7: c_int = 324;
pub const CLK_I2C_HDMI: c_int = 325;
pub const CLK_TSADC: c_int = 326;
pub const CLK_SPI0: c_int = 327;
pub const CLK_SPI1: c_int = 328;
pub const CLK_SPI2: c_int = 329;
pub const CLK_I2S1: c_int = 330;
pub const CLK_I2S2: c_int = 331;
pub const CLK_PCM0: c_int = 332;
pub const CLK_I2S0: c_int = 333;
pub const CLK_PCM1: c_int = 334;
pub const CLK_PCM2: c_int = 335;
pub const CLK_PWM: c_int = 336;
pub const CLK_SLIMBUS: c_int = 337;
pub const CLK_SPDIF: c_int = 338;
pub const CLK_AC97: c_int = 339;
pub const CLK_MODEMIF: c_int = 340;
pub const CLK_CHIPID: c_int = 341;
pub const CLK_SYSREG: c_int = 342;
pub const CLK_HDMI_CEC: c_int = 343;
pub const CLK_MCT: c_int = 344;
pub const CLK_WDT: c_int = 345;
pub const CLK_RTC: c_int = 346;
pub const CLK_KEYIF: c_int = 347;
pub const CLK_AUDSS: c_int = 348;

pub const CLK_PIXELASYNCM0: c_int = 351;
pub const CLK_PIXELASYNCM1: c_int = 352;

pub const CLK_TMU_APBIF: c_int = 383;
// mux clocks
pub const CLK_MOUT_FIMC0: c_int = 384;
pub const CLK_MOUT_FIMC1: c_int = 385;
pub const CLK_MOUT_FIMC2: c_int = 386;
pub const CLK_MOUT_FIMC3: c_int = 387;
pub const CLK_MOUT_CAM0: c_int = 388;
pub const CLK_MOUT_CAM1: c_int = 389;
pub const CLK_MOUT_CSIS0: c_int = 390;
pub const CLK_MOUT_CSIS1: c_int = 391;
pub const CLK_MOUT_G3D0: c_int = 392;
pub const CLK_MOUT_G3D1: c_int = 393;
pub const CLK_MOUT_G3D: c_int = 394;

pub const CLK_MOUT_HDMI: c_int = 396;
pub const CLK_MOUT_MIXER: c_int = 397;
pub const CLK_MOUT_VPLLSRC: c_int = 398;
// gate clocks - ppmu
pub const CLK_PPMULEFT: c_int = 400;
pub const CLK_PPMURIGHT: c_int = 401;
pub const CLK_PPMUCAMIF: c_int = 402;
pub const CLK_PPMUTV: c_int = 403;
pub const CLK_PPMUMFC_L: c_int = 404;
pub const CLK_PPMUMFC_R: c_int = 405;
pub const CLK_PPMUG3D: c_int = 406;
pub const CLK_PPMUIMAGE: c_int = 407;
pub const CLK_PPMULCD0: c_int = 408;

pub const CLK_PPMUFILE: c_int = 410;
pub const CLK_PPMUGPS: c_int = 411;
pub const CLK_PPMUDMC0: c_int = 412;
pub const CLK_PPMUDMC1: c_int = 413;
pub const CLK_PPMUCPU: c_int = 414;
pub const CLK_PPMUACP: c_int = 415;
// div clocks

pub const CLK_DIV_ACP: c_int = 456;
pub const CLK_DIV_DMC: c_int = 457;

pub const CLK_DIV_GDL: c_int = 459;
pub const CLK_DIV_GDR: c_int = 460;
pub const CLK_DIV_CORE2: c_int = 461;
// Exynos4x12 ISP clocks
pub const CLK_ISP_FIMC_ISP: c_int = 1;
pub const CLK_ISP_FIMC_DRC: c_int = 2;
pub const CLK_ISP_FIMC_FD: c_int = 3;
pub const CLK_ISP_FIMC_LITE0: c_int = 4;
pub const CLK_ISP_FIMC_LITE1: c_int = 5;
pub const CLK_ISP_MCUISP: c_int = 6;
pub const CLK_ISP_GICISP: c_int = 7;
pub const CLK_ISP_SMMU_ISP: c_int = 8;
pub const CLK_ISP_SMMU_DRC: c_int = 9;
pub const CLK_ISP_SMMU_FD: c_int = 10;
pub const CLK_ISP_SMMU_LITE0: c_int = 11;
pub const CLK_ISP_SMMU_LITE1: c_int = 12;
pub const CLK_ISP_PPMUISPMX: c_int = 13;
pub const CLK_ISP_PPMUISPX: c_int = 14;
pub const CLK_ISP_MCUCTL_ISP: c_int = 15;
pub const CLK_ISP_MPWM_ISP: c_int = 16;
pub const CLK_ISP_I2C0_ISP: c_int = 17;
pub const CLK_ISP_I2C1_ISP: c_int = 18;
pub const CLK_ISP_MTCADC_ISP: c_int = 19;
pub const CLK_ISP_PWM_ISP: c_int = 20;
pub const CLK_ISP_WDT_ISP: c_int = 21;
pub const CLK_ISP_UART_ISP: c_int = 22;
pub const CLK_ISP_ASYNCAXIM: c_int = 23;
pub const CLK_ISP_SMMU_ISPCX: c_int = 24;
pub const CLK_ISP_SPI0_ISP: c_int = 25;
pub const CLK_ISP_SPI1_ISP: c_int = 26;
pub const CLK_ISP_DIV_ISP0: c_int = 27;
pub const CLK_ISP_DIV_ISP1: c_int = 28;
pub const CLK_ISP_DIV_MCUISP0: c_int = 29;
pub const CLK_ISP_DIV_MCUISP1: c_int = 30;
