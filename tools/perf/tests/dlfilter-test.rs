//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/dlfilter-test.c
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
// Test dlfilter C API. A perf.data file is synthesized and then processed
// by perf script with dlfilters named dlfilter-test-api-v*.so. Also a C file
// is compiled to provide a dso to match the synthesized perf.data file.
//

pub const MAP_START: c_uint = 0x400000;
pub const DLFILTER_TEST_NAME_MAX: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub tool: perf_tool,
    pub machine: *mut machine,
    pub fd: c_int,
    pub foo: u64,
    pub bar: u64,
    pub ip: u64,
    pub addr: u64,
    pub name: [c_char; DLFILTER_TEST_NAME_MAX],
    pub desc: [c_char; DLFILTER_TEST_NAME_MAX],
    pub perf: [c_char; PATH_MAX],
    pub perf_data_file_name: [c_char; PATH_MAX],
    pub c_file_name: [c_char; PATH_MAX],
    pub prog_file_name: [c_char; PATH_MAX],
    pub dlfilters: [c_char; PATH_MAX],
}

#[no_mangle]
unsafe extern "C" fn test_result(msg: *const c_char, ret: c_int) -> c_int {
    static int test_result(const char *msg, int ret)
    {
    pr_debug("%s\n", msg);
    return ret;
    }
    static int process(const struct perf_tool *tool, union perf_event *event,
    struct perf_sample *sample __maybe_unused,
    struct machine *machine __maybe_unused)
    {
    struct test_data *td = container_of(tool, struct test_data, tool);
    let mut fd: c_int = td.fd;
    if (writen(fd, event, event.header.size) != event.header.size)
    return -1;
    return 0;
    }
pub const MAXCMD: c_int = 4096;

#[no_mangle]
pub unsafe extern "C" fn __printf(_arg: 1, fmt: *const 2) int system_cmd(char, ...) -> static {
    static __printf(1, 2) int system_cmd(const char *fmt, ...)
    {
    char cmd[MAXCMD + sizeof(REDIRECT_TO_DEV_NULL)];
    int ret;
    va_list args;
    va_start(args, fmt);
    ret = vsnprintf(cmd, MAXCMD, fmt, args);
    va_end(args);
    if (ret <= 0 || ret >= MAXCMD)
    return -1;
    if (verbose <= 0)
    strcat(cmd, REDIRECT_TO_DEV_NULL);
    pr_debug("Command: %s\n", cmd);
    ret = system(cmd);
    if (ret)
    pr_debug("Failed with return value %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn have_gcc() -> bool {
    static bool have_gcc(void)
    {
    pr_debug("Checking for gcc\n");
    return !system_cmd("gcc --version");
    }
#[no_mangle]
unsafe extern "C" fn write_attr(td: *mut test_data, sample_type: u64, id: *mut u64) -> c_int {
    static int write_attr(struct test_data *td, u64 sample_type, u64 *id)
    {
    struct perf_event_attr attr = {
    .size = sizeof(attr),
    .type = PERF_TYPE_HARDWARE,
    .config = PERF_COUNT_HW_BRANCH_INSTRUCTIONS,
    .sample_type = sample_type,
    .sample_period = 1,
    };
    return perf_event__synthesize_attr(&td.tool, &attr, 1, id, process);
    }
#[no_mangle]
unsafe extern "C" fn write_comm(fd: c_int, pid: pid_t, tid: pid_t, comm_str: *const c_char) -> c_int {
    static int write_comm(int fd, pid_t pid, pid_t tid, const char *comm_str)
    {
    struct perf_record_comm comm;
    let mut sz: isize = sizeof(comm);
    comm.header.type = PERF_RECORD_COMM;
    comm.header.misc = PERF_RECORD_MISC_USER;
    comm.header.size = sz;
    comm.pid = pid;
    comm.tid = tid;
    strncpy(comm.comm, comm_str, 16);
    if (writen(fd, &comm, sz) != sz) {
    pr_debug("%s failed\n", __func__);
    return -1;
    }
    return 0;
    }
    static int write_mmap(int fd, pid_t pid, pid_t tid, u64 start, u64 len, u64 pgoff,
    const char *filename)
    {
    char buf[PERF_SAMPLE_MAX_SIZE];
    struct perf_record_mmap *mmap = (struct perf_record_mmap *)buf;
    let mut fsz: usize = roundup(strlen(filename) + 1, 8);
    let mut sz: isize = sizeof(*mmap) - sizeof(mmap.filename) + fsz;
    mmap.header.type = PERF_RECORD_MMAP;
    mmap.header.misc = PERF_RECORD_MISC_USER;
    mmap.header.size = sz;
    mmap.pid   = pid;
    mmap.tid   = tid;
    mmap.start = start;
    mmap.len   = len;
    mmap.pgoff = pgoff;
    strncpy(mmap.filename, filename, sizeof(mmap.filename));
    if (writen(fd, mmap, sz) != sz) {
    pr_debug("%s failed\n", __func__);
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn write_sample(td: *mut test_data, sample_type: u64, id: u64, pid: pid_t, tid: pid_t) -> c_int {
    static int write_sample(struct test_data *td, u64 sample_type, u64 id, pid_t pid, pid_t tid)
    {
    char buf[PERF_SAMPLE_MAX_SIZE];
    union perf_event *event = (union perf_event *)buf;
    struct perf_sample sample = {
    .ip		= td.ip,
    .addr		= td.addr,
    .id		= id,
    .time		= 1234567890,
    .cpu		= 31,
    .pid		= pid,
    .tid		= tid,
    .period		= 543212345,
    .stream_id	= 101,
    };
    int err;
    event.header.type = PERF_RECORD_SAMPLE;
    event.header.misc = PERF_RECORD_MISC_USER;
    event.header.size = perf_event__sample_event_size(&sample, sample_type,
// read_format=*/0,
// branch_sample_type=*/0);
    err = perf_event__synthesize_sample(event, sample_type,
// read_format=*/0,
// branch_sample_type=*/0, &sample);
    if (err)
    return test_result("perf_event__synthesize_sample() failed", TEST_FAIL);
    err = process(&td.tool, event, &sample, td.machine);
    if (err)
    return test_result("Failed to write sample", TEST_FAIL);
    return TEST_OK;
    }
#[no_mangle]
unsafe extern "C" fn close_fd(fd: c_int) {
    static void close_fd(int fd)
    {
    if (fd >= 0)
    close(fd);
    }
    static const char *prog = "int bar(){};int foo(){bar();};int main(){foo();return 0;}";
#[no_mangle]
unsafe extern "C" fn write_prog(file_name: *mut c_char) -> c_int {
    static int write_prog(char *file_name)
    {
    let mut fd: c_int = creat(file_name, 0644);
    let mut n: isize = strlen(prog);
    let mut err: bool = fd < 0 || writen(fd, prog, n) != n;
    close_fd(fd);
    return err ? -1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn get_dlfilters_path(name: *const c_char, buf: *mut c_char, sz: usize) -> c_int {
    static int get_dlfilters_path(const char *name, char *buf, size_t sz)
    {
    char perf[PATH_MAX];
    char path[PATH_MAX];
    char *perf_path;
    char *exec_path;
    perf_exe(perf, sizeof(perf));
    perf_path = dirname(perf);
    snprintf(path, sizeof(path), "%s/dlfilters/%s", perf_path, name);
    if (access(path, R_OK)) {
    exec_path = get_argv_exec_path();
    if (!exec_path)
    return -1;
    snprintf(path, sizeof(path), "%s/dlfilters/%s", exec_path, name);
    free(exec_path);
    if (access(path, R_OK))
    return -1;
    }
    strlcpy(buf, dirname(path), sz);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_filter_desc(td: *mut test_data) -> c_int {
    static int check_filter_desc(struct test_data *td)
    {
    char *long_desc = core::ptr::null_mut();
    char *desc = core::ptr::null_mut();
    int ret;
    if (get_filter_desc(td.dlfilters, td.name, &desc, &long_desc) &&
    long_desc && !strcmp(long_desc, "Filter used by the 'dlfilter C API' perf test") &&
    desc && !strcmp(desc, td.desc))
    ret = 0;
    else
    ret = -1;
    free(desc);
    free(long_desc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_ip_addr(td: *mut test_data) -> c_int {
    static int get_ip_addr(struct test_data *td)
    {
    struct map *map;
    struct symbol *sym;
    map = dso__new_map(td.prog_file_name);
    if (!map)
    return -1;
    sym = map__find_symbol_by_name(map, "foo");
    if (sym)
    td.foo = sym.start;
    sym = map__find_symbol_by_name(map, "bar");
    if (sym)
    td.bar = sym.start;
    map__put(map);
    td.ip = MAP_START + td.foo;
    td.addr = MAP_START + td.bar;
    return td.foo && td.bar ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn do_run_perf_script(td: *mut test_data, do_early: c_int) -> c_int {
    static int do_run_perf_script(struct test_data *td, int do_early)
    {
    return system_cmd("%s script -i %s "
    "--dlfilter %s/%s "
    "--dlarg first "
    "--dlarg %d "
    "--dlarg %" PRIu64 " "
    "--dlarg %" PRIu64 " "
    "--dlarg %d "
    "--dlarg last",
    td.perf, td.perf_data_file_name, td.dlfilters,
    td.name, verbose, td.ip, td.addr, do_early);
    }
#[no_mangle]
unsafe extern "C" fn run_perf_script(td: *mut test_data) -> c_int {
    static int run_perf_script(struct test_data *td)
    {
    int do_early;
    int err;
    for (do_early = 0; do_early < 3; do_early++) {
    err = do_run_perf_script(td, do_early);
    if (err)
    return err;
    }
    return 0;
    }

    PERF_SAMPLE_IDENTIFIER | PERF_SAMPLE_TIME | \
    PERF_SAMPLE_ADDR | PERF_SAMPLE_CPU | \
    PERF_SAMPLE_PERIOD | PERF_SAMPLE_STREAM_ID)
#[no_mangle]
unsafe extern "C" fn test__dlfilter_test(td: *mut test_data) -> c_int {
    static int test__dlfilter_test(struct test_data *td)
    {
    struct perf_env host_env;
    let mut sample_type: u64 = TEST_SAMPLE_TYPE;
    let mut pid: pid_t = 12345;
    let mut tid: pid_t = 12346;
    let mut id: u64 = 99;
    let mut err: c_int = TEST_OK;
    if (get_dlfilters_path(td.name, td.dlfilters, PATH_MAX))
    return test_result("dlfilters not found", TEST_SKIP);
    if (check_filter_desc(td))
    return test_result("Failed to get expected filter description", TEST_FAIL);
    if (!have_gcc())
    return test_result("gcc not found", TEST_SKIP);
    pr_debug("dlfilters path: %s\n", td.dlfilters);
    if (write_prog(td.c_file_name))
    return test_result("Failed to write test C file", TEST_FAIL);
    if (verbose > 1)
    system_cmd("cat %s ; echo", td.c_file_name);
    if (system_cmd("gcc -g -o %s %s", td.prog_file_name, td.c_file_name))
    return TEST_FAIL;
    if (verbose > 2)
    system_cmd("objdump -x -dS %s", td.prog_file_name);
    if (get_ip_addr(td))
    return test_result("Failed to find program symbols", TEST_FAIL);
    pr_debug("Creating new host machine structure\n");
    perf_env__init(&host_env);
    td.machine = machine__new_host(&host_env);
    td.fd = creat(td.perf_data_file_name, 0644);
    if (td.fd < 0)
    return test_result("Failed to create test perf.data file", TEST_FAIL);
    err = perf_header__write_pipe(td.fd);
    if (err < 0) {
    err = test_result("perf_header__write_pipe() failed", TEST_FAIL);
    goto out;
    }
    err = write_attr(td, sample_type, &id);
    if (err) {
    err = test_result("perf_event__synthesize_attr() failed", TEST_FAIL);
    goto out;
    }
    if (write_comm(td.fd, pid, tid, "test-prog")) {
    err = TEST_FAIL;
    goto out;
    }
    if (write_mmap(td.fd, pid, tid, MAP_START, 0x10000, 0, td.prog_file_name)) {
    err = TEST_FAIL;
    goto out;
    }
    if (write_sample(td, sample_type, id, pid, tid) != TEST_OK) {
    err = TEST_FAIL;
    goto out;
    }
    if (verbose > 1)
    system_cmd("%s script -i %s -D", td.perf, td.perf_data_file_name);
    err = run_perf_script(td) ? TEST_FAIL : TEST_OK;
    out:
    perf_env__exit(&host_env);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn unlink_path(path: *const c_char) {
    static void unlink_path(const char *path)
    {
    if (*path)
    unlink(path);
    }
#[no_mangle]
unsafe extern "C" fn test_data__free(td: *mut test_data) {
    static void test_data__free(struct test_data *td)
    {
    machine__delete(td.machine);
    close_fd(td.fd);
    if (verbose <= 2) {
    unlink_path(td.c_file_name);
    unlink_path(td.prog_file_name);
    unlink_path(td.perf_data_file_name);
    }
    }
#[no_mangle]
unsafe extern "C" fn test__dlfilter_ver(ver: c_int) -> c_int {
    static int test__dlfilter_ver(int ver)
    {
    let mut td: test_data = {.fd = -1};
    let mut pid: c_int = getpid();
    int err;
    pr_debug("\n-- Testing version %d API --\n", ver);
    perf_exe(td.perf, sizeof(td.perf));
    snprintf(td.name, sizeof(td.name), "dlfilter-test-api-v%d.so", ver);
    snprintf(td.desc, sizeof(td.desc), "dlfilter to test v%d C API", ver);
    snprintf(td.perf_data_file_name, PATH_MAX, "/tmp/dlfilter-test-%u-perf-data", pid);
    snprintf(td.c_file_name, PATH_MAX, "/tmp/dlfilter-test-%u-prog.c", pid);
    snprintf(td.prog_file_name, PATH_MAX, "/tmp/dlfilter-test-%u-prog", pid);
    err = test__dlfilter_test(&td);
    test_data__free(&td);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test__dlfilter(__maybe_unused: *mut *mut test_suite test, __maybe_unused: int subtest) -> c_int {
    static int test__dlfilter(struct test_suite *test __maybe_unused, int subtest __maybe_unused)
    {
    let mut err: c_int = test__dlfilter_ver(0);
    if (err)
    return err;
// No test for version 1
    return test__dlfilter_ver(2);
    }
    DEFINE_SUITE("dlfilter C API", dlfilter);
