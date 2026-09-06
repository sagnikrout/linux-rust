//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/gf100.h
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
// Copyright 2010 Red Hat Inc.
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
// Authors: Ben Skeggs
//

pub const GPC_MAX: c_int = 32;
pub const TPC_MAX_PER_GPC: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_zbc_color {
    pub format: u32,
    pub ds: [u32; 4],
    pub l2: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_zbc_depth {
    pub format: u32,
    pub ds: u32,
    pub l2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_zbc_stencil {
    pub format: u32,
    pub ds: u32,
    pub l2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr {
    pub func: *const gf100_gr_func,
    pub base: nvkm_gr,
    pub falcon: nvkm_falcon,
    pub inst: nvkm_blob,
    pub data: nvkm_blob,
    pub mutex: mutex,
    pub disable: u32,
    pub fecs: },
    pub falcon: nvkm_falcon,
    pub inst: nvkm_blob,
    pub data: nvkm_blob,
    pub gpccs: },
    pub firmware: bool,
//
// Used if the register packs are loaded from NVIDIA fw instead of
// using hardcoded arrays. To be allocated with vzalloc().
//
    pub sw_nonctx: *mut gf100_gr_pack,
    pub sw_nonctx1: *mut gf100_gr_pack,
    pub sw_nonctx2: *mut gf100_gr_pack,
    pub sw_nonctx3: *mut gf100_gr_pack,
    pub sw_nonctx4: *mut gf100_gr_pack,
    pub sw_ctx: *mut gf100_gr_pack,
    pub bundle: *mut gf100_gr_pack,
    pub bundle_veid: *mut gf100_gr_pack,
    pub bundle64: *mut gf100_gr_pack,
    pub method: *mut gf100_gr_pack,
    pub zbc_color: [gf100_gr_zbc_color; NVKM_LTC_MAX_ZBC_COLOR_CNT],
    pub zbc_depth: [gf100_gr_zbc_depth; NVKM_LTC_MAX_ZBC_DEPTH_CNT],
    pub zbc_stencil: [gf100_gr_zbc_stencil; NVKM_LTC_MAX_ZBC_DEPTH_CNT],
    pub rop_nr: u8,
    pub gpc_nr: u8,
    pub tpc_nr: [u8; GPC_MAX],
    pub tpc_max: u8,
    pub tpc_total: u8,
    pub ppc_nr: [u8; GPC_MAX],
    pub ppc_mask: [u8; GPC_MAX],
    pub ppc_tpc_mask: [u8; GPC_MAX][4],
    pub ppc_tpc_nr: [u8; GPC_MAX][4],
    pub ppc_tpc_min: u8,
    pub ppc_tpc_max: u8,
    pub ppc_total: u8,
    pub pagepool: *mut nvkm_memory,
    pub bundle_cb: *mut nvkm_memory,
    pub attrib_cb: *mut nvkm_memory,
    pub unknown: *mut nvkm_memory,
    pub screen_tile_row_offset: u8,
    pub tile: [u8; TPC_MAX],
    pub gpc: u8,
    pub tpc: u8,
    pub sm: [}; TPC_MAX],
    pub sm_nr: u8,
    pub size: u32,
    pub data: *mut u32,
    pub size_zcull: u32,
    pub size_pm: u32,
}

