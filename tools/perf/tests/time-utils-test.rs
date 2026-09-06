//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/time-utils-test.c
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

#[no_mangle]
unsafe extern "C" fn test__parse_nsec_time(str: *const c_char, expected: u64) -> bool {
    static bool test__parse_nsec_time(const char *str, u64 expected)
    {
    u64 ptime;
    int err;
    pr_debug("\nparse_nsec_time(\"%s\")\n", str);
    err = parse_nsec_time(str, &ptime);
    if (err) {
    pr_debug("error %d\n", err);
    return false;
    }
    if (ptime != expected) {
    pr_debug("Failed. ptime %" PRIu64 " expected %" PRIu64 "\n",
    ptime, expected);
    return false;
    }
    pr_debug("%" PRIu64 "\n", ptime);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn test__perf_time__parse_str(ostr: *const c_char, start: u64, end: u64) -> bool {
    static bool test__perf_time__parse_str(const char *ostr, u64 start, u64 end)
    {
    struct perf_time_interval ptime;
    int err;
    pr_debug("\nperf_time__parse_str(\"%s\")\n", ostr);
    err = perf_time__parse_str(&ptime, ostr);
    if (err) {
    pr_debug("Error %d\n", err);
    return false;
    }
    if (ptime.start != start || ptime.end != end) {
    pr_debug("Failed. Expected %" PRIu64 " to %" PRIu64 "\n",
    start, end);
    return false;
    }
    return true;
    }
pub const TEST_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub str: *const c_char,
    pub first: u64,
    pub last: u64,
    pub ptime: [perf_time_interval; TEST_MAX],
    pub num: c_int,
    pub skip: [u64; TEST_MAX],
    pub noskip: [u64; TEST_MAX],
}

#[no_mangle]
unsafe extern "C" fn test__perf_time__parse_for_ranges(d: *mut test_data) -> bool {
    static bool test__perf_time__parse_for_ranges(struct test_data *d)
    {
    struct evlist *evlist = evlist__new();
    let mut session: perf_session = { .evlist = evlist };
    struct perf_time_interval *ptime = core::ptr::null_mut();
    int range_size, range_num;
    let mut pass: bool = false;
    int i, err;
    if (!evlist) {
    pr_debug("Missing evlist\n");
    return false;
    }
    evlist__set_first_sample_time(evlist, d.first);
    evlist__set_last_sample_time(evlist, d.last);
    pr_debug("\nperf_time__parse_for_ranges(\"%s\")\n", d.str);
    if (strchr(d.str, '%'))
    pr_debug("first_sample_time %" PRIu64 " last_sample_time %" PRIu64 "\n",
    d.first, d.last);
    err = perf_time__parse_for_ranges(d.str, &session, &ptime, &range_size,
    &range_num);
    if (err) {
    pr_debug("error %d\n", err);
    goto out;
    }
    if (range_size < d.num || range_num != d.num) {
    pr_debug("bad size: range_size %d range_num %d expected num %d\n",
    range_size, range_num, d.num);
    goto out;
    }
    for (i = 0; i < d.num; i++) {
    if (ptime[i].start != d.ptime[i].start ||
    ptime[i].end != d.ptime[i].end) {
    pr_debug("bad range %d expected %" PRIu64 " to %" PRIu64 "\n",
    i, d.ptime[i].start, d.ptime[i].end);
    goto out;
    }
    }
    if (perf_time__ranges_skip_sample(ptime, d.num, 0)) {
    pr_debug("failed to keep 0\n");
    goto out;
    }
    for (i = 0; i < TEST_MAX; i++) {
    if (d.skip[i] &&
    !perf_time__ranges_skip_sample(ptime, d.num, d.skip[i])) {
    pr_debug("failed to skip %" PRIu64 "\n", d.skip[i]);
    goto out;
    }
    if (d.noskip[i] &&
    perf_time__ranges_skip_sample(ptime, d.num, d.noskip[i])) {
    pr_debug("failed to keep %" PRIu64 "\n", d.noskip[i]);
    goto out;
    }
    }
    pass = true;
    out:
    evlist__put(evlist);
    free(ptime);
    return pass;
    }
#[no_mangle]
unsafe extern "C" fn test__time_utils(__maybe_unused: *mut *mut test_suite t, __maybe_unused: int subtest) -> c_int {
    static int test__time_utils(struct test_suite *t __maybe_unused, int subtest __maybe_unused)
    {
    let mut pass: bool = true;
    pass &= test__parse_nsec_time("0", 0);
    pass &= test__parse_nsec_time("1", 1000000000ULL);
    pass &= test__parse_nsec_time("0.000000001", 1);
    pass &= test__parse_nsec_time("1.000000001", 1000000001ULL);
    pass &= test__parse_nsec_time("123456.123456", 123456123456000ULL);
    pass &= test__parse_nsec_time("1234567.123456789", 1234567123456789ULL);
    pass &= test__parse_nsec_time("18446744073.709551615",
    0xFFFFFFFFFFFFFFFFULL);
    pass &= test__perf_time__parse_str("1234567.123456789,1234567.123456789",
    1234567123456789ULL, 1234567123456789ULL);
    pass &= test__perf_time__parse_str("1234567.123456789,1234567.123456790",
    1234567123456789ULL, 1234567123456790ULL);
    pass &= test__perf_time__parse_str("1234567.123456789,",
    1234567123456789ULL, 0);
    pass &= test__perf_time__parse_str(",1234567.123456789",
    0, 1234567123456789ULL);
    pass &= test__perf_time__parse_str("0,1234567.123456789",
    0, 1234567123456789ULL);
    {
    let mut b: u64 = 1234567123456789ULL;
    struct test_data d = {
    .str   = "1234567.123456789,1234567.123456790",
    .ptime = { {b, b + 1}, },
    .num = 1,
    .skip = { b - 1, b + 2, },
    .noskip = { b, b + 1, },
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    {
    let mut b: u64 = 1234567123456789ULL;
    let mut c: u64 = 7654321987654321ULL;
    let mut e: u64 = 8000000000000000ULL;
    struct test_data d = {
    .str   = "1234567.123456789,1234567.123456790 "
    "7654321.987654321,7654321.987654444 "
    "8000000,8000000.000000005",
    .ptime = { {b, b + 1}, {c, c + 123}, {e, e + 5}, },
    .num = 3,
    .skip = { b - 1, b + 2, c - 1, c + 124, e - 1, e + 6 },
    .noskip = { b, b + 1, c, c + 123, e, e + 5 },
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    {
    let mut b: u64 = 7654321ULL * NSEC_PER_SEC;
    struct test_data d = {
    .str    = "10%/1",
    .first  = b,
    .last   = b + 100,
    .ptime  = { {b, b + 9}, },
    .num    = 1,
    .skip   = { b - 1, b + 10, },
    .noskip = { b, b + 9, },
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    {
    let mut b: u64 = 7654321ULL * NSEC_PER_SEC;
    struct test_data d = {
    .str    = "10%/2",
    .first  = b,
    .last   = b + 100,
    .ptime  = { {b + 10, b + 19}, },
    .num    = 1,
    .skip   = { b + 9, b + 20, },
    .noskip = { b + 10, b + 19, },
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    {
    let mut b: u64 = 11223344ULL * NSEC_PER_SEC;
    struct test_data d = {
    .str    = "10%/1,10%/2",
    .first  = b,
    .last   = b + 100,
    .ptime  = { {b, b + 9}, {b + 10, b + 19}, },
    .num    = 2,
    .skip   = { b - 1, b + 20, },
    .noskip = { b, b + 8, b + 9, b + 10, b + 11, b + 12, b + 19, },
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    {
    let mut b: u64 = 11223344ULL * NSEC_PER_SEC;
    struct test_data d = {
    .str    = "10%/1,10%/3,10%/10",
    .first  = b,
    .last   = b + 100,
    .ptime  = { {b, b + 9}, {b + 20, b + 29}, { b + 90, b + 100}, },
    .num    = 3,
    .skip   = { b - 1, b + 10, b + 19, b + 30, b + 89, b + 101 },
    .noskip = { b, b + 9, b + 20, b + 29, b + 90, b + 100},
    };
    pass &= test__perf_time__parse_for_ranges(&d);
    }
    pr_debug("\n");
    return pass ? 0 : TEST_FAIL;
    }
    DEFINE_SUITE("time utils", time_utils);
