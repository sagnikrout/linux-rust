//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/rsi/rsi_91x_usb_ops.c
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
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// rsi_usb_rx_thread() - This is a kernel thread to receive the packets from
// the USB device.
// @data: Pointer to the driver private structure.
//
// Return: 0.
//
#[no_mangle]
pub unsafe extern "C" fn rsi_usb_rx_thread(data: *mut c_void) -> c_int {
    int rsi_usb_rx_thread(void *data)
    {
    struct rsi_common *common = data;
    struct rsi_hw *adapter = common.priv;
    struct rsi_91x_usbdev *dev = adapter.rsi_dev;
    int status;
    struct sk_buff *skb;
    do {
    rsi_wait_event(&dev.rx_thread.event, EVENT_WAIT_FOREVER);
    rsi_reset_event(&dev.rx_thread.event);
    while (true) {
    if (atomic_read(&dev.rx_thread.thread_done))
    goto out;
    skb = skb_dequeue(&dev.rx_q);
    if (!skb)
    break;
    status = rsi_read_pkt(common, skb.data, 0);
    if (status) {
    rsi_dbg(ERR_ZONE, "%s: Failed To read data",
    __func__);
    break;
    }
    dev_kfree_skb(skb);
    }
    } while (1);
    out:
    rsi_dbg(INFO_ZONE, "%s: Terminated thread\n", __func__);
    skb_queue_purge(&dev.rx_q);
    kthread_complete_and_exit(&dev.rx_thread.completion, 0);
    }
