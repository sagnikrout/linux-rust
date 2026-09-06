//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/ipv6_flowlabel_mgr.c
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
// Test IPV6_FLOWINFO_MGR
// Macro flag: #define _GNU_SOURCE

// uapi/glibc weirdness may leave this undefined

pub const IPV6_FLOWLABEL_MGR: c_int = 32;

pub const IPV6_FLOWINFO_SEND: c_int = 33;

// from net/ipv6/ip6_flowlabel.c
pub const FL_MIN_LINGER: c_int = 6;
#[no_mangle]
unsafe extern "C" fn flowlabel_get(fd: c_int, label: u32, share: u8, flags: u16) -> c_int {
    static int flowlabel_get(int fd, uint32_t label, uint8_t share, uint16_t flags)
    {
    struct in6_flowlabel_req req = {
    .flr_action = IPV6_FL_A_GET,
    .flr_label = htonl(label),
    .flr_flags = flags,
    .flr_share = share,
    };
// do not pass IPV6_ADDR_ANY or IPV6_ADDR_MAPPED
    req.flr_dst.s6_addr[0] = 0xfd;
    req.flr_dst.s6_addr[15] = 0x1;
    return setsockopt(fd, SOL_IPV6, IPV6_FLOWLABEL_MGR, &req, sizeof(req));
    }
#[no_mangle]
unsafe extern "C" fn flowlabel_put(fd: c_int, label: u32) -> c_int {
    static int flowlabel_put(int fd, uint32_t label)
    {
    struct in6_flowlabel_req req = {
    .flr_action = IPV6_FL_A_PUT,
    .flr_label = htonl(label),
    };
    return setsockopt(fd, SOL_IPV6, IPV6_FLOWLABEL_MGR, &req, sizeof(req));
    }
    static int flowlabel_renew(int fd, uint32_t label, uint8_t share,
    uint16_t linger)
    {
    struct in6_flowlabel_req req = {
    .flr_action = IPV6_FL_A_RENEW,
    .flr_label = htonl(label),
    .flr_share = share,
    .flr_linger = linger,
    };
    return setsockopt(fd, SOL_IPV6, IPV6_FLOWLABEL_MGR, &req, sizeof(req));
    }
#[no_mangle]
unsafe extern "C" fn loopback_addr() -> sockaddr_in6 {
    static struct sockaddr_in6 loopback_addr(void)
    {
    struct sockaddr_in6 addr = {
    .sin6_family	= AF_INET6,
    .sin6_addr	= IN6ADDR_LOOPBACK_INIT,
    .sin6_port	= htons(8888),
    };
    return addr;
    }
#[no_mangle]
unsafe extern "C" fn tcp_listen() -> c_int {
    static int tcp_listen(void)
    {
    let mut addr: sockaddr_in6 = loopback_addr();
    let mut one: c_int = 1;
    int fd;
    fd = socket(PF_INET6, SOCK_STREAM, 0);
    if (fd == -1)
    error(1, errno, "socket listener");
    if (setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one)))
    error(1, errno, "setsockopt SO_REUSEADDR");
    if (bind(fd, (void *)&addr, sizeof(addr)))
    error(1, errno, "bind");
    if (listen(fd, 1))
    error(1, errno, "listen");
    return fd;
    }
    static void tcp_connect(int listener, uint32_t flowlabel,
    int *client, int *accepted)
    {
    let mut addr: sockaddr_in6 = loopback_addr();
    let mut one: c_int = 1;
    int cfd, afd;
    cfd = socket(PF_INET6, SOCK_STREAM, 0);
    if (cfd == -1)
    error(1, errno, "socket client");
    if (flowlabel_get(cfd, flowlabel, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE))
    error(1, errno, "flowlabel_get");
    if (setsockopt(cfd, SOL_IPV6, IPV6_FLOWINFO_SEND, &one, sizeof(one)))
    error(1, errno, "setsockopt flowinfo_send");
    addr.sin6_flowinfo = htonl(flowlabel);
    if (connect(cfd, (void *)&addr, sizeof(addr)))
    error(1, errno, "connect");
    afd = accept(listener, core::ptr::null_mut(), core::ptr::null_mut());
    if (afd == -1)
    error(1, errno, "accept");
    if (flowlabel_put(cfd, flowlabel))
    error(1, errno, "flowlabel_put");
// client = cfd;
// accepted = afd;
    }
#[no_mangle]
unsafe extern "C" fn bringup_loopback() -> c_int {
    static int bringup_loopback(void)
    {
    struct ifreq ifr = {
    .ifr_name = "lo"
    };
    int fd;
    fd = socket(AF_LOCAL, SOCK_STREAM, 0);
    if (fd < 0)
    return -1;
    if (ioctl(fd, SIOCGIFFLAGS, &ifr) < 0)
    goto err;
    ifr.ifr_flags = ifr.ifr_flags | IFF_UP;
    if (ioctl(fd, SIOCSIFFLAGS, &ifr) < 0)
    goto err;
    close(fd);
    return 0;
    err:
    close(fd);
    return -1;
    }
    FIXTURE(flowlabel) {};
    FIXTURE_SETUP(flowlabel)
    {
    int ret;
    ret = unshare(CLONE_NEWNET);
    ASSERT_EQ(ret, 0) {
    TH_LOG("unshare(CLONE_NEWNET) failed: %s", strerror(errno));
    }
    ret = bringup_loopback();
    ASSERT_EQ(ret, 0) TH_LOG("Failed to bring up loopback interface");
    }
    FIXTURE_TEARDOWN(flowlabel)
    {
    }
    TEST_F(flowlabel, cannot_get_non_existent_label)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 9, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE(err) TH_LOG("expected get of a non-existent label to fail");
    EXPECT_EQ(ENOENT, errno) TH_LOG("expected ENOENT, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, cannot_put_non_existent_label)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_put(fd, 10);
    EXPECT_TRUE(err) TH_LOG("expected put of a non-existent label to fail");
    EXPECT_EQ(ESRCH, errno) TH_LOG("expected ESRCH, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, cannot_create_label_greater_than_20_bits)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 0x1FFFFF, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(err) TH_LOG("expected label > 20 bits to be rejected");
    EXPECT_EQ(EINVAL, errno) TH_LOG("expected EINVAL, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, can_create_and_get_and_put_labels)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err) TH_LOG("failed to create label (FL_F_CREATE)");
    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE(!err) TH_LOG("failed to get the label without FL_F_CREATE");
    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to get it again with create flag set, too");
    err = flowlabel_get(fd, 1, IPV6_FL_S_ANY,
    IPV6_FL_F_CREATE | IPV6_FL_F_EXCL);
    EXPECT_TRUE(err)
    TH_LOG("expected FL_F_EXCL to reject existing label");
    EXPECT_EQ(EEXIST, errno) TH_LOG("expected EEXIST, got %d", errno);
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE(!err) TH_LOG("failed to put first reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE(!err) TH_LOG("failed to put second reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE(!err) TH_LOG("failed to put third reference");
    err = flowlabel_put(fd, 1);
    EXPECT_TRUE(err)
    TH_LOG("expected fourth put to fail, no references left");
    EXPECT_EQ(ESRCH, errno) TH_LOG("expected ESRCH, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, exclusive_label_share)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 2, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to create a new exclusive label (FL_S_EXCL)");
    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(err) TH_LOG("expected reuse in non-exclusive mode to fail");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    err = flowlabel_get(fd, 2, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE(err) TH_LOG("expected reuse in exclusive mode to fail too");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    err = flowlabel_put(fd, 2);
    EXPECT_TRUE(!err) TH_LOG("failed to put the exclusive label");
    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(err) TH_LOG("expected reuse to fail, due to linger");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    sleep(FL_MIN_LINGER * 2 + 1);
    err = flowlabel_get(fd, 2, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err) TH_LOG("expected reuse to succeed after linger");
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, user_private_label_share)
    {
    int fd, err, wstatus;
    pid_t pid;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to create a new user-private label (FL_S_USER)");
    err = flowlabel_get(fd, 3, IPV6_FL_S_ANY, 0);
    EXPECT_TRUE(err) TH_LOG("expected get in non-exclusive mode to fail");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    err = flowlabel_get(fd, 3, IPV6_FL_S_EXCL, 0);
    EXPECT_TRUE(err) TH_LOG("expected get in exclusive mode to fail");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
    EXPECT_TRUE(!err) TH_LOG("failed to get it again in user mode");
    pid = fork();
    ASSERT_NE(-1, pid) TH_LOG("fork failed");
    if (!pid) {
    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
    EXPECT_TRUE(!err)
    TH_LOG("child failed to get the user-private label");
    if (setuid(USHRT_MAX))
    exit(KSFT_SKIP);
    err = flowlabel_get(fd, 3, IPV6_FL_S_USER, 0);
    EXPECT_TRUE(err)
    TH_LOG("child unexpectedly got label after setuid");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    exit(0);
    }
    ASSERT_EQ(pid, wait(&wstatus)) TH_LOG("wait failed");
    ASSERT_TRUE(WIFEXITED(wstatus)) TH_LOG("child did not exit normally");
    if (WEXITSTATUS(wstatus) == KSFT_SKIP)
    SKIP(return,
    "setuid(USHRT_MAX) unavailable (no CAP_SETUID or uid unmapped)");
    EXPECT_EQ(0, WEXITSTATUS(wstatus))
    TH_LOG("child reported unexpected result");
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, process_private_label_share)
    {
    int fd, err, wstatus;
    pid_t pid;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to create a new process-private label");
    err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, 0);
    EXPECT_TRUE(!err) TH_LOG("failed to get it again");
    pid = fork();
    ASSERT_NE(-1, pid) TH_LOG("fork failed");
    if (!pid) {
    err = flowlabel_get(fd, 4, IPV6_FL_S_PROCESS, 0);
    EXPECT_TRUE(err)
    TH_LOG("child unexpectedly got process-private label");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    exit(0);
    }
    ASSERT_EQ(pid, wait(&wstatus)) TH_LOG("wait failed");
    ASSERT_TRUE(WIFEXITED(wstatus)) TH_LOG("child did not exit normally");
    EXPECT_EQ(0, WEXITSTATUS(wstatus))
    TH_LOG("child reported unexpected result");
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, cannot_renew_non_existent_label)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_renew(fd, 5, IPV6_FL_S_EXCL,
    2 * (FL_MIN_LINGER * 2 + 1));
    EXPECT_TRUE(err)
    TH_LOG("expected renew of a non-existent label to fail");
    EXPECT_EQ(ESRCH, errno) TH_LOG("expected ESRCH, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, can_renew_existing_label)
    {
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 5, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to create a new label for renew validation");
    err = flowlabel_renew(fd, 5, IPV6_FL_S_EXCL,
    2 * (FL_MIN_LINGER * 2 + 1));
    EXPECT_TRUE(!err) TH_LOG("failed to renew an existing valid label");
    err = flowlabel_put(fd, 5);
    EXPECT_TRUE(!err) TH_LOG("failed to put the label");
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, renew_label_linger)
    {
// RENEW must extend a label's linger period: putting a renewed
// label and waiting out its original linger time must not be
// enough to allow the label to be recreated.
//
    int fd, err;
    fd = socket(PF_INET6, SOCK_DGRAM, 0);
    ASSERT_GE(fd, 0) TH_LOG("socket failed");
    err = flowlabel_get(fd, 6, IPV6_FL_S_EXCL, IPV6_FL_F_CREATE);
    EXPECT_TRUE(!err)
    TH_LOG("failed to create label with FL_MIN_LINGER linger time");
    err = flowlabel_renew(fd, 6, IPV6_FL_S_EXCL,
    2 * (FL_MIN_LINGER * 2 + 1));
    EXPECT_TRUE(!err)
    TH_LOG("failed to renew the label to increase its linger time");
    err = flowlabel_put(fd, 6);
    EXPECT_TRUE(!err) TH_LOG("failed to put the label");
    sleep(FL_MIN_LINGER * 2 + 1);
    err = flowlabel_get(fd, 6, IPV6_FL_S_ANY, IPV6_FL_F_CREATE);
    EXPECT_TRUE(err)
    TH_LOG("expected reuse to fail, new linger time not over yet");
    EXPECT_EQ(EPERM, errno) TH_LOG("expected EPERM, got %d", errno);
    EXPECT_EQ(0, close(fd));
    }
    TEST_F(flowlabel, remote_flag)
    {
// The REMOTE flag, used for getsockopt, is expected to retrieve the
// label from the latest received header.
//
    struct in6_flowlabel_req freq = {
    .flr_action = IPV6_FL_A_GET,
    .flr_flags = IPV6_FL_F_REMOTE,
    };
    let mut freq_len: socklen_t = sizeof(freq);
    int listener, cfd, afd, err;
    listener = tcp_listen();
    tcp_connect(listener, 7, &cfd, &afd);
    err = getsockopt(afd, SOL_IPV6, IPV6_FLOWLABEL_MGR, &freq, &freq_len);
    EXPECT_TRUE(!err) TH_LOG("getsockopt with IPV6_FL_F_REMOTE failed");
    EXPECT_EQ(7, ntohl(freq.flr_label))
    TH_LOG("unexpected remote flow label");
    EXPECT_EQ(0, close(afd));
    EXPECT_EQ(0, close(cfd));
    EXPECT_EQ(0, close(listener));
    }
#[no_mangle]
unsafe extern "C" fn disable_flowlabel_consistency() -> bool {
    static bool disable_flowlabel_consistency(void)
    {
    int fd;
    fd = open("/proc/sys/net/ipv6/flowlabel_consistency", O_WRONLY);
    if (fd == -1)
    return false;
    if (write(fd, "0", 1) != 1) {
    close(fd);
    return false;
    }
    close(fd);
    return true;
    }
    TEST_F(flowlabel, reflect_flag)
    {
// The REFLECT flag acts as a trigger to the REPFLOW bit. When REPFLOW
// is triggered for a socket, it adopts the label received from the
// connected socket.
//
    struct in6_flowlabel_req reflect_on = {
    .flr_action = IPV6_FL_A_GET,
    .flr_flags = IPV6_FL_F_REFLECT,
    };
    struct in6_flowlabel_req reflect_query = {
    .flr_action = IPV6_FL_A_GET,
    };
    struct in6_flowlabel_req reflect_off = {
    .flr_action = IPV6_FL_A_PUT,
    .flr_flags = IPV6_FL_F_REFLECT,
    };
    let mut reflect_query_len: socklen_t = sizeof(reflect_query);
    int listener, cfd, afd, err;
    if (!disable_flowlabel_consistency())
    SKIP(return,
    "cannot disable net.ipv6.flowlabel_consistency");
    listener = tcp_listen();
    err = setsockopt(listener, SOL_IPV6, IPV6_FLOWLABEL_MGR,
    &reflect_on, sizeof(reflect_on));
    EXPECT_TRUE(!err) TH_LOG("failed to enable REFLECT on the listener");
    tcp_connect(listener, 8, &cfd, &afd);
    err = getsockopt(afd, SOL_IPV6, IPV6_FLOWLABEL_MGR,
    &reflect_query, &reflect_query_len);
    EXPECT_TRUE(!err)
    TH_LOG("failed to query the accepted socket's outgoing label");
    EXPECT_EQ(8, ntohl(reflect_query.flr_label))
    TH_LOG("accepted socket did not reflect client's label");
    err = setsockopt(afd, SOL_IPV6, IPV6_FLOWLABEL_MGR,
    &reflect_off, sizeof(reflect_off));
    EXPECT_TRUE(!err)
    TH_LOG("failed to disable REFLECT on the accepted socket");
    err = setsockopt(afd, SOL_IPV6, IPV6_FLOWLABEL_MGR,
    &reflect_off, sizeof(reflect_off));
    EXPECT_TRUE(err) TH_LOG("expected disabling REFLECT twice to fail");
    EXPECT_EQ(ESRCH, errno) TH_LOG("expected ESRCH, got %d", errno);
    EXPECT_EQ(0, close(afd));
    EXPECT_EQ(0, close(cfd));
    EXPECT_EQ(0, close(listener));
    }
    TEST_HARNESS_MAIN
