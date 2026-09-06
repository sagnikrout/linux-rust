//! Automatically rewritten from C to Rust
//! Source: sound/soc/renesas/rcar/ctu.c
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
// ctu.c
//
// Copyright (c) 2015 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

//
// User needs to setup CTU by amixer, and its settings are
// based on below registers
//
// CTUn_CPMDR : amixser set "CTU Pass"
// CTUn_SV0xR : amixser set "CTU SV0"
// CTUn_SV1xR : amixser set "CTU SV1"
// CTUn_SV2xR : amixser set "CTU SV2"
// CTUn_SV3xR : amixser set "CTU SV3"
//
// [CTU Pass]
// 0000: default
// 0001: Connect input data of channel 0
// 0010: Connect input data of channel 1
// 0011: Connect input data of channel 2
// 0100: Connect input data of channel 3
// 0101: Connect input data of channel 4
// 0110: Connect input data of channel 5
// 0111: Connect input data of channel 6
// 1000: Connect input data of channel 7
// 1001: Connect calculated data by scale values of matrix row 0
// 1010: Connect calculated data by scale values of matrix row 1
// 1011: Connect calculated data by scale values of matrix row 2
// 1100: Connect calculated data by scale values of matrix row 3
//
// [CTU SVx]
// [Output0] = [SV00, SV01, SV02, SV03, SV04, SV05, SV06, SV07]
// [Output1] = [SV10, SV11, SV12, SV13, SV14, SV15, SV16, SV17]
// [Output2] = [SV20, SV21, SV22, SV23, SV24, SV25, SV26, SV27]
// [Output3] = [SV30, SV31, SV32, SV33, SV34, SV35, SV36, SV37]
// [Output4] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
// [Output5] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
// [Output6] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
// [Output7] = [ 0,   0,    0,    0,    0,    0,    0,    0   ]
//
// [SVxx]
// Plus					Minus
// value	time		dB	value		time		dB
// -----------------------------------------------------------------------
// H'7F_FFFF	2		6	H'80_0000	2		6
// ...
// H'40_0000	1		0	H'C0_0000	1		0
// ...
// H'00_0001	2.38 x 10^-7	-132
// H'00_0000	0		Mute	H'FF_FFFF	2.38 x 10^-7	-132
//
// Ex) Input ch -> Output ch
// 1ch     ->  0ch
// 0ch     ->  1ch
//
// amixer set "CTU Reset" on
// amixer set "CTU Pass" 9,10
// amixer set "CTU SV0" 0,4194304
// amixer set "CTU SV1" 4194304,0
// or
// amixer set "CTU Reset" on
// amixer set "CTU Pass" 2,1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_ctu {
    pub mod: rsnd_mod,
    pub pass: rsnd_kctrl_cfg_m,
    pub sv: [rsnd_kctrl_cfg_m; 4],
    pub reset: rsnd_kctrl_cfg_s,
    pub channels: c_int,
    pub flags: u32,
}

    for ((i) = 0;							\
    ((i) < rsnd_ctu_nr(priv)) &&				\
    ((pos) = (struct rsnd_ctu *)(priv).ctu + i);	\
    i++)

    container_of((_mod), struct rsnd_ctu, mod)

