//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/vDSO/vdso_test_getcpu.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vdso_test_getcpu.c: Sample code to test parse_vdso.c and vDSO getcpu()
//
// Copyright (c) 2020 Arm Ltd
//

    typedef long (*getcpu_t)(unsigned int *, unsigned int *, void *);
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *version = versions[VDSO_VERSION];
    const char **name = (const char **)&names[VDSO_NAMES];
    unsigned long sysinfo_ehdr;
    unsigned int cpu, node;
    getcpu_t get_cpu;
    long ret;
    sysinfo_ehdr = getauxval(AT_SYSINFO_EHDR);
    if (!sysinfo_ehdr) {
    printf("AT_SYSINFO_EHDR is not present!\n");
    return KSFT_SKIP;
    }
    vdso_init_from_sysinfo_ehdr(getauxval(AT_SYSINFO_EHDR));
    get_cpu = (getcpu_t)vdso_sym(version, name[4]);
    if (!get_cpu) {
    printf("Could not find %s\n", name[4]);
    return KSFT_SKIP;
    }
    ret = VDSO_CALL(get_cpu, 3, &cpu, &node, 0);
    if (ret == 0) {
    printf("Running on CPU %u node %u\n", cpu, node);
    } else {
    printf("%s failed\n", name[4]);
    return KSFT_FAIL;
    }
    return 0;
    }
