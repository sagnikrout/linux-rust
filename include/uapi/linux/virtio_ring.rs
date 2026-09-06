//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_ring.h
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


// An interface for efficient virtio implementation, currently for use by KVM,
// but hopefully others soon.  Do NOT change this since it will
// break existing servers and clients.
//
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
//
// Copyright Rusty Russell IBM Corporation 2007.

// This marks a buffer as continuing via the next field.
pub const VRING_DESC_F_NEXT: c_int = 1;
// This marks a buffer as write-only (otherwise read-only).
pub const VRING_DESC_F_WRITE: c_int = 2;
// This means the buffer contains a list of buffer descriptors.
pub const VRING_DESC_F_INDIRECT: c_int = 4;
//
// Mark a descriptor as available or used in packed ring.
// Notice: they are defined as shifts instead of shifted values.
//
pub const VRING_PACKED_DESC_F_AVAIL: c_int = 7;
pub const VRING_PACKED_DESC_F_USED: c_int = 15;
// The Host uses this in used->flags to advise the Guest: don't kick me when
// you add a buffer.  It's unreliable, so it's simply an optimization.  Guest
// will still kick if it's out of buffers.
pub const VRING_USED_F_NO_NOTIFY: c_int = 1;
// The Guest uses this in avail->flags to advise the Host: don't interrupt me
// when you consume a buffer.  It's unreliable, so it's simply an
// optimization.
pub const VRING_AVAIL_F_NO_INTERRUPT: c_int = 1;
// Enable events in packed ring.
pub const VRING_PACKED_EVENT_FLAG_ENABLE: c_uint = 0x0;
// Disable events in packed ring.
pub const VRING_PACKED_EVENT_FLAG_DISABLE: c_uint = 0x1;
//
// Enable events for a specific descriptor in packed ring.
// (as specified by Descriptor Ring Change Event Offset/Wrap Counter).
// Only valid if VIRTIO_RING_F_EVENT_IDX has been negotiated.
//
pub const VRING_PACKED_EVENT_FLAG_DESC: c_uint = 0x2;
//
// Wrap counter bit shift in event suppression structure
// of packed ring.
//
pub const VRING_PACKED_EVENT_F_WRAP_CTR: c_int = 15;
// We support indirect buffer descriptors
pub const VIRTIO_RING_F_INDIRECT_DESC: c_int = 28;
// The Guest publishes the used index for which it expects an interrupt
// at the end of the avail ring. Host should ignore the avail->flags field.
// The Host publishes the avail index for which it expects a kick
// at the end of the used ring. Guest should ignore the used->flags field.
pub const VIRTIO_RING_F_EVENT_IDX: c_int = 29;
// Alignment requirements for vring elements.
// When using pre-virtio 1.0 layout, these fall out naturally.
//
pub const VRING_AVAIL_ALIGN_SIZE: c_int = 2;
pub const VRING_USED_ALIGN_SIZE: c_int = 4;
pub const VRING_DESC_ALIGN_SIZE: c_int = 16;
//
// struct vring_desc - Virtio ring descriptors,
// 16 bytes long. These can chain together via @next.
//
// @addr: buffer address (guest-physical)
// @len: buffer length
// @flags: descriptor flags
// @next: index of the next descriptor in the chain,
// if the VRING_DESC_F_NEXT flag is set. We chain unused
// descriptors via this, too.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_desc {
    pub addr: __virtio64,
    pub len: __virtio32,
    pub flags: __virtio16,
    pub next: __virtio16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_avail {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: [__virtio16; ],
}

// u32 is used here for ids for padding reasons.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_used_elem {
// Index of start of used descriptor chain.
    pub id: __virtio32,
// Total length of the descriptor chain which was used (written to)
    pub len: __virtio32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_used {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: [vring_used_elem_t; ],
}

//
// The ring element addresses are passed between components with different
// alignments assumptions. Thus, we might need to decrease the compiler-selected
// alignment, and so must use a typedef to make sure the aligned attribute
// actually takes hold:
//
// https://gcc.gnu.org/onlinedocs//gcc/Common-Type-Attributes.html#Common-Type-Attributes
//
// When used on a struct, or struct member, the aligned attribute can only
// increase the alignment; in order to decrease it, the packed attribute must
// be specified as well. When used as part of a typedef, the aligned attribute
// can both increase and decrease alignment, and specifying the packed
// attribute generates a warning.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring {
    pub num: c_uint,
    pub desc: *mut vring_desc_t,
    pub avail: *mut vring_avail_t,
    pub used: *mut vring_used_t,
}

// The standard layout for the ring is a continuous chunk of memory which looks
// like this.  We assume num is a power of 2.
//
// struct vring
// {
// // The actual descriptors (16 bytes each)
// struct vring_desc desc[num];
//
// // A ring of available descriptor heads with free-running index.
// __virtio16 avail_flags;
// __virtio16 avail_idx;
// __virtio16 available[num];
// __virtio16 used_event_idx;
//
// // Padding to the next align boundary.
// char pad[];
//
// // A ring of used descriptor heads with free-running index.
// __virtio16 used_flags;
// __virtio16 used_idx;
// struct vring_used_elem used[num];
// __virtio16 avail_event_idx;
// };
//
// We publish the used event index at the end of the available ring, and vice
// versa. They are at the end for backwards compatibility.

// The following is used with USED_EVENT_IDX and AVAIL_EVENT_IDX
// Assuming a given event_idx value from the other side, if
// we have just incremented index from old to new_idx,
// should we trigger an event?
// Note: Xen has similar logic for notification hold-off
// in include/xen/interface/io/ring.h with req_event and req_prod
// corresponding to event_idx + 1 and new_idx respectively.
// Note also that req_event and req_prod in Xen start at 1,
// event indexes in virtio start at 0.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_packed_desc_event {
// Descriptor Ring Change Event Offset/Wrap Counter.
    pub off_wrap: __le16,
// Descriptor Ring Change Event Flags.
    pub flags: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_packed_desc {
// Buffer Address.
    pub addr: __le64,
// Buffer Length.
    pub len: __le32,
// Buffer ID.
    pub id: __le16,
// The flags depending on descriptor type.
    pub flags: __le16,
}
