//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/imx-dsp.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2019 NXP
// Author: Daniel Baluta <daniel.baluta@nxp.com>
//
// Implementation of the DSP IPC interface (host side)
//

//
// imx_dsp_ring_doorbell - triggers an interrupt on the other side (DSP)
//
// @dsp: DSP IPC handle
// @chan_idx: index of the channel where to trigger the interrupt
//
// Returns non-negative value for success, negative value for error
//
#[no_mangle]
pub unsafe extern "C" fn imx_dsp_ring_doorbell(ipc: *mut imx_dsp_ipc, idx: c_uint) -> c_int {
    int imx_dsp_ring_doorbell(struct imx_dsp_ipc *ipc, unsigned int idx)
    {
    int ret;
    struct imx_dsp_chan *dsp_chan;
    if (idx >= DSP_MU_CHAN_NUM)
    return -EINVAL;
    dsp_chan = &ipc.chans[idx];
    ret = mbox_send_message(dsp_chan.ch, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    return 0;
    }
    EXPORT_SYMBOL(imx_dsp_ring_doorbell);
//
// imx_dsp_handle_rx - rx callback used by imx mailbox
//
// @c: mbox client
// @msg: message received
//
// Users of DSP IPC will need to privde handle_reply and handle_request
// callbacks.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_handle_rx(c: *mut mbox_client, msg: *mut c_void) {
    static void imx_dsp_handle_rx(struct mbox_client *c, void *msg)
    {
    struct imx_dsp_chan *chan = container_of(c, struct imx_dsp_chan, cl);
    if (chan.idx == 0) {
    chan.ipc.ops.handle_reply(chan.ipc);
    } else {
    chan.ipc.ops.handle_request(chan.ipc);
    imx_dsp_ring_doorbell(chan.ipc, 1);
    }
    }
    struct mbox_chan *imx_dsp_request_channel(struct imx_dsp_ipc *dsp_ipc, int idx)
    {
    struct imx_dsp_chan *dsp_chan;
    if (idx >= DSP_MU_CHAN_NUM)
    return ERR_PTR(-EINVAL);
    dsp_chan = &dsp_ipc.chans[idx];
    dsp_chan.ch = mbox_request_channel_byname(&dsp_chan.cl, dsp_chan.name);
    return dsp_chan.ch;
    }
    EXPORT_SYMBOL(imx_dsp_request_channel);
#[no_mangle]
pub unsafe extern "C" fn imx_dsp_free_channel(dsp_ipc: *mut imx_dsp_ipc, idx: c_int) {
    void imx_dsp_free_channel(struct imx_dsp_ipc *dsp_ipc, int idx)
    {
    struct imx_dsp_chan *dsp_chan;
    if (idx >= DSP_MU_CHAN_NUM)
    return;
    dsp_chan = &dsp_ipc.chans[idx];
    mbox_free_channel(dsp_chan.ch);
    }
    EXPORT_SYMBOL(imx_dsp_free_channel);
#[no_mangle]
unsafe extern "C" fn imx_dsp_setup_channels(dsp_ipc: *mut imx_dsp_ipc) -> c_int {
    static int imx_dsp_setup_channels(struct imx_dsp_ipc *dsp_ipc)
    {
    struct device *dev = dsp_ipc.dev;
    struct imx_dsp_chan *dsp_chan;
    struct mbox_client *cl;
    char *chan_name;
    int ret;
    int i, j;
    for (i = 0; i < DSP_MU_CHAN_NUM; i++) {
    if (i < 2)
    chan_name = kasprintf(GFP_KERNEL, "txdb%d", i);
    else
    chan_name = kasprintf(GFP_KERNEL, "rxdb%d", i - 2);
    if (!chan_name)
    return -ENOMEM;
    dsp_chan = &dsp_ipc.chans[i];
    dsp_chan.name = chan_name;
    cl = &dsp_chan.cl;
    cl.dev = dev;
    cl.tx_block = false;
    cl.knows_txdone = true;
    cl.rx_callback = imx_dsp_handle_rx;
    dsp_chan.ipc = dsp_ipc;
    dsp_chan.idx = i % 2;
    dsp_chan.ch = mbox_request_channel_byname(cl, chan_name);
    if (IS_ERR(dsp_chan.ch)) {
    ret = PTR_ERR(dsp_chan.ch);
    if (ret != -EPROBE_DEFER)
    dev_err(dev, "Failed to request mbox chan %s ret %d\n",
    chan_name, ret);
    kfree(dsp_chan.name);
    goto out;
    }
    dev_dbg(dev, "request mbox chan %s\n", chan_name);
    }
    return 0;
    out:
    for (j = 0; j < i; j++) {
    dsp_chan = &dsp_ipc.chans[j];
    mbox_free_channel(dsp_chan.ch);
    kfree(dsp_chan.name);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_probe(pdev: *mut platform_device) -> c_int {
    static int imx_dsp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct imx_dsp_ipc *dsp_ipc;
    int ret;
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    dsp_ipc = devm_kzalloc(dev, sizeof(*dsp_ipc), GFP_KERNEL);
    if (!dsp_ipc)
    return -ENOMEM;
    dsp_ipc.dev = dev;
    dev_set_drvdata(dev, dsp_ipc);
    ret = imx_dsp_setup_channels(dsp_ipc);
    if (ret < 0)
    return ret;
    dev_info(dev, "NXP i.MX DSP IPC initialized\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_remove(pdev: *mut platform_device) {
    static void imx_dsp_remove(struct platform_device *pdev)
    {
    struct imx_dsp_chan *dsp_chan;
    struct imx_dsp_ipc *dsp_ipc;
    int i;
    dsp_ipc = dev_get_drvdata(&pdev.dev);
    for (i = 0; i < DSP_MU_CHAN_NUM; i++) {
    dsp_chan = &dsp_ipc.chans[i];
    mbox_free_channel(dsp_chan.ch);
    kfree(dsp_chan.name);
    }
    }
    static struct platform_driver imx_dsp_driver = {
    .driver = {
    .name = "imx-dsp",
    },
    .probe = imx_dsp_probe,
    .remove = imx_dsp_remove,
    };
    builtin_platform_driver(imx_dsp_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@nxp.com>");
    MODULE_DESCRIPTION("IMX DSP IPC protocol driver");
    MODULE_LICENSE("GPL v2");
