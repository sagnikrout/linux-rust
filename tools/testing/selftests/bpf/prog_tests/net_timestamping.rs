//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/net_timestamping.c
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


    static const char addr4_str[] = "127.0.0.1";
    static const char addr6_str[] = "::1";
    static struct net_timestamping *skel;
    let mut cfg_payload_len: static int = 30;
    static struct timespec usr_ts;
    static u64 delay_tolerance_nsec = 10000000000; /* 10 seconds */
    int SK_TS_SCHED;
    int SK_TS_TXSW;
    int SK_TS_ACK;
#[no_mangle]
unsafe extern "C" fn timespec_to_ns64(ts: *mut timespec) -> i64 {
    static int64_t timespec_to_ns64(struct timespec *ts)
    {
    return ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
    }
#[no_mangle]
unsafe extern "C" fn validate_key(tskey: c_int, tstype: c_int) {
    static void validate_key(int tskey, int tstype)
    {
    let mut expected_tskey: static int = -1;
    if (tstype == SCM_TSTAMP_SCHED)
    expected_tskey = cfg_payload_len - 1;
    ASSERT_EQ(expected_tskey, tskey, "tskey mismatch");
    expected_tskey = tskey;
    }
#[no_mangle]
unsafe extern "C" fn validate_timestamp(cur: *mut timespec, prev: *mut timespec) {
    static void validate_timestamp(struct timespec *cur, struct timespec *prev)
    {
    int64_t cur_ns, prev_ns;
    cur_ns = timespec_to_ns64(cur);
    prev_ns = timespec_to_ns64(prev);
    ASSERT_LT(cur_ns - prev_ns, delay_tolerance_nsec, "latency");
    }
    static void test_socket_timestamp(struct scm_timestamping *tss, int tstype,
    int tskey)
    {
    static struct timespec prev_ts;
    validate_key(tskey, tstype);
    switch (tstype) {
    case SCM_TSTAMP_SCHED:
    validate_timestamp(&tss.ts[0], &usr_ts);
    SK_TS_SCHED += 1;
    break;
    case SCM_TSTAMP_SND:
    validate_timestamp(&tss.ts[0], &prev_ts);
    SK_TS_TXSW += 1;
    break;
    case SCM_TSTAMP_ACK:
    validate_timestamp(&tss.ts[0], &prev_ts);
    SK_TS_ACK += 1;
    break;
    }
    prev_ts = tss.ts[0];
    }
#[no_mangle]
unsafe extern "C" fn test_recv_errmsg_cmsg(msg: *mut msghdr) {
    static void test_recv_errmsg_cmsg(struct msghdr *msg)
    {
    struct sock_extended_err *serr = core::ptr::null_mut();
    struct scm_timestamping *tss = core::ptr::null_mut();
    struct cmsghdr *cm;
    for (cm = CMSG_FIRSTHDR(msg);
    cm && cm.cmsg_len;
    cm = CMSG_NXTHDR(msg, cm)) {
    if (cm.cmsg_level == SOL_SOCKET &&
    cm.cmsg_type == SCM_TIMESTAMPING) {
    tss = (void *)CMSG_DATA(cm);
    } else if ((cm.cmsg_level == SOL_IP &&
    cm.cmsg_type == IP_RECVERR) ||
    (cm.cmsg_level == SOL_IPV6 &&
    cm.cmsg_type == IPV6_RECVERR) ||
    (cm.cmsg_level == SOL_PACKET &&
    cm.cmsg_type == PACKET_TX_TIMESTAMP)) {
    serr = (void *)CMSG_DATA(cm);
    ASSERT_EQ(serr.ee_origin, SO_EE_ORIGIN_TIMESTAMPING,
    "cmsg type");
    }
    if (serr && tss)
    test_socket_timestamp(tss, serr.ee_info,
    serr.ee_data);
    }
    }
#[no_mangle]
unsafe extern "C" fn socket_recv_errmsg(fd: c_int) -> bool {
    static bool socket_recv_errmsg(int fd)
    {
    static char ctrl[1024 /* overprovision*/];
    char data[cfg_payload_len];
    static struct msghdr msg;
    struct iovec entry;
    let mut n: c_int = 0;
    memset(&msg, 0, sizeof(msg));
    memset(&entry, 0, sizeof(entry));
    memset(ctrl, 0, sizeof(ctrl));
    entry.iov_base = data;
    entry.iov_len = cfg_payload_len;
    msg.msg_iov = &entry;
    msg.msg_iovlen = 1;
    msg.msg_name = core::ptr::null_mut();
    msg.msg_namelen = 0;
    msg.msg_control = ctrl;
    msg.msg_controllen = sizeof(ctrl);
    n = recvmsg(fd, &msg, MSG_ERRQUEUE);
    if (n == -1)
    ASSERT_EQ(errno, EAGAIN, "recvmsg MSG_ERRQUEUE");
    if (n >= 0)
    test_recv_errmsg_cmsg(&msg);
    let mut n: return = = -1;
    }
#[no_mangle]
unsafe extern "C" fn test_socket_timestamping(fd: c_int) {
    static void test_socket_timestamping(int fd)
    {
    while (!socket_recv_errmsg(fd));
    ASSERT_EQ(SK_TS_SCHED, 1, "SCM_TSTAMP_SCHED");
    ASSERT_EQ(SK_TS_TXSW, 1, "SCM_TSTAMP_SND");
    ASSERT_EQ(SK_TS_ACK, 1, "SCM_TSTAMP_ACK");
    SK_TS_SCHED = 0;
    SK_TS_TXSW = 0;
    SK_TS_ACK = 0;
    }
#[no_mangle]
unsafe extern "C" fn test_tcp(family: c_int, enable_socket_timestamping: bool) {
    static void test_tcp(int family, bool enable_socket_timestamping)
    {
    struct net_timestamping__bss *bss;
    char buf[cfg_payload_len];
    let mut sfd: c_int = -1, cfd = -1;
    unsigned int sock_opt;
    struct netns_obj *ns;
    int cg_fd;
    int ret;
    cg_fd = test__join_cgroup(CG_NAME);
    if (!ASSERT_OK_FD(cg_fd, "join cgroup"))
    return;
    ns = netns_new("net_timestamping_ns", true);
    if (!ASSERT_OK_PTR(ns, "create ns"))
    goto out;
    skel = net_timestamping__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open and load skel"))
    goto out;
    if (!ASSERT_OK(net_timestamping__attach(skel), "attach skel"))
    goto out;
    skel.links.skops_sockopt =
    bpf_program__attach_cgroup(skel.progs.skops_sockopt, cg_fd);
    if (!ASSERT_OK_PTR(skel.links.skops_sockopt, "attach cgroup"))
    goto out;
    bss = skel.bss;
    memset(bss, 0, sizeof(*bss));
    skel.bss.monitored_pid = getpid();
    sfd = start_server(family, SOCK_STREAM,
    family == AF_INET6 ? addr6_str : addr4_str, 0, 0);
    if (!ASSERT_OK_FD(sfd, "start_server"))
    goto out;
    cfd = connect_to_fd(sfd, 0);
    if (!ASSERT_OK_FD(cfd, "connect_to_fd_server"))
    goto out;
    if (enable_socket_timestamping) {
    sock_opt = SOF_TIMESTAMPING_SOFTWARE |
    SOF_TIMESTAMPING_OPT_ID |
    SOF_TIMESTAMPING_TX_SCHED |
    SOF_TIMESTAMPING_TX_SOFTWARE |
    SOF_TIMESTAMPING_TX_ACK;
    ret = setsockopt(cfd, SOL_SOCKET, SO_TIMESTAMPING,
    (char *) &sock_opt, sizeof(sock_opt));
    if (!ASSERT_OK(ret, "setsockopt SO_TIMESTAMPING"))
    goto out;
    ret = clock_gettime(CLOCK_REALTIME, &usr_ts);
    if (!ASSERT_OK(ret, "get user time"))
    goto out;
    }
    ret = write(cfd, buf, sizeof(buf));
    if (!ASSERT_EQ(ret, sizeof(buf), "send to server"))
    goto out;
    if (enable_socket_timestamping)
    test_socket_timestamping(cfd);
    ASSERT_EQ(bss.nr_active, 1, "nr_active");
    ASSERT_EQ(bss.nr_snd, 2, "nr_snd");
    ASSERT_EQ(bss.nr_sched, 1, "nr_sched");
    ASSERT_EQ(bss.nr_txsw, 1, "nr_txsw");
    ASSERT_EQ(bss.nr_ack, 1, "nr_ack");
    out:
    if (sfd >= 0)
    close(sfd);
    if (cfd >= 0)
    close(cfd);
    net_timestamping__destroy(skel);
    netns_free(ns);
    close(cg_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn test_net_timestamping() {
    void test_net_timestamping(void)
    {
    if (test__start_subtest("INET4: bpf timestamping"))
    test_tcp(AF_INET, false);
    if (test__start_subtest("INET4: bpf and socket timestamping"))
    test_tcp(AF_INET, true);
    if (test__start_subtest("INET6: bpf timestamping"))
    test_tcp(AF_INET6, false);
    if (test__start_subtest("INET6: bpf and socket timestamping"))
    test_tcp(AF_INET6, true);
    }
