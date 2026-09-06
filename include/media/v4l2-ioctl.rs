//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-ioctl.h
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
// V 4 L 2   D R I V E R   H E L P E R   A P I
//
// Moved from videodev2.h
//
// Some commonly needed functions for drivers (v4l2-common.o module)
//

//
// struct v4l2_ioctl_ops - describe operations for each V4L2 ioctl
//
// @vidioc_querycap: pointer to the function that implements
// :ref:`VIDIOC_QUERYCAP <vidioc_querycap>` ioctl
// @vidioc_enum_fmt_vid_cap: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for video capture in single and multi plane mode
// @vidioc_enum_fmt_vid_overlay: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for video overlay
// @vidioc_enum_fmt_vid_out: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for video output in single and multi plane mode
// @vidioc_enum_fmt_sdr_cap: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for Software Defined Radio capture
// @vidioc_enum_fmt_sdr_out: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for Software Defined Radio output
// @vidioc_enum_fmt_meta_cap: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for metadata capture
// @vidioc_enum_fmt_meta_out: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FMT <vidioc_enum_fmt>` ioctl logic
// for metadata output
// @vidioc_g_fmt_vid_cap: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in single plane mode
// @vidioc_g_fmt_vid_overlay: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video overlay
// @vidioc_g_fmt_vid_out: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video out
// in single plane mode
// @vidioc_g_fmt_vid_out_overlay: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video overlay output
// @vidioc_g_fmt_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for raw VBI capture
// @vidioc_g_fmt_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for raw VBI output
// @vidioc_g_fmt_sliced_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI capture
// @vidioc_g_fmt_sliced_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI output
// @vidioc_g_fmt_vid_cap_mplane: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in multiple plane mode
// @vidioc_g_fmt_vid_out_mplane: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for video out
// in multiplane plane mode
// @vidioc_g_fmt_sdr_cap: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio capture
// @vidioc_g_fmt_sdr_out: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio output
// @vidioc_g_fmt_meta_cap: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for metadata capture
// @vidioc_g_fmt_meta_out: pointer to the function that implements
// :ref:`VIDIOC_G_FMT <vidioc_g_fmt>` ioctl logic for metadata output
// @vidioc_s_fmt_vid_cap: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in single plane mode
// @vidioc_s_fmt_vid_overlay: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video overlay
// @vidioc_s_fmt_vid_out: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video out
// in single plane mode
// @vidioc_s_fmt_vid_out_overlay: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video overlay output
// @vidioc_s_fmt_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for raw VBI capture
// @vidioc_s_fmt_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for raw VBI output
// @vidioc_s_fmt_sliced_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI capture
// @vidioc_s_fmt_sliced_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI output
// @vidioc_s_fmt_vid_cap_mplane: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in multiple plane mode
// @vidioc_s_fmt_vid_out_mplane: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for video out
// in multiplane plane mode
// @vidioc_s_fmt_sdr_cap: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio capture
// @vidioc_s_fmt_sdr_out: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio output
// @vidioc_s_fmt_meta_cap: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for metadata capture
// @vidioc_s_fmt_meta_out: pointer to the function that implements
// :ref:`VIDIOC_S_FMT <vidioc_g_fmt>` ioctl logic for metadata output
// @vidioc_try_fmt_vid_cap: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in single plane mode
// @vidioc_try_fmt_vid_overlay: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video overlay
// @vidioc_try_fmt_vid_out: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video out
// in single plane mode
// @vidioc_try_fmt_vid_out_overlay: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video overlay
// output
// @vidioc_try_fmt_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for raw VBI capture
// @vidioc_try_fmt_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for raw VBI output
// @vidioc_try_fmt_sliced_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI
// capture
// @vidioc_try_fmt_sliced_vbi_out: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for sliced VBI output
// @vidioc_try_fmt_vid_cap_mplane: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video capture
// in multiple plane mode
// @vidioc_try_fmt_vid_out_mplane: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for video out
// in multiplane plane mode
// @vidioc_try_fmt_sdr_cap: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio capture
// @vidioc_try_fmt_sdr_out: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for Software Defined
// Radio output
// @vidioc_try_fmt_meta_cap: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for metadata capture
// @vidioc_try_fmt_meta_out: pointer to the function that implements
// :ref:`VIDIOC_TRY_FMT <vidioc_g_fmt>` ioctl logic for metadata output
// @vidioc_reqbufs: pointer to the function that implements
// :ref:`VIDIOC_REQBUFS <vidioc_reqbufs>` ioctl
// @vidioc_querybuf: pointer to the function that implements
// :ref:`VIDIOC_QUERYBUF <vidioc_querybuf>` ioctl
// @vidioc_qbuf: pointer to the function that implements
// :ref:`VIDIOC_QBUF <vidioc_qbuf>` ioctl
// @vidioc_expbuf: pointer to the function that implements
// :ref:`VIDIOC_EXPBUF <vidioc_expbuf>` ioctl
// @vidioc_dqbuf: pointer to the function that implements
// :ref:`VIDIOC_DQBUF <vidioc_qbuf>` ioctl
// @vidioc_create_bufs: pointer to the function that implements
// :ref:`VIDIOC_CREATE_BUFS <vidioc_create_bufs>` ioctl
// @vidioc_prepare_buf: pointer to the function that implements
// :ref:`VIDIOC_PREPARE_BUF <vidioc_prepare_buf>` ioctl
// @vidioc_remove_bufs: pointer to the function that implements
// :ref:`VIDIOC_REMOVE_BUFS <vidioc_remove_bufs>` ioctl
// @vidioc_overlay: pointer to the function that implements
// :ref:`VIDIOC_OVERLAY <vidioc_overlay>` ioctl
// @vidioc_g_fbuf: pointer to the function that implements
// :ref:`VIDIOC_G_FBUF <vidioc_g_fbuf>` ioctl
// @vidioc_s_fbuf: pointer to the function that implements
// :ref:`VIDIOC_S_FBUF <vidioc_g_fbuf>` ioctl
// @vidioc_streamon: pointer to the function that implements
// :ref:`VIDIOC_STREAMON <vidioc_streamon>` ioctl
// @vidioc_streamoff: pointer to the function that implements
// :ref:`VIDIOC_STREAMOFF <vidioc_streamon>` ioctl
// @vidioc_g_std: pointer to the function that implements
// :ref:`VIDIOC_G_STD <vidioc_g_std>` ioctl
// @vidioc_s_std: pointer to the function that implements
// :ref:`VIDIOC_S_STD <vidioc_g_std>` ioctl
// @vidioc_querystd: pointer to the function that implements
// :ref:`VIDIOC_QUERYSTD <vidioc_querystd>` ioctl
// @vidioc_enum_input: pointer to the function that implements
// :ref:`VIDIOC_ENUM_INPUT <vidioc_g_input>` ioctl
// @vidioc_g_input: pointer to the function that implements
// :ref:`VIDIOC_G_INPUT <vidioc_g_input>` ioctl
// @vidioc_s_input: pointer to the function that implements
// :ref:`VIDIOC_S_INPUT <vidioc_g_input>` ioctl
// @vidioc_enum_output: pointer to the function that implements
// :ref:`VIDIOC_ENUM_OUTPUT <vidioc_g_output>` ioctl
// @vidioc_g_output: pointer to the function that implements
// :ref:`VIDIOC_G_OUTPUT <vidioc_g_output>` ioctl
// @vidioc_s_output: pointer to the function that implements
// :ref:`VIDIOC_S_OUTPUT <vidioc_g_output>` ioctl
// @vidioc_query_ext_ctrl: pointer to the function that implements
// :ref:`VIDIOC_QUERY_EXT_CTRL <vidioc_queryctrl>` ioctl
// @vidioc_g_ext_ctrls: pointer to the function that implements
// :ref:`VIDIOC_G_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
// @vidioc_s_ext_ctrls: pointer to the function that implements
// :ref:`VIDIOC_S_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
// @vidioc_try_ext_ctrls: pointer to the function that implements
// :ref:`VIDIOC_TRY_EXT_CTRLS <vidioc_g_ext_ctrls>` ioctl
// @vidioc_querymenu: pointer to the function that implements
// :ref:`VIDIOC_QUERYMENU <vidioc_queryctrl>` ioctl
// @vidioc_enumaudio: pointer to the function that implements
// :ref:`VIDIOC_ENUMAUDIO <vidioc_enumaudio>` ioctl
// @vidioc_g_audio: pointer to the function that implements
// :ref:`VIDIOC_G_AUDIO <vidioc_g_audio>` ioctl
// @vidioc_s_audio: pointer to the function that implements
// :ref:`VIDIOC_S_AUDIO <vidioc_g_audio>` ioctl
// @vidioc_enumaudout: pointer to the function that implements
// :ref:`VIDIOC_ENUMAUDOUT <vidioc_enumaudout>` ioctl
// @vidioc_g_audout: pointer to the function that implements
// :ref:`VIDIOC_G_AUDOUT <vidioc_g_audout>` ioctl
// @vidioc_s_audout: pointer to the function that implements
// :ref:`VIDIOC_S_AUDOUT <vidioc_g_audout>` ioctl
// @vidioc_g_modulator: pointer to the function that implements
// :ref:`VIDIOC_G_MODULATOR <vidioc_g_modulator>` ioctl
// @vidioc_s_modulator: pointer to the function that implements
// :ref:`VIDIOC_S_MODULATOR <vidioc_g_modulator>` ioctl
// @vidioc_g_pixelaspect: pointer to the function that implements
// the pixelaspect part of the :ref:`VIDIOC_CROPCAP <vidioc_cropcap>` ioctl
// @vidioc_g_selection: pointer to the function that implements
// :ref:`VIDIOC_G_SELECTION <vidioc_g_selection>` ioctl
// @vidioc_s_selection: pointer to the function that implements
// :ref:`VIDIOC_S_SELECTION <vidioc_g_selection>` ioctl
// @vidioc_g_jpegcomp: pointer to the function that implements
// :ref:`VIDIOC_G_JPEGCOMP <vidioc_g_jpegcomp>` ioctl
// @vidioc_s_jpegcomp: pointer to the function that implements
// :ref:`VIDIOC_S_JPEGCOMP <vidioc_g_jpegcomp>` ioctl
// @vidioc_g_enc_index: pointer to the function that implements
// :ref:`VIDIOC_G_ENC_INDEX <vidioc_g_enc_index>` ioctl
// @vidioc_encoder_cmd: pointer to the function that implements
// :ref:`VIDIOC_ENCODER_CMD <vidioc_encoder_cmd>` ioctl
// @vidioc_try_encoder_cmd: pointer to the function that implements
// :ref:`VIDIOC_TRY_ENCODER_CMD <vidioc_encoder_cmd>` ioctl
// @vidioc_decoder_cmd: pointer to the function that implements
// :ref:`VIDIOC_DECODER_CMD <vidioc_decoder_cmd>` ioctl
// @vidioc_try_decoder_cmd: pointer to the function that implements
// :ref:`VIDIOC_TRY_DECODER_CMD <vidioc_decoder_cmd>` ioctl
// @vidioc_g_parm: pointer to the function that implements
// :ref:`VIDIOC_G_PARM <vidioc_g_parm>` ioctl
// @vidioc_s_parm: pointer to the function that implements
// :ref:`VIDIOC_S_PARM <vidioc_g_parm>` ioctl
// @vidioc_g_tuner: pointer to the function that implements
// :ref:`VIDIOC_G_TUNER <vidioc_g_tuner>` ioctl
// @vidioc_s_tuner: pointer to the function that implements
// :ref:`VIDIOC_S_TUNER <vidioc_g_tuner>` ioctl
// @vidioc_g_frequency: pointer to the function that implements
// :ref:`VIDIOC_G_FREQUENCY <vidioc_g_frequency>` ioctl
// @vidioc_s_frequency: pointer to the function that implements
// :ref:`VIDIOC_S_FREQUENCY <vidioc_g_frequency>` ioctl
// @vidioc_enum_freq_bands: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FREQ_BANDS <vidioc_enum_freq_bands>` ioctl
// @vidioc_g_sliced_vbi_cap: pointer to the function that implements
// :ref:`VIDIOC_G_SLICED_VBI_CAP <vidioc_g_sliced_vbi_cap>` ioctl
// @vidioc_log_status: pointer to the function that implements
// :ref:`VIDIOC_LOG_STATUS <vidioc_log_status>` ioctl
// @vidioc_s_hw_freq_seek: pointer to the function that implements
// :ref:`VIDIOC_S_HW_FREQ_SEEK <vidioc_s_hw_freq_seek>` ioctl
// @vidioc_g_register: pointer to the function that implements
// :ref:`VIDIOC_DBG_G_REGISTER <vidioc_dbg_g_register>` ioctl
// @vidioc_s_register: pointer to the function that implements
// :ref:`VIDIOC_DBG_S_REGISTER <vidioc_dbg_g_register>` ioctl
// @vidioc_g_chip_info: pointer to the function that implements
// :ref:`VIDIOC_DBG_G_CHIP_INFO <vidioc_dbg_g_chip_info>` ioctl
// @vidioc_enum_framesizes: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FRAMESIZES <vidioc_enum_framesizes>` ioctl
// @vidioc_enum_frameintervals: pointer to the function that implements
// :ref:`VIDIOC_ENUM_FRAMEINTERVALS <vidioc_enum_frameintervals>` ioctl
// @vidioc_s_dv_timings: pointer to the function that implements
// :ref:`VIDIOC_S_DV_TIMINGS <vidioc_g_dv_timings>` ioctl
// @vidioc_g_dv_timings: pointer to the function that implements
// :ref:`VIDIOC_G_DV_TIMINGS <vidioc_g_dv_timings>` ioctl
// @vidioc_query_dv_timings: pointer to the function that implements
// :ref:`VIDIOC_QUERY_DV_TIMINGS <vidioc_query_dv_timings>` ioctl
// @vidioc_enum_dv_timings: pointer to the function that implements
// :ref:`VIDIOC_ENUM_DV_TIMINGS <vidioc_enum_dv_timings>` ioctl
// @vidioc_dv_timings_cap: pointer to the function that implements
// :ref:`VIDIOC_DV_TIMINGS_CAP <vidioc_dv_timings_cap>` ioctl
// @vidioc_g_edid: pointer to the function that implements
// :ref:`VIDIOC_G_EDID <vidioc_g_edid>` ioctl
// @vidioc_s_edid: pointer to the function that implements
// :ref:`VIDIOC_S_EDID <vidioc_g_edid>` ioctl
// @vidioc_subscribe_event: pointer to the function that implements
// :ref:`VIDIOC_SUBSCRIBE_EVENT <vidioc_subscribe_event>` ioctl
// @vidioc_unsubscribe_event: pointer to the function that implements
// :ref:`VIDIOC_UNSUBSCRIBE_EVENT <vidioc_unsubscribe_event>` ioctl
// @vidioc_default: pointed used to allow other ioctls
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_ioctl_ops {
// ioctl callbacks
// VIDIOC_QUERYCAP handler
    pub cap): *mut v4l2_capability,
