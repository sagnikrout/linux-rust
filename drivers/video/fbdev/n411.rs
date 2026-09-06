//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/n411.c
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
// linux/drivers/video/n411.c -- Platform device for N411 EPD kit
//
// Copyright (C) 2008, Jaya Kumar
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// Layout is based on skeletonfb.c by James Simmons and Geert Uytterhoeven.
//
// This driver is written to be used with the Hecuba display controller
// board, and tested with the EInk 800x600 display in 1 bit mode.
// The interface between Hecuba and the host is TTL based GPIO. The
// GPIO requirements are 8 writable data lines and 6 lines for control.
// Only 4 of the controls are actually used here but 6 for future use.
// The driver requires the IO addresses for data and control GPIO at
// load time. It is also possible to use this display with a standard
// PC parallel port.
//
// General notes:
// - User must set dio_addr=0xIOADDR cio_addr=0xIOADDR c2io_addr=0xIOADDR
//

    static unsigned long dio_addr;
    static unsigned long cio_addr;
    static unsigned long c2io_addr;
    static unsigned long splashval;
    static unsigned int nosplash;
    static unsigned char ctl;
    static void n411_set_ctl(struct hecubafb_par *par, unsigned char bit, unsigned
    char state)
    {
    switch (bit) {
    case HCB_CD_BIT:
    if (state)
    ctl &= ~(HCB_CD_BIT);
    else
    ctl |= HCB_CD_BIT;
    break;
    case HCB_DS_BIT:
    if (state)
    ctl &= ~(HCB_DS_BIT);
    else
    ctl |= HCB_DS_BIT;
    break;
    }
    outb(ctl, cio_addr);
    }
#[no_mangle]
unsafe extern "C" fn n411_get_ctl(par: *mut hecubafb_par) -> c_uchar {
    static unsigned char n411_get_ctl(struct hecubafb_par *par)
    {
    return inb(c2io_addr);
    }
#[no_mangle]
unsafe extern "C" fn n411_set_data(par: *mut hecubafb_par, value: c_uchar) {
    static void n411_set_data(struct hecubafb_par *par, unsigned char value)
    {
    outb(value, dio_addr);
    }
#[no_mangle]
unsafe extern "C" fn n411_wait_for_ack(par: *mut hecubafb_par, clear: c_int) {
    static void n411_wait_for_ack(struct hecubafb_par *par, int clear)
    {
    int timeout;
    unsigned char tmp;
    timeout = 500;
    do {
    tmp = n411_get_ctl(par);
    if ((tmp & HCB_ACK_BIT) && (!clear))
    return;
#[no_mangle]
pub unsafe extern "C" fn if((clear): !(tmp & HCB_ACK_BIT) &&) -> else {
    else if (!(tmp & HCB_ACK_BIT) && (clear))
    return;
    udelay(1);
    } while (timeout--);
    printk(KERN_ERR "timed out waiting for ack\n");
    }
#[no_mangle]
unsafe extern "C" fn n411_init_control(par: *mut hecubafb_par) -> c_int {
    static int n411_init_control(struct hecubafb_par *par)
    {
    unsigned char tmp;
// for init, we want the following setup to be set:
    WUP = lo
    ACK = hi
    DS = hi
    RW = hi
    CD = lo
//
// write WUP to lo, DS to hi, RW to hi, CD to lo
    ctl = HCB_WUP_BIT | HCB_RW_BIT | HCB_CD_BIT ;
    n411_set_ctl(par, HCB_DS_BIT, 1);
// check ACK is not lo
    tmp = n411_get_ctl(par);
    if (tmp & HCB_ACK_BIT) {
    printk(KERN_ERR "Fail because ACK is already low\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn n411_init_board(par: *mut hecubafb_par) -> c_int {
    static int n411_init_board(struct hecubafb_par *par)
    {
    int retval;
    retval = n411_init_control(par);
    if (retval)
    return retval;
    par.send_command(par, APOLLO_INIT_DISPLAY);
    par.send_data(par, 0x81);
// have to wait while display resets
    udelay(1000);
// if we were told to splash the screen, we just clear it
    if (!nosplash) {
    par.send_command(par, APOLLO_ERASE_DISPLAY);
    par.send_data(par, splashval);
    }
    return 0;
    }
    static struct hecuba_board n411_board = {
    .owner			= THIS_MODULE,
    .init			= n411_init_board,
    .set_ctl		= n411_set_ctl,
    .set_data		= n411_set_data,
    .wait_for_ack		= n411_wait_for_ack,
    };
    static struct platform_device *n411_device;
#[no_mangle]
unsafe extern "C" fn n411_init() -> int __init {
    static int __init n411_init(void)
    {
    int ret;
    if (!dio_addr || !cio_addr || !c2io_addr) {
    printk(KERN_WARNING "no IO addresses supplied\n");
    return -EINVAL;
    }
// request our platform independent driver
    request_module("hecubafb");
    n411_device = platform_device_alloc("hecubafb", -1);
    if (!n411_device)
    return -ENOMEM;
    ret = platform_device_add_data(n411_device, &n411_board,
    sizeof(n411_board));
    if (ret)
    goto put_plat_device;
// this _add binds hecubafb to n411. hecubafb refcounts n411
    ret = platform_device_add(n411_device);
    if (ret)
    goto put_plat_device;
    return 0;
    put_plat_device:
    platform_device_put(n411_device);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn n411_exit() -> void __exit {
    static void __exit n411_exit(void)
    {
    platform_device_unregister(n411_device);
    }
    module_init(n411_init);
    module_exit(n411_exit);
    module_param(nosplash, uint, 0);
    MODULE_PARM_DESC(nosplash, "Disable doing the splash screen");
    module_param_hw(dio_addr, ulong, ioport, 0);
    MODULE_PARM_DESC(dio_addr, "IO address for data, eg: 0x480");
    module_param_hw(cio_addr, ulong, ioport, 0);
    MODULE_PARM_DESC(cio_addr, "IO address for control, eg: 0x400");
    module_param_hw(c2io_addr, ulong, ioport, 0);
    MODULE_PARM_DESC(c2io_addr, "IO address for secondary control, eg: 0x408");
    module_param(splashval, ulong, 0);
    MODULE_PARM_DESC(splashval, "Splash pattern: 0x00 is black, 0x01 is white");
    MODULE_DESCRIPTION("board driver for n411 hecuba/apollo epd kit");
    MODULE_AUTHOR("Jaya Kumar");
    MODULE_LICENSE("GPL");
