//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/ipc.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// Generic IPC layer that can work over MMIO and SPI/I2C. PHY layer provided
// by platform driver code.
//

//
// sof_ipc_send_msg - generic function to prepare and send one IPC message
// @sdev:		pointer to SOF core device struct
// @msg_data:		pointer to a message to send
// @msg_bytes:		number of bytes in the message
// @reply_bytes:	number of bytes available for the reply.
// The buffer for the reply data is not passed to this
// function, the available size is an information for the
// reply handling functions.
//
// On success the function returns 0, otherwise negative error number.
//
// Note: higher level sdev->ipc->tx_mutex must be held to make sure that
// transfers are synchronized.
//
    int sof_ipc_send_msg(struct snd_sof_dev *sdev, void *msg_data, size_t msg_bytes,
    size_t reply_bytes)
    {
    struct snd_sof_ipc *ipc = sdev.ipc;
    struct snd_sof_ipc_msg *msg;
    int ret;
    if (ipc.disable_ipc_tx || sdev.fw_state != SOF_FW_BOOT_COMPLETE)
    return -ENODEV;
//
// The spin-lock is needed to protect message objects against other
// atomic contexts.
//
    guard(spinlock_irq)(&sdev.ipc_lock);
// initialise the message
    msg = &ipc.msg;
// attach message data
    msg.msg_data = msg_data;
    msg.msg_size = msg_bytes;
    msg.reply_size = reply_bytes;
    msg.reply_error = 0;
    sdev.msg = msg;
    ret = snd_sof_dsp_send_msg(sdev, msg);
// Next reply that we receive will be related to this message
    if (!ret)
    msg.ipc_complete = false;
    return ret;
    }
// send IPC message from host to DSP
    int sof_ipc_tx_message(struct snd_sof_ipc *ipc, void *msg_data, size_t msg_bytes,
    void *reply_data, size_t reply_bytes)
    {
    if (msg_bytes > ipc.max_payload_size ||
    reply_bytes > ipc.max_payload_size)
    return -ENOBUFS;
    return ipc.ops.tx_msg(ipc.sdev, msg_data, msg_bytes, reply_data,
    reply_bytes, false);
    }
    EXPORT_SYMBOL(sof_ipc_tx_message);
// IPC set or get data from host to DSP
    int sof_ipc_set_get_data(struct snd_sof_ipc *ipc, void *msg_data,
    size_t msg_bytes, bool set)
    {
    return ipc.ops.set_get_data(ipc.sdev, msg_data, msg_bytes, set);
    }
    EXPORT_SYMBOL(sof_ipc_set_get_data);
//
// send IPC message from host to DSP without modifying the DSP state.
// This will be used for IPC's that can be handled by the DSP
// even in a low-power D0 substate.
//
    int sof_ipc_tx_message_no_pm(struct snd_sof_ipc *ipc, void *msg_data, size_t msg_bytes,
    void *reply_data, size_t reply_bytes)
    {
    if (msg_bytes > ipc.max_payload_size ||
    reply_bytes > ipc.max_payload_size)
    return -ENOBUFS;
    return ipc.ops.tx_msg(ipc.sdev, msg_data, msg_bytes, reply_data,
    reply_bytes, true);
    }
    EXPORT_SYMBOL(sof_ipc_tx_message_no_pm);
// Generic helper function to retrieve the reply
#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev) {
    void snd_sof_ipc_get_reply(struct snd_sof_dev *sdev)
    {
//
// Sometimes, there is unexpected reply ipc arriving. The reply
// ipc belongs to none of the ipcs sent from driver.
// In this case, the driver must ignore the ipc.
//
    if (!sdev.msg) {
    dev_warn(sdev.dev, "unexpected ipc interrupt raised!\n");
    return;
    }
    sdev.msg.reply_error = sdev.ipc.ops.get_reply(sdev);
    }
    EXPORT_SYMBOL(snd_sof_ipc_get_reply);
// handle reply message from DSP
#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg_id: u32) {
    void snd_sof_ipc_reply(struct snd_sof_dev *sdev, u32 msg_id)
    {
    struct snd_sof_ipc_msg *msg = &sdev.ipc.msg;
    if (msg.ipc_complete) {
    dev_dbg(sdev.dev,
    "no reply expected, received 0x%x, will be ignored",
    msg_id);
    return;
    }
// wake up and return the error if we have waiters on this message ?
    msg.ipc_complete = true;
    wake_up(&msg.waitq);
    }
    EXPORT_SYMBOL(snd_sof_ipc_reply);
    struct snd_sof_ipc *snd_sof_ipc_init(struct snd_sof_dev *sdev)
    {
    struct snd_sof_ipc *ipc;
    struct snd_sof_ipc_msg *msg;
    const struct sof_ipc_ops *ops;
    ipc = devm_kzalloc(sdev.dev, sizeof(*ipc), GFP_KERNEL);
    if (!ipc)
    return core::ptr::null_mut();
    mutex_init(&ipc.tx_mutex);
    ipc.sdev = sdev;
    msg = &ipc.msg;
// indicate that we aren't sending a message ATM
    msg.ipc_complete = true;
    init_waitqueue_head(&msg.waitq);
    switch (sdev.pdata.ipc_type) {

    case SOF_IPC_TYPE_3:
    ops = &ipc3_ops;
    break;

    case SOF_IPC_TYPE_4:
    ops = &ipc4_ops;
    break;

    default:
    dev_err(sdev.dev, "Not supported IPC version: %d\n",
    sdev.pdata.ipc_type);
    return core::ptr::null_mut();
    }
// check for mandatory ops
    if (!ops.tx_msg || !ops.rx_msg || !ops.set_get_data || !ops.get_reply) {
    dev_err(sdev.dev, "Missing IPC message handling ops\n");
    return core::ptr::null_mut();
    }
    if (!ops.fw_loader || !ops.fw_loader.validate ||
    !ops.fw_loader.parse_ext_manifest) {
    dev_err(sdev.dev, "Missing IPC firmware loading ops\n");
    return core::ptr::null_mut();
    }
    if (!ops.pcm) {
    dev_err(sdev.dev, "Missing IPC PCM ops\n");
    return core::ptr::null_mut();
    }
    if (!ops.tplg || !ops.tplg.widget || !ops.tplg.control) {
    dev_err(sdev.dev, "Missing IPC topology ops\n");
    return core::ptr::null_mut();
    }
    if (ops.fw_tracing && (!ops.fw_tracing.init || !ops.fw_tracing.suspend ||
    !ops.fw_tracing.resume)) {
    dev_err(sdev.dev, "Missing firmware tracing ops\n");
    return core::ptr::null_mut();
    }
    if (ops.init && ops.init(sdev))
    return core::ptr::null_mut();
    ipc.ops = ops;
    return ipc;
    }
    EXPORT_SYMBOL(snd_sof_ipc_init);
#[no_mangle]
pub unsafe extern "C" fn snd_sof_ipc_free(sdev: *mut snd_sof_dev) {
    void snd_sof_ipc_free(struct snd_sof_dev *sdev)
    {
    struct snd_sof_ipc *ipc = sdev.ipc;
    if (!ipc)
    return;
// disable sending of ipc's
    scoped_guard(mutex, &ipc.tx_mutex)
    ipc.disable_ipc_tx = true;
    if (ipc.ops.exit)
    ipc.ops.exit(sdev);
    }
    EXPORT_SYMBOL(snd_sof_ipc_free);
