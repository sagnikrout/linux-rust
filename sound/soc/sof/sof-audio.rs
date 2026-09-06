//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/sof-audio.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2019 Intel Corporation
//
// Author: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//

//
// The ipc4 firmware only supports up to 8 sink or source pins
// per widget, because only 3 bits are used for queue(pin) ID
// in ipc4 protocol.
//
pub const SOF_WIDGET_MAX_NUM_PINS: c_int = 8;
// Widget pin type
pub const SOF_PIN_TYPE_INPUT: c_int = 0;
pub const SOF_PIN_TYPE_OUTPUT: c_int = 1;
// max number of FE PCMs before BEs
pub const SOF_BE_PCM_BASE: c_int = 16;
pub const DMA_CHAN_INVALID: c_uint = 0xFFFFFFFF;

pub const SOF_DAI_PARAM_INTEL_SSP_MCLK: c_int = 0;
pub const SOF_DAI_PARAM_INTEL_SSP_BCLK: c_int = 1;
pub const SOF_DAI_PARAM_INTEL_SSP_TDM_SLOTS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_widget_op {
    SOF_WIDGET_PREPARE,
    SOF_WIDGET_SETUP,
    SOF_WIDGET_FREE,
    SOF_WIDGET_UNPREPARE,
}

//
// Volume fractional word length define to 16 sets
// the volume linear gain value to use Qx.16 format
//
pub const VOLUME_FWL: c_int = 16;
pub const SOF_TLV_ITEMS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dai_config_data {
    pub dai_index: c_int,
    pub /: *mut *mut int dai_data; / contains DAI-specific information,
    pub /: *mut *mut int dai_node_id; / contains DAI-specific information for Gateway configuration,
}

//
// struct sof_ipc_pcm_ops - IPC-specific PCM ops
// @hw_params: Function pointer for hw_params
// @hw_free: Function pointer for hw_free
// @trigger: Function pointer for trigger
// @dai_link_fixup: Function pointer for DAI link fixup
// @pcm_setup: Function pointer for IPC-specific PCM set up that can be used for allocating
// additional memory in the SOF PCM stream structure
// @pcm_free: Function pointer for PCM free that can be used for freeing any
// additional memory in the SOF PCM stream structure
// @pointer: Function pointer for pcm pointer
// Note: the @pointer callback may return -EOPNOTSUPP which should be
// handled in a same way as if the callback is not provided
// @delay: Function pointer for pcm delay reporting
// @reset_hw_params_during_stop: Flag indicating whether the hw_params should be reset during the
// STOP pcm trigger
// @ipc_first_on_start: Send IPC before invoking platform trigger during
// START/PAUSE_RELEASE triggers
// @platform_stop_during_hw_free: Invoke the platform trigger during hw_free. This is needed for
// IPC4 where a pipeline is only paused during stop/pause/suspend
// triggers. The FW keeps the host DMA running in this case and
// therefore the host must do the same and should stop the DMA during
// hw_free.
// @d0i3_supported_in_s0ix: Allow DSP D0I3 during S0iX
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pcm_ops {
    pub platform_params): *mut snd_sof_platform_stream_params,
    pub substream): *mut *mut *mut int (hw_free)(struct snd_soc_component component, struct snd_pcm_substream,
    pub cmd): c_int,
    pub params): *mut *mut *mut int (dai_link_fixup)(struct snd_soc_pcm_runtime rtd, struct snd_pcm_hw_params,
    pub spcm): *mut *mut *mut int (pcm_setup)(struct snd_sof_dev sdev, struct snd_sof_pcm,
    pub spcm): *mut *mut *mut void (pcm_free)(struct snd_sof_dev sdev, struct snd_sof_pcm,
    pub pointer): *mut snd_pcm_uframes_t,
    pub substream): *mut snd_pcm_substream,
    pub reset_hw_params_during_stop: bool,
    pub ipc_first_on_start: bool,
    pub platform_stop_during_hw_free: bool,
    pub d0i3_supported_in_s0ix: bool,
}

//
// struct sof_ipc_tplg_control_ops - IPC-specific ops for topology kcontrol IO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_tplg_control_ops {
    pub ucontrol): *mut *mut *mut bool (volume_put)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (volume_get)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut bool (switch_put)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (switch_get)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut bool (enum_put)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (enum_get)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (bytes_put)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub ucontrol): *mut *mut *mut int (bytes_get)(struct snd_sof_control scontrol, struct snd_ctl_elem_value,
    pub size): *const *const unsigned int __user binary_data, unsigned int,
    pub size): *const *const unsigned int __user binary_data, unsigned int,
    pub size): *const *const unsigned int __user binary_data, unsigned int,
