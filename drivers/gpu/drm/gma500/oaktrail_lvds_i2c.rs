//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/oaktrail_lvds_i2c.c
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
// Copyright (c) 2002-2010, Intel Corporation.
// Copyright (c) 2014 ATRON electronic GmbH
// Author: Jan Safrata <jan.nikitenko@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//

//
// LPC GPIO based I2C bus for LVDS of Atom E6xx
//
// -----------------------------------------------------------------------------
// LPC Register Offsets. Used for LVDS GPIO Bit Bashing. Registers are part
// Atom E6xx [D31:F0]
    ----------------------------------------------------------------------------*/
pub const RGEN: c_uint = 0x20;
pub const RGIO: c_uint = 0x24;
pub const RGLVL: c_uint = 0x28;
pub const RGTPE: c_uint = 0x2C;
pub const RGTNE: c_uint = 0x30;
pub const RGGPE: c_uint = 0x34;
pub const RGSMI: c_uint = 0x38;
pub const RGTS: c_uint = 0x3C;
// The LVDS GPIO clock lines are GPIOSUS[3]
// The LVDS GPIO data lines are GPIOSUS[4]
//
pub const GPIO_CLOCK: c_uint = 0x08;
pub const GPIO_DATA: c_uint = 0x10;

#[no_mangle]
unsafe extern "C" fn get_clock(data: *mut c_void) -> c_int {
    static int get_clock(void *data)
    {
    struct gma_i2c_chan *chan = data;
    u32 val;
    val = LPC_READ_REG(chan, RGIO);
    val |= GPIO_CLOCK;
    LPC_WRITE_REG(chan, RGIO, val);
    LPC_READ_REG(chan, RGLVL);
    val = (LPC_READ_REG(chan, RGLVL) & GPIO_CLOCK) ? 1 : 0;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn get_data(data: *mut c_void) -> c_int {
    static int get_data(void *data)
    {
    struct gma_i2c_chan *chan = data;
    u32 val;
    val = LPC_READ_REG(chan, RGIO);
    val |= GPIO_DATA;
    LPC_WRITE_REG(chan, RGIO, val);
    LPC_READ_REG(chan, RGLVL);
    val = (LPC_READ_REG(chan, RGLVL) & GPIO_DATA) ? 1 : 0;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn set_clock(data: *mut c_void, state_high: c_int) {
    static void set_clock(void *data, int state_high)
    {
    struct gma_i2c_chan *chan = data;
    u32 val;
    if (state_high) {
    val = LPC_READ_REG(chan, RGIO);
    val |= GPIO_CLOCK;
    LPC_WRITE_REG(chan, RGIO, val);
    } else {
    val = LPC_READ_REG(chan, RGIO);
    val &= ~GPIO_CLOCK;
    LPC_WRITE_REG(chan, RGIO, val);
    val = LPC_READ_REG(chan, RGLVL);
    val &= ~GPIO_CLOCK;
    LPC_WRITE_REG(chan, RGLVL, val);
    }
    }
#[no_mangle]
unsafe extern "C" fn set_data(data: *mut c_void, state_high: c_int) {
    static void set_data(void *data, int state_high)
    {
    struct gma_i2c_chan *chan = data;
    u32 val;
    if (state_high) {
    val = LPC_READ_REG(chan, RGIO);
    val |= GPIO_DATA;
    LPC_WRITE_REG(chan, RGIO, val);
    } else {
    val = LPC_READ_REG(chan, RGIO);
    val &= ~GPIO_DATA;
    LPC_WRITE_REG(chan, RGIO, val);
    val = LPC_READ_REG(chan, RGLVL);
    val &= ~GPIO_DATA;
    LPC_WRITE_REG(chan, RGLVL, val);
    }
    }
    struct gma_i2c_chan *oaktrail_lvds_i2c_init(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    struct gma_i2c_chan *chan;
    int ret;
    chan = kzalloc_obj(struct gma_i2c_chan);
    if (!chan)
    return ERR_PTR(-ENOMEM);
    chan.drm_dev = dev;
    chan.reg = dev_priv.lpc_gpio_base;
    strscpy(chan.base.name, "gma500 LPC",  sizeof(chan.base.name));
    chan.base.owner = THIS_MODULE;
    chan.base.algo_data = &chan.algo;
    chan.base.dev.parent = dev.dev;
    chan.algo.setsda = set_data;
    chan.algo.setscl = set_clock;
    chan.algo.getsda = get_data;
    chan.algo.getscl = get_clock;
    chan.algo.udelay = 100;
    chan.algo.timeout = usecs_to_jiffies(2200);
    chan.algo.data = chan;
    i2c_set_adapdata(&chan.base, chan);
    set_data(chan, 1);
    set_clock(chan, 1);
    udelay(50);
    ret = i2c_bit_add_bus(&chan.base);
    if (ret < 0) {
    kfree(chan);
    return ERR_PTR(ret);
    }
    return chan;
    }
