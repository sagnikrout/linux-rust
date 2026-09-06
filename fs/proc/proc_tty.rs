//! Automatically rewritten from C to Rust
//! Source: fs/proc/proc_tty.c
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
// proc_tty.c -- handles /proc/tty
//
// Copyright 1997, Theodore Ts'o
//

//
// The /proc/tty directory inodes...
//
    static struct proc_dir_entry *proc_tty_driver;
//
// This is the handler for /proc/tty/drivers
//
    static void show_tty_range(struct seq_file *m, struct tty_driver *p,
    dev_t from, int num)
    {
    seq_printf(m, "%-20s ", p.driver_name ? p.driver_name : "unknown");
    seq_printf(m, "/dev/%-8s ", p.name);
    if (p.num > 1) {
    seq_printf(m, "%3d %d-%d ", MAJOR(from), MINOR(from),
    MINOR(from) + num - 1);
    } else {
    seq_printf(m, "%3d %7d ", MAJOR(from), MINOR(from));
    }
    switch (p.type) {
    case TTY_DRIVER_TYPE_SYSTEM:
    seq_puts(m, "system");
    if (p.subtype == SYSTEM_TYPE_TTY)
    seq_puts(m, ":/dev/tty");
#[no_mangle]
pub unsafe extern "C" fn if(SYSTEM_TYPE_SYSCONS: p->subtype ==) -> else {
    else if (p.subtype == SYSTEM_TYPE_SYSCONS)
    seq_puts(m, ":console");
#[no_mangle]
pub unsafe extern "C" fn if(SYSTEM_TYPE_CONSOLE: p->subtype ==) -> else {
    else if (p.subtype == SYSTEM_TYPE_CONSOLE)
    seq_puts(m, ":vtmaster");
    break;
    case TTY_DRIVER_TYPE_CONSOLE:
    seq_puts(m, "console");
    break;
    case TTY_DRIVER_TYPE_SERIAL:
    seq_puts(m, "serial");
    break;
    case TTY_DRIVER_TYPE_PTY:
    if (p.subtype == PTY_TYPE_MASTER)
    seq_puts(m, "pty:master");
#[no_mangle]
pub unsafe extern "C" fn if(PTY_TYPE_SLAVE: p->subtype ==) -> else {
    else if (p.subtype == PTY_TYPE_SLAVE)
    seq_puts(m, "pty:slave");
    else
    seq_puts(m, "pty");
    break;
    default:
    seq_printf(m, "type:%d.%d", p.type, p.subtype);
    }
    seq_putc(m, '\n');
    }
#[no_mangle]
unsafe extern "C" fn show_tty_driver(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_tty_driver(struct seq_file *m, void *v)
    {
    struct tty_driver *p = list_entry(v, struct tty_driver, tty_drivers);
    let mut from: dev_t = MKDEV(p.major, p.minor_start);
    let mut to: dev_t = from + p.num;
    if (&p.tty_drivers == tty_drivers.next) {
// pseudo-drivers first
    seq_printf(m, "%-20s /dev/%-8s ", "/dev/tty", "tty");
    seq_printf(m, "%3d %7d ", TTYAUX_MAJOR, 0);
    seq_puts(m, "system:/dev/tty\n");
    seq_printf(m, "%-20s /dev/%-8s ", "/dev/console", "console");
    seq_printf(m, "%3d %7d ", TTYAUX_MAJOR, 1);
    seq_puts(m, "system:console\n");

    seq_printf(m, "%-20s /dev/%-8s ", "/dev/ptmx", "ptmx");
    seq_printf(m, "%3d %7d ", TTYAUX_MAJOR, 2);
    seq_puts(m, "system\n");

    seq_printf(m, "%-20s /dev/%-8s ", "/dev/vc/0", "vc/0");
    seq_printf(m, "%3d %7d ", TTY_MAJOR, 0);
    seq_puts(m, "system:vtmaster\n");

    }
    while (MAJOR(from) < MAJOR(to)) {
    let mut next: dev_t = MKDEV(MAJOR(from)+1, 0);
    show_tty_range(m, p, from, next - from);
    from = next;
    }
    if (from != to)
    show_tty_range(m, p, from, to - from);
    return 0;
    }
// iterator
    static void *t_start(struct seq_file *m, loff_t *pos)
    {
    mutex_lock(&tty_mutex);
    return seq_list_start(&tty_drivers, *pos);
    }
    static void *t_next(struct seq_file *m, void *v, loff_t *pos)
    {
    return seq_list_next(v, &tty_drivers, pos);
    }
#[no_mangle]
unsafe extern "C" fn t_stop(m: *mut seq_file, v: *mut c_void) {
    static void t_stop(struct seq_file *m, void *v)
    {
    mutex_unlock(&tty_mutex);
    }
    static const struct seq_operations tty_drivers_op = {
    .start	= t_start,
    .next	= t_next,
    .stop	= t_stop,
    .show	= show_tty_driver
    };
//
// This function is called by tty_register_driver() to handle
// registering the driver's /proc handler into /proc/tty/driver/<foo>
//
#[no_mangle]
pub unsafe extern "C" fn proc_tty_register_driver(driver: *mut tty_driver) {
    void proc_tty_register_driver(struct tty_driver *driver)
    {
    struct proc_dir_entry *ent;
    if (!driver.driver_name || driver.proc_entry ||
    !driver.ops.proc_show)
    return;
    ent = proc_create_single_data(driver.driver_name, 0, proc_tty_driver,
    driver.ops.proc_show, driver);
    driver.proc_entry = ent;
    }
//
// This function is called by tty_unregister_driver()
//
#[no_mangle]
pub unsafe extern "C" fn proc_tty_unregister_driver(driver: *mut tty_driver) {
    void proc_tty_unregister_driver(struct tty_driver *driver)
    {
    struct proc_dir_entry *ent;
    ent = driver.proc_entry;
    if (!ent)
    return;
    remove_proc_entry(ent.name, proc_tty_driver);
    driver.proc_entry = core::ptr::null_mut();
    }
//
// Called by proc_root_init() to initialize the /proc/tty subtree
//
#[no_mangle]
pub unsafe extern "C" fn proc_tty_init() -> void __init {
    void __init proc_tty_init(void)
    {
    if (!proc_mkdir("tty", core::ptr::null_mut()))
    return;
    proc_mkdir("tty/ldisc", core::ptr::null_mut());	/* Preserved: it's userspace visible */
//
// /proc/tty/driver/serial reveals the exact character counts for
// serial links which is just too easy to abuse for inferring
// password lengths and inter-keystroke timings during password
// entry.
//
    proc_tty_driver = proc_mkdir_mode("tty/driver", S_IRUSR|S_IXUSR, core::ptr::null_mut());
    proc_create_seq("tty/ldiscs", 0, core::ptr::null_mut(), &tty_ldiscs_seq_ops);
    proc_create_seq("tty/drivers", 0, core::ptr::null_mut(), &tty_drivers_op);
    }
