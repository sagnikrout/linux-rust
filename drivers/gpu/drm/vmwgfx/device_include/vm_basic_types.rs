//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/vmwgfx/device_include/vm_basic_types.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2015-2021 VMware, Inc.
//
// Permission is hereby granted, free of charge, to any person
// obtaining a copy of this software and associated documentation
// files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub type uint32 = u32;
pub type int32 = i32;
pub type uint64 = u64;
pub type uint16 = u16;
pub type int16 = i16;
pub type uint8 = u8;
pub type int8 = i8;
pub type PA = uint64;
pub type PPN = uint32;
pub type PPN32 = uint32;
pub type PPN64 = uint64;
pub type Bool = bool;

pub const MBYTES_SHIFT: c_int = 20;

//
// MKS Guest Stats types
//
// Flags for MKSGuestStatInfoEntry::flags below
//
pub const MKS_GUEST_STAT_FLAG_NONE: c_int = 0;

pub const MKS_GUEST_STAT_INSTANCE_DESC_LENGTH: c_int = 1024;
pub const MKS_GUEST_STAT_INSTANCE_MAX_STATS: c_int = 4096;

pub const MKS_GUEST_STAT_AVERAGE_NAME_LENGTH: c_int = 40;

//
// The MKSGuestStatInstanceDescriptor is used as main interface to
// communicate guest stats back to the host code.  The guest must
// allocate an instance of this structure at the start of a page and
// provide the physical address to the host.  From there the host code
// can walk this structure to find other (pinned) pages containing the
// stats data.
//
// Since the MKSGuestStatInfoEntry structures contain userlevel
// pointers, the InstanceDescriptor also contains pointers to the
// beginning of these sections allowing the host side code to correctly
// interpret the pointers.
//
// Because the host side code never acknowledges anything back to the
// guest there is no strict requirement to maintain compatability
// across releases.  If the interface changes the host might not be
// able to log stats, but the guest will continue to run normally.
//
