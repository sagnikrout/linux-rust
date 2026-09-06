//! Automatically rewritten from C to Rust
//! Source: net/handshake/tlshd.c
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
// Establish a TLS session for a kernel socket consumer
// using the tlshd user space handler.
//
// Author: Chuck Lever <chuck.lever@oracle.com>
//
// Copyright (c) 2021-2023, Oracle and/or its affiliates.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tls_handshake_req {
    void			(*th_consumer_done)(void *data, int status,
    pub peerid): key_serial_t,
    pub th_consumer_data: *mut c_void,
    pub th_type: c_int,
    pub th_timeout_ms: c_uint,
    pub th_auth_mode: c_int,
    pub th_peername: *const c_char,
    pub th_keyring: key_serial_t,
    pub th_certificate: key_serial_t,
    pub th_privkey: key_serial_t,
    pub th_num_peerids: c_uint,
    pub th_peerid: [key_serial_t; 5],
}

    static struct tls_handshake_req *
    tls_handshake_req_init(struct handshake_req *req,
    const struct tls_handshake_args *args)
    {
    struct tls_handshake_req *treq = handshake_req_private(req);
    treq.th_timeout_ms = args.ta_timeout_ms;
    treq.th_consumer_done = args.ta_done;
    treq.th_consumer_data = args.ta_data;
    treq.th_peername = args.ta_peername;
    treq.th_keyring = args.ta_keyring;
    treq.th_num_peerids = 0;
    treq.th_certificate = TLS_NO_CERT;
    treq.th_privkey = TLS_NO_PRIVKEY;
    return treq;
    }
    static void tls_handshake_remote_peerids(struct tls_handshake_req *treq,
    struct genl_info *info)
    {
    struct nlattr *head = nlmsg_attrdata(info.nlhdr, GENL_HDRLEN);
    int rem, len = nlmsg_attrlen(info.nlhdr, GENL_HDRLEN);
    struct nlattr *nla;
    unsigned int i;
    i = 0;
    nla_for_each_attr(nla, head, len, rem) {
    if (nla_type(nla) == HANDSHAKE_A_DONE_REMOTE_AUTH)
    i++;
    }
    if (!i)
    return;
    treq.th_num_peerids = min_t(unsigned int, i,
    ARRAY_SIZE(treq.th_peerid));
    i = 0;
    nla_for_each_attr(nla, head, len, rem) {
    if (nla_type(nla) == HANDSHAKE_A_DONE_REMOTE_AUTH)
    treq.th_peerid[i++] = nla_get_u32(nla);
    if (i >= treq.th_num_peerids)
    break;
    }
    }
//
// tls_handshake_done - callback to handle a CMD_DONE request
// @req: socket on which the handshake was performed
// @status: session status code
// @info: full results of session establishment
//
    static void tls_handshake_done(struct handshake_req *req,
    int status, struct genl_info *info)
    {
    struct tls_handshake_req *treq = handshake_req_private(req);
    treq.th_peerid[0] = TLS_NO_PEERID;
    if (info)
    tls_handshake_remote_peerids(treq, info);
    if (!status)
    set_bit(HANDSHAKE_F_REQ_SESSION, &req.hr_flags);
    treq.th_consumer_done(treq.th_consumer_data, status,
    treq.th_peerid[0]);
    }

