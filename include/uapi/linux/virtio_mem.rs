//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_mem.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Virtio Mem Device
//
// Copyright Red Hat, Inc. 2020
//
// Authors:
// David Hildenbrand <david@redhat.com>
//
// This header is BSD licensed so anyone can use the definitions
// to implement compatible drivers/servers:
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
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF
// USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT
// OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

//
// Each virtio-mem device manages a dedicated region in physical address
// space. Each device can belong to a single NUMA node, multiple devices
// for a single NUMA node are possible. A virtio-mem device is like a
// "resizable DIMM" consisting of small memory blocks that can be plugged
// or unplugged. The device driver is responsible for (un)plugging memory
// blocks on demand.
//
// Virtio-mem devices can only operate on their assigned memory region in
// order to (un)plug memory. A device cannot (un)plug memory belonging to
// other devices.
//
// The "region_size" corresponds to the maximum amount of memory that can
// be provided by a device. The "size" corresponds to the amount of memory
// that is currently plugged. "requested_size" corresponds to a request
// from the device to the device driver to (un)plug blocks. The
// device driver should try to (un)plug blocks in order to reach the
// "requested_size". It is impossible to plug more memory than requested.
//
// The "usable_region_size" represents the memory region that can actually
// be used to (un)plug memory. It is always at least as big as the
// "requested_size" and will grow dynamically. It will only shrink when
// explicitly triggered (VIRTIO_MEM_REQ_UNPLUG).
//
// There are no guarantees what will happen if unplugged memory is
// read/written. In general, unplugged memory should not be touched, because
// the resulting action is undefined. There is one exception: without
// VIRTIO_MEM_F_UNPLUGGED_INACCESSIBLE, unplugged memory inside the usable
// region can be read, to simplify creation of memory dumps.
//
// It can happen that the device cannot process a request, because it is
// busy. The device driver has to retry later.
//
// Usually, during system resets all memory will get unplugged, so the
// device driver can start with a clean state. However, in specific
// scenarios (if the device is busy) it can happen that the device still
// has memory plugged. The device driver can request to unplug all memory
// (VIRTIO_MEM_REQ_UNPLUG) - which might take a while to succeed if the
// device is busy.
//
// --- virtio-mem: feature bits ---
// node_id is an ACPI PXM and is valid
pub const VIRTIO_MEM_F_ACPI_PXM: c_int = 0;
// unplugged memory must not be accessed
pub const VIRTIO_MEM_F_UNPLUGGED_INACCESSIBLE: c_int = 1;
// plugged memory will remain plugged when suspending+resuming
pub const VIRTIO_MEM_F_PERSISTENT_SUSPEND: c_int = 2;
// --- virtio-mem: guest -> host requests ---
// request to plug memory blocks
pub const VIRTIO_MEM_REQ_PLUG: c_int = 0;
// request to unplug memory blocks
pub const VIRTIO_MEM_REQ_UNPLUG: c_int = 1;
// request to unplug all blocks and shrink the usable size
pub const VIRTIO_MEM_REQ_UNPLUG_ALL: c_int = 2;
// request information about the plugged state of memory blocks
pub const VIRTIO_MEM_REQ_STATE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_plug {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_unplug {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req_state {
    pub addr: __virtio64,
    pub nb_blocks: __virtio16,
    pub padding: [__virtio16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_req {
    pub type: __virtio16,
    pub padding: [__virtio16; 3],
    pub plug: virtio_mem_req_plug,
    pub unplug: virtio_mem_req_unplug,
    pub state: virtio_mem_req_state,
    pub u: },
}

// --- virtio-mem: host -> guest response ---
//
// Request processed successfully, applicable for
// - VIRTIO_MEM_REQ_PLUG
// - VIRTIO_MEM_REQ_UNPLUG
// - VIRTIO_MEM_REQ_UNPLUG_ALL
// - VIRTIO_MEM_REQ_STATE
//
pub const VIRTIO_MEM_RESP_ACK: c_int = 0;
//
// Request denied - e.g. trying to plug more than requested, applicable for
// - VIRTIO_MEM_REQ_PLUG
//
pub const VIRTIO_MEM_RESP_NACK: c_int = 1;
//
// Request cannot be processed right now, try again later, applicable for
// - VIRTIO_MEM_REQ_PLUG
// - VIRTIO_MEM_REQ_UNPLUG
// - VIRTIO_MEM_REQ_UNPLUG_ALL
//
pub const VIRTIO_MEM_RESP_BUSY: c_int = 2;
//
// Error in request (e.g. addresses/alignment), applicable for
// - VIRTIO_MEM_REQ_PLUG
// - VIRTIO_MEM_REQ_UNPLUG
// - VIRTIO_MEM_REQ_STATE
//
pub const VIRTIO_MEM_RESP_ERROR: c_int = 3;
// State of memory blocks is "plugged"
pub const VIRTIO_MEM_STATE_PLUGGED: c_int = 0;
// State of memory blocks is "unplugged"
pub const VIRTIO_MEM_STATE_UNPLUGGED: c_int = 1;
// State of memory blocks is "mixed"
pub const VIRTIO_MEM_STATE_MIXED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_resp_state {
    pub state: __virtio16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_resp {
    pub type: __virtio16,
    pub padding: [__virtio16; 3],
    pub state: virtio_mem_resp_state,
    pub u: },
}

// --- virtio-mem: configuration ---
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_mem_config {
// Block size and alignment. Cannot change.
    pub block_size: __le64,
// Valid with VIRTIO_MEM_F_ACPI_PXM. Cannot change.
    pub node_id: __le16,
    pub padding: [__u8; 6],
// Start address of the memory region. Cannot change.
    pub addr: __le64,
// Region size (maximum). Cannot change.
    pub region_size: __le64,
//
// Currently usable region size. Can grow up to region_size. Can
// shrink due to VIRTIO_MEM_REQ_UNPLUG_ALL (in which case no config
// update will be sent).
//
    pub usable_region_size: __le64,
//
// Currently used size. Changes due to plug/unplug requests, but no
// config updates will be sent.
//
    pub plugged_size: __le64,
// Requested size. New plug requests cannot exceed it. Can change.
    pub requested_size: __le64,
}
