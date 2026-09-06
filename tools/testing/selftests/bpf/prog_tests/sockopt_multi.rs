//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sockopt_multi.c
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

    static int run_getsockopt_test(struct sockopt_multi *obj, int cg_parent,
    int cg_child, int sock_fd)
    {
    struct bpf_link *link_parent = core::ptr::null_mut();
    struct bpf_link *link_child = core::ptr::null_mut();
    socklen_t optlen;
    __u8 buf;
    int err;
// Set IP_TOS to the expected value (0x80).
    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0x80) {
    log_err("Unexpected getsockopt 0x%x != 0x80 without BPF", buf);
    err = -1;
    goto detach;
    }
// Attach child program and make sure it returns new value:
// - kernel:      -> 0x80
// - child:  0x80 -> 0x90
//
    link_child = bpf_program__attach_cgroup(obj.progs._getsockopt_child,
    cg_child);
    if (!ASSERT_OK_PTR(link_child, "cg-attach-getsockopt_child"))
    goto detach;
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0x90) {
    log_err("Unexpected getsockopt 0x%x != 0x90", buf);
    err = -1;
    goto detach;
    }
// Attach parent program and make sure it returns new value:
// - kernel:      -> 0x80
// - child:  0x80 -> 0x90
// - parent: 0x90 -> 0xA0
//
    link_parent = bpf_program__attach_cgroup(obj.progs._getsockopt_parent,
    cg_parent);
    if (!ASSERT_OK_PTR(link_parent, "cg-attach-getsockopt_parent"))
    goto detach;
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0xA0) {
    log_err("Unexpected getsockopt 0x%x != 0xA0", buf);
    err = -1;
    goto detach;
    }
// Setting unexpected initial sockopt should return EPERM:
// - kernel: -> 0x40
// - child:  unexpected 0x40, EPERM
// - parent: unexpected 0x40, EPERM
//
    buf = 0x40;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (!err) {
    log_err("Unexpected success from getsockopt(IP_TOS)");
    goto detach;
    }
// Detach child program and make sure we still get EPERM:
// - kernel: -> 0x40
// - parent: unexpected 0x40, EPERM
//
    bpf_link__destroy(link_child);
    link_child = core::ptr::null_mut();
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (!err) {
    log_err("Unexpected success from getsockopt(IP_TOS)");
    goto detach;
    }
// Set initial value to the one the parent program expects:
// - kernel:      -> 0x90
// - parent: 0x90 -> 0xA0
//
    buf = 0x90;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0xA0) {
    log_err("Unexpected getsockopt 0x%x != 0xA0", buf);
    err = -1;
    goto detach;
    }
    detach:
    bpf_link__destroy(link_child);
    bpf_link__destroy(link_parent);
    return err;
    }
    static int run_setsockopt_test(struct sockopt_multi *obj, int cg_parent,
    int cg_child, int sock_fd)
    {
    struct bpf_link *link_parent = core::ptr::null_mut();
    struct bpf_link *link_child = core::ptr::null_mut();
    socklen_t optlen;
    __u8 buf;
    int err;
// Set IP_TOS to the expected value (0x80).
    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0x80) {
    log_err("Unexpected getsockopt 0x%x != 0x80 without BPF", buf);
    err = -1;
    goto detach;
    }
// Attach child program and make sure it adds 0x10.
    link_child = bpf_program__attach_cgroup(obj.progs._setsockopt,
    cg_child);
    if (!ASSERT_OK_PTR(link_child, "cg-attach-setsockopt_child"))
    goto detach;
    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0x80 + 0x10) {
    log_err("Unexpected getsockopt 0x%x != 0x80 + 0x10", buf);
    err = -1;
    goto detach;
    }
// Attach parent program and make sure it adds another 0x10.
    link_parent = bpf_program__attach_cgroup(obj.progs._setsockopt,
    cg_parent);
    if (!ASSERT_OK_PTR(link_parent, "cg-attach-setsockopt_parent"))
    goto detach;
    buf = 0x80;
    err = setsockopt(sock_fd, SOL_IP, IP_TOS, &buf, 1);
    if (err < 0) {
    log_err("Failed to call setsockopt(IP_TOS)");
    goto detach;
    }
    buf = 0x00;
    optlen = 1;
    err = getsockopt(sock_fd, SOL_IP, IP_TOS, &buf, &optlen);
    if (err) {
    log_err("Failed to call getsockopt(IP_TOS)");
    goto detach;
    }
    if (buf != 0x80 + 2 * 0x10) {
    log_err("Unexpected getsockopt 0x%x != 0x80 + 2 * 0x10", buf);
    err = -1;
    goto detach;
    }
    detach:
    bpf_link__destroy(link_child);
    bpf_link__destroy(link_parent);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn test_sockopt_multi() {
    void test_sockopt_multi(void)
    {
    let mut cg_parent: c_int = -1, cg_child = -1;
    struct sockopt_multi *obj = core::ptr::null_mut();
    let mut sock_fd: c_int = -1;
    cg_parent = test__join_cgroup("/parent");
    if (!ASSERT_GE(cg_parent, 0, "join_cgroup /parent"))
    goto out;
    cg_child = test__join_cgroup("/parent/child");
    if (!ASSERT_GE(cg_child, 0, "join_cgroup /parent/child"))
    goto out;
    obj = sockopt_multi__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    goto out;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (!ASSERT_GE(sock_fd, 0, "socket"))
    goto out;
    ASSERT_OK(run_getsockopt_test(obj, cg_parent, cg_child, sock_fd), "getsockopt_test");
    ASSERT_OK(run_setsockopt_test(obj, cg_parent, cg_child, sock_fd), "setsockopt_test");
    out:
    close(sock_fd);
    sockopt_multi__destroy(obj);
    close(cg_child);
    close(cg_parent);
    }