// VIDIOC_ENUM_FMT handlers
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
    pub f): *mut v4l2_fmtdesc,
// VIDIOC_G_FMT handlers
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
// VIDIOC_S_FMT handlers
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
// VIDIOC_TRY_FMT handlers
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
    pub f): *mut v4l2_format,
// Buffer handlers
    pub b): *mut v4l2_requestbuffers,
    pub b): *mut v4l2_buffer,
    pub b): *mut v4l2_buffer,
    pub e): *mut v4l2_exportbuffer,
    pub b): *mut v4l2_buffer,
    pub b): *mut v4l2_create_buffers,
    pub b): *mut v4l2_buffer,
    pub d): *mut v4l2_remove_buffers,
    pub i): *mut *mut *mut *mut int (vidioc_overlay)(struct file file, void priv, unsigned int,
    pub a): *mut v4l2_framebuffer,
    pub a): *const v4l2_framebuffer,
// Stream on/off
    pub i): v4l2_buf_type,
    pub i): v4l2_buf_type,
//
// Standard handling
//
// Note: ENUMSTD is handled by videodev.c
//
    pub norm): *mut *mut *mut *mut int (vidioc_g_std)(struct file file, void priv, v4l2_std_id,
    pub norm): *mut *mut *mut *mut int (vidioc_s_std)(struct file file, void priv, v4l2_std_id,
    pub a): *mut *mut *mut *mut int (vidioc_querystd)(struct file file, void priv, v4l2_std_id,
// Input handling
    pub inp): *mut v4l2_input,
    pub i): *mut *mut *mut *mut int (vidioc_g_input)(struct file file, void priv, unsigned int,
    pub i): *mut *mut *mut *mut int (vidioc_s_input)(struct file file, void priv, unsigned int,
// Output handling
    pub a): *mut v4l2_output,
    pub i): *mut *mut *mut *mut int (vidioc_g_output)(struct file file, void priv, unsigned int,
    pub i): *mut *mut *mut *mut int (vidioc_s_output)(struct file file, void priv, unsigned int,
// Control handling
    pub a): *mut v4l2_query_ext_ctrl,
    pub a): *mut v4l2_ext_controls,
    pub a): *mut v4l2_ext_controls,
    pub a): *mut v4l2_ext_controls,
    pub a): *mut v4l2_querymenu,
