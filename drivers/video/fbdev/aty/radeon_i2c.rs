//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/aty/radeon_i2c.c
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

#[no_mangle]
unsafe extern "C" fn radeon_gpio_setscl(data: *mut *mut c_void, state: c_int) {
    static void radeon_gpio_setscl(void* data, int state)
    {
    struct radeon_i2c_chan 	*chan = data;
    struct radeonfb_info	*rinfo = chan.rinfo;
    u32			val;
    val = INREG(chan.ddc_reg) & ~(VGA_DDC_CLK_OUT_EN);
    if (!state)
    val |= VGA_DDC_CLK_OUT_EN;
    OUTREG(chan.ddc_reg, val);
    (void)INREG(chan.ddc_reg);
    }
#[no_mangle]
unsafe extern "C" fn radeon_gpio_setsda(data: *mut *mut c_void, state: c_int) {
    static void radeon_gpio_setsda(void* data, int state)
    {
    struct radeon_i2c_chan 	*chan = data;
    struct radeonfb_info	*rinfo = chan.rinfo;
    u32			val;
    val = INREG(chan.ddc_reg) & ~(VGA_DDC_DATA_OUT_EN);
    if (!state)
    val |= VGA_DDC_DATA_OUT_EN;
    OUTREG(chan.ddc_reg, val);
    (void)INREG(chan.ddc_reg);
    }
#[no_mangle]
unsafe extern "C" fn radeon_gpio_getscl(data: *mut *mut c_void) -> c_int {
    static int radeon_gpio_getscl(void* data)
    {
    struct radeon_i2c_chan 	*chan = data;
    struct radeonfb_info	*rinfo = chan.rinfo;
    u32			val;
    val = INREG(chan.ddc_reg);
    return (val & VGA_DDC_CLK_INPUT) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn radeon_gpio_getsda(data: *mut *mut c_void) -> c_int {
    static int radeon_gpio_getsda(void* data)
    {
    struct radeon_i2c_chan 	*chan = data;
    struct radeonfb_info	*rinfo = chan.rinfo;
    u32			val;
    val = INREG(chan.ddc_reg);
    return (val & VGA_DDC_DATA_INPUT) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn radeon_setup_i2c_bus(chan: *mut radeon_i2c_chan, name: *const c_char) -> c_int {
    static int radeon_setup_i2c_bus(struct radeon_i2c_chan *chan, const char *name)
    {
    int rc;
    snprintf(chan.adapter.name, sizeof(chan.adapter.name),
    "radeonfb %s", name);
    chan.adapter.owner		= THIS_MODULE;
    chan.adapter.algo_data		= &chan.algo;
    chan.adapter.dev.parent	= &chan.rinfo.pdev.dev;
    chan.algo.setsda		= radeon_gpio_setsda;
    chan.algo.setscl		= radeon_gpio_setscl;
    chan.algo.getsda		= radeon_gpio_getsda;
    chan.algo.getscl		= radeon_gpio_getscl;
    chan.algo.udelay		= 10;
    chan.algo.timeout		= 20;
    chan.algo.data 		= chan;
    i2c_set_adapdata(&chan.adapter, chan);
// Raise SCL and SDA
    radeon_gpio_setsda(chan, 1);
    radeon_gpio_setscl(chan, 1);
    udelay(20);
    rc = i2c_bit_add_bus(&chan.adapter);
    if (rc == 0)
    dev_dbg(&chan.rinfo.pdev.dev, "I2C bus %s registered.\n", name);
    else
    dev_warn(&chan.rinfo.pdev.dev, "Failed to register I2C bus %s.\n", name);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn radeon_create_i2c_busses(rinfo: *mut radeonfb_info) {
    void radeon_create_i2c_busses(struct radeonfb_info *rinfo)
    {
    rinfo.i2c[0].rinfo	= rinfo;
    rinfo.i2c[0].ddc_reg	= GPIO_MONID;

    rinfo.i2c[0].adapter.class = I2C_CLASS_HWMON;

    radeon_setup_i2c_bus(&rinfo.i2c[0], "monid");
    rinfo.i2c[1].rinfo	= rinfo;
    rinfo.i2c[1].ddc_reg	= GPIO_DVI_DDC;
    radeon_setup_i2c_bus(&rinfo.i2c[1], "dvi");
    rinfo.i2c[2].rinfo	= rinfo;
    rinfo.i2c[2].ddc_reg	= GPIO_VGA_DDC;
    radeon_setup_i2c_bus(&rinfo.i2c[2], "vga");
    rinfo.i2c[3].rinfo	= rinfo;
    rinfo.i2c[3].ddc_reg	= GPIO_CRT2_DDC;
    radeon_setup_i2c_bus(&rinfo.i2c[3], "crt2");
    }
#[no_mangle]
pub unsafe extern "C" fn radeon_delete_i2c_busses(rinfo: *mut radeonfb_info) {
    void radeon_delete_i2c_busses(struct radeonfb_info *rinfo)
    {
    if (rinfo.i2c[0].rinfo)
    i2c_del_adapter(&rinfo.i2c[0].adapter);
    rinfo.i2c[0].rinfo = core::ptr::null_mut();
    if (rinfo.i2c[1].rinfo)
    i2c_del_adapter(&rinfo.i2c[1].adapter);
    rinfo.i2c[1].rinfo = core::ptr::null_mut();
    if (rinfo.i2c[2].rinfo)
    i2c_del_adapter(&rinfo.i2c[2].adapter);
    rinfo.i2c[2].rinfo = core::ptr::null_mut();
    if (rinfo.i2c[3].rinfo)
    i2c_del_adapter(&rinfo.i2c[3].adapter);
    rinfo.i2c[3].rinfo = core::ptr::null_mut();
    }
    int radeon_probe_i2c_connector(struct radeonfb_info *rinfo, int conn,
    u8 **out_edid)
    {
    u8 *edid;
    edid = fb_ddc_read(&rinfo.i2c[conn-1].adapter);
    if (out_edid)
// out_edid = edid;
    if (!edid) {
    pr_debug("radeonfb: I2C (port %d) ... not found\n", conn);
    return MT_NONE;
    }
    if (edid[0x14] & 0x80) {
// Fix detection using BIOS tables
    if (rinfo.is_mobility /*&& conn == ddc_dvi*/ &&
    (INREG(LVDS_GEN_CNTL) & LVDS_ON)) {
    pr_debug("radeonfb: I2C (port %d) ... found LVDS panel\n", conn);
    return MT_LCD;
    } else {
    pr_debug("radeonfb: I2C (port %d) ... found TMDS panel\n", conn);
    return MT_DFP;
    }
    }
    pr_debug("radeonfb: I2C (port %d) ... found CRT display\n", conn);
    return MT_CRT;
    }
