//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_asic.h
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


//
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2008 Red Hat Inc.
// Copyright 2009 Jerome Glisse.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Dave Airlie
// Alex Deucher
// Jerome Glisse
//
// common functions
//
extern "C" {
    pub fn radeon_legacy_get_engine_clock(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn radeon_legacy_set_engine_clock(rdev: *mut radeon_device, eng_clock: u32);
}
extern "C" {
    pub fn radeon_legacy_get_memory_clock(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn radeon_legacy_set_clock_gating(rdev: *mut radeon_device, enable: c_int);
}
extern "C" {
    pub fn radeon_atom_get_engine_clock(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn radeon_atom_set_engine_clock(rdev: *mut radeon_device, eng_clock: u32);
}
extern "C" {
    pub fn radeon_atom_get_memory_clock(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn radeon_atom_set_memory_clock(rdev: *mut radeon_device, mem_clock: u32);
}
extern "C" {
    pub fn radeon_atom_set_clock_gating(rdev: *mut radeon_device, enable: c_int);
}
extern "C" {
    pub fn atombios_set_backlight_level(radeon_encoder: *mut radeon_encoder, level: u8);
}
extern "C" {
    pub fn atombios_get_backlight_level(radeon_encoder: *mut radeon_encoder) -> u8;
}
extern "C" {
    pub fn radeon_legacy_set_backlight_level(radeon_encoder: *mut radeon_encoder, level: u8);
}
extern "C" {
    pub fn radeon_legacy_get_backlight_level(radeon_encoder: *mut radeon_encoder) -> u8;
}
//
// r100,rv100,rs100,rv200,rs200
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct r100_mc_save {
    pub GENMO_WT: u32,
    pub CRTC_EXT_CNTL: u32,
    pub CRTC_GEN_CNTL: u32,
    pub CRTC2_GEN_CNTL: u32,
    pub CUR_OFFSET: u32,
    pub CUR2_OFFSET: u32,
}

extern "C" {
    pub fn r100_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_vga_set_state(rdev: *mut radeon_device, state: bool);
}
extern "C" {
    pub fn r100_gpu_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn r100_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn r100_get_vblank_counter(rdev: *mut radeon_device, crtc: c_int) -> u32;
}
extern "C" {
    pub fn r100_pci_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pci_gart_get_page_entry(addr: u64, flags: u32) -> u64;
}
extern "C" {
    pub fn r100_ring_start(rdev: *mut radeon_device, ring: *mut radeon_ring);
}
extern "C" {
    pub fn r100_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn r100_pll_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r100_pll_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r100_clear_surface_reg(rdev: *mut radeon_device, reg: c_int);
}
extern "C" {
    pub fn r100_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn r100_ring_test(rdev: *mut radeon_device, cp: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r100_hpd_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_hpd_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_hpd_sense(rdev: *mut radeon_device, hpd: radeon_hpd_id) -> bool;
}
extern "C" {
    pub fn r100_debugfs_rbbm_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_debugfs_cp_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_cp_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_cp_init(rdev: *mut radeon_device, ring_size: unsigned) -> c_int;
}
extern "C" {
    pub fn r100_cp_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pci_gart_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_pci_gart_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pci_gart_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_pci_gart_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_debugfs_mc_info_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_gui_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r100_irq_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_mc_stop(rdev: *mut radeon_device, save: *mut r100_mc_save);
}
extern "C" {
    pub fn r100_mc_resume(rdev: *mut radeon_device, save: *mut r100_mc_save);
}
extern "C" {
    pub fn r100_vram_init_sizes(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_cp_reset(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r100_vga_render_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_restore_sanity(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_enable_bm(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_set_common_regs(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_bm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_gui_idle(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn r100_pm_misc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pm_prepare(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pm_finish(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_pm_get_dynpm_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r100_page_flip_pending(rdev: *mut radeon_device, crtc: c_int) -> bool;
}
extern "C" {
    pub fn r100_wait_for_vblank(rdev: *mut radeon_device, crtc: c_int);
}
extern "C" {
    pub fn r100_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// r200,rv250,rs300,rv280
//
extern "C" {
    pub fn r200_set_safe_registers(rdev: *mut radeon_device);
}
//
// r300,r350,rv350,rv380
//
extern "C" {
    pub fn r300_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r300_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_gpu_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r300_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r300_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn r300_ring_start(rdev: *mut radeon_device, ring: *mut radeon_ring);
}
extern "C" {
    pub fn r300_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn rv370_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv370_pcie_gart_get_page_entry(addr: u64, flags: u32) -> u64;
}
extern "C" {
    pub fn rv370_set_pcie_lanes(rdev: *mut radeon_device, lanes: c_int);
}
extern "C" {
    pub fn rv370_get_pcie_lanes(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r300_set_reg_safe(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_mc_program(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_mc_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_clock_startup(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r300_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv370_pcie_gart_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv370_pcie_gart_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv370_pcie_gart_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv370_pcie_gart_disable(rdev: *mut radeon_device);
}
//
// r420,r423,rv410
//
extern "C" {
    pub fn r420_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r420_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r420_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r420_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r420_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r420_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r420_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r420_debugfs_pipes_info_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r420_pipes_init(rdev: *mut radeon_device);
}
//
// rs400,rs480
//
extern "C" {
    pub fn rs400_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs400_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs400_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs400_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs400_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs400_gart_get_page_entry(addr: u64, flags: u32) -> u64;
}
extern "C" {
    pub fn rs400_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rs400_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn rs400_gart_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs400_gart_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs400_gart_adjust_size(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs400_gart_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs400_gart_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs400_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// rs600.
//
extern "C" {
    pub fn rs600_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn rs600_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs600_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs600_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs600_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs600_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs600_irq_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_get_vblank_counter(rdev: *mut radeon_device, crtc: c_int) -> u32;
}
extern "C" {
    pub fn rs600_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_gart_get_page_entry(addr: u64, flags: u32) -> u64;
}
extern "C" {
    pub fn rs600_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rs600_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn rs600_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_hpd_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_hpd_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_hpd_sense(rdev: *mut radeon_device, hpd: radeon_hpd_id) -> bool;
}
extern "C" {
    pub fn rs600_pm_misc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_pm_prepare(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_pm_finish(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs600_page_flip_pending(rdev: *mut radeon_device, crtc: c_int) -> bool;
}
extern "C" {
    pub fn rs600_set_safe_registers(rdev: *mut radeon_device);
}
extern "C" {
    pub fn avivo_wait_for_vblank(rdev: *mut radeon_device, crtc: c_int);
}
extern "C" {
    pub fn rs600_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// rs690,rs740
//
extern "C" {
    pub fn rs690_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs690_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs690_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs690_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs690_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rs690_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn rs690_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs690_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// rv515
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv515_mc_save {
    pub vga_render_control: u32,
    pub vga_hdp_control: u32,
    pub crtc_enabled: [bool; 2],
}

extern "C" {
    pub fn rv515_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv515_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rv515_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn rv515_ring_start(rdev: *mut radeon_device, ring: *mut radeon_ring);
}
extern "C" {
    pub fn rv515_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv515_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv515_bandwidth_avivo_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_vga_render_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_set_safe_registers(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_mc_stop(rdev: *mut radeon_device, save: *mut rv515_mc_save);
}
extern "C" {
    pub fn rv515_mc_resume(rdev: *mut radeon_device, save: *mut rv515_mc_save);
}
extern "C" {
    pub fn rv515_clock_startup(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_debugfs(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv515_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// r520,rv530,rv560,rv570,r580
//
extern "C" {
    pub fn r520_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r520_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r520_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
//
// r600,rv610,rv630,rv620,rv635,rv670,rs780,rs880
//
extern "C" {
    pub fn r600_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_vga_set_state(rdev: *mut radeon_device, state: bool);
}
extern "C" {
    pub fn r600_wb_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_wb_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_pciep_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn r600_pciep_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r600_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn r600_dma_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn r600_dma_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn r600_dma_is_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn r600_gfx_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn r600_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn r600_clear_surface_reg(rdev: *mut radeon_device, reg: c_int);
}
extern "C" {
    pub fn r600_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r600_dma_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r600_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn r600_ring_test(rdev: *mut radeon_device, cp: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r600_dma_ring_test(rdev: *mut radeon_device, cp: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn r600_hpd_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_hpd_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_hpd_sense(rdev: *mut radeon_device, hpd: radeon_hpd_id) -> bool;
}
extern "C" {
    pub fn r600_mmio_hdp_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_gui_idle(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn r600_pm_misc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_mc_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn rs780_mc_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn r600_pm_get_dynpm_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_set_pcie_lanes(rdev: *mut radeon_device, lanes: c_int);
}
extern "C" {
    pub fn r600_get_pcie_lanes(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_card_posted(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn r600_cp_stop(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_cp_start(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_ring_init(rdev: *mut radeon_device, cp: *mut radeon_ring, ring_size: unsigned);
}
extern "C" {
    pub fn r600_cp_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_cp_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_count_pipe_bits(val: u32) -> c_int;
}
extern "C" {
    pub fn r600_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_pcie_gart_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_scratch_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_init_microcode(rdev: *mut radeon_device) -> c_int;
}
// r600 irq
extern "C" {
    pub fn r600_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_irq_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_irq_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_ih_ring_init(rdev: *mut radeon_device, ring_size: unsigned);
}
extern "C" {
    pub fn r600_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_irq_suspend(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_disable_interrupts(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_rlc_stop(rdev: *mut radeon_device);
}
// r600 audio
extern "C" {
    pub fn r600_audio_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_audio_set_dto(encoder: *mut drm_encoder, clock: u32);
}
extern "C" {
    pub fn r600_hdmi_update_ACR(encoder: *mut drm_encoder, clock: u32);
}
extern "C" {
    pub fn r600_hdmi_buffer_status_changed(encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn r600_hdmi_update_audio_settings(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn r600_get_xclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn r600_get_gpu_clock_counter(rdev: *mut radeon_device) -> u64;
}
extern "C" {
    pub fn rv6xx_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn r600_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn r600_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r600_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
// r600 dma
// rv6xx dpm
extern "C" {
    pub fn rv6xx_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv6xx_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv6xx_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv6xx_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv6xx_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv6xx_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv6xx_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv6xx_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rv6xx_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rv6xx_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn rv6xx_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
// rs780 dpm
extern "C" {
    pub fn rs780_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs780_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs780_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rs780_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rs780_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rs780_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rs780_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn rs780_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
//
// rv770,rv730,rv710,rv740
//
extern "C" {
    pub fn rv770_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_pm_misc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_page_flip_pending(rdev: *mut radeon_device, crtc: c_int) -> bool;
}
extern "C" {
    pub fn r700_vram_gtt_location(rdev: *mut radeon_device, mc: *mut radeon_mc);
}
extern "C" {
    pub fn r700_cp_stop(rdev: *mut radeon_device);
}
extern "C" {
    pub fn r700_cp_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_get_xclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn rv770_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn rv770_get_temp(rdev: *mut radeon_device) -> c_int;
}
// rv7xx pm
extern "C" {
    pub fn rv770_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn rv770_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn rv770_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rv770_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn rv770_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn rv770_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn rv770_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
//
// evergreen
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct evergreen_mc_save {
    pub vga_render_control: u32,
    pub vga_hdp_control: u32,
    pub crtc_enabled: [bool; RADEON_MAX_CRTCS],
}

extern "C" {
    pub fn evergreen_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_gfx_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn evergreen_dma_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn evergreen_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn evergreen_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn evergreen_hpd_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_hpd_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_hpd_sense(rdev: *mut radeon_device, hpd: radeon_hpd_id) -> bool;
}
extern "C" {
    pub fn evergreen_get_vblank_counter(rdev: *mut radeon_device, crtc: c_int) -> u32;
}
extern "C" {
    pub fn evergreen_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn evergreen_dma_cs_parse(p: *mut radeon_cs_parser) -> c_int;
}
extern "C" {
    pub fn evergreen_pm_misc(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_pm_prepare(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_pm_finish(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn btc_pm_init_profile(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn evergreen_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn evergreen_page_flip_pending(rdev: *mut radeon_device, crtc: c_int) -> bool;
}
extern "C" {
    pub fn dce4_wait_for_vblank(rdev: *mut radeon_device, crtc: c_int);
}
extern "C" {
    pub fn evergreen_disable_interrupt_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn evergreen_mc_wait_for_idle(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn evergreen_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn tn_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cypress_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cypress_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn btc_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn btc_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn btc_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn btc_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn btc_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn btc_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn btc_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn btc_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn btc_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn btc_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn btc_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn btc_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn btc_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn sumo_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn sumo_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn sumo_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn sumo_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn sumo_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn sumo_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn sumo_dpm_get_current_vddc(rdev: *mut radeon_device) -> u16;
}
//
// cayman
//
extern "C" {
    pub fn cayman_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cayman_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cayman_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cayman_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cayman_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cayman_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn cayman_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn cayman_vm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cayman_vm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cayman_vm_page_flags(rdev: *mut radeon_device, flags: u32) -> u32;
}
extern "C" {
    pub fn evergreen_ib_parse(rdev: *mut radeon_device, ib: *mut radeon_ib) -> c_int;
}
extern "C" {
    pub fn evergreen_dma_ib_parse(rdev: *mut radeon_device, ib: *mut radeon_ib) -> c_int;
}
extern "C" {
    pub fn cayman_gfx_is_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn cayman_dma_is_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn cayman_dma_vm_pad_ib(ib: *mut radeon_ib);
}
extern "C" {
    pub fn ni_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ni_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ni_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ni_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ni_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ni_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ni_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ni_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ni_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn ni_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn ni_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn ni_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn ni_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn trinity_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn trinity_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn trinity_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn trinity_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn trinity_dpm_enable_bapm(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn trinity_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn trinity_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn tn_set_vce_clocks(rdev: *mut radeon_device, evclk: u32, ecclk: u32) -> c_int;
}
// DCE6 - SI
extern "C" {
    pub fn dce6_bandwidth_update(rdev: *mut radeon_device);
}
extern "C" {
    pub fn dce6_audio_fini(rdev: *mut radeon_device);
}
//
// si
//
extern "C" {
    pub fn si_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_gfx_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn si_dma_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn si_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn si_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn si_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_vm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_vm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_ib_parse(rdev: *mut radeon_device, ib: *mut radeon_ib) -> c_int;
}
extern "C" {
    pub fn si_get_xclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn si_get_gpu_clock_counter(rdev: *mut radeon_device) -> u64;
}
extern "C" {
    pub fn si_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn si_set_vce_clocks(rdev: *mut radeon_device, evclk: u32, ecclk: u32) -> c_int;
}
extern "C" {
    pub fn si_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn si_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn si_fan_ctrl_get_mode(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn si_fan_ctrl_set_mode(rdev: *mut radeon_device, mode: u32);
}
extern "C" {
    pub fn si_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn si_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
// DCE8 - CIK
extern "C" {
    pub fn dce8_bandwidth_update(rdev: *mut radeon_device);
}
//
// cik
//
extern "C" {
    pub fn cik_get_gpu_clock_counter(rdev: *mut radeon_device) -> u64;
}
extern "C" {
    pub fn cik_get_xclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn cik_pciep_rreg(rdev: *mut radeon_device, reg: u32) -> u32;
}
extern "C" {
    pub fn cik_pciep_wreg(rdev: *mut radeon_device, reg: u32, v: u32);
}
extern "C" {
    pub fn cik_set_uvd_clocks(rdev: *mut radeon_device, vclk: u32, dclk: u32) -> c_int;
}
extern "C" {
    pub fn cik_set_vce_clocks(rdev: *mut radeon_device, evclk: u32, ecclk: u32) -> c_int;
}
extern "C" {
    pub fn cik_sdma_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn cik_sdma_ring_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn cik_sdma_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn cik_sdma_is_lockup(rdev: *mut radeon_device, ring: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn cik_pcie_gart_tlb_flush(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cik_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cik_suspend(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_gfx_is_lockup(rdev: *mut radeon_device, cp: *mut radeon_ring) -> bool;
}
extern "C" {
    pub fn cik_asic_reset(rdev: *mut radeon_device, hard: bool) -> c_int;
}
extern "C" {
    pub fn cik_ring_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
extern "C" {
    pub fn cik_ring_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn cik_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn cik_irq_set(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_irq_process(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_vm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn cik_vm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn cik_sdma_vm_pad_ib(ib: *mut radeon_ib);
}
extern "C" {
    pub fn cik_ib_parse(rdev: *mut radeon_device, ib: *mut radeon_ib) -> c_int;
}
extern "C" {
    pub fn ci_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_get_temp(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn ci_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn ci_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn ci_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn ci_dpm_vblank_too_short(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn ci_dpm_powergate_uvd(rdev: *mut radeon_device, gate: bool);
}
extern "C" {
    pub fn ci_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn ci_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn ci_fan_ctrl_get_mode(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn ci_fan_ctrl_set_mode(rdev: *mut radeon_device, mode: u32);
}
extern "C" {
    pub fn kv_dpm_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_dpm_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_dpm_late_enable(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_dpm_disable(rdev: *mut radeon_device);
}
extern "C" {
    pub fn kv_dpm_pre_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_dpm_set_power_state(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn kv_dpm_post_set_power_state(rdev: *mut radeon_device);
}
extern "C" {
    pub fn kv_dpm_setup_asic(rdev: *mut radeon_device);
}
extern "C" {
    pub fn kv_dpm_display_configuration_changed(rdev: *mut radeon_device);
}
extern "C" {
    pub fn kv_dpm_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn kv_dpm_get_sclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn kv_dpm_get_mclk(rdev: *mut radeon_device, low: bool) -> u32;
}
extern "C" {
    pub fn kv_dpm_powergate_uvd(rdev: *mut radeon_device, gate: bool);
}
extern "C" {
    pub fn kv_dpm_enable_bapm(rdev: *mut radeon_device, enable: bool);
}
extern "C" {
    pub fn kv_dpm_get_current_sclk(rdev: *mut radeon_device) -> u32;
}
extern "C" {
    pub fn kv_dpm_get_current_mclk(rdev: *mut radeon_device) -> u32;
}
// uvd v1.0
extern "C" {
    pub fn uvd_v1_0_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn uvd_v1_0_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn uvd_v1_0_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn uvd_v1_0_start(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn uvd_v1_0_stop(rdev: *mut radeon_device);
}
extern "C" {
    pub fn uvd_v1_0_ring_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn uvd_v1_0_ib_test(rdev: *mut radeon_device, ring: *mut radeon_ring) -> c_int;
}
extern "C" {
    pub fn uvd_v1_0_ib_execute(rdev: *mut radeon_device, ib: *mut radeon_ib);
}
// uvd v2.2
extern "C" {
    pub fn uvd_v2_2_resume(rdev: *mut radeon_device) -> c_int;
}
// uvd v3.1
// uvd v4.2
extern "C" {
    pub fn uvd_v4_2_resume(rdev: *mut radeon_device) -> c_int;
}
// vce v1.0
extern "C" {
    pub fn vce_v1_0_load_fw(rdev: *mut radeon_device, data: *mut u32) -> c_int;
}
extern "C" {
    pub fn vce_v1_0_bo_size(rdev: *mut radeon_device) -> unsigned;
}
extern "C" {
    pub fn vce_v1_0_resume(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn vce_v1_0_init(rdev: *mut radeon_device) -> c_int;
}
extern "C" {
    pub fn vce_v1_0_start(rdev: *mut radeon_device) -> c_int;
}
// vce v2.0
extern "C" {
    pub fn vce_v2_0_bo_size(rdev: *mut radeon_device) -> unsigned;
}
extern "C" {
    pub fn vce_v2_0_resume(rdev: *mut radeon_device) -> c_int;
}
