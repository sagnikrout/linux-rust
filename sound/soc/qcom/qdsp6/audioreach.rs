//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/qdsp6/audioreach.h
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

// Module IDs
pub const MODULE_ID_WR_SHARED_MEM_EP: c_uint = 0x07001000;
pub const MODULE_ID_RD_SHARED_MEM_EP: c_uint = 0x07001001;
pub const MODULE_ID_GAIN: c_uint = 0x07001002;
pub const MODULE_ID_PCM_CNV: c_uint = 0x07001003;
pub const MODULE_ID_PCM_ENC: c_uint = 0x07001004;
pub const MODULE_ID_PCM_DEC: c_uint = 0x07001005;
pub const MODULE_ID_SH_MEM_PULL_MODE: c_uint = 0x07001006;
pub const MODULE_ID_SH_MEM_PUSH_MODE: c_uint = 0x07001007;
pub const MODULE_ID_PLACEHOLDER_ENCODER: c_uint = 0x07001008;
pub const MODULE_ID_PLACEHOLDER_DECODER: c_uint = 0x07001009;
pub const MODULE_ID_I2S_SINK: c_uint = 0x0700100A;
pub const MODULE_ID_I2S_SOURCE: c_uint = 0x0700100B;
pub const MODULE_ID_SAL: c_uint = 0x07001010;
pub const MODULE_ID_MFC: c_uint = 0x07001015;
pub const MODULE_ID_DATA_LOGGING: c_uint = 0x0700101A;
pub const MODULE_ID_AAC_DEC: c_uint = 0x0700101F;
pub const MODULE_ID_CODEC_DMA_SINK: c_uint = 0x07001023;
pub const MODULE_ID_CODEC_DMA_SOURCE: c_uint = 0x07001024;
pub const MODULE_ID_FLAC_DEC: c_uint = 0x0700102F;
pub const MODULE_ID_SMECNS_V2: c_uint = 0x07001031;
pub const MODULE_ID_MP3_DECODE: c_uint = 0x0700103B;
pub const MODULE_ID_GAPLESS: c_uint = 0x0700104D;
pub const MODULE_ID_DISPLAY_PORT_SINK: c_uint = 0x07001069;
pub const MODULE_ID_SPEAKER_PROTECTION: c_uint = 0x070010E2;
pub const MODULE_ID_SPEAKER_PROTECTION_VI: c_uint = 0x070010E3;
pub const MODULE_ID_OPUS_DEC: c_uint = 0x07001174;
pub const MODULE_ID_AUDIO_IF_SINK: c_uint = 0x0700117C;
pub const MODULE_ID_AUDIO_IF_SOURCE: c_uint = 0x0700117D;
pub const APM_CMD_GET_SPF_STATE: c_uint = 0x01001021;
pub const APM_CMD_RSP_GET_SPF_STATE: c_uint = 0x02001007;
pub const APM_MODULE_INSTANCE_ID: c_uint = 0x00000001;
pub const PRM_MODULE_INSTANCE_ID: c_uint = 0x00000002;
pub const AMDB_MODULE_INSTANCE_ID: c_uint = 0x00000003;
pub const VCPM_MODULE_INSTANCE_ID: c_uint = 0x00000004;
pub const AR_MODULE_INSTANCE_ID_START: c_uint = 0x00006000;
pub const AR_MODULE_INSTANCE_ID_END: c_uint = 0x00007000;
pub const AR_MODULE_DYNAMIC_INSTANCE_ID_START: c_uint = 0x00007000;
pub const AR_MODULE_DYNAMIC_INSTANCE_ID_END: c_uint = 0x00008000;
pub const AR_CONT_INSTANCE_ID_START: c_uint = 0x00005000;
pub const AR_CONT_INSTANCE_ID_END: c_uint = 0x00006000;
pub const AR_SG_INSTANCE_ID_START: c_uint = 0x00004000;
pub const APM_CMD_GRAPH_OPEN: c_uint = 0x01001000;
pub const APM_CMD_GRAPH_PREPARE: c_uint = 0x01001001;
pub const APM_CMD_GRAPH_START: c_uint = 0x01001002;
pub const APM_CMD_GRAPH_STOP: c_uint = 0x01001003;
pub const APM_CMD_GRAPH_CLOSE: c_uint = 0x01001004;
pub const APM_CMD_GRAPH_FLUSH: c_uint = 0x01001005;
pub const APM_CMD_SET_CFG: c_uint = 0x01001006;
pub const APM_CMD_GET_CFG: c_uint = 0x01001007;
pub const APM_CMD_SHARED_MEM_MAP_REGIONS: c_uint = 0x0100100C;
pub const APM_CMD_SHARED_MEM_UNMAP_REGIONS: c_uint = 0x0100100D;
pub const APM_CMD_REGISTER_MODULE_EVENTS: c_uint = 0x0100100E;
pub const APM_EVENT_MODULE_TO_CLIENT: c_uint = 0x03001000;
pub const APM_CMD_RSP_SHARED_MEM_MAP_REGIONS: c_uint = 0x02001001;

