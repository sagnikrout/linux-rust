//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm/util/pmu.c
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
// Copyright(C) 2015 Linaro Limited. All rights reserved.
// Author: Mathieu Poirier <mathieu.poirier@linaro.org>
//

#[no_mangle]
pub unsafe extern "C" fn perf_pmu__arch_init(pmu: *mut perf_pmu) {
    void perf_pmu__arch_init(struct perf_pmu *pmu)
    {
    struct perf_cpu_map *intersect, *online = cpu_map__online();
    if (!strcmp(pmu.name, CORESIGHT_ETM_PMU_NAME)) {
// add ETM default config here
    pmu.auxtrace = true;
    pmu.selectable = true;
    pmu.perf_event_attr_init_default = cs_etm_get_default_config;

    } else if (strstarts(pmu.name, ARM_SPE_PMU_NAME)) {
    pmu.auxtrace = true;
    pmu.selectable = true;
    pmu.is_uncore = false;
    pmu.perf_event_attr_init_default = arm_spe_pmu_default_config;
    if (strstarts(pmu.name, "arm_spe_"))
    pmu.mem_events = perf_mem_events_arm;
    } else if (strstarts(pmu.name, HISI_PTT_PMU_NAME)) {
    pmu.auxtrace = true;
    pmu.selectable = true;

    }
// Workaround some ARM PMU's failing to correctly set CPU maps for online processors.
    intersect = perf_cpu_map__intersect(online, pmu.cpus);
    perf_cpu_map__put(online);
    perf_cpu_map__put(pmu.cpus);
    pmu.cpus = intersect;
    }
