//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/vector/vstate_ptrace.c
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

    int parent_set_val, child_set_val;
#[no_mangle]
unsafe extern "C" fn do_ptrace(op: enum __ptrace_request, pid: pid_t, type: c_long, size: usize, data: *mut c_void) -> c_long {
    static long do_ptrace(enum __ptrace_request op, pid_t pid, long type, size_t size, void *data)
    {
    struct iovec v_iovec = {
    .iov_len = size,
    .iov_base = data
    };
    return ptrace(op, pid, type, &v_iovec);
    }
#[no_mangle]
unsafe extern "C" fn do_child() -> c_int {
    static int do_child(void)
    {
    int out;
    if (ptrace(PTRACE_TRACEME, -1, core::ptr::null_mut(), core::ptr::null_mut())) {
    ksft_perror("PTRACE_TRACEME failed\n");
    return EXIT_FAILURE;
    }
    asm volatile (".option push\n\t"
    ".option	arch, +v\n\t"
    ".option	norvc\n\t"
    "vsetivli	x0, 1, e32, m1, ta, ma\n\t"
    "vmv.s.x	v31, %[in]\n\t"
    "ebreak\n\t"
    "vmv.x.s	%[out], v31\n\t"
    ".option pop\n\t"
    : [out] "=r" (out)
    : [in] "r" (child_set_val));
    if (out != parent_set_val)
    return EXIT_FAILURE;
    return EXIT_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn do_parent(child: pid_t) {
    static void do_parent(pid_t child)
    {
    int status;
    void *data = core::ptr::null_mut();
// Attach to the child
    while (waitpid(child, &status, 0)) {
    if (WIFEXITED(status)) {
    ksft_test_result(WEXITSTATUS(status) == 0, "SETREGSET vector\n");
    goto out;
    } else if (WIFSTOPPED(status) && (WSTOPSIG(status) == SIGTRAP)) {
    size_t size;
    void *data, *v31;
    struct __riscv_v_regset_state *v_regset_hdr;
    struct user_regs_struct *gpreg;
    size = sizeof(*v_regset_hdr);
    data = malloc(size);
    if (!data)
    goto out;
    v_regset_hdr = (struct __riscv_v_regset_state *)data;
    if (do_ptrace(PTRACE_GETREGSET, child, NT_RISCV_VECTOR, size, data))
    goto out;
    ksft_print_msg("vlenb %ld\n", v_regset_hdr.vlenb);
    data = realloc(data, size + v_regset_hdr.vlenb * 32);
    if (!data)
    goto out;
    v_regset_hdr = (struct __riscv_v_regset_state *)data;
    v31 = (void *)(data + size + v_regset_hdr.vlenb * 31);
    size += v_regset_hdr.vlenb * 32;
    if (do_ptrace(PTRACE_GETREGSET, child, NT_RISCV_VECTOR, size, data))
    goto out;
    ksft_test_result(*(int *)v31 == child_set_val, "GETREGSET vector\n");
// (int *)v31 = parent_set_val;
    if (do_ptrace(PTRACE_SETREGSET, child, NT_RISCV_VECTOR, size, data))
    goto out;
// move the pc forward
    size = sizeof(*gpreg);
    data = realloc(data, size);
    gpreg = (struct user_regs_struct *)data;
    if (do_ptrace(PTRACE_GETREGSET, child, NT_PRSTATUS, size, data))
    goto out;
    gpreg.pc += 4;
    if (do_ptrace(PTRACE_SETREGSET, child, NT_PRSTATUS, size, data))
    goto out;
    }
    ptrace(PTRACE_CONT, child, core::ptr::null_mut(), core::ptr::null_mut());
    }
    out:
    free(data);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    pid_t child;
    ksft_set_plan(2);
    if (!is_vector_supported() && !is_xtheadvector_supported())
    ksft_exit_skip("Vector not supported\n");
    srandom(getpid());
    parent_set_val = rand();
    child_set_val = rand();
    child = fork();
    if (child < 0)
    ksft_exit_fail_msg("Fork failed %d\n", child);
    if (!child)
    return do_child();
    do_parent(child);
    ksft_finished();
    }
