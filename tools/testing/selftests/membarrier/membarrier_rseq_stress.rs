//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/membarrier/membarrier_rseq_stress.c
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
// Membarrier stress test for CFS throttle interactions.
//
// Reproducer for the interaction between CFS throttle and expedited membarrier.
//

// Macro flag: #define _GNU_SOURCE

// -- Architecture-specific rseq signature --

// -- rseq ABI (kernel uapi; define locally for portability) --

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rseq_abi {
    pub cpu_id_start: __u32,
    pub cpu_id: __u32,
    pub rseq_cs: __u64,
    pub flags: __u32,
    pub node_id: __u32,
    pub mm_cid: __u32,
    pub end: [c_char; 0],
    pub __aligned(32): },
// -- membarrier constants (not in all distro headers) --

// -- Test parameters --
pub const N_SIBLINGS: c_int = 2000;
pub const NEST_DEPTH: c_int = 5;
    pub g_cgroup_path: [static char; 4096],
    pub use_cgroup_v2: static int,
pub const CFS_QUOTA_US: c_int = 1000;
pub const CFS_PERIOD_US: c_int = 5000;
pub const N_HAMMER_PER_CPU: c_int = 25;
pub const N_BURNER_PER_CPU: c_int = 50;
pub const MAX_STRESS_CPUS: c_int = 1024;
pub const TEST_DURATION_SEC: c_int = 20;
// Latency thresholds for the sentinel
pub const LATENCY_WARN_MS: c_int = 50;
pub const LATENCY_CRITICAL_MS: c_int = 200;
// Sentinel sampling interval
pub const SENTINEL_INTERVAL_US: c_int = 500;
// -- Shared globals --
    pub g_stop: static atomic_int,
    pub g_stop_sentinel: static atomic_int,
    pub g_max_latency_us: static atomic_long,
    pub g_interval_max_latency_us: static atomic_long,
    pub g_mb_ok: static atomic_long,
    pub g_mb_err: static atomic_long,
    pub g_ncpus_stress: static int,
    pub g_stress_cpus: *mut static int,
    pub g_test_ready: static atomic_int,
// Per-thread rseq ABI block registered with the kernel
    static __thread struct rseq_abi tls_rseq
    __attribute__((tls_model("initial-exec"))) __aligned(32) = {
    .cpu_id = RSEQ_CPU_ID_UNINITIALIZED,
}

