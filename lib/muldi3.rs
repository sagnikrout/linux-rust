//! Automatically rewritten from C to Rust
//! Source: lib/muldi3.c
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

pub const W_TYPE_SIZE: c_int = 32;

// If we still don't have umul_ppmm, define it using plain C.

    do {								\
    unsigned long __x0, __x1, __x2, __x3;			\
    unsigned short __ul, __vl, __uh, __vh;			\
    \
    __ul = __ll_lowpart(u);					\
    __uh = __ll_highpart(u);				\
    __vl = __ll_lowpart(v);					\
    __vh = __ll_highpart(v);				\
    \
    __x0 = (unsigned long) __ul * __vl;			\
    __x1 = (unsigned long) __ul * __vh;			\
    __x2 = (unsigned long) __uh * __vl;			\
    __x3 = (unsigned long) __uh * __vh;			\
    \
    __x1 += __ll_highpart(__x0); /* this can't give carry */\
    __x1 += __x2; /* but this indeed can */			\
    if (__x1 < __x2) /* did we get it? */			\
    __x3 += __ll_B; /* yes, add it in the proper pos */	\
    \
    (w1) = __x3 + __ll_highpart(__x1);			\
    (w0) = __ll_lowpart(__x1) * __ll_B + __ll_lowpart(__x0);\
    } while (0)

    DWunion __w;					\
    umul_ppmm(__w.s.high, __w.s.low, u, v);		\
    __w.ll;						\
    })

#[no_mangle]
pub unsafe extern "C" fn __muldi3(u: c_longlong, v: c_longlong) -> long long notrace {
    long long notrace __muldi3(long long u, long long v)
    {
    let mut uu: DWunion = {.ll = u};
    let mut vv: DWunion = {.ll = v};
    let mut w: DWunion = {.ll = __umulsidi3(uu.s.low, vv.s.low)};
    w.s.high += ((unsigned long) uu.s.low * (unsigned long) vv.s.high
    + (unsigned long) uu.s.high * (unsigned long) vv.s.low);
    return w.ll;
    }
    EXPORT_SYMBOL(__muldi3);
