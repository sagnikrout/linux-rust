//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc.h
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
// linux/sound/soc.h -- ALSA SoC Layer
//
// Author:	Liam Girdwood
// Created:	Aug 11th 2005
// Copyright:	Wolfson Microelectronics. PLC.
//

// For the current users of sound/soc.h to avoid build issues

//
// Convenience kcontrol builders
//

//
// SND_SOC_BYTES_EXT is deprecated, please USE SND_SOC_BYTES_TLV instead
//

//
// Simplified versions of above macros, declaring a struct and calculating
// ARRAY_SIZE internally
//

extern "C" {
    pub fn snd_soc_register_card(card: *mut snd_soc_card) -> c_int;
}
extern "C" {
    pub fn snd_soc_unregister_card(card: *mut snd_soc_card);
}
extern "C" {
    pub fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

extern "C" {
    pub fn snd_soc_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_soc_resume(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn snd_soc_poweroff(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn soc_new_pcm(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}

extern "C" {
    pub fn snd_soc_new_compress(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}

extern "C" {
    pub fn snd_soc_runtime_ignore_pmdown_time(rtd: *mut snd_soc_pcm_runtime) -> bool;
}
// Utility functions to get clock rates from various things
extern "C" {
    pub fn snd_soc_calc_frame_size(sample_size: c_int, channels: c_int, tdm_slots: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_params_to_frame_size(params: *const snd_pcm_hw_params) -> c_int;
}
extern "C" {
    pub fn snd_soc_calc_bclk(fs: c_int, sample_size: c_int, channels: c_int, tdm_slots: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_params_to_bclk(parms: *const snd_pcm_hw_params) -> c_int;
}
extern "C" {
    pub fn snd_soc_ret(dev: *const device, ret: c_int, fmt: *const c_char, ...) -> c_int;
}
// set runtime hw params
extern "C" {
    pub fn snd_soc_free_ac97_component(ac97: *mut snd_ac97);
}

extern "C" {
    pub fn snd_soc_set_ac97_ops(ops: *mut snd_ac97_bus_ops) -> c_int;
}

//
// Controls
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_trigger_order {
// start			stop
    SND_SOC_TRIGGER_ORDER_DEFAULT	= 0,	/* Link->Component->DAI		DAI->Component->Link */
    SND_SOC_TRIGGER_ORDER_LDC,		/* Link->DAI->Component		Component->DAI->Link */

    SND_SOC_TRIGGER_ORDER_MAX,
}

// SoC PCM stream information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub /: *mut *mut *mut u64 formats; / SNDRV_PCM_FMTBIT_,
    pub /: *mut *mut *mut u32 subformats; / for S32_LE format, SNDRV_PCM_SUBFMTBIT_,
    pub /: *mut *mut *mut unsigned int rates; / SNDRV_PCM_RATE_,
    pub /: *mut *mut unsigned int rate_min; / min rate,
    pub /: *mut *mut unsigned int rate_max; / max rate,
    pub /: *mut *mut unsigned int channels_min; / min channels,
    pub /: *mut *mut unsigned int channels_max; / max channels,
    pub /: *mut *mut unsigned int sig_bits; / number of bits of content,
}

// SoC audio ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_ops {
    pub ): *mut *mut int (startup)(struct snd_pcm_substream,
    pub ): *mut *mut void (shutdown)(struct snd_pcm_substream,
    pub ): *mut *mut *mut int (hw_params)(struct snd_pcm_substream , struct snd_pcm_hw_params,
    pub ): *mut *mut int (hw_free)(struct snd_pcm_substream,
    pub ): *mut *mut int (prepare)(struct snd_pcm_substream,
    pub int): *mut *mut *mut int (trigger)(struct snd_pcm_substream ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_compr_ops {
    pub ): *mut *mut int (startup)(struct snd_compr_stream,
    pub ): *mut *mut void (shutdown)(struct snd_compr_stream,
    pub ): *mut *mut int (set_params)(struct snd_compr_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub dai_args: *const of_phandle_args,
//
// Extra format = SND_SOC_DAIFMT_Bx_Fx
//
// [Note] it is Bx_Fx base, not CBx_CFx
//
// It will be used with dai_link->dai_fmt
// see
// snd_soc_runtime_set_dai_fmt()
//
    pub ext_fmt: c_uint,
}

//
// [dai_link->ch_maps Image sample]
//
// -------------------------
// CPU0 <---> Codec0
//
// ch-map[0].cpu = 0	ch-map[0].codec = 0
//
// -------------------------
// CPU0 <---> Codec0
// CPU1 <---> Codec1
// CPU2 <---> Codec2
//
// ch-map[0].cpu = 0	ch-map[0].codec = 0
// ch-map[1].cpu = 1	ch-map[1].codec = 1
// ch-map[2].cpu = 2	ch-map[2].codec = 2
//
// -------------------------
// CPU0 <---> Codec0
// CPU1 <-+-> Codec1
// CPU2 <-
//
// ch-map[0].cpu = 0	ch-map[0].codec = 0
// ch-map[1].cpu = 1	ch-map[1].codec = 1
// ch-map[2].cpu = 2	ch-map[2].codec = 1
//
// -------------------------
// CPU0 <---> Codec0
// CPU1 <-+-> Codec1
// \-> Codec2
//
// ch-map[0].cpu = 0	ch-map[0].codec = 0
// ch-map[1].cpu = 1	ch-map[1].codec = 1
// ch-map[2].cpu = 1	ch-map[2].codec = 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link_ch_map {
    pub cpu: c_uint,
    pub codec: c_uint,
    pub ch_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_link {
// config - must be set by machine driver
    pub /: *const *const *const char name; / Codec name,
    pub /: *const *const *const char stream_name; / Stream name,
//
// You MAY specify the link's CPU-side device, either by device name,
// or by DT/OF node, but not both. If this information is omitted,
// the CPU-side DAI is matched using .cpu_dai_name only, which hence
// must be globally unique. These fields are currently typically used
// only for codec to codec links, or systems using device tree.
//
// You MAY specify the DAI name of the CPU DAI. If this information is
// omitted, the CPU-side DAI is matched using .cpu_name/.cpu_of_node
// only, which only works well when that device exposes a single DAI.
//
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
//
// You MUST specify the link's codec, either by device name, or by
// DT/OF node, but not both.
//
// You MUST specify the DAI name within the codec
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
// num_ch_maps = max(num_cpu, num_codecs)
    pub ch_maps: *mut snd_soc_dai_link_ch_map,
//
// You MAY specify the link's platform/PCM/DMA driver, either by
// device name, or by DT/OF node, but not both. Some forms of link
// do not need a platform. In such case, platforms are not mandatory.
//
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub /: *mut *mut int id; / optional ID for machine driver link identification,
//
// for Codec2Codec
//
    pub c2c_params: *const snd_soc_pcm_stream,
    pub num_c2c_params: c_uint,
    pub /: *mut *mut unsigned int dai_fmt; / format to set on init,
    pub /: *mut *mut snd_soc_dpcm_trigger trigger[2]; / trigger type for DPCM,
// codec/machine specific init - e.g. add machine controls
    pub rtd): *mut *mut int (init)(struct snd_soc_pcm_runtime,
// codec/machine specific exit - dual of init()
    pub rtd): *mut *mut void (exit)(struct snd_soc_pcm_runtime,
// optional hw_params re-writing for BE and FE sync
    pub params): *mut snd_pcm_hw_params,
// machine stream operations
    pub ops: *const snd_soc_ops,
    pub compr_ops: *const snd_soc_compr_ops,
//
// soc_pcm_trigger() start/stop sequence.
// see also
// snd_soc_component_driver
// soc_pcm_trigger()
//
    pub trigger_start: snd_soc_trigger_order,
    pub trigger_stop: snd_soc_trigger_order,
// Mark this pcm with non atomic ops
    pub nonatomic:1: c_uint,
// For unidirectional dai links
    pub playback_only:1: c_uint,
    pub capture_only:1: c_uint,
// Keep DAI active over suspend
    pub ignore_suspend:1: c_uint,
// Symmetry requirements
    pub symmetric_rate:1: c_uint,
    pub symmetric_channels:1: c_uint,
    pub symmetric_sample_bits:1: c_uint,
// Do not create a PCM for this DAI link (Backend link)
    pub no_pcm:1: c_uint,
// This DAI link can route to other DAI links at runtime (Frontend)
    pub dynamic:1: c_uint,
// DPCM used FE & BE merged format
    pub dpcm_merged_format:1: c_uint,
// DPCM used FE & BE merged channel
    pub dpcm_merged_chan:1: c_uint,
// DPCM used FE & BE merged rate
    pub dpcm_merged_rate:1: c_uint,
// pmdown_time is ignored at stop
    pub ignore_pmdown_time:1: c_uint,
// Do not create a PCM for this DAI link (Backend link)
    pub ignore:1: c_uint,

    pub /: *mut *mut snd_soc_dobj dobj; / For topology,

}

extern "C" {
    pub fn max(_arg: link->num_cpus, _arg: link->num_codecs) -> return;
}

//
// Sample 1 : Single CPU/Codec/Platform
//
// SND_SOC_DAILINK_DEFS(test,
// DAILINK_COMP_ARRAY(COMP_CPU("cpu_dai")),
// DAILINK_COMP_ARRAY(COMP_CODEC("codec", "codec_dai")),
// DAILINK_COMP_ARRAY(COMP_PLATFORM("platform")));
//
// struct snd_soc_dai_link link = {
// ...
// SND_SOC_DAILINK_REG(test),
// };
//
// Sample 2 : Multi CPU/Codec, no Platform
//
// SND_SOC_DAILINK_DEFS(test,
// DAILINK_COMP_ARRAY(COMP_CPU("cpu_dai1"),
// COMP_CPU("cpu_dai2")),
// DAILINK_COMP_ARRAY(COMP_CODEC("codec1", "codec_dai1"),
// COMP_CODEC("codec2", "codec_dai2")));
//
// struct snd_soc_dai_link link = {
// ...
// SND_SOC_DAILINK_REG(test),
// };
//
// Sample 3 : Define each CPU/Codec/Platform manually
//
// SND_SOC_DAILINK_DEF(test_cpu,
// DAILINK_COMP_ARRAY(COMP_CPU("cpu_dai1"),
// COMP_CPU("cpu_dai2")));
// SND_SOC_DAILINK_DEF(test_codec,
// DAILINK_COMP_ARRAY(COMP_CODEC("codec1", "codec_dai1"),
// COMP_CODEC("codec2", "codec_dai2")));
// SND_SOC_DAILINK_DEF(test_platform,
// DAILINK_COMP_ARRAY(COMP_PLATFORM("platform")));
//
// struct snd_soc_dai_link link = {
// ...
// SND_SOC_DAILINK_REG(test_cpu,
// test_codec,
// test_platform),
// };
//
// Sample 4 : Sample3 without platform
//
// struct snd_soc_dai_link link = {
// ...
// SND_SOC_DAILINK_REG(test_cpu,
// test_codec);
// };
//

extern "C" {
    pub fn snd_soc_dlc_is_dummy(dlc: *mut snd_soc_dai_link_component) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_codec_conf {
//
// specify device either by device name, or by
// DT/OF node, but not both.
//
    pub dlc: snd_soc_dai_link_component,
//
// optional map of kcontrol, widget and path name prefixes that are
// associated per device
//
    pub name_prefix: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_aux_dev {
//
// specify multi-codec either by device name, or by
// DT/OF node, but not both.
//
    pub dlc: snd_soc_dai_link_component,
// codec/machine specific init - e.g. add machine controls
    pub component): *mut *mut int (init)(struct snd_soc_component,
}

// SoC card
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub long_name: *const c_char,
    pub driver_name: *const c_char,
    pub components: *const c_char,

//
// PCI does not define 0 as invalid, so pci_subsystem_set indicates
// whether a value has been written to these fields.
//
    pub pci_subsystem_vendor: c_ushort,
    pub pci_subsystem_device: c_ushort,
    pub pci_subsystem_set: bool,

    pub topology_shortname: *mut c_char,
    pub dev: *mut device,
    pub snd_card: *mut snd_card,
    pub owner: *mut module,
    pub mutex: mutex,
    pub dapm_mutex: mutex,
// Mutex for PCM operations
    pub pcm_mutex: mutex,
    pub card): *mut *mut int (probe)(struct snd_soc_card,
    pub card): *mut *mut int (late_probe)(struct snd_soc_card,
    pub card): *mut *mut void (fixup_controls)(struct snd_soc_card,
    pub card): *mut *mut int (remove)(struct snd_soc_card,
// the pre and post PM functions are used to do any PM work before and
// after the codec and DAI's do any PM work.
    pub card): *mut *mut int (suspend_pre)(struct snd_soc_card,
    pub card): *mut *mut int (suspend_post)(struct snd_soc_card,
    pub card): *mut *mut int (resume_pre)(struct snd_soc_card,
    pub card): *mut *mut int (resume_post)(struct snd_soc_card,
// callbacks
    pub level): snd_soc_bias_level,
    pub level): snd_soc_bias_level,
    pub link): *mut snd_soc_dai_link,
    pub link): *mut snd_soc_dai_link,
// CPU <--> Codec DAI links
    pub /: *mut *mut *mut snd_soc_dai_link dai_link; / predefined links only,
    pub /: *mut *mut int num_links; / predefined links only,
    pub rtd_list: list_head,
    pub num_rtd: c_int,
// optional codec specific configuration
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_int,
//
// optional auxiliary devices such as amplifiers or codecs with DAI
// link unused
//
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_int,
    pub aux_comp_list: list_head,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
//
// Card-specific routes and widgets.
// Note: of_dapm_xxx for Device Tree; Otherwise for driver build-in.
//
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub ignore_suspend_widgets: *const c_char,
    pub num_ignore_suspend_widgets: c_int,
    pub of_dapm_widgets: *const snd_soc_dapm_widget,
    pub num_of_dapm_widgets: c_int,
    pub of_dapm_routes: *const snd_soc_dapm_route,
    pub num_of_dapm_routes: c_int,
    pub of_ignore_suspend_widgets: *const c_char,
    pub num_of_ignore_suspend_widgets: c_int,
// lists of probed devices belonging to this card
    pub component_dev_list: list_head,
    pub list: list_head,
    pub widgets: list_head,
    pub paths: list_head,
    pub dapm_list: list_head,
    pub dapm_dirty: list_head,
// Generic DAPM context for the card
    pub dapm: *mut snd_soc_dapm_context,
    pub dapm_stats: snd_soc_dapm_stats,

    pub debugfs_card_root: *mut dentry,

    pub deferred_resume_work: work_struct,

// bit field
    pub instantiated:1: c_uint,
    pub fully_routed:1: c_uint,
    pub probed:1: c_uint,
    pub component_chaining:1: c_uint,
    pub devres_dev: *mut device,
    pub drvdata: *mut c_void,
}

// SoC machine DAI configuration, glues a codec and cpu DAI together
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
    pub ops: snd_pcm_ops,
    pub /: *mut *mut unsigned int c2c_params_select; / currently selected c2c_param for dai link,
// Dynamic PCM BE runtime data
    pub 1]: snd_soc_dpcm_runtime dpcm[SNDRV_PCM_STREAM_LAST +,
    pub 1]: *mut *mut snd_soc_dapm_widget c2c_widget[SNDRV_PCM_STREAM_LAST +,
    pub pmdown_time: c_long,
// runtime devices
    pub pcm: *mut snd_pcm,
    pub compr: *mut snd_compr,
//
// dais = cpu_dai + codec_dai
// see
// soc_new_pcm_runtime()
// snd_soc_rtd_to_cpu()
// snd_soc_rtd_to_codec()
//
    pub dais: *mut snd_soc_dai,
    pub delayed_work: delayed_work,
    pub rtd): *mut *mut void (close_delayed_work_func)(struct snd_soc_pcm_runtime,

    pub debugfs_dpcm_root: *mut dentry,

    pub /: *mut *mut unsigned int id; / 0-based and monotonic increasing,
    pub /: *mut *mut list_head list; / rtd list of the soc card,
// function mark
    pub mark_startup: *mut snd_pcm_substream,
    pub mark_hw_params: *mut snd_pcm_substream,
    pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_startup: *mut snd_compr_stream,
// bit field
    pub pop_wait:1: c_uint,
    pub /: *mut *mut unsigned int fe_compr:1; / for Dynamic PCM,
    pub initialized:1: c_uint,
// CPU/Codec/Platform
    pub num_components: c_int,
    pub __counted_by(num_components): *mut *mut snd_soc_component components[],
}

// see soc_new_pcm_runtime()

extern "C" {
    pub fn snd_pcm_substream_chip(_arg: substream) -> return;
}

extern "C" {
    pub fn snd_soc_close_delayed_work(rtd: *mut snd_soc_pcm_runtime);
}
// mixer control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_mixer_control {
// Minimum and maximum specified as written to the hardware
    pub max: int min,,
// Limited maximum value specified as presented through the control
    pub platform_max: c_int,
    pub rreg: int reg,,
    pub rshift: unsigned int shift,,
    pub num_channels: u32,
    pub sign_bit: c_uint,
    pub invert:1: c_uint,
    pub autodisable:1: c_uint,

    pub dobj: snd_soc_dobj,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_bytes {
    pub base: c_int,
    pub num_regs: c_int,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_bytes_ext {
    pub max: c_int,

    pub dobj: snd_soc_dobj,

// used for TLV byte control
    pub size): c_uint,
    pub size): c_uint,
}

