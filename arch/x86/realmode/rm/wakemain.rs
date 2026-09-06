//! Automatically rewritten from C to Rust
//! Source: arch/x86/realmode/rm/wakemain.c
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

#[no_mangle]
unsafe extern "C" fn udelay(loops: c_int) {
    static void udelay(int loops)
    {
    while (loops--)
    io_delay();	/* Approximately 1 us */
    }
#[no_mangle]
unsafe extern "C" fn beep(hz: c_uint) {
    static void beep(unsigned int hz)
    {
    u8 enable;
    if (!hz) {
    enable = 0x00;		/* Turn off speaker */
    } else {
    let mut div: u16 = 1193181/hz;
    outb(0xb6, 0x43);	/* Ctr 2, squarewave, load, binary */
    io_delay();
    outb(div, 0x42);	/* LSB of counter */
    io_delay();
    outb(div >> 8, 0x42);	/* MSB of counter */
    io_delay();
    enable = 0x03;		/* Turn on speaker */
    }
    inb(0x61);		/* Dummy read of System Control Port B */
    io_delay();
    outb(enable, 0x61);	/* Enable timer 2 output to speaker */
    io_delay();
    }
pub const DOT_HZ: c_int = 880;
pub const DASH_HZ: c_int = 587;
pub const US_PER_DOT: c_int = 125000;
// Okay, this is totally silly, but it's kind of fun.
#[no_mangle]
unsafe extern "C" fn send_morse(pattern: *const c_char) {
    static void send_morse(const char *pattern)
    {
    char s;
    while ((s = *pattern++)) {
    switch (s) {
    case '.':
    beep(DOT_HZ);
    udelay(US_PER_DOT);
    beep(0);
    udelay(US_PER_DOT);
    break;
    case '-':
    beep(DASH_HZ);
    udelay(US_PER_DOT * 3);
    beep(0);
    udelay(US_PER_DOT);
    break;
    default:	/* Assume it's a space */
    udelay(US_PER_DOT * 3);
    break;
    }
    }
    }
    struct port_io_ops pio_ops;
#[no_mangle]
pub unsafe extern "C" fn main() {
    void main(void)
    {
    init_default_io_ops();
// Kill machine if structures are wrong
    if (wakeup_header.real_magic != 0x12345678)
    while (1)
    ;
    if (wakeup_header.realmode_flags & 4)
    send_morse("...-");
    if (wakeup_header.realmode_flags & 1)
    asm volatile("lcallw   $0xc000,$3");
    if (wakeup_header.realmode_flags & 2) {
// Need to call BIOS
    probe_cards(0);
    set_mode(wakeup_header.video_mode);
    }
    }
