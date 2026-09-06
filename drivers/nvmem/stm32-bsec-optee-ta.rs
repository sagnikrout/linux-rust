//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvmem/stm32-bsec-optee-ta.h
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
// OP-TEE STM32MP BSEC PTA interface, used by STM32 ROMEM driver
//
// Copyright (C) 2022, STMicroelectronics - All Rights Reserved
//

//
// stm32_bsec_optee_ta_open() - initialize the STM32 BSEC TA
// @ctx: the OP-TEE context on success
//
// Return:
// On success, 0. On failure, -errno.
//
extern "C" {
    pub fn stm32_bsec_optee_ta_open(ctx: *mut tee_context) -> c_int;
}
//
// stm32_bsec_optee_ta_close() - release the STM32 BSEC TA
// @ctx: the OP-TEE context
//
// This function used to clean the OP-TEE resources initialized in
// stm32_bsec_optee_ta_open(); it can be used as callback to
// devm_add_action_or_reset()
//
extern "C" {
    pub fn stm32_bsec_optee_ta_close(ctx: *mut c_void);
}
//
// stm32_bsec_optee_ta_read() - nvmem read access using TA client driver
// @ctx: the OP-TEE context provided by stm32_bsec_optee_ta_open
// @offset: nvmem offset
// @buf: buffer to fill with nvem values
// @bytes: number of bytes to read
//
// Return:
// On success, 0. On failure, -errno.
//
// stm32_bsec_optee_ta_write() - nvmem write access using TA client driver
// @ctx: the OP-TEE context provided by stm32_bsec_optee_ta_open
// @lower: number of lower OTP, not protected by ECC
// @offset: nvmem offset
// @buf: buffer with nvem values
// @bytes: number of bytes to write
//
// Return:
// On success, 0. On failure, -errno.
//

