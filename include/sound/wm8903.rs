//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/wm8903.h
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
// linux/sound/wm8903.h -- Platform data for WM8903
//
// Copyright 2010 Wolfson Microelectronics. PLC.
//
// Used to enable configuration of a GPIO to all zeros; a gpio_cfg value of
// zero in platform data means "don't touch this pin".
//
pub const WM8903_GPIO_CONFIG_ZERO: c_uint = 0x8000;
//
// R6 (0x06) - Mic Bias Control 0
//
pub const WM8903_MICDET_THR_MASK: c_uint = 0x0030  /* MICDET_THR - [5:4] */;

pub const WM8903_MICSHORT_THR_MASK: c_uint = 0x000C  /* MICSHORT_THR - [3:2] */;

pub const WM8903_MICDET_ENA: c_uint = 0x0002  /* MICDET_ENA */;
pub const WM8903_MICDET_ENA_MASK: c_uint = 0x0002  /* MICDET_ENA */;

pub const WM8903_MICBIAS_ENA: c_uint = 0x0001  /* MICBIAS_ENA */;
pub const WM8903_MICBIAS_ENA_MASK: c_uint = 0x0001  /* MICBIAS_ENA */;

//
// WM8903_GPn_FN values
//
// See datasheets for list of valid values per pin
//
pub const WM8903_GPn_FN_GPIO_OUTPUT: c_int = 0;
pub const WM8903_GPn_FN_BCLK: c_int = 1;
pub const WM8903_GPn_FN_IRQ_OUTPT: c_int = 2;
pub const WM8903_GPn_FN_GPIO_INPUT: c_int = 3;
pub const WM8903_GPn_FN_MICBIAS_CURRENT_DETECT: c_int = 4;
pub const WM8903_GPn_FN_MICBIAS_SHORT_DETECT: c_int = 5;
pub const WM8903_GPn_FN_DMIC_LR_CLK_OUTPUT: c_int = 6;
pub const WM8903_GPn_FN_FLL_LOCK_OUTPUT: c_int = 8;
pub const WM8903_GPn_FN_FLL_CLOCK_OUTPUT: c_int = 9;
//
// R116 (0x74) - GPIO Control 1
//
pub const WM8903_GP1_FN_MASK: c_uint = 0x1F00  /* GP1_FN - [12:8] */;

pub const WM8903_GP1_DIR: c_uint = 0x0080  /* GP1_DIR */;
pub const WM8903_GP1_DIR_MASK: c_uint = 0x0080  /* GP1_DIR */;

pub const WM8903_GP1_OP_CFG: c_uint = 0x0040  /* GP1_OP_CFG */;
pub const WM8903_GP1_OP_CFG_MASK: c_uint = 0x0040  /* GP1_OP_CFG */;

pub const WM8903_GP1_IP_CFG: c_uint = 0x0020  /* GP1_IP_CFG */;
pub const WM8903_GP1_IP_CFG_MASK: c_uint = 0x0020  /* GP1_IP_CFG */;

pub const WM8903_GP1_LVL: c_uint = 0x0010  /* GP1_LVL */;
pub const WM8903_GP1_LVL_MASK: c_uint = 0x0010  /* GP1_LVL */;

pub const WM8903_GP1_PD: c_uint = 0x0008  /* GP1_PD */;
pub const WM8903_GP1_PD_MASK: c_uint = 0x0008  /* GP1_PD */;

pub const WM8903_GP1_PU: c_uint = 0x0004  /* GP1_PU */;
pub const WM8903_GP1_PU_MASK: c_uint = 0x0004  /* GP1_PU */;

pub const WM8903_GP1_INTMODE: c_uint = 0x0002  /* GP1_INTMODE */;
pub const WM8903_GP1_INTMODE_MASK: c_uint = 0x0002  /* GP1_INTMODE */;

pub const WM8903_GP1_DB: c_uint = 0x0001  /* GP1_DB */;
pub const WM8903_GP1_DB_MASK: c_uint = 0x0001  /* GP1_DB */;