extern "C" {
    pub fn gf100_gr_fecs_bind_pointer(: *mut gf100_gr, inst: u32) -> c_int;
}
extern "C" {
    pub fn gf100_gr_fecs_wfi_golden_save(: *mut gf100_gr, inst: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_func_zbc {
    pub zbc): *mut *mut *mut void (clear_color)(struct gf100_gr , int,
    pub zbc): *mut *mut *mut void (clear_depth)(struct gf100_gr , int,
    pub l2): u32 ds, u32,
    pub zbc): *mut *mut *mut void (clear_stencil)(struct gf100_gr , int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_func {
    pub ): *mut *mut int (nonstall)(struct gf100_gr,
    pub ): *mut *mut *mut *mut nvkm_intr (oneinit_intr)(gf100_gr , enum nvkm_intr_type,
    pub ): *mut *mut void (oneinit_tiles)(struct gf100_gr,
    pub ): *mut *mut int (oneinit_sm_id)(struct gf100_gr,
    pub ): *mut *mut int (init)(struct gf100_gr,
    pub ): *mut *mut void (init_419bd8)(struct gf100_gr,
    pub ): *mut *mut void (init_gpc_mmu)(struct gf100_gr,
    pub ): *mut *mut void (init_r405a14)(struct gf100_gr,
    pub ): *mut *mut void (init_bios)(struct gf100_gr,
    pub ): *mut *mut void (init_vsc_stream_master)(struct gf100_gr,
    pub ): *mut *mut void (init_zcull)(struct gf100_gr,
    pub ): *mut *mut void (init_num_active_ltcs)(struct gf100_gr,
    pub ): *mut *mut void (init_rop_active_fbps)(struct gf100_gr,
    pub ): *mut *mut void (init_bios_2)(struct gf100_gr,
    pub ): *mut *mut void (init_swdx_pes_mask)(struct gf100_gr,
    pub ): *mut *mut void (init_fs)(struct gf100_gr,
    pub ): *mut *mut void (init_fecs_exceptions)(struct gf100_gr,
    pub ): *mut *mut void (init_40a790)(struct gf100_gr,
    pub ): *mut *mut void (init_ds_hww_esr_2)(struct gf100_gr,
    pub ): *mut *mut void (init_40601c)(struct gf100_gr,
    pub ): *mut *mut void (init_sked_hww_esr)(struct gf100_gr,
    pub ): *mut *mut void (init_419cc0)(struct gf100_gr,
    pub ): *mut *mut void (init_419eb4)(struct gf100_gr,
    pub ): *mut *mut void (init_419c9c)(struct gf100_gr,
    pub ): *mut *mut void (init_ppc_exceptions)(struct gf100_gr,
    pub tpc): *mut *mut *mut void (init_tex_hww_esr)(struct gf100_gr , int gpc, int,
    pub tpc): *mut *mut *mut void (init_504430)(struct gf100_gr , int gpc, int,
    pub tpc): *mut *mut *mut void (init_shader_exceptions)(struct gf100_gr , int gpc, int,
    pub ): *mut *mut void (init_rop_exceptions)(struct gf100_gr,
    pub ): *mut *mut void (init_exception2)(struct gf100_gr,
    pub ): *mut *mut void (init_400054)(struct gf100_gr,
    pub ): *mut *mut void (init_4188a4)(struct gf100_gr,
    pub tpc): *mut *mut *mut void (trap_mp)(struct gf100_gr , int gpc, int,
    pub ): *mut *mut void (set_hww_esr_report_mask)(struct gf100_gr,
    pub mmio: *const gf100_gr_pack,
    pub ucode: *mut gf100_gr_ucode,
    pub ): *mut *mut void (reset)(struct gf100_gr,
    pub fecs: },
    pub ucode: *mut gf100_gr_ucode,
    pub ): *mut *mut void (reset)(struct gf100_gr,
    pub gpccs: },
    pub ): *mut *mut int (rops)(struct gf100_gr,
    pub gpc_nr: c_int,
    pub tpc_nr: c_int,
    pub ppc_nr: c_int,
    pub grctx: *const gf100_grctx_func,
    pub clkgate_pack: *const nvkm_therm_clkgate_pack,
    pub zbc: *const gf100_gr_func_zbc,
    pub sclass: [nvkm_sclass; ],
}

