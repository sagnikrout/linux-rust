//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/liveupdate.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Userspace interface for /dev/liveupdate
// Live Update Orchestrator
//
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

//
// DOC: General ioctl format
//
// The ioctl interface follows a general format to allow for extensibility. Each
// ioctl is passed in a structure pointer as the argument providing the size of
// the structure in the first u32. The kernel checks that any structure space
// beyond what it understands is 0. This allows userspace to use the backward
// compatible portion while consistently using the newer, larger, structures.
//
// ioctls use a standard meaning for common errnos:
//
// - ENOTTY: The IOCTL number itself is not supported at all
// - E2BIG: The IOCTL number is supported, but the provided structure has
// non-zero in a part the kernel does not understand.
// - EOPNOTSUPP: The IOCTL number is supported, and the structure is
// understood, however a known field has a value the kernel does not
// understand or support.
// - EINVAL: Everything about the IOCTL was understood, but a field is not
// correct.
// - ENOENT: A provided token does not exist.
// - ENOMEM: Out of memory.
// - EOVERFLOW: Mathematics overflowed.
//
// As well as additional errnos, within specific ioctls.
//
// The ioctl type, documented in ioctl-number.rst
pub const LIVEUPDATE_IOCTL_TYPE: c_uint = 0xBA;
// The maximum length of session name including null termination
pub const LIVEUPDATE_SESSION_NAME_LENGTH: c_int = 64;
// The /dev/liveupdate ioctl commands
// ioctl commands for session file descriptors
//
// struct liveupdate_ioctl_create_session - ioctl(LIVEUPDATE_IOCTL_CREATE_SESSION)
// @size:	Input; sizeof(struct liveupdate_ioctl_create_session)
// @fd:		Output; The new file descriptor for the created session.
// @name:	Input; A null-terminated string for the session name, max
// length %LIVEUPDATE_SESSION_NAME_LENGTH including termination
// character.
//
// Creates a new live update session for managing preserved resources.
// This ioctl can only be called on the main /dev/liveupdate device.
//
// Return: 0 on success, negative error code on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_ioctl_create_session {
    pub size: __u32,
    pub fd: __s32,
    pub name: [__u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

//
// struct liveupdate_ioctl_retrieve_session - ioctl(LIVEUPDATE_IOCTL_RETRIEVE_SESSION)
// @size:    Input; sizeof(struct liveupdate_ioctl_retrieve_session)
// @fd:      Output; The new file descriptor for the retrieved session.
// @name:    Input; A null-terminated string identifying the session to retrieve.
// The name must exactly match the name used when the session was
// created in the previous kernel.
//
// Retrieves a handle (a new file descriptor) for a preserved session by its
// name. This is the primary mechanism for a userspace agent to regain control
// of its preserved resources after a live update.
//
// The userspace application provides the null-terminated `name` of a session
// it created before the live update. If a preserved session with a matching
// name is found, the kernel instantiates it and returns a new file descriptor
// in the `fd` field. This new session FD can then be used for all file-specific
// operations, such as restoring individual file descriptors with
// LIVEUPDATE_SESSION_RETRIEVE_FD.
//
// It is the responsibility of the userspace application to know the names of
// the sessions it needs to retrieve. If no session with the given name is
// found, the ioctl will fail with -ENOENT.
//
// This ioctl can only be called on the main /dev/liveupdate device when the
// system is in the LIVEUPDATE_STATE_UPDATED state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_ioctl_retrieve_session {
    pub size: __u32,
    pub fd: __s32,
    pub name: [__u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

// Session specific IOCTLs
//
// struct liveupdate_session_preserve_fd - ioctl(LIVEUPDATE_SESSION_PRESERVE_FD)
// @size:  Input; sizeof(struct liveupdate_session_preserve_fd)
// @fd:    Input; The user-space file descriptor to be preserved.
// @token: Input; An opaque, unique token for preserved resource.
//
// Holds parameters for preserving a file descriptor.
//
// User sets the @fd field identifying the file descriptor to preserve
// (e.g., memfd, kvm, iommufd, VFIO). The kernel validates if this FD type
// and its dependencies are supported for preservation. If validation passes,
// the kernel marks the FD internally and *initiates the process* of preparing
// its state for saving. The actual snapshotting of the state typically occurs
// during the subsequent %LIVEUPDATE_IOCTL_PREPARE execution phase, though
// some finalization might occur during freeze.
// On successful validation and initiation, the kernel uses the @token
// field with an opaque identifier representing the resource being preserved.
// This token confirms the FD is targeted for preservation and is required for
// the subsequent %LIVEUPDATE_SESSION_RETRIEVE_FD call after the live update.
//
// Return: 0 on success (validation passed, preservation initiated), negative
// error code on failure (e.g., unsupported FD type, dependency issue,
// validation failed).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_session_preserve_fd {
    pub size: __u32,
    pub fd: __s32,
    pub token: __aligned_u64,
}

//
// struct liveupdate_session_retrieve_fd - ioctl(LIVEUPDATE_SESSION_RETRIEVE_FD)
// @size:  Input; sizeof(struct liveupdate_session_retrieve_fd)
// @fd:    Output; The new file descriptor representing the fully restored
// kernel resource.
// @token: Input; An opaque, token that was used to preserve the resource.
//
// Retrieve a previously preserved file descriptor.
//
// User sets the @token field to the value obtained from a successful
// %LIVEUPDATE_IOCTL_FD_PRESERVE call before the live update. On success,
// the kernel restores the state (saved during the PREPARE/FREEZE phases)
// associated with the token and populates the @fd field with a new file
// descriptor referencing the restored resource in the current (new) kernel.
// This operation must be performed *before* signaling completion via
// %LIVEUPDATE_IOCTL_FINISH. If a retrieve of a token fails, subsequent
// attempts to retrieve the token fail with the same error code. Failed
// retrieves are not retried.
//
// Return: 0 on success, negative error code on failure (e.g., invalid token).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_session_retrieve_fd {
    pub size: __u32,
    pub fd: __s32,
    pub token: __aligned_u64,
}

