//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/user_events/dyn_test.c
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
// User Events Dyn Events Test Program
//
// Copyright (c) 2021 Beau Belgrave <beaub@linux.microsoft.com>
//

    const char *dyn_file = "/sys/kernel/tracing/dynamic_events";
    const char *abi_file = "/sys/kernel/tracing/user_events_data";
    const char *enable_file = "/sys/kernel/tracing/events/user_events/__test_event/enable";
#[no_mangle]
unsafe extern "C" fn event_delete() -> c_int {
    static int event_delete(void)
    {
    let mut fd: c_int = open(abi_file, O_RDWR);
    int ret;
    if (fd < 0)
    return -1;
    ret = ioctl(fd, DIAG_IOCSDEL, "__test_event");
    close(fd);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wait_for_delete() -> bool {
    static bool wait_for_delete(void)
    {
    int i;
    for (i = 0; i < 1000; ++i) {
    let mut fd: c_int = open(enable_file, O_RDONLY);
    if (fd == -1)
    return true;
    close(fd);
    usleep(1000);
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn reg_event(fd: c_int, check: *mut c_int, bit: c_int, value: *const c_char) -> c_int {
    static int reg_event(int fd, int *check, int bit, const char *value)
    {
    let mut reg: user_reg = {0};
    reg.size = sizeof(reg);
    reg.name_args = (__u64)value;
    reg.enable_bit = bit;
    reg.enable_addr = (__u64)check;
    reg.enable_size = sizeof(*check);
    if (ioctl(fd, DIAG_IOCSREG, &reg) == -1)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unreg_event(fd: c_int, check: *mut c_int, bit: c_int) -> c_int {
    static int unreg_event(int fd, int *check, int bit)
    {
    let mut unreg: user_unreg = {0};
    unreg.size = sizeof(unreg);
    unreg.disable_bit = bit;
    unreg.disable_addr = (__u64)check;
    return ioctl(fd, DIAG_IOCSUNREG, &unreg);
    }
#[no_mangle]
unsafe extern "C" fn parse_dyn(value: *const c_char) -> c_int {
    static int parse_dyn(const char *value)
    {
    let mut fd: c_int = open(dyn_file, O_RDWR | O_APPEND);
    let mut len: c_int = strlen(value);
    int ret;
    if (fd == -1)
    return -1;
    ret = write(fd, value, len);
    if (ret == len)
    ret = 0;
    else
    ret = -1;
    close(fd);
    if (ret == 0)
    event_delete();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse_abi(check: *mut c_int, value: *const c_char) -> c_int {
    static int parse_abi(int *check, const char *value)
    {
    let mut fd: c_int = open(abi_file, O_RDWR);
    int ret;
    if (fd == -1)
    return -1;
// Until we have persist flags via dynamic events, use the base name
    if (value[0] != 'u' || value[1] != ':') {
    close(fd);
    return -1;
    }
    ret = reg_event(fd, check, 31, value + 2);
    if (ret != -1) {
    if (unreg_event(fd, check, 31) == -1)
    printf("WARN: Couldn't unreg event\n");
    }
    close(fd);
    wait_for_delete();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn parse(check: *mut c_int, value: *const c_char) -> c_int {
    static int parse(int *check, const char *value)
    {
    let mut abi_ret: c_int = parse_abi(check, value);
    let mut dyn_ret: c_int = parse_dyn(value);
// Ensure both ABI and DYN parse the same way
    if (dyn_ret != abi_ret)
    return -1;
    return dyn_ret;
    }
#[no_mangle]
unsafe extern "C" fn check_match(check: *mut c_int, first: *const c_char, second: *const c_char, match: *mut bool) -> c_int {
    static int check_match(int *check, const char *first, const char *second, bool *match)
    {
    let mut fd: c_int = open(abi_file, O_RDWR);
    let mut ret: c_int = -1;
    if (fd == -1)
    return -1;
    if (reg_event(fd, check, 31, first) == -1)
    goto cleanup;
    if (reg_event(fd, check, 30, second) == -1) {
    if (errno == EADDRINUSE) {
// Name is in use, with different fields
// match = false;
    ret = 0;
    }
    goto cleanup;
    }
// match = true;
    ret = 0;
    cleanup:
    unreg_event(fd, check, 31);
    unreg_event(fd, check, 30);
    close(fd);
    wait_for_delete();
    return ret;
    }

    do { \
    bool match; \
    ASSERT_NE(-1, check_match(&self.check, x, y, &match)); \
    ASSERT_EQ(true, match); \
    } while (0)

    do { \
    bool match; \
    ASSERT_NE(-1, check_match(&self.check, x, y, &match)); \
    ASSERT_EQ(false, match); \
    } while (0)

    FIXTURE(user) {
    int check;
    bool umount;
    };
    FIXTURE_SETUP(user) {
    USER_EVENT_FIXTURE_SETUP(return, self.umount);
    }
    FIXTURE_TEARDOWN(user) {
    USER_EVENT_FIXTURE_TEARDOWN(self.umount);
    wait_for_delete();
    }
    TEST_F(user, basic_types) {
// All should work
    TEST_PARSE("u:__test_event u64 a");
    TEST_PARSE("u:__test_event u32 a");
    TEST_PARSE("u:__test_event u16 a");
    TEST_PARSE("u:__test_event u8 a");
    TEST_PARSE("u:__test_event char a");
    TEST_PARSE("u:__test_event unsigned char a");
    TEST_PARSE("u:__test_event int a");
    TEST_PARSE("u:__test_event unsigned int a");
    TEST_PARSE("u:__test_event short a");
    TEST_PARSE("u:__test_event unsigned short a");
    TEST_PARSE("u:__test_event char[20] a");
    TEST_PARSE("u:__test_event unsigned char[20] a");
    TEST_PARSE("u:__test_event char[0x14] a");
    TEST_PARSE("u:__test_event unsigned char[0x14] a");
// Bad size format should fail
    TEST_NPARSE("u:__test_event char[aa] a");
// Large size should fail
    TEST_NPARSE("u:__test_event char[9999] a");
// Long size string should fail
    TEST_NPARSE("u:__test_event char[0x0000000000001] a");
    }
    TEST_F(user, loc_types) {
// All should work
    TEST_PARSE("u:__test_event __data_loc char[] a");
    TEST_PARSE("u:__test_event __data_loc unsigned char[] a");
    TEST_PARSE("u:__test_event __rel_loc char[] a");
    TEST_PARSE("u:__test_event __rel_loc unsigned char[] a");
    }
    TEST_F(user, size_types) {
// Should work
    TEST_PARSE("u:__test_event struct custom a 20");
// Size not specified on struct should fail
    TEST_NPARSE("u:__test_event struct custom a");
// Size specified on non-struct should fail
    TEST_NPARSE("u:__test_event char a 20");
    }
    TEST_F(user, matching) {
// Single name matches
    TEST_MATCH("__test_event u32 a",
    "__test_event u32 a");
// Multiple names match
    TEST_MATCH("__test_event u32 a; u32 b",
    "__test_event u32 a; u32 b");
// Multiple names match with dangling ;
    TEST_MATCH("__test_event u32 a; u32 b",
    "__test_event u32 a; u32 b;");
// Single name doesn't match
    TEST_NMATCH("__test_event u32 a",
    "__test_event u32 b");
// Multiple names don't match
    TEST_NMATCH("__test_event u32 a; u32 b",
    "__test_event u32 b; u32 a");
// Types don't match
    TEST_NMATCH("__test_event u64 a; u64 b",
    "__test_event u32 a; u32 b");
// Struct name and size matches
    TEST_MATCH("__test_event struct my_struct a 20",
    "__test_event struct my_struct a 20");
// Struct name don't match
    TEST_NMATCH("__test_event struct my_struct a 20",
    "__test_event struct my_struct b 20");
// Struct size don't match
    TEST_NMATCH("__test_event struct my_struct a 20",
    "__test_event struct my_struct a 21");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness_run(argc, argv);
    }