extern "C" {
    pub fn gf100_gr_rops(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gf100_gr_oneinit_tiles(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_oneinit_sm_id(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gf100_gr_init(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gf100_gr_init_vsc_stream_master(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_zcull(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_num_active_ltcs(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_fecs_exceptions(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_40601c(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_419cc0(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_419eb4(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_tex_hww_esr(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gf100_gr_init_shader_exceptions(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gf100_gr_init_rop_exceptions(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_exception2(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_400054(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_init_num_tpc_per_gpc(: *mut gf100_gr, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn gf100_gr_fecs_reset(: *mut gf100_gr);
}
extern "C" {
    pub fn gf117_gr_init_zcull(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_gr_init_vsc_stream_master(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_gr_init_rop_active_fbps(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_gr_init_ppc_exceptions(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_gr_init_sked_hww_esr(: *mut gf100_gr);
}
extern "C" {
    pub fn gk110_gr_init_419eb4(: *mut gf100_gr);
}
extern "C" {
    pub fn gm107_gr_init_504430(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gm107_gr_init_shader_exceptions(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gm107_gr_init_400054(: *mut gf100_gr);
}
extern "C" {
    pub fn gk20a_gr_init(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_av_to_init_(: *mut nvkm_blob, count: u8, pitch: u32, : *mut gf100_gr_pack) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_av_to_init(: *mut nvkm_blob, : *mut gf100_gr_pack) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_aiv_to_init(: *mut nvkm_blob, : *mut gf100_gr_pack) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_av_to_method(: *mut nvkm_blob, : *mut gf100_gr_pack) -> c_int;
}
extern "C" {
    pub fn gm200_gr_oneinit_tiles(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_gr_oneinit_sm_id(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gm200_gr_rops(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gm200_gr_init_num_active_ltcs(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_gr_init_ds_hww_esr_2(: *mut gf100_gr);
}
extern "C" {
    pub fn gp100_gr_init_rop_active_fbps(: *mut gf100_gr);
}
extern "C" {
    pub fn gp100_gr_init_fecs_exceptions(: *mut gf100_gr);
}
extern "C" {
    pub fn gp100_gr_init_shader_exceptions(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gp100_gr_zbc_clear_color(: *mut gf100_gr, _arg: c_int);
}
extern "C" {
    pub fn gp100_gr_zbc_clear_depth(: *mut gf100_gr, _arg: c_int);
}
extern "C" {
    pub fn gp102_gr_init_swdx_pes_mask(: *mut gf100_gr);
}
extern "C" {
    pub fn gp102_gr_zbc_stencil_get(: *mut gf100_gr, _arg: c_int, u32: const, u32: const) -> c_int;
}
extern "C" {
    pub fn gp102_gr_zbc_clear_stencil(: *mut gf100_gr, _arg: c_int);
}
extern "C" {
    pub fn gv100_gr_oneinit_sm_id(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gv100_gr_nonpes_aware_tpc(gr: *mut gf100_gr, gpc: u32, tpc: u32) -> u32;
}
extern "C" {
    pub fn gv100_gr_init_419bd8(: *mut gf100_gr);
}
extern "C" {
    pub fn gv100_gr_init_504430(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gv100_gr_init_shader_exceptions(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gv100_gr_init_4188a4(: *mut gf100_gr);
}
extern "C" {
    pub fn gv100_gr_trap_mp(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn tu102_gr_av_to_init_veid(: *mut nvkm_blob, : *mut gf100_gr_pack) -> c_int;
}
extern "C" {
    pub fn tu102_gr_init_zcull(: *mut gf100_gr);
}
extern "C" {
    pub fn tu102_gr_init_fs(: *mut gf100_gr);
}
extern "C" {
    pub fn tu102_gr_init_fecs_exceptions(: *mut gf100_gr);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_chan {
    pub object: nvkm_object,
    pub gr: *mut gf100_gr,
    pub vmm: *mut nvkm_vmm,
    pub pagepool: *mut nvkm_vma,
    pub bundle_cb: *mut nvkm_vma,
    pub attrib_cb: *mut nvkm_vma,
    pub unknown: *mut nvkm_vma,
    pub mmio: *mut nvkm_memory,
    pub mmio_vma: *mut nvkm_vma,
    pub mmio_nr: c_int,
}

extern "C" {
    pub fn gf100_gr_ctxctl_debug(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_units(: *mut nvkm_gr) -> u64;
}
extern "C" {
    pub fn gf100_gr_zbc_init(: *mut gf100_gr);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_init {
    pub addr: u32,
    pub count: u8,
    pub pitch: u32,
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_pack {
    pub init: *const gf100_gr_init,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_ucode {
    pub code: nvkm_blob,
    pub data: nvkm_blob,
}

extern "C" {
    pub fn gf100_gr_wait_idle(: *mut gf100_gr) -> c_int;
}
extern "C" {
    pub fn gf100_gr_mmio(: *mut gf100_gr, : *const gf100_gr_pack);
}
extern "C" {
    pub fn gf100_gr_icmd(: *mut gf100_gr, : *const gf100_gr_pack);
}
extern "C" {
    pub fn gf100_gr_mthd(: *mut gf100_gr, : *const gf100_gr_pack);
}
extern "C" {
    pub fn gf100_gr_init_ctxctl(: *mut gf100_gr) -> c_int;
}
// register init value lists
extern "C" {
    pub fn gf100_gr_init_gpc_mmu(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_gr_trap_mp(: *mut gf100_gr, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gm107_gr_init_bios(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_gr_init_gpc_mmu(: *mut gf100_gr);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_gr_fwif {
    pub version: c_int,
    pub ): *const *const *const int (load)(struct gf100_gr , int ver, struct gf100_gr_fwif,
    pub func: *const gf100_gr_func,
    pub fecs: *const nvkm_acr_lsf_func,
    pub gpccs: *const nvkm_acr_lsf_func,
}

extern "C" {
    pub fn gf100_gr_load(: *mut gf100_gr, _arg: c_int, : *const gf100_gr_fwif) -> c_int;
}
extern "C" {
    pub fn gf100_gr_nofw(: *mut gf100_gr, _arg: c_int, : *const gf100_gr_fwif) -> c_int;
}
extern "C" {
    pub fn gk20a_gr_load_sw(: *mut gf100_gr, path: *const c_char, ver: c_int) -> c_int;
}
extern "C" {
    pub fn gm200_gr_nofw(: *mut gf100_gr, _arg: c_int, : *const gf100_gr_fwif) -> c_int;
}
extern "C" {
    pub fn gm200_gr_load(: *mut gf100_gr, _arg: c_int, : *const gf100_gr_fwif) -> c_int;
}
extern "C" {
    pub fn gm20b_gr_acr_bld_write(: *mut nvkm_acr, _arg: u32, : *mut nvkm_acr_lsfw);
}
extern "C" {
    pub fn gm20b_gr_acr_bld_patch(: *mut nvkm_acr, _arg: u32, _arg: i64);
}
extern "C" {
    pub fn gp108_gr_acr_bld_write(: *mut nvkm_acr, _arg: u32, : *mut nvkm_acr_lsfw);
}
extern "C" {
    pub fn gp108_gr_acr_bld_patch(: *mut nvkm_acr, _arg: u32, _arg: i64);
}