#[no_mangle]
unsafe extern "C" fn rsnd_ctu_activation(mod: *mut rsnd_mod) {
    static void rsnd_ctu_activation(struct rsnd_mod *mod)
    {
    rsnd_mod_write(mod, CTU_SWRSR, 0);
    rsnd_mod_write(mod, CTU_SWRSR, 1);
    }
#[no_mangle]
unsafe extern "C" fn rsnd_ctu_halt(mod: *mut rsnd_mod) {
    static void rsnd_ctu_halt(struct rsnd_mod *mod)
    {
    rsnd_mod_write(mod, CTU_CTUIR, 1);
    rsnd_mod_write(mod, CTU_SWRSR, 0);
    }
    static int rsnd_ctu_probe_(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    return rsnd_cmd_attach(io, rsnd_mod_id(mod));
    }
    static void rsnd_ctu_value_init(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_ctu *ctu = rsnd_mod_to_ctu(mod);
    let mut cpmdr: u32 = 0;
    let mut scmdr: u32 = 0;
    int i, j;
    for (i = 0; i < RSND_MAX_CHANNELS; i++) {
    let mut val: u32 = rsnd_kctrl_valm(ctu.pass, i);
    cpmdr |= val << (28 - (i * 4));
    if ((val > 0x8) && (scmdr < (val - 0x8)))
    scmdr = val - 0x8;
    }
    rsnd_mod_write(mod, CTU_CTUIR, 1);
    rsnd_mod_write(mod, CTU_ADINR, rsnd_runtime_channel_original(io));
    rsnd_mod_write(mod, CTU_CPMDR, cpmdr);
    rsnd_mod_write(mod, CTU_SCMDR, scmdr);
    for (i = 0; i < 4; i++) {
    if (i >= scmdr)
    break;
    for (j = 0; j < RSND_MAX_CHANNELS; j++)
    rsnd_mod_write(mod, CTU_SVxxR(i, j), rsnd_kctrl_valm(ctu.sv[i], j));
    }
    rsnd_mod_write(mod, CTU_CTUIR, 0);
    }
    static void rsnd_ctu_value_reset(struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    struct rsnd_ctu *ctu = rsnd_mod_to_ctu(mod);
    int i;
    if (!rsnd_kctrl_vals(ctu.reset))
    return;
    for (i = 0; i < RSND_MAX_CHANNELS; i++) {
    rsnd_kctrl_valm(ctu.pass, i) = 0;
    rsnd_kctrl_valm(ctu.sv[0],  i) = 0;
    rsnd_kctrl_valm(ctu.sv[1],  i) = 0;
    rsnd_kctrl_valm(ctu.sv[2],  i) = 0;
    rsnd_kctrl_valm(ctu.sv[3],  i) = 0;
    }
    rsnd_kctrl_vals(ctu.reset) = 0;
    }
    static int rsnd_ctu_init(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    int ret;
    ret = rsnd_mod_power_on(mod);
    if (ret < 0)
    return ret;
    rsnd_ctu_activation(mod);
    rsnd_ctu_value_init(io, mod);
    return 0;
    }
    static int rsnd_ctu_quit(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct rsnd_priv *priv)
    {
    rsnd_ctu_halt(mod);
    rsnd_mod_power_off(mod);
    return 0;
    }
    static int rsnd_ctu_pcm_new(struct rsnd_mod *mod,
    struct rsnd_dai_stream *io,
    struct snd_soc_pcm_runtime *rtd)
    {
    struct rsnd_ctu *ctu = rsnd_mod_to_ctu(mod);
    int ret;
    if (rsnd_flags_has(ctu, KCTRL_INITIALIZED))
    return 0;
// CTU Pass
    ret = rsnd_kctrl_new_m(mod, io, rtd, "CTU Pass",
    rsnd_kctrl_accept_anytime,
    core::ptr::null_mut(),
    &ctu.pass, RSND_MAX_CHANNELS,
    0xC);
    if (ret < 0)
    return ret;
// ROW0
    ret = rsnd_kctrl_new_m(mod, io, rtd, "CTU SV0",
    rsnd_kctrl_accept_anytime,
    core::ptr::null_mut(),
    &ctu.sv[0], RSND_MAX_CHANNELS,
    0x00FFFFFF);
    if (ret < 0)
    return ret;
// ROW1
    ret = rsnd_kctrl_new_m(mod, io, rtd, "CTU SV1",
    rsnd_kctrl_accept_anytime,
    core::ptr::null_mut(),
    &ctu.sv[1], RSND_MAX_CHANNELS,
    0x00FFFFFF);
    if (ret < 0)
    return ret;
// ROW2
    ret = rsnd_kctrl_new_m(mod, io, rtd, "CTU SV2",
    rsnd_kctrl_accept_anytime,
    core::ptr::null_mut(),
    &ctu.sv[2], RSND_MAX_CHANNELS,
    0x00FFFFFF);
    if (ret < 0)
    return ret;
// ROW3
    ret = rsnd_kctrl_new_m(mod, io, rtd, "CTU SV3",
    rsnd_kctrl_accept_anytime,
    core::ptr::null_mut(),
    &ctu.sv[3], RSND_MAX_CHANNELS,
    0x00FFFFFF);
    if (ret < 0)
    return ret;
// Reset
    ret = rsnd_kctrl_new_s(mod, io, rtd, "CTU Reset",
    rsnd_kctrl_accept_anytime,
    rsnd_ctu_value_reset,
    &ctu.reset, 1);
    rsnd_flags_set(ctu, KCTRL_INITIALIZED);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rsnd_ctu_id(mod: *mut rsnd_mod) -> c_int {
    static int rsnd_ctu_id(struct rsnd_mod *mod)
    {
//
// ctu00: -> 0, ctu01: -> 0, ctu02: -> 0, ctu03: -> 0
// ctu10: -> 1, ctu11: -> 1, ctu12: -> 1, ctu13: -> 1
//
    return mod.id / 4;
    }
#[no_mangle]
unsafe extern "C" fn rsnd_ctu_id_sub(mod: *mut rsnd_mod) -> c_int {
    static int rsnd_ctu_id_sub(struct rsnd_mod *mod)
    {
//
// ctu00: -> 0, ctu01: -> 1, ctu02: -> 2, ctu03: -> 3
// ctu10: -> 0, ctu11: -> 1, ctu12: -> 2, ctu13: -> 3
//
    return mod.id % 4;
    }

    static void rsnd_ctu_debug_info(struct seq_file *m,
    struct rsnd_dai_stream *io,
    struct rsnd_mod *mod)
    {
    rsnd_debugfs_mod_reg_show(m, mod, RSND_BASE_SCU,
    0x500 + rsnd_mod_id_raw(mod) * 0x100, 0x100);
    }

// Macro flag: #define DEBUG_INFO

    static struct rsnd_mod_ops rsnd_ctu_ops = {
    .name		= CTU_NAME,
    .probe		= rsnd_ctu_probe_,
    .init		= rsnd_ctu_init,
    .quit		= rsnd_ctu_quit,
    .pcm_new	= rsnd_ctu_pcm_new,
    .get_status	= rsnd_mod_get_status,
    .id		= rsnd_ctu_id,
    .id_sub		= rsnd_ctu_id_sub,
    .id_cmd		= rsnd_mod_id_raw,
    DEBUG_INFO
    };
    struct rsnd_mod *rsnd_ctu_mod_get(struct rsnd_priv *priv, int id)
    {
    if (WARN_ON(id < 0 || id >= rsnd_ctu_nr(priv)))
    id = 0;
    return rsnd_mod_get(rsnd_ctu_get(priv, id));
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ctu_probe(priv: *mut rsnd_priv) -> c_int {
    int rsnd_ctu_probe(struct rsnd_priv *priv)
    {
    struct device_node *node;
    struct device *dev = rsnd_priv_to_dev(priv);
    struct rsnd_ctu *ctu;
    struct clk *clk;
    int i, nr, ret;
    node = rsnd_ctu_of_node(priv);
    if (!node)
    return 0; /* not used is not error */
    nr = of_get_child_count(node);
    if (!nr) {
    ret = -EINVAL;
    goto rsnd_ctu_probe_done;
    }
    ctu = devm_kcalloc(dev, nr, sizeof(*ctu), GFP_KERNEL);
    if (!ctu) {
    ret = -ENOMEM;
    goto rsnd_ctu_probe_done;
    }
    priv.ctu_nr	= nr;
    priv.ctu	= ctu;
    i = 0;
    ret = 0;
    for_each_child_of_node_scoped(node, np) {
    ctu = rsnd_ctu_get(priv, i);
//
// CTU00, CTU01, CTU02, CTU03 => CTU0
// CTU10, CTU11, CTU12, CTU13 => CTU1
//
    clk = rsnd_devm_clk_get_indexed(dev, CTU_NAME, i / 4);
    if (IS_ERR(clk)) {
    ret = PTR_ERR(clk);
    goto rsnd_ctu_probe_done;
    }
    ret = rsnd_mod_init(priv, rsnd_mod_get(ctu), &rsnd_ctu_ops,
    clk, core::ptr::null_mut(), RSND_MOD_CTU, i);
    if (ret)
    goto rsnd_ctu_probe_done;
    i++;
    }
    rsnd_ctu_probe_done:
    of_node_put(node);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ctu_remove(priv: *mut rsnd_priv) {
    void rsnd_ctu_remove(struct rsnd_priv *priv)
    {
    struct rsnd_ctu *ctu;
    int i;
    for_each_rsnd_ctu(ctu, priv, i) {
    rsnd_mod_quit(rsnd_mod_get(ctu));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ctu_suspend(priv: *mut rsnd_priv) {
    void rsnd_ctu_suspend(struct rsnd_priv *priv)
    {
    struct rsnd_ctu *ctu;
    int i;
    for_each_rsnd_ctu(ctu, priv, i)
    rsnd_suspend_clk_reset(rsnd_mod_get(ctu).clk,
    rsnd_mod_get(ctu).rstc);
    }
#[no_mangle]
pub unsafe extern "C" fn rsnd_ctu_resume(priv: *mut rsnd_priv) {
    void rsnd_ctu_resume(struct rsnd_priv *priv)
    {
    struct rsnd_ctu *ctu;
    int i;
    for_each_rsnd_ctu(ctu, priv, i)
    rsnd_resume_clk_reset(rsnd_mod_get(ctu).clk,
    rsnd_mod_get(ctu).rstc);
    }
