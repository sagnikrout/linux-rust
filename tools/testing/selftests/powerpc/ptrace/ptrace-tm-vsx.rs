//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/ptrace/ptrace-tm-vsx.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Ptrace test for VMX/VSX registers in the TM context
//
// Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
//

    int shm_id;
    unsigned long *cptr, *pptr;
    unsigned long fp_load[VEC_MAX];
    unsigned long fp_store[VEC_MAX];
    unsigned long fp_load_ckpt[VEC_MAX];
    unsigned long fp_load_ckpt_new[VEC_MAX];
    __attribute__((used)) void load_vsx(void)
    {
    loadvsx(fp_load, 0);
    }
    __attribute__((used)) void load_vsx_ckpt(void)
    {
    loadvsx(fp_load_ckpt, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn tm_vsx() {
    void tm_vsx(void)
    {
    unsigned long result, texasr;
    int ret;
    cptr = (unsigned long *)shmat(shm_id, core::ptr::null_mut(), 0);
    trans:
    cptr[1] = 0;
    asm __volatile__(
    "bl load_vsx_ckpt;"
    "1: ;"
    "tbegin.;"
    "beq 2f;"
    "bl load_vsx;"
    "tsuspend.;"
    "li 7, 1;"
    "stw 7, 0(%[cptr1]);"
    "tresume.;"
    "b .;"
    "tend.;"
    "li 0, 0;"
    "ori %[res], 0, 0;"
    "b 3f;"
    "2: ;"
    "li 0, 1;"
    "ori %[res], 0, 0;"
    "mfspr %[texasr], %[sprn_texasr];"
    "3: ;"
    : [res] "=r" (result), [texasr] "=r" (texasr)
    : [sprn_texasr] "i"  (SPRN_TEXASR), [cptr1] "b" (&cptr[1])
    : "memory", "r0", "r3", "r4",
    "r7", "r8", "r9", "r10", "r11", "lr"
    );
    if (result) {
    if (!cptr[0])
    goto trans;
    shmdt((void *)cptr);
    storevsx(fp_store, 0);
    ret = compare_vsx_vmx(fp_store, fp_load_ckpt_new);
    if (ret)
    exit(1);
    exit(0);
    }
    shmdt((void *)cptr);
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn trace_tm_vsx(child: pid_t) -> c_int {
    int trace_tm_vsx(pid_t child)
    {
    unsigned long vsx[VSX_MAX];
    unsigned long vmx[VMX_MAX + 2][2];
    FAIL_IF(start_trace(child));
    FAIL_IF(show_vsx(child, vsx));
    FAIL_IF(validate_vsx(vsx, fp_load));
    FAIL_IF(show_vmx(child, vmx));
    FAIL_IF(validate_vmx(vmx, fp_load));
    FAIL_IF(show_vsx_ckpt(child, vsx));
    FAIL_IF(validate_vsx(vsx, fp_load_ckpt));
    FAIL_IF(show_vmx_ckpt(child, vmx));
    FAIL_IF(validate_vmx(vmx, fp_load_ckpt));
    memset(vsx, 0, sizeof(vsx));
    memset(vmx, 0, sizeof(vmx));
    load_vsx_vmx(fp_load_ckpt_new, vsx, vmx);
    FAIL_IF(write_vsx_ckpt(child, vsx));
    FAIL_IF(write_vmx_ckpt(child, vmx));
    pptr[0] = 1;
    FAIL_IF(stop_trace(child));
    return TEST_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn ptrace_tm_vsx() -> c_int {
    int ptrace_tm_vsx(void)
    {
    pid_t pid;
    int ret, status, i;
    SKIP_IF_MSG(!have_htm(), "Don't have transactional memory");
    SKIP_IF_MSG(htm_is_synthetic(), "Transactional memory is synthetic");
    shm_id = shmget(IPC_PRIVATE, sizeof(int) * 2, 0777|IPC_CREAT);
    for (i = 0; i < 128; i++) {
    fp_load[i] = 1 + rand();
    fp_load_ckpt[i] = 1 + 2 * rand();
    fp_load_ckpt_new[i] = 1 + 3 * rand();
    }
    pid = fork();
    if (pid < 0) {
    perror("fork() failed");
    return TEST_FAIL;
    }
    if (pid == 0)
    tm_vsx();
    if (pid) {
    pptr = (unsigned long *)shmat(shm_id, core::ptr::null_mut(), 0);
    while (!pptr[1])
    asm volatile("" : : : "memory");
    ret = trace_tm_vsx(pid);
    if (ret) {
    kill(pid, SIGKILL);
    shmdt((void *)pptr);
    shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
    return TEST_FAIL;
    }
    shmdt((void *)pptr);
    ret = wait(&status);
    shmctl(shm_id, IPC_RMID, core::ptr::null_mut());
    if (ret != pid) {
    printf("Child's exit status not captured\n");
    return TEST_FAIL;
    }
    return (WIFEXITED(status) && WEXITSTATUS(status)) ? TEST_FAIL :
    TEST_PASS;
    }
    return TEST_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(ptrace_tm_vsx, "ptrace_tm_vsx");
    }
