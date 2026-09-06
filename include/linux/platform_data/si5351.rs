//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/si5351.h
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
// Si5351A/B/C programmable clock generator platform_data.
//
// enum si5351_pll_src - Si5351 pll clock source
// @SI5351_PLL_SRC_DEFAULT: default, do not change eeprom config
// @SI5351_PLL_SRC_XTAL: pll source clock is XTAL input
// @SI5351_PLL_SRC_CLKIN: pll source clock is CLKIN input (Si5351C only)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_pll_src {
    SI5351_PLL_SRC_DEFAULT = 0,
    SI5351_PLL_SRC_XTAL = 1,
    SI5351_PLL_SRC_CLKIN = 2,
}

//
// enum si5351_multisynth_src - Si5351 multisynth clock source
// @SI5351_MULTISYNTH_SRC_DEFAULT: default, do not change eeprom config
// @SI5351_MULTISYNTH_SRC_VCO0: multisynth source clock is VCO0
// @SI5351_MULTISYNTH_SRC_VCO1: multisynth source clock is VCO1/VXCO
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_multisynth_src {
    SI5351_MULTISYNTH_SRC_DEFAULT = 0,
    SI5351_MULTISYNTH_SRC_VCO0 = 1,
    SI5351_MULTISYNTH_SRC_VCO1 = 2,
}

//
// enum si5351_clkout_src - Si5351 clock output clock source
// @SI5351_CLKOUT_SRC_DEFAULT: default, do not change eeprom config
// @SI5351_CLKOUT_SRC_MSYNTH_N: clkout N source clock is multisynth N
// @SI5351_CLKOUT_SRC_MSYNTH_0_4: clkout N source clock is multisynth 0 (N<4)
// or 4 (N>=4)
// @SI5351_CLKOUT_SRC_XTAL: clkout N source clock is XTAL
// @SI5351_CLKOUT_SRC_CLKIN: clkout N source clock is CLKIN (Si5351C only)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_clkout_src {
    SI5351_CLKOUT_SRC_DEFAULT = 0,
    SI5351_CLKOUT_SRC_MSYNTH_N = 1,
    SI5351_CLKOUT_SRC_MSYNTH_0_4 = 2,
    SI5351_CLKOUT_SRC_XTAL = 3,
    SI5351_CLKOUT_SRC_CLKIN = 4,
}

//
// enum si5351_drive_strength - Si5351 clock output drive strength
// @SI5351_DRIVE_DEFAULT: default, do not change eeprom config
// @SI5351_DRIVE_2MA: 2mA clock output drive strength
// @SI5351_DRIVE_4MA: 4mA clock output drive strength
// @SI5351_DRIVE_6MA: 6mA clock output drive strength
// @SI5351_DRIVE_8MA: 8mA clock output drive strength
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_drive_strength {
    SI5351_DRIVE_DEFAULT = 0,
    SI5351_DRIVE_2MA = 2,
    SI5351_DRIVE_4MA = 4,
    SI5351_DRIVE_6MA = 6,
    SI5351_DRIVE_8MA = 8,
}

//
// enum si5351_disable_state - Si5351 clock output disable state
// @SI5351_DISABLE_DEFAULT: default, do not change eeprom config
// @SI5351_DISABLE_LOW: CLKx is set to a LOW state when disabled
// @SI5351_DISABLE_HIGH: CLKx is set to a HIGH state when disabled
// @SI5351_DISABLE_FLOATING: CLKx is set to a FLOATING state when
// disabled
// @SI5351_DISABLE_NEVER: CLKx is NEVER disabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_disable_state {
    SI5351_DISABLE_DEFAULT = 0,
    SI5351_DISABLE_LOW,
    SI5351_DISABLE_HIGH,
    SI5351_DISABLE_FLOATING,
    SI5351_DISABLE_NEVER,
}

//
// struct si5351_clkout_config - Si5351 clock output configuration
// @clkout: clkout number
// @multisynth_src: multisynth source clock
// @clkout_src: clkout source clock
// @pll_master: if true, clkout can also change pll rate
// @pll_reset: if true, clkout can reset its pll
// @drive: output drive strength
// @rate: initial clkout rate, or default if 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si5351_clkout_config {
    pub multisynth_src: si5351_multisynth_src,
    pub clkout_src: si5351_clkout_src,
    pub drive: si5351_drive_strength,
    pub disable_state: si5351_disable_state,
    pub pll_master: bool,
    pub pll_reset: bool,
    pub rate: c_ulong,
}

//
// struct si5351_platform_data - Platform data for the Si5351 clock driver
// @clk_xtal: xtal input clock
// @clk_clkin: clkin input clock
// @pll_src: array of pll source clock setting
// @pll_reset: array indicating if plls should be reset after setting the rate
// @clkout: array of clkout configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si5351_platform_data {
    pub pll_src: [si5351_pll_src; 2],
    pub pll_reset: [bool; 2],
    pub clkout: [si5351_clkout_config; 8],
}
