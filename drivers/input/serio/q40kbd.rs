//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/q40kbd.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2000-2001 Vojtech Pavlik
//
// Based on the work of:
// Richard Zidlicky <Richard.Zidlicky@stud.informatik.uni-erlangen.de>
//
// Q40 PS/2 keyboard controller driver for Linux/m68k
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Q40 PS/2 keyboard controller driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q40kbd {
    pub port: *mut serio,
    pub lock: spinlock_t,
}

#[no_mangle]
unsafe extern "C" fn q40kbd_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t q40kbd_interrupt(int irq, void *dev_id)
    {
    struct q40kbd *q40kbd = dev_id;
    guard(spinlock_irqsave)(&q40kbd.lock);
    if (Q40_IRQ_KEYB_MASK & master_inb(INTERRUPT_REG))
    serio_interrupt(q40kbd.port, master_inb(KEYCODE_REG), 0);
    master_outb(-1, KEYBOARD_UNLOCK_REG);
    return IRQ_HANDLED;
    }
//
// q40kbd_flush() flushes all data that may be in the keyboard buffers
//
#[no_mangle]
unsafe extern "C" fn q40kbd_flush(q40kbd: *mut q40kbd) {
    static void q40kbd_flush(struct q40kbd *q40kbd)
    {
    let mut maxread: c_int = 100;
    guard(spinlock_irqsave)(&q40kbd.lock);
    while (maxread-- && (Q40_IRQ_KEYB_MASK & master_inb(INTERRUPT_REG)))
    master_inb(KEYCODE_REG);
    }
#[no_mangle]
unsafe extern "C" fn q40kbd_stop() {
    static void q40kbd_stop(void)
    {
    master_outb(0, KEY_IRQ_ENABLE_REG);
    master_outb(-1, KEYBOARD_UNLOCK_REG);
    }
//
// q40kbd_open() is called when a port is open by the higher layer.
// It allocates the interrupt and enables in in the chip.
//
#[no_mangle]
unsafe extern "C" fn q40kbd_open(port: *mut serio) -> c_int {
    static int q40kbd_open(struct serio *port)
    {
    struct q40kbd *q40kbd = port.port_data;
    q40kbd_flush(q40kbd);
// off we go
    master_outb(-1, KEYBOARD_UNLOCK_REG);
    master_outb(1, KEY_IRQ_ENABLE_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn q40kbd_close(port: *mut serio) {
    static void q40kbd_close(struct serio *port)
    {
    struct q40kbd *q40kbd = port.port_data;
    q40kbd_stop();
    q40kbd_flush(q40kbd);
    }
#[no_mangle]
unsafe extern "C" fn q40kbd_probe(pdev: *mut platform_device) -> c_int {
    static int q40kbd_probe(struct platform_device *pdev)
    {
    struct q40kbd *q40kbd;
    struct serio *port;
    int error;
    q40kbd = kzalloc_obj(*q40kbd);
    port = kzalloc_obj(*port);
    if (!q40kbd || !port) {
    error = -ENOMEM;
    goto err_free_mem;
    }
    q40kbd.port = port;
    spin_lock_init(&q40kbd.lock);
    port.id.type = SERIO_8042;
    port.open = q40kbd_open;
    port.close = q40kbd_close;
    port.port_data = q40kbd;
    port.dev.parent = &pdev.dev;
    strscpy(port.name, "Q40 Kbd Port", sizeof(port.name));
    strscpy(port.phys, "Q40", sizeof(port.phys));
    q40kbd_stop();
    error = request_irq(Q40_IRQ_KEYBOARD, q40kbd_interrupt, 0,
    DRV_NAME, q40kbd);
    if (error) {
    dev_err(&pdev.dev, "Can't get irq %d.\n", Q40_IRQ_KEYBOARD);
    goto err_free_mem;
    }
    serio_register_port(q40kbd.port);
    platform_set_drvdata(pdev, q40kbd);
    printk(KERN_INFO "serio: Q40 kbd registered\n");
    return 0;
    err_free_mem:
    kfree(port);
    kfree(q40kbd);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn q40kbd_remove(pdev: *mut platform_device) {
    static void q40kbd_remove(struct platform_device *pdev)
    {
    struct q40kbd *q40kbd = platform_get_drvdata(pdev);
//
// q40kbd_close() will be called as part of unregistering
// and will ensure that IRQ is turned off, so it is safe
// to unregister port first and free IRQ later.
//
    serio_unregister_port(q40kbd.port);
    free_irq(Q40_IRQ_KEYBOARD, q40kbd);
    kfree(q40kbd);
    }
    static struct platform_driver q40kbd_driver = {
    .driver		= {
    .name	= "q40kbd",
    },
    .remove		= q40kbd_remove,
    };
    module_platform_driver_probe(q40kbd_driver, q40kbd_probe);
