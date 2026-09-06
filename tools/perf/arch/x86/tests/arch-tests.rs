//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/tests/arch-tests.c
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

    DEFINE_SUITE("x86 instruction decoder - new instructions", insn_x86);

    static struct test_case intel_pt_tests[] = {
    TEST_CASE("Intel PT packet decoder", intel_pt_pkt_decoder),
    TEST_CASE("Intel PT hybrid CPU compatibility", intel_pt_hybrid_compat),
    { .name = core::ptr::null_mut(), }
    };
    struct test_suite suite__intel_pt = {
    .desc = "Intel PT",
    .test_cases = intel_pt_tests,
    };

    DEFINE_SUITE("x86 bp modify", bp_modify);

    DEFINE_SUITE("AMD IBS via core pmu", amd_ibs_via_core_pmu);
    DEFINE_SUITE_EXCLUSIVE("AMD IBS sample period", amd_ibs_period);
    static struct test_case hybrid_tests[] = {
    TEST_CASE_REASON("x86 hybrid event parsing", hybrid, "not hybrid"),
    { .name = core::ptr::null_mut(), }
    };
    struct test_suite suite__hybrid = {
    .desc = "x86 hybrid",
    .test_cases = hybrid_tests,
    };
    struct test_suite *arch_tests[] = {

    &suite__dwarf_unwind,

    &suite__insn_x86,

    &suite__intel_pt,

    &suite__bp_modify,

    &suite__amd_ibs_via_core_pmu,
    &suite__amd_ibs_period,
    &suite__hybrid,
    &suite__x86_topdown,
    core::ptr::null_mut(),
    };
