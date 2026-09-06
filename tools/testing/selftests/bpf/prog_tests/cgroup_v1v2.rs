//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_v1v2.c
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
unsafe extern "C" fn run_test(cgroup_fd: c_int, server_fd: c_int, classid: bool) -> c_int {
    static int run_test(int cgroup_fd, int server_fd, bool classid)
    {
    struct connect4_dropper *skel;
    int fd, err = 0, port;
    skel = connect4_dropper__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return -1;
    port = get_socket_local_port(server_fd);
    if (!ASSERT_GE(port, 0, "get_socket_local_port"))
    return -1;
    skel.bss.port = ntohs(port);
    skel.links.connect_v4_dropper =
    bpf_program__attach_cgroup(skel.progs.connect_v4_dropper,
    cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.connect_v4_dropper, "prog_attach")) {
    err = -1;
    goto out;
    }
    if (classid && !ASSERT_OK(join_classid(), "join_classid")) {
    err = -1;
    goto out;
    }
    errno = 0;
    fd = connect_to_fd_opts(server_fd, core::ptr::null_mut());
    if (fd >= 0) {
    log_err("Unexpected success to connect to server");
    err = -1;
    close(fd);
    } else if (errno != EPERM) {
    log_err("Unexpected errno from connect to server");
    err = -1;
    }
    out:
    connect4_dropper__destroy(skel);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_v1v2() {
    void test_cgroup_v1v2(void)
    {
    let mut opts: network_helper_opts = {};
    int server_fd, client_fd, cgroup_fd;
// Step 1: Check base connectivity works without any BPF.
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(server_fd, 0, "server_fd"))
    return;
    client_fd = connect_to_fd_opts(server_fd, &opts);
    if (!ASSERT_GE(client_fd, 0, "client_fd")) {
    close(server_fd);
    return;
    }
    close(client_fd);
    close(server_fd);
// Step 2: Check BPF policy prog attached to cgroups drops connectivity.
    cgroup_fd = test__join_cgroup("/connect_dropper");
    if (!ASSERT_GE(cgroup_fd, 0, "cgroup_fd"))
    return;
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (!ASSERT_GE(server_fd, 0, "server_fd")) {
    close(cgroup_fd);
    return;
    }
    ASSERT_OK(run_test(cgroup_fd, server_fd, false), "cgroup-v2-only");
    setup_classid_environment();
    set_classid();
    ASSERT_OK(run_test(cgroup_fd, server_fd, true), "cgroup-v1v2");
    cleanup_classid_environment();
    close(server_fd);
    close(cgroup_fd);
    }
