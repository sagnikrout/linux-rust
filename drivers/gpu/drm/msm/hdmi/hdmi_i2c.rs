//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/hdmi/hdmi_i2c.c
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_i2c_adapter {
    pub base: i2c_adapter,
    pub hdmi: *mut hdmi,
    pub sw_done: bool,
    pub ddc_event: wait_queue_head_t,
}

#[no_mangle]
unsafe extern "C" fn init_ddc(hdmi_i2c: *mut hdmi_i2c_adapter) {
    static void init_ddc(struct hdmi_i2c_adapter *hdmi_i2c)
    {
    struct hdmi *hdmi = hdmi_i2c.hdmi;
    hdmi_write(hdmi, REG_HDMI_DDC_CTRL,
    HDMI_DDC_CTRL_SW_STATUS_RESET);
    hdmi_write(hdmi, REG_HDMI_DDC_CTRL,
    HDMI_DDC_CTRL_SOFT_RESET);
    hdmi_write(hdmi, REG_HDMI_DDC_SPEED,
    HDMI_DDC_SPEED_THRESHOLD(2) |
    HDMI_DDC_SPEED_PRESCALE(10));
    hdmi_write(hdmi, REG_HDMI_DDC_SETUP,
    HDMI_DDC_SETUP_TIMEOUT(0xff));
// enable reference timer for 27us
    hdmi_write(hdmi, REG_HDMI_DDC_REF,
    HDMI_DDC_REF_REFTIMER_ENABLE |
    HDMI_DDC_REF_REFTIMER(27));
    }
#[no_mangle]
unsafe extern "C" fn ddc_clear_irq(hdmi_i2c: *mut hdmi_i2c_adapter) -> c_int {
    static int ddc_clear_irq(struct hdmi_i2c_adapter *hdmi_i2c)
    {
    struct hdmi *hdmi = hdmi_i2c.hdmi;
    struct drm_device *dev = hdmi.dev;
    let mut retry: u32 = 0xffff;
    u32 ddc_int_ctrl;
    do {
    --retry;
    hdmi_write(hdmi, REG_HDMI_DDC_INT_CTRL,
    HDMI_DDC_INT_CTRL_SW_DONE_ACK |
    HDMI_DDC_INT_CTRL_SW_DONE_MASK);
    ddc_int_ctrl = hdmi_read(hdmi, REG_HDMI_DDC_INT_CTRL);
    } while ((ddc_int_ctrl & HDMI_DDC_INT_CTRL_SW_DONE_INT) && retry);
    if (!retry) {
    DRM_DEV_ERROR(dev.dev, "timeout waiting for DDC\n");
    return -ETIMEDOUT;
    }
    hdmi_i2c.sw_done = false;
    return 0;
    }
pub const MAX_TRANSACTIONS: c_int = 4;
#[no_mangle]
unsafe extern "C" fn sw_done(hdmi_i2c: *mut hdmi_i2c_adapter) -> bool {
    static bool sw_done(struct hdmi_i2c_adapter *hdmi_i2c)
    {
    struct hdmi *hdmi = hdmi_i2c.hdmi;
    if (!hdmi_i2c.sw_done) {
    u32 ddc_int_ctrl;
    ddc_int_ctrl = hdmi_read(hdmi, REG_HDMI_DDC_INT_CTRL);
    if ((ddc_int_ctrl & HDMI_DDC_INT_CTRL_SW_DONE_MASK) &&
    (ddc_int_ctrl & HDMI_DDC_INT_CTRL_SW_DONE_INT)) {
    hdmi_i2c.sw_done = true;
    hdmi_write(hdmi, REG_HDMI_DDC_INT_CTRL,
    HDMI_DDC_INT_CTRL_SW_DONE_ACK);
    }
    }
    return hdmi_i2c.sw_done;
    }
    static int msm_hdmi_i2c_xfer(struct i2c_adapter *i2c,
    struct i2c_msg *msgs, int num)
    {
    struct hdmi_i2c_adapter *hdmi_i2c = to_hdmi_i2c_adapter(i2c);
    struct hdmi *hdmi = hdmi_i2c.hdmi;
    struct drm_device *dev = hdmi.dev;
    static const u32 nack[] = {
    HDMI_DDC_SW_STATUS_NACK0, HDMI_DDC_SW_STATUS_NACK1,
    HDMI_DDC_SW_STATUS_NACK2, HDMI_DDC_SW_STATUS_NACK3,
    };
    int indices[MAX_TRANSACTIONS];
    int ret, i, j, index = 0;
    u32 ddc_status, ddc_data, i2c_trans;
    num = min(num, MAX_TRANSACTIONS);
    WARN_ON(!(hdmi_read(hdmi, REG_HDMI_CTRL) & HDMI_CTRL_ENABLE));
    if (num == 0)
    return num;
    ret = pm_runtime_resume_and_get(&hdmi.pdev.dev);
    if (ret)
    return ret;
    init_ddc(hdmi_i2c);
    ret = ddc_clear_irq(hdmi_i2c);
    if (ret)
    goto fail;
    for (i = 0; i < num; i++) {
    struct i2c_msg *p = &msgs[i];
    let mut raw_addr: u32 = p.addr << 1;
    if (p.flags & I2C_M_RD)
    raw_addr |= 1;
    ddc_data = HDMI_DDC_DATA_DATA(raw_addr) |
    HDMI_DDC_DATA_DATA_RW(DDC_WRITE);
    if (i == 0) {
    ddc_data |= HDMI_DDC_DATA_INDEX(0) |
    HDMI_DDC_DATA_INDEX_WRITE;
    }
    hdmi_write(hdmi, REG_HDMI_DDC_DATA, ddc_data);
    index++;
    indices[i] = index;
    if (p.flags & I2C_M_RD) {
    index += p.len;
    } else {
    for (j = 0; j < p.len; j++) {
    ddc_data = HDMI_DDC_DATA_DATA(p.buf[j]) |
    HDMI_DDC_DATA_DATA_RW(DDC_WRITE);
    hdmi_write(hdmi, REG_HDMI_DDC_DATA, ddc_data);
    index++;
    }
    }
    i2c_trans = HDMI_I2C_TRANSACTION_REG_CNT(p.len) |
    HDMI_I2C_TRANSACTION_REG_RW(
    (p.flags & I2C_M_RD) ? DDC_READ : DDC_WRITE) |
    HDMI_I2C_TRANSACTION_REG_START;
    if (i == (num - 1))
    i2c_trans |= HDMI_I2C_TRANSACTION_REG_STOP;
    hdmi_write(hdmi, REG_HDMI_I2C_TRANSACTION(i), i2c_trans);
    }
// trigger the transfer:
    hdmi_write(hdmi, REG_HDMI_DDC_CTRL,
    HDMI_DDC_CTRL_TRANSACTION_CNT(num - 1) |
    HDMI_DDC_CTRL_GO);
    ret = wait_event_timeout(hdmi_i2c.ddc_event, sw_done(hdmi_i2c), HZ/4);
    if (ret <= 0) {
    if (ret == 0)
    ret = -ETIMEDOUT;
    dev_warn(dev.dev, "DDC timeout: %d\n", ret);
    DBG("sw_status=%08x, hw_status=%08x, int_ctrl=%08x",
    hdmi_read(hdmi, REG_HDMI_DDC_SW_STATUS),
    hdmi_read(hdmi, REG_HDMI_DDC_HW_STATUS),
    hdmi_read(hdmi, REG_HDMI_DDC_INT_CTRL));
    goto fail;
    }
    ddc_status = hdmi_read(hdmi, REG_HDMI_DDC_SW_STATUS);
// read back results of any read transactions:
    for (i = 0; i < num; i++) {
    struct i2c_msg *p = &msgs[i];
    if (!(p.flags & I2C_M_RD))
    continue;
// check for NACK:
    if (ddc_status & nack[i]) {
    DBG("ddc_status=%08x", ddc_status);
    break;
    }
    ddc_data = HDMI_DDC_DATA_DATA_RW(DDC_READ) |
    HDMI_DDC_DATA_INDEX(indices[i]) |
    HDMI_DDC_DATA_INDEX_WRITE;
    hdmi_write(hdmi, REG_HDMI_DDC_DATA, ddc_data);
// discard first byte:
    hdmi_read(hdmi, REG_HDMI_DDC_DATA);
    for (j = 0; j < p.len; j++) {
    ddc_data = hdmi_read(hdmi, REG_HDMI_DDC_DATA);
    p.buf[j] = FIELD(ddc_data, HDMI_DDC_DATA_DATA);
    }
    }
    pm_runtime_put(&hdmi.pdev.dev);
    return i;
    fail:
    pm_runtime_put(&hdmi.pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn msm_hdmi_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 msm_hdmi_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm msm_hdmi_i2c_algorithm = {
    .master_xfer	= msm_hdmi_i2c_xfer,
    .functionality	= msm_hdmi_i2c_func,
    };
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_i2c_irq(i2c: *mut i2c_adapter) {
    void msm_hdmi_i2c_irq(struct i2c_adapter *i2c)
    {
    struct hdmi_i2c_adapter *hdmi_i2c = to_hdmi_i2c_adapter(i2c);
    if (sw_done(hdmi_i2c))
    wake_up_all(&hdmi_i2c.ddc_event);
    }
#[no_mangle]
pub unsafe extern "C" fn msm_hdmi_i2c_destroy(i2c: *mut i2c_adapter) {
    void msm_hdmi_i2c_destroy(struct i2c_adapter *i2c)
    {
    struct hdmi_i2c_adapter *hdmi_i2c = to_hdmi_i2c_adapter(i2c);
    i2c_del_adapter(i2c);
    kfree(hdmi_i2c);
    }
    struct i2c_adapter *msm_hdmi_i2c_init(struct hdmi *hdmi)
    {
    struct hdmi_i2c_adapter *hdmi_i2c;
    struct i2c_adapter *i2c = core::ptr::null_mut();
    int ret;
    hdmi_i2c = kzalloc_obj(*hdmi_i2c);
    if (!hdmi_i2c) {
    ret = -ENOMEM;
    goto fail;
    }
    i2c = &hdmi_i2c.base;
    hdmi_i2c.hdmi = hdmi;
    init_waitqueue_head(&hdmi_i2c.ddc_event);
    i2c.owner = THIS_MODULE;
    snprintf(i2c.name, sizeof(i2c.name), "msm hdmi i2c");
    i2c.dev.parent = &hdmi.pdev.dev;
    i2c.algo = &msm_hdmi_i2c_algorithm;
    ret = i2c_add_adapter(i2c);
    if (ret)
    goto fail;
    return i2c;
    fail:
    if (i2c)
    msm_hdmi_i2c_destroy(i2c);
    return ERR_PTR(ret);
    }
