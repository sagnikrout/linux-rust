//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/s390/util/pmu.c
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
// Copyright IBM Corp. 2023
// Author(s): Thomas Richter <tmricht@linux.ibm.com>
//

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    void perf_pmu__arch_init(struct perf_pmu *pmu)
    {
    if (!strcmp(pmu.name, S390_PMUPAI_CRYPTO) ||
    !strcmp(pmu.name, S390_PMUPAI_EXT) ||
    !strcmp(pmu.name, S390_PMUCPUM_CF))
    pmu.selectable = true;
    }
