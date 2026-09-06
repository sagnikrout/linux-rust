//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/event_groups.c
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

// hw: cycles,instructions sw: context-switch, uncore: [arch dependent]
    static int types[] = {0, 1, -1};
    static unsigned long configs[] = {0, 3, 0};
    static unsigned long configs_hw[] = {1};
pub const NR_UNCORE_PMUS: c_int = 5;
// Uncore pmus that support more than 3 counters
    static struct uncore_pmus {
    const char *name;
    __u64 config;
    } uncore_pmus[NR_UNCORE_PMUS] = {
    { "amd_l3", 0x0 },
    { "amd_df", 0x0 },
    { "uncore_imc_0", 0x1 },         /* Intel */
    { "core_imc", 0x318 },           /* PowerPC: core_imc/CPM_STCX_FIN/ */
    { "hv_24x7", 0x22000000003 },    /* PowerPC: hv_24x7/CPM_STCX_FIN/ */
    };
#[no_mangle]
unsafe extern "C" fn event_open(type: c_int, config: c_ulong, group_fd: c_int) -> c_int {
    static int event_open(int type, unsigned long config, int group_fd)
    {
    struct perf_event_attr attr;
    memset(&attr, 0, sizeof(struct perf_event_attr));
    attr.type = type;
    attr.size = sizeof(struct perf_event_attr);
    attr.config = config;
//
// When creating an event group, typically the group leader is
// initialized with disabled set to 1 and any child events are
// initialized with disabled set to 0. Despite disabled being 0,
// the child events will not start until the group leader is
// enabled.
//
    attr.disabled = group_fd == -1 ? 1 : 0;
    return sys_perf_event_open(&attr, -1, 0, group_fd, 0);
    }
#[no_mangle]
unsafe extern "C" fn setup_uncore_event() -> c_int {
    static int setup_uncore_event(void)
    {
    struct perf_pmu *pmu = core::ptr::null_mut();
    int i, fd;
    while ((pmu = perf_pmus__scan(pmu)) != core::ptr::null_mut()) {
    for (i = 0; i < NR_UNCORE_PMUS; i++) {
    if (!strcmp(uncore_pmus[i].name, pmu.name)) {
    pr_debug("Using %s for uncore pmu event\n", pmu.name);
    types[2] = pmu.type;
    configs[2] = uncore_pmus[i].config;
//
// Check if the chosen uncore pmu event can be
// used in the test. For example, incase of accessing
// hv_24x7 pmu counters, partition should have
// additional permissions. If not, event open will
// fail. So check if the event open succeeds
// before proceeding.
//
    fd = event_open(types[2], configs[2], -1);
    if (fd < 0)
    return -1;
    close(fd);
    return 0;
    }
    }
    }
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn run_test(i: c_int, j: c_int, k: c_int) -> c_int {
    static int run_test(int i, int j, int k)
    {
    let mut erroneous: c_int = ((((1 << i) | (1 << j) | (1 << k)) & 5) == 5);
    int group_fd, sibling_fd1, sibling_fd2;
    group_fd = event_open(types[i], configs[i], -1);
    if (group_fd == -1)
    return -1;
    sibling_fd1 = event_open(types[j], configs[j], group_fd);
    if (sibling_fd1 == -1) {
    close(group_fd);
    return erroneous ? 0 : -1;
    }
//
// if all three events (leader and two sibling events)
// are hardware events, use instructions as one of the
// sibling event. There is event constraint in powerpc that
// events using same counter cannot be programmed in a group.
// Since PERF_COUNT_HW_INSTRUCTIONS is a generic hardware
// event and present in all platforms, lets use that.
//
    if (!i && !j && !k)
    sibling_fd2 = event_open(types[k], configs_hw[k], group_fd);
    else
    sibling_fd2 = event_open(types[k], configs[k], group_fd);
    if (sibling_fd2 == -1) {
    close(sibling_fd1);
    close(group_fd);
    return erroneous ? 0 : -1;
    }
    close(sibling_fd2);
    close(sibling_fd1);
    close(group_fd);
    return erroneous ? -1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn test__event_groups(__maybe_unused: *mut *mut test_suite text, __maybe_unused: int subtest) -> c_int {
    static int test__event_groups(struct test_suite *text __maybe_unused, int subtest __maybe_unused)
    {
    int i, j, k;
    int ret;
    int r;
    ret = setup_uncore_event();
    if (ret || types[2] == -1)
    return TEST_SKIP;
    ret = TEST_OK;
    for (i = 0; i < 3; i++) {
    for (j = 0; j < 3; j++) {
    for (k = 0; k < 3; k++) {
    r = run_test(i, j, k);
    if (r)
    ret = TEST_FAIL;
//
// For all three events as HW events, second sibling
// event is picked from configs_hw. So print accordingly
//
    if (!i && !j && !k)
    pr_debug("0x%x 0x%lx, 0x%x 0x%lx, 0x%x 0x%lx: %s\n",
    types[i], configs[i], types[j], configs[j],
    types[k], configs_hw[k], r ? "Fail" : "Pass");
    else
    pr_debug("0x%x 0x%lx, 0x%x 0x%lx, 0x%x 0x%lx: %s\n",
    types[i], configs[i], types[j], configs[j],
    types[k], configs[k], r ? "Fail" : "Pass");
    }
    }
    }
    return ret;
    }
    DEFINE_SUITE("Event groups", event_groups);
