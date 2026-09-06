//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/early_serial_console.c
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
// Serial port routines for use during early boot reporting. This code is
// included from both the compressed kernel and the regular kernel.
//

pub const DEFAULT_SERIAL_PORT: c_uint = 0x3f8 /* ttyS0 */;
pub const DLAB: c_uint = 0x80;

pub const DEFAULT_BAUD: c_int = 9600;
#[no_mangle]
unsafe extern "C" fn early_serial_init(port: c_int, baud: c_int) {
    static void early_serial_init(int port, int baud)
    {
    unsigned char c;
    unsigned divisor;
    outb(0x3, port + LCR);	/* 8n1 */
    outb(0, port + IER);	/* no interrupt */
    outb(0, port + FCR);	/* no fifo */
    outb(0x3, port + MCR);	/* DTR + RTS */
    divisor	= 115200 / baud;
    c = inb(port + LCR);
    outb(c | DLAB, port + LCR);
    outb(divisor & 0xff, port + DLL);
    outb((divisor >> 8) & 0xff, port + DLH);
    outb(c & ~DLAB, port + LCR);
    early_serial_base = port;
    }
#[no_mangle]
unsafe extern "C" fn parse_earlyprintk() {
    static void parse_earlyprintk(void)
    {
    let mut baud: c_int = DEFAULT_BAUD;
    char arg[32];
    let mut pos: c_int = 0;
    let mut port: c_int = 0;
    if (cmdline_find_option("earlyprintk", arg, sizeof(arg)) > 0) {
    char *e;
    if (!strncmp(arg, "serial", 6)) {
    port = DEFAULT_SERIAL_PORT;
    pos += 6;
    }
    if (arg[pos] == ',')
    pos++;
//
// make sure we have
// "serial,0x3f8,115200"
// "serial,ttyS0,115200"
// "ttyS0,115200"
//
    if (pos == 7 && !strncmp(arg + pos, "0x", 2)) {
    port = simple_strtoull(arg + pos, &e, 16);
    if (port == 0 || arg + pos == e)
    port = DEFAULT_SERIAL_PORT;
    else
    pos = e - arg;
    } else if (!strncmp(arg + pos, "ttyS", 4)) {
    static const int bases[] = { 0x3f8, 0x2f8 };
    let mut idx: c_int = 0;
// += strlen("ttyS");
    pos += 4;
    if (arg[pos++] == '1')
    idx = 1;
    port = bases[idx];
    }
    if (arg[pos] == ',')
    pos++;
    baud = simple_strtoull(arg + pos, &e, 0);
    if (baud == 0 || arg + pos == e)
    baud = DEFAULT_BAUD;
    }
    if (port)
    early_serial_init(port, baud);
    }

#[no_mangle]
unsafe extern "C" fn probe_baud(port: c_int) -> c_uint {
    static unsigned int probe_baud(int port)
    {
    unsigned char lcr, dll, dlh;
    unsigned int quot;
    lcr = inb(port + LCR);
    outb(lcr | DLAB, port + LCR);
    dll = inb(port + DLL);
    dlh = inb(port + DLH);
    outb(lcr, port + LCR);
    quot = (dlh << 8) | dll;
    return BASE_BAUD / quot;
    }
#[no_mangle]
unsafe extern "C" fn parse_console_uart8250() {
    static void parse_console_uart8250(void)
    {
    char optstr[64], *options;
    int baud;
    let mut port: c_int = 0;
//
// console=uart8250,io,0x3f8,115200n8
// need to make sure it is last one console !
//
    if (cmdline_find_option("console", optstr, sizeof(optstr)) <= 0)
    return;
    options = optstr;
    if (!strncmp(options, "uart8250,io,", 12))
    port = simple_strtoull(options + 12, &options, 0);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(options, _arg: "uart, _arg: io, _arg: ", _arg: 8)) -> else {
    else if (!strncmp(options, "uart,io,", 8))
    port = simple_strtoull(options + 8, &options, 0);
    else
    return;
    if (options && (options[0] == ',')) {
    baud = simple_strtoull(options + 1, core::ptr::null_mut(), 0);
    if (!baud)
    baud = DEFAULT_BAUD;
    } else {
    baud = probe_baud(port);
    }
    if (port)
    early_serial_init(port, baud);
    }
#[no_mangle]
pub unsafe extern "C" fn console_init() {
    void console_init(void)
    {
    parse_earlyprintk();
    if (!early_serial_base)
    parse_console_uart8250();
    }
