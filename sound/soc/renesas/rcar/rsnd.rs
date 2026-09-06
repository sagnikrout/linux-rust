//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/renesas/rcar/rsnd.h
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
// Renesas R-Car
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

pub const RSND_BASE_ADG: c_int = 0;
pub const RSND_BASE_SSI: c_int = 1;
pub const RSND_BASE_SSIU: c_int = 2;

pub const RSND_BASE_MAX: c_int = 4;
//
// pseudo register
//
// The register address offsets SRU/SCU/SSIU on Gen1/Gen2 are very different.
// This driver uses pseudo register in order to hide it.
// see gen1/gen2 for detail
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsnd_reg {
// SCU (MIX/CTU/DVC)
    SRC_I_BUSIF_MODE,
    SRC_O_BUSIF_MODE,
    SRC_ROUTE_MODE0,
    SRC_SWRSR,
    SRC_SRCIR,
    SRC_ADINR,
    SRC_IFSCR,
    SRC_IFSVR,
    SRC_SRCCR,
    SRC_CTRL,
    SRC_BSDSR,
    SRC_BSISR,
    SRC_INT_ENABLE0,
    SRC_BUSIF_DALIGN,
    SRCIN_TIMSEL0,
    SRCIN_TIMSEL1,
    SRCIN_TIMSEL2,
    SRCIN_TIMSEL3,
    SRCIN_TIMSEL4,
    SRCOUT_TIMSEL0,
    SRCOUT_TIMSEL1,
    SRCOUT_TIMSEL2,
    SRCOUT_TIMSEL3,
    SRCOUT_TIMSEL4,
    SCU_SYS_STATUS0,
    SCU_SYS_STATUS1,
    SCU_SYS_INT_EN0,
    SCU_SYS_INT_EN1,
    CMD_CTRL,
    CMD_BUSIF_MODE,
    CMD_BUSIF_DALIGN,
    CMD_ROUTE_SLCT,
    CMDOUT_TIMSEL,
    CTU_SWRSR,
    CTU_CTUIR,
    CTU_ADINR,
    CTU_CPMDR,
    CTU_SCMDR,
    CTU_SV00R,
    CTU_SV01R,
    CTU_SV02R,
    CTU_SV03R,
    CTU_SV04R,
    CTU_SV05R,
    CTU_SV06R,
    CTU_SV07R,
    CTU_SV10R,
    CTU_SV11R,
    CTU_SV12R,
    CTU_SV13R,
    CTU_SV14R,
    CTU_SV15R,
    CTU_SV16R,
    CTU_SV17R,
    CTU_SV20R,
    CTU_SV21R,
    CTU_SV22R,
    CTU_SV23R,
    CTU_SV24R,
    CTU_SV25R,
    CTU_SV26R,
    CTU_SV27R,
    CTU_SV30R,
    CTU_SV31R,
    CTU_SV32R,
    CTU_SV33R,
    CTU_SV34R,
    CTU_SV35R,
    CTU_SV36R,
    CTU_SV37R,
    MIX_SWRSR,
    MIX_MIXIR,
    MIX_ADINR,
    MIX_MIXMR,
    MIX_MVPDR,
    MIX_MDBAR,
    MIX_MDBBR,
    MIX_MDBCR,
    MIX_MDBDR,
    MIX_MDBER,
    DVC_SWRSR,
    DVC_DVUIR,
    DVC_ADINR,
    DVC_DVUCR,
    DVC_ZCMCR,
    DVC_VOL0R,
    DVC_VOL1R,
    DVC_VOL2R,
    DVC_VOL3R,
    DVC_VOL4R,
    DVC_VOL5R,
    DVC_VOL6R,
    DVC_VOL7R,
    DVC_DVUER,
    DVC_VRCTR,
    DVC_VRPDR,
    DVC_VRDBR,

// ADG
    BRRA,
    BRRB,
    BRGCKR,
    DIV_EN,
    AUDIO_CLK_SEL0,
    AUDIO_CLK_SEL1,
    AUDIO_CLK_SEL2,
    AUDIO_CLK_SEL3,

// SSIU
    SSI_MODE,
    SSI_MODE0,
    SSI_MODE1,
    SSI_MODE2,
    SSI_MODE3,
    SSI_CONTROL,
    SSI_CONTROL2,
    SSI_CTRL,
    SSI_BUSIF0_MODE,
    SSI_BUSIF1_MODE,
    SSI_BUSIF2_MODE,
    SSI_BUSIF3_MODE,
    SSI_BUSIF4_MODE,
    SSI_BUSIF5_MODE,
    SSI_BUSIF6_MODE,
    SSI_BUSIF7_MODE,
    SSI_BUSIF0_ADINR,
    SSI_BUSIF1_ADINR,
    SSI_BUSIF2_ADINR,
    SSI_BUSIF3_ADINR,
    SSI_BUSIF4_ADINR,
    SSI_BUSIF5_ADINR,
    SSI_BUSIF6_ADINR,
    SSI_BUSIF7_ADINR,
    SSI_BUSIF0_DALIGN,
    SSI_BUSIF1_DALIGN,
    SSI_BUSIF2_DALIGN,
    SSI_BUSIF3_DALIGN,
    SSI_BUSIF4_DALIGN,
    SSI_BUSIF5_DALIGN,
    SSI_BUSIF6_DALIGN,
    SSI_BUSIF7_DALIGN,
    SSI_INT_ENABLE,
    SSI_SYS_STATUS0,
    SSI_SYS_STATUS1,
    SSI_SYS_STATUS2,
    SSI_SYS_STATUS3,
    SSI_SYS_STATUS4,
    SSI_SYS_STATUS5,
    SSI_SYS_STATUS6,
    SSI_SYS_STATUS7,
    SSI_SYS_INT_ENABLE0,
    SSI_SYS_INT_ENABLE1,
    SSI_SYS_INT_ENABLE2,
    SSI_SYS_INT_ENABLE3,
    SSI_SYS_INT_ENABLE4,
    SSI_SYS_INT_ENABLE5,
    SSI_SYS_INT_ENABLE6,
    SSI_SYS_INT_ENABLE7,
    HDMI0_SEL,
    HDMI1_SEL,
    SSI9_BUSIF0_MODE,
    SSI9_BUSIF1_MODE,
    SSI9_BUSIF2_MODE,
    SSI9_BUSIF3_MODE,
    SSI9_BUSIF4_MODE,
    SSI9_BUSIF5_MODE,
    SSI9_BUSIF6_MODE,
    SSI9_BUSIF7_MODE,
    SSI9_BUSIF0_ADINR,
    SSI9_BUSIF1_ADINR,
    SSI9_BUSIF2_ADINR,
    SSI9_BUSIF3_ADINR,
    SSI9_BUSIF4_ADINR,
    SSI9_BUSIF5_ADINR,
    SSI9_BUSIF6_ADINR,
    SSI9_BUSIF7_ADINR,
    SSI9_BUSIF0_DALIGN,
    SSI9_BUSIF1_DALIGN,
    SSI9_BUSIF2_DALIGN,
    SSI9_BUSIF3_DALIGN,
    SSI9_BUSIF4_DALIGN,
    SSI9_BUSIF5_DALIGN,
    SSI9_BUSIF6_DALIGN,
    SSI9_BUSIF7_DALIGN,

// SSI
    SSICR,
    SSISR,
    SSITDR,
    SSIRDR,
    SSIWSR,

    REG_MAX,
}

