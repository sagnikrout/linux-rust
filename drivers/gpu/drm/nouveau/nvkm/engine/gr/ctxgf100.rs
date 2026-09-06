//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/ctxgf100.h
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

extern "C" {
    pub fn gf100_grctx_patch_wr32(: *mut gf100_gr_chan, addr: u32, data: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf100_grctx_func {
    pub on): *mut *mut *mut void (unkn88c)(struct gf100_gr , bool,
// main context generation function
    pub ): *mut *mut void (main)(struct gf100_gr_chan,
// context-specific modify-on-first-load list generation function
    pub ): *mut *mut void (unkn)(struct gf100_gr,
// mmio context data
    pub hub: *const gf100_gr_pack,
    pub gpc_0: *const gf100_gr_pack,
    pub gpc_1: *const gf100_gr_pack,
    pub zcull: *const gf100_gr_pack,
    pub tpc: *const gf100_gr_pack,
    pub ppc: *const gf100_gr_pack,
// indirect context data, generated with icmds/mthds
    pub icmd: *const gf100_gr_pack,
    pub mthd: *const gf100_gr_pack,
    pub sw_veid_bundle_init: *const gf100_gr_pack,
    pub sw_bundle64_init: *const gf100_gr_pack,
// bundle circular buffer
    pub size): *mut *mut *mut void (bundle)(struct gf100_gr_chan , u64 addr, u32,
    pub bundle_size: u32,
    pub bundle_min_gpm_fifo_depth: u32,
    pub bundle_token_limit: u32,
// pagepool
    pub addr): *mut *mut *mut void (pagepool)(struct gf100_gr_chan , u64,
    pub pagepool_size: u32,
// attribute(/alpha) circular buffer
    pub ): *mut *mut u32 (attrib_cb_size)(struct gf100_gr,
    pub size): *mut *mut *mut void (attrib_cb)(struct gf100_gr_chan , u64 addr, u32,
    pub ): *mut *mut void (attrib)(struct gf100_gr_chan,
    pub attrib_nr_max: u32,
    pub attrib_nr: u32,
    pub alpha_nr_max: u32,
    pub alpha_nr: u32,
    pub gfxp_nr: u32,
// some other context buffer
    pub size): *mut *mut *mut void (unknown)(struct gf100_gr_chan , u64 addr, u32,
    pub unknown_size: u32,
// other patch buffer stuff
    pub ): *mut *mut void (patch_ltc)(struct gf100_gr_chan,
// floorsweeping
    pub sm): *mut *mut *mut void (sm_id)(struct gf100_gr , int gpc, int tpc, int,
    pub gpc): *mut *mut *mut void (tpc_nr)(struct gf100_gr , int,
    pub skip_pd_num_tpc_per_gpc: bool,
    pub ): *mut *mut void (r4060a8)(struct gf100_gr,
    pub ): *mut *mut void (rop_mapping)(struct gf100_gr,
    pub ): *mut *mut void (alpha_beta_tables)(struct gf100_gr,
    pub ): *mut *mut void (max_ways_evict)(struct gf100_gr,
    pub ): *mut *mut void (dist_skip_table)(struct gf100_gr,
    pub ): *mut *mut void (r406500)(struct gf100_gr,
    pub ): *mut *mut void (gpc_tpc_nr)(struct gf100_gr,
    pub ): *mut *mut void (r419f78)(struct gf100_gr,
    pub ): *mut *mut void (tpc_mask)(struct gf100_gr,
    pub ): *mut *mut void (smid_config)(struct gf100_gr,
// misc other things
    pub bool): *mut *mut *mut void (r400088)(struct gf100_gr ,,
    pub ): *mut *mut void (r419cb8)(struct gf100_gr,
    pub ): *mut *mut void (r418800)(struct gf100_gr,
    pub ): *mut *mut void (r419eb0)(struct gf100_gr,
    pub ): *mut *mut void (r419e00)(struct gf100_gr,
    pub ): *mut *mut void (r418e94)(struct gf100_gr,
    pub ): *mut *mut void (r419a3c)(struct gf100_gr,
    pub ): *mut *mut void (r408840)(struct gf100_gr,
    pub ): *mut *mut void (r419c0c)(struct gf100_gr,
    pub ): *mut *mut void (r419ea8)(struct gf100_gr,
}

extern "C" {
    pub fn gf100_grctx_generate(: *mut gf100_gr, : *mut gf100_gr_chan, inst: *mut nvkm_gpuobj) -> c_int;
}
extern "C" {
    pub fn gf100_grctx_generate_main(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gf100_grctx_generate_pagepool(: *mut gf100_gr_chan, _arg: u64);
}
extern "C" {
    pub fn gf100_grctx_generate_bundle(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gf100_grctx_generate_attrib_cb_size(: *mut gf100_gr) -> u32;
}
extern "C" {
    pub fn gf100_grctx_generate_attrib_cb(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gf100_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gf100_grctx_generate_unkn(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_floorsweep(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_sm_id(: *mut gf100_gr, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gf100_grctx_generate_tpc_nr(: *mut gf100_gr, _arg: c_int);
}
extern "C" {
    pub fn gf100_grctx_generate_r4060a8(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_rop_mapping(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_alpha_beta_tables(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_max_ways_evict(: *mut gf100_gr);
}
extern "C" {
    pub fn gf100_grctx_generate_r419cb8(: *mut gf100_gr);
}
extern "C" {
    pub fn gf108_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gf108_grctx_generate_unkn(: *mut gf100_gr);
}
extern "C" {
    pub fn gf117_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gf117_grctx_generate_rop_mapping(: *mut gf100_gr);
}
extern "C" {
    pub fn gf117_grctx_generate_dist_skip_table(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_grctx_generate_alpha_beta_tables(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_grctx_generate_gpc_tpc_nr(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_grctx_generate_pagepool(: *mut gf100_gr_chan, _arg: u64);
}
extern "C" {
    pub fn gk104_grctx_generate_bundle(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gk104_grctx_generate_patch_ltc(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gk104_grctx_generate_unkn(: *mut gf100_gr);
}
extern "C" {
    pub fn gk104_grctx_generate_r418800(: *mut gf100_gr);
}
extern "C" {
    pub fn gk110_grctx_generate_r419eb0(: *mut gf100_gr);
}
extern "C" {
    pub fn gk110_grctx_generate_r419f78(: *mut gf100_gr);
}
extern "C" {
    pub fn gm107_grctx_generate_pagepool(: *mut gf100_gr_chan, _arg: u64);
}
extern "C" {
    pub fn gm107_grctx_generate_bundle(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gm107_grctx_generate_attrib_cb(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gm107_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gm107_grctx_generate_sm_id(: *mut gf100_gr, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gm200_grctx_generate_dist_skip_table(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_grctx_generate_r406500(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_grctx_generate_tpc_mask(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_grctx_generate_smid_config(: *mut gf100_gr);
}
extern "C" {
    pub fn gm200_grctx_generate_r419a3c(: *mut gf100_gr);
}
extern "C" {
    pub fn gp100_grctx_generate_pagepool(: *mut gf100_gr_chan, _arg: u64);
}
extern "C" {
    pub fn gp100_grctx_generate_attrib_cb(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gp100_grctx_generate_smid_config(: *mut gf100_gr);
}
extern "C" {
    pub fn gp102_grctx_generate_attrib_cb_size(: *mut gf100_gr) -> u32;
}
extern "C" {
    pub fn gp102_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gv100_grctx_unkn88c(: *mut gf100_gr, _arg: bool);
}
extern "C" {
    pub fn gv100_grctx_generate_unkn(: *mut gf100_gr);
}
extern "C" {
    pub fn gv100_grctx_generate_attrib_cb(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
extern "C" {
    pub fn gv100_grctx_generate_attrib(: *mut gf100_gr_chan);
}
extern "C" {
    pub fn gv100_grctx_generate_rop_mapping(: *mut gf100_gr);
}
extern "C" {
    pub fn gv100_grctx_generate_r400088(: *mut gf100_gr, _arg: bool);
}
extern "C" {
    pub fn tu102_grctx_generate_unknown(: *mut gf100_gr_chan, _arg: u64, _arg: u32);
}
// context init value lists
