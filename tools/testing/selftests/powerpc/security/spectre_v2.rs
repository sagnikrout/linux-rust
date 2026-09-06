//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/security/spectre_v2.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018-2019 IBM Corporation.
//
// Macro flag: #define __SANE_USERSPACE_TYPES__

    extern void pattern_cache_loop(void);
    extern void indirect_branch_loop(void);
#[no_mangle]
unsafe extern "C" fn do_count_loop(events: *mut event, is_p9: bool, miss_percent: *mut i64) -> c_int {
    static int do_count_loop(struct event *events, bool is_p9, s64 *miss_percent)
    {
    u64 pred, mpred;
    prctl(PR_TASK_PERF_EVENTS_ENABLE);
    if (is_p9)
    pattern_cache_loop();
    else
    indirect_branch_loop();
    prctl(PR_TASK_PERF_EVENTS_DISABLE);
    event_read(&events[0]);
    event_read(&events[1]);
// We could scale all the events by running/enabled but we're lazy
// As long as the PMU is uncontended they should all run
    FAIL_IF(events[0].result.running != events[0].result.enabled);
    FAIL_IF(events[1].result.running != events[1].result.enabled);
    pred =  events[0].result.value;
    mpred = events[1].result.value;
    if (is_p9) {
    event_read(&events[2]);
    event_read(&events[3]);
    FAIL_IF(events[2].result.running != events[2].result.enabled);
    FAIL_IF(events[3].result.running != events[3].result.enabled);
    pred  += events[2].result.value;
    mpred += events[3].result.value;
    }
// miss_percent = 100 * mpred / pred;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_event(e: *mut event, config: u64, name: *mut c_char) {
    static void setup_event(struct event *e, u64 config, char *name)
    {
    event_init_named(e, config, name);
    e.attr.disabled = 1;
    e.attr.exclude_kernel = 1;
    e.attr.exclude_hv = 1;
    e.attr.exclude_idle = 1;
    }
    enum spectre_v2_state {
    VULNERABLE = 0,
    UNKNOWN = 1,		// Works with FAIL_IF()
    NOT_AFFECTED,
    BRANCH_SERIALISATION,
    COUNT_CACHE_DISABLED,
    COUNT_CACHE_FLUSH_SW,
    COUNT_CACHE_FLUSH_HW,
    BTB_FLUSH,
    };
#[no_mangle]
unsafe extern "C" fn get_sysfs_state() -> enum spectre_v2_state {
    static enum spectre_v2_state get_sysfs_state(void)
    {
    let mut state: enum spectre_v2_state = UNKNOWN;
    char buf[256];
    int len;
    memset(buf, 0, sizeof(buf));
    FAIL_IF(read_sysfs_file("devices/system/cpu/vulnerabilities/spectre_v2", buf, sizeof(buf)));
// Make sure it's NULL terminated
    buf[sizeof(buf) - 1] = '\0';
// Trim the trailing newline
    len = strlen(buf);
    FAIL_IF(len < 1);
    buf[len - 1] = '\0';
    printf("sysfs reports: '%s'\n", buf);
// Order matters
    if (strstr(buf, "Vulnerable"))
    state = VULNERABLE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, affected"): "Not) -> else {
    else if (strstr(buf, "Not affected"))
    state = NOT_AFFECTED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, only)"): "Indirect branch serialisation (kernel) -> else {
    else if (strstr(buf, "Indirect branch serialisation (kernel only)"))
    state = BRANCH_SERIALISATION;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, disabled"): "Indirect branch cache) -> else {
    else if (strstr(buf, "Indirect branch cache disabled"))
    state = COUNT_CACHE_DISABLED;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, accelerated)"): "Software count cache flush (hardware) -> else {
    else if (strstr(buf, "Software count cache flush (hardware accelerated)"))
    state = COUNT_CACHE_FLUSH_HW;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, flush"): "Software count cache) -> else {
    else if (strstr(buf, "Software count cache flush"))
    state = COUNT_CACHE_FLUSH_SW;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstr(buf, flush"): "Branch predictor state) -> else {
    else if (strstr(buf, "Branch predictor state flush"))
    state = BTB_FLUSH;
    return state;
    }
pub const PM_BR_PRED_CCACHE: c_uint = 0x040a4	// P8 + P9;
pub const PM_BR_MPRED_CCACHE: c_uint = 0x040ac	// P8 + P9;
pub const PM_BR_PRED_PCACHE: c_uint = 0x048a0	// P9 only;
pub const PM_BR_MPRED_PCACHE: c_uint = 0x048b0	// P9 only;
#[no_mangle]
pub unsafe extern "C" fn spectre_v2_test() -> c_int {
    int spectre_v2_test(void)
    {
    enum spectre_v2_state state;
    struct event events[4];
    s64 miss_percent;
    bool is_p9;
// The PMU events we use only work on Power8 or later
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    state = get_sysfs_state();
    if (state == UNKNOWN) {
    printf("Error: couldn't determine spectre_v2 mitigation state?\n");
    return -1;
    }
    memset(events, 0, sizeof(events));
    setup_event(&events[0], PM_BR_PRED_CCACHE,  "PM_BR_PRED_CCACHE");
    setup_event(&events[1], PM_BR_MPRED_CCACHE, "PM_BR_MPRED_CCACHE");
    FAIL_IF(event_open(&events[0]));
    FAIL_IF(event_open_with_group(&events[1], events[0].fd) == -1);
    is_p9 = ((mfspr(SPRN_PVR) >>  16) & 0xFFFF) == 0x4e;
    if (is_p9) {
// Count pattern cache too
    setup_event(&events[2], PM_BR_PRED_PCACHE,  "PM_BR_PRED_PCACHE");
    setup_event(&events[3], PM_BR_MPRED_PCACHE, "PM_BR_MPRED_PCACHE");
    FAIL_IF(event_open_with_group(&events[2], events[0].fd) == -1);
    FAIL_IF(event_open_with_group(&events[3], events[0].fd) == -1);
    }
    FAIL_IF(do_count_loop(events, is_p9, &miss_percent));
    event_report_justified(&events[0], 18, 10);
    event_report_justified(&events[1], 18, 10);
    event_close(&events[0]);
    event_close(&events[1]);
    if (is_p9) {
    event_report_justified(&events[2], 18, 10);
    event_report_justified(&events[3], 18, 10);
    event_close(&events[2]);
    event_close(&events[3]);
    }
    printf("Miss percent %lld %%\n", miss_percent);
    switch (state) {
    case VULNERABLE:
    case NOT_AFFECTED:
    case COUNT_CACHE_FLUSH_SW:
    case COUNT_CACHE_FLUSH_HW:
// These should all not affect userspace branch prediction
    if (miss_percent > 15) {
    if (miss_percent > 95) {
//
// Such a mismatch may be caused by a system being unaware
// the count cache is disabled. This may be to enable
// guest migration between hosts with different settings.
// Return skip code to avoid detecting this as an error.
// We are not vulnerable and reporting otherwise, so
// missing such a mismatch is safe.
//
    printf("Branch misses > 95%% unexpected in this configuration.\n");
    printf("Count cache likely disabled without Linux knowing.\n");
    if (state == COUNT_CACHE_FLUSH_SW)
    printf("WARNING: Kernel performing unnecessary flushes.\n");
    return 4;
    }
    printf("Branch misses > 15%% unexpected in this configuration!\n");
    printf("Possible mismatch between reported & actual mitigation\n");
    return 1;
    }
    break;
    case BRANCH_SERIALISATION:
// This seems to affect userspace branch prediction a bit?
    if (miss_percent > 25) {
    printf("Branch misses > 25%% unexpected in this configuration!\n");
    printf("Possible mismatch between reported & actual mitigation\n");
    return 1;
    }
    break;
    case COUNT_CACHE_DISABLED:
    if (miss_percent < 95) {
    printf("Branch misses < 95%% unexpected in this configuration!\n");
    printf("Possible mismatch between reported & actual mitigation\n");
    return 1;
    }
    break;
    case UNKNOWN:
    case BTB_FLUSH:
    printf("Not sure!\n");
    return 1;
    }
    printf("OK - Measured branch prediction rates match reported spectre v2 mitigation.\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(spectre_v2_test, "spectre_v2");
    }
