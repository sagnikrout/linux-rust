//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/wireguard/qemu/init.c
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
// Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//
// Macro flag: #define _GNU_SOURCE

    __attribute__((noreturn)) static void poweroff(void)
    {
    fflush(stdout);
    fflush(stderr);
    reboot(RB_AUTOBOOT);
    sleep(30);
    fprintf(stderr, "\x1b[37m\x1b[41m\x1b[1mFailed to power off!!!\x1b[0m\n");
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn panic(what: *const c_char) {
    static void panic(const char *what)
    {
    fprintf(stderr, "\n\n\x1b[37m\x1b[41m\x1b[1mSOMETHING WENT HORRIBLY WRONG\x1b[0m\n\n    \x1b[31m\x1b[1m%s: %s\x1b[0m\n\n\x1b[37m\x1b[44m\x1b[1mPower off...\x1b[0m\n\n", what, strerror(errno));
    poweroff();
    }

#[no_mangle]
unsafe extern "C" fn print_banner() {
    static void print_banner(void)
    {
    struct utsname utsname;
    int len;
    if (uname(&utsname) < 0)
    panic("uname");
    len = strlen("    WireGuard Test Suite on       ") + strlen(utsname.sysname) + strlen(utsname.release) + strlen(utsname.machine);
    printf("\x1b[45m\x1b[33m\x1b[1m%*.s\x1b[0m\n\x1b[45m\x1b[33m\x1b[1m    WireGuard Test Suite on %s %s %s    \x1b[0m\n\x1b[45m\x1b[33m\x1b[1m%*.s\x1b[0m\n\n", len, "", utsname.sysname, utsname.release, utsname.machine, len, "");
    }
#[no_mangle]
unsafe extern "C" fn seed_rng() {
    static void seed_rng(void)
    {
    let mut bits: c_int = 256, fd;
    if (!getrandom(core::ptr::null_mut(), 0, GRND_NONBLOCK))
    return;
    pretty_message("[+] Fake seeding RNG...");
    fd = open("/dev/random", O_WRONLY);
    if (fd < 0)
    panic("open(random)");
    if (ioctl(fd, RNDADDTOENTCNT, &bits) < 0)
    panic("ioctl(RNDADDTOENTCNT)");
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn set_time() {
    static void set_time(void)
    {
    if (time(core::ptr::null_mut()))
    return;
    pretty_message("[+] Setting fake time...");
    if (stime(&(time_t){1433512680}) < 0)
    panic("settimeofday()");
    }
#[no_mangle]
unsafe extern "C" fn mount_filesystems() {
    static void mount_filesystems(void)
    {
    pretty_message("[+] Mounting filesystems...");
    mkdir("/dev", 0755);
    mkdir("/proc", 0755);
    mkdir("/sys", 0755);
    mkdir("/tmp", 0755);
    mkdir("/run", 0755);
    mkdir("/var", 0755);
    if (mount("none", "/dev", "devtmpfs", 0, core::ptr::null_mut()))
    panic("devtmpfs mount");
    if (mount("none", "/proc", "proc", 0, core::ptr::null_mut()))
    panic("procfs mount");
    if (mount("none", "/sys", "sysfs", 0, core::ptr::null_mut()))
    panic("sysfs mount");
    if (mount("none", "/tmp", "tmpfs", 0, core::ptr::null_mut()))
    panic("tmpfs mount");
    if (mount("none", "/run", "tmpfs", 0, core::ptr::null_mut()))
    panic("tmpfs mount");
    if (mount("none", "/sys/kernel/debug", "debugfs", 0, core::ptr::null_mut()))
    ; /* Not a problem if it fails.*/
    if (symlink("/run", "/var/run"))
    panic("run symlink");
    if (symlink("/proc/self/fd", "/dev/fd"))
    panic("fd symlink");
    }
#[no_mangle]
unsafe extern "C" fn enable_logging() {
    static void enable_logging(void)
    {
    int fd;
    pretty_message("[+] Enabling logging...");
    fd = open("/proc/sys/kernel/printk", O_WRONLY);
    if (fd >= 0) {
    if (write(fd, "9\n", 2) != 2)
    panic("write(printk)");
    close(fd);
    }
    fd = open("/proc/sys/debug/exception-trace", O_WRONLY);
    if (fd >= 0) {
    if (write(fd, "1\n", 2) != 2)
    panic("write(exception-trace)");
    close(fd);
    }
    }
#[no_mangle]
unsafe extern "C" fn kmod_selftests() {
    static void kmod_selftests(void)
    {
    FILE *file;
    char line[2048], *start, *pass;
    let mut success: bool = true;
    pretty_message("[+] Module self-tests:");
    file = fopen("/proc/kmsg", "r");
    if (!file)
    panic("fopen(kmsg)");
    if (fcntl(fileno(file), F_SETFL, O_NONBLOCK) < 0)
    panic("fcntl(kmsg, nonblock)");
    while (fgets(line, sizeof(line), file)) {
    start = strstr(line, "wireguard: ");
    if (!start)
    continue;
    start += 11;
// strchrnul(start, '\n') = '\0';
    if (strstr(start, "www.wireguard.com"))
    break;
    pass = strstr(start, ": pass");
    if (!pass || pass[6] != '\0') {
    success = false;
    printf(" \x1b[31m*  %s\x1b[0m\n", start);
    } else
    printf(" \x1b[32m*  %s\x1b[0m\n", start);
    }
    fclose(file);
    if (!success) {
    puts("\x1b[31m\x1b[1m[-] Tests failed! \u2639\x1b[0m");
    poweroff();
    }
    }
#[no_mangle]
unsafe extern "C" fn launch_tests() {
    static void launch_tests(void)
    {
    char cmdline[4096], *success_dev;
    int status, fd;
    pid_t pid;
    pretty_message("[+] Launching tests...");
    pid = fork();
    if (pid == -1)
    panic("fork");
#[no_mangle]
pub unsafe extern "C" fn if(0: pid ==) -> else {
    execl("/init.sh", "init", core::ptr::null_mut());
    panic("exec");
    }
    if (waitpid(pid, &status, 0) < 0)
    panic("waitpid");
    if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
    pretty_message("[+] Tests successful! :-)");
    fd = open("/proc/cmdline", O_RDONLY);
    if (fd < 0)
    panic("open(/proc/cmdline)");
    if (read(fd, cmdline, sizeof(cmdline) - 1) <= 0)
    panic("read(/proc/cmdline)");
    cmdline[sizeof(cmdline) - 1] = '\0';
    for (success_dev = strtok(cmdline, " \n"); success_dev; success_dev = strtok(core::ptr::null_mut(), " \n")) {
    if (strncmp(success_dev, "wg.success=", 11))
    continue;
    memcpy(success_dev + 11 - 5, "/dev/", 5);
    success_dev += 11 - 5;
    break;
    }
    if (!success_dev || !strlen(success_dev))
    panic("Unable to find success device");
    fd = open(success_dev, O_WRONLY);
    if (fd < 0)
    panic("open(success_dev)");
    if (write(fd, "success\n", 8) != 8)
    panic("write(success_dev)");
    close(fd);
    } else {
    const char *why = "unknown cause";
    let mut what: c_int = -1;
    if (WIFEXITED(status)) {
    why = "exit code";
    what = WEXITSTATUS(status);
    } else if (WIFSIGNALED(status)) {
    why = "signal";
    what = WTERMSIG(status);
    }
    printf("\x1b[31m\x1b[1m[-] Tests failed with %s %d! \u2639\x1b[0m\n", why, what);
    }
    }
#[no_mangle]
unsafe extern "C" fn ensure_console() {
    static void ensure_console(void)
    {
    for (unsigned int i = 0; i < 1000; ++i) {
    let mut fd: c_int = open("/dev/console", O_RDWR);
    if (fd < 0) {
    usleep(50000);
    continue;
    }
    dup2(fd, 0);
    dup2(fd, 1);
    dup2(fd, 2);
    close(fd);
    if (write(1, "\0\0\0\0\n", 5) == 5)
    return;
    }
    panic("Unable to open console device");
    }
#[no_mangle]
unsafe extern "C" fn clear_leaks() {
    static void clear_leaks(void)
    {
    int fd;
    fd = open("/sys/kernel/debug/kmemleak", O_WRONLY);
    if (fd < 0)
    return;
    pretty_message("[+] Starting memory leak detection...");
    write(fd, "clear\n", 5);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn check_leaks() {
    static void check_leaks(void)
    {
    int fd;
    fd = open("/sys/kernel/debug/kmemleak", O_WRONLY);
    if (fd < 0)
    return;
    pretty_message("[+] Scanning for memory leaks...");
    sleep(2); /* Wait for any grace periods. */
    write(fd, "scan\n", 5);
    close(fd);
    fd = open("/sys/kernel/debug/kmemleak", O_RDONLY);
    if (fd < 0)
    return;
    if (sendfile(1, fd, core::ptr::null_mut(), 0x7ffff000) > 0)
    panic("Memory leaks encountered");
    close(fd);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    ensure_console();
    print_banner();
    mount_filesystems();
    seed_rng();
    set_time();
    kmod_selftests();
    enable_logging();
    clear_leaks();
    launch_tests();
    check_leaks();
    poweroff();
    return 1;
    }