// Audio ioctls
    pub a): *mut v4l2_audio,
    pub a): *mut v4l2_audio,
    pub a): *const v4l2_audio,
// Audio out ioctls
    pub a): *mut v4l2_audioout,
    pub a): *mut v4l2_audioout,
    pub a): *const v4l2_audioout,
    pub a): *mut v4l2_modulator,
    pub a): *const v4l2_modulator,
// Crop ioctls
    pub aspect): *mut int buf_type, struct v4l2_fract,
    pub s): *mut v4l2_selection,
    pub s): *mut v4l2_selection,
// Compression ioctls
    pub a): *mut v4l2_jpegcompression,
    pub a): *const v4l2_jpegcompression,
    pub a): *mut v4l2_enc_idx,
    pub a): *mut v4l2_encoder_cmd,
    pub a): *mut v4l2_encoder_cmd,
    pub a): *mut v4l2_decoder_cmd,
    pub a): *mut v4l2_decoder_cmd,
// Stream type-dependent parameter ioctls
    pub a): *mut v4l2_streamparm,
    pub a): *mut v4l2_streamparm,
// Tuner ioctls
    pub a): *mut v4l2_tuner,
    pub a): *const v4l2_tuner,
    pub a): *mut v4l2_frequency,
    pub a): *const v4l2_frequency,
    pub band): *mut v4l2_frequency_band,