//
// R-Car basic functions
//
extern "C" {
    pub fn rsnd_mod_read(mod: *mut rsnd_mod, reg: rsnd_reg) -> u32;
}
extern "C" {
    pub fn rsnd_mod_write(mod: *mut rsnd_mod, reg: rsnd_reg, data: u32);
}
extern "C" {
    pub fn rsnd_mod_bset(mod: *mut rsnd_mod, reg: rsnd_reg, mask: u32, data: u32);
}
extern "C" {
    pub fn rsnd_get_adinr_bit(mod: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
}
extern "C" {
    pub fn rsnd_get_dalign(mod: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
}
extern "C" {
    pub fn rsnd_get_busif_shift(io: *mut rsnd_dai_stream, mod: *mut rsnd_mod) -> u32;
}
//
// R-Car DMA
//
extern "C" {
    pub fn rsnd_dma_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_dma_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_dma_resume(priv: *mut rsnd_priv);
}
//
// R-Car sound mod
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsnd_mod_type {
    RSND_MOD_AUDMAPP,
    RSND_MOD_AUDMA,
    RSND_MOD_DVC,
    RSND_MOD_MIX,
    RSND_MOD_CTU,
    RSND_MOD_CMD,
    RSND_MOD_SRC,
    RSND_MOD_SSIM3,		/* SSI multi 3 */
    RSND_MOD_SSIM2,		/* SSI multi 2 */
    RSND_MOD_SSIM1,		/* SSI multi 1 */
    RSND_MOD_SSIP,		/* SSI parent */
    RSND_MOD_SSI,
    RSND_MOD_SSIU,
    RSND_MOD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_mod_ops {
    pub name: *mut c_char,
    pub mod): *mut rsnd_mod,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub enable): *mut *mut rsnd_priv priv, int,
    pub rtd): *mut snd_soc_pcm_runtime,
    pub hw_params): *mut snd_pcm_hw_params,
    pub pointer): *mut snd_pcm_uframes_t,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub priv): *mut rsnd_priv,
    pub substream): *mut snd_pcm_substream,
    pub type): rsnd_mod_type,
    pub mod): *mut *mut int (id)(struct rsnd_mod,
    pub mod): *mut *mut int (id_sub)(struct rsnd_mod,
    pub mod): *mut *mut int (id_cmd)(struct rsnd_mod,

    pub mod): *mut *mut rsnd_dai_stream io, rsnd_mod,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_mod {
    pub id: c_int,
    pub type: rsnd_mod_type,
    pub ops: *mut rsnd_mod_ops,
    pub priv: *mut rsnd_priv,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub status: u32,
}

