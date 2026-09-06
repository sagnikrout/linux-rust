//! Automatically rewritten from C to Rust
//! Source: samples/connector/ucon.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ucon.c
//
// Copyright (c) 2004+ Evgeniy Polyakov <zbr@ioremap.net>
//

// Macro flag: #define DEBUG
pub const NETLINK_CONNECTOR: c_int = 11;
// Hopefully your userspace connector.h matches this kernel

pub const CN_TEST_VAL: c_uint = 0x456;

    static int need_exit;
    static __u32 seq;
#[no_mangle]
unsafe extern "C" fn netlink_send(s: c_int, msg: *mut cn_msg) -> c_int {
    static int netlink_send(int s, struct cn_msg *msg)
    {
    struct nlmsghdr *nlh;
    unsigned int size;
    int err;
    char buf[128];
    struct cn_msg *m;
    size = NLMSG_SPACE(sizeof(struct cn_msg) + msg.len);
    nlh = (struct nlmsghdr *)buf;
    nlh.nlmsg_seq = seq++;
    nlh.nlmsg_pid = getpid();
    nlh.nlmsg_type = NLMSG_DONE;
    nlh.nlmsg_len = size;
    nlh.nlmsg_flags = 0;
    m = NLMSG_DATA(nlh);

    ulog("%s: [%08x.%08x] len=%u, seq=%u, ack=%u.\n",
    __func__, msg.id.idx, msg.id.val, msg.len, msg.seq, msg.ack);

    memcpy(m, msg, sizeof(*m) + msg.len);
    err = send(s, nlh, size, 0);
    if (err == -1)
    ulog("Failed to send: %s [%d].\n",
    strerror(errno), errno);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf(
    "Usage: ucon [options] [output file]\n"
    "\n"
    "\t-h\tthis help screen\n"
    "\t-s\tsend buffers to the test module\n"
    "\n"
    "The default behavior of ucon is to subscribe to the test module\n"
    "and wait for state messages.  Any ones received are dumped to the\n"
    "specified output file (or stdout).  The test module is assumed to\n"
    "have an id of {%u.%u}\n"
    "\n"
    "If you get no output, then verify the cn_test module id matches\n"
    "the expected id above.\n"
    , CN_TEST_IDX, CN_TEST_VAL
    );
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int s;
    char buf[1024];
    int len;
    struct nlmsghdr *reply;
    struct sockaddr_nl l_local;
    struct cn_msg *data;
    FILE *out;
    time_t tm;
    struct pollfd pfd;
    let mut send_msgs: bool = false;
    while ((s = getopt(argc, argv, "hs")) != -1) {
    switch (s) {
    case 's':
    send_msgs = true;
    break;
    case 'h':
    usage();
    return 0;
    default:
// getopt() outputs an error for us
    usage();
    return 1;
    }
    }
    if (argc != optind) {
    out = fopen(argv[optind], "a+");
    if (!out) {
    ulog("Unable to open %s for writing: %s\n",
    argv[1], strerror(errno));
    out = stdout;
    }
    } else
    out = stdout;
    memset(buf, 0, sizeof(buf));
    s = socket(PF_NETLINK, SOCK_DGRAM, NETLINK_CONNECTOR);
    if (s == -1) {
    perror("socket");
    return -1;
    }
    l_local.nl_family = AF_NETLINK;
    l_local.nl_groups = -1; /* bitmask of requested groups */
    l_local.nl_pid = 0;
    ulog("subscribing to %u.%u\n", CN_TEST_IDX, CN_TEST_VAL);
    if (bind(s, (struct sockaddr *)&l_local, sizeof(struct sockaddr_nl)) == -1) {
    perror("bind");
    close(s);
    return -1;
    }

    {
    int on = 0x57; /* Additional group number */
    setsockopt(s, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP, &on, sizeof(on));
    }

    if (send_msgs) {
    int i, j;
    memset(buf, 0, sizeof(buf));
    data = (struct cn_msg *)buf;
    data.id.idx = CN_TEST_IDX;
    data.id.val = CN_TEST_VAL;
    data.seq = seq++;
    data.ack = 0;
    data.len = 0;
    for (j=0; j<10; ++j) {
    for (i=0; i<1000; ++i) {
    len = netlink_send(s, data);
    }
    ulog("%d messages have been sent to %08x.%08x.\n", i, data.id.idx, data.id.val);
    }
    return 0;
    }
    pfd.fd = s;
    while (!need_exit) {
    pfd.events = POLLIN;
    pfd.revents = 0;
    switch (poll(&pfd, 1, -1)) {
    case 0:
    need_exit = 1;
    break;
    case -1:
    if (errno != EINTR) {
    need_exit = 1;
    break;
    }
    continue;
    }
    if (need_exit)
    break;
    memset(buf, 0, sizeof(buf));
    len = recv(s, buf, sizeof(buf), 0);
    if (len == -1) {
    perror("recv buf");
    close(s);
    return -1;
    }
    reply = (struct nlmsghdr *)buf;
    switch (reply.nlmsg_type) {
    case NLMSG_ERROR:
    fprintf(out, "Error message received.\n");
    fflush(out);
    break;
    case NLMSG_DONE:
    data = (struct cn_msg *)NLMSG_DATA(reply);
    time(&tm);
    fprintf(out, "%.24s : [%x.%x] [%08u.%08u].\n",
    ctime(&tm), data.id.idx, data.id.val, data.seq, data.ack);
    fflush(out);
    break;
    default:
    break;
    }
    }
    close(s);
    return 0;
    }
