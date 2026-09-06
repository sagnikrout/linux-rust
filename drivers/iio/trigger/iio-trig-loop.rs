//! Automatically rewritten from C to Rust
//! Source: drivers/iio/trigger/iio-trig-loop.c
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
// Copyright 2016 Jonathan Cameron <jic23@kernel.org>
//
// Based on a mashup of the hrtimer trigger and continuous sampling proposal of
// Gregor Boirie <gregor.boirie@parrot.com>
//
// Note this is still rather experimental and may eat babies.
//
// Todo
// * Protect against connection of devices that 'need' the top half
// handler.
// * Work out how to run top half handlers in this context if it is
// safe to do so (timestamp grabbing for example)
//
// Tested against a max1363. Used about 33% cpu for the thread and 20%
// for generic_buffer piping to /dev/null. Watermark set at 64 on a 128
// element kfifo buffer.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_loop_info {
    pub swt: iio_sw_trigger,
    pub task: *mut task_struct,
}

    static const struct config_item_type iio_loop_type = {
    .ct_owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn iio_loop_thread(data: *mut c_void) -> c_int {
    static int iio_loop_thread(void *data)
    {
    struct iio_trigger *trig = data;
    set_freezable();
    do {
    iio_trigger_poll_nested(trig);
    } while (likely(!kthread_freezable_should_stop(core::ptr::null_mut())));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_loop_trigger_set_state(trig: *mut iio_trigger, state: bool) -> c_int {
    static int iio_loop_trigger_set_state(struct iio_trigger *trig, bool state)
    {
    struct iio_loop_info *loop_trig = iio_trigger_get_drvdata(trig);
    if (state) {
    loop_trig.task = kthread_run(iio_loop_thread,
    trig, trig.name);
    if (IS_ERR(loop_trig.task)) {
    dev_err(&trig.dev,
    "failed to create trigger loop thread\n");
    return PTR_ERR(loop_trig.task);
    }
    } else {
    kthread_stop(loop_trig.task);
    }
    return 0;
    }
    static const struct iio_trigger_ops iio_loop_trigger_ops = {
    .set_trigger_state = iio_loop_trigger_set_state,
    };
    static struct iio_sw_trigger *iio_trig_loop_probe(const char *name)
    {
    struct iio_loop_info *trig_info;
    int ret;
    trig_info = kzalloc_obj(*trig_info);
    if (!trig_info)
    return ERR_PTR(-ENOMEM);
    trig_info.swt.trigger = iio_trigger_alloc(core::ptr::null_mut(), "%s", name);
    if (!trig_info.swt.trigger) {
    ret = -ENOMEM;
    goto err_free_trig_info;
    }
    iio_trigger_set_drvdata(trig_info.swt.trigger, trig_info);
    trig_info.swt.trigger.ops = &iio_loop_trigger_ops;
    ret = iio_trigger_register(trig_info.swt.trigger);
    if (ret)
    goto err_free_trigger;
    iio_swt_group_init_type_name(&trig_info.swt, name, &iio_loop_type);
    return &trig_info.swt;
    err_free_trigger:
    iio_trigger_free(trig_info.swt.trigger);
    err_free_trig_info:
    kfree(trig_info);
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn iio_trig_loop_remove(swt: *mut iio_sw_trigger) -> c_int {
    static int iio_trig_loop_remove(struct iio_sw_trigger *swt)
    {
    struct iio_loop_info *trig_info;
    trig_info = iio_trigger_get_drvdata(swt.trigger);
    iio_trigger_unregister(swt.trigger);
    iio_trigger_free(swt.trigger);
    kfree(trig_info);
    return 0;
    }
    static const struct iio_sw_trigger_ops iio_trig_loop_ops = {
    .probe = iio_trig_loop_probe,
    .remove = iio_trig_loop_remove,
    };
    static struct iio_sw_trigger_type iio_trig_loop = {
    .name = "loop",
    .owner = THIS_MODULE,
    .ops = &iio_trig_loop_ops,
    };
    module_iio_sw_trigger_driver(iio_trig_loop);
    MODULE_AUTHOR("Jonathan Cameron <jic23@kernel.org>");
    MODULE_DESCRIPTION("Loop based trigger for the iio subsystem");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:iio-trig-loop");
