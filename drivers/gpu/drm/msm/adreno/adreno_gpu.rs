//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/adreno_gpu.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//
// Copyright (c) 2014,2017, 2019 The Linux Foundation. All rights reserved.
//

//
// @enum adreno_family: identify generation and possibly sub-generation
//
// In some cases there are distinct sub-generations within a major revision
// so it helps to be able to group the GPU devices by generation and if
// necessary sub-generation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adreno_family {
    ADRENO_2XX_GEN1,  /* a20x */
    ADRENO_2XX_GEN2,  /* a22x */
    ADRENO_3XX,
    ADRENO_4XX,
    ADRENO_5XX,
    ADRENO_6XX_GEN1,  /* a630 family */
    ADRENO_6XX_GEN2,  /* a640 family */
    ADRENO_6XX_GEN3,  /* a650 family */
    ADRENO_6XX_GEN4,  /* a660 family */
    ADRENO_7XX_GEN1,  /* a730 family */
    ADRENO_7XX_GEN2,  /* a740 family */
    ADRENO_7XX_GEN3,  /* a750 family */
    ADRENO_8XX_GEN1,  /* a830 family */
    ADRENO_8XX_GEN2,  /* a840 family */
}

// Helper for formating the chip_id in the way that userspace tools like
// crashdec expect.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_gpu_funcs {
    pub base: msm_gpu_funcs,
    pub dev): *mut *mut *mut msm_gpu (init)(drm_device,
    pub gpu): *mut *mut u64 (get_timestamp)(struct msm_gpu,
    pub gx_off): *mut *mut *mut void (bus_halt)(struct adreno_gpu adreno_gpu, bool,
    pub data): *mut *mut *mut int (mmu_fault_handler)(void arg, unsigned long iova, int flags, void,
    pub adreno_gpu): *mut *mut bool (gx_is_on)(struct adreno_gpu,
    pub adreno_gpu): *mut *mut bool (aqe_is_enabled)(struct adreno_gpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_reglist {
    pub offset: u32,
    pub value: u32,
}

// Reglist with pipe information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_reglist_pipe {
    pub offset: u32,
    pub value: u32,
    pub pipe: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_speedbin {
    pub fuse: u16,
    pub speedbin: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_info {
    pub machine: *const c_char,
//
// @chipids: Table of matching chip-ids
//
// Terminated with 0 sentinal
//
    pub chip_ids: *mut u32,
    pub family: adreno_family,
    pub revn: u32,
    pub fw: [*const c_char; ADRENO_FW_MAX],
    pub gmem: u32,
    pub quirks: u64,
    pub funcs: *const adreno_gpu_funcs,
    pub zapfw: *const c_char,
    pub inactive_period: u32,
    pub a6xx: *const a6xx_info,
}

//
// @speedbins: Optional table of fuse to speedbin mappings
//
// Consists of pairs of fuse, index mappings, terminated with
// {SHRT_MAX, 0} sentinal.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_gpulist {
    pub gpus: *const adreno_info,
    pub gpus_count: unsigned,
}

//
// Helper to build a speedbin table, ie. the table:
// fuse | speedbin
// -----+---------
// 0  |   0
// 169 |   1
// 174 |   2
//
// would be declared as:
//
// .speedbins = ADRENO_SPEEDBINS(
// { 0,   0 },
// { 169, 1 },
// { 174, 2 },
// ),
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_protect {
    pub regs: *const u32,
    pub count: u32,
    pub count_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_reglist_list {
// @reg: List of register
    pub regs: *const u32,
// @count: Number of registers in the list
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_reglist_pipe_list {
// @reg: List of register
    pub regs: *const adreno_reglist_pipe,
// @count: Number of registers in the list
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_gpu {
    pub base: msm_gpu,
    pub info: *const adreno_info,
    pub chip_id: u32,
    pub speedbin: u16,
    pub funcs: *const adreno_gpu_funcs,
    pub fault_coredump_done: completion,
// interesting register offsets to dump:
    pub registers: *const c_uint,
//
// Are we loading fw from legacy path?  Prior to addition
// of gpu firmware to linux-firmware, the fw files were
// placed in toplevel firmware directory, following qcom's
// android kernel.  But linux-firmware preferred they be
// placed in a 'qcom' subdirectory.
//
// For backwards compatibility, we try first to load from
// the new path, using request_firmware_direct() to avoid
// any potential timeout waiting for usermode helper, then
// fall back to the old path (with direct load).  And
// finally fall back to request_firmware() with the new
// path to allow the usermode helper.
//
    pub fwloc: },
// firmware:
    pub fw: [*const firmware; ADRENO_FW_MAX],
    pub ubwc_config: *const qcom_ubwc_cfg_data,
//
// Register offsets are different between some GPUs.
// GPU specific offsets will be exported by GPU specific
// code (a3xx_gpu.c) and stored in this common location.
//
    pub reg_offsets: *const c_uint,
    pub gmu_is_wrapper: bool,
    pub has_ray_tracing: bool,
    pub uche_trap_base: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_ocmem {
    pub ocmem: *mut ocmem,
    pub base: c_ulong,
    pub hdl: *mut c_void,
}

// platform config data (ie. from DT, or pdata)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adreno_platform_config {
    pub chip_id: u32,
    pub info: *const adreno_info,
}

// It is probably ok to assume legacy "adreno_rev" format
// for all a6xx devices, but probably best to limit this
// to older things.
//
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 225) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 305) -> return;
}
// yes, 307, because a305c is 306
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 307) -> return;
}
// a306a (marketing name is a308)
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 308) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 320) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 330) -> return;
}
extern "C" {
    pub fn adreno_is_a330(0: gpu) && (adreno_patchid(gpu) >) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 405) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 420) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 430) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 505) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 506) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 508) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 509) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 510) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 512) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 530) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 540) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 610) -> return;
}
extern "C" {
    pub fn adreno_is_a612(_arg: gpu) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 618) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 619) -> return;
}
extern "C" {
    pub fn adreno_is_a619(adreno_has_gmu_wrapper(gpu: gpu) &&) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 630) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 640) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 650) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 660) -> return;
}
extern "C" {
    pub fn adreno_is_revn(_arg: gpu, _arg: 680) -> return;
}
// TODO: 615/616
// check for a650, a660, or any derivatives
// Update with non-fake (i.e. non-A702) Gen 7 GPUs
// Put vm_start above 32b to catch issues with not setting xyz_BASE_HI
pub const ADRENO_VM_START: c_uint = 0x100000000ULL;
extern "C" {
    pub fn adreno_private_vm_size(gpu: *mut msm_gpu) -> u64;
}
extern "C" {
    pub fn adreno_hw_init(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn adreno_recover(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn adreno_flush(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer, reg: u32);
}
extern "C" {
    pub fn adreno_idle(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer) -> bool;
}

extern "C" {
    pub fn adreno_dump_info(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn adreno_dump(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn adreno_wait_ring(ring: *mut msm_ringbuffer, ndwords: u32);
}
extern "C" {
    pub fn adreno_gpu_ocmem_cleanup(ocmem: *mut adreno_ocmem);
}
extern "C" {
    pub fn adreno_gpu_cleanup(gpu: *mut adreno_gpu);
}
extern "C" {
    pub fn adreno_load_fw(adreno_gpu: *mut adreno_gpu) -> c_int;
}
extern "C" {
    pub fn adreno_gpu_state_destroy(state: *mut msm_gpu_state);
}
extern "C" {
    pub fn adreno_gpu_state_get(gpu: *mut msm_gpu, state: *mut msm_gpu_state) -> c_int;
}
extern "C" {
    pub fn adreno_gpu_state_put(state: *mut msm_gpu_state) -> c_int;
}
//
// Common helper function to initialize the default address space for arm-smmu
// attached targets
//
extern "C" {
    pub fn adreno_check_and_reenable_stall(gpu: *mut adreno_gpu);
}
extern "C" {
    pub fn adreno_read_speedbin(dev: *mut device, speedbin: *mut u32) -> c_int;
}
//
// For a5xx and a6xx targets load the zap shader that is used to pull the GPU
// out of secure mode
//
extern "C" {
    pub fn adreno_zap_shader_load(gpu: *mut msm_gpu, pasid: u32) -> c_int;
}
// ringbuffer helpers (the parts that are adreno specific)
// no-op packet:
// Maximum number of values that can be executed for one opcode
pub const TYPE4_MAX_PAYLOAD: c_int = 127;

//
// Given a register and a count, return a value to program into
// REG_CP_PROTECT_REG(n) - this will block both reads and writes for _len
// registers starting at _reg.
//
// The register base needs to be a multiple of the length. If it is not, the
// hardware will quietly mask off the bits for you and shift the size. For
// example, if you intend the protection to start at 0x07 for a length of 4
// (0x07-0x0A) the hardware will actually protect (0x04-0x07) which might
// expose registers you intended to protect!
//

//
// Same as above, but allow reads over the range. For areas of mixed use (such
// as performance counters) this allows us to protect a much larger range with a
// single register
//

