//! Automatically rewritten from C to Rust
//! Source: lib/tests/seq_buf_kunit.c
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
// KUnit tests for the seq_buf API
//
// Copyright (C) 2025, Google LLC.
//

#[no_mangle]
unsafe extern "C" fn seq_buf_init_test(test: *mut kunit) {
    static void seq_buf_init_test(struct kunit *test)
    {
    char buf[32];
    struct seq_buf s;
    seq_buf_init(&s, buf, sizeof(buf));
    KUNIT_EXPECT_EQ(test, s.size, 32);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_buffer_left(&s), 32);
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 0);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_declare_test(test: *mut kunit) {
    static void seq_buf_declare_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 24);
    KUNIT_EXPECT_EQ(test, s.size, 24);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_buffer_left(&s), 24);
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 0);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_clear_test(test: *mut kunit) {
    static void seq_buf_clear_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 128);
    seq_buf_puts(&s, "hello");
    KUNIT_EXPECT_EQ(test, s.len, 5);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello");
    seq_buf_clear(&s);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_puts_test(test: *mut kunit) {
    static void seq_buf_puts_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 16);
    seq_buf_puts(&s, "hello");
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 5);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello");
    seq_buf_puts(&s, " world");
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 11);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello world");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_puts_overflow_test(test: *mut kunit) {
    static void seq_buf_puts_overflow_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 10);
    seq_buf_puts(&s, "123456789");
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 9);
    seq_buf_puts(&s, "0");
    KUNIT_EXPECT_TRUE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 10);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "123456789");
    seq_buf_clear(&s);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_putc_test(test: *mut kunit) {
    static void seq_buf_putc_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 4);
    seq_buf_putc(&s, 'a');
    seq_buf_putc(&s, 'b');
    seq_buf_putc(&s, 'c');
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 3);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "abc");
    seq_buf_putc(&s, 'd');
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 4);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "abc");
    seq_buf_putc(&s, 'e');
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 4);
    KUNIT_EXPECT_TRUE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "abc");
    seq_buf_clear(&s);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_printf_test(test: *mut kunit) {
    static void seq_buf_printf_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 32);
    seq_buf_printf(&s, "hello %s", "world");
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 11);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello world");
    seq_buf_printf(&s, " %d", 123);
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 15);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello world 123");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_printf_overflow_test(test: *mut kunit) {
    static void seq_buf_printf_overflow_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 16);
    seq_buf_printf(&s, "%lu", 1234567890UL);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 10);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "1234567890");
    seq_buf_printf(&s, "%s", "abcdefghij");
    KUNIT_EXPECT_TRUE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 16);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "1234567890abcde");
    seq_buf_clear(&s);
    KUNIT_EXPECT_EQ(test, s.len, 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "");
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_get_buf_commit_test(test: *mut kunit) {
    static void seq_buf_get_buf_commit_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 16);
    char *buf;
    size_t len;
    len = seq_buf_get_buf(&s, &buf);
    KUNIT_EXPECT_EQ(test, len, 16);
    KUNIT_EXPECT_PTR_NE(test, buf, core::ptr::null_mut());
    memcpy(buf, "hello", 5);
    seq_buf_commit(&s, 5);
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 5);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello");
    len = seq_buf_get_buf(&s, &buf);
    KUNIT_EXPECT_EQ(test, len, 11);
    KUNIT_EXPECT_PTR_NE(test, buf, core::ptr::null_mut());
    memcpy(buf, " worlds!", 8);
    seq_buf_commit(&s, 6);
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 11);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), "hello world");
    len = seq_buf_get_buf(&s, &buf);
    KUNIT_EXPECT_EQ(test, len, 5);
    KUNIT_EXPECT_PTR_NE(test, buf, core::ptr::null_mut());
    seq_buf_commit(&s, -1);
    KUNIT_EXPECT_TRUE(test, seq_buf_has_overflowed(&s));
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_putmem_hex_test(test: *mut kunit) {
    static void seq_buf_putmem_hex_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 24);
    const u8 data[] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };

    const char *expected = "0001020304050607 0809 ";

    const char *expected = "0706050403020100 0908 ";

    KUNIT_EXPECT_EQ(test, seq_buf_putmem_hex(&s, data, sizeof(data)), 0);
    KUNIT_EXPECT_FALSE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), strlen(expected));
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), expected);
    }
#[no_mangle]
unsafe extern "C" fn seq_buf_putmem_hex_overflow_test(test: *mut kunit) {
    static void seq_buf_putmem_hex_overflow_test(struct kunit *test)
    {
    DECLARE_SEQ_BUF(s, 20);
    const u8 data[] = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 };

    const char *expected = "0001020304050607 ";

    const char *expected = "0706050403020100 ";

    KUNIT_EXPECT_EQ(test, seq_buf_putmem_hex(&s, data, sizeof(data)), -1);
    KUNIT_EXPECT_TRUE(test, seq_buf_has_overflowed(&s));
    KUNIT_EXPECT_EQ(test, seq_buf_used(&s), 20);
    KUNIT_EXPECT_STREQ(test, seq_buf_str(&s), expected);
    }
    static struct kunit_case seq_buf_test_cases[] = {
    KUNIT_CASE(seq_buf_init_test),
    KUNIT_CASE(seq_buf_declare_test),
    KUNIT_CASE(seq_buf_clear_test),
    KUNIT_CASE(seq_buf_puts_test),
    KUNIT_CASE(seq_buf_puts_overflow_test),
    KUNIT_CASE(seq_buf_putc_test),
    KUNIT_CASE(seq_buf_printf_test),
    KUNIT_CASE(seq_buf_printf_overflow_test),
    KUNIT_CASE(seq_buf_get_buf_commit_test),
    KUNIT_CASE(seq_buf_putmem_hex_test),
    KUNIT_CASE(seq_buf_putmem_hex_overflow_test),
    {}
    };
    static struct kunit_suite seq_buf_test_suite = {
    .name = "seq_buf",
    .test_cases = seq_buf_test_cases,
    };
    kunit_test_suite(seq_buf_test_suite);
    MODULE_DESCRIPTION("Runtime test cases for seq_buf string API");
    MODULE_LICENSE("GPL");
