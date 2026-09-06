//! Automatically rewritten from C to Rust
//! Source: drivers/net/wwan/t7xx/t7xx_port_wwan.c
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
// Copyright (c) 2021, MediaTek Inc.
// Copyright (c) 2021-2022, Intel Corporation.
// Copyright (c) 2024, Fibocom Wireless Inc.
//
// Authors:
// Amir Hanania <amir.hanania@intel.com>
// Chandrashekar Devegowda <chandrashekar.devegowda@intel.com>
// Haijun Liu <haijun.liu@mediatek.com>
// Moises Veleta <moises.veleta@intel.com>
// Ricardo Martinez <ricardo.martinez@linux.intel.com>
//
// Contributors:
// Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Chiranjeevi Rapolu <chiranjeevi.rapolu@intel.com>
// Eliot Lee <eliot.lee@intel.com>
// Sreehari Kancharla <sreehari.kancharla@intel.com>
// Jinjian Song <jinjian.song@fibocom.com>
//

#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_start(port: *mut wwan_port) -> c_int {
    static int t7xx_port_wwan_start(struct wwan_port *port)
    {
    struct t7xx_port *port_mtk = wwan_port_get_drvdata(port);
    if (atomic_read(&port_mtk.usage_cnt))
    return -EBUSY;
    atomic_inc(&port_mtk.usage_cnt);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_stop(port: *mut wwan_port) {
    static void t7xx_port_wwan_stop(struct wwan_port *port)
    {
    struct t7xx_port *port_mtk = wwan_port_get_drvdata(port);
    atomic_dec(&port_mtk.usage_cnt);
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_fastboot_tx(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int {
    static int t7xx_port_fastboot_tx(struct t7xx_port *port, struct sk_buff *skb)
    {
    struct sk_buff *cur = skb, *tx_skb;
    size_t actual, len, offset = 0;
    int txq_mtu;
    int ret;
    txq_mtu = t7xx_get_port_mtu(port);
    if (txq_mtu < 0)
    return -EINVAL;
    actual = cur.len;
    while (actual) {
    len = min_t(size_t, actual, txq_mtu);
    tx_skb = __dev_alloc_skb(len, GFP_KERNEL);
    if (!tx_skb)
    return -ENOMEM;
    skb_put_data(tx_skb, cur.data + offset, len);
    ret = t7xx_port_send_raw_skb(port, tx_skb);
    if (ret) {
    dev_kfree_skb(tx_skb);
    dev_err(port.dev, "Write error on fastboot port, %d\n", ret);
    break;
    }
    offset += len;
    actual -= len;
    }
    dev_kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_ctrl_tx(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int {
    static int t7xx_port_ctrl_tx(struct t7xx_port *port, struct sk_buff *skb)
    {
    const struct t7xx_port_conf *port_conf;
    struct sk_buff *cur = skb, *cloned;
    struct t7xx_fsm_ctl *ctl;
    enum md_state md_state;
    let mut cnt: c_int = 0, ret;
    port_conf = port.port_conf;
    ctl = port.t7xx_dev.md.fsm_ctl;
    md_state = t7xx_fsm_get_md_state(ctl);
    if (md_state == MD_STATE_WAITING_FOR_HS1 || md_state == MD_STATE_WAITING_FOR_HS2) {
    dev_warn(port.dev, "Cannot write to %s port when md_state=%d\n",
    port_conf.name, md_state);
    return -ENODEV;
    }
    while (cur) {
    cloned = skb_clone(cur, GFP_KERNEL);
    if (!cloned)
    return cnt ? cnt : -ENOMEM;
    cloned.len = skb_headlen(cur);
    ret = t7xx_port_send_skb(port, cloned, 0, 0);
    if (ret) {
    dev_kfree_skb(cloned);
    dev_err(port.dev, "Write error on %s port, %d\n",
    port_conf.name, ret);
    return cnt ? cnt + ret : ret;
    }
    cnt += cur.len;
    if (cur == skb)
    cur = skb_shinfo(skb).frag_list;
    else
    cur = cur.next;
    }
    dev_kfree_skb(skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_tx(port: *mut wwan_port, skb: *mut sk_buff) -> c_int {
    static int t7xx_port_wwan_tx(struct wwan_port *port, struct sk_buff *skb)
    {
    struct t7xx_port *port_private = wwan_port_get_drvdata(port);
    const struct t7xx_port_conf *port_conf = port_private.port_conf;
    int ret;
    if (!port_private.chan_enable)
    return -EINVAL;
    if (port_conf.port_type != WWAN_PORT_FASTBOOT)
    ret = t7xx_port_ctrl_tx(port_private, skb);
    else
    ret = t7xx_port_fastboot_tx(port_private, skb);
    return ret;
    }
    static const struct wwan_port_ops wwan_ops = {
    .start = t7xx_port_wwan_start,
    .stop = t7xx_port_wwan_stop,
    .tx = t7xx_port_wwan_tx,
    };
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_create(port: *mut t7xx_port) {
    static void t7xx_port_wwan_create(struct t7xx_port *port)
    {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    let mut header_len: c_uint = sizeof(struct ccci_header), mtu;
    struct wwan_port_caps caps;
    if (!port.wwan.wwan_port) {
    mtu = t7xx_get_port_mtu(port);
    caps.frag_len = mtu - header_len;
    caps.headroom_len = header_len;
    port.wwan.wwan_port = wwan_create_port(port.dev, port_conf.port_type,
    &wwan_ops, &caps, port);
    if (IS_ERR(port.wwan.wwan_port))
    dev_err(port.dev, "Unable to create WWAN port %s", port_conf.name);
    }
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_init(port: *mut t7xx_port) -> c_int {
    static int t7xx_port_wwan_init(struct t7xx_port *port)
    {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    if (port_conf.port_type == WWAN_PORT_FASTBOOT ||
    port_conf.port_type == WWAN_PORT_ADB ||
    port_conf.port_type == WWAN_PORT_MIPC)
    t7xx_port_wwan_create(port);
    port.rx_length_th = RX_QUEUE_MAXLEN;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_uninit(port: *mut t7xx_port) {
    static void t7xx_port_wwan_uninit(struct t7xx_port *port)
    {
    if (!port.wwan.wwan_port)
    return;
    port.rx_length_th = 0;
    wwan_remove_port(port.wwan.wwan_port);
    port.wwan.wwan_port = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_recv_skb(port: *mut t7xx_port, skb: *mut sk_buff) -> c_int {
    static int t7xx_port_wwan_recv_skb(struct t7xx_port *port, struct sk_buff *skb)
    {
    if (!atomic_read(&port.usage_cnt) || !port.chan_enable) {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    dev_kfree_skb_any(skb);
    dev_err_ratelimited(port.dev, "Port %s is not opened, drop packets\n",
    port_conf.name);
// Dropping skb, caller should not access skb.
    return 0;
    }
    wwan_port_rx(port.wwan.wwan_port, skb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_enable_chl(port: *mut t7xx_port) -> c_int {
    static int t7xx_port_wwan_enable_chl(struct t7xx_port *port)
    {
    spin_lock(&port.port_update_lock);
    port.chan_enable = true;
    spin_unlock(&port.port_update_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_disable_chl(port: *mut t7xx_port) -> c_int {
    static int t7xx_port_wwan_disable_chl(struct t7xx_port *port)
    {
    spin_lock(&port.port_update_lock);
    port.chan_enable = false;
    spin_unlock(&port.port_update_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn t7xx_port_wwan_md_state_notify(port: *mut t7xx_port, state: c_uint) {
    static void t7xx_port_wwan_md_state_notify(struct t7xx_port *port, unsigned int state)
    {
    const struct t7xx_port_conf *port_conf = port.port_conf;
    if (port_conf.port_type == WWAN_PORT_FASTBOOT ||
    port_conf.port_type == WWAN_PORT_ADB ||
    port_conf.port_type == WWAN_PORT_MIPC)
    return;
    if (state != MD_STATE_READY)
    return;
    t7xx_port_wwan_create(port);
    }
    struct port_ops wwan_sub_port_ops = {
    .init = t7xx_port_wwan_init,
    .recv_skb = t7xx_port_wwan_recv_skb,
    .uninit = t7xx_port_wwan_uninit,
    .enable_chl = t7xx_port_wwan_enable_chl,
    .disable_chl = t7xx_port_wwan_disable_chl,
    .md_state_notify = t7xx_port_wwan_md_state_notify,
    };
