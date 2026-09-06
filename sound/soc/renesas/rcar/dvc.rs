//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/rcar/dvc.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car DVC support
//
// Copyright (C) 2014 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// Playback Volume
// amixer set "DVC Out" 100%
//
// Capture Volume
// amixer set "DVC In" 100%
//
// Playback Mute
// amixer set "DVC Out Mute" on
//
// Capture Mute
// amixer set "DVC In Mute" on
//
// Volume Ramp
// amixer set "DVC Out Ramp Up Rate"   "0.125 dB/64 steps"
// amixer set "DVC Out Ramp Down Rate" "0.125 dB/512 steps"
// amixer set "DVC Out Ramp" on
// aplay xxx.wav &
// amixer set "DVC Out"  80%  // Volume Down
// amixer set "DVC Out" 100%  // Volume Up
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_dvc {
    pub mod: rsnd_mod,
    pub volume: rsnd_kctrl_cfg_m,
    pub mute: rsnd_kctrl_cfg_m,
    pub /: *mut *mut rsnd_kctrl_cfg_s ren; / Ramp Enable,
    pub /: *mut *mut rsnd_kctrl_cfg_s rup; / Ramp Rate Up,
    pub /: *mut *mut rsnd_kctrl_cfg_s rdown; / Ramp Rate Down,
}

    container_of((_mod), struct rsnd_dvc, mod)

    for ((i) = 0;						\
    ((i) < rsnd_dvc_nr(priv)) &&			\
    ((pos) = (struct rsnd_dvc *)(priv).dvc + i);	\
    i++)
