//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_blk.h
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

// Feature bits

// Legacy feature bits

// Old (deprecated) name for VIRTIO_BLK_F_FLUSH.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_config {
// The capacity (in 512-byte sectors).
    pub capacity: __virtio64,
// The maximum segment size (if VIRTIO_BLK_F_SIZE_MAX)
    pub size_max: __virtio32,
// The maximum number of segments (if VIRTIO_BLK_F_SEG_MAX)
    pub seg_max: __virtio32,
// geometry of the device (if VIRTIO_BLK_F_GEOMETRY)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_geometry {
    pub cylinders: __virtio16,
    pub heads: __u8,
    pub sectors: __u8,
    pub geometry: },
// block size of device (if VIRTIO_BLK_F_BLK_SIZE)
    pub blk_size: __virtio32,
// the next 4 entries are guarded by VIRTIO_BLK_F_TOPOLOGY
// exponent for physical block per logical block.
    pub physical_block_exp: __u8,
// alignment offset in logical blocks.
    pub alignment_offset: __u8,
// minimum I/O size without performance penalty in logical blocks.
    pub min_io_size: __virtio16,
// optimal sustained I/O size in logical blocks.
    pub opt_io_size: __virtio32,
// writeback mode (if VIRTIO_BLK_F_CONFIG_WCE)
    pub wce: __u8,
    pub unused: __u8,
// number of vqs, only available when VIRTIO_BLK_F_MQ is set
    pub num_queues: __virtio16,
// the next 3 entries are guarded by VIRTIO_BLK_F_DISCARD
//
// The maximum discard sectors (in 512-byte sectors) for
// one segment.
//
    pub max_discard_sectors: __virtio32,
//
// The maximum number of discard segments in a
// discard command.
//
    pub max_discard_seg: __virtio32,
// Discard commands must be aligned to this number of sectors.
    pub discard_sector_alignment: __virtio32,
// the next 3 entries are guarded by VIRTIO_BLK_F_WRITE_ZEROES
//
// The maximum number of write zeroes sectors (in 512-byte sectors) in
// one segment.
//
    pub max_write_zeroes_sectors: __virtio32,
//
// The maximum number of segments in a write zeroes
// command.
//
    pub max_write_zeroes_seg: __virtio32,
//
// Set if a VIRTIO_BLK_T_WRITE_ZEROES request may result in the
// deallocation of one or more of the sectors.
//
    pub write_zeroes_may_unmap: __u8,
    pub unused1: [__u8; 3],
// the next 3 entries are guarded by VIRTIO_BLK_F_SECURE_ERASE
//
// The maximum secure erase sectors (in 512-byte sectors) for
// one segment.
//
    pub max_secure_erase_sectors: __virtio32,
//
// The maximum number of secure erase segments in a
// secure erase command.
//
    pub max_secure_erase_seg: __virtio32,
// Secure erase commands must be aligned to this number of sectors.
    pub secure_erase_sector_alignment: __virtio32,
// Zoned block device characteristics (if VIRTIO_BLK_F_ZONED)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_zoned_characteristics {
    pub zone_sectors: __virtio32,
    pub max_open_zones: __virtio32,
    pub max_active_zones: __virtio32,
    pub max_append_sectors: __virtio32,
    pub write_granularity: __virtio32,
    pub model: __u8,
    pub unused2: [__u8; 3],
    pub zoned: },
    pub __attribute__((packed)): },
//
// Command types
//
// Usage is a bit tricky as some bits are used as flags and some are not.
//
// Rules:
// VIRTIO_BLK_T_OUT may be combined with VIRTIO_BLK_T_SCSI_CMD or
// VIRTIO_BLK_T_BARRIER.  VIRTIO_BLK_T_FLUSH is a command of its own
// and may not be combined with any of the other flags.
//
// These two define direction.
pub const VIRTIO_BLK_T_IN: c_int = 0;
pub const VIRTIO_BLK_T_OUT: c_int = 1;

// This bit says it's a scsi command, not an actual read or write.
pub const VIRTIO_BLK_T_SCSI_CMD: c_int = 2;

