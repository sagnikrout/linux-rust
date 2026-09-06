//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_mmio.h
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
// Virtio platform device driver
//
// Copyright 2011, ARM Ltd.
//
// Based on Virtio PCI driver by Anthony Liguori, copyright IBM Corp. 2007
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
// Control registers
//
// Magic value ("virt" string) - Read Only
pub const VIRTIO_MMIO_MAGIC_VALUE: c_uint = 0x000;
// Virtio device version - Read Only
pub const VIRTIO_MMIO_VERSION: c_uint = 0x004;
// Virtio device ID - Read Only
pub const VIRTIO_MMIO_DEVICE_ID: c_uint = 0x008;
// Virtio vendor ID - Read Only
pub const VIRTIO_MMIO_VENDOR_ID: c_uint = 0x00c;
// Bitmask of the features supported by the device (host)
// (32 bits per set) - Read Only
pub const VIRTIO_MMIO_DEVICE_FEATURES: c_uint = 0x010;
// Device (host) features set selector - Write Only
pub const VIRTIO_MMIO_DEVICE_FEATURES_SEL: c_uint = 0x014;
// Bitmask of features activated by the driver (guest)
// (32 bits per set) - Write Only
pub const VIRTIO_MMIO_DRIVER_FEATURES: c_uint = 0x020;
// Activated features set selector - Write Only
pub const VIRTIO_MMIO_DRIVER_FEATURES_SEL: c_uint = 0x024;

// Guest's memory page size in bytes - Write Only
pub const VIRTIO_MMIO_GUEST_PAGE_SIZE: c_uint = 0x028;

// Queue selector - Write Only
pub const VIRTIO_MMIO_QUEUE_SEL: c_uint = 0x030;
// Maximum size of the currently selected queue - Read Only
pub const VIRTIO_MMIO_QUEUE_NUM_MAX: c_uint = 0x034;
// Queue size for the currently selected queue - Write Only
pub const VIRTIO_MMIO_QUEUE_NUM: c_uint = 0x038;

// Used Ring alignment for the currently selected queue - Write Only
pub const VIRTIO_MMIO_QUEUE_ALIGN: c_uint = 0x03c;
// Guest's PFN for the currently selected queue - Read Write
pub const VIRTIO_MMIO_QUEUE_PFN: c_uint = 0x040;

// Ready bit for the currently selected queue - Read Write
pub const VIRTIO_MMIO_QUEUE_READY: c_uint = 0x044;
// Queue notifier - Write Only
pub const VIRTIO_MMIO_QUEUE_NOTIFY: c_uint = 0x050;
// Interrupt status - Read Only
pub const VIRTIO_MMIO_INTERRUPT_STATUS: c_uint = 0x060;
// Interrupt acknowledge - Write Only
pub const VIRTIO_MMIO_INTERRUPT_ACK: c_uint = 0x064;
// Device status register - Read Write
pub const VIRTIO_MMIO_STATUS: c_uint = 0x070;
// Selected queue's Descriptor Table address, 64 bits in two halves
pub const VIRTIO_MMIO_QUEUE_DESC_LOW: c_uint = 0x080;
pub const VIRTIO_MMIO_QUEUE_DESC_HIGH: c_uint = 0x084;
// Selected queue's Available Ring address, 64 bits in two halves
pub const VIRTIO_MMIO_QUEUE_AVAIL_LOW: c_uint = 0x090;
pub const VIRTIO_MMIO_QUEUE_AVAIL_HIGH: c_uint = 0x094;
// Selected queue's Used Ring address, 64 bits in two halves
pub const VIRTIO_MMIO_QUEUE_USED_LOW: c_uint = 0x0a0;
pub const VIRTIO_MMIO_QUEUE_USED_HIGH: c_uint = 0x0a4;
// Shared memory region id
pub const VIRTIO_MMIO_SHM_SEL: c_uint = 0x0ac;
// Shared memory region length, 64 bits in two halves
pub const VIRTIO_MMIO_SHM_LEN_LOW: c_uint = 0x0b0;
pub const VIRTIO_MMIO_SHM_LEN_HIGH: c_uint = 0x0b4;
// Shared memory region base address, 64 bits in two halves
pub const VIRTIO_MMIO_SHM_BASE_LOW: c_uint = 0x0b8;
pub const VIRTIO_MMIO_SHM_BASE_HIGH: c_uint = 0x0bc;
// Configuration atomicity value
pub const VIRTIO_MMIO_CONFIG_GENERATION: c_uint = 0x0fc;
// The config space is defined by each driver as
// the per-driver configuration space - Read Write
pub const VIRTIO_MMIO_CONFIG: c_uint = 0x100;
//
// Interrupt flags (re: interrupt status & acknowledge registers)
//

