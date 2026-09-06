//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/x86/util/intel-bts.c
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
// intel-bts.c: Intel Processor Trace support
// Copyright (c) 2013-2015, Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_bts_snapshot_ref {
    pub ref_buf: *mut c_void,
    pub ref_offset: usize,
    pub wrapped: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_bts_recording {
    pub itr: auxtrace_record,
    pub intel_bts_pmu: *mut perf_pmu,
    pub evlist: *mut evlist,
    pub snapshot_mode: bool,
    pub snapshot_size: usize,
    pub snapshot_ref_cnt: c_int,
    pub snapshot_refs: *mut intel_bts_snapshot_ref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch {
    pub from: u64,
    pub to: u64,
    pub misc: u64,
}

    static size_t
    intel_bts_info_priv_size(struct auxtrace_record *itr __maybe_unused,
    struct evlist *evlist __maybe_unused)
    {
    return INTEL_BTS_AUXTRACE_PRIV_SIZE;
    }
    static int intel_bts_info_fill(struct auxtrace_record *itr,
    struct perf_session *session,
    struct perf_record_auxtrace_info *auxtrace_info,
    size_t priv_size)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    struct perf_pmu *intel_bts_pmu = btsr.intel_bts_pmu;
    struct perf_event_mmap_page *pc;
    let mut tc: perf_tsc_conversion = { .time_mult = 0, };
    let mut cap_user_time_zero: bool = false;
    int err;
    if (priv_size != INTEL_BTS_AUXTRACE_PRIV_SIZE)
    return -EINVAL;
    if (!evlist__core(session.evlist).nr_mmaps)
    return -EINVAL;
    pc = evlist__mmap(session.evlist)[0].core.base;
    if (pc) {
    err = perf_read_tsc_conversion(pc, &tc);
    if (err) {
    if (err != -EOPNOTSUPP)
    return err;
    } else {
    cap_user_time_zero = tc.time_mult != 0;
    }
    if (!cap_user_time_zero)
    ui__warning("Intel BTS: TSC not available\n");
    }
    auxtrace_info.type = PERF_AUXTRACE_INTEL_BTS;
    auxtrace_info.priv[INTEL_BTS_PMU_TYPE] = intel_bts_pmu.type;
    auxtrace_info.priv[INTEL_BTS_TIME_SHIFT] = tc.time_shift;
    auxtrace_info.priv[INTEL_BTS_TIME_MULT] = tc.time_mult;
    auxtrace_info.priv[INTEL_BTS_TIME_ZERO] = tc.time_zero;
    auxtrace_info.priv[INTEL_BTS_CAP_USER_TIME_ZERO] = cap_user_time_zero;
    auxtrace_info.priv[INTEL_BTS_SNAPSHOT_MODE] = btsr.snapshot_mode;
    return 0;
    }
    static int intel_bts_recording_options(struct auxtrace_record *itr,
    struct evlist *evlist,
    struct record_opts *opts)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    struct perf_pmu *intel_bts_pmu = btsr.intel_bts_pmu;
    struct evsel *evsel, *intel_bts_evsel = core::ptr::null_mut();
    const struct perf_cpu_map *cpus = evlist__core(evlist).user_requested_cpus;
    let mut privileged: bool = perf_event_paranoid_check(-1);
    if (opts.auxtrace_sample_mode) {
    pr_err("Intel BTS does not support AUX area sampling\n");
    return -EINVAL;
    }
    btsr.evlist = evlist;
    btsr.snapshot_mode = opts.auxtrace_snapshot_mode;
    evlist__for_each_entry(evlist, evsel) {
    if (evsel.core.attr.type == intel_bts_pmu.type) {
    if (intel_bts_evsel) {
    pr_err("There may be only one " INTEL_BTS_PMU_NAME " event\n");
    return -EINVAL;
    }
    evsel.core.attr.freq = 0;
    evsel.core.attr.sample_period = 1;
    evsel.needs_auxtrace_mmap = true;
    intel_bts_evsel = evsel;
    opts.full_auxtrace = true;
    }
    }
    if (opts.auxtrace_snapshot_mode && !opts.full_auxtrace) {
    pr_err("Snapshot mode (-S option) requires " INTEL_BTS_PMU_NAME " PMU event (-e " INTEL_BTS_PMU_NAME ")\n");
    return -EINVAL;
    }
    if (!opts.full_auxtrace)
    return 0;
    if (opts.full_auxtrace && !perf_cpu_map__is_any_cpu_or_is_empty(cpus)) {
    pr_err(INTEL_BTS_PMU_NAME " does not support per-cpu recording\n");
    return -EINVAL;
    }
