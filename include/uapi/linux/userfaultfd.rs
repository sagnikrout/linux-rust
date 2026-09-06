//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/userfaultfd.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// include/linux/userfaultfd.h
//
// Copyright (C) 2007  Davide Libenzi <davidel@xmailserver.org>
// Copyright (C) 2015  Red Hat, Inc.
//

// ioctls for /dev/userfaultfd
pub const USERFAULTFD_IOC: c_uint = 0xAA;

//
// If the UFFDIO_API is upgraded someday, the UFFDIO_UNREGISTER and
// UFFDIO_WAKE ioctls should be defined as _IOW and not as _IOR.  In
// userfaultfd.h we assumed the kernel was reading (instead _IOC_READ
// means the userland is reading).
//

//
// Valid ioctl command number range with this API is from 0x00 to
// 0x3F.  UFFDIO_API is the fixed number, everything else can be
// changed by implementing a different UFFD_API. If sticking to the
// same UFFD_API more ioctl can be added and userland will be aware of
// which ioctl the running kernel implements through the ioctl command
// bitmask written by the UFFDIO_API.
//

// userfaultfd ioctl ids
pub const UFFDIO: c_uint = 0xAA;

// read() structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffd_msg {
    pub event: __u8,
    pub reserved1: __u8,
    pub reserved2: __u16,
    pub reserved3: __u32,
    pub flags: __u64,
    pub address: __u64,
    pub ptid: __u32,
    pub feat: },
    pub pagefault: },
    pub ufd: __u32,
    pub fork: },
    pub from: __u64,
    pub to: __u64,
    pub len: __u64,
    pub remap: },
    pub start: __u64,
    pub end: __u64,
    pub remove: },
// unused reserved fields
    pub reserved1: __u64,
    pub reserved2: __u64,
    pub reserved3: __u64,
    pub reserved: },
    pub arg: },
    pub __packed: },
//
// Start at 0x12 and not at 0 to be more strict against bugs.
//
pub const UFFD_EVENT_PAGEFAULT: c_uint = 0x12;
pub const UFFD_EVENT_FORK: c_uint = 0x13;
pub const UFFD_EVENT_REMAP: c_uint = 0x14;
pub const UFFD_EVENT_REMOVE: c_uint = 0x15;
pub const UFFD_EVENT_UNMAP: c_uint = 0x16;
// flags for UFFD_EVENT_PAGEFAULT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_api {
// userland asks for an API number and the features to enable
    pub api: __u64,
