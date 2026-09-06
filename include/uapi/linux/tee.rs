//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tee.h
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
// Copyright (c) 2015-2016, Linaro Limited
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

//
// This file describes the API provided by a TEE driver to user space.
//
// Each TEE driver defines a TEE specific protocol which is used for the
// data passed back and forth using TEE_IOC_CMD.
//
// Helpers to make the ioctl defines
pub const TEE_IOC_MAGIC: c_uint = 0xa4;
pub const TEE_IOC_BASE: c_int = 0;
pub const TEE_MAX_ARG_SIZE: c_int = 4096;

//
// TEE Implementation ID
//
pub const TEE_IMPL_ID_OPTEE: c_int = 1;
pub const TEE_IMPL_ID_AMDTEE: c_int = 2;
pub const TEE_IMPL_ID_TSTEE: c_int = 3;
pub const TEE_IMPL_ID_QTEE: c_int = 4;
//
// OP-TEE specific capabilities
//

//
// struct tee_ioctl_version_data - TEE version
// @impl_id:	[out] TEE implementation id
// @impl_caps:	[out] Implementation specific capabilities
// @gen_caps:	[out] Generic capabilities, defined by TEE_GEN_CAPS_* above
//
// Identifies the TEE implementation, @impl_id is one of TEE_IMPL_ID_* above.
// @impl_caps is implementation specific, for example TEE_OPTEE_CAP_
// is valid when @impl_id == TEE_IMPL_ID_OPTEE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_version_data {
    pub impl_id: __u32,
    pub impl_caps: __u32,
    pub gen_caps: __u32,
}

//
// TEE_IOC_VERSION - query version of TEE
//
// Takes a tee_ioctl_version_data struct and returns with the TEE version
// data filled in.
//

//
// struct tee_ioctl_shm_alloc_data - Shared memory allocate argument
// @size:	[in/out] Size of shared memory to allocate
// @flags:	[in/out] Flags to/from allocation.
// @id:		[out] Identifier of the shared memory
//
// The flags field should currently be zero as input. Updated by the call
// with actual flags as defined by TEE_IOCTL_SHM_* above.
// This structure is used as argument for TEE_IOC_SHM_ALLOC below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_shm_alloc_data {
    pub size: __u64,
    pub flags: __u32,
    pub id: __s32,
}

//
// TEE_IOC_SHM_ALLOC - allocate shared memory
//
// Allocates shared memory between the user space process and secure OS.
//
// Returns a file descriptor on success or < 0 on failure
//
// The returned file descriptor is used to map the shared memory into user
// space. The shared memory is freed when the descriptor is closed and the
// memory is unmapped.
//

//
// struct tee_ioctl_buf_data - Variable sized buffer
// @buf_ptr:	[in] A __user pointer to a buffer
// @buf_len:	[in] Length of the buffer above
//
// Used as argument for TEE_IOC_OPEN_SESSION, TEE_IOC_INVOKE,
// TEE_IOC_SUPPL_RECV, and TEE_IOC_SUPPL_SEND below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_buf_data {
    pub buf_ptr: __u64,
    pub buf_len: __u64,
}

//
// Attributes for struct tee_ioctl_param, selects field in the union
//

//
// These defines value parameters (struct tee_ioctl_param_value)
//
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT: c_int = 1;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT: c_int = 2;

//
// These defines shared memory reference parameters (struct
// tee_ioctl_param_memref)
//
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT: c_int = 5;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT: c_int = 6;

//
// These defines userspace buffer parameters.
//
pub const TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_INPUT: c_int = 8;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_UBUF_OUTPUT: c_int = 9;

//
// These defines object reference parameters.
//
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INPUT: c_int = 11;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_OUTPUT: c_int = 12;
pub const TEE_IOCTL_PARAM_ATTR_TYPE_OBJREF_INOUT: c_int = 13;
//
// Mask for the type part of the attribute, leaves room for more types
//
pub const TEE_IOCTL_PARAM_ATTR_TYPE_MASK: c_uint = 0xff;
// Meta parameter carrying extra information about the message.
pub const TEE_IOCTL_PARAM_ATTR_META: c_uint = 0x100;
// Mask of all known attr bits

