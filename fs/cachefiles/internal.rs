//! Automatically rewritten from C Header to Rust Module
//! Source: fs/cachefiles/internal.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// General netfs cache on cache files internal defs
//
// Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

pub const CACHEFILES_DIO_BLOCK_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_content {
// These values are saved on disk
    CACHEFILES_CONTENT_NO_DATA	= 0, /* No content stored */
    CACHEFILES_CONTENT_SINGLE	= 1, /* Content is monolithic, all is present */
    CACHEFILES_CONTENT_ALL		= 2, /* Content is all present, no map */
    CACHEFILES_CONTENT_BACKFS_MAP	= 3, /* Content is piecemeal, mapped through backing fs */
    CACHEFILES_CONTENT_DIRTY	= 4, /* Content is dirty (only seen on disk) */
    nr__cachefiles_content
}

//
// Cached volume representation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachefiles_volume {
    pub cache: *mut cachefiles_cache,
    pub /: *mut *mut list_head cache_link; / Link in cache->volumes,
    pub /: *mut *mut *mut fscache_volume vcookie; / The netfs's representation,
    pub /: *mut *mut *mut dentry dentry; / The volume dentry,
    pub /: *mut *mut *mut dentry fanout[256]; / Fanout subdirs,
}

//
// Backing file state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachefiles_object {
    pub /: *mut *mut *mut fscache_cookie cookie; / Netfs data storage object cookie,
    pub /: *mut *mut *mut cachefiles_volume volume; / Cache volume that holds this object,
    pub /: *mut *mut *mut list_head cache_link; / Link in cache->_list,
    pub /: *mut *mut *mut file file; / The file representing this object,
    pub /: *mut *mut *mut char d_name; / Backing file name,
    pub debug_id: c_int,
    pub lock: spinlock_t,
    pub ref: refcount_t,
    pub /: *mut *mut cachefiles_content content_info:8; / Info about content presence,
    pub flags: c_ulong,

}

//
// Cache files cache definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachefiles_cache {
    pub /: *mut *mut *mut fscache_cache cache; / Cache cookie,
    pub /: *mut *mut *mut vfsmount mnt; / mountpoint holding the cache,
    pub /: *mut *mut *mut dentry store; / Directory into which live objects go,
    pub /: *mut *mut *mut dentry graveyard; / directory into which dead objects go,
    pub /: *mut *mut *mut file cachefilesd; / manager daemon handle,
    pub /: *mut *mut list_head volumes; / List of volume objects,
    pub /: *mut *mut list_head object_list; / List of active objects,
    pub /: *mut *mut spinlock_t object_list_lock; / Lock for volumes and object_list,
    pub /: *const *const *const cred cache_cred; / security override for accessing cache,
    pub /: *mut *mut mutex daemon_mutex; / command serialisation mutex,
    pub /: *mut *mut wait_queue_head_t daemon_pollwq; / poll waitqueue for daemon,
    pub /: *mut *mut atomic_t gravecounter; / graveyard uniquifier,
    pub /: *mut *mut atomic_t f_released; / number of objects released lately,
    pub /: *mut *mut atomic_long_t b_released; / number of blocks released lately,
    pub /: *mut *mut atomic_long_t b_writing; / Number of blocks being written,
    pub /: *mut *mut unsigned frun_percent; / when to stop culling (% files),
    pub /: *mut *mut unsigned fcull_percent; / when to start culling (% files),
    pub /: *mut *mut unsigned fstop_percent; / when to stop allocating (% files),
    pub /: *mut *mut unsigned brun_percent; / when to stop culling (% blocks),
    pub /: *mut *mut unsigned bcull_percent; / when to start culling (% blocks),
    pub /: *mut *mut unsigned bstop_percent; / when to stop allocating (% blocks),
    pub /: *mut *mut unsigned bsize; / cache's block size,
    pub /: *mut *mut unsigned bshift; / ilog2(bsize),
    pub /: *mut *mut uint64_t frun; / when to stop culling,
    pub /: *mut *mut uint64_t fcull; / when to start culling,
    pub /: *mut *mut uint64_t fstop; / when to stop allocating,
    pub /: *mut *mut sector_t brun; / when to stop culling,
    pub /: *mut *mut sector_t bcull; / when to start culling,
    pub /: *mut *mut sector_t bstop; / when to stop allocating,
    pub flags: c_ulong,

    pub /: *mut *mut *mut char rootdirname; / name of cache root directory,
    pub /: *mut *mut *mut char tag; / cache binding tag,
    pub /: *mut *mut u32 secid; / LSM security id,
    pub /: *mut *mut bool have_secid; / whether "secid" was set,
}

//
// note change of state for daemon
//
// cache.c
//
extern "C" {
    pub fn cachefiles_add_cache(cache: *mut cachefiles_cache) -> c_int;
}
extern "C" {
    pub fn cachefiles_withdraw_cache(cache: *mut cachefiles_cache);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cachefiles_has_space_for {
    cachefiles_has_space_check,
    cachefiles_has_space_for_write,
    cachefiles_has_space_for_create,
}

//
// daemon.c
//
// error_inject.c
//

extern "C" {
    pub fn cachefiles_register_error_injection() -> c_int;
}
extern "C" {
    pub fn cachefiles_unregister_error_injection();
}

pub const cachefiles_error_injection_state: c_int = 0;

//
// interface.c
//
// io.c
//
// key.c
//
extern "C" {
    pub fn cachefiles_cook_key(object: *mut cachefiles_object) -> bool;
}
//
// main.c
//
// namei.c
//
extern "C" {
    pub fn cachefiles_look_up_object(object: *mut cachefiles_object) -> bool;
}
extern "C" {
    pub fn cachefiles_put_directory(dir: *mut dentry);
}
//
// security.c
//
extern "C" {
    pub fn cachefiles_get_security_ID(cache: *mut cachefiles_cache) -> c_int;
}
// _saved_cred = override_creds(cache->cache_cred);
//
// volume.c
//
extern "C" {
    pub fn cachefiles_acquire_volume(volume: *mut fscache_volume);
}
extern "C" {
    pub fn cachefiles_free_volume(volume: *mut fscache_volume);
}
extern "C" {
    pub fn cachefiles_withdraw_volume(volume: *mut cachefiles_volume);
}
//
// xattr.c
//
extern "C" {
    pub fn cachefiles_set_object_xattr(object: *mut cachefiles_object) -> c_int;
}
extern "C" {
    pub fn cachefiles_prepare_to_write(cookie: *mut fscache_cookie);
}
extern "C" {
    pub fn cachefiles_set_volume_xattr(volume: *mut cachefiles_volume) -> bool;
}
extern "C" {
    pub fn cachefiles_check_volume_xattr(volume: *mut cachefiles_volume) -> c_int;
}
//
// Error handling
//

//
// Debug tracing
//
pub const CACHEFILES_DEBUG_KENTER: c_int = 1;
pub const CACHEFILES_DEBUG_KLEAVE: c_int = 2;
pub const CACHEFILES_DEBUG_KDEBUG: c_int = 4;