//
// status
//
// 0xH000DCB0
//
// B	0: init		1: quit
// C	0: start	1: stop
// D	0: hw_params	1: hw_free
//
// H is always called (see __rsnd_mod_call)
//
pub const __rsnd_mod_shift_init: c_int = 4;
pub const __rsnd_mod_shift_quit: c_int = 4;
pub const __rsnd_mod_shift_start: c_int = 8;
pub const __rsnd_mod_shift_stop: c_int = 8;
pub const __rsnd_mod_shift_hw_params: c_int = 12;
pub const __rsnd_mod_shift_hw_free: c_int = 12;

pub const __rsnd_mod_add_probe: c_int = 0;
pub const __rsnd_mod_add_remove: c_int = 0;
pub const __rsnd_mod_add_prepare: c_int = 0;
pub const __rsnd_mod_add_cleanup: c_int = 0;

pub const __rsnd_mod_add_irq: c_int = 0;
pub const __rsnd_mod_add_pcm_new: c_int = 0;
pub const __rsnd_mod_add_fallback: c_int = 0;
pub const __rsnd_mod_add_pointer: c_int = 0;
pub const __rsnd_mod_call_probe: c_int = 0;
pub const __rsnd_mod_call_remove: c_int = 0;
pub const __rsnd_mod_call_prepare: c_int = 0;
pub const __rsnd_mod_call_cleanup: c_int = 0;

pub const __rsnd_mod_call_irq: c_int = 0;
pub const __rsnd_mod_call_pcm_new: c_int = 0;
pub const __rsnd_mod_call_fallback: c_int = 0;
pub const __rsnd_mod_call_pointer: c_int = 0;

extern "C" {
    pub fn rsnd_mod_quit(mod: *mut rsnd_mod);
}
extern "C" {
    pub fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control);
}
extern "C" {
    pub fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control);
}
extern "C" {
    pub fn rsnd_mod_id(mod: *mut rsnd_mod) -> c_int;
}
extern "C" {
    pub fn rsnd_mod_id_raw(mod: *mut rsnd_mod) -> c_int;
}
extern "C" {
    pub fn rsnd_mod_id_sub(mod: *mut rsnd_mod) -> c_int;
}

