//! Automatically rewritten from C Header to Rust Module
//! Source: fs/nfs/blocklayout/blocklayout.h
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
// linux/fs/nfs/blocklayout/blocklayout.h
//
// Module for the NFSv4.1 pNFS block layout driver.
//
// Copyright (c) 2006 The Regents of the University of Michigan.
// All rights reserved.
//
// Andy Adamson <andros@citi.umich.edu>
// Fred Isaman <iisaman@umich.edu>
//
// permission is granted to use, copy, create derivative works and
// redistribute this software and such derivative works for any purpose,
// so long as the name of the university of michigan is not used in
// any advertising or publicity pertaining to the use or distribution
// of this software without specific, written prior authorization.  if
// the above copyright notice or any other identification of the
// university of michigan is included in any copy of any portion of
// this software, then the disclaimer below must also be included.
//
// this software is provided as is, without representation from the
// university of michigan as to its fitness for any purpose, and without
// warranty by the university of michigan of any kind, either express
// or implied, including without limitation the implied warranties of
// merchantability and fitness for a particular purpose.  the regents
// of the university of michigan shall not be liable for any damages,
// including special, indirect, incidental, or consequential damages,
// with respect to any claim arising out or in connection with the use
// of the software, even if it has been or is hereafter advised of the
// possibility of such damages.
//

pub const PNFS_BLOCK_MAX_UUIDS: c_int = 4;
pub const PNFS_BLOCK_MAX_DEVICES: c_int = 64;
//
// Random upper cap for the uuid length to avoid unbounded allocation.
// Not actually limited by the protocol.
//
pub const PNFS_BLOCK_UUID_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_volume {
    pub type: pnfs_block_volume_type,
    pub len: c_int,
    pub nr_sigs: c_int,
    pub offset: u64,
    pub sig_len: u32,
    pub sig: [u8; PNFS_BLOCK_UUID_LEN],
    pub sigs: [}; PNFS_BLOCK_MAX_UUIDS],
    pub simple: },
    pub start: u64,
    pub len: u64,
    pub volume: u32,
    pub slice: },
    pub volumes_count: u32,
    pub volumes: [u32; PNFS_BLOCK_MAX_DEVICES],
    pub concat: },
    pub chunk_size: u64,
    pub volumes_count: u32,
    pub volumes: [u32; PNFS_BLOCK_MAX_DEVICES],
    pub stripe: },
    pub code_set: scsi_code_set,
    pub designator_type: scsi_designator_type,
    pub designator_len: c_int,
    pub designator: [u8; 256],
    pub pr_key: u64,
    pub scsi: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_dev_map {
    pub start: u64,
    pub len: u64,
    pub disk_offset: u64,
    pub bdev: *mut block_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_dev {
    pub node: nfs4_deviceid_node,
    pub start: u64,
    pub len: u64,
    pub type: pnfs_block_volume_type,
    pub nr_children: u32,
    pub children: *mut pnfs_block_dev,
    pub chunk_size: u64,
    pub bdev_file: *mut file,
    pub disk_offset: u64,
    pub flags: c_ulong,
    pub pr_key: u64,
    pub map): *mut pnfs_block_dev_map,
}

// pnfs_block_dev flag bits
// sector_t fields are all in 512-byte sectors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_extent {
    pub be_node: rb_node,
    pub be_list: list_head,
}

pub const EXTENT_WRITTEN: c_int = 1;
pub const EXTENT_COMMITTING: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnfs_block_layout {
    pub bl_layout: pnfs_layout_hdr,
    pub bl_ext_rw: rb_root,
    pub bl_ext_ro: rb_root,
    pub /: *mut *mut spinlock_t bl_ext_lock; / Protects list manipulation,
    pub bl_scsi_layout: bool,
    pub bl_lwb: u64,
}

extern "C" {
    pub fn container_of(_arg: lo, pnfs_block_layout: struct, _arg: bl_layout) -> return;
}
extern "C" {
    pub fn BLK_LO2EXT(_arg: lseg->pls_layout) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_pipe_msg {
    pub msg: rpc_pipe_msg,
    pub bl_wq: *mut wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bl_msg_hdr {
    pub type: u8,
    pub /: *mut *mut u16 totallen; / length of entire message, including hdr itself,
}

pub const BL_DEVICE_UMOUNT: c_uint = 0x0 /* Umount--delete devices */;
pub const BL_DEVICE_MOUNT: c_uint = 0x1 /* Mount--create devices*/;
pub const BL_DEVICE_REQUEST_INIT: c_uint = 0x0 /* Start request */;
pub const BL_DEVICE_REQUEST_PROC: c_uint = 0x1 /* User level process succeeds */;
pub const BL_DEVICE_REQUEST_ERR: c_uint = 0x2 /* User level process fails */;
// dev.c
extern "C" {
    pub fn bl_register_dev(d: *mut pnfs_block_dev) -> bool;
}
extern "C" {
    pub fn bl_free_deviceid_node(d: *mut nfs4_deviceid_node);
}
// extent_tree.c
extern "C" {
    pub fn ext_tree_prepare_commit(arg: *mut nfs4_layoutcommit_args) -> c_int;
}
extern "C" {
    pub fn ext_tree_mark_committed(arg: *mut nfs4_layoutcommit_args, status: c_int);
}
// rpc_pipefs.c
extern "C" {
    pub fn bl_init_pipefs() -> int __init;
}
extern "C" {
    pub fn bl_cleanup_pipefs();
}
