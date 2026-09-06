//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/phy/phy_qmath.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

extern "C" {
    pub fn qm_mulu16(op1: u16, op2: u16) -> u16;
}
extern "C" {
    pub fn qm_muls16(op1: i16, op2: i16) -> i16;
}
extern "C" {
    pub fn qm_add32(op1: i32, op2: i32) -> i32;
}
extern "C" {
    pub fn qm_add16(op1: i16, op2: i16) -> i16;
}
extern "C" {
    pub fn qm_sub16(op1: i16, op2: i16) -> i16;
}
extern "C" {
    pub fn qm_shl32(op: i32, shift: c_int) -> i32;
}
extern "C" {
    pub fn qm_shl16(op: i16, shift: c_int) -> i16;
}
extern "C" {
    pub fn qm_shr16(op: i16, shift: c_int) -> i16;
}
extern "C" {
    pub fn qm_norm32(op: i32) -> i16;
}
extern "C" {
    pub fn qm_log10(N: i32, qN: i16, log10N: *mut i16, qLog10N: *mut i16);
}
