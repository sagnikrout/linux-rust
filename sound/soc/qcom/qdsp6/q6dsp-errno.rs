//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/q6dsp-errno.h
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

// Success. The operation completed with no errors.
pub const ADSP_EOK: c_uint = 0x00000000;
// General failure.
pub const ADSP_EFAILED: c_uint = 0x00000001;
// Bad operation parameter.
pub const ADSP_EBADPARAM: c_uint = 0x00000002;
// Unsupported routine or operation.
pub const ADSP_EUNSUPPORTED: c_uint = 0x00000003;
// Unsupported version.
pub const ADSP_EVERSION: c_uint = 0x00000004;
// Unexpected problem encountered.
pub const ADSP_EUNEXPECTED: c_uint = 0x00000005;
// Unhandled problem occurred.
pub const ADSP_EPANIC: c_uint = 0x00000006;
// Unable to allocate resource.
pub const ADSP_ENORESOURCE: c_uint = 0x00000007;
// Invalid handle.
pub const ADSP_EHANDLE: c_uint = 0x00000008;
// Operation is already processed.
pub const ADSP_EALREADY: c_uint = 0x00000009;
// Operation is not ready to be processed.
pub const ADSP_ENOTREADY: c_uint = 0x0000000A;
// Operation is pending completion.
pub const ADSP_EPENDING: c_uint = 0x0000000B;
// Operation could not be accepted or processed.
pub const ADSP_EBUSY: c_uint = 0x0000000C;
// Operation aborted due to an error.
pub const ADSP_EABORTED: c_uint = 0x0000000D;
// Operation preempted by a higher priority.
pub const ADSP_EPREEMPTED: c_uint = 0x0000000E;
// Operation requests intervention to complete.
pub const ADSP_ECONTINUE: c_uint = 0x0000000F;
// Operation requests immediate intervention to complete.
pub const ADSP_EIMMEDIATE: c_uint = 0x00000010;
// Operation is not implemented.
pub const ADSP_ENOTIMPL: c_uint = 0x00000011;
// Operation needs more data or resources.
pub const ADSP_ENEEDMORE: c_uint = 0x00000012;
// Operation does not have memory.
pub const ADSP_ENOMEMORY: c_uint = 0x00000014;
// Item does not exist.
pub const ADSP_ENOTEXIST: c_uint = 0x00000015;
// Max count for adsp error code sent to HLOS
