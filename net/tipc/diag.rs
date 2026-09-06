//! Automatically rewritten from C to Rust
//! Source: net/tipc/diag.c
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


//
// net/tipc/diag.c: TIPC socket diag
//
// Copyright (c) 2018, Ericsson AB
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "ASIS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

#[no_mangle]
unsafe extern "C" fn __tipc_diag_gen_cookie(sk: *mut sock) -> u64 {
    static u64 __tipc_diag_gen_cookie(struct sock *sk)
    {
    u32 res[2];
    sock_diag_save_cookie(sk, res);
    return *((u64 *)res);
    }
    static int __tipc_add_sock_diag(struct sk_buff *skb,
    struct netlink_callback *cb,
    struct tipc_sock *tsk)
    {
    struct tipc_sock_diag_req *req = nlmsg_data(cb.nlh);
    struct nlmsghdr *nlh;
    int err;
    nlh = nlmsg_put_answer(skb, cb, SOCK_DIAG_BY_FAMILY, 0,
    NLM_F_MULTI);
    if (!nlh)
    return -EMSGSIZE;
    err = tipc_sk_fill_sock_diag(skb, cb, tsk, req.tidiag_states,
    __tipc_diag_gen_cookie);
    if (err)
    return err;
    nlmsg_end(skb, nlh);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tipc_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int tipc_diag_dump(struct sk_buff *skb, struct netlink_callback *cb)
    {
    return tipc_nl_sk_walk(skb, cb, __tipc_add_sock_diag);
    }
    static int tipc_sock_diag_handler_dump(struct sk_buff *skb,
    struct nlmsghdr *h)
    {
    let mut hdrlen: c_int = sizeof(struct tipc_sock_diag_req);
    struct net *net = sock_net(skb.sk);
    if (nlmsg_len(h) < hdrlen)
    return -EINVAL;
    if (h.nlmsg_flags & NLM_F_DUMP) {
    struct netlink_dump_control c = {
    .start = tipc_dump_start,
    .dump = tipc_diag_dump,
    .done = tipc_dump_done,
    };
    netlink_dump_start(net.diag_nlsk, skb, h, &c);
    return 0;
    }
    return -EOPNOTSUPP;
    }
    static const struct sock_diag_handler tipc_sock_diag_handler = {
    .owner = THIS_MODULE,
    .family = AF_TIPC,
    .dump = tipc_sock_diag_handler_dump,
    };
#[no_mangle]
unsafe extern "C" fn tipc_diag_init() -> int __init {
    static int __init tipc_diag_init(void)
    {
    return sock_diag_register(&tipc_sock_diag_handler);
    }
#[no_mangle]
unsafe extern "C" fn tipc_diag_exit() -> void __exit {
    static void __exit tipc_diag_exit(void)
    {
    sock_diag_unregister(&tipc_sock_diag_handler);
    }
    module_init(tipc_diag_init);
    module_exit(tipc_diag_exit);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("TIPC socket monitoring via SOCK_DIAG");
    MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, AF_TIPC);