// Set default sizes for snapshot mode
    if (opts.auxtrace_snapshot_mode) {
    if (!opts.auxtrace_snapshot_size && !opts.auxtrace_mmap_pages) {
    if (privileged) {
    opts.auxtrace_mmap_pages = MiB(4) / page_size;
    } else {
    opts.auxtrace_mmap_pages = KiB(128) / page_size;
    if (opts.mmap_pages == UINT_MAX)
    opts.mmap_pages = KiB(256) / page_size;
    }
    } else if (!opts.auxtrace_mmap_pages && !privileged &&
    opts.mmap_pages == UINT_MAX) {
    opts.mmap_pages = KiB(256) / page_size;
    }
    if (!opts.auxtrace_snapshot_size)
    opts.auxtrace_snapshot_size =
    opts.auxtrace_mmap_pages * (size_t)page_size;
    if (!opts.auxtrace_mmap_pages) {
    let mut sz: usize = opts.auxtrace_snapshot_size;
    sz = round_up(sz, page_size) / page_size;
    opts.auxtrace_mmap_pages = roundup_pow_of_two(sz);
    }
    if (opts.auxtrace_snapshot_size >
    opts.auxtrace_mmap_pages * (size_t)page_size) {
    pr_err("Snapshot size %zu must not be greater than AUX area tracing mmap size %zu\n",
    opts.auxtrace_snapshot_size,
    opts.auxtrace_mmap_pages * (size_t)page_size);
    return -EINVAL;
    }
    if (!opts.auxtrace_snapshot_size || !opts.auxtrace_mmap_pages) {
    pr_err("Failed to calculate default snapshot size and/or AUX area tracing mmap pages\n");
    return -EINVAL;
    }
    pr_debug2("Intel BTS snapshot size: %zu\n",
    opts.auxtrace_snapshot_size);
    }
// Set default sizes for full trace mode
    if (opts.full_auxtrace && !opts.auxtrace_mmap_pages) {
    if (privileged) {
    opts.auxtrace_mmap_pages = MiB(4) / page_size;
    } else {
    opts.auxtrace_mmap_pages = KiB(128) / page_size;
    if (opts.mmap_pages == UINT_MAX)
    opts.mmap_pages = KiB(256) / page_size;
    }
    }
// Validate auxtrace_mmap_pages
    if (opts.auxtrace_mmap_pages) {
    let mut sz: usize = opts.auxtrace_mmap_pages * (size_t)page_size;
    size_t min_sz;
    if (opts.auxtrace_snapshot_mode)
    min_sz = KiB(4);
    else
    min_sz = KiB(8);
    if (sz < min_sz || !is_power_of_2(sz)) {
    pr_err("Invalid mmap size for Intel BTS: must be at least %zuKiB and a power of 2\n",
    min_sz / 1024);
    return -EINVAL;
    }
    }
    if (intel_bts_evsel) {
//
// To obtain the auxtrace buffer file descriptor, the auxtrace event
// must come first.
//
    evlist__to_front(evlist, intel_bts_evsel);
//
// In the case of per-cpu mmaps, we need the CPU on the
// AUX event.
//
    if (!perf_cpu_map__is_any_cpu_or_is_empty(cpus))
    evsel__set_sample_bit(intel_bts_evsel, CPU);
    }
