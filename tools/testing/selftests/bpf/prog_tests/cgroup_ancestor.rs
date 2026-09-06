//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/cgroup_ancestor.c
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

pub const NUM_CGROUP_LEVELS: c_int = 4;
pub const WAIT_AUTO_IP_MAX_ATTEMPT: c_int = 10;

pub const DST_PORT: c_int = 1234;
pub const MAX_ASSERT_NAME: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub skel: *mut cgroup_ancestor,
    pub qdisc: bpf_tc_hook,
    pub tc_attach: bpf_tc_opts,
    pub ns: *mut nstoken,
}

#[no_mangle]
unsafe extern "C" fn send_datagram() -> c_int {
    static int send_datagram(void)
    {
    unsigned char buf[] = "some random test data";
    struct sockaddr_in6 addr = { .sin6_family = AF_INET6,
    .sin6_port = htons(DST_PORT), };
    int sock, n;
    if (!ASSERT_EQ(inet_pton(AF_INET6, DST_ADDR, &addr.sin6_addr), 1,
    "inet_pton"))
    return -1;
    sock = socket(AF_INET6, SOCK_DGRAM, 0);
    if (!ASSERT_OK_FD(sock, "create socket"))
    return sock;
    if (!ASSERT_OK(connect(sock, (struct sockaddr *)&addr, sizeof(addr)), "connect")) {
    close(sock);
    return -1;
    }
    n = sendto(sock, buf, sizeof(buf), 0, (const struct sockaddr *)&addr,
    sizeof(addr));
    close(sock);
    return ASSERT_EQ(n, sizeof(buf), "send data") ? 0 : -1;
    }
#[no_mangle]
unsafe extern "C" fn setup_network(t: *mut test_data) -> c_int {
    static int setup_network(struct test_data *t)
    {
    SYS(fail, "ip netns add %s", TEST_NS);
    t.ns = open_netns(TEST_NS);
    if (!ASSERT_OK_PTR(t.ns, "open netns"))
    goto cleanup_ns;
    SYS(close_ns, "ip link set lo up");
    memset(&t.qdisc, 0, sizeof(t.qdisc));
    t.qdisc.sz = sizeof(t.qdisc);
    t.qdisc.attach_point = BPF_TC_EGRESS;
    t.qdisc.ifindex = if_nametoindex("lo");
    if (!ASSERT_NEQ(t.qdisc.ifindex, 0, "if_nametoindex"))
    goto close_ns;
    if (!ASSERT_OK(bpf_tc_hook_create(&t.qdisc), "qdisc add"))
    goto close_ns;
    memset(&t.tc_attach, 0, sizeof(t.tc_attach));
    t.tc_attach.sz = sizeof(t.tc_attach);
    t.tc_attach.prog_fd = bpf_program__fd(t.skel.progs.log_cgroup_id);
    if (!ASSERT_OK(bpf_tc_attach(&t.qdisc, &t.tc_attach), "filter add"))
    goto cleanup_qdisc;
    return 0;
    cleanup_qdisc:
    bpf_tc_hook_destroy(&t.qdisc);
    close_ns:
    close_netns(t.ns);
    cleanup_ns:
    SYS_NOFAIL("ip netns del %s", TEST_NS);
    fail:
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_network(t: *mut test_data) {
    static void cleanup_network(struct test_data *t)
    {
    bpf_tc_detach(&t.qdisc, &t.tc_attach);
    bpf_tc_hook_destroy(&t.qdisc);
    close_netns(t.ns);
    SYS_NOFAIL("ip netns del %s", TEST_NS);
    }
#[no_mangle]
unsafe extern "C" fn check_ancestors_ids(t: *mut test_data) {
    static void check_ancestors_ids(struct test_data *t)
    {
    __u64 expected_ids[NUM_CGROUP_LEVELS];
    char assert_name[MAX_ASSERT_NAME];
    __u32 level;
    expected_ids[0] = get_cgroup_id("/.."); /* root cgroup */
    expected_ids[1] = get_cgroup_id("");
    expected_ids[2] = get_cgroup_id(CGROUP_PATH);
    expected_ids[3] = 0; /* non-existent cgroup */
    for (level = 0; level < NUM_CGROUP_LEVELS; level++) {
    snprintf(assert_name, MAX_ASSERT_NAME,
    "ancestor id at level %d", level);
    ASSERT_EQ(t.skel.bss.cgroup_ids[level], expected_ids[level],
    assert_name);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_cgroup_ancestor() {
    void test_cgroup_ancestor(void)
    {
    struct test_data t;
    int cgroup_fd;
    t.skel = cgroup_ancestor__open_and_load();
    if (!ASSERT_OK_PTR(t.skel, "open and load"))
    return;
    t.skel.bss.dport = htons(DST_PORT);
    cgroup_fd = cgroup_setup_and_join(CGROUP_PATH);
    if (cgroup_fd < 0)
    goto cleanup_progs;
    if (setup_network(&t))
    goto cleanup_cgroups;
    if (send_datagram())
    goto cleanup_network;
    check_ancestors_ids(&t);
    cleanup_network:
    cleanup_network(&t);
    cleanup_cgroups:
    close(cgroup_fd);
    cleanup_cgroup_environment();
    cleanup_progs:
    cgroup_ancestor__destroy(t.skel);
    }
