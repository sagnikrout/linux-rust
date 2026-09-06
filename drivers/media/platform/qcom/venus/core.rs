//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/core.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//

pub const VIDC_CLKS_NUM_MAX: c_int = 4;
pub const VIDC_VCODEC_CLKS_NUM_MAX: c_int = 2;
pub const VIDC_RESETS_NUM_MAX: c_int = 2;
pub const VIDC_MAX_HIER_CODING_LAYER: c_int = 6;
pub const VENUS_MAX_FPS: c_int = 240;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_tbl {
    pub load: c_uint,
    pub freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_val {
    pub reg: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bw_tbl {
    pub mbs_per_sec: u32,
    pub avg: u32,
    pub peak: u32,
    pub avg_10bit: u32,
    pub peak_10bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vpu_version {
    VPU_VERSION_AR50,
    VPU_VERSION_AR50_LITE,
    VPU_VERSION_IRIS1,

    VPU_VERSION_IRIS2,
    VPU_VERSION_IRIS2_1,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware_version {
    pub major: u32,
    pub minor: u32,
    pub rev: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_resources {
    pub dma_mask: u64,
    pub freq_tbl: *const freq_tbl,
    pub freq_tbl_size: c_uint,
    pub bw_tbl_enc: *const bw_tbl,
    pub bw_tbl_enc_size: c_uint,
    pub bw_tbl_dec: *const bw_tbl,
    pub bw_tbl_dec_size: c_uint,
    pub reg_tbl: *const reg_val,
    pub reg_tbl_size: c_uint,
    pub ubwc_conf: *const hfi_ubwc_config,
    pub clks: [*const *const c_char; VIDC_CLKS_NUM_MAX],
    pub clks_num: c_uint,
    pub vcodec_clks: [*const *const c_char; VIDC_VCODEC_CLKS_NUM_MAX],
    pub vcodec0_clks: [*const *const c_char; VIDC_VCODEC_CLKS_NUM_MAX],
    pub vcodec1_clks: [*const *const c_char; VIDC_VCODEC_CLKS_NUM_MAX],
    pub vcodec_clks_num: c_uint,
    pub vcodec_pmdomains: *const c_char,
    pub vcodec_pmdomains_num: c_uint,
    pub opp_pmdomain: *const c_char,
    pub opp_pmdomain_num: c_uint,
    pub vcodec_num: c_uint,
    pub dec_codec_blacklist: u32,
    pub enc_codec_blacklist: u32,
    pub resets: [*const *const c_char; VIDC_RESETS_NUM_MAX],
    pub resets_num: c_uint,
    pub hfi_version: hfi_version,
    pub vpu_version: vpu_version,
    pub num_vpp_pipes: u8,
    pub max_load: u32,
    pub vmem_id: c_uint,
    pub vmem_size: u32,
    pub vmem_addr: u32,
    pub cp_start: u32,
    pub cp_size: u32,
    pub cp_nonpixel_start: u32,
    pub cp_nonpixel_size: u32,
    pub fwname: *const c_char,
    pub enc_nodename: *const c_char,
    pub dec_nodename: *const c_char,
    pub min_fw: *const firmware_version,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venus_fmt {
    VENUS_FMT_NV12			= 0,
    VENUS_FMT_QC08C			= 1,
    VENUS_FMT_QC10C			= 2,
    VENUS_FMT_P010			= 3,
    VENUS_FMT_H264			= 4,
    VENUS_FMT_VP8			= 5,
    VENUS_FMT_VP9			= 6,
    VENUS_FMT_HEVC			= 7,
    VENUS_FMT_VC1_ANNEX_G		= 8,
    VENUS_FMT_VC1_ANNEX_L		= 9,
    VENUS_FMT_MPEG4			= 10,
    VENUS_FMT_MPEG2			= 11,
    VENUS_FMT_H263			= 12,
    VENUS_FMT_XVID			= 13,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_format {
    pub pixfmt: u32,
    pub num_planes: c_uint,
    pub type: u32,
    pub flags: u32,
}

//
// struct venus_core - holds core parameters valid for all instances
//
// @base:	IO memory base address
// @vbif_base:	IO memory vbif base address
// @cpu_base:	IO memory cpu base address
// @cpu_cs_base:	IO memory cpu_cs base address
// @cpu_ic_base:	IO memory cpu_ic base address
// @wrapper_base:	IO memory wrapper base address
// @wrapper_tz_base:	IO memory wrapper TZ base address
// @aon_base:	AON base address
// @irq:		Venus irq
// @clks:	an array of struct clk pointers
// @vcodec_clks: an array of vcodec struct clk pointers
// @vcodec0_clks: an array of vcodec0 struct clk pointers
// @vcodec1_clks: an array of vcodec1 struct clk pointers
// @video_path: an interconnect handle to video to/from memory path
// @cpucfg_path: an interconnect handle to cpu configuration path
// @pmdomains:	a pointer to a list of pmdomains
// @opp_pmdomain: an OPP power-domain
// @resets: an array of reset signals
// @vdev_dec:	a reference to video device structure for decoder instances
// @vdev_enc:	a reference to video device structure for encoder instances
// @v4l2_dev:	a holder for v4l2 device structure
// @res:		a reference to venus resources structure
// @dev:		convenience struct device pointer
// @dev_dec:	convenience struct device pointer for decoder device
// @dev_enc:	convenience struct device pointer for encoder device
// @use_tz:	a flag that suggests presence of trustzone
// @fw:		structure of firmware parameters
// @lock:	a lock for this strucure
// @instances:	a list_head of all instances
// @insts_count:	num of instances
// @state:	the state of the venus core
// @done:	a completion for sync HFI operations
// @error:	an error returned during last HFI sync operations
// @sys_error:	an error flag that signal system error event
// @sys_err_done: a waitqueue to wait for system error recovery end
// @core_ops:	the core operations
// @pm_ops:	a pointer to pm operations
// @pm_lock:	a lock for PM operations
// @enc_codecs:	encoders supported by this core
// @dec_codecs:	decoders supported by this core
// @max_sessions_supported:	holds the maximum number of sessions
// @priv:	a private filed for HFI operations
// @ops:		the core HFI operations
// @work:	a delayed work for handling system fatal error
// @caps:	an array of supported HFI capabilities
// @codecs_count: platform codecs count
// @core0_usage_count: usage counter for core0
// @core1_usage_count: usage counter for core1
// @root:	debugfs root directory
// @venus_ver:	the venus firmware version
// @dump_core:	a flag indicating that a core dump is required
// @ocs:	OF changeset pointer
// @hwmode_dev:	a flag indicating that HW_CTRL_TRIGGER is used in clock driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_core {
    pub base: *mut void __iomem,
    pub vbif_base: *mut void __iomem,
    pub cpu_base: *mut void __iomem,
    pub cpu_cs_base: *mut void __iomem,
    pub cpu_ic_base: *mut void __iomem,
    pub wrapper_base: *mut void __iomem,
    pub wrapper_tz_base: *mut void __iomem,
    pub aon_base: *mut void __iomem,
    pub irq: c_int,
    pub clks: [*mut clk; VIDC_CLKS_NUM_MAX],
    pub vcodec_clks: [*mut clk; VIDC_VCODEC_CLKS_NUM_MAX],
    pub vcodec0_clks: [*mut clk; VIDC_VCODEC_CLKS_NUM_MAX],
    pub vcodec1_clks: [*mut clk; VIDC_VCODEC_CLKS_NUM_MAX],
    pub video_path: *mut icc_path,
    pub cpucfg_path: *mut icc_path,
    pub pmdomains: *mut dev_pm_domain_list,
    pub opp_pmdomain: *mut dev_pm_domain_list,
    pub resets: [*mut reset_control; VIDC_RESETS_NUM_MAX],
    pub vdev_dec: *mut video_device,
    pub vdev_enc: *mut video_device,
    pub v4l2_dev: v4l2_device,
    pub res: *const venus_resources,
    pub dev: *mut device,
    pub dev_dec: *mut device,
    pub dev_enc: *mut device,
    pub use_tz: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_firmware {
    pub dev: *mut device,
    pub iommu_domain: *mut iommu_domain,
    pub mapped_mem_size: usize,
    pub mem_phys: phys_addr_t,
    pub mem_size: usize,
    pub fw: },
    pub lock: mutex,
    pub instances: list_head,
    pub insts_count: core::sync::atomic::AtomicI32,
    pub state: c_uint,
    pub done: completion,
    pub error: c_uint,
    pub sys_error: c_ulong,
    pub sys_err_done: wait_queue_head_t,
    pub core_ops: *const hfi_core_ops,
    pub pm_ops: *const venus_pm_ops,
    pub pm_lock: mutex,
    pub enc_codecs: c_ulong,
    pub dec_codecs: c_ulong,
    pub max_sessions_supported: c_uint,
    pub priv: *mut c_void,
    pub ops: *const hfi_ops,
    pub work: delayed_work,
    pub caps: [hfi_plat_caps; MAX_CODEC_NUM],
    pub codecs_count: c_uint,
    pub core0_usage_count: c_uint,
    pub core1_usage_count: c_uint,
    pub root: *mut dentry,
    pub venus_ver: firmware_version,
    pub dump_core: c_ulong,
    pub ocs: *mut of_changeset,
    pub hwmode_dev: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_controls {
    pub post_loop_deb_mode: u32,
    pub profile: u32,
    pub level: u32,
    pub display_delay: u32,
    pub display_delay_enable: u32,
    pub conceal_color: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venc_controls {
    pub gop_size: u16,
    pub num_p_frames: u32,
    pub num_b_frames: u32,
    pub bitrate_mode: u32,
    pub bitrate: u32,
    pub bitrate_peak: u32,
    pub rc_enable: u32,
    pub const_quality: u32,
    pub frame_skip_mode: u32,
    pub layer_bitrate: u32,
    pub h264_i_period: u32,
    pub h264_entropy_mode: u32,
    pub h264_i_qp: u32,
    pub h264_p_qp: u32,
    pub h264_b_qp: u32,
    pub h264_min_qp: u32,
    pub h264_max_qp: u32,
    pub h264_i_min_qp: u32,
    pub h264_i_max_qp: u32,
    pub h264_p_min_qp: u32,
    pub h264_p_max_qp: u32,
    pub h264_b_min_qp: u32,
    pub h264_b_max_qp: u32,
    pub h264_loop_filter_mode: u32,
    pub h264_loop_filter_alpha: i32,
    pub h264_loop_filter_beta: i32,
    pub h264_8x8_transform: u32,
    pub h264_hier_layers: u32,
    pub h264_hier_layer_bitrate: [u32; VIDC_MAX_HIER_CODING_LAYER],
    pub hevc_i_qp: u32,
    pub hevc_p_qp: u32,
    pub hevc_b_qp: u32,
    pub hevc_min_qp: u32,
    pub hevc_max_qp: u32,
    pub hevc_i_min_qp: u32,
    pub hevc_i_max_qp: u32,
    pub hevc_p_min_qp: u32,
    pub hevc_p_max_qp: u32,
    pub hevc_b_min_qp: u32,
    pub hevc_b_max_qp: u32,
    pub vp8_min_qp: u32,
    pub vp8_max_qp: u32,
    pub multi_slice_mode: u32,
    pub multi_slice_max_bytes: u32,
    pub multi_slice_max_mb: u32,
    pub header_mode: u32,
    pub aud_enable: bool,
    pub intra_refresh_type: u32,
    pub intra_refresh_period: u32,
    pub h264: u32,
    pub mpeg4: u32,
    pub hevc: u32,
    pub vp8: u32,
    pub vp9: u32,
    pub profile: },
    pub h264: u32,
    pub mpeg4: u32,
    pub hevc: u32,
    pub vp9: u32,
    pub level: },
    pub base_priority_id: u32,
    pub ltr_count: u32,
    pub cll: v4l2_ctrl_hdr10_cll_info,
    pub mastering: v4l2_ctrl_hdr10_mastering_display,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub dma_addr: dma_addr_t,
    pub size: u32,
    pub reg_list: list_head,
    pub flags: u32,
    pub ref_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_data {
    pub core_id: u32,
    pub freq: c_ulong,
    pub vpp_freq: c_ulong,
    pub vsp_freq: c_ulong,
    pub low_power_freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venus_dec_state {
    VENUS_DEC_STATE_DEINIT		= 0,
    VENUS_DEC_STATE_INIT		= 1,
    VENUS_DEC_STATE_CAPTURE_SETUP	= 2,
    VENUS_DEC_STATE_STOPPED		= 3,
    VENUS_DEC_STATE_SEEK		= 4,
    VENUS_DEC_STATE_DRAIN		= 5,
    VENUS_DEC_STATE_DECODING	= 6,
    VENUS_DEC_STATE_DRC		= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venus_enc_state {
    VENUS_ENC_STATE_DEINIT		= 0,
    VENUS_ENC_STATE_INIT		= 1,
    VENUS_ENC_STATE_ENCODING	= 2,
    VENUS_ENC_STATE_STOPPED		= 3,
    VENUS_ENC_STATE_DRAIN		= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_ts_metadata {
    pub used: bool,
    pub ts_ns: u64,
    pub ts_us: u64,
    pub flags: u32,
    pub tc: v4l2_timecode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum venus_inst_modes {
    VENUS_LOW_POWER = BIT(0),
}

//
// struct venus_inst - holds per instance parameters
//
// @list:	used for attach an instance to the core
// @lock:	instance lock
// @core:	a reference to the core struct
// @clk_data:	clock data per core ID
// @dpbbufs:	a list of decoded picture buffers
// @internalbufs:	a list of internal bufferes
// @registeredbufs:	a list of registered capture bufferes
// @delayed_process:	a list of delayed buffers
// @delayed_process_work:	a work_struct for process delayed buffers
// @nonblock:		nonblocking flag
// @ctrl_handler:	v4l control handler
// @controls:	a union of decoder and encoder control parameters
// @fh:	 a holder of v4l file handle structure
// @streamon_cap: stream on flag for capture queue
// @streamon_out: stream on flag for output queue
// @width:	current capture width
// @height:	current capture height
// @crop:	current crop rectangle
// @fw_min_cnt:	 firmware minimum buffer count
// @out_width:	current output width
// @out_height:	current output height
// @colorspace:	current color space
// @ycbcr_enc:	current YCbCr encoding
// @quantization:	current quantization
// @xfer_func:	current xfer function
// @codec_state:	current decoder API state (see DEC_STATE_)
// @enc_state:		current encoder API state (see ENC_STATE_)
// @reconf_wait:	wait queue for resolution change event
// @subscriptions:	used to hold current events subscriptions
// @buf_count:		used to count number of buffers (reqbuf(0))
// @tss:		timestamp metadata
// @payloads:		cache plane payload to use it for clock/BW scaling
// @fps:		holds current FPS
// @timeperframe:	holds current time per frame structure
// @fmt_out:	a reference to output format structure
// @fmt_cap:	a reference to capture format structure
// @num_input_bufs:	holds number of input buffers
// @num_output_bufs:	holds number of output buffers
// @input_buf_size:	holds input buffer size
// @output_buf_size:	holds output buffer size
// @output2_buf_size:	holds secondary decoder output buffer size
// @dpb_buftype:	decoded picture buffer type
// @dpb_fmt:		decoded picture buffer raw format
// @opb_buftype:	output picture buffer type
// @opb_fmt:		output picture buffer raw format
// @reconfig:	a flag raised by decoder when the stream resolution changed
// @hfi_codec:		current codec for this instance in HFI space
// @sequence_cap:	a sequence counter for capture queue
// @sequence_out:	a sequence counter for output queue
// @m2m_dev:	a reference to m2m device structure
// @m2m_ctx:	a reference to m2m context structure
// @ctx_q_lock:	a lock to serialize video device ioctl calls
// @state:	current state of the instance
// @done:	a completion for sync HFI operation
// @error:	an error returned during last HFI sync operation
// @session_error:	a flag rised by HFI interface in case of session error
// @ops:		HFI operations
// @session_type:	the type of the session (decoder or encoder)
// @hprop:	a union used as a holder by get property
// @core_acquired:	the Core has been acquired
// @bit_depth:		current bitstream bit-depth
// @pic_struct:		bitstream progressive vs interlaced
// @next_buf_last: a flag to mark next queued capture buffer as last
// @drain_active:	Drain sequence is in progress
// @flags:	bitmask flags describing current instance mode
// @dpb_ids:	DPB buffer ID's
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_inst {
    pub list: list_head,
    pub lock: mutex,
    pub core: *mut venus_core,
    pub clk_data: clock_data,
    pub dpbbufs: list_head,
    pub internalbufs: list_head,
    pub registeredbufs: list_head,
    pub delayed_process: list_head,
    pub delayed_process_work: work_struct,
    pub nonblock: bool,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub dec: vdec_controls,
    pub enc: venc_controls,
    pub controls: },
    pub fh: v4l2_fh,
    pub streamon_out: unsigned int streamon_cap,,
    pub width: u32,
    pub height: u32,
    pub crop: v4l2_rect,
    pub fw_min_cnt: u32,
    pub out_width: u32,
    pub out_height: u32,
    pub colorspace: u32,
    pub ycbcr_enc: u8,
    pub quantization: u8,
    pub xfer_func: u8,
    pub codec_state: venus_dec_state,
    pub enc_state: venus_enc_state,
    pub reconf_wait: wait_queue_head_t,
    pub subscriptions: c_uint,
    pub buf_count: c_int,
    pub tss: [venus_ts_metadata; VIDEO_MAX_FRAME],
    pub payloads: [c_ulong; VIDEO_MAX_FRAME],
    pub fps: u64,
    pub timeperframe: v4l2_fract,
    pub fmt_out: *const venus_format,
    pub fmt_cap: *const venus_format,
    pub num_input_bufs: c_uint,
    pub num_output_bufs: c_uint,
    pub input_buf_size: c_uint,
    pub output_buf_size: c_uint,
    pub output2_buf_size: c_uint,
    pub dpb_buftype: u32,
    pub dpb_fmt: u32,
    pub opb_buftype: u32,
    pub opb_fmt: u32,
    pub reconfig: bool,
    pub hfi_codec: u32,
    pub sequence_cap: u32,
    pub sequence_out: u32,
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub m2m_ctx: *mut v4l2_m2m_ctx,
    pub ctx_q_lock: mutex,
    pub state: c_uint,
    pub done: completion,
    pub error: c_uint,
    pub session_error: bool,
    pub ops: *const hfi_inst_ops,
    pub session_type: u32,
    pub hprop: hfi_get_property,
    pub 1: unsigned int core_acquired:,
    pub bit_depth: c_uint,
    pub pic_struct: c_uint,
    pub next_buf_last: bool,
    pub drain_active: bool,
    pub flags: venus_inst_modes,
    pub dpb_ids: ida,
}

extern "C" {
    pub fn IS_AR50_LITE(_arg: core) -> return;
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), venus_inst: struct, _arg: fh) -> return;
}
extern "C" {
    pub fn venus_close_common(inst: *mut venus_inst, filp: *mut file);
}
