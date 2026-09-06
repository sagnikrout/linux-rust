//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm/util/auxtrace.c
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

    static struct perf_pmu **find_all_arm_spe_pmus(int *nr_spes, int *err)
    {
    struct perf_pmu **arm_spe_pmus = core::ptr::null_mut();
    int ret, i, nr_cpus = sysconf(_SC_NPROCESSORS_CONF);
// arm_spe_xxxxxxxxx\0
    char arm_spe_pmu_name[sizeof(ARM_SPE_PMU_NAME) + 10];
    arm_spe_pmus = calloc(nr_cpus, sizeof(struct perf_pmu *));
    if (!arm_spe_pmus) {
    pr_err("spes alloc failed\n");
// err = -ENOMEM;
    return core::ptr::null_mut();
    }
    for (i = 0; i < nr_cpus; i++) {
    ret = sprintf(arm_spe_pmu_name, "%s%d", ARM_SPE_PMU_NAME, i);
    if (ret < 0) {
    pr_err("sprintf failed\n");
// err = -ENOMEM;
    return core::ptr::null_mut();
    }
    arm_spe_pmus[*nr_spes] = perf_pmus__find(arm_spe_pmu_name);
    if (arm_spe_pmus[*nr_spes]) {
    pr_debug2("%s %d: arm_spe_pmu %d type %d name %s\n",
    __func__, __LINE__, *nr_spes,
    arm_spe_pmus[*nr_spes].type,
    arm_spe_pmus[*nr_spes].name);
    (*nr_spes)++;
    }
    }
    return arm_spe_pmus;
    }
    static struct perf_pmu **find_all_hisi_ptt_pmus(int *nr_ptts, int *err)
    {
    struct perf_pmu **hisi_ptt_pmus = core::ptr::null_mut();
    struct dirent *dent;
    char path[PATH_MAX];
    DIR *dir = core::ptr::null_mut();
    let mut idx: c_int = 0;
    perf_pmu__event_source_devices_scnprintf(path, sizeof(path));
    dir = opendir(path);
    if (!dir) {
    pr_err("can't read directory '%s'\n", path);
// err = -EINVAL;
    return core::ptr::null_mut();
    }
    while ((dent = readdir(dir))) {
    if (strstr(dent.d_name, HISI_PTT_PMU_NAME))
    (*nr_ptts)++;
    }
    if (!(*nr_ptts))
    goto out;
    hisi_ptt_pmus = calloc((*nr_ptts), sizeof(struct perf_pmu *));
    if (!hisi_ptt_pmus) {
    pr_err("hisi_ptt alloc failed\n");
// err = -ENOMEM;
    goto out;
    }
    rewinddir(dir);
    while ((dent = readdir(dir))) {
    if (strstr(dent.d_name, HISI_PTT_PMU_NAME) && idx < *nr_ptts) {
    hisi_ptt_pmus[idx] = perf_pmus__find(dent.d_name);
    if (hisi_ptt_pmus[idx])
    idx++;
    }
    }
    out:
    closedir(dir);
    return hisi_ptt_pmus;
    }
    static struct perf_pmu *find_pmu_for_event(struct perf_pmu **pmus,
    int pmu_nr, struct evsel *evsel)
    {
    int i;
    if (!pmus)
    return core::ptr::null_mut();
    for (i = 0; i < pmu_nr; i++) {
    if (evsel.core.attr.type == pmus[i].type)
    return pmus[i];
    }
    return core::ptr::null_mut();
    }
    struct auxtrace_record
// auxtrace_record__init(struct evlist *evlist, int *err)
    {
    struct perf_pmu	*cs_etm_pmu = core::ptr::null_mut();
    struct perf_pmu **arm_spe_pmus = core::ptr::null_mut();
    struct perf_pmu **hisi_ptt_pmus = core::ptr::null_mut();
    struct evsel *evsel;
    struct perf_pmu *found_etm = core::ptr::null_mut();
    struct perf_pmu *found_spe = core::ptr::null_mut();
    struct perf_pmu *found_ptt = core::ptr::null_mut();
    let mut auxtrace_event_cnt: c_int = 0;
    let mut nr_spes: c_int = 0;
    let mut nr_ptts: c_int = 0;
    if (!evlist)
    return core::ptr::null_mut();
    cs_etm_pmu = perf_pmus__find(CORESIGHT_ETM_PMU_NAME);
    arm_spe_pmus = find_all_arm_spe_pmus(&nr_spes, err);
    hisi_ptt_pmus = find_all_hisi_ptt_pmus(&nr_ptts, err);
    evlist__for_each_entry(evlist, evsel) {
    if (cs_etm_pmu && !found_etm)
    found_etm = find_pmu_for_event(&cs_etm_pmu, 1, evsel);
    if (arm_spe_pmus && !found_spe)
    found_spe = find_pmu_for_event(arm_spe_pmus, nr_spes, evsel);
    if (hisi_ptt_pmus && !found_ptt)
    found_ptt = find_pmu_for_event(hisi_ptt_pmus, nr_ptts, evsel);
    }
    free(arm_spe_pmus);
    free(hisi_ptt_pmus);
    if (found_etm)
    auxtrace_event_cnt++;
    if (found_spe)
    auxtrace_event_cnt++;
    if (found_ptt)
    auxtrace_event_cnt++;
    if (auxtrace_event_cnt > 1) {
    pr_err("Concurrent AUX trace operation not currently supported\n");
// err = -EOPNOTSUPP;
    return core::ptr::null_mut();
    }
    if (found_etm)
    return cs_etm_record_init(err);

    if (found_spe)
    return arm_spe_recording_init(err, found_spe);
    if (found_ptt)
    return hisi_ptt_recording_init(err, found_ptt);

//
// Clear 'err' even if we haven't found an event - that way perf
// record can still be used even if tracers aren't present.  The NULL
// return value will take care of telling the infrastructure HW tracing
// isn't available.
//
// err = 0;
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn compat_auxtrace_mmap__read_head(mm: *mut auxtrace_mmap) -> u64 {
    u64 compat_auxtrace_mmap__read_head(struct auxtrace_mmap *mm)
    {
    struct perf_event_mmap_page *pc = mm.userpg;
    u64 result;
    __asm__ __volatile__(
    "	ldrd    %0, %H0, [%1]"
    : "=&r" (result)
    : "r" (&pc.aux_head), "Qo" (pc.aux_head)
    );
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_auxtrace_mmap__write_tail(mm: *mut auxtrace_mmap, tail: u64) -> c_int {
    int compat_auxtrace_mmap__write_tail(struct auxtrace_mmap *mm, u64 tail)
    {
    struct perf_event_mmap_page *pc = mm.userpg;
// Ensure all reads are done before we write the tail out
    smp_mb();
    __asm__ __volatile__(
    "	strd    %2, %H2, [%1]"
    : "=Qo" (pc.aux_tail)
    : "r" (&pc.aux_tail), "r" (tail)
    );
    return 0;
    }
