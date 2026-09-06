//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/exynos/exynos-pinctrl.h
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
// Samsung Exynos DTS pinctrl constants
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
// http://www.samsung.com
// Copyright (c) 2022 Linaro Ltd
// Author: Krzysztof Kozlowski <krzk@kernel.org>
//
pub const EXYNOS_PIN_PULL_NONE: c_int = 0;
pub const EXYNOS_PIN_PULL_DOWN: c_int = 1;
pub const EXYNOS_PIN_PULL_UP: c_int = 3;
// Pin function in power down mode
pub const EXYNOS_PIN_PDN_OUT0: c_int = 0;
pub const EXYNOS_PIN_PDN_OUT1: c_int = 1;
pub const EXYNOS_PIN_PDN_INPUT: c_int = 2;
pub const EXYNOS_PIN_PDN_PREV: c_int = 3;
//
// Drive strengths for Exynos5410, Exynos542x, Exynos5800, Exynos7885, Exynos850
// (except GPIO_HSI block), ExynosAutov9 (FSI0, PERIC1)
//
pub const EXYNOS5420_PIN_DRV_LV1: c_int = 0;
pub const EXYNOS5420_PIN_DRV_LV2: c_int = 1;
pub const EXYNOS5420_PIN_DRV_LV3: c_int = 2;
pub const EXYNOS5420_PIN_DRV_LV4: c_int = 3;
// Drive strengths for Exynos5433
pub const EXYNOS5433_PIN_DRV_FAST_SR1: c_int = 0;
pub const EXYNOS5433_PIN_DRV_FAST_SR2: c_int = 1;
pub const EXYNOS5433_PIN_DRV_FAST_SR3: c_int = 2;
pub const EXYNOS5433_PIN_DRV_FAST_SR4: c_int = 3;
pub const EXYNOS5433_PIN_DRV_FAST_SR5: c_int = 4;
pub const EXYNOS5433_PIN_DRV_FAST_SR6: c_int = 5;
pub const EXYNOS5433_PIN_DRV_SLOW_SR1: c_int = 8;
pub const EXYNOS5433_PIN_DRV_SLOW_SR2: c_int = 9;
pub const EXYNOS5433_PIN_DRV_SLOW_SR3: c_uint = 0xa;
pub const EXYNOS5433_PIN_DRV_SLOW_SR4: c_uint = 0xb;
pub const EXYNOS5433_PIN_DRV_SLOW_SR5: c_uint = 0xc;
pub const EXYNOS5433_PIN_DRV_SLOW_SR6: c_uint = 0xf;
// Drive strengths for Exynos7 (except FSYS1)
pub const EXYNOS7_PIN_DRV_LV1: c_int = 0;
pub const EXYNOS7_PIN_DRV_LV2: c_int = 2;
pub const EXYNOS7_PIN_DRV_LV3: c_int = 1;
pub const EXYNOS7_PIN_DRV_LV4: c_int = 3;
// Drive strengths for Exynos7 FSYS1 block
pub const EXYNOS7_FSYS1_PIN_DRV_LV1: c_int = 0;
pub const EXYNOS7_FSYS1_PIN_DRV_LV2: c_int = 4;
pub const EXYNOS7_FSYS1_PIN_DRV_LV3: c_int = 2;
pub const EXYNOS7_FSYS1_PIN_DRV_LV4: c_int = 6;
pub const EXYNOS7_FSYS1_PIN_DRV_LV5: c_int = 1;
pub const EXYNOS7_FSYS1_PIN_DRV_LV6: c_int = 5;
// Drive strengths for Exynos850 GPIO_HSI block

pub const EXYNOS_PIN_FUNC_INPUT: c_int = 0;
pub const EXYNOS_PIN_FUNC_OUTPUT: c_int = 1;
pub const EXYNOS_PIN_FUNC_2: c_int = 2;
pub const EXYNOS_PIN_FUNC_3: c_int = 3;
pub const EXYNOS_PIN_FUNC_4: c_int = 4;
pub const EXYNOS_PIN_FUNC_5: c_int = 5;
pub const EXYNOS_PIN_FUNC_6: c_int = 6;
pub const EXYNOS_PIN_FUNC_EINT: c_uint = 0xf;

