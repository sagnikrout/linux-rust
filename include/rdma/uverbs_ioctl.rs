//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/uverbs_ioctl.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved.
//

//
// =======================================
// Verbs action specifications
// =======================================
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uverbs_attr_type {
    UVERBS_ATTR_TYPE_NA,
    UVERBS_ATTR_TYPE_PTR_IN,
    UVERBS_ATTR_TYPE_PTR_OUT,
    UVERBS_ATTR_TYPE_IDR,
    UVERBS_ATTR_TYPE_FD,
    UVERBS_ATTR_TYPE_RAW_FD,
    UVERBS_ATTR_TYPE_ENUM_IN,
    UVERBS_ATTR_TYPE_IDRS_ARRAY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uverbs_obj_access {
    UVERBS_ACCESS_READ,
    UVERBS_ACCESS_WRITE,
    UVERBS_ACCESS_NEW,
    UVERBS_ACCESS_DESTROY
}

// Specification of a single attribute inside the ioctl message
// good size 16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_attr_spec {
    pub type: u8,
//
// Support extending attributes by length. Allow the user to provide
// more bytes than ptr.len, but check that everything after is zero'd
// by the user.
//
    pub zero_trailing:1: u8,
//
// Valid only for PTR_IN. Allocate and copy the data inside
// the parser
//
    pub alloc_and_copy:1: u8,
    pub mandatory:1: u8,
// True if this is from UVERBS_ATTR_UHW
    pub is_udata:1: u8,
// Current known size to kernel
    pub len: u16,
// User isn't allowed to provide something < min_len
    pub min_len: u16,
    pub ptr: },
//
// higher bits mean the namespace and lower bits mean
// the type id within the namespace.
//
    pub obj_type: u16,
    pub access: u8,
    pub obj: },
    pub num_elems: u8,
    pub enum_def: },
    pub u: },
// This weird split lets us remove some padding
//
// The enum attribute can select one of the attributes
// contained in the ids array. Currently only PTR_IN
// attributes are supported in the ids array.
//
    pub ids: *const uverbs_attr_spec,
    pub enum_def: },
//
// higher bits mean the namespace and lower bits mean
// the type id within the namespace.
//
    pub obj_type: u16,
    pub min_len: u16,
    pub max_len: u16,
    pub access: u8,
    pub objs_arr: },
    pub u2: },
}

//
// Information about the API is loaded into a radix tree. For IOCTL we start
// with a tuple of:
// object_id, attr_id, method_id
//
// Which is a 48 bit value, with most of the bits guaranteed to be zero. Based
// on the current kernel support this is compressed into 16 bit key for the
// radix tree. Since this compression is entirely internal to the kernel the
// below limits can be revised if the kernel gains additional data.
//
// With 64 leafs per node this is a 3 level radix tree.
//
// The tree encodes multiple types, and uses a scheme where OBJ_ID,0,0 returns
// the object slot, and OBJ_ID,METH_ID,0 and returns the method slot.
//
// This also encodes the tables for the write() and write() extended commands
// using the coding
// OBJ_ID,UVERBS_API_METHOD_IS_WRITE,command #
// OBJ_ID,UVERBS_API_METHOD_IS_WRITE_EX,command_ex #
// ie the WRITE path is treated as a special method type in the ioctl
// framework.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uapi_radix_data {
    UVERBS_API_NS_FLAG = 1U << UVERBS_ID_NS_SHIFT,

    UVERBS_API_ATTR_KEY_BITS = 6,
    UVERBS_API_ATTR_KEY_MASK = GENMASK(UVERBS_API_ATTR_KEY_BITS - 1, 0),
    UVERBS_API_ATTR_BKEY_LEN = (1 << UVERBS_API_ATTR_KEY_BITS) - 1,
    UVERBS_API_WRITE_KEY_NUM = 1 << UVERBS_API_ATTR_KEY_BITS,

    UVERBS_API_METHOD_KEY_BITS = 5,
    UVERBS_API_METHOD_KEY_SHIFT = UVERBS_API_ATTR_KEY_BITS,
    UVERBS_API_METHOD_KEY_NUM_CORE = 22,
    UVERBS_API_METHOD_IS_WRITE = 30 << UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_METHOD_IS_WRITE_EX = 31 << UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_METHOD_KEY_NUM_DRIVER =
    (UVERBS_API_METHOD_IS_WRITE >> UVERBS_API_METHOD_KEY_SHIFT) -
    UVERBS_API_METHOD_KEY_NUM_CORE,
    UVERBS_API_METHOD_KEY_MASK = GENMASK(
    UVERBS_API_METHOD_KEY_BITS + UVERBS_API_METHOD_KEY_SHIFT - 1,
    UVERBS_API_METHOD_KEY_SHIFT),

    UVERBS_API_OBJ_KEY_BITS = 5,
    UVERBS_API_OBJ_KEY_SHIFT =
    UVERBS_API_METHOD_KEY_BITS + UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_OBJ_KEY_NUM_CORE = 20,
    UVERBS_API_OBJ_KEY_NUM_DRIVER =
    (1 << UVERBS_API_OBJ_KEY_BITS) - UVERBS_API_OBJ_KEY_NUM_CORE,
    UVERBS_API_OBJ_KEY_MASK = GENMASK(31, UVERBS_API_OBJ_KEY_SHIFT),

