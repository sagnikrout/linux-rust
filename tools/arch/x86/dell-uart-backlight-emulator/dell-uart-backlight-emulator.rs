//! Automatically rewritten from C to Rust
//! Source: tools/arch/x86/dell-uart-backlight-emulator/dell-uart-backlight-emulator.c
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
// Dell AIO Serial Backlight board emulator for testing
// the Linux dell-uart-backlight driver.
//
// Copyright (C) 2024 Hans de Goede <hansg@kernel.org>
//

    int serial_fd;
    let mut brightness: c_int = 50;
#[no_mangle]
unsafe extern "C" fn dell_uart_checksum(buf: *mut c_uchar, len: c_int) -> c_uchar {
    static unsigned char dell_uart_checksum(unsigned char *buf, int len)
    {
    let mut val: c_uchar = 0;
    while (len-- > 0)
    val += buf[len];
    return val ^ 0xff;
    }
// read() will return -1 on SIGINT / SIGTERM causing the mainloop to cleanly exit
#[no_mangle]
pub unsafe extern "C" fn signalhdlr(signum: c_int) {
    void signalhdlr(int signum)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut sigact: sigaction = { .sa_handler = signalhdlr };
    unsigned char buf[4], csum, response[32];
    const char *version_str = "PHI23-V321";
    struct termios tty, saved_tty;
    int ret, idx, len = 0;
    if (argc != 2) {
    fprintf(stderr, "Invalid or missing arguments\n");
    fprintf(stderr, "Usage: %s <serial-port>\n", argv[0]);
    return 1;
    }
    serial_fd = open(argv[1], O_RDWR | O_NOCTTY);
    if (serial_fd == -1) {
    fprintf(stderr, "Error opening %s: %s\n", argv[1], strerror(errno));
    return 1;
    }
    ret = tcgetattr(serial_fd, &tty);
    if (ret == -1) {
    fprintf(stderr, "Error getting tcattr: %s\n", strerror(errno));
    goto out_close;
    }
    saved_tty = tty;
    cfsetspeed(&tty, 9600);
    cfmakeraw(&tty);
    tty.c_cflag &= ~CSTOPB;
    tty.c_cflag &= ~CRTSCTS;
    tty.c_cflag |= CLOCAL | CREAD;
    ret = tcsetattr(serial_fd, TCSANOW, &tty);
    if (ret == -1) {
    fprintf(stderr, "Error setting tcattr: %s\n", strerror(errno));
    goto out_restore;
    }
    sigaction(SIGINT, &sigact, 0);
    sigaction(SIGTERM, &sigact, 0);
    idx = 0;
    while (read(serial_fd, &buf[idx], 1) == 1) {
    if (idx == 0) {
    switch (buf[0]) {
// 3 MSB bits: cmd-len + 01010 SOF marker
    case 0x6a: len = 3; break;
    case 0x8a: len = 4; break;
    default:
    fprintf(stderr, "Error unexpected first byte: 0x%02x\n", buf[0]);
    continue; /* Try to sync up with sender */
    }
    }
// Process msg when len bytes have been received
    if (idx != (len - 1)) {
    idx++;
    continue;
    }
// Reset idx for next command
    idx = 0;
    csum = dell_uart_checksum(buf, len - 1);
    if (buf[len - 1] != csum) {
    fprintf(stderr, "Error checksum mismatch got 0x%02x expected 0x%02x\n",
    buf[len - 1], csum);
    continue;
    }
    switch ((buf[0] << 8) | buf[1]) {
    case 0x6a06: /* cmd = 0x06, get version */
    len = strlen(version_str);
    strcpy((char *)&response[2], version_str);
    printf("Get version, reply: %s\n", version_str);
    break;
    case 0x8a0b: /* cmd = 0x0b, set brightness */
    if (buf[2] > 100) {
    fprintf(stderr, "Error invalid brightness param: %d\n", buf[2]);
    continue;
    }
    len = 0;
    brightness = buf[2];
    printf("Set brightness %d\n", brightness);
    break;
    case 0x6a0c: /* cmd = 0x0c, get brightness */
    len = 1;
    response[2] = brightness;
    printf("Get brightness, reply: %d\n", brightness);
    break;
    case 0x8a0e: /* cmd = 0x0e, set backlight power */
    if (buf[2] != 0 && buf[2] != 1) {
    fprintf(stderr, "Error invalid set power param: %d\n", buf[2]);
    continue;
    }
    len = 0;
    printf("Set power %d\n", buf[2]);
    break;
    default:
    fprintf(stderr, "Error unknown cmd 0x%04x\n",
    (buf[0] << 8) | buf[1]);
    continue;
    }
// Respond with <total-len> <cmd> <data...> <csum>
    response[0] = len + 3; /* response length in bytes */
    response[1] = buf[1];  /* ack cmd */
    csum = dell_uart_checksum(response, len + 2);
    response[len + 2] = csum;
    ret = write(serial_fd, response, response[0]);
    if (ret != (response[0]))
    fprintf(stderr, "Error writing %d bytes: %d\n",
    response[0], ret);
    }
    ret = 0;
    out_restore:
    tcsetattr(serial_fd, TCSANOW, &saved_tty);
    out_close:
    close(serial_fd);
    return ret;
    }
