//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/clk-si5351.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// clk-si5351.h: Skyworks / Silicon Labs Si5351A/B/C I2C Clock Generator
//
// Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
// Rabeeh Khoury <rabeeh@solid-run.com>
//
pub const SI5351_BUS_BASE_ADDR: c_uint = 0x60;
pub const SI5351_PLL_VCO_MIN: c_int = 600000000;
pub const SI5351_PLL_VCO_MAX: c_int = 900000000;
pub const SI5351_MULTISYNTH_MIN_FREQ: c_int = 1000000;
pub const SI5351_MULTISYNTH_DIVBY4_FREQ: c_int = 150000000;
pub const SI5351_MULTISYNTH_MAX_FREQ: c_int = 160000000;

pub const SI5351_CLKOUT_MIN_FREQ: c_int = 8000;

pub const SI5351_PLL_A_MIN: c_int = 15;
pub const SI5351_PLL_A_MAX: c_int = 90;

pub const SI5351_PLL_C_MAX: c_int = 1048575;
pub const SI5351_MULTISYNTH_A_MIN: c_int = 6;
pub const SI5351_MULTISYNTH_A_MAX: c_int = 1800;
pub const SI5351_MULTISYNTH67_A_MAX: c_int = 254;

pub const SI5351_MULTISYNTH_C_MAX: c_int = 1048575;

pub const SI5351_DEVICE_STATUS: c_int = 0;
pub const SI5351_INTERRUPT_STATUS: c_int = 1;
pub const SI5351_INTERRUPT_MASK: c_int = 2;

pub const SI5351_OUTPUT_ENABLE_CTRL: c_int = 3;
pub const SI5351_OEB_PIN_ENABLE_CTRL: c_int = 9;
pub const SI5351_PLL_INPUT_SOURCE: c_int = 15;

pub const SI5351_CLK0_CTRL: c_int = 16;
pub const SI5351_CLK1_CTRL: c_int = 17;
pub const SI5351_CLK2_CTRL: c_int = 18;
pub const SI5351_CLK3_CTRL: c_int = 19;
pub const SI5351_CLK4_CTRL: c_int = 20;
pub const SI5351_CLK5_CTRL: c_int = 21;
pub const SI5351_CLK6_CTRL: c_int = 22;
pub const SI5351_CLK7_CTRL: c_int = 23;

pub const SI5351_CLK3_0_DISABLE_STATE: c_int = 24;
pub const SI5351_CLK7_4_DISABLE_STATE: c_int = 25;
pub const SI5351_CLK_DISABLE_STATE_MASK: c_int = 3;
pub const SI5351_CLK_DISABLE_STATE_LOW: c_int = 0;
pub const SI5351_CLK_DISABLE_STATE_HIGH: c_int = 1;
pub const SI5351_CLK_DISABLE_STATE_FLOAT: c_int = 2;
pub const SI5351_CLK_DISABLE_STATE_NEVER: c_int = 3;
pub const SI5351_PARAMETERS_LENGTH: c_int = 8;
pub const SI5351_PLLA_PARAMETERS: c_int = 26;
pub const SI5351_PLLB_PARAMETERS: c_int = 34;
pub const SI5351_CLK0_PARAMETERS: c_int = 42;
pub const SI5351_CLK1_PARAMETERS: c_int = 50;
pub const SI5351_CLK2_PARAMETERS: c_int = 58;
pub const SI5351_CLK3_PARAMETERS: c_int = 66;
pub const SI5351_CLK4_PARAMETERS: c_int = 74;
pub const SI5351_CLK5_PARAMETERS: c_int = 82;
pub const SI5351_CLK6_PARAMETERS: c_int = 90;
pub const SI5351_CLK7_PARAMETERS: c_int = 91;
pub const SI5351_CLK6_7_OUTPUT_DIVIDER: c_int = 92;

pub const SI5351_OUTPUT_CLK_DIV_SHIFT: c_int = 4;
pub const SI5351_OUTPUT_CLK_DIV6_SHIFT: c_int = 0;
pub const SI5351_OUTPUT_CLK_DIV_1: c_int = 0;
pub const SI5351_OUTPUT_CLK_DIV_2: c_int = 1;
pub const SI5351_OUTPUT_CLK_DIV_4: c_int = 2;
pub const SI5351_OUTPUT_CLK_DIV_8: c_int = 3;
pub const SI5351_OUTPUT_CLK_DIV_16: c_int = 4;
pub const SI5351_OUTPUT_CLK_DIV_32: c_int = 5;
pub const SI5351_OUTPUT_CLK_DIV_64: c_int = 6;
pub const SI5351_OUTPUT_CLK_DIV_128: c_int = 7;

pub const SI5351_SSC_PARAM0: c_int = 149;
pub const SI5351_SSC_PARAM1: c_int = 150;
pub const SI5351_SSC_PARAM2: c_int = 151;
pub const SI5351_SSC_PARAM3: c_int = 152;
pub const SI5351_SSC_PARAM4: c_int = 153;
pub const SI5351_SSC_PARAM5: c_int = 154;
pub const SI5351_SSC_PARAM6: c_int = 155;
pub const SI5351_SSC_PARAM7: c_int = 156;
pub const SI5351_SSC_PARAM8: c_int = 157;
pub const SI5351_SSC_PARAM9: c_int = 158;
pub const SI5351_SSC_PARAM10: c_int = 159;
pub const SI5351_SSC_PARAM11: c_int = 160;
pub const SI5351_SSC_PARAM12: c_int = 161;
pub const SI5351_VXCO_PARAMETERS_LOW: c_int = 162;
pub const SI5351_VXCO_PARAMETERS_MID: c_int = 163;
pub const SI5351_VXCO_PARAMETERS_HIGH: c_int = 164;
pub const SI5351_CLK0_PHASE_OFFSET: c_int = 165;
pub const SI5351_CLK1_PHASE_OFFSET: c_int = 166;
pub const SI5351_CLK2_PHASE_OFFSET: c_int = 167;
pub const SI5351_CLK3_PHASE_OFFSET: c_int = 168;
pub const SI5351_CLK4_PHASE_OFFSET: c_int = 169;
pub const SI5351_CLK5_PHASE_OFFSET: c_int = 170;
pub const SI5351_PLL_RESET: c_int = 177;

pub const SI5351_CRYSTAL_LOAD: c_int = 183;

pub const SI5351_FANOUT_ENABLE: c_int = 187;

//
// enum si5351_variant - SiLabs Si5351 chip variant
// @SI5351_VARIANT_A: Si5351A (8 output clocks, XTAL input)
// @SI5351_VARIANT_A3: Si5351A MSOP10 (3 output clocks, XTAL input)
// @SI5351_VARIANT_B: Si5351B (8 output clocks, XTAL/VXCO input)
// @SI5351_VARIANT_C: Si5351C (8 output clocks, XTAL/CLKIN input)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si5351_variant {
    SI5351_VARIANT_A = 1,
    SI5351_VARIANT_A3 = 2,
    SI5351_VARIANT_B = 3,
    SI5351_VARIANT_C = 4,
}