#[no_mangle]
unsafe extern "C" fn tls_handshake_private_keyring(treq: *mut tls_handshake_req) -> c_int {
    static int tls_handshake_private_keyring(struct tls_handshake_req *treq)
    {
    key_ref_t process_keyring_ref, keyring_ref;
    int ret;
    if (treq.th_keyring == TLS_NO_KEYRING)
    return 0;
    process_keyring_ref = lookup_user_key(KEY_SPEC_PROCESS_KEYRING,
    KEY_LOOKUP_CREATE,
    KEY_NEED_WRITE);
    if (IS_ERR(process_keyring_ref)) {
    ret = PTR_ERR(process_keyring_ref);
    goto out;
    }
    keyring_ref = lookup_user_key(treq.th_keyring, KEY_LOOKUP_CREATE,
    KEY_NEED_LINK);
    if (IS_ERR(keyring_ref)) {
    ret = PTR_ERR(keyring_ref);
    goto out_put_key;
    }
    ret = key_link(key_ref_to_ptr(process_keyring_ref),
    key_ref_to_ptr(keyring_ref));
    key_ref_put(keyring_ref);
    out_put_key:
    key_ref_put(process_keyring_ref);
    out:
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn tls_handshake_private_keyring(treq: *mut tls_handshake_req) -> c_int {
    static int tls_handshake_private_keyring(struct tls_handshake_req *treq)
    {
    return 0;
    }

    static int tls_handshake_put_peer_identity(struct sk_buff *msg,
    struct tls_handshake_req *treq)
    {
    unsigned int i;
    for (i = 0; i < treq.th_num_peerids; i++)
    if (nla_put_u32(msg, HANDSHAKE_A_ACCEPT_PEER_IDENTITY,
    treq.th_peerid[i]) < 0)
    return -EMSGSIZE;
    return 0;
    }
    static int tls_handshake_put_certificate(struct sk_buff *msg,
    struct tls_handshake_req *treq)
    {
    struct nlattr *entry_attr;
    if (treq.th_certificate == TLS_NO_CERT &&
    treq.th_privkey == TLS_NO_PRIVKEY)
    return 0;
    entry_attr = nla_nest_start(msg, HANDSHAKE_A_ACCEPT_CERTIFICATE);
    if (!entry_attr)
    return -EMSGSIZE;
    if (nla_put_s32(msg, HANDSHAKE_A_X509_CERT,
    treq.th_certificate) ||
    nla_put_s32(msg, HANDSHAKE_A_X509_PRIVKEY,
    treq.th_privkey)) {
    nla_nest_cancel(msg, entry_attr);
    return -EMSGSIZE;
    }
    nla_nest_end(msg, entry_attr);
    return 0;
    }
//
// tls_handshake_accept - callback to construct a CMD_ACCEPT response
// @req: handshake parameters to return
// @info: generic netlink message context
// @fd: file descriptor to be returned
//
// Returns zero on success, or a negative errno on failure.
//
    static int tls_handshake_accept(struct handshake_req *req,
    struct genl_info *info, int fd)
    {
    struct tls_handshake_req *treq = handshake_req_private(req);
    struct nlmsghdr *hdr;
    struct sk_buff *msg;
    int ret;
    ret = tls_handshake_private_keyring(treq);
    if (ret < 0)
    goto out;
    ret = -ENOMEM;
    msg = genlmsg_new(GENLMSG_DEFAULT_SIZE, GFP_KERNEL);
    if (!msg)
    goto out;
    hdr = handshake_genl_put(msg, info);
    if (!hdr)
    goto out_cancel;
    ret = nla_put_s32(msg, HANDSHAKE_A_ACCEPT_SOCKFD, fd);
    if (ret < 0)
    goto out_cancel;
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_MESSAGE_TYPE, treq.th_type);
    if (ret < 0)
    goto out_cancel;
    if (treq.th_peername) {
    ret = nla_put_string(msg, HANDSHAKE_A_ACCEPT_PEERNAME,
    treq.th_peername);
    if (ret < 0)
    goto out_cancel;
    }
    if (treq.th_timeout_ms) {
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_TIMEOUT, treq.th_timeout_ms);
    if (ret < 0)
    goto out_cancel;
    }
    if (treq.th_keyring) {
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_KEYRING,
    treq.th_keyring);
    if (ret < 0)
    goto out_cancel;
    }
    ret = nla_put_u32(msg, HANDSHAKE_A_ACCEPT_AUTH_MODE,
    treq.th_auth_mode);
    if (ret < 0)
    goto out_cancel;
    switch (treq.th_auth_mode) {
    case HANDSHAKE_AUTH_PSK:
    ret = tls_handshake_put_peer_identity(msg, treq);
    if (ret < 0)
    goto out_cancel;
    break;
    case HANDSHAKE_AUTH_X509:
    ret = tls_handshake_put_certificate(msg, treq);
    if (ret < 0)
    goto out_cancel;
    break;
    }
    genlmsg_end(msg, hdr);
    return genlmsg_reply(msg, info);
    out_cancel:
    genlmsg_cancel(msg, hdr);
    nlmsg_free(msg);
    out:
    return ret;
    }
    static const struct handshake_proto tls_handshake_proto = {
    .hp_handler_class	= HANDSHAKE_HANDLER_CLASS_TLSHD,
    .hp_privsize		= sizeof(struct tls_handshake_req),
    .hp_flags		= BIT(HANDSHAKE_F_PROTO_NOTIFY),
    .hp_accept		= tls_handshake_accept,
    .hp_done		= tls_handshake_done,
    };
