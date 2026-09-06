//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_stackid_cannot_attach.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
pub unsafe extern "C" fn test_get_stackid_cannot_attach() {
    void test_get_stackid_cannot_attach(void)
    {
    struct perf_event_attr attr = {
// .type = PERF_TYPE_SOFTWARE,
    .type = PERF_TYPE_HARDWARE,
    .config = PERF_COUNT_HW_CPU_CYCLES,
    .precise_ip = 1,
    .sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_BRANCH_STACK,
    .branch_sample_type = PERF_SAMPLE_BRANCH_USER |
    PERF_SAMPLE_BRANCH_NO_FLAGS |
    PERF_SAMPLE_BRANCH_NO_CYCLES |
    PERF_SAMPLE_BRANCH_CALL_STACK,
    .sample_period = 5000,
    .size = sizeof(struct perf_event_attr),
    };
    struct test_stacktrace_build_id *skel;
    let mut duration: __u32 = 0;
    int pmu_fd, err;
    skel = test_stacktrace_build_id__open();
    if (CHECK(!skel, "skel_open", "skeleton open failed\n"))
    return;
// override program type
    bpf_program__set_type(skel.progs.oncpu, BPF_PROG_TYPE_PERF_EVENT);
    err = test_stacktrace_build_id__load(skel);
    if (CHECK(err, "skel_load", "skeleton load failed: %d\n", err))
    goto cleanup;
    pmu_fd = syscall(__NR_perf_event_open, &attr, -1 /* pid */,
    0 /* cpu 0 */, -1 /* group id */,
    0 /* flags */);
    if (pmu_fd < 0 && (errno == ENOENT || errno == EOPNOTSUPP)) {
    printf("%s:SKIP:cannot open PERF_COUNT_HW_CPU_CYCLES with precise_ip > 0\n",
    __func__);
    test__skip();
    goto cleanup;
    }
    if (CHECK(pmu_fd < 0, "perf_event_open", "err %d errno %d\n",
    pmu_fd, errno))
    goto cleanup;
    skel.links.oncpu = bpf_program__attach_perf_event(skel.progs.oncpu,
    pmu_fd);
    ASSERT_ERR_PTR(skel.links.oncpu, "attach_perf_event_no_callchain");
    close(pmu_fd);
// add PERF_SAMPLE_CALLCHAIN, attach should succeed
    attr.sample_type |= PERF_SAMPLE_CALLCHAIN;
    pmu_fd = syscall(__NR_perf_event_open, &attr, -1 /* pid */,
    0 /* cpu 0 */, -1 /* group id */,
    0 /* flags */);
    if (CHECK(pmu_fd < 0, "perf_event_open", "err %d errno %d\n",
    pmu_fd, errno))
    goto cleanup;
    skel.links.oncpu = bpf_program__attach_perf_event(skel.progs.oncpu,
    pmu_fd);
    ASSERT_OK_PTR(skel.links.oncpu, "attach_perf_event_callchain");
    bpf_link__destroy(skel.links.oncpu);
    close(pmu_fd);
// add exclude_callchain_kernel, attach should fail
    attr.exclude_callchain_kernel = 1;
    pmu_fd = syscall(__NR_perf_event_open, &attr, -1 /* pid */,
    0 /* cpu 0 */, -1 /* group id */,
    0 /* flags */);
    if (CHECK(pmu_fd < 0, "perf_event_open", "err %d errno %d\n",
    pmu_fd, errno))
    goto cleanup;
    skel.links.oncpu = bpf_program__attach_perf_event(skel.progs.oncpu,
    pmu_fd);
    ASSERT_ERR_PTR(skel.links.oncpu, "attach_perf_event_exclude_callchain_kernel");
    close(pmu_fd);
    cleanup:
    test_stacktrace_build_id__destroy(skel);
    }
