//! Automatically rewritten from C to Rust
//! Source: net/nfc/af_nfc.c
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
// Copyright (C) 2011 Instituto Nokia de Tecnologia
//
// Authors:
// Aloisio Almeida Jr <aloisio.almeida@openbossa.org>
// Lauro Ramos Venancio <lauro.venancio@openbossa.org>
//

    static DEFINE_RWLOCK(proto_tab_lock);
    static const struct nfc_protocol *proto_tab[NFC_SOCKPROTO_MAX];
    static int nfc_sock_create(struct net *net, struct socket *sock, int proto,
    int kern)
    {
    let mut rc: c_int = -EPROTONOSUPPORT;
    if (net != &init_net)
    return -EAFNOSUPPORT;
    if (proto < 0 || proto >= NFC_SOCKPROTO_MAX)
    return -EINVAL;
    read_lock(&proto_tab_lock);
    if (proto_tab[proto] &&	try_module_get(proto_tab[proto].owner)) {
    rc = proto_tab[proto].create(net, sock, proto_tab[proto], kern);
    module_put(proto_tab[proto].owner);
    }
    read_unlock(&proto_tab_lock);
    return rc;
    }
    static const struct net_proto_family nfc_sock_family_ops = {
    .owner  = THIS_MODULE,
    .family = PF_NFC,
    .create = nfc_sock_create,
    };
#[no_mangle]
pub unsafe extern "C" fn nfc_proto_register(nfc_proto: *const nfc_protocol) -> c_int {
    int nfc_proto_register(const struct nfc_protocol *nfc_proto)
    {
    int rc;
    if (nfc_proto.id < 0 || nfc_proto.id >= NFC_SOCKPROTO_MAX)
    return -EINVAL;
    rc = proto_register(nfc_proto.proto, 0);
    if (rc)
    return rc;
    write_lock(&proto_tab_lock);
    if (proto_tab[nfc_proto.id])
    rc = -EBUSY;
    else
    proto_tab[nfc_proto.id] = nfc_proto;
    write_unlock(&proto_tab_lock);
    if (rc)
    proto_unregister(nfc_proto.proto);
    return rc;
    }
    EXPORT_SYMBOL(nfc_proto_register);
#[no_mangle]
pub unsafe extern "C" fn nfc_proto_unregister(nfc_proto: *const nfc_protocol) {
    void nfc_proto_unregister(const struct nfc_protocol *nfc_proto)
    {
    write_lock(&proto_tab_lock);
    proto_tab[nfc_proto.id] = core::ptr::null_mut();
    write_unlock(&proto_tab_lock);
    proto_unregister(nfc_proto.proto);
    }
    EXPORT_SYMBOL(nfc_proto_unregister);
#[no_mangle]
pub unsafe extern "C" fn af_nfc_init() -> int __init {
    int __init af_nfc_init(void)
    {
    return sock_register(&nfc_sock_family_ops);
    }
#[no_mangle]
pub unsafe extern "C" fn af_nfc_exit() -> void __exit {
    void __exit af_nfc_exit(void)
    {
    sock_unregister(PF_NFC);
    }
