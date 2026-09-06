//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/copyloops/exc_validate.c
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


    extern char __start___ex_table[];
    extern char __stop___ex_table[];

#[no_mangle]
unsafe extern "C" fn segv_handler(signr: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    static void segv_handler(int signr, siginfo_t *info, void *ptr)
    {
    ucontext_t *uc = (ucontext_t *)ptr;
    let mut addr: c_ulong = (unsigned long)info.si_addr;
    unsigned long *ip = &UCONTEXT_NIA(uc);
    unsigned long *ex_p = (unsigned long *)__start___ex_table;
    while (ex_p < (unsigned long *)__stop___ex_table) {
    unsigned long insn, fixup;
    insn = *ex_p++;
    fixup = *ex_p++;
    if (insn == *ip) {
// ip = fixup;
    return;
    }
    }
    printf("No exception table match for NIA %lx ADDR %lx\n", *ip, addr);
    abort();
    }
#[no_mangle]
unsafe extern "C" fn setup_segv_handler() {
    static void setup_segv_handler(void)
    {
    struct sigaction action;
    memset(&action, 0, sizeof(action));
    action.sa_sigaction = segv_handler;
    action.sa_flags = SA_SIGINFO;
    sigaction(SIGSEGV, &action, core::ptr::null_mut());
    }
    unsigned long COPY_LOOP(void *to, const void *from, unsigned long size);
    unsigned long test_copy_tofrom_user_reference(void *to, const void *from, unsigned long size);
    static int total_passed;
    static int total_failed;
#[no_mangle]
unsafe extern "C" fn do_one_test(dstp: *mut c_char, srcp: *mut c_char, len: c_ulong) {
    static void do_one_test(char *dstp, char *srcp, unsigned long len)
    {
    unsigned long got, expected;
    got = COPY_LOOP(dstp, srcp, len);
    expected = test_copy_tofrom_user_reference(dstp, srcp, len);
    if (got != expected) {
    total_failed++;
    printf("FAIL from=%p to=%p len=%ld returned %ld, expected %ld\n",
    srcp, dstp, len, got, expected);
// abort();
    } else
    total_passed++;
    }
// #define MAX_LEN 512
pub const MAX_LEN: c_int = 16;
#[no_mangle]
pub unsafe extern "C" fn test_copy_exception() -> c_int {
    int test_copy_exception(void)
    {
    int page_size;
    static char *p, *q;
    unsigned long src, dst, len;
    page_size = getpagesize();
    p = mmap(core::ptr::null_mut(), page_size * 2, PROT_READ|PROT_WRITE,
    MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    if (p == MAP_FAILED) {
    perror("mmap");
    exit(1);
    }
    memset(p, 0, page_size);
    setup_segv_handler();
    if (mprotect(p + page_size, page_size, PROT_NONE)) {
    perror("mprotect");
    exit(1);
    }
    q = p + page_size - MAX_LEN;
    for (src = 0; src < MAX_LEN; src++) {
    for (dst = 0; dst < MAX_LEN; dst++) {
    for (len = 0; len < MAX_LEN+1; len++) {
// printf("from=%p to=%p len=%ld\n", q+dst, q+src, len);
    do_one_test(q+dst, q+src, len);
    }
    }
    }
    printf("Totals:\n");
    printf("  Pass: %d\n", total_passed);
    printf("  Fail: %d\n", total_failed);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_copy_exception, str(COPY_LOOP));
    }
