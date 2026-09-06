//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/mtk-adsp-ipc.c
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
// Copyright (c) 2022 MediaTek Corporation. All rights reserved.
// Author: Allen-KH Cheng <allen-kh.cheng@mediatek.com>
//

    static const char * const adsp_mbox_ch_names[MTK_ADSP_MBOX_NUM] = { "rx", "tx" };
//
// mtk_adsp_ipc_send - send ipc cmd to MTK ADSP
//
// @ipc: ADSP IPC handle
// @idx: index of the mailbox channel
// @msg: IPC cmd (reply or request)
//
// Returns zero for success from mbox_send_message
// negative value for error
//
#[no_mangle]
pub unsafe extern "C" fn mtk_adsp_ipc_send(ipc: *mut mtk_adsp_ipc, idx: c_uint, msg: u32) -> c_int {
    int mtk_adsp_ipc_send(struct mtk_adsp_ipc *ipc, unsigned int idx, uint32_t msg)
    {
    struct mtk_adsp_chan *adsp_chan;
    int ret;
    if (idx >= MTK_ADSP_MBOX_NUM)
    return -EINVAL;
    adsp_chan = &ipc.chans[idx];
    ret = mbox_send_message(adsp_chan.ch, &msg);
    if (ret < 0)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL_GPL(mtk_adsp_ipc_send);
//
// mtk_adsp_ipc_recv - recv callback used by MTK ADSP mailbox
//
// @c: mbox client
// @msg: message received
//
// Users of ADSP IPC will need to privde handle_reply and handle_request
// callbacks.
//
#[no_mangle]
unsafe extern "C" fn mtk_adsp_ipc_recv(c: *mut mbox_client, msg: *mut c_void) {
    static void mtk_adsp_ipc_recv(struct mbox_client *c, void *msg)
    {
    struct mtk_adsp_chan *chan = container_of(c, struct mtk_adsp_chan, cl);
    struct device *dev = c.dev;
    switch (chan.idx) {
    case MTK_ADSP_MBOX_REPLY:
    chan.ipc.ops.handle_reply(chan.ipc);
    break;
    case MTK_ADSP_MBOX_REQUEST:
    chan.ipc.ops.handle_request(chan.ipc);
    break;
    default:
    dev_err(dev, "wrong mbox chan %d\n", chan.idx);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mtk_adsp_ipc_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_adsp_ipc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_adsp_ipc *adsp_ipc;
    struct mtk_adsp_chan *adsp_chan;
    struct mbox_client *cl;
    int ret;
    int i, j;
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    adsp_ipc = devm_kzalloc(dev, sizeof(*adsp_ipc), GFP_KERNEL);
    if (!adsp_ipc)
    return -ENOMEM;
    for (i = 0; i < MTK_ADSP_MBOX_NUM; i++) {
    adsp_chan = &adsp_ipc.chans[i];
    cl = &adsp_chan.cl;
    cl.dev = dev.parent;
    cl.tx_block = false;
    cl.knows_txdone = false;
    cl.tx_prepare = core::ptr::null_mut();
    cl.rx_callback = mtk_adsp_ipc_recv;
    adsp_chan.ipc = adsp_ipc;
    adsp_chan.idx = i;
    adsp_chan.ch = mbox_request_channel_byname(cl, adsp_mbox_ch_names[i]);
    if (IS_ERR(adsp_chan.ch)) {
    ret = dev_err_probe(dev, PTR_ERR(adsp_chan.ch),
    "Failed to request mbox channel %s\n",
    adsp_mbox_ch_names[i]);
    for (j = 0; j < i; j++) {
    adsp_chan = &adsp_ipc.chans[j];
    mbox_free_channel(adsp_chan.ch);
    }
    return ret;
    }
    }
    adsp_ipc.dev = dev;
    dev_set_drvdata(dev, adsp_ipc);
    dev_dbg(dev, "MTK ADSP IPC initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_adsp_ipc_remove(pdev: *mut platform_device) {
    static void mtk_adsp_ipc_remove(struct platform_device *pdev)
    {
    struct mtk_adsp_ipc *adsp_ipc = dev_get_drvdata(&pdev.dev);
    struct mtk_adsp_chan *adsp_chan;
    int i;
    for (i = 0; i < MTK_ADSP_MBOX_NUM; i++) {
    adsp_chan = &adsp_ipc.chans[i];
    mbox_free_channel(adsp_chan.ch);
    }
    }
    static struct platform_driver mtk_adsp_ipc_driver = {
    .driver = {
    .name = "mtk-adsp-ipc",
    },
    .probe = mtk_adsp_ipc_probe,
    .remove = mtk_adsp_ipc_remove,
    };
    builtin_platform_driver(mtk_adsp_ipc_driver);
    MODULE_AUTHOR("Allen-KH Cheng <allen-kh.cheng@mediatek.com>");
    MODULE_DESCRIPTION("MTK ADSP IPC Driver");
    MODULE_LICENSE("GPL");
