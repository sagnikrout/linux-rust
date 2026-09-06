//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/nau8810.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// NAU8810 ALSA SoC audio driver
//
// Copyright 2016 Nuvoton Technology Corp.
// Author: David Lin <ctlin0@nuvoton.com>
//
pub const NAU8810_REG_RESET: c_uint = 0x00;
pub const NAU8810_REG_POWER1: c_uint = 0x01;
pub const NAU8810_REG_POWER2: c_uint = 0x02;
pub const NAU8810_REG_POWER3: c_uint = 0x03;
pub const NAU8810_REG_IFACE: c_uint = 0x04;
pub const NAU8810_REG_COMP: c_uint = 0x05;
pub const NAU8810_REG_CLOCK: c_uint = 0x06;
pub const NAU8810_REG_SMPLR: c_uint = 0x07;
pub const NAU8810_REG_DAC: c_uint = 0x0A;
pub const NAU8810_REG_DACGAIN: c_uint = 0x0B;
pub const NAU8810_REG_ADC: c_uint = 0x0E;
pub const NAU8810_REG_ADCGAIN: c_uint = 0x0F;
pub const NAU8810_REG_EQ1: c_uint = 0x12;
pub const NAU8810_REG_EQ2: c_uint = 0x13;
pub const NAU8810_REG_EQ3: c_uint = 0x14;
pub const NAU8810_REG_EQ4: c_uint = 0x15;
pub const NAU8810_REG_EQ5: c_uint = 0x16;
pub const NAU8810_REG_DACLIM1: c_uint = 0x18;
pub const NAU8810_REG_DACLIM2: c_uint = 0x19;
pub const NAU8810_REG_NOTCH1: c_uint = 0x1B;
pub const NAU8810_REG_NOTCH2: c_uint = 0x1C;
pub const NAU8810_REG_NOTCH3: c_uint = 0x1D;
pub const NAU8810_REG_NOTCH4: c_uint = 0x1E;
pub const NAU8810_REG_ALC1: c_uint = 0x20;
pub const NAU8810_REG_ALC2: c_uint = 0x21;
pub const NAU8810_REG_ALC3: c_uint = 0x22;
pub const NAU8810_REG_NOISEGATE: c_uint = 0x23;
pub const NAU8810_REG_PLLN: c_uint = 0x24;
pub const NAU8810_REG_PLLK1: c_uint = 0x25;
pub const NAU8810_REG_PLLK2: c_uint = 0x26;
pub const NAU8810_REG_PLLK3: c_uint = 0x27;
pub const NAU8810_REG_ATTEN: c_uint = 0x28;
pub const NAU8810_REG_INPUT_SIGNAL: c_uint = 0x2C;
pub const NAU8810_REG_PGAGAIN: c_uint = 0x2D;
pub const NAU8810_REG_ADCBOOST: c_uint = 0x2F;
pub const NAU8810_REG_OUTPUT: c_uint = 0x31;
pub const NAU8810_REG_SPKMIX: c_uint = 0x32;
pub const NAU8810_REG_SPKGAIN: c_uint = 0x36;
pub const NAU8810_REG_MONOMIX: c_uint = 0x38;
pub const NAU8810_REG_POWER4: c_uint = 0x3A;
pub const NAU8810_REG_TSLOTCTL1: c_uint = 0x3B;
pub const NAU8810_REG_TSLOTCTL2: c_uint = 0x3C;
pub const NAU8810_REG_DEVICE_REVID: c_uint = 0x3E;
pub const NAU8810_REG_I2C_DEVICEID: c_uint = 0x3F;
pub const NAU8810_REG_ADDITIONID: c_uint = 0x40;
pub const NAU8810_REG_RESERVE: c_uint = 0x41;
pub const NAU8810_REG_OUTCTL: c_uint = 0x45;
pub const NAU8810_REG_ALC1ENHAN1: c_uint = 0x46;
pub const NAU8810_REG_ALC1ENHAN2: c_uint = 0x47;
pub const NAU8810_REG_MISCCTL: c_uint = 0x49;
pub const NAU8810_REG_OUTTIEOFF: c_uint = 0x4B;
pub const NAU8810_REG_AGCP2POUT: c_uint = 0x4C;
pub const NAU8810_REG_AGCPOUT: c_uint = 0x4D;
pub const NAU8810_REG_AMTCTL: c_uint = 0x4E;
pub const NAU8810_REG_OUTTIEOFFMAN: c_uint = 0x4F;

