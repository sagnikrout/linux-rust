//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/goldfish/goldfish_pipe_qemu.h
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
// IMPORTANT: The following constants must match the ones used and defined in
// external/qemu/include/hw/misc/goldfish_pipe.h
//
// List of bitflags returned in status of CMD_POLL command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipePollFlags {
    PIPE_POLL_IN	= 1 << 0,
    PIPE_POLL_OUT	= 1 << 1,
    PIPE_POLL_HUP	= 1 << 2
}

// Possible status values used to signal errors
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeErrors {
    PIPE_ERROR_INVAL	= -1,
    PIPE_ERROR_AGAIN	= -2,
    PIPE_ERROR_NOMEM	= -3,
    PIPE_ERROR_IO		= -4
}

// Bit-flags used to signal events from the emulator
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeWakeFlags {
// emulator closed pipe
    PIPE_WAKE_CLOSED		= 1 << 0,

// pipe can now be read from
    PIPE_WAKE_READ			= 1 << 1,

// pipe can now be written to
    PIPE_WAKE_WRITE			= 1 << 2,

// unlock this pipe's DMA buffer
    PIPE_WAKE_UNLOCK_DMA		= 1 << 3,

// unlock DMA buffer of the pipe shared to this pipe
    PIPE_WAKE_UNLOCK_DMA_SHARED	= 1 << 4,
}

// Possible pipe closing reasons
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeCloseReason {
// guest sent a close command
    PIPE_CLOSE_GRACEFUL		= 0,

// guest rebooted, we're closing the pipes
    PIPE_CLOSE_REBOOT		= 1,

// close old pipes on snapshot load
    PIPE_CLOSE_LOAD_SNAPSHOT	= 2,

// some unrecoverable error on the pipe
    PIPE_CLOSE_ERROR		= 3,
}

// Bit flags for the 'flags' field
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeFlagsBits {
    BIT_CLOSED_ON_HOST = 0,  /* pipe closed by host */
    BIT_WAKE_ON_WRITE  = 1,  /* want to be woken on writes */
    BIT_WAKE_ON_READ   = 2,  /* want to be woken on reads */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeRegs {
    PIPE_REG_CMD = 0,

    PIPE_REG_SIGNAL_BUFFER_HIGH = 4,
    PIPE_REG_SIGNAL_BUFFER = 8,
    PIPE_REG_SIGNAL_BUFFER_COUNT = 12,

    PIPE_REG_OPEN_BUFFER_HIGH = 20,
    PIPE_REG_OPEN_BUFFER = 24,

    PIPE_REG_VERSION = 36,

    PIPE_REG_GET_SIGNALLED = 48,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PipeCmdCode {
// to be used by the pipe device itself
    PIPE_CMD_OPEN		= 1,

    PIPE_CMD_CLOSE,
    PIPE_CMD_POLL,
    PIPE_CMD_WRITE,
    PIPE_CMD_WAKE_ON_WRITE,
    PIPE_CMD_READ,
    PIPE_CMD_WAKE_ON_READ,

//
// TODO(zyy): implement a deferred read/write execution to allow
// parallel processing of pipe operations on the host.
//
    PIPE_CMD_WAKE_ON_DONE_IO,
}
