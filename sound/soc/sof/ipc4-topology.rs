//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/ipc4-topology.h
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
// Copyright(c) 2022 Intel Corporation
//

//
// Two module schedule domains in fw :
// LL domain - Low latency domain
// DP domain - Data processing domain
// The LL setting should be equal to !DP setting
//

pub const SOF_IPC4_MODULE_INIT_CONFIG_TYPE_BASE_CFG: c_int = 0;
pub const SOF_IPC4_MODULE_INIT_CONFIG_TYPE_BASE_CFG_WITH_EXT: c_int = 1;
pub const SOF_IPC4_MODULE_INSTANCE_LIST_ITEM_SIZE: c_int = 12;
pub const SOF_IPC4_PIPELINE_OBJECT_SIZE: c_int = 448;
pub const SOF_IPC4_DATA_QUEUE_OBJECT_SIZE: c_int = 128;
pub const SOF_IPC4_LL_TASK_OBJECT_SIZE: c_int = 72;
pub const SOF_IPC4_DP_TASK_OBJECT_SIZE: c_int = 104;

pub const SOF_IPC4_LL_TASK_LIST_ITEM_SIZE: c_int = 12;
pub const SOF_IPC4_FW_MAX_PAGE_COUNT: c_int = 20;
pub const SOF_IPC4_FW_MAX_QUEUE_COUNT: c_int = 8;
// IPC4 sample types
pub const SOF_IPC4_TYPE_MSB_INTEGER: c_int = 0;
pub const SOF_IPC4_TYPE_LSB_INTEGER: c_int = 1;
pub const SOF_IPC4_TYPE_SIGNED_INTEGER: c_int = 2;
pub const SOF_IPC4_TYPE_UNSIGNED_INTEGER: c_int = 3;
pub const SOF_IPC4_TYPE_FLOAT: c_int = 4;
pub const SOF_IPC4_TYPE_A_LAW: c_int = 5;
pub const SOF_IPC4_TYPE_MU_LAW: c_int = 6;
// Node index and mask applicable for host copier and ALH/HDA type DAI copiers
pub const SOF_IPC4_NODE_INDEX_MASK: c_uint = 0xFF;

// Node ID for SSP type DAI copiers

// Node ID for DMIC type DAI copiers

pub const SOF_IPC4_GAIN_ALL_CHANNELS_MASK: c_uint = 0xffffffff;
pub const SOF_IPC4_VOL_ZERO_DB: c_uint = 0x7fffffff;
pub const SOF_IPC4_DMA_DEVICE_MAX_COUNT: c_int = 16;
pub const SOF_IPC4_CHAIN_DMA_NODE_ID: c_uint = 0x7fffffff;
pub const SOF_IPC4_INVALID_NODE_ID: c_uint = 0xffffffff;
// FW requires minimum 4ms DMA buffer size
pub const SOF_IPC4_MIN_DMA_BUFFER_SIZE: c_int = 4;
// ChainDMA in fw uses 5ms DMA buffer
pub const SOF_IPC4_CHAIN_DMA_BUFFER_SIZE: c_int = 5;
//
// The base of multi-gateways. Multi-gateways addressing starts from
// ALH_MULTI_GTW_BASE and there are ALH_MULTI_GTW_COUNT multi-sources
// and ALH_MULTI_GTW_COUNT multi-sinks available.
// Addressing is continuous from ALH_MULTI_GTW_BASE to
// ALH_MULTI_GTW_BASE + ALH_MULTI_GTW_COUNT - 1.
//
pub const ALH_MULTI_GTW_BASE: c_uint = 0x50;
// A magic number from FW
pub const ALH_MULTI_GTW_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_ipc4_copier_module_config_params {
//
// Use LARGE_CONFIG_SET to initialize timestamp event. Ipc mailbox must
// contain properly built CopierConfigTimestampInitData struct.
//
    SOF_IPC4_COPIER_MODULE_CFG_PARAM_TIMESTAMP_INIT = 1,
//
// Use LARGE_CONFIG_SET to initialize copier sink. Ipc mailbox must contain
// properly built CopierConfigSetSinkFormat struct.
//
    SOF_IPC4_COPIER_MODULE_CFG_PARAM_SET_SINK_FORMAT,
//
// Use LARGE_CONFIG_SET to initialize and enable on Copier data segment
// event. Ipc mailbox must contain properly built DataSegmentEnabled struct.
//
    SOF_IPC4_COPIER_MODULE_CFG_PARAM_DATA_SEGMENT_ENABLED,
//
// Use LARGE_CONFIG_GET to retrieve Linear Link Position (LLP) value for non
// HD-A gateways.
//
    SOF_IPC4_COPIER_MODULE_CFG_PARAM_LLP_READING,
//
// Use LARGE_CONFIG_GET to retrieve Linear Link Position (LLP) value for non
// HD-A gateways and corresponding total processed data
//
    SOF_IPC4_COPIER_MODULE_CFG_PARAM_LLP_READING_EXTENDED,
//
// Use LARGE_CONFIG_SET to setup attenuation on output pins. Data is just uint32_t.
// note Config is only allowed when output pin is set up for 32bit and source
// is connected to Gateway
//
    SOF_IPC4_COPIER_MODULE_CFG_ATTENUATION,
}