// This id guaranteed to not exist in the radix tree
    UVERBS_API_KEY_ERR = 0xFFFFFFFF,
}

// 0 is the method slot itself
//
// The attr is designed to fit in the typical single radix tree node
// of 64 entries. Since allmost all methods have driver attributes we
// organize things so that the driver and core attributes interleave to
// reduce the length of the attributes array in typical cases.
//
// Only true for ioctl methods
//
// This returns a value in the range [0 to UVERBS_API_ATTR_BKEY_LEN),
// basically it undoes the reservation of 0 in the ID numbering. attr_key
// must already be masked with UVERBS_API_ATTR_KEY_MASK, or be the output of
// uapi_key_attr().
//
// =======================================
// Verbs definitions
// =======================================
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_attr_def {
    pub id: u16,
    pub attr: uverbs_attr_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_method_def {
    pub id: u16,
// Combination of bits from enum UVERBS_ACTION_FLAG_XXXX
    pub flags: u32,
    pub num_attrs: usize,
    pub (*attrs)[]: *const *const uverbs_attr_def,
    pub attrs): *mut *mut int (handler)(struct uverbs_attr_bundle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_object_def {
    pub id: u16,
    pub type_attrs: *const uverbs_obj_type,
    pub num_methods: usize,
    pub (*methods)[]: *const *const uverbs_method_def,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uapi_definition_kind {
    UAPI_DEF_END = 0,
    UAPI_DEF_OBJECT_START,
    UAPI_DEF_WRITE,
    UAPI_DEF_CHAIN_OBJ_TREE,
    UAPI_DEF_CHAIN,
    UAPI_DEF_IS_SUPPORTED_FUNC,
    UAPI_DEF_IS_SUPPORTED_DEV_FN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uapi_definition_scope {
    UAPI_SCOPE_OBJECT = 1,
    UAPI_SCOPE_METHOD = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uapi_definition {
    pub kind: u8,
    pub scope: u8,
    pub object_id: u16,
    pub object_start: },
    pub command_num: u16,
    pub is_ex:1: u8,
    pub has_udata:1: u8,
    pub has_resp:1: u8,
    pub req_size: u8,
    pub resp_size: u8,
    pub write: },
}

// Define things connected to object_id

// Use in a var_args of DECLARE_UVERBS_OBJECT

// Use in a var_args of DECLARE_UVERBS_OBJECT

//
// Object is only supported if the function pointer named ibdev_fn in struct
// ib_device is not NULL.
//

//
// Method is only supported if the function pointer named ibdev_fn in struct
// ib_device is not NULL.
//

// Call a function to determine if the entire object is supported or not

// Include another struct uapi_definition in this one

// Temporary until the tree base description is replaced

//
// =======================================
// Attribute Specifications
// =======================================
//

//
// Specifies a uapi structure that cannot be extended. The user must always
// supply the whole structure and nothing more. The structure must be declared
// in a header under include/uapi/rdma.
//

//
// Specifies a uapi structure where the user must provide at least up to
// member 'last'.  Anything after last and up until the end of the structure
// can be non-zero, anything longer than the end of the structure must be
// zero. The structure must be declared in a header under include/uapi/rdma.
//

//
// Specifies at least min_len bytes must be passed in, but the amount can be
// larger, up to the protocol maximum size. No check for zeroing is done.
//

// Must be used in the '...' of any UVERBS_ATTR

//
// min_len must be bigger than 0 and _max_len must be smaller than 4095.  Only
// READ\WRITE accesses are supported.
//

//
// Only for use with UVERBS_ATTR_IDR, allows any uobject type to be accepted,
// the user must validate the type of the uobject instead.
//
pub const UVERBS_IDR_ANY_OBJECT: c_uint = 0xFFFF;

// _enum_arry should be a 'static const union uverbs_attr_spec[]'

// An input value that is a member in the enum _enum_type.

//
// An input value that is a bitwise combination of values of _enum_type.
// This permits the flag value to be passed as either a u32 or u64, it must
// be retrieved via uverbs_get_flag().
//

//
// This spec is used in order to pass information to the hardware driver in a
// legacy way. Every verb that could get driver specific data should get this
// spec.
//

//
// Per-attribute UMEM descriptor. The payload is a single
// struct ib_uverbs_buffer_desc identifying a memory region backed by
// dma-buf or user virtual address. _access selects UA_OPTIONAL or
// UA_MANDATORY. Drivers obtain a umem from the attribute via the
// ib_umem_get_*() wrapper helpers.
//

//
// Bit masks of the @flags / @optional_flags fields of struct
// ib_uverbs_buffer_desc that the kernel understands. @flags is strict:
// any bit outside the known mask makes the call fail with -EINVAL.
// @optional_flags is advisory: bits outside the known mask are silently
// dropped. Both masks are extended as new bits are introduced.
//

// =================================================
// Parsing infrastructure
// =================================================
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_ptr_attr {
//
// If UVERBS_ATTR_SPEC_F_ALLOC_AND_COPY is set then the 'ptr' is
// used.
//
    pub ptr: *mut c_void,
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_obj_attr {
    pub uobject: *mut ib_uobject,
    pub attr_elm: *const uverbs_api_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_objs_arr_attr {
    pub uobjects: *mut ib_uobject,
    pub len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_attr {
    pub ptr_attr: uverbs_ptr_attr,
    pub obj_attr: uverbs_obj_attr,
    pub objs_arr_attr: uverbs_objs_arr_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_attr_bundle {
    pub driver_udata: ib_udata,
    pub ucore: ib_udata,
    pub ufile: *mut ib_uverbs_file,
    pub context: *mut ib_ucontext,
    pub uobject: *mut ib_uobject,
    pub method_elm: *const uverbs_api_ioctl_method,
    pub UVERBS_API_ATTR_BKEY_LEN): DECLARE_BITMAP(attr_present,,
    pub attrs: [uverbs_attr; ],
}

//
// rdma_udata_to_drv_context - Helper macro to get the driver's context out of
// ib_udata which is embedded in uverbs_attr_bundle.
//
// If udata is not NULL this cannot fail. Otherwise a NULL udata will result
// in a NULL ucontext pointer, as a safety precaution. Callers should be using
// 'udata' to determine if the driver call is in user or kernel mode, not
// 'ucontext'.
//
extern "C" {
    pub fn container_of(_arg: udata, uverbs_attr_bundle: struct, _arg: driver_udata) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: attr) -> return;
}
extern "C" {
    pub fn ERR_CAST(_arg: attr) -> return;
}
extern "C" {
    pub fn ERR_CAST(_arg: attr) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: attr) -> return;
}
//
// uverbs_attr_ptr_get_array_size() - Get array size pointer by a ptr
// attribute.
// @attrs: The attribute bundle
// @idx: The ID of the attribute
// @elem_size: The size of the element in the array
//
// uverbs_attr_get_uobjs_arr() - Provides array's properties for attribute for
// UVERBS_ATTR_TYPE_IDRS_ARRAY.
// @arr: Returned pointer to array of pointers for uobjects or NULL if
// the attribute isn't provided.
//
// Return: The array length or 0 if no attribute was provided.
//
// arr = NULL;
// arr = attr->objs_arr_attr.uobjects;
extern "C" {
    pub fn PTR_ERR(_arg: attr) -> return;
}
//
// Validation ensures attr->ptr_attr.len >= size. If the caller is
// using UVERBS_ATTR_SPEC_F_MIN_SZ_OR_ZERO then it must call
// uverbs_copy_from_or_zero.
//
extern "C" {
    pub fn PTR_ERR(_arg: attr) -> return;
}

extern "C" {
    pub fn ib_uverbs_get_ucontext_file(_arg: attrs->ufile) -> return;
}

extern "C" {
    pub fn _uverbs_alloc(_arg: bundle, _arg: size, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn _uverbs_alloc(_arg: bundle, _arg: size, __GFP_ZERO: GFP_KERNEL |) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOVERFLOW) -> return;
}
extern "C" {
    pub fn uverbs_zalloc(_arg: bundle, _arg: bytes) -> return;
}
extern "C" {
    pub fn _ib_respond_udata(udata: *mut ib_udata, src: *const c_void, len: usize) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn uverbs_get_const_signed(_arg: to, _arg: attrs_bundle, _arg: idx) -> return;
}
//
// ib_copy_validate_udata_in - Copy and validate that the request structure is
// compatible with this kernel
// @_udata: The system calls ib_udata struct
// @_req: The name of an on-stack structure that holds the driver data
// @_end_member: The member in the struct that is the original end of struct
// from the first kernel to introduce it.
//
// Check that the udata input request struct is properly formed for this kernel.
// Then copy it into req
//

//
// ib_copy_validate_udata_in_cm - Copy the req structure and check the comp_mask
// @_udata: The system calls ib_udata struct
// @_req: The name of an on-stack structure that holds the driver data
// @_end_member: The member in the struct that is the original end of struct
// from the first kernel to introduce it.
// @_valid_cm: A bitmask of bits permitted in the comp_mask_field.
//
// Check that the udata input request struct is properly formed for this kernel.
// Then copy it into req
//

//
// ib_is_udata_in_empty - Check if the udata input buffer is all zeros
// @udata: The system calls ib_udata struct
//
// This should be used if the driver does not currently define a driver data
// struct. Returns 0 if the buffer is empty or all zeros, -EOPNOTSUPP if
// non-zero data is present, or a negative error code on failure.
//
extern "C" {
    pub fn _ib_copy_validate_udata_in(_arg: udata, _arg: NULL, _arg: 0, _arg: 0) -> return;
}
//
// ib_respond_udata - Copy a driver data response to userspace
// @_udata: The system calls ib_udata struct
// @_rep: Kernel buffer containing the response driver data on the stack
//
// Copy driver data response structures back to userspace in a way that
// is forwards and backwards compatible. Longer kernel structs are truncated,
// userspace has made some kind of error if it needed the truncated information.
// Shorter structs are zero padded.
//

//
// ib_respond_empty_udata - Zero fill the response buffer to userspace
// @_udata: The system calls ib_udata struct
//
// Used when there is no driver response data to return. Provides forward
// compatability by zeroing any buffer the user may have provided.
//
// ib_no_udata_io - Ensure no input data and zero fill the response buffer
// @udata: The system call's ib_udata struct
//
// Driver ops which do not accept any input data and do not provide any response
// data may call this at the beginning of their handler to fully adhere to the
// uAPI forward/backward compatibility rules.
//
// Return: Negative failure code if the op should be denied, 0 otherwise.
//
extern "C" {
    pub fn ib_respond_empty_udata(_arg: udata) -> return;
}
