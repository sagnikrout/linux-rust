//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_config.h
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


// This header, excluding the #ifdef __KERNEL__ part, is BSD licensed so
// anyone can use the definitions to implement compatible drivers/servers.
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
// Virtio devices use a standardized configuration space to define their
// features and pass configuration information, but each implementation can
// store and access that space differently.

// Status byte for guest to report progress, and synchronize features.
// We have seen device and processed generic fields (VIRTIO_CONFIG_F_VIRTIO)
pub const VIRTIO_CONFIG_S_ACKNOWLEDGE: c_int = 1;
// We have found a driver for the device.
pub const VIRTIO_CONFIG_S_DRIVER: c_int = 2;
// Driver has used its parts of the config, and is happy
pub const VIRTIO_CONFIG_S_DRIVER_OK: c_int = 4;
// Driver has finished configuring features
pub const VIRTIO_CONFIG_S_FEATURES_OK: c_int = 8;
// Device entered invalid state, driver must reset it
pub const VIRTIO_CONFIG_S_NEEDS_RESET: c_uint = 0x40;
// We've given up on this device.
pub const VIRTIO_CONFIG_S_FAILED: c_uint = 0x80;
//
// Virtio feature bits VIRTIO_TRANSPORT_F_START through
// VIRTIO_TRANSPORT_F_END are reserved for the transport
// being used (e.g. virtio_ring, virtio_pci etc.), the
// rest are per-device feature bits.
//
pub const VIRTIO_TRANSPORT_F_START: c_int = 28;
pub const VIRTIO_TRANSPORT_F_END: c_int = 42;

// Do we get callbacks when the ring is completely used, even if we've
// suppressed them?
pub const VIRTIO_F_NOTIFY_ON_EMPTY: c_int = 24;
// Can the device handle any descriptor layout?
pub const VIRTIO_F_ANY_LAYOUT: c_int = 27;

// v1.0 compliant.
pub const VIRTIO_F_VERSION_1: c_int = 32;
//
// If clear - device has the platform DMA (e.g. IOMMU) bypass quirk feature.
// If set - use platform DMA tools to access the memory.
//
// Note the reverse polarity (compared to most other features),
// this is for compatibility with legacy systems.
//
pub const VIRTIO_F_ACCESS_PLATFORM: c_int = 33;
// Legacy name for VIRTIO_F_ACCESS_PLATFORM (for compatibility with old userspace)

// This feature indicates support for the packed virtqueue layout.
pub const VIRTIO_F_RING_PACKED: c_int = 34;
//
// Inorder feature indicates that all buffers are used by the device
// in the same order in which they have been made available.
//
pub const VIRTIO_F_IN_ORDER: c_int = 35;
//
// This feature indicates that memory accesses by the driver and the
// device are ordered in a way described by the platform.
//
pub const VIRTIO_F_ORDER_PLATFORM: c_int = 36;
//
// Does the device support Single Root I/O Virtualization?
//
pub const VIRTIO_F_SR_IOV: c_int = 37;
//
// This feature indicates that the driver passes extra data (besides
// identifying the virtqueue) in its device notifications.
//
pub const VIRTIO_F_NOTIFICATION_DATA: c_int = 38;
// This feature indicates that the driver uses the data provided by the device
// as a virtqueue identifier in available buffer notifications.
//
pub const VIRTIO_F_NOTIF_CONFIG_DATA: c_int = 39;
//
// This feature indicates that the driver can reset a queue individually.
//
pub const VIRTIO_F_RING_RESET: c_int = 40;
//
// This feature indicates that the device support administration virtqueues.
//
pub const VIRTIO_F_ADMIN_VQ: c_int = 41;
