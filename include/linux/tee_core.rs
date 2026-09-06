//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tee_core.h
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
// Copyright (c) 2024 Linaro Limited
//

//
// The file describes the API provided by the generic TEE driver to the
// specific TEE driver.
//

// in secure world

// dma_alloc_pages()
pub const TEE_DEVICE_FLAG_REGISTERED: c_uint = 0x1;
pub const TEE_MAX_DEV_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tee_dma_heap_id {
    TEE_DMA_HEAP_SECURE_VIDEO_PLAY = 1,
    TEE_DMA_HEAP_TRUSTED_UI,
    TEE_DMA_HEAP_SECURE_VIDEO_RECORD,
}

//
// struct tee_device - TEE Device representation
// @name:	name of device
// @desc:	description of device
// @id:		unique id of device
// @flags:	represented by TEE_DEVICE_FLAG_REGISTERED above
// @dev:	embedded basic device structure
// @cdev:	embedded cdev
// @num_users:	number of active users of this device
// @c_no_users:	completion used when unregistering the device
// @mutex:	mutex protecting @num_users and @idr
// @idr:	register of user space shared memory objects allocated or
// registered on this device
// @pool:	shared memory pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_device {
    pub name: [c_char; TEE_MAX_DEV_NAME_LEN],
    pub desc: *const tee_desc,
    pub id: c_int,
    pub flags: c_uint,
    pub dev: device,
    pub cdev: cdev,
    pub num_users: usize,
    pub c_no_users: completion,
    pub /: *mut *mut mutex mutex; / protects num_users and idr,
    pub idr: idr,
    pub pool: *mut tee_shm_pool,
}

//
// struct tee_driver_ops - driver operations vtable
// @get_version:	returns version of driver
// @get_tee_revision:	returns revision string (diagnostic only);
// do not infer feature support from this, use
// TEE_IOC_VERSION instead
// @open:		called for a context when the device file is opened
// @close_context:	called when the device file is closed
// @release:		called to release the context
// @open_session:	open a new session
// @close_session:	close a session
// @system_session:	declare session as a system session
// @invoke_func:	invoke a trusted function
// @object_invoke_func:	invoke a TEE object
// @cancel_req:		request cancel of an ongoing invoke or open
// @supp_recv:		called for supplicant to get a command
// @supp_send:		called for supplicant to send a response
// @shm_register:	register shared memory buffer in TEE
// @shm_unregister:	unregister shared memory buffer in TEE
//
// The context given to @open might last longer than the device file if it is
// tied to other resources in the TEE driver. @close_context is called when the
// client closes the device file, even if there are existing references to the
// context. The TEE driver can use @close_context to start cleaning up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_driver_ops {
    pub vers): *mut tee_ioctl_version_data,
    pub len): *mut *mut char buf, size_t,
    pub ctx): *mut *mut int (open)(struct tee_context,
    pub ctx): *mut *mut void (close_context)(struct tee_context,
    pub ctx): *mut *mut void (release)(struct tee_context,
    pub param): *mut tee_param,
    pub session): *mut *mut *mut int (close_session)(struct tee_context ctx, u32,
    pub session): *mut *mut *mut int (system_session)(struct tee_context ctx, u32,
    pub param): *mut tee_param,
    pub param): *mut tee_param,
    pub session): *mut *mut *mut int (cancel_req)(struct tee_context ctx, u32 cancel_id, u32,
    pub param): *mut tee_param,
    pub param): *mut tee_param,
    pub start): c_ulong,
    pub shm): *mut *mut *mut int (shm_unregister)(struct tee_context ctx, struct tee_shm,
}

// Size for TEE revision string buffer used by get_tee_revision().
pub const TEE_REVISION_STR_SIZE: c_int = 128;
pub const TEE_DESC_PRIVILEGED: c_uint = 0x1;
//
// struct tee_desc - Describes the TEE driver to the subsystem
// @name:	name of driver
// @ops:	driver operations vtable
// @owner:	module providing the driver
// @flags:	Extra properties of driver, defined by TEE_DESC_* below
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_desc {
    pub name: *const c_char,
    pub ops: *const tee_driver_ops,
    pub owner: *mut module,
    pub flags: u32,
}

//
// struct tee_protmem_pool - protected memory pool
// @ops:		operations
//
// This is an abstract interface where this struct is expected to be
// embedded in another struct specific to the implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_protmem_pool {
    pub ops: *const tee_protmem_pool_ops,
}

//
// struct tee_protmem_pool_ops - protected memory pool operations
// @alloc:		called when allocating protected memory
// @free:		called when freeing protected memory
// @update_shm:		called when registering a dma-buf to update the @shm
// with physical address of the buffer or to return the
// @parent_shm of the memory pool
// @destroy_pool:	called when destroying the pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_protmem_pool_ops {
    pub offs): *mut size_t size, size_t,
    pub sgt): *mut *mut *mut void (free)(struct tee_protmem_pool pool, struct sg_table,
    pub parent_shm): *mut tee_shm,
    pub pool): *mut *mut void (destroy_pool)(struct tee_protmem_pool,
}

