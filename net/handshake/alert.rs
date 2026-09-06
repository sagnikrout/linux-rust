//! Automatically rewritten from C to Rust
//! Source: net/handshake/alert.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Handle the TLS Alert protocol
//
// Author: Chuck Lever <chuck.lever@oracle.com>
//
// Copyright (c) 2023, Oracle and/or its affiliates.
//

//
// tls_alert_send - send a TLS Alert on a kTLS socket
// @sock: open kTLS socket to send on
// @level: TLS Alert level
// @description: TLS Alert description
//
// Returns zero on success or a negative errno.
//
#[no_mangle]
pub unsafe extern "C" fn tls_alert_send(sock: *mut socket, level: u8, description: u8) -> c_int {
    int tls_alert_send(struct socket *sock, u8 level, u8 description)
    {
    let mut record_type: u8 = TLS_RECORD_TYPE_ALERT;
    u8 buf[CMSG_SPACE(sizeof(record_type))];
    let mut msg: msghdr = { 0 };
    struct cmsghdr *cmsg;
    struct kvec iov;
    u8 alert[2];
    int ret;
    trace_tls_alert_send(sock.sk, level, description);
    alert[0] = level;
    alert[1] = description;
    iov.iov_base = alert;
    iov.iov_len = sizeof(alert);
    memset(buf, 0, sizeof(buf));
    msg.msg_control = buf;
    msg.msg_controllen = sizeof(buf);
    msg.msg_flags = MSG_DONTWAIT;
    cmsg = CMSG_FIRSTHDR(&msg);
    cmsg.cmsg_level = SOL_TLS;
    cmsg.cmsg_type = TLS_SET_RECORD_TYPE;
    cmsg.cmsg_len = CMSG_LEN(sizeof(record_type));
    memcpy(CMSG_DATA(cmsg), &record_type, sizeof(record_type));
    iov_iter_kvec(&msg.msg_iter, ITER_SOURCE, &iov, 1, iov.iov_len);
    ret = sock_sendmsg(sock, &msg);
    return ret < 0 ? ret : 0;
    }
//
// tls_get_record_type - Look for TLS RECORD_TYPE information
// @sk: socket (for IP address information)
// @cmsg: incoming message to be parsed
//
// Returns zero or a TLS_RECORD_TYPE value.
//
#[no_mangle]
pub unsafe extern "C" fn tls_get_record_type(sk: *const sock, cmsg: *const cmsghdr) -> u8 {
    u8 tls_get_record_type(const struct sock *sk, const struct cmsghdr *cmsg)
    {
    u8 record_type;
    if (cmsg.cmsg_level != SOL_TLS)
    return 0;
    if (cmsg.cmsg_type != TLS_GET_RECORD_TYPE)
    return 0;
    record_type = *((u8 *)CMSG_DATA(cmsg));
    trace_tls_contenttype(sk, record_type);
    return record_type;
    }
    EXPORT_SYMBOL(tls_get_record_type);
//
// tls_alert_recv - Parse TLS Alert messages
// @sk: socket (for IP address information)
// @msg: incoming message to be parsed
// @level: OUT - TLS AlertLevel value
// @description: OUT - TLS AlertDescription value
//
    void tls_alert_recv(const struct sock *sk, const struct msghdr *msg,
    u8 *level, u8 *description)
    {
    const struct kvec *iov;
    u8 *data;
    iov = msg.msg_iter.kvec;
    data = iov.iov_base;
// level = data[0];
// description = data[1];
    trace_tls_alert_recv(sk, *level, *description);
    }
    EXPORT_SYMBOL(tls_alert_recv);
