//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/netfilter/audit_logread.c
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

    static int fd;
pub const MAX_AUDIT_MESSAGE_LENGTH: c_int = 8970;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audit_message {
    pub nlh: nlmsghdr,
    union {
    pub s: audit_status,
    pub data: [c_char; MAX_AUDIT_MESSAGE_LENGTH],
    pub u: },
}

#[no_mangle]
pub unsafe extern "C" fn audit_recv(fd: c_int, rep: *mut audit_message) -> c_int {
    int audit_recv(int fd, struct audit_message *rep)
    {
    struct sockaddr_nl addr;
    let mut addrlen: socklen_t = sizeof(addr);
    int ret;
    do {
    ret = recvfrom(fd, rep, sizeof(*rep), 0,
    (struct sockaddr *)&addr, &addrlen);
    } while (ret < 0 && errno == EINTR);
    if (ret < 0 ||
    addrlen != sizeof(addr) ||
    addr.nl_pid != 0 ||
    rep.nlh.nlmsg_type == NLMSG_ERROR) /* short-cut for now */
    return -1;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_send(fd: c_int, type: u16, key: u32, val: u32) -> c_int {
    int audit_send(int fd, uint16_t type, uint32_t key, uint32_t val)
    {
    let mut seq: static int = 0;
    struct audit_message msg = {
    .nlh = {
    .nlmsg_len   = NLMSG_SPACE(sizeof(msg.u.s)),
    .nlmsg_type  = type,
    .nlmsg_flags = NLM_F_REQUEST | NLM_F_ACK,
    .nlmsg_seq   = ++seq,
    },
    .u.s = {
    .mask    = key,
    .enabled = key == AUDIT_STATUS_ENABLED ? val : 0,
    .pid     = key == AUDIT_STATUS_PID ? val : 0,
    }
    };
    struct sockaddr_nl addr = {
    .nl_family = AF_NETLINK,
    };
    int ret;
    do {
    ret = sendto(fd, &msg, msg.nlh.nlmsg_len, 0,
    (struct sockaddr *)&addr, sizeof(addr));
    } while (ret < 0 && errno == EINTR);
    if (ret != (int)msg.nlh.nlmsg_len)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn audit_set(fd: c_int, key: u32, val: u32) -> c_int {
    int audit_set(int fd, uint32_t key, uint32_t val)
    {
    let mut rep: audit_message = { 0 };
    int ret;
    ret = audit_send(fd, AUDIT_SET, key, val);
    if (ret)
    return ret;
    ret = audit_recv(fd, &rep);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn readlog(fd: c_int) -> c_int {
    int readlog(int fd)
    {
    let mut rep: audit_message = { 0 };
    let mut ret: c_int = audit_recv(fd, &rep);
    const char *sep = "";
    char *k, *v;
    if (ret < 0)
    return ret;
    if (rep.nlh.nlmsg_type != AUDIT_NETFILTER_CFG)
    return 0;
// skip the initial "audit(...): " part
    strtok(rep.u.data, " ");
    while ((k = strtok(core::ptr::null_mut(), "="))) {
    v = strtok(core::ptr::null_mut(), " ");
// these vary and/or are uninteresting, ignore
    if (!strcmp(k, "pid") ||
    !strcmp(k, "comm") ||
    !strcmp(k, "subj"))
    continue;
// strip the varying sequence number
    if (!strcmp(k, "table"))
// strchrnul(v, ':') = '\0';
    printf("%s%s=%s", sep, k, v);
    sep = " ";
    }
    if (*sep) {
    printf("\n");
    fflush(stdout);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cleanup(sig: c_int) {
    void cleanup(int sig)
    {
    audit_set(fd, AUDIT_STATUS_ENABLED, 0);
    close(fd);
    if (sig)
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct sigaction act = {
    .sa_handler = cleanup,
    };
    fd = socket(PF_NETLINK, SOCK_RAW, NETLINK_AUDIT);
    if (fd < 0) {
    perror("Can't open netlink socket");
    return -1;
    }
    if (sigaction(SIGTERM, &act, core::ptr::null_mut()) < 0 ||
    sigaction(SIGINT, &act, core::ptr::null_mut()) < 0) {
    perror("Can't set signal handler");
    close(fd);
    return -1;
    }
    audit_set(fd, AUDIT_STATUS_ENABLED, 1);
    audit_set(fd, AUDIT_STATUS_PID, getpid());
    while (1)
    readlog(fd);
    }
