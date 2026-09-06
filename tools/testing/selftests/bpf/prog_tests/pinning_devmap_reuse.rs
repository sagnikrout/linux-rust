//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/pinning_devmap_reuse.c
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

#[no_mangle]
pub unsafe extern "C" fn test_pinning_devmap_reuse() {
    void test_pinning_devmap_reuse(void)
    {
    const char *pinpath1 = "/sys/fs/bpf/pinmap1";
    const char *pinpath2 = "/sys/fs/bpf/pinmap2";
    struct test_pinning_devmap *skel1 = core::ptr::null_mut(), *skel2 = core::ptr::null_mut();
    int err;
    DECLARE_LIBBPF_OPTS(bpf_object_open_opts, opts);
// load the object a first time
    skel1 = test_pinning_devmap__open_and_load();
    if (!ASSERT_OK_PTR(skel1, "skel_load1"))
    goto out;
// load the object a second time, re-using the pinned map
    skel2 = test_pinning_devmap__open_and_load();
    if (!ASSERT_OK_PTR(skel2, "skel_load2"))
    goto out;
// we can close the reference safely without
// the map's refcount falling to 0
//
    test_pinning_devmap__destroy(skel1);
    skel1 = core::ptr::null_mut();
// now, swap the pins
    err = renameat2(0, pinpath1, 0, pinpath2, RENAME_EXCHANGE);
    if (!ASSERT_OK(err, "swap pins"))
    goto out;
// load the object again, this time the re-use should fail
    skel1 = test_pinning_devmap__open_and_load();
    if (!ASSERT_ERR_PTR(skel1, "skel_load3"))
    goto out;
    out:
    unlink(pinpath1);
    unlink(pinpath2);
    test_pinning_devmap__destroy(skel1);
    test_pinning_devmap__destroy(skel2);
    }
