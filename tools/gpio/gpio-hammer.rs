//! Automatically rewritten from C to Rust
//! Source: tools/gpio/gpio-hammer.c
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
// gpio-hammer - example swiss army knife to shake GPIO lines on a system
//
// Copyright (C) 2016 Linus Walleij
//
// Usage:
// gpio-hammer -n <device-name> -o <offset1> -o <offset2>
//

    int hammer_device(const char *device_name, unsigned int *lines, int num_lines,
    unsigned int loops)
    {
    struct gpio_v2_line_values values;
    struct gpio_v2_line_config config;
    char swirr[] = "-\\|/";
    int fd;
    int ret;
    int i, j;
    let mut iteration: c_uint = 0;
    memset(&config, 0, sizeof(config));
    config.flags = GPIO_V2_LINE_FLAG_OUTPUT;
    ret = gpiotools_request_line(device_name, lines, num_lines,
    &config, "gpio-hammer");
    if (ret < 0)
    goto exit_error;
    else
    fd = ret;
    values.mask = 0;
    values.bits = 0;
    for (i = 0; i < num_lines; i++)
    gpiotools_set_bit(&values.mask, i);
    ret = gpiotools_get_values(fd, &values);
    if (ret < 0)
    goto exit_close_error;
    fprintf(stdout, "Hammer lines [");
    for (i = 0; i < num_lines; i++) {
    fprintf(stdout, "%u", lines[i]);
    if (i != (num_lines - 1))
    fprintf(stdout, ", ");
    }
    fprintf(stdout, "] on %s, initial states: [", device_name);
    for (i = 0; i < num_lines; i++) {
    fprintf(stdout, "%d", gpiotools_test_bit(values.bits, i));
    if (i != (num_lines - 1))
    fprintf(stdout, ", ");
    }
    fprintf(stdout, "]\n");
// Hammertime!
    j = 0;
    while (1) {
// Invert all lines so we blink
    for (i = 0; i < num_lines; i++)
    gpiotools_change_bit(&values.bits, i);
    ret = gpiotools_set_values(fd, &values);
    if (ret < 0)
    goto exit_close_error;
// Re-read values to get status
    ret = gpiotools_get_values(fd, &values);
    if (ret < 0)
    goto exit_close_error;
    fprintf(stdout, "[%c] ", swirr[j]);
    j++;
    if (j == sizeof(swirr) - 1)
    j = 0;
    fprintf(stdout, "[");
    for (i = 0; i < num_lines; i++) {
    fprintf(stdout, "%u: %d", lines[i],
    gpiotools_test_bit(values.bits, i));
    if (i != (num_lines - 1))
    fprintf(stdout, ", ");
    }
    fprintf(stdout, "]\r");
    fflush(stdout);
    sleep(1);
    iteration++;
    if (loops && iteration == loops)
    break;
    }
    fprintf(stdout, "\n");
    ret = 0;
    exit_close_error:
    gpiotools_release_line(fd);
    exit_error:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn print_usage() {
    void print_usage(void)
    {
    fprintf(stderr, "Usage: gpio-hammer [options]...\n"
    "Hammer GPIO lines, 0.1.0.1...\n"
    "  -n <name>  Hammer GPIOs on a named device (must be stated)\n"
    "  -o <n>     Offset[s] to hammer, at least one, several can be stated\n"
    " [-c <n>]    Do <n> loops (optional, infinite loop if not stated)\n"
    "  -?         This helptext\n"
    "\n"
    "Example:\n"
    "gpio-hammer -n gpiochip0 -o 4\n"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *device_name = core::ptr::null_mut();
    unsigned int lines[GPIOHANDLES_MAX];
    let mut loops: c_uint = 0;
    int num_lines;
    int c;
    int i;
    i = 0;
    while ((c = getopt(argc, argv, "c:n:o:?")) != -1) {
    switch (c) {
    case 'c':
    loops = strtoul(optarg, core::ptr::null_mut(), 10);
    break;
    case 'n':
    device_name = optarg;
    break;
    case 'o':
//
// Avoid overflow. Do not immediately error, we want to
// be able to accurately report on the amount of times
// '-o' was given to give an accurate error message
//
    if (i < GPIOHANDLES_MAX)
    lines[i] = strtoul(optarg, core::ptr::null_mut(), 10);
    i++;
    break;
    case '?':
    print_usage();
    return -1;
    }
    }
    if (i >= GPIOHANDLES_MAX) {
    fprintf(stderr,
    "Only %d occurrences of '-o' are allowed, %d were found\n",
    GPIOHANDLES_MAX, i + 1);
    return -1;
    }
    num_lines = i;
    if (!device_name || !num_lines) {
    print_usage();
    return -1;
    }
    return hammer_device(device_name, lines, num_lines, loops);
    }
