//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/sched-seccomp-notify.c
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

    let mut loops: static uint64_t = LOOPS_DEFAULT;
    static bool sync_mode;
    static const struct option options[] = {
    OPT_U64('l', "loop",	&loops,		"Specify number of loops"),
    OPT_BOOLEAN('s', "sync-mode", &sync_mode,
    "Enable the synchronous mode for seccomp notifications"),
    OPT_END()
    };
    static const char * const bench_seccomp_usage[] = {
    "perf bench sched secccomp-notify <options>",
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn seccomp(op: c_uint, flags: c_uint, args: *mut c_void) -> c_int {
    static int seccomp(unsigned int op, unsigned int flags, void *args)
    {
    return syscall(__NR_seccomp, op, flags, args);
    }
#[no_mangle]
unsafe extern "C" fn user_notif_syscall(nr: c_int, flags: c_uint) -> c_int {
    static int user_notif_syscall(int nr, unsigned int flags)
    {
    struct sock_filter filter[] = {
    BPF_STMT(BPF_LD|BPF_W|BPF_ABS,
    offsetof(struct seccomp_data, nr)),
    BPF_JUMP(BPF_JMP|BPF_JEQ|BPF_K, nr, 0, 1),
    BPF_STMT(BPF_RET|BPF_K, SECCOMP_RET_USER_NOTIF),
    BPF_STMT(BPF_RET|BPF_K, SECCOMP_RET_ALLOW),
    };
    struct sock_fprog prog = {
    .len = (unsigned short)ARRAY_SIZE(filter),
    .filter = filter,
    };
    return seccomp(SECCOMP_SET_MODE_FILTER, flags, &prog);
    }

#[no_mangle]
unsafe extern "C" fn user_notification_sync_loop(listener: c_int) {
    static void user_notification_sync_loop(int listener)
    {
    struct seccomp_notif_resp resp;
    struct seccomp_notif req;
    uint64_t nr;
    for (nr = 0; nr < loops; nr++) {
    memset(&req, 0, sizeof(req));
    if (ioctl(listener, SECCOMP_IOCTL_NOTIF_RECV, &req))
    err(EXIT_FAILURE, "SECCOMP_IOCTL_NOTIF_RECV failed");
    if (req.data.nr != __NR_gettid)
    errx(EXIT_FAILURE, "unexpected syscall: %d", req.data.nr);
    resp.id = req.id;
    resp.error = 0;
    resp.val = USER_NOTIF_MAGIC;
    resp.flags = 0;
    if (ioctl(listener, SECCOMP_IOCTL_NOTIF_SEND, &resp))
    err(EXIT_FAILURE, "SECCOMP_IOCTL_NOTIF_SEND failed");
    }
    }

#[no_mangle]
pub unsafe extern "C" fn bench_sched_seccomp_notify(argc: c_int, argv: *const c_char) -> c_int {
    int bench_sched_seccomp_notify(int argc, const char **argv)
    {
    struct timeval start, stop, diff;
    let mut result_usec: c_ulonglong = 0;
    int status, listener;
    pid_t pid;
    long ret;
    argc = parse_options(argc, argv, options, bench_seccomp_usage, 0);
    gettimeofday(&start, core::ptr::null_mut());
    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
    listener = user_notif_syscall(__NR_gettid,
    SECCOMP_FILTER_FLAG_NEW_LISTENER);
    if (listener < 0)
    err(EXIT_FAILURE, "can't create a notification descriptor");
    pid = fork();
    if (pid < 0)
    err(EXIT_FAILURE, "fork");
    if (pid == 0) {
    if (prctl(PR_SET_PDEATHSIG, SIGKILL, 0, 0, 0))
    err(EXIT_FAILURE, "can't set the parent death signal");
    while (1) {
    ret = syscall(__NR_gettid);
    if (ret == USER_NOTIF_MAGIC)
    continue;
    break;
    }
    _exit(1);
    }
    if (sync_mode) {
    if (ioctl(listener, SECCOMP_IOCTL_NOTIF_SET_FLAGS,
    SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP, 0))
    err(EXIT_FAILURE,
    "can't set SECCOMP_USER_NOTIF_FD_SYNC_WAKE_UP");
    }
    user_notification_sync_loop(listener);
    kill(pid, SIGKILL);
    if (waitpid(pid, &status, 0) != pid)
    err(EXIT_FAILURE, "waitpid(%d) failed", pid);
    if (!WIFSIGNALED(status) || WTERMSIG(status) != SIGKILL)
    errx(EXIT_FAILURE, "unexpected exit code: %d", status);
    gettimeofday(&stop, core::ptr::null_mut());
    timersub(&stop, &start, &diff);
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    printf("# Executed %" PRIu64 " system calls\n\n",
    loops);
    result_usec = diff.tv_sec * USEC_PER_SEC;
    result_usec += diff.tv_usec;
    printf(" %14s: %lu.%03lu [sec]\n\n", "Total time",
    (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec / USEC_PER_MSEC));
    printf(" %14lf usecs/op\n",
    (double)result_usec / (double)loops);
    printf(" %14d ops/sec\n",
    (int)((double)loops /
    ((double)result_usec / (double)USEC_PER_SEC)));
    break;
    case BENCH_FORMAT_SIMPLE:
    printf("%lu.%03lu\n",
    (unsigned long) diff.tv_sec,
    (unsigned long) (diff.tv_usec / USEC_PER_MSEC));
    break;
    default:
// reaching here is something disaster
    fprintf(stderr, "Unknown format:%d\n", bench_format);
    exit(1);
    break;
    }
    return 0;
    }
