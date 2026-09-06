//! Automatically rewritten from C to Rust
//! Source: tools/power/x86/intel-speed-select/hfi-events.c
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
//
// Intel Speed Select -- Read HFI events for OOB
// Copyright (c) 2022 Intel Corporation.
//
// This file incorporates work covered by the following copyright and
// permission notice:
// WPA Supplicant - driver interaction with Linux nl80211/cfg80211
// Copyright (c) 2003-2008, Jouni Malinen <j@w1.fi>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//
// Alternatively, this software may be distributed under the terms of
// BSD license.
//
// Requires
// libnl-genl-3-dev
//
// For Fedora/CenOS
// dnf install libnl3-devel
// For Ubuntu
// apt install libnl-3-dev libnl-genl-3-dev
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_event_data {
    pub nl_handle: *mut nl_sock,
    pub nl_cb: *mut nl_cb,
}

    struct hfi_event_data drv;
#[no_mangle]
unsafe extern "C" fn ack_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int ack_handler(struct nl_msg *msg, void *arg)
    {
    int *err = arg;
// err = 0;
    return NL_STOP;
    }
#[no_mangle]
unsafe extern "C" fn finish_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int finish_handler(struct nl_msg *msg, void *arg)
    {
    int *ret = arg;
// ret = 0;
    return NL_SKIP;
    }
    static int error_handler(struct sockaddr_nl *nla, struct nlmsgerr *err,
    void *arg)
    {
    int *ret = arg;
// ret = err->error;
    return NL_SKIP;
    }