extern "C" {
    pub fn rsnd_node_count(priv: *mut rsnd_priv, node: *mut device_node, name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn rsnd_node_fixed_index(dev: *mut device, node: *mut device_node, name: *mut c_char, idx: c_int) -> c_int;
}
extern "C" {
    pub fn rsnd_channel_normalization(chan: c_int) -> c_int;
}

extern "C" {
    pub fn rsnd_runtime_is_multi_ssi(io: *mut rsnd_dai_stream) -> c_int;
}
extern "C" {
    pub fn rsnd_runtime_is_tdm(io: *mut rsnd_dai_stream) -> c_int;
}
extern "C" {
    pub fn rsnd_runtime_is_tdm_split(io: *mut rsnd_dai_stream) -> c_int;
}
//
// Indexed clock and reset name helpers.
//
// Historically the rsnd driver has looked up per-instance clocks and
// resets using dot-separated names (e.g. "ssi.0", "src.0", "adg.ssi.0").
// Newer Renesas SoC bindings (RZ/G3E and later) use hyphen-separated
// names ("ssi-0", "src-0", ...) to follow the standard Device Tree
// naming convention. These helpers look up the hyphenated name first
// and transparently fall back to the dotted name, so a single driver
// build supports both conventions.
//
// DT
//

//
// R-Car sound DAI
//
pub const RSND_DAI_NAME_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_dai_stream {
    pub name: [c_char; RSND_DAI_NAME_SIZE],
    pub substream: *mut snd_pcm_substream,
    pub mod: [*mut rsnd_mod; RSND_MOD_MAX],
    pub dma: *mut rsnd_mod,
    pub rdai: *mut rsnd_dai,
    pub /: *mut *mut *mut device dmac_dev; / for IPMMU,
    pub /: *mut *mut u32 converted_rate; / converted sampling rate,
    pub /: *mut *mut int converted_chan; / converted channels,
    pub parent_ssi_status: u32,
    pub flags: u32,
}

// flags

extern "C" {
    pub fn rsnd_io_is_working(io: *mut rsnd_dai_stream) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_dai {
    pub name: [c_char; RSND_DAI_NAME_SIZE],
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
    pub priv: *mut rsnd_priv,
    pub constraint: snd_pcm_hw_constraint_list,
    pub dai_args: of_phandle_args,
    pub /: *mut *mut int max_channels; / 2ch - 16ch,
    pub /: *mut *mut int ssi_lane; / 1lane - 4lane,
    pub /: *mut *mut int chan_width; / 16/24/32 bit width,
    pub clk_master:1: c_uint,
    pub bit_clk_inv:1: c_uint,
    pub frm_clk_inv:1: c_uint,
    pub sys_delay:1: c_uint,
    pub data_alignment:1: c_uint,
}

extern "C" {
    pub fn rsnd_rdai_width_ctrl(rdai: *mut rsnd_dai, width: c_int) -> c_int;
}
//
// R-Car Gen1/Gen2
//
extern "C" {
    pub fn rsnd_gen_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_gen_get_phy_addr(priv: *mut rsnd_priv, reg_id: c_int) -> phys_addr_t;
}

//
// R-Car ADG
//
extern "C" {
    pub fn rsnd_adg_clk_query(priv: *mut rsnd_priv, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn rsnd_adg_ssi_clk_stop(ssi_mod: *mut rsnd_mod) -> c_int;
}
extern "C" {
    pub fn rsnd_adg_ssi_clk_try_start(ssi_mod: *mut rsnd_mod, rate: c_uint) -> c_int;
}
extern "C" {
    pub fn rsnd_adg_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_adg_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_adg_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_adg_resume(priv: *mut rsnd_priv);
}

extern "C" {
    pub fn rsnd_adg_clk_control(priv: *mut rsnd_priv, enable: c_int) -> c_int;
}
extern "C" {
    pub fn rsnd_adg_clk_dbg_info(priv: *mut rsnd_priv, m: *mut seq_file);
}
//
// R-Car sound priv
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_priv {
    pub pdev: *mut platform_device,
    pub lock: spinlock_t,
    pub flags: c_ulong,
//
// Flags layout: 0xDCBA
//
// A: R-Car generation (Gen1/Gen2/Gen3/Gen4)
// B: R-Car SoC variant (e.g. SOC_E for E1/E2/E3)
// C: RZ series generation
// D: RZ series SoC identifier (e.g. RZG3E)
//
// Bits 16+ are used for capability flags.
//

//
// below value will be filled on rsnd_gen_probe()
//
    pub gen: *mut c_void,
//
// below value will be filled on rsnd_adg_probe()
//
    pub adg: *mut c_void,
//
// below value will be filled on rsnd_dma_probe()
//
    pub dma: *mut c_void,
//
// below value will be filled on rsnd_ssi_probe()
//
    pub ssi: *mut c_void,
    pub ssi_nr: c_int,
//
// below value will be filled on rsnd_ssiu_probe()
//
    pub ssiu_ctrl: *mut c_void,
    pub ssiu: *mut c_void,
    pub ssiu_nr: c_int,
//
// below value will be filled on rsnd_src_probe()
//
    pub src_ctrl: *mut c_void,
    pub src: *mut c_void,
    pub src_nr: c_int,
//
// below value will be filled on rsnd_ctu_probe()
//
    pub ctu: *mut c_void,
    pub ctu_nr: c_int,
//
// below value will be filled on rsnd_mix_probe()
//
    pub mix: *mut c_void,
    pub mix_nr: c_int,
//
// below value will be filled on rsnd_dvc_probe()
//
    pub dvc: *mut c_void,
    pub dvc_nr: c_int,
//
// below value will be filled on rsnd_cmd_probe()
//
    pub cmd: *mut c_void,
    pub cmd_nr: c_int,
//
// below value will be filled on rsnd_dai_probe()
//
    pub daidrv: *mut snd_soc_dai_driver,
    pub rdai: *mut rsnd_dai,
    pub rdai_nr: c_int,
pub const RSND_MAX_COMPONENT: c_int = 3;
    pub component_dais: [c_int; RSND_MAX_COMPONENT],
}

//
// rsnd_kctrl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_kctrl_cfg {
    pub max: c_uint,
    pub size: c_uint,
    pub val: *mut u32,
    pub texts: *const *const c_char,
    pub io): *mut *mut int (accept)(struct rsnd_dai_stream,
    pub mod): *mut *mut *mut void (update)(struct rsnd_dai_stream io, struct rsnd_mod,
    pub io: *mut rsnd_dai_stream,
    pub card: *mut snd_card,
    pub kctrl: *mut snd_kcontrol,
    pub mod: *mut rsnd_mod,
}

pub const RSND_MAX_CHANNELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_kctrl_cfg_m {
    pub cfg: rsnd_kctrl_cfg,
    pub val: [u32; RSND_MAX_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsnd_kctrl_cfg_s {
    pub cfg: rsnd_kctrl_cfg,
    pub val: u32,
}

extern "C" {
    pub fn rsnd_kctrl_accept_anytime(io: *mut rsnd_dai_stream) -> c_int;
}

//
// R-Car SSI
//
extern "C" {
    pub fn rsnd_ssi_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_ssi_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ssi_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ssi_resume(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ssi_use_busif(io: *mut rsnd_dai_stream) -> c_int;
}
extern "C" {
    pub fn rsnd_ssi_multi_secondaries_runtime(io: *mut rsnd_dai_stream) -> u32;
}
extern "C" {
    pub fn rsnd_ssi_is_dma_mode(mod: *mut rsnd_mod) -> c_int;
}

extern "C" {
    pub fn __rsnd_ssi_is_pin_sharing(mod: *mut rsnd_mod) -> c_int;
}

//
// R-Car SSIU
//
extern "C" {
    pub fn rsnd_ssiu_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_ssiu_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ssiu_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ssiu_resume(priv: *mut rsnd_priv);
}

extern "C" {
    pub fn rsnd_ssiu_busif_err_status_clear(mod: *mut rsnd_mod) -> bool;
}
//
// R-Car SRC
//
extern "C" {
    pub fn rsnd_src_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_src_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_src_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_src_resume(priv: *mut rsnd_priv);
}

//
// R-Car CTU
//
extern "C" {
    pub fn rsnd_ctu_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_ctu_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ctu_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_ctu_resume(priv: *mut rsnd_priv);
}

//
// R-Car MIX
//
extern "C" {
    pub fn rsnd_mix_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_mix_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_mix_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_mix_resume(priv: *mut rsnd_priv);
}

//
// R-Car DVC
//
extern "C" {
    pub fn rsnd_dvc_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_dvc_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_dvc_suspend(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_dvc_resume(priv: *mut rsnd_priv);
}

//
// R-Car CMD
//
extern "C" {
    pub fn rsnd_cmd_probe(priv: *mut rsnd_priv) -> c_int;
}
extern "C" {
    pub fn rsnd_cmd_remove(priv: *mut rsnd_priv);
}
extern "C" {
    pub fn rsnd_cmd_attach(io: *mut rsnd_dai_stream, id: c_int) -> c_int;
}
extern "C" {
    pub fn rsnd_mod_make_sure(mod: *mut rsnd_mod, type: rsnd_mod_type);
}
//
// If you don't need interrupt status debug message,
// define RSND_DEBUG_NO_IRQ_STATUS as 1 on top of src.c/ssi.c
//
// #define RSND_DEBUG_NO_IRQ_STATUS 1
//

extern "C" {
    pub fn rsnd_debugfs_probe(component: *mut snd_soc_component) -> c_int;
}

