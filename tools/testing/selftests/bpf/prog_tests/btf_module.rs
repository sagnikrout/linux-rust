//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/btf_module.c
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
// Copyright (c) 2021 Hengqi Chen

    static const char *module_name = "bpf_testmod";
    static const char *symbol_name = "bpf_testmod_test_read";
#[no_mangle]
pub unsafe extern "C" fn test_btf_module() {
    void test_btf_module()
    {
    struct btf *vmlinux_btf, *module_btf;
    __s32 type_id;
    if (!env.has_testmod) {
    test__skip();
    return;
    }
    vmlinux_btf = btf__load_vmlinux_btf();
    if (!ASSERT_OK_PTR(vmlinux_btf, "could not load vmlinux BTF"))
    return;
    module_btf = btf__load_module_btf(module_name, vmlinux_btf);
    if (!ASSERT_OK_PTR(module_btf, "could not load module BTF"))
    goto cleanup;
    type_id = btf__find_by_name(module_btf, symbol_name);
    ASSERT_GT(type_id, 0, "func not found");
    cleanup:
    btf__free(module_btf);
    btf__free(vmlinux_btf);
    }
