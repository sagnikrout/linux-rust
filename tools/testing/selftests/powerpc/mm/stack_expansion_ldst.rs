//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/stack_expansion_ldst.c
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
// Test that loads/stores expand the stack segment, or trigger a SEGV, in
// various conditions.
//
// Based on test code by Tom Lane.
//

    volatile char *stack_top_ptr;
    volatile unsigned long stack_top_sp;
    volatile char c;
    enum access_type {
    LOAD,
    STORE,
    };
//
// Consume stack until the stack pointer is below @target_sp, then do an access
// (load or store) at offset @delta from either the base of the stack or the
// current stack pointer.
//
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn consume_stack(target_sp: c_ulong, stack_high: c_ulong, delta: c_int, type: enum access_type) -> c_int {
    int consume_stack(unsigned long target_sp, unsigned long stack_high, int delta, enum access_type type)
    {
    unsigned long target;
    char stack_cur;
    if ((unsigned long)&stack_cur > target_sp)
    return consume_stack(target_sp, stack_high, delta, type);
    else {
// We don't really need this, but without it GCC might not
// generate a recursive call above.
    stack_top_ptr = &stack_cur;

    asm volatile ("mr %[sp], %%r1" : [sp] "=r" (stack_top_sp));

    asm volatile ("mov %%rsp, %[sp]" : [sp] "=r" (stack_top_sp));

    target = stack_high - delta + 1;
    volatile char *p = (char *)target;
    if (type == STORE)
// p = c;
    else
    c = *p;
// Do something to prevent the stack frame being popped prior to
// our access above.
    getpid();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn search_proc_maps(needle: *mut c_char, low: *mut c_ulong, high: *mut c_ulong) -> c_int {
    static int search_proc_maps(char *needle, unsigned long *low, unsigned long *high)
    {
    unsigned long start, end;
    static char buf[4096];
    char name[128];
    FILE *f;
    int rc;
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
#[no_mangle]
pub unsafe extern "C" fn child(stack_used: c_uint, delta: c_int, type: enum access_type) -> c_int {
    int child(unsigned int stack_used, int delta, enum access_type type)
    {
    unsigned long low, stack_high;
    assert(search_proc_maps("[stack]", &low, &stack_high) == 0);
    assert(consume_stack(stack_high - stack_used, stack_high, delta, type) == 0);
    printf("Access OK: %s delta %-7d used size 0x%06x stack high 0x%lx top_ptr %p top sp 0x%lx actual used 0x%lx\n",
    type == LOAD ? "load" : "store", delta, stack_used, stack_high,
    stack_top_ptr, stack_top_sp, stack_high - stack_top_sp + 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_one(stack_used: c_uint, delta: c_int, type: enum access_type) -> c_int {
    static int test_one(unsigned int stack_used, int delta, enum access_type type)
    {
    pid_t pid;
    int rc;
    pid = fork();
    if (pid == 0)
    exit(child(stack_used, delta, type));
    assert(waitpid(pid, &rc, 0) != -1);
    if (WIFEXITED(rc) && WEXITSTATUS(rc) == 0)
    return 0;
// We don't expect a non-zero exit that's not a signal
    assert(!WIFEXITED(rc));
    printf("Faulted:   %s delta %-7d used size 0x%06x signal %d\n",
    type == LOAD ? "load" : "store", delta, stack_used,
    WTERMSIG(rc));
    return 1;
    }
// This is fairly arbitrary but is well below any of the targets below,
// so that the delta between the stack pointer and the target is large.

#[no_mangle]
unsafe extern "C" fn test_one_type(type: enum access_type, page_size: c_ulong, rlim_cur: c_ulong) {
    static void test_one_type(enum access_type type, unsigned long page_size, unsigned long rlim_cur)
    {
    unsigned long delta;
// We should be able to access anywhere within the rlimit
    for (delta = page_size; delta <= rlim_cur; delta += page_size)
    assert(test_one(DEFAULT_SIZE, delta, type) == 0);
    assert(test_one(DEFAULT_SIZE, rlim_cur, type) == 0);
// But if we go past the rlimit it should fail
    assert(test_one(DEFAULT_SIZE, rlim_cur + 1, type) != 0);
    }
#[no_mangle]
unsafe extern "C" fn test() -> c_int {
    static int test(void)
    {
    unsigned long page_size;
    struct rlimit rlimit;
    page_size = getpagesize();
    getrlimit(RLIMIT_STACK, &rlimit);
    printf("Stack rlimit is 0x%llx\n", (unsigned long long)rlimit.rlim_cur);
    printf("Testing loads ...\n");
    test_one_type(LOAD, page_size, rlimit.rlim_cur);
    printf("Testing stores ...\n");
    test_one_type(STORE, page_size, rlimit.rlim_cur);
    printf("All OK\n");
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test, "stack_expansion_ldst");
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test();
    }
