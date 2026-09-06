//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/mseal_system_mappings/sysmap_is_sealed.c
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
// test system mappings are sealed when
// KCONFIG_MSEAL_SYSTEM_MAPPINGS=y
//
// Macro flag: #define _GNU_SOURCE

pub const MAX_LINE_LEN: c_int = 512;
#[no_mangle]
pub unsafe extern "C" fn has_mapping(name: *mut c_char, maps: *mut FILE) -> bool {
    bool has_mapping(char *name, FILE *maps)
    {
    char line[MAX_LINE_LEN];
    while (fgets(line, sizeof(line), maps)) {
    if (strstr(line, name))
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn mapping_is_sealed(name: *mut c_char, maps: *mut FILE) -> bool {
    bool mapping_is_sealed(char *name, FILE *maps)
    {
    char line[MAX_LINE_LEN];
    while (fgets(line, sizeof(line), maps)) {
    if (!strncmp(line, VMFLAGS, strlen(VMFLAGS))) {
    if (strstr(line, MSEAL_FLAGS))
    return true;
    return false;
    }
    }
    return false;
    }
    FIXTURE(basic) {
    FILE *maps;
    };
    FIXTURE_SETUP(basic)
    {
    self.maps = fopen("/proc/self/smaps", "r");
    if (!self.maps)
    SKIP(return, "Could not open /proc/self/smap, errno=%d",
    errno);
    };
    FIXTURE_TEARDOWN(basic)
    {
    if (self.maps)
    fclose(self.maps);
    };
    FIXTURE_VARIANT(basic)
    {
    char *name;
    bool sealed;
    };
    FIXTURE_VARIANT_ADD(basic, vdso) {
    .name = "[vdso]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, vvar) {
    .name = "[vvar]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, vvar_vclock) {
    .name = "[vvar_vclock]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, sigpage) {
    .name = "[sigpage]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, vectors) {
    .name = "[vectors]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, uprobes) {
    .name = "[uprobes]",
    .sealed = true,
    };
    FIXTURE_VARIANT_ADD(basic, stack) {
    .name = "[stack]",
    .sealed = false,
    };
    TEST_F(basic, check_sealed)
    {
    if (!has_mapping(variant.name, self.maps)) {
    SKIP(return, "could not find the mapping, %s",
    variant.name);
    }
    EXPECT_EQ(variant.sealed,
    mapping_is_sealed(variant.name, self.maps));
    };
    TEST_HARNESS_MAIN
