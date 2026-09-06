//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/sve-probe-vls.c
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
// Copyright (C) 2015-2020 ARM Limited.
// Original author: Dave Martin <Dave.Martin@arm.com>
//

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    unsigned int vq;
    int vl;
    static unsigned int vqs[SVE_VQ_MAX];
    let mut nvqs: c_uint = 0;
    ksft_print_header();
    ksft_set_plan(2);
    if (!(getauxval(AT_HWCAP) & HWCAP_SVE))
    ksft_exit_skip("SVE not available\n");
//
// Enumerate up to SVE_VQ_MAX vector lengths
//
    for (vq = SVE_VQ_MAX; vq > 0; --vq) {
    vl = prctl(PR_SVE_SET_VL, vq * 16);
    if (vl == -1)
    ksft_exit_fail_msg("PR_SVE_SET_VL failed: %s (%d)\n",
    strerror(errno), errno);
    vl &= PR_SVE_VL_LEN_MASK;
    if (rdvl_sve() != vl)
    ksft_exit_fail_msg("PR_SVE_SET_VL reports %d, RDVL %d\n",
    vl, rdvl_sve());
    if (!sve_vl_valid(vl))
    ksft_exit_fail_msg("VL %d invalid\n", vl);
    vq = sve_vq_from_vl(vl);
    if (!(nvqs < SVE_VQ_MAX))
    ksft_exit_fail_msg("Too many VLs %u >= SVE_VQ_MAX\n",
    nvqs);
    vqs[nvqs++] = vq;
    }
    ksft_test_result_pass("Enumerated %d vector lengths\n", nvqs);
    ksft_test_result_pass("All vector lengths valid\n");
// Print out the vector lengths in ascending order:
    while (nvqs--)
    ksft_print_msg("%u\n", 16 * vqs[nvqs]);
    ksft_exit_pass();
    }