// Scheduling domain, unset, Low Latency, or Data Processing
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_comp_domain {
    SOF_COMP_DOMAIN_UNSET = 0,	/* Take domain value from manifest */
    SOF_COMP_DOMAIN_LL,		/* Low Latency scheduling domain */
    SOF_COMP_DOMAIN_DP,		/* Data Processing scheduling domain */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_copier_config_set_sink_format {
// Id of sink
    pub sink_id: u32,
//
// Input format used by the source
// attention must be the same as present if already initialized.
//
    pub source_fmt: sof_ipc4_audio_format,
// Output format used by the sink
    pub sink_fmt: sof_ipc4_audio_format,
    pub __aligned(4): } __packed,
//
// struct sof_ipc4_pipeline - pipeline config data
// @priority: Priority of this pipeline
// @lp_mode: Low power mode
// @mem_usage: Memory usage
// @core_id: Target core for the pipeline
// @state: Pipeline state
// @use_chain_dma: flag to indicate if the firmware shall use chained DMA
// @msg: message structure for pipeline
// @skip_during_fe_trigger: skip triggering this pipeline during the FE DAI trigger
// @direction_valid: flag indicating if valid direction is set in topology
// @direction: pipeline direction set in topology if direction_valid is true
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_pipeline {
    pub priority: u32,
    pub lp_mode: u32,
    pub mem_usage: u32,
    pub core_id: u32,
    pub state: c_int,
    pub use_chain_dma: bool,
    pub msg: sof_ipc4_msg,
    pub skip_during_fe_trigger: bool,
    pub direction_valid: bool,
    pub direction: u32,
}

//
// struct ipc4_pipeline_set_state_data - multi pipeline trigger IPC data
// @count: Number of pipelines to be triggered
// @pipeline_instance_ids: Flexible array of IDs of the pipelines to be triggered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc4_pipeline_set_state_data {
    pub count: u32,
    pub pipeline_instance_ids): DECLARE_FLEX_ARRAY(u32,,
    pub __packed: },
//
// struct sof_ipc4_pin_format - Module pin format
// @pin_index: pin index
// @buffer_size: buffer size in bytes
// @audio_fmt: audio format for the pin
//
// This structure can be used for both output or input pins and the pin_index is relative to the
// pin type i.e output/input pin
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_pin_format {
    pub pin_index: u32,
    pub buffer_size: u32,
    pub audio_fmt: sof_ipc4_audio_format,
}

