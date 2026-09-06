//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/cpucaps.h
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
// Check whether a cpucap is possible at compiletime.
//
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_EPAN) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_SVE) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_SME) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_CNP) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_PTR_AUTH) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_PSEUDO_NMI) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_MTE) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_BTI) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_TLB_RANGE) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_POE) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_GCS) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_HAFT) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_UNMAP_KERNEL_AT_EL0) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_843419) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_1742098) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_2645198) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_2658417) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_CAVIUM_ERRATUM_23154) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_WORKAROUND_DISABLE_CNP) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_WORKAROUND_REPEAT_TLBI_SYNC) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_3194386) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_ERRATUM_4193714) -> return;
}
//
// KVM MPAM support doesn't rely on the host kernel supporting MPAM.
//
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_HW_PERF_EVENTS) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARM64_LSUI) -> return;
}

