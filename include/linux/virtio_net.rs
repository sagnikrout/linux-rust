//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_net.h
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

// gso packets without NEEDS_CSUM do not set transport_offset.
// probe and drop if does not match one of the above types.
//
// UFO does not specify ipv4 or 6: try both
// UFO may not include transport header in gso_size.
// Kernel has a special handling for GSO_BY_FRAGS.
// Too small packets are not really GSO ones.
// Header must be checked, and gso_segs computed.
extern "C" {
    pub fn __virtio_net_hdr_to_skb(_arg: skb, _arg: hdr, _arg: little_endian, _arg: hdr->gso_type) -> return;
}
// This function must be called after virtio_net_hdr_from_skb().
// This is a hint as to how much should be linear.
extern "C" {
    pub fn virtio_net_hdr_to_skb(_arg: skb, _arg: hdr, _arg: little_endian) -> return;
}
// Tunnel not supported/negotiated, but the hdr asks for it.
// Either ipv4 or ipv6.
// The UDP tunnel must carry a GSO packet, but no UFO.
// Rely on csum being present.
// Validate offsets.
// Let the basic parsing deal with plain GSO features.
// In case of USO, the inner protocol is still unknown and
// `inner_isv6` is just a guess, additional parsing is needed.
// The previous validation ensures that accessing an ipv4 inner
// network header is safe.
//
// Checksum-related fields validation for the driver
// tunnel csum packets are invalid when the related
// feature has not been negotiated
//
// DATA_VALID is mutually exclusive with NEEDS_CSUM, and GSO
// over UDP tunnel requires the latter
//
// vlan_hlen always refers to the outermost MAC header. That also
// means it refers to the only MAC header, if the packet does not carry
// any encapsulation.
//
// Tunnel support not negotiated but skb ask for it.
// Let the basic parsing deal with plain GSO features.
