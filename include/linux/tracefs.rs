//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tracefs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// tracefs.h - a pseudo file system for activating tracing
//
// Based on debugfs by: 2004 Greg Kroah-Hartman <greg@kroah.com>
//
// Copyright (C) 2014 Red Hat Inc, author: Steven Rostedt <srostedt@redhat.com>
//
// tracefs is the file system that is used by the tracing infrastructure.
//

//
// eventfs_callback - A callback function to create dynamic files in eventfs
// @name: The name of the file that is to be created
// @mode: return the file mode for the file (RW access, etc)
// @data: data to pass to the created file ops
// @fops: the file operations of the created file
//
// The eventfs files are dynamically created. The struct eventfs_entry array
// is passed to eventfs_create_dir() or eventfs_create_events_dir() that will
// be used to create the files within those directories. When a lookup
// or access to a file within the directory is made, the struct eventfs_entry
// array is used to find a callback() with the matching name that is being
// referenced (for lookups, the entire array is iterated and each callback
// will be called).
//
// The callback will be called with @name for the name of the file to create.
// The callback can return less than 1 to indicate  that no file should be
// created.
//
// If a file is to be created, then @mode should be populated with the file
// mode (permissions) for which the file is created for. This would be
// used to set the created inode i_mode field.
//
// The @data should be set to the data passed to the other file operations
// (read, write, etc). Note, @data will also point to the data passed in
// to eventfs_create_dir() or eventfs_create_events_dir(), but the callback
// can replace the data if it chooses to. Otherwise, the original data
// will be used for the file operation functions.
//
// The @fops should be set to the file operations that will be used to create
// the inode.
//
// NB. This callback is called while holding internal locks of the eventfs
// system. The callback must not call any code that might also call into
// the tracefs or eventfs system or it will risk creating a deadlock.
//
extern "C" {
    pub fn void(name: *const *const eventfs_release)(char, data: *mut c_void) -> typedef;
}
//
// struct eventfs_entry - dynamically created eventfs file call back handler
// @name:	Then name of the dynamic file in an eventfs directory
// @callback:	The callback to get the fops of the file when it is created
//
// See evenfs_callback() typedef for how to set up @callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eventfs_entry {
    pub name: *const c_char,
    pub callback: eventfs_callback,
    pub release: eventfs_release,
}

extern "C" {
    pub fn eventfs_remove_events_dir(ei: *mut eventfs_inode);
}
extern "C" {
    pub fn eventfs_remove_dir(ei: *mut eventfs_inode);
}
extern "C" {
    pub fn tracefs_remove(dentry: *mut dentry);
}
extern "C" {
    pub fn tracefs_initialized() -> bool;
}

