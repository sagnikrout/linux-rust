//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/prot_sao.c
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
// Copyright 2016, Michael Ellerman, IBM Corp.
//

#[no_mangle]
pub unsafe extern "C" fn test_prot_sao() -> c_int {
    int test_prot_sao(void)
    {
    char *p;
//
// SAO was introduced in 2.06 and removed in 3.1. It's disabled in
// guests/LPARs by default, so also skip if we are running in a guest.
//
    SKIP_IF(!have_hwcap(PPC_FEATURE_ARCH_2_06) ||
    have_hwcap2(PPC_FEATURE2_ARCH_3_1) ||
    access("/proc/device-tree/rtas/ibm,hypertas-functions", F_OK) == 0);
//
// Ensure we can ask for PROT_SAO.
// We can't really verify that it does the right thing, but at least we
// confirm the kernel will accept it.
//
    p = mmap(core::ptr::null_mut(), SIZE, PROT_READ | PROT_WRITE | PROT_SAO,
    MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    FAIL_IF(p == MAP_FAILED);
// Write to the mapping, to at least cause a fault
    memset(p, 0xaa, SIZE);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_prot_sao, "prot-sao");
    }
