//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_strsearch.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 100); /* number of pages */
    } arena SEC(".maps");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glob_test {
    pub str: *const *const char __arena pat,,
    pub expected: bool,
}

#[no_mangle]
unsafe extern "C" fn test(pat: *const char __arena, str: *const char __arena, expected: bool) -> bool {
    static bool test(char const __arena *pat, char const __arena *str, bool expected)
    {
    let mut match: bool = glob_match(pat, str);
    let mut success: bool = match == expected;
// bpf_printk("glob_match %s %s res %d ok %d", pat, str, match, success);
    return success;
    }
//
// The tests are all jammed together in one array to make it simpler
// to place that array in the .init.rodata section.  The obvious
// "array of structures containing char *" has no way to force the
// pointed-to strings to be in a particular section.
//
// Anyway, a test consists of:
// 1. Expected glob_match result: '1' or '0'.
// 2. Pattern to match: null-terminated string
// 3. String to match against: null-terminated string
//
// The list of tests is terminated with a final '\0' instead of
// a glob_match result character.
//
    static const char __arena glob_tests[] =
// Some basic tests
    "1" "a\0" "a\0"
    "0" "a\0" "b\0"
    "0" "a\0" "aa\0"
    "0" "a\0" "\0"
    "1" "\0" "\0"
    "0" "\0" "a\0"
// Simple character class tests
    "1" "[a]\0" "a\0"
    "0" "[a]\0" "b\0"
    "0" "[!a]\0" "a\0"
    "1" "[!a]\0" "b\0"
    "1" "[ab]\0" "a\0"
    "1" "[ab]\0" "b\0"
    "0" "[ab]\0" "c\0"
    "1" "[!ab]\0" "c\0"
    "1" "[a-c]\0" "b\0"
    "0" "[a-c]\0" "d\0"
// Corner cases in character class parsing
    "1" "[a-c-e-g]\0" "-\0"
    "0" "[a-c-e-g]\0" "d\0"
    "1" "[a-c-e-g]\0" "f\0"
    "1" "[]a-ceg-ik[]\0" "a\0"
    "1" "[]a-ceg-ik[]\0" "]\0"
    "1" "[]a-ceg-ik[]\0" "[\0"
    "1" "[]a-ceg-ik[]\0" "h\0"
    "0" "[]a-ceg-ik[]\0" "f\0"
    "0" "[!]a-ceg-ik[]\0" "h\0"
    "0" "[!]a-ceg-ik[]\0" "]\0"
    "1" "[!]a-ceg-ik[]\0" "f\0"
// Simple wild cards
    "1" "?\0" "a\0"
    "0" "?\0" "aa\0"
    "0" "??\0" "a\0"
    "1" "?x?\0" "axb\0"
    "0" "?x?\0" "abx\0"
    "0" "?x?\0" "xab\0"
// Asterisk wild cards (backtracking)
    "0" "*??\0" "a\0"
    "1" "*??\0" "ab\0"
    "1" "*??\0" "abc\0"
    "1" "*??\0" "abcd\0"
    "0" "??*\0" "a\0"
    "1" "??*\0" "ab\0"
    "1" "??*\0" "abc\0"
    "1" "??*\0" "abcd\0"
    "0" "?*?\0" "a\0"
    "1" "?*?\0" "ab\0"
    "1" "?*?\0" "abc\0"
    "1" "?*?\0" "abcd\0"
    "1" "*b\0" "b\0"
    "1" "*b\0" "ab\0"
    "0" "*b\0" "ba\0"
    "1" "*b\0" "bb\0"
    "1" "*b\0" "abb\0"
    "1" "*b\0" "bab\0"
    "1" "*bc\0" "abbc\0"
    "1" "*bc\0" "bc\0"
    "1" "*bc\0" "bbc\0"
    "1" "*bc\0" "bcbc\0"
// Multiple asterisks (complex backtracking)
    "1" "*ac*\0" "abacadaeafag\0"
    "1" "*ac*ae*ag*\0" "abacadaeafag\0"
    "1" "*a*b*[bc]*[ef]*g*\0" "abacadaeafag\0"
    "0" "*a*b*[ef]*[cd]*g*\0" "abacadaeafag\0"
    "1" "*abcd*\0" "abcabcabcabcdefg\0"
    "1" "*ab*cd*\0" "abcabcabcabcdefg\0"
    "1" "*abcd*abcdef*\0" "abcabcdabcdeabcdefg\0"
    "0" "*abcd*\0" "abcabcabcabcefg\0"
    "0" "*ab*cd*\0" "abcabcabcabcefg\0";
    let mut skip: bool = false;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_strsearch(ctx: *mut c_void) -> c_int {
    int arena_strsearch(void *ctx)
    {
    let mut successes: unsigned = 0;
    let mut n: unsigned = 0;
    char const __arena *p = glob_tests;
//
// Tests are jammed together in a string.  The first byte is '1'
// or '0' to indicate the expected outcome, or '\0' to indicate the
// end of the tests.  Then come two null-terminated strings: the
// pattern and the string to match it against.
//
    while (*p) {
    let mut expected: bool = *p++ & 1;
    char const __arena *pat = p;
    cond_break;
    p += bpf_arena_strlen(p) + 1;
    successes += test(pat, p, expected);
    p += bpf_arena_strlen(p) + 1;
    n++;
    }
    n -= successes;
// bpf_printk("glob: %u self-tests passed, %u failed\n", successes, n);
    return n ? -1 : 0;
    }
    char _license[] SEC("license") = "GPL";
