//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvbdev.h
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
// dvbdev.h
//
// Copyright (C) 2000 Ralph Metzler & Marcus Metzler
// for convergence integrated media GmbH
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Lesser Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//

pub const DVB_MAJOR: c_int = 212;

pub const DVB_MAX_ADAPTERS: c_int = 16;

// List of DVB device types
//
// enum dvb_device_type - type of the Digital TV device
//
// @DVB_DEVICE_SEC:		Digital TV standalone Common Interface (CI)
// @DVB_DEVICE_FRONTEND:	Digital TV frontend.
// @DVB_DEVICE_DEMUX:		Digital TV demux.
// @DVB_DEVICE_DVR:		Digital TV digital video record (DVR).
// @DVB_DEVICE_CA:		Digital TV Conditional Access (CA).
// @DVB_DEVICE_NET:		Digital TV network.
//
// @DVB_DEVICE_VIDEO:		Digital TV video decoder.
// Deprecated. Used only on av7110-av.
// @DVB_DEVICE_AUDIO:		Digital TV audio decoder.
// Deprecated. Used only on av7110-av.
// @DVB_DEVICE_OSD:		Digital TV On Screen Display (OSD).
// Deprecated. Used only on av7110.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvb_device_type {
    DVB_DEVICE_SEC,
    DVB_DEVICE_FRONTEND,
    DVB_DEVICE_DEMUX,
    DVB_DEVICE_DVR,
    DVB_DEVICE_CA,
    DVB_DEVICE_NET,

    DVB_DEVICE_VIDEO,
    DVB_DEVICE_AUDIO,
    DVB_DEVICE_OSD,
}

//
// struct dvb_adapter - represents a Digital TV adapter using Linux DVB API
//
// @num:		Number of the adapter
// @list_head:		List with the DVB adapters
// @device_list:	List with the DVB devices
// @name:		Name of the adapter
// @proposed_mac:	proposed MAC address for the adapter
// @priv:		private data
// @device:		pointer to struct device
// @module:		pointer to struct module
// @mfe_shared:		indicates mutually exclusive frontends.
// 1 = legacy exclusion behavior: blocking any open() call
// 2 = enhanced exclusion behavior, emulating the standard
// behavior of busy frontends: allowing read-only sharing
// and otherwise returning immediately with -EBUSY when any
// of the frontends is already opened with write access.
// @mfe_dvbdev:		Frontend device in use, in the case of MFE
// @mfe_lock:		Lock to prevent using the other frontends when MFE is
// used.
// @mdev_lock:          Protect access to the mdev pointer.
// @mdev:		pointer to struct media_device, used when the media
// controller is used.
// @conn:		RF connector. Used only if the device has no separate
// tuner.
// @conn_pads:		pointer to struct media_pad associated with @conn;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_adapter {
    pub num: c_int,
    pub list_head: list_head,
    pub device_list: list_head,
    pub name: *const c_char,
    pub [6]: u8 proposed_mac,
    pub priv: *mut *mut c_void,
    pub device: *mut device,
    pub module: *mut module,
    pub /: *mut *mut int mfe_shared; / indicates mutually exclusive frontends,
    pub /: *mut *mut *mut dvb_device mfe_dvbdev; / frontend device in use,
    pub /: *mut *mut mutex mfe_lock; / access lock for thread creation,

    pub mdev_lock: mutex,
    pub mdev: *mut media_device,
    pub conn: *mut media_entity,
    pub conn_pads: *mut media_pad,

}

