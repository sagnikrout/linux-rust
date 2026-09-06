//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/gsp.h
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


pub const GSP_PAGE_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_mem {
    pub dev: *mut device,
    pub size: usize,
    pub data: *mut c_void,
    pub addr: dma_addr_t,
}

extern "C" {
    pub fn nvkm_gsp_mem_ctor(: *mut nvkm_gsp, size: usize, : *mut nvkm_gsp_mem) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_mem_dtor(: *mut nvkm_gsp_mem);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_radix3 {
    pub lvl0: nvkm_gsp_mem,
    pub lvl1: nvkm_gsp_mem,
    pub lvl2: sg_table,
}

extern "C" {
    pub fn nvkm_gsp_sg(: *mut nvkm_device, size: u64, : *mut sg_table) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_sg_free(: *mut nvkm_device, : *mut sg_table);
}
extern "C" {
    pub fn int(priv: *mut *mut nvkm_gsp_msg_ntfy_func)(void, fn: u32, repv: *mut c_void, repc: u32) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut nvkm_gsp_event_func)(struct nvkm_gsp_event, repv: *mut c_void, repc: u32) -> typedef;
}
//
// DOC: GSP message handling policy
//
// When sending a GSP RPC command, there can be multiple cases of handling
// the GSP RPC messages, which are the reply of GSP RPC commands, according
// to the requirement of the callers and the nature of the GSP RPC commands.
//
// NVKM_GSP_RPC_REPLY_NOWAIT - If specified, immediately return to the
// caller after the GSP RPC command is issued.
//
// NVKM_GSP_RPC_REPLY_NOSEQ - If specified, exactly like NOWAIT
// but don't emit RPC sequence number.
//
// NVKM_GSP_RPC_REPLY_RECV - If specified, wait and receive the entire GSP
// RPC message after the GSP RPC command is issued.
//
// NVKM_GSP_RPC_REPLY_POLL - If specified, wait for the specific reply and
// discard the reply before returning to the caller.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_gsp_rpc_reply_policy {
    NVKM_GSP_RPC_REPLY_NOWAIT = 0,
    NVKM_GSP_RPC_REPLY_NOSEQ,
    NVKM_GSP_RPC_REPLY_RECV,
    NVKM_GSP_RPC_REPLY_POLL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp {
    pub func: *const nvkm_gsp_func,
    pub subdev: nvkm_subdev,
    pub falcon: nvkm_falcon,
    pub load: *const firmware,
    pub unload: *const firmware,
    pub booter: },
    pub fmc: *const firmware,
    pub bl: *const firmware,
    pub rm: *const firmware,
    pub sb: nvkm_falcon_fw,
    pub falcon: },
    pub fws: },
    pub fw: nvkm_firmware,
    pub sig: nvkm_gsp_mem,
    pub radix3: nvkm_gsp_radix3,
    pub addr: u64,
    pub size: u64,
    pub vga_workspace: },
    pub addr: u64,
    pub size: u64,
    pub bios: },
    pub addr: u64,
    pub size: u64,
    pub heap: } frts, boot, elf,,
    pub addr: u64,
    pub size: u64,
    pub wpr2: },
    pub addr: u64,
    pub size: u64,
    pub heap: },
    pub addr: u64,
    pub size: u64,
    pub addr: u64,
    pub size: u64,
    pub region: [}; 16],
    pub region_nr: c_int,
    pub rsvd_size: u32,
    pub fb: },
    pub load: nvkm_falcon_fw,
    pub unload: nvkm_falcon_fw,
    pub booter: },
    pub fw: nvkm_gsp_mem,
    pub hash: *mut u8,
    pub pkey: *mut u8,
    pub sig: *mut u8,
    pub args: nvkm_gsp_mem,
    pub fmc: },
    pub fw: nvkm_gsp_mem,
    pub code_offset: u32,
    pub data_offset: u32,
    pub manifest_offset: u32,
    pub app_version: u32,
    pub boot: },
    pub libos: nvkm_gsp_mem,
    pub loginit: nvkm_gsp_mem,
    pub logintr: nvkm_gsp_mem,
    pub logrm: nvkm_gsp_mem,
    pub rmargs: nvkm_gsp_mem,
    pub wpr_meta: nvkm_gsp_mem,
    pub sgt: sg_table,
    pub radix3: nvkm_gsp_radix3,
    pub meta: nvkm_gsp_mem,
    pub fbsr: sg_table,
    pub sr: },
    pub mem: nvkm_gsp_mem,
    pub nr: c_int,
    pub size: u32,
    pub ptr: *mut u64,
    pub ptes: },
    pub size: u32,
    pub ptr: *mut c_void,
    pub msgq: } cmdq,,
    pub shm: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_cmdq {
    pub mutex: mutex,
    pub cnt: u32,
    pub seq: u32,
    pub wptr: *mut u32,
    pub rptr: *mut u32,
    pub cmdq: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_msgq {
    pub mutex: mutex,
    pub cnt: u32,
    pub wptr: *mut u32,
    pub rptr: *mut u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_msgq_ntfy {
    pub fn: u32,
    pub func: nvkm_gsp_msg_ntfy_func,
    pub priv: *mut c_void,
    pub ntfy: [}; 16],
    pub ntfy_nr: c_int,
    pub work: work_struct,
    pub msgq: },
    pub running: bool,
// Internal GSP-RM control handles.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_client {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_object {
    pub client: *mut nvkm_gsp_client,
    pub parent: *mut nvkm_gsp_object,
    pub handle: u32,
    pub object: },
    pub gsp: *mut nvkm_gsp,
    pub events: list_head,
    pub client: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_device {
    pub object: nvkm_gsp_object,
    pub subdevice: nvkm_gsp_object,
    pub device: },
    pub internal: },
    pub type: nvkm_subdev_type,
    pub inst: c_int,
    pub stall: u32,
    pub nonstall: u32,
    pub intr: [}; 32],
    pub intr_nr: c_int,
    pub rm_bar1_pdb: u64,
    pub rm_bar2_pdb: u64,
    pub bar: },
    pub gpcs: u8,
    pub tpcs: u8,
    pub gr: },
    pub rm: *mut nvkm_rm,
    pub mutex: mutex,
    pub idr: idr,
    pub client_id: },