//
// struct liveupdate_session_finish - ioctl(LIVEUPDATE_SESSION_FINISH)
// @size:     Input; sizeof(struct liveupdate_session_finish)
// @reserved: Input; Must be zero. Reserved for future use.
//
// Signals the completion of the restoration process for a retrieved session.
// This is the final operation that should be performed on a session file
// descriptor after a live update.
//
// This ioctl must be called once all required file descriptors for the session
// have been successfully retrieved (using %LIVEUPDATE_SESSION_RETRIEVE_FD) and
// are fully restored from the userspace and kernel perspective.
//
// Upon success, the kernel releases its ownership of the preserved resources
// associated with this session. This allows internal resources to be freed,
// typically by decrementing reference counts on the underlying preserved
// objects.
//
// If this operation fails, the resources remain preserved in memory. Userspace
// may attempt to call finish again. The resources will otherwise be reset
// during the next live update cycle.
//
// Return: 0 on success, negative error code on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_session_finish {
    pub size: __u32,
    pub reserved: __u32,
}

//
// struct liveupdate_session_get_name - ioctl(LIVEUPDATE_SESSION_GET_NAME)
// @size:  Input; sizeof(struct liveupdate_session_get_name)
// @reserved: Input; Must be zero. Reserved for future use.
// @name:  Output; A null-terminated string with the full session name.
//
// Retrieves the full name of the session associated with this file descriptor.
// This is useful because the kernel may truncate the name shown in /proc.
//
// Return: 0 on success, negative error code on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct liveupdate_session_get_name {
    pub size: __u32,
    pub reserved: __u32,
    pub name: [__u8; LIVEUPDATE_SESSION_NAME_LENGTH],
}