//
// tls_client_hello_anon - request an anonymous TLS handshake on a socket
// @args: socket and handshake parameters for this request
// @flags: memory allocation control flags
//
// Return values:
// %0: Handshake request enqueue; ->done will be called when complete
// %-ESRCH: No user agent is available
// %-ENOMEM: Memory allocation failed
//
#[no_mangle]
pub unsafe extern "C" fn tls_client_hello_anon(args: *const tls_handshake_args, flags: gfp_t) -> c_int {
    int tls_client_hello_anon(const struct tls_handshake_args *args, gfp_t flags)
    {
    struct tls_handshake_req *treq;
    struct handshake_req *req;
    req = handshake_req_alloc(&tls_handshake_proto, flags);
    if (!req)
    return -ENOMEM;
    treq = tls_handshake_req_init(req, args);
    treq.th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO;
    treq.th_auth_mode = HANDSHAKE_AUTH_UNAUTH;
    return handshake_req_submit(args.ta_sock, req, flags);
    }
    EXPORT_SYMBOL(tls_client_hello_anon);
//
// tls_client_hello_x509 - request an x.509-based TLS handshake on a socket
// @args: socket and handshake parameters for this request
// @flags: memory allocation control flags
//
// Return values:
// %0: Handshake request enqueue; ->done will be called when complete
// %-ESRCH: No user agent is available
// %-ENOMEM: Memory allocation failed
//
#[no_mangle]
pub unsafe extern "C" fn tls_client_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> c_int {
    int tls_client_hello_x509(const struct tls_handshake_args *args, gfp_t flags)
    {
    struct tls_handshake_req *treq;
    struct handshake_req *req;
    req = handshake_req_alloc(&tls_handshake_proto, flags);
    if (!req)
    return -ENOMEM;
    treq = tls_handshake_req_init(req, args);
    treq.th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO;
    treq.th_auth_mode = HANDSHAKE_AUTH_X509;
    treq.th_certificate = args.ta_my_cert;
    treq.th_privkey = args.ta_my_privkey;
    return handshake_req_submit(args.ta_sock, req, flags);
    }
    EXPORT_SYMBOL(tls_client_hello_x509);
//
// tls_client_hello_psk - request a PSK-based TLS handshake on a socket
// @args: socket and handshake parameters for this request
// @flags: memory allocation control flags
//
// Return values:
// %0: Handshake request enqueue; ->done will be called when complete
// %-EINVAL: Wrong number of local peer IDs
// %-ESRCH: No user agent is available
// %-ENOMEM: Memory allocation failed
//
#[no_mangle]
pub unsafe extern "C" fn tls_client_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> c_int {
    int tls_client_hello_psk(const struct tls_handshake_args *args, gfp_t flags)
    {
    struct tls_handshake_req *treq;
    struct handshake_req *req;
    unsigned int i;
    if (!args.ta_num_peerids ||
    args.ta_num_peerids > ARRAY_SIZE(treq.th_peerid))
    return -EINVAL;
    req = handshake_req_alloc(&tls_handshake_proto, flags);
    if (!req)
    return -ENOMEM;
    treq = tls_handshake_req_init(req, args);
    treq.th_type = HANDSHAKE_MSG_TYPE_CLIENTHELLO;
    treq.th_auth_mode = HANDSHAKE_AUTH_PSK;
    treq.th_num_peerids = args.ta_num_peerids;
    for (i = 0; i < args.ta_num_peerids; i++)
    treq.th_peerid[i] = args.ta_my_peerids[i];
    return handshake_req_submit(args.ta_sock, req, flags);
    }
    EXPORT_SYMBOL(tls_client_hello_psk);
//
// tls_server_hello_x509 - request a server TLS handshake on a socket
// @args: socket and handshake parameters for this request
// @flags: memory allocation control flags
//
// Return values:
// %0: Handshake request enqueue; ->done will be called when complete
// %-ESRCH: No user agent is available
// %-ENOMEM: Memory allocation failed
//
#[no_mangle]
pub unsafe extern "C" fn tls_server_hello_x509(args: *const tls_handshake_args, flags: gfp_t) -> c_int {
    int tls_server_hello_x509(const struct tls_handshake_args *args, gfp_t flags)
    {
    struct tls_handshake_req *treq;
    struct handshake_req *req;
    req = handshake_req_alloc(&tls_handshake_proto, flags);
    if (!req)
    return -ENOMEM;
    treq = tls_handshake_req_init(req, args);
    treq.th_type = HANDSHAKE_MSG_TYPE_SERVERHELLO;
    treq.th_auth_mode = HANDSHAKE_AUTH_X509;
    treq.th_certificate = args.ta_my_cert;
    treq.th_privkey = args.ta_my_privkey;
    return handshake_req_submit(args.ta_sock, req, flags);
    }
    EXPORT_SYMBOL(tls_server_hello_x509);
//
// tls_server_hello_psk - request a server TLS handshake on a socket
// @args: socket and handshake parameters for this request
// @flags: memory allocation control flags
//
// Return values:
// %0: Handshake request enqueue; ->done will be called when complete
// %-ESRCH: No user agent is available
// %-ENOMEM: Memory allocation failed
//
#[no_mangle]
pub unsafe extern "C" fn tls_server_hello_psk(args: *const tls_handshake_args, flags: gfp_t) -> c_int {
    int tls_server_hello_psk(const struct tls_handshake_args *args, gfp_t flags)
    {
    struct tls_handshake_req *treq;
    struct handshake_req *req;
    req = handshake_req_alloc(&tls_handshake_proto, flags);
    if (!req)
    return -ENOMEM;
    treq = tls_handshake_req_init(req, args);
    treq.th_type = HANDSHAKE_MSG_TYPE_SERVERHELLO;
    treq.th_auth_mode = HANDSHAKE_AUTH_PSK;
    treq.th_num_peerids = 1;
    treq.th_peerid[0] = args.ta_my_peerids[0];
    return handshake_req_submit(args.ta_sock, req, flags);
    }
    EXPORT_SYMBOL(tls_server_hello_psk);
//
// tls_handshake_cancel - cancel a pending handshake
// @sk: socket on which there is an ongoing handshake
//
// Request cancellation races with request completion. To determine
// who won, callers examine the return value from this function.
//
// Context: May be called from process or softirq context.
//
// Return values:
// %true - Uncompleted handshake request was canceled
// %false - Handshake request already completed or not found
//
#[no_mangle]
pub unsafe extern "C" fn tls_handshake_cancel(sk: *mut sock) -> bool {
    bool tls_handshake_cancel(struct sock *sk)
    {
    return handshake_req_cancel(sk);
    }
    EXPORT_SYMBOL(tls_handshake_cancel);
//
// tls_handshake_close - send a Closure alert
// @sock: an open socket
//
#[no_mangle]
pub unsafe extern "C" fn tls_handshake_close(sock: *mut socket) {
    void tls_handshake_close(struct socket *sock)
    {
    struct handshake_req *req;
    req = handshake_req_hash_lookup(sock.sk);
    if (!req)
    return;
    if (!test_and_clear_bit(HANDSHAKE_F_REQ_SESSION, &req.hr_flags))
    return;
    tls_alert_send(sock, TLS_ALERT_LEVEL_WARNING,
    TLS_ALERT_DESC_CLOSE_NOTIFY);
    }
    EXPORT_SYMBOL(tls_handshake_close);
