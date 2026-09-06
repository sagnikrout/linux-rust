//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/hw_random/cctrng.h
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
// Copyright (C) 2019-2020 ARM Limited or its affiliates.

pub const POWER_DOWN_ENABLE: c_uint = 0x01;
pub const POWER_DOWN_DISABLE: c_uint = 0x00;
// hwrng quality: bits of true entropy per 1024 bits of input
pub const CC_TRNG_QUALITY: c_int = 1024;
// CryptoCell TRNG HW definitions
pub const CC_TRNG_NUM_OF_ROSCS: c_int = 4;
// The number of words generated in the entropy holding register (EHR)
// 6 words (192 bit) according to HW implementation
//
pub const CC_TRNG_EHR_IN_WORDS: c_int = 6;

// RNG interrupt mask

// --------------------------------------
// BLOCK: RNG
// --------------------------------------
pub const CC_RNG_IMR_REG_OFFSET: c_uint = 0x0100UL;
pub const CC_RNG_IMR_EHR_VALID_INT_MASK_BIT_SHIFT: c_uint = 0x0UL;
pub const CC_RNG_IMR_AUTOCORR_ERR_INT_MASK_BIT_SHIFT: c_uint = 0x1UL;
pub const CC_RNG_IMR_CRNGT_ERR_INT_MASK_BIT_SHIFT: c_uint = 0x2UL;
pub const CC_RNG_IMR_VN_ERR_INT_MASK_BIT_SHIFT: c_uint = 0x3UL;
pub const CC_RNG_IMR_WATCHDOG_INT_MASK_BIT_SHIFT: c_uint = 0x4UL;
pub const CC_RNG_ISR_REG_OFFSET: c_uint = 0x0104UL;
pub const CC_RNG_ISR_EHR_VALID_BIT_SHIFT: c_uint = 0x0UL;
pub const CC_RNG_ISR_EHR_VALID_BIT_SIZE: c_uint = 0x1UL;
pub const CC_RNG_ISR_AUTOCORR_ERR_BIT_SHIFT: c_uint = 0x1UL;
pub const CC_RNG_ISR_AUTOCORR_ERR_BIT_SIZE: c_uint = 0x1UL;
pub const CC_RNG_ISR_CRNGT_ERR_BIT_SHIFT: c_uint = 0x2UL;
pub const CC_RNG_ISR_CRNGT_ERR_BIT_SIZE: c_uint = 0x1UL;
pub const CC_RNG_ISR_WATCHDOG_BIT_SHIFT: c_uint = 0x4UL;
pub const CC_RNG_ISR_WATCHDOG_BIT_SIZE: c_uint = 0x1UL;
pub const CC_RNG_ICR_REG_OFFSET: c_uint = 0x0108UL;
pub const CC_TRNG_CONFIG_REG_OFFSET: c_uint = 0x010CUL;
pub const CC_EHR_DATA_0_REG_OFFSET: c_uint = 0x0114UL;
pub const CC_RND_SOURCE_ENABLE_REG_OFFSET: c_uint = 0x012CUL;
pub const CC_SAMPLE_CNT1_REG_OFFSET: c_uint = 0x0130UL;
pub const CC_TRNG_DEBUG_CONTROL_REG_OFFSET: c_uint = 0x0138UL;
pub const CC_RNG_SW_RESET_REG_OFFSET: c_uint = 0x0140UL;
pub const CC_RNG_CLK_ENABLE_REG_OFFSET: c_uint = 0x01C4UL;
pub const CC_RNG_DMA_ENABLE_REG_OFFSET: c_uint = 0x01C8UL;
pub const CC_RNG_WATCHDOG_VAL_REG_OFFSET: c_uint = 0x01D8UL;
// --------------------------------------
// BLOCK: SEC_HOST_RGF
// --------------------------------------
pub const CC_HOST_RGF_IRR_REG_OFFSET: c_uint = 0x0A00UL;
pub const CC_HOST_RGF_IRR_RNG_INT_BIT_SHIFT: c_uint = 0xAUL;
pub const CC_HOST_RGF_IMR_REG_OFFSET: c_uint = 0x0A04UL;
pub const CC_HOST_RGF_ICR_REG_OFFSET: c_uint = 0x0A08UL;
pub const CC_HOST_POWER_DOWN_EN_REG_OFFSET: c_uint = 0x0A78UL;
// --------------------------------------
// BLOCK: NVM
// --------------------------------------
pub const CC_NVM_IS_IDLE_REG_OFFSET: c_uint = 0x0F10UL;
pub const CC_NVM_IS_IDLE_VALUE_BIT_SHIFT: c_uint = 0x0UL;
pub const CC_NVM_IS_IDLE_VALUE_BIT_SIZE: c_uint = 0x1UL;
