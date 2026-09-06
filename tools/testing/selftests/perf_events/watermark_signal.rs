//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/perf_events/watermark_signal.c
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

    static int sigio_count;
    static void handle_sigio(int signum __maybe_unused,
    siginfo_t *oh __maybe_unused,
    void *uc __maybe_unused)
    {
    ++sigio_count;
    }
#[no_mangle]
unsafe extern "C" fn do_child() {
    static void do_child(void)
    {
    raise(SIGSTOP);
    for (int i = 0; i < 20; ++i)
    sleep(1);
    raise(SIGSTOP);
    exit(0);
    }
    TEST(watermark_signal)
    {
    struct perf_event_attr attr;
    struct perf_event_mmap_page *p = core::ptr::null_mut();
    struct sigaction previous_sigio, sigio = { 0 };
    let mut child: pid_t = -1;
    int child_status;
    let mut fd: c_int = -1;
    let mut page_size: c_long = sysconf(_SC_PAGE_SIZE);
    sigio.sa_sigaction = handle_sigio;
    EXPECT_EQ(sigaction(SIGIO, &sigio, &previous_sigio), 0);
    memset(&attr, 0, sizeof(attr));
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_DUMMY;
    attr.sample_period = 1;
    attr.disabled = 1;
    attr.watermark = 1;
    attr.context_switch = 1;
    attr.wakeup_watermark = 1;
    child = fork();
    EXPECT_GE(child, 0);
    if (child == 0)
    do_child();
#[no_mangle]
pub unsafe extern "C" fn if(0: child <) -> else {
    perror("fork()");
    goto cleanup;
    }
    if (waitpid(child, &child_status, WSTOPPED) != child ||
    !(WIFSTOPPED(child_status) && WSTOPSIG(child_status) == SIGSTOP)) {
    fprintf(stderr,
    "failed to synchronize with child errno=%d status=%x\n",
    errno,
    child_status);
    goto cleanup;
    }
    fd = syscall(__NR_perf_event_open, &attr, child, -1, -1,
    PERF_FLAG_FD_CLOEXEC);
    if (fd < 0) {
    fprintf(stderr, "failed opening event %llx\n", attr.config);
    goto cleanup;
    }
    if (fcntl(fd, F_SETFL, FASYNC)) {
    perror("F_SETFL FASYNC");
    goto cleanup;
    }
    if (fcntl(fd, F_SETOWN, getpid())) {
    perror("F_SETOWN getpid()");
    goto cleanup;
    }
    if (fcntl(fd, F_SETSIG, SIGIO)) {
    perror("F_SETSIG SIGIO");
    goto cleanup;
    }
    p = mmap(core::ptr::null_mut(), 2 * page_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if (p == MAP_FAILED) {
    perror("mmap");
    goto cleanup;
    }
    if (ioctl(fd, PERF_EVENT_IOC_ENABLE, 0)) {
    perror("PERF_EVENT_IOC_ENABLE");
    goto cleanup;
    }
    if (kill(child, SIGCONT) < 0) {
    perror("SIGCONT");
    goto cleanup;
    }
    if (waitpid(child, &child_status, WSTOPPED) != -1 || errno != EINTR)
    fprintf(stderr,
    "expected SIGIO to terminate wait errno=%d status=%x\n%d",
    errno,
    child_status,
    sigio_count);
    EXPECT_GE(sigio_count, 1);
    cleanup:
    if (p != core::ptr::null_mut())
    munmap(p, 2 * page_size);
    if (fd >= 0)
    close(fd);
    if (child > 0) {
    kill(child, SIGKILL);
    waitpid(child, core::ptr::null_mut(), 0);
    }
    sigaction(SIGIO, &previous_sigio, core::ptr::null_mut());
    }
    TEST_HARNESS_MAIN
