//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/leafloop.c
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

// We want to check these symbols in perf script
    noinline void leaf(void);
    noinline void parent(void);
    static volatile sig_atomic_t done asm("leafloop_done");
#[no_mangle]
unsafe extern "C" fn sighandler(__maybe_unused: int sig) {
    static void sighandler(int sig __maybe_unused)
    {
    done = 1;
    }

//
// Write leaf() in assembly so it stays as a minimal leaf function with no
// stack frame and won't get silently broken in the future by any Perf wide
// compilation options like -fstack-protector-all.
//
    asm(
    ".pushsection .text,\"ax\",%progbits\n"
    ".global leaf\n"
    ".type leaf, %function\n"
    "leaf:\n"
    "	adrp	x1, leafloop_done\n"
    "	ldr	w2, [x1, #:lo12:leafloop_done]\n"
    "	cbz	w2, leaf\n"
    "	ret\n"
    ".size leaf, .-leaf\n"
    ".popsection\n"
    );

#[no_mangle]
pub unsafe extern "C" fn leaf() -> noinline void {
    noinline void leaf(void)
    {
    while (!done)
    ;
    }

#[no_mangle]
pub unsafe extern "C" fn parent() -> noinline void {
    noinline void parent(void)
    {
    leaf();
    }
#[no_mangle]
unsafe extern "C" fn leafloop(argc: c_int, argv: *const c_char) -> c_int {
    static int leafloop(int argc, const char **argv)
    {
    let mut sec: c_int = 1;
    if (argc > 0)
    sec = atoi(argv[0]);
    signal(SIGINT, sighandler);
    signal(SIGALRM, sighandler);
    alarm(sec);
    parent();
    return 0;
    }
    DEFINE_WORKLOAD(leafloop);
