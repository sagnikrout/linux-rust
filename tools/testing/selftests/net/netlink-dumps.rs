//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netlink-dumps.c
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
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_ack {
    pub err: c_int,
    pub attr_offs: __u32,
    pub miss_type: __u32,
    pub miss_nest: __u32,
    pub str: *const c_char,
}

    enum get_ea_ret {
    ERROR = -1,
    NO_CTRL = 0,
    FOUND_DONE,
    FOUND_ERR,
    FOUND_EXTACK,
    };
    static enum get_ea_ret
    nl_get_extack(char *buf, size_t n, struct ext_ack *ea)
    {
    let mut ret: enum get_ea_ret = NO_CTRL;
    const struct nlmsghdr *nlh;
    const struct nlattr *attr;
    ssize_t rem;
    for (rem = n; rem > 0; NLMSG_NEXT(nlh, rem)) {
    nlh = (struct nlmsghdr *)&buf[n - rem];
    if (!NLMSG_OK(nlh, rem))
    return ERROR;
    if (nlh.nlmsg_type == NLMSG_ERROR)
    ret = FOUND_ERR;
#[no_mangle]
pub unsafe extern "C" fn if(NLMSG_DONE: nlh->nlmsg_type ==) -> else {
    else if (nlh.nlmsg_type == NLMSG_DONE)
    ret = FOUND_DONE;
    else
    continue;
    ea.err = -*(int *)NLMSG_DATA(nlh);
    if (!(nlh.nlmsg_flags & NLM_F_ACK_TLVS))
    return ret;
    ynl_attr_for_each(attr, nlh, sizeof(int)) {
    switch (ynl_attr_type(attr)) {
    case NLMSGERR_ATTR_OFFS:
    ea.attr_offs = ynl_attr_get_u32(attr);
    break;
    case NLMSGERR_ATTR_MISS_TYPE:
    ea.miss_type = ynl_attr_get_u32(attr);
    break;
    case NLMSGERR_ATTR_MISS_NEST:
    ea.miss_nest = ynl_attr_get_u32(attr);
    break;
    case NLMSGERR_ATTR_MSG:
    ea.str = ynl_attr_get_str(attr);
    break;
    }
    }
    return FOUND_EXTACK;
    }
    return ret;
    }
    static const struct {
    struct nlmsghdr nlhdr;
    struct ndmsg ndm;
    struct nlattr ahdr;
    __u32 val;
    } dump_neigh_bad = {
    .nlhdr = {
    .nlmsg_len	= sizeof(dump_neigh_bad),
    .nlmsg_type	= RTM_GETNEIGH,
    .nlmsg_flags	= NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP,
    .nlmsg_seq	= 1,
    },
    .ndm = {
    .ndm_family	= 123,
    },
    .ahdr = {
    .nla_len	= 4 + 4,
    .nla_type	= NDA_FLAGS_EXT,
    },
    .val = -1, // should fail MASK validation
    };
    TEST(dump_extack)
    {
    int netlink_sock;
    int i, cnt, ret;
    char buf[8192];
    let mut one: c_int = 1;
    ssize_t n;
    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_ROUTE);
    ASSERT_GE(netlink_sock, 0);
    n = setsockopt(netlink_sock, SOL_NETLINK, NETLINK_CAP_ACK,
    &one, sizeof(one));
    ASSERT_EQ(n, 0);
    n = setsockopt(netlink_sock, SOL_NETLINK, NETLINK_EXT_ACK,
    &one, sizeof(one));
    ASSERT_EQ(n, 0);
    n = setsockopt(netlink_sock, SOL_NETLINK, NETLINK_GET_STRICT_CHK,
    &one, sizeof(one));
    ASSERT_EQ(n, 0);
// Dump so many times we fill up the buffer
    cnt = 80;
    for (i = 0; i < cnt; i++) {
    n = send(netlink_sock, &dump_neigh_bad,
    sizeof(dump_neigh_bad), 0);
    ASSERT_EQ(n, sizeof(dump_neigh_bad));
    }
// Read out the ENOBUFS
    n = recv(netlink_sock, buf, sizeof(buf), MSG_DONTWAIT);
    EXPECT_EQ(n, -1);
    EXPECT_EQ(errno, ENOBUFS);
    ret = NO_CTRL;
    for (i = 0; i < cnt; i++) {
    let mut ea: ext_ack = {};
    n = recv(netlink_sock, buf, sizeof(buf), MSG_DONTWAIT);
    if (n < 0) {
    ASSERT_GE(i, 10);
    break;
    }
    ASSERT_GE(n, (ssize_t)sizeof(struct nlmsghdr));
    ret = nl_get_extack(buf, n, &ea);
// Once we fill the buffer we'll see one ENOBUFS followed
// by a number of EBUSYs. Then the last recv() will finally
// trigger and complete the dump.
//
    if (ret == FOUND_ERR && (ea.err == ENOBUFS || ea.err == EBUSY))
    continue;
    EXPECT_EQ(ret, FOUND_EXTACK);
    EXPECT_EQ(ea.err, EINVAL);
    EXPECT_EQ(ea.attr_offs,
    sizeof(struct nlmsghdr) + sizeof(struct ndmsg));
    }
// Make sure last message was a full DONE+extack
    EXPECT_EQ(ret, FOUND_EXTACK);
    }
    static const struct {
    struct nlmsghdr nlhdr;
    struct genlmsghdr genlhdr;
    struct nlattr ahdr;
    __u16 val;
    __u16 pad;
    } dump_policies = {
    .nlhdr = {
    .nlmsg_len	= sizeof(dump_policies),
    .nlmsg_type	= GENL_ID_CTRL,
    .nlmsg_flags	= NLM_F_REQUEST | NLM_F_ACK | NLM_F_DUMP,
    .nlmsg_seq	= 1,
    },
    .genlhdr = {
    .cmd		= CTRL_CMD_GETPOLICY,
    .version	= 2,
    },
    .ahdr = {
    .nla_len	= 6,
    .nla_type	= CTRL_ATTR_FAMILY_ID,
    },
    .val = GENL_ID_CTRL,
    .pad = 0,
    };
