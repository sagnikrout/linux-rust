//! Automatically rewritten from C to Rust
//! Source: drivers/input/serio/serport.c
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
// Input device TTY line discipline
//
// Copyright (c) 1999-2002 Vojtech Pavlik
//
// This is a module that converts a tty line into a much simpler
// 'serial io port' abstraction that the input device drivers use.
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("Input device TTY line discipline");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS_LDISC(N_MOUSE);
pub const SERPORT_BUSY: c_int = 1;
pub const SERPORT_ACTIVE: c_int = 2;
pub const SERPORT_DEAD: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serport {
    pub tty: *mut tty_struct,
    pub wait: wait_queue_head_t,
    pub serio: *mut serio,
    pub id: serio_device_id,
    pub lock: spinlock_t,
    pub flags: c_ulong,
}

//
// Callback functions from the serio code.
//
#[no_mangle]
unsafe extern "C" fn serport_serio_write(serio: *mut serio, data: c_uchar) -> c_int {
    static int serport_serio_write(struct serio *serio, unsigned char data)
    {
    struct serport *serport = serio.port_data;
    return -(serport.tty.ops.write(serport.tty, &data, 1) != 1);
    }
#[no_mangle]
unsafe extern "C" fn serport_serio_open(serio: *mut serio) -> c_int {
    static int serport_serio_open(struct serio *serio)
    {
    struct serport *serport = serio.port_data;
    guard(spinlock_irqsave)(&serport.lock);
    set_bit(SERPORT_ACTIVE, &serport.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serport_serio_close(serio: *mut serio) {
    static void serport_serio_close(struct serio *serio)
    {
    struct serport *serport = serio.port_data;
    guard(spinlock_irqsave)(&serport.lock);
    clear_bit(SERPORT_ACTIVE, &serport.flags);
    }
//
// serport_ldisc_open() is the routine that is called upon setting our line
// discipline on a tty. It prepares the serio struct.
//
#[no_mangle]
unsafe extern "C" fn serport_ldisc_open(tty: *mut tty_struct) -> c_int {
    static int serport_ldisc_open(struct tty_struct *tty)
    {
    struct serport *serport;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    serport = kzalloc_obj(*serport);
    if (!serport)
    return -ENOMEM;
    serport.tty = tty;
    spin_lock_init(&serport.lock);
    init_waitqueue_head(&serport.wait);
    tty.disc_data = serport;
    tty.receive_room = 256;
    set_bit(TTY_DO_WRITE_WAKEUP, &tty.flags);
    return 0;
    }
//
// serport_ldisc_close() is the opposite of serport_ldisc_open()
//
#[no_mangle]
unsafe extern "C" fn serport_ldisc_close(tty: *mut tty_struct) {
    static void serport_ldisc_close(struct tty_struct *tty)
    {
    struct serport *serport = tty.disc_data;
    kfree(serport);
    }
//
// serport_ldisc_receive() is called by the low level tty driver when characters
// are ready for us. We forward the characters and flags, one by one to the
// 'interrupt' routine.
//
    static void serport_ldisc_receive(struct tty_struct *tty, const u8 *cp,
    const u8 *fp, size_t count)
    {
    struct serport *serport = tty.disc_data;
    let mut ch_flags: c_uint = 0;
    int i;
    guard(spinlock_irqsave)(&serport.lock);
    if (!test_bit(SERPORT_ACTIVE, &serport.flags))
    return;
    for (i = 0; i < count; i++) {
    if (fp) {
    switch (fp[i]) {
    case TTY_FRAME:
    ch_flags = SERIO_FRAME;
    break;
    case TTY_PARITY:
    ch_flags = SERIO_PARITY;
    break;
    default:
    ch_flags = 0;
    break;
    }
    }
    serio_interrupt(serport.serio, cp[i], ch_flags);
    }
    }
//
// serport_ldisc_read() just waits indefinitely if everything goes well.
// However, when the serio driver closes the serio port, it finishes,
// returning 0 characters.
//
    static ssize_t serport_ldisc_read(struct tty_struct * tty, struct file * file,
    u8 *kbuf, size_t nr, void **cookie,
    unsigned long offset)
    {
    struct serport *serport = tty.disc_data;
    struct serio *serio;
    if (test_and_set_bit(SERPORT_BUSY, &serport.flags))
    return -EBUSY;
    serport.serio = serio = kzalloc_obj(*serio);
    if (!serio)
    return -ENOMEM;
    strscpy(serio.name, "Serial port", sizeof(serio.name));
    snprintf(serio.phys, sizeof(serio.phys), "%s/serio0", tty_name(tty));
    serio.id = serport.id;
    serio.id.type = SERIO_RS232;
    serio.write = serport_serio_write;
    serio.open = serport_serio_open;
    serio.close = serport_serio_close;
    serio.port_data = serport;
    serio.dev.parent = tty.dev;
    serio_register_port(serport.serio);
    printk(KERN_INFO "serio: Serial port %s\n", tty_name(tty));
    wait_event_interruptible(serport.wait, test_bit(SERPORT_DEAD, &serport.flags));
    serio_unregister_port(serport.serio);
    serport.serio = core::ptr::null_mut();
    clear_bit(SERPORT_DEAD, &serport.flags);
    clear_bit(SERPORT_BUSY, &serport.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serport_set_type(tty: *mut tty_struct, type: c_ulong) {
    static void serport_set_type(struct tty_struct *tty, unsigned long type)
    {
    struct serport *serport = tty.disc_data;
    serport.id.proto = type & 0x000000ff;
    serport.id.id    = (type & 0x0000ff00) >> 8;
    serport.id.extra = (type & 0x00ff0000) >> 16;
    }
//
// serport_ldisc_ioctl() allows to set the port protocol, and device ID
//
    static int serport_ldisc_ioctl(struct tty_struct *tty, unsigned int cmd,
    unsigned long arg)
    {
    if (cmd == SPIOCSTYPE) {
    unsigned long type;
    if (get_user(type, (unsigned long __user *) arg))
    return -EFAULT;
    serport_set_type(tty, type);
    return 0;
    }
    return -EINVAL;
    }

    static int serport_ldisc_compat_ioctl(struct tty_struct *tty,
    unsigned int cmd, unsigned long arg)
    {
    if (cmd == COMPAT_SPIOCSTYPE) {
    void __user *uarg = compat_ptr(arg);
    compat_ulong_t compat_type;
    if (get_user(compat_type, (compat_ulong_t __user *)uarg))
    return -EFAULT;
    serport_set_type(tty, compat_type);
    return 0;
    }
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn serport_ldisc_hangup(tty: *mut tty_struct) {
    static void serport_ldisc_hangup(struct tty_struct *tty)
    {
    struct serport *serport = tty.disc_data;
    scoped_guard(spinlock_irqsave, &serport.lock)
    set_bit(SERPORT_DEAD, &serport.flags);
    wake_up_interruptible(&serport.wait);
    }
#[no_mangle]
unsafe extern "C" fn serport_ldisc_write_wakeup(tty: *mut *mut tty_struct) {
    static void serport_ldisc_write_wakeup(struct tty_struct * tty)
    {
    struct serport *serport = tty.disc_data;
    guard(spinlock_irqsave)(&serport.lock);
    if (test_bit(SERPORT_ACTIVE, &serport.flags))
    serio_drv_write_wakeup(serport.serio);
    }
//
// The line discipline structure.
//
    static struct tty_ldisc_ops serport_ldisc = {
    .owner =	THIS_MODULE,
    .num =		N_MOUSE,
    .name =		"input",
    .open =		serport_ldisc_open,
    .close =	serport_ldisc_close,
    .read =		serport_ldisc_read,
    .ioctl =	serport_ldisc_ioctl,

    .compat_ioctl =	serport_ldisc_compat_ioctl,

    .receive_buf =	serport_ldisc_receive,
    .hangup =	serport_ldisc_hangup,
    .write_wakeup =	serport_ldisc_write_wakeup
    };
//
// The functions for insering/removing us as a module.
//
#[no_mangle]
unsafe extern "C" fn serport_init() -> int __init {
    static int __init serport_init(void)
    {
    int retval;
    retval = tty_register_ldisc(&serport_ldisc);
    if (retval)
    printk(KERN_ERR "serport.c: Error registering line discipline.\n");
    return  retval;
    }
#[no_mangle]
unsafe extern "C" fn serport_exit() -> void __exit {
    static void __exit serport_exit(void)
    {
    tty_unregister_ldisc(&serport_ldisc);
    }
    module_init(serport_init);
    module_exit(serport_exit);