// update control data based on notification from the DSP
    pub ipc_control_message): *mut *mut *mut void (update)(struct snd_sof_dev sdev, void,
// Optional callback to setup kcontrols associated with an swidget
    pub swidget): *mut *mut *mut int (widget_kcontrol_setup)(struct snd_sof_dev sdev, struct snd_sof_widget,
// mandatory callback to set up volume table for volume kcontrols
    pub size): c_int,
}

//
// struct sof_ipc_tplg_widget_ops - IPC-specific ops for topology widgets
// @ipc_setup: Function pointer for setting up widget IPC params
// @ipc_free: Function pointer for freeing widget IPC params
// @token_list: List of token ID's that should be parsed for the widget
// @token_list_size: number of elements in token_list
// @bind_event: Function pointer for binding events to the widget
// @ipc_prepare: Optional op for preparing a widget for set up
// @ipc_unprepare: Optional op for unpreparing a widget
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_tplg_widget_ops {
    pub swidget): *mut *mut int (ipc_setup)(struct snd_sof_widget,
    pub swidget): *mut *mut void (ipc_free)(struct snd_sof_widget,
    pub token_list: *mut sof_tokens,
    pub token_list_size: c_int,
    pub event_type): u16,
    pub dir): *mut *mut snd_pcm_hw_params source_params, int,
    pub swidget): *mut *mut void (ipc_unprepare)(struct snd_sof_widget,
}

//
// struct sof_ipc_tplg_ops - IPC-specific topology ops
// @widget: Array of pointers to IPC-specific ops for widgets. This should always be of size
// SND_SOF_DAPM_TYPE_COUNT i.e one per widget type. Unsupported widget types will be
// initialized to 0.
// @control: Pointer to the IPC-specific ops for topology kcontrol IO
// @route_setup: Function pointer for setting up pipeline connections
// @route_free: Function pointer for freeing pipeline connections.
// @token_list: List of all tokens supported by the IPC version. The size of the token_list
// array should be SOF_TOKEN_COUNT. The unused elements in the array will be
// initialized to 0.
// @control_setup: Function pointer for setting up kcontrol IPC-specific data
// @control_free: Function pointer for freeing kcontrol IPC-specific data
// @pipeline_complete: Function pointer for pipeline complete IPC
// @widget_setup: Function pointer for setting up setup in the DSP
// @widget_free: Function pointer for freeing widget in the DSP
// @dai_config: Function pointer for sending DAI config IPC to the DSP
// @host_config: Function pointer for setting the DMA ID for host widgets
// @dai_get_param: Function pointer for getting the DAI parameter
// @set_up_all_pipelines: Function pointer for setting up all topology pipelines
// @tear_down_all_pipelines: Function pointer for tearing down all topology pipelines
// @parse_manifest: Function pointer for ipc4 specific parsing of topology manifest
// @link_setup: Function pointer for IPC-specific DAI link set up
//
// Note: function pointers (ops) are optional
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_tplg_ops {
    pub widget: *const sof_ipc_tplg_widget_ops,
    pub control: *const sof_ipc_tplg_control_ops,
    pub sroute): *mut *mut *mut int (route_setup)(struct snd_sof_dev sdev, struct snd_sof_route,
    pub sroute): *mut *mut *mut int (route_free)(struct snd_sof_dev sdev, struct snd_sof_route,
    pub token_list: *const sof_token_info,
    pub scontrol): *mut *mut *mut int (control_setup)(struct snd_sof_dev sdev, struct snd_sof_control,
    pub scontrol): *mut *mut *mut int (control_free)(struct snd_sof_dev sdev, struct snd_sof_control,
    pub swidget): *mut *mut *mut int (pipeline_complete)(struct snd_sof_dev sdev, struct snd_sof_widget,
    pub swidget): *mut *mut *mut int (widget_setup)(struct snd_sof_dev sdev, struct snd_sof_widget,
    pub swidget): *mut *mut *mut int (widget_free)(struct snd_sof_dev sdev, struct snd_sof_widget,
    pub data): *mut unsigned int flags, struct snd_sof_dai_config_data,
    pub platform_params): *mut snd_sof_platform_stream_params,
    pub param_type): *mut *mut *mut *mut int (dai_get_param)(struct snd_sof_dev sdev, struct snd_sof_dai dai, int,
    pub verify): *mut *mut *mut int (set_up_all_pipelines)(struct snd_sof_dev sdev, bool,
    pub verify): *mut *mut *mut int (tear_down_all_pipelines)(struct snd_sof_dev sdev, bool,
    pub man): *mut snd_soc_tplg_manifest,
    pub link): *mut *mut *mut int (link_setup)(struct snd_sof_dev sdev, struct snd_soc_dai_link,
}

