//! Automatically rewritten from C to Rust
//! Source: fs/proc/consoles.c
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
// Copyright (c) 2010 Werner Fink, Jiri Slaby
//

//
// This is handler for /proc/consoles
//
#[no_mangle]
unsafe extern "C" fn show_console_dev(m: *mut seq_file, v: *mut c_void) -> c_int {
    static int show_console_dev(struct seq_file *m, void *v)
    {
    static const struct {
    short flag;
    char name;
    } con_flags[] = {
    { CON_ENABLED,		'E' },
    { CON_CONSDEV,		'C' },
    { CON_BOOT,		'B' },
    { CON_NBCON,		'N' },
    { CON_PRINTBUFFER,	'p' },
    { CON_BRL,		'b' },
    { CON_ANYTIME,		'a' },
    };
    char flags[ARRAY_SIZE(con_flags) + 1];
    struct console *con = v;
    unsigned int a;
    let mut dev: dev_t = 0;
    if (con.device) {
    const struct tty_driver *driver;
    int index;
//
// Take console_lock to serialize device() callback with
// other console operations. For example, fg_console is
// modified under console_lock when switching vt.
//
    console_lock();
    driver = con.device(con, &index);
    console_unlock();
    if (driver) {
    dev = MKDEV(driver.major, driver.minor_start);
    dev += index;
    }
    }
    for (a = 0; a < ARRAY_SIZE(con_flags); a++)
    flags[a] = (con.flags & con_flags[a].flag) ?
    con_flags[a].name : ' ';
    flags[a] = 0;
    seq_setwidth(m, 21 - 1);
    seq_printf(m, "%s%d", con.name, con.index);
    seq_pad(m, ' ');
    seq_printf(m, "%c%c%c (%s)", con.read ? 'R' : '-',
    ((con.flags & CON_NBCON) || con.write) ? 'W' : '-',
    con.unblank ? 'U' : '-', flags);
    if (dev)
    seq_printf(m, " %4d:%d", MAJOR(dev), MINOR(dev));
    seq_putc(m, '\n');
    return 0;
    }
    static void *c_start(struct seq_file *m, loff_t *pos)
    __acquires(&console_mutex)
    {
    struct console *con;
    let mut off: loff_t = 0;
//
// Hold the console_list_lock to guarantee safe traversal of the
// console list. SRCU cannot be used because there is no
// place to store the SRCU cookie.
//
    console_list_lock();
    for_each_console(con)
    if (off++ == *pos)
    break;
    return con;
    }
    static void *c_next(struct seq_file *m, void *v, loff_t *pos)
    {
    struct console *con = v;
    ++*pos;
    return hlist_entry_safe(con.node.next, struct console, node);
    }
#[no_mangle]
unsafe extern "C" fn c_stop(m: *mut seq_file, v: *mut c_void) {
    static void c_stop(struct seq_file *m, void *v)
    __releases(&console_mutex)
    {
    console_list_unlock();
    }
    static const struct seq_operations consoles_op = {
    .start	= c_start,
    .next	= c_next,
    .stop	= c_stop,
    .show	= show_console_dev
    };
#[no_mangle]
unsafe extern "C" fn proc_consoles_init() -> int __init {
    static int __init proc_consoles_init(void)
    {
    proc_create_seq("consoles", 0, core::ptr::null_mut(), &consoles_op);
    return 0;
    }
    fs_initcall(proc_consoles_init);
