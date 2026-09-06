//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/matrox/i2c-matroxfb.c
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
// Hardware accelerated Matrox Millennium I, II, Mystique, G100, G200, G400 and G450.
//
// (c) 1998-2002 Petr Vandrovec <vandrove@vc.cvut.cz>
//
// Version: 1.64 2002/06/10
//
// See matroxfb_base.c for contributors.
//

// MGA-TVO I2C for G200, G400
pub const MAT_CLK: c_uint = 0x20;
pub const MAT_DATA: c_uint = 0x10;
// primary head DDC for Mystique(?), G100, G200, G400
pub const DDC1_CLK: c_uint = 0x08;
pub const DDC1_DATA: c_uint = 0x02;
// primary head DDC for Millennium, Millennium II
pub const DDC1B_CLK: c_uint = 0x10;
pub const DDC1B_DATA: c_uint = 0x04;
// secondary head DDC for G400
pub const DDC2_CLK: c_uint = 0x04;
pub const DDC2_DATA: c_uint = 0x01;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct matroxfb_dh_maven_info {
    pub maven: i2c_bit_adapter,
    pub ddc1: i2c_bit_adapter,
    pub ddc2: i2c_bit_adapter,
}

#[no_mangle]
unsafe extern "C" fn matroxfb_read_gpio(minfo: *mut *mut matrox_fb_info) -> c_int {
    unsigned long flags;
    int v;
    matroxfb_DAC_lock_irqsave(flags);
    v = matroxfb_DAC_in(minfo, DAC_XGENIODATA);
    matroxfb_DAC_unlock_irqrestore(flags);
    return v;
    }
#[no_mangle]
unsafe extern "C" fn matroxfb_set_gpio(minfo: *mut *mut matrox_fb_info, mask: c_int, val: c_int) {
    unsigned long flags;
    int v;
    matroxfb_DAC_lock_irqsave(flags);
    v = (matroxfb_DAC_in(minfo, DAC_XGENIOCTRL) & mask) | val;
    matroxfb_DAC_out(minfo, DAC_XGENIOCTRL, v);
// We must reset GENIODATA very often... XFree plays with this register
    matroxfb_DAC_out(minfo, DAC_XGENIODATA, 0x00);
    matroxfb_DAC_unlock_irqrestore(flags);
    }
// software I2C functions
#[no_mangle]
pub unsafe extern "C" fn matroxfb_i2c_set(minfo: *mut *mut matrox_fb_info, mask: c_int, state: c_int) {
    if (state)
    state = 0;
    else
    state = mask;
    matroxfb_set_gpio(minfo, ~mask, state);
    }
#[no_mangle]
unsafe extern "C" fn matroxfb_gpio_setsda(data: *mut *mut c_void, state: c_int) {
    let mut b: *mut i2c_bit_adapter = data;
    matroxfb_i2c_set(b.minfo, b.mask.data, state);
    }
#[no_mangle]
unsafe extern "C" fn matroxfb_gpio_setscl(data: *mut *mut c_void, state: c_int) {
    let mut b: *mut i2c_bit_adapter = data;
    matroxfb_i2c_set(b.minfo, b.mask.clock, state);
    }