// struct snd_sof_tuple - Tuple info
// @token:	Token ID
// @value:	union of a string or a u32 values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_tuple {
    pub token: u32,
    pub v: u32,
    pub s: *const c_char,
    pub value: },
}

//
// List of SOF token ID's. The order of ID's does not matter as token arrays are looked up based on
// the ID.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_tokens {
    SOF_PCM_TOKENS,
    SOF_PIPELINE_TOKENS,
    SOF_SCHED_TOKENS,
    SOF_ASRC_TOKENS,
    SOF_SRC_TOKENS,
    SOF_COMP_TOKENS,
    SOF_BUFFER_TOKENS,
    SOF_VOLUME_TOKENS,
    SOF_PROCESS_TOKENS,
    SOF_DAI_TOKENS,
    SOF_DAI_LINK_TOKENS,
    SOF_HDA_TOKENS,
    SOF_SSP_TOKENS,
    SOF_ALH_TOKENS,
    SOF_DMIC_TOKENS,
    SOF_DMIC_PDM_TOKENS,
    SOF_ESAI_TOKENS,
    SOF_SAI_TOKENS,
    SOF_AFE_TOKENS,
    SOF_CORE_TOKENS,
    SOF_COMP_EXT_TOKENS,
    SOF_IN_AUDIO_FORMAT_TOKENS,
    SOF_OUT_AUDIO_FORMAT_TOKENS,
    SOF_COPIER_DEEP_BUFFER_TOKENS,
    SOF_COPIER_TOKENS,
    SOF_AUDIO_FMT_NUM_TOKENS,
    SOF_COPIER_FORMAT_TOKENS,
    SOF_GAIN_TOKENS,
    SOF_ACPDMIC_TOKENS,
    SOF_ACPI2S_TOKENS,
    SOF_MICFIL_TOKENS,
    SOF_ACP_SDW_TOKENS,

// this should be the last
    SOF_TOKEN_COUNT,
}

//
// struct sof_topology_token - SOF topology token definition
// @token:		Token number
// @type:		Token type
// @get_token:		Function pointer to parse the token value and save it in a object
// @offset:		Offset within an object to save the token value into
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_topology_token {
    pub token: u32,
    pub type: u32,
    pub offset): *mut *mut *mut *mut int (get_token)(void elem, void object, u32,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_token_info {
    pub name: *const c_char,
    pub tokens: *const sof_topology_token,
    pub count: c_int,
}

//
// struct snd_sof_pcm_stream_pipeline_list - List of pipelines associated with a PCM stream
// @pipelines: array of pipelines
// @count: number of pipeline widgets in the @pipe_widgets array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_pcm_stream_pipeline_list {
    pub pipelines: *mut snd_sof_pipeline,
    pub count: u32,
}

// PCM stream, mapped to FW component
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_pcm_stream {
    pub comp_id: u32,
    pub page_table: snd_dma_buffer,
    pub posn: sof_ipc_stream_posn,
    pub substream: *mut snd_pcm_substream,
    pub cstream: *mut snd_compr_stream,
    pub period_elapsed_work: work_struct,
    pub /: *mut *mut *mut snd_soc_dapm_widget_list list; / list of connected DAPM widgets,
    pub /: *mut *mut bool d0i3_compatible; / DSP can be in D0I3 when this pcm is opened,
    pub /: *mut *mut bool pause_supported; / PCM device supports PAUSE operation,
    pub /: *mut *mut unsigned int dsp_max_burst_size_in_ms; / The maximum size of the host DMA burst in ms,
//
// flag to indicate that the DSP pipelines should be kept
// active or not while suspending the stream
//
    pub suspend_ignored: bool,
    pub pipeline_list: snd_sof_pcm_stream_pipeline_list,
// used by IPC implementation and core does not touch it
    pub private: *mut c_void,
}

// ALSA SOF PCM device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_pcm {
    pub scomp: *mut snd_soc_component,
    pub stream: [snd_sof_pcm_stream; 2],
    pub /: *mut *mut list_head list; / list in sdev pcm list,
    pub params: [snd_pcm_hw_params; 2],
    pub platform_params: [snd_sof_platform_stream_params; 2],
    pub /: *mut *mut bool prepared[2]; / PCM_PARAMS set successfully,
    pub /: *mut *mut bool setup_done[2]; / the setup of the SOF PCM device is done,
    pub /: *mut *mut bool pending_stop[2]; / only used if (!pcm_ops->platform_stop_during_hw_free),