// A linked list of registry items. The registry RPC will be built from it.
    pub registry_list: list_head,
// The size of the registry RPC
    pub registry_rpc_size: usize,
    pub rpc_seq: u32,

//
// Logging buffers in debugfs. The wrapper objects need to remain
// in memory until the dentry is deleted.
//
    pub parent: *mut dentry,
    pub init: *mut dentry,
    pub rm: *mut dentry,
    pub intr: *mut dentry,
    pub pmu: *mut dentry,
    pub debugfs: },
    pub blob_init: debugfs_blob_wrapper,
    pub blob_intr: debugfs_blob_wrapper,
    pub blob_rm: debugfs_blob_wrapper,
    pub blob_pmu: debugfs_blob_wrapper,

}

extern "C" {
    pub fn nvkm_gsp_rpc_push(_arg: gsp, _arg: argv, _arg: NVKM_GSP_RPC_REPLY_RECV, _arg: argc) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: repv) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}
extern "C" {
    pub fn PTR_ERR(_arg: repv) -> return;
}
extern "C" {
    pub fn nvkm_gsp_rm_alloc_wr(_arg: object, _arg: argv) -> return;
}
extern "C" {
    pub fn nvkm_gsp_client_ctor(: *mut nvkm_gsp, : *mut nvkm_gsp_client) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_client_dtor(: *mut nvkm_gsp_client);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_gsp_event {
    pub device: *mut nvkm_gsp_device,
    pub id: u32,
    pub func: nvkm_gsp_event_func,
    pub object: nvkm_gsp_object,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_gsp_intr_stall(: *mut nvkm_gsp, nvkm_subdev_type: enum, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nvkm_gsp_intr_nonstall(: *mut nvkm_gsp, nvkm_subdev_type: enum, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gv100_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu102_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn tu116_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn ga100_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn ga102_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gh100_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn ad102_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gb100_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
extern "C" {
    pub fn gb202_gsp_new(: *mut nvkm_device, nvkm_subdev_type: enum, _arg: c_int, : *mut nvkm_gsp) -> c_int;
}