//
// struct sof_ipc4_available_audio_format - Available audio formats
// @output_pin_fmts: Available output pin formats
// @input_pin_fmts: Available input pin formats
// @num_input_formats: Number of input pin formats
// @num_output_formats: Number of output pin formats
// @changed_params: Mask of changed params by the module instance between it's
// input and output formts (rate, channels, depth)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_available_audio_format {
    pub output_pin_fmts: *mut sof_ipc4_pin_format,
    pub input_pin_fmts: *mut sof_ipc4_pin_format,
    pub num_input_formats: u32,
    pub num_output_formats: u32,
    pub changed_params: u32,
}

//
// struct sof_copier_gateway_cfg - IPC gateway configuration
// @node_id: ID of Gateway Node
// @dma_buffer_size: Preferred Gateway DMA buffer size (in bytes)
// @config_length: Length of gateway node configuration blob specified in #config_data
// @config_data: Gateway node configuration blob
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_copier_gateway_cfg {
    pub node_id: u32,
    pub dma_buffer_size: u32,
    pub config_length: u32,
    pub config_data: [u32; ],
}

//
// struct sof_ipc4_copier_data - IPC data for copier
// @base_config: Base configuration including input audio format
// @out_format: Output audio format
// @copier_feature_mask: Copier feature mask
// @gtw_cfg: Gateway configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_copier_data {
    pub base_config: sof_ipc4_base_module_cfg,
    pub out_format: sof_ipc4_audio_format,
    pub copier_feature_mask: u32,
    pub gtw_cfg: sof_copier_gateway_cfg,
}

//
// struct sof_ipc4_gtw_attributes: Gateway attributes
// @lp_buffer_alloc: Gateway data requested in low power memory
// @alloc_from_reg_file: Gateway data requested in register file memory
// @rsvd: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_gtw_attributes {
    pub 1: uint32_t lp_buffer_alloc :,
    pub 1: uint32_t alloc_from_reg_file :,
    pub 30: uint32_t rsvd :,
}

//
// struct sof_ipc4_dma_device_stream_ch_map: abstract representation of
// channel mapping to DMAs
// @device: representation of hardware device address or FIFO
// @channel_mask: channels handled by @device. Channels are expected to be
// contiguous
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_dma_device_stream_ch_map {
    pub device: u32,
    pub channel_mask: u32,
}

//
// struct sof_ipc4_dma_stream_ch_map: DMA configuration data
// @device_count: Number valid items in mapping array
// @mapping: device address and channel mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_dma_stream_ch_map {
    pub device_count: u32,
    pub mapping: [sof_ipc4_dma_device_stream_ch_map; SOF_IPC4_DMA_DEVICE_MAX_COUNT],
    pub __packed: },
pub const SOF_IPC4_DMA_METHOD_HDA: c_int = 1;

pub const SOF_IPC4_CHAIN_DMA_BUF_SIZE_MS: c_int = 2;
//
// struct sof_ipc4_dma_config: DMA configuration
// @dma_method: HDAudio or GPDMA
// @pre_allocated_by_host: 1 if host driver allocates DMA channels, 0 otherwise
// @dma_channel_id: for HDaudio defined as @stream_id - 1
// @stream_id: HDaudio stream tag
// @dma_stream_channel_map: array of device/channel mappings
// @dma_priv_config_size: currently not used
// @dma_priv_config: currently not used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_dma_config {
    pub dma_method: u8,
    pub pre_allocated_by_host: u8,
// private:
    pub rsvd: u16,
// public:
    pub dma_channel_id: u32,
    pub stream_id: u32,
    pub dma_stream_channel_map: sof_ipc4_dma_stream_ch_map,
    pub dma_priv_config_size: u32,
    pub dma_priv_config: [u8; ],
    pub __packed: },
