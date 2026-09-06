//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/stack_var_off.c
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

// Test read and writes to the stack performed with offsets that are not
// statically known.
//
#[no_mangle]
pub unsafe extern "C" fn test_stack_var_off() {
    void test_stack_var_off(void)
    {
    let mut duration: c_int = 0;
    struct test_stack_var_off *skel;
    skel = test_stack_var_off__open_and_load();
    if (CHECK(!skel, "skel_open", "failed to open skeleton\n"))
    return;
// Give pid to bpf prog so it doesn't trigger for anyone else.
    skel.bss.test_pid = getpid();
// Initialize the probe's input.
    skel.bss.input[0] = 2;
    skel.bss.input[1] = 42;  /* This will be returned in probe_res. */
    if (!ASSERT_OK(test_stack_var_off__attach(skel), "skel_attach"))
    goto cleanup;
// Trigger probe.
    usleep(1);
    if (CHECK(skel.bss.probe_res != 42, "check_probe_res",
    "wrong probe res: %d\n", skel.bss.probe_res))
    goto cleanup;
    cleanup:
    test_stack_var_off__destroy(skel);
    }