// multi register control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_mreg_control {
    pub max: long min,,
    pub invert: unsigned int regbase, regcount, nbits,,
}

// enumerated kcontrol
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_enum {
    pub reg: c_int,
    pub shift_l: c_uchar,
    pub shift_r: c_uchar,
    pub items: c_uint,
    pub mask: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
    pub autodisable:1: c_uint,

    pub dobj: snd_soc_dobj,

}

//
// mc->reg == mc->rreg && mc->shift != mc->rshift, or
// mc->reg != mc->rreg means that the control is
// stereo (bits in one register or in two registers)
//
extern "C" {
    pub fn snd_soc_util_init() -> c_int;
}
extern "C" {
    pub fn snd_soc_util_exit();
}
extern "C" {
    pub fn snd_soc_of_parse_pin_switches(card: *mut snd_soc_card, prop: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_of_parse_aux_devs(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_of_parse_ignore_suspend_widgets(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_soc_daifmt_clock_provider_flipped(dai_fmt: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_soc_daifmt_clock_provider_from_bitmap(bit_frame: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_soc_daifmt_parse_format(np: *mut device_node, prefix: *const c_char) -> c_uint;
}

extern "C" {
    pub fn snd_soc_get_stream_cpu(dai_link: *const snd_soc_dai_link, stream: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_get_dai_id(ep: *mut device_node) -> c_int;
}
extern "C" {
    pub fn snd_soc_of_put_dai_link_codecs(dai_link: *mut snd_soc_dai_link);
}
extern "C" {
    pub fn snd_soc_of_put_dai_link_cpus(dai_link: *mut snd_soc_dai_link);
}
extern "C" {
    pub fn snd_soc_unregister_dai(dai: *mut snd_soc_dai);
}

// set platform name for each dailink
// only single platform is supported for now

//
// DAPM helper functions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_soc_dapm_subclass {
    SND_SOC_DAPM_CLASS_ROOT		= 0,
    SND_SOC_DAPM_CLASS_RUNTIME	= 1,
}

//
// PCM helper functions
//

