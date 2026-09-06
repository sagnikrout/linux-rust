//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/sched/cs_prctl_test.c
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
// Use the core scheduling prctl() to test core scheduling cookies control.
//
// Copyright (c) 2021 Oracle and/or its affiliates.
// Author: Chris Hyser <chris.hyser@oracle.com>
//
// This library is free software; you can redistribute it and/or modify it
// under the terms of version 2.1 of the GNU Lesser General Public License as
// published by the Free Software Foundation.
//
// This library is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
// for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this library; if not, see <http://www.gnu.org/licenses>.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn gettid() -> pid_t {
    static pid_t gettid(void)
    {
    return syscall(SYS_gettid);
    }

pub const PR_SCHED_CORE: c_int = 62;
pub const PR_SCHED_CORE_GET: c_int = 0;

pub const PR_SCHED_CORE_MAX: c_int = 4;

pub const MAX_PROCESSES: c_int = 128;
pub const MAX_THREADS: c_int = 128;
    static const char USAGE[] = "cs_prctl_test [options]\n"
    "    options:\n"
    "	-P  : number of processes to create.\n"
    "	-T  : number of threads per process to create.\n"
    "	-d  : delay time to keep tasks alive.\n"
    "	-k  : keep tasks alive until keypress.\n";
    enum pid_type {PIDTYPE_PID = 0, PIDTYPE_TGID, PIDTYPE_PGID};
    let mut THREAD_CLONE_FLAGS: c_int = CLONE_THREAD | CLONE_SIGHAND | CLONE_FS | CLONE_VM | CLONE_FILES;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_args {
    pub num_threads: c_int,
    pub pfd: [c_int; 2],
    pub cpid: c_int,
    pub thr_tids: [c_int; MAX_THREADS],
}

    static struct child_args procs[MAX_PROCESSES];
    let mut num_processes: static int = 2;
    static int need_cleanup;
    static int _prctl(int option, unsigned long arg2, unsigned long arg3, unsigned long arg4,
    unsigned long arg5)
    {
    int res;
    res = prctl(option, arg2, arg3, arg4, arg5);
    printf("%d = prctl(%d, %ld, %ld, %ld, %lx)\n", res, option, (long)arg2, (long)arg3,
    (long)arg4, arg5);
    return res;
    }

