//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/subdev/fb.h
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

// memory type/access flags, do not match hardware values
pub const NV_MEM_ACCESS_RO: c_int = 1;
pub const NV_MEM_ACCESS_WO: c_int = 2;

pub const NV_MEM_ACCESS_SYS: c_int = 4;
pub const NV_MEM_ACCESS_VM: c_int = 8;
pub const NV_MEM_ACCESS_NOSNOOP: c_int = 16;
pub const NV_MEM_TARGET_VRAM: c_int = 0;
pub const NV_MEM_TARGET_PCI: c_int = 1;
pub const NV_MEM_TARGET_PCI_NOSNOOP: c_int = 2;
pub const NV_MEM_TARGET_VM: c_int = 3;
pub const NV_MEM_TARGET_GART: c_int = 4;
pub const NVKM_RAM_TYPE_VM: c_uint = 0x7f;
pub const NV_MEM_COMP_VM: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fb_tile {
    pub tag: *mut nvkm_mm_node,
    pub addr: u32,
    pub limit: u32,
    pub pitch: u32,
    pub zcomp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fb {
    pub func: *const nvkm_fb_func,
    pub subdev: nvkm_subdev,
    pub vpr_scrubber: nvkm_falcon_fw,
    pub flush_page: *mut page,
    pub flush_page_addr: dma_addr_t,
    pub sysmem: },
    pub ram: *mut nvkm_ram,
    pub /: *mut *mut mutex mutex; / protects mm and nvkm_memory::tags,
    pub mm: nvkm_mm,
    pub tags: },
    pub region: [nvkm_fb_tile; 16],
    pub regions: c_int,
    pub tile: },
    pub page: u8,
    pub mmu_rd: *mut nvkm_memory,
    pub mmu_wr: *mut nvkm_memory,
}

extern "C" {
    pub fn nvkm_fb_vidmem_size(: *mut nvkm_device) -> u64;
}
extern "C" {
    pub fn nvkm_fb_mem_unlock(: *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nvkm_fb_tile_fini(: *mut nvkm_fb, region: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nvkm_fb_tile_prog(: *mut nvkm_fb, region: c_int, : *mut nvkm_fb_tile);
}
extern "C" {
    pub fn nv04_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv10_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv1a_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv20_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv25_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv30_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv35_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv36_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv40_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv41_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv44_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv46_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv47_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv49_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv4e_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn nv50_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn g84_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gt215_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn mcp77_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn mcp89_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gf100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gf108_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gk104_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gk110_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gk20a_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gm107_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gm200_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gm20b_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gp100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gp102_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gp10b_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gv100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn tu102_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn ga100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn ga102_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gh100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gb100_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}
extern "C" {
    pub fn gb202_fb_new(: *mut nvkm_device, nvkm_subdev_type: enum, inst: c_int, : *mut nvkm_fb) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ram_data {
    pub head: list_head,
    pub bios: nvbios_ramcfg,
    pub freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_ram_type {
    NVKM_RAM_TYPE_UNKNOWN = 0,
    NVKM_RAM_TYPE_STOLEN,
    NVKM_RAM_TYPE_SGRAM,
    NVKM_RAM_TYPE_SDRAM,
    NVKM_RAM_TYPE_DDR1,
    NVKM_RAM_TYPE_DDR2,
    NVKM_RAM_TYPE_DDR3,
    NVKM_RAM_TYPE_GDDR2,
    NVKM_RAM_TYPE_GDDR3,
    NVKM_RAM_TYPE_GDDR4,
    NVKM_RAM_TYPE_GDDR5,
    NVKM_RAM_TYPE_GDDR5X,
    NVKM_RAM_TYPE_GDDR6,
    NVKM_RAM_TYPE_HBM2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ram {
    pub func: *const nvkm_ram_func,
    pub fb: *mut nvkm_fb,
    pub type: nvkm_ram_type,
    pub size: u64,
pub const NVKM_RAM_MM_SHIFT: c_int = 12;

    pub vram: nvkm_mm,
    pub stolen: u64,
    pub mutex: mutex,
    pub ranks: c_int,
    pub parts: c_int,
    pub part_mask: c_int,
    pub freq: u32,
    pub mr: [u32; 16],
    pub mr1_nuts: u32,
    pub next: *mut nvkm_ram_data,
    pub former: nvkm_ram_data,
    pub xition: nvkm_ram_data,
    pub target: nvkm_ram_data,
}

extern "C" {
    pub fn nvkm_ram_wrap(: *mut nvkm_device, addr: u64, size: u64, : *mut nvkm_memory) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ram_func {
    pub upper: u64,
    pub pltcs): *mut int fbp, int,
    pub pltcs): *mut *mut nvkm_device , int fbp, int,
    pub fbpa): *mut *mut *mut u32 (probe_fbpa_amount)(struct nvkm_device , int,
    pub ): *mut *mut *mut void (dtor)(struct nvkm_ram,
    pub ): *mut *mut int (init)(struct nvkm_ram,
    pub freq): *mut *mut *mut int (calc)(struct nvkm_ram , u32,
    pub ): *mut *mut int (prog)(struct nvkm_ram,
    pub ): *mut *mut void (tidy)(struct nvkm_ram,
}
