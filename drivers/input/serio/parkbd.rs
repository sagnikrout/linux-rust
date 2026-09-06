//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/parkbd.c
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
// Parallel port to Keyboard port adapter driver for Linux
//
// Copyright (c) 1999-2004 Vojtech Pavlik
//
// To connect an AT or XT keyboard to the parallel port, a fairly simple adapter
// can be made:
//
// Parallel port            Keyboard port
//
// +5V --------------------- +5V (4)
//
// ______
// +5V -------|______|--.
// |
// ACK (10) ------------|
// |--- KBD CLOCK (5)
// STROBE (1) ---|<|----'
//
// ______
// +5V -------|______|--.
// |
// BUSY (11) -----------|
// |--- KBD DATA (1)
// AUTOFD (14) --|<|----'
//
// GND (18-25) ------------- GND (3)
//
// The diodes can be fairly any type, and the resistors should be somewhere
// around 5 kOhm, but the adapter will likely work without the resistors,
// too.
//
// The +5V source can be taken either from USB, from mouse or keyboard ports,
// or from a joystick port. Unfortunately, the parallel port of a PC doesn't
// have a +5V pin, and feeding the keyboard from signal pins is out of question
// with 300 mA power reqirement of a typical AT keyboard.
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Parallel port to Keyboard port adapter driver");
    MODULE_LICENSE("GPL");
    static unsigned int parkbd_pp_no;
    module_param_named(port, parkbd_pp_no, int, 0);
    MODULE_PARM_DESC(port, "Parallel port the adapter is connected to (default is 0)");
    let mut parkbd_mode: static unsigned int = SERIO_8042;
    module_param_named(mode, parkbd_mode, uint, 0);
    MODULE_PARM_DESC(mode, "Mode of operation: XT = 0/AT = 1 (default)");
pub const PARKBD_CLOCK: c_uint = 0x01	/* Strobe & Ack */;
pub const PARKBD_DATA: c_uint = 0x02	/* AutoFd & Busy */;
    static int parkbd_buffer;
    static int parkbd_counter;
    static unsigned long parkbd_last;
    static int parkbd_writing;
    static unsigned long parkbd_start;
    static struct pardevice *parkbd_dev;
    static struct serio *parkbd_port;
#[no_mangle]
unsafe extern "C" fn parkbd_readlines() -> c_int {
    static int parkbd_readlines(void)
    {
    return (parport_read_status(parkbd_dev.port) >> 6) ^ 2;
    }
#[no_mangle]
unsafe extern "C" fn parkbd_writelines(data: c_int) {
    static void parkbd_writelines(int data)
    {
    parport_write_control(parkbd_dev.port, (~data & 3) | 0x10);
    }
#[no_mangle]
unsafe extern "C" fn parkbd_write(port: *mut serio, c: c_uchar) -> c_int {
    static int parkbd_write(struct serio *port, unsigned char c)
    {
    unsigned char p;
    if (!parkbd_mode) return -1;
    p = c ^ (c >> 4);
    p = p ^ (p >> 2);
    p = p ^ (p >> 1);
    parkbd_counter = 0;
    parkbd_writing = 1;
    parkbd_buffer = c | (((int) (~p & 1)) << 8) | 0x600;
    parkbd_writelines(2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parkbd_interrupt(dev_id: *mut c_void) {
    static void parkbd_interrupt(void *dev_id)
    {
    if (parkbd_writing) {
    if (parkbd_counter && ((parkbd_counter == 11) || time_after(jiffies, parkbd_last + HZ/100))) {
    parkbd_counter = 0;
    parkbd_buffer = 0;
    parkbd_writing = 0;
    parkbd_writelines(3);
    return;
    }
    parkbd_writelines(((parkbd_buffer >> parkbd_counter++) & 1) | 2);
    if (parkbd_counter == 11) {
    parkbd_counter = 0;
    parkbd_buffer = 0;
    parkbd_writing = 0;
    parkbd_writelines(3);
    }
    } else {
    if ((parkbd_counter == parkbd_mode + 10) || time_after(jiffies, parkbd_last + HZ/100)) {
    parkbd_counter = 0;
    parkbd_buffer = 0;
    }
    parkbd_buffer |= (parkbd_readlines() >> 1) << parkbd_counter++;
    if (parkbd_counter == parkbd_mode + 10)
    serio_interrupt(parkbd_port, (parkbd_buffer >> (2 - parkbd_mode)) & 0xff, 0);
    }
    parkbd_last = jiffies;
    }
#[no_mangle]
unsafe extern "C" fn parkbd_getport(pp: *mut parport) -> c_int {
    static int parkbd_getport(struct parport *pp)
    {
    struct pardev_cb parkbd_parport_cb;
    memset(&parkbd_parport_cb, 0, sizeof(parkbd_parport_cb));
    parkbd_parport_cb.irq_func = parkbd_interrupt;
    parkbd_parport_cb.flags = PARPORT_FLAG_EXCL;
    parkbd_dev = parport_register_dev_model(pp, "parkbd",
    &parkbd_parport_cb, 0);
    if (!parkbd_dev)
    return -ENODEV;
    if (parport_claim(parkbd_dev)) {
    parport_unregister_device(parkbd_dev);
    return -EBUSY;
    }
    parkbd_start = jiffies;
    return 0;
    }
    static struct serio *parkbd_allocate_serio(void)
    {
    struct serio *serio;
    serio = kzalloc_obj(*serio);
    if (serio) {
    serio.id.type = parkbd_mode;
    serio.write = parkbd_write;
    strscpy(serio.name, "PARKBD AT/XT keyboard adapter", sizeof(serio.name));
    snprintf(serio.phys, sizeof(serio.phys), "%s/serio0", parkbd_dev.port.name);
    }
    return serio;
    }
#[no_mangle]
unsafe extern "C" fn parkbd_attach(pp: *mut parport) {
    static void parkbd_attach(struct parport *pp)
    {
    if (pp.number != parkbd_pp_no) {
    pr_debug("Not using parport%d.\n", pp.number);
    return;
    }
    if (parkbd_getport(pp))
    return;
    parkbd_port = parkbd_allocate_serio();
    if (!parkbd_port) {
    parport_release(parkbd_dev);
    parport_unregister_device(parkbd_dev);
    return;
    }
    parkbd_writelines(3);
    serio_register_port(parkbd_port);
    printk(KERN_INFO "serio: PARKBD %s adapter on %s\n",
    parkbd_mode ? "AT" : "XT", parkbd_dev.port.name);
    return;
    }
#[no_mangle]
unsafe extern "C" fn parkbd_detach(port: *mut parport) {
    static void parkbd_detach(struct parport *port)
    {
    if (!parkbd_port || port.number != parkbd_pp_no)
    return;
    parport_release(parkbd_dev);
    serio_unregister_port(parkbd_port);
    parport_unregister_device(parkbd_dev);
    parkbd_port = core::ptr::null_mut();
    }
    static struct parport_driver parkbd_parport_driver = {
    .name = "parkbd",
    .match_port = parkbd_attach,
    .detach = parkbd_detach,
    };
    module_parport_driver(parkbd_parport_driver);
