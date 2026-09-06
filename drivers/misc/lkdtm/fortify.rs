//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/fortify.c
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
// Copyright (c) 2020 Francis Laniel <laniel_francis@privacyrequired.com>
//
// Add tests related to fortified functions in this file.
//

    static volatile int fortify_scratch_space;
#[no_mangle]
unsafe extern "C" fn lkdtm_FORTIFY_STR_MEMBER() {
    static void lkdtm_FORTIFY_STR_MEMBER(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target {
    pub a: [c_char; 10],
    pub b: [c_char; 10],
    pub target: },
    pub 20: volatile int size =,
    pub src: *mut c_char,
    pub GFP_KERNEL): src = kmalloc(size,,
    if (!src)
// 15 bytes: past end of a[] but not target.
    pub size): strscpy(src, "over ten bytes",,
    pub 1: size = strlen(src) +,
    pub member...\n"): pr_info("trying to strscpy() past the end of a struct,
//
// strscpy(target.a, src, 15); will hit a compile error because the
// compiler knows at build time that target.a < 15 bytes. Use a
// volatile to force a runtime error.
//
    pub size): strscpy(target.a, src,,
// Store result to global to prevent the code from being eliminated
    pub target.a[3]: fortify_scratch_space =,
    pub overflow!\n"): pr_err("FAIL: fortify did not block a strscpy() struct member write,
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_FORTIFY_MEM_OBJECT() {
    static void lkdtm_FORTIFY_MEM_OBJECT(void)
    {
    pub before: [c_int; 10],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target {
    pub a: [c_char; 10],
    pub foo: c_int,
    pub {}: } target =,
    pub after: [c_int; 10],
//
// Using volatile prevents the compiler from determining the value of
// 'size' at compile time. Without that, we would get a compile error
// rather than a runtime error.
//
    pub 20: volatile int size =,
    pub sizeof(before)): memset(before, 0,,
    pub sizeof(after)): memset(after, 0,,
    pub before: [fortify_scratch_space =; 5],
    pub after: [fortify_scratch_space =; 5],
    pub struct\n"): pr_info("trying to memcpy() past the end of a,
    pub 0)): pr_info("0: %zu\n", __builtin_object_size(&target,,
    pub 1)): pr_info("1: %zu\n", __builtin_object_size(&target,,
    pub size): pr_info("s: %d\n",,
    pub size): memcpy(&target, &before,,
// Store result to global to prevent the code from being eliminated
    pub target.a[3]: fortify_scratch_space =,
    pub overflow!\n"): pr_err("FAIL: fortify did not block a memcpy() object write,
    }
#[no_mangle]
unsafe extern "C" fn lkdtm_FORTIFY_MEM_MEMBER() {
    static void lkdtm_FORTIFY_MEM_MEMBER(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target {
    pub a: [c_char; 10],
    pub b: [c_char; 10],
    pub target: },
    pub 20: volatile int size =,
    pub src: *mut c_char,
    pub GFP_KERNEL): src = kmalloc(size,,
    if (!src)
    pub size): strscpy(src, "over ten bytes",,
    pub 1: size = strlen(src) +,
    pub member...\n"): pr_info("trying to memcpy() past the end of a struct,
//
// memcpy(target.a, src, 20); will hit a compile error because the
// compiler knows at build time that target.a < 20 bytes. Use a
// volatile to force a runtime error.
//
    pub size): memcpy(target.a, src,,
// Store result to global to prevent the code from being eliminated
    pub target.a[3]: fortify_scratch_space =,
    pub overflow!\n"): pr_err("FAIL: fortify did not block a memcpy() struct member write,
    }
//
// Calls fortified strscpy to test that it returns the same result as vanilla
// strscpy and generate a panic because there is a write overflow (i.e. src
// length is greater than dst length).
//
#[no_mangle]
unsafe extern "C" fn lkdtm_FORTIFY_STRSCPY() {
    static void lkdtm_FORTIFY_STRSCPY(void)
    {
    pub src: *mut c_char,
    pub dst: [c_char; 5],
    struct {
    union {
    pub big: [c_char; 10],
    pub src: [c_char; 5],
}

    } weird = { .big = "hello!" };
    char weird_dst[sizeof(weird.src) + 1];
    src = kstrdup("foobar", GFP_KERNEL);
    if (src == core::ptr::null_mut())
    return;
// Vanilla strscpy returns -E2BIG if size is 0.
    if (strscpy(dst, src, 0) != -E2BIG)
    pr_warn("FAIL: strscpy() of 0 length did not return -E2BIG\n");
// Vanilla strscpy returns -E2BIG if src is truncated.
    if (strscpy(dst, src, sizeof(dst)) != -E2BIG)
    pr_warn("FAIL: strscpy() did not return -E2BIG while src is truncated\n");
// After above call, dst must contain "foob" because src was truncated.
    if (strncmp(dst, "foob", sizeof(dst)) != 0)
    pr_warn("FAIL: after strscpy() dst does not contain \"foob\" but \"%s\"\n",
    dst);
// Shrink src so the strscpy() below succeeds.
    src[3] = '\0';
//
// Vanilla strscpy returns number of character copied if everything goes
// well.
//
    if (strscpy(dst, src, sizeof(dst)) != 3)
    pr_warn("FAIL: strscpy() did not return 3 while src was copied entirely truncated\n");
// After above call, dst must contain "foo" because src was copied.
    if (strncmp(dst, "foo", sizeof(dst)) != 0)
    pr_warn("FAIL: after strscpy() dst does not contain \"foo\" but \"%s\"\n",
    dst);
// Test when src is embedded inside a union.
    strscpy(weird_dst, weird.src, sizeof(weird_dst));
    if (strcmp(weird_dst, "hello") != 0)
    pr_warn("FAIL: after strscpy() weird_dst does not contain \"hello\" but \"%s\"\n",
    weird_dst);
// Restore src to its initial value.
    src[3] = 'b';
//
// Use strlen here so size cannot be known at compile time and there is
// a runtime write overflow.
//
    strscpy(dst, src, strlen(src));
    pr_err("FAIL: strscpy() overflow not detected!\n");
    pr_expected_config(CONFIG_FORTIFY_SOURCE);
    kfree(src);
    }
    static struct crashtype crashtypes[] = {
    CRASHTYPE(FORTIFY_STR_MEMBER),
    CRASHTYPE(FORTIFY_MEM_OBJECT),
    CRASHTYPE(FORTIFY_MEM_MEMBER),
    CRASHTYPE(FORTIFY_STRSCPY),
    };
    struct crashtype_category fortify_crashtypes = {
    .crashtypes = crashtypes,
    .len	    = ARRAY_SIZE(crashtypes),
    };
