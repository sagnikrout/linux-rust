//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_storage.c
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
unsafe extern "C" fn setup_network(token: *mut nstoken) -> c_int {
    static int setup_network(struct nstoken **token)
    {
    SYS(fail, "ip netns add %s", TEST_NS);
// token = open_netns(TEST_NS);
    if (!ASSERT_OK_PTR(*token, "open netns"))
    goto cleanup_ns;
    SYS(cleanup_ns, "ip link set lo up");
    return 0;
    cleanup_ns:
    SYS_NOFAIL("ip netns del %s", TEST_NS);
    fail:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_network(ns: *mut nstoken) {
    static void cleanup_network(struct nstoken *ns)
    {
    close_netns(ns);
    SYS_NOFAIL("ip netns del %s", TEST_NS);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_storage() {
    void test_cgroup_storage(void)
    {
    struct bpf_cgroup_storage_key key;
    struct cgroup_storage *skel;
    struct nstoken *ns = core::ptr::null_mut();
    unsigned long long value;
    int cgroup_fd;
    int err;
    cgroup_fd = cgroup_setup_and_join(TEST_CGROUP);
    if (!ASSERT_OK_FD(cgroup_fd, "create cgroup"))
    return;
    if (!ASSERT_OK(setup_network(&ns), "setup network"))
    goto cleanup_cgroup;
    skel = cgroup_storage__open_and_load();
    if (!ASSERT_OK_PTR(skel, "load program"))
    goto cleanup_network;
    skel.links.bpf_prog =
    bpf_program__attach_cgroup(skel.progs.bpf_prog, cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.bpf_prog, "attach program"))
    goto cleanup_progs;
// Check that one out of every two packets is dropped
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_OK(err, "first ping");
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_NEQ(err, 0, "second ping");
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_OK(err, "third ping");
    err = bpf_map__get_next_key(skel.maps.cgroup_storage, core::ptr::null_mut(), &key,
    sizeof(key));
    if (!ASSERT_OK(err, "get first key"))
    goto cleanup_progs;
    err = bpf_map__lookup_elem(skel.maps.cgroup_storage, &key, sizeof(key),
    &value, sizeof(value), 0);
    if (!ASSERT_OK(err, "first packet count read"))
    goto cleanup_progs;
// Add one to the packet counter, check again packet filtering
    value++;
    err = bpf_map__update_elem(skel.maps.cgroup_storage, &key, sizeof(key),
    &value, sizeof(value), 0);
    if (!ASSERT_OK(err, "increment packet counter"))
    goto cleanup_progs;
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_OK(err, "fourth ping");
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_NEQ(err, 0, "fifth ping");
    err = SYS_NOFAIL(PING_CMD);
    ASSERT_OK(err, "sixth ping");
    err = bpf_map__get_next_key(skel.maps.cgroup_storage, &key, &key,
    sizeof(key));
    ASSERT_ERR(err, "bpf_map__get_next_key should fail");
    ASSERT_EQ(errno, ENOENT, "no second key");
    cleanup_progs:
    cgroup_storage__destroy(skel);
    cleanup_network:
    cleanup_network(ns);
    cleanup_cgroup:
    close(cgroup_fd);
    cleanup_cgroup_environment();
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_storage_oob() {
    void test_cgroup_storage_oob(void)
    {
    struct cgroup_storage *skel;
    int cgroup_fd, sock_fd;
    cgroup_fd = cgroup_setup_and_join(TEST_CGROUP);
    if (!ASSERT_OK_FD(cgroup_fd, "create cgroup"))
    return;
// Load and attach BPF program
    skel = cgroup_storage__open_and_load();
    if (!ASSERT_OK_PTR(skel, "cgroup_storage__open_and_load"))
    goto cleanup_cgroup;
    skel.links.trigger_oob = bpf_program__attach_cgroup(skel.progs.trigger_oob,
    cgroup_fd);
    if (!ASSERT_OK_PTR(skel.links.trigger_oob, "attach_cgroup"))
    goto cleanup_skel;
// Create a socket to trigger cgroup/sock_create hook.
// This will execute our BPF program and trigger the OOB read
// if the bug is present (before the fix).
//
    sock_fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (!ASSERT_OK_FD(sock_fd, "create socket"))
    goto cleanup_skel;
    close(sock_fd);
// If we reach here without a kernel panic or KASAN report,
// the test passes (the fix is working).
//
    cleanup_skel:
    cgroup_storage__destroy(skel);
    cleanup_cgroup:
    close(cgroup_fd);
    cleanup_cgroup_environment();
    }
