//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/xen/gntalloc.h
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


//
// gntalloc.h
//
// Interface to /dev/xen/gntalloc.
//
// Author: Daniel De Graaf <dgdegra@tycho.nsa.gov>
//
// This file is in the public domain.
//

//
// Allocates a new page and creates a new grant reference.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_gntalloc_alloc_gref {
// IN parameters
// The ID of the domain to be given access to the grants.
    pub domid: __u16,
// Flags for this mapping
    pub flags: __u16,
// Number of pages to map
    pub count: __u32,
// OUT parameters
// The offset to be used on a subsequent call to mmap().
    pub index: __u64,
// The grant references of the newly created grant, one per page
// Variable size, depending on count
    pub gref_ids: [__u32; 1],
    pub gref_ids_flex): __DECLARE_FLEX_ARRAY(__u32,,
}

pub const GNTALLOC_FLAG_WRITABLE: c_int = 1;
//
// Deallocates the grant reference, allowing the associated page to be freed if
// no other domains are using it.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_gntalloc_dealloc_gref {
// IN parameters
// The offset returned in the map operation
    pub index: __u64,
// Number of references to unmap
    pub count: __u32,
}

//
// Sets up an unmap notification within the page, so that the other side can do
// cleanup if this side crashes. Required to implement cross-domain robust
// mutexes or close notification on communication channels.
//
// Each mapped page only supports one notification; multiple calls referring to
// the same page overwrite the previous notification. You must clear the
// notification prior to the IOCTL_GNTALLOC_DEALLOC_GREF if you do not want it
// to occur.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_gntalloc_unmap_notify {
// IN parameters
// Offset in the file descriptor for a byte within the page (same as
// used in mmap). If using UNMAP_NOTIFY_CLEAR_BYTE, this is the byte to
// be cleared. Otherwise, it can be any byte in the page whose
// notification we are adjusting.
//
    pub index: __u64,
// Action(s) to take on unmap
    pub action: __u32,
// Event channel to notify
    pub event_channel_port: __u32,
}

// Clear (set to zero) the byte specified by index
pub const UNMAP_NOTIFY_CLEAR_BYTE: c_uint = 0x1;
// Send an interrupt on the indicated event channel
pub const UNMAP_NOTIFY_SEND_EVENT: c_uint = 0x2;