pub const SOF_IPC4_GTW_DMA_CONFIG_ID: c_uint = 0x1000;
//
// struct sof_ipc4_dma_config_tlv - DMA configuration
// @type: set to SOF_IPC4_GTW_DMA_CONFIG_ID
// @length: sizeof(struct sof_ipc4_dma_config) + dma_config.dma_priv_config_size
// @dma_config: actual DMA configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_dma_config_tlv {
    pub type: u32,
    pub length: u32,
    pub dma_config: sof_ipc4_dma_config,
    pub __packed: },
// struct sof_ipc4_alh_configuration_blob: ALH blob
// @gw_attr: Gateway attributes
// @alh_cfg: ALH configuration data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_alh_configuration_blob {
    pub gw_attr: sof_ipc4_gtw_attributes,
    pub alh_cfg: sof_ipc4_dma_stream_ch_map,
}

//
// struct sof_ipc4_copier - copier config data
// @data: IPC copier data
// @copier_config: Copier + blob
// @ipc_config_size: Size of copier_config
// @ipc_config_data: Copier module config data
// @available_fmt: Available audio format
// @frame_fmt: frame format
// @msg: message structure for copier
// @gtw_attr: Gateway attributes for copier blob
// @dai_type: DAI type
// @dai_index: DAI index
// @dma_config_tlv: DMA configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_copier {
    pub data: sof_ipc4_copier_data,
    pub copier_config: *mut u32,
    pub ipc_config_size: u32,
    pub ipc_config_data: *mut c_void,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub frame_fmt: u32,
    pub msg: sof_ipc4_msg,
    pub gtw_attr: *mut sof_ipc4_gtw_attributes,
    pub dai_type: u32,
    pub dai_index: c_int,
    pub dma_config_tlv: [sof_ipc4_dma_config_tlv; SOF_IPC4_DMA_DEVICE_MAX_COUNT],
}

//
// struct sof_ipc4_ctrl_value_chan: generic channel mapped value data
// @channel: Channel ID
// @value: Value associated with @channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_ctrl_value_chan {
    pub channel: u32,
    pub value: u32,
}

//
// struct sof_ipc4_control_data - IPC data for kcontrol IO
// @msg: message structure for kcontrol IO
// @index: pipeline ID
// @chanv: channel ID and value array used by volume type controls
// @data: data for binary kcontrols
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_control_data {
    pub msg: sof_ipc4_msg,
    pub index: c_int,
    pub chanv): DECLARE_FLEX_ARRAY(struct sof_ipc4_ctrl_value_chan,,
    pub data): DECLARE_FLEX_ARRAY(struct sof_abi_hdr,,
}

pub const SOF_IPC4_SWITCH_CONTROL_PARAM_ID: c_int = 200;
pub const SOF_IPC4_ENUM_CONTROL_PARAM_ID: c_int = 201;
pub const SOF_IPC4_BYTES_CONTROL_PARAM_ID: c_int = 202;
//
// struct sof_ipc4_control_msg_payload - IPC payload for kcontrol parameters
// @id: unique id of the control
// @num_elems: Number of elements in the chanv array or number of bytes in data
// @reserved: reserved for future use, must be set to 0
// @chanv: channel ID and value array
// @data: binary payload
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_control_msg_payload {
    pub id: u16,
    pub num_elems: u16,
    pub reserved: [u32; 4],
    pub chanv): DECLARE_FLEX_ARRAY(struct sof_ipc4_ctrl_value_chan,,
    pub data): DECLARE_FLEX_ARRAY(uint8_t,,
}

//
// struct sof_ipc4_gain_params - IPC gain parameters
// @channels: Channels
// @init_val: Initial value
// @curve_type: Curve type
// @reserved: reserved for future use
// @curve_duration_l: Curve duration low part
// @curve_duration_h: Curve duration high part
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_gain_params {
    pub channels: u32,
    pub init_val: u32,
    pub curve_type: u32,
    pub reserved: u32,
    pub curve_duration_l: u32,
    pub curve_duration_h: u32,
    pub __aligned(4): } __packed,