// Add dummy event to keep tracking
    if (opts.full_auxtrace) {
    struct evsel *tracking_evsel;
    int err;
    err = parse_event(evlist, "dummy:u");
    if (err)
    return err;
    tracking_evsel = evlist__last(evlist);
    evlist__set_tracking_event(evlist, tracking_evsel);
    tracking_evsel.core.attr.freq = 0;
    tracking_evsel.core.attr.sample_period = 1;
    }
    return 0;
    }
    static int intel_bts_parse_snapshot_options(struct auxtrace_record *itr,
    struct record_opts *opts,
    const char *str)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    let mut snapshot_size: c_ulonglong = 0;
    char *endptr;
    if (str) {
    snapshot_size = strtoull(str, &endptr, 0);
    if (*endptr || snapshot_size > SIZE_MAX)
    return -1;
    }
    opts.auxtrace_snapshot_mode = true;
    opts.auxtrace_snapshot_size = snapshot_size;
    btsr.snapshot_size = snapshot_size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_reference(__maybe_unused: *mut *mut auxtrace_record itr) -> u64 {
    static u64 intel_bts_reference(struct auxtrace_record *itr __maybe_unused)
    {
    return rdtsc();
    }
    static int intel_bts_alloc_snapshot_refs(struct intel_bts_recording *btsr,
    int idx)
    {
    let mut sz: usize = sizeof(struct intel_bts_snapshot_ref);
    let mut cnt: c_int = btsr.snapshot_ref_cnt, new_cnt = cnt * 2;
    struct intel_bts_snapshot_ref *refs;
    if (!new_cnt)
    new_cnt = 16;
    while (new_cnt <= idx)
    new_cnt *= 2;
    refs = calloc(new_cnt, sz);
    if (!refs)
    return -ENOMEM;
    memcpy(refs, btsr.snapshot_refs, cnt * sz);
    btsr.snapshot_refs = refs;
    btsr.snapshot_ref_cnt = new_cnt;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_free_snapshot_refs(btsr: *mut intel_bts_recording) {
    static void intel_bts_free_snapshot_refs(struct intel_bts_recording *btsr)
    {
    int i;
    for (i = 0; i < btsr.snapshot_ref_cnt; i++)
    zfree(&btsr.snapshot_refs[i].ref_buf);
    zfree(&btsr.snapshot_refs);
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_recording_free(itr: *mut auxtrace_record) {
    static void intel_bts_recording_free(struct auxtrace_record *itr)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    intel_bts_free_snapshot_refs(btsr);
    free(btsr);
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_snapshot_start(itr: *mut auxtrace_record) -> c_int {
    static int intel_bts_snapshot_start(struct auxtrace_record *itr)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    struct evsel *evsel;
    evlist__for_each_entry(btsr.evlist, evsel) {
    if (evsel.core.attr.type == btsr.intel_bts_pmu.type)
    return evsel__disable(evsel);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_snapshot_finish(itr: *mut auxtrace_record) -> c_int {
    static int intel_bts_snapshot_finish(struct auxtrace_record *itr)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    struct evsel *evsel;
    evlist__for_each_entry(btsr.evlist, evsel) {
    if (evsel.core.attr.type == btsr.intel_bts_pmu.type)
    return evsel__enable(evsel);
    }
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn intel_bts_first_wrap(data: *mut u64, buf_size: usize) -> bool {
    static bool intel_bts_first_wrap(u64 *data, size_t buf_size)
    {
    int i, a, b;
    b = buf_size >> 3;
    a = b - 512;
    if (a < 0)
    a = 0;
    for (i = a; i < b; i++) {
    if (data[i])
    return true;
    }
    return false;
    }
    static int intel_bts_find_snapshot(struct auxtrace_record *itr, int idx,
    struct auxtrace_mmap *mm, unsigned char *data,
    u64 *head, u64 *old)
    {
    struct intel_bts_recording *btsr =
    container_of(itr, struct intel_bts_recording, itr);
    bool wrapped;
    int err;
    pr_debug3("%s: mmap index %d old head %zu new head %zu\n",
    __func__, idx, (size_t)*old, (size_t)*head);
    if (idx >= btsr.snapshot_ref_cnt) {
    err = intel_bts_alloc_snapshot_refs(btsr, idx);
    if (err)
    goto out_err;
    }
    wrapped = btsr.snapshot_refs[idx].wrapped;
    if (!wrapped && intel_bts_first_wrap((u64 *)data, mm.len)) {
    btsr.snapshot_refs[idx].wrapped = true;
    wrapped = true;
    }
//
// In full trace mode 'head' continually increases.  However in snapshot
// mode 'head' is an offset within the buffer.  Here 'old' and 'head'
// are adjusted to match the full trace case which expects that 'old' is
// always less than 'head'.
//
    if (wrapped) {
// old = *head;
// head += mm->len;
    } else {
    if (mm.mask)
// old &= mm->mask;
    else
// old %= mm->len;
    if (*old > *head)
// head += mm->len;
    }
    pr_debug3("%s: wrap-around %sdetected, adjusted old head %zu adjusted new head %zu\n",
    __func__, wrapped ? "" : "not ", (size_t)*old, (size_t)*head);
    return 0;
    out_err:
    pr_err("%s: failed, error %d\n", __func__, err);
    return err;
    }
    struct auxtrace_record *intel_bts_recording_init(int *err)
    {
    struct perf_pmu *intel_bts_pmu = perf_pmus__find(INTEL_BTS_PMU_NAME);
    struct intel_bts_recording *btsr;
    if (!intel_bts_pmu)
    return core::ptr::null_mut();
    if (setenv("JITDUMP_USE_ARCH_TIMESTAMP", "1", 1)) {
// err = -errno;
    return core::ptr::null_mut();
    }
    btsr = zalloc(sizeof(struct intel_bts_recording));
    if (!btsr) {
// err = -ENOMEM;
    return core::ptr::null_mut();
    }
    btsr.intel_bts_pmu = intel_bts_pmu;
    btsr.itr.recording_options = intel_bts_recording_options;
    btsr.itr.info_priv_size = intel_bts_info_priv_size;
    btsr.itr.info_fill = intel_bts_info_fill;
    btsr.itr.free = intel_bts_recording_free;
    btsr.itr.snapshot_start = intel_bts_snapshot_start;
    btsr.itr.snapshot_finish = intel_bts_snapshot_finish;
    btsr.itr.find_snapshot = intel_bts_find_snapshot;
    btsr.itr.parse_snapshot_options = intel_bts_parse_snapshot_options;
    btsr.itr.reference = intel_bts_reference;
    btsr.itr.read_finish = auxtrace_record__read_finish;
    btsr.itr.alignment = sizeof(struct branch);
    return &btsr.itr;
    }
