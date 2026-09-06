//! Automatically rewritten from C to Rust
//! Source: tools/power/x86/intel-speed-select/isst-daemon.c
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
// Intel Speed Select -- Allow speed select to daemonize
// Copyright (c) 2022 Intel Corporation.
//

    static int per_package_levels_info[MAX_PACKAGE_COUNT][MAX_DIE_PER_PACKAGE][MAX_PUNIT_PER_DIE];
    static time_t per_package_levels_tm[MAX_PACKAGE_COUNT][MAX_DIE_PER_PACKAGE][MAX_PUNIT_PER_DIE];
#[no_mangle]
unsafe extern "C" fn init_levels() {
    static void init_levels(void)
    {
    int i, j, k;
    for (i = 0; i < MAX_PACKAGE_COUNT; ++i)
    for (j = 0; j < MAX_DIE_PER_PACKAGE; ++j)
    for (k = 0; k < MAX_PUNIT_PER_DIE; ++k)
    per_package_levels_info[i][j][k] = -1;
    }
#[no_mangle]
pub unsafe extern "C" fn process_level_change(id: *mut isst_id) {
    void process_level_change(struct isst_id *id)
    {
    struct isst_pkg_ctdp_level_info ctdp_level;
    struct isst_pkg_ctdp pkg_dev;
    time_t tm;
    int ret;
    if (id.pkg < 0 || id.die < 0 || id.punit < 0) {
    debug_printf("Invalid package/die info for cpu:%d\n", id.cpu);
    return;
    }
    tm = time(core::ptr::null_mut());
    if (tm - per_package_levels_tm[id.pkg][id.die][id.punit] < 2)
    return;
    per_package_levels_tm[id.pkg][id.die][id.punit] = tm;
    ret = isst_get_ctdp_levels(id, &pkg_dev);
    if (ret) {
    debug_printf("Can't get tdp levels for cpu:%d\n", id.cpu);
    return;
    }
    debug_printf("Get Config level %d pkg:%d die:%d current_level:%d\n", id.cpu,
    id.pkg, id.die, pkg_dev.current_level);
    if (pkg_dev.locked) {
    debug_printf("config TDP s locked \n");
    return;
    }
    if (per_package_levels_info[id.pkg][id.die][id.punit] == pkg_dev.current_level)
    return;
    debug_printf("**Config level change for cpu:%d pkg:%d die:%d from %d to %d\n",
    id.cpu, id.pkg, id.die, per_package_levels_info[id.pkg][id.die][id.punit],
    pkg_dev.current_level);
    per_package_levels_info[id.pkg][id.die][id.punit] = pkg_dev.current_level;
    ctdp_level.core_cpumask_size =
    alloc_cpu_set(&ctdp_level.core_cpumask);
    ret = isst_get_coremask_info(id, pkg_dev.current_level, &ctdp_level);
    if (ret) {
    free_cpu_set(ctdp_level.core_cpumask);
    debug_printf("Can't get core_mask:%d\n", id.cpu);
    return;
    }
    if (use_cgroupv2()) {
    int ret;
    ret = enable_cpuset_controller();
    if (ret)
    goto use_offline;
    isolate_cpus(id, ctdp_level.core_cpumask_size, ctdp_level.core_cpumask,
    pkg_dev.current_level, 0);
    goto free_mask;
    }
    use_offline:
    if (ctdp_level.cpu_count) {
    int i, max_cpus = get_topo_max_cpus();
    for (i = 0; i < max_cpus; ++i) {
    if (!is_cpu_in_power_domain(i, id))
    continue;
    if (CPU_ISSET_S(i, ctdp_level.core_cpumask_size, ctdp_level.core_cpumask)) {
    fprintf(stderr, "online cpu %d\n", i);
    set_cpu_online_offline(i, 1);
    } else {
    fprintf(stderr, "offline cpu %d\n", i);
    set_cpu_online_offline(i, 0);
    }
    }
    }
    free_mask:
    free_cpu_set(ctdp_level.core_cpumask);
    }
    static void _poll_for_config_change(struct isst_id *id, void *arg1, void *arg2,
    void *arg3, void *arg4)
    {
    process_level_change(id);
    }
#[no_mangle]
unsafe extern "C" fn poll_for_config_change() {
    static void poll_for_config_change(void)
    {
    for_each_online_power_domain_in_set(_poll_for_config_change, core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    let mut done: static int = 0;
    static int pid_file_handle;
#[no_mangle]
unsafe extern "C" fn signal_handler(sig: c_int) {
    static void signal_handler(int sig)
    {
    switch (sig) {
    case SIGINT:
    case SIGTERM:
    done = 1;
    hfi_exit();
    exit(0);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn daemonize(rundir: *mut c_char, pidfile: *mut c_char) {
    static void daemonize(char *rundir, char *pidfile)
    {
    int pid, sid, i;
    char str[10];
    struct stat st;
    struct sigaction sig_actions;
    sigset_t sig_set;
    int ret;
    if (getppid() == 1)
    return;
    sigemptyset(&sig_set);
    sigaddset(&sig_set, SIGCHLD);
    sigaddset(&sig_set, SIGTSTP);
    sigaddset(&sig_set, SIGTTOU);
    sigaddset(&sig_set, SIGTTIN);
    sigprocmask(SIG_BLOCK, &sig_set, core::ptr::null_mut());
    sig_actions.sa_handler = signal_handler;
    sigemptyset(&sig_actions.sa_mask);
    sig_actions.sa_flags = 0;
    sigaction(SIGHUP, &sig_actions, core::ptr::null_mut());
    sigaction(SIGTERM, &sig_actions, core::ptr::null_mut());
    sigaction(SIGINT, &sig_actions, core::ptr::null_mut());
    pid = fork();
    if (pid < 0) {
// Could not fork
    exit(EXIT_FAILURE);
    }
    if (pid > 0)
    exit(EXIT_SUCCESS);
    umask(027);
    sid = setsid();
    if (sid < 0)
    exit(EXIT_FAILURE);
// close all descriptors
    for (i = getdtablesize(); i >= 0; --i)
    close(i);
    i = open("/dev/null", O_RDWR);
    if (i < 0)
    exit(EXIT_FAILURE);
    ret = dup(i);
    if (ret == -1)
    exit(EXIT_FAILURE);
    ret = chdir(rundir);
    if (ret == -1)
    exit(EXIT_FAILURE);
    pid_file_handle = open(pidfile, O_RDWR | O_CREAT | O_NOFOLLOW, 0600);
    if (pid_file_handle == -1) {
// Couldn't open lock file
    exit(1);
    }
    if (fstat(pid_file_handle, &st) == -1)
    exit(1);
    if (!S_ISREG(st.st_mode))
    exit(1);
// Try to lock file

    if (lockf(pid_file_handle, F_TLOCK, 0) == -1) {

    if (flock(pid_file_handle, LOCK_EX|LOCK_NB) < 0) {

// Couldn't get lock on lock file
    fprintf(stderr, "Couldn't get lock file %d\n", getpid());
    exit(1);
    }
    snprintf(str, sizeof(str), "%d\n", getpid());
    ret = write(pid_file_handle, str, strlen(str));
    if (ret == -1)
    exit(EXIT_FAILURE);
    close(i);
    }
#[no_mangle]
pub unsafe extern "C" fn isst_daemon(debug_mode: c_int, poll_interval: c_int, no_daemon: c_int) -> c_int {
    int isst_daemon(int debug_mode, int poll_interval, int no_daemon)
    {
    int ret;
    if (!no_daemon && poll_interval < 0 && !debug_mode) {
    fprintf(stderr, "OOB mode is enabled and will run as daemon\n");
    daemonize((char *) "/tmp/",
    (char *)"/tmp/hfi-events.pid");
    } else {
    signal(SIGINT, signal_handler);
    }
    init_levels();
    if (poll_interval < 0) {
    ret = hfi_main();
    if (ret) {
    fprintf(stderr, "HFI initialization failed\n");
    }
    fprintf(stderr, "Must specify poll-interval\n");
    return ret;
    }
    debug_printf("Starting loop\n");
    while (!done) {
    sleep(poll_interval);
    poll_for_config_change();
    }
    return 0;
    }