// Must be last - ends in a flex-array member.
    pub pcm: snd_soc_tplg_pcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_led_control {
    pub use_led: c_uint,
    pub direction: c_uint,
    pub led_value: c_int,
}

// ALSA SOF Kcontrol device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_control {
    pub scomp: *mut snd_soc_component,
    pub name: *const c_char,
    pub comp_id: c_int,
    pub /: *mut *mut int min_volume_step; / min volume step for volume_table,
    pub /: *mut *mut int max_volume_step; / max volume step for volume_table,
    pub num_channels: c_int,
    pub access: c_uint,
    pub info_type: c_int,
    pub /: *mut *mut int index; / pipeline ID,
    pub /: *mut *mut *mut void priv; / private data copied from topology,
    pub /: *mut *mut size_t priv_size; / size of private data,
    pub max_size: usize,
    pub ipc_control_data: *mut c_void,
    pub old_ipc_control_data: *mut c_void,
    pub /: *mut *mut int max; / applicable to volume controls,
    pub /: *mut *mut u32 size; / cdata size,
    pub data*/: *mut *mut *mut u32 volume_table; / volume table computed from tlv,
    pub /: *mut *mut list_head list; / list in sdev control list,
    pub led_ctl: snd_sof_led_control,
// if true, the control's data needs to be updated from Firmware
    pub comp_data_dirty: bool,
}

// struct snd_sof_dai_link - DAI link info
// @tuples: array of parsed tuples
// @num_tuples: number of tuples in the tuples array
// @link: Pointer to snd_soc_dai_link
// @hw_configs: Pointer to hw configs in topology
// @num_hw_configs: Number of hw configs in topology
// @default_hw_cfg_id: Default hw config ID
// @type: DAI type
// @list: item in snd_sof_dev dai_link list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dai_link {
    pub tuples: *mut snd_sof_tuple,
    pub num_tuples: c_int,
    pub link: *mut snd_soc_dai_link,
    pub num_hw_configs: c_int,
    pub default_hw_cfg_id: c_int,
    pub type: c_int,
    pub list: list_head,
    pub __counted_by(num_hw_configs): snd_soc_tplg_hw_config hw_configs[],
}

// ASoC SOF DAPM widget
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_widget {
    pub scomp: *mut snd_soc_component,
    pub comp_id: c_int,
    pub pipeline_id: c_int,
//
// the prepared flag is used to indicate that a widget has been prepared for getting set
// up in the DSP.
//
    pub prepared: bool,
    pub /: *mut *mut mutex setup_mutex; / to protect the swidget setup and free operations,
//
// use_count is protected by the PCM mutex held by the core and the
// setup_mutex against non stream domain races (kcontrol access for
// example)
//
    pub use_count: c_int,
    pub core: c_int,
    pub /: *mut *mut int id; / id is the DAPM widget type,
//
// Instance ID is set dynamically when the widget gets set up in the FW. It should be
// unique for each module type across all pipelines. This will not be used in SOF_IPC.
//
    pub instance_id: c_int,
//
// Flag indicating if the widget should be set up dynamically when a PCM is opened.
// This flag is only set for the scheduler type widget in topology. During topology
// loading, this flag is propagated to all the widgets belonging to the same pipeline.
// When this flag is not set, a widget is set up at the time of topology loading
// and retained until the DSP enters D3. It will need to be set up again when resuming
// from D3.
//
    pub dynamic_pipeline_widget: bool,
// Scheduling domain (enum sof_comp_domain), unset, Low Latency, or Data Processing
    pub comp_domain: u32,
// Module instance's memory configuration.
    pub /: *mut *mut u32 domain_id; / Module instance's userspace domain ID,
    pub /: *mut *mut u32 stack_bytes; / Module instance's stack size requirement,
    pub /: *mut *mut u32 heap_bytes; / Module instance's heap size requirement,
    pub widget: *mut snd_soc_dapm_widget,
    pub /: *mut *mut list_head list; / list in sdev widget list,
    pub spipe: *mut snd_sof_pipeline,
    pub module_info: *mut c_void,
    pub uuid: guid_t,
    pub num_tuples: c_int,
    pub tuples: *mut snd_sof_tuple,
//
// The allowed range for num_input/output_pins is [0, SOF_WIDGET_MAX_NUM_PINS].
// Widgets may have zero input or output pins, for example the tone widget has
// zero input pins.
//
    pub num_input_pins: u32,
    pub num_output_pins: u32,
//
// The input/output pin binding array, it takes the form of
// [widget_name_connected_to_pin0, widget_name_connected_to_pin1, ...],
// with the index as the queue ID.
//
// The array is used for special pin binding. Note that even if there
// is only one input/output pin requires special pin binding, pin binding
// should be defined for all input/output pins in topology, for pin(s) that
// are not used, give the value "NotConnected".
//
// If pin binding is not defined in topology, nothing to parse in the kernel,
// input_pin_binding and output_pin_binding shall be NULL.
//
    pub input_pin_binding: *mut c_char,
    pub output_pin_binding: *mut c_char,
    pub output_queue_ida: ida,
    pub input_queue_ida: ida,
    pub /: *mut *mut *mut void private; / core does not touch this,
}

