//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/gma500/intel_gmbus.c
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
// Copyright (c) 2006 Dave Airlie <airlied@linux.ie>
// Copyright © 2006-2008,2010 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
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
// Eric Anholt <eric@anholt.net>
// Chris Wilson <chris@chris-wilson.co.uk>
//

    unsigned long timeout__ = jiffies + msecs_to_jiffies(MS);	\
    int ret__ = 0;							\
    while (! (COND)) {						\
    if (time_after(jiffies, timeout__)) {			\
    ret__ = -ETIMEDOUT;				\
    break;						\
    }							\
    if (W && !(in_dbg_master()))				\
    msleep(W);					\
    }								\
    ret__;								\
    })

// Intel GPIO access functions
pub const I2C_RISEFALL_TIME: c_int = 20;
    static inline struct intel_gmbus *
    to_intel_gmbus(struct i2c_adapter *i2c)
    {
    return container_of(i2c, struct intel_gmbus, adapter);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gpio {
    pub adapter: i2c_adapter,
    pub algo: i2c_algo_bit_data,
    pub dev_priv: *mut drm_psb_private,
    pub reg: u32,
}

    void
    gma_intel_i2c_reset(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    GMBUS_REG_WRITE(GMBUS0, 0);
    }
#[no_mangle]
unsafe extern "C" fn intel_i2c_quirk_set(dev_priv: *mut drm_psb_private, enable: bool) {
    static void intel_i2c_quirk_set(struct drm_psb_private *dev_priv, bool enable)
    {
// When using bit bashing for I2C, this bit needs to be set to 1
// FIXME: We are never Pineview, right?
    u32 val;
    if (!IS_PINEVIEW(dev_priv.dev))
    return;
    val = REG_READ(DSPCLK_GATE_D);
    if (enable)
    val |= DPCUNIT_CLOCK_GATE_DISABLE;
    else
    val &= ~DPCUNIT_CLOCK_GATE_DISABLE;
    REG_WRITE(DSPCLK_GATE_D, val);
    return;
//
    }
#[no_mangle]
unsafe extern "C" fn get_reserved(gpio: *mut intel_gpio) -> u32 {
    static u32 get_reserved(struct intel_gpio *gpio)
    {
    struct drm_psb_private *dev_priv = gpio.dev_priv;
    let mut reserved: u32 = 0;
// On most chips, these bits must be preserved in software.
    reserved = GMBUS_REG_READ(gpio.reg) &
    (GPIO_DATA_PULLUP_DISABLE |
    GPIO_CLOCK_PULLUP_DISABLE);
    return reserved;
    }
#[no_mangle]
unsafe extern "C" fn get_clock(data: *mut c_void) -> c_int {
    static int get_clock(void *data)
    {
    struct intel_gpio *gpio = data;
    struct drm_psb_private *dev_priv = gpio.dev_priv;
    let mut reserved: u32 = get_reserved(gpio);
    GMBUS_REG_WRITE(gpio.reg, reserved | GPIO_CLOCK_DIR_MASK);
    GMBUS_REG_WRITE(gpio.reg, reserved);
    return (GMBUS_REG_READ(gpio.reg) & GPIO_CLOCK_VAL_IN) != 0;
    }
#[no_mangle]
unsafe extern "C" fn get_data(data: *mut c_void) -> c_int {
    static int get_data(void *data)
    {
    struct intel_gpio *gpio = data;
    struct drm_psb_private *dev_priv = gpio.dev_priv;
    let mut reserved: u32 = get_reserved(gpio);
    GMBUS_REG_WRITE(gpio.reg, reserved | GPIO_DATA_DIR_MASK);
    GMBUS_REG_WRITE(gpio.reg, reserved);
    return (GMBUS_REG_READ(gpio.reg) & GPIO_DATA_VAL_IN) != 0;
    }
#[no_mangle]
unsafe extern "C" fn set_clock(data: *mut c_void, state_high: c_int) {
    static void set_clock(void *data, int state_high)
    {
    struct intel_gpio *gpio = data;
    struct drm_psb_private *dev_priv = gpio.dev_priv;
    let mut reserved: u32 = get_reserved(gpio);
    u32 clock_bits;
    if (state_high)
    clock_bits = GPIO_CLOCK_DIR_IN | GPIO_CLOCK_DIR_MASK;
    else
    clock_bits = GPIO_CLOCK_DIR_OUT | GPIO_CLOCK_DIR_MASK |
    GPIO_CLOCK_VAL_MASK;
    GMBUS_REG_WRITE(gpio.reg, reserved | clock_bits);
    GMBUS_REG_READ(gpio.reg); /* Posting */
    }
#[no_mangle]
unsafe extern "C" fn set_data(data: *mut c_void, state_high: c_int) {
    static void set_data(void *data, int state_high)
    {
    struct intel_gpio *gpio = data;
    struct drm_psb_private *dev_priv = gpio.dev_priv;
    let mut reserved: u32 = get_reserved(gpio);
    u32 data_bits;
    if (state_high)
    data_bits = GPIO_DATA_DIR_IN | GPIO_DATA_DIR_MASK;
    else
    data_bits = GPIO_DATA_DIR_OUT | GPIO_DATA_DIR_MASK |
    GPIO_DATA_VAL_MASK;
    GMBUS_REG_WRITE(gpio.reg, reserved | data_bits);
    GMBUS_REG_READ(gpio.reg);
    }
    static struct i2c_adapter *
    intel_gpio_create(struct drm_psb_private *dev_priv, u32 pin)
    {
    static const int map_pin_to_reg[] = {
    0,
    GPIOB,
    GPIOA,
    GPIOC,
    GPIOD,
    GPIOE,
    0,
    GPIOF,
    };
    struct intel_gpio *gpio;
    if (pin >= ARRAY_SIZE(map_pin_to_reg) || !map_pin_to_reg[pin])
    return core::ptr::null_mut();
    gpio = kzalloc_obj(struct intel_gpio);
    if (gpio == core::ptr::null_mut())
    return core::ptr::null_mut();
    gpio.reg = map_pin_to_reg[pin];
    gpio.dev_priv = dev_priv;
    snprintf(gpio.adapter.name, sizeof(gpio.adapter.name),
    "gma500 GPIO%c", "?BACDE?F"[pin]);
    gpio.adapter.owner = THIS_MODULE;
    gpio.adapter.algo_data	= &gpio.algo;
    gpio.adapter.dev.parent = dev_priv.dev.dev;
    gpio.algo.setsda = set_data;
    gpio.algo.setscl = set_clock;
    gpio.algo.getsda = get_data;
    gpio.algo.getscl = get_clock;
    gpio.algo.udelay = I2C_RISEFALL_TIME;
    gpio.algo.timeout = usecs_to_jiffies(2200);
    gpio.algo.data = gpio;
    if (i2c_bit_add_bus(&gpio.adapter))
    goto out_free;
    return &gpio.adapter;
    out_free:
    kfree(gpio);
    return core::ptr::null_mut();
    }
    static int
    intel_i2c_quirk_xfer(struct drm_psb_private *dev_priv,
    struct i2c_adapter *adapter,
    struct i2c_msg *msgs,
    int num)
    {
    struct intel_gpio *gpio = container_of(adapter,
    struct intel_gpio,
    adapter);
    int ret;
    gma_intel_i2c_reset(&dev_priv.dev);
    intel_i2c_quirk_set(dev_priv, true);
    set_data(gpio, 1);
    set_clock(gpio, 1);
    udelay(I2C_RISEFALL_TIME);
    ret = adapter.algo.master_xfer(adapter, msgs, num);
    set_data(gpio, 1);
    set_clock(gpio, 1);
    intel_i2c_quirk_set(dev_priv, false);
    return ret;
    }
    static int
    gmbus_xfer(struct i2c_adapter *adapter,
    struct i2c_msg *msgs,
    int num)
    {
    struct intel_gmbus *bus = container_of(adapter,
    struct intel_gmbus,
    adapter);
    struct drm_psb_private *dev_priv = adapter.algo_data;
    int i, reg_offset;
    if (bus.force_bit)
    return intel_i2c_quirk_xfer(dev_priv,
    bus.force_bit, msgs, num);
    reg_offset = 0;
    GMBUS_REG_WRITE(GMBUS0 + reg_offset, bus.reg0);
    for (i = 0; i < num; i++) {
    let mut len: u16 = msgs[i].len;
    u8 *buf = msgs[i].buf;
    if (msgs[i].flags & I2C_M_RD) {
    GMBUS_REG_WRITE(GMBUS1 + reg_offset,
    GMBUS_CYCLE_WAIT |
    (i + 1 == num ? GMBUS_CYCLE_STOP : 0) |
    (len << GMBUS_BYTE_COUNT_SHIFT) |
    (msgs[i].addr << GMBUS_SLAVE_ADDR_SHIFT) |
    GMBUS_SLAVE_READ | GMBUS_SW_RDY);
    GMBUS_REG_READ(GMBUS2+reg_offset);
    do {
    u32 val, loop = 0;
    if (wait_for(GMBUS_REG_READ(GMBUS2 + reg_offset) &
    (GMBUS_SATOER | GMBUS_HW_RDY), 50))
    goto timeout;
    if (GMBUS_REG_READ(GMBUS2 + reg_offset) & GMBUS_SATOER)
    goto clear_err;
    val = GMBUS_REG_READ(GMBUS3 + reg_offset);
    do {
// buf++ = val & 0xff;
    val >>= 8;
    } while (--len && ++loop < 4);
    } while (len);
    } else {
    u32 val, loop;
    val = loop = 0;
    do {
    val |= *buf++ << (8 * loop);
    } while (--len && ++loop < 4);
    GMBUS_REG_WRITE(GMBUS3 + reg_offset, val);
    GMBUS_REG_WRITE(GMBUS1 + reg_offset,
    (i + 1 == num ? GMBUS_CYCLE_STOP : GMBUS_CYCLE_WAIT) |
    (msgs[i].len << GMBUS_BYTE_COUNT_SHIFT) |
    (msgs[i].addr << GMBUS_SLAVE_ADDR_SHIFT) |
    GMBUS_SLAVE_WRITE | GMBUS_SW_RDY);
    GMBUS_REG_READ(GMBUS2+reg_offset);
    while (len) {
    if (wait_for(GMBUS_REG_READ(GMBUS2 + reg_offset) &
    (GMBUS_SATOER | GMBUS_HW_RDY), 50))
    goto timeout;
    if (GMBUS_REG_READ(GMBUS2 + reg_offset) &
    GMBUS_SATOER)
    goto clear_err;
    val = loop = 0;
    do {
    val |= *buf++ << (8 * loop);
    } while (--len && ++loop < 4);
    GMBUS_REG_WRITE(GMBUS3 + reg_offset, val);
    GMBUS_REG_READ(GMBUS2+reg_offset);
    }
    }
    if (i + 1 < num && wait_for(GMBUS_REG_READ(GMBUS2 + reg_offset) & (GMBUS_SATOER | GMBUS_HW_WAIT_PHASE), 50))
    goto timeout;
    if (GMBUS_REG_READ(GMBUS2 + reg_offset) & GMBUS_SATOER)
    goto clear_err;
    }
    goto done;
    clear_err:
// Toggle the Software Clear Interrupt bit. This has the effect
// of resetting the GMBUS controller and so clearing the
// BUS_ERROR raised by the target's NAK.
//
    GMBUS_REG_WRITE(GMBUS1 + reg_offset, GMBUS_SW_CLR_INT);
    GMBUS_REG_WRITE(GMBUS1 + reg_offset, 0);
    done:
// Mark the GMBUS interface as disabled. We will re-enable it at the
// start of the next xfer, till then let it sleep.
//
    GMBUS_REG_WRITE(GMBUS0 + reg_offset, 0);
    return i;
    timeout:
    DRM_INFO("GMBUS timed out, falling back to bit banging on pin %d [%s]\n",
    bus.reg0 & 0xff, bus.adapter.name);
    GMBUS_REG_WRITE(GMBUS0 + reg_offset, 0);
// Hardware may not support GMBUS over these pins? Try GPIO bitbanging instead.
    bus.force_bit = intel_gpio_create(dev_priv, bus.reg0 & 0xff);
    if (!bus.force_bit)
    return -ENOMEM;
    return intel_i2c_quirk_xfer(dev_priv, bus.force_bit, msgs, num);
    }
#[no_mangle]
unsafe extern "C" fn gmbus_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 gmbus_func(struct i2c_adapter *adapter)
    {
    struct intel_gmbus *bus = container_of(adapter,
    struct intel_gmbus,
    adapter);
    if (bus.force_bit)
    bus.force_bit.algo.functionality(bus.force_bit);
    return (I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL |
// I2C_FUNC_10BIT_ADDR |
    I2C_FUNC_SMBUS_READ_BLOCK_DATA |
    I2C_FUNC_SMBUS_BLOCK_PROC_CALL);
    }
    static const struct i2c_algorithm gmbus_algorithm = {
    .master_xfer	= gmbus_xfer,
    .functionality	= gmbus_func
    };
//
// gma_intel_setup_gmbus() - instantiate all Intel i2c GMBuses
// @dev: DRM device
//
#[no_mangle]
pub unsafe extern "C" fn gma_intel_setup_gmbus(dev: *mut drm_device) -> c_int {
    int gma_intel_setup_gmbus(struct drm_device *dev)
    {
    static const char *names[GMBUS_NUM_PORTS] = {
    "disabled",
    "ssc",
    "vga",
    "panel",
    "dpc",
    "dpb",
    "reserved",
    "dpd",
    };
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    int ret, i;
    dev_priv.gmbus = kzalloc_objs(struct intel_gmbus, GMBUS_NUM_PORTS);
    if (dev_priv.gmbus == core::ptr::null_mut())
    return -ENOMEM;
    if (IS_MRST(dev))
    dev_priv.gmbus_reg = dev_priv.aux_reg;
    else
    dev_priv.gmbus_reg = dev_priv.vdc_reg;
    for (i = 0; i < GMBUS_NUM_PORTS; i++) {
    struct intel_gmbus *bus = &dev_priv.gmbus[i];
    bus.adapter.owner = THIS_MODULE;
    snprintf(bus.adapter.name,
    sizeof(bus.adapter.name),
    "gma500 gmbus %s",
    names[i]);
    bus.adapter.dev.parent = dev.dev;
    bus.adapter.algo_data	= dev_priv;
    bus.adapter.algo = &gmbus_algorithm;
    ret = i2c_add_adapter(&bus.adapter);
    if (ret)
    goto err;
// By default use a conservative clock rate
    bus.reg0 = i | GMBUS_RATE_100KHZ;
// XXX force bit banging until GMBUS is fully debugged
    bus.force_bit = intel_gpio_create(dev_priv, i);
    }
    gma_intel_i2c_reset(&dev_priv.dev);
    return 0;
    err:
    while (i--) {
    struct intel_gmbus *bus = &dev_priv.gmbus[i];
    i2c_del_adapter(&bus.adapter);
    }
    kfree(dev_priv.gmbus);
    dev_priv.gmbus = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn gma_intel_gmbus_set_speed(adapter: *mut i2c_adapter, speed: c_int) {
    void gma_intel_gmbus_set_speed(struct i2c_adapter *adapter, int speed)
    {
    struct intel_gmbus *bus = to_intel_gmbus(adapter);
// speed:
// 0x0 = 100 KHz
// 0x1 = 50 KHz
// 0x2 = 400 KHz
// 0x3 = 1000 Khz
//
    bus.reg0 = (bus.reg0 & ~(0x3 << 8)) | (speed << 8);
    }
#[no_mangle]
pub unsafe extern "C" fn gma_intel_gmbus_force_bit(adapter: *mut i2c_adapter, force_bit: bool) {
    void gma_intel_gmbus_force_bit(struct i2c_adapter *adapter, bool force_bit)
    {
    struct intel_gmbus *bus = to_intel_gmbus(adapter);
    if (force_bit) {
    if (bus.force_bit == core::ptr::null_mut()) {
    struct drm_psb_private *dev_priv = adapter.algo_data;
    bus.force_bit = intel_gpio_create(dev_priv,
    bus.reg0 & 0xff);
    }
    } else {
    if (bus.force_bit) {
    i2c_del_adapter(bus.force_bit);
    kfree(bus.force_bit);
    bus.force_bit = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn gma_intel_teardown_gmbus(dev: *mut drm_device) {
    void gma_intel_teardown_gmbus(struct drm_device *dev)
    {
    struct drm_psb_private *dev_priv = to_drm_psb_private(dev);
    int i;
    if (dev_priv.gmbus == core::ptr::null_mut())
    return;
    for (i = 0; i < GMBUS_NUM_PORTS; i++) {
    struct intel_gmbus *bus = &dev_priv.gmbus[i];
    if (bus.force_bit) {
    i2c_del_adapter(bus.force_bit);
    kfree(bus.force_bit);
    }
    i2c_del_adapter(&bus.adapter);
    }
    dev_priv.gmbus_reg = core::ptr::null_mut(); /* iounmap is done in driver_unload */
    kfree(dev_priv.gmbus);
    dev_priv.gmbus = core::ptr::null_mut();
    }