// NAU8810_REG_POWER1 (0x1)

pub const NAU8810_AUX_EN_SFT: c_int = 6;
pub const NAU8810_PLL_EN_SFT: c_int = 5;
pub const NAU8810_MICBIAS_EN_SFT: c_int = 4;

pub const NAU8810_REFIMP_MASK: c_uint = 0x3;
pub const NAU8810_REFIMP_DIS: c_uint = 0x0;
pub const NAU8810_REFIMP_80K: c_uint = 0x1;
pub const NAU8810_REFIMP_300K: c_uint = 0x2;
pub const NAU8810_REFIMP_3K: c_uint = 0x3;
// NAU8810_REG_POWER2 (0x2)
pub const NAU8810_BST_EN_SFT: c_int = 4;
pub const NAU8810_PGA_EN_SFT: c_int = 2;
pub const NAU8810_ADC_EN_SFT: c_int = 0;
// NAU8810_REG_POWER3 (0x3)
pub const NAU8810_DAC_EN_SFT: c_int = 0;
pub const NAU8810_SPKMX_EN_SFT: c_int = 2;
pub const NAU8810_MOUTMX_EN_SFT: c_int = 3;
pub const NAU8810_PSPK_EN_SFT: c_int = 5;
pub const NAU8810_NSPK_EN_SFT: c_int = 6;
pub const NAU8810_MOUT_EN_SFT: c_int = 7;
// NAU8810_REG_IFACE (0x4)
pub const NAU8810_AIFMT_SFT: c_int = 3;

pub const NAU8810_WLEN_SFT: c_int = 5;

// NAU8810_REG_COMP (0x5)
pub const NAU8810_ADDAP_SFT: c_int = 0;
pub const NAU8810_ADCCM_SFT: c_int = 1;
pub const NAU8810_DACCM_SFT: c_int = 3;
// NAU8810_REG_CLOCK (0x6)
pub const NAU8810_CLKIO_MASK: c_uint = 0x1;
pub const NAU8810_CLKIO_SLAVE: c_uint = 0x0;
pub const NAU8810_CLKIO_MASTER: c_uint = 0x1;
pub const NAU8810_BCLKSEL_SFT: c_int = 2;

pub const NAU8810_MCLKSEL_SFT: c_int = 5;

pub const NAU8810_CLKM_SFT: c_int = 8;

// NAU8810_REG_SMPLR (0x7)
pub const NAU8810_SMPLR_SFT: c_int = 1;

// NAU8810_REG_DAC (0xA)
pub const NAU8810_DACPL_SFT: c_int = 0;
pub const NAU8810_DACOS_SFT: c_int = 3;
pub const NAU8810_DEEMP_SFT: c_int = 4;
// NAU8810_REG_DACGAIN (0xB)
pub const NAU8810_DACGAIN_SFT: c_int = 0;
// NAU8810_REG_ADC (0xE)
pub const NAU8810_ADCPL_SFT: c_int = 0;
pub const NAU8810_ADCOS_SFT: c_int = 3;
pub const NAU8810_HPF_SFT: c_int = 4;
pub const NAU8810_HPFEN_SFT: c_int = 8;
// NAU8810_REG_ADCGAIN (0xF)
pub const NAU8810_ADCGAIN_SFT: c_int = 0;
// NAU8810_REG_EQ1 (0x12)
pub const NAU8810_EQ1GC_SFT: c_int = 0;
pub const NAU8810_EQ1CF_SFT: c_int = 5;
pub const NAU8810_EQM_SFT: c_int = 8;
// NAU8810_REG_EQ2 (0x13)
pub const NAU8810_EQ2GC_SFT: c_int = 0;
pub const NAU8810_EQ2CF_SFT: c_int = 5;
pub const NAU8810_EQ2BW_SFT: c_int = 8;
// NAU8810_REG_EQ3 (0x14)
pub const NAU8810_EQ3GC_SFT: c_int = 0;
pub const NAU8810_EQ3CF_SFT: c_int = 5;
pub const NAU8810_EQ3BW_SFT: c_int = 8;
// NAU8810_REG_EQ4 (0x15)
pub const NAU8810_EQ4GC_SFT: c_int = 0;
pub const NAU8810_EQ4CF_SFT: c_int = 5;
pub const NAU8810_EQ4BW_SFT: c_int = 8;
// NAU8810_REG_EQ5 (0x16)
pub const NAU8810_EQ5GC_SFT: c_int = 0;
pub const NAU8810_EQ5CF_SFT: c_int = 5;
// NAU8810_REG_DACLIM1 (0x18)
pub const NAU8810_DACLIMATK_SFT: c_int = 0;
pub const NAU8810_DACLIMDCY_SFT: c_int = 4;
pub const NAU8810_DACLIMEN_SFT: c_int = 8;
// NAU8810_REG_DACLIM2 (0x19)
pub const NAU8810_DACLIMBST_SFT: c_int = 0;
pub const NAU8810_DACLIMTHL_SFT: c_int = 4;
// NAU8810_REG_ALC1 (0x20)
pub const NAU8810_ALCMINGAIN_SFT: c_int = 0;
pub const NAU8810_ALCMXGAIN_SFT: c_int = 3;
pub const NAU8810_ALCEN_SFT: c_int = 8;
// NAU8810_REG_ALC2 (0x21)
pub const NAU8810_ALCSL_SFT: c_int = 0;
pub const NAU8810_ALCHT_SFT: c_int = 4;
pub const NAU8810_ALCZC_SFT: c_int = 8;
// NAU8810_REG_ALC3 (0x22)
pub const NAU8810_ALCATK_SFT: c_int = 0;
pub const NAU8810_ALCDCY_SFT: c_int = 4;
pub const NAU8810_ALCM_SFT: c_int = 8;
// NAU8810_REG_NOISEGATE (0x23)
pub const NAU8810_ALCNTH_SFT: c_int = 0;
pub const NAU8810_ALCNEN_SFT: c_int = 3;
// NAU8810_REG_PLLN (0x24)
pub const NAU8810_PLLN_MASK: c_uint = 0xF;

