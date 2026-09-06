//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/sigreturn_vdso.c
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
// Test that we can take signals with and without the VDSO mapped, which trigger
// different paths in the signal handling code.
//
// See handle_rt_signal64() and setup_trampoline() in signal_64.c
//
// Macro flag: #define _GNU_SOURCE

// Ensure assert() is not compiled out

#[no_mangle]
unsafe extern "C" fn search_proc_maps(needle: *mut c_char, low: *mut c_ulong, high: *mut c_ulong) -> c_int {
    static int search_proc_maps(char *needle, unsigned long *low, unsigned long *high)
    {
    unsigned long start, end;
    static char buf[4096];
    char name[128];
    FILE *f;
    let mut rc: c_int = -1;
    f = fopen("/proc/self/maps", "r");
    if (!f) {
    perror("fopen");
    return -1;
    }
    while (fgets(buf, sizeof(buf), f)) {
    rc = sscanf(buf, "%lx-%lx %*c%*c%*c%*c %*x %*d:%*d %*d %127s\n",
    &start, &end, name);
    if (rc == 2)
    continue;
    if (rc != 3) {
    printf("sscanf errored\n");
    rc = -1;
    break;
    }
    if (strstr(name, needle)) {
// low = start;
// high = end - 1;
    rc = 0;
    break;
    }
    }
    fclose(f);
    return rc;
    }
    let mut took_signal: static volatile sig_atomic_t = 0;
#[no_mangle]
unsafe extern "C" fn sigusr1_handler(sig: c_int) {
    static void sigusr1_handler(int sig)
    {
    took_signal++;
    }
#[no_mangle]
pub unsafe extern "C" fn test_sigreturn_vdso() -> c_int {
    int test_sigreturn_vdso(void)
    {
    unsigned long low, high, size;
    struct sigaction act;
    char *p;
    act.sa_handler = sigusr1_handler;
    act.sa_flags = 0;
    sigemptyset(&act.sa_mask);
    assert(sigaction(SIGUSR1, &act, core::ptr::null_mut()) == 0);
// Confirm the VDSO is mapped, and work out where it is
    assert(search_proc_maps("[vdso]", &low, &high) == 0);
    size = high - low + 1;
    printf("VDSO is at 0x%lx-0x%lx (%lu bytes)\n", low, high, size);
    kill(getpid(), SIGUSR1);
    assert(took_signal == 1);
    printf("Signal delivered OK with VDSO mapped\n");
// Remap the VDSO somewhere else
    p = mmap(core::ptr::null_mut(), size, PROT_READ|PROT_WRITE, MAP_ANONYMOUS|MAP_PRIVATE, -1, 0);
    assert(p != MAP_FAILED);
    assert(mremap((void *)low, size, size, MREMAP_MAYMOVE|MREMAP_FIXED, p) != MAP_FAILED);
    assert(search_proc_maps("[vdso]", &low, &high) == 0);
    size = high - low + 1;
    printf("VDSO moved to 0x%lx-0x%lx (%lu bytes)\n", low, high, size);
    kill(getpid(), SIGUSR1);
    assert(took_signal == 2);
    printf("Signal delivered OK with VDSO moved\n");
    assert(munmap((void *)low, size) == 0);
    printf("Unmapped VDSO\n");
// Confirm the VDSO is not mapped anymore
    assert(search_proc_maps("[vdso]", &low, &high) != 0);
// Make the stack executable
    assert(search_proc_maps("[stack]", &low, &high) == 0);
    size = high - low + 1;
    mprotect((void *)low, size, PROT_READ|PROT_WRITE|PROT_EXEC);
    printf("Remapped the stack executable\n");
    kill(getpid(), SIGUSR1);
    assert(took_signal == 3);
    printf("Signal delivered OK with VDSO unmapped\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_sigreturn_vdso, "sigreturn_vdso");
    }
