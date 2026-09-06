//! Automatically rewritten from C to Rust
//! Source: drivers/slimbus/messaging.c
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
// Copyright (c) 2011-2017, The Linux Foundation
//

//
// slim_msg_response() - Deliver Message response received from a device to the
// framework.
//
// @ctrl: Controller handle
// @reply: Reply received from the device
// @tid: Transaction ID received with which framework can associate reply.
// @len: Length of the reply
//
// Called by controller to inform framework about the response received.
// This helps in making the API asynchronous, and controller-driver doesn't need
// to manage 1 more table other than the one managed by framework mapping TID
// with buffers
//
#[no_mangle]
pub unsafe extern "C" fn slim_msg_response(ctrl: *mut slim_controller, reply: *mut u8, tid: u8, len: u8) {
    void slim_msg_response(struct slim_controller *ctrl, u8 *reply, u8 tid, u8 len)
    {
    struct slim_msg_txn *txn;
    struct slim_val_inf *msg;
    unsigned long flags;
    spin_lock_irqsave(&ctrl.txn_lock, flags);
    txn = idr_find(&ctrl.tid_idr, tid);
    spin_unlock_irqrestore(&ctrl.txn_lock, flags);
    if (txn == core::ptr::null_mut())
    return;
    msg = txn.msg;
    if (msg == core::ptr::null_mut() || msg.rbuf == core::ptr::null_mut()) {
    dev_err(ctrl.dev, "Got response to invalid TID:%d, len:%d\n",
    tid, len);
    return;
    }
    slim_free_txn_tid(ctrl, txn);
    memcpy(msg.rbuf, reply, len);
    if (txn.comp)
    complete(txn.comp);
// Remove runtime-pm vote now that response was received for TID txn
    pm_runtime_mark_last_busy(ctrl.dev);
    pm_runtime_put_autosuspend(ctrl.dev);
    }
    EXPORT_SYMBOL_GPL(slim_msg_response);
//
// slim_alloc_txn_tid() - Allocate a tid to txn
//
// @ctrl: Controller handle
// @txn: transaction to be allocated with tid.
//
// Return: zero on success with valid txn->tid and error code on failures.
//
#[no_mangle]
pub unsafe extern "C" fn slim_alloc_txn_tid(ctrl: *mut slim_controller, txn: *mut slim_msg_txn) -> c_int {
    int slim_alloc_txn_tid(struct slim_controller *ctrl, struct slim_msg_txn *txn)
    {
    unsigned long flags;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&ctrl.txn_lock, flags);
    ret = idr_alloc_cyclic(&ctrl.tid_idr, txn, 1,
    SLIM_MAX_TIDS, GFP_ATOMIC);
    if (ret < 0) {
    spin_unlock_irqrestore(&ctrl.txn_lock, flags);
    return ret;
    }
    txn.tid = ret;
    spin_unlock_irqrestore(&ctrl.txn_lock, flags);
    return 0;
    }
    EXPORT_SYMBOL_GPL(slim_alloc_txn_tid);
//
// slim_free_txn_tid() - Free tid of txn
//
// @ctrl: Controller handle
// @txn: transaction whose tid should be freed
//
#[no_mangle]
pub unsafe extern "C" fn slim_free_txn_tid(ctrl: *mut slim_controller, txn: *mut slim_msg_txn) {
    void slim_free_txn_tid(struct slim_controller *ctrl, struct slim_msg_txn *txn)
    {
    unsigned long flags;
    spin_lock_irqsave(&ctrl.txn_lock, flags);
    idr_remove(&ctrl.tid_idr, txn.tid);
    spin_unlock_irqrestore(&ctrl.txn_lock, flags);
    }
    EXPORT_SYMBOL_GPL(slim_free_txn_tid);
//
// slim_do_transfer() - Process a SLIMbus-messaging transaction
//
// @ctrl: Controller handle
// @txn: Transaction to be sent over SLIMbus
//
// Called by controller to transmit messaging transactions not dealing with
// Interface/Value elements. (e.g. transmitting a message to assign logical
// address to a slave device
//
// Return: -ETIMEDOUT: If transmission of this message timed out
// (e.g. due to bus lines not being clocked or driven by controller)
//
#[no_mangle]
pub unsafe extern "C" fn slim_do_transfer(ctrl: *mut slim_controller, txn: *mut slim_msg_txn) -> c_int {
    int slim_do_transfer(struct slim_controller *ctrl, struct slim_msg_txn *txn)
    {
    DECLARE_COMPLETION_ONSTACK(done);
    let mut need_tid: bool = false, clk_pause_msg = false;
    int ret;
    unsigned long time_left;
//
// do not vote for runtime-PM if the transactions are part of clock
// pause sequence
//
    if (ctrl.sched.clk_state == SLIM_CLK_ENTERING_PAUSE &&
    (txn.mt == SLIM_MSG_MT_CORE &&
    txn.mc >= SLIM_MSG_MC_BEGIN_RECONFIGURATION &&
    txn.mc <= SLIM_MSG_MC_RECONFIGURE_NOW))
    clk_pause_msg = true;
    if (!clk_pause_msg) {
    ret = pm_runtime_get_sync(ctrl.dev);
    if (ctrl.sched.clk_state != SLIM_CLK_ACTIVE) {
    dev_err(ctrl.dev, "ctrl wrong state:%d, ret:%d\n",
    ctrl.sched.clk_state, ret);
    goto slim_xfer_err;
    }
    }
// Initialize tid to invalid value
    txn.tid = 0;
    need_tid = slim_tid_txn(txn.mt, txn.mc);
    if (need_tid) {
    ret = slim_alloc_txn_tid(ctrl, txn);
    if (ret)
    return ret;
    if (!txn.msg.comp)
    txn.comp = &done;
    }
    ret = ctrl.xfer_msg(ctrl, txn);
    if (ret == -ETIMEDOUT) {
    slim_free_txn_tid(ctrl, txn);
    } else if (!ret && need_tid && !txn.msg.comp) {
    let mut ms: c_ulong = txn.rl + HZ;
    time_left = wait_for_completion_timeout(txn.comp,
    msecs_to_jiffies(ms));
    if (!time_left) {
    ret = -ETIMEDOUT;
    slim_free_txn_tid(ctrl, txn);
    }
    }
    if (ret)
    dev_err(ctrl.dev, "Tx:MT:0x%x, MC:0x%x, LA:0x%x failed:%d\n",
    txn.mt, txn.mc, txn.la, ret);
    slim_xfer_err:
    if (!clk_pause_msg && (txn.tid == 0  || ret == -ETIMEDOUT)) {
//
// remove runtime-pm vote if this was TX only, or
// if there was error during this transaction
//
    pm_runtime_mark_last_busy(ctrl.dev);
    pm_runtime_put_autosuspend(ctrl.dev);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(slim_do_transfer);
    static int slim_val_inf_sanity(struct slim_controller *ctrl,
    struct slim_val_inf *msg, u8 mc)
    {
    if (!msg || msg.num_bytes > 16 ||
    (msg.start_offset + msg.num_bytes) > 0xC00)
    goto reterr;
    switch (mc) {
    case SLIM_MSG_MC_REQUEST_VALUE:
    case SLIM_MSG_MC_REQUEST_INFORMATION:
    if (msg.rbuf != core::ptr::null_mut())
    return 0;
    break;
    case SLIM_MSG_MC_CHANGE_VALUE:
    case SLIM_MSG_MC_CLEAR_INFORMATION:
    if (msg.wbuf != core::ptr::null_mut())
    return 0;
    break;
    case SLIM_MSG_MC_REQUEST_CHANGE_VALUE:
    case SLIM_MSG_MC_REQUEST_CLEAR_INFORMATION:
    if (msg.rbuf != core::ptr::null_mut() && msg.wbuf != core::ptr::null_mut())
    return 0;
    break;
    }
    reterr:
    if (msg)
    dev_err(ctrl.dev, "Sanity check failed:msg:offset:0x%x, mc:%d\n",
    msg.start_offset, mc);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn slim_slicesize(code: c_int) -> u16 {
    static u16 slim_slicesize(int code)
    {
    static const u8 sizetocode[16] = {
    0, 1, 2, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7
    };
    code = clamp(code, 1, (int)ARRAY_SIZE(sizetocode));
    return sizetocode[code - 1];
    }
//
// slim_xfer_msg() - Transfer a value info message on slim device
//
// @sbdev: slim device to which this msg has to be transferred
// @msg: value info message pointer
// @mc: message code of the message
//
// Called by drivers which want to transfer a vlaue or info elements.
//
// Return: -ETIMEDOUT: If transmission of this message timed out
//
    int slim_xfer_msg(struct slim_device *sbdev, struct slim_val_inf *msg,
    u8 mc)
    {
    DEFINE_SLIM_LDEST_TXN(txn_stack, mc, 6, sbdev.laddr, msg);
    struct slim_msg_txn *txn = &txn_stack;
    struct slim_controller *ctrl = sbdev.ctrl;
    int ret;
    u16 sl;
    if (!ctrl)
    return -EINVAL;
    ret = slim_val_inf_sanity(ctrl, msg, mc);
    if (ret)
    return ret;
    sl = slim_slicesize(msg.num_bytes);
    dev_dbg(ctrl.dev, "SB xfer msg:os:%x, len:%d, MC:%x, sl:%x\n",
    msg.start_offset, msg.num_bytes, mc, sl);
    txn.ec = ((sl | (1 << 3)) | ((msg.start_offset & 0xFFF) << 4));
    switch (mc) {
    case SLIM_MSG_MC_REQUEST_CHANGE_VALUE:
    case SLIM_MSG_MC_CHANGE_VALUE:
    case SLIM_MSG_MC_REQUEST_CLEAR_INFORMATION:
    case SLIM_MSG_MC_CLEAR_INFORMATION:
    txn.rl += msg.num_bytes;
    break;
    default:
    break;
    }
    if (slim_tid_txn(txn.mt, txn.mc))
    txn.rl++;
    return slim_do_transfer(ctrl, txn);
    }
    EXPORT_SYMBOL_GPL(slim_xfer_msg);
    static void slim_fill_msg(struct slim_val_inf *msg, u32 addr,
    size_t count, u8 *rbuf, u8 *wbuf)
    {
    msg.start_offset = addr;
    msg.num_bytes = count;
    msg.rbuf = rbuf;
    msg.wbuf = wbuf;
    msg.comp = core::ptr::null_mut();
    }
//
// slim_read() - Read SLIMbus value element
//
// @sdev: client handle.
// @addr:  address of value element to read.
// @count: number of bytes to read. Maximum bytes allowed are 16.
// @val: will return what the value element value was
//
// Return: -EINVAL for Invalid parameters, -ETIMEDOUT If transmission of
// this message timed out (e.g. due to bus lines not being clocked
// or driven by controller)
//
#[no_mangle]
pub unsafe extern "C" fn slim_read(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> c_int {
    int slim_read(struct slim_device *sdev, u32 addr, size_t count, u8 *val)
    {
    struct slim_val_inf msg;
    slim_fill_msg(&msg, addr, count, val, core::ptr::null_mut());
    return slim_xfer_msg(sdev, &msg, SLIM_MSG_MC_REQUEST_VALUE);
    }
    EXPORT_SYMBOL_GPL(slim_read);
//
// slim_readb() - Read byte from SLIMbus value element
//
// @sdev: client handle.
// @addr:  address in the value element to read.
//
// Return: byte value of value element.
//
#[no_mangle]
pub unsafe extern "C" fn slim_readb(sdev: *mut slim_device, addr: u32) -> c_int {
    int slim_readb(struct slim_device *sdev, u32 addr)
    {
    int ret;
    u8 buf;
    ret = slim_read(sdev, addr, 1, &buf);
    if (ret < 0)
    return ret;
    else
    return buf;
    }
    EXPORT_SYMBOL_GPL(slim_readb);
//
// slim_write() - Write SLIMbus value element
//
// @sdev: client handle.
// @addr:  address in the value element to write.
// @count: number of bytes to write. Maximum bytes allowed are 16.
// @val: value to write to value element
//
// Return: -EINVAL for Invalid parameters, -ETIMEDOUT If transmission of
// this message timed out (e.g. due to bus lines not being clocked
// or driven by controller)
//
#[no_mangle]
pub unsafe extern "C" fn slim_write(sdev: *mut slim_device, addr: u32, count: usize, val: *mut u8) -> c_int {
    int slim_write(struct slim_device *sdev, u32 addr, size_t count, u8 *val)
    {
    struct slim_val_inf msg;
    slim_fill_msg(&msg, addr, count,  core::ptr::null_mut(), val);
    return slim_xfer_msg(sdev, &msg, SLIM_MSG_MC_CHANGE_VALUE);
    }
    EXPORT_SYMBOL_GPL(slim_write);
//
// slim_writeb() - Write byte to SLIMbus value element
//
// @sdev: client handle.
// @addr:  address of value element to write.
// @value: value to write to value element
//
// Return: -EINVAL for Invalid parameters, -ETIMEDOUT If transmission of
// this message timed out (e.g. due to bus lines not being clocked
// or driven by controller)
//
#[no_mangle]
pub unsafe extern "C" fn slim_writeb(sdev: *mut slim_device, addr: u32, value: u8) -> c_int {
    int slim_writeb(struct slim_device *sdev, u32 addr, u8 value)
    {
    return slim_write(sdev, addr, 1, &value);
    }
    EXPORT_SYMBOL_GPL(slim_writeb);
