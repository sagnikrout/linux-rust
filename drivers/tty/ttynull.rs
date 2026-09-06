//! Automatically rewritten from C to Rust
//! Source: drivers/tty/ttynull.c
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
//
// Copyright (C) 2019 Axis Communications AB
//
// Based on ttyprintk.c:
// Copyright (C) 2010 Samo Pogacnik
//

    static const struct tty_port_operations ttynull_port_ops;
    static struct tty_driver *ttynull_driver;
    static struct tty_port ttynull_port;
#[no_mangle]
unsafe extern "C" fn ttynull_open(tty: *mut tty_struct, filp: *mut file) -> c_int {
    static int ttynull_open(struct tty_struct *tty, struct file *filp)
    {
    return tty_port_open(&ttynull_port, tty, filp);
    }
#[no_mangle]
unsafe extern "C" fn ttynull_close(tty: *mut tty_struct, filp: *mut file) {
    static void ttynull_close(struct tty_struct *tty, struct file *filp)
    {
    tty_port_close(&ttynull_port, tty, filp);
    }
#[no_mangle]
unsafe extern "C" fn ttynull_hangup(tty: *mut tty_struct) {
    static void ttynull_hangup(struct tty_struct *tty)
    {
    tty_port_hangup(&ttynull_port);
    }
    static ssize_t ttynull_write(struct tty_struct *tty, const u8 *buf,
    size_t count)
    {
    return count;
    }
#[no_mangle]
unsafe extern "C" fn ttynull_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int ttynull_write_room(struct tty_struct *tty)
    {
    return 65536;
    }
    static const struct tty_operations ttynull_ops = {
    .open = ttynull_open,
    .close = ttynull_close,
    .hangup = ttynull_hangup,
    .write = ttynull_write,
    .write_room = ttynull_write_room,
    };
    static struct tty_driver *ttynull_device(struct console *c, int *index)
    {
// index = 0;
    return ttynull_driver;
    }
    static struct console ttynull_console = {
    .name = "ttynull",
    .device = ttynull_device,
    };
#[no_mangle]
unsafe extern "C" fn ttynull_init() -> int __init {
    static int __init ttynull_init(void)
    {
    struct tty_driver *driver;
    int ret;
    driver = tty_alloc_driver(1,
    TTY_DRIVER_RESET_TERMIOS |
    TTY_DRIVER_REAL_RAW |
    TTY_DRIVER_UNNUMBERED_NODE);
    if (IS_ERR(driver))
    return PTR_ERR(driver);
    tty_port_init(&ttynull_port);
    ttynull_port.ops = &ttynull_port_ops;
    driver.driver_name = "ttynull";
    driver.name = "ttynull";
    driver.type = TTY_DRIVER_TYPE_CONSOLE;
    driver.init_termios = tty_std_termios;
    driver.init_termios.c_oflag = OPOST | OCRNL | ONOCR | ONLRET;
    tty_set_operations(driver, &ttynull_ops);
    tty_port_link_device(&ttynull_port, driver, 0);
    ret = tty_register_driver(driver);
    if (ret < 0) {
    tty_driver_kref_put(driver);
    tty_port_destroy(&ttynull_port);
    return ret;
    }
    ttynull_driver = driver;
    register_console(&ttynull_console);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ttynull_exit() -> void __exit {
    static void __exit ttynull_exit(void)
    {
    unregister_console(&ttynull_console);
    tty_unregister_driver(ttynull_driver);
    tty_driver_kref_put(ttynull_driver);
    tty_port_destroy(&ttynull_port);
    }
    module_init(ttynull_init);
    module_exit(ttynull_exit);
    MODULE_DESCRIPTION("core::ptr::null_mut() TTY driver");
    MODULE_LICENSE("GPL v2");
