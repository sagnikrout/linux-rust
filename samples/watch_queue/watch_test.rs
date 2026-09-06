//! Automatically rewritten from C to Rust
//! Source: samples/watch_queue/watch_test.c
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
// Use watch_queue API to watch for notifications.
//
// Copyright (C) 2020 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//
// Macro flag: #define _GNU_SOURCE

// Work around glibc header silliness

pub const BUF_SIZE: c_int = 256;
#[no_mangle]
unsafe extern "C" fn keyctl_watch_key(key: c_int, watch_fd: c_int, watch_id: c_int) -> c_long {
    static long keyctl_watch_key(int key, int watch_fd, int watch_id)
    {
    return syscall(__NR_keyctl, KEYCTL_WATCH_KEY, key, watch_fd, watch_id);
    }
    static const char *key_subtypes[256] = {
    [NOTIFY_KEY_INSTANTIATED]	= "instantiated",
    [NOTIFY_KEY_UPDATED]		= "updated",
    [NOTIFY_KEY_LINKED]		= "linked",
    [NOTIFY_KEY_UNLINKED]		= "unlinked",
    [NOTIFY_KEY_CLEARED]		= "cleared",
    [NOTIFY_KEY_REVOKED]		= "revoked",
    [NOTIFY_KEY_INVALIDATED]	= "invalidated",
    [NOTIFY_KEY_SETATTR]		= "setattr",
    };
#[no_mangle]
unsafe extern "C" fn saw_key_change(n: *mut watch_notification, len: usize) {
    static void saw_key_change(struct watch_notification *n, size_t len)
    {
    struct key_notification *k = (struct key_notification *)n;
    if (len != sizeof(struct key_notification)) {
    fprintf(stderr, "Incorrect key message length\n");
    return;
    }
    printf("KEY %08x change=%u[%s] aux=%u\n",
    k.key_id, n.subtype, key_subtypes[n.subtype], k.aux);
    }
//
// Consume and display events.
//
#[no_mangle]
unsafe extern "C" fn consumer(fd: c_int) {
    static void consumer(int fd)
    {
    unsigned char buffer[433], *p, *end;
    union {
    struct watch_notification n;
    unsigned char buf1[128];
    } n;
    ssize_t buf_len;
    for (;;) {
    buf_len = read(fd, buffer, sizeof(buffer));
    if (buf_len == -1) {
    perror("read");
    exit(1);
    }
    if (buf_len == 0) {
    printf("-- END --\n");
    return;
    }
    if (buf_len > sizeof(buffer)) {
    fprintf(stderr, "Read buffer overrun: %zd\n", buf_len);
    return;
    }
    printf("read() = %zd\n", buf_len);
    p = buffer;
    end = buffer + buf_len;
    while (p < end) {
    size_t largest, len;
    largest = end - p;
    if (largest > 128)
    largest = 128;
    if (largest < sizeof(struct watch_notification)) {
    fprintf(stderr, "Short message header: %zu\n", largest);
    return;
    }
    memcpy(&n, p, largest);
    printf("NOTIFY[%03zx]: ty=%06x sy=%02x i=%08x\n",
    p - buffer, n.n.type, n.n.subtype, n.n.info);
    len = n.n.info & WATCH_INFO_LENGTH;
    if (len < sizeof(n.n) || len > largest) {
    fprintf(stderr, "Bad message length: %zu/%zu\n", len, largest);
    exit(1);
    }
    switch (n.n.type) {
    case WATCH_TYPE_META:
    switch (n.n.subtype) {
    case WATCH_META_REMOVAL_NOTIFICATION:
    printf("REMOVAL of watchpoint %08x\n",
    (n.n.info & WATCH_INFO_ID) >>
    WATCH_INFO_ID__SHIFT);
    break;
    case WATCH_META_LOSS_NOTIFICATION:
    printf("-- LOSS --\n");
    break;
    default:
    printf("other meta record\n");
    break;
    }
    break;
    case WATCH_TYPE_KEY_NOTIFY:
    saw_key_change(&n.n, len);
    break;
    default:
    printf("other type\n");
    break;
    }
    p += len;
    }
    }
    }
    static struct watch_notification_filter filter = {
    .nr_filters	= 1,
    .filters = {
    [0]	= {
    .type			= WATCH_TYPE_KEY_NOTIFY,
    .subtype_filter[0]	= UINT_MAX,
    },
    },
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int pipefd[2], fd;
    if (pipe2(pipefd, O_NOTIFICATION_PIPE) == -1) {
    perror("pipe2");
    exit(1);
    }
    fd = pipefd[0];
    if (ioctl(fd, IOC_WATCH_QUEUE_SET_SIZE, BUF_SIZE) == -1) {
    perror("watch_queue(size)");
    exit(1);
    }
    if (ioctl(fd, IOC_WATCH_QUEUE_SET_FILTER, &filter) == -1) {
    perror("watch_queue(filter)");
    exit(1);
    }
    if (keyctl_watch_key(KEY_SPEC_SESSION_KEYRING, fd, 0x01) == -1) {
    perror("keyctl");
    exit(1);
    }
    if (keyctl_watch_key(KEY_SPEC_USER_KEYRING, fd, 0x02) == -1) {
    perror("keyctl");
    exit(1);
    }
    consumer(fd);
    exit(0);
    }
