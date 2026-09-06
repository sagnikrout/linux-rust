//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/media-devnode.h
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
// Media device node
//
// Copyright (C) 2010 Nokia Corporation
//
// Contacts: Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Sakari Ailus <sakari.ailus@iki.fi>
//
// --
//
// Common functions for media-related drivers to register and unregister media
// device nodes.
//

// debugfs top-level media directory
//
// Flag to mark the media_devnode struct as registered. Drivers must not touch
// this flag directly, it will be set and cleared by media_devnode_register and
// media_devnode_unregister.
//
pub const MEDIA_FLAG_REGISTERED: c_int = 0;
//
// struct media_file_operations - Media device file operations
//
// @owner: should be filled with %THIS_MODULE
// @read: pointer to the function that implements read() syscall
// @write: pointer to the function that implements write() syscall
// @poll: pointer to the function that implements poll() syscall
// @ioctl: pointer to the function that implements ioctl() syscall
// @compat_ioctl: pointer to the function that will handle 32 bits userspace
// calls to the ioctl() syscall on a Kernel compiled with 64 bits.
// @open: pointer to the function that implements open() syscall
// @release: pointer to the function that will release the resources allocated
// by the @open function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_file_operations {
    pub owner: *mut module,
    pub ): *mut *mut *mut *mut ssize_t (read) (struct file , char __user , size_t, loff_t,
    pub ): *const *const *const *const ssize_t (write) (struct file , char __user , size_t, loff_t,
    pub ): *mut *mut *mut __poll_t (poll) (struct file , struct poll_table_struct,
    pub long): *mut *mut *mut long (ioctl) (struct file , unsigned int, unsigned,
    pub long): *mut *mut *mut long (compat_ioctl) (struct file , unsigned int, unsigned,
    pub ): *mut *mut int (open) (struct file,
    pub ): *mut *mut int (release) (struct file,
}

//
// struct media_devnode - Media device node
// @media_dev:	pointer to struct &media_device
// @fops:	pointer to struct &media_file_operations with media device ops
// @dev:	pointer to struct &device containing the media controller device
// @cdev:	struct cdev pointer character device
// @parent:	parent device
// @minor:	device node minor number
// @flags:	flags, combination of the ``MEDIA_FLAG_*`` constants
// @release:	release callback called at the end of ``media_devnode_release()``
// routine at media-device.c.
//
// This structure represents a media-related device node.
//
// The @parent is a physical device. It must be set by core or device drivers
// before registering the node.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_devnode {
    pub media_dev: *mut media_device,
// device ops
    pub fops: *const media_file_operations,
// sysfs
    pub /: *mut *mut device dev; / media device,
    pub /: *mut *mut cdev cdev; / character device,
    pub /: *mut *mut *mut device parent; / device parent,
// device info
    pub minor: c_int,
    pub /: *mut *mut unsigned long flags; / Use bitops to access flags,
// callbacks
    pub devnode): *mut *mut void (release)(struct media_devnode,
}

// dev to media_devnode

//
// media_devnode_register - register a media device node
//
// @mdev: struct media_device we want to register a device node
// @devnode: media device node structure we want to register
// @owner: should be filled with %THIS_MODULE
//
// The registration code assigns minor numbers and registers the new device node
// with the kernel. An error is returned if no free minor number can be found,
// or if the registration of the device node fails.
//
// Zero is returned on success.
//
// Note that if the media_devnode_register call fails, the release() callback of
// the media_devnode structure is *not* called, so the caller is responsible for
// freeing any data.
//
// media_devnode_unregister_prepare - clear the media device node register bit
// @devnode: the device node to prepare for unregister
//
// This clears the passed device register bit. Future open calls will be met
// with errors. Should be called before media_devnode_unregister() to avoid
// races with unregister and device file open calls.
//
// This function can safely be called if the device node has never been
// registered or has already been unregistered.
//
extern "C" {
    pub fn media_devnode_unregister_prepare(devnode: *mut media_devnode);
}
//
// media_devnode_unregister - unregister a media device node
// @devnode: the device node to unregister
//
// This unregisters the passed device. Future open calls will be met with
// errors.
//
// Should be called after media_devnode_unregister_prepare()
//
extern "C" {
    pub fn media_devnode_unregister(devnode: *mut media_devnode);
}
//
// media_devnode_data - returns a pointer to the &media_devnode
//
// @filp: pointer to struct &file
//
// media_devnode_is_registered - returns true if &media_devnode is registered;
// false otherwise.
//
// @devnode: pointer to struct &media_devnode.
//
// Note: If mdev is NULL, it also returns false.
//
extern "C" {
    pub fn test_bit(_arg: MEDIA_FLAG_REGISTERED, _arg: &devnode->flags) -> return;
}
