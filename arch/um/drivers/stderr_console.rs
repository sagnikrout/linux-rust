//! Automatically rewritten from C to Rust
//! Source: arch/um/drivers/stderr_console.c
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

// -----------------------------------------------------------------------------
// trivial console driver -- simply dump everything to stderr
//
// Don't register by default -- as this registers very early in the
// boot process it becomes the default console.
//
// Initialized at init time.
//
    let mut use_stderr_console: static int = 0;
    static void stderr_console_write(struct console *console, const char *string,
    unsigned len)
    {
    generic_write(2 /* stderr */, string, len, core::ptr::null_mut());
    }
    static struct console stderr_console = {
    .name		= "stderr",
    .write		= stderr_console_write,
    .flags		= CON_PRINTBUFFER,
    };
#[no_mangle]
unsafe extern "C" fn stderr_console_init() -> int __init {
    static int __init stderr_console_init(void)
    {
    if (use_stderr_console)
    register_console(&stderr_console);
    return 0;
    }
    console_initcall(stderr_console_init);
#[no_mangle]
unsafe extern "C" fn stderr_setup(str: *mut c_char) -> c_int {
    static int stderr_setup(char *str)
    {
    if (!str)
    return 0;
    use_stderr_console = simple_strtoul(str,&str,0);
    return 1;
    }
    __setup("stderr=", stderr_setup);
// The previous behavior of not unregistering led to /dev/console being
// impossible to open.  My FC5 filesystem started having init die, and the
// system panicing because of this.  Unregistering causes the real
// console to become the default console, and /dev/console can then be
// opened.  Making this an initcall makes this happen late enough that
// there is no added value in dumping everything to stderr, and the
// normal console is good enough to show you all available output.
//
#[no_mangle]
unsafe extern "C" fn unregister_stderr() -> int __init {
    static int __init unregister_stderr(void)
    {
    unregister_console(&stderr_console);
    return 0;
    }
    __initcall(unregister_stderr);