#[no_mangle]
unsafe extern "C" fn matroxfb_gpio_getsda(data: *mut *mut c_void) -> c_int {
    let mut b: *mut i2c_bit_adapter = data;
    return (matroxfb_read_gpio(b.minfo) & b.mask.data) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn matroxfb_gpio_getscl(data: *mut *mut c_void) -> c_int {
    let mut b: *mut i2c_bit_adapter = data;
    return (matroxfb_read_gpio(b.minfo) & b.mask.clock) ? 1 : 0;
    }
    static const struct i2c_algo_bit_data matrox_i2c_algo_template =
    {
    .setsda		= matroxfb_gpio_setsda,
    .setscl		= matroxfb_gpio_setscl,
    .getsda		= matroxfb_gpio_getsda,
    .getscl		= matroxfb_gpio_getscl,
    .udelay		= 10,
    .timeout	= 100,
    };
    static int i2c_bus_reg(struct i2c_bit_adapter* b, struct matrox_fb_info* minfo,
    unsigned int data, unsigned int clock, const char *name)
    {
    int err;
    b.minfo = minfo;
    b.mask.data = data;
    b.mask.clock = clock;
    b.adapter.owner = THIS_MODULE;
    snprintf(b.adapter.name, sizeof(b.adapter.name), name,
    minfo.fbcon.node);
    i2c_set_adapdata(&b.adapter, b);
    b.adapter.algo_data = &b.bac;
    b.adapter.dev.parent = &minfo.pcidev.dev;
    b.bac = matrox_i2c_algo_template;
    b.bac.data = b;
    err = i2c_bit_add_bus(&b.adapter);
    b.initialized = !err;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn i2c_bit_bus_del(b: *mut *mut i2c_bit_adapter) {
    if (b.initialized) {
    i2c_del_adapter(&b.adapter);
    b.initialized = 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_maven_done(minfo2: *mut *mut matroxfb_dh_maven_info) {
    i2c_bit_bus_del(&minfo2.maven);
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_ddc1_done(minfo2: *mut *mut matroxfb_dh_maven_info) {
    i2c_bit_bus_del(&minfo2.ddc1);
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_ddc2_done(minfo2: *mut *mut matroxfb_dh_maven_info) {
    i2c_bit_bus_del(&minfo2.ddc2);
    }
#[no_mangle]
unsafe extern "C" fn i2c_matroxfb_probe(minfo: *mut *mut matrox_fb_info) -> *mut c_void {
    int err;
    unsigned long flags;
    struct matroxfb_dh_maven_info* m2info;
    m2info = kzalloc_obj(*m2info);
    if (!m2info)
    return core::ptr::null_mut();
    matroxfb_DAC_lock_irqsave(flags);
    matroxfb_DAC_out(minfo, DAC_XGENIODATA, 0xFF);
    matroxfb_DAC_out(minfo, DAC_XGENIOCTRL, 0x00);
    matroxfb_DAC_unlock_irqrestore(flags);
    switch (minfo.chip) {
    case MGA_2064:
    case MGA_2164:
    err = i2c_bus_reg(&m2info.ddc1, minfo,
    DDC1B_DATA, DDC1B_CLK,
    "DDC:fb%u #0");
    break;
    default:
    err = i2c_bus_reg(&m2info.ddc1, minfo,
    DDC1_DATA, DDC1_CLK,
    "DDC:fb%u #0");
    break;
    }
    if (err)
    goto fail_ddc1;
    if (minfo.devflags.dualhead) {
    err = i2c_bus_reg(&m2info.ddc2, minfo, DDC2_DATA, DDC2_CLK, "DDC:fb%u #1");
    if (err == -ENODEV) {
    printk(KERN_INFO "i2c-matroxfb: VGA.TV plug detected, DDC unavailable.\n");
    } else if (err)
    printk(KERN_INFO "i2c-matroxfb: Could not register secondary output i2c bus. Continuing anyway.\n");
// Register maven bus even on G450/G550
    err = i2c_bus_reg(&m2info.maven, minfo, MAT_DATA, MAT_CLK, "MAVEN:fb%u");
    if (err)
    printk(KERN_INFO "i2c-matroxfb: Could not register Maven i2c bus. Continuing anyway.\n");
    else {
    struct i2c_board_info maven_info = {
    I2C_BOARD_INFO("maven", 0x1b),
    };
    unsigned short const addr_list[2] = {
    0x1b, I2C_CLIENT_END
    };
    i2c_new_scanned_device(&m2info.maven.adapter,
    &maven_info, addr_list, core::ptr::null_mut());
    }
    }
    return m2info;
    fail_ddc1:;
    kfree(m2info);
    printk(KERN_ERR "i2c-matroxfb: Could not register primary adapter DDC bus.\n");
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn i2c_matroxfb_remove(minfo: *mut *mut matrox_fb_info, data: *mut *mut c_void) {
    let mut m2info: *mut matroxfb_dh_maven_info = data;
    i2c_maven_done(m2info);
    i2c_ddc2_done(m2info);
    i2c_ddc1_done(m2info);
    kfree(m2info);
    }
    static struct matroxfb_driver i2c_matroxfb = {
    .node =		LIST_HEAD_INIT(i2c_matroxfb.node),
    .name =		"i2c-matroxfb",
    .probe = 	i2c_matroxfb_probe,
    .remove =	i2c_matroxfb_remove,
    };
#[no_mangle]
unsafe extern "C" fn i2c_matroxfb_init() -> int __init {
    if (matroxfb_register_driver(&i2c_matroxfb)) {
    printk(KERN_ERR "i2c-matroxfb: failed to register driver\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_matroxfb_exit() -> void __exit {
    matroxfb_unregister_driver(&i2c_matroxfb);
    }
    MODULE_AUTHOR("(c) 1999-2002 Petr Vandrovec <vandrove@vc.cvut.cz>");
    MODULE_DESCRIPTION("Support module providing I2C buses present on Matrox videocards");
    module_init(i2c_matroxfb_init);
    module_exit(i2c_matroxfb_exit);
// no __setup required
    MODULE_LICENSE("GPL");
