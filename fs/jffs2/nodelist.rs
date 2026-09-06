//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jffs2/nodelist.h
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
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

// Macro flag: #define JFFS2_NATIVE_ENDIAN
// Note we handle mode bits conversion from JFFS2 (i.e. Linux) to/from

// The minimal node header size

//

// Use blocks of about 256 bytes

// Link to another block of refs
// End of chain
// NB. This can be a jffs2_xattr_datum or jffs2_xattr_ref and
// flash_offset & 3 always has to be zero, because nodes are

// Dirent nodes should be REF_PRISTINE only if they are not a deletion

// NB: REF_PRISTINE for an inode-less node (ref->next_in_ino == NULL) indicates
// For each inode in the filesystem, we need to keep a record of
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jffs2_inode_cache {
// First part of structure is shared with other objects which
    pub hold: *mut *mut *mut jffs2_full_dirent scan_dents; / Used during scan to,
    pub nodes: *mut jffs2_raw_node_ref,
    pub /: *mut *mut uint8_t class; / It's used for identification,
// end of shared structure
    pub flags: u8,
    pub state: u16,
    pub ino: u32,
    pub next: *mut jffs2_inode_cache,

    pub xref: *mut jffs2_xattr_ref,

    pub inode: *mut *mut uint32_t pino_nlink; / Directories store parent,
    pub nlink.: here; other inodes store,
}

// Inode states for 'state' above. We need the 'GC' state to prevent

pub const INO_FLAGS_XATTR_CHECKED: c_uint = 0x01	/* has no duplicate xattr_ref */;
pub const INO_FLAGS_IS_DIR: c_uint = 0x02	/* is a directory */;
pub const RAWNODE_CLASS_INODE_CACHE: c_int = 0;
pub const RAWNODE_CLASS_XATTR_DATUM: c_int = 1;
pub const RAWNODE_CLASS_XATTR_REF: c_int = 2;
pub const INOCACHE_HASHSIZE_MIN: c_int = 128;
pub const INOCACHE_HASHSIZE_MAX: c_int = 1024;

//
// Temporary data structure used during readinode.
//

// How much dirty space before it goes on the very_dirty_list

// check if dirty space is more than 255 Byte

extern "C" {
    pub fn sizeof(_arg: jdev->old_id) -> return;
}
extern "C" {
    pub fn sizeof(_arg: jdev->new_id) -> return;
}
extern "C" {
    pub fn rb_entry(_arg: node, jffs2_node_frag: struct, _arg: rb) -> return;
}
extern "C" {
    pub fn rb_entry(_arg: node, jffs2_node_frag: struct, _arg: rb) -> return;
}

// nodelist.c
extern "C" {
    pub fn jffs2_add_fd_to_list(c: *mut jffs2_sb_info, new: *mut jffs2_full_dirent, list: *mut jffs2_full_dirent);
}
extern "C" {
    pub fn jffs2_set_inocache_state(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache, state: c_int);
}
extern "C" {
    pub fn jffs2_add_ino_cache(c: *mut jffs2_sb_info, new: *mut jffs2_inode_cache);
}
extern "C" {
    pub fn jffs2_del_ino_cache(c: *mut jffs2_sb_info, old: *mut jffs2_inode_cache);
}
extern "C" {
    pub fn jffs2_free_ino_caches(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_free_raw_node_refs(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_kill_fragtree(root: *mut rb_root, c_delete: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_add_full_dnode_to_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, fn: *mut jffs2_full_dnode) -> c_int;
}
extern "C" {
    pub fn jffs2_truncate_fragtree(c: *mut jffs2_sb_info, list: *mut rb_root, size: u32) -> u32;
}
// nodemgmt.c
extern "C" {
    pub fn jffs2_thread_should_wake(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_complete_reservation(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, raw: *mut jffs2_raw_node_ref);
}
// write.c
extern "C" {
    pub fn jffs2_do_new_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info, mode: u32, ri: *mut jffs2_raw_inode) -> c_int;
}
// readinode.c
extern "C" {
    pub fn jffs2_do_crccheck_inode(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache) -> c_int;
}
extern "C" {
    pub fn jffs2_do_clear_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info);
}
// malloc.c
extern "C" {
    pub fn jffs2_create_slab_caches() -> c_int;
}
extern "C" {
    pub fn jffs2_destroy_slab_caches();
}
extern "C" {
    pub fn jffs2_free_full_dirent(: *mut jffs2_full_dirent);
}
extern "C" {
    pub fn jffs2_free_full_dnode(: *mut jffs2_full_dnode);
}
extern "C" {
    pub fn jffs2_free_raw_dirent(: *mut jffs2_raw_dirent);
}
extern "C" {
    pub fn jffs2_free_raw_inode(: *mut jffs2_raw_inode);
}
extern "C" {
    pub fn jffs2_free_tmp_dnode_info(: *mut jffs2_tmp_dnode_info);
}
extern "C" {
    pub fn jffs2_free_refblock(: *mut jffs2_raw_node_ref);
}
extern "C" {
    pub fn jffs2_free_node_frag(: *mut jffs2_node_frag);
}
extern "C" {
    pub fn jffs2_free_inode_cache(: *mut jffs2_inode_cache);
}

extern "C" {
    pub fn jffs2_free_xattr_datum(: *mut jffs2_xattr_datum);
}
extern "C" {
    pub fn jffs2_free_xattr_ref(: *mut jffs2_xattr_ref);
}

// gc.c
extern "C" {
    pub fn jffs2_garbage_collect_pass(c: *mut jffs2_sb_info) -> c_int;
}
// read.c
// scan.c
extern "C" {
    pub fn jffs2_scan_medium(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_rotate_lists(c: *mut jffs2_sb_info);
}
extern "C" {
    pub fn jffs2_scan_classify_jeb(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> c_int;
}
extern "C" {
    pub fn jffs2_scan_dirty_space(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, size: u32) -> c_int;
}
// build.c
extern "C" {
    pub fn jffs2_do_mount_fs(c: *mut jffs2_sb_info) -> c_int;
}
// erase.c
extern "C" {
    pub fn jffs2_erase_pending_blocks(c: *mut jffs2_sb_info, count: c_int) -> c_int;
}
extern "C" {
    pub fn jffs2_free_jeb_node_refs(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock);
}

// wbuf.c
extern "C" {
    pub fn jffs2_flush_wbuf_gc(c: *mut jffs2_sb_info, ino: u32) -> c_int;
}
extern "C" {
    pub fn jffs2_flush_wbuf_pad(c: *mut jffs2_sb_info) -> c_int;
}
extern "C" {
    pub fn jffs2_check_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> c_int;
}
extern "C" {
    pub fn jffs2_write_nand_cleanmarker(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock) -> c_int;
}

