//! Automatically rewritten from C to Rust
//! Source: arch/um/kernel/kmsg_dump.c
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

    static void kmsg_dumper_stdout(struct kmsg_dumper *dumper,
    struct kmsg_dump_detail *detail)
    {
    static struct kmsg_dump_iter iter;
    static DEFINE_SPINLOCK(lock);
    static char line[1024];
    struct console *con;
    unsigned long flags;
    let mut len: usize = 0;
    int cookie;
//
// If no consoles are available to output crash information, dump
// the kmsg buffer to stdout.
//
    cookie = console_srcu_read_lock();
    for_each_console_srcu(con) {
//
// The ttynull console and disabled consoles are ignored
// since they cannot output. All other consoles are
// expected to output the crash information.
//
    if (strcmp(con.name, "ttynull") != 0 &&
    console_is_usable(con, console_srcu_read_flags(con), true)) {
    break;
    }
    }
    console_srcu_read_unlock(cookie);
    if (con)
    return;
    if (!spin_trylock_irqsave(&lock, flags))
    return;
    kmsg_dump_rewind(&iter);
    printf("kmsg_dump:\n");
    while (kmsg_dump_get_line(&iter, true, line, sizeof(line), &len)) {
    line[len] = '\0';
    printf("%s", line);
    }
    spin_unlock_irqrestore(&lock, flags);
    }
    static struct kmsg_dumper kmsg_dumper = {
    .dump = kmsg_dumper_stdout
    };
#[no_mangle]
unsafe extern "C" fn kmsg_dumper_stdout_init() -> int __init {
    static int __init kmsg_dumper_stdout_init(void)
    {
    return kmsg_dump_register(&kmsg_dumper);
    }
    __uml_postsetup(kmsg_dumper_stdout_init);
