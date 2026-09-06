//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/task_fd_query_rawtp.c
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

#[no_mangle]
pub unsafe extern "C" fn test_task_fd_query_rawtp() {
    void test_task_fd_query_rawtp(void)
    {
    const char *file = "./test_get_stack_rawtp.bpf.o";
    __u64 probe_offset, probe_addr;
    __u32 len, prog_id, fd_type;
    struct bpf_object *obj;
    int efd, err, prog_fd;
    let mut duration: __u32 = 0;
    char buf[256];
    err = bpf_prog_test_load(file, BPF_PROG_TYPE_RAW_TRACEPOINT, &obj, &prog_fd);
    if (CHECK(err, "prog_load raw tp", "err %d errno %d\n", err, errno))
    return;
    efd = bpf_raw_tracepoint_open("sys_enter", prog_fd);
    if (CHECK(efd < 0, "raw_tp_open", "err %d errno %d\n", efd, errno))
    goto close_prog;
// query (getpid(), efd)
    len = sizeof(buf);
    err = bpf_task_fd_query(getpid(), efd, 0, buf, &len, &prog_id,
    &fd_type, &probe_offset, &probe_addr);
    if (CHECK(err < 0, "bpf_task_fd_query", "err %d errno %d\n", err,
    errno))
    goto close_prog;
    err = fd_type == BPF_FD_TYPE_RAW_TRACEPOINT &&
    strcmp(buf, "sys_enter") == 0;
    if (CHECK(!err, "check_results", "fd_type %d tp_name %s\n",
    fd_type, buf))
    goto close_prog;
// test zero len
    len = 0;
    err = bpf_task_fd_query(getpid(), efd, 0, buf, &len, &prog_id,
    &fd_type, &probe_offset, &probe_addr);
    if (CHECK(err < 0, "bpf_task_fd_query (len = 0)", "err %d errno %d\n",
    err, errno))
    goto close_prog;
    err = fd_type == BPF_FD_TYPE_RAW_TRACEPOINT &&
    len == strlen("sys_enter");
    if (CHECK(!err, "check_results", "fd_type %d len %u\n", fd_type, len))
    goto close_prog;
// test empty buffer
    len = sizeof(buf);
    err = bpf_task_fd_query(getpid(), efd, 0, 0, &len, &prog_id,
    &fd_type, &probe_offset, &probe_addr);
    if (CHECK(err < 0, "bpf_task_fd_query (buf = 0)", "err %d errno %d\n",
    err, errno))
    goto close_prog;
    err = fd_type == BPF_FD_TYPE_RAW_TRACEPOINT &&
    len == strlen("sys_enter");
    if (CHECK(!err, "check_results", "fd_type %d len %u\n", fd_type, len))
    goto close_prog;
// test smaller buffer
    len = 3;
    err = bpf_task_fd_query(getpid(), efd, 0, buf, &len, &prog_id,
    &fd_type, &probe_offset, &probe_addr);
    if (CHECK(err >= 0 || errno != ENOSPC, "bpf_task_fd_query (len = 3)",
    "err %d errno %d\n", err, errno))
    goto close_prog;
    err = fd_type == BPF_FD_TYPE_RAW_TRACEPOINT &&
    len == strlen("sys_enter") &&
    strcmp(buf, "sy") == 0;
    if (CHECK(!err, "check_results", "fd_type %d len %u\n", fd_type, len))
    goto close_prog;
    close_prog:
    bpf_object__close(obj);
    }