//
// struct dvb_device - represents a DVB device node
//
// @list_head:	List head with all DVB devices
// @ref:	reference count for this device
// @fops:	pointer to struct file_operations
// @adapter:	pointer to the adapter that holds this device node
// @type:	type of the device, as defined by &enum dvb_device_type.
// @minor:	devnode minor number. Major number is always DVB_MAJOR.
// @id:		device ID number, inside the adapter
// @readers:	Initialized by the caller. Each call to open() in Read Only mode
// decreases this counter by one.
// @writers:	Initialized by the caller. Each call to open() in Read/Write
// mode decreases this counter by one.
// @users:	Initialized by the caller. Each call to open() in any mode
// decreases this counter by one.
// @wait_queue:	wait queue, used to wait for certain events inside one of
// the DVB API callers
// @kernel_ioctl: callback function used to handle ioctl calls from userspace.
// @name:	Name to be used for the device at the Media Controller
// @entity:	pointer to struct media_entity associated with the device node
// @pads:	pointer to struct media_pad associated with @entity;
// @priv:	private data
// @intf_devnode: Pointer to media_intf_devnode. Used by the dvbdev core to
// store the MC device node interface
// @tsout_num_entities: Number of Transport Stream output entities
// @tsout_entity: array with MC entities associated to each TS output node
// @tsout_pads: array with the source pads for each @tsout_entity
//
// This structure is used by the DVB core (frontend, CA, net, demux) in
// order to create the device nodes. Usually, driver should not initialize
// this struct diretly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_device {
    pub list_head: list_head,
    pub ref: kref,
    pub fops: *const file_operations,
    pub adapter: *mut dvb_adapter,
    pub type: dvb_device_type,
    pub minor: c_int,
    pub id: u32,
// in theory, 'users' can vanish now,
    pub readers: c_int,
    pub writers: c_int,
    pub users: c_int,
    pub wait_queue: wait_queue_head_t,
// don't really need those !? -- FIXME: use video_usercopy
    pub arg): *mut *mut *mut int (kernel_ioctl)(struct file file, unsigned int cmd, void,
// Needed for media controller register/unregister

    pub name: *const c_char,
// Allocated and filled inside dvbdev.c
    pub intf_devnode: *mut media_intf_devnode,
    pub tsout_num_entities: unsigned,
    pub tsout_entity: *mut *mut media_entity entity,,
    pub tsout_pads: *mut *mut media_pad pads,,

    pub priv: *mut c_void,
}

//
// struct dvbdevfops_node - fops nodes registered in dvbdevfops_list
//
// @fops:		Dynamically allocated fops for ->owner registration
// @type:		type of dvb_device
// @template:		dvb_device used for registration
// @list_head:		list_head for dvbdevfops_list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvbdevfops_node {
    pub fops: *mut file_operations,
    pub type: dvb_device_type,
    pub template: *const dvb_device,
    pub list_head: list_head,
}

//
// dvb_device_get - Increase dvb_device reference
//
// @dvbdev:	pointer to struct dvb_device
//
// dvb_device_put - Decrease dvb_device reference
//
// @dvbdev:	pointer to struct dvb_device
//
extern "C" {
    pub fn dvb_device_put(dvbdev: *mut dvb_device);
}
//
// dvb_register_adapter - Registers a new DVB adapter
//
// @adap:	pointer to struct dvb_adapter
// @name:	Adapter's name
// @module:	initialized with THIS_MODULE at the caller
// @device:	pointer to struct device that corresponds to the device driver
// @adapter_nums: Array with a list of the numbers for @dvb_register_adapter;
// to select among them. Typically, initialized with:
// DVB_DEFINE_MOD_OPT_ADAPTER_NR(adapter_nums)
//
// dvb_unregister_adapter - Unregisters a DVB adapter
//
// @adap:	pointer to struct dvb_adapter
//
extern "C" {
    pub fn dvb_unregister_adapter(adap: *mut dvb_adapter) -> c_int;
}
//
// dvb_register_device - Registers a new DVB device
//
// @adap:	pointer to struct dvb_adapter
// @pdvbdev:	pointer to the place where the new struct dvb_device will be
// stored
// @template:	Template used to create &pdvbdev;
// @priv:	private data
// @type:	type of the device, as defined by &enum dvb_device_type.
// @demux_sink_pads: Number of demux outputs, to be used to create the TS
// outputs via the Media Controller.
//
// dvb_remove_device - Remove a registered DVB device
//
// @dvbdev:	pointer to struct dvb_device
//
// This does not free memory. dvb_free_device() will do that when
// reference counter is empty
//
extern "C" {
    pub fn dvb_remove_device(dvbdev: *mut dvb_device);
}
//
// dvb_unregister_device - Unregisters a DVB device
//
// @dvbdev:	pointer to struct dvb_device
//
extern "C" {
    pub fn dvb_unregister_device(dvbdev: *mut dvb_device);
}

