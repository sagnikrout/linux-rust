//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/aio_abi.h
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


// include/linux/aio_abi.h
//
// Copyright 2000,2001,2002 Red Hat.
//
// Written by Benjamin LaHaise <bcrl@kvack.org>
//
// Distribute under the terms of the GPLv2 (see ../../COPYING) or under
// the following terms.
//
// Permission to use, copy, modify, and distribute this software and its
// documentation is hereby granted, provided that the above copyright
// notice appears in all copies.  This software is provided without any
// warranty, express or implied.  Red Hat makes no representations about
// the suitability of this software for any purpose.
//
// IN NO EVENT SHALL RED HAT BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT,
// SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES ARISING OUT OF THE USE OF
// THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF RED HAT HAS BEEN ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.
//
// RED HAT DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS IS" BASIS, AND
// RED HAT HAS NO OBLIGATION TO PROVIDE MAINTENANCE, SUPPORT, UPDATES,
// ENHANCEMENTS, OR MODIFICATIONS.
//

pub type aio_context_t = __kernel_ulong_t;
// 4 was the experimental IOCB_CMD_PREADX
//
// Valid flags for the "aio_flags" member of the "struct iocb".
//
// IOCB_FLAG_RESFD - Set if the "aio_resfd" member of the "struct iocb"
// is valid.
// IOCB_FLAG_IOPRIO - Set if the "aio_reqprio" member of the "struct iocb"
// is valid.
//

// read() from /dev/aio returns these structures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_event {
    pub /: *mut *mut __u64 data; / the data field from the iocb,
    pub /: *mut *mut __u64 obj; / what iocb this event came from,
    pub /: *mut *mut __s64 res; / result code for this event,
    pub /: *mut *mut __s64 res2; / secondary result,
}

//
// we always use a 64bit off_t when communicating
// with userland.  its up to libraries to do the
// proper padding and aio_error abstraction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iocb {
// these are internal to the kernel/libc.
    pub /: *mut *mut __u64 aio_data; / data to be returned in event's data,

    pub /: *mut *mut __u32 aio_key; / the kernel sets aio_key to the req #,
    pub /: *mut *mut *mut __kernel_rwf_t aio_rw_flags; / RWF_ flags,

    pub /: *mut *mut *mut __kernel_rwf_t aio_rw_flags; / RWF_ flags,
    pub /: *mut *mut __u32 aio_key; / the kernel sets aio_key to the req #,

// common fields
    pub /: *mut *mut __u16 aio_lio_opcode; / see IOCB_CMD_ above,
    pub aio_reqprio: __s16,
    pub aio_fildes: __u32,
    pub aio_buf: __u64,
    pub aio_nbytes: __u64,
    pub aio_offset: __s64,
// extra parameters
    pub /: *mut *mut *mut __u64 aio_reserved2; / TODO: use this for a (struct sigevent ),
// flags for the "struct iocb"
    pub aio_flags: __u32,
//
// if the IOCB_FLAG_RESFD flag of "aio_flags" is set, this is an
// eventfd to signal AIO readiness to
//
    pub aio_resfd: __u32,
}

