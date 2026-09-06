//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/engine/falcon.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_falcon_dmaidx {
    FALCON_DMAIDX_UCODE		= 0,
    FALCON_DMAIDX_VIRT		= 1,
    FALCON_DMAIDX_PHYS_VID		= 2,
    FALCON_DMAIDX_PHYS_SYS_COH	= 3,
    FALCON_DMAIDX_PHYS_SYS_NCOH	= 4,
    FALCON_SEC2_DMAIDX_UCODE	= 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon {
    pub func: *const nvkm_falcon_func,
    pub owner: *mut nvkm_subdev,
    pub name: *const c_char,
    pub addr: u32,
    pub addr2: u32,
    pub mutex: mutex,
    pub dmem_mutex: mutex,
    pub oneinit: bool,
    pub user: *mut nvkm_subdev,
    pub version: u8,
    pub secret: u8,
    pub debug: bool,
    pub core: *mut nvkm_memory,
    pub external: bool,
    pub limit: u32,
    pub data: *mut u32,
    pub size: u32,
    pub ports: u8,
    pub code: },
    pub limit: u32,
    pub data: *mut u32,
    pub size: u32,
    pub ports: u8,
    pub data: },
    pub engine: nvkm_engine,
}

extern "C" {
    pub fn nvkm_falcon_get(: *mut nvkm_falcon, : *mut nvkm_subdev) -> c_int;
}
extern "C" {
    pub fn nvkm_falcon_put(: *mut nvkm_falcon, : *mut nvkm_subdev);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_func {
    pub ): *mut *mut int (disable)(struct nvkm_falcon,
    pub ): *mut *mut int (enable)(struct nvkm_falcon,
    pub ): *mut *mut int (select)(struct nvkm_falcon,
    pub addr2: u32,
    pub riscv_irqmask: u32,
    pub reset_pmc: bool,
    pub ): *mut *mut int (reset_eng)(struct nvkm_falcon,
    pub ): *mut *mut int (reset_prep)(struct nvkm_falcon,
    pub ): *mut *mut int (reset_wait_mem_scrubbing)(struct nvkm_falcon,
    pub debug: u32,
    pub addr): *mut *mut *mut void (bind_inst)(struct nvkm_falcon , int target, u64,
    pub intr): *mut *mut *mut int (bind_stat)(struct nvkm_falcon , bool,
    pub bind_intr: bool,
    pub imem_pio: *const nvkm_falcon_func_pio,
    pub imem_dma: *const nvkm_falcon_func_dma,
    pub dmem_pio: *const nvkm_falcon_func_pio,
    pub dmem_dma: *const nvkm_falcon_func_dma,
    pub emem_addr: u32,
    pub emem_pio: *const nvkm_falcon_func_pio,
    pub head: u32,
    pub tail: u32,
    pub stride: u32,
    pub msgq: } cmdq,,
    pub ): *mut *mut bool (riscv_active)(struct nvkm_falcon,
    pub ): *mut *mut void (intr_retrigger)(struct nvkm_falcon,
    pub data: *mut u32,
    pub size: u32,
    pub code: },
    pub data: *mut u32,
    pub size: u32,
    pub data: },
    pub ): *mut *mut void (init)(struct nvkm_falcon,
    pub ): *mut *mut *mut void (intr)(struct nvkm_falcon , struct nvkm_chan,
    pub bool): *mut *mut *mut *mut void (load_imem)(struct nvkm_falcon , void , u32, u32, u16, u8,,
    pub u8): *mut *mut *mut *mut void (load_dmem)(struct nvkm_falcon , void , u32, u32,,
    pub ): *mut *mut void (start)(struct nvkm_falcon,
    pub sclass: [nvkm_sclass; ],
}

extern "C" {
    pub fn nvkm_rd32(_arg: falcon->owner->device, addr: falcon->addr +) -> return;
}
extern "C" {
    pub fn nvkm_mask(_arg: device, addr: falcon->addr +, _arg: mask, _arg: val) -> return;
}
extern "C" {
    pub fn nvkm_falcon_load_dmem(: *mut nvkm_falcon, : *mut c_void, _arg: u32, _arg: u32, _arg: u8);
}
extern "C" {
    pub fn nvkm_falcon_start(: *mut nvkm_falcon);
}
