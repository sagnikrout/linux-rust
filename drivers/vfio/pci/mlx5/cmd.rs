//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/mlx5/cmd.h
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
// Copyright (c) 2021-2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_vf_migf_state {
    MLX5_MIGF_STATE_ERROR = 1,
    MLX5_MIGF_STATE_PRE_COPY_ERROR,
    MLX5_MIGF_STATE_PRE_COPY,
    MLX5_MIGF_STATE_SAVE_STOP_COPY_CHUNK,
    MLX5_MIGF_STATE_COMPLETE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_vf_load_state {
    MLX5_VF_LOAD_STATE_READ_HEADER,
    MLX5_VF_LOAD_STATE_PREP_HEADER_DATA,
    MLX5_VF_LOAD_STATE_READ_HEADER_DATA,
    MLX5_VF_LOAD_STATE_PREP_IMAGE,
    MLX5_VF_LOAD_STATE_READ_IMAGE,
    MLX5_VF_LOAD_STATE_LOAD_IMAGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vf_migration_tag_stop_copy_data {
    pub stop_copy_size: __le64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_vf_migf_header_flags {
    MLX5_MIGF_HEADER_FLAGS_TAG_MANDATORY = 0,
    MLX5_MIGF_HEADER_FLAGS_TAG_OPTIONAL = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_vf_migf_header_tag {
    MLX5_MIGF_HEADER_TAG_FW_DATA = 0,
    MLX5_MIGF_HEADER_TAG_STOP_COPY_SIZE = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vf_migration_header {
    pub record_size: __le64,
// For future use in case we may need to change the kernel protocol
    pub /: *mut *mut __le32 flags; / Use mlx5_vf_migf_header_flags,
    pub /: *mut *mut __le32 tag; / Use mlx5_vf_migf_header_tag,
    pub /: *mut *mut __u8 data[]; / Its size is given in the record_size,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_data_buffer {
    pub page_list: *mut page,
    pub state: dma_iova_state,
    pub start_pos: loff_t,
    pub length: u64,
    pub npages: u32,
    pub mkey: u32,
    pub mkey_in: *mut u32,
    pub dma_dir: dma_data_direction,
    pub stop_copy_chunk_num: u8,
    pub pre_copy_init_bytes_chunk: bool,
    pub buf_elm: list_head,
    pub migf: *mut mlx5_vf_migration_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5vf_async_data {
    pub cb_work: mlx5_async_work,
    pub work: work_struct,
    pub buf: *mut mlx5_vhca_data_buffer,
    pub header_buf: *mut mlx5_vhca_data_buffer,
    pub status: c_int,
    pub stop_copy_chunk:1: u8,
    pub out: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5vf_save_work_data {
    pub migf: *mut mlx5_vf_migration_file,
    pub next_required_umem_size: usize,
    pub work: work_struct,
    pub chunk_num: u8,
}

pub const MAX_NUM_CHUNKS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vf_migration_file {
    pub filp: *mut file,
    pub lock: mutex,
    pub state: mlx5_vf_migf_state,
    pub load_state: mlx5_vf_load_state,
    pub pdn: u32,
    pub max_pos: loff_t,
    pub record_size: u64,
    pub record_tag: u32,
    pub stop_copy_prep_size: u64,
    pub pre_copy_initial_bytes: u64,
    pub pre_copy_initial_bytes_start: u64,
    pub next_required_umem_size: usize,
    pub num_ready_chunks: u8,
// Upon chunk mode preserve another set of buffers for stop_copy phase
    pub buf: [*mut mlx5_vhca_data_buffer; MAX_NUM_CHUNKS],
    pub buf_header: [*mut mlx5_vhca_data_buffer; MAX_NUM_CHUNKS],
    pub save_data: [mlx5vf_save_work_data; MAX_NUM_CHUNKS],
    pub list_lock: spinlock_t,
    pub buf_list: list_head,
    pub avail_list: list_head,
    pub mvdev: *mut mlx5vf_pci_core_device,
    pub poll_wait: wait_queue_head_t,
    pub save_comp: completion,
    pub async_ctx: mlx5_async_ctx,
    pub async_data: mlx5vf_async_data,
    pub inflight_save:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_cq_buf {
    pub fbc: mlx5_frag_buf_ctrl,
    pub frag_buf: mlx5_frag_buf,
    pub cqe_size: c_int,
    pub nent: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_cq {
    pub buf: mlx5_vhca_cq_buf,
    pub db: mlx5_db,
    pub mcq: mlx5_core_cq,
    pub ncqe: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_recv_buf {
    pub npages: u32,
    pub page_list: *mut page,
    pub state: dma_iova_state,
    pub next_rq_offset: u32,
    pub mkey_in: *mut u32,
    pub mkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_qp {
    pub buf: mlx5_frag_buf,
    pub db: mlx5_db,
    pub recv_buf: mlx5_vhca_recv_buf,
    pub tracked_page_size: u32,
    pub max_msg_size: u32,
    pub qpn: u32,
    pub pc: c_uint,
    pub cc: c_uint,
    pub wqe_cnt: c_uint,
    pub db: *mut __be32,
    pub fbc: mlx5_frag_buf_ctrl,
    pub rq: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_vhca_page_tracker {
    pub id: u32,
    pub pdn: u32,
// Flags modified at runtime - dedicated storage unit
    pub is_err: u8,
    pub object_changed: u8,
    pub status: c_int,
    pub uar: *mut mlx5_uars_page,
    pub cq: mlx5_vhca_cq,
    pub host_qp: *mut mlx5_vhca_qp,
    pub fw_qp: *mut mlx5_vhca_qp,
    pub nb: mlx5_nb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5vf_pci_core_device {
    pub core_device: vfio_pci_core_device,
    pub vf_id: c_int,
    pub vhca_id: u16,
// Flags only modified on setup/release - bitfield ok
    pub migrate_cap:1: u8,
    pub chunk_mode:1: u8,
    pub mig_state_cap:1: u8,
// Flags modified at runtime - dedicated storage unit
    pub mdev_detach: u8,
    pub log_active: u8,
    pub deferred_reset: u8,
    pub tracker_comp: completion,
// protect migration state
    pub state_mutex: mutex,
    pub mig_state: vfio_device_mig_state,
// protect the reset_done flow
    pub reset_lock: spinlock_t,
    pub resuming_migf: *mut mlx5_vf_migration_file,
    pub saving_migf: *mut mlx5_vf_migration_file,
    pub tracker: mlx5_vhca_page_tracker,
    pub cb_wq: *mut workqueue_struct,
    pub nb: notifier_block,
    pub mdev: *mut mlx5_core_dev,
}

extern "C" {
    pub fn mlx5vf_cmd_suspend_vhca(mvdev: *mut mlx5vf_pci_core_device, op_mod: u16) -> c_int;
}
extern "C" {
    pub fn mlx5vf_cmd_resume_vhca(mvdev: *mut mlx5vf_pci_core_device, op_mod: u16) -> c_int;
}
extern "C" {
    pub fn mlx5vf_cmd_remove_migratable(mvdev: *mut mlx5vf_pci_core_device);
}
extern "C" {
    pub fn mlx5vf_cmd_close_migratable(mvdev: *mut mlx5vf_pci_core_device);
}
extern "C" {
    pub fn mlx5vf_cmd_alloc_pd(migf: *mut mlx5_vf_migration_file) -> c_int;
}
extern "C" {
    pub fn mlx5vf_cmd_dealloc_pd(migf: *mut mlx5_vf_migration_file);
}
extern "C" {
    pub fn mlx5fv_cmd_clean_migf_resources(migf: *mut mlx5_vf_migration_file);
}
extern "C" {
    pub fn mlx5vf_free_data_buffer(buf: *mut mlx5_vhca_data_buffer);
}
extern "C" {
    pub fn mlx5vf_put_data_buffer(buf: *mut mlx5_vhca_data_buffer);
}
extern "C" {
    pub fn mlx5vf_state_mutex_unlock(mvdev: *mut mlx5vf_pci_core_device);
}
extern "C" {
    pub fn mlx5vf_mig_file_cleanup_cb(_work: *mut work_struct);
}
extern "C" {
    pub fn mlx5vf_stop_page_tracker(vdev: *mut vfio_device) -> c_int;
}