//
// dvb_create_media_graph - Creates media graph for the Digital TV part of the
// device.
//
// @adap:			pointer to &struct dvb_adapter
// @create_rf_connector:	if true, it creates the RF connector too
//
// This function checks all DVB-related functions at the media controller
// entities and creates the needed links for the media graph. It is
// capable of working with multiple tuners or multiple frontends, but it
// won't create links if the device has multiple tuners and multiple frontends
// or if the device has multiple muxes. In such case, the caller driver should
// manually create the remaining links.
//
// dvb_register_media_controller - registers a media controller at DVB adapter
//
// @adap:			pointer to &struct dvb_adapter
// @mdev:			pointer to &struct media_device
//
// dvb_get_media_controller - gets the associated media controller
//
// @adap:			pointer to &struct dvb_adapter
//

//
// dvb_generic_open - Digital TV open function, used by DVB devices
//
// @inode: pointer to &struct inode.
// @file: pointer to &struct file.
//
// Checks if a DVB devnode is still valid, and if the permissions are
// OK and increment negative use count.
//
extern "C" {
    pub fn dvb_generic_open(inode: *mut inode, file: *mut file) -> c_int;
}
//
// dvb_generic_release - Digital TV close function, used by DVB devices
//
// @inode: pointer to &struct inode.
// @file: pointer to &struct file.
//
// Checks if a DVB devnode is still valid, and if the permissions are
// OK and decrement negative use count.
//
extern "C" {
    pub fn dvb_generic_release(inode: *mut inode, file: *mut file) -> c_int;
}
//
// dvb_generic_ioctl - Digital TV close function, used by DVB devices
//
// @file: pointer to &struct file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
//
// Checks if a DVB devnode and struct dvbdev.kernel_ioctl is still valid.
// If so, calls dvb_usercopy().
//
// dvb_usercopy - copies data from/to userspace memory when an ioctl is
// issued.
//
// @file: Pointer to struct &file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
// @func: function that will actually handle the ioctl
//
// Ancillary function that uses ioctl direction and size to copy from
// userspace. Then, it calls @func, and, if needed, data is copied back
// to userspace.
//

//
// dvb_module_probe - helper routine to probe an I2C module
//
// @module_name:
// Name of the I2C module to be probed
// @name:
// Optional name for the I2C module. Used for debug purposes.
// If %NULL, defaults to @module_name.
// @adap:
// pointer to &struct i2c_adapter that describes the I2C adapter where
// the module will be bound.
// @addr:
// I2C address of the adapter, in 7-bit notation.
// @platform_data:
// Platform data to be passed to the I2C module probed.
//
// This function binds an I2C device into the DVB core. Should be used by
// all drivers that use I2C bus to control the hardware. A module bound
// with dvb_module_probe() should use dvb_module_release() to unbind.
//
// Return:
// On success, return an &struct i2c_client, pointing to the bound
// I2C device. %NULL otherwise.
//
// .. note::
//
// In the past, DVB modules (mainly, frontends) were bound via dvb_attach()
// macro, with does an ugly hack, using I2C low level functions. Such
// usage is deprecated and will be removed soon. Instead, use this routine.
//
// dvb_module_release - releases an I2C device allocated with
// dvb_module_probe().
//
// @client: pointer to &struct i2c_client with the I2C client to be released.
// can be %NULL.
//
// This function should be used to free all resources reserved by
// dvb_module_probe() and unbinding the I2C hardware.
//
extern "C" {
    pub fn dvb_module_release(client: *mut i2c_client);
}

// Legacy generic DVB attach function.

//
// dvb_attach - attaches a DVB frontend into the DVB core.
//
// @FUNCTION:	function on a frontend module to be called.
// @ARGS:	@FUNCTION arguments.
//
// This ancillary function loads a frontend module in runtime and runs
// the @FUNCTION function there, with @ARGS.
// As it increments symbol usage cont, at unregister, dvb_detach()
// should be called.
//
// .. note::
//
// In the past, DVB modules (mainly, frontends) were bound via dvb_attach()
// macro, with does an ugly hack, using I2C low level functions. Such
// usage is deprecated and will be removed soon. Instead, you should use
// dvb_module_probe().
//

//
// dvb_detach - detaches a DVB frontend loaded via dvb_attach()
//
// @FUNC:	attach function
//
// Decrements usage count for a function previously called via dvb_attach().
//

