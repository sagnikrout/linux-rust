//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_irq.h
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
// Copyright (C) 2020-21 Intel Corporation.
//
// ipc_doorbell_fire - fire doorbell to CP
// @ipc_pcie:	Pointer to iosm_pcie
// @irq_n:	Doorbell type
// @data:	ipc state
//
extern "C" {
    pub fn ipc_doorbell_fire(ipc_pcie: *mut iosm_pcie, irq_n: c_int, data: u32);
}
//
// ipc_release_irq - Release the IRQ handler.
// @ipc_pcie:	Pointer to iosm_pcie struct
//
extern "C" {
    pub fn ipc_release_irq(ipc_pcie: *mut iosm_pcie);
}
//
// ipc_acquire_irq - acquire IRQ & register IRQ handler.
// @ipc_pcie:	Pointer to iosm_pcie struct
//
// Return: 0 on success and failure value on error
//
extern "C" {
    pub fn ipc_acquire_irq(ipc_pcie: *mut iosm_pcie) -> c_int;
}
