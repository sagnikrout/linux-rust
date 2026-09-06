//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/io_sch.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_subchannel_dma_area {
    pub /: *mut *mut ccw1 sense_ccw; / static ccw for sense command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_subchannel_private {
    pub /: *mut *mut orb orb; / operation request block,
    pub /: *mut *mut *mut ccw_device cdev;/ pointer to the child ccw device,
    pub /: *mut *mut unsigned int suspend:1; / allow suspend,
    pub /: *mut *mut unsigned int prefetch:1;/ deny prefetch,
    pub /: *mut *mut unsigned int inter:1; / suppress intermediate interrupts,
    pub options: } __packed,
    pub dma_area: *mut io_subchannel_dma_area,
    pub dma_area_dma: dma_addr_t,
    pub __aligned(8): },

    pub to_io_private(sch): *mut *mut io_subchannel_private priv =,
    pub NULL: return priv ? priv->cdev :,
    pub to_io_private(sch): *mut *mut io_subchannel_private priv =,
    pub cdev: priv->cdev =,
pub const MAX_CIWS: c_int = 8;
//
// Possible status values for a CCW request's I/O.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum io_status {
    IO_DONE,
    IO_RUNNING,
    IO_STATUS_ERROR,
    IO_PATH_ERROR,
    IO_REJECTED,
    IO_KILLED
}

//
// ccw_request - Internal CCW request.
// @cp: channel program to start
// @timeout: maximum allowable time in jiffies between start I/O and interrupt
// @maxretries: number of retries per I/O operation and path
// @lpm: mask of paths to use
// @check: optional callback that determines if results are final
// @filter: optional callback to adjust request status based on IRB data
// @callback: final callback
// @data: user-defined pointer passed to all callbacks
// @singlepath: if set, use only one path from @lpm per start I/O
// @cancel: non-zero if request was cancelled
// @done: non-zero if request was finished
// @mask: current path mask
// @retries: current number of retries
// @drc: delayed return code
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_request {
    pub cp: *mut ccw1,
    pub timeout: c_ulong,
    pub maxretries: u16,
    pub lpm: u8,
    pub ): *mut *mut *mut int (check)(struct ccw_device , void,
    pub io_status): enum,
    pub int): *mut *mut *mut *mut void (callback)(struct ccw_device , void ,,
    pub data: *mut c_void,
    pub singlepath:1: c_uint,
// These fields are used internally.
    pub cancel:1: c_uint,
    pub done:1: c_uint,
    pub mask: u16,
    pub retries: u16,
    pub drc: c_int,
    pub __attribute__((packed)): },
//
// sense-id response buffer layout
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct senseid {
// common part
    pub /: *mut *mut u8 reserved; / always 0x'FF',
    pub /: *mut *mut u16 cu_type; / control unit type,
    pub /: *mut *mut u8 cu_model; / control unit model,
    pub /: *mut *mut u16 dev_type; / device type,
    pub /: *mut *mut u8 dev_model; / device model,
    pub /: *mut *mut u8 unused; / padding byte,
// extended part
    pub /: *mut *mut ciw ciw[MAX_CIWS]; / variable # of CIWs,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cdev_todo {
    CDEV_TODO_NOTHING,
    CDEV_TODO_ENABLE_CMF,
    CDEV_TODO_REBIND,
    CDEV_TODO_REGISTER,
    CDEV_TODO_UNREG,
    CDEV_TODO_UNREG_EVAL,
}

pub const FAKE_CMD_IRB: c_int = 1;
pub const FAKE_TM_IRB: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_device_dma_area {
    pub /: *mut *mut senseid senseid; / SenseID info,
    pub /: *mut *mut ccw1 iccws[2]; / ccws for SNID/SID/SPGID commands,
    pub /: *mut *mut irb irb; / device status,
    pub chpid*/: *mut *mut pgid pgid[8]; / path group IDs per,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_device_private {
    pub cdev: *mut ccw_device,
    pub sch: *mut subchannel,
    pub /: *mut *mut int state; / device state,
    pub onoff: core::sync::atomic::AtomicI32,
    pub /: *mut *mut ccw_dev_id dev_id; / device id,
    pub /: *mut *mut ccw_request req; / internal I/O request,
    pub iretry: c_int,
    pub /: *mut *mut u8 pgid_valid_mask; / mask of valid PGIDs,
    pub /: *mut *mut u8 pgid_todo_mask; / mask of PGIDs to be adjusted,
    pub /: *mut *mut u8 pgid_reset_mask; / mask of PGIDs which were reset,
    pub was: *mut *mut u8 path_noirq_mask; / mask of paths for which no irq,
    pub found: *mut *mut u8 path_notoper_mask; / mask of paths which were,
    pub /: *mut *mut u8 path_gone_mask; / mask of paths, that became unavailable,
    pub /: *mut *mut u8 path_new_mask; / mask of paths, that became available,
    pub be: *mut *mut u8 path_broken_mask; / mask of paths, which were found to,
    pub /: *mut *mut unsigned int fast:1; / post with "channel end",
    pub /: *mut *mut unsigned int repall:1; / report every interrupt status,
    pub /: *mut *mut unsigned int pgroup:1; / do path grouping,
    pub /: *mut *mut unsigned int force:1; / allow forced online,
    pub /: *mut *mut unsigned int mpath:1; / do multipathing,
// C attribute field omitted
    pub /: *mut *mut unsigned int esid:1; / Ext. SenseID supported by HW,
    pub /: *mut *mut unsigned int dosense:1; / delayed SENSE required,
    pub /: *mut *mut unsigned int doverify:1; / delayed path verification,
    pub /: *mut *mut unsigned int donotify:1; / call notify function,
    pub /: *mut *mut unsigned int recog_done:1; / dev. recog. complete,
    pub /: *mut *mut unsigned int fake_irb:2; / deliver faked irb,
    pub /: *mut *mut unsigned int pgroup:1; / pathgroup is set up,
    pub /: *mut *mut unsigned int mpath:1; / multipathing is set up,
    pub /: *mut *mut unsigned int pgid_unknown:1;/ unknown pgid state,
    pub /: *mut *mut unsigned int initialized:1; / set if initial reference held,
// C attribute field omitted
    pub /: *mut *mut unsigned long intparm; / user interruption parameter,
    pub qdio_data: *mut qdio_irq,
    pub async_kill_io_rc: c_int,
    pub todo_work: work_struct,
    pub todo: cdev_todo,
    pub wait_q: wait_queue_head_t,
    pub timer: timer_list,
    pub /: *mut *mut *mut void cmb; / measurement information,
    pub /: *mut *mut list_head cmb_list; / list of measured devices,
    pub /: *mut *mut u64 cmb_start_time; / clock value of cmb reset,
    pub /: *mut *mut *mut void cmb_wait; / deferred cmb enable/disable,
    pub dma_pool: *mut gen_pool,
    pub dma_area: *mut ccw_device_dma_area,
    pub int_class: interruption_class,
}
