//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu.h
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
// Copyright 2020-2021 NXP
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imx_plat_type {
    IMX8QXP = 0,
    IMX8QM  = 1,
    IMX8DM,
    IMX8DX,
    PLAT_TYPE_RESERVED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_core_type {
    VPU_CORE_TYPE_ENC = 0,
    VPU_CORE_TYPE_DEC = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_resources {
    pub plat_type: imx_plat_type,
    pub mreg_base: u32,
    pub vpu): *mut *mut int (setup)(struct vpu_dev,
    pub vpu): *mut *mut int (setup_encoder)(struct vpu_dev,
    pub vpu): *mut *mut int (setup_decoder)(struct vpu_dev,
    pub vpu): *mut *mut int (reset)(struct vpu_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_buffer {
    pub virt: *mut c_void,
    pub phys: dma_addr_t,
    pub length: u32,
    pub bytesused: u32,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_func {
    pub vfd: *mut video_device,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub type: vpu_core_type,
    pub function: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_dev {
    pub base: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub /: *mut *mut mutex lock; / protect vpu device,
    pub res: *const vpu_resources,
    pub cores: list_head,
    pub v4l2_dev: v4l2_device,
    pub encoder: vpu_func,
    pub decoder: vpu_func,
    pub mdev: media_device,
    pub watchdog_work: delayed_work,
    pub vpu): *mut *mut void (get_vpu)(struct vpu_dev,
    pub vpu): *mut *mut void (put_vpu)(struct vpu_dev,
    pub vpu): *mut *mut void (get_enc)(struct vpu_dev,
    pub vpu): *mut *mut void (put_enc)(struct vpu_dev,
    pub vpu): *mut *mut void (get_dec)(struct vpu_dev,
    pub vpu): *mut *mut void (put_dec)(struct vpu_dev,
    pub ref_vpu: core::sync::atomic::AtomicI32,
    pub ref_enc: core::sync::atomic::AtomicI32,
    pub ref_dec: core::sync::atomic::AtomicI32,
    pub debugfs: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_format {
    pub pixfmt: u32,
    pub mem_planes: u32,
    pub comp_planes: u32,
    pub type: u32,
    pub flags: u32,
    pub width: u32,
    pub height: u32,
    pub sizeimage: [u32; VIDEO_MAX_PLANES],
    pub bytesperline: [u32; VIDEO_MAX_PLANES],
    pub field: u32,
    pub sibling: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_core_resources {
    pub type: vpu_core_type,
    pub fwname: *const c_char,
    pub stride: u32,
    pub max_width: u32,
    pub min_width: u32,
    pub step_width: u32,
    pub max_height: u32,
    pub min_height: u32,
    pub step_height: u32,
    pub rpc_size: u32,
    pub fwlog_size: u32,
    pub act_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_mbox {
    pub name: [c_char; 20],
    pub cl: mbox_client,
    pub ch: *mut mbox_chan,
    pub block: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_core_state {
    VPU_CORE_DEINIT = 0,
    VPU_CORE_ACTIVE,
    VPU_CORE_HANG
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_core {
    pub base: *mut void __iomem,
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub parent: *mut device,
    pub pd: *mut device,
    pub pd_link: *mut device_link,
    pub /: *mut *mut mutex lock; / protect vpu core,
    pub /: *mut *mut mutex cmd_lock; / Lock vpu command,
    pub list: list_head,
    pub type: vpu_core_type,
    pub id: c_int,
    pub res: *const vpu_core_resources,
    pub instance_mask: c_ulong,
    pub supported_instance_count: u32,
    pub hang_mask: c_ulong,
    pub request_count: u32,
    pub instances: list_head,
    pub state: vpu_core_state,
    pub fw_version: u32,
    pub fw: vpu_buffer,
    pub rpc: vpu_buffer,
    pub log: vpu_buffer,
    pub act: vpu_buffer,
    pub tx_type: vpu_mbox,
    pub tx_data: vpu_mbox,
    pub rx: vpu_mbox,
    pub ack_wq: wait_queue_head_t,
    pub cmp: completion,
    pub workqueue: *mut workqueue_struct,
    pub msg_work: work_struct,
    pub msg_delayed_work: delayed_work,
    pub msg_fifo: kfifo,
    pub msg_buffer: *mut c_void,
    pub vpu: *mut vpu_dev,
    pub iface: *mut c_void,
    pub debugfs: *mut dentry,
    pub debugfs_fwlog: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_codec_state {
    VPU_CODEC_STATE_DEINIT = 1,
    VPU_CODEC_STATE_CONFIGURED,
    VPU_CODEC_STATE_START,
    VPU_CODEC_STATE_STARTED,
    VPU_CODEC_STATE_ACTIVE,
    VPU_CODEC_STATE_SEEK,
    VPU_CODEC_STATE_STOP,
    VPU_CODEC_STATE_DRAIN,
    VPU_CODEC_STATE_DYAMIC_RESOLUTION_CHANGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_frame_info {
    pub type: u32,
    pub id: u32,
    pub sequence: u32,
    pub luma: u32,
    pub chroma_u: u32,
    pub chroma_v: u32,
    pub data_offset: u32,
    pub flags: u32,
    pub skipped: u32,
    pub timestamp: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_inst_ops {
    pub inst): *mut *mut int (ctrl_init)(struct vpu_inst,
    pub type): *mut *mut *mut int (start)(struct vpu_inst inst, u32,
    pub type): *mut *mut *mut int (stop)(struct vpu_inst inst, u32,
    pub inst): *mut *mut int (abort)(struct vpu_inst,
    pub type): *mut *mut *mut bool (check_ready)(struct vpu_inst inst, unsigned int,
    pub frame): *mut *mut *mut void (buf_done)(struct vpu_inst inst, struct vpu_frame_info,
    pub data): *mut *mut *mut void (event_notify)(struct vpu_inst inst, u32 event, void,
    pub inst): *mut *mut void (release)(struct vpu_inst,
    pub inst): *mut *mut void (cleanup)(struct vpu_inst,
    pub act_frame_num): u32,
    pub inst): *mut *mut void (input_done)(struct vpu_inst,
    pub inst): *mut *mut void (stop_done)(struct vpu_inst,
    pub vb): *mut *mut *mut int (process_output)(struct vpu_inst inst, struct vb2_buffer,
    pub vb): *mut *mut *mut int (process_capture)(struct vpu_inst inst, struct vb2_buffer,
    pub info): *mut *mut *mut int (get_one_frame)(struct vpu_inst inst, void,
    pub type): *mut *mut *mut void (on_queue_empty)(struct vpu_inst inst, u32,
    pub i): *mut *mut *mut *mut int (get_debug_info)(struct vpu_inst inst, char str, u32 size, u32,
    pub inst): *mut *mut void (wait_prepare)(struct vpu_inst,
    pub inst): *mut *mut void (wait_finish)(struct vpu_inst,
    pub vb): *mut *mut *mut void (attach_frame_store)(struct vpu_inst inst, struct vb2_buffer,
    pub inst): *mut *mut void (reset_frame_store)(struct vpu_inst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_inst {
    pub list: list_head,
    pub /: *mut *mut mutex lock; / v4l2 and videobuf2 lock,
    pub vpu: *mut vpu_dev,
    pub core: *mut vpu_core,
    pub dev: *mut device,
    pub id: c_int,
    pub fh: v4l2_fh,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub inst): *mut *mut int (release)(struct vpu_inst,
    pub state: vpu_codec_state,
    pub type: vpu_core_type,
    pub workqueue: *mut workqueue_struct,
    pub msg_work: work_struct,
    pub msg_fifo: kfifo,
    pub msg_buffer: [u8; VPU_MSG_BUFFER_SIZE],
    pub stream_buffer: vpu_buffer,
    pub use_stream_buffer: bool,
    pub act: vpu_buffer,
    pub cmd_q: list_head,
    pub pending: *mut c_void,
    pub cmd_seq: c_ulong,
    pub last_response_cmd: atomic_long_t,
    pub ops: *mut vpu_inst_ops,
    pub formats: *const vpu_format,
    pub out_format: vpu_format,
    pub cap_format: vpu_format,
    pub min_buffer_cap: u32,
    pub min_buffer_out: u32,
    pub total_input_count: u32,
    pub crop: v4l2_rect,
    pub colorspace: u32,
    pub ycbcr_enc: u8,
    pub quantization: u8,
    pub xfer_func: u8,
    pub sequence: u32,
    pub extra_size: u32,
    pub flows: [u32; 16],
    pub flow_idx: u32,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub debugfs: *mut dentry,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_vb2_buffer {
    pub m2m_buf: v4l2_m2m_buffer,
    pub luma: dma_addr_t,
    pub chroma_u: dma_addr_t,
    pub chroma_v: dma_addr_t,
    pub state: c_uint,
    pub average_qp: u32,
    pub fs_id: i32,
}

extern "C" {
    pub fn vpu_writel(vpu: *mut vpu_dev, reg: u32, val: u32);
}
extern "C" {
    pub fn vpu_readl(vpu: *mut vpu_dev, reg: u32) -> u32;
}
extern "C" {
    pub fn container_of(_arg: m2m_buf, vpu_vb2_buffer: struct, _arg: m2m_buf) -> return;
}
extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), vpu_inst: struct, _arg: fh) -> return;
}

extern "C" {
    pub fn vpu_add_func(vpu: *mut vpu_dev, func: *mut vpu_func) -> c_int;
}
extern "C" {
    pub fn vpu_remove_func(func: *mut vpu_func);
}
extern "C" {
    pub fn vpu_inst_put(inst: *mut vpu_inst);
}
extern "C" {
    pub fn vpu_release_core(core: *mut vpu_core);
}
extern "C" {
    pub fn vpu_inst_register(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_inst_unregister(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_inst_create_dbgfs_file(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_inst_remove_dbgfs_file(inst: *mut vpu_inst) -> c_int;
}
extern "C" {
    pub fn vpu_core_create_dbgfs_file(core: *mut vpu_core) -> c_int;
}
extern "C" {
    pub fn vpu_core_remove_dbgfs_file(core: *mut vpu_core) -> c_int;
}
extern "C" {
    pub fn vpu_inst_record_flow(inst: *mut vpu_inst, flow: u32);
}
extern "C" {
    pub fn vpu_core_driver_init() -> c_int;
}
extern "C" {
    pub fn vpu_core_driver_exit();
}