//
// Matches TEEC_LOGIN_* in GP TEE Client API
// Are only defined for GP compliant TEEs
//
pub const TEE_IOCTL_LOGIN_PUBLIC: c_int = 0;
pub const TEE_IOCTL_LOGIN_USER: c_int = 1;
pub const TEE_IOCTL_LOGIN_GROUP: c_int = 2;
pub const TEE_IOCTL_LOGIN_APPLICATION: c_int = 4;
pub const TEE_IOCTL_LOGIN_USER_APPLICATION: c_int = 5;
pub const TEE_IOCTL_LOGIN_GROUP_APPLICATION: c_int = 6;
//
// Disallow user-space to use GP implementation specific login
// method range (0x80000000 - 0xBFFFFFFF). This range is rather
// being reserved for REE kernel clients or TEE implementation.
//
pub const TEE_IOCTL_LOGIN_REE_KERNEL_MIN: c_uint = 0x80000000;
pub const TEE_IOCTL_LOGIN_REE_KERNEL_MAX: c_uint = 0xBFFFFFFF;
// Private login method for REE kernel clients
pub const TEE_IOCTL_LOGIN_REE_KERNEL: c_uint = 0x80000000;
//
// struct tee_ioctl_param - parameter
// @attr: attributes
// @a: if a memref, offset into the shared memory object,
// else if a ubuf, address of the user buffer,
// else if an objref, object identifier, else a value parameter
// @b: if a memref or ubuf, size of the buffer,
// else if objref, flags for the object, else a value parameter
// @c: if a memref, shared memory identifier, else a value parameter
//
// @attr & TEE_PARAM_ATTR_TYPE_MASK indicates if memref, ubuf, or value is
// used in the union. TEE_PARAM_ATTR_TYPE_VALUE_* indicates value,
// TEE_PARAM_ATTR_TYPE_MEMREF_* indicates memref, TEE_PARAM_ATTR_TYPE_UBUF_
// indicates ubuf, and TEE_PARAM_ATTR_TYPE_OBJREF_* indicates objref.
// TEE_PARAM_ATTR_TYPE_NONE indicates that none of the members are used.
//
// Shared memory is allocated with TEE_IOC_SHM_ALLOC which returns an
// identifier representing the shared memory object. A memref can reference
// a part of a shared memory by specifying an offset (@a) and size (@b) of
// the object. To supply the entire shared memory object set the offset
// (@a) to 0 and size (@b) to the previously returned size of the object.
//
// A client may need to present a NULL pointer in the argument
// passed to a trusted application in the TEE.
// This is also a requirement in GlobalPlatform Client API v1.0c
// (section 3.2.5 memory references), which can be found at
// http://www.globalplatform.org/specificationsdevice.asp
//
// If a NULL pointer is passed to a TA in the TEE, the (@c)
// IOCTL parameters value must be set to TEE_MEMREF_NULL indicating a NULL
// memory reference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_param {
    pub attr: __u64,
    pub a: __u64,
    pub b: __u64,
    pub c: __u64,
}

pub const TEE_IOCTL_UUID_LEN: c_int = 16;
//
// struct tee_ioctl_open_session_arg - Open session argument
// @uuid:	[in] UUID of the Trusted Application
// @clnt_uuid:	[in] UUID of client
// @clnt_login:	[in] Login class of client, TEE_IOCTL_LOGIN_* above
// @cancel_id:	[in] Cancellation id, a unique value to identify this request
// @session:	[out] Session id
// @ret:	[out] return value
// @ret_origin:	[out] origin of the return value
// @num_params:	[in] number of &struct tee_ioctl_param entries in @params
// @params:	array of ioctl parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_open_session_arg {
    pub uuid: [__u8; TEE_IOCTL_UUID_LEN],
    pub clnt_uuid: [__u8; TEE_IOCTL_UUID_LEN],
    pub clnt_login: __u32,
    pub cancel_id: __u32,
    pub session: __u32,
    pub ret: __u32,
    pub ret_origin: __u32,
    pub num_params: __u32,
// num_params tells the actual number of element in params
    pub params: [tee_ioctl_param; ],
}

//
// TEE_IOC_OPEN_SESSION - opens a session to a Trusted Application
//
// Takes a struct tee_ioctl_buf_data which contains a struct
// tee_ioctl_open_session_arg followed by any array of struct
// tee_ioctl_param
//

//
// struct tee_ioctl_invoke_arg - Invokes a function in a Trusted Application
// @func:	[in] Trusted Application function, specific to the TA
// @session:	[in] Session id
// @cancel_id:	[in] Cancellation id, a unique value to identify this request
// @ret:	[out] return value
// @ret_origin:	[out] origin of the return value
// @num_params:	[in] number of parameters following this struct
// @params:	array of ioctl parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_invoke_arg {
    pub func: __u32,
    pub session: __u32,
    pub cancel_id: __u32,
    pub ret: __u32,
    pub ret_origin: __u32,
    pub num_params: __u32,
// num_params tells the actual number of element in params
    pub params: [tee_ioctl_param; ],
}

//
// TEE_IOC_INVOKE - Invokes a function in a Trusted Application
//
// Takes a struct tee_ioctl_buf_data which contains a struct
// tee_invoke_func_arg followed by any array of struct tee_param
//

