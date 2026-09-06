//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_FCOMI.c
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

pub const _GNU_SOURCE: c_int = 1;

pub const __USE_GNU: c_int = 1;

    enum {
    CF = 1 << 0,
    PF = 1 << 2,
    ZF = 1 << 6,
    ARITH = CF | PF | ZF,
    };
    long res_fcomi_pi_1;
    long res_fcomi_1_pi;
    long res_fcomi_1_1;
    long res_fcomi_nan_1;
// sNaN is s|111 1111 1|1xx xxxx xxxx xxxx xxxx xxxx
// qNaN is s|111 1111 1|0xx xxxx xxxx xxxx xxxx xxxx (some x must be nonzero)
    let mut snan: c_int = 0x7fc11111;
    let mut qnan: c_int = 0x7f811111;
    unsigned short snan1[5];
// sNaN80 is s|111 1111 1111 1111 |10xx xx...xx (some x must be nonzero)
    unsigned short snan80[5] = { 0x1111, 0x1111, 0x1111, 0x8111, 0x7fff };
#[no_mangle]
pub unsafe extern "C" fn test(flags: c_long) -> c_int {
    int test(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fld1""\n"
    "	fldpi""\n"
    "	fcomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_1_pi""\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fldpi""\n"
    "	fld1""\n"
    "	fcomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_pi_1""\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fld1""\n"
    "	fld1""\n"
    "	fcomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_1_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_1_pi & ARITH) != (0)) {
    printf("[BAD]\tfcomi_1_pi with flags:%lx\n", flags);
    return 1;
    }
    if ((res_fcomi_pi_1 & ARITH) != (CF)) {
    printf("[BAD]\tfcomi_pi_1 with flags:%lx.%lx\n", flags, res_fcomi_pi_1 & ARITH);
    return 1;
    }
    if ((res_fcomi_1_1 & ARITH) != (ZF)) {
    printf("[BAD]\tfcomi_1_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != 0) {
    printf("[BAD]\tFE_INVALID is set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_qnan(flags: c_long) -> c_int {
    int test_qnan(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	flds	qnan""\n"
    "	fld1""\n"
    "	fnclex""\n"		// fld of a qnan raised FE_INVALID, clear it
    "	fcomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_nan_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_nan_1 & ARITH) != (ZF|CF|PF)) {
    printf("[BAD]\tfcomi_qnan_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != FE_INVALID) {
    printf("[BAD]\tFE_INVALID is not set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn testu_qnan(flags: c_long) -> c_int {
    int testu_qnan(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	flds	qnan""\n"
    "	fld1""\n"
    "	fnclex""\n"		// fld of a qnan raised FE_INVALID, clear it
    "	fucomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_nan_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_nan_1 & ARITH) != (ZF|CF|PF)) {
    printf("[BAD]\tfcomi_qnan_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != 0) {
    printf("[BAD]\tFE_INVALID is set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn testu_snan(flags: c_long) -> c_int {
    int testu_snan(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
// "	flds	snan""\n"	// WRONG, this will convert 32-bit fp snan to a *qnan* in 80-bit fp register!
// "	fstpt	snan1""\n"	// if uncommented, it prints "snan1:7fff c111 1100 0000 0000" - c111, not 8111!
// "	fnclex""\n"		// flds of a snan raised FE_INVALID, clear it
    "	fldt	snan80""\n"	// fldt never raise FE_INVALID
    "	fld1""\n"
    "	fucomi	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	ffree	%%st(1)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_nan_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_nan_1 & ARITH) != (ZF|CF|PF)) {
    printf("[BAD]\tfcomi_qnan_1 with flags:%lx\n", flags);
    return 1;
    }
// printf("snan:%x snan1:%04x %04x %04x %04x %04x\n", snan, snan1[4], snan1[3], snan1[2], snan1[1], snan1[0]);
    if (fetestexcept(FE_INVALID) != FE_INVALID) {
    printf("[BAD]\tFE_INVALID is not set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn testp(flags: c_long) -> c_int {
    int testp(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fld1""\n"
    "	fldpi""\n"
    "	fcomip	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_1_pi""\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fldpi""\n"
    "	fld1""\n"
    "	fcomip	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_pi_1""\n"
    "	push	%0""\n"
    "	popf""\n"
    "	fld1""\n"
    "	fld1""\n"
    "	fcomip	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_1_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_1_pi & ARITH) != (0)) {
    printf("[BAD]\tfcomi_1_pi with flags:%lx\n", flags);
    return 1;
    }
    if ((res_fcomi_pi_1 & ARITH) != (CF)) {
    printf("[BAD]\tfcomi_pi_1 with flags:%lx.%lx\n", flags, res_fcomi_pi_1 & ARITH);
    return 1;
    }
    if ((res_fcomi_1_1 & ARITH) != (ZF)) {
    printf("[BAD]\tfcomi_1_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != 0) {
    printf("[BAD]\tFE_INVALID is set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn testp_qnan(flags: c_long) -> c_int {
    int testp_qnan(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	flds	qnan""\n"
    "	fld1""\n"
    "	fnclex""\n"		// fld of a qnan raised FE_INVALID, clear it
    "	fcomip	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_nan_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_nan_1 & ARITH) != (ZF|CF|PF)) {
    printf("[BAD]\tfcomi_qnan_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != FE_INVALID) {
    printf("[BAD]\tFE_INVALID is not set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn testup_qnan(flags: c_long) -> c_int {
    int testup_qnan(long flags)
    {
    feclearexcept(FE_DIVBYZERO|FE_INEXACT|FE_INVALID|FE_OVERFLOW|FE_UNDERFLOW);
    asm ("\n"
    "	push	%0""\n"
    "	popf""\n"
    "	flds	qnan""\n"
    "	fld1""\n"
    "	fnclex""\n"		// fld of a qnan raised FE_INVALID, clear it
    "	fucomip	%%st(1), %%st" "\n"
    "	ffree	%%st(0)" "\n"
    "	pushf""\n"
    "	pop	res_fcomi_nan_1""\n"
    :
    : "r" (flags)
    );
    if ((res_fcomi_nan_1 & ARITH) != (ZF|CF|PF)) {
    printf("[BAD]\tfcomi_qnan_1 with flags:%lx\n", flags);
    return 1;
    }
    if (fetestexcept(FE_INVALID) != 0) {
    printf("[BAD]\tFE_INVALID is set in %s\n", __func__);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sighandler(sig: c_int) {
    void sighandler(int sig)
    {
    printf("[FAIL]\tGot signal %d, exiting\n", sig);
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char, envp: *mut c_char) -> c_int {
    int main(int argc, char **argv, char **envp)
    {
    let mut err: c_int = 0;
// SIGILL triggers on 32-bit kernels w/o fcomi emulation
// when run with "no387 nofxsr". Other signals are caught
// just in case.
//
    signal(SIGILL, sighandler);
    signal(SIGFPE, sighandler);
    signal(SIGSEGV, sighandler);
    printf("[RUN]\tTesting f[u]comi[p] instructions\n");
    err |= test(0);
    err |= test_qnan(0);
    err |= testu_qnan(0);
    err |= testu_snan(0);
    err |= test(CF|ZF|PF);
    err |= test_qnan(CF|ZF|PF);
    err |= testu_qnan(CF|ZF|PF);
    err |= testu_snan(CF|ZF|PF);
    err |= testp(0);
    err |= testp_qnan(0);
    err |= testup_qnan(0);
    err |= testp(CF|ZF|PF);
    err |= testp_qnan(CF|ZF|PF);
    err |= testup_qnan(CF|ZF|PF);
    if (!err)
    printf("[OK]\tf[u]comi[p]\n");
    else
    printf("[FAIL]\tf[u]comi[p] errors: %d\n", err);
    return err;
    }