#[no_mangle]
unsafe extern "C" fn __handle_error(fn: *mut c_char, ln: c_int, msg: *mut c_char) {
    static void __handle_error(char *fn, int ln, char *msg)
    {
    int pidx;
    printf("(%s:%d) - ", fn, ln);
    perror(msg);
    if (need_cleanup) {
    for (pidx = 0; pidx < num_processes; ++pidx)
    kill(procs[pidx].cpid, 15);
    need_cleanup = 0;
    }
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn handle_usage(rc: c_int, msg: *mut c_char) {
    static void handle_usage(int rc, char *msg)
    {
    puts(USAGE);
    puts(msg);
    putchar('\n');
    exit(rc);
    }
#[no_mangle]
unsafe extern "C" fn get_cs_cookie(pid: c_int) -> c_ulong {
    static unsigned long get_cs_cookie(int pid)
    {
    unsigned long long cookie;
    int ret;
    ret = prctl(PR_SCHED_CORE, PR_SCHED_CORE_GET, pid, PIDTYPE_PID,
    (unsigned long)&cookie);
    if (ret) {
    printf("Not a core sched system\n");
    return -1UL;
    }
    return cookie;
    }
#[no_mangle]
unsafe extern "C" fn child_func_thread(__attribute__((unused))*arg: *mut c_void) -> c_int {
    static int child_func_thread(void __attribute__((unused))*arg)
    {
    while (1)
    usleep(20000);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn create_threads(num_threads: c_int, thr_tids[]: c_int) {
    static void create_threads(int num_threads, int thr_tids[])
    {
    void *child_stack;
    pid_t tid;
    int i;
    for (i = 0; i < num_threads; ++i) {
    child_stack = malloc(STACK_SIZE);
    if (!child_stack)
    handle_error("child stack allocate");
    tid = clone(child_func_thread, child_stack + STACK_SIZE, THREAD_CLONE_FLAGS, core::ptr::null_mut());
    if (tid == -1)
    handle_error("clone thread");
    thr_tids[i] = tid;
    }
    }
#[no_mangle]
unsafe extern "C" fn child_func_process(arg: *mut c_void) -> c_int {
    static int child_func_process(void *arg)
    {
    struct child_args *ca = (struct child_args *)arg;
    int ret;
    close(ca.pfd[0]);
    create_threads(ca.num_threads, ca.thr_tids);
    ret = write(ca.pfd[1], &ca.thr_tids, sizeof(int) * ca.num_threads);
    if (ret == -1)
    printf("write failed on pfd[%d] - error (%s)\n",
    ca.pfd[1], strerror(errno));
    close(ca.pfd[1]);
    while (1)
    usleep(20000);
    return 0;
    }
    static unsigned char child_func_process_stack[STACK_SIZE];
#[no_mangle]
pub unsafe extern "C" fn create_processes(num_processes: c_int, num_threads: c_int, proc[]: child_args) {
    void create_processes(int num_processes, int num_threads, struct child_args proc[])
    {
    pid_t cpid;
    int i, ret;
    for (i = 0; i < num_processes; ++i) {
    proc[i].num_threads = num_threads;
    if (pipe(proc[i].pfd) == -1)
    handle_error("pipe() failed");
    cpid = clone(child_func_process, child_func_process_stack + STACK_SIZE,
    SIGCHLD, &proc[i]);
    proc[i].cpid = cpid;
    close(proc[i].pfd[1]);
    }
    for (i = 0; i < num_processes; ++i) {
    ret = read(proc[i].pfd[0], &proc[i].thr_tids, sizeof(int) * proc[i].num_threads);
    if (ret == -1)
    printf("read failed on proc[%d].pfd[0] error (%s)\n",
    i, strerror(errno));
    close(proc[i].pfd[0]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn disp_processes(num_processes: c_int, proc[]: child_args) {
    void disp_processes(int num_processes, struct child_args proc[])
    {
    int i, j;
    printf("tid=%d, / tgid=%d / pgid=%d: %lx\n", gettid(), getpid(), getpgid(0),
    get_cs_cookie(getpid()));
    for (i = 0; i < num_processes; ++i) {
    printf("    tid=%d, / tgid=%d / pgid=%d: %lx\n", proc[i].cpid, proc[i].cpid,
    getpgid(proc[i].cpid), get_cs_cookie(proc[i].cpid));
    for (j = 0; j < proc[i].num_threads; ++j) {
    printf("        tid=%d, / tgid=%d / pgid=%d: %lx\n", proc[i].thr_tids[j],
    proc[i].cpid, getpgid(0), get_cs_cookie(proc[i].thr_tids[j]));
    }
    }
    puts("\n");
    }
    static int errors;

#[no_mangle]
pub unsafe extern "C" fn _validate(line: c_int, val: c_int, msg: *mut c_char) {
    void _validate(int line, int val, char *msg)
    {
    if (!val) {
    ++errors;
    printf("(%d) FAILED: %s\n", line, msg);
    } else {
    printf("(%d) PASSED: %s\n", line, msg);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut keypress: c_int = 0;
    let mut num_threads: c_int = 3;
    let mut delay: c_int = 0;
    let mut res: c_int = 0;
    int pidx;
    int pid;
    int opt;
    while ((opt = getopt(argc, argv, ":hkT:P:d:")) != -1) {
    switch (opt) {
    case 'P':
    num_processes = (int)strtol(optarg, core::ptr::null_mut(), 10);
    break;
    case 'T':
    num_threads = (int)strtoul(optarg, core::ptr::null_mut(), 10);
    break;
    case 'd':
    delay = (int)strtol(optarg, core::ptr::null_mut(), 10);
    break;
    case 'k':
    keypress = 1;
    break;
    case 'h':
    printf(USAGE);
    exit(EXIT_SUCCESS);
    default:
    handle_usage(20, "unknown option");
    }
    }
    if (num_processes < 1 || num_processes > MAX_PROCESSES)
    handle_usage(1, "Bad processes value");
    if (num_threads < 1 || num_threads > MAX_THREADS)
    handle_usage(2, "Bad thread value");
    if (keypress)
    delay = -1;
    srand(time(core::ptr::null_mut()));
// put into separate process group
    if (setpgid(0, 0) != 0)
    handle_error("process group");
    printf("\n## Create a thread/process/process group hierarchy\n");
    create_processes(num_processes, num_threads, procs);
    need_cleanup = 1;
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) == 0);
    printf("\n## Set a cookie on entire process group\n");
    if (_prctl(PR_SCHED_CORE, PR_SCHED_CORE_CREATE, 0, PIDTYPE_PGID, 0) < 0)
    handle_error("core_sched create failed -- PGID");
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) != 0);
// get a random process pid
    pidx = rand() % num_processes;
    pid = procs[pidx].cpid;
    validate(get_cs_cookie(0) == get_cs_cookie(pid));
    validate(get_cs_cookie(0) == get_cs_cookie(procs[pidx].thr_tids[0]));
    printf("\n## Set a new cookie on entire process/TGID [%d]\n", pid);
    if (_prctl(PR_SCHED_CORE, PR_SCHED_CORE_CREATE, pid, PIDTYPE_TGID, 0) < 0)
    handle_error("core_sched create failed -- TGID");
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) != get_cs_cookie(pid));
    validate(get_cs_cookie(pid) != 0);
    validate(get_cs_cookie(pid) == get_cs_cookie(procs[pidx].thr_tids[0]));
    printf("\n## Copy the cookie of current/PGID[%d], to pid [%d] as PIDTYPE_PID\n",
    getpid(), pid);
    if (_prctl(PR_SCHED_CORE, PR_SCHED_CORE_SHARE_TO, pid, PIDTYPE_PID, 0) < 0)
    handle_error("core_sched share to itself failed -- PID");
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) == get_cs_cookie(pid));
    validate(get_cs_cookie(pid) != 0);
    validate(get_cs_cookie(pid) != get_cs_cookie(procs[pidx].thr_tids[0]));
    printf("\n## Copy cookie from a thread [%d] to current/PGID [%d] as PIDTYPE_PID\n",
    procs[pidx].thr_tids[0], getpid());
    if (_prctl(PR_SCHED_CORE, PR_SCHED_CORE_SHARE_FROM, procs[pidx].thr_tids[0],
    PIDTYPE_PID, 0) < 0)
    handle_error("core_sched share from thread failed -- PID");
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) == get_cs_cookie(procs[pidx].thr_tids[0]));
    validate(get_cs_cookie(pid) != get_cs_cookie(procs[pidx].thr_tids[0]));
    printf("\n## Copy cookie from current [%d] to current as pidtype PGID\n", getpid());
    if (_prctl(PR_SCHED_CORE, PR_SCHED_CORE_SHARE_TO, 0, PIDTYPE_PGID, 0) < 0)
    handle_error("core_sched share to self failed -- PGID");
    disp_processes(num_processes, procs);
    validate(get_cs_cookie(0) == get_cs_cookie(pid));
    validate(get_cs_cookie(pid) != 0);
    validate(get_cs_cookie(pid) == get_cs_cookie(procs[pidx].thr_tids[0]));
    validate(_prctl(PR_SCHED_CORE, PR_SCHED_CORE_MAX, 0, PIDTYPE_PGID, 0) < 0
    && errno == EINVAL);
    validate(_prctl(PR_SCHED_CORE, PR_SCHED_CORE_SHARE_TO, 0, PIDTYPE_PGID, 1) < 0
    && errno == EINVAL);
    if (errors) {
    printf("TESTS FAILED. errors: %d\n", errors);
    res = 10;
    } else {
    printf("SUCCESS !!!\n");
    }
    if (keypress)
    getchar();
    else
    sleep(delay);
    for (pidx = 0; pidx < num_processes; ++pidx)
    kill(procs[pidx].cpid, 15);
    return res;
    }
