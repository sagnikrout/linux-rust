//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/tests/amd-ibs-via-core-pmu.c
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

pub const NR_SUB_TESTS: c_int = 5;
    static struct sub_tests {
    int type;
    unsigned long config;
    bool valid;
    } sub_tests[NR_SUB_TESTS] = {
    { PERF_TYPE_HARDWARE, PERF_COUNT_HW_CPU_CYCLES, true },
    { PERF_TYPE_HARDWARE, PERF_COUNT_HW_INSTRUCTIONS, false },
    { PERF_TYPE_RAW, 0x076, true },
    { PERF_TYPE_RAW, 0x0C1, true },
    { PERF_TYPE_RAW, 0x012, false },
    };
#[no_mangle]
unsafe extern "C" fn event_open(type: c_int, config: c_ulong) -> c_int {
    static int event_open(int type, unsigned long config)
    {
    struct perf_event_attr attr;
    memset(&attr, 0, sizeof(struct perf_event_attr));
    attr.type = type;
    attr.size = sizeof(struct perf_event_attr);
    attr.config = config;
    attr.disabled = 1;
    attr.precise_ip = 1;
    attr.sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_TID;
    attr.sample_period = 100000;
    return sys_perf_event_open(&attr, -1, 0, -1, 0);
    }
    int test__amd_ibs_via_core_pmu(struct test_suite *test __maybe_unused,
    int subtest __maybe_unused)
    {
    struct perf_pmu *ibs_pmu;
    let mut ret: c_int = TEST_OK;
    int fd, i;
    ibs_pmu = perf_pmus__find("ibs_op");
    if (!ibs_pmu)
    return TEST_SKIP;
    for (i = 0; i < NR_SUB_TESTS; i++) {
    fd = event_open(sub_tests[i].type, sub_tests[i].config);
    pr_debug("type: 0x%x, config: 0x%lx, fd: %d  -  ", sub_tests[i].type,
    sub_tests[i].config, fd);
    if ((sub_tests[i].valid && fd == -1) ||
    (!sub_tests[i].valid && fd > 0)) {
    pr_debug("Fail\n");
    ret = TEST_FAIL;
    } else {
    pr_debug("Pass\n");
    }
    if (fd > 0)
    close(fd);
    }
    return ret;
    }
