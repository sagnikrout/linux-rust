//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_mremap_vdso.c
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
// 32-bit test to check vDSO mremap.
//
// Copyright (c) 2016 Dmitry Safonov
// Suggested-by: Andrew Lutomirski
//
// Can be built statically:
// gcc -Os -Wall -static -m32 test_mremap_vdso.c
//
// Macro flag: #define _GNU_SOURCE

pub const PAGE_SIZE: c_int = 4096;
#[no_mangle]
unsafe extern "C" fn try_to_remap(vdso_addr: *mut c_void, size: c_ulong) -> c_int {
    static int try_to_remap(void *vdso_addr, unsigned long size)
    {
    void *dest_addr, *new_addr;
// Searching for memory location where to remap
    dest_addr = mmap(0, size, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (dest_addr == MAP_FAILED) {
    ksft_print_msg("WARN: mmap failed (%d): %m\n", errno);
    return 0;
    }
    ksft_print_msg("Moving vDSO: [%p, %#lx] . [%p, %#lx]\n",
    vdso_addr, (unsigned long)vdso_addr + size,
    dest_addr, (unsigned long)dest_addr + size);
    fflush(stdout);
    new_addr = mremap(vdso_addr, size, size,
    MREMAP_FIXED|MREMAP_MAYMOVE, dest_addr);
    if ((unsigned long)new_addr == (unsigned long)-1) {
    munmap(dest_addr, size);
    if (errno == EINVAL) {
    ksft_print_msg("vDSO partial move failed, will try with bigger size\n");
    return -1; /* Retry with larger */
    }
    ksft_print_msg("[FAIL]\tmremap failed (%d): %m\n", errno);
    return 1;
    }
    return 0;
    }

pub const MAX_LINE_LEN: c_int = 512;
#[no_mangle]
pub unsafe extern "C" fn vdso_sealed(maps: *mut FILE) -> bool {
    bool vdso_sealed(FILE *maps)
    {
    char line[MAX_LINE_LEN];
    let mut has_vdso: bool = false;
    while (fgets(line, sizeof(line), maps)) {
    if (strstr(line, VDSO_NAME))
    has_vdso = true;
    if (has_vdso && !strncmp(line, VMFLAGS, strlen(VMFLAGS))) {
    if (strstr(line, MSEAL_FLAGS))
    return true;
    return false;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char, envp: *mut c_char) -> c_int {
    int main(int argc, char **argv, char **envp)
    {
    pid_t child;
    FILE *maps;
    ksft_print_header();
    ksft_set_plan(1);
    maps = fopen("/proc/self/smaps", "r");
    if (!maps) {
    ksft_test_result_skip(
    "Could not open /proc/self/smaps, errno=%d\n",
    errno);
    return 0;
    }
    if (vdso_sealed(maps)) {
    ksft_test_result_skip("vdso is sealed\n");
    return 0;
    }
    fclose(maps);
    child = fork();
    if (child == -1)
    ksft_exit_fail_msg("failed to fork (%d): %m\n", errno);
    if (child == 0) {
    let mut vdso_size: c_ulong = PAGE_SIZE;
    unsigned long auxval;
    let mut ret: c_int = -1;
    auxval = getauxval(AT_SYSINFO_EHDR);
    ksft_print_msg("AT_SYSINFO_EHDR is %#lx\n", auxval);
    if (!auxval || auxval == -ENOENT) {
    ksft_print_msg("WARN: getauxval failed\n");
    return 0;
    }
// Simpler than parsing ELF header
    while (ret < 0) {
    ret = try_to_remap((void *)auxval, vdso_size);
    vdso_size += PAGE_SIZE;
    }

// Glibc is likely to explode now - exit with raw syscall
    asm volatile ("int $0x80" : : "a" (__NR_exit), "b" (!!ret));

    syscall(SYS_exit, ret);

    } else {
    int status;
    if (waitpid(child, &status, 0) != child ||
    !WIFEXITED(status))
    ksft_test_result_fail("mremap() of the vDSO does not work on this kernel!\n");
#[no_mangle]
pub unsafe extern "C" fn if(0: WEXITSTATUS(status) !=) -> else {
    else if (WEXITSTATUS(status) != 0)
    ksft_test_result_fail("Child failed with %d\n", WEXITSTATUS(status));
    else
    ksft_test_result_pass("%s\n", __func__);
    }
    ksft_finished();
    }