pub const APM_MMAP_TOKEN_MAP_TYPE_SHIFT: c_int = 16;
pub const APM_CMD_RSP_GET_CFG: c_uint = 0x02001000;
pub const APM_CMD_CLOSE_ALL: c_uint = 0x01001013;
pub const APM_CMD_REGISTER_SHARED_CFG: c_uint = 0x0100100A;
pub const EVENT_ID_SH_MEM_PULL_PUSH_MODE_WATERMARK: c_uint = 0x0800101C;
//
// struct event_cfg_sh_mem_pull_push_mode_watermark_t - Watermark config
// @num_water_mark_levels: Number of watermark levels.
// @level: Watermark levels.
//
// If @num_water_mark_levels is zero, no watermark levels are specified
// and watermark events are not supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_cfg_sh_mem_pull_push_mode_watermark_t {
    pub num_water_mark_levels: u32,
    pub level: [u32; ],
    pub __packed: },
//
// struct apm_module_register_events - Register or unregister module events
// @module_instance_id: Module instance identifier.
// @event_id: Module event identifier.
// @is_register: 1 to register the event, 0 to unregister it.
// @error_code: Error code for out-of-band command mode.
// @event_config_payload_size: Event configuration payload size in bytes.
// @reserved: Reserved for alignment; must be zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_register_events {
    pub module_instance_id: u32,
    pub event_id: u32,
    pub is_register: u32,
    pub error_code: u32,
    pub event_config_payload_size: u32,
    pub reserved: u32,
    pub __packed: },
//
// struct apm_module_event - Module event descriptor
// @event_id: Module event identifier.
// @event_payload_size: Event payload size in bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_event {
    pub event_id: u32,
    pub event_payload_size: u32,
    pub __packed: },