// Sliced VBI cap
    pub a): *mut v4l2_sliced_vbi_cap,
// Log status ioctl
    pub priv): *mut *mut *mut int (vidioc_log_status)(struct file file, void,
    pub a): *const v4l2_hw_freq_seek,
// Debugging ioctls

    pub reg): *mut v4l2_dbg_register,
    pub reg): *const v4l2_dbg_register,
    pub chip): *mut v4l2_dbg_chip_info,

    pub fsize): *mut v4l2_frmsizeenum,
    pub fival): *mut v4l2_frmivalenum,
// DV Timings IOCTLs
    pub timings): *mut v4l2_dv_timings,
    pub timings): *mut v4l2_dv_timings,
    pub timings): *mut v4l2_dv_timings,
    pub timings): *mut v4l2_enum_dv_timings,
    pub cap): *mut v4l2_dv_timings_cap,
    pub edid): *mut v4l2_edid,
    pub edid): *mut v4l2_edid,
    pub sub): *const v4l2_event_subscription,
    pub sub): *const v4l2_event_subscription,
// For other private ioctls
    pub arg): *mut bool valid_prio, unsigned int cmd, void,
}

// v4l debugging and diagnostics
// Device debug flags to be used with the video device debug attribute
// Just log the ioctl name + error code
pub const V4L2_DEV_DEBUG_IOCTL: c_uint = 0x01;
// Log the ioctl name arguments + error code
pub const V4L2_DEV_DEBUG_IOCTL_ARG: c_uint = 0x02;
// Log the file operations open, release, mmap and get_unmapped_area
pub const V4L2_DEV_DEBUG_FOP: c_uint = 0x04;
// Log the read and write file operations and the VIDIOC_(D)QBUF ioctls
pub const V4L2_DEV_DEBUG_STREAMING: c_uint = 0x08;
// Log poll()
pub const V4L2_DEV_DEBUG_POLL: c_uint = 0x10;
// Log controls
pub const V4L2_DEV_DEBUG_CTRL: c_uint = 0x20;
// Video standard functions
//
// v4l2_norm_to_name - Ancillary routine to analog TV standard name from its ID.
//
// @id:	analog TV standard ID.
//
// Return: returns a string with the name of the analog TV standard.
// If the standard is not found or if @id points to multiple standard,
// it returns "Unknown".
//
// v4l2_video_std_frame_period - Ancillary routine that fills a
// struct &v4l2_fract pointer with the default framerate fraction.
//
// @id: analog TV standard ID.
// @frameperiod: struct &v4l2_fract pointer to be filled
//
extern "C" {
    pub fn v4l2_video_std_frame_period(id: c_int, frameperiod: *mut v4l2_fract);
}
//
// v4l2_video_std_construct - Ancillary routine that fills in the fields of
// a &v4l2_standard structure according to the @id parameter.
//
// @vs: struct &v4l2_standard pointer to be filled
// @id: analog TV standard ID.
// @name: name of the standard to be used
//
// .. note::
//
// This ancillary routine is obsolete. Shouldn't be used on newer drivers.
//
// v4l_video_std_enumstd - Ancillary routine that fills in the fields of
// a &v4l2_standard structure according to the @id and @vs->index
// parameters.
//
// @vs: struct &v4l2_standard pointer to be filled.
// @id: analog TV standard ID.
//
extern "C" {
    pub fn v4l_video_std_enumstd(vs: *mut v4l2_standard, id: v4l2_std_id) -> c_int;
}
//
// v4l_printk_ioctl - Ancillary routine that prints the ioctl in a
// human-readable format.
//
// @prefix: prefix to be added at the ioctl prints.
// @cmd: ioctl name
//
// .. note::
//
// If prefix != %NULL, then it will issue a
// ``printk(KERN_DEBUG "%s: ", prefix)`` first.
//
extern "C" {
    pub fn v4l_printk_ioctl(prefix: *const c_char, cmd: c_uint);
}
// names for fancy debug output
//
// var v4l2_field_names - Helper array mapping ``V4L2_FIELD_*`` to strings.
//
// Specially when printing debug messages, it is interesting to output
// the field order at the V4L2 buffers. This array associates all possible
// values of field pix format from V4L2 API into a string.
//
// var v4l2_type_names - Helper array mapping ``V4L2_BUF_TYPE_*`` to strings.
//
// When printing debug messages, it is interesting to output the V4L2 buffer
// type number with a name that represents its content.
//

