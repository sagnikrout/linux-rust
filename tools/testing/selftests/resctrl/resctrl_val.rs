//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/resctrl/resctrl_val.c
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
// Memory bandwidth monitoring and allocation library
//
// Copyright (C) 2018 Intel Corporation
//
// Authors:
// Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
// Fenghua Yu <fenghua.yu@intel.com>
//

pub const MAX_IMCS: c_int = 40;
pub const MAX_TOKENS: c_int = 5;

    "%s/%s/mon_data/mon_L3_%02d/mbm_local_bytes"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct membw_read_format {
    pub /: *mut *mut __u64 value; / The value of the event,
    pub /: *mut *mut __u64 time_enabled; / if PERF_FORMAT_TOTAL_TIME_ENABLED,
    pub /: *mut *mut __u64 time_running; / if PERF_FORMAT_TOTAL_TIME_RUNNING,
    pub /: *mut *mut __u64 id; / if PERF_FORMAT_ID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_counter_config {
    pub type: __u32,
    pub event: __u64,
    pub umask: __u64,
    pub pe: perf_event_attr,
    pub fd: c_int,
}

    static char mbm_total_path[1024];
    static int imcs;
    static struct imc_counter_config imc_counters_config[MAX_IMCS];
    static const struct resctrl_test *current_test;
#[no_mangle]
unsafe extern "C" fn read_mem_bw_initialize_perf_event_attr(i: c_int) {
    static void read_mem_bw_initialize_perf_event_attr(int i)
    {
    memset(&imc_counters_config[i].pe, 0,
    sizeof(struct perf_event_attr));
    imc_counters_config[i].pe.type = imc_counters_config[i].type;
    imc_counters_config[i].pe.size = sizeof(struct perf_event_attr);
    imc_counters_config[i].pe.disabled = 1;
    imc_counters_config[i].pe.inherit = 1;
    imc_counters_config[i].pe.exclude_guest = 0;
    imc_counters_config[i].pe.config =
    imc_counters_config[i].umask << 8 |
    imc_counters_config[i].event;
    imc_counters_config[i].pe.sample_type = PERF_SAMPLE_IDENTIFIER;
    imc_counters_config[i].pe.read_format =
    PERF_FORMAT_TOTAL_TIME_ENABLED | PERF_FORMAT_TOTAL_TIME_RUNNING;
    }
#[no_mangle]
unsafe extern "C" fn read_mem_bw_ioctl_perf_event_ioc_reset_enable(i: c_int) {
    static void read_mem_bw_ioctl_perf_event_ioc_reset_enable(int i)
    {
    ioctl(imc_counters_config[i].fd, PERF_EVENT_IOC_RESET, 0);
    ioctl(imc_counters_config[i].fd, PERF_EVENT_IOC_ENABLE, 0);
    }
#[no_mangle]
unsafe extern "C" fn read_mem_bw_ioctl_perf_event_ioc_disable(i: c_int) {
    static void read_mem_bw_ioctl_perf_event_ioc_disable(int i)
    {
    ioctl(imc_counters_config[i].fd, PERF_EVENT_IOC_DISABLE, 0);
    }
//
// get_read_event_and_umask:	Parse config into event and umask
// @cas_count_cfg:	Config
// @count:		iMC number
//
#[no_mangle]
unsafe extern "C" fn get_read_event_and_umask(cas_count_cfg: *mut c_char, count: c_uint) {
    static void get_read_event_and_umask(char *cas_count_cfg, unsigned int count)
    {
    char *token[MAX_TOKENS];
    let mut i: c_int = 0;
    token[0] = strtok(cas_count_cfg, "=,");
    for (i = 1; i < MAX_TOKENS; i++)
    token[i] = strtok(core::ptr::null_mut(), "=,");
    for (i = 0; i < MAX_TOKENS - 1; i++) {
    if (!token[i])
    break;
    if (strcmp(token[i], "event") == 0)
    imc_counters_config[count].event = strtol(token[i + 1], core::ptr::null_mut(), 16);
    if (strcmp(token[i], "umask") == 0)
    imc_counters_config[count].umask = strtol(token[i + 1], core::ptr::null_mut(), 16);
    }
    }
#[no_mangle]
unsafe extern "C" fn open_perf_read_event(i: c_int, cpu_no: c_int) -> c_int {
    static int open_perf_read_event(int i, int cpu_no)
    {
    imc_counters_config[i].fd =
    perf_event_open(&imc_counters_config[i].pe, -1, cpu_no, -1,
    PERF_FLAG_FD_CLOEXEC);
    if (imc_counters_config[i].fd == -1) {
    fprintf(stderr, "Error opening leader %llx\n",
    imc_counters_config[i].pe.config);
    return -1;
    }
    return 0;
    }
    static int parse_imc_read_bw_events(char *imc_dir, unsigned int type,
    unsigned int *count)
    {
    char imc_events_dir[PATH_MAX], imc_counter_cfg[PATH_MAX];
    let mut orig_count: c_uint = *count;
    char cas_count_cfg[1024];
    struct dirent *ep;
    int path_len;
    let mut ret: c_int = -1;
    int num_cfg;
    FILE *fp;
    DIR *dp;
    path_len = snprintf(imc_events_dir, sizeof(imc_events_dir), "%sevents",
    imc_dir);
    if (path_len >= sizeof(imc_events_dir)) {
    ksft_print_msg("Unable to create path to %sevents\n", imc_dir);
    return -1;
    }
    dp = opendir(imc_events_dir);
    if (!dp) {
    ksft_perror("Unable to open PMU events directory");
    return -1;
    }
    while ((ep = readdir(dp))) {
//
// Parse all event files with READ_FILE_NAME prefix that
// contain the event number and umask. Skip files containing
// "." that contain unused properties of event.
//
    if (!strstr(ep.d_name, READ_FILE_NAME) ||
    strchr(ep.d_name, '.'))
    continue;
    path_len = snprintf(imc_counter_cfg, sizeof(imc_counter_cfg),
    "%s/%s", imc_events_dir, ep.d_name);
    if (path_len >= sizeof(imc_counter_cfg)) {
    ksft_print_msg("Unable to create path to %s/%s\n",
    imc_events_dir, ep.d_name);
    goto out_close;
    }
    fp = fopen(imc_counter_cfg, "r");
    if (!fp) {
    ksft_perror("Failed to open iMC config file");
    goto out_close;
    }
    num_cfg = fscanf(fp, "%1023s", cas_count_cfg);
    fclose(fp);
    if (num_cfg <= 0) {
    ksft_perror("Could not get iMC cas count read");
    goto out_close;
    }
    if (*count >= MAX_IMCS) {
    ksft_print_msg("Maximum iMC count exceeded\n");
    goto out_close;
    }
    imc_counters_config[*count].type = type;
    get_read_event_and_umask(cas_count_cfg, *count);
// Do not fail after incrementing *count.
// count += 1;
    }
    if (*count == orig_count) {
    ksft_print_msg("Unable to find events in %s\n", imc_events_dir);
    goto out_close;
    }
    ret = 0;
    out_close:
    closedir(dp);
    return ret;
    }
// Get type and config of an iMC counter's read event.
#[no_mangle]
unsafe extern "C" fn read_from_imc_dir(imc_dir: *mut c_char, count: *mut c_uint) -> c_int {
    static int read_from_imc_dir(char *imc_dir, unsigned int *count)
    {
    char imc_counter_type[PATH_MAX];
    unsigned int type;
    int path_len;
    FILE *fp;
    int ret;
// Get type of iMC counter
    path_len = snprintf(imc_counter_type, sizeof(imc_counter_type),
    "%s%s", imc_dir, "type");
    if (path_len >= sizeof(imc_counter_type)) {
    ksft_print_msg("Unable to create path to %s%s\n",
    imc_dir, "type");
    return -1;
    }
    fp = fopen(imc_counter_type, "r");
    if (!fp) {
    ksft_perror("Failed to open iMC counter type file");
    return -1;
    }
    ret = fscanf(fp, "%u", &type);
    fclose(fp);
    if (ret <= 0) {
    ksft_perror("Could not get iMC type");
    return -1;
    }
    ret = parse_imc_read_bw_events(imc_dir, type, count);
    if (ret) {
    ksft_print_msg("Unable to parse bandwidth event and umask\n");
    return ret;
    }
    return 0;
    }
//
// A system can have 'n' number of iMC (Integrated Memory Controller)
// counters, get that 'n'. Discover the properties of the available
// counters in support of needed performance measurement via perf.
// For each iMC counter get it's type and config. Also obtain each
// counter's event and umask for the memory read events that will be
// measured.
//
// Enumerate all these details into an array of structures.
//
// Return: >= 0 on success. < 0 on failure.
//
#[no_mangle]
unsafe extern "C" fn num_of_imcs() -> c_int {
    static int num_of_imcs(void)
    {
    char imc_dir[512], *temp;
    let mut count: c_uint = 0;
    struct dirent *ep;
    int ret;
    DIR *dp;
    dp = opendir(DYN_PMU_PATH);
    if (dp) {
    while ((ep = readdir(dp))) {
    temp = strstr(ep.d_name, UNCORE_IMC);
    if (!temp)
    continue;
//
// imc counters are named as "uncore_imc_<n>", hence
// increment the pointer to point to <n>. Note that
// sizeof(UNCORE_IMC) would count for null character as
// well and hence the last underscore character in
// uncore_imc'_' need not be counted.
//
    temp = temp + sizeof(UNCORE_IMC);
//
// Some directories under "DYN_PMU_PATH" could have
// names like "uncore_imc_free_running", hence, check if
// first character is a numerical digit or not.
//
    if (temp[0] >= '0' && temp[0] <= '9') {
    sprintf(imc_dir, "%s/%s/", DYN_PMU_PATH,
    ep.d_name);
    ret = read_from_imc_dir(imc_dir, &count);
    if (ret) {
    closedir(dp);
    return ret;
    }
    }
    }
    closedir(dp);
    if (count == 0) {
    ksft_print_msg("Unable to find iMC counters\n");
    return -1;
    }
    } else {
    ksft_perror("Unable to open PMU directory");
    return -1;
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn initialize_read_mem_bw_imc() -> c_int {
    int initialize_read_mem_bw_imc(void)
    {
    int imc;
    imcs = num_of_imcs();
    if (imcs <= 0)
    return imcs;
// Initialize perf_event_attr structures for all iMC's
    for (imc = 0; imc < imcs; imc++)
    read_mem_bw_initialize_perf_event_attr(imc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn perf_close_imc_read_mem_bw() {
    static void perf_close_imc_read_mem_bw(void)
    {
    int mc;
    for (mc = 0; mc < imcs; mc++) {
    if (imc_counters_config[mc].fd != -1)
    close(imc_counters_config[mc].fd);
    }
    }
//
// perf_open_imc_read_mem_bw - Open perf fds for IMCs
// @cpu_no: CPU number that the benchmark PID is bound to
//
// Return: = 0 on success. < 0 on failure.
//
#[no_mangle]
unsafe extern "C" fn perf_open_imc_read_mem_bw(cpu_no: c_int) -> c_int {
    static int perf_open_imc_read_mem_bw(int cpu_no)
    {
    int imc, ret;
    for (imc = 0; imc < imcs; imc++)
    imc_counters_config[imc].fd = -1;
    for (imc = 0; imc < imcs; imc++) {
    ret = open_perf_read_event(imc, cpu_no);
    if (ret)
    goto close_fds;
    }
    return 0;
    close_fds:
    perf_close_imc_read_mem_bw();
    return -1;
    }
//
// do_imc_read_mem_bw_test - Perform memory bandwidth test
//
// Runs memory bandwidth test over one second period. Also, handles starting
// and stopping of the IMC perf counters around the test.
//
#[no_mangle]
unsafe extern "C" fn do_imc_read_mem_bw_test() {
    static void do_imc_read_mem_bw_test(void)
    {
    int imc;
    for (imc = 0; imc < imcs; imc++)
    read_mem_bw_ioctl_perf_event_ioc_reset_enable(imc);
    sleep(1);
// Stop counters after a second to get results.
    for (imc = 0; imc < imcs; imc++)
    read_mem_bw_ioctl_perf_event_ioc_disable(imc);
    }
//
// get_read_mem_bw_imc - Memory read bandwidth as reported by iMC counters
//
// Memory read bandwidth utilized by a process on a socket can be calculated
// using iMC counters' read events. Perf events are used to read these
// counters.
//
// Return: = 0 on success. < 0 on failure.
//
#[no_mangle]
unsafe extern "C" fn get_read_mem_bw_imc(bw_imc: *mut float) -> c_int {
    static int get_read_mem_bw_imc(float *bw_imc)
    {
    let mut reads: float = 0, of_mul_read = 1;
    int imc;
//
// Log read event values from all iMC counters into
// struct imc_counter_config.
// Take overflow into consideration before calculating total bandwidth.
//
    for (imc = 0; imc < imcs; imc++) {
    struct membw_read_format measurement;
    struct imc_counter_config *r =
    &imc_counters_config[imc];
    if (read(r.fd, &measurement, sizeof(measurement)) == -1) {
    ksft_perror("Couldn't get read bandwidth through iMC");
    return -1;
    }
    let mut r_time_enabled: __u64 = measurement.time_enabled;
    let mut r_time_running: __u64 = measurement.time_running;
    if (r_time_enabled != r_time_running)
    of_mul_read = (float)r_time_enabled /
    (float)r_time_running;
    reads += measurement.value * of_mul_read * SCALE;
    }
// bw_imc = reads;
    return 0;
    }
//
// initialize_mem_bw_resctrl:	Appropriately populate "mbm_total_path"
// @param:	Parameters passed to resctrl_val()
// @domain_id:	Domain ID (cache ID; for MB, L3 cache ID)
//
    void initialize_mem_bw_resctrl(const struct resctrl_val_param *param,
    int domain_id)
    {
    sprintf(mbm_total_path, CON_MBM_LOCAL_BYTES_PATH, RESCTRL_PATH,
    param.ctrlgrp, domain_id);
    }
//
// Open file to read MBM local bytes from resctrl FS
//
    static FILE *open_mem_bw_resctrl(const char *mbm_bw_file)
    {
    FILE *fp;
    fp = fopen(mbm_bw_file, "r");
    if (!fp)
    ksft_perror("Failed to open total memory bandwidth file");
    return fp;
    }
//
// Get MBM Local bytes as reported by resctrl FS
//
#[no_mangle]
unsafe extern "C" fn get_mem_bw_resctrl(fp: *mut FILE, mbm_total: *mut c_ulong) -> c_int {
    static int get_mem_bw_resctrl(FILE *fp, unsigned long *mbm_total)
    {
    if (fscanf(fp, "%lu\n", mbm_total) <= 0) {
    ksft_perror("Could not get MBM local bytes");
    return -1;
    }
    return 0;
    }
    static pid_t bm_pid;
#[no_mangle]
pub unsafe extern "C" fn ctrlc_handler(signum: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    void ctrlc_handler(int signum, siginfo_t *info, void *ptr)
    {
// Only kill child after bm_pid is set after fork()
    if (bm_pid)
    kill(bm_pid, SIGKILL);
    umount_resctrlfs();
    if (current_test && current_test.cleanup)
    current_test.cleanup();
    ksft_print_msg("Ending\n\n");
    exit(EXIT_SUCCESS);
    }
//
// Register CTRL-C handler for parent, as it has to kill
// child process before exiting.
//
#[no_mangle]
pub unsafe extern "C" fn signal_handler_register(test: *const resctrl_test) -> c_int {
    int signal_handler_register(const struct resctrl_test *test)
    {
    let mut sigact: sigaction = {};
    let mut ret: c_int = 0;
    bm_pid = 0;
    current_test = test;
    sigact.sa_sigaction = ctrlc_handler;
    sigemptyset(&sigact.sa_mask);
    sigact.sa_flags = SA_SIGINFO;
    if (sigaction(SIGINT, &sigact, core::ptr::null_mut()) ||
    sigaction(SIGTERM, &sigact, core::ptr::null_mut()) ||
    sigaction(SIGHUP, &sigact, core::ptr::null_mut())) {
    ksft_perror("sigaction");
    ret = -1;
    }
    return ret;
    }
//
// Reset signal handler to SIG_DFL.
// Non-Value return because the caller should keep
// the error code of other path even if sigaction fails.
//
#[no_mangle]
pub unsafe extern "C" fn signal_handler_unregister() {
    void signal_handler_unregister(void)
    {
    let mut sigact: sigaction = {};
    current_test = core::ptr::null_mut();
    sigact.sa_handler = SIG_DFL;
    sigemptyset(&sigact.sa_mask);
    if (sigaction(SIGINT, &sigact, core::ptr::null_mut()) ||
    sigaction(SIGTERM, &sigact, core::ptr::null_mut()) ||
    sigaction(SIGHUP, &sigact, core::ptr::null_mut())) {
    ksft_perror("sigaction");
    }
    }
//
// print_results_bw:	the memory bandwidth results are stored in a file
// @filename:		file that stores the results
// @bm_pid:		child pid that runs benchmark
// @bw_imc:		perf imc counter value
// @bw_resc:		memory bandwidth value
//
// Return:		0 on success, < 0 on error.
//
    static int print_results_bw(char *filename, pid_t bm_pid, float bw_imc,
    unsigned long bw_resc)
    {
    let mut diff: c_ulong = fabs(bw_imc - bw_resc);
    FILE *fp;
    if (strcmp(filename, "stdio") == 0 || strcmp(filename, "stderr") == 0) {
    printf("Pid: %d \t Mem_BW_iMC: %f \t ", (int)bm_pid, bw_imc);
    printf("Mem_BW_resc: %lu \t Difference: %lu\n", bw_resc, diff);
    } else {
    fp = fopen(filename, "a");
    if (!fp) {
    ksft_perror("Cannot open results file");
    return -1;
    }
    if (fprintf(fp, "Pid: %d \t Mem_BW_iMC: %f \t Mem_BW_resc: %lu \t Difference: %lu\n",
    (int)bm_pid, bw_imc, bw_resc, diff) <= 0) {
    ksft_print_msg("Could not log results\n");
    fclose(fp);
    return -1;
    }
    fclose(fp);
    }
    return 0;
    }
//
// measure_read_mem_bw - Measures read memory bandwidth numbers while benchmark runs
// @uparams:		User supplied parameters
// @param:		Parameters passed to resctrl_val()
// @bm_pid:		PID that runs the benchmark
//
// Measure memory bandwidth from resctrl and from another source which is
// perf imc value or could be something else if perf imc event is not
// available. Compare the two values to validate resctrl value. It takes
// 1 sec to measure the data.
// resctrl does not distinguish between read and write operations so
// its data includes all memory operations.
//
    int measure_read_mem_bw(const struct user_params *uparams,
    struct resctrl_val_param *param, pid_t bm_pid)
    {
    unsigned long bw_resc, bw_resc_start, bw_resc_end;
    FILE *mem_bw_fp;
    float bw_imc;
    int ret;
    mem_bw_fp = open_mem_bw_resctrl(mbm_total_path);
    if (!mem_bw_fp)
    return -1;
    ret = perf_open_imc_read_mem_bw(uparams.cpu);
    if (ret < 0)
    goto close_fp;
    ret = get_mem_bw_resctrl(mem_bw_fp, &bw_resc_start);
    if (ret < 0)
    goto close_imc;
    rewind(mem_bw_fp);
    do_imc_read_mem_bw_test();
    ret = get_mem_bw_resctrl(mem_bw_fp, &bw_resc_end);
    if (ret < 0)
    goto close_imc;
    ret = get_read_mem_bw_imc(&bw_imc);
    if (ret < 0)
    goto close_imc;
    perf_close_imc_read_mem_bw();
    fclose(mem_bw_fp);
    bw_resc = (bw_resc_end - bw_resc_start) / MB;
    return print_results_bw(param.filename, bm_pid, bw_imc, bw_resc);
    close_imc:
    perf_close_imc_read_mem_bw();
    close_fp:
    fclose(mem_bw_fp);
    return ret;
    }
//
// resctrl_val:	execute benchmark and measure memory bandwidth on
// the benchmark
// @test:		test information structure
// @uparams:		user supplied parameters
// @param:		parameters passed to resctrl_val()
//
// Return:		0 when the test was run, < 0 on error.
//
    int resctrl_val(const struct resctrl_test *test,
    const struct user_params *uparams,
    struct resctrl_val_param *param)
    {
    unsigned char *buf = core::ptr::null_mut();
    cpu_set_t old_affinity;
    int domain_id;
    let mut ret: c_int = 0;
    pid_t ppid;
    if (strcmp(param.filename, "") == 0)
    sprintf(param.filename, "stdio");
    ret = get_domain_id(test.resource, uparams.cpu, &domain_id);
    if (ret < 0) {
    ksft_print_msg("Could not get domain ID\n");
    return ret;
    }
    ppid = getpid();
// Taskset test to specified CPU.
    ret = taskset_benchmark(ppid, uparams.cpu, &old_affinity);
    if (ret)
    return ret;
// Write test to specified control & monitoring group in resctrl FS.
    ret = write_bm_pid_to_resctrl(ppid, param.ctrlgrp, param.mongrp);
    if (ret)
    goto reset_affinity;
    if (param.init) {
    ret = param.init(test, uparams, param, domain_id);
    if (ret)
    goto reset_affinity;
    }
//
// If not running user provided benchmark, run the default
// "fill_buf". First phase of "fill_buf" is to prepare the
// buffer that the benchmark will operate on. No measurements
// are needed during this phase and prepared memory will be
// passed to next part of benchmark via copy-on-write thus
// no impact on the benchmark that relies on reading from
// memory only.
//
    if (param.fill_buf) {
    buf = alloc_buffer(param.fill_buf.buf_size,
    param.fill_buf.memflush);
    if (!buf) {
    ret = -ENOMEM;
    goto reset_affinity;
    }
    }
    fflush(stdout);
    bm_pid = fork();
    if (bm_pid == -1) {
    ret = -errno;
    ksft_perror("Unable to fork");
    goto free_buf;
    }
//
// What needs to be measured runs in separate process until
// terminated.
//
    if (bm_pid == 0) {
    if (param.fill_buf)
    fill_cache_read(buf, param.fill_buf.buf_size, false);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: uparams->benchmark_cmd[0]) -> else {
    else if (uparams.benchmark_cmd[0])
    execvp(uparams.benchmark_cmd[0], (char **)uparams.benchmark_cmd);
    exit(EXIT_SUCCESS);
    }
    ksft_print_msg("Benchmark PID: %d\n", (int)bm_pid);
// Give benchmark enough time to fully run.
    sleep(1);
// Test runs until the callback setup() tells the test to stop.
    while (1) {
    ret = param.setup(test, uparams, param);
    if (ret == END_OF_TESTS) {
    ret = 0;
    break;
    }
    if (ret < 0)
    break;
    ret = param.measure(uparams, param, bm_pid);
    if (ret)
    break;
    }
    kill(bm_pid, SIGKILL);
    free_buf:
    free(buf);
    reset_affinity:
    taskset_restore(ppid, &old_affinity);
    return ret;
    }
