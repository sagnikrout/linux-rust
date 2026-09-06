//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/bestcomm/bestcomm.h
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
// Public header for the MPC52xx processor BestComm driver
//
// Copyright (C) 2006      Sylvain Munaut <tnt@246tNt.com>
// Copyright (C) 2005      Varma Electronics Oy,
// ( by Andrey Volkov <avolkov@varma-el.com> )
// Copyright (C) 2003-2004 MontaVista, Software, Inc.
// ( by Dale Farnsworth <dfarnsworth@mvista.com> )
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// struct bcom_bd - Structure describing a generic BestComm buffer descriptor
// @status: The current status of this buffer. Exact meaning depends on the
// task type
// @data: An array of u32 extra data.  Size of array is task dependent.
//
// Note: Don't dereference a bcom_bd pointer as an array.  The size of the
// bcom_bd is variable.  Use bcom_get_bd() instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_bd {
    pub status: u32,
    pub /: *mut *mut u32 data[]; / variable payload size,
}

// ========================================================================
// Generic task management
// ========================================================================
//
// struct bcom_task - Structure describing a loaded BestComm task
//
// This structure is never built by the driver it self. It's built and
// filled the intermediate layer of the BestComm API, the task dependent
// support code.
//
// Most likely you don't need to poke around inside this structure. The
// fields are exposed in the header just for the sake of inline functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_task {
    pub tasknum: c_uint,
    pub flags: c_uint,
    pub irq: c_int,
    pub bd: *mut bcom_bd,
    pub bd_pa: phys_addr_t,
    pub cookie: *mut c_void,
    pub index: c_ushort,
    pub outdex: c_ushort,
    pub num_bd: c_uint,
    pub bd_size: c_uint,
    pub priv: *mut *mut c_void,
}

pub const BCOM_FLAGS_NONE: c_uint = 0x00000000ul;

//
// bcom_enable - Enable a BestComm task
// @tsk: The BestComm task structure
//
// This function makes sure the given task is enabled and can be run
// by the BestComm engine as needed
//
extern "C" {
    pub fn bcom_enable(tsk: *mut bcom_task);
}
//
// bcom_disable - Disable a BestComm task
// @tsk: The BestComm task structure
//
// This function disable a given task, making sure it's not executed
// by the BestComm engine.
//
extern "C" {
    pub fn bcom_disable(tsk: *mut bcom_task);
}
//
// bcom_get_task_irq - Returns the irq number of a BestComm task
// @tsk: The BestComm task structure
//
// ========================================================================
// BD based tasks helpers
// ========================================================================
pub const BCOM_BD_READY: c_uint = 0x40000000ul;
// _bcom_next_index - Get next input index.
// @tsk: pointer to task structure
//
// Support function; Device drivers should not call this
//
// _bcom_next_outdex - Get next output index.
// @tsk: pointer to task structure
//
// Support function; Device drivers should not call this
//
// bcom_queue_empty - Checks if a BestComm task BD queue is empty
// @tsk: The BestComm task structure
//
// bcom_queue_full - Checks if a BestComm task BD queue is full
// @tsk: The BestComm task structure
//
// bcom_get_bd - Get a BD from the queue
// @tsk: The BestComm task structure
// index: Index of the BD to fetch
//
// bcom_get_bd(struct bcom_task *tsk, unsigned int index)
// A cast to (void*) so the address can be incremented by the
// real size instead of by sizeof(struct bcom_bd)
//
// bcom_buffer_done - Checks if a BestComm
// @tsk: The BestComm task structure
//
// bcom_prepare_next_buffer - clear status of next available buffer.
// @tsk: The BestComm task structure
//
// Returns pointer to next buffer descriptor
//
// p_status = bd->status;
// p_bd = bd;
