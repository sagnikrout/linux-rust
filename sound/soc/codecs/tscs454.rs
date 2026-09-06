//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tscs454.h
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
// tscs454.h -- TSCS454 ALSA SoC Audio driver
// Copyright 2018 Tempo Semiconductor, Inc.
// Author: Steven Eckhoff <steven.eckhoff.opensource@gmail.com>
pub const VIRT_BASE: c_uint = 0x00;
pub const PAGE_LEN: c_uint = 0x100;

pub const R_PAGESEL: c_uint = 0x0;

// *** PLLCTL
pub const FB_PLLCTL_VCCI_PLL: c_int = 6;
pub const FM_PLLCTL_VCCI_PLL: c_uint = 0xC0;
pub const FB_PLLCTL_RZ_PLL: c_int = 3;
pub const FM_PLLCTL_RZ_PLL: c_uint = 0x38;
pub const FB_PLLCTL_CP_PLL: c_int = 0;
pub const FM_PLLCTL_CP_PLL: c_uint = 0x7;
// *** PLLRDIV
pub const FB_PLLRDIV_REFDIV_PLL: c_int = 0;
pub const FM_PLLRDIV_REFDIV_PLL: c_uint = 0xFF;
// *** PLLODIV
pub const FB_PLLODIV_OUTDIV_PLL: c_int = 0;
pub const FM_PLLODIV_OUTDIV_PLL: c_uint = 0xFF;
// *** PLLFDIVL
pub const FB_PLLFDIVL_FBDIVL_PLL: c_int = 0;
pub const FM_PLLFDIVL_FBDIVL_PLL: c_uint = 0xFF;
// *** PLLFDIVH
pub const FB_PLLFDIVH_FBDIVH_PLL: c_int = 0;
pub const FM_PLLFDIVH_FBDIVH_PLL: c_uint = 0xF;
// *** I2SPCTL
pub const FB_I2SPCTL_BCLKSTAT: c_int = 7;
pub const FM_I2SPCTL_BCLKSTAT: c_uint = 0x80;
pub const FV_BCLKSTAT_LOST: c_uint = 0x80;
pub const FV_BCLKSTAT_NOT_LOST: c_uint = 0x0;
pub const FB_I2SPCTL_BCLKP: c_int = 6;
pub const FM_I2SPCTL_BCLKP: c_uint = 0x40;
pub const FV_BCLKP_NOT_INVERTED: c_uint = 0x0;
pub const FV_BCLKP_INVERTED: c_uint = 0x40;
pub const FB_I2SPCTL_PORTMS: c_int = 5;
pub const FM_I2SPCTL_PORTMS: c_uint = 0x20;
pub const FV_PORTMS_SLAVE: c_uint = 0x0;
pub const FV_PORTMS_MASTER: c_uint = 0x20;
pub const FB_I2SPCTL_LRCLKP: c_int = 4;
pub const FM_I2SPCTL_LRCLKP: c_uint = 0x10;
pub const FV_LRCLKP_NOT_INVERTED: c_uint = 0x0;
pub const FV_LRCLKP_INVERTED: c_uint = 0x10;
pub const FB_I2SPCTL_WL: c_int = 2;
pub const FM_I2SPCTL_WL: c_uint = 0xC;
pub const FV_WL_16: c_uint = 0x0;
pub const FV_WL_20: c_uint = 0x4;
pub const FV_WL_24: c_uint = 0x8;
pub const FV_WL_32: c_uint = 0xC;
pub const FB_I2SPCTL_FORMAT: c_int = 0;
pub const FM_I2SPCTL_FORMAT: c_uint = 0x3;
pub const FV_FORMAT_RIGHT: c_uint = 0x0;
pub const FV_FORMAT_LEFT: c_uint = 0x1;
pub const FV_FORMAT_I2S: c_uint = 0x2;
pub const FV_FORMAT_TDM: c_uint = 0x3;
// *** I2SMRATE
pub const FB_I2SMRATE_I2SMCLKHALF: c_int = 7;
pub const FM_I2SMRATE_I2SMCLKHALF: c_uint = 0x80;
pub const FV_I2SMCLKHALF_I2S1MCLKDIV_DIV_2: c_uint = 0x0;
pub const FV_I2SMCLKHALF_I2S1MCLKDIV_ONLY: c_uint = 0x80;
pub const FB_I2SMRATE_I2SMCLKDIV: c_int = 5;
pub const FM_I2SMRATE_I2SMCLKDIV: c_uint = 0x60;
pub const FV_I2SMCLKDIV_125: c_uint = 0x0;
pub const FV_I2SMCLKDIV_128: c_uint = 0x20;
pub const FV_I2SMCLKDIV_136: c_uint = 0x40;
pub const FV_I2SMCLKDIV_192: c_uint = 0x60;
pub const FB_I2SMRATE_I2SMBR: c_int = 3;
pub const FM_I2SMRATE_I2SMBR: c_uint = 0x18;
pub const FV_I2SMBR_32: c_uint = 0x0;
pub const FV_I2SMBR_44PT1: c_uint = 0x8;
pub const FV_I2SMBR_48: c_uint = 0x10;
pub const FV_I2SMBR_MCLK_MODE: c_uint = 0x18;
pub const FB_I2SMRATE_I2SMBM: c_int = 0;
pub const FM_I2SMRATE_I2SMBM: c_uint = 0x3;
pub const FV_I2SMBM_0PT25: c_uint = 0x0;
pub const FV_I2SMBM_0PT5: c_uint = 0x1;
pub const FV_I2SMBM_1: c_uint = 0x2;
pub const FV_I2SMBM_2: c_uint = 0x3;
// *** PCMPCTL0
pub const FB_PCMPCTL0_PCMFLENP: c_int = 2;
pub const FM_PCMPCTL0_PCMFLENP: c_uint = 0x4;
pub const FV_PCMFLENP_128: c_uint = 0x0;
pub const FV_PCMFLENP_256: c_uint = 0x4;
pub const FB_PCMPCTL0_SLSYNCP: c_int = 1;
pub const FM_PCMPCTL0_SLSYNCP: c_uint = 0x2;
pub const FV_SLSYNCP_SHORT: c_uint = 0x0;
pub const FV_SLSYNCP_LONG: c_uint = 0x2;
pub const FB_PCMPCTL0_BDELAYP: c_int = 0;
pub const FM_PCMPCTL0_BDELAYP: c_uint = 0x1;
pub const FV_BDELAYP_NO_DELAY: c_uint = 0x0;
pub const FV_BDELAYP_1BCLK_DELAY: c_uint = 0x1;
// *** PCMPCTL1
pub const FB_PCMPCTL1_PCMMOMP: c_int = 6;
pub const FM_PCMPCTL1_PCMMOMP: c_uint = 0x40;
pub const FB_PCMPCTL1_PCMSOP: c_int = 5;
pub const FM_PCMPCTL1_PCMSOP: c_uint = 0x20;
pub const FV_PCMSOP_1: c_uint = 0x0;
pub const FV_PCMSOP_2: c_uint = 0x20;
pub const FB_PCMPCTL1_PCMDSSP: c_int = 3;
pub const FM_PCMPCTL1_PCMDSSP: c_uint = 0x18;
pub const FV_PCMDSSP_16: c_uint = 0x0;
pub const FV_PCMDSSP_24: c_uint = 0x8;
pub const FV_PCMDSSP_32: c_uint = 0x10;
pub const FB_PCMPCTL1_PCMMIMP: c_int = 1;
pub const FM_PCMPCTL1_PCMMIMP: c_uint = 0x2;
pub const FB_PCMPCTL1_PCMSIP: c_int = 0;
pub const FM_PCMPCTL1_PCMSIP: c_uint = 0x1;
pub const FV_PCMSIP_1: c_uint = 0x0;
pub const FV_PCMSIP_2: c_uint = 0x1;
// *** CHAIC
pub const FB_CHAIC_MICBST: c_int = 4;
pub const FM_CHAIC_MICBST: c_uint = 0x30;
// *** PGACTL
pub const FB_PGACTL_PGAMUTE: c_int = 7;
pub const FM_PGACTL_PGAMUTE: c_uint = 0x80;
pub const FB_PGACTL_PGAVOL: c_int = 0;
pub const FM_PGACTL_PGAVOL: c_uint = 0x3F;
// *** ICHVOL
pub const FB_ICHVOL_ICHVOL: c_int = 0;
pub const FM_ICHVOL_ICHVOL: c_uint = 0xFF;
// *** SPKMBCMUG
pub const FB_SPKMBCMUG_PHASE: c_int = 5;
pub const FM_SPKMBCMUG_PHASE: c_uint = 0x20;
pub const FB_SPKMBCMUG_MUGAIN: c_int = 0;
pub const FM_SPKMBCMUG_MUGAIN: c_uint = 0x1F;
// *** SPKMBCTHR
pub const FB_SPKMBCTHR_THRESH: c_int = 0;
pub const FM_SPKMBCTHR_THRESH: c_uint = 0xFF;
// *** SPKMBCRAT
pub const FB_SPKMBCRAT_RATIO: c_int = 0;
pub const FM_SPKMBCRAT_RATIO: c_uint = 0x1F;
// *** SPKMBCATKL
pub const FB_SPKMBCATKL_TCATKL: c_int = 0;
pub const FM_SPKMBCATKL_TCATKL: c_uint = 0xFF;
// *** SPKMBCATKH
pub const FB_SPKMBCATKH_TCATKH: c_int = 0;
pub const FM_SPKMBCATKH_TCATKH: c_uint = 0xFF;
// *** SPKMBCRELL
pub const FB_SPKMBCRELL_TCRELL: c_int = 0;
pub const FM_SPKMBCRELL_TCRELL: c_uint = 0xFF;
// *** SPKMBCRELH
pub const FB_SPKMBCRELH_TCRELH: c_int = 0;
pub const FM_SPKMBCRELH_TCRELH: c_uint = 0xFF;
// *** DACMBCMUG
pub const FB_DACMBCMUG_PHASE: c_int = 5;
pub const FM_DACMBCMUG_PHASE: c_uint = 0x20;
pub const FB_DACMBCMUG_MUGAIN: c_int = 0;
pub const FM_DACMBCMUG_MUGAIN: c_uint = 0x1F;
// *** DACMBCTHR
pub const FB_DACMBCTHR_THRESH: c_int = 0;
pub const FM_DACMBCTHR_THRESH: c_uint = 0xFF;
// *** DACMBCRAT
pub const FB_DACMBCRAT_RATIO: c_int = 0;
pub const FM_DACMBCRAT_RATIO: c_uint = 0x1F;
// *** DACMBCATKL
pub const FB_DACMBCATKL_TCATKL: c_int = 0;
pub const FM_DACMBCATKL_TCATKL: c_uint = 0xFF;
// *** DACMBCATKH
pub const FB_DACMBCATKH_TCATKH: c_int = 0;
pub const FM_DACMBCATKH_TCATKH: c_uint = 0xFF;
// *** DACMBCRELL
pub const FB_DACMBCRELL_TCRELL: c_int = 0;
pub const FM_DACMBCRELL_TCRELL: c_uint = 0xFF;
// *** DACMBCRELH
pub const FB_DACMBCRELH_TCRELH: c_int = 0;
pub const FM_DACMBCRELH_TCRELH: c_uint = 0xFF;
// *** SUBMBCMUG
pub const FB_SUBMBCMUG_PHASE: c_int = 5;
pub const FM_SUBMBCMUG_PHASE: c_uint = 0x20;
pub const FB_SUBMBCMUG_MUGAIN: c_int = 0;
pub const FM_SUBMBCMUG_MUGAIN: c_uint = 0x1F;
// *** SUBMBCTHR
pub const FB_SUBMBCTHR_THRESH: c_int = 0;
pub const FM_SUBMBCTHR_THRESH: c_uint = 0xFF;
// *** SUBMBCRAT
pub const FB_SUBMBCRAT_RATIO: c_int = 0;
pub const FM_SUBMBCRAT_RATIO: c_uint = 0x1F;
// *** SUBMBCATKL
pub const FB_SUBMBCATKL_TCATKL: c_int = 0;
pub const FM_SUBMBCATKL_TCATKL: c_uint = 0xFF;
// *** SUBMBCATKH
pub const FB_SUBMBCATKH_TCATKH: c_int = 0;
pub const FM_SUBMBCATKH_TCATKH: c_uint = 0xFF;
// *** SUBMBCRELL
pub const FB_SUBMBCRELL_TCRELL: c_int = 0;
pub const FM_SUBMBCRELL_TCRELL: c_uint = 0xFF;
// *** SUBMBCRELH
pub const FB_SUBMBCRELH_TCRELH: c_int = 0;
pub const FM_SUBMBCRELH_TCRELH: c_uint = 0xFF;
// *** PAGESEL
pub const FB_PAGESEL_PAGESEL: c_int = 0;
pub const FM_PAGESEL_PAGESEL: c_uint = 0xFF;
// *** RESET
pub const FB_RESET_RESET: c_int = 0;
pub const FM_RESET_RESET: c_uint = 0xFF;
pub const FV_RESET_PWR_ON_DEFAULTS: c_uint = 0x85;
// *** IRQEN
pub const FB_IRQEN_THRMINTEN: c_int = 6;
pub const FM_IRQEN_THRMINTEN: c_uint = 0x40;
pub const FV_THRMINTEN_ENABLED: c_uint = 0x40;
pub const FV_THRMINTEN_DISABLED: c_uint = 0x0;
pub const FB_IRQEN_HBPINTEN: c_int = 5;
pub const FM_IRQEN_HBPINTEN: c_uint = 0x20;
pub const FV_HBPINTEN_ENABLED: c_uint = 0x20;
pub const FV_HBPINTEN_DISABLED: c_uint = 0x0;
pub const FB_IRQEN_HSDINTEN: c_int = 4;
pub const FM_IRQEN_HSDINTEN: c_uint = 0x10;
pub const FV_HSDINTEN_ENABLED: c_uint = 0x10;
pub const FV_HSDINTEN_DISABLED: c_uint = 0x0;
pub const FB_IRQEN_HPDINTEN: c_int = 3;
pub const FM_IRQEN_HPDINTEN: c_uint = 0x8;
pub const FV_HPDINTEN_ENABLED: c_uint = 0x8;
pub const FV_HPDINTEN_DISABLED: c_uint = 0x0;
pub const FB_IRQEN_GPIO3INTEN: c_int = 1;
pub const FM_IRQEN_GPIO3INTEN: c_uint = 0x2;
pub const FV_GPIO3INTEN_ENABLED: c_uint = 0x2;
pub const FV_GPIO3INTEN_DISABLED: c_uint = 0x0;
pub const FB_IRQEN_GPIO2INTEN: c_int = 0;
pub const FM_IRQEN_GPIO2INTEN: c_uint = 0x1;
pub const FV_GPIO2INTEN_ENABLED: c_uint = 0x1;
pub const FV_GPIO2INTEN_DISABLED: c_uint = 0x0;
pub const IRQEN_GPIOINTEN_ENABLED: c_uint = 0x1;
pub const IRQEN_GPIOINTEN_DISABLED: c_uint = 0x0;
// *** IRQMASK
pub const FB_IRQMASK_THRMIM: c_int = 6;
pub const FM_IRQMASK_THRMIM: c_uint = 0x40;
pub const FV_THRMIM_MASKED: c_uint = 0x0;
pub const FV_THRMIM_NOT_MASKED: c_uint = 0x40;
pub const FB_IRQMASK_HBPIM: c_int = 5;
pub const FM_IRQMASK_HBPIM: c_uint = 0x20;
pub const FV_HBPIM_MASKED: c_uint = 0x0;
pub const FV_HBPIM_NOT_MASKED: c_uint = 0x20;
pub const FB_IRQMASK_HSDIM: c_int = 4;
pub const FM_IRQMASK_HSDIM: c_uint = 0x10;
pub const FV_HSDIM_MASKED: c_uint = 0x0;
pub const FV_HSDIM_NOT_MASKED: c_uint = 0x10;
pub const FB_IRQMASK_HPDIM: c_int = 3;
pub const FM_IRQMASK_HPDIM: c_uint = 0x8;
pub const FV_HPDIM_MASKED: c_uint = 0x0;
pub const FV_HPDIM_NOT_MASKED: c_uint = 0x8;
pub const FB_IRQMASK_GPIO3M: c_int = 1;
pub const FM_IRQMASK_GPIO3M: c_uint = 0x2;
pub const FV_GPIO3M_MASKED: c_uint = 0x0;
pub const FV_GPIO3M_NOT_MASKED: c_uint = 0x2;
pub const FB_IRQMASK_GPIO2M: c_int = 0;
pub const FM_IRQMASK_GPIO2M: c_uint = 0x1;
pub const FV_GPIO2M_MASKED: c_uint = 0x0;
pub const FV_GPIO2M_NOT_MASKED: c_uint = 0x1;
pub const IRQMASK_GPIOM_MASKED: c_uint = 0x0;
pub const IRQMASK_GPIOM_NOT_MASKED: c_uint = 0x1;
// *** IRQSTAT
pub const FB_IRQSTAT_THRMINT: c_int = 6;
pub const FM_IRQSTAT_THRMINT: c_uint = 0x40;
pub const FV_THRMINT_INTERRUPTED: c_uint = 0x40;
pub const FV_THRMINT_NOT_INTERRUPTED: c_uint = 0x0;
pub const FB_IRQSTAT_HBPINT: c_int = 5;
pub const FM_IRQSTAT_HBPINT: c_uint = 0x20;
pub const FV_HBPINT_INTERRUPTED: c_uint = 0x20;
pub const FV_HBPINT_NOT_INTERRUPTED: c_uint = 0x0;
pub const FB_IRQSTAT_HSDINT: c_int = 4;
pub const FM_IRQSTAT_HSDINT: c_uint = 0x10;
pub const FV_HSDINT_INTERRUPTED: c_uint = 0x10;
pub const FV_HSDINT_NOT_INTERRUPTED: c_uint = 0x0;
pub const FB_IRQSTAT_HPDINT: c_int = 3;
pub const FM_IRQSTAT_HPDINT: c_uint = 0x8;
pub const FV_HPDINT_INTERRUPTED: c_uint = 0x8;
pub const FV_HPDINT_NOT_INTERRUPTED: c_uint = 0x0;
pub const FB_IRQSTAT_GPIO3INT: c_int = 1;
pub const FM_IRQSTAT_GPIO3INT: c_uint = 0x2;
pub const FV_GPIO3INT_INTERRUPTED: c_uint = 0x2;
pub const FV_GPIO3INT_NOT_INTERRUPTED: c_uint = 0x0;
pub const FB_IRQSTAT_GPIO2INT: c_int = 0;
pub const FM_IRQSTAT_GPIO2INT: c_uint = 0x1;
pub const FV_GPIO2INT_INTERRUPTED: c_uint = 0x1;
pub const FV_GPIO2INT_NOT_INTERRUPTED: c_uint = 0x0;
pub const IRQSTAT_GPIOINT_INTERRUPTED: c_uint = 0x1;
pub const IRQSTAT_GPIOINT_NOT_INTERRUPTED: c_uint = 0x0;
// *** DEVADD0
pub const FB_DEVADD0_DEVADD0: c_int = 1;
pub const FM_DEVADD0_DEVADD0: c_uint = 0xFE;
pub const FB_DEVADD0_I2C_ADDRLK: c_int = 0;
pub const FM_DEVADD0_I2C_ADDRLK: c_uint = 0x1;
pub const FV_I2C_ADDRLK_LOCK: c_uint = 0x1;
// *** DEVID
pub const FB_DEVID_DEV_ID: c_int = 0;
pub const FM_DEVID_DEV_ID: c_uint = 0xFF;
// *** DEVREV
pub const FB_DEVREV_MAJ_REV: c_int = 4;
pub const FM_DEVREV_MAJ_REV: c_uint = 0xF0;
pub const FB_DEVREV_MIN_REV: c_int = 0;
pub const FM_DEVREV_MIN_REV: c_uint = 0xF;
// *** PLLSTAT
pub const FB_PLLSTAT_PLL2LK: c_int = 1;
pub const FM_PLLSTAT_PLL2LK: c_uint = 0x2;
pub const FV_PLL2LK_LOCKED: c_uint = 0x2;
pub const FV_PLL2LK_UNLOCKED: c_uint = 0x0;
pub const FB_PLLSTAT_PLL1LK: c_int = 0;
pub const FM_PLLSTAT_PLL1LK: c_uint = 0x1;
pub const FV_PLL1LK_LOCKED: c_uint = 0x1;
pub const FV_PLL1LK_UNLOCKED: c_uint = 0x0;
pub const PLLSTAT_PLLLK_LOCKED: c_uint = 0x1;
pub const PLLSTAT_PLLLK_UNLOCKED: c_uint = 0x0;
// *** PLLCTL
pub const FB_PLLCTL_PU_PLL2: c_int = 7;
pub const FM_PLLCTL_PU_PLL2: c_uint = 0x80;
pub const FV_PU_PLL2_PWR_UP: c_uint = 0x80;
pub const FV_PU_PLL2_PWR_DWN: c_uint = 0x0;
pub const FB_PLLCTL_PU_PLL1: c_int = 6;
pub const FM_PLLCTL_PU_PLL1: c_uint = 0x40;
pub const FV_PU_PLL1_PWR_UP: c_uint = 0x40;
pub const FV_PU_PLL1_PWR_DWN: c_uint = 0x0;
pub const FB_PLLCTL_PLL2CLKEN: c_int = 5;
pub const FM_PLLCTL_PLL2CLKEN: c_uint = 0x20;
pub const FV_PLL2CLKEN_ENABLE: c_uint = 0x20;
pub const FV_PLL2CLKEN_DISABLE: c_uint = 0x0;
pub const FB_PLLCTL_PLL1CLKEN: c_int = 4;
pub const FM_PLLCTL_PLL1CLKEN: c_uint = 0x10;
pub const FV_PLL1CLKEN_ENABLE: c_uint = 0x10;
pub const FV_PLL1CLKEN_DISABLE: c_uint = 0x0;
pub const FB_PLLCTL_BCLKSEL: c_int = 2;
pub const FM_PLLCTL_BCLKSEL: c_uint = 0xC;
pub const FV_BCLKSEL_BCLK1: c_uint = 0x0;
pub const FV_BCLKSEL_BCLK2: c_uint = 0x4;
pub const FV_BCLKSEL_BCLK3: c_uint = 0x8;
pub const FB_PLLCTL_PLLISEL: c_int = 0;
pub const FM_PLLCTL_PLLISEL: c_uint = 0x3;
pub const FV_PLLISEL_XTAL: c_uint = 0x0;
pub const FV_PLLISEL_MCLK1: c_uint = 0x1;
pub const FV_PLLISEL_MCLK2: c_uint = 0x2;
pub const FV_PLLISEL_BCLK: c_uint = 0x3;
pub const PLLCTL_PU_PLL_PWR_UP: c_uint = 0x1;
pub const PLLCTL_PU_PLL_PWR_DWN: c_uint = 0x0;
pub const PLLCTL_PLLCLKEN_ENABLE: c_uint = 0x1;
pub const PLLCTL_PLLCLKEN_DISABLE: c_uint = 0x0;
// *** ISRC
pub const FB_ISRC_IBR: c_int = 2;
pub const FM_ISRC_IBR: c_uint = 0x4;
pub const FV_IBR_44PT1: c_uint = 0x0;
pub const FV_IBR_48: c_uint = 0x4;
pub const FB_ISRC_IBM: c_int = 0;
pub const FM_ISRC_IBM: c_uint = 0x3;
pub const FV_IBM_0PT25: c_uint = 0x0;
pub const FV_IBM_0PT5: c_uint = 0x1;
pub const FV_IBM_1: c_uint = 0x2;
pub const FV_IBM_2: c_uint = 0x3;
// *** SCLKCTL
pub const FB_SCLKCTL_ASDM: c_int = 6;
pub const FM_SCLKCTL_ASDM: c_uint = 0xC0;
pub const FV_ASDM_HALF: c_uint = 0x40;
pub const FV_ASDM_FULL: c_uint = 0x80;
pub const FV_ASDM_AUTO: c_uint = 0xC0;
pub const FB_SCLKCTL_DSDM: c_int = 4;
pub const FM_SCLKCTL_DSDM: c_uint = 0x30;
pub const FV_DSDM_HALF: c_uint = 0x10;
pub const FV_DSDM_FULL: c_uint = 0x20;
pub const FV_DSDM_AUTO: c_uint = 0x30;
// *** TIMEBASE
pub const FB_TIMEBASE_TIMEBASE: c_int = 0;
pub const FM_TIMEBASE_TIMEBASE: c_uint = 0xFF;
// *** I2SCMC
pub const FB_I2SCMC_BCMP3: c_int = 4;
pub const FM_I2SCMC_BCMP3: c_uint = 0x30;
pub const FV_BCMP3_AUTO: c_uint = 0x0;
pub const FV_BCMP3_32X: c_uint = 0x10;
pub const FV_BCMP3_40X: c_uint = 0x20;
pub const FV_BCMP3_64X: c_uint = 0x30;
pub const FB_I2SCMC_BCMP2: c_int = 2;
pub const FM_I2SCMC_BCMP2: c_uint = 0xC;
pub const FV_BCMP2_AUTO: c_uint = 0x0;
pub const FV_BCMP2_32X: c_uint = 0x4;
pub const FV_BCMP2_40X: c_uint = 0x8;
pub const FV_BCMP2_64X: c_uint = 0xC;
pub const FB_I2SCMC_BCMP1: c_int = 0;
pub const FM_I2SCMC_BCMP1: c_uint = 0x3;
pub const FV_BCMP1_AUTO: c_uint = 0x0;
pub const FV_BCMP1_32X: c_uint = 0x1;
pub const FV_BCMP1_40X: c_uint = 0x2;
pub const FV_BCMP1_64X: c_uint = 0x3;
pub const I2SCMC_BCMP_AUTO: c_uint = 0x0;
pub const I2SCMC_BCMP_32X: c_uint = 0x1;
pub const I2SCMC_BCMP_40X: c_uint = 0x2;
pub const I2SCMC_BCMP_64X: c_uint = 0x3;
// *** MCLK2PINC
pub const FB_MCLK2PINC_SLEWOUT: c_int = 4;
pub const FM_MCLK2PINC_SLEWOUT: c_uint = 0xF0;
pub const FB_MCLK2PINC_MCLK2IO: c_int = 2;
pub const FM_MCLK2PINC_MCLK2IO: c_uint = 0x4;
pub const FV_MCLK2IO_INPUT: c_uint = 0x0;
pub const FV_MCLK2IO_OUTPUT: c_uint = 0x4;
pub const FB_MCLK2PINC_MCLK2OS: c_int = 0;
pub const FM_MCLK2PINC_MCLK2OS: c_uint = 0x3;
pub const FV_MCLK2OS_24PT576: c_uint = 0x0;
pub const FV_MCLK2OS_22PT5792: c_uint = 0x1;
pub const FV_MCLK2OS_PLL2: c_uint = 0x2;
// *** I2SPINC0
pub const FB_I2SPINC0_SDO3TRI: c_int = 7;
pub const FM_I2SPINC0_SDO3TRI: c_uint = 0x80;
pub const FB_I2SPINC0_SDO2TRI: c_int = 6;
pub const FM_I2SPINC0_SDO2TRI: c_uint = 0x40;
pub const FB_I2SPINC0_SDO1TRI: c_int = 5;
pub const FM_I2SPINC0_SDO1TRI: c_uint = 0x20;
pub const FB_I2SPINC0_PCM3TRI: c_int = 2;
pub const FM_I2SPINC0_PCM3TRI: c_uint = 0x4;
pub const FB_I2SPINC0_PCM2TRI: c_int = 1;
pub const FM_I2SPINC0_PCM2TRI: c_uint = 0x2;
pub const FB_I2SPINC0_PCM1TRI: c_int = 0;
pub const FM_I2SPINC0_PCM1TRI: c_uint = 0x1;
// *** I2SPINC1
pub const FB_I2SPINC1_SDO3PDD: c_int = 2;
pub const FM_I2SPINC1_SDO3PDD: c_uint = 0x4;
pub const FB_I2SPINC1_SDO2PDD: c_int = 1;
pub const FM_I2SPINC1_SDO2PDD: c_uint = 0x2;
pub const FB_I2SPINC1_SDO1PDD: c_int = 0;
pub const FM_I2SPINC1_SDO1PDD: c_uint = 0x1;
// *** I2SPINC2
pub const FB_I2SPINC2_LR3PDD: c_int = 5;
pub const FM_I2SPINC2_LR3PDD: c_uint = 0x20;
pub const FB_I2SPINC2_BC3PDD: c_int = 4;
pub const FM_I2SPINC2_BC3PDD: c_uint = 0x10;
pub const FB_I2SPINC2_LR2PDD: c_int = 3;
pub const FM_I2SPINC2_LR2PDD: c_uint = 0x8;
pub const FB_I2SPINC2_BC2PDD: c_int = 2;
pub const FM_I2SPINC2_BC2PDD: c_uint = 0x4;
pub const FB_I2SPINC2_LR1PDD: c_int = 1;
pub const FM_I2SPINC2_LR1PDD: c_uint = 0x2;
pub const FB_I2SPINC2_BC1PDD: c_int = 0;
pub const FM_I2SPINC2_BC1PDD: c_uint = 0x1;
// *** GPIOCTL0
pub const FB_GPIOCTL0_GPIO3INTP: c_int = 7;
pub const FM_GPIOCTL0_GPIO3INTP: c_uint = 0x80;
pub const FB_GPIOCTL0_GPIO2INTP: c_int = 6;
pub const FM_GPIOCTL0_GPIO2INTP: c_uint = 0x40;
pub const FB_GPIOCTL0_GPIO3CFG: c_int = 5;
pub const FM_GPIOCTL0_GPIO3CFG: c_uint = 0x20;
pub const FB_GPIOCTL0_GPIO2CFG: c_int = 4;
pub const FM_GPIOCTL0_GPIO2CFG: c_uint = 0x10;
pub const FB_GPIOCTL0_GPIO3IO: c_int = 3;
pub const FM_GPIOCTL0_GPIO3IO: c_uint = 0x8;
pub const FB_GPIOCTL0_GPIO2IO: c_int = 2;
pub const FM_GPIOCTL0_GPIO2IO: c_uint = 0x4;
pub const FB_GPIOCTL0_GPIO1IO: c_int = 1;
pub const FM_GPIOCTL0_GPIO1IO: c_uint = 0x2;
pub const FB_GPIOCTL0_GPIO0IO: c_int = 0;
pub const FM_GPIOCTL0_GPIO0IO: c_uint = 0x1;
// *** GPIOCTL1
pub const FB_GPIOCTL1_GPIO3: c_int = 7;
pub const FM_GPIOCTL1_GPIO3: c_uint = 0x80;
pub const FB_GPIOCTL1_GPIO2: c_int = 6;
pub const FM_GPIOCTL1_GPIO2: c_uint = 0x40;
pub const FB_GPIOCTL1_GPIO1: c_int = 5;
pub const FM_GPIOCTL1_GPIO1: c_uint = 0x20;
pub const FB_GPIOCTL1_GPIO0: c_int = 4;
pub const FM_GPIOCTL1_GPIO0: c_uint = 0x10;
pub const FB_GPIOCTL1_GPIO3RD: c_int = 3;
pub const FM_GPIOCTL1_GPIO3RD: c_uint = 0x8;
pub const FB_GPIOCTL1_GPIO2RD: c_int = 2;
pub const FM_GPIOCTL1_GPIO2RD: c_uint = 0x4;
pub const FB_GPIOCTL1_GPIO1RD: c_int = 1;
pub const FM_GPIOCTL1_GPIO1RD: c_uint = 0x2;
pub const FB_GPIOCTL1_GPIO0RD: c_int = 0;
pub const FM_GPIOCTL1_GPIO0RD: c_uint = 0x1;
// *** ASRC
pub const FB_ASRC_ASRCOBW: c_int = 7;
pub const FM_ASRC_ASRCOBW: c_uint = 0x80;
pub const FB_ASRC_ASRCIBW: c_int = 6;
pub const FM_ASRC_ASRCIBW: c_uint = 0x40;
pub const FB_ASRC_ASRCOB: c_int = 5;
pub const FM_ASRC_ASRCOB: c_uint = 0x20;
pub const FV_ASRCOB_ACTIVE: c_uint = 0x0;
pub const FV_ASRCOB_BYPASSED: c_uint = 0x20;
pub const FB_ASRC_ASRCIB: c_int = 4;
pub const FM_ASRC_ASRCIB: c_uint = 0x10;
pub const FV_ASRCIB_ACTIVE: c_uint = 0x0;
pub const FV_ASRCIB_BYPASSED: c_uint = 0x10;
pub const FB_ASRC_ASRCOL: c_int = 3;
pub const FM_ASRC_ASRCOL: c_uint = 0x8;
pub const FB_ASRC_ASRCIL: c_int = 2;
pub const FM_ASRC_ASRCIL: c_uint = 0x4;
// *** TDMCTL0
pub const FB_TDMCTL0_TDMMD: c_int = 2;
pub const FM_TDMCTL0_TDMMD: c_uint = 0x4;
pub const FV_TDMMD_200: c_uint = 0x0;
pub const FV_TDMMD_256: c_uint = 0x4;
pub const FB_TDMCTL0_SLSYNC: c_int = 1;
pub const FM_TDMCTL0_SLSYNC: c_uint = 0x2;
pub const FV_SLSYNC_SHORT: c_uint = 0x0;
pub const FV_SLSYNC_LONG: c_uint = 0x2;
pub const FB_TDMCTL0_BDELAY: c_int = 0;
pub const FM_TDMCTL0_BDELAY: c_uint = 0x1;
pub const FV_BDELAY_NO_DELAY: c_uint = 0x0;
pub const FV_BDELAY_1BCLK_DELAY: c_uint = 0x1;
// *** TDMCTL1
pub const FB_TDMCTL1_TDMSO: c_int = 5;
pub const FM_TDMCTL1_TDMSO: c_uint = 0x60;
pub const FV_TDMSO_2: c_uint = 0x0;
pub const FV_TDMSO_4: c_uint = 0x20;
pub const FV_TDMSO_6: c_uint = 0x40;
pub const FB_TDMCTL1_TDMDSS: c_int = 3;
pub const FM_TDMCTL1_TDMDSS: c_uint = 0x18;
pub const FV_TDMDSS_16: c_uint = 0x0;
pub const FV_TDMDSS_24: c_uint = 0x10;
pub const FV_TDMDSS_32: c_uint = 0x18;
pub const FB_TDMCTL1_TDMSI: c_int = 0;
pub const FM_TDMCTL1_TDMSI: c_uint = 0x3;
pub const FV_TDMSI_2: c_uint = 0x0;
pub const FV_TDMSI_4: c_uint = 0x1;
pub const FV_TDMSI_6: c_uint = 0x2;
// *** PWRM0
pub const FB_PWRM0_INPROC3PU: c_int = 6;
pub const FM_PWRM0_INPROC3PU: c_uint = 0x40;
pub const FB_PWRM0_INPROC2PU: c_int = 5;
pub const FM_PWRM0_INPROC2PU: c_uint = 0x20;
pub const FB_PWRM0_INPROC1PU: c_int = 4;
pub const FM_PWRM0_INPROC1PU: c_uint = 0x10;
pub const FB_PWRM0_INPROC0PU: c_int = 3;
pub const FM_PWRM0_INPROC0PU: c_uint = 0x8;
pub const FB_PWRM0_MICB2PU: c_int = 2;
pub const FM_PWRM0_MICB2PU: c_uint = 0x4;
pub const FB_PWRM0_MICB1PU: c_int = 1;
pub const FM_PWRM0_MICB1PU: c_uint = 0x2;
pub const FB_PWRM0_MCLKPEN: c_int = 0;
pub const FM_PWRM0_MCLKPEN: c_uint = 0x1;
// *** PWRM1
pub const FB_PWRM1_SUBPU: c_int = 7;
pub const FM_PWRM1_SUBPU: c_uint = 0x80;
pub const FB_PWRM1_HPLPU: c_int = 6;
pub const FM_PWRM1_HPLPU: c_uint = 0x40;
pub const FB_PWRM1_HPRPU: c_int = 5;
pub const FM_PWRM1_HPRPU: c_uint = 0x20;
pub const FB_PWRM1_SPKLPU: c_int = 4;
pub const FM_PWRM1_SPKLPU: c_uint = 0x10;
pub const FB_PWRM1_SPKRPU: c_int = 3;
pub const FM_PWRM1_SPKRPU: c_uint = 0x8;
pub const FB_PWRM1_D2S2PU: c_int = 2;
pub const FM_PWRM1_D2S2PU: c_uint = 0x4;
pub const FB_PWRM1_D2S1PU: c_int = 1;
pub const FM_PWRM1_D2S1PU: c_uint = 0x2;
pub const FB_PWRM1_VREFPU: c_int = 0;
pub const FM_PWRM1_VREFPU: c_uint = 0x1;
// *** PWRM2
pub const FB_PWRM2_I2S3OPU: c_int = 5;
pub const FM_PWRM2_I2S3OPU: c_uint = 0x20;
pub const FV_I2S3OPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S3OPU_PWR_UP: c_uint = 0x20;
pub const FB_PWRM2_I2S2OPU: c_int = 4;
pub const FM_PWRM2_I2S2OPU: c_uint = 0x10;
pub const FV_I2S2OPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S2OPU_PWR_UP: c_uint = 0x10;
pub const FB_PWRM2_I2S1OPU: c_int = 3;
pub const FM_PWRM2_I2S1OPU: c_uint = 0x8;
pub const FV_I2S1OPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S1OPU_PWR_UP: c_uint = 0x8;
pub const FB_PWRM2_I2S3IPU: c_int = 2;
pub const FM_PWRM2_I2S3IPU: c_uint = 0x4;
pub const FV_I2S3IPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S3IPU_PWR_UP: c_uint = 0x4;
pub const FB_PWRM2_I2S2IPU: c_int = 1;
pub const FM_PWRM2_I2S2IPU: c_uint = 0x2;
pub const FV_I2S2IPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S2IPU_PWR_UP: c_uint = 0x2;
pub const FB_PWRM2_I2S1IPU: c_int = 0;
pub const FM_PWRM2_I2S1IPU: c_uint = 0x1;
pub const FV_I2S1IPU_PWR_DOWN: c_uint = 0x0;
pub const FV_I2S1IPU_PWR_UP: c_uint = 0x1;
pub const PWRM2_I2SOPU_PWR_DOWN: c_uint = 0x0;
pub const PWRM2_I2SOPU_PWR_UP: c_uint = 0x1;
pub const PWRM2_I2SIPU_PWR_DOWN: c_uint = 0x0;
pub const PWRM2_I2SIPU_PWR_UP: c_uint = 0x1;
// *** PWRM3
pub const FB_PWRM3_BGSBUP: c_int = 6;
pub const FM_PWRM3_BGSBUP: c_uint = 0x40;
pub const FV_BGSBUP_ON: c_uint = 0x0;
pub const FV_BGSBUP_OFF: c_uint = 0x40;
pub const FB_PWRM3_VGBAPU: c_int = 5;
pub const FM_PWRM3_VGBAPU: c_uint = 0x20;
pub const FV_VGBAPU_ON: c_uint = 0x0;
pub const FV_VGBAPU_OFF: c_uint = 0x20;
pub const FB_PWRM3_LLINEPU: c_int = 4;
pub const FM_PWRM3_LLINEPU: c_uint = 0x10;
pub const FB_PWRM3_RLINEPU: c_int = 3;
pub const FM_PWRM3_RLINEPU: c_uint = 0x8;
// *** PWRM4
pub const FB_PWRM4_OPSUBPU: c_int = 4;
pub const FM_PWRM4_OPSUBPU: c_uint = 0x10;
pub const FB_PWRM4_OPDACLPU: c_int = 3;
pub const FM_PWRM4_OPDACLPU: c_uint = 0x8;
pub const FB_PWRM4_OPDACRPU: c_int = 2;
pub const FM_PWRM4_OPDACRPU: c_uint = 0x4;
pub const FB_PWRM4_OPSPKLPU: c_int = 1;
pub const FM_PWRM4_OPSPKLPU: c_uint = 0x2;
pub const FB_PWRM4_OPSPKRPU: c_int = 0;
pub const FM_PWRM4_OPSPKRPU: c_uint = 0x1;
// *** I2SIDCTL
pub const FB_I2SIDCTL_I2SI3DCTL: c_int = 4;
pub const FM_I2SIDCTL_I2SI3DCTL: c_uint = 0x30;
pub const FB_I2SIDCTL_I2SI2DCTL: c_int = 2;
pub const FM_I2SIDCTL_I2SI2DCTL: c_uint = 0xC;
pub const FB_I2SIDCTL_I2SI1DCTL: c_int = 0;
pub const FM_I2SIDCTL_I2SI1DCTL: c_uint = 0x3;
// *** I2SODCTL
pub const FB_I2SODCTL_I2SO3DCTL: c_int = 4;
pub const FM_I2SODCTL_I2SO3DCTL: c_uint = 0x30;
pub const FB_I2SODCTL_I2SO2DCTL: c_int = 2;
pub const FM_I2SODCTL_I2SO2DCTL: c_uint = 0xC;
pub const FB_I2SODCTL_I2SO1DCTL: c_int = 0;
pub const FM_I2SODCTL_I2SO1DCTL: c_uint = 0x3;
// *** AUDIOMUX1
pub const FB_AUDIOMUX1_ASRCIMUX: c_int = 6;
pub const FM_AUDIOMUX1_ASRCIMUX: c_uint = 0xC0;
pub const FV_ASRCIMUX_NONE: c_uint = 0x0;
pub const FV_ASRCIMUX_I2S1: c_uint = 0x40;
pub const FV_ASRCIMUX_I2S2: c_uint = 0x80;
pub const FV_ASRCIMUX_I2S3: c_uint = 0xC0;
pub const FB_AUDIOMUX1_I2S2MUX: c_int = 3;
pub const FM_AUDIOMUX1_I2S2MUX: c_uint = 0x38;
pub const FV_I2S2MUX_I2S1: c_uint = 0x0;
pub const FV_I2S2MUX_I2S2: c_uint = 0x8;
pub const FV_I2S2MUX_I2S3: c_uint = 0x10;
pub const FV_I2S2MUX_ADC_DMIC: c_uint = 0x18;
pub const FV_I2S2MUX_DMIC2: c_uint = 0x20;
pub const FV_I2S2MUX_CLASSD_DSP: c_uint = 0x28;
pub const FV_I2S2MUX_DAC_DSP: c_uint = 0x30;
pub const FV_I2S2MUX_SUB_DSP: c_uint = 0x38;
pub const FB_AUDIOMUX1_I2S1MUX: c_int = 0;
pub const FM_AUDIOMUX1_I2S1MUX: c_uint = 0x7;
pub const FV_I2S1MUX_I2S1: c_uint = 0x0;
pub const FV_I2S1MUX_I2S2: c_uint = 0x1;
pub const FV_I2S1MUX_I2S3: c_uint = 0x2;
pub const FV_I2S1MUX_ADC_DMIC: c_uint = 0x3;
pub const FV_I2S1MUX_DMIC2: c_uint = 0x4;
pub const FV_I2S1MUX_CLASSD_DSP: c_uint = 0x5;
pub const FV_I2S1MUX_DAC_DSP: c_uint = 0x6;
pub const FV_I2S1MUX_SUB_DSP: c_uint = 0x7;
pub const AUDIOMUX1_I2SMUX_I2S1: c_uint = 0x0;
pub const AUDIOMUX1_I2SMUX_I2S2: c_uint = 0x1;
pub const AUDIOMUX1_I2SMUX_I2S3: c_uint = 0x2;
pub const AUDIOMUX1_I2SMUX_ADC_DMIC: c_uint = 0x3;
pub const AUDIOMUX1_I2SMUX_DMIC2: c_uint = 0x4;
pub const AUDIOMUX1_I2SMUX_CLASSD_DSP: c_uint = 0x5;
pub const AUDIOMUX1_I2SMUX_DAC_DSP: c_uint = 0x6;
pub const AUDIOMUX1_I2SMUX_SUB_DSP: c_uint = 0x7;
// *** AUDIOMUX2
pub const FB_AUDIOMUX2_ASRCOMUX: c_int = 6;
pub const FM_AUDIOMUX2_ASRCOMUX: c_uint = 0xC0;
pub const FV_ASRCOMUX_NONE: c_uint = 0x0;
pub const FV_ASRCOMUX_I2S1: c_uint = 0x40;
pub const FV_ASRCOMUX_I2S2: c_uint = 0x80;
pub const FV_ASRCOMUX_I2S3: c_uint = 0xC0;
pub const FB_AUDIOMUX2_DACMUX: c_int = 3;
pub const FM_AUDIOMUX2_DACMUX: c_uint = 0x38;
pub const FV_DACMUX_I2S1: c_uint = 0x0;
pub const FV_DACMUX_I2S2: c_uint = 0x8;
pub const FV_DACMUX_I2S3: c_uint = 0x10;
pub const FV_DACMUX_ADC_DMIC: c_uint = 0x18;
pub const FV_DACMUX_DMIC2: c_uint = 0x20;
pub const FV_DACMUX_CLASSD_DSP: c_uint = 0x28;
pub const FV_DACMUX_DAC_DSP: c_uint = 0x30;
pub const FV_DACMUX_SUB_DSP: c_uint = 0x38;
pub const FB_AUDIOMUX2_I2S3MUX: c_int = 0;
pub const FM_AUDIOMUX2_I2S3MUX: c_uint = 0x7;
pub const FV_I2S3MUX_I2S1: c_uint = 0x0;
pub const FV_I2S3MUX_I2S2: c_uint = 0x1;
pub const FV_I2S3MUX_I2S3: c_uint = 0x2;
pub const FV_I2S3MUX_ADC_DMIC: c_uint = 0x3;
pub const FV_I2S3MUX_DMIC2: c_uint = 0x4;
pub const FV_I2S3MUX_CLASSD_DSP: c_uint = 0x5;
pub const FV_I2S3MUX_DAC_DSP: c_uint = 0x6;
pub const FV_I2S3MUX_SUB_DSP: c_uint = 0x7;
// *** AUDIOMUX3
pub const FB_AUDIOMUX3_SUBMUX: c_int = 3;
pub const FM_AUDIOMUX3_SUBMUX: c_uint = 0xF8;
pub const FV_SUBMUX_I2S1_L: c_uint = 0x0;
pub const FV_SUBMUX_I2S1_R: c_uint = 0x8;
pub const FV_SUBMUX_I2S1_LR: c_uint = 0x10;
pub const FV_SUBMUX_I2S2_L: c_uint = 0x18;
pub const FV_SUBMUX_I2S2_R: c_uint = 0x20;
pub const FV_SUBMUX_I2S2_LR: c_uint = 0x28;
pub const FV_SUBMUX_I2S3_L: c_uint = 0x30;
pub const FV_SUBMUX_I2S3_R: c_uint = 0x38;
pub const FV_SUBMUX_I2S3_LR: c_uint = 0x40;
pub const FV_SUBMUX_ADC_DMIC_L: c_uint = 0x48;
pub const FV_SUBMUX_ADC_DMIC_R: c_uint = 0x50;
pub const FV_SUBMUX_ADC_DMIC_LR: c_uint = 0x58;
pub const FV_SUBMUX_DMIC_L: c_uint = 0x60;
pub const FV_SUBMUX_DMIC_R: c_uint = 0x68;
pub const FV_SUBMUX_DMIC_LR: c_uint = 0x70;
pub const FV_SUBMUX_CLASSD_DSP_L: c_uint = 0x78;
pub const FV_SUBMUX_CLASSD_DSP_R: c_uint = 0x80;
pub const FV_SUBMUX_CLASSD_DSP_LR: c_uint = 0x88;
pub const FB_AUDIOMUX3_CLSSDMUX: c_int = 0;
pub const FM_AUDIOMUX3_CLSSDMUX: c_uint = 0x7;
pub const FV_CLSSDMUX_I2S1: c_uint = 0x0;
pub const FV_CLSSDMUX_I2S2: c_uint = 0x1;
pub const FV_CLSSDMUX_I2S3: c_uint = 0x2;
pub const FV_CLSSDMUX_ADC_DMIC: c_uint = 0x3;
pub const FV_CLSSDMUX_DMIC2: c_uint = 0x4;
pub const FV_CLSSDMUX_CLASSD_DSP: c_uint = 0x5;
pub const FV_CLSSDMUX_DAC_DSP: c_uint = 0x6;
pub const FV_CLSSDMUX_SUB_DSP: c_uint = 0x7;
// *** HSDCTL1
pub const FB_HSDCTL1_HPJKTYPE: c_int = 7;
pub const FM_HSDCTL1_HPJKTYPE: c_uint = 0x80;
pub const FB_HSDCTL1_CON_DET_PWD: c_int = 6;
pub const FM_HSDCTL1_CON_DET_PWD: c_uint = 0x40;
pub const FB_HSDCTL1_DETCYC: c_int = 4;
pub const FM_HSDCTL1_DETCYC: c_uint = 0x30;
pub const FB_HSDCTL1_HPDLYBYP: c_int = 3;
pub const FM_HSDCTL1_HPDLYBYP: c_uint = 0x8;
pub const FB_HSDCTL1_HSDETPOL: c_int = 2;
pub const FM_HSDCTL1_HSDETPOL: c_uint = 0x4;
pub const FB_HSDCTL1_HPID_EN: c_int = 1;
pub const FM_HSDCTL1_HPID_EN: c_uint = 0x2;
pub const FB_HSDCTL1_GBLHS_EN: c_int = 0;
pub const FM_HSDCTL1_GBLHS_EN: c_uint = 0x1;
// *** HSDCTL2
pub const FB_HSDCTL2_FMICBIAS1: c_int = 6;
pub const FM_HSDCTL2_FMICBIAS1: c_uint = 0xC0;
pub const FB_HSDCTL2_MB1MODE: c_int = 5;
pub const FM_HSDCTL2_MB1MODE: c_uint = 0x20;
pub const FV_MB1MODE_AUTO: c_uint = 0x0;
pub const FV_MB1MODE_MANUAL: c_uint = 0x20;
pub const FB_HSDCTL2_FORCETRG: c_int = 4;
pub const FM_HSDCTL2_FORCETRG: c_uint = 0x10;
pub const FB_HSDCTL2_SWMODE: c_int = 3;
pub const FM_HSDCTL2_SWMODE: c_uint = 0x8;
pub const FB_HSDCTL2_GHSHIZ: c_int = 2;
pub const FM_HSDCTL2_GHSHIZ: c_uint = 0x4;
pub const FB_HSDCTL2_FPLUGTYPE: c_int = 0;
pub const FM_HSDCTL2_FPLUGTYPE: c_uint = 0x3;
// *** HSDSTAT
pub const FB_HSDSTAT_MBIAS1DRV: c_int = 5;
pub const FM_HSDSTAT_MBIAS1DRV: c_uint = 0x60;
pub const FB_HSDSTAT_HSDETSTAT: c_int = 3;
pub const FM_HSDSTAT_HSDETSTAT: c_uint = 0x8;
pub const FB_HSDSTAT_PLUGTYPE: c_int = 1;
pub const FM_HSDSTAT_PLUGTYPE: c_uint = 0x6;
pub const FB_HSDSTAT_HSDETDONE: c_int = 0;
pub const FM_HSDSTAT_HSDETDONE: c_uint = 0x1;
// *** HSDDELAY
pub const FB_HSDDELAY_T_STABLE: c_int = 0;
pub const FM_HSDDELAY_T_STABLE: c_uint = 0x7;
// *** BUTCTL
pub const FB_BUTCTL_BPUSHSTAT: c_int = 7;
pub const FM_BUTCTL_BPUSHSTAT: c_uint = 0x80;
pub const FB_BUTCTL_BPUSHDET: c_int = 6;
pub const FM_BUTCTL_BPUSHDET: c_uint = 0x40;
pub const FB_BUTCTL_BPUSHEN: c_int = 5;
pub const FM_BUTCTL_BPUSHEN: c_uint = 0x20;
pub const FB_BUTCTL_BSTABLE_L: c_int = 3;
pub const FM_BUTCTL_BSTABLE_L: c_uint = 0x18;
pub const FB_BUTCTL_BSTABLE_S: c_int = 0;
pub const FM_BUTCTL_BSTABLE_S: c_uint = 0x7;
// *** CH0AIC
pub const FB_CH0AIC_INSELL: c_int = 6;
pub const FM_CH0AIC_INSELL: c_uint = 0xC0;
pub const FB_CH0AIC_MICBST0: c_int = 4;
pub const FM_CH0AIC_MICBST0: c_uint = 0x30;
pub const FB_CH0AIC_LADCIN: c_int = 2;
pub const FM_CH0AIC_LADCIN: c_uint = 0xC;
pub const FB_CH0AIC_IN_BYPS_L_SEL: c_int = 1;
pub const FM_CH0AIC_IN_BYPS_L_SEL: c_uint = 0x2;
pub const FB_CH0AIC_IPCH0S: c_int = 0;
pub const FM_CH0AIC_IPCH0S: c_uint = 0x1;
// *** CH1AIC
pub const FB_CH1AIC_INSELR: c_int = 6;
pub const FM_CH1AIC_INSELR: c_uint = 0xC0;
pub const FB_CH1AIC_MICBST1: c_int = 4;
pub const FM_CH1AIC_MICBST1: c_uint = 0x30;
pub const FB_CH1AIC_RADCIN: c_int = 2;
pub const FM_CH1AIC_RADCIN: c_uint = 0xC;
pub const FB_CH1AIC_IN_BYPS_R_SEL: c_int = 1;
pub const FM_CH1AIC_IN_BYPS_R_SEL: c_uint = 0x2;
pub const FB_CH1AIC_IPCH1S: c_int = 0;
pub const FM_CH1AIC_IPCH1S: c_uint = 0x1;
// *** ICTL0
pub const FB_ICTL0_IN1POL: c_int = 7;
pub const FM_ICTL0_IN1POL: c_uint = 0x80;
pub const FB_ICTL0_IN0POL: c_int = 6;
pub const FM_ICTL0_IN0POL: c_uint = 0x40;
pub const FB_ICTL0_INPCH10SEL: c_int = 4;
pub const FM_ICTL0_INPCH10SEL: c_uint = 0x30;
pub const FB_ICTL0_IN1MUTE: c_int = 3;
pub const FM_ICTL0_IN1MUTE: c_uint = 0x8;
pub const FB_ICTL0_IN0MUTE: c_int = 2;
pub const FM_ICTL0_IN0MUTE: c_uint = 0x4;
pub const FB_ICTL0_IN1HP: c_int = 1;
pub const FM_ICTL0_IN1HP: c_uint = 0x2;
pub const FB_ICTL0_IN0HP: c_int = 0;
pub const FM_ICTL0_IN0HP: c_uint = 0x1;
// *** ICTL1
pub const FB_ICTL1_IN3POL: c_int = 7;
pub const FM_ICTL1_IN3POL: c_uint = 0x80;
pub const FB_ICTL1_IN2POL: c_int = 6;
pub const FM_ICTL1_IN2POL: c_uint = 0x40;
pub const FB_ICTL1_INPCH32SEL: c_int = 4;
pub const FM_ICTL1_INPCH32SEL: c_uint = 0x30;
pub const FB_ICTL1_IN3MUTE: c_int = 3;
pub const FM_ICTL1_IN3MUTE: c_uint = 0x8;
pub const FB_ICTL1_IN2MUTE: c_int = 2;
pub const FM_ICTL1_IN2MUTE: c_uint = 0x4;
pub const FB_ICTL1_IN3HP: c_int = 1;
pub const FM_ICTL1_IN3HP: c_uint = 0x2;
pub const FB_ICTL1_IN2HP: c_int = 0;
pub const FM_ICTL1_IN2HP: c_uint = 0x1;
// *** MICBIAS
pub const FB_MICBIAS_MICBOV2: c_int = 4;
pub const FM_MICBIAS_MICBOV2: c_uint = 0x30;
pub const FB_MICBIAS_MICBOV1: c_int = 6;
pub const FM_MICBIAS_MICBOV1: c_uint = 0xC0;
pub const FB_MICBIAS_SPARE1: c_int = 2;
pub const FM_MICBIAS_SPARE1: c_uint = 0xC;
pub const FB_MICBIAS_SPARE2: c_int = 0;
pub const FM_MICBIAS_SPARE2: c_uint = 0x3;
// *** PGAZ
pub const FB_PGAZ_INHPOR: c_int = 1;
pub const FM_PGAZ_INHPOR: c_uint = 0x2;
pub const FB_PGAZ_TOEN: c_int = 0;
pub const FM_PGAZ_TOEN: c_uint = 0x1;
// *** ASRCILVOL
pub const FB_ASRCILVOL_ASRCILVOL: c_int = 0;
pub const FM_ASRCILVOL_ASRCILVOL: c_uint = 0xFF;
// *** ASRCIRVOL
pub const FB_ASRCIRVOL_ASRCIRVOL: c_int = 0;
pub const FM_ASRCIRVOL_ASRCIRVOL: c_uint = 0xFF;
// *** ASRCOLVOL
pub const FB_ASRCOLVOL_ASRCOLVOL: c_int = 0;
pub const FM_ASRCOLVOL_ASRCOLVOL: c_uint = 0xFF;
// *** ASRCORVOL
pub const FB_ASRCORVOL_ASRCOLVOL: c_int = 0;
pub const FM_ASRCORVOL_ASRCOLVOL: c_uint = 0xFF;
// *** IVOLCTLU
pub const FB_IVOLCTLU_IFADE: c_int = 3;
pub const FM_IVOLCTLU_IFADE: c_uint = 0x8;
pub const FB_IVOLCTLU_INPVOLU: c_int = 2;
pub const FM_IVOLCTLU_INPVOLU: c_uint = 0x4;
pub const FB_IVOLCTLU_PGAVOLU: c_int = 1;
pub const FM_IVOLCTLU_PGAVOLU: c_uint = 0x2;
pub const FB_IVOLCTLU_ASRCVOLU: c_int = 0;
pub const FM_IVOLCTLU_ASRCVOLU: c_uint = 0x1;
// *** ALCCTL0
pub const FB_ALCCTL0_ALCMODE: c_int = 7;
pub const FM_ALCCTL0_ALCMODE: c_uint = 0x80;
pub const FB_ALCCTL0_ALCREF: c_int = 4;
pub const FM_ALCCTL0_ALCREF: c_uint = 0x70;
pub const FB_ALCCTL0_ALCEN3: c_int = 3;
pub const FM_ALCCTL0_ALCEN3: c_uint = 0x8;
pub const FB_ALCCTL0_ALCEN2: c_int = 2;
pub const FM_ALCCTL0_ALCEN2: c_uint = 0x4;
pub const FB_ALCCTL0_ALCEN1: c_int = 1;
pub const FM_ALCCTL0_ALCEN1: c_uint = 0x2;
pub const FB_ALCCTL0_ALCEN0: c_int = 0;
pub const FM_ALCCTL0_ALCEN0: c_uint = 0x1;
// *** ALCCTL1
pub const FB_ALCCTL1_MAXGAIN: c_int = 4;
pub const FM_ALCCTL1_MAXGAIN: c_uint = 0x70;
pub const FB_ALCCTL1_ALCL: c_int = 0;
pub const FM_ALCCTL1_ALCL: c_uint = 0xF;
// *** ALCCTL2
pub const FB_ALCCTL2_ALCZC: c_int = 7;
pub const FM_ALCCTL2_ALCZC: c_uint = 0x80;
pub const FB_ALCCTL2_MINGAIN: c_int = 4;
pub const FM_ALCCTL2_MINGAIN: c_uint = 0x70;
pub const FB_ALCCTL2_HLD: c_int = 0;
pub const FM_ALCCTL2_HLD: c_uint = 0xF;
// *** ALCCTL3
pub const FB_ALCCTL3_DCY: c_int = 4;
pub const FM_ALCCTL3_DCY: c_uint = 0xF0;
pub const FB_ALCCTL3_ATK: c_int = 0;
pub const FM_ALCCTL3_ATK: c_uint = 0xF;
// *** NGATE
pub const FB_NGATE_NGTH: c_int = 3;
pub const FM_NGATE_NGTH: c_uint = 0xF8;
pub const FB_NGATE_NGG: c_int = 1;
pub const FM_NGATE_NGG: c_uint = 0x6;
pub const FB_NGATE_NGAT: c_int = 0;
pub const FM_NGATE_NGAT: c_uint = 0x1;
// *** DMICCTL
pub const FB_DMICCTL_DMIC2EN: c_int = 7;
pub const FM_DMICCTL_DMIC2EN: c_uint = 0x80;
pub const FB_DMICCTL_DMIC1EN: c_int = 6;
pub const FM_DMICCTL_DMIC1EN: c_uint = 0x40;
pub const FB_DMICCTL_DMONO: c_int = 4;
pub const FM_DMICCTL_DMONO: c_uint = 0x10;
pub const FB_DMICCTL_DMDCLK: c_int = 2;
pub const FM_DMICCTL_DMDCLK: c_uint = 0xC;
pub const FB_DMICCTL_DMRATE: c_int = 0;
pub const FM_DMICCTL_DMRATE: c_uint = 0x3;
// *** DACCTL
pub const FB_DACCTL_DACPOLR: c_int = 7;
pub const FM_DACCTL_DACPOLR: c_uint = 0x80;
pub const FV_DACPOLR_NORMAL: c_uint = 0x0;
pub const FV_DACPOLR_INVERTED: c_uint = 0x80;
pub const FB_DACCTL_DACPOLL: c_int = 6;
pub const FM_DACCTL_DACPOLL: c_uint = 0x40;
pub const FV_DACPOLL_NORMAL: c_uint = 0x0;
pub const FV_DACPOLL_INVERTED: c_uint = 0x40;
pub const FB_DACCTL_DACDITH: c_int = 4;
pub const FM_DACCTL_DACDITH: c_uint = 0x30;
pub const FV_DACDITH_DYNAMIC_HALF: c_uint = 0x0;
pub const FV_DACDITH_DYNAMIC_FULL: c_uint = 0x10;
pub const FV_DACDITH_DISABLED: c_uint = 0x20;
pub const FV_DACDITH_STATIC: c_uint = 0x30;
pub const FB_DACCTL_DACMUTE: c_int = 3;
pub const FM_DACCTL_DACMUTE: c_uint = 0x8;
pub const FV_DACMUTE_ENABLE: c_uint = 0x8;
pub const FV_DACMUTE_DISABLE: c_uint = 0x0;
pub const FB_DACCTL_DACDEM: c_int = 2;
pub const FM_DACCTL_DACDEM: c_uint = 0x4;
pub const FV_DACDEM_ENABLE: c_uint = 0x4;
pub const FV_DACDEM_DISABLE: c_uint = 0x0;
pub const FB_DACCTL_ABYPASS: c_int = 0;
pub const FM_DACCTL_ABYPASS: c_uint = 0x1;
// *** SPKCTL
pub const FB_SPKCTL_SPKPOLR: c_int = 7;
pub const FM_SPKCTL_SPKPOLR: c_uint = 0x80;
pub const FV_SPKPOLR_NORMAL: c_uint = 0x0;
pub const FV_SPKPOLR_INVERTED: c_uint = 0x80;
pub const FB_SPKCTL_SPKPOLL: c_int = 6;
pub const FM_SPKCTL_SPKPOLL: c_uint = 0x40;
pub const FV_SPKPOLL_NORMAL: c_uint = 0x0;
pub const FV_SPKPOLL_INVERTED: c_uint = 0x40;
pub const FB_SPKCTL_SPKMUTE: c_int = 3;
pub const FM_SPKCTL_SPKMUTE: c_uint = 0x8;
pub const FV_SPKMUTE_ENABLE: c_uint = 0x8;
pub const FV_SPKMUTE_DISABLE: c_uint = 0x0;
pub const FB_SPKCTL_SPKDEM: c_int = 2;
pub const FM_SPKCTL_SPKDEM: c_uint = 0x4;
pub const FV_SPKDEM_ENABLE: c_uint = 0x4;
pub const FV_SPKDEM_DISABLE: c_uint = 0x0;
// *** SUBCTL
pub const FB_SUBCTL_SUBPOL: c_int = 7;
pub const FM_SUBCTL_SUBPOL: c_uint = 0x80;
pub const FB_SUBCTL_SUBMUTE: c_int = 3;
pub const FM_SUBCTL_SUBMUTE: c_uint = 0x8;
pub const FB_SUBCTL_SUBDEM: c_int = 2;
pub const FM_SUBCTL_SUBDEM: c_uint = 0x4;
pub const FB_SUBCTL_SUBMUX: c_int = 1;
pub const FM_SUBCTL_SUBMUX: c_uint = 0x2;
pub const FB_SUBCTL_SUBILMDIS: c_int = 0;
pub const FM_SUBCTL_SUBILMDIS: c_uint = 0x1;
// *** DCCTL
pub const FB_DCCTL_SUBDCBYP: c_int = 7;
pub const FM_DCCTL_SUBDCBYP: c_uint = 0x80;
pub const FB_DCCTL_DACDCBYP: c_int = 6;
pub const FM_DCCTL_DACDCBYP: c_uint = 0x40;
pub const FB_DCCTL_SPKDCBYP: c_int = 5;
pub const FM_DCCTL_SPKDCBYP: c_uint = 0x20;
pub const FB_DCCTL_DCCOEFSEL: c_int = 0;
pub const FM_DCCTL_DCCOEFSEL: c_uint = 0x7;
// *** OVOLCTLU
pub const FB_OVOLCTLU_OFADE: c_int = 4;
pub const FM_OVOLCTLU_OFADE: c_uint = 0x10;
pub const FB_OVOLCTLU_SUBVOLU: c_int = 3;
pub const FM_OVOLCTLU_SUBVOLU: c_uint = 0x8;
pub const FB_OVOLCTLU_MVOLU: c_int = 2;
pub const FM_OVOLCTLU_MVOLU: c_uint = 0x4;
pub const FB_OVOLCTLU_SPKVOLU: c_int = 1;
pub const FM_OVOLCTLU_SPKVOLU: c_uint = 0x2;
pub const FB_OVOLCTLU_HPVOLU: c_int = 0;
pub const FM_OVOLCTLU_HPVOLU: c_uint = 0x1;
// *** MUTEC
pub const FB_MUTEC_ZDSTAT: c_int = 7;
pub const FM_MUTEC_ZDSTAT: c_uint = 0x80;
pub const FB_MUTEC_ZDLEN: c_int = 4;
pub const FM_MUTEC_ZDLEN: c_uint = 0x30;
pub const FB_MUTEC_APWD: c_int = 3;
pub const FM_MUTEC_APWD: c_uint = 0x8;
pub const FB_MUTEC_AMUTE: c_int = 2;
pub const FM_MUTEC_AMUTE: c_uint = 0x4;
// *** MVOLL
pub const FB_MVOLL_MVOL_L: c_int = 0;
pub const FM_MVOLL_MVOL_L: c_uint = 0xFF;
// *** MVOLR
pub const FB_MVOLR_MVOL_R: c_int = 0;
pub const FM_MVOLR_MVOL_R: c_uint = 0xFF;
// *** HPVOLL
pub const FB_HPVOLL_HPVOL_L: c_int = 0;
pub const FM_HPVOLL_HPVOL_L: c_uint = 0x7F;
// *** HPVOLR
pub const FB_HPVOLR_HPVOL_R: c_int = 0;
pub const FM_HPVOLR_HPVOL_R: c_uint = 0x7F;
// *** SPKVOLL
pub const FB_SPKVOLL_SPKVOL_L: c_int = 0;
pub const FM_SPKVOLL_SPKVOL_L: c_uint = 0x7F;
// *** SPKVOLR
pub const FB_SPKVOLR_SPKVOL_R: c_int = 0;
pub const FM_SPKVOLR_SPKVOL_R: c_uint = 0x7F;
// *** SUBVOL
pub const FB_SUBVOL_SUBVOL: c_int = 0;
pub const FM_SUBVOL_SUBVOL: c_uint = 0x7F;
// *** COP0
pub const FB_COP0_COPATTEN: c_int = 7;
pub const FM_COP0_COPATTEN: c_uint = 0x80;
pub const FB_COP0_COPGAIN: c_int = 6;
pub const FM_COP0_COPGAIN: c_uint = 0x40;
pub const FB_COP0_HDELTAEN: c_int = 5;
pub const FM_COP0_HDELTAEN: c_uint = 0x20;
pub const FB_COP0_COPTARGET: c_int = 0;
pub const FM_COP0_COPTARGET: c_uint = 0x1F;
// *** COP1
pub const FB_COP1_HDCOMPMODE: c_int = 6;
pub const FM_COP1_HDCOMPMODE: c_uint = 0x40;
pub const FB_COP1_AVGLENGTH: c_int = 2;
pub const FM_COP1_AVGLENGTH: c_uint = 0x3C;
pub const FB_COP1_MONRATE: c_int = 0;
pub const FM_COP1_MONRATE: c_uint = 0x3;
// *** COPSTAT
pub const FB_COPSTAT_HDELTADET: c_int = 7;
pub const FM_COPSTAT_HDELTADET: c_uint = 0x80;
pub const FB_COPSTAT_UV: c_int = 6;
pub const FM_COPSTAT_UV: c_uint = 0x40;
pub const FB_COPSTAT_COPADJ: c_int = 0;
pub const FM_COPSTAT_COPADJ: c_uint = 0x3F;
// *** PWM0
pub const FB_PWM0_SCTO: c_int = 6;
pub const FM_PWM0_SCTO: c_uint = 0xC0;
pub const FB_PWM0_UVLO: c_int = 5;
pub const FM_PWM0_UVLO: c_uint = 0x20;
pub const FB_PWM0_BFDIS: c_int = 3;
pub const FM_PWM0_BFDIS: c_uint = 0x8;
pub const FB_PWM0_PWMMODE: c_int = 2;
pub const FM_PWM0_PWMMODE: c_uint = 0x4;
pub const FB_PWM0_NOOFFSET: c_int = 0;
pub const FM_PWM0_NOOFFSET: c_uint = 0x1;
// *** PWM1
pub const FB_PWM1_DITHPOS: c_int = 4;
pub const FM_PWM1_DITHPOS: c_uint = 0x70;
pub const FB_PWM1_DYNDITH: c_int = 1;
pub const FM_PWM1_DYNDITH: c_uint = 0x2;
pub const FB_PWM1_DITHDIS: c_int = 0;
pub const FM_PWM1_DITHDIS: c_uint = 0x1;
// *** PWM2
// *** PWM3
pub const FB_PWM3_PWMMUX: c_int = 6;
pub const FM_PWM3_PWMMUX: c_uint = 0xC0;
pub const FB_PWM3_CVALUE: c_int = 0;
pub const FM_PWM3_CVALUE: c_uint = 0x7;
// *** HPSW
pub const FB_HPSW_HPDETSTATE: c_int = 4;
pub const FM_HPSW_HPDETSTATE: c_uint = 0x10;
pub const FB_HPSW_HPSWEN: c_int = 2;
pub const FM_HPSW_HPSWEN: c_uint = 0xC;
pub const FB_HPSW_HPSWPOL: c_int = 1;
pub const FM_HPSW_HPSWPOL: c_uint = 0x2;
pub const FB_HPSW_TSDEN: c_int = 0;
pub const FM_HPSW_TSDEN: c_uint = 0x1;
// *** THERMTS
pub const FB_THERMTS_TRIPHS: c_int = 7;
pub const FM_THERMTS_TRIPHS: c_uint = 0x80;
pub const FB_THERMTS_TRIPLS: c_int = 6;
pub const FM_THERMTS_TRIPLS: c_uint = 0x40;
pub const FB_THERMTS_TRIPSPLIT: c_int = 4;
pub const FM_THERMTS_TRIPSPLIT: c_uint = 0x30;
pub const FB_THERMTS_TRIPSHIFT: c_int = 2;
pub const FM_THERMTS_TRIPSHIFT: c_uint = 0xC;
pub const FB_THERMTS_TSPOLL: c_int = 0;
pub const FM_THERMTS_TSPOLL: c_uint = 0x3;
// *** THERMSPK1
pub const FB_THERMSPK1_FORCEPWD: c_int = 7;
pub const FM_THERMSPK1_FORCEPWD: c_uint = 0x80;
pub const FB_THERMSPK1_INSTCUTMODE: c_int = 6;
pub const FM_THERMSPK1_INSTCUTMODE: c_uint = 0x40;
pub const FB_THERMSPK1_INCRATIO: c_int = 4;
pub const FM_THERMSPK1_INCRATIO: c_uint = 0x30;
pub const FB_THERMSPK1_INCSTEP: c_int = 2;
pub const FM_THERMSPK1_INCSTEP: c_uint = 0xC;
pub const FB_THERMSPK1_DECSTEP: c_int = 0;
pub const FM_THERMSPK1_DECSTEP: c_uint = 0x3;
// *** THERMSTAT
pub const FB_THERMSTAT_FPWDS: c_int = 7;
pub const FM_THERMSTAT_FPWDS: c_uint = 0x80;
pub const FB_THERMSTAT_VOLSTAT: c_int = 0;
pub const FM_THERMSTAT_VOLSTAT: c_uint = 0x7F;
// *** SCSTAT
pub const FB_SCSTAT_ESDF: c_int = 3;
pub const FM_SCSTAT_ESDF: c_uint = 0x18;
pub const FB_SCSTAT_CPF: c_int = 2;
pub const FM_SCSTAT_CPF: c_uint = 0x4;
pub const FB_SCSTAT_CLSDF: c_int = 0;
pub const FM_SCSTAT_CLSDF: c_uint = 0x3;
// *** SDMON
pub const FB_SDMON_SDFORCE: c_int = 7;
pub const FM_SDMON_SDFORCE: c_uint = 0x80;
pub const FB_SDMON_SDVALUE: c_int = 0;
pub const FM_SDMON_SDVALUE: c_uint = 0x1F;
// *** SPKEQFILT
pub const FB_SPKEQFILT_EQ2EN: c_int = 7;
pub const FM_SPKEQFILT_EQ2EN: c_uint = 0x80;
pub const FV_EQ2EN_ENABLE: c_uint = 0x80;
pub const FV_EQ2EN_DISABLE: c_uint = 0x0;
pub const FB_SPKEQFILT_EQ2BE: c_int = 4;
pub const FM_SPKEQFILT_EQ2BE: c_uint = 0x70;
pub const FB_SPKEQFILT_EQ1EN: c_int = 3;
pub const FM_SPKEQFILT_EQ1EN: c_uint = 0x8;
pub const FV_EQ1EN_ENABLE: c_uint = 0x8;
pub const FV_EQ1EN_DISABLE: c_uint = 0x0;
pub const FB_SPKEQFILT_EQ1BE: c_int = 0;
pub const FM_SPKEQFILT_EQ1BE: c_uint = 0x7;
pub const SPKEQFILT_EQEN_ENABLE: c_uint = 0x1;
pub const SPKEQFILT_EQEN_DISABLE: c_uint = 0x0;
// *** SPKCRWDL
pub const FB_SPKCRWDL_WDATA_L: c_int = 0;
pub const FM_SPKCRWDL_WDATA_L: c_uint = 0xFF;
// *** SPKCRWDM
pub const FB_SPKCRWDM_WDATA_M: c_int = 0;
pub const FM_SPKCRWDM_WDATA_M: c_uint = 0xFF;
// *** SPKCRWDH
pub const FB_SPKCRWDH_WDATA_H: c_int = 0;
pub const FM_SPKCRWDH_WDATA_H: c_uint = 0xFF;
// *** SPKCRRDL
pub const FB_SPKCRRDL_RDATA_L: c_int = 0;
pub const FM_SPKCRRDL_RDATA_L: c_uint = 0xFF;
// *** SPKCRRDM
pub const FB_SPKCRRDM_RDATA_M: c_int = 0;
pub const FM_SPKCRRDM_RDATA_M: c_uint = 0xFF;
// *** SPKCRRDH
pub const FB_SPKCRRDH_RDATA_H: c_int = 0;
pub const FM_SPKCRRDH_RDATA_H: c_uint = 0xFF;
// *** SPKCRADD
pub const FB_SPKCRADD_ADDRESS: c_int = 0;
pub const FM_SPKCRADD_ADDRESS: c_uint = 0xFF;
// *** SPKCRS
pub const FB_SPKCRS_ACCSTAT: c_int = 7;
pub const FM_SPKCRS_ACCSTAT: c_uint = 0x80;
// *** SPKMBCEN
pub const FB_SPKMBCEN_MBCEN3: c_int = 2;
pub const FM_SPKMBCEN_MBCEN3: c_uint = 0x4;
pub const FV_MBCEN3_ENABLE: c_uint = 0x4;
pub const FV_MBCEN3_DISABLE: c_uint = 0x0;
pub const FB_SPKMBCEN_MBCEN2: c_int = 1;
pub const FM_SPKMBCEN_MBCEN2: c_uint = 0x2;
pub const FV_MBCEN2_ENABLE: c_uint = 0x2;
pub const FV_MBCEN2_DISABLE: c_uint = 0x0;
pub const FB_SPKMBCEN_MBCEN1: c_int = 0;
pub const FM_SPKMBCEN_MBCEN1: c_uint = 0x1;
pub const FV_MBCEN1_ENABLE: c_uint = 0x1;
pub const FV_MBCEN1_DISABLE: c_uint = 0x0;
pub const SPKMBCEN_MBCEN_ENABLE: c_uint = 0x1;
pub const SPKMBCEN_MBCEN_DISABLE: c_uint = 0x0;
// *** SPKMBCCTL
pub const FB_SPKMBCCTL_LVLMODE3: c_int = 5;
pub const FM_SPKMBCCTL_LVLMODE3: c_uint = 0x20;
pub const FB_SPKMBCCTL_WINSEL3: c_int = 4;
pub const FM_SPKMBCCTL_WINSEL3: c_uint = 0x10;
pub const FB_SPKMBCCTL_LVLMODE2: c_int = 3;
pub const FM_SPKMBCCTL_LVLMODE2: c_uint = 0x8;
pub const FB_SPKMBCCTL_WINSEL2: c_int = 2;
pub const FM_SPKMBCCTL_WINSEL2: c_uint = 0x4;
pub const FB_SPKMBCCTL_LVLMODE1: c_int = 1;
pub const FM_SPKMBCCTL_LVLMODE1: c_uint = 0x2;
pub const FB_SPKMBCCTL_WINSEL1: c_int = 0;
pub const FM_SPKMBCCTL_WINSEL1: c_uint = 0x1;
// *** SPKCLECTL
pub const FB_SPKCLECTL_LVLMODE: c_int = 4;
pub const FM_SPKCLECTL_LVLMODE: c_uint = 0x10;
pub const FB_SPKCLECTL_WINSEL: c_int = 3;
pub const FM_SPKCLECTL_WINSEL: c_uint = 0x8;
pub const FB_SPKCLECTL_EXPEN: c_int = 2;
pub const FM_SPKCLECTL_EXPEN: c_uint = 0x4;
pub const FV_EXPEN_ENABLE: c_uint = 0x4;
pub const FV_EXPEN_DISABLE: c_uint = 0x0;
pub const FB_SPKCLECTL_LIMEN: c_int = 1;
pub const FM_SPKCLECTL_LIMEN: c_uint = 0x2;
pub const FV_LIMEN_ENABLE: c_uint = 0x2;
pub const FV_LIMEN_DISABLE: c_uint = 0x0;
pub const FB_SPKCLECTL_COMPEN: c_int = 0;
pub const FM_SPKCLECTL_COMPEN: c_uint = 0x1;
pub const FV_COMPEN_ENABLE: c_uint = 0x1;
pub const FV_COMPEN_DISABLE: c_uint = 0x0;
// *** SPKCLEMUG
pub const FB_SPKCLEMUG_MUGAIN: c_int = 0;
pub const FM_SPKCLEMUG_MUGAIN: c_uint = 0x1F;
// *** SPKCOMPTHR
pub const FB_SPKCOMPTHR_THRESH: c_int = 0;
pub const FM_SPKCOMPTHR_THRESH: c_uint = 0xFF;
// *** SPKCOMPRAT
pub const FB_SPKCOMPRAT_RATIO: c_int = 0;
pub const FM_SPKCOMPRAT_RATIO: c_uint = 0x1F;
// *** SPKCOMPATKL
pub const FB_SPKCOMPATKL_TCATKL: c_int = 0;
pub const FM_SPKCOMPATKL_TCATKL: c_uint = 0xFF;
// *** SPKCOMPATKH
pub const FB_SPKCOMPATKH_TCATKH: c_int = 0;
pub const FM_SPKCOMPATKH_TCATKH: c_uint = 0xFF;
// *** SPKCOMPRELL
pub const FB_SPKCOMPRELL_TCRELL: c_int = 0;
pub const FM_SPKCOMPRELL_TCRELL: c_uint = 0xFF;
// *** SPKCOMPRELH
pub const FB_SPKCOMPRELH_TCRELH: c_int = 0;
pub const FM_SPKCOMPRELH_TCRELH: c_uint = 0xFF;
// *** SPKLIMTHR
pub const FB_SPKLIMTHR_THRESH: c_int = 0;
pub const FM_SPKLIMTHR_THRESH: c_uint = 0xFF;
// *** SPKLIMTGT
pub const FB_SPKLIMTGT_TARGET: c_int = 0;
pub const FM_SPKLIMTGT_TARGET: c_uint = 0xFF;
// *** SPKLIMATKL
pub const FB_SPKLIMATKL_TCATKL: c_int = 0;
pub const FM_SPKLIMATKL_TCATKL: c_uint = 0xFF;
// *** SPKLIMATKH
pub const FB_SPKLIMATKH_TCATKH: c_int = 0;
pub const FM_SPKLIMATKH_TCATKH: c_uint = 0xFF;
// *** SPKLIMRELL
pub const FB_SPKLIMRELL_TCRELL: c_int = 0;
pub const FM_SPKLIMRELL_TCRELL: c_uint = 0xFF;
// *** SPKLIMRELH
pub const FB_SPKLIMRELH_TCRELH: c_int = 0;
pub const FM_SPKLIMRELH_TCRELH: c_uint = 0xFF;
// *** SPKEXPTHR
pub const FB_SPKEXPTHR_THRESH: c_int = 0;
pub const FM_SPKEXPTHR_THRESH: c_uint = 0xFF;
// *** SPKEXPRAT
pub const FB_SPKEXPRAT_RATIO: c_int = 0;
pub const FM_SPKEXPRAT_RATIO: c_uint = 0x7;
// *** SPKEXPATKL
pub const FB_SPKEXPATKL_TCATKL: c_int = 0;
pub const FM_SPKEXPATKL_TCATKL: c_uint = 0xFF;
// *** SPKEXPATKH
pub const FB_SPKEXPATKH_TCATKH: c_int = 0;
pub const FM_SPKEXPATKH_TCATKH: c_uint = 0xFF;
// *** SPKEXPRELL
pub const FB_SPKEXPRELL_TCRELL: c_int = 0;
pub const FM_SPKEXPRELL_TCRELL: c_uint = 0xFF;
// *** SPKEXPRELH
pub const FB_SPKEXPRELH_TCRELH: c_int = 0;
pub const FM_SPKEXPRELH_TCRELH: c_uint = 0xFF;
// *** SPKFXCTL
pub const FB_SPKFXCTL_3DEN: c_int = 4;
pub const FM_SPKFXCTL_3DEN: c_uint = 0x10;
pub const FB_SPKFXCTL_TEEN: c_int = 3;
pub const FM_SPKFXCTL_TEEN: c_uint = 0x8;
pub const FB_SPKFXCTL_TNLFBYP: c_int = 2;
pub const FM_SPKFXCTL_TNLFBYP: c_uint = 0x4;
pub const FB_SPKFXCTL_BEEN: c_int = 1;
pub const FM_SPKFXCTL_BEEN: c_uint = 0x2;
pub const FB_SPKFXCTL_BNLFBYP: c_int = 0;
pub const FM_SPKFXCTL_BNLFBYP: c_uint = 0x1;
// *** DACEQFILT
pub const FB_DACEQFILT_EQ2EN: c_int = 7;
pub const FM_DACEQFILT_EQ2EN: c_uint = 0x80;
pub const FV_EQ2EN_ENABLE: c_uint = 0x80;
pub const FV_EQ2EN_DISABLE: c_uint = 0x0;
pub const FB_DACEQFILT_EQ2BE: c_int = 4;
pub const FM_DACEQFILT_EQ2BE: c_uint = 0x70;
pub const FB_DACEQFILT_EQ1EN: c_int = 3;
pub const FM_DACEQFILT_EQ1EN: c_uint = 0x8;
pub const FV_EQ1EN_ENABLE: c_uint = 0x8;
pub const FV_EQ1EN_DISABLE: c_uint = 0x0;
pub const FB_DACEQFILT_EQ1BE: c_int = 0;
pub const FM_DACEQFILT_EQ1BE: c_uint = 0x7;
pub const DACEQFILT_EQEN_ENABLE: c_uint = 0x1;
pub const DACEQFILT_EQEN_DISABLE: c_uint = 0x0;
// *** DACCRWDL
pub const FB_DACCRWDL_WDATA_L: c_int = 0;
pub const FM_DACCRWDL_WDATA_L: c_uint = 0xFF;
// *** DACCRWDM
pub const FB_DACCRWDM_WDATA_M: c_int = 0;
pub const FM_DACCRWDM_WDATA_M: c_uint = 0xFF;
// *** DACCRWDH
pub const FB_DACCRWDH_WDATA_H: c_int = 0;
pub const FM_DACCRWDH_WDATA_H: c_uint = 0xFF;
// *** DACCRRDL
pub const FB_DACCRRDL_RDATA_L: c_int = 0;
pub const FM_DACCRRDL_RDATA_L: c_uint = 0xFF;
// *** DACCRRDM
pub const FB_DACCRRDM_RDATA_M: c_int = 0;
pub const FM_DACCRRDM_RDATA_M: c_uint = 0xFF;
// *** DACCRRDH
pub const FB_DACCRRDH_RDATA_H: c_int = 0;
pub const FM_DACCRRDH_RDATA_H: c_uint = 0xFF;
// *** DACCRADD
pub const FB_DACCRADD_ADDRESS: c_int = 0;
pub const FM_DACCRADD_ADDRESS: c_uint = 0xFF;
// *** DACCRS
pub const FB_DACCRS_ACCSTAT: c_int = 7;
pub const FM_DACCRS_ACCSTAT: c_uint = 0x80;
// *** DACMBCEN
pub const FB_DACMBCEN_MBCEN3: c_int = 2;
pub const FM_DACMBCEN_MBCEN3: c_uint = 0x4;
pub const FV_MBCEN3_ENABLE: c_uint = 0x4;
pub const FV_MBCEN3_DISABLE: c_uint = 0x0;
pub const FB_DACMBCEN_MBCEN2: c_int = 1;
pub const FM_DACMBCEN_MBCEN2: c_uint = 0x2;
pub const FV_MBCEN2_ENABLE: c_uint = 0x2;
pub const FV_MBCEN2_DISABLE: c_uint = 0x0;
pub const FB_DACMBCEN_MBCEN1: c_int = 0;
pub const FM_DACMBCEN_MBCEN1: c_uint = 0x1;
pub const FV_MBCEN1_ENABLE: c_uint = 0x1;
pub const FV_MBCEN1_DISABLE: c_uint = 0x0;
pub const DACMBCEN_MBCEN_ENABLE: c_uint = 0x1;
pub const DACMBCEN_MBCEN_DISABLE: c_uint = 0x0;
// *** DACMBCCTL
pub const FB_DACMBCCTL_LVLMODE3: c_int = 5;
pub const FM_DACMBCCTL_LVLMODE3: c_uint = 0x20;
pub const FB_DACMBCCTL_WINSEL3: c_int = 4;
pub const FM_DACMBCCTL_WINSEL3: c_uint = 0x10;
pub const FB_DACMBCCTL_LVLMODE2: c_int = 3;
pub const FM_DACMBCCTL_LVLMODE2: c_uint = 0x8;
pub const FB_DACMBCCTL_WINSEL2: c_int = 2;
pub const FM_DACMBCCTL_WINSEL2: c_uint = 0x4;
pub const FB_DACMBCCTL_LVLMODE1: c_int = 1;
pub const FM_DACMBCCTL_LVLMODE1: c_uint = 0x2;
pub const FB_DACMBCCTL_WINSEL1: c_int = 0;
pub const FM_DACMBCCTL_WINSEL1: c_uint = 0x1;
// *** DACCLECTL
pub const FB_DACCLECTL_LVLMODE: c_int = 4;
pub const FM_DACCLECTL_LVLMODE: c_uint = 0x10;
pub const FB_DACCLECTL_WINSEL: c_int = 3;
pub const FM_DACCLECTL_WINSEL: c_uint = 0x8;
pub const FB_DACCLECTL_EXPEN: c_int = 2;
pub const FM_DACCLECTL_EXPEN: c_uint = 0x4;
pub const FV_EXPEN_ENABLE: c_uint = 0x4;
pub const FV_EXPEN_DISABLE: c_uint = 0x0;
pub const FB_DACCLECTL_LIMEN: c_int = 1;
pub const FM_DACCLECTL_LIMEN: c_uint = 0x2;
pub const FV_LIMEN_ENABLE: c_uint = 0x2;
pub const FV_LIMEN_DISABLE: c_uint = 0x0;
pub const FB_DACCLECTL_COMPEN: c_int = 0;
pub const FM_DACCLECTL_COMPEN: c_uint = 0x1;
pub const FV_COMPEN_ENABLE: c_uint = 0x1;
pub const FV_COMPEN_DISABLE: c_uint = 0x0;
// *** DACCLEMUG
pub const FB_DACCLEMUG_MUGAIN: c_int = 0;
pub const FM_DACCLEMUG_MUGAIN: c_uint = 0x1F;
// *** DACCOMPTHR
pub const FB_DACCOMPTHR_THRESH: c_int = 0;
pub const FM_DACCOMPTHR_THRESH: c_uint = 0xFF;
// *** DACCOMPRAT
pub const FB_DACCOMPRAT_RATIO: c_int = 0;
pub const FM_DACCOMPRAT_RATIO: c_uint = 0x1F;
// *** DACCOMPATKL
pub const FB_DACCOMPATKL_TCATKL: c_int = 0;
pub const FM_DACCOMPATKL_TCATKL: c_uint = 0xFF;
// *** DACCOMPATKH
pub const FB_DACCOMPATKH_TCATKH: c_int = 0;
pub const FM_DACCOMPATKH_TCATKH: c_uint = 0xFF;
// *** DACCOMPRELL
pub const FB_DACCOMPRELL_TCRELL: c_int = 0;
pub const FM_DACCOMPRELL_TCRELL: c_uint = 0xFF;
// *** DACCOMPRELH
pub const FB_DACCOMPRELH_TCRELH: c_int = 0;
pub const FM_DACCOMPRELH_TCRELH: c_uint = 0xFF;
// *** DACLIMTHR
pub const FB_DACLIMTHR_THRESH: c_int = 0;
pub const FM_DACLIMTHR_THRESH: c_uint = 0xFF;
// *** DACLIMTGT
pub const FB_DACLIMTGT_TARGET: c_int = 0;
pub const FM_DACLIMTGT_TARGET: c_uint = 0xFF;
// *** DACLIMATKL
pub const FB_DACLIMATKL_TCATKL: c_int = 0;
pub const FM_DACLIMATKL_TCATKL: c_uint = 0xFF;
// *** DACLIMATKH
pub const FB_DACLIMATKH_TCATKH: c_int = 0;
pub const FM_DACLIMATKH_TCATKH: c_uint = 0xFF;
// *** DACLIMRELL
pub const FB_DACLIMRELL_TCRELL: c_int = 0;
pub const FM_DACLIMRELL_TCRELL: c_uint = 0xFF;
// *** DACLIMRELH
pub const FB_DACLIMRELH_TCRELH: c_int = 0;
pub const FM_DACLIMRELH_TCRELH: c_uint = 0xFF;
// *** DACEXPTHR
pub const FB_DACEXPTHR_THRESH: c_int = 0;
pub const FM_DACEXPTHR_THRESH: c_uint = 0xFF;
// *** DACEXPRAT
pub const FB_DACEXPRAT_RATIO: c_int = 0;
pub const FM_DACEXPRAT_RATIO: c_uint = 0x7;
// *** DACEXPATKL
pub const FB_DACEXPATKL_TCATKL: c_int = 0;
pub const FM_DACEXPATKL_TCATKL: c_uint = 0xFF;
// *** DACEXPATKH
pub const FB_DACEXPATKH_TCATKH: c_int = 0;
pub const FM_DACEXPATKH_TCATKH: c_uint = 0xFF;
// *** DACEXPRELL
pub const FB_DACEXPRELL_TCRELL: c_int = 0;
pub const FM_DACEXPRELL_TCRELL: c_uint = 0xFF;
// *** DACEXPRELH
pub const FB_DACEXPRELH_TCRELH: c_int = 0;
pub const FM_DACEXPRELH_TCRELH: c_uint = 0xFF;
// *** DACFXCTL
pub const FB_DACFXCTL_3DEN: c_int = 4;
pub const FM_DACFXCTL_3DEN: c_uint = 0x10;
pub const FB_DACFXCTL_TEEN: c_int = 3;
pub const FM_DACFXCTL_TEEN: c_uint = 0x8;
pub const FB_DACFXCTL_TNLFBYP: c_int = 2;
pub const FM_DACFXCTL_TNLFBYP: c_uint = 0x4;
pub const FB_DACFXCTL_BEEN: c_int = 1;
pub const FM_DACFXCTL_BEEN: c_uint = 0x2;
pub const FB_DACFXCTL_BNLFBYP: c_int = 0;
pub const FM_DACFXCTL_BNLFBYP: c_uint = 0x1;
// *** SUBEQFILT
pub const FB_SUBEQFILT_EQ2EN: c_int = 7;
pub const FM_SUBEQFILT_EQ2EN: c_uint = 0x80;
pub const FV_EQ2EN_ENABLE: c_uint = 0x80;
pub const FV_EQ2EN_DISABLE: c_uint = 0x0;
pub const FB_SUBEQFILT_EQ2BE: c_int = 4;
pub const FM_SUBEQFILT_EQ2BE: c_uint = 0x70;
pub const FB_SUBEQFILT_EQ1EN: c_int = 3;
pub const FM_SUBEQFILT_EQ1EN: c_uint = 0x8;
pub const FV_EQ1EN_ENABLE: c_uint = 0x8;
pub const FV_EQ1EN_DISABLE: c_uint = 0x0;
pub const FB_SUBEQFILT_EQ1BE: c_int = 0;
pub const FM_SUBEQFILT_EQ1BE: c_uint = 0x7;
pub const SUBEQFILT_EQEN_ENABLE: c_uint = 0x1;
pub const SUBEQFILT_EQEN_DISABLE: c_uint = 0x0;
// *** SUBCRWDL
pub const FB_SUBCRWDL_WDATA_L: c_int = 0;
pub const FM_SUBCRWDL_WDATA_L: c_uint = 0xFF;
// *** SUBCRWDM
pub const FB_SUBCRWDM_WDATA_M: c_int = 0;
pub const FM_SUBCRWDM_WDATA_M: c_uint = 0xFF;
// *** SUBCRWDH
pub const FB_SUBCRWDH_WDATA_H: c_int = 0;
pub const FM_SUBCRWDH_WDATA_H: c_uint = 0xFF;
// *** SUBCRRDL
pub const FB_SUBCRRDL_RDATA_L: c_int = 0;
pub const FM_SUBCRRDL_RDATA_L: c_uint = 0xFF;
// *** SUBCRRDM
pub const FB_SUBCRRDM_RDATA_M: c_int = 0;
pub const FM_SUBCRRDM_RDATA_M: c_uint = 0xFF;
// *** SUBCRRDH
pub const FB_SUBCRRDH_RDATA_H: c_int = 0;
pub const FM_SUBCRRDH_RDATA_H: c_uint = 0xFF;
// *** SUBCRADD
pub const FB_SUBCRADD_ADDRESS: c_int = 0;
pub const FM_SUBCRADD_ADDRESS: c_uint = 0xFF;
// *** SUBCRS
pub const FB_SUBCRS_ACCSTAT: c_int = 7;
pub const FM_SUBCRS_ACCSTAT: c_uint = 0x80;
// *** SUBMBCEN
pub const FB_SUBMBCEN_MBCEN3: c_int = 2;
pub const FM_SUBMBCEN_MBCEN3: c_uint = 0x4;
pub const FV_MBCEN3_ENABLE: c_uint = 0x4;
pub const FV_MBCEN3_DISABLE: c_uint = 0x0;
pub const FB_SUBMBCEN_MBCEN2: c_int = 1;
pub const FM_SUBMBCEN_MBCEN2: c_uint = 0x2;
pub const FV_MBCEN2_ENABLE: c_uint = 0x2;
pub const FV_MBCEN2_DISABLE: c_uint = 0x0;
pub const FB_SUBMBCEN_MBCEN1: c_int = 0;
pub const FM_SUBMBCEN_MBCEN1: c_uint = 0x1;
pub const FV_MBCEN1_ENABLE: c_uint = 0x1;
pub const FV_MBCEN1_DISABLE: c_uint = 0x0;
pub const SUBMBCEN_MBCEN_ENABLE: c_uint = 0x1;
pub const SUBMBCEN_MBCEN_DISABLE: c_uint = 0x0;
// *** SUBMBCCTL
pub const FB_SUBMBCCTL_LVLMODE3: c_int = 5;
pub const FM_SUBMBCCTL_LVLMODE3: c_uint = 0x20;
pub const FB_SUBMBCCTL_WINSEL3: c_int = 4;
pub const FM_SUBMBCCTL_WINSEL3: c_uint = 0x10;
pub const FB_SUBMBCCTL_LVLMODE2: c_int = 3;
pub const FM_SUBMBCCTL_LVLMODE2: c_uint = 0x8;
pub const FB_SUBMBCCTL_WINSEL2: c_int = 2;
pub const FM_SUBMBCCTL_WINSEL2: c_uint = 0x4;
pub const FB_SUBMBCCTL_LVLMODE1: c_int = 1;
pub const FM_SUBMBCCTL_LVLMODE1: c_uint = 0x2;
pub const FB_SUBMBCCTL_WINSEL1: c_int = 0;
pub const FM_SUBMBCCTL_WINSEL1: c_uint = 0x1;
// *** SUBCLECTL
pub const FB_SUBCLECTL_LVLMODE: c_int = 4;
pub const FM_SUBCLECTL_LVLMODE: c_uint = 0x10;
pub const FB_SUBCLECTL_WINSEL: c_int = 3;
pub const FM_SUBCLECTL_WINSEL: c_uint = 0x8;
pub const FB_SUBCLECTL_EXPEN: c_int = 2;
pub const FM_SUBCLECTL_EXPEN: c_uint = 0x4;
pub const FV_EXPEN_ENABLE: c_uint = 0x4;
pub const FV_EXPEN_DISABLE: c_uint = 0x0;
pub const FB_SUBCLECTL_LIMEN: c_int = 1;
pub const FM_SUBCLECTL_LIMEN: c_uint = 0x2;
pub const FV_LIMEN_ENABLE: c_uint = 0x2;
pub const FV_LIMEN_DISABLE: c_uint = 0x0;
pub const FB_SUBCLECTL_COMPEN: c_int = 0;
pub const FM_SUBCLECTL_COMPEN: c_uint = 0x1;
pub const FV_COMPEN_ENABLE: c_uint = 0x1;
pub const FV_COMPEN_DISABLE: c_uint = 0x0;
// *** SUBCLEMUG
pub const FB_SUBCLEMUG_MUGAIN: c_int = 0;
pub const FM_SUBCLEMUG_MUGAIN: c_uint = 0x1F;
// *** SUBCOMPTHR
pub const FB_SUBCOMPTHR_THRESH: c_int = 0;
pub const FM_SUBCOMPTHR_THRESH: c_uint = 0xFF;
// *** SUBCOMPRAT
pub const FB_SUBCOMPRAT_RATIO: c_int = 0;
pub const FM_SUBCOMPRAT_RATIO: c_uint = 0x1F;
// *** SUBCOMPATKL
pub const FB_SUBCOMPATKL_TCATKL: c_int = 0;
pub const FM_SUBCOMPATKL_TCATKL: c_uint = 0xFF;
// *** SUBCOMPATKH
pub const FB_SUBCOMPATKH_TCATKH: c_int = 0;
pub const FM_SUBCOMPATKH_TCATKH: c_uint = 0xFF;
// *** SUBCOMPRELL
pub const FB_SUBCOMPRELL_TCRELL: c_int = 0;
pub const FM_SUBCOMPRELL_TCRELL: c_uint = 0xFF;
// *** SUBCOMPRELH
pub const FB_SUBCOMPRELH_TCRELH: c_int = 0;
pub const FM_SUBCOMPRELH_TCRELH: c_uint = 0xFF;
// *** SUBLIMTHR
pub const FB_SUBLIMTHR_THRESH: c_int = 0;
pub const FM_SUBLIMTHR_THRESH: c_uint = 0xFF;
// *** SUBLIMTGT
pub const FB_SUBLIMTGT_TARGET: c_int = 0;
pub const FM_SUBLIMTGT_TARGET: c_uint = 0xFF;
// *** SUBLIMATKL
pub const FB_SUBLIMATKL_TCATKL: c_int = 0;
pub const FM_SUBLIMATKL_TCATKL: c_uint = 0xFF;
// *** SUBLIMATKH
pub const FB_SUBLIMATKH_TCATKH: c_int = 0;
pub const FM_SUBLIMATKH_TCATKH: c_uint = 0xFF;
// *** SUBLIMRELL
pub const FB_SUBLIMRELL_TCRELL: c_int = 0;
pub const FM_SUBLIMRELL_TCRELL: c_uint = 0xFF;
// *** SUBLIMRELH
pub const FB_SUBLIMRELH_TCRELH: c_int = 0;
pub const FM_SUBLIMRELH_TCRELH: c_uint = 0xFF;
// *** SUBEXPTHR
pub const FB_SUBEXPTHR_THRESH: c_int = 0;
pub const FM_SUBEXPTHR_THRESH: c_uint = 0xFF;
// *** SUBEXPRAT
pub const FB_SUBEXPRAT_RATIO: c_int = 0;
pub const FM_SUBEXPRAT_RATIO: c_uint = 0x7;
// *** SUBEXPATKL
pub const FB_SUBEXPATKL_TCATKL: c_int = 0;
pub const FM_SUBEXPATKL_TCATKL: c_uint = 0xFF;
// *** SUBEXPATKH
pub const FB_SUBEXPATKH_TCATKH: c_int = 0;
pub const FM_SUBEXPATKH_TCATKH: c_uint = 0xFF;
// *** SUBEXPRELL
pub const FB_SUBEXPRELL_TCRELL: c_int = 0;
pub const FM_SUBEXPRELL_TCRELL: c_uint = 0xFF;
// *** SUBEXPRELH
pub const FB_SUBEXPRELH_TCRELH: c_int = 0;
pub const FM_SUBEXPRELH_TCRELH: c_uint = 0xFF;
// *** SUBFXCTL
pub const FB_SUBFXCTL_TEEN: c_int = 3;
pub const FM_SUBFXCTL_TEEN: c_uint = 0x8;
pub const FB_SUBFXCTL_TNLFBYP: c_int = 2;
pub const FM_SUBFXCTL_TNLFBYP: c_uint = 0x4;
pub const FB_SUBFXCTL_BEEN: c_int = 1;
pub const FM_SUBFXCTL_BEEN: c_uint = 0x2;
pub const FB_SUBFXCTL_BNLFBYP: c_int = 0;
pub const FM_SUBFXCTL_BNLFBYP: c_uint = 0x1;