// struct snd_sof_pipeline - ASoC SOF pipeline
// @pipe_widget: Pointer to the pipeline widget
// @started_count: Count of number of PCM's that have started this pipeline
// @paused_count: Count of number of PCM's that have started and have currently paused this
// @complete: flag used to indicate that pipeline set up is complete.
// @core_mask: Mask containing target cores for all modules in the pipeline
// @list: List item in sdev pipeline_list
// @direction_valid: flag indicating if the direction is set in topology
// @direction: pipeline direction set in topology, valid is direction_valid is true
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_pipeline {
    pub pipe_widget: *mut snd_sof_widget,
    pub started_count: c_int,
    pub paused_count: c_int,
    pub complete: c_int,
    pub core_mask: c_ulong,
    pub list: list_head,
    pub direction_valid: bool,
    pub direction: u32,
}

// ASoC SOF DAPM route
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_route {
    pub scomp: *mut snd_soc_component,
    pub route: *mut snd_soc_dapm_route,
    pub /: *mut *mut list_head list; / list in sdev route list,
    pub src_widget: *mut snd_sof_widget,
    pub sink_widget: *mut snd_sof_widget,
    pub setup: bool,
    pub src_queue_id: c_int,
    pub dst_queue_id: c_int,
    pub private: *mut c_void,
}

// ASoC DAI device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dai {
    pub scomp: *mut snd_soc_component,
    pub name: *const c_char,
    pub type: u32,
    pub number_configs: c_int,
    pub current_config: c_int,
    pub /: *mut *mut list_head list; / list in sdev dai list,
// core should not touch this
    pub platform_private: *const c_void,
    pub private: *mut c_void,
}

//
// Kcontrols.
//
// Topology.
// There is no snd_sof_free_topology since topology components will
// be freed by snd_soc_unregister_component,
//
extern "C" {
    pub fn snd_sof_load_topology(scomp: *mut snd_soc_component, file: *const c_char) -> c_int;
}
//
// Stream IPC
//
extern "C" {
    pub fn snd_sof_pcm_period_elapsed(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_sof_pcm_init_elapsed_work(work: *mut work_struct);
}
//
// snd_sof_pcm specific wrappers for dev_dbg() and dev_err() to provide
// consistent and useful prints.
//

extern "C" {
    pub fn snd_sof_compr_fragment_elapsed(cstream: *mut snd_compr_stream);
}
extern "C" {
    pub fn snd_sof_compr_init_elapsed_work(work: *mut work_struct);
}

// DAI link fixup
extern "C" {
    pub fn sof_pcm_dai_link_fixup(rtd: *mut snd_soc_pcm_runtime, params: *mut snd_pcm_hw_params) -> c_int;
}
// PM
extern "C" {
    pub fn snd_sof_stream_suspend_ignored(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn snd_sof_dsp_only_d0i3_compatible_stream_active(sdev: *mut snd_sof_dev) -> bool;
}
// Machine driver enumeration
extern "C" {
    pub fn sof_machine_register(sdev: *mut snd_sof_dev, pdata: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sof_machine_unregister(sdev: *mut snd_sof_dev, pdata: *mut c_void);
}
extern "C" {
    pub fn sof_widget_setup(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int;
}
extern "C" {
    pub fn sof_widget_free(sdev: *mut snd_sof_dev, swidget: *mut snd_sof_widget) -> c_int;
}
// PCM
extern "C" {
    pub fn sof_widget_list_unprepare(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: c_int);
}
extern "C" {
    pub fn sof_widget_list_free(sdev: *mut snd_sof_dev, spcm: *mut snd_sof_pcm, dir: c_int) -> c_int;
}
extern "C" {
    pub fn sof_pcm_free_all_streams(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn get_token_u32(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn get_token_u16(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn get_token_comp_format(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn get_token_dai_type(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn get_token_uuid(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn get_token_string(elem: *mut c_void, object: *mut c_void, offset: u32) -> c_int;
}
extern "C" {
    pub fn vol_compute_gain(value: u32, tlv: *mut c_int) -> u32;
}
