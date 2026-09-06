//! Automatically rewritten from C to Rust
//! Source: tools/lib/thermal/sampling.c
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


// SPDX-License-Identifier: LGPL-2.1+
// Copyright (C) 2022, Linaro Ltd - Daniel Lezcano <daniel.lezcano@linaro.org>

#[no_mangle]
unsafe extern "C" fn handle_thermal_sample(n: *mut nl_msg, arg: *mut c_void) -> c_int {
    static int handle_thermal_sample(struct nl_msg *n, void *arg)
    {
    struct nlmsghdr *nlh = nlmsg_hdr(n);
    struct genlmsghdr *genlhdr = genlmsg_hdr(nlh);
    struct nlattr *attrs[THERMAL_GENL_ATTR_MAX + 1];
    struct thermal_handler_param *thp = arg;
    struct thermal_handler *th = thp.th;
    arg = thp.arg;
    genlmsg_parse(nlh, 0, attrs, THERMAL_GENL_ATTR_MAX, core::ptr::null_mut());
    switch (genlhdr.cmd) {
    case THERMAL_GENL_SAMPLING_TEMP:
    return th.ops.sampling.tz_temp(
    nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_ID]),
    nla_get_u32(attrs[THERMAL_GENL_ATTR_TZ_TEMP]), arg);
    default:
    return THERMAL_ERROR;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_handle(th: *mut thermal_handler, arg: *mut c_void) -> thermal_error_t {
    thermal_error_t thermal_sampling_handle(struct thermal_handler *th, void *arg)
    {
    let mut thp: thermal_handler_param = { .th = th, .arg = arg };
    if (!th)
    return THERMAL_ERROR;
    if (nl_cb_set(th.cb_sampling, NL_CB_VALID, NL_CB_CUSTOM,
    handle_thermal_sample, &thp))
    return THERMAL_ERROR;
    return nl_recvmsgs(th.sk_sampling, th.cb_sampling);
    }
#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_fd(th: *mut thermal_handler) -> c_int {
    int thermal_sampling_fd(struct thermal_handler *th)
    {
    if (!th)
    return -1;
    return nl_socket_get_fd(th.sk_sampling);
    }
#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_exit(th: *mut thermal_handler) -> thermal_error_t {
    thermal_error_t thermal_sampling_exit(struct thermal_handler *th)
    {
    if (nl_unsubscribe_thermal(th.sk_sampling, th.cb_sampling,
    THERMAL_GENL_SAMPLING_GROUP_NAME))
    return THERMAL_ERROR;
    nl_thermal_disconnect(th.sk_sampling, th.cb_sampling);
    return THERMAL_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn thermal_sampling_init(th: *mut thermal_handler) -> thermal_error_t {
    thermal_error_t thermal_sampling_init(struct thermal_handler *th)
    {
    if (nl_thermal_connect(&th.sk_sampling, &th.cb_sampling))
    return THERMAL_ERROR;
    if (nl_subscribe_thermal(th.sk_sampling, th.cb_sampling,
    THERMAL_GENL_SAMPLING_GROUP_NAME))
    return THERMAL_ERROR;
    return THERMAL_SUCCESS;
    }