// Sanity check for the test itself, make sure the dump doesn't fit in one msg
    TEST(test_sanity)
    {
    int netlink_sock;
    char buf[8192];
    ssize_t n;
    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE(netlink_sock, 0);
    n = send(netlink_sock, &dump_policies, sizeof(dump_policies), 0);
    ASSERT_EQ(n, sizeof(dump_policies));
    n = recv(netlink_sock, buf, sizeof(buf), MSG_DONTWAIT);
    ASSERT_GE(n, (ssize_t)sizeof(struct nlmsghdr));
    n = recv(netlink_sock, buf, sizeof(buf), MSG_DONTWAIT);
    ASSERT_GE(n, (ssize_t)sizeof(struct nlmsghdr));
    close(netlink_sock);
    }
    TEST(close_in_progress)
    {
    int netlink_sock;
    ssize_t n;
    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE(netlink_sock, 0);
    n = send(netlink_sock, &dump_policies, sizeof(dump_policies), 0);
    ASSERT_EQ(n, sizeof(dump_policies));
    close(netlink_sock);
    }
    TEST(close_with_ref)
    {
    char cookie[NOTIFY_COOKIE_LEN] = {};
    int netlink_sock, mq_fd;
    struct sigevent sigev;
    ssize_t n;
    netlink_sock = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    ASSERT_GE(netlink_sock, 0);
    n = send(netlink_sock, &dump_policies, sizeof(dump_policies), 0);
    ASSERT_EQ(n, sizeof(dump_policies));
    mq_fd = syscall(__NR_mq_open, "sed", O_CREAT | O_WRONLY, 0600, 0);
    ASSERT_GE(mq_fd, 0);
    memset(&sigev, 0, sizeof(sigev));
    sigev.sigev_notify		= SIGEV_THREAD;
    sigev.sigev_value.sival_ptr	= cookie;
    sigev.sigev_signo		= netlink_sock;
    syscall(__NR_mq_notify, mq_fd, &sigev);
    close(netlink_sock);
// give mqueue time to fire
    usleep(100 * 1000);
    }
    TEST_HARNESS_MAIN