#[no_mangle]
unsafe extern "C" fn rsnd_dvc_activation(mod: *mut rsnd_mod) {
    static void rsnd_dvc_activation(struct rsnd_mod *mod)
    {
    rsnd_mod_write(mod, DVC_SWRSR, 0);
    rsnd_mod_write(mod, DVC_SWRSR, 1);
    }
#[no_mangle]
unsafe extern "C" fn rsnd_dvc_halt(mod: *mut rsnd_mod) {
    static void rsnd_dvc_halt(struct rsnd_mod *mod)
    {
    rsnd_mod_write(mod, DVC_DVUIR, 1);
    rsnd_mod_write(mod, DVC_SWRSR, 0);
    }

    rsnd_kctrl_vals(dvc.rdown))

    static void rsnd_dvc_volume_parameter(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_dvc *dvc = rsnd_mod_to_dvc(mod);
    u32 val[RSND_MAX_CHANNELS];
    int i;
// Enable Ramp
    if (rsnd_kctrl_vals(dvc.ren))
    for (i = 0; i < RSND_MAX_CHANNELS; i++)
    val[i] = rsnd_kctrl_max(dvc.volume);
    else
    for (i = 0; i < RSND_MAX_CHANNELS; i++)
    val[i] = rsnd_kctrl_valm(dvc.volume, i);
// Enable Digital Volume
    for (i = 0; i < RSND_MAX_CHANNELS; i++)
    rsnd_mod_write(mod, DVC_VOLxR(i), val[i]);
    }
    static void rsnd_dvc_volume_init(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_dvc *dvc = rsnd_mod_to_dvc(mod);
    let mut adinr: u32 = 0;
    let mut dvucr: u32 = 0;
    let mut vrctr: u32 = 0;
    let mut vrpdr: u32 = 0;
    let mut vrdbr: u32 = 0;
    adinr = rsnd_get_adinr_bit(mod, io) |
    rsnd_runtime_channel_after_ctu(io);
// Enable Digital Volume, Zero Cross Mute Mode
    dvucr |= 0x101;
// Enable Ramp
    if (rsnd_kctrl_vals(dvc.ren)) {
    dvucr |= 0x10;
//
// FIXME !!
// use scale-downed Digital Volume
// as Volume Ramp
// 7F FFFF -> 3FF
//
    vrctr = 0xff;
    vrpdr = rsnd_dvc_get_vrpdr(dvc);
    vrdbr = rsnd_dvc_get_vrdbr(dvc);
    }
// Initialize operation
    rsnd_mod_write(mod, DVC_DVUIR, 1);
// General Information
    rsnd_mod_write(mod, DVC_ADINR, adinr);
    rsnd_mod_write(mod, DVC_DVUCR, dvucr);
// Volume Ramp Parameter
    rsnd_mod_write(mod, DVC_VRCTR, vrctr);
    rsnd_mod_write(mod, DVC_VRPDR, vrpdr);
    rsnd_mod_write(mod, DVC_VRDBR, vrdbr);
// Digital Volume Function Parameter
    rsnd_dvc_volume_parameter(io, mod);
// cancel operation
    rsnd_mod_write(mod, DVC_DVUIR, 0);
    }
    static void rsnd_dvc_volume_update(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_dvc *dvc = rsnd_mod_to_dvc(mod);
    let mut zcmcr: u32 = 0;
    let mut vrpdr: u32 = 0;
    let mut vrdbr: u32 = 0;
    int i;
    for (i = 0; i < rsnd_kctrl_size(dvc.mute); i++)
    zcmcr |= (!!rsnd_kctrl_valm(dvc.mute, i)) << i;
    if (rsnd_kctrl_vals(dvc.ren)) {
    vrpdr = rsnd_dvc_get_vrpdr(dvc);
    vrdbr = rsnd_dvc_get_vrdbr(dvc);
    }
// Disable DVC Register access
    rsnd_mod_write(mod, DVC_DVUER, 0);
// Zero Cross Mute Function
    rsnd_mod_write(mod, DVC_ZCMCR, zcmcr);
// Volume Ramp Function
    rsnd_mod_write(mod, DVC_VRPDR, vrpdr);
    rsnd_mod_write(mod, DVC_VRDBR, vrdbr);
// add DVC_VRWTR here
// Digital Volume Function Parameter
    rsnd_dvc_volume_parameter(io, mod);
// Enable DVC Register access
    rsnd_mod_write(mod, DVC_DVUER, 1);
    }
    static int rsnd_dvc_probe_(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    return rsnd_cmd_attach(io, rsnd_mod_id(mod));
    }
    static int rsnd_dvc_init(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    int ret;
    ret = rsnd_mod_power_on(mod);
    if (ret < 0)
    return ret;
    rsnd_dvc_activation(mod);
    rsnd_dvc_volume_init(io, mod);
    rsnd_dvc_volume_update(io, mod);
    return 0;
    }
    static int rsnd_dvc_quit(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    rsnd_dvc_halt(mod);
    rsnd_mod_power_off(mod);
    return 0;
    }
    static int rsnd_dvc_pcm_new(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct rsnd_dvc *dvc = rsnd_mod_to_dvc(mod);
    struct rsnd_dai *rdai = rsnd_io_to_rdai(io);
    let mut is_play: c_int = rsnd_io_is_play(io);
    let mut channels: c_int = rsnd_rdai_channels_get(rdai);
    int ret;
// Volume
    ret = rsnd_kctrl_new_m(mod, io, rtd,
    is_play ?
    "DVC Out Playback Volume" : "DVC In Capture Volume",
    rsnd_kctrl_accept_anytime,
    rsnd_dvc_volume_update,
    &dvc.volume, channels,
    0x00800000 - 1);
    if (ret < 0)
    return ret;
// Mute
    ret = rsnd_kctrl_new_m(mod, io, rtd,
    is_play ?
    "DVC Out Mute Switch" : "DVC In Mute Switch",
    rsnd_kctrl_accept_anytime,
    rsnd_dvc_volume_update,
    &dvc.mute, channels,
    1);
    if (ret < 0)
    return ret;
// Ramp
    ret = rsnd_kctrl_new_s(mod, io, rtd,
    is_play ?
    "DVC Out Ramp Switch" : "DVC In Ramp Switch",
    rsnd_kctrl_accept_anytime,
    rsnd_dvc_volume_update,
    &dvc.ren, 1);
    if (ret < 0)
    return ret;
    ret = rsnd_kctrl_new_e(mod, io, rtd,
    is_play ?
    "DVC Out Ramp Up Rate" : "DVC In Ramp Up Rate",
    rsnd_kctrl_accept_anytime,
    rsnd_dvc_volume_update,
    &dvc.rup,
    volume_ramp_rate,
    VOLUME_RAMP_MAX_DVC);
    if (ret < 0)
    return ret;
    ret = rsnd_kctrl_new_e(mod, io, rtd,
    is_play ?
    "DVC Out Ramp Down Rate" : "DVC In Ramp Down Rate",
    rsnd_kctrl_accept_anytime,
    rsnd_dvc_volume_update,
    &dvc.rdown,
    volume_ramp_rate,
    VOLUME_RAMP_MAX_DVC);
    if (ret < 0)
    return ret;
    return 0;
    }
    static struct dma_chan *rsnd_dvc_dma_req(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_priv *priv = rsnd_mod_to_priv(mod);
    return rsnd_dma_request_channel(rsnd_dvc_of_node(priv),
    DVC_NAME, mod, "tx");
    }

    static void rsnd_dvc_debug_info(struct seq_file *m,
    struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    rsnd_debugfs_mod_reg_show(m, mod, RSND_BASE_SCU,
    0xe00 + rsnd_mod_id(mod) * 0x100, 0x60);
    }

// Macro flag: #define DEBUG_INFO

    static struct rsnd_mod_ops rsnd_dvc_ops = {
    .name		= DVC_NAME,
    .dma_req	= rsnd_dvc_dma_req,
    .probe		= rsnd_dvc_probe_,
    .init		= rsnd_dvc_init,
    .quit		= rsnd_dvc_quit,
    .pcm_new	= rsnd_dvc_pcm_new,
    .get_status	= rsnd_mod_get_status,
    DEBUG_INFO
    };
    struct rsnd_mod *rsnd_dvc_mod_get(struct rsnd_priv *priv, int id)
    {
    if (WARN_ON(id < 0 || id >= rsnd_dvc_nr(priv)))
    id = 0;
    return rsnd_mod_get(rsnd_dvc_get(priv, id));
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_dvc_probe(priv: *mut rsnd_priv) -> c_int {
    int rsnd_dvc_probe(struct rsnd_priv *priv)
    {
    struct device_node *node;
    struct device *dev = rsnd_priv_to_dev(priv);
    struct rsnd_dvc *dvc;
    struct clk *clk;
    int i, nr, ret;
    node = rsnd_dvc_of_node(priv);
    if (!node)
    return 0; /* not used is not error */
    nr = of_get_child_count(node);
    if (!nr) {
    ret = -EINVAL;
    goto rsnd_dvc_probe_done;
    }
    dvc	= devm_kcalloc(dev, nr, sizeof(*dvc), GFP_KERNEL);
    if (!dvc) {
    ret = -ENOMEM;
    goto rsnd_dvc_probe_done;
    }
    priv.dvc_nr	= nr;
    priv.dvc	= dvc;
    i = 0;
    ret = 0;
    for_each_child_of_node_scoped(node, np) {
    dvc = rsnd_dvc_get(priv, i);
    clk = rsnd_devm_clk_get_indexed(dev, DVC_NAME, i);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    goto rsnd_dvc_probe_done;
    }
    ret = rsnd_mod_init(priv, rsnd_mod_get(dvc), &rsnd_dvc_ops,
    clk, core::ptr::null_mut(), RSND_MOD_DVC, i);
    if (ret)
    goto rsnd_dvc_probe_done;
    i++;
    }
    rsnd_dvc_probe_done:
    of_node_put(node);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_dvc_remove(priv: *mut rsnd_priv) {
    void rsnd_dvc_remove(struct rsnd_priv *priv)
    {
    struct rsnd_dvc *dvc;
    int i;
    for_each_rsnd_dvc(dvc, priv, i) {
    rsnd_mod_quit(rsnd_mod_get(dvc));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_dvc_suspend(priv: *mut rsnd_priv) {
    void rsnd_dvc_suspend(struct rsnd_priv *priv)
    {
    struct rsnd_dvc *dvc;
    int i;
    for_each_rsnd_dvc(dvc, priv, i)
    rsnd_suspend_clk_reset(rsnd_mod_get(dvc).clk,
    rsnd_mod_get(dvc).rstc);
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_dvc_resume(priv: *mut rsnd_priv) {
    void rsnd_dvc_resume(struct rsnd_priv *priv)
    {
    struct rsnd_dvc *dvc;
    int i;
    for_each_rsnd_dvc(dvc, priv, i)
    rsnd_resume_clk_reset(rsnd_mod_get(dvc).clk,
    rsnd_mod_get(dvc).rstc);
    }
