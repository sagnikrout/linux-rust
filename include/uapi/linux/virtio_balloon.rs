//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_balloon.h
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


// This header is BSD licensed so anyone can use the definitions to implement
// compatible drivers/servers.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.

// The feature bitmap for virtio balloon

// Size of a PFN in the balloon interface.
pub const VIRTIO_BALLOON_PFN_SHIFT: c_int = 12;
pub const VIRTIO_BALLOON_CMD_ID_STOP: c_int = 0;
pub const VIRTIO_BALLOON_CMD_ID_DONE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_balloon_config {
// Number of pages host wants Guest to give up.
    pub num_pages: __le32,
// Number of pages we've actually got in balloon.
    pub actual: __le32,
//
// Free page hint command id, readonly by guest.
// Was previously named free_page_report_cmd_id so we
// need to carry that name for legacy support.
//
    pub free_page_hint_cmd_id: __le32,
    pub /: *mut *mut __le32 free_page_report_cmd_id; / deprecated,
}

// Stores PAGE_POISON if page poisoning is in use

pub const VIRTIO_BALLOON_S_NR: c_int = 16;

//
// Memory statistics structure.
// Driver fills an array of these structures and passes to device.
//
// NOTE: fields are laid out in a way that would make compiler add padding
// between and after fields, so we have to use compiler-specific attributes to
// pack it, to disable this padding. This also often causes compiler to
// generate suboptimal code.
//
// We maintain this statistics structure format for backwards compatibility,
// but don't follow this example.
//
// If implementing a similar structure, do something like the below instead:
// struct virtio_balloon_stat {
// __virtio16 tag;
// __u8 reserved[6];
// __virtio64 val;
// };
//
// In other words, add explicit reserved fields to align field and
// structure boundaries at field size, avoiding compiler padding
// without the packed attribute.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_balloon_stat {
    pub tag: __virtio16,
    pub val: __virtio64,
    pub __attribute__((packed)): },
