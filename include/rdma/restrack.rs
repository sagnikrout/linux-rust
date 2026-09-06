//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/restrack.h
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
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved.
//

// Mark entry as containing driver specific details, it is used to provide QP subtype for now

//
// enum rdma_restrack_type - HW objects to track
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_restrack_type {
//
// @RDMA_RESTRACK_PD: Protection domain (PD)
//
    RDMA_RESTRACK_PD,
//
// @RDMA_RESTRACK_CQ: Completion queue (CQ)
//
    RDMA_RESTRACK_CQ,
//
// @RDMA_RESTRACK_QP: Queue pair (QP)
//
    RDMA_RESTRACK_QP,
//
// @RDMA_RESTRACK_CM_ID: Connection Manager ID (CM_ID)
//
    RDMA_RESTRACK_CM_ID,
//
// @RDMA_RESTRACK_MR: Memory Region (MR)
//
    RDMA_RESTRACK_MR,
//
// @RDMA_RESTRACK_CTX: Verbs contexts (CTX)
//
    RDMA_RESTRACK_CTX,
//
// @RDMA_RESTRACK_COUNTER: Statistic Counter
//
    RDMA_RESTRACK_COUNTER,
//
// @RDMA_RESTRACK_SRQ: Shared receive queue (SRQ)
//
    RDMA_RESTRACK_SRQ,
//
// @RDMA_RESTRACK_DMAH: DMA handle
//
    RDMA_RESTRACK_DMAH,
//
// @RDMA_RESTRACK_COMP_CNTR: Completion Counter
//
    RDMA_RESTRACK_COMP_CNTR,
//
// @RDMA_RESTRACK_MAX: Last entry, used for array dclarations
//
    RDMA_RESTRACK_MAX
}

//
// struct rdma_restrack_entry - metadata per-entry
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_restrack_entry {
//
// @valid: validity indicator
//
// The entries are filled during rdma_restrack_add,
// can be attempted to be free during rdma_restrack_del.
//
// As an example for that, see mlx5 QPs with type MLX5_IB_QPT_HW_GSI
//
    pub valid: bool,
//
// @no_track: don't add this entry to restrack DB
//
// This field is used to mark an entry that doesn't need to be added to
// internal restrack DB and presented later to the users at the nldev
// query stage.
//
    pub 1: u8 no_track :,
//
// @kref: Protect destroy of the resource
//
    pub kref: kref,
//
// @comp: Signal that all consumers of resource are completed their work
//
    pub comp: completion,
//
// @task: owner of resource tracking entity
//
// There are two types of entities: created by user and created
// by kernel.
//
// This is relevant for the entities created by users.
// For the entities created by kernel, this pointer will be NULL.
//
    pub task: *mut task_struct,
//
// @kern_name: name of owner for the kernel created entities.
//
    pub kern_name: *const c_char,
//
// @type: various objects in restrack database
//
    pub type: rdma_restrack_type,
//
// @user: user resource
//
    pub user: bool,
//
// @id: ID to expose to users
//
    pub id: u32,
}

//
// rdma_is_kernel_res() - check the owner of resource
// @res:  resource entry
//
// rdma_restrack_get() - grab to protect resource from release
// @res:  resource entry
//
extern "C" {
    pub fn rdma_restrack_get(res: *mut rdma_restrack_entry) -> int __must_check;
}
//
// rdma_restrack_put() - release resource
// @res:  resource entry
//
extern "C" {
    pub fn rdma_restrack_put(res: *mut rdma_restrack_entry) -> c_int;
}
//
// Helper functions for rdma drivers when filling out
// nldev driver attributes.
//
extern "C" {
    pub fn rdma_nl_put_driver_u32(msg: *mut sk_buff, name: *const c_char, value: u32) -> c_int;
}
extern "C" {
    pub fn rdma_nl_put_driver_u64(msg: *mut sk_buff, name: *const c_char, value: u64) -> c_int;
}
//
// rdma_restrack_no_track() - don't add resource to the DB
// @res: resource entry
//
// Every user of this API should be cross examined.
// Probably you don't need to use this function.
//