//
// R117 (0x75) - GPIO Control 2
//
pub const WM8903_GP2_FN_MASK: c_uint = 0x1F00  /* GP2_FN - [12:8] */;

pub const WM8903_GP2_DIR: c_uint = 0x0080  /* GP2_DIR */;
pub const WM8903_GP2_DIR_MASK: c_uint = 0x0080  /* GP2_DIR */;

pub const WM8903_GP2_OP_CFG: c_uint = 0x0040  /* GP2_OP_CFG */;
pub const WM8903_GP2_OP_CFG_MASK: c_uint = 0x0040  /* GP2_OP_CFG */;

pub const WM8903_GP2_IP_CFG: c_uint = 0x0020  /* GP2_IP_CFG */;
pub const WM8903_GP2_IP_CFG_MASK: c_uint = 0x0020  /* GP2_IP_CFG */;

pub const WM8903_GP2_LVL: c_uint = 0x0010  /* GP2_LVL */;
pub const WM8903_GP2_LVL_MASK: c_uint = 0x0010  /* GP2_LVL */;

pub const WM8903_GP2_PD: c_uint = 0x0008  /* GP2_PD */;
pub const WM8903_GP2_PD_MASK: c_uint = 0x0008  /* GP2_PD */;

pub const WM8903_GP2_PU: c_uint = 0x0004  /* GP2_PU */;
pub const WM8903_GP2_PU_MASK: c_uint = 0x0004  /* GP2_PU */;

pub const WM8903_GP2_INTMODE: c_uint = 0x0002  /* GP2_INTMODE */;
pub const WM8903_GP2_INTMODE_MASK: c_uint = 0x0002  /* GP2_INTMODE */;

pub const WM8903_GP2_DB: c_uint = 0x0001  /* GP2_DB */;
pub const WM8903_GP2_DB_MASK: c_uint = 0x0001  /* GP2_DB */;

//
// R118 (0x76) - GPIO Control 3
//
pub const WM8903_GP3_FN_MASK: c_uint = 0x1F00  /* GP3_FN - [12:8] */;

pub const WM8903_GP3_DIR: c_uint = 0x0080  /* GP3_DIR */;
pub const WM8903_GP3_DIR_MASK: c_uint = 0x0080  /* GP3_DIR */;

pub const WM8903_GP3_OP_CFG: c_uint = 0x0040  /* GP3_OP_CFG */;
pub const WM8903_GP3_OP_CFG_MASK: c_uint = 0x0040  /* GP3_OP_CFG */;

pub const WM8903_GP3_IP_CFG: c_uint = 0x0020  /* GP3_IP_CFG */;
pub const WM8903_GP3_IP_CFG_MASK: c_uint = 0x0020  /* GP3_IP_CFG */;

pub const WM8903_GP3_LVL: c_uint = 0x0010  /* GP3_LVL */;
pub const WM8903_GP3_LVL_MASK: c_uint = 0x0010  /* GP3_LVL */;

pub const WM8903_GP3_PD: c_uint = 0x0008  /* GP3_PD */;
pub const WM8903_GP3_PD_MASK: c_uint = 0x0008  /* GP3_PD */;

pub const WM8903_GP3_PU: c_uint = 0x0004  /* GP3_PU */;
pub const WM8903_GP3_PU_MASK: c_uint = 0x0004  /* GP3_PU */;

pub const WM8903_GP3_INTMODE: c_uint = 0x0002  /* GP3_INTMODE */;
pub const WM8903_GP3_INTMODE_MASK: c_uint = 0x0002  /* GP3_INTMODE */;

pub const WM8903_GP3_DB: c_uint = 0x0001  /* GP3_DB */;
pub const WM8903_GP3_DB_MASK: c_uint = 0x0001  /* GP3_DB */;

//
// R119 (0x77) - GPIO Control 4
//
pub const WM8903_GP4_FN_MASK: c_uint = 0x1F00  /* GP4_FN - [12:8] */;

pub const WM8903_GP4_DIR: c_uint = 0x0080  /* GP4_DIR */;
pub const WM8903_GP4_DIR_MASK: c_uint = 0x0080  /* GP4_DIR */;

