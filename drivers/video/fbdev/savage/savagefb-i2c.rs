//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/savage/savagefb-i2c.c
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
// linux/drivers/video/savage/savagefb-i2c.c - S3 Savage DDC2
//
// Copyright 2004 Antonino A. Daplas <adaplas @pol.net>
//
// Based partly on rivafb-i2c.c
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

pub const SAVAGE_DDC: c_uint = 0x50;
pub const VGA_CR_IX: c_uint = 0x3d4;
pub const VGA_CR_DATA: c_uint = 0x3d5;
pub const CR_SERIAL1: c_uint = 0xa0	/* I2C serial communications interface */;
pub const MM_SERIAL1: c_uint = 0xff20;
pub const CR_SERIAL2: c_uint = 0xb1	/* DDC2 monitor communications interface */;
// based on vt8365 documentation
pub const PROSAVAGE_I2C_ENAB: c_uint = 0x10;
pub const PROSAVAGE_I2C_SCL_OUT: c_uint = 0x01;
pub const PROSAVAGE_I2C_SDA_OUT: c_uint = 0x02;
pub const PROSAVAGE_I2C_SCL_IN: c_uint = 0x04;
pub const PROSAVAGE_I2C_SDA_IN: c_uint = 0x08;
pub const SAVAGE4_I2C_ENAB: c_uint = 0x00000020;
pub const SAVAGE4_I2C_SCL_OUT: c_uint = 0x00000001;
pub const SAVAGE4_I2C_SDA_OUT: c_uint = 0x00000002;
pub const SAVAGE4_I2C_SCL_IN: c_uint = 0x00000008;
pub const SAVAGE4_I2C_SDA_IN: c_uint = 0x00000010;
#[no_mangle]
unsafe extern "C" fn savage4_gpio_setscl(data: *mut c_void, val: c_int) {
    static void savage4_gpio_setscl(void *data, int val)
    {
    struct savagefb_i2c_chan *chan = data;
    unsigned int r;
    r = readl(chan.ioaddr + chan.reg);
    if(val)
    r |= SAVAGE4_I2C_SCL_OUT;
    else
    r &= ~SAVAGE4_I2C_SCL_OUT;
    writel(r, chan.ioaddr + chan.reg);
    readl(chan.ioaddr + chan.reg);	/* flush posted write */
    }
#[no_mangle]
unsafe extern "C" fn savage4_gpio_setsda(data: *mut c_void, val: c_int) {
    static void savage4_gpio_setsda(void *data, int val)
    {
    struct savagefb_i2c_chan *chan = data;
    unsigned int r;
    r = readl(chan.ioaddr + chan.reg);
    if(val)
    r |= SAVAGE4_I2C_SDA_OUT;
    else
    r &= ~SAVAGE4_I2C_SDA_OUT;
    writel(r, chan.ioaddr + chan.reg);
    readl(chan.ioaddr + chan.reg);	/* flush posted write */
    }
#[no_mangle]
unsafe extern "C" fn savage4_gpio_getscl(data: *mut c_void) -> c_int {
    static int savage4_gpio_getscl(void *data)
    {
    struct savagefb_i2c_chan *chan = data;
    return (0 != (readl(chan.ioaddr + chan.reg) & SAVAGE4_I2C_SCL_IN));
    }
#[no_mangle]
unsafe extern "C" fn savage4_gpio_getsda(data: *mut c_void) -> c_int {
    static int savage4_gpio_getsda(void *data)
    {
    struct savagefb_i2c_chan *chan = data;
    return (0 != (readl(chan.ioaddr + chan.reg) & SAVAGE4_I2C_SDA_IN));
    }
#[no_mangle]
unsafe extern "C" fn prosavage_gpio_setscl(data: *mut *mut c_void, val: c_int) {
    static void prosavage_gpio_setscl(void* data, int val)
    {
    struct savagefb_i2c_chan *chan = data;
    u32			  r;
    r = VGArCR(chan.reg, chan.par);
    r |= PROSAVAGE_I2C_ENAB;
    if (val) {
    r |= PROSAVAGE_I2C_SCL_OUT;
    } else {
    r &= ~PROSAVAGE_I2C_SCL_OUT;
    }
    VGAwCR(chan.reg, r, chan.par);
    }
#[no_mangle]
unsafe extern "C" fn prosavage_gpio_setsda(data: *mut *mut c_void, val: c_int) {
    static void prosavage_gpio_setsda(void* data, int val)
    {
    struct savagefb_i2c_chan *chan = data;
    unsigned int r;
    r = VGArCR(chan.reg, chan.par);
    r |= PROSAVAGE_I2C_ENAB;
    if (val) {
    r |= PROSAVAGE_I2C_SDA_OUT;
    } else {
    r &= ~PROSAVAGE_I2C_SDA_OUT;
    }
    VGAwCR(chan.reg, r, chan.par);
    }
#[no_mangle]
unsafe extern "C" fn prosavage_gpio_getscl(data: *mut *mut c_void) -> c_int {
    static int prosavage_gpio_getscl(void* data)
    {
    struct savagefb_i2c_chan *chan = data;
    return (VGArCR(chan.reg, chan.par) & PROSAVAGE_I2C_SCL_IN) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn prosavage_gpio_getsda(data: *mut *mut c_void) -> c_int {
    static int prosavage_gpio_getsda(void* data)
    {
    struct savagefb_i2c_chan *chan = data;
    return (VGArCR(chan.reg, chan.par) & PROSAVAGE_I2C_SDA_IN) ? 1 : 0;
    }
    static int savage_setup_i2c_bus(struct savagefb_i2c_chan *chan,
    const char *name)
    {
    let mut rc: c_int = 0;
    if (chan.par) {
    strcpy(chan.adapter.name, name);
    chan.adapter.owner		= THIS_MODULE;
    chan.adapter.algo_data		= &chan.algo;
    chan.adapter.dev.parent	= &chan.par.pcidev.dev;
    chan.algo.udelay		= 10;
    chan.algo.timeout		= 20;
    chan.algo.data 		= chan;
    i2c_set_adapdata(&chan.adapter, chan);
// Raise SCL and SDA
    chan.algo.setsda(chan, 1);
    chan.algo.setscl(chan, 1);
    udelay(20);
    rc = i2c_bit_add_bus(&chan.adapter);
    if (rc == 0)
    dev_dbg(&chan.par.pcidev.dev,
    "I2C bus %s registered.\n", name);
    else
    dev_warn(&chan.par.pcidev.dev,
    "Failed to register I2C bus %s.\n", name);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn savagefb_create_i2c_busses(info: *mut fb_info) {
    void savagefb_create_i2c_busses(struct fb_info *info)
    {
    struct savagefb_par *par = info.par;
    par.chan.par	= par;
    switch (par.chip) {
    case S3_PROSAVAGE:
    case S3_PROSAVAGEDDR:
    case S3_TWISTER:
    par.chan.reg         = CR_SERIAL2;
    par.chan.ioaddr      = par.mmio.vbase;
    par.chan.algo.setsda = prosavage_gpio_setsda;
    par.chan.algo.setscl = prosavage_gpio_setscl;
    par.chan.algo.getsda = prosavage_gpio_getsda;
    par.chan.algo.getscl = prosavage_gpio_getscl;
    break;
    case S3_SAVAGE4:
    par.chan.reg = CR_SERIAL1;
    if (par.pcidev.revision > 1 && !(VGArCR(0xa6, par) & 0x40))
    par.chan.reg = CR_SERIAL2;
    par.chan.ioaddr      = par.mmio.vbase;
    par.chan.algo.setsda = prosavage_gpio_setsda;
    par.chan.algo.setscl = prosavage_gpio_setscl;
    par.chan.algo.getsda = prosavage_gpio_getsda;
    par.chan.algo.getscl = prosavage_gpio_getscl;
    break;
    case S3_SAVAGE2000:
    par.chan.reg         = MM_SERIAL1;
    par.chan.ioaddr      = par.mmio.vbase;
    par.chan.algo.setsda = savage4_gpio_setsda;
    par.chan.algo.setscl = savage4_gpio_setscl;
    par.chan.algo.getsda = savage4_gpio_getsda;
    par.chan.algo.getscl = savage4_gpio_getscl;
    break;
    default:
    par.chan.par = core::ptr::null_mut();
    }
    savage_setup_i2c_bus(&par.chan, "SAVAGE DDC2");
    }
#[no_mangle]
pub unsafe extern "C" fn savagefb_delete_i2c_busses(info: *mut fb_info) {
    void savagefb_delete_i2c_busses(struct fb_info *info)
    {
    struct savagefb_par *par = info.par;
    if (par.chan.par)
    i2c_del_adapter(&par.chan.adapter);
    par.chan.par = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn savagefb_probe_i2c_connector(info: *mut fb_info, out_edid: *mut u8) -> c_int {
    int savagefb_probe_i2c_connector(struct fb_info *info, u8 **out_edid)
    {
    struct savagefb_par *par = info.par;
    u8 *edid;
    if (par.chan.par)
    edid = fb_ddc_read(&par.chan.adapter);
    else
    edid = core::ptr::null_mut();
    if (!edid) {
// try to get from firmware
    const u8 *e = fb_firmware_edid(info.device);
    if (e)
    edid = kmemdup(e, EDID_LENGTH, GFP_KERNEL);
    }
// out_edid = edid;
    return (edid) ? 0 : 1;
    }
    MODULE_LICENSE("GPL");
