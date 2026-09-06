//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timers/clocksource-switch.c
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


// Clocksource change test
// by: john stultz (johnstul@us.ibm.com)
// (C) Copyright IBM 2012
// Licensed under the GPLv2
//
// NOTE: This is a meta-test which quickly changes the clocksource and
// then uses other tests to detect problems. Thus this test requires
// that the inconsistency-check and nanosleep tests be present in the
// same directory it is run from.
//
// To build:
// $ gcc clocksource-switch.c -o clocksource-switch -lrt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

#[no_mangle]
pub unsafe extern "C" fn get_clocksources(list[][30]: c_char) -> c_int {
    int get_clocksources(char list[][30])
    {
    int fd, i;
    size_t size;
    char buf[512];
    char *head, *tmp;
    fd = open("/sys/devices/system/clocksource/clocksource0/available_clocksource", O_RDONLY);
    size = read(fd, buf, 512);
    close(fd);
    for (i = 0; i < 10; i++)
    list[i][0] = '\0';
    head = buf;
    i = 0;
    while (head - buf < size) {
// Find the next space
    for (tmp = head; *tmp != ' '; tmp++) {
    if (*tmp == '\n')
    break;
    if (*tmp == '\0')
    break;
    }
// tmp = '\0';
    strcpy(list[i], head);
    head = tmp + 1;
    i++;
    }
    return i-1;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cur_clocksource(buf: *mut c_char, size: usize) -> c_int {
    int get_cur_clocksource(char *buf, size_t size)
    {
    int fd;
    fd = open("/sys/devices/system/clocksource/clocksource0/current_clocksource", O_RDONLY);
    size = read(fd, buf, size);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn change_clocksource(clocksource: *mut c_char) -> c_int {
    int change_clocksource(char *clocksource)
    {
    int fd;
    ssize_t size;
    fd = open("/sys/devices/system/clocksource/clocksource0/current_clocksource", O_WRONLY);
    if (fd < 0)
    return -1;
    size = write(fd, clocksource, strlen(clocksource));
    if (size < 0)
    return -1;
    close(fd);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn run_tests(secs: c_int) -> c_int {
    int run_tests(int secs)
    {
    int ret;
    char buf[255];
    sprintf(buf, "./inconsistency-check -t %i", secs);
    ret = system(buf);
    if (WIFEXITED(ret) && WEXITSTATUS(ret))
    return WEXITSTATUS(ret);
    ret = system("./nanosleep");
    return WIFEXITED(ret) ? WEXITSTATUS(ret) : 0;
    }
    char clocksource_list[10][30];
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char orig_clk[512];
    int count, i, status, opt;
    let mut do_sanity_check: c_int = 1;
    let mut runtime: c_int = 60;
    pid_t pid;
// Process arguments
    while ((opt = getopt(argc, argv, "st:")) != -1) {
    switch (opt) {
    case 's':
    do_sanity_check = 0;
    break;
    case 't':
    runtime = atoi(optarg);
    break;
    default:
    printf("Usage: %s [-s] [-t <secs>]\n", argv[0]);
    printf("	-s: skip sanity checks\n");
    printf("	-t: Number of seconds to run\n");
    exit(-1);
    }
    }
    get_cur_clocksource(orig_clk, 512);
    count = get_clocksources(clocksource_list);
    if (change_clocksource(clocksource_list[0])) {
    printf("Error: You probably need to run this as root\n");
    return -1;
    }
// Check everything is sane before we start switching asynchronously
    if (do_sanity_check) {
    for (i = 0; i < count; i++) {
    ksft_print_msg("Validating clocksource %s\n",
    clocksource_list[i]);
    if (change_clocksource(clocksource_list[i])) {
    status = -1;
    goto out;
    }
    if (run_tests(5)) {
    status = -1;
    goto out;
    }
    }
    }
    ksft_print_msg("Running Asynchronous Switching Tests...\n");
    pid = fork();
    if (!pid)
    return run_tests(runtime);
    while (pid != waitpid(pid, &status, WNOHANG))
    for (i = 0; i < count; i++)
    if (change_clocksource(clocksource_list[i])) {
    status = -1;
    goto out;
    }
    out:
    change_clocksource(orig_clk);
// Print at the end to not mix output with child process
    ksft_print_header();
    ksft_set_plan(1);
    ksft_test_result(!status, "clocksource-switch\n");
    ksft_exit(!status);
    }
