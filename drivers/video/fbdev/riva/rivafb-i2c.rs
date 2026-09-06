//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/riva/rivafb-i2c.c
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
// linux/drivers/video/riva/fbdev-i2c.c - nVidia i2c
//
// Maintained by Ani Joshi <ajoshi@shell.unixbox.com>
//
// Copyright 2004 Antonino A. Daplas <adaplas @pol.net>
//
// Based on radeonfb-i2c.c
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

#[no_mangle]
unsafe extern "C" fn riva_gpio_setscl(data: *mut *mut c_void, state: c_int) {
    static void riva_gpio_setscl(void* data, int state)
    {
    struct riva_i2c_chan 	*chan = data;
    struct riva_par 	*par = chan.par;
    u32			val;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base + 1);
    val = VGA_RD08(par.riva.PCIO, 0x3d5) & 0xf0;
    if (state)
    val |= 0x20;
    else
    val &= ~0x20;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base + 1);
    VGA_WR08(par.riva.PCIO, 0x3d5, val | 0x1);
    }
#[no_mangle]
unsafe extern "C" fn riva_gpio_setsda(data: *mut *mut c_void, state: c_int) {
    static void riva_gpio_setsda(void* data, int state)
    {
    struct riva_i2c_chan 	*chan = data;
    struct riva_par 	*par = chan.par;
    u32			val;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base + 1);
    val = VGA_RD08(par.riva.PCIO, 0x3d5) & 0xf0;
    if (state)
    val |= 0x10;
    else
    val &= ~0x10;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base + 1);
    VGA_WR08(par.riva.PCIO, 0x3d5, val | 0x1);
    }
#[no_mangle]
unsafe extern "C" fn riva_gpio_getscl(data: *mut *mut c_void) -> c_int {
    static int riva_gpio_getscl(void* data)
    {
    struct riva_i2c_chan 	*chan = data;
    struct riva_par 	*par = chan.par;
    let mut val: u32 = 0;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base);
    if (VGA_RD08(par.riva.PCIO, 0x3d5) & 0x04)
    val = 1;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn riva_gpio_getsda(data: *mut *mut c_void) -> c_int {
    static int riva_gpio_getsda(void* data)
    {
    struct riva_i2c_chan 	*chan = data;
    struct riva_par 	*par = chan.par;
    let mut val: u32 = 0;
    VGA_WR08(par.riva.PCIO, 0x3d4, chan.ddc_base);
    if (VGA_RD08(par.riva.PCIO, 0x3d5) & 0x08)
    val = 1;
    return val;
    }
    static int riva_setup_i2c_bus(struct riva_i2c_chan *chan, const char *name,
    unsigned int i2c_class)
    {
    int rc;
    strscpy(chan.adapter.name, name);
    chan.adapter.owner		= THIS_MODULE;
    chan.adapter.class		= i2c_class;
    chan.adapter.algo_data		= &chan.algo;
    chan.adapter.dev.parent	= &chan.par.pdev.dev;
    chan.algo.setsda		= riva_gpio_setsda;
    chan.algo.setscl		= riva_gpio_setscl;
    chan.algo.getsda		= riva_gpio_getsda;
    chan.algo.getscl		= riva_gpio_getscl;
    chan.algo.udelay		= 40;
    chan.algo.timeout		= msecs_to_jiffies(2);
    chan.algo.data 		= chan;
    i2c_set_adapdata(&chan.adapter, chan);
// Raise SCL and SDA
    riva_gpio_setsda(chan, 1);
    riva_gpio_setscl(chan, 1);
    udelay(20);
    rc = i2c_bit_add_bus(&chan.adapter);
    if (rc == 0)
    dev_dbg(&chan.par.pdev.dev, "I2C bus %s registered.\n", name);
    else {
    dev_warn(&chan.par.pdev.dev,
    "Failed to register I2C bus %s.\n", name);
    chan.par = core::ptr::null_mut();
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn riva_create_i2c_busses(par: *mut riva_par) {
    void riva_create_i2c_busses(struct riva_par *par)
    {
    par.chan[0].par	= par;
    par.chan[1].par	= par;
    par.chan[2].par        = par;
    par.chan[0].ddc_base = 0x36;
    par.chan[1].ddc_base = 0x3e;
    par.chan[2].ddc_base = 0x50;
    riva_setup_i2c_bus(&par.chan[0], "BUS1", I2C_CLASS_HWMON);
    riva_setup_i2c_bus(&par.chan[1], "BUS2", 0);
    riva_setup_i2c_bus(&par.chan[2], "BUS3", 0);
    }
#[no_mangle]
pub unsafe extern "C" fn riva_delete_i2c_busses(par: *mut riva_par) {
    void riva_delete_i2c_busses(struct riva_par *par)
    {
    int i;
    for (i = 0; i < 3; i++) {
    if (!par.chan[i].par)
    continue;
    i2c_del_adapter(&par.chan[i].adapter);
    par.chan[i].par = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn riva_probe_i2c_connector(par: *mut riva_par, conn: c_int, out_edid: *mut u8) -> c_int {
    int riva_probe_i2c_connector(struct riva_par *par, int conn, u8 **out_edid)
    {
    u8 *edid = core::ptr::null_mut();
    if (par.chan[conn].par)
    edid = fb_ddc_read(&par.chan[conn].adapter);
    if (out_edid)
// out_edid = edid;
    if (!edid)
    return 1;
    return 0;
    }
