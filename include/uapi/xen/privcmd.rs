//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/xen/privcmd.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR MIT)
//
// privcmd.h
//
// Interface to /proc/xen/privcmd.
//
// Copyright (c) 2003-2005, K A Fraser
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_hypercall {
    pub op: __u64,
    pub arg: [__u64; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_mmap_entry {
    pub va: __u64,
//
// This should be a GFN. It's not possible to change the name because
// it's exposed to the user-space.
//
    pub mfn: __u64,
    pub npages: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_mmap {
    pub num: c_int,
    pub /: *mut *mut domid_t dom; / target domain,
    pub entry: *mut privcmd_mmap_entry __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_mmapbatch {
    pub /: *mut *mut int num; / number of pages to populate,
    pub /: *mut *mut domid_t dom; / target domain,
    pub /: *mut *mut __u64 addr; / virtual address,
    pub with: *mut *mut *mut xen_pfn_t __user arr; / array of mfns - or'd,
}

pub const PRIVCMD_MMAPBATCH_MFN_ERROR: c_uint = 0xf0000000U;
pub const PRIVCMD_MMAPBATCH_PAGED_ERROR: c_uint = 0x80000000U;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_mmapbatch_v2 {
    pub /: *mut *mut unsigned int num; / number of pages to populate,
    pub /: *mut *mut domid_t dom; / target domain,
    pub /: *mut *mut __u64 addr; / virtual address,
    pub /: *const *const *const xen_pfn_t __user arr; / array of mfns,
    pub /: *mut *mut *mut int __user err; / array of error codes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_dm_op_buf {
    pub uptr: *mut void __user,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_dm_op {
    pub dom: domid_t,
    pub num: __u16,
    pub ubufs: *const privcmd_dm_op_buf __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_mmap_resource {
    pub dom: domid_t,
    pub type: __u32,
    pub id: __u32,
    pub idx: __u32,
    pub num: __u64,
    pub addr: __u64,
}

// For privcmd_irqfd::flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_irqfd {
    pub dm_op: __u64,
    pub /: *mut *mut __u32 size; / Size of structure pointed by dm_op,
    pub fd: __u32,
    pub flags: __u32,
    pub dom: domid_t,
    pub pad: [__u8; 2],
}

// For privcmd_ioeventfd::flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_ioeventfd {
    pub ioreq: __u64,
    pub ports: __u64,
    pub addr: __u64,
    pub addr_len: __u32,
    pub event_fd: __u32,
    pub vcpus: __u32,
    pub vq: __u32,
    pub flags: __u32,
    pub dom: domid_t,
    pub pad: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_pcidev_get_gsi {
    pub sbdf: __u32,
    pub gsi: __u32,
}

//
// @cmd: IOCTL_PRIVCMD_HYPERCALL
// @arg: &privcmd_hypercall_t
// Return: Value returned from execution of the specified hypercall.
//
// @cmd: IOCTL_PRIVCMD_MMAPBATCH_V2
// @arg: &struct privcmd_mmapbatch_v2
// Return: 0 on success (i.e., arg->err contains valid error codes for
// each frame).  On an error other than a failed frame remap, -1 is
// returned and errno is set to EINVAL, EFAULT etc.  As an exception,
// if the operation was otherwise successful but any frame failed with
// -ENOENT, then -1 is returned and errno is set to ENOENT.
//

