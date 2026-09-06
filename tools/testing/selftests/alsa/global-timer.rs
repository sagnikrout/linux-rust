//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/alsa/global-timer.c
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
// This tool is used by the utimer test, and it allows us to
// count the ticks of a global timer in a certain time frame
// (which is set by `timeout` parameter).
//
// Author: Ivan Orlov <ivan.orlov0322@gmail.com>
//

    static int ticked;
#[no_mangle]
unsafe extern "C" fn async_callback(ahandler: *mut snd_async_handler_t) {
    static void async_callback(snd_async_handler_t *ahandler)
    {
    ticked++;
    }
    static char timer_name[64];
#[no_mangle]
unsafe extern "C" fn bind_to_timer(device: c_int, subdevice: c_int, timeout: c_int) {
    static void bind_to_timer(int device, int subdevice, int timeout)
    {
    snd_timer_t *handle;
    snd_timer_params_t *params;
    snd_async_handler_t *ahandler;
    time_t end;
    sprintf(timer_name, "hw:CLASS=%d,SCLASS=%d,DEV=%d,SUBDEV=%d",
    SND_TIMER_CLASS_GLOBAL, SND_TIMER_SCLASS_NONE,
    device, subdevice);
    snd_timer_params_alloca(&params);
    if (snd_timer_open(&handle, timer_name, SND_TIMER_OPEN_NONBLOCK) < 0) {
    perror("Can't open the timer");
    exit(EXIT_FAILURE);
    }
    snd_timer_params_set_auto_start(params, 1);
    snd_timer_params_set_ticks(params, 1);
    if (snd_timer_params(handle, params) < 0) {
    perror("Can't set timer params");
    exit(EXIT_FAILURE);
    }
    if (snd_async_add_timer_handler(&ahandler, handle, async_callback, core::ptr::null_mut()) < 0) {
    perror("Can't create a handler");
    exit(EXIT_FAILURE);
    }
    end = time(core::ptr::null_mut()) + timeout;
    if (snd_timer_start(handle) < 0) {
    perror("Failed to start the timer");
    exit(EXIT_FAILURE);
    }
    printf("Timer has started\n");
    while (time(core::ptr::null_mut()) <= end) {
//
// Waiting for the timeout to elapse. Can't use sleep here, as it gets
// constantly interrupted by the signal from the timer (SIGIO)
//
    }
    snd_timer_stop(handle);
    snd_timer_close(handle);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int device, subdevice, timeout;
    if (argc < 4) {
    perror("Usage: %s <device> <subdevice> <timeout>");
    return EXIT_FAILURE;
    }
    setlinebuf(stdout);
    device = atoi(argv[1]);
    subdevice = atoi(argv[2]);
    timeout = atoi(argv[3]);
    bind_to_timer(device, subdevice, timeout);
    printf("Total ticks count: %d\n", ticked);
    return EXIT_SUCCESS;
    }
