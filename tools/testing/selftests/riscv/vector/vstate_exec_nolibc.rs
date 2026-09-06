//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/vector/vstate_exec_nolibc.c
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

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int rc, pid, status, test_inherit = 0, xtheadvector = 0;
    long ctrl, ctrl_c;
    char *exec_argv[2], *exec_envp[2];
    if (argc > 1 && strcmp(argv[1], "x"))
    test_inherit = 1;
    if (argc > 2 && strcmp(argv[2], "x"))
    xtheadvector = 1;
    ctrl = prctl(PR_RISCV_V_GET_CONTROL, 0, 0, 0, 0);
    if (ctrl == -1) {
    puts("PR_RISCV_V_GET_CONTROL is not supported\n");
    exit(-1);
    }
    if (test_inherit) {
    pid = fork();
    if (pid == -1) {
    puts("fork failed\n");
    exit(-1);
    }
// child
    if (!pid) {
    exec_argv[0] = THIS_PROGRAM;
    exec_argv[1] = core::ptr::null_mut();
    exec_envp[0] = core::ptr::null_mut();
    exec_envp[1] = core::ptr::null_mut();
// launch the program again to check inherit
    rc = execve(THIS_PROGRAM, exec_argv, exec_envp);
    if (rc) {
    puts("child execve failed\n");
    exit(-1);
    }
    }
    } else {
    pid = fork();
    if (pid == -1) {
    puts("fork failed\n");
    exit(-1);
    }
    if (!pid) {
    rc = prctl(PR_RISCV_V_GET_CONTROL, 0, 0, 0, 0);
    if (rc != ctrl) {
    puts("child's vstate_ctrl not equal to parent's\n");
    exit(-1);
    }
    if (xtheadvector)
    asm volatile (".4byte	0x00007ed7");
    else
    asm volatile (".option push\n\t"
    ".option arch, +v\n\t"
    "vsetvli x0, x0, e32, m8, ta, ma\n\t"
    ".option pop\n\t"
    );
    exit(ctrl);
    }
    }
    rc = waitpid(-1, &status, 0);
    if (WIFEXITED(status) && WEXITSTATUS(status) == -1) {
    puts("child exited abnormally\n");
    exit(-1);
    }
    if (WIFSIGNALED(status)) {
    if (WTERMSIG(status) != SIGILL) {
    puts("child was terminated by unexpected signal\n");
    exit(-1);
    }
    if ((ctrl & PR_RISCV_V_VSTATE_CTRL_CUR_MASK) != PR_RISCV_V_VSTATE_CTRL_OFF) {
    puts("child signaled by illegal V access but vstate_ctrl is not off\n");
    exit(-1);
    }
// child terminated, and its vstate_ctrl is off
    exit(ctrl);
    }
    ctrl_c = WEXITSTATUS(status);
    if (test_inherit) {
    if (ctrl & PR_RISCV_V_VSTATE_CTRL_INHERIT) {
    if (!(ctrl_c & PR_RISCV_V_VSTATE_CTRL_INHERIT)) {
    puts("parent has inherit bit, but child has not\n");
    exit(-1);
    }
    }
    rc = (ctrl & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK) >> 2;
    if (rc != PR_RISCV_V_VSTATE_CTRL_DEFAULT) {
    if (rc != (ctrl_c & PR_RISCV_V_VSTATE_CTRL_CUR_MASK)) {
    puts("parent's next setting does not equal to child's\n");
    exit(-1);
    }
    if (!(ctrl & PR_RISCV_V_VSTATE_CTRL_INHERIT)) {
    if ((ctrl_c & PR_RISCV_V_VSTATE_CTRL_NEXT_MASK) !=
    PR_RISCV_V_VSTATE_CTRL_DEFAULT) {
    puts("must clear child's next vstate_ctrl if !inherit\n");
    exit(-1);
    }
    }
    }
    }
    return ctrl;
    }
