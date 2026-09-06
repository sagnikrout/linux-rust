//! Automatically rewritten from C to Rust
//! Source: drivers/net/wwan/t7xx/t7xx_port_trace.c
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
// Copyright (C) 2022 Intel Corporation.
//

pub const T7XX_TRC_SUB_BUFF_SIZE: c_int = 131072;
pub const T7XX_TRC_N_SUB_BUFF: c_int = 32;
    static struct dentry *t7xx_trace_create_buf_file_handler(const char *filename,
    struct dentry *parent,
    umode_t mode,
    struct rchan_buf *buf,
    int *is_global)
    {
// is_global = 1;
    return debugfs_create_file(filename, mode, parent, buf,
    &relay_file_operations);
    }
#[no_mangle]
unsafe extern "C" fn t7xx_trace_remove_buf_file_handler(dentry: *mut dentry) -> c_int {
    static int t7xx_trace_remove_buf_file_handler(struct dentry *dentry)
    {
    debugfs_remove(dentry);
    return 0;
    }
    static int t7xx_trace_subbuf_start_handler(struct rchan_buf *buf, void *subbuf,
    void *prev_subbuf)
    {
    if (relay_buf_full(buf)) {
    pr_err_ratelimited("Relay_buf full dropping traces");
    return 0;
    }
    return 1;
    }
    static struct rchan_callbacks relay_callbacks = {
    .subbuf_start = t7xx_trace_subbuf_start_handler,
    .create_buf_file = t7xx_trace_create_buf_file_handler,
    .remove_buf_file = t7xx_trace_remove_buf_file_handler,
    };
#[no_mangle]
unsafe extern "C" fn t7xx_trace_port_uninit(port: *mut t7xx_port) {
    static void t7xx_trace_port_uninit(struct t7xx_port *port)
    {
    struct dentry *debugfs_dir = port.t7xx_dev.debugfs_dir;
    struct rchan *relaych = port.log.relaych;
    if (!relaych)
    return;
    relay_close(relaych);
    debugfs_remove_recursive(debugfs_dir);
    port.log.relaych = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn t7xx_trace_port_recv_skb(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int {
    static int t7xx_trace_port_recv_skb(struct t7xx_port *port, struct sk_buff *skb)
    {
    struct rchan *relaych = port.log.relaych;
    if (!relaych)
    return -EINVAL;
    relay_write(relaych, skb.data, skb.len);
    dev_kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_trace_md_state_notify(port: *mut t7xx_port, state: c_uint) {
    static void t7xx_port_trace_md_state_notify(struct t7xx_port *port, unsigned int state)
    {
    struct rchan *relaych = port.log.relaych;
    struct dentry *debugfs_wwan_dir;
    struct dentry *debugfs_dir;
    if (state != MD_STATE_READY || relaych)
    return;
    debugfs_wwan_dir = wwan_get_debugfs_dir(port.dev);
    if (IS_ERR(debugfs_wwan_dir))
    return;
    debugfs_dir = debugfs_create_dir(KBUILD_MODNAME, debugfs_wwan_dir);
    if (IS_ERR_OR_NULL(debugfs_dir)) {
    wwan_put_debugfs_dir(debugfs_wwan_dir);
    dev_err(port.dev, "Unable to create debugfs for trace");
    return;
    }
    relaych = relay_open("relay_ch", debugfs_dir, T7XX_TRC_SUB_BUFF_SIZE,
    T7XX_TRC_N_SUB_BUFF, &relay_callbacks, core::ptr::null_mut());
    if (!relaych)
    goto err_rm_debugfs_dir;
    wwan_put_debugfs_dir(debugfs_wwan_dir);
    port.log.relaych = relaych;
    port.t7xx_dev.debugfs_dir = debugfs_dir;
    return;
    err_rm_debugfs_dir:
    debugfs_remove_recursive(debugfs_dir);
    wwan_put_debugfs_dir(debugfs_wwan_dir);
    dev_err(port.dev, "Unable to create trace port %s", port.port_conf.name);
    }
    struct port_ops t7xx_trace_port_ops = {
    .recv_skb = t7xx_trace_port_recv_skb,
    .uninit = t7xx_trace_port_uninit,
    .md_state_notify = t7xx_port_trace_md_state_notify,
    };