//
// tee_device_alloc() - Allocate a new struct tee_device instance
// @teedesc:	Descriptor for this driver
// @dev:	Parent device for this device
// @pool:	Shared memory pool, NULL if not used
// @driver_data: Private driver data for this device
//
// Allocates a new struct tee_device instance. The device is
// removed by tee_device_unregister().
//
// @returns: a pointer to a 'struct tee_device' or an ERR_PTR on failure
//
// tee_device_register() - Registers a TEE device
// @teedev:	Device to register
//
// tee_device_unregister() need to be called to remove the @teedev if
// this function fails.
//
// @returns: < 0 on failure
//
extern "C" {
    pub fn tee_device_register(teedev: *mut tee_device) -> c_int;
}
//
// tee_device_unregister() - Removes a TEE device
// @teedev:	Device to unregister
//
// This function should be called to remove the @teedev even if
// tee_device_register() hasn't been called yet. Does nothing if
// @teedev is NULL.
//
extern "C" {
    pub fn tee_device_unregister(teedev: *mut tee_device);
}
extern "C" {
    pub fn tee_device_put_all_dma_heaps(teedev: *mut tee_device);
}
//
// tee_device_get() - Increment the user count for a tee_device
// @teedev: Pointer to the tee_device
//
// If tee_device_unregister() has been called and the final user of @teedev
// has already released the device, this function will fail to prevent new users
// from accessing the device during the unregistration process.
//
// Returns: true if @teedev remains valid, otherwise false
//
extern "C" {
    pub fn tee_device_get(teedev: *mut tee_device) -> bool;
}
//
// tee_device_put() - Decrease the user count for a tee_device
// @teedev: pointer to the tee_device
//
extern "C" {
    pub fn tee_device_put(teedev: *mut tee_device);
}
//
// tee_device_set_dev_groups() - Set device attribute groups
// @teedev:	Device to register
// @dev_groups: Attribute groups
//
// Assigns the provided @dev_groups to the @teedev to be registered later
// with tee_device_register(). Calling this function is optional, but if
// it's called it must be called before tee_device_register().
//
// tee_session_calc_client_uuid() - Calculates client UUID for session
// @uuid:		Resulting UUID
// @connection_method:	Connection method for session (TEE_IOCTL_LOGIN_*)
// @connection_data:	Connection data for opening session
//
// Based on connection method calculates UUIDv5 based client UUID.
//
// For group based logins verifies that calling process has specified
// credentials.
//
// @returns: < 0 on failure
//
// struct tee_shm_pool - shared memory pool
// @ops:		operations
// @private_data:	private data for the shared memory manager
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_shm_pool {
    pub ops: *const tee_shm_pool_ops,
    pub private_data: *mut c_void,
}

//
// struct tee_shm_pool_ops - shared memory pool operations
// @alloc:		called when allocating shared memory
// @free:		called when freeing shared memory
// @destroy_pool:	called when destroying the pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_shm_pool_ops {
    pub align): size_t size, size_t,
    pub shm): *mut *mut *mut void (free)(struct tee_shm_pool pool, struct tee_shm,
    pub pool): *mut *mut void (destroy_pool)(struct tee_shm_pool,
}

//
// tee_shm_pool_alloc_res_mem() - Create a shm manager for reserved memory
// @vaddr:	Virtual address of start of pool
// @paddr:	Physical address of start of pool
// @size:	Size in bytes of the pool
//
// @returns: pointer to a 'struct tee_shm_pool' or an ERR_PTR on failure.
//
// tee_shm_pool_free() - Free a shared memory pool
// @pool:	The shared memory pool to free
//
// The must be no remaining shared memory allocated from this pool when
// this function is called.
//
// tee_protmem_static_pool_alloc() - Create a protected memory manager
// @paddr:	Physical address of start of pool
// @size:	Size in bytes of the pool
//
// @returns: pointer to a 'struct tee_protmem_pool' or an ERR_PTR on failure.
//
// tee_get_drvdata() - Return driver_data pointer
// @teedev: Pointer to the tee_device
//
// @returns: the driver_data pointer supplied to tee_register().
//
// tee_shm_alloc_priv_buf() - Allocate shared memory for private use by specific
// TEE driver
// @ctx:	The TEE context for shared memory allocation
// @size:	Shared memory allocation size
// @returns: a pointer to 'struct tee_shm' on success or an ERR_PTR on failure
//
// tee_shm_is_dynamic() - Check if shared memory object is of the dynamic kind
// @shm:	Shared memory handle
// @returns: true if object is dynamic shared memory
//
// tee_shm_put() - Decrease reference count on a shared memory handle
// @shm:	Shared memory handle
//
extern "C" {
    pub fn tee_shm_put(shm: *mut tee_shm);
}
//
// tee_shm_get_id() - Get id of a shared memory object
// @shm:	Shared memory handle
// @returns: id
//
// tee_shm_get_from_id() - Find shared memory object and increase reference
// count
// @ctx:	Context owning the shared memory
// @id:		Id of shared memory object
// @returns: a pointer to 'struct tee_shm' on success or an ERR_PTR on failure
//
// teedev_open() - Open a struct tee_device
// @teedev:	Device to open
//
// @returns: pointer to struct tee_context on success or an ERR_PTR on failure.
//
// teedev_close_context() - closes a struct tee_context
// @ctx:	The struct tee_context to close
//
extern "C" {
    pub fn teedev_close_context(ctx: *mut tee_context);
}
//
// teedev_ctx_get() - Increment the reference count of a context
// @ctx: Pointer to the context
//
// This function increases the refcount of the context, which is tied to
// resources shared by the same tee_device. During the unregistration process,
// the context may remain valid even after tee_device_unregister() has returned.
//
// Users should ensure that the context's refcount is properly decreased before
// calling tee_device_put(), typically within the context's release() function.
// Alternatively, users can call tee_device_get() and teedev_ctx_get() together
// and release them simultaneously (see shm_alloc_helper()).
//
extern "C" {
    pub fn teedev_ctx_get(ctx: *mut tee_context);
}
//
// teedev_ctx_put() - Decrease reference count on a context
// @ctx: pointer to the context
//
extern "C" {
    pub fn teedev_ctx_put(ctx: *mut tee_context);
}
