//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/test_cpu.c
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
// Macro flag: #define _GNU_SOURCE

    enum hog_clock_type {
// Count elapsed time using the CLOCK_PROCESS_CPUTIME_ID clock.
    CPU_HOG_CLOCK_PROCESS,
// Count elapsed time using system wallclock time.
    CPU_HOG_CLOCK_WALL,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_hogger {
    pub cgroup: *mut c_char,
    pub pid: pid_t,
    pub usage: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_hog_func_param {
    pub nprocs: c_int,
    pub ts: timespec,
    pub clock_type: enum hog_clock_type,
}

//
// This test creates two nested cgroups with and without enabling
// the cpu controller.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_subtree_control(root: *const c_char) -> c_int {
    static int test_cpucg_subtree_control(const char *root)
    {
    char *parent = core::ptr::null_mut(), *child = core::ptr::null_mut(), *parent2 = core::ptr::null_mut(), *child2 = core::ptr::null_mut();
    let mut ret: c_int = KSFT_FAIL;
// Create two nested cgroups with the cpu controller enabled.
    parent = cg_name(root, "cpucg_test_0");
    if (!parent)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+cpu"))
    goto cleanup;
    child = cg_name(parent, "cpucg_test_child");
    if (!child)
    goto cleanup;
    if (cg_create(child))
    goto cleanup;
    if (cg_read_strstr(child, "cgroup.controllers", "cpu"))
    goto cleanup;
// Create two nested cgroups without enabling the cpu controller.
    parent2 = cg_name(root, "cpucg_test_1");
    if (!parent2)
    goto cleanup;
    if (cg_create(parent2))
    goto cleanup;
    child2 = cg_name(parent2, "cpucg_test_child");
    if (!child2)
    goto cleanup;
    if (cg_create(child2))
    goto cleanup;
    if (!cg_read_strstr(child2, "cgroup.controllers", "cpu"))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(child);
    free(child);
    cg_destroy(child2);
    free(child2);
    cg_destroy(parent);
    free(parent);
    cg_destroy(parent2);
    free(parent2);
    return ret;
    }
    static void *hog_cpu_thread_func(void *arg)
    {
    while (1)
    ;
    return core::ptr::null_mut();
    }
    static struct timespec
    timespec_sub(const struct timespec *lhs, const struct timespec *rhs)
    {
    struct timespec zero = {
    .tv_sec = 0,
    .tv_nsec = 0,
    };
    struct timespec ret;
    if (lhs.tv_sec < rhs.tv_sec)
    return zero;
    ret.tv_sec = lhs.tv_sec - rhs.tv_sec;
    if (lhs.tv_nsec < rhs.tv_nsec) {
    if (ret.tv_sec == 0)
    return zero;
    ret.tv_sec--;
    ret.tv_nsec = NSEC_PER_SEC - rhs.tv_nsec + lhs.tv_nsec;
    } else
    ret.tv_nsec = lhs.tv_nsec - rhs.tv_nsec;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hog_cpus_timed(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int hog_cpus_timed(const char *cgroup, void *arg)
    {
    const struct cpu_hog_func_param *param =
    (struct cpu_hog_func_param *)arg;
    let mut ts_run: timespec = param.ts;
    let mut ts_remaining: timespec = ts_run;
    struct timespec ts_start;
    int i, ret;
    ret = clock_gettime(CLOCK_MONOTONIC, &ts_start);
    if (ret != 0)
    return ret;
    for (i = 0; i < param.nprocs; i++) {
    pthread_t tid;
    ret = pthread_create(&tid, core::ptr::null_mut(), &hog_cpu_thread_func, core::ptr::null_mut());
    if (ret != 0)
    return ret;
    }
    while (ts_remaining.tv_sec > 0 || ts_remaining.tv_nsec > 0) {
    struct timespec ts_total;
    ret = nanosleep(&ts_remaining, core::ptr::null_mut());
    if (ret && errno != EINTR)
    return ret;
    if (param.clock_type == CPU_HOG_CLOCK_PROCESS) {
    ret = clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &ts_total);
    if (ret != 0)
    return ret;
    } else {
    struct timespec ts_current;
    ret = clock_gettime(CLOCK_MONOTONIC, &ts_current);
    if (ret != 0)
    return ret;
    ts_total = timespec_sub(&ts_current, &ts_start);
    }
    ts_remaining = timespec_sub(&ts_run, &ts_total);
    }
    return 0;
    }
//
// Creates a cpu cgroup, burns a CPU for a few quanta, and verifies that
// cpu.stat shows the expected output.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_stats(root: *const c_char) -> c_int {
    static int test_cpucg_stats(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    long usage_usec, user_usec, system_usec;
    let mut usage_seconds: c_long = 2;
    let mut expected_usage_usec: c_long = usage_seconds * USEC_PER_SEC;
    char *cpucg;
    cpucg = cg_name(root, "cpucg_test");
    if (!cpucg)
    goto cleanup;
    if (cg_create(cpucg))
    goto cleanup;
    usage_usec = cg_read_key_long(cpucg, "cpu.stat", "usage_usec");
    user_usec = cg_read_key_long(cpucg, "cpu.stat", "user_usec");
    system_usec = cg_read_key_long(cpucg, "cpu.stat", "system_usec");
    if (usage_usec != 0 || user_usec != 0 || system_usec != 0)
    goto cleanup;
    struct cpu_hog_func_param param = {
    .nprocs = 1,
    .ts = {
    .tv_sec = usage_seconds,
    .tv_nsec = 0,
    },
    .clock_type = CPU_HOG_CLOCK_PROCESS,
    };
    if (cg_run(cpucg, hog_cpus_timed, (void *)&param))
    goto cleanup;
    usage_usec = cg_read_key_long(cpucg, "cpu.stat", "usage_usec");
    user_usec = cg_read_key_long(cpucg, "cpu.stat", "user_usec");
    if (user_usec <= 0)
    goto cleanup;
    if (!values_close_report(usage_usec, expected_usage_usec, 1))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(cpucg);
    free(cpucg);
    return ret;
    }
//
// Creates a nice process that consumes CPU and checks that the elapsed
// usertime in the cgroup is close to the expected time.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_nice(root: *const c_char) -> c_int {
    static int test_cpucg_nice(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    int status;
    long user_usec, nice_usec;
    let mut usage_seconds: c_long = 2;
    let mut expected_nice_usec: c_long = usage_seconds * USEC_PER_SEC;
    char *cpucg;
    pid_t pid;
    cpucg = cg_name(root, "cpucg_test");
    if (!cpucg)
    goto cleanup;
    if (cg_create(cpucg))
    goto cleanup;
    user_usec = cg_read_key_long(cpucg, "cpu.stat", "user_usec");
    nice_usec = cg_read_key_long(cpucg, "cpu.stat", "nice_usec");
    if (nice_usec == -1)
    ret = KSFT_SKIP;
    if (user_usec != 0 || nice_usec != 0)
    goto cleanup;
//
// We fork here to create a new process that can be niced without
// polluting the nice value of other selftests
//
    pid = fork();
    if (pid < 0) {
    goto cleanup;
    } else if (pid == 0) {
    struct cpu_hog_func_param param = {
    .nprocs = 1,
    .ts = {
    .tv_sec = usage_seconds,
    .tv_nsec = 0,
    },
    .clock_type = CPU_HOG_CLOCK_PROCESS,
    };
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", getpid());
    if (cg_write(cpucg, "cgroup.procs", buf))
    exit(EXIT_FAILURE);
// Try to keep niced CPU usage as constrained to hog_cpu as possible
    nice(1);
    hog_cpus_timed(cpucg, &param);
    exit(0);
    } else {
    waitpid(pid, &status, 0);
    if (!WIFEXITED(status))
    goto cleanup;
    user_usec = cg_read_key_long(cpucg, "cpu.stat", "user_usec");
    nice_usec = cg_read_key_long(cpucg, "cpu.stat", "nice_usec");
    if (user_usec <= 0)
    goto cleanup;
    if (!values_close_report(nice_usec, expected_nice_usec, 1))
    goto cleanup;
    ret = KSFT_PASS;
    }
    cleanup:
    cg_destroy(cpucg);
    free(cpucg);
    return ret;
    }
    static int
    run_cpucg_weight_test(
    const char *root,
    pid_t (*spawn_child)(const struct cpu_hogger *child),
    int (*validate)(const struct cpu_hogger *children, int num_children))
    {
    let mut ret: c_int = KSFT_FAIL, i;
    char *parent = core::ptr::null_mut();
    struct cpu_hogger children[3] = {};
    parent = cg_name(root, "cpucg_test_0");
    if (!parent)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+cpu"))
    goto cleanup;
    for (i = 0; i < ARRAY_SIZE(children); i++) {
    children[i].cgroup = cg_name_indexed(parent, "cpucg_child", i);
    if (!children[i].cgroup)
    goto cleanup;
    if (cg_create(children[i].cgroup))
    goto cleanup;
    if (cg_write_numeric(children[i].cgroup, "cpu.weight",
    50 * (i + 1)))
    goto cleanup;
    }
    for (i = 0; i < ARRAY_SIZE(children); i++) {
    let mut pid: pid_t = spawn_child(&children[i]);
    if (pid <= 0)
    goto cleanup;
    children[i].pid = pid;
    }
    for (i = 0; i < ARRAY_SIZE(children); i++) {
    int retcode;
    waitpid(children[i].pid, &retcode, 0);
    if (!WIFEXITED(retcode))
    goto cleanup;
    if (WEXITSTATUS(retcode))
    goto cleanup;
    }
    for (i = 0; i < ARRAY_SIZE(children); i++)
    children[i].usage = cg_read_key_long(children[i].cgroup,
    "cpu.stat", "usage_usec");
    if (validate(children, ARRAY_SIZE(children)))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    for (i = 0; i < ARRAY_SIZE(children); i++) {
    cg_destroy(children[i].cgroup);
    free(children[i].cgroup);
    }
    cg_destroy(parent);
    free(parent);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn weight_hog_ncpus(child: *const cpu_hogger, ncpus: c_int) -> pid_t {
    static pid_t weight_hog_ncpus(const struct cpu_hogger *child, int ncpus)
    {
    let mut usage_seconds: c_long = 10;
    struct cpu_hog_func_param param = {
    .nprocs = ncpus,
    .ts = {
    .tv_sec = usage_seconds,
    .tv_nsec = 0,
    },
    .clock_type = CPU_HOG_CLOCK_WALL,
    };
    return cg_run_nowait(child.cgroup, hog_cpus_timed, (void *)&param);
    }
#[no_mangle]
unsafe extern "C" fn weight_hog_all_cpus(child: *const cpu_hogger) -> pid_t {
    static pid_t weight_hog_all_cpus(const struct cpu_hogger *child)
    {
    return weight_hog_ncpus(child, get_nprocs());
    }
    static int
    overprovision_validate(const struct cpu_hogger *children, int num_children)
    {
    let mut ret: c_int = KSFT_FAIL, i;
    for (i = 0; i < num_children - 1; i++) {
    long delta;
    if (children[i + 1].usage <= children[i].usage)
    goto cleanup;
    delta = children[i + 1].usage - children[i].usage;
    if (!values_close_report(delta, children[0].usage, 35))
    goto cleanup;
    }
    ret = KSFT_PASS;
    cleanup:
    return ret;
    }
//
// First, this test creates the following hierarchy:
// A
// A/B     cpu.weight = 50
// A/C     cpu.weight = 100
// A/D     cpu.weight = 150
//
// A separate process is then created for each child cgroup which spawns as
// many threads as there are cores, and hogs each CPU as much as possible
// for some time interval.
//
// Once all of the children have exited, we verify that each child cgroup
// was given proportional runtime as informed by their cpu.weight.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_weight_overprovisioned(root: *const c_char) -> c_int {
    static int test_cpucg_weight_overprovisioned(const char *root)
    {
    return run_cpucg_weight_test(root, weight_hog_all_cpus,
    overprovision_validate);
    }
#[no_mangle]
unsafe extern "C" fn weight_hog_one_cpu(child: *const cpu_hogger) -> pid_t {
    static pid_t weight_hog_one_cpu(const struct cpu_hogger *child)
    {
    return weight_hog_ncpus(child, 1);
    }
    static int
    underprovision_validate(const struct cpu_hogger *children, int num_children)
    {
    let mut ret: c_int = KSFT_FAIL, i;
    for (i = 0; i < num_children - 1; i++) {
    if (!values_close_report(children[i + 1].usage, children[0].usage, 15))
    goto cleanup;
    }
    ret = KSFT_PASS;
    cleanup:
    return ret;
    }
//
// First, this test creates the following hierarchy:
// A
// A/B     cpu.weight = 50
// A/C     cpu.weight = 100
// A/D     cpu.weight = 150
//
// A separate process is then created for each child cgroup which spawns a
// single thread that hogs a CPU. The testcase is only run on systems that
// have at least one core per-thread in the child processes.
//
// Once all of the children have exited, we verify that each child cgroup
// had roughly the same runtime despite having different cpu.weight.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_weight_underprovisioned(root: *const c_char) -> c_int {
    static int test_cpucg_weight_underprovisioned(const char *root)
    {
// Only run the test if there are enough cores to avoid overprovisioning
// the system.
    if (get_nprocs() < 4)
    return KSFT_SKIP;
    return run_cpucg_weight_test(root, weight_hog_one_cpu,
    underprovision_validate);
    }
    static int
    run_cpucg_nested_weight_test(const char *root, bool overprovisioned)
    {
    let mut ret: c_int = KSFT_FAIL, i;
    char *parent = core::ptr::null_mut(), *child = core::ptr::null_mut();
    struct cpu_hogger leaf[3] = {};
    long nested_leaf_usage, child_usage;
    let mut nprocs: c_int = get_nprocs();
    if (!overprovisioned) {
    if (nprocs < 4)
//
// Only run the test if there are enough cores to avoid overprovisioning
// the system.
//
    return KSFT_SKIP;
    nprocs /= 4;
    }
    parent = cg_name(root, "cpucg_test");
    child = cg_name(parent, "cpucg_child");
    if (!parent || !child)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+cpu"))
    goto cleanup;
    if (cg_create(child))
    goto cleanup;
    if (cg_write(child, "cgroup.subtree_control", "+cpu"))
    goto cleanup;
    if (cg_write(child, "cpu.weight", "1000"))
    goto cleanup;
    for (i = 0; i < ARRAY_SIZE(leaf); i++) {
    const char *ancestor;
    long weight;
    if (i == 0) {
    ancestor = parent;
    weight = 1000;
    } else {
    ancestor = child;
    weight = 5000;
    }
    leaf[i].cgroup = cg_name_indexed(ancestor, "cpucg_leaf", i);
    if (!leaf[i].cgroup)
    goto cleanup;
    if (cg_create(leaf[i].cgroup))
    goto cleanup;
    if (cg_write_numeric(leaf[i].cgroup, "cpu.weight", weight))
    goto cleanup;
    }
    for (i = 0; i < ARRAY_SIZE(leaf); i++) {
    pid_t pid;
    struct cpu_hog_func_param param = {
    .nprocs = nprocs,
    .ts = {
    .tv_sec = 10,
    .tv_nsec = 0,
    },
    .clock_type = CPU_HOG_CLOCK_WALL,
    };
    pid = cg_run_nowait(leaf[i].cgroup, hog_cpus_timed,
    (void *)&param);
    if (pid <= 0)
    goto cleanup;
    leaf[i].pid = pid;
    }
    for (i = 0; i < ARRAY_SIZE(leaf); i++) {
    int retcode;
    waitpid(leaf[i].pid, &retcode, 0);
    if (!WIFEXITED(retcode))
    goto cleanup;
    if (WEXITSTATUS(retcode))
    goto cleanup;
    }
    for (i = 0; i < ARRAY_SIZE(leaf); i++) {
    leaf[i].usage = cg_read_key_long(leaf[i].cgroup,
    "cpu.stat", "usage_usec");
    if (leaf[i].usage <= 0)
    goto cleanup;
    }
    nested_leaf_usage = leaf[1].usage + leaf[2].usage;
    if (overprovisioned) {
    if (!values_close_report(leaf[0].usage, nested_leaf_usage, 15))
    goto cleanup;
    } else if (!values_close_report(leaf[0].usage * 2, nested_leaf_usage, 15))
    goto cleanup;
    child_usage = cg_read_key_long(child, "cpu.stat", "usage_usec");
    if (child_usage <= 0)
    goto cleanup;
    if (!values_close_report(child_usage, nested_leaf_usage, 1))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    for (i = 0; i < ARRAY_SIZE(leaf); i++) {
    cg_destroy(leaf[i].cgroup);
    free(leaf[i].cgroup);
    }
    cg_destroy(child);
    free(child);
    cg_destroy(parent);
    free(parent);
    return ret;
    }
//
// First, this test creates the following hierarchy:
// A
// A/B     cpu.weight = 1000
// A/C     cpu.weight = 1000
// A/C/D   cpu.weight = 5000
// A/C/E   cpu.weight = 5000
//
// A separate process is then created for each leaf, which spawn nproc threads
// that burn a CPU for a few seconds.
//
// Once all of those processes have exited, we verify that each of the leaf
// cgroups have roughly the same usage from cpu.stat.
//
    static int
    test_cpucg_nested_weight_overprovisioned(const char *root)
    {
    return run_cpucg_nested_weight_test(root, true);
    }
//
// First, this test creates the following hierarchy:
// A
// A/B     cpu.weight = 1000
// A/C     cpu.weight = 1000
// A/C/D   cpu.weight = 5000
// A/C/E   cpu.weight = 5000
//
// A separate process is then created for each leaf, which nproc / 4 threads
// that burns a CPU for a few seconds.
//
// Once all of those processes have exited, we verify that each of the leaf
// cgroups have roughly the same usage from cpu.stat.
//
    static int
    test_cpucg_nested_weight_underprovisioned(const char *root)
    {
    return run_cpucg_nested_weight_test(root, false);
    }
//
// Best effort attempt to get the kernel's HZ value from the config.
// Return the HZ value if found otherwise return 1000 (the default) to
// indicate failure.
//
    static long
    get_config_hz(void)
    {
    let mut hz: c_long = 1000;
    FILE *f;
    char cmd[256] = "zcat /proc/config.gz 2>/dev/null | grep '^CONFIG_HZ='";
    f = popen(cmd, "r");
    if (!f)
    return hz;
    if (fscanf(f, "CONFIG_HZ=%ld", &hz) == EOF)
    goto out;
    out:
    pclose(f);
    return hz;
    }
//
// This test creates a cgroup with some maximum value within a period, and
// verifies that a process in the cgroup is not overscheduled.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_max(root: *const c_char) -> c_int {
    static int test_cpucg_max(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    let mut hz: c_long = get_config_hz();
    let mut quota_usec: c_long = 1000;
    long default_period_usec = 100000; /* cpu.max's default period */
    let mut duration_seconds: c_long = 1;
    long duration_usec;
    long usage_usec, n_periods, remainder_usec, expected_usage_usec;
    char *cpucg;
    char quota_buf[32];
    duration_usec = duration_seconds * USEC_PER_SEC * 1000 / hz;
    snprintf(quota_buf, sizeof(quota_buf), "%ld", quota_usec);
    cpucg = cg_name(root, "cpucg_test");
    if (!cpucg)
    goto cleanup;
    if (cg_create(cpucg))
    goto cleanup;
    if (cg_write(cpucg, "cpu.max", quota_buf))
    goto cleanup;
    struct cpu_hog_func_param param = {
    .nprocs = 1,
    .ts = {
    .tv_sec = duration_usec / USEC_PER_SEC,
    .tv_nsec = duration_usec % USEC_PER_SEC * NSEC_PER_USEC,
    },
    .clock_type = CPU_HOG_CLOCK_WALL,
    };
    if (cg_run(cpucg, hog_cpus_timed, (void *)&param))
    goto cleanup;
    usage_usec = cg_read_key_long(cpucg, "cpu.stat", "usage_usec");
    if (usage_usec <= 0)
    goto cleanup;
//
// The following calculation applies only since
// the cpu hog is set to run as per wall-clock time
//
    n_periods = duration_usec / default_period_usec;
    remainder_usec = duration_usec - n_periods * default_period_usec;
    expected_usage_usec
    = n_periods * quota_usec + MIN(remainder_usec, quota_usec);
    if (!values_close_report(usage_usec, expected_usage_usec, 10))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(cpucg);
    free(cpucg);
    return ret;
    }
//
// This test verifies that a process inside of a nested cgroup whose parent
// group has a cpu.max value set, is properly throttled.
//
#[no_mangle]
unsafe extern "C" fn test_cpucg_max_nested(root: *const c_char) -> c_int {
    static int test_cpucg_max_nested(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    let mut hz: c_long = get_config_hz();
    let mut quota_usec: c_long = 1000;
    long default_period_usec = 100000; /* cpu.max's default period */
    let mut duration_seconds: c_long = 1;
    long duration_usec;
    long usage_usec, n_periods, remainder_usec, expected_usage_usec;
    char *parent, *child;
    char quota_buf[32];
    duration_usec = duration_seconds * USEC_PER_SEC * 1000 / hz;
    snprintf(quota_buf, sizeof(quota_buf), "%ld", quota_usec);
    parent = cg_name(root, "cpucg_parent");
    child = cg_name(parent, "cpucg_child");
    if (!parent || !child)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+cpu"))
    goto cleanup;
    if (cg_create(child))
    goto cleanup;
    if (cg_write(parent, "cpu.max", quota_buf))
    goto cleanup;
    struct cpu_hog_func_param param = {
    .nprocs = 1,
    .ts = {
    .tv_sec = duration_usec / USEC_PER_SEC,
    .tv_nsec = duration_usec % USEC_PER_SEC * NSEC_PER_USEC,
    },
    .clock_type = CPU_HOG_CLOCK_WALL,
    };
    if (cg_run(child, hog_cpus_timed, (void *)&param))
    goto cleanup;
    usage_usec = cg_read_key_long(child, "cpu.stat", "usage_usec");
    if (usage_usec <= 0)
    goto cleanup;
//
// The following calculation applies only since
// the cpu hog is set to run as per wall-clock time
//
    n_periods = duration_usec / default_period_usec;
    remainder_usec = duration_usec - n_periods * default_period_usec;
    expected_usage_usec
    = n_periods * quota_usec + MIN(remainder_usec, quota_usec);
    if (!values_close_report(usage_usec, expected_usage_usec, 10))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(child);
    free(child);
    cg_destroy(parent);
    free(parent);
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpucg_test {
    pub root): *const *const int (fn)(char,
    pub name: *const c_char,
    } tests[] = {
    T(test_cpucg_subtree_control),
    T(test_cpucg_stats),
    T(test_cpucg_nice),
    T(test_cpucg_weight_overprovisioned),
    T(test_cpucg_weight_underprovisioned),
    T(test_cpucg_nested_weight_overprovisioned),
    T(test_cpucg_nested_weight_underprovisioned),
    T(test_cpucg_max),
    T(test_cpucg_max_nested),
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    char root[PATH_MAX];
    int i;
    ksft_print_header();
    if (cg_find_unified_root(root, sizeof(root), core::ptr::null_mut()))
    ksft_exit_skip("cgroup v2 isn't mounted\n");
    if (cg_read_strstr(root, "cgroup.subtree_control", "cpu"))
    if (cg_write(root, "cgroup.subtree_control", "+cpu"))
    ksft_exit_skip("Failed to set cpu controller\n");
    ksft_set_plan(ARRAY_SIZE(tests));
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    switch (tests[i].fn(root)) {
    case KSFT_PASS:
    ksft_test_result_pass("%s\n", tests[i].name);
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("%s\n", tests[i].name);
    break;
    default:
    ksft_test_result_fail("%s\n", tests[i].name);
    break;
    }
    }
    ksft_finished();
    }