//
// v4l2_compat_ioctl32 -32 Bits compatibility layer for 64 bits processors
//
// @file: Pointer to struct &file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
//

extern "C" {
    pub fn v4l2_compat_translate_cmd(cmd: c_uint) -> c_uint;
}
extern "C" {
    pub fn v4l2_translate_cmd(cmd: c_uint) -> c_uint;
}
extern "C" {
    pub fn v4l2_compat_get_user(arg: *mut void __user, parg: *mut c_void, cmd: c_uint) -> c_int;
}
extern "C" {
    pub fn v4l2_compat_put_user(arg: *mut void __user, parg: *mut c_void, cmd: c_uint) -> c_int;
}
//
// typedef v4l2_kioctl - Typedef used to pass an ioctl handler.
//
// @file: Pointer to struct &file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
//
extern "C" {
    pub fn long(file: *mut *mut v4l2_kioctl)(struct file, cmd: c_uint, arg: *mut c_void) -> typedef;
}
//
// video_usercopy - copies data from/to userspace memory when an ioctl is
// issued.
//
// @file: Pointer to struct &file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
// @func: function that will handle the ioctl
//
// .. note::
//
// This routine should be used only inside the V4L2 core.
//
// video_ioctl2 - Handles a V4L2 ioctl.
//
// @file: Pointer to struct &file.
// @cmd: Ioctl name.
// @arg: Ioctl argument.
//
// Method used to handle an ioctl. Should be used to fill the
// &v4l2_file_operations.unlocked_ioctl on all V4L2 drivers.
//
// The user space interpretation of the 'v4l2_event' differs
// based on the 'time_t' definition on 32-bit architectures, so
// the kernel has to handle both.
// This is the old version for 32-bit architectures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_event_time32 {
    pub type: __u32,
    pub vsync: v4l2_event_vsync,
    pub ctrl: v4l2_event_ctrl,
    pub frame_sync: v4l2_event_frame_sync,
    pub src_change: v4l2_event_src_change,
    pub motion_det: v4l2_event_motion_det,
    pub data: [__u8; 64],
    pub u: },
    pub pending: __u32,
    pub sequence: __u32,
    pub timestamp: old_timespec32,
    pub id: __u32,
    pub reserved: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_buffer_time32 {
    pub index: __u32,
    pub type: __u32,
    pub bytesused: __u32,
    pub flags: __u32,
    pub field: __u32,
    pub timestamp: old_timeval32,
    pub timecode: v4l2_timecode,
    pub sequence: __u32,
// memory location
    pub memory: __u32,
    pub offset: __u32,
    pub userptr: c_ulong,
    pub planes: *mut v4l2_plane,
    pub fd: __s32,
    pub m: },
    pub length: __u32,
    pub reserved2: __u32,
    pub request_fd: __s32,
    pub reserved: __u32,
}

