//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/ksock.c
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
// Copyright (c) 2026 Isovalent

pub const RECV_PORT: c_int = 7777;
pub const RECV_TIMEOUT_SEC: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksock_test_env {
    pub nstoken: *mut nstoken,
    pub rfd: c_int,
}

#[no_mangle]
unsafe extern "C" fn ksock_test_env_setup(env: *mut ksock_test_env) -> bool {
    static bool ksock_test_env_setup(struct ksock_test_env *env)
    {
    struct sockaddr_in addr = {
    .sin_family = AF_INET,
    .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    .sin_port = htons(RECV_PORT),
    };
    let mut tv: timeval = { .tv_sec = RECV_TIMEOUT_SEC };
    int err;
    memset(env, 0, sizeof(*env));
    env.rfd = -1;
    if (!ASSERT_OK(make_netns(NS_TEST), "make_netns"))
    goto fail;
    env.nstoken = open_netns(NS_TEST);
    if (!ASSERT_OK_PTR(env.nstoken, "open_netns"))
    goto fail;
    env.rfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (!ASSERT_OK_FD(env.rfd, "receiver socket"))
    goto fail;
    err = bind(env.rfd, (struct sockaddr *)&addr, sizeof(addr));
    if (!ASSERT_OK(err, "bind receiver"))
    goto fail;
    err = setsockopt(env.rfd, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));
    if (!ASSERT_OK(err, "set rcvtimeo"))
    goto fail;
    return true;
    fail:
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn test_ksock_lsm() {
    void test_ksock_lsm(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct ksock_test_env env;
    struct sockaddr_in trigger_addr = {
    .sin_family = AF_INET,
    .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    struct ksock_lsm *skel;
    char recv_data[sizeof(skel.data.send_data)] = {};
    ssize_t n;
    let mut tfd: c_int = -1;
    int err;
    skel = ksock_lsm__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel open_and_load"))
    return;
    if (!ksock_test_env_setup(&env))
    goto fail;
// Step 1: Run the setup SYSCALL prog to create the ksock
    skel.bss.ipv4_remote = htonl(INADDR_LOOPBACK);
    skel.bss.remote_port = RECV_PORT;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.ksock_setup),
    &opts);
    if (!ASSERT_OK(err, "ksock_setup run"))
    goto fail;
    if (!ASSERT_OK(opts.retval, "ksock_setup retval"))
    goto fail;
// Step 2: Attach LSM prog and trigger socket_bind from userspace
    skel.links.ksock_socket_bind =
    bpf_program__attach_lsm(skel.progs.ksock_socket_bind);
    if (!ASSERT_OK_PTR(skel.links.ksock_socket_bind,
    "attach socket_bind lsm"))
    goto fail;
    tfd = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
    if (!ASSERT_OK_FD(tfd, "trigger socket"))
    goto fail;
    skel.bss.target_pid = getpid();
    err = bind(tfd, (struct sockaddr *)&trigger_addr, sizeof(trigger_addr));
    skel.bss.target_pid = 0;
    if (!ASSERT_OK(err, "trigger bind"))
    goto fail;
// Step 3: Verify the LSM hook sent the notification
    if (!ASSERT_EQ(skel.data.send_ret, sizeof(skel.data.send_data),
    "LSM send bytes"))
    goto fail;
    n = recvfrom(env.rfd, recv_data, sizeof(recv_data), 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (ASSERT_EQ(n, sizeof(recv_data), "recvfrom len"))
    ASSERT_MEMEQ(recv_data, skel.data.send_data, sizeof(recv_data),
    "payload match");
    fail:
    if (tfd >= 0)
    close(tfd);
    if (env.rfd >= 0)
    close(env.rfd);
    if (env.nstoken)
    close_netns(env.nstoken);
    remove_netns(NS_TEST);
    ksock_lsm__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_ksock_lsm_verifier() {
    void test_ksock_lsm_verifier(void)
    {
    RUN_TESTS(ksock_lsm_verifier);
    }