pub const WM8903_GP4_OP_CFG: c_uint = 0x0040  /* GP4_OP_CFG */;
pub const WM8903_GP4_OP_CFG_MASK: c_uint = 0x0040  /* GP4_OP_CFG */;

pub const WM8903_GP4_IP_CFG: c_uint = 0x0020  /* GP4_IP_CFG */;
pub const WM8903_GP4_IP_CFG_MASK: c_uint = 0x0020  /* GP4_IP_CFG */;

pub const WM8903_GP4_LVL: c_uint = 0x0010  /* GP4_LVL */;
pub const WM8903_GP4_LVL_MASK: c_uint = 0x0010  /* GP4_LVL */;

pub const WM8903_GP4_PD: c_uint = 0x0008  /* GP4_PD */;
pub const WM8903_GP4_PD_MASK: c_uint = 0x0008  /* GP4_PD */;

pub const WM8903_GP4_PU: c_uint = 0x0004  /* GP4_PU */;
pub const WM8903_GP4_PU_MASK: c_uint = 0x0004  /* GP4_PU */;

pub const WM8903_GP4_INTMODE: c_uint = 0x0002  /* GP4_INTMODE */;
pub const WM8903_GP4_INTMODE_MASK: c_uint = 0x0002  /* GP4_INTMODE */;

pub const WM8903_GP4_DB: c_uint = 0x0001  /* GP4_DB */;
pub const WM8903_GP4_DB_MASK: c_uint = 0x0001  /* GP4_DB */;

//
// R120 (0x78) - GPIO Control 5
//
pub const WM8903_GP5_FN_MASK: c_uint = 0x1F00  /* GP5_FN - [12:8] */;

pub const WM8903_GP5_DIR: c_uint = 0x0080  /* GP5_DIR */;
pub const WM8903_GP5_DIR_MASK: c_uint = 0x0080  /* GP5_DIR */;

pub const WM8903_GP5_OP_CFG: c_uint = 0x0040  /* GP5_OP_CFG */;
pub const WM8903_GP5_OP_CFG_MASK: c_uint = 0x0040  /* GP5_OP_CFG */;

pub const WM8903_GP5_IP_CFG: c_uint = 0x0020  /* GP5_IP_CFG */;
pub const WM8903_GP5_IP_CFG_MASK: c_uint = 0x0020  /* GP5_IP_CFG */;

pub const WM8903_GP5_LVL: c_uint = 0x0010  /* GP5_LVL */;
pub const WM8903_GP5_LVL_MASK: c_uint = 0x0010  /* GP5_LVL */;

pub const WM8903_GP5_PD: c_uint = 0x0008  /* GP5_PD */;
pub const WM8903_GP5_PD_MASK: c_uint = 0x0008  /* GP5_PD */;

pub const WM8903_GP5_PU: c_uint = 0x0004  /* GP5_PU */;
pub const WM8903_GP5_PU_MASK: c_uint = 0x0004  /* GP5_PU */;

pub const WM8903_GP5_INTMODE: c_uint = 0x0002  /* GP5_INTMODE */;
pub const WM8903_GP5_INTMODE_MASK: c_uint = 0x0002  /* GP5_INTMODE */;

pub const WM8903_GP5_DB: c_uint = 0x0001  /* GP5_DB */;
pub const WM8903_GP5_DB_MASK: c_uint = 0x0001  /* GP5_DB */;

pub const WM8903_NUM_GPIO: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8903_platform_data {
    pub /: *mut *mut bool irq_active_low; / Set if IRQ active low, default high,
// Default register value for R6 (Mic bias), used to configure
// microphone detection.  In conjunction with gpio_cfg this
// can be used to route the microphone status signals out onto
// the GPIOs for use with snd_soc_jack_add_gpios().
//
    pub micdet_cfg: u16,
    pub /: *mut *mut int micdet_delay; / Delay after microphone detection (ms),
    pub gpio_base: c_int,
    pub /: *mut *mut u32 gpio_cfg[WM8903_NUM_GPIO]; / Default register values for GPIO pin mux,
}
