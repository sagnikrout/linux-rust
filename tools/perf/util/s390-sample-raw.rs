//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/s390-sample-raw.c
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
// Copyright IBM Corp. 2019
// Author(s): Thomas Richter <tmricht@linux.ibm.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License (version 2 only)
// as published by the Free Software Foundation.
//
// Architecture specific trace_event function. Save event's bc000 raw data
// to file. File name is aux.ctr.## where ## stands for the CPU number the
// sample was taken from.
//

#[no_mangle]
unsafe extern "C" fn ctrset_size(set: *mut cf_ctrset_entry) -> usize {
    static size_t ctrset_size(struct cf_ctrset_entry *set)
    {
    return sizeof(*set) + set.ctr * sizeof(u64);
    }
#[no_mangle]
unsafe extern "C" fn ctrset_valid(set: *mut cf_ctrset_entry) -> bool {
    static bool ctrset_valid(struct cf_ctrset_entry *set)
    {
    return set.def == S390_CPUMCF_DIAG_DEF;
    }
// CPU Measurement Counter Facility raw data is a byte stream. It is 8 byte
// aligned and might have trailing padding bytes.
// Display the raw data on screen.
//
#[no_mangle]
unsafe extern "C" fn s390_cpumcfdg_testctr(sample: *mut perf_sample) -> bool {
    static bool s390_cpumcfdg_testctr(struct perf_sample *sample)
    {
    let mut len: usize = sample.raw_size, offset = 0;
    unsigned char *buf = sample.raw_data;
    struct cf_trailer_entry *te;
    struct cf_ctrset_entry *cep, ce;
    while (offset < len) {
    cep = (struct cf_ctrset_entry *)(buf + offset);
    ce.def = be16_to_cpu(cep.def);
    ce.set = be16_to_cpu(cep.set);
    ce.ctr = be16_to_cpu(cep.ctr);
    ce.res1 = be16_to_cpu(cep.res1);
    if (!ctrset_valid(&ce) || offset + ctrset_size(&ce) > len) {
// Raw data for counter sets are always multiple of 8
// bytes. Prepending a 4 bytes size field to the
// raw data block in the sample causes the perf tool
// to append 4 padding bytes to make the raw data part
// of the sample a multiple of eight bytes again.
//
// If the last entry (trailer) is 4 bytes off the raw
// area data end, all is good.
//
    if (len - offset - sizeof(*te) == 4)
    break;
    pr_err("Invalid counter set entry at %zd\n", offset);
    return false;
    }
    offset += ctrset_size(&ce);
    }
    return true;
    }
// Dump event bc000 on screen, already tested on correctness.
    static void s390_cpumcfdg_dumptrail(const char *color, size_t offset,
    struct cf_trailer_entry *tep)
    {
    struct cf_trailer_entry  te;
    te.flags = be64_to_cpu(tep.flags);
    te.cfvn = be16_to_cpu(tep.cfvn);
    te.csvn = be16_to_cpu(tep.csvn);
    te.cpu_speed = be32_to_cpu(tep.cpu_speed);
    te.timestamp = be64_to_cpu(tep.timestamp);
    te.progusage1 = be64_to_cpu(tep.progusage1);
    te.progusage2 = be64_to_cpu(tep.progusage2);
    te.progusage3 = be64_to_cpu(tep.progusage3);
    te.tod_base = be64_to_cpu(tep.tod_base);
    te.mach_type = be16_to_cpu(tep.mach_type);
    te.res1 = be16_to_cpu(tep.res1);
    te.res2 = be32_to_cpu(tep.res2);
    color_fprintf(stdout, color, "    [%#08zx] Trailer:%c%c%c%c%c"
    " Cfvn:%d Csvn:%d Speed:%d TOD:%#lx\n",
    offset, te.clock_base ? 'T' : ' ',
    te.speed ? 'S' : ' ', te.mtda ? 'M' : ' ',
    te.caca ? 'C' : ' ', te.lcda ? 'L' : ' ',
    te.cfvn, te.csvn, te.cpu_speed, te.timestamp);
    color_fprintf(stdout, color, "\t\t1:%lx 2:%lx 3:%lx TOD-Base:%#lx"
    " Type:%x\n\n",
    te.progusage1, te.progusage2, te.progusage3,
    te.tod_base, te.mach_type);
    }
// Return starting number of a counter set
#[no_mangle]
unsafe extern "C" fn get_counterset_start(setnr: c_int) -> c_int {
    static int get_counterset_start(int setnr)
    {
    switch (setnr) {
    case CPUMF_CTR_SET_BASIC:		/* Basic counter set */
    return 0;
    case CPUMF_CTR_SET_USER:		/* Problem state counter set */
    return 32;
    case CPUMF_CTR_SET_CRYPTO:		/* Crypto counter set */
    return 64;
    case CPUMF_CTR_SET_EXT:			/* Extended counter set */
    return 128;
    case CPUMF_CTR_SET_MT_DIAG:		/* Diagnostic counter set */
    return 448;
    case PERF_EVENT_PAI_NNPA_ALL:		/* PAI NNPA counter set */
    case PERF_EVENT_PAI_CRYPTO_ALL:		/* PAI CRYPTO counter set */
    return setnr;
    default:
    return -1;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct get_counter_name_data {
    pub wanted: c_long,
    pub result: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn get_counter_name_callback(vdata: *mut c_void, info: *mut pmu_event_info) -> c_int {
    static int get_counter_name_callback(void *vdata, struct pmu_event_info *info)
    {
    struct get_counter_name_data *data = vdata;
    int rc, event_nr;
    const char *event_str;
    if (info.str == core::ptr::null_mut())
    return 0;
    event_str = strstr(info.str, "event=");
    if (!event_str)
    return 0;
    rc = sscanf(event_str, "event=%x", &event_nr);
    if (rc == 1 && event_nr == data.wanted) {
    data.result = info.name;
    return 1; /* Terminate the search. */
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_counter_name_hash_fn(key: c_long, __maybe_unused: *mut *mut void ctx) -> usize {
    static size_t get_counter_name_hash_fn(long key, void *ctx __maybe_unused)
    {
    return key;
    }
#[no_mangle]
unsafe extern "C" fn get_counter_name_hashmap_equal_fn(key1: c_long, key2: c_long, __maybe_unused: *mut *mut void ctx) -> bool {
    static bool get_counter_name_hashmap_equal_fn(long key1, long key2, void *ctx __maybe_unused)
    {
    let mut key1: return = = key2;
    }
// Scan the PMU and extract the logical name of a counter from the event. Input
// is the counter set and counter number with in the set. Construct the event
// number and use this as key. If they match return the name of this counter.
// If no match is found a NULL pointer is returned.
//
    static char *get_counter_name(int set, int nr, struct perf_pmu *pmu)
    {
    static struct hashmap *cache;
    static struct perf_pmu *cache_pmu;
    let mut cache_key: c_long = get_counterset_start(set) + nr;
    struct get_counter_name_data data = {
    .wanted = cache_key,
    .result = core::ptr::null_mut(),
    };
    char *result = core::ptr::null_mut();
    if (!pmu)
    return core::ptr::null_mut();
    if (cache_pmu == pmu && hashmap__find(cache, cache_key, &result))
    return strdup(result);
    perf_pmu__for_each_event(pmu, /*skip_duplicate_pmus=*/ true,
    &data, get_counter_name_callback);
    result = strdup(data.result ?: "<unknown>");
    if (cache_pmu == core::ptr::null_mut()) {
    struct hashmap *tmp = hashmap__new(get_counter_name_hash_fn,
    get_counter_name_hashmap_equal_fn,
// ctx=*/NULL);
    if (!IS_ERR(tmp)) {
    cache = tmp;
    cache_pmu = pmu;
    }
    }
    if (cache_pmu == pmu && result) {
    char *old_value = core::ptr::null_mut(), *new_value = strdup(result);
    if (new_value) {
    hashmap__set(cache, cache_key, new_value, /*old_key=*/core::ptr::null_mut(), &old_value);
//
// Free in case of a race, but resizing would be broken
// in that case.
//
    free(old_value);
    }
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn s390_cpumcfdg_dump(sample: *mut perf_sample) {
    static void s390_cpumcfdg_dump(struct perf_sample *sample)
    {
    struct perf_pmu *pmu = sample.evsel.pmu;
    size_t i, len = sample.raw_size, offset = 0;
    unsigned char *buf = sample.raw_data;
    const char *color = PERF_COLOR_BLUE;
    struct cf_ctrset_entry *cep, ce;
    u64 *p;
    while (offset < len) {
    cep = (struct cf_ctrset_entry *)(buf + offset);
    ce.def = be16_to_cpu(cep.def);
    ce.set = be16_to_cpu(cep.set);
    ce.ctr = be16_to_cpu(cep.ctr);
    ce.res1 = be16_to_cpu(cep.res1);
    if (!ctrset_valid(&ce)) {	/* Print trailer */
    s390_cpumcfdg_dumptrail(color, offset,
    (struct cf_trailer_entry *)cep);
    return;
    }
    color_fprintf(stdout, color, "    [%#08zx] Counterset:%d"
    " Counters:%d\n", offset, ce.set, ce.ctr);
    for (i = 0, p = (u64 *)(cep + 1); i < ce.ctr; ++i, ++p) {
    char *ev_name = get_counter_name(ce.set, i, pmu);
    color_fprintf(stdout, color,
    "\tCounter:%03zd %s Value:%#018"PRIx64"\n", i,
    ev_name ?: "<unknown>", be64_to_cpu(*p));
    free(ev_name);
    }
    offset += ctrset_size(&ce);
    }
    }

//
// Check for consistency of PAI_CRYPTO/PAI_NNPA raw data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pai_data {
    pub event_nr: u16,
    pub event_val: u64,
    pub __packed: },

//
// Test for valid raw data. At least one PAI event should be in the raw
// data section.
//
#[no_mangle]
unsafe extern "C" fn s390_pai_all_test(sample: *mut perf_sample) -> bool {
    static bool s390_pai_all_test(struct perf_sample *sample)
    {
    pub sample->raw_size: size_t len =,
    if (len < 0xa)
    pub false: return,
    pub true: return,
    }
#[no_mangle]
unsafe extern "C" fn s390_pai_all_dump(sample: *mut perf_sample) {
    static void s390_pai_all_dump(struct perf_sample *sample)
    {
    pub sample->evsel: *mut *mut evsel evsel =,
    pub 0: size_t len = sample->raw_size, offset =,
    pub sample->raw_data: *mut *mut unsigned char p =,
    pub PERF_COLOR_BLUE: *const *const char color =,
    pub pai_data: pai_data,
    pub ev_name: *mut c_char,
    while (offset < len) {
    pub sizeof(pai_data.event_nr)): memcpy(&pai_data.event_nr, p,,
    pub be16_to_cpu(pai_data.event_nr): pai_data.event_nr =,
    pub sizeof(pai_data.event_nr): p +=,
    pub sizeof(pai_data.event_nr): offset +=,
    pub sizeof(pai_data.event_val)): memcpy(&pai_data.event_val, p,,
    pub be64_to_cpu(pai_data.event_val): pai_data.event_val =,
    pub sizeof(pai_data.event_val): p +=,
    pub sizeof(pai_data.event_val): offset +=,
    ev_name = get_counter_name(evsel.core.attr.config,
    pub evsel->pmu): pai_data.event_nr,,
    color_fprintf(stdout, color, "\tCounter:%03d %s Value:%#018"PRIx64"\n",
    pai_data.event_nr, ev_name ?: "<unknown>",
    if (offset + 0xa > len)
    }
    pub "\n"): color_fprintf(stdout, color,,
    }
// S390 specific trace event function. Check for PERF_RECORD_SAMPLE events
// and if the event was triggered by a
// - counter set diagnostic event
// - processor activity assist (PAI) crypto counter event
// - processor activity assist (PAI) neural network processor assist (NNPA)
// counter event
// display its raw data.
// The function is only invoked when the dump flag -D is set.
//
// Function evlist__s390_sample_raw() is defined as call back after it has
// been verified that the perf.data file was created on s390 platform.
//
    void evlist__s390_sample_raw(struct evlist *evlist, union perf_event *event,
    struct perf_sample *sample)
    {
    pub pai_name: *const c_char,
    if (event.header.type != PERF_RECORD_SAMPLE)
    if (!sample.evsel) {
    pub event): sample->evsel = evlist__event2evsel(evlist,,
    if (!sample.evsel)
    }
// Check for raw data in sample
    if (!sample.raw_size || !sample.raw_data)
// Display raw data on screen
    if (sample.evsel.core.attr.config == PERF_EVENT_CPUM_CF_DIAG) {
    if (!sample.evsel.pmu)
    pub perf_pmus__find("cpum_cf"): sample->evsel->pmu =,
    if (!s390_cpumcfdg_testctr(sample))
    pub encountered\n"): pr_err("Invalid counter set data,
    else
    }
    switch (sample.evsel.core.attr.config) {
    case PERF_EVENT_PAI_NNPA_ALL:
    pub "NNPA_ALL": pai_name =,
    case PERF_EVENT_PAI_CRYPTO_ALL:
    pub "CRYPTO_ALL": pai_name =,
    default:
    }
    if (!s390_pai_all_test(sample)) {
    pub pai_name): pr_err("Invalid %s raw data encountered\n",,
    } else {
    if (!sample.evsel.pmu)
    pub perf_pmus__find_by_type(sample->evsel->core.attr.type): sample->evsel->pmu =,
    }
    }