//
// Kernel answers below with the all available features for
// the API, this notifies userland of which events and/or
// which flags for each event are enabled in the current
// kernel.
//
// Note: UFFD_EVENT_PAGEFAULT and UFFD_PAGEFAULT_FLAG_WRITE
// are to be considered implicitly always enabled in all kernels as
// long as the uffdio_api.api requested matches UFFD_API.
//
// UFFD_FEATURE_MISSING_HUGETLBFS means an UFFDIO_REGISTER
// with UFFDIO_REGISTER_MODE_MISSING mode will succeed on
// hugetlbfs virtual memory ranges. Adding or not adding
// UFFD_FEATURE_MISSING_HUGETLBFS to uffdio_api.features has
// no real functional effect after UFFDIO_API returns, but
// it's only useful for an initial feature set probe at
// UFFDIO_API time. There are two ways to use it:
//
// 1) by adding UFFD_FEATURE_MISSING_HUGETLBFS to the
// uffdio_api.features before calling UFFDIO_API, an error
// will be returned by UFFDIO_API on a kernel without
// hugetlbfs missing support
//
// 2) the UFFD_FEATURE_MISSING_HUGETLBFS can not be added in
// uffdio_api.features and instead it will be set by the
// kernel in the uffdio_api.features if the kernel supports
// it, so userland can later check if the feature flag is
// present in uffdio_api.features after UFFDIO_API
// succeeded.
//
// UFFD_FEATURE_MISSING_SHMEM works the same as
// UFFD_FEATURE_MISSING_HUGETLBFS, but it applies to shmem
// (i.e. tmpfs and other shmem based APIs).
//
// UFFD_FEATURE_SIGBUS feature means no page-fault
// (UFFD_EVENT_PAGEFAULT) event will be delivered, instead
// a SIGBUS signal will be sent to the faulting process.
//
// UFFD_FEATURE_THREAD_ID pid of the page faulted task_struct will
// be returned, if feature is not requested 0 will be returned.
//
// UFFD_FEATURE_MINOR_HUGETLBFS indicates that minor faults
// can be intercepted (via REGISTER_MODE_MINOR) for
// hugetlbfs-backed pages.
//
// UFFD_FEATURE_MINOR_SHMEM indicates the same support as
// UFFD_FEATURE_MINOR_HUGETLBFS, but for shmem-backed pages instead.
//
// UFFD_FEATURE_EXACT_ADDRESS indicates that the exact address of page
// faults would be provided and the offset within the page would not be
// masked.
//
// UFFD_FEATURE_WP_HUGETLBFS_SHMEM indicates that userfaultfd
// write-protection mode is supported on both shmem and hugetlbfs.
//
// UFFD_FEATURE_WP_UNPOPULATED indicates that userfaultfd
// write-protection mode will always apply to unpopulated pages
// (i.e. empty ptes).  This will be the default behavior for shmem
// & hugetlbfs, so this flag only affects anonymous memory behavior
// when userfault write-protection mode is registered.
//
// UFFD_FEATURE_WP_ASYNC indicates that userfaultfd write-protection
// asynchronous mode is supported in which the write fault is
// automatically resolved and write-protection is un-set.
// It implies UFFD_FEATURE_WP_UNPOPULATED.
//
// UFFD_FEATURE_MOVE indicates that the kernel supports moving an
// existing page contents from userspace.
//
// UFFD_FEATURE_RWP indicates that the kernel supports
// UFFDIO_REGISTER_MODE_RWP for read-write protection tracking.
// Pages are made inaccessible via UFFDIO_RWPROTECT and faults
// are delivered when the pages are re-accessed.
//
// UFFD_FEATURE_RWP_ASYNC indicates asynchronous mode for
// UFFDIO_REGISTER_MODE_RWP.  When set, faults on read-write
// protected pages are auto-resolved by the kernel (PTE
// permissions restored immediately) without delivering a message
// to the userfaultfd handler.  Use PAGEMAP_SCAN with inverted
// PAGE_IS_ACCESSED to find pages that were not re-accessed.
//

    pub features: __u64,
    pub ioctls: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_range {
    pub start: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_register {
    pub range: uffdio_range,

    pub mode: __u64,
//
// kernel answers which ioctl commands are available for the
// range, keep at the end as the last 8 bytes aren't read.
//
    pub ioctls: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_copy {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,

//
// UFFDIO_COPY_MODE_WP will map the page write protected on
// the fly.  UFFDIO_COPY_MODE_WP is available only if the
// write protected ioctl is implemented for the range
// according to the uffdio_register.ioctls.
//

    pub mode: __u64,
//
// "copy" is written by the ioctl and must be at the end: the
// copy_from_user will not read the last 8 bytes.
//
    pub copy: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_zeropage {
    pub range: uffdio_range,

    pub mode: __u64,
//
// "zeropage" is written by the ioctl and must be at the end:
// the copy_from_user will not read the last 8 bytes.
//
    pub zeropage: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_writeprotect {
    pub range: uffdio_range,
//
// UFFDIO_WRITEPROTECT_MODE_WP: set the flag to write protect a range,
// unset the flag to undo protection of a range which was previously
// write protected.
//
// UFFDIO_WRITEPROTECT_MODE_DONTWAKE: set the flag to avoid waking up
// any wait thread after the operation succeeds.
//
// NOTE: Write protecting a region (WP=1) is unrelated to page faults,
// therefore DONTWAKE flag is meaningless with WP=1.  Removing write
// protection (WP=0) in response to a page fault wakes the faulting
// task unless DONTWAKE is set.
//

    pub mode: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_continue {
    pub range: uffdio_range,

//
// UFFDIO_CONTINUE_MODE_WP will map the page write protected on
// the fly.  UFFDIO_CONTINUE_MODE_WP is available only if the
// write protected ioctl is implemented for the range
// according to the uffdio_register.ioctls.
//

    pub mode: __u64,
//
// Fields below here are written by the ioctl and must be at the end:
// the copy_from_user will not read past here.
//
    pub mapped: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_poison {
    pub range: uffdio_range,

    pub mode: __u64,
//
// Fields below here are written by the ioctl and must be at the end:
// the copy_from_user will not read past here.
//
    pub updated: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_rwprotect {
    pub range: uffdio_range,
// !RWP means undo RWP-protection

    pub mode: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_move {
    pub dst: __u64,
    pub src: __u64,
    pub len: __u64,
//
// Especially if used to atomically remove memory from the
// address space the wake on the dst range is not needed.
//

    pub mode: __u64,
//
// "move" is written by the ioctl and must be at the end: the
// copy_from_user will not read the last 8 bytes.
//
    pub move: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uffdio_set_mode {
//
// Toggle async mode for features at runtime.
// Supported: UFFD_FEATURE_RWP_ASYNC.
// Setting a bit in both enable and disable is invalid.
//
    pub enable: __u64,
    pub disable: __u64,
}

//
// Flags for the userfaultfd(2) system call itself.
//
// Create a userfaultfd that can handle page faults only in user mode.
//
pub const UFFD_USER_MODE_ONLY: c_int = 1;
