//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/task_fd_query_tp.c
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

    static void test_task_fd_query_tp_core(const char *probe_name,
    const char *tp_name)
    {
    const char *file = "./test_tracepoint.bpf.o";
    int err, bytes, efd, prog_fd, pmu_fd;
    let mut attr: perf_event_attr = {};
    __u64 probe_offset, probe_addr;
    __u32 len, prog_id, fd_type;
    struct bpf_object *obj = core::ptr::null_mut();
    let mut duration: __u32 = 0;
    char buf[256];
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_TRACEPOINT, &obj, &prog_fd);
    if (CHECK(err, "bpf_prog_test_load", "err %d errno %d\n", err, errno))
    goto close_prog;
    if (access("/sys/kernel/tracing/trace", F_OK) == 0) {
    snprintf(buf, sizeof(buf),
    "/sys/kernel/tracing/events/%s/id", probe_name);
    } else {
    snprintf(buf, sizeof(buf),
    "/sys/kernel/debug/tracing/events/%s/id", probe_name);
    }
    efd = open(buf, O_RDONLY, 0);
    if (CHECK(efd < 0, "open", "err %d errno %d\n", efd, errno))
    goto close_prog;
    bytes = read(efd, buf, sizeof(buf));
    close(efd);
    if (CHECK(bytes <= 0 || bytes >= sizeof(buf), "read",
    "bytes %d errno %d\n", bytes, errno))
    goto close_prog;
    attr.config = strtol(buf, core::ptr::null_mut(), 0);
    attr.type = PERF_TYPE_TRACEPOINT;
    attr.sample_type = PERF_SAMPLE_RAW;
    attr.sample_period = 1;
    attr.wakeup_events = 1;
    pmu_fd = syscall(__NR_perf_event_open, &attr, -1 /* pid */,
    0 /* cpu 0 */, -1 /* group id */,
    0 /* flags */);
    if (CHECK(err, "perf_event_open", "err %d errno %d\n", err, errno))
    goto close_pmu;
    err = ioctl(pmu_fd, PERF_EVENT_IOC_ENABLE, 0);
    if (CHECK(err, "perf_event_ioc_enable", "err %d errno %d\n", err,
    errno))
    goto close_pmu;
    err = ioctl(pmu_fd, PERF_EVENT_IOC_SET_BPF, prog_fd);
    if (CHECK(err, "perf_event_ioc_set_bpf", "err %d errno %d\n", err,
    errno))
    goto close_pmu;
// query (getpid(), pmu_fd)
    len = sizeof(buf);
    err = bpf_task_fd_query(getpid(), pmu_fd, 0, buf, &len, &prog_id,
    &fd_type, &probe_offset, &probe_addr);
    if (CHECK(err < 0, "bpf_task_fd_query", "err %d errno %d\n", err,
    errno))
    goto close_pmu;
    err = (fd_type == BPF_FD_TYPE_TRACEPOINT) && !strcmp(buf, tp_name);
    if (CHECK(!err, "check_results", "fd_type %d tp_name %s\n",
    fd_type, buf))
    goto close_pmu;
    close_pmu:
    close(pmu_fd);
    close_prog:
    bpf_object__close(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_task_fd_query_tp() {
    void test_task_fd_query_tp(void)
    {
    test_task_fd_query_tp_core("sched/sched_switch",
    "sched_switch");
    test_task_fd_query_tp_core("syscalls/sys_enter_read",
    "sys_enter_read");
    }
