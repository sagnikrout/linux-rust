//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/udbg.c
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
// polling mode stateless debugging stuff, originally for NS16550 Serial Ports
//
// c 2001 PPC 64 Team, IBM Corp
//

    void (*udbg_putc)(char c);
    void (*udbg_flush)(void);
    int (*udbg_getc)(void);
    int (*udbg_getc_poll)(void);
//
// Early debugging facilities. You can enable _one_ of these via .config,
// if you do so your kernel _will not boot_ on anything else. Be careful.
//
#[no_mangle]
pub unsafe extern "C" fn udbg_early_init() -> void __init {
    void __init udbg_early_init(void)
    {

// For LPAR machines that have an HVC console on vterm 0
    udbg_init_debug_lpar();

// For LPAR machines that have an HVSI console on vterm 0
    udbg_init_debug_lpar_hvsi();

// For use on Apple G5 machines
    udbg_init_pmac_realmode();

// RTAS panel debug
    udbg_init_rtas_panel();

    udbg_init_pas_realmode();

    udbg_init_btext();

// PPC44x debug
    udbg_init_44x_as1();

    udbg_init_cpm();

    udbg_init_usbgecko();

// In memory console
    udbg_init_memcons();

    udbg_init_ehv_bc();

    udbg_init_ps3gelic();

    udbg_init_debug_opal_raw();

    udbg_init_debug_opal_hvsi();

    udbg_init_debug_16550();

    console_loglevel = CONSOLE_LOGLEVEL_DEBUG;
    register_early_udbg_console();

    }
// udbg library, used by xmon et al
#[no_mangle]
pub unsafe extern "C" fn udbg_puts(s: *const c_char) {
    void udbg_puts(const char *s)
    {
    if (udbg_putc) {
    char c;
    if (s && *s != '\0') {
    while ((c = *s++) != '\0')
    udbg_putc(c);
    }
    if (udbg_flush)
    udbg_flush();
    }

    else {
    printk("%s", s);
    }

    }
#[no_mangle]
pub unsafe extern "C" fn udbg_write(s: *const c_char, n: c_int) -> c_int {
    int udbg_write(const char *s, int n)
    {
    let mut remain: c_int = n;
    char c;
    if (!udbg_putc)
    return 0;
    if (s && *s != '\0') {
    while (((c = *s++) != '\0') && (remain-- > 0)) {
    udbg_putc(c);
    }
    }
    if (udbg_flush)
    udbg_flush();
    return n - remain;
    }
pub const UDBG_BUFSIZE: c_int = 256;
#[no_mangle]
pub unsafe extern "C" fn udbg_printf(fmt: *const c_char, ...) {
    void udbg_printf(const char *fmt, ...)
    {
    if (udbg_putc) {
    char buf[UDBG_BUFSIZE];
    va_list args;
    va_start(args, fmt);
    vsnprintf(buf, UDBG_BUFSIZE, fmt, args);
    udbg_puts(buf);
    va_end(args);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn udbg_progress(s: *mut c_char, hex: c_ushort) -> void __init {
    void __init udbg_progress(char *s, unsigned short hex)
    {
    udbg_puts(s);
    udbg_puts("\n");
    }
//
// Early boot console based on udbg
//
    static void udbg_console_write(struct console *con, const char *s,
    unsigned int n)
    {
    udbg_write(s, n);
    }
    static struct console udbg_console = {
    .name	= "udbg",
    .write	= udbg_console_write,
    .flags	= CON_PRINTBUFFER | CON_ENABLED | CON_BOOT | CON_ANYTIME,
    .index	= 0,
    };
//
// Called by setup_system after ppc_md->probe and ppc_md->early_init.
// Call it again after setting udbg_putc in ppc_md->setup_arch.
//
#[no_mangle]
pub unsafe extern "C" fn register_early_udbg_console() -> void __init {
    void __init register_early_udbg_console(void)
    {
    if (early_console)
    return;
    if (!udbg_putc)
    return;
    if (strstr(boot_command_line, "udbg-immortal")) {
    printk(KERN_INFO "early console immortal !\n");
    udbg_console.flags &= ~CON_BOOT;
    }
    early_console = &udbg_console;
    register_console(&udbg_console);
    }

    console_initcall(register_udbg_console);
