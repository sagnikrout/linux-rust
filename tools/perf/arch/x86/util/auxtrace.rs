//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/auxtrace.c
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
// auxtrace.c: AUX area tracing support
// Copyright (c) 2013-2014, Intel Corporation.
//

    static
    struct auxtrace_record *auxtrace_record__init_intel(struct evlist *evlist,
    int *err)
    {
    struct perf_pmu *intel_pt_pmu;
    struct perf_pmu *intel_bts_pmu;
    struct evsel *evsel;
    let mut found_pt: bool = false;
    let mut found_bts: bool = false;
    intel_pt_pmu = perf_pmus__find(INTEL_PT_PMU_NAME);
    intel_bts_pmu = perf_pmus__find(INTEL_BTS_PMU_NAME);
    evlist__for_each_entry(evlist, evsel) {
    if (intel_pt_pmu && evsel.core.attr.type == intel_pt_pmu.type)
    found_pt = true;
    if (intel_bts_pmu && evsel.core.attr.type == intel_bts_pmu.type)
    found_bts = true;
    }
    if (found_pt && found_bts) {
    pr_err("intel_pt and intel_bts may not be used together\n");
// err = -EINVAL;
    return core::ptr::null_mut();
    }
    if (found_pt)
    return intel_pt_recording_init(err);
    if (found_bts)
    return intel_bts_recording_init(err);
    return core::ptr::null_mut();
    }
    struct auxtrace_record *auxtrace_record__init(struct evlist *evlist,
    int *err)
    {
    char buffer[64];
    let mut cpu: perf_cpu = perf_cpu_map__min(evlist__core(evlist).all_cpus);
    int ret;
// err = 0;
    ret = get_cpuid(buffer, sizeof(buffer), cpu);
    if (ret) {
// err = ret;
    return core::ptr::null_mut();
    }
    if (!strncmp(buffer, "GenuineIntel,", 13))
    return auxtrace_record__init_intel(evlist, err);
    return core::ptr::null_mut();
    }