pub const APM_MEMORY_MAP_SHMEM8_4K_POOL: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cmd_shared_mem_map_regions {
    pub mem_pool_id: u16,
    pub num_regions: u16,
    pub property_flag: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_shared_map_region_payload {
    pub shm_addr_lsw: u32,
    pub shm_addr_msw: u32,
    pub mem_size_bytes: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cmd_shared_mem_unmap_regions {
    pub mem_map_handle: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cmd_rsp_shared_mem_map_regions {
    pub mem_map_handle: u32,
    pub __packed: },
// APM module
pub const APM_PARAM_ID_SUB_GRAPH_LIST: c_uint = 0x08001005;
pub const APM_PARAM_ID_MODULE_LIST: c_uint = 0x08001002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_param_id_modules_list {
    pub num_modules_list: u32,
    pub __packed: },
pub const APM_PARAM_ID_MODULE_PROP: c_uint = 0x08001003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_param_id_module_prop {
    pub num_modules_prop_cfg: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_prop_cfg {
    pub instance_id: u32,
    pub num_props: u32,
    pub __packed: },
pub const APM_PARAM_ID_MODULE_CONN: c_uint = 0x08001004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_param_id_module_conn {
    pub num_connections: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_conn_obj {
    pub src_mod_inst_id: u32,
    pub src_mod_op_port_id: u32,
    pub dst_mod_inst_id: u32,
    pub dst_mod_ip_port_id: u32,
    pub __packed: },
pub const APM_PARAM_ID_GAIN: c_uint = 0x08001006;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_gain_cfg {
    pub gain: u16,
    pub reserved: u16,
    pub __packed: },
pub const PARAM_ID_PCM_OUTPUT_FORMAT_CFG: c_uint = 0x08001008;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_pcm_output_format_cfg {
    pub data_format: u32,
    pub fmt_id: u32,
    pub payload_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct payload_pcm_output_format_cfg {
    pub bit_width: u16,
    pub alignment: u16,
    pub bits_per_sample: u16,
    pub q_factor: u16,
    pub endianness: u16,
    pub interleaved: u16,
    pub reserved: u16,
    pub num_channels: u16,
    pub channel_mapping: [u8; ],
    pub __packed: },
pub const PARAM_ID_ENC_BITRATE: c_uint = 0x08001052;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_enc_bitrate_param {
    pub bitrate: u32,
    pub __packed: },
pub const DATA_FORMAT_FIXED_POINT: c_int = 1;
pub const DATA_FORMAT_GENERIC_COMPRESSED: c_int = 5;
pub const DATA_FORMAT_RAW_COMPRESSED: c_int = 6;
pub const PCM_LSB_ALIGNED: c_int = 1;
pub const PCM_MSB_ALIGNED: c_int = 2;
pub const PCM_LITTLE_ENDIAN: c_int = 1;
pub const PCM_BIT_ENDIAN: c_int = 2;
pub const MEDIA_FMT_ID_PCM: c_uint = 0x09001000;
pub const MEDIA_FMT_ID_MP3: c_uint = 0x09001009;
pub const SAMPLE_RATE_48K: c_int = 48000;
pub const BIT_WIDTH_16: c_int = 16;
pub const APM_PARAM_ID_PROP_PORT_INFO: c_uint = 0x08001015;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_modules_prop_info {
    pub max_ip_port: u32,
    pub max_op_port: u32,
    pub __packed: },
// Shared memory module
pub const DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER: c_uint = 0x04001000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_data_cmd_wr_sh_mem_ep_data_buffer {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub buf_size: u32,
    pub timestamp_lsw: u32,
    pub timestamp_msw: u32,
    pub flags: u32,
    pub __packed: },
pub const DATA_CMD_WR_SH_MEM_EP_DATA_BUFFER_V2: c_uint = 0x0400100A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_data_cmd_wr_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub buf_size: u32,
    pub timestamp_lsw: u32,
    pub timestamp_msw: u32,
    pub flags: u32,
    pub md_addr_lsw: u32,
    pub md_addr_msw: u32,
    pub md_map_handle: u32,
    pub md_buf_size: u32,
    pub __packed: },
pub const DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE: c_uint = 0x05001000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rsp_wr_sh_mem_ep_data_buffer_done {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub status: u32,
    pub __packed: },
pub const DATA_CMD_RSP_WR_SH_MEM_EP_DATA_BUFFER_DONE_V2: c_uint = 0x05001004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rsp_wr_sh_mem_ep_data_buffer_done_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub status: u32,
    pub md_buf_addr_lsw: u32,
    pub md_buf_addr_msw: u32,
    pub md_mem_map_handle: u32,
    pub md_status: u32,
    pub __packed: },
pub const PARAM_ID_MEDIA_FORMAT: c_uint = 0x0800100C;
pub const DATA_CMD_WR_SH_MEM_EP_MEDIA_FORMAT: c_uint = 0x04001001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_media_format {
    pub data_format: u32,
    pub fmt_id: u32,
    pub payload_size: u32,
    pub __packed: },
pub const MEDIA_FMT_ID_FLAC: c_uint = 0x09001004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct payload_media_fmt_flac_t {
    pub num_channels: u16,
    pub sample_size: u16,
    pub min_blk_size: u16,
    pub max_blk_size: u16,
    pub sample_rate: u32,
    pub min_frame_size: u32,
    pub max_frame_size: u32,
    pub __packed: },
pub const MEDIA_FMT_ID_AAC: c_uint = 0x09001001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct payload_media_fmt_aac_t {
    pub aac_fmt_flag: u16,
    pub audio_obj_type: u16,
    pub num_channels: u16,
    pub total_size_of_PCE_bits: u16,
    pub sample_rate: u32,
    pub __packed: },
pub const MEDIA_FMT_ID_OPUS: c_uint = 0x09001039;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct payload_media_fmt_opus_t {
    pub bitstream_format: u16,
    pub payload_type: u16,
    pub version: u8,
    pub num_channels: u8,
    pub pre_skip: u16,
    pub sample_rate: u32,
    pub output_gain: u16,
    pub mapping_family: u8,
    pub stream_count: u8,
    pub coupled_count: u8,
    pub channel_mapping: [u8; 8],
    pub reserved: [u8; 3],
    pub __packed: },
pub const DATA_CMD_WR_SH_MEM_EP_EOS: c_uint = 0x04001002;
pub const WR_SH_MEM_EP_EOS_POLICY_LAST: c_int = 1;
pub const WR_SH_MEM_EP_EOS_POLICY_EACH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_wr_sh_mem_ep_eos {
    pub policy: u32,
    pub __packed: },
pub const DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER: c_uint = 0x04001003;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rd_sh_mem_ep_data_buffer {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub buf_size: u32,
    pub __packed: },
pub const DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER: c_uint = 0x05001002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rsp_rd_sh_mem_ep_data_buffer_done {
    pub status: u32,
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub data_size: u32,
    pub offset: u32,
    pub timestamp_lsw: u32,
    pub timestamp_msw: u32,
    pub flags: u32,
    pub num_frames: u32,
    pub __packed: },
pub const DATA_CMD_RD_SH_MEM_EP_DATA_BUFFER_V2: c_uint = 0x0400100B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rd_sh_mem_ep_data_buffer_v2 {
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub buf_size: u32,
    pub md_buf_addr_lsw: u32,
    pub md_buf_addr_msw: u32,
    pub md_mem_map_handle: u32,
    pub md_buf_size: u32,
    pub __packed: },
pub const DATA_CMD_RSP_RD_SH_MEM_EP_DATA_BUFFER_V2: c_uint = 0x05001005;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_rsp_rd_sh_mem_ep_data_buffer_done_v2 {
    pub status: u32,
    pub buf_addr_lsw: u32,
    pub buf_addr_msw: u32,
    pub mem_map_handle: u32,
    pub data_size: u32,
    pub offset: u32,
    pub timestamp_lsw: u32,
    pub timestamp_msw: u32,
    pub flags: u32,
    pub num_frames: u32,
    pub md_status: u32,
    pub md_buf_addr_lsw: u32,
    pub md_buf_addr_msw: u32,
    pub md_mem_map_handle: u32,
    pub md_size: u32,
    pub __packed: },
pub const PARAM_ID_RD_SH_MEM_CFG: c_uint = 0x08001007;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_rd_sh_mem_cfg {
    pub num_frames_per_buffer: u32,
    pub metadata_control_flags: u32,
    pub __packed: },
pub const DATA_CMD_WR_SH_MEM_EP_EOS_RENDERED: c_uint = 0x05001001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_cmd_wr_sh_mem_ep_eos_rendered {
    pub module_instance_id: u32,
    pub render_status: u32,
    pub __packed: },
pub const MODULE_ID_WR_SHARED_MEM_EP: c_uint = 0x07001000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cmd_header {
    pub payload_address_lsw: u32,
    pub payload_address_msw: u32,
    pub mem_map_handle: u32,
    pub payload_size: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_param_data {
    pub module_instance_id: u32,
    pub param_id: u32,
    pub param_size: u32,
    pub error_code: u32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_param_shared_data {
    pub param_id: u32,
    pub param_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_prop_data {
    pub prop_id: u32,
    pub prop_size: u32,
    pub __packed: },
// Sub-Graph Properties
pub const APM_PARAM_ID_SUB_GRAPH_CONFIG: c_uint = 0x08001001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_param_id_sub_graph_cfg {
    pub num_sub_graphs: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_sub_graph_cfg {
    pub sub_graph_id: u32,
    pub num_sub_graph_prop: u32,
    pub __packed: },
pub const APM_SUB_GRAPH_PROP_ID_PERF_MODE: c_uint = 0x0800100E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_sg_prop_id_perf_mode {
    pub perf_mode: u32,
    pub __packed: },
pub const APM_SG_PROP_ID_PERF_MODE_SIZE: c_int = 4;
pub const APM_SUB_GRAPH_PROP_ID_DIRECTION: c_uint = 0x0800100F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_sg_prop_id_direction {
    pub direction: u32,
    pub __packed: },
pub const APM_SG_PROP_ID_DIR_SIZE: c_int = 4;
pub const APM_SUB_GRAPH_PROP_ID_SCENARIO_ID: c_uint = 0x08001010;
pub const APM_SUB_GRAPH_SID_AUDIO_PLAYBACK: c_uint = 0x1;
pub const APM_SUB_GRAPH_SID_AUDIO_RECORD: c_uint = 0x2;
pub const APM_SUB_GRAPH_SID_AUDIO_VOICE_CALL: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_sg_prop_id_scenario_id {
    pub scenario_id: u32,
    pub __packed: },
pub const APM_SG_PROP_ID_SID_SIZE: c_int = 4;
// container api
pub const APM_PARAM_ID_CONTAINER_CONFIG: c_uint = 0x08001000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_param_id_container_cfg {
    pub num_containers: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_container_cfg {
    pub container_id: u32,
    pub num_prop: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_capability {
    pub capability_id: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_CAPABILITY_LIST: c_uint = 0x08001011;
pub const APM_CONTAINER_PROP_ID_CAPABILITY_SIZE: c_int = 8;
pub const APM_PROP_ID_INVALID: c_uint = 0x0;
pub const APM_CONTAINER_CAP_ID_PP: c_uint = 0x1;
pub const APM_CONTAINER_CAP_ID_PP: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_cap_list {
    pub num_capability_id: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_GRAPH_POS: c_uint = 0x08001012;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_graph_pos {
    pub graph_pos: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_STACK_SIZE: c_uint = 0x08001013;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_stack_size {
    pub stack_size: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_PROC_DOMAIN: c_uint = 0x08001014;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_domain {
    pub proc_domain: u32,
    pub __packed: },
pub const CONFIG_I2S_WS_SRC_EXTERNAL: c_uint = 0x0;
pub const CONFIG_I2S_WS_SRC_INTERNAL: c_uint = 0x1;
pub const PARAM_ID_I2S_INTF_CFG: c_uint = 0x08001019;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_i2s_intf_cfg {
    pub lpaif_type: u32,
    pub intf_idx: u32,
    pub sd_line_idx: u16,
    pub ws_src: u16,
    pub __packed: },
pub const I2S_INTF_TYPE_PRIMARY: c_int = 0;
pub const I2S_INTF_TYPE_SECONDARY: c_int = 1;
pub const I2S_INTF_TYPE_TERTIARY: c_int = 2;
pub const I2S_INTF_TYPE_QUATERNARY: c_int = 3;
pub const I2S_INTF_TYPE_QUINARY: c_int = 4;
pub const I2S_SD0: c_int = 1;
pub const I2S_SD1: c_int = 2;
pub const I2S_SD2: c_int = 3;
pub const I2S_SD3: c_int = 4;
pub const PORT_ID_I2S_INPUT: c_int = 2;
pub const PORT_ID_I2S_OUPUT: c_int = 1;
pub const I2S_STACK_SIZE: c_int = 2048;
pub const PARAM_ID_AUDIO_IF_INTF_CFG: c_uint = 0x08001B11;
//
// struct param_id_audio_if_intf_cfg - Audio interface configuration
// @qaif_type: Audio interface type (e.g. QAIF, QAIF_VA)
// @intf_idx: Interface instance index
// @intf_mode: Interface operating mode (TDM/PCM/I2S)
// @ctrl_data_out_enable: Enable sharing of data-out signal with other masters
// @active_slot_mask: Bitmask indicating active slots
// @nslots_per_frame: Number of slots per audio frame
// @slot_width: Width of each slot in bits
// @active_lane_mask: Bitmask of active data lanes
// @frame_sync_rate: Frame sync rate in Hz
// @frame_sync_src: Frame sync source selection
// @frame_sync_mode: Frame sync mode configuration
// @invert_frame_sync_pulse: Invert frame sync polarity when set
// @frame_sync_data_delay: Data delay from frame sync in bit clocks
// @bit_clk_type: Bit clock type (internal / external)
// @inv_int_bit_clk: Invert internal bit clock when set
// @inv_ext_bit_clk: Invert external bit clock when set
//
// This structure defines configuration parameters for the Qualcomm
// Audio Interface (QAIF) block. It is used to program interface
// characteristics such as slot configuration, clocking and frame
// synchronization behaviour.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_audio_if_intf_cfg {
    pub qaif_type: u16,
    pub intf_idx: u16,
    pub intf_mode: u16,
    pub ctrl_data_out_enable: u16,
    pub active_slot_mask: u32,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub active_lane_mask: u32,
    pub frame_sync_rate: u32,
    pub frame_sync_src: u16,
    pub frame_sync_mode: u16,
    pub invert_frame_sync_pulse: u16,
    pub frame_sync_data_delay: u16,
    pub bit_clk_type: u16,
    pub inv_int_bit_clk: u8,
    pub inv_ext_bit_clk: u8,
    pub __packed: },
pub const PARAM_ID_HW_EP_FRAME_DURATION: c_uint = 0x08001B2F;
pub const AUDIO_IF_FRAME_DURATION_US: c_int = 1000;
pub const AUDIO_IF_FRAME_DURATION_NORMALIZATION_ENABLE: c_int = 1;
pub const AUDIO_IF_FRAME_DURATION_MIN_US: c_int = 1;
pub const AUDIO_IF_FRAME_DURATION_MAX_US: c_int = 100000;
//
// struct param_id_hw_ep_frame_duration - Hardware endpoint frame duration
// @frame_duration_in_us: Frame duration in microseconds.
// @allow_frame_duration_normalization: Permit SPF to normalize frame duration.
// @min_normalized_frame_dur_us: Minimum normalized frame duration in microseconds.
// @max_normalized_frame_dur_us: Maximum normalized frame duration in microseconds.
//
// This structure configures the frame duration for the Audio IF hardware
// endpoint and, when enabled, the allowed normalization range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_hw_ep_frame_duration {
    pub frame_duration_in_us: u32,
    pub allow_frame_duration_normalization: u32,
    pub min_normalized_frame_dur_us: u32,
    pub max_normalized_frame_dur_us: u32,
    pub __packed: },
pub const PARAM_ID_DISPLAY_PORT_INTF_CFG: c_uint = 0x08001154;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_display_port_intf_cfg {
    pub channel_allocation: u32,
// Multi-Steam Transport index
    pub mst_idx: u32,
    pub dptx_idx: u32,
    pub __packed: },
pub const PARAM_ID_HW_EP_MF_CFG: c_uint = 0x08001017;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_hw_ep_mf {
    pub sample_rate: u32,
    pub bit_width: u16,
    pub num_channels: u16,
    pub data_format: u32,
    pub __packed: },
pub const PARAM_ID_HW_EP_FRAME_SIZE_FACTOR: c_uint = 0x08001018;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_fram_size_factor {
    pub frame_size_factor: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_PARENT_CONTAINER_ID: c_uint = 0x080010CB;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_parent_container {
    pub parent_container_id: u32,
    pub __packed: },
pub const APM_CONTAINER_PROP_ID_HEAP_ID: c_uint = 0x08001174;
pub const APM_CONT_HEAP_DEFAULT: c_uint = 0x1;
pub const APM_CONT_HEAP_LOW_POWER: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_cont_prop_id_headp_id {
    pub heap_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_modules_list {
    pub sub_graph_id: u32,
    pub container_id: u32,
    pub num_modules: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_obj {
    pub module_id: u32,
    pub instance_id: u32,
    pub __packed: },
pub const APM_MODULE_PROP_ID_PORT_INFO: c_uint = 0x08001015;
pub const APM_MODULE_PROP_ID_PORT_INFO_SZ: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apm_module_prop_id_port_info {
    pub max_ip_port: u32,
    pub max_op_port: u32,
    pub __packed: },
pub const DATA_LOGGING_MAX_INPUT_PORTS: c_uint = 0x1;
pub const DATA_LOGGING_MAX_OUTPUT_PORTS: c_uint = 0x1;
pub const DATA_LOGGING_STACK_SIZE: c_int = 2048;
pub const PARAM_ID_DATA_LOGGING_CONFIG: c_uint = 0x08001031;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_logging_config {
    pub log_code: u32,
    pub log_tap_point_id: u32,
    pub mode: u32,
    pub __packed: },
// Speaker Protection
pub const PARAM_ID_SP_OP_MODE: c_uint = 0x080011e9;
pub const PARAM_ID_SP_OP_MODE_NORMAL: c_int = 0;
pub const PARAM_ID_SP_OP_MODE_CALIBRATION: c_int = 1;
pub const PARAM_ID_SP_OP_MODE_FACTORY_TEST: c_int = 2;
pub const PARAM_ID_SP_OP_MODE_VALIDATION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sp_op_mode {
    pub operation_mode: u32,
    pub __packed: },
// Speaker Protection VI
pub const PARAM_ID_SP_VI_OP_MODE_CFG: c_uint = 0x080011f4;
pub const PARAM_ID_SP_VI_OP_MODE_NORMAL: c_int = 0;
pub const PARAM_ID_SP_VI_OP_MODE_CALIBRATION: c_int = 1;
pub const PARAM_ID_SP_VI_OP_MODE_FACTORY_TEST: c_int = 2;
pub const PARAM_ID_SP_VI_OP_MODE_VALIDATION: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sp_vi_op_mode_cfg {
    pub num_channels: u32,
    pub operation_mode: u32,
    pub quick_calibration: u32,
    pub r0_t0_selection: [u32; ],
    pub __packed: },
pub const PARAM_ID_SP_VI_EX_MODE_CFG: c_uint = 0x080011ff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sp_vi_ex_mode_cfg {
    pub factory_mode: u32,
    pub __packed: },
pub const PARAM_ID_SP_VI_CHANNEL_MAP_CFG: c_uint = 0x08001203;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sp_vi_channel_map_cfg {
    pub num_channels: u32,
// [ Vsense of ch 1, Isense of ch 1, Vsense of ch 2, Isense of ch 2, ... ]
    pub channel_mapping: [u32; ],
    pub __packed: },
pub const PARAM_ID_SAL_OUTPUT_CFG: c_uint = 0x08001016;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sal_output_config {
    pub bits_per_sample: u32,
    pub __packed: },
pub const PARAM_ID_SAL_LIMITER_ENABLE: c_uint = 0x0800101E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sal_limiter_enable {
    pub enable_lim: u32,
    pub __packed: },
pub const PARAM_ID_MFC_OUTPUT_MEDIA_FORMAT: c_uint = 0x08001024;
pub const PARAM_ID_EARLY_EOS_DELAY: c_uint = 0x0800114C;
pub const EARLY_EOS_DELAY_MS: c_int = 150;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_mfc_media_format {
    pub sample_rate: u32,
    pub bit_width: u16,
    pub num_channels: u16,
    pub channel_mapping: [u16; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_gapless_early_eos_delay_t {
    pub early_eos_delay_ms: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_format {
    pub data_format: u32,
    pub fmt_id: u32,
    pub payload_size: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct payload_media_fmt_pcm {
    pub sample_rate: u32,
    pub bit_width: u16,
    pub alignment: u16,
    pub bits_per_sample: u16,
    pub q_factor: u16,
    pub endianness: u16,
    pub num_channels: u16,
    pub channel_mapping: [u8; ],
    pub __packed: },
pub const PARAM_ID_MODULE_ENABLE: c_uint = 0x08001026;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_module_enable {
    pub enable: u32,
    pub __packed: },
pub const PARAM_ID_CODEC_DMA_INTF_CFG: c_uint = 0x08001063;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_codec_dma_intf_cfg {
// 1 - RXTX
// 2 - WSA
// 3 - VA
// 4 - AXI
//
    pub lpaif_type: u32,
//
// RX0 | TX0 = 1
// RX1 | TX1 = 2
// RX2 | TX2 = 3... so on
//
    pub intf_index: u32,
    pub active_channels_mask: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_hw_clk_cfg {
    pub clock_id: u32,
    pub clock_freq: u32,
    pub clock_attri: u32,
    pub clock_root: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_hw_clk_rel_cfg {
    pub clock_id: u32,
    pub __packed: },
pub const PARAM_ID_HW_EP_POWER_MODE_CFG: c_uint = 0x8001176;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_hw_ep_power_mode_cfg {
    pub power_mode: u32,
    pub __packed: },
pub const PARAM_ID_HW_EP_DMA_DATA_ALIGN: c_uint = 0x08001233;
pub const AR_HW_EP_DMA_DATA_ALIGN_MSB: c_int = 0;
pub const AR_HW_EP_DMA_DATA_ALIGN_LSB: c_int = 1;
pub const AR_PCM_MAX_NUM_CHANNEL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_hw_ep_dma_data_align {
    pub dma_data_align: u32,
    pub __packed: },
pub const PARAM_ID_VOL_CTRL_MASTER_GAIN: c_uint = 0x08001035;
pub const VOL_CTRL_DEFAULT_GAIN: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_vol_ctrl_master_gain {
    pub master_gain: u16,
    pub reserved: u16,
    pub __packed: },
pub const PARAM_ID_REMOVE_INITIAL_SILENCE: c_uint = 0x0800114B;
pub const PARAM_ID_REMOVE_TRAILING_SILENCE: c_uint = 0x0800115D;
pub const PARAM_ID_REAL_MODULE_ID: c_uint = 0x0800100B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_placeholder_real_module_id {
    pub real_module_id: u32,
    pub __packed: },
pub const PARAM_ID_SH_MEM_PULL_PUSH_MODE_CFG: c_uint = 0x0800100A;
//
// struct param_id_sh_mem_pull_push_mode_cfg - Shared memory push/pull config
// @shared_circ_buf_addr_lsw: Lower 32 bits of the circular buffer address.
// @shared_circ_buf_addr_msw: Upper 32 bits of the circular buffer address.
// @shared_circ_buf_size: Circular buffer size in bytes.
// @circ_buf_mem_map_handle: Circular buffer memory map handle.
// @shared_pos_buf_addr_lsw: Lower 32 bits of the position buffer address.
// @shared_pos_buf_addr_msw: Upper 32 bits of the position buffer address.
// @pos_buf_mem_map_handle: Position buffer memory map handle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct param_id_sh_mem_pull_push_mode_cfg {
    pub shared_circ_buf_addr_lsw: u32,
    pub shared_circ_buf_addr_msw: u32,
    pub shared_circ_buf_size: u32,
    pub circ_buf_mem_map_handle: u32,
    pub shared_pos_buf_addr_lsw: u32,
    pub shared_pos_buf_addr_msw: u32,
    pub pos_buf_mem_map_handle: u32,
    pub __packed: },
//
// struct sh_mem_pull_push_mode_position_buffer - Shared position buffer
// @frame_counter: Synchronization counter.
// @index: Current read/write index in bytes.
// @timestamp_us_lsw: Lower 32 bits of the timestamp in microseconds.
// @timestamp_us_msw: Upper 32 bits of the timestamp in microseconds.
//
// The frame counter should be read before and after the other fields to
// ensure the DSP did not update them while they were being read.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sh_mem_pull_push_mode_position_buffer {
    pub frame_counter: u32,
    pub index: u32,
    pub timestamp_us_lsw: u32,
    pub timestamp_us_msw: u32,
    pub __packed: },
// Graph
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_connection {
// Connections
    pub src_mod_inst_id: u32,
    pub src_mod_op_port_id: u32,
    pub dst_mod_inst_id: u32,
    pub dst_mod_ip_port_id: u32,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_graph_info {
    pub id: c_int,
    pub mem_map_handle: u32,
    pub pos_buf_mem_map_handle: u32,
    pub num_sub_graphs: u32,
    pub sg_list: list_head,
    pub is_push_pull_mode: bool,
// DPCM connection from FE Graph to BE graph
    pub src_mod_inst_id: u32,
    pub src_mod_op_port_id: u32,
    pub dst_mod_inst_id: u32,
    pub dst_mod_ip_port_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_sub_graph {
    pub sub_graph_id: u32,
    pub perf_mode: u32,
    pub direction: u32,
    pub scenario_id: u32,
    pub node: list_head,
    pub info: *mut audioreach_graph_info,
    pub num_containers: u32,
    pub container_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_container {
    pub container_id: u32,
    pub capability_id: u32,
    pub graph_pos: u32,
    pub stack_size: u32,
    pub proc_domain: u32,
    pub node: list_head,
    pub num_modules: u32,
    pub modules_list: list_head,
    pub sub_graph: *mut audioreach_sub_graph,
}

pub const AR_MAX_MOD_LINKS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_module {
    pub module_id: u32,
    pub instance_id: u32,
    pub max_ip_port: u32,
    pub max_op_port: u32,
    pub num_connections: u32,
// Connections
    pub src_mod_inst_id: u32,
    pub src_mod_op_port_id: [u32; AR_MAX_MOD_LINKS],
    pub dst_mod_inst_id: [u32; AR_MAX_MOD_LINKS],
    pub dst_mod_ip_port_id: [u32; AR_MAX_MOD_LINKS],
// Format specifics
    pub ch_fmt: u32,
    pub rate: u32,
    pub bit_depth: u32,
// I2S module
    pub hw_interface_idx: u32,
    pub sd_line_idx: u32,
    pub ws_src: u32,
    pub frame_size_factor: u32,
    pub data_format: u32,
    pub hw_interface_type: u32,
// Audio IF module (TDM/PCM/I2S)
    pub slot_mask: u32,
    pub active_lane_mask: u32,
    pub frame_sync_rate: u32,
    pub qaif_type: u16,
    pub sync_src: u16,
    pub ctrl_data_out_enable: u16,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub intf_mode: u16,
    pub sync_mode: u16,
    pub ctrl_invert_sync_pulse: u16,
    pub ctrl_sync_data_delay: u16,
    pub bit_clk_type: u16,
    pub inv_int_bit_clk: u8,
    pub inv_ext_bit_clk: u8,
// PCM module specific
    pub interleave_type: u32,
// GAIN/Vol Control Module
    pub gain: u16,
// Logging
    pub log_code: u32,
    pub log_tap_point_id: u32,
    pub log_mode: u32,
// bookkeeping
    pub node: list_head,
    pub container: *mut audioreach_container,
    pub widget: *mut snd_soc_dapm_widget,
    pub data: *mut audioreach_module_priv_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioreach_module_config {
    pub direction: c_int,
    pub sample_rate: u32,
    pub bit_width: u16,
    pub bits_per_sample: u16,
    pub data_format: u16,
    pub num_channels: u16,
    pub dp_idx: u16,
    pub channel_allocation: u32,
    pub sd_line_mask: u32,
    pub fmt: c_int,
    pub slot_mask: u32,
    pub nslots_per_frame: u16,
    pub slot_width: u16,
    pub codec: snd_codec,
    pub channel_map: [u8; AR_PCM_MAX_NUM_CHANNEL],
}

// Packet Allocation routines
extern "C" {
    pub fn audioreach_set_default_channel_mapping(ch_map: *mut u8, num_channels: c_int);
}
// Topology specific
extern "C" {
    pub fn audioreach_tplg_init(component: *mut snd_soc_component) -> c_int;
}
// Module specific
extern "C" {
    pub fn audioreach_graph_free_buf(graph: *mut q6apm_graph);
}
extern "C" {
    pub fn audioreach_shared_memory_send_eos(graph: *mut q6apm_graph) -> c_int;
}
extern "C" {
    pub fn audioreach_map_memory_position_buffer(graph: *mut q6apm_graph, dir: c_uint) -> c_int;
}
extern "C" {
    pub fn audioreach_shmem_register_event(graph: *mut q6apm_graph, bytes: c_int, num_levels: c_int) -> c_int;
}
