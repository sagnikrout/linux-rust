//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/mediatek/mdp3/mtk-mdp3-core.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Ping-Hsun Wu <ping-hsun.wu@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_infra_id {
//
// Due to the sequential nature of function "mdp_mm_subsys_deploy",
// adding new enum. necessitates careful consideration.
//
    MDP_INFRA_MMSYS,
    MDP_INFRA_MMSYS2,
    MDP_INFRA_MUTEX,
    MDP_INFRA_MUTEX2,
    MDP_INFRA_SCP,
    MDP_INFRA_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_mm_subsys_id {
    MDP_MM_SUBSYS_0,
    MDP_MM_SUBSYS_1,
    MDP_MM_SUBSYS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_buffer_usage {
    MDP_BUFFER_USAGE_HW_READ,
    MDP_BUFFER_USAGE_MDP,
    MDP_BUFFER_USAGE_MDP2,
    MDP_BUFFER_USAGE_ISP,
    MDP_BUFFER_USAGE_WPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_platform_config {
    pub rdma_support_10bit: bool,
    pub rdma_rsz1_sram_sharing: bool,
    pub rdma_upsample_repeat_only: bool,
    pub rdma_esl_setting: bool,
    pub rdma_event_num: u32,
    pub rsz_disable_dcm_small_sample: bool,
    pub rsz_etc_control: bool,
    pub wrot_filter_constraint: bool,
    pub wrot_support_10bit: bool,
    pub wrot_event_num: u32,
    pub tdshp_hist_num: u32,
    pub tdshp_constrain: bool,
    pub tdshp_contour: bool,
}

// indicate which mutex is used by each pipepline
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mdp_pipe_id {
    MDP_PIPE_WPEI,
    MDP_PIPE_WPEI2,
    MDP_PIPE_IMGI,
    MDP_PIPE_RDMA0,
    MDP_PIPE_RDMA1,
    MDP_PIPE_RDMA2,
    MDP_PIPE_RDMA3,
    MDP_PIPE_SPLIT,
    MDP_PIPE_SPLIT2,
    MDP_PIPE_VPP0_SOUT,
    MDP_PIPE_VPP1_SOUT,
    MDP_PIPE_MAX
}

// MDP parallel pipe control

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mdp_driver_data {
    pub mdp_plat_id: c_int,
    pub mdp_con_res: resource_size_t,
    pub mdp_probe_infra: *const of_device_id,
    pub mdp_cfg: *const mdp_platform_config,
    pub mdp_mutex_table_idx: *const u32,
    pub comp_data: *const mdp_comp_data,
    pub comp_data_len: c_uint,
    pub mdp_sub_comp_dt_ids: *const of_device_id,
    pub format: *const mdp_format,
    pub format_len: c_uint,
    pub def_limit: *const mdp_limit,
    pub pipe_info: *const mdp_pipe_info,
    pub pipe_info_len: c_uint,
    pub pp_criteria: *const v4l2_rect,
    pub pp_used: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_mm_subsys {
    pub mmsys: *mut device,
    pub mutex: *mut device,
    pub mdp_mutex: [*mut mtk_mutex; MDP_PIPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_dev {
    pub pdev: *mut platform_device,
    pub mm_subsys: [mdp_mm_subsys; MDP_MM_SUBSYS_MAX],
    pub comp: [*mut mdp_comp; MDP_MAX_COMP_COUNT],
    pub mdp_data: *const mtk_mdp_driver_data,
    pub job_wq: *mut workqueue_struct,
    pub clock_wq: *mut workqueue_struct,
    pub vpu: mdp_vpu_dev,
    pub scp: *mut mtk_scp,
    pub rproc_handle: *mut rproc,
// synchronization protect for accessing vpu working buffer info
    pub vpu_lock: mutex,
    pub vpu_count: i32,
    pub id_count: u32,
    pub mdp_ida: ida,
    pub cmdq_clt: [*mut cmdq_client; MDP_PP_MAX],
    pub cmdq_shift_pa: [u8; MDP_PP_MAX],
    pub callback_wq: wait_queue_head_t,
    pub v4l2_dev: v4l2_device,
    pub m2m_vdev: *mut video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
// synchronization protect for m2m device operation
    pub m2m_lock: mutex,
    pub suspended: core::sync::atomic::AtomicI32,
    pub job_count: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp_pipe_info {
    pub pipe_id: mdp_pipe_id,
    pub sub_id: mdp_mm_subsys_id,
    pub mutex_id: u32,
}

extern "C" {
    pub fn mdp_vpu_get_locked(mdp: *mut mdp_dev) -> c_int;
}
extern "C" {
    pub fn mdp_vpu_put_locked(mdp: *mut mdp_dev);
}
extern "C" {
    pub fn mdp_vpu_register(mdp: *mut mdp_dev) -> c_int;
}
extern "C" {
    pub fn mdp_vpu_unregister(mdp: *mut mdp_dev);
}
extern "C" {
    pub fn mdp_video_device_release(vdev: *mut video_device);
}
