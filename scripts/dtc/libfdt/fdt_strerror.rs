//! Automatically rewritten from C to Rust
//! Source: scripts/dtc/libfdt/fdt_strerror.c
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


// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//
// libfdt - Flat Device Tree manipulation
// Copyright (C) 2006 David Gibson, IBM Corporation.
// EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_errtabent {
    pub str: *const c_char,
}

    [(val)] = { .str = #val, }
    static struct fdt_errtabent fdt_errtable[] = {
    FDT_ERRTABENT(FDT_ERR_NOTFOUND),
    FDT_ERRTABENT(FDT_ERR_EXISTS),
    FDT_ERRTABENT(FDT_ERR_NOSPACE),
    FDT_ERRTABENT(FDT_ERR_BADOFFSET),
    FDT_ERRTABENT(FDT_ERR_BADPATH),
    FDT_ERRTABENT(FDT_ERR_BADPHANDLE),
    FDT_ERRTABENT(FDT_ERR_BADSTATE),
    FDT_ERRTABENT(FDT_ERR_TRUNCATED),
    FDT_ERRTABENT(FDT_ERR_BADMAGIC),
    FDT_ERRTABENT(FDT_ERR_BADVERSION),
    FDT_ERRTABENT(FDT_ERR_BADSTRUCTURE),
    FDT_ERRTABENT(FDT_ERR_BADLAYOUT),
    FDT_ERRTABENT(FDT_ERR_INTERNAL),
    FDT_ERRTABENT(FDT_ERR_BADNCELLS),
    FDT_ERRTABENT(FDT_ERR_BADVALUE),
    FDT_ERRTABENT(FDT_ERR_BADOVERLAY),
    FDT_ERRTABENT(FDT_ERR_NOPHANDLES),
    FDT_ERRTABENT(FDT_ERR_BADFLAGS),
    FDT_ERRTABENT(FDT_ERR_ALIGNMENT),
    };

    const char *fdt_strerror(int errval)
    {
    if (errval > 0)
    return "<valid offset/length>";
#[no_mangle]
pub unsafe extern "C" fn if(0: errval ==) -> else {
    else if (errval == 0)
    return "<no error>";
#[no_mangle]
pub unsafe extern "C" fn if(FDT_ERRTABSIZE: -errval <) -> else {
    const char *s = fdt_errtable[-errval].str;
    if (s)
    return s;
    }
    return "<unknown error>";
    }