// NAU8810_REG_PLLK1 (0x25)
pub const NAU8810_PLLK1_SFT: c_int = 18;
pub const NAU8810_PLLK1_MASK: c_uint = 0x3F;
// NAU8810_REG_PLLK2 (0x26)
pub const NAU8810_PLLK2_SFT: c_int = 9;
pub const NAU8810_PLLK2_MASK: c_uint = 0x1FF;
// NAU8810_REG_PLLK3 (0x27)
pub const NAU8810_PLLK3_MASK: c_uint = 0x1FF;
// NAU8810_REG_INPUT_SIGNAL (0x2C)
pub const NAU8810_PMICPGA_SFT: c_int = 0;

pub const NAU8810_NMICPGA_SFT: c_int = 1;

pub const NAU8810_AUXPGA_SFT: c_int = 2;
// NAU8810_REG_PGAGAIN (0x2D)
pub const NAU8810_PGAGAIN_SFT: c_int = 0;
pub const NAU8810_PGAMT_SFT: c_int = 6;
pub const NAU8810_PGAZC_SFT: c_int = 7;
// NAU8810_REG_ADCBOOST (0x2F)
pub const NAU8810_AUXBSTGAIN_SFT: c_int = 0;
pub const NAU8810_PMICBSTGAIN_SFT: c_int = 4;

pub const NAU8810_PGABST_SFT: c_int = 8;
// NAU8810_REG_SPKMIX (0x32)
pub const NAU8810_DACSPK_SFT: c_int = 0;
pub const NAU8810_BYPSPK_SFT: c_int = 1;
pub const NAU8810_AUXSPK_SFT: c_int = 5;
// NAU8810_REG_SPKGAIN (0x36)
pub const NAU8810_SPKGAIN_SFT: c_int = 0;
pub const NAU8810_SPKMT_SFT: c_int = 6;
pub const NAU8810_SPKZC_SFT: c_int = 7;
// NAU8810_REG_MONOMIX (0x38)
pub const NAU8810_DACMOUT_SFT: c_int = 0;
pub const NAU8810_BYPMOUT_SFT: c_int = 1;
pub const NAU8810_AUXMOUT_SFT: c_int = 2;
pub const NAU8810_MOUTMXMT_SFT: c_int = 6;
// System Clock Source
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8810_pll {
    pub pre_factor: c_int,
    pub mclk_scaler: c_int,
    pub pll_frac: c_int,
    pub pll_int: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8810 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub pll: nau8810_pll,
    pub sysclk: c_int,
    pub clk_id: c_int,
}
