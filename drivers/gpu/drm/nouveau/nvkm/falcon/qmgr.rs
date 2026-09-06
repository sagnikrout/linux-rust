//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/falcon/qmgr.h
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


// SPDX-License-Identifier: MIT

pub const QUEUE_ALIGNMENT: c_int = 4;
// max size of the messages we can receive
pub const MSG_BUF_SIZE: c_int = 128;
//
// struct nvkm_falcon_qmgr_seq - keep track of ongoing commands
//
// Every time a command is sent, a sequence is assigned to it so the
// corresponding message can be matched. Upon receiving the message, a callback
// can be called and/or a completion signaled.
//
// @id:		sequence ID
// @state:	current state
// @callback:	callback to call upon receiving matching message
// @completion:	completion to signal after callback is called
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_qmgr_seq {
    pub id: u16,
    pub state: },
    pub async: bool,
    pub callback: nvkm_falcon_qmgr_callback,
    pub priv: *mut c_void,
    pub done: completion,
    pub result: c_int,
}

//
// We can have an arbitrary number of sequences, but realistically we will
// probably not use that much simultaneously.
//
pub const NVKM_FALCON_QMGR_SEQ_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_qmgr {
    pub falcon: *mut nvkm_falcon,
    pub mutex: mutex,
    pub id: [nvkm_falcon_qmgr_seq; NVKM_FALCON_QMGR_SEQ_NUM],
    pub tbl: [c_ulong; BITS_TO_LONGS(NVKM_FALCON_QMGR_SEQ_NUM)],
    pub seq: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_cmdq {
    pub qmgr: *mut nvkm_falcon_qmgr,
    pub name: *const c_char,
    pub mutex: mutex,
    pub ready: completion,
    pub head_reg: u32,
    pub tail_reg: u32,
    pub offset: u32,
    pub size: u32,
    pub position: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_msgq {
    pub qmgr: *mut nvkm_falcon_qmgr,
    pub name: *const c_char,
    pub lock: spinlock_t,
    pub head_reg: u32,
    pub tail_reg: u32,
    pub offset: u32,
    pub position: u32,
}

