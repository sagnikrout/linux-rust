//! Automatically rewritten from C to Rust
//! Source: drivers/char/ttyprintk.c
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
// linux/drivers/char/ttyprintk.c
//
// Copyright (C) 2010  Samo Pogacnik
//
// This pseudo device allows user to make printk messages. It is possible
// to store "console" messages inline with kernel messages for better analyses
// of the boot process, for example.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttyprintk_port {
    pub port: tty_port,
    pub spinlock: spinlock_t,
}

    static struct ttyprintk_port tpk_port;
//
// Our simple preformatting supports transparent output of (time-stamped)
// printk messages (also suitable for logging service):
// - any cr is replaced by nl
// - adds a ttyprintk source tag in front of each line
// - too long message is fragmented, with '\'nl between fragments
// - TPK_STR_SIZE isn't really the write_room limiting factor, because
// it is emptied on the fly during preformatting.
//

    static int tpk_curr;
    static u8 tpk_buffer[TPK_STR_SIZE + 4];
#[no_mangle]
unsafe extern "C" fn tpk_flush() {
    static void tpk_flush(void)
    {
    if (tpk_curr > 0) {
    tpk_buffer[tpk_curr] = '\0';
    printk(TPK_PREFIX "[U] %s\n", tpk_buffer);
    tpk_curr = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn tpk_printk(buf: *const u8, count: usize) -> c_int {
    static int tpk_printk(const u8 *buf, size_t count)
    {
    size_t i;
    for (i = 0; i < count; i++) {
    if (tpk_curr >= TPK_STR_SIZE) {
// end of tmp buffer reached: cut the message in two
    tpk_buffer[tpk_curr++] = '\\';
    tpk_flush();
    }
    switch (buf[i]) {
    case '\r':
    tpk_flush();
    if ((i + 1) < count && buf[i + 1] == '\n')
    i++;
    break;
    case '\n':
    tpk_flush();
    break;
    default:
    tpk_buffer[tpk_curr++] = buf[i];
    break;
    }
    }
    return count;
    }
//
// TTY operations open function.
//
#[no_mangle]
unsafe extern "C" fn tpk_open(tty: *mut tty_struct, filp: *mut file) -> c_int {
    static int tpk_open(struct tty_struct *tty, struct file *filp)
    {
    tty.driver_data = &tpk_port;
    return tty_port_open(&tpk_port.port, tty, filp);
    }
//
// TTY operations close function.
//
#[no_mangle]
unsafe extern "C" fn tpk_close(tty: *mut tty_struct, filp: *mut file) {
    static void tpk_close(struct tty_struct *tty, struct file *filp)
    {
    struct ttyprintk_port *tpkp = tty.driver_data;
    tty_port_close(&tpkp.port, tty, filp);
    }
//
// TTY operations write function.
//
#[no_mangle]
unsafe extern "C" fn tpk_write(tty: *mut tty_struct, buf: *const u8, count: usize) -> isize {
    static ssize_t tpk_write(struct tty_struct *tty, const u8 *buf, size_t count)
    {
    struct ttyprintk_port *tpkp = tty.driver_data;
    unsigned long flags;
    int ret;
// exclusive use of tpk_printk within this tty
    spin_lock_irqsave(&tpkp.spinlock, flags);
    ret = tpk_printk(buf, count);
    spin_unlock_irqrestore(&tpkp.spinlock, flags);
    return ret;
    }
//
// TTY operations write_room function.
//
#[no_mangle]
unsafe extern "C" fn tpk_write_room(tty: *mut tty_struct) -> c_uint {
    static unsigned int tpk_write_room(struct tty_struct *tty)
    {
    return TPK_MAX_ROOM;
    }
//
// TTY operations hangup function.
//
#[no_mangle]
unsafe extern "C" fn tpk_hangup(tty: *mut tty_struct) {
    static void tpk_hangup(struct tty_struct *tty)
    {
    struct ttyprintk_port *tpkp = tty.driver_data;
    tty_port_hangup(&tpkp.port);
    }
//
// TTY port operations shutdown function.
//
#[no_mangle]
unsafe extern "C" fn tpk_port_shutdown(tport: *mut tty_port) {
    static void tpk_port_shutdown(struct tty_port *tport)
    {
    struct ttyprintk_port *tpkp =
    container_of(tport, struct ttyprintk_port, port);
    unsigned long flags;
    spin_lock_irqsave(&tpkp.spinlock, flags);
    tpk_flush();
    spin_unlock_irqrestore(&tpkp.spinlock, flags);
    }
    static const struct tty_operations ttyprintk_ops = {
    .open = tpk_open,
    .close = tpk_close,
    .write = tpk_write,
    .write_room = tpk_write_room,
    .hangup = tpk_hangup,
    };
    static const struct tty_port_operations tpk_port_ops = {
    .shutdown = tpk_port_shutdown,
    };
    static struct tty_driver *ttyprintk_driver;
    static struct tty_driver *ttyprintk_console_device(struct console *c,
    int *index)
    {
// index = 0;
    return ttyprintk_driver;
    }
    static struct console ttyprintk_console = {
    .name = "ttyprintk",
    .device = ttyprintk_console_device,
    };
#[no_mangle]
unsafe extern "C" fn ttyprintk_init() -> int __init {
    static int __init ttyprintk_init(void)
    {
    int ret;
    spin_lock_init(&tpk_port.spinlock);
    ttyprintk_driver = tty_alloc_driver(1,
    TTY_DRIVER_RESET_TERMIOS |
    TTY_DRIVER_REAL_RAW |
    TTY_DRIVER_UNNUMBERED_NODE);
    if (IS_ERR(ttyprintk_driver))
    return PTR_ERR(ttyprintk_driver);
    tty_port_init(&tpk_port.port);
    tpk_port.port.ops = &tpk_port_ops;
    ttyprintk_driver.driver_name = "ttyprintk";
    ttyprintk_driver.name = "ttyprintk";
    ttyprintk_driver.major = TTYAUX_MAJOR;
    ttyprintk_driver.minor_start = 3;
    ttyprintk_driver.type = TTY_DRIVER_TYPE_CONSOLE;
    ttyprintk_driver.init_termios = tty_std_termios;
    ttyprintk_driver.init_termios.c_oflag = OPOST | OCRNL | ONOCR | ONLRET;
    tty_set_operations(ttyprintk_driver, &ttyprintk_ops);
    tty_port_link_device(&tpk_port.port, ttyprintk_driver, 0);
    ret = tty_register_driver(ttyprintk_driver);
    if (ret < 0) {
    printk(KERN_ERR "Couldn't register ttyprintk driver\n");
    goto error;
    }
    register_console(&ttyprintk_console);
    return 0;
    error:
    tty_driver_kref_put(ttyprintk_driver);
    tty_port_destroy(&tpk_port.port);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ttyprintk_exit() -> void __exit {
    static void __exit ttyprintk_exit(void)
    {
    unregister_console(&ttyprintk_console);
    tty_unregister_driver(ttyprintk_driver);
    tty_driver_kref_put(ttyprintk_driver);
    tty_port_destroy(&tpk_port.port);
    }
    device_initcall(ttyprintk_init);
    module_exit(ttyprintk_exit);
    MODULE_DESCRIPTION("TTY driver to output user messages via printk");
    MODULE_LICENSE("GPL");
