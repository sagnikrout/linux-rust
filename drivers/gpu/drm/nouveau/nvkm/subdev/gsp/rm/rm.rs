//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/gsp/rm/rm.h
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
//
// Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_rm_impl {
    pub wpr: *const nvkm_rm_wpr,
    pub api: *const nvkm_rm_api,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_rm {
    pub device: *mut nvkm_device,
    pub gpu: *const nvkm_rm_gpu,
    pub wpr: *const nvkm_rm_wpr,
    pub api: *const nvkm_rm_api,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_rm_wpr {
    pub os_carveout_size: u32,
    pub base_size: u32,
    pub heap_size_min: u64,
    pub heap_size_non_wpr: u32,
    pub rsvd_size_pmu: u32,
    pub offset_set_by_acr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_rm_api {
    pub resume): *mut *mut *mut void (set_rmargs)(struct nvkm_gsp , bool,
    pub ): *mut *mut int (set_system_info)(struct nvkm_gsp,
    pub ): *mut *mut int (get_static_info)(struct nvkm_gsp,
    pub inst): *mut *mut *mut bool (xlat_mc_engine_idx)(u32 mc_engine_idx, enum nvkm_subdev_type , int,
    pub ): *mut *mut void (drop_send_user_shared_data)(struct nvkm_gsp,
    pub ): *mut *mut void (drop_post_nocat_record)(struct nvkm_gsp,
    pub ): *mut *mut u32 (sr_data_size)(struct nvkm_gsp,
    pub gsp: *mut },
    pub argc): *mut *mut *mut *mut void (get)(struct nvkm_gsp , u32 fn, u32,
    pub repc): nvkm_gsp_rpc_reply_policy policy, u32,
    pub repv): *mut *mut *mut void (done)(struct nvkm_gsp gsp, void,
    pub rpc: *mut },
    pub params_size): *mut *mut *mut *mut void (get)(struct nvkm_gsp_object , u32 cmd, u32,
    pub repc): *mut *mut *mut *mut *mut int (push)(struct nvkm_gsp_object , void params, u32,
    pub params): *mut *mut *mut void (done)(struct nvkm_gsp_object , void,
    pub ctrl: *mut },
    pub params_size): *mut *mut *mut *mut void (get)(struct nvkm_gsp_object , u32 oclass, u32,
    pub params): *mut *mut *mut *mut void (push)(struct nvkm_gsp_object , void,
    pub params): *mut *mut *mut void (done)(struct nvkm_gsp_object , void,
    pub ): *mut *mut int (free)(struct nvkm_gsp_object,
    pub alloc: *mut },
    pub handle): *mut *mut *mut int (ctor)(struct nvkm_gsp_client , u32,
    pub client: *mut },
    pub ): *mut *mut *mut int (ctor)(struct nvkm_gsp_client , struct nvkm_gsp_device,
    pub ): *mut *mut void (dtor)(struct nvkm_gsp_device,
    pub ): *mut nvkm_gsp_event_func, struct nvkm_gsp_event,
    pub ): *mut *mut void (dtor)(struct nvkm_gsp_event,
    pub event: },
    pub device: *mut },
    pub runtime): *mut *mut *mut int (suspend)(struct nvkm_gsp , bool,
    pub ): *mut *mut void (resume)(struct nvkm_gsp,
    pub fbsr: *mut },
    pub ): *mut *mut int (get_static_info)(struct nvkm_disp,
    pub display_mask): *mut *mut *mut int (get_supported)(struct nvkm_disp , unsigned long,
    pub display_id): *mut *mut *mut int (get_connect_state)(struct nvkm_disp , unsigned,
    pub display_id): *mut *mut *mut int (get_active)(struct nvkm_disp , unsigned head, u32,
    pub val): *mut *mut *mut int (bl_ctrl)(struct nvkm_disp , unsigned display_id, bool set, int,
    pub wm): *mut *mut *mut *mut *mut int (get_caps)(struct nvkm_disp , int link_bw, bool mst, bool,
    pub ): *mut *mut int (set_indexed_link_rates)(struct nvkm_outp,
    pub vblanksym): u32 watermark, u32 hblanksym, u32,
    pub aligned_pbn): u8 slot, u8 slot_nr, u16 pbn, u16,
    pub dp: },
    pub ): *mut nvkm_memory,
    pub ): *mut nvkm_gsp_object,
    pub chan: },
    pub disp: *mut },
    pub nv2080_type): *mut *mut nvkm_subdev_type , int,
    pub ): *mut *mut int (ectx_size)(struct nvkm_fifo,
    pub rsvd_chids: unsigned,
    pub repc): *mut *mut *mut *mut int (rc_triggered)(void priv, u32 fn, void repv, u32,
    pub ): *mut nvkm_gsp_object,
    pub chan: },
    pub fifo: *mut },
    pub ): *mut nvkm_gsp_object,
    pub ofa: *mut *mut *mut *mut *mut } ce, nvdec, nvenc, nvjpg,,
    pub ): *mut *mut int (get_ctxbufs_and_zcull_info)(struct r535_gr,
    pub ): *mut *mut int (init)(struct r535_gr,
    pub ): *mut *mut void (fini)(struct r535_gr,
    pub scrubber: },
    pub gr: *mut },
}

pub type DOD_METHOD_DATA = DOD_METHOD_DATA;
pub type JT_METHOD_DATA = JT_METHOD_DATA;
pub type CAPS_METHOD_DATA = CAPS_METHOD_DATA;
extern "C" {
    pub fn r535_gsp_acpi_dod(_arg: acpi_handle, : *mut DOD_METHOD_DATA);
}
extern "C" {
    pub fn r535_gsp_acpi_jt(_arg: acpi_handle, : *mut JT_METHOD_DATA);
}
extern "C" {
    pub fn r535_gsp_acpi_caps(_arg: acpi_handle, : *mut CAPS_METHOD_DATA);
}
extern "C" {
    pub fn r535_gsp_client_dtor(: *mut nvkm_gsp_client);
}
extern "C" {
    pub fn r535_mmu_vaspace_new(: *mut nvkm_vmm, handle: u32, external: bool) -> c_int;
}
extern "C" {
    pub fn r535_mmu_vaspace_del(: *mut nvkm_vmm);
}
extern "C" {
    pub fn r535_fbsr_resume(: *mut nvkm_gsp);
}
extern "C" {
    pub fn r535_fifo_rc_chid(: *mut nvkm_fifo, chid: c_int);
}
extern "C" {
    pub fn r535_gr_oneinit(: *mut nvkm_gr) -> c_int;
}
extern "C" {
    pub fn r535_gr_units(: *mut nvkm_gr) -> u64;
}
extern "C" {
    pub fn r570_gr_gpc_mask(: *mut nvkm_gsp, mask: *mut u32) -> c_int;
}
extern "C" {
    pub fn r570_gr_tpc_mask(: *mut nvkm_gsp, gpc: c_int, mask: *mut u32) -> c_int;
}
