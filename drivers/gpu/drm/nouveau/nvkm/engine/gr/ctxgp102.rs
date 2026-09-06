//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/gr/ctxgp102.c
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
// Copyright 2016 Red Hat Inc.
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
// Authors: Ben Skeggs <bskeggs@redhat.com>
//

//
// PGRAPH context implementation
//
    static void
    gp102_grctx_generate_r408840(struct gf100_gr *gr)
    {
    struct nvkm_device *device = gr.base.engine.subdev.device;
    nvkm_mask(device, 0x408840, 0x00000003, 0x00000000);
    }
    void
    gp102_grctx_generate_attrib(struct gf100_gr_chan *chan)
    {
    struct gf100_gr *gr = chan.gr;
    const struct gf100_grctx_func *grctx = gr.func.grctx;
    let mut alpha: u32 = grctx.alpha_nr;
    let mut attrib: u32 = grctx.attrib_nr;
    let mut gfxp: u32 = grctx.gfxp_nr;
    let mut max_batches: c_int = 0xffff;
    let mut size: u32 = grctx.alpha_nr_max * gr.tpc_total;
    let mut ao: u32 = 0;
    let mut bo: u32 = ao + size;
    int gpc, ppc, n = 0;
    gf100_grctx_patch_wr32(chan, 0x405830, attrib);
    gf100_grctx_patch_wr32(chan, 0x40585c, alpha);
    gf100_grctx_patch_wr32(chan, 0x4064c4, ((alpha / 4) << 16) | max_batches);
    for (gpc = 0; gpc < gr.gpc_nr; gpc++) {
    for (ppc = 0; ppc < gr.func.ppc_nr; ppc++, n++) {
    let mut as: u32 = alpha * gr.ppc_tpc_nr[gpc][ppc];
    let mut bs: u32 = attrib * gr.ppc_tpc_max;
    let mut gs: u32 = gfxp * gr.ppc_tpc_max;
    let mut u: u32 = 0x418ea0 + (n * 0x04);
    let mut o: u32 = PPC_UNIT(gpc, ppc, 0);
    let mut p: u32 = GPC_UNIT(gpc, 0xc44 + (ppc * 4));
    if (!(gr.ppc_mask[gpc] & (1 << ppc)))
    continue;
    gf100_grctx_patch_wr32(chan, o + 0xc0, gs);
    gf100_grctx_patch_wr32(chan, p, bs);
    gf100_grctx_patch_wr32(chan, o + 0xf4, bo);
    gf100_grctx_patch_wr32(chan, o + 0xf0, bs);
    bo += gs;
    gf100_grctx_patch_wr32(chan, o + 0xe4, as);
    gf100_grctx_patch_wr32(chan, o + 0xf8, ao);
    ao += grctx.alpha_nr_max * gr.ppc_tpc_nr[gpc][ppc];
    gf100_grctx_patch_wr32(chan, u, bs);
    }
    }
    gf100_grctx_patch_wr32(chan, 0x4181e4, 0x00000100);
    gf100_grctx_patch_wr32(chan, 0x41befc, 0x00000100);
    }
    u32
    gp102_grctx_generate_attrib_cb_size(struct gf100_gr *gr)
    {
    const struct gf100_grctx_func *grctx = gr.func.grctx;
    let mut size: u32 = grctx.alpha_nr_max * gr.tpc_total;
    int gpc;
    for (gpc = 0; gpc < gr.gpc_nr; gpc++)
    size += grctx.gfxp_nr * gr.func.ppc_nr * gr.ppc_tpc_max;
    return ((size * 0x20) + 127) & ~127;
    }
    const struct gf100_grctx_func
    gp102_grctx = {
    .main = gf100_grctx_generate_main,
    .unkn = gk104_grctx_generate_unkn,
    .bundle = gm107_grctx_generate_bundle,
    .bundle_size = 0x3000,
    .bundle_min_gpm_fifo_depth = 0x180,
    .bundle_token_limit = 0x900,
    .pagepool = gp100_grctx_generate_pagepool,
    .pagepool_size = 0x20000,
    .attrib_cb_size = gp102_grctx_generate_attrib_cb_size,
    .attrib_cb = gp100_grctx_generate_attrib_cb,
    .attrib = gp102_grctx_generate_attrib,
    .attrib_nr_max = 0x4b0,
    .attrib_nr = 0x320,
    .alpha_nr_max = 0xc00,
    .alpha_nr = 0x800,
    .gfxp_nr = 0xba8,
    .sm_id = gm107_grctx_generate_sm_id,
    .rop_mapping = gf117_grctx_generate_rop_mapping,
    .dist_skip_table = gm200_grctx_generate_dist_skip_table,
    .r406500 = gm200_grctx_generate_r406500,
    .gpc_tpc_nr = gk104_grctx_generate_gpc_tpc_nr,
    .tpc_mask = gm200_grctx_generate_tpc_mask,
    .smid_config = gp100_grctx_generate_smid_config,
    .r419a3c = gm200_grctx_generate_r419a3c,
    .r408840 = gp102_grctx_generate_r408840,
    };
