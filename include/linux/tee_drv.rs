//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tee_drv.h
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
// Copyright (c) 2015-2024 Linaro Limited
//

//
// The file describes the API provided by the TEE subsystem to the
// TEE client drivers.
//
// struct tee_context - driver specific context on file pointer data
// @teedev:	pointer to this drivers struct tee_device
// @data:	driver specific context data, managed by the driver
// @refcount:	reference counter for this structure
// @releasing:  flag that indicates if context is being released right now.
// It is needed to break circular dependency on context during
// shared memory release.
// @supp_nowait: flag that indicates that requests in this context should not
// wait for tee-supplicant daemon to be started if not present
// and just return with an error code. It is needed for requests
// that arises from TEE based kernel drivers that should be
// non-blocking in nature.
// @cap_memref_null: flag indicating if the TEE Client support shared
// memory buffer with a NULL pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_context {
    pub teedev: *mut tee_device,
    pub data: *mut c_void,
    pub refcount: kref,
    pub releasing: bool,
    pub supp_nowait: bool,
    pub cap_memref_null: bool,
}

//
// struct tee_shm - shared memory object
// @ctx:	context using the object
// @paddr:	physical address of the shared memory
// @kaddr:	virtual address of the shared memory
// @size:	size of shared memory
// @offset:	offset of buffer in user space
// @pages:	locked pages from userspace
// @num_pages:	number of locked pages
// @refcount:	reference counter
// @flags:	defined by TEE_SHM_* in tee_core.h
// @id:		unique id of a shared memory object on this device, shared
// with user space
// @sec_world_id:
// secure world assigned id of this shared memory object, not
// used by all drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_shm {
    pub ctx: *mut tee_context,
    pub paddr: phys_addr_t,
    pub kaddr: *mut c_void,
    pub size: usize,
    pub offset: c_uint,
    pub pages: *mut page,
    pub num_pages: usize,
    pub refcount: refcount_t,
    pub flags: u32,
    pub id: c_int,
    pub sec_world_id: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_param_memref {
    pub shm_offs: usize,
    pub size: usize,
    pub shm: *mut tee_shm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_param_ubuf {
    pub uaddr: *mut void __user,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_param_objref {
    pub id: u64,
    pub flags: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_param_value {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_param {
    pub attr: u64,
    pub memref: tee_param_memref,
    pub objref: tee_param_objref,
    pub ubuf: tee_param_ubuf,
    pub value: tee_param_value,
    pub u: },
}

//
// tee_shm_alloc_kernel_buf() - Allocate kernel shared memory for a
// particular TEE client driver
// @ctx:	The TEE context for shared memory allocation
// @size:	Shared memory allocation size
// @returns a pointer to 'struct tee_shm' on success or an ERR_PTR on failure
//
// tee_shm_register_kernel_buf() - Register kernel shared memory for a
// particular TEE client driver
// @ctx:	The TEE context for shared memory registration
// @addr:	Kernel buffer address
// @length:	Kernel buffer length
// @returns a pointer to 'struct tee_shm' on success or an ERR_PTR on failure
//
// tee_shm_register_fd() - Register shared memory from file descriptor
//
// @ctx:	Context that allocates the shared memory
// @fd:		Shared memory file descriptor reference
//
// @returns a pointer to 'struct tee_shm' on success, and ERR_PTR on failure
//
// tee_shm_free() - Free shared memory
// @shm:	Handle to shared memory to free
//
extern "C" {
    pub fn tee_shm_free(shm: *mut tee_shm);
}
//
// tee_shm_get_va() - Get virtual address of a shared memory plus an offset
// @shm:	Shared memory handle
// @offs:	Offset from start of this shared memory
// @returns virtual address of the shared memory + offs if offs is within
// the bounds of this shared memory, else an ERR_PTR
//
// tee_shm_get_pa() - Get physical address of a shared memory plus an offset
// @shm:	Shared memory handle
// @offs:	Offset from start of this shared memory
// @pa:		Physical address to return
// @returns 0 if offs is within the bounds of this shared memory, else an
// error code.
//
extern "C" {
    pub fn tee_shm_get_pa(shm: *mut tee_shm, offs: usize, pa: *mut phys_addr_t) -> c_int;
}
//
// tee_shm_get_size() - Get size of shared memory buffer
// @shm:	Shared memory handle
// @returns size of shared memory
//
// tee_shm_get_pages() - Get list of pages that hold shared buffer
// @shm:	Shared memory handle
// @num_pages:	Number of pages will be stored there
// @returns pointer to pages array
//
// num_pages = shm->num_pages;
//
// tee_shm_get_page_offset() - Get shared buffer offset from page start
// @shm:	Shared memory handle
// @returns page offset of shared buffer
//
// tee_client_open_context() - Open a TEE context
// @start:	if not NULL, continue search after this context
// @match:	function to check TEE device
// @data:	data for match function
// @vers:	if not NULL, version data of TEE device of the context returned
//
// This function does an operation similar to open("/dev/teeX") in user space.
// A returned context must be released with tee_client_close_context().
//
// Returns a TEE context of the first TEE device matched by the match()
// callback or an ERR_PTR.
//
// tee_client_close_context() - Close a TEE context
// @ctx:	TEE context to close
//
// Note that all sessions previously opened with this context will be
// closed when this function is called.
//
extern "C" {
    pub fn tee_client_close_context(ctx: *mut tee_context);
}
//
// tee_client_get_version() - Query version of TEE
// @ctx:	TEE context to TEE to query
// @vers:	Pointer to version data
//
// tee_client_open_session() - Open a session to a Trusted Application
// @ctx:	TEE context
// @arg:	Open session arguments, see description of
// struct tee_ioctl_open_session_arg
// @param:	Parameters passed to the Trusted Application
//
// Returns < 0 on error else see @arg->ret for result. If @arg->ret
// is TEEC_SUCCESS the session identifier is available in @arg->session.
//
// tee_client_close_session() - Close a session to a Trusted Application
// @ctx:	TEE Context
// @session:	Session id
//
// Return < 0 on error else 0, regardless the session will not be
// valid after this function has returned.
//
extern "C" {
    pub fn tee_client_close_session(ctx: *mut tee_context, session: u32) -> c_int;
}
//
// tee_client_system_session() - Declare session as a system session
// @ctx:	TEE Context
// @session:	Session id
//
// This function requests TEE to provision an entry context ready to use for
// that session only. The provisioned entry context is used for command
// invocation and session closure, not for command cancelling requests.
// TEE releases the provisioned context upon session closure.
//
// Return < 0 on error else 0 if an entry context has been provisioned.
//
extern "C" {
    pub fn tee_client_system_session(ctx: *mut tee_context, session: u32) -> c_int;
}
//
// tee_client_invoke_func() - Invoke a function in a Trusted Application
// @ctx:	TEE Context
// @arg:	Invoke arguments, see description of
// struct tee_ioctl_invoke_arg
// @param:	Parameters passed to the Trusted Application
//
// Returns < 0 on error else see @arg->ret for result.
//
// tee_client_cancel_req() - Request cancellation of the previous open-session
// or invoke-command operations in a Trusted Application
// @ctx:       TEE Context
// @arg:       Cancellation arguments, see description of
// struct tee_ioctl_cancel_arg
//
// Returns < 0 on error else 0 if the cancellation was successfully requested.
//
// struct tee_client_device - tee based device
// @id:			device identifier
// @dev:		device structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_client_device {
    pub id: tee_client_device_id,
    pub dev: device,
}

//
// struct tee_client_driver - tee client driver
// @id_table:		device id table supported by this driver
// @driver:		driver structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_client_driver {
    pub ): *mut *mut int (probe)(struct tee_client_device,
    pub ): *mut *mut void (remove)(struct tee_client_device,
    pub ): *mut *mut void (shutdown)(struct tee_client_device,
    pub id_table: *const tee_client_device_id,
    pub driver: device_driver,
}

extern "C" {
    pub fn __tee_client_driver_register(: *mut tee_client_driver, : *mut module) -> c_int;
}
extern "C" {
    pub fn tee_client_driver_unregister(: *mut tee_client_driver);
}