//
// struct sof_ipc4_gain_data - IPC gain init blob
// @base_config: IPC base config data
// @params: Initial parameters for the gain module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_gain_data {
    pub base_config: sof_ipc4_base_module_cfg,
    pub params: sof_ipc4_gain_params,
    pub __aligned(4): } __packed,
//
// struct sof_ipc4_gain - gain config data
// @data: IPC gain blob
// @available_fmt: Available audio format
// @msg: message structure for gain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_gain {
    pub data: sof_ipc4_gain_data,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub msg: sof_ipc4_msg,
}

//
// struct sof_ipc4_mixer - mixer config data
// @base_config: IPC base config data
// @available_fmt: Available audio format
// @msg: IPC4 message struct containing header and data info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_mixer {
    pub base_config: sof_ipc4_base_module_cfg,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub msg: sof_ipc4_msg,
}

//
// struct sof_ipc4_src_data - IPC data for SRC
// @base_config: IPC base config data
// @sink_rate: Output rate for sink module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_src_data {
    pub base_config: sof_ipc4_base_module_cfg,
    pub sink_rate: u32,
    pub __aligned(4): } __packed,
//
// struct sof_ipc4_src - SRC config data
// @data: IPC base config data
// @available_fmt: Available audio format
// @msg: IPC4 message struct containing header and data info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_src {
    pub data: sof_ipc4_src_data,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub msg: sof_ipc4_msg,
}

//
// struct sof_ipc4_asrc_data - IPC data for ASRC
// @base_config: IPC base config data
// @out_freq: Output rate for sink module, passed as such from topology to FW.
// @asrc_mode: Control for ASRC features with bit-fields, passed as such from topolgy to FW.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_asrc_data {
    pub base_config: sof_ipc4_base_module_cfg,
    pub out_freq: u32,
    pub asrc_mode: u32,
    pub __aligned(4): } __packed,
//
// struct sof_ipc4_asrc - ASRC config data
// @data: IPC base config data
// @available_fmt: Available audio format
// @msg: IPC4 message struct containing header and data info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_asrc {
    pub data: sof_ipc4_asrc_data,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub msg: sof_ipc4_msg,
}

//
// struct sof_ipc4_base_module_cfg_ext - base module config extension containing the pin format
// information for the module. Both @num_input_pin_fmts and @num_output_pin_fmts cannot be 0 for a
// module.
// @num_input_pin_fmts: number of input pin formats in the @pin_formats array
// @num_output_pin_fmts: number of output pin formats in the @pin_formats array
// @reserved: reserved for future use
// @pin_formats: flexible array consisting of @num_input_pin_fmts input pin format items followed
// by @num_output_pin_fmts output pin format items
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_base_module_cfg_ext {
    pub num_input_pin_fmts: u16,
    pub num_output_pin_fmts: u16,
    pub reserved: [u8; 12],
    pub pin_formats): DECLARE_FLEX_ARRAY(struct sof_ipc4_pin_format,,
    pub __packed: },
//
// struct sof_ipc4_process - process config data
// @base_config: IPC base config data
// @base_config_ext: Base config extension data for module init
// @output_format: Output audio format
// @available_fmt: Available audio format
// @ipc_config_data: Process module config data
// @ipc_config_size: Size of process module config data
// @msg: IPC4 message struct containing header and data info
// @base_config_ext_size: Size of the base config extension data in bytes
// @init_config: Module init config type (SOF_IPC4_MODULE_INIT_CONFIG_TYPE_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc4_process {
    pub base_config: sof_ipc4_base_module_cfg,
    pub base_config_ext: *mut sof_ipc4_base_module_cfg_ext,
    pub output_format: sof_ipc4_audio_format,
    pub available_fmt: sof_ipc4_available_audio_format,
    pub ipc_config_data: *mut c_void,
    pub ipc_config_size: u32,
    pub msg: sof_ipc4_msg,
    pub base_config_ext_size: u32,
    pub init_config: u32,
}
