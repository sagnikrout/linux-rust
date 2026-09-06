//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/sigreturn/sigreturn.c
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

pub const RISCV_V_MAGIC: c_uint = 0x53465457;
pub const DEFAULT_VALUE: c_int = 2;
pub const SIGNAL_HANDLER_OVERRIDE: c_int = 3;
#[no_mangle]
unsafe extern "C" fn simple_handle(sig_no: c_int, info: *mut siginfo_t, vcontext: *mut c_void) {
    static void simple_handle(int sig_no, siginfo_t *info, void *vcontext)
    {
    ucontext_t *context = vcontext;
    context.uc_mcontext.__gregs[REG_PC] = context.uc_mcontext.__gregs[REG_PC] + 4;
    }
#[no_mangle]
unsafe extern "C" fn vector_override(sig_no: c_int, info: *mut siginfo_t, vcontext: *mut c_void) {
    static void vector_override(int sig_no, siginfo_t *info, void *vcontext)
    {
    ucontext_t *context = vcontext;
// vector state
    struct __riscv_extra_ext_header *ext;
    struct __riscv_v_ext_state *v_ext_state;
// Find the vector context.
    ext = (void *)(&context.uc_mcontext.__fpregs);
    if (ext.hdr.magic != RISCV_V_MAGIC) {
    fprintf(stderr, "bad vector magic: %x\n", ext.hdr.magic);
    abort();
    }
    v_ext_state = (void *)((char *)(ext) + sizeof(*ext));
// (int *)v_ext_state->datap = SIGNAL_HANDLER_OVERRIDE;
    context.uc_mcontext.__gregs[REG_PC] = context.uc_mcontext.__gregs[REG_PC] + 4;
    }
#[no_mangle]
unsafe extern "C" fn vector_sigreturn(data: c_int, (*handler)(int: *mut c_void, : *mut siginfo_t, ): *mut c_void) -> c_int {
    static int vector_sigreturn(int data, void (*handler)(int, siginfo_t *, void *))
    {
    int after_sigreturn;
    struct sigaction sig_action = {
    .sa_sigaction = handler,
    .sa_flags = SA_SIGINFO
    };
    sigaction(SIGSEGV, &sig_action, 0);
    asm(".option push				\n\
    .option		arch, +v		\n\
    vsetivli	x0, 1, e32, m1, ta, ma	\n\
    vmv.s.x		v0, %1			\n\

    lw		a0, 0(x0)		\n\
    vmv.x.s		%0, v0			\n\
    .option pop" : "=r" (after_sigreturn) : "r" (data));
    return after_sigreturn;
    }
    TEST(vector_restore)
    {
    int result;
    result = vector_sigreturn(DEFAULT_VALUE, &simple_handle);
    EXPECT_EQ(DEFAULT_VALUE, result);
    }
    TEST(vector_restore_signal_handler_override)
    {
    int result;
    result = vector_sigreturn(DEFAULT_VALUE, &vector_override);
    EXPECT_EQ(SIGNAL_HANDLER_OVERRIDE, result);
    }
    TEST_HARNESS_MAIN