// -- Utility --
#[no_mangle]
unsafe extern "C" fn write_file(path: *const c_char, val: *const c_char) -> c_int {
    static int write_file(const char *path, const char *val)
    {
    let mut fd: c_int = open(path, O_WRONLY | O_CLOEXEC);
    if (fd < 0)
    return -errno;
    let mut len: usize = strlen(val);
    let mut r: isize = write(fd, val, len);
    close(fd);
    if (r < 0)
    return -errno;
    if ((size_t)r != len)
    return -EIO;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn monotonic_us() -> u64 {
    static uint64_t monotonic_us(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000ULL + ts.tv_nsec / 1000ULL;
    }
#[no_mangle]
unsafe extern "C" fn update_max_latency(lat: c_long) {
    static void update_max_latency(long lat)
    {
    let mut old: c_long = atomic_load_explicit(&g_max_latency_us, memory_order_relaxed);
    while (lat > old) {
    if (atomic_compare_exchange_weak_explicit(&g_max_latency_us, &old, lat,
    memory_order_relaxed, memory_order_relaxed))
    break;
    }
    old = atomic_load_explicit(&g_interval_max_latency_us, memory_order_relaxed);
    while (lat > old) {
    if (atomic_compare_exchange_weak_explicit(&g_interval_max_latency_us, &old, lat,
    memory_order_relaxed, memory_order_relaxed))
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn init_stress_cpus() {
    static void init_stress_cpus(void)
    {
    cpu_set_t set;
    let mut capacity: c_int = MAX_STRESS_CPUS;
    g_stress_cpus = malloc(capacity * sizeof(int));
    if (!g_stress_cpus)
    ksft_exit_fail_msg("malloc failed for g_stress_cpus\n");
    if (sched_getaffinity(0, sizeof(set), &set) < 0)
    ksft_exit_fail_msg("sched_getaffinity failed\n");
    for (int i = 0; i < CPU_SETSIZE && g_ncpus_stress < capacity; i++) {
    if (CPU_ISSET(i, &set))
    g_stress_cpus[g_ncpus_stress++] = i;
    }
    if (g_ncpus_stress == 0)
    ksft_exit_skip("No CPUs available for stress test\n");
    ksft_print_msg("Stressing %d CPUs discovered via affinity\n", g_ncpus_stress);
    }
// -- rseq / membarrier helpers --
#[no_mangle]
unsafe extern "C" fn rseq_register_thread() -> c_int {
    static int rseq_register_thread(void)
    {
    let mut r: c_int = syscall(SYS_rseq, &tls_rseq, sizeof(tls_rseq), 0, RSEQ_SIG);
    return (r == 0 || errno == EBUSY || errno == EINVAL) ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn rseq_register_thread_at(rseq: *mut rseq_abi) -> c_int {
    static int rseq_register_thread_at(struct rseq_abi *rseq)
    {
    let mut r: c_int = syscall(SYS_rseq, rseq, sizeof(*rseq), 0, RSEQ_SIG);
    return (r == 0 || errno == EBUSY || errno == EINVAL) ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn membarrier_register_rseq_mm() -> c_int {
    static int membarrier_register_rseq_mm(void)
    {
    return syscall(SYS_membarrier,
    MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ, 0, 0);
    }
// -- cgroup helpers --
#[no_mangle]
unsafe extern "C" fn rm_cgroup_recursive(path: *const c_char) {
    static void rm_cgroup_recursive(const char *path)
    {
    DIR *dir = opendir(path);
    if (!dir)
    return;
    struct dirent *entry;
    while ((entry = readdir(dir)) != core::ptr::null_mut()) {
    if (strcmp(entry.d_name, ".") == 0 || strcmp(entry.d_name, "..") == 0)
    continue;
    if (entry.d_type == DT_DIR) {
    char sub_path[4096];
    snprintf(sub_path, sizeof(sub_path), "%s/%s", path, entry.d_name);
    rm_cgroup_recursive(sub_path);
    }
    }
    closedir(dir);
    rmdir(path);
    }
    static void cgroup_teardown(void);
#[no_mangle]
unsafe extern "C" fn cgroup_setup() -> c_int {
    static int cgroup_setup(void)
    {
    struct stat st;
    if (stat("/sys/fs/cgroup/cpu", &st) == 0) {
    use_cgroup_v2 = 0;
    snprintf(g_cgroup_path, sizeof(g_cgroup_path),
    "/sys/fs/cgroup/cpu/membarrier_stress_test");
    } else if (stat("/dev/cgroup/cpu", &st) == 0) {
    use_cgroup_v2 = 0;
    snprintf(g_cgroup_path, sizeof(g_cgroup_path),
    "/dev/cgroup/cpu/membarrier_stress_test");
    } else if (stat("/cgroup/cpu", &st) == 0) {
    use_cgroup_v2 = 0;
    snprintf(g_cgroup_path, sizeof(g_cgroup_path),
    "/cgroup/cpu/membarrier_stress_test");
    } else if (stat("/sys/fs/cgroup/cgroup.controllers", &st) == 0) {
    use_cgroup_v2 = 1;
    snprintf(g_cgroup_path, sizeof(g_cgroup_path),
    "/sys/fs/cgroup/membarrier_stress_test");
    } else {
    ksft_print_msg("WARN: cgroup mount not found. Using v2 at /sys/fs/cgroup\n");
    use_cgroup_v2 = 1;
    snprintf(g_cgroup_path, sizeof(g_cgroup_path),
    "/sys/fs/cgroup/membarrier_stress_test");
    }
// Robust cleanup before setup
    cgroup_teardown();
    if (use_cgroup_v2) {
// Enable cpu controller in root cgroup
    if (write_file("/sys/fs/cgroup/cgroup.subtree_control", "+cpu") < 0)
    ksft_print_msg("WARN: failed to enable cpu controller in /sys/fs/cgroup\n");
    }
    if (mkdir(g_cgroup_path, 0755) < 0 && errno != EEXIST) {
    ksft_print_msg("mkdir base %s failed: %s\n", g_cgroup_path, strerror(errno));
    return -1;
    }
    if (use_cgroup_v2) {
    char ctrl_path[4096];
    snprintf(ctrl_path, sizeof(ctrl_path), "%s/cgroup.subtree_control", g_cgroup_path);
    if (write_file(ctrl_path, "+cpu") < 0)
    ksft_print_msg("WARN: failed to enable cpu controller in %s\n",
    g_cgroup_path);
    }
    for (int i = 0; i < N_SIBLINGS; i++) {
    char sibling_path[4096];
    snprintf(sibling_path, sizeof(sibling_path), "%s/n%d", g_cgroup_path, i);
    if (mkdir(sibling_path, 0755) < 0 && errno != EEXIST) {
    ksft_print_msg("mkdir wide %s failed: %s\n", sibling_path, strerror(errno));
    return -1;
    }
    if (use_cgroup_v2) {
    char ctrl_path[4096];
    snprintf(ctrl_path, sizeof(ctrl_path),
    "%s/cgroup.subtree_control", sibling_path);
    if (write_file(ctrl_path, "+cpu") < 0)
    ksft_print_msg("WARN: failed to enable cpu controller in %s\n",
    sibling_path);
    }
    char current_path[4096];
    snprintf(current_path, sizeof(current_path), "%s", sibling_path);
    for (int j = 0; j < NEST_DEPTH; j++) {
    snprintf(current_path + strlen(current_path),
    sizeof(current_path) - strlen(current_path), "/d%d", j);
    if (mkdir(current_path, 0755) < 0 && errno != EEXIST) {
    ksft_print_msg("mkdir deep %s failed: %s\n",
    current_path, strerror(errno));
    return -1;
    }
// Enable for all but the leaf
    if (use_cgroup_v2 && j < NEST_DEPTH - 1) {
    char ctrl_path[4096];
    snprintf(ctrl_path, sizeof(ctrl_path), "%s/cgroup.subtree_control",
    current_path);
    if (write_file(ctrl_path, "+cpu") < 0)
    ksft_print_msg("WARN: cannot enable cpu controller in %s\n",
    current_path);
    }
    }
    }
    char quota[64], period[64], max_str[128];
    snprintf(quota, sizeof(quota), "%d", CFS_QUOTA_US);
    snprintf(period, sizeof(period), "%d", CFS_PERIOD_US);
    snprintf(max_str, sizeof(max_str), "%d %d", CFS_QUOTA_US, CFS_PERIOD_US);
    if (use_cgroup_v2) {
    char max_path[4096];
    snprintf(max_path, sizeof(max_path), "%s/cpu.max", g_cgroup_path);
    if (write_file(max_path, max_str) < 0) {
    ksft_print_msg("ERROR: cannot write cpu.max at %s\n", max_path);
    return -1;
    }
    ksft_print_msg("cgroup (v2) %s: cpu.max=%s\n", g_cgroup_path, max_str);
    } else {
    char quota_path[4096], period_path[4096];
    snprintf(quota_path, sizeof(quota_path), "%s/cpu.cfs_quota_us", g_cgroup_path);
    snprintf(period_path, sizeof(period_path), "%s/cpu.cfs_period_us", g_cgroup_path);
    if (write_file(period_path, period) < 0) {
    ksft_print_msg("ERROR: cannot write cpu.cfs_period_us at %s\n",
    period_path);
    return -1;
    }
    if (write_file(quota_path, quota) < 0) {
    ksft_print_msg("ERROR: cannot write cpu.cfs_quota_us at %s\n", quota_path);
    return -1;
    }
    ksft_print_msg("cgroup (v1) %s: cpu.cfs_quota_us=%d cpu.cfs_period_us=%d\n",
    g_cgroup_path, CFS_QUOTA_US, CFS_PERIOD_US);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_add_pid_to_path(pid: pid_t, path: *const c_char) -> c_int {
    static int cgroup_add_pid_to_path(pid_t pid, const char *path)
    {
    char buf[32], file_path[4096];
    snprintf(buf, sizeof(buf), "%d", (int)pid);
    if (use_cgroup_v2) {
    snprintf(file_path, sizeof(file_path), "%s/cgroup.procs", path);
    return write_file(file_path, buf);
    }
// In v1, try tasks first, fallback to cgroup.procs
    snprintf(file_path, sizeof(file_path), "%s/tasks", path);
    let mut r: c_int = write_file(file_path, buf);
    if (r < 0) {
    snprintf(file_path, sizeof(file_path), "%s/cgroup.procs", path);
    r = write_file(file_path, buf);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn cgroup_teardown() {
    static void cgroup_teardown(void)
    {
    rm_cgroup_recursive(g_cgroup_path);
    }
#[no_mangle]
unsafe extern "C" fn cgroup_unthrottle() {
    static void cgroup_unthrottle(void)
    {
    if (use_cgroup_v2) {
    char max_path[4096];
    snprintf(max_path, sizeof(max_path), "%s/cpu.max", g_cgroup_path);
    write_file(max_path, "max");
    } else {
    char quota_path[4096];
    snprintf(quota_path, sizeof(quota_path), "%s/cpu.cfs_quota_us", g_cgroup_path);
    write_file(quota_path, "-1");
    }
    }
// -- CPU burner (inside throttled child process) --
    static void *burner_thread_fn(void *arg)
    {
    struct rseq_abi my_rseq;
    let mut cpu: c_int = (int)(uintptr_t)arg;
    memset(&my_rseq, 0, sizeof(my_rseq));
    my_rseq.cpu_id = RSEQ_CPU_ID_UNINITIALIZED;
    if (rseq_register_thread_at(&my_rseq) < 0) {
    perror("rseq_register (burner)");
    return core::ptr::null_mut();
    }
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(cpu, &set);
    if (sched_setaffinity(0, sizeof(set), &set) < 0)
    perror("sched_setaffinity (burner)");
    let mut sink: c_ulong = 0;
    while (!atomic_load_explicit(&g_stop, memory_order_relaxed)) {
    sink++;
// Prevent compiler from optimizing the loop away
    asm volatile("" : "+g"(sink));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn burner_thread_fn_wrapper(arg: *mut c_void) -> c_int {
    static int burner_thread_fn_wrapper(void *arg)
    {
    burner_thread_fn(arg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn leaf_child_fn(arg: *mut c_void) -> c_int {
    static int leaf_child_fn(void *arg)
    {
    let mut i: c_int = (int)(uintptr_t)arg;
    let mut total_burners: c_int = g_ncpus_stress * N_BURNER_PER_CPU;
    let mut n_threads_per_leaf: c_int = total_burners / N_SIBLINGS;
    if (i < (total_burners % N_SIBLINGS))
    n_threads_per_leaf++;
    prctl(PR_SET_PDEATHSIG, SIGTERM);
    if (getppid() == 1)
    _exit(1);
    char leaf_path[4096];
    snprintf(leaf_path, sizeof(leaf_path), "%s/n%d", g_cgroup_path, i);
    for (int j = 0; j < NEST_DEPTH; j++)
    snprintf(leaf_path + strlen(leaf_path),
    sizeof(leaf_path) - strlen(leaf_path), "/d%d", j);
    let mut r: c_int = cgroup_add_pid_to_path(getpid(), leaf_path);
    if (r < 0) {
    char buf[512];
    int len = snprintf(buf, sizeof(buf),
    "[leaf child %d] failed to join cgroup %s: err %d\n",
    i, leaf_path, -r);
    (void)!write(2, buf, len);
    _exit(1);
    }
    for (int j = 0; j < n_threads_per_leaf; j++) {
    let mut cpu: c_int = g_stress_cpus[(i * n_threads_per_leaf + j) % g_ncpus_stress];
// Allocate stack via mmap (bypasses heap)
    let mut stack_size: usize = 64 * 1024;
    void *stack = mmap(core::ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (stack == MAP_FAILED) {
    const char *msg = "mmap stack failed\n";
    (void)!write(2, msg, strlen(msg));
    _exit(1);
    }
// Use raw clone to create a thread sharing the VM and thread group
    pid_t pid = clone(burner_thread_fn_wrapper, stack + stack_size,
    CLONE_VM | CLONE_THREAD | CLONE_SIGHAND,
    (void *)(uintptr_t)cpu);
    if (pid < 0) {
    const char *msg = "clone burner failed\n";
    (void)!write(2, msg, strlen(msg));
    _exit(1);
    }
    }
// Wait for SIGTERM
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGTERM);
    int sig;
    sigwait(&mask, &sig);
    _exit(0);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct leaf_info {
    pub pid: pid_t,
    pub stack: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn run_throttle_child(arg: *mut c_void) -> c_int {
    static int run_throttle_child(void *arg)
    {
    (void)arg;
    prctl(PR_SET_PDEATHSIG, SIGTERM);
    if (getppid() == 1)
    _exit(1);
    let mut n_leafs: c_int = N_SIBLINGS;
// Block signals before spawning to avoid missing early failures
    sigset_t mask;
    sigemptyset(&mask);
    sigaddset(&mask, SIGTERM);
    sigaddset(&mask, SIGCHLD);
    sigprocmask(SIG_BLOCK, &mask, core::ptr::null_mut());
// Use mmap for tracking structures to avoid glibc heap usage
    struct leaf_info *leaves = mmap(core::ptr::null_mut(), n_leafs * sizeof(struct leaf_info),
    PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (leaves == MAP_FAILED) {
    const char *msg = "mmap leaves array failed\n";
    (void)!write(2, msg, strlen(msg));
    _exit(1);
    }
    for (int i = 0; i < n_leafs; i++) {
    let mut stack_size: usize = 64 * 1024;
    void *stack = mmap(core::ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (stack == MAP_FAILED) {
    const char *msg = "mmap leaf stack failed\n";
    (void)!write(2, msg, strlen(msg));
    _exit(1);
    }
    leaves[i].stack = stack;
    pid_t pid = clone(leaf_child_fn, stack + stack_size,
    CLONE_VM | SIGCHLD, (void *)(uintptr_t)i);
    if (pid < 0) {
    const char *msg = "clone (leaf child) failed\n";
    (void)!write(2, msg, strlen(msg));
// Clean up successfully spawned children
    for (int j = 0; j < i; j++) {
    kill(leaves[j].pid, SIGTERM);
    waitpid(leaves[j].pid, core::ptr::null_mut(), 0);
    munmap(leaves[j].stack, stack_size);
    }
    munmap(leaves, n_leafs * sizeof(struct leaf_info));
    if (errno == EAGAIN)
    _exit(4);
    else
    _exit(1);
    }
    leaves[i].pid = pid;
    }
    let mut failed: c_int = 0;
    while (1) {
    int sig;
    sigwait(&mask, &sig);
    if (sig == SIGTERM) {
    break;
    } else if (sig == SIGCHLD) {
    int status;
    pid_t pid;
// Reap all dead children
    while ((pid = waitpid(-1, &status, WNOHANG)) > 0) {
    for (int i = 0; i < n_leafs; i++) {
    if (leaves[i].pid == pid) {
    leaves[i].pid = 0;
    break;
    }
    }
    if ((WIFEXITED(status) && WEXITSTATUS(status) != 0) ||
    WIFSIGNALED(status)) {
    char buf[128];
    int len = snprintf(buf, sizeof(buf),
    "[manager] child %d died unexpectedly (status %d)\n",
    pid, WEXITSTATUS(status));
    (void)!write(2, buf, len);
    failed = 1;
    }
    }
    if (failed)
    break;
    }
    }
// Terminate all leaf kids
    for (int i = 0; i < n_leafs; i++) {
    if (leaves[i].pid > 0)
    kill(leaves[i].pid, SIGTERM);
    }
    for (int i = 0; i < n_leafs; i++) {
    if (leaves[i].pid > 0)
    waitpid(leaves[i].pid, core::ptr::null_mut(), 0);
    munmap(leaves[i].stack, 64 * 1024);
    }
    munmap(leaves, n_leafs * sizeof(struct leaf_info));
    _exit(failed ? 1 : 0);
    }
// -- Membarrier hammer thread --
    static void *hammer_thread_fn(void *arg)
    {
    let mut target_cpu: c_int = *(int *)arg;
    let mut local_ok: c_long = 0;
    let mut local_err: c_long = 0;
    let mut count: c_int = 0;
    let mut batch_size: c_int = 1024;
    if (rseq_register_thread() < 0) {
    ksft_print_msg("[hammer] rseq_register failed: %s\n", strerror(errno));
    return core::ptr::null_mut();
    }
    membarrier_register_rseq_mm();
    while (!atomic_load_explicit(&g_stop, memory_order_relaxed)) {
    int r = syscall(SYS_membarrier,
    MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ,
    MEMBARRIER_CMD_FLAG_CPU,
    target_cpu);
    if (__builtin_expect(r == 0, 1))
    local_ok++;
    else
    local_err++;
    count++;
    if (__builtin_expect(count >= batch_size, 0)) {
    atomic_fetch_add_explicit(&g_mb_ok, local_ok, memory_order_relaxed);
    atomic_fetch_add_explicit(&g_mb_err, local_err, memory_order_relaxed);
    local_ok = 0;
    local_err = 0;
    count = 0;
    }
    }
// Flush any remaining counts on exit
    if (local_ok > 0)
    atomic_fetch_add_explicit(&g_mb_ok, local_ok, memory_order_relaxed);
    if (local_err > 0)
    atomic_fetch_add_explicit(&g_mb_err, local_err, memory_order_relaxed);
    return core::ptr::null_mut();
    }
// -- Latency sentinel --
    static void *sentinel_thread_fn(void *arg)
    {
    (void)arg;
    let mut sp: sched_param = { .sched_priority = 20 };
    if (sched_setscheduler(0, SCHED_FIFO, &sp) < 0)
    ksft_print_msg("WARN: no SCHED_FIFO for sentinel (less precise)\n");
    while (!atomic_load_explicit(&g_test_ready, memory_order_relaxed) &&
    !atomic_load_explicit(&g_stop_sentinel, memory_order_relaxed)) {
    struct timespec ts = {0, 1000 * 1000}; /* 1ms */
    clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, core::ptr::null_mut());
    }
    let mut prev: u64 = monotonic_us();
    while (!atomic_load_explicit(&g_stop_sentinel, memory_order_relaxed)) {
    struct timespec ts = {
    .tv_sec  = 0,
    .tv_nsec = SENTINEL_INTERVAL_US * 1000L,
    };
    clock_nanosleep(CLOCK_MONOTONIC, 0, &ts, core::ptr::null_mut());
    let mut now: u64 = monotonic_us();
    let mut latency_us: c_long = (long)(now - prev) - SENTINEL_INTERVAL_US;
    prev = now;
    if (latency_us <= 0)
    continue;
    update_max_latency(latency_us);
    if (latency_us > LATENCY_CRITICAL_MS * 1000L) {
    ksft_print_msg("\n[SENTINEL] CRITICAL: %ld ms delay (lockup precursor!)\n",
    latency_us / 1000);
    } else if (latency_us > LATENCY_WARN_MS * 1000L) {
    ksft_print_msg("\n[SENTINEL] WARN: %ld ms latency spike\n",
    latency_us / 1000);
    }
    }
    return core::ptr::null_mut();
    }
// -- Progress reporter --
    static void *reporter_thread_fn(void *arg)
    {
    (void)arg;
    let mut elapsed: c_int = 0;
    while (!atomic_load_explicit(&g_stop_sentinel, memory_order_relaxed)) {
    for (int i = 0; i < 5; i++) {
    sleep(1);
    if (atomic_load_explicit(&g_stop_sentinel, memory_order_relaxed))
    break;
    }
    if (atomic_load_explicit(&g_stop_sentinel, memory_order_relaxed))
    break;
    elapsed += 5;
    long interval_max = atomic_exchange_explicit(&g_interval_max_latency_us,
    0, memory_order_relaxed);
    ksft_print_msg("[%3ds] mb: ok=%-10ld err=%-8ld | max_lat=%ld us\n",
    elapsed,
    atomic_load(&g_mb_ok),
    atomic_load(&g_mb_err),
    interval_max);
    }
    return core::ptr::null_mut();
    }
// -- Main --
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    ksft_print_header();

    ksft_exit_skip("Unsupported architecture\n");

    ksft_set_plan(1);
    if (geteuid() != 0)
    ksft_exit_skip("Must run as root (cgroup + SCHED_FIFO)\n");
    init_stress_cpus();
    ksft_print_msg("=== membarrier rseq + CFS unthrottle stress ===\n");
    ksft_print_msg("Stressing CPUs: %d\n", g_ncpus_stress);
    ksft_print_msg("Quota: %d/%d us  (~%d unthrottles/sec/CPU)\n",
    CFS_QUOTA_US, CFS_PERIOD_US,
    1000000 / CFS_PERIOD_US);
    ksft_print_msg("Hammer threads: %d per CPU (%d total)\n",
    N_HAMMER_PER_CPU, g_ncpus_stress * N_HAMMER_PER_CPU);
    ksft_print_msg("Duration: %d seconds\n\n", TEST_DURATION_SEC);
    if (cgroup_setup() < 0) {
    cgroup_teardown();
    ksft_exit_skip("cgroup_setup failed (missing permissions or v2 ctrls?)\n");
    }
    if (rseq_register_thread() < 0) {
    ksft_print_msg("rseq_register (%s) failed: %s\n", __func__, strerror(errno));
    cgroup_teardown();
    ksft_exit_skip("rseq syscall failed or not available\n");
    }
    if (membarrier_register_rseq_mm() < 0) {
    ksft_print_msg("MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ: %s\n"
    "Kernel >= 5.10 with CONFIG_RSEQ required.\n",
    strerror(errno));
    cgroup_teardown();
    ksft_exit_skip("membarrier register failed\n");
    }
    ksft_print_msg("rseq membarrier registered OK\n");
    sigset_t sigmask;
    sigemptyset(&sigmask);
    sigaddset(&sigmask, SIGTERM);
    sigprocmask(SIG_BLOCK, &sigmask, core::ptr::null_mut());
    void *stack = malloc(1024 * 1024);
    if (!stack) {
    perror("malloc stack");
    cgroup_teardown();
    ksft_exit_fail_msg("Malloc stack failed\n");
    }
    let mut child: pid_t = clone(run_throttle_child, stack + 1024 * 1024, CLONE_VM | SIGCHLD, core::ptr::null_mut());
    if (child < 0) {
    perror("clone");
    cgroup_teardown();
    ksft_exit_fail_msg("Clone failed\n");
    }
    sigprocmask(SIG_UNBLOCK, &sigmask, core::ptr::null_mut());
    ksft_print_msg("Throttle child PID %d started\n", child);
    let mut n_threads: c_int = g_ncpus_stress * N_HAMMER_PER_CPU + 2;
    pthread_t *threads = (pthread_t *)calloc(n_threads, sizeof(pthread_t));
    int       *cpuargs = (int *)calloc(g_ncpus_stress * N_HAMMER_PER_CPU, sizeof(int));
    if (!threads || !cpuargs) {
    perror("calloc");
    kill(child, SIGTERM);
    waitpid(child, core::ptr::null_mut(), 0);
    cgroup_teardown();
    ksft_exit_fail_msg("Thread allocation failed\n");
    }
    let mut ti: c_int = 0, ai = 0;
    int r;
    ksft_print_msg("Creating sentinel thread...\n");
    r = pthread_create(&threads[ti], core::ptr::null_mut(), sentinel_thread_fn, core::ptr::null_mut());
    if (r != 0) {
    kill(child, SIGTERM);
    waitpid(child, core::ptr::null_mut(), 0);
    cgroup_teardown();
    free(threads);
    free(cpuargs);
    free(g_stress_cpus);
    ksft_exit_fail_msg("pthread_create (sentinel) failed: %s\n", strerror(r));
    }
    ti++;
    ksft_print_msg("Creating reporter thread...\n");
    r = pthread_create(&threads[ti], core::ptr::null_mut(), reporter_thread_fn, core::ptr::null_mut());
    if (r != 0) {
    atomic_store(&g_stop_sentinel, 1);
    pthread_join(threads[0], core::ptr::null_mut());
    kill(child, SIGTERM);
    waitpid(child, core::ptr::null_mut(), 0);
    cgroup_teardown();
    free(threads);
    free(cpuargs);
    free(g_stress_cpus);
    ksft_exit_fail_msg("pthread_create (reporter) failed: %s\n", strerror(r));
    }
    ti++;
    ksft_print_msg("Creating %d hammer threads...\n", g_ncpus_stress * N_HAMMER_PER_CPU);
    for (int i = 0; i < g_ncpus_stress; i++) {
    let mut cpu: c_int = g_stress_cpus[i];
    for (int j = 0; j < N_HAMMER_PER_CPU; j++) {
    cpuargs[ai] = cpu;
    r = pthread_create(&threads[ti], core::ptr::null_mut(), hammer_thread_fn, &cpuargs[ai]);
    if (r != 0) {
    ksft_print_msg("pthread_create failed at thread %d: %s\n",
    ti, strerror(r));
    atomic_store(&g_stop_sentinel, 1);
    pthread_join(threads[0], core::ptr::null_mut());
    pthread_join(threads[1], core::ptr::null_mut());
    atomic_store(&g_stop, 1);
    for (int k = 2; k < ti; k++)
    pthread_join(threads[k], core::ptr::null_mut());
    kill(child, SIGTERM);
    waitpid(child, core::ptr::null_mut(), 0);
    cgroup_teardown();
    free(threads);
    free(cpuargs);
    free(g_stress_cpus);
    if (r == EAGAIN)
    ksft_exit_skip("Resource limits prevent threads\n");
    else
    ksft_exit_fail_msg("Failed to create hammer thread\n");
    }
    ti++;
    ai++;
    }
    }
    ksft_print_msg("All threads running. Tip: monitor dmesg for lockups\n\n");
    atomic_store_explicit(&g_test_ready, 1, memory_order_relaxed);
    let mut child_failed: c_int = 0;
    let mut child_status: c_int = 0;
    for (int i = 0; i < TEST_DURATION_SEC; i++) {
    sleep(1);
    let mut r: c_int = waitpid(child, &child_status, WNOHANG);
    if (r == child) {
    child_failed = 1;
    break;
    }
    }
    atomic_store(&g_stop_sentinel, 1);
    pthread_join(threads[0], core::ptr::null_mut());
    pthread_join(threads[1], core::ptr::null_mut());
    atomic_store(&g_stop, 1);
// Unthrottle to allow children to exit quickly
    cgroup_unthrottle();
    if (!child_failed) {
    kill(child, SIGTERM);
    waitpid(child, core::ptr::null_mut(), 0);
    }
    for (int i = 2; i < ti; i++)
    pthread_join(threads[i], core::ptr::null_mut());
    let mut max_lat: c_long = atomic_load(&g_max_latency_us);
    let mut total_ok: c_long = atomic_load(&g_mb_ok);
    let mut total_err: c_long = atomic_load(&g_mb_err);
    ksft_print_msg("\n=== RESULTS ===\n");
    ksft_print_msg("membarrier syscalls : %ld ok  %ld errors\n", total_ok, total_err);
    ksft_print_msg("Max scheduler latency: %ld us  (%ld ms)\n", max_lat, max_lat / 1000);
    cgroup_teardown();
    free(threads);
    free(cpuargs);
    free(g_stress_cpus);
    if (child_failed) {
    if (WIFEXITED(child_status) && WEXITSTATUS(child_status) == 4)
    ksft_exit_skip("Manager child skipped (resource limits?)\n");
    ksft_test_result_fail("membarrier_rseq_stress: Manager child died early\n");
    ksft_exit_fail();
    } else if (total_ok == 0) {
    ksft_test_result_fail("membarrier_rseq_stress: No successful membarrier calls\n");
    ksft_exit_fail();
    } else if (total_err > 0) {
    ksft_test_result_fail("membarrier_rseq_stress: syscall errors\n");
    ksft_exit_fail();
    } else if (max_lat > LATENCY_CRITICAL_MS * 1000L) {
    ksft_test_result_fail("membarrier_rseq_stress: LOCKUP PRECURSOR\n");
    ksft_exit_fail();
    } else if (max_lat > LATENCY_WARN_MS * 1000L) {
    ksft_test_result_fail("membarrier_rseq_stress: significant latency spike\n");
    ksft_exit_fail();
    } else {
    ksft_test_result_pass("membarrier_rseq_stress\n");
    ksft_exit_pass();
    }
    return 0;
    }
