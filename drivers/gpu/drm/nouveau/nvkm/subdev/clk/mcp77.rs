//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/clk/mcp77.c
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
// Copyright 2012 Red Hat Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcp77_clk {
    pub base: nvkm_clk,
    pub vsrc: enum nv_clk_src csrc, ssrc,,
    pub sctrl: u32 cctrl,,
    pub scoef: u32 ccoef,,
    pub spost: u32 cpost,,
    pub vdiv: u32,
}

    static u32
    read_div(struct mcp77_clk *clk)
    {
    struct nvkm_device *device = clk.base.subdev.device;
    return nvkm_rd32(device, 0x004600);
    }
    static u32
    read_pll(struct mcp77_clk *clk, u32 base)
    {
    struct nvkm_device *device = clk.base.subdev.device;
    let mut ctrl: u32 = nvkm_rd32(device, base + 0);
    let mut coef: u32 = nvkm_rd32(device, base + 4);
    let mut ref: u32 = nvkm_clk_read(&clk.base, nv_clk_src_href);
    let mut post_div: u32 = 0;
    let mut clock: u32 = 0;
    int N1, M1;
    switch (base){
    case 0x4020:
    post_div = 1 << ((nvkm_rd32(device, 0x4070) & 0x000f0000) >> 16);
    break;
    case 0x4028:
    post_div = (nvkm_rd32(device, 0x4040) & 0x000f0000) >> 16;
    break;
    default:
    break;
    }
    N1 = (coef & 0x0000ff00) >> 8;
    M1 = (coef & 0x000000ff);
    if ((ctrl & 0x80000000) && M1) {
    clock = ref * N1 / M1;
    clock = clock / post_div;
    }
    return clock;
    }
    static int
    mcp77_clk_read(struct nvkm_clk *base, enum nv_clk_src src)
    {
    struct mcp77_clk *clk = mcp77_clk(base);
    struct nvkm_subdev *subdev = &clk.base.subdev;
    struct nvkm_device *device = subdev.device;
    let mut mast: u32 = nvkm_rd32(device, 0x00c054);
    let mut P: u32 = 0;
    switch (src) {
    case nv_clk_src_crystal:
    return device.crystal;
    case nv_clk_src_href:
    return 100000; /* PCIE reference clock */
    case nv_clk_src_hclkm4:
    return nvkm_clk_read(&clk.base, nv_clk_src_href) * 4;
    case nv_clk_src_hclkm2d3:
    return nvkm_clk_read(&clk.base, nv_clk_src_href) * 2 / 3;
    case nv_clk_src_host:
    switch (mast & 0x000c0000) {
    case 0x00000000: return nvkm_clk_read(&clk.base, nv_clk_src_hclkm2d3);
    case 0x00040000: break;
    case 0x00080000: return nvkm_clk_read(&clk.base, nv_clk_src_hclkm4);
    case 0x000c0000: return nvkm_clk_read(&clk.base, nv_clk_src_cclk);
    }
    break;
    case nv_clk_src_core:
    P = (nvkm_rd32(device, 0x004028) & 0x00070000) >> 16;
    switch (mast & 0x00000003) {
    case 0x00000000: return nvkm_clk_read(&clk.base, nv_clk_src_crystal) >> P;
    case 0x00000001: return 0;
    case 0x00000002: return nvkm_clk_read(&clk.base, nv_clk_src_hclkm4) >> P;
    case 0x00000003: return read_pll(clk, 0x004028) >> P;
    }
    break;
    case nv_clk_src_cclk:
    if ((mast & 0x03000000) != 0x03000000)
    return nvkm_clk_read(&clk.base, nv_clk_src_core);
    if ((mast & 0x00000200) == 0x00000000)
    return nvkm_clk_read(&clk.base, nv_clk_src_core);
    switch (mast & 0x00000c00) {
    case 0x00000000: return nvkm_clk_read(&clk.base, nv_clk_src_href);
    case 0x00000400: return nvkm_clk_read(&clk.base, nv_clk_src_hclkm4);
    case 0x00000800: return nvkm_clk_read(&clk.base, nv_clk_src_hclkm2d3);
    default: return 0;
    }
    case nv_clk_src_shader:
    P = (nvkm_rd32(device, 0x004020) & 0x00070000) >> 16;
    switch (mast & 0x00000030) {
    case 0x00000000:
    if (mast & 0x00000040)
    return nvkm_clk_read(&clk.base, nv_clk_src_href) >> P;
    return nvkm_clk_read(&clk.base, nv_clk_src_crystal) >> P;
    case 0x00000010: break;
    case 0x00000020: return read_pll(clk, 0x004028) >> P;
    case 0x00000030: return read_pll(clk, 0x004020) >> P;
    }
    break;
    case nv_clk_src_mem:
    return 0;
    case nv_clk_src_vdec:
    P = (read_div(clk) & 0x00000700) >> 8;
    switch (mast & 0x00400000) {
    case 0x00400000:
    return nvkm_clk_read(&clk.base, nv_clk_src_core) >> P;
    default:
    return 500000 >> P;
    }
    break;
    default:
    break;
    }
    nvkm_debug(subdev, "unknown clock source %d %08x\n", src, mast);
    return 0;
    }
    static u32
    calc_pll(struct mcp77_clk *clk, u32 reg,
    u32 clock, int *N, int *M, int *P)
    {
    struct nvkm_subdev *subdev = &clk.base.subdev;
    struct nvbios_pll pll;
    int ret;
    ret = nvbios_pll_parse(subdev.device.bios, reg, &pll);
    if (ret)
    return 0;
    pll.vco2.max_freq = 0;
    pll.refclk = nvkm_clk_read(&clk.base, nv_clk_src_href);
    if (!pll.refclk)
    return 0;
    return nv04_pll_calc(subdev, &pll, clock, N, M, core::ptr::null_mut(), core::ptr::null_mut(), P);
    }
    static inline u32
    calc_P(u32 src, u32 target, int *div)
    {
    let mut clk0: u32 = src, clk1 = src;
    for (*div = 0; *div <= 7; (*div)++) {
    if (clk0 <= target) {
    clk1 = clk0 << (*div ? 1 : 0);
    break;
    }
    clk0 >>= 1;
    }
    if (target - clk0 <= clk1 - target)
    return clk0;
    (*div)--;
    return clk1;
    }
    static int
    mcp77_clk_calc(struct nvkm_clk *base, struct nvkm_cstate *cstate)
    {
    struct mcp77_clk *clk = mcp77_clk(base);
    let mut shader: c_int = cstate.domain[nv_clk_src_shader];
    let mut core: c_int = cstate.domain[nv_clk_src_core];
    let mut vdec: c_int = cstate.domain[nv_clk_src_vdec];
    struct nvkm_subdev *subdev = &clk.base.subdev;
    let mut out: u32 = 0, clock = 0;
    int N, M, P1, P2 = 0;
    let mut divs: c_int = 0;
// cclk: find suitable source, disable PLL if we can
    if (core < nvkm_clk_read(&clk.base, nv_clk_src_hclkm4))
    out = calc_P(nvkm_clk_read(&clk.base, nv_clk_src_hclkm4), core, &divs);
// Calculate clock * 2, so shader clock can use it too
    clock = calc_pll(clk, 0x4028, (core << 1), &N, &M, &P1);
    if (abs(core - out) <= abs(core - (clock >> 1))) {
    clk.csrc = nv_clk_src_hclkm4;
    clk.cctrl = divs << 16;
    } else {
// NVCTRL is actually used _after_ NVPOST, and after what we
// call NVPLL. To make matters worse, NVPOST is an integer
// divider instead of a right-shift number.
    if(P1 > 2) {
    P2 = P1 - 2;
    P1 = 2;
    }
    clk.csrc = nv_clk_src_core;
    clk.ccoef = (N << 8) | M;
    clk.cctrl = (P2 + 1) << 16;
    clk.cpost = (1 << P1) << 16;
    }
// sclk: nvpll + divisor, href or spll
    out = 0;
    if (shader == nvkm_clk_read(&clk.base, nv_clk_src_href)) {
    clk.ssrc = nv_clk_src_href;
    } else {
    clock = calc_pll(clk, 0x4020, shader, &N, &M, &P1);
    if (clk.csrc == nv_clk_src_core)
    out = calc_P((core << 1), shader, &divs);
    if (abs(shader - out) <=
    abs(shader - clock) &&
    (divs + P2) <= 7) {
    clk.ssrc = nv_clk_src_core;
    clk.sctrl = (divs + P2) << 16;
    } else {
    clk.ssrc = nv_clk_src_shader;
    clk.scoef = (N << 8) | M;
    clk.sctrl = P1 << 16;
    }
    }
// vclk
    out = calc_P(core, vdec, &divs);
    clock = calc_P(500000, vdec, &P1);
    if(abs(vdec - out) <= abs(vdec - clock)) {
    clk.vsrc = nv_clk_src_cclk;
    clk.vdiv = divs << 16;
    } else {
    clk.vsrc = nv_clk_src_vdec;
    clk.vdiv = P1 << 16;
    }
// Print strategy!
    nvkm_debug(subdev, "nvpll: %08x %08x %08x\n",
    clk.ccoef, clk.cpost, clk.cctrl);
    nvkm_debug(subdev, " spll: %08x %08x %08x\n",
    clk.scoef, clk.spost, clk.sctrl);
    nvkm_debug(subdev, " vdiv: %08x\n", clk.vdiv);
    if (clk.csrc == nv_clk_src_hclkm4)
    nvkm_debug(subdev, "core: hrefm4\n");
    else
    nvkm_debug(subdev, "core: nvpll\n");
    if (clk.ssrc == nv_clk_src_hclkm4)
    nvkm_debug(subdev, "shader: hrefm4\n");
#[no_mangle]
pub unsafe extern "C" fn if(nv_clk_src_core: clk->ssrc ==) -> else {
    else if (clk.ssrc == nv_clk_src_core)
    nvkm_debug(subdev, "shader: nvpll\n");
    else
    nvkm_debug(subdev, "shader: spll\n");
    if (clk.vsrc == nv_clk_src_hclkm4)
    nvkm_debug(subdev, "vdec: 500MHz\n");
    else
    nvkm_debug(subdev, "vdec: core\n");
    return 0;
    }
    static int
    mcp77_clk_prog(struct nvkm_clk *base)
    {
    struct mcp77_clk *clk = mcp77_clk(base);
    struct nvkm_subdev *subdev = &clk.base.subdev;
    struct nvkm_device *device = subdev.device;
    let mut pllmask: u32 = 0, mast;
    unsigned long flags;
    unsigned long *f = &flags;
    let mut ret: c_int = 0;
    ret = gt215_clk_pre(&clk.base, f);
    if (ret)
    goto out;
// First switch to safe clocks: href
    mast = nvkm_mask(device, 0xc054, 0x03400e70, 0x03400640);
    mast &= ~0x00400e73;
    mast |= 0x03000000;
    switch (clk.csrc) {
    case nv_clk_src_hclkm4:
    nvkm_mask(device, 0x4028, 0x00070000, clk.cctrl);
    mast |= 0x00000002;
    break;
    case nv_clk_src_core:
    nvkm_wr32(device, 0x402c, clk.ccoef);
    nvkm_wr32(device, 0x4028, 0x80000000 | clk.cctrl);
    nvkm_wr32(device, 0x4040, clk.cpost);
    pllmask |= (0x3 << 8);
    mast |= 0x00000003;
    break;
    default:
    nvkm_warn(subdev, "Reclocking failed: unknown core clock\n");
    goto resume;
    }
    switch (clk.ssrc) {
    case nv_clk_src_href:
    nvkm_mask(device, 0x4020, 0x00070000, 0x00000000);
// mast |= 0x00000000;
    break;
    case nv_clk_src_core:
    nvkm_mask(device, 0x4020, 0x00070000, clk.sctrl);
    mast |= 0x00000020;
    break;
    case nv_clk_src_shader:
    nvkm_wr32(device, 0x4024, clk.scoef);
    nvkm_wr32(device, 0x4020, 0x80000000 | clk.sctrl);
    nvkm_wr32(device, 0x4070, clk.spost);
    pllmask |= (0x3 << 12);
    mast |= 0x00000030;
    break;
    default:
    nvkm_warn(subdev, "Reclocking failed: unknown sclk clock\n");
    goto resume;
    }
    if (nvkm_msec(device, 2000,
    let mut tmp: u32 = nvkm_rd32(device, 0x004080) & pllmask;
    if (tmp == pllmask)
    break;
    ) < 0)
    goto resume;
    switch (clk.vsrc) {
    case nv_clk_src_cclk:
    mast |= 0x00400000;
    fallthrough;
    default:
    nvkm_wr32(device, 0x4600, clk.vdiv);
    }
    nvkm_wr32(device, 0xc054, mast);
    resume:
// Disable some PLLs and dividers when unused
    if (clk.csrc != nv_clk_src_core) {
    nvkm_wr32(device, 0x4040, 0x00000000);
    nvkm_mask(device, 0x4028, 0x80000000, 0x00000000);
    }
    if (clk.ssrc != nv_clk_src_shader) {
    nvkm_wr32(device, 0x4070, 0x00000000);
    nvkm_mask(device, 0x4020, 0x80000000, 0x00000000);
    }
    out:
    if (ret == -EBUSY)
    f = core::ptr::null_mut();
    gt215_clk_post(&clk.base, f);
    return ret;
    }
    static void
    mcp77_clk_tidy(struct nvkm_clk *base)
    {
    }
    static const struct nvkm_clk_func
    mcp77_clk = {
    .read = mcp77_clk_read,
    .calc = mcp77_clk_calc,
    .prog = mcp77_clk_prog,
    .tidy = mcp77_clk_tidy,
    .domains = {
    { nv_clk_src_crystal, 0xff },
    { nv_clk_src_href   , 0xff },
    { nv_clk_src_core   , 0xff, 0, "core", 1000 },
    { nv_clk_src_shader , 0xff, 0, "shader", 1000 },
    { nv_clk_src_vdec   , 0xff, 0, "vdec", 1000 },
    { nv_clk_src_max }
    }
    };
    int
    mcp77_clk_new(struct nvkm_device *device, enum nvkm_subdev_type type, int inst,
    struct nvkm_clk **pclk)
    {
    struct mcp77_clk *clk;
    if (!(clk = kzalloc_obj(*clk)))
    return -ENOMEM;
// pclk = &clk->base;
    return nvkm_clk_ctor(&mcp77_clk, device, type, inst, true, &clk.base);
    }
