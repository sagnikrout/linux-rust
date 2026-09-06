//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/oaktrail_hdmi_i2c.c
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
// Copyright © 2010 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//
// Authors:
// Li Peng <peng.li@intel.com>
//

pub const HDMI_HCR: c_uint = 0x1000;

pub const HDMI_HICR: c_uint = 0x1004;

pub const HDMI_HSR: c_uint = 0x1008;
pub const HDMI_HISR: c_uint = 0x100C;
pub const HDMI_HI2CRDB0: c_uint = 0x1200;
pub const HDMI_HI2CHCR: c_uint = 0x1240;

pub const HDMI_ICRH: c_uint = 0x1100;
pub const HDMI_HI2CTDR0: c_uint = 0x1244;
pub const HDMI_HI2CTDR1: c_uint = 0x1248;
pub const I2C_STAT_INIT: c_int = 0;
pub const I2C_READ_DONE: c_int = 1;
pub const I2C_TRANSACTION_DONE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_i2c_dev {
    pub adap: *mut i2c_adapter,
    pub i2c_lock: mutex,
    pub complete: completion,
    pub status: c_int,
    pub msg: *mut i2c_msg,
    pub buf_offset: c_int,
}

#[no_mangle]
unsafe extern "C" fn hdmi_i2c_irq_enable(hdmi_dev: *mut oaktrail_hdmi_dev) {
    static void hdmi_i2c_irq_enable(struct oaktrail_hdmi_dev *hdmi_dev)
    {
    u32 temp;
    temp = HDMI_READ(HDMI_HICR);
    temp |= (HDMI_INTR_I2C_ERROR | HDMI_INTR_I2C_FULL | HDMI_INTR_I2C_DONE);
    HDMI_WRITE(HDMI_HICR, temp);
    HDMI_READ(HDMI_HICR);
    }
#[no_mangle]
unsafe extern "C" fn hdmi_i2c_irq_disable(hdmi_dev: *mut oaktrail_hdmi_dev) {
    static void hdmi_i2c_irq_disable(struct oaktrail_hdmi_dev *hdmi_dev)
    {
    HDMI_WRITE(HDMI_HICR, 0x0);
    HDMI_READ(HDMI_HICR);
    }
#[no_mangle]
unsafe extern "C" fn xfer_read(adap: *mut i2c_adapter, pmsg: *mut i2c_msg) -> c_int {
    static int xfer_read(struct i2c_adapter *adap, struct i2c_msg *pmsg)
    {
    struct oaktrail_hdmi_dev *hdmi_dev = i2c_get_adapdata(adap);
    struct hdmi_i2c_dev *i2c_dev = hdmi_dev.i2c_dev;
    u32 temp;
    int ret;
    i2c_dev.status = I2C_STAT_INIT;
    i2c_dev.msg = pmsg;
    i2c_dev.buf_offset = 0;
    reinit_completion(&i2c_dev.complete);
// Enable I2C transaction
    temp = ((pmsg.len) << 20) | HI2C_EDID_READ | HI2C_ENABLE_TRANSACTION;
    HDMI_WRITE(HDMI_HI2CHCR, temp);
    HDMI_READ(HDMI_HI2CHCR);
    while (i2c_dev.status != I2C_TRANSACTION_DONE) {
    ret = wait_for_completion_interruptible_timeout(&i2c_dev.complete,
    10 * HZ);
    if (ret < 0)
    return ret;
    if (!ret)
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xfer_write(adap: *mut i2c_adapter, pmsg: *mut i2c_msg) -> c_int {
    static int xfer_write(struct i2c_adapter *adap, struct i2c_msg *pmsg)
    {
//
// XXX: i2c write seems isn't useful for EDID probe, don't do anything
//
    return 0;
    }
    static int oaktrail_hdmi_i2c_access(struct i2c_adapter *adap,
    struct i2c_msg *pmsg,
    int num)
    {
    struct oaktrail_hdmi_dev *hdmi_dev = i2c_get_adapdata(adap);
    struct hdmi_i2c_dev *i2c_dev = hdmi_dev.i2c_dev;
    int i, ret = 0;
    mutex_lock(&i2c_dev.i2c_lock);
// Enable i2c unit
    HDMI_WRITE(HDMI_ICRH, 0x00008760);
// Enable irq
    hdmi_i2c_irq_enable(hdmi_dev);
    for (i = 0; i < num; i++) {
    if (pmsg.len && pmsg.buf) {
    if (pmsg.flags & I2C_M_RD)
    ret = xfer_read(adap, pmsg);
    else
    ret = xfer_write(adap, pmsg);
    if (ret)
    break;
    }
    pmsg++;         /* next message */
    }
// Disable irq
    hdmi_i2c_irq_disable(hdmi_dev);
    mutex_unlock(&i2c_dev.i2c_lock);
    if (ret)
    return ret;
    return i;
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_hdmi_i2c_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 oaktrail_hdmi_i2c_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_10BIT_ADDR;
    }
    static const struct i2c_algorithm oaktrail_hdmi_i2c_algorithm = {
    .master_xfer	= oaktrail_hdmi_i2c_access,
    .functionality  = oaktrail_hdmi_i2c_func,
    };
    static struct i2c_adapter oaktrail_hdmi_i2c_adapter = {
    .name		= "oaktrail_hdmi_i2c",
    .nr		= 3,
    .owner		= THIS_MODULE,
    .algo		= &oaktrail_hdmi_i2c_algorithm,
    };
#[no_mangle]
unsafe extern "C" fn hdmi_i2c_read(hdmi_dev: *mut oaktrail_hdmi_dev) {
    static void hdmi_i2c_read(struct oaktrail_hdmi_dev *hdmi_dev)
    {
    struct hdmi_i2c_dev *i2c_dev = hdmi_dev.i2c_dev;
    struct i2c_msg *msg = i2c_dev.msg;
    u8 *buf = msg.buf;
    u32 temp;
    int i, offset;
    offset = i2c_dev.buf_offset;
    for (i = 0; i < 0x10; i++) {
    temp = HDMI_READ(HDMI_HI2CRDB0 + (i * 4));
    memcpy(buf + (offset + i * 4), &temp, 4);
    }
    i2c_dev.buf_offset += (0x10 * 4);
// clearing read buffer full intr
    temp = HDMI_READ(HDMI_HISR);
    HDMI_WRITE(HDMI_HISR, temp | HDMI_INTR_I2C_FULL);
    HDMI_READ(HDMI_HISR);
// continue read transaction
    temp = HDMI_READ(HDMI_HI2CHCR);
    HDMI_WRITE(HDMI_HI2CHCR, temp | HI2C_READ_CONTINUE);
    HDMI_READ(HDMI_HI2CHCR);
    i2c_dev.status = I2C_READ_DONE;
    return;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_i2c_transaction_done(hdmi_dev: *mut oaktrail_hdmi_dev) {
    static void hdmi_i2c_transaction_done(struct oaktrail_hdmi_dev *hdmi_dev)
    {
    struct hdmi_i2c_dev *i2c_dev = hdmi_dev.i2c_dev;
    u32 temp;
// clear transaction done intr
    temp = HDMI_READ(HDMI_HISR);
    HDMI_WRITE(HDMI_HISR, temp | HDMI_INTR_I2C_DONE);
    HDMI_READ(HDMI_HISR);
    temp = HDMI_READ(HDMI_HI2CHCR);
    HDMI_WRITE(HDMI_HI2CHCR, temp & ~HI2C_ENABLE_TRANSACTION);
    HDMI_READ(HDMI_HI2CHCR);
    i2c_dev.status = I2C_TRANSACTION_DONE;
    return;
    }
#[no_mangle]
unsafe extern "C" fn oaktrail_hdmi_i2c_handler(this_irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t oaktrail_hdmi_i2c_handler(int this_irq, void *dev)
    {
    struct oaktrail_hdmi_dev *hdmi_dev = dev;
    struct hdmi_i2c_dev *i2c_dev = hdmi_dev.i2c_dev;
    u32 stat;
    stat = HDMI_READ(HDMI_HISR);
    if (stat & HDMI_INTR_HPD) {
    HDMI_WRITE(HDMI_HISR, stat | HDMI_INTR_HPD);
    HDMI_READ(HDMI_HISR);
    }
    if (stat & HDMI_INTR_I2C_FULL)
    hdmi_i2c_read(hdmi_dev);
    if (stat & HDMI_INTR_I2C_DONE)
    hdmi_i2c_transaction_done(hdmi_dev);
    complete(&i2c_dev.complete);
    return IRQ_HANDLED;
    }
//
// choose alternate function 2 of GPIO pin 52, 53,
// which is used by HDMI I2C logic
//
#[no_mangle]
unsafe extern "C" fn oaktrail_hdmi_i2c_gpio_fix() {
    static void oaktrail_hdmi_i2c_gpio_fix(void)
    {
    void __iomem *base;
    let mut gpio_base: c_uint = 0xff12c000;
    let mut gpio_len: c_int = 0x1000;
    u32 temp;
    base = ioremap((resource_size_t)gpio_base, gpio_len);
    if (base == core::ptr::null_mut()) {
    DRM_ERROR("gpio ioremap fail\n");
    return;
    }
    temp = readl(base + 0x44);
    DRM_DEBUG_DRIVER("old gpio val %x\n", temp);
    writel((temp | 0x00000a00), (base +  0x44));
    temp = readl(base + 0x44);
    DRM_DEBUG_DRIVER("new gpio val %x\n", temp);
    iounmap(base);
    }
#[no_mangle]
pub unsafe extern "C" fn oaktrail_hdmi_i2c_init(dev: *mut pci_dev) -> c_int {
    int oaktrail_hdmi_i2c_init(struct pci_dev *dev)
    {
    struct oaktrail_hdmi_dev *hdmi_dev;
    struct hdmi_i2c_dev *i2c_dev;
    int ret;
    hdmi_dev = pci_get_drvdata(dev);
    i2c_dev = kzalloc_obj(struct hdmi_i2c_dev);
    if (!i2c_dev)
    return -ENOMEM;
    i2c_dev.adap = &oaktrail_hdmi_i2c_adapter;
    i2c_dev.status = I2C_STAT_INIT;
    init_completion(&i2c_dev.complete);
    mutex_init(&i2c_dev.i2c_lock);
    i2c_set_adapdata(&oaktrail_hdmi_i2c_adapter, hdmi_dev);
    hdmi_dev.i2c_dev = i2c_dev;
// Enable HDMI I2C function on gpio
    oaktrail_hdmi_i2c_gpio_fix();
// request irq
    ret = request_irq(dev.irq, oaktrail_hdmi_i2c_handler, IRQF_SHARED,
    oaktrail_hdmi_i2c_adapter.name, hdmi_dev);
    if (ret) {
    DRM_ERROR("Failed to request IRQ for I2C controller\n");
    goto free_dev;
    }
// Adapter registration
    ret = i2c_add_numbered_adapter(&oaktrail_hdmi_i2c_adapter);
    if (ret) {
    DRM_ERROR("Failed to add I2C adapter\n");
    goto free_irq;
    }
    return 0;
    free_irq:
    free_irq(dev.irq, hdmi_dev);
    free_dev:
    kfree(i2c_dev);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn oaktrail_hdmi_i2c_exit(dev: *mut pci_dev) {
    void oaktrail_hdmi_i2c_exit(struct pci_dev *dev)
    {
    struct oaktrail_hdmi_dev *hdmi_dev;
    struct hdmi_i2c_dev *i2c_dev;
    hdmi_dev = pci_get_drvdata(dev);
    i2c_del_adapter(&oaktrail_hdmi_i2c_adapter);
    i2c_dev = hdmi_dev.i2c_dev;
    kfree(i2c_dev);
    free_irq(dev.irq, hdmi_dev);
    }