// Cache flush command
pub const VIRTIO_BLK_T_FLUSH: c_int = 4;
// Get device ID command
pub const VIRTIO_BLK_T_GET_ID: c_int = 8;
// Discard command
pub const VIRTIO_BLK_T_DISCARD: c_int = 11;
// Write zeroes command
pub const VIRTIO_BLK_T_WRITE_ZEROES: c_int = 13;
// Secure erase command
pub const VIRTIO_BLK_T_SECURE_ERASE: c_int = 14;
// Zone append command
pub const VIRTIO_BLK_T_ZONE_APPEND: c_int = 15;
// Report zones command
pub const VIRTIO_BLK_T_ZONE_REPORT: c_int = 16;
// Open zone command
pub const VIRTIO_BLK_T_ZONE_OPEN: c_int = 18;
// Close zone command
pub const VIRTIO_BLK_T_ZONE_CLOSE: c_int = 20;
// Finish zone command
pub const VIRTIO_BLK_T_ZONE_FINISH: c_int = 22;
// Reset zone command
pub const VIRTIO_BLK_T_ZONE_RESET: c_int = 24;
// Reset All zones command
pub const VIRTIO_BLK_T_ZONE_RESET_ALL: c_int = 26;

// Barrier before this op.
pub const VIRTIO_BLK_T_BARRIER: c_uint = 0x80000000;

//
// This comes first in the read scatter-gather list.
// For legacy virtio, if VIRTIO_F_ANY_LAYOUT is not negotiated,
// this is the first element of the read scatter-gather list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_outhdr {
// VIRTIO_BLK_T*
    pub type: __virtio32,
// io priority.
    pub ioprio: __virtio32,
// Sector (ie. 512 byte offset)
    pub sector: __virtio64,
}

//
// Supported zoned device models.
//
// Regular block device
pub const VIRTIO_BLK_Z_NONE: c_int = 0;
// Host-managed zoned device
pub const VIRTIO_BLK_Z_HM: c_int = 1;
// Host-aware zoned device
pub const VIRTIO_BLK_Z_HA: c_int = 2;
//
// Zone descriptor. A part of VIRTIO_BLK_T_ZONE_REPORT command reply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_zone_descriptor {
// Zone capacity
    pub z_cap: __virtio64,
// The starting sector of the zone
    pub z_start: __virtio64,
// Zone write pointer position in sectors
    pub z_wp: __virtio64,
// Zone type
    pub z_type: __u8,
// Zone state
    pub z_state: __u8,
    pub reserved: [__u8; 38],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_zone_report {
    pub nr_zones: __virtio64,
    pub reserved: [__u8; 56],
    pub zones: [virtio_blk_zone_descriptor; ],
}

//
// Supported zone types.
//
// Conventional zone
pub const VIRTIO_BLK_ZT_CONV: c_int = 1;
// Sequential Write Required zone
pub const VIRTIO_BLK_ZT_SWR: c_int = 2;
// Sequential Write Preferred zone
pub const VIRTIO_BLK_ZT_SWP: c_int = 3;
//
// Zone states that are available for zones of all types.
//
// Not a write pointer (conventional zones only)
pub const VIRTIO_BLK_ZS_NOT_WP: c_int = 0;
// Empty
pub const VIRTIO_BLK_ZS_EMPTY: c_int = 1;
// Implicitly Open
pub const VIRTIO_BLK_ZS_IOPEN: c_int = 2;
// Explicitly Open
pub const VIRTIO_BLK_ZS_EOPEN: c_int = 3;
// Closed
pub const VIRTIO_BLK_ZS_CLOSED: c_int = 4;
// Read-Only
pub const VIRTIO_BLK_ZS_RDONLY: c_int = 13;
// Full
pub const VIRTIO_BLK_ZS_FULL: c_int = 14;
// Offline
pub const VIRTIO_BLK_ZS_OFFLINE: c_int = 15;
// Unmap this range (only valid for write zeroes command)
pub const VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP: c_uint = 0x00000001;
// Discard/write zeroes range for each request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_blk_discard_write_zeroes {
// discard/write zeroes start sector
    pub sector: __le64,
// number of discard/write zeroes sectors
    pub num_sectors: __le32,
// flags for this range
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_inhdr {
    pub errors: __virtio32,
    pub data_len: __virtio32,
    pub sense_len: __virtio32,
    pub residual: __virtio32,
}

// And this is the final byte of the write scatter-gather list.
pub const VIRTIO_BLK_S_OK: c_int = 0;
pub const VIRTIO_BLK_S_IOERR: c_int = 1;
pub const VIRTIO_BLK_S_UNSUPP: c_int = 2;
// Error codes that are specific to zoned block devices
pub const VIRTIO_BLK_S_ZONE_INVALID_CMD: c_int = 3;
pub const VIRTIO_BLK_S_ZONE_UNALIGNED_WP: c_int = 4;
pub const VIRTIO_BLK_S_ZONE_OPEN_RESOURCE: c_int = 5;
pub const VIRTIO_BLK_S_ZONE_ACTIVE_RESOURCE: c_int = 6;
