//! Automatically rewritten from C to Rust
//! Source: drivers/accessibility/speakup/buffers.c
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

    static u16 synth_buffer[SYNTH_BUF_SIZE];	/* guess what this is for! */
    static u16 *buff_in = synth_buffer;
    static u16 *buff_out = synth_buffer;
    static u16 *buffer_end = synth_buffer + SYNTH_BUF_SIZE - 1;
// These try to throttle applications by stopping the TTYs
// Note: we need to make sure that we will restart them eventually, which is
// usually not possible to do from the notifiers. TODO: it should be possible
// starting from linux 2.6.26.
//
// So we only stop when we know alive == 1 (else we discard the data anyway),
// and the alive synth will eventually call start_ttys from the thread context.
//
#[no_mangle]
pub unsafe extern "C" fn speakup_start_ttys() {
    void speakup_start_ttys(void)
    {
    int i;
    for (i = 0; i < MAX_NR_CONSOLES; i++) {
    if (speakup_console[i] && speakup_console[i].tty_stopped)
    continue;
    if (vc_cons[i].d && vc_cons[i].d.port.tty)
    start_tty(vc_cons[i].d.port.tty);
    }
    }
    EXPORT_SYMBOL_GPL(speakup_start_ttys);
#[no_mangle]
unsafe extern "C" fn speakup_stop_ttys() {
    static void speakup_stop_ttys(void)
    {
    int i;
    for (i = 0; i < MAX_NR_CONSOLES; i++)
    if (vc_cons[i].d && vc_cons[i].d.port.tty)
    stop_tty(vc_cons[i].d.port.tty);
    }
#[no_mangle]
unsafe extern "C" fn synth_buffer_free() -> c_int {
    static int synth_buffer_free(void)
    {
    int chars_free;
    if (buff_in >= buff_out)
    chars_free = SYNTH_BUF_SIZE - (buff_in - buff_out);
    else
    chars_free = buff_out - buff_in;
    return chars_free;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_empty() -> c_int {
    int synth_buffer_empty(void)
    {
    return (buff_in == buff_out);
    }
    EXPORT_SYMBOL_GPL(synth_buffer_empty);
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_add(ch: u16) {
    void synth_buffer_add(u16 ch)
    {
    if (!synth.alive) {
// This makes sure that we won't stop TTYs if there is no synth
// to restart them
//
    return;
    }
    if (synth_buffer_free() <= 100) {
    synth_start();
    speakup_stop_ttys();
    }
    if (synth_buffer_free() <= 1)
    return;
// buff_in++ = ch;
    if (buff_in > buffer_end)
    buff_in = synth_buffer;
// We have written something to the speech synthesis, so we are not
// paused any more.
//
    spk_paused = false;
    }
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_getc() -> u16 {
    u16 synth_buffer_getc(void)
    {
    u16 ch;
    if (buff_out == buff_in)
    return 0;
    ch = *buff_out++;
    if (buff_out > buffer_end)
    buff_out = synth_buffer;
    return ch;
    }
    EXPORT_SYMBOL_GPL(synth_buffer_getc);
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_peek() -> u16 {
    u16 synth_buffer_peek(void)
    {
    if (buff_out == buff_in)
    return 0;
    return *buff_out;
    }
    EXPORT_SYMBOL_GPL(synth_buffer_peek);
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_skip_nonlatin1() {
    void synth_buffer_skip_nonlatin1(void)
    {
    while (buff_out != buff_in) {
    if (*buff_out < 0x100)
    return;
    buff_out++;
    if (buff_out > buffer_end)
    buff_out = synth_buffer;
    }
    }
    EXPORT_SYMBOL_GPL(synth_buffer_skip_nonlatin1);
#[no_mangle]
pub unsafe extern "C" fn synth_buffer_clear() {
    void synth_buffer_clear(void)
    {
    buff_in = synth_buffer;
    buff_out = synth_buffer;
    }
    EXPORT_SYMBOL_GPL(synth_buffer_clear);
