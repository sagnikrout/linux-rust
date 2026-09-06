//! Automatically rewritten from C to Rust
//! Source: tools/laptop/freefall/freefall.c
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
// Disk protection for HP/DELL machines.
//
// Copyright 2008 Eric Piel
// Copyright 2009 Pavel Machek <pavel@ucw.cz>
// Copyright 2012 Sonal Santan
// Copyright 2014 Pali Rohár <pali@kernel.org>
//

    static int noled;
    static char unload_heads_path[64];
    static char device_path[32];
    static const char app_name[] = "FREE FALL";
#[no_mangle]
unsafe extern "C" fn set_unload_heads_path(device: *mut c_char) -> c_int {
    static int set_unload_heads_path(char *device)
    {
    if (strlen(device) <= 5 || strncmp(device, "/dev/", 5) != 0)
    return -EINVAL;
    strncpy(device_path, device, sizeof(device_path) - 1);
    snprintf(unload_heads_path, sizeof(unload_heads_path) - 1,
    "/sys/block/%s/device/unload_heads", device+5);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn valid_disk() -> c_int {
    static int valid_disk(void)
    {
    let mut fd: c_int = open(unload_heads_path, O_RDONLY);
    if (fd < 0) {
    perror(unload_heads_path);
    return 0;
    }
    close(fd);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn write_int(path: *mut c_char, i: c_int) {
    static void write_int(char *path, int i)
    {
    char buf[1024];
    let mut fd: c_int = open(path, O_RDWR);
    if (fd < 0) {
    perror("open");
    exit(1);
    }
    sprintf(buf, "%d", i);
    if (write(fd, buf, strlen(buf)) != strlen(buf)) {
    perror("write");
    exit(1);
    }
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn set_led(on: c_int) {
    static void set_led(int on)
    {
    if (noled)
    return;
    write_int("/sys/class/leds/hp::hddprotect/brightness", on);
    }
#[no_mangle]
unsafe extern "C" fn protect(seconds: c_int) {
    static void protect(int seconds)
    {
    const char *str = (seconds == 0) ? "Unparked" : "Parked";
    write_int(unload_heads_path, seconds*1000);
    syslog(LOG_INFO, "%s %s disk head\n", str, device_path);
    }
#[no_mangle]
unsafe extern "C" fn on_ac() -> c_int {
    static int on_ac(void)
    {
// /sys/class/power_supply/AC0/online
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn lid_open() -> c_int {
    static int lid_open(void)
    {
// /proc/acpi/button/lid/LID/state
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn ignore_me(signum: c_int) {
    static void ignore_me(int signum)
    {
    protect(0);
    set_led(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int fd, ret;
    struct stat st;
    struct sched_param param;
    if (argc == 1)
    ret = set_unload_heads_path("/dev/sda");
#[no_mangle]
pub unsafe extern "C" fn if(2: argc ==) -> else {
    else if (argc == 2)
    ret = set_unload_heads_path(argv[1]);
    else
    ret = -EINVAL;
    if (ret || !valid_disk()) {
    fprintf(stderr, "usage: %s <device> (default: /dev/sda)\n",
    argv[0]);
    exit(1);
    }
    fd = open("/dev/freefall", O_RDONLY);
    if (fd < 0) {
    perror("/dev/freefall");
    return EXIT_FAILURE;
    }
    if (stat("/sys/class/leds/hp::hddprotect/brightness", &st))
    noled = 1;
    if (daemon(0, 0) != 0) {
    perror("daemon");
    return EXIT_FAILURE;
    }
    openlog(app_name, LOG_CONS | LOG_PID | LOG_NDELAY, LOG_LOCAL1);
    param.sched_priority = sched_get_priority_max(SCHED_FIFO);
    sched_setscheduler(0, SCHED_FIFO, &param);
    mlockall(MCL_CURRENT|MCL_FUTURE);
    signal(SIGALRM, ignore_me);
    for (;;) {
    unsigned char count;
    ret = read(fd, &count, sizeof(count));
    alarm(0);
    if ((ret == -1) && (errno == EINTR)) {
// Alarm expired, time to unpark the heads
    continue;
    }
    if (ret != sizeof(count)) {
    perror("read");
    break;
    }
    protect(21);
    set_led(1);
    if (1 || on_ac() || lid_open())
    alarm(2);
    else
    alarm(20);
    }
    closelog();
    close(fd);
    return EXIT_SUCCESS;
    }
