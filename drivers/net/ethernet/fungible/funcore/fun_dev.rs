//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/fungible/funcore/fun_dev.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)

// doorbell fields
// Callback for asynchronous admin commands.
// Invoked on reception of command response.
//
// Callback for events/notifications received by an admin queue.
extern "C" {
    pub fn void(fdev: *mut *mut fun_admin_event_cb)(struct fun_dev, cqe: *mut c_void) -> typedef;
}
// Callback for pending work handled by the service task.
extern "C" {
    pub fn void(fd: *mut *mut fun_serv_cb)(struct fun_dev) -> typedef;
}
// service task flags
// Driver state associated with a PCI function.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dev {
    pub dev: *mut device,
    pub /: *mut *mut *mut void __iomem bar; / start of BAR0 mapping,
    pub /: *mut *mut *mut u32 __iomem dbs; / start of doorbells in BAR0 mapping,
// admin queue
    pub admin_q: *mut fun_queue,
    pub admin_sbq: sbitmap_queue,
    pub cmd_ctx: *mut fun_cmd_ctx,
    pub adminq_cb: fun_admin_event_cb,
    pub /: *mut *mut bool suppress_cmds; / if set don't write commands to SQ,
// address increment between consecutive doorbells, in 4B units
    pub db_stride: c_uint,
// SW versions of device registers
    pub /: *mut *mut u32 cc_reg; / CC register,
    pub /: *mut *mut u64 cap_reg; / CAPability register,
    pub /: *mut *mut unsigned int q_depth; / max queue depth supported by device,
    pub /: *mut *mut unsigned int max_qid; / = #queues - 1, separately for SQs and CQs,
    pub /: *mut *mut unsigned int kern_end_qid; / last qid in the kernel range + 1,
    pub fw_handle: c_uint,
// IRQ manager
    pub num_irqs: c_uint,
    pub irqs_avail: c_uint,
    pub irqmgr_lock: spinlock_t,
    pub irq_map: *mut c_ulong,
// The service task handles work that needs a process context
    pub service_task: work_struct,
    pub service_flags: c_ulong,
    pub serv_cb: fun_serv_cb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fun_dev_params {
    pub /: *mut *mut u8 cqe_size_log2; / admin q CQE size,
    pub /: *mut *mut u8 sqe_size_log2; / admin q SQE size,
// admin q depths
    pub cq_depth: u16,
    pub sq_depth: u16,
    pub rq_depth: u16,
    pub /: *mut *mut u16 min_msix; / min vectors needed by requesting driver,
    pub event_cb: fun_admin_event_cb,
    pub serv_cb: fun_serv_cb,
}

// Return the BAR address of a doorbell.
// Return the BAR address of an SQ doorbell. SQ and CQ DBs alternate,
// SQs have even DB indices.
//
extern "C" {
    pub fn fun_db_addr(_arg: fdev, 2: *mut *mut sqid) -> return;
}
extern "C" {
    pub fn fun_db_addr(_arg: fdev, 1: *mut *mut cqid  2 +) -> return;
}
extern "C" {
    pub fn fun_get_res_count(fdev: *mut fun_dev, res: fun_admin_op) -> c_int;
}
extern "C" {
    pub fn fun_dev_disable(fdev: *mut fun_dev);
}
extern "C" {
    pub fn fun_serv_stop(fd: *mut fun_dev);
}
extern "C" {
    pub fn fun_serv_restart(fd: *mut fun_dev);
}
extern "C" {
    pub fn fun_serv_sched(fd: *mut fun_dev);
}
