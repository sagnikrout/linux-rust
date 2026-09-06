//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ceph/ioctl.h
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

pub const CEPH_IOCTL_MAGIC: c_uint = 0x97;
//
// CEPH_IOC_GET_LAYOUT - get file layout or dir layout policy
// CEPH_IOC_SET_LAYOUT - set file layout
// CEPH_IOC_SET_LAYOUT_POLICY - set dir layout policy
//
// The file layout specifies how file data is striped over objects in
// the distributed object store, which object pool they belong to (if
// it differs from the default), and an optional 'preferred osd' to
// store them on.
//
// Files get a new layout based on the policy set on the containing
// directory or one of its ancestors.  The GET_LAYOUT ioctl will let
// you examine the layout for a file or the policy on a directory.
//
// SET_LAYOUT will let you set a layout on a newly created file.  This
// only works immediately after the file is created and before any
// data is written to it.
//
// SET_LAYOUT_POLICY will let you set a layout policy (default layout)
// on a directory that will apply to any new files created in that
// directory (or any child directory that doesn't specify a layout of
// its own).
//
// use u64 to align sanely on all archs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_ioctl_layout {
    pub object_size: __u64 stripe_unit, stripe_count,,
    pub data_pool: __u64,
// obsolete.  new values ignored, always return -1
    pub preferred_osd: __s64,
}

//
// CEPH_IOC_GET_DATALOC - get location of file data in the cluster
//
// Extract identity, address of the OSD and object storing a given
// file offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_ioctl_dataloc {
    pub /: *mut *mut __u64 file_offset; / in+out: file offset,
    pub /: *mut *mut __u64 object_offset; / out: offset in object,
    pub /: *mut *mut __u64 object_no; / out: object #,
    pub /: *mut *mut __u64 object_size; / out: object size,
    pub /: *mut *mut char object_name[64]; / out: object name,
    pub /: *mut *mut __u64 block_offset; / out: offset in block,
    pub /: *mut *mut __u64 block_size; / out: block length,
    pub /: *mut *mut __s64 osd; / out: osd #,
    pub /: *mut *mut sockaddr_storage osd_addr; / out: osd address,
}

//
// CEPH_IOC_LAZYIO - relax consistency
//
// Normally Ceph switches to synchronous IO when multiple clients have
// the file open (and or more for write).  Reads and writes bypass the
// page cache and go directly to the OSD.  Setting this flag on a file
// descriptor will allow buffered IO for this file in cases where the
// application knows it won't interfere with other nodes (or doesn't
// care).
//

//
// CEPH_IOC_SYNCIO - force synchronous IO
//
// This ioctl sets a file flag that forces the synchronous IO that
// bypasses the page cache, even if it is not necessary.  This is
// essentially the opposite behavior of IOC_LAZYIO.  This forces the
// same read/write path as a file opened by multiple clients when one
// or more of those clients is opened for write.
//
// Note that this type of sync IO takes a different path than a file
// opened with O_SYNC/D_SYNC (writes hit the page cache and are
// immediately flushed on page boundaries).  It is very similar to
// O_DIRECT (writes bypass the page cache) excep that O_DIRECT writes
// are not copied (user page must remain stable) and O_DIRECT writes
// have alignment restrictions (on the buffer and file offset).
//