//
// struct tee_ioctl_cancel_arg - Cancels an open session or invoke ioctl
// @cancel_id:	[in] Cancellation id, a unique value to identify this request
// @session:	[in] Session id, if the session is opened, else set to 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_cancel_arg {
    pub cancel_id: __u32,
    pub session: __u32,
}

//
// TEE_IOC_CANCEL - Cancels an open session or invoke
//

//
// struct tee_ioctl_close_session_arg - Closes an open session
// @session:	[in] Session id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_close_session_arg {
    pub session: __u32,
}

//
// TEE_IOC_CLOSE_SESSION - Closes a session
//

//
// struct tee_iocl_supp_recv_arg - Receive a request for a supplicant function
// @func:	[in] supplicant function
// @num_params:	[in/out] number of &struct tee_ioctl_param entries in @params
// @params:	array of ioctl parameters
//
// @num_params is the number of params that tee-supplicant has room to
// receive when input, @num_params is the number of actual params
// tee-supplicant receives when output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_iocl_supp_recv_arg {
    pub func: __u32,
    pub num_params: __u32,
// num_params tells the actual number of element in params
    pub params: [tee_ioctl_param; ],
}

//
// TEE_IOC_SUPPL_RECV - Receive a request for a supplicant function
//
// Takes a struct tee_ioctl_buf_data which contains a struct
// tee_iocl_supp_recv_arg followed by any array of struct tee_param
//

//
// struct tee_iocl_supp_send_arg - Send a response to a received request
// @ret:	[out] return value
// @num_params:	[in] number of &struct tee_ioctl_param entries in @params
// @params:	array of ioctl parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_iocl_supp_send_arg {
    pub ret: __u32,
    pub num_params: __u32,
// num_params tells the actual number of element in params
    pub params: [tee_ioctl_param; ],
}

//
// TEE_IOC_SUPPL_SEND - Send a response to a received request
//
// Takes a struct tee_ioctl_buf_data which contains a struct
// tee_iocl_supp_send_arg followed by any array of struct tee_param
//

//
// struct tee_ioctl_shm_register_data - Shared memory register argument
// @addr:      [in] Start address of shared memory to register
// @length:    [in/out] Length of shared memory to register
// @flags:     [in/out] Flags to/from registration.
// @id:                [out] Identifier of the shared memory
//
// The flags field should currently be zero as input. Updated by the call
// with actual flags as defined by TEE_IOCTL_SHM_* above.
// This structure is used as argument for TEE_IOC_SHM_REGISTER below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_shm_register_data {
    pub addr: __u64,
    pub length: __u64,
    pub flags: __u32,
    pub id: __s32,
}

//
// struct tee_ioctl_shm_register_fd_data - Shared memory registering argument
// @fd:		[in] File descriptor identifying dmabuf reference
// @size:	[out] Size of referenced memory
// @flags:	[in] Flags to/from allocation.
// @id:		[out] Identifier of the shared memory
//
// The flags field should currently be zero as input. Updated by the call
// with actual flags as defined by TEE_IOCTL_SHM_* above.
// This structure is used as argument for TEE_IOC_SHM_REGISTER_FD below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_shm_register_fd_data {
    pub fd: __s64,
    pub size: __u64,
    pub flags: __u32,
    pub id: __s32,
}

//
// TEE_IOC_SHM_REGISTER_FD - register a shared memory from a file descriptor
//
// Returns a file descriptor on success or < 0 on failure
//
// The returned file descriptor refers to the shared memory object in the
// kernel. The supplied file deccriptor can be closed if it's not needed
// for other purposes. The shared memory is freed when the descriptor is
// closed.
//

//
// TEE_IOC_SHM_REGISTER - Register shared memory argument
//
// Registers shared memory between the user space process and secure OS.
//
// Returns a file descriptor on success or < 0 on failure
//
// The shared memory is unregisterred when the descriptor is closed.
//

//
// Five syscalls are used when communicating with the TEE driver.
// open(): opens the device associated with the driver
// ioctl(): as described above operating on the file descriptor from open()
// close(): two cases
// - closes the device file descriptor
// - closes a file descriptor connected to allocated shared memory
// mmap(): maps shared memory into user space using information from struct
// tee_ioctl_shm_alloc_data
// munmap(): unmaps previously shared memory
//
// struct tee_ioctl_object_invoke_arg - Invokes an object in a
// Trusted Application
// @id:		[in] Object id
// @op:		[in] Object operation, specific to the object
// @ret:	[out] return value
// @num_params:	[in] number of parameters following this struct
// @params:	array of ioctl parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tee_ioctl_object_invoke_arg {
    pub id: __u64,
    pub op: __u32,
    pub ret: __u32,
    pub num_params: __u32,
    pub :32: __u32,
// num_params tells the actual number of element in params
    pub params: [tee_ioctl_param; ],
}

