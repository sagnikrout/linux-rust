//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_bpf_smc.c
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

pub const IPPROTO_SMC: c_int = 256;

pub const SERVICE_1: c_int = 80;
pub const SERVICE_2: c_int = 443;
pub const SERVICE_3: c_int = 8443;

    static struct netns_obj *test_netns;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_policy_ip_key {
    pub sip: __u32,
    pub dip: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_policy_ip_value {
    pub mode: __u8,
}

// s390x has default seid
    static bool setup_ueid(void) { return true; }
    static void cleanup_ueid(void) {}

    enum {
    SMC_NETLINK_ADD_UEID = 10,
    SMC_NETLINK_REMOVE_UEID
    };
    enum {
    SMC_NLA_EID_TABLE_UNSPEC,
    SMC_NLA_EID_TABLE_ENTRY,    /* string */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgtemplate {
    pub n: nlmsghdr,
    pub g: genlmsghdr,
    pub buf: [c_char; 1024],
}

    let mut smc_nl_family_id: static uint16_t = -1;
    static int send_cmd(int fd, __u16 nlmsg_type, __u32 nlmsg_pid,
    __u16 nlmsg_flags, __u8 genl_cmd, __u16 nla_type,
    void *nla_data, int nla_len)
    {
    struct nlattr *na;
    struct sockaddr_nl nladdr;
    int r, buflen;
    char *buf;
    let mut msg: msgtemplate = {0};
    msg.n.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN);
    msg.n.nlmsg_type = nlmsg_type;
    msg.n.nlmsg_flags = nlmsg_flags;
    msg.n.nlmsg_seq = 0;
    msg.n.nlmsg_pid = nlmsg_pid;
    msg.g.cmd = genl_cmd;
    msg.g.version = 1;
    na = (struct nlattr *)GENLMSG_DATA(&msg);
    na.nla_type = nla_type;
    na.nla_len = nla_len + 1 + NLA_HDRLEN;
    memcpy(NLA_DATA(na), nla_data, nla_len);
    msg.n.nlmsg_len += NLMSG_ALIGN(na.nla_len);
    buf = (char *)&msg;
    buflen = msg.n.nlmsg_len;
    memset(&nladdr, 0, sizeof(nladdr));
    nladdr.nl_family = AF_NETLINK;
    while ((r = sendto(fd, buf, buflen, 0, (struct sockaddr *)&nladdr,
    sizeof(nladdr))) < buflen) {
    if (r > 0) {
    buf += r;
    buflen -= r;
    } else if (errno != EAGAIN) {
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_smc_nl_family_id() -> bool {
    static bool get_smc_nl_family_id(void)
    {
    struct sockaddr_nl nl_src;
    struct msgtemplate msg;
    struct nlattr *nl;
    int fd, ret;
    pid_t pid;
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if (!ASSERT_OK_FD(fd, "nl_family socket"))
    return false;
    pid = getpid();
    memset(&nl_src, 0, sizeof(nl_src));
    nl_src.nl_family = AF_NETLINK;
    nl_src.nl_pid = pid;
    ret = bind(fd, (struct sockaddr *)&nl_src, sizeof(nl_src));
    if (!ASSERT_OK(ret, "nl_family bind"))
    goto fail;
    ret = send_cmd(fd, GENL_ID_CTRL, pid,
    NLM_F_REQUEST, CTRL_CMD_GETFAMILY,
    CTRL_ATTR_FAMILY_NAME, (void *)SMC_GENL_FAMILY_NAME,
    strlen(SMC_GENL_FAMILY_NAME));
    if (!ASSERT_OK(ret, "nl_family query"))
    goto fail;
    ret = recv(fd, &msg, sizeof(msg), 0);
    if (msg.n.nlmsg_type == NLMSG_ERROR)
    goto fail;
    if (!ASSERT_FALSE(ret < 0 || !NLMSG_OK(&msg.n, ret),
    "nl_family response"))
    goto fail;
    nl = (struct nlattr *)GENLMSG_DATA(&msg);
    nl = (struct nlattr *)((char *)nl + NLA_ALIGN(nl.nla_len));
    if (!ASSERT_EQ(nl.nla_type, CTRL_ATTR_FAMILY_ID, "nl_family nla type"))
    goto fail;
    smc_nl_family_id = *(uint16_t *)NLA_DATA(nl);
    close(fd);
    return true;
    fail:
    close(fd);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn smc_ueid(op: c_int) -> bool {
    static bool smc_ueid(int op)
    {
    struct sockaddr_nl nl_src;
    struct msgtemplate msg;
    struct nlmsgerr *err;
    char test_ueid[32];
    int fd, ret;
    pid_t pid;
// UEID required
    memset(test_ueid, '\x20', sizeof(test_ueid));
    memcpy(test_ueid, SMC_BPFTEST_UEID, strlen(SMC_BPFTEST_UEID));
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if (!ASSERT_OK_FD(fd, "ueid socket"))
    return false;
    pid = getpid();
    memset(&nl_src, 0, sizeof(nl_src));
    nl_src.nl_family = AF_NETLINK;
    nl_src.nl_pid = pid;
    ret = bind(fd, (struct sockaddr *)&nl_src, sizeof(nl_src));
    if (!ASSERT_OK(ret, "ueid bind"))
    goto fail;
    ret = send_cmd(fd, smc_nl_family_id, pid,
    NLM_F_REQUEST | NLM_F_ACK, op, SMC_NLA_EID_TABLE_ENTRY,
    (void *)test_ueid, sizeof(test_ueid));
    if (!ASSERT_OK(ret, "ueid cmd"))
    goto fail;
    ret = recv(fd, &msg, sizeof(msg), 0);
    if (!ASSERT_FALSE(ret < 0 ||
    !NLMSG_OK(&msg.n, ret), "ueid response"))
    goto fail;
    if (msg.n.nlmsg_type == NLMSG_ERROR) {
    err = NLMSG_DATA(&msg);
    switch (op) {
    case SMC_NETLINK_REMOVE_UEID:
    if (!ASSERT_FALSE((err.error && err.error != -ENOENT),
    "ueid remove"))
    goto fail;
    break;
    case SMC_NETLINK_ADD_UEID:
    if (!ASSERT_OK(err.error, "ueid add"))
    goto fail;
    break;
    default:
    break;
    }
    }
    close(fd);
    return true;
    fail:
    close(fd);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn setup_ueid() -> bool {
    static bool setup_ueid(void)
    {
// get smc nl id
    if (!get_smc_nl_family_id())
    return false;
// clear old ueid for bpftest
    smc_ueid(SMC_NETLINK_REMOVE_UEID);
// smc-loopback required ueid
    return smc_ueid(SMC_NETLINK_ADD_UEID);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_ueid() {
    static void cleanup_ueid(void)
    {
    smc_ueid(SMC_NETLINK_REMOVE_UEID);
    }

#[no_mangle]
unsafe extern "C" fn setup_netns() -> bool {
    static bool setup_netns(void)
    {
    test_netns = netns_new(TEST_NS, true);
    if (!ASSERT_OK_PTR(test_netns, "open net namespace"))
    goto fail_netns;
    SYS(fail_ip, "ip addr add 127.0.1.0/8 dev lo");
    SYS(fail_ip, "ip addr add 127.0.2.0/8 dev lo");
    return true;
    fail_ip:
    netns_free(test_netns);
    fail_netns:
    return false;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_netns() {
    static void cleanup_netns(void)
    {
    netns_free(test_netns);
    }
#[no_mangle]
unsafe extern "C" fn setup_smc() -> bool {
    static bool setup_smc(void)
    {
    if (!setup_ueid())
    return false;
    if (!setup_netns())
    goto fail_netns;
    return true;
    fail_netns:
    cleanup_ueid();
    return false;
    }
#[no_mangle]
unsafe extern "C" fn set_client_addr_cb(fd: c_int, opts: *mut c_void) -> c_int {
    static int set_client_addr_cb(int fd, void *opts)
    {
    const char *src = (const char *)opts;
    struct sockaddr_in localaddr;
    localaddr.sin_family = AF_INET;
    localaddr.sin_port = htons(0);
    localaddr.sin_addr.s_addr = inet_addr(src);
    return !ASSERT_OK(bind(fd, &localaddr, sizeof(localaddr)), "client bind");
    }
#[no_mangle]
unsafe extern "C" fn run_link(src: *const c_char, dst: *const c_char, port: c_int) {
    static void run_link(const char *src, const char *dst, int port)
    {
    let mut opts: network_helper_opts = {0};
    int server, client;
    server = start_server_str(AF_INET, SOCK_STREAM, dst, port, core::ptr::null_mut());
    if (!ASSERT_OK_FD(server, "start service_1"))
    return;
    opts.proto = IPPROTO_TCP;
    opts.post_socket_cb = set_client_addr_cb;
    opts.cb_opts = (void *)src;
    client = connect_to_fd_opts(server, &opts);
    if (!ASSERT_OK_FD(client, "start connect"))
    goto fail_client;
    close(client);
    fail_client:
    close(server);
    }
#[no_mangle]
unsafe extern "C" fn block_link(map_fd: c_int, src: *const c_char, dst: *const c_char) {
    static void block_link(int map_fd, const char *src, const char *dst)
    {
    let mut val: smc_policy_ip_value = { .mode = /* block */ 0 };
    struct smc_policy_ip_key key = {
    .sip = inet_addr(src),
    .dip = inet_addr(dst),
    };
    bpf_map_update_elem(map_fd, &key, &val, BPF_ANY);
    }
//
// This test describes a real-life service topology as follows:
//
// +-------------> service_1
// link 1           |                     |
// +--------------------> server                   |  link 2
// |                         |                     V
// |                         +-------------> service_2
// |        link 3
// client -------------------> server_via_unsafe_path -> service_3
//
// Among them,
// 1. link-1 is very suitable for using SMC.
// 2. link-2 is not suitable for using SMC, because the mode of this link is
// kind of short-link services.
// 3. link-3 is also not suitable for using SMC, because the RDMA link is
// unavailable and needs to go through a long timeout before it can fallback
// to TCP.
// To achieve this goal, we use a customized SMC ip strategy via smc_hs_ctrl.
//
#[no_mangle]
unsafe extern "C" fn test_topo() {
    static void test_topo(void)
    {
    struct bpf_smc *skel;
    int rc, map_fd;
    skel = bpf_smc__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_smc__open_and_load"))
    return;
    rc = bpf_smc__attach(skel);
    if (!ASSERT_OK(rc, "bpf_smc__attach"))
    goto fail;
    map_fd = bpf_map__fd(skel.maps.smc_policy_ip);
    if (!ASSERT_OK_FD(map_fd, "bpf_map__fd"))
    goto fail;
// Mock the process of transparent replacement, since we will modify
// protocol to ipproto_smc accropding to it via
// fmod_ret/update_socket_protocol.
//
    write_sysctl("/proc/sys/net/smc/hs_ctrl", "linkcheck");
// Configure ip strat
    block_link(map_fd, CLIENT_IP, SERVER_IP_VIA_RISK_PATH);
    block_link(map_fd, SERVER_IP, SERVER_IP);
// should go with smc
    run_link(CLIENT_IP, SERVER_IP, SERVICE_1);
// should go with smc fallback
    run_link(SERVER_IP, SERVER_IP, SERVICE_2);
    ASSERT_EQ(skel.bss.smc_cnt, 2, "smc count");
    ASSERT_EQ(skel.bss.fallback_cnt, 1, "fallback count");
// should go with smc
    run_link(CLIENT_IP, SERVER_IP, SERVICE_2);
    ASSERT_EQ(skel.bss.smc_cnt, 3, "smc count");
    ASSERT_EQ(skel.bss.fallback_cnt, 1, "fallback count");
// should go with smc fallback
    run_link(CLIENT_IP, SERVER_IP_VIA_RISK_PATH, SERVICE_3);
    ASSERT_EQ(skel.bss.smc_cnt, 4, "smc count");
    ASSERT_EQ(skel.bss.fallback_cnt, 2, "fallback count");
    fail:
    bpf_smc__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_smc() {
    void test_bpf_smc(void)
    {
    if (!setup_smc()) {
    printf("setup for smc test failed, test SKIP:\n");
    test__skip();
    return;
    }
    if (test__start_subtest("topo"))
    test_topo();
    cleanup_ueid();
    cleanup_netns();
    }