#[no_mangle]
unsafe extern "C" fn seq_check_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int seq_check_handler(struct nl_msg *msg, void *arg)
    {
    return NL_OK;
    }
    static int send_and_recv_msgs(struct hfi_event_data *drv,
    struct nl_msg *msg,
    int (*valid_handler)(struct nl_msg *, void *),
    void *valid_data)
    {
    struct nl_cb *cb;
    let mut err: c_int = -ENOMEM;
    cb = nl_cb_clone(drv.nl_cb);
    if (!cb)
    goto out;
    err = nl_send_auto_complete(drv.nl_handle, msg);
    if (err < 0)
    goto out;
    err = 1;
    nl_cb_err(cb, NL_CB_CUSTOM, error_handler, &err);
    nl_cb_set(cb, NL_CB_FINISH, NL_CB_CUSTOM, finish_handler, &err);
    nl_cb_set(cb, NL_CB_ACK, NL_CB_CUSTOM, ack_handler, &err);
    if (valid_handler)
    nl_cb_set(cb, NL_CB_VALID, NL_CB_CUSTOM,
    valid_handler, valid_data);
    while (err > 0)
    nl_recvmsgs(drv.nl_handle, cb);
    out:
    nl_cb_put(cb);
    nlmsg_free(msg);
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct family_data {
    pub group: *const c_char,
    pub id: c_int,
}

#[no_mangle]
unsafe extern "C" fn family_handler(msg: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int family_handler(struct nl_msg *msg, void *arg)
    {
    struct family_data *res = arg;
    struct nlattr *tb[CTRL_ATTR_MAX + 1];
    struct genlmsghdr *gnlh = nlmsg_data(nlmsg_hdr(msg));
    struct nlattr *mcgrp;
    int i;
    nla_parse(tb, CTRL_ATTR_MAX, genlmsg_attrdata(gnlh, 0),
    genlmsg_attrlen(gnlh, 0), core::ptr::null_mut());
    if (!tb[CTRL_ATTR_MCAST_GROUPS])
    return NL_SKIP;
    nla_for_each_nested(mcgrp, tb[CTRL_ATTR_MCAST_GROUPS], i) {
    struct nlattr *tb2[CTRL_ATTR_MCAST_GRP_MAX + 1];
    nla_parse(tb2, CTRL_ATTR_MCAST_GRP_MAX, nla_data(mcgrp),
    nla_len(mcgrp), core::ptr::null_mut());
    if (!tb2[CTRL_ATTR_MCAST_GRP_NAME] ||
    !tb2[CTRL_ATTR_MCAST_GRP_ID] ||
    strncmp(nla_data(tb2[CTRL_ATTR_MCAST_GRP_NAME]),
    res.group,
    nla_len(tb2[CTRL_ATTR_MCAST_GRP_NAME])) != 0)
    continue;
    res.id = nla_get_u32(tb2[CTRL_ATTR_MCAST_GRP_ID]);
    break;
    }
    return 0;
    }
    static int nl_get_multicast_id(struct hfi_event_data *drv,
    const char *family, const char *group)
    {
    struct nl_msg *msg;
    let mut ret: c_int = -1;
    let mut res: family_data = { group, -ENOENT };
    msg = nlmsg_alloc();
    if (!msg)
    return -ENOMEM;
    genlmsg_put(msg, 0, 0, genl_ctrl_resolve(drv.nl_handle, "nlctrl"),
    0, 0, CTRL_CMD_GETFAMILY, 0);
    NLA_PUT_STRING(msg, CTRL_ATTR_FAMILY_NAME, family);
    ret = send_and_recv_msgs(drv, msg, family_handler, &res);
    msg = core::ptr::null_mut();
    if (ret == 0)
    ret = res.id;
    nla_put_failure:
    nlmsg_free(msg);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cap {
    pub cpu: c_int,
    pub perf: c_int,
    pub eff: c_int,
}

#[no_mangle]
unsafe extern "C" fn process_hfi_event(perf_cap: *mut perf_cap) {
    static void process_hfi_event(struct perf_cap *perf_cap)
    {
    struct isst_id id;
    set_isst_id(&id, perf_cap.cpu);
    process_level_change(&id);
    }
#[no_mangle]
unsafe extern "C" fn handle_event(n: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int handle_event(struct nl_msg *n, void *arg)
    {
    struct nlmsghdr *nlh = nlmsg_hdr(n);
    struct genlmsghdr *genlhdr = genlmsg_hdr(nlh);
    struct nlattr *attrs[THERMAL_GENL_ATTR_MAX + 1];
    int ret;
    let mut perf_cap: perf_cap = {0};
    ret = genlmsg_parse(nlh, 0, attrs, THERMAL_GENL_ATTR_MAX, core::ptr::null_mut());
    debug_printf("Received event %d parse_rer:%d\n", genlhdr.cmd, ret);
    if (genlhdr.cmd == THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE) {
    struct nlattr *cap;
    int j, index = 0;
    debug_printf("THERMAL_GENL_EVENT_CPU_CAPABILITY_CHANGE\n");
    nla_for_each_nested(cap, attrs[THERMAL_GENL_ATTR_CPU_CAPABILITY], j) {
    switch (index) {
    case 0:
    perf_cap.cpu = nla_get_u32(cap);
    break;
    case 1:
    perf_cap.perf = nla_get_u32(cap);
    break;
    case 2:
    perf_cap.eff = nla_get_u32(cap);
    break;
    default:
    break;
    }
    ++index;
    if (index == 3) {
    index = 0;
    process_hfi_event(&perf_cap);
    }
    }
    }
    return 0;
    }
    static int _hfi_exit;
#[no_mangle]
unsafe extern "C" fn check_hf_suport() -> c_int {
    static int check_hf_suport(void)
    {
    let mut eax: c_uint = 0, ebx = 0, ecx = 0, edx = 0;
    __cpuid(6, eax, ebx, ecx, edx);
    if (eax & BIT(19))
    return 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hfi_main() -> c_int {
    int hfi_main(void)
    {
    struct nl_sock *sock;
    struct nl_cb *cb;
    let mut err: c_int = 0;
    int mcast_id;
    if (!check_hf_suport()) {
    fprintf(stderr, "CPU Doesn't support HFI\n");
    return -1;
    }
    sock = nl_socket_alloc();
    if (!sock) {
    fprintf(stderr, "nl_socket_alloc failed\n");
    return -1;
    }
    if (genl_connect(sock)) {
    fprintf(stderr, "genl_connect(sk_event) failed\n");
    goto free_sock;
    }
    drv.nl_handle = sock;
    drv.nl_cb = cb = nl_cb_alloc(NL_CB_DEFAULT);
    if (drv.nl_cb == core::ptr::null_mut()) {
    printf("Failed to allocate netlink callbacks");
    goto free_sock;
    }
    mcast_id = nl_get_multicast_id(&drv, THERMAL_GENL_FAMILY_NAME,
    THERMAL_GENL_EVENT_GROUP_NAME);
    if (mcast_id < 0) {
    fprintf(stderr, "nl_get_multicast_id failed\n");
    goto free_sock;
    }
    if (nl_socket_add_membership(sock, mcast_id)) {
    fprintf(stderr, "nl_socket_add_membership failed");
    goto free_sock;
    }
    nl_cb_set(cb, NL_CB_SEQ_CHECK, NL_CB_CUSTOM, seq_check_handler, 0);
    nl_cb_set(cb, NL_CB_VALID, NL_CB_CUSTOM, handle_event, core::ptr::null_mut());
    debug_printf("hfi is initialized\n");
    while (!_hfi_exit && !err) {
    err = nl_recvmsgs(sock, cb);
    debug_printf("nl_recv_message err:%d\n", err);
    }
    return 0;
// Netlink library doesn't have calls to dealloc cb or disconnect
    free_sock:
    nl_socket_free(sock);
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn hfi_exit() {
    void hfi_exit(void)
    {
    _hfi_exit = 1;
    }
