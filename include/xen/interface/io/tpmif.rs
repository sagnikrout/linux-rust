//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/io/tpmif.h
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
// tpmif.h
//
// TPM I/O interface for Xen guest OSes, v2
//
// This file is in the public domain.
//
// Xenbus state machine
//
// Device open:
// 1. Both ends start in XenbusStateInitialising
// 2. Backend transitions to InitWait (frontend does not wait on this step)
// 3. Frontend populates ring-ref, event-channel, feature-protocol-v2
// 4. Frontend transitions to Initialised
// 5. Backend maps grant and event channel, verifies feature-protocol-v2
// 6. Backend transitions to Connected
// 7. Frontend verifies feature-protocol-v2, transitions to Connected
//
// Device close:
// 1. State is changed to XenbusStateClosing
// 2. Frontend transitions to Closed
// 3. Backend unmaps grant and event, changes state to InitWait
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vtpm_shared_page_state {
    VTPM_STATE_IDLE,         /* no contents / vTPM idle / cancel complete */
    VTPM_STATE_SUBMIT,       /* request ready / vTPM working */
    VTPM_STATE_FINISH,       /* response ready / vTPM idle */
    VTPM_STATE_CANCEL,       /* cancel requested / vTPM working */
}

// The backend should only change state to IDLE or FINISH, while the
// frontend should only change to SUBMIT or CANCEL.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vtpm_shared_page {
    pub /: *mut *mut uint32_t length; / request/response length in bytes,
    pub /: *mut *mut uint8_t state; / enum vtpm_shared_page_state,
    pub /: *mut *mut uint8_t locality; / for the current request,
    pub pad: u8,
    pub /: *mut *mut uint8_t nr_extra_pages; / extra pages for long packets; may be zero,
    pub /: *mut *mut uint32_t extra_pages[]; / grant IDs; length in nr_extra_pages,
}
