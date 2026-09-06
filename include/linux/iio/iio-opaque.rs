//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/iio-opaque.h
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
//
// struct iio_dev_opaque - industrial I/O device opaque information
// @indio_dev:			public industrial I/O device information
// @id:			used to identify device internally
// @currentmode:		operating mode currently in use, may be eventually
// checked by device drivers but should be considered
// read-only as this is a core internal bit
// @driver_module:		used to make it harder to undercut users
// @mlock:			lock used to prevent simultaneous device state changes
// @mlock_key:			lockdep class for iio_dev lock
// @info_exist_lock:		lock to prevent use during removal
// @info_exist_key:		lockdep class for info_exist lock
// @trig_readonly:		mark the current trigger immutable
// @event_interface:		event chrdevs associated with interrupt lines
// @attached_buffers:		array of buffers statically attached by the driver
// @attached_buffers_cnt:	number of buffers in the array of statically attached buffers
// @buffer_ioctl_handler:	ioctl() handler for this IIO device's buffer interface
// @buffer_list:		list of all buffers currently attached
// @channel_attr_list:		keep track of automatically created channel
// attributes
// @chan_attr_group:		group for all attrs in base directory
// @ioctl_handlers:		ioctl handlers registered with the core handler
// @groups:			attribute groups
// @groupcounter:		index of next attribute group
// @legacy_scan_el_group:	attribute group for legacy scan elements attribute group
// @legacy_buffer_group:	attribute group for legacy buffer attributes group
// @bounce_buffer:		for devices that call iio_push_to_buffers_with_ts_unaligned()
// @bounce_buffer_size:		size of currently allocate bounce buffer
// @scan_index_timestamp:	cache of the index to the timestamp
// @clock_id:			timestamping clock posix identifier
// @chrdev:			associated character device
// @flags:			file ops related flags including busy flag.
// @debugfs_dentry:		device specific debugfs dentry
// @cached_reg_addr:		cached register address for debugfs reads
// @read_buf:			read buffer to be used for the initial reg read
// @read_buf_len:		data length in @read_buf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dev_opaque {
    pub indio_dev: iio_dev,
    pub currentmode: c_int,
    pub id: c_int,
    pub driver_module: *mut module,
    pub mlock: mutex,
    pub mlock_key: lock_class_key,
    pub info_exist_lock: mutex,
    pub info_exist_key: lock_class_key,
    pub trig_readonly: bool,
    pub event_interface: *mut iio_event_interface,
    pub attached_buffers: *mut iio_buffer,
    pub attached_buffers_cnt: c_uint,
    pub buffer_ioctl_handler: *mut iio_ioctl_handler,
    pub buffer_list: list_head,
    pub channel_attr_list: list_head,
    pub chan_attr_group: attribute_group,
    pub ioctl_handlers: list_head,
    pub groups: *const attribute_group,
    pub groupcounter: c_int,
    pub legacy_scan_el_group: attribute_group,
    pub legacy_buffer_group: attribute_group,
    pub bounce_buffer: *mut c_void,
    pub bounce_buffer_size: usize,
    pub scan_index_timestamp: c_uint,
    pub clock_id: clockid_t,
    pub chrdev: cdev,
    pub flags: c_ulong,

    pub debugfs_dentry: *mut dentry,
    pub cached_reg_addr: c_uint,
    pub read_buf: [c_char; 20],
    pub read_buf_len: c_uint,

}

