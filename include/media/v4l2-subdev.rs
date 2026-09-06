//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-subdev.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// V4L2 sub-device support header.
//
// Copyright (C) 2008  Hans Verkuil <hverkuil@kernel.org>
//

// generic v4l2_device notify callback notification values

pub const V4L2_SUBDEV_IR_RX_FIFO_SERVICE_REQ: c_uint = 0x00000001;
pub const V4L2_SUBDEV_IR_RX_END_OF_RX_DETECTED: c_uint = 0x00000002;
pub const V4L2_SUBDEV_IR_RX_HW_FIFO_OVERRUN: c_uint = 0x00000004;
pub const V4L2_SUBDEV_IR_RX_SW_FIFO_OVERRUN: c_uint = 0x00000008;

pub const V4L2_SUBDEV_IR_TX_FIFO_SERVICE_REQ: c_uint = 0x00000001;

//
// struct v4l2_decode_vbi_line - used to decode_vbi_line
//
// @is_second_field: Set to 0 for the first (odd) field;
// set to 1 for the second (even) field.
// @p: Pointer to the sliced VBI data from the decoder. On exit, points to
// the start of the payload.
// @line: Line number of the sliced VBI data (1-23)
// @type: VBI service type (V4L2_SLICED_*). 0 if no service found
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_decode_vbi_line {
    pub is_second_field: u32,
    pub p: *mut u8,
    pub line: u32,
    pub type: u32,
}

//
// Sub-devices are devices that are connected somehow to the main bridge
// device. These devices are usually audio/video muxers/encoders/decoders or
// sensors and webcam controllers.
//
// Usually these devices are controlled through an i2c bus, but other buses
// may also be used.
//
// The v4l2_subdev struct provides a way of accessing these devices in a
// generic manner. Most operations that these sub-devices support fall in
// a few categories: core ops, audio ops, video ops and tuner ops.
//
// More categories can be added if needed, although this should remain a
// limited set (no more than approx. 8 categories).
//
// Each category has its own set of ops that subdev drivers can implement.
//
// A subdev driver can leave the pointer to the category ops NULL if
// it does not implement them (e.g. an audio subdev will generally not
// implement the video category ops). The exception is the core category:
// this must always be present.
//
// These ops are all used internally so it is no problem to change, remove
// or add ops or move ops from one to another category. Currently these
// ops are based on the original ioctls, but since ops are not limited to
// one argument there is room for improvement here once all i2c subdev
// drivers are converted to use these ops.
//
// Core ops: it is highly recommended to implement at least these ops:
//
// log_status
// g_register
// s_register
//
// This provides basic debugging support.
//
// The ioctl ops is meant for generic ioctl-like commands. Depending on
// the use-case it might be better to use subdev-specific ops (currently
// not yet implemented) since ops provide proper type-checking.
//
// enum v4l2_subdev_io_pin_bits - Subdevice external IO pin configuration
// bits
//
// @V4L2_SUBDEV_IO_PIN_DISABLE: disables a pin config. ENABLE assumed.
// @V4L2_SUBDEV_IO_PIN_OUTPUT: set it if pin is an output.
// @V4L2_SUBDEV_IO_PIN_INPUT: set it if pin is an input.
// @V4L2_SUBDEV_IO_PIN_SET_VALUE: to set the output value via
// &struct v4l2_subdev_io_pin_config->value.
// @V4L2_SUBDEV_IO_PIN_ACTIVE_LOW: pin active is bit 0.
// Otherwise, ACTIVE HIGH is assumed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_subdev_io_pin_bits {
    V4L2_SUBDEV_IO_PIN_DISABLE	= 0,
    V4L2_SUBDEV_IO_PIN_OUTPUT	= 1,
    V4L2_SUBDEV_IO_PIN_INPUT	= 2,
    V4L2_SUBDEV_IO_PIN_SET_VALUE	= 3,
    V4L2_SUBDEV_IO_PIN_ACTIVE_LOW	= 4,
}

//
// struct v4l2_subdev_io_pin_config - Subdevice external IO pin configuration
//
// @flags: bitmask with flags for this pin's config, whose bits are defined by
// &enum v4l2_subdev_io_pin_bits.
// @pin: Chip external IO pin to configure
// @function: Internal signal pad/function to route to IO pin
// @value: Initial value for pin - e.g. GPIO output value
// @strength: Pin drive strength
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_io_pin_config {
    pub flags: u32,
    pub pin: u8,
    pub function: u8,
    pub value: u8,
    pub strength: u8,
}

//
// struct v4l2_subdev_core_ops - Define core ops callbacks for subdevs
//
// @log_status: callback for VIDIOC_LOG_STATUS() ioctl handler code.
//
// @s_io_pin_config: configure one or more chip I/O pins for chips that
// multiplex different internal signal pads out to IO pins.  This function
// takes a pointer to an array of 'n' pin configuration entries, one for
// each pin being configured.  This function could be called at times
// other than just subdevice initialization.
//
// @init: initialize the sensor registers to some sort of reasonable default
// values. Do not use for new drivers and should be removed in existing
// drivers.
//
// @load_fw: load firmware.
//
// @reset: generic reset command. The argument selects which subsystems to
// reset. Passing 0 will always reset the whole chip. Do not use for new
// drivers without discussing this first on the linux-media mailinglist.
// There should be no reason normally to reset a device.
//
// @s_gpio: set GPIO pins. Very simple right now, might need to be extended with
// a direction argument if needed.
//
// @command: called by in-kernel drivers in order to call functions internal
// to subdev drivers driver that have a separate callback.
//
// @ioctl: called at the end of ioctl() syscall handler at the V4L2 core.
// used to provide support for private ioctls used on the driver.
//
// @compat_ioctl32: called when a 32 bits application uses a 64 bits Kernel,
// in order to fix data passed from/to userspace.
//
// @g_register: callback for VIDIOC_DBG_G_REGISTER() ioctl handler code.
//
// @s_register: callback for VIDIOC_DBG_S_REGISTER() ioctl handler code.
//
// @s_power: puts subdevice in power saving mode (on == 0) or normal operation
// mode (on == 1). DEPRECATED. See
// Documentation/driver-api/media/camera-sensor.rst . pre_streamon and
// post_streamoff callbacks can be used for e.g. setting the bus to LP-11
// mode before s_stream is called.
//
// @interrupt_service_routine: Called by the bridge chip's interrupt service
// handler, when an interrupt status has be raised due to this subdev,
// so that this subdev can handle the details.  It may schedule work to be
// performed later.  It must not sleep. **Called from an IRQ context**.
//
// @subscribe_event: used by the drivers to request the control framework that
// for it to be warned when the value of a control changes.
//
// @unsubscribe_event: remove event subscription from the control framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_core_ops {
    pub sd): *mut *mut int (log_status)(struct v4l2_subdev,
    pub pincfg): *mut v4l2_subdev_io_pin_config,
    pub val): *mut *mut *mut int (init)(struct v4l2_subdev sd, u32,
    pub sd): *mut *mut int (load_fw)(struct v4l2_subdev,
    pub val): *mut *mut *mut int (reset)(struct v4l2_subdev sd, u32,
    pub val): *mut *mut *mut int (s_gpio)(struct v4l2_subdev sd, u32,
    pub arg): *mut *mut *mut long (command)(struct v4l2_subdev sd, unsigned int cmd, void,
    pub arg): *mut *mut *mut long (ioctl)(struct v4l2_subdev sd, unsigned int cmd, void,

    pub arg): c_ulong,

    pub reg): *mut *mut *mut int (g_register)(struct v4l2_subdev sd, struct v4l2_dbg_register,
    pub reg): *const *const *const int (s_register)(struct v4l2_subdev sd, struct v4l2_dbg_register,

    pub on): *mut *mut *mut int (s_power)(struct v4l2_subdev sd, int,
    pub handled): *mut u32 status, bool,
    pub sub): *mut v4l2_event_subscription,
    pub sub): *mut v4l2_event_subscription,
}

//
// struct v4l2_subdev_tuner_ops - Callbacks used when v4l device was opened
// in radio mode.
//
// @standby: puts the tuner in standby mode. It will be woken up
// automatically the next time it is used.
//
// @s_radio: callback that switches the tuner to radio mode.
// drivers should explicitly call it when a tuner ops should
// operate on radio mode, before being able to handle it.
// Used on devices that have both AM/FM radio receiver and TV.
//
// @s_frequency: callback for VIDIOC_S_FREQUENCY() ioctl handler code.
//
// @g_frequency: callback for VIDIOC_G_FREQUENCY() ioctl handler code.
// freq->type must be filled in. Normally done by video_ioctl2()
// or the bridge driver.
//
// @enum_freq_bands: callback for VIDIOC_ENUM_FREQ_BANDS() ioctl handler code.
//
// @g_tuner: callback for VIDIOC_G_TUNER() ioctl handler code.
//
// @s_tuner: callback for VIDIOC_S_TUNER() ioctl handler code. @vt->type must be
// filled in. Normally done by video_ioctl2 or the
// bridge driver.
//
// @g_modulator: callback for VIDIOC_G_MODULATOR() ioctl handler code.
//
// @s_modulator: callback for VIDIOC_S_MODULATOR() ioctl handler code.
//
// @s_type_addr: sets tuner type and its I2C addr.
//
// @s_config: sets tda9887 specific stuff, like port1, port2 and qss
//
// .. note::
//
// On devices that have both AM/FM and TV, it is up to the driver
// to explicitly call s_radio when the tuner should be switched to
// radio mode, before handling other &struct v4l2_subdev_tuner_ops
// that would require it. An example of such usage is::
//
// static void s_frequency(void *priv, const struct v4l2_frequency *f)
// {
// ...
// if (f.type == V4L2_TUNER_RADIO)
// v4l2_device_call_all(v4l2_dev, 0, tuner, s_radio);
// ...
// v4l2_device_call_all(v4l2_dev, 0, tuner, s_frequency);
// }
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_tuner_ops {
    pub sd): *mut *mut int (standby)(struct v4l2_subdev,
    pub sd): *mut *mut int (s_radio)(struct v4l2_subdev,
    pub freq): *const *const *const int (s_frequency)(struct v4l2_subdev sd, struct v4l2_frequency,
    pub freq): *mut *mut *mut int (g_frequency)(struct v4l2_subdev sd, struct v4l2_frequency,
    pub band): *mut *mut *mut int (enum_freq_bands)(struct v4l2_subdev sd, struct v4l2_frequency_band,
    pub vt): *mut *mut *mut int (g_tuner)(struct v4l2_subdev sd, struct v4l2_tuner,
    pub vt): *const *const *const int (s_tuner)(struct v4l2_subdev sd, struct v4l2_tuner,
    pub vm): *mut *mut *mut int (g_modulator)(struct v4l2_subdev sd, struct v4l2_modulator,
    pub vm): *const *const *const int (s_modulator)(struct v4l2_subdev sd, struct v4l2_modulator,
    pub type): *mut *mut *mut int (s_type_addr)(struct v4l2_subdev sd, struct tuner_setup,
    pub config): *const *const *const int (s_config)(struct v4l2_subdev sd, struct v4l2_priv_tun_config,
}

//
// struct v4l2_subdev_audio_ops - Callbacks used for audio-related settings
//
// @s_clock_freq: set the frequency (in Hz) of the audio clock output.
// Used to slave an audio processor to the video decoder, ensuring that
// audio and video remain synchronized. Usual values for the frequency
// are 48000, 44100 or 32000 Hz. If the frequency is not supported, then
// -EINVAL is returned.
//
// @s_i2s_clock_freq: sets I2S speed in bps. This is used to provide a standard
// way to select I2S clock used by driving digital audio streams at some
// board designs. Usual values for the frequency are 1024000 and 2048000.
// If the frequency is not supported, then %-EINVAL is returned.
//
// @s_routing: used to define the input and/or output pins of an audio chip,
// and any additional configuration data.
// Never attempt to use user-level input IDs (e.g. Composite, S-Video,
// Tuner) at this level. An i2c device shouldn't know about whether an
// input pin is connected to a Composite connector, become on another
// board or platform it might be connected to something else entirely.
// The calling driver is responsible for mapping a user-level input to
// the right pins on the i2c device.
//
// @s_stream: used to notify the audio code that stream will start or has
// stopped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_audio_ops {
    pub freq): *mut *mut *mut int (s_clock_freq)(struct v4l2_subdev sd, u32,
    pub freq): *mut *mut *mut int (s_i2s_clock_freq)(struct v4l2_subdev sd, u32,
    pub config): *mut *mut *mut int (s_routing)(struct v4l2_subdev sd, u32 input, u32 output, u32,
    pub enable): *mut *mut *mut int (s_stream)(struct v4l2_subdev sd, int,
}

//
// struct v4l2_mbus_frame_desc_entry_csi2
//
// @vc: CSI-2 virtual channel
// @dt: CSI-2 data type ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_frame_desc_entry_csi2 {
    pub vc: u8,
    pub dt: u8,
}

//
// enum v4l2_mbus_frame_desc_flags - media bus frame description flags
//
// @V4L2_MBUS_FRAME_DESC_FL_LEN_MAX:
// Indicates that &struct v4l2_mbus_frame_desc_entry->length field
// specifies maximum data length.
// @V4L2_MBUS_FRAME_DESC_FL_BLOB:
// Indicates that the format does not have line offsets, i.e.
// the receiver should use 1D DMA.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mbus_frame_desc_flags {
    V4L2_MBUS_FRAME_DESC_FL_LEN_MAX	= BIT(0),
    V4L2_MBUS_FRAME_DESC_FL_BLOB	= BIT(1),
}

//
// struct v4l2_mbus_frame_desc_entry - media bus frame description structure
//
// @flags:	bitmask flags, as defined by &enum v4l2_mbus_frame_desc_flags.
// @stream:	stream in routing configuration
// @pixelcode:	media bus pixel code, valid if @flags
// %FRAME_DESC_FL_BLOB is not set.
// @length:	number of octets per frame, valid if @flags
// %V4L2_MBUS_FRAME_DESC_FL_LEN_MAX is set.
// @bus:	Bus-specific frame descriptor parameters
// @bus.csi2:	CSI-2-specific bus configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_frame_desc_entry {
    pub flags: v4l2_mbus_frame_desc_flags,
    pub stream: u32,
    pub pixelcode: u32,
    pub length: u32,
    pub csi2: v4l2_mbus_frame_desc_entry_csi2,
    pub bus: },
}

//
// If this number is too small, it should be dropped altogether and the
// API switched to a dynamic number of frame descriptor entries.
//
pub const V4L2_FRAME_DESC_ENTRY_MAX: c_int = 8;
//
// enum v4l2_mbus_frame_desc_type - media bus frame description type
//
// @V4L2_MBUS_FRAME_DESC_TYPE_UNDEFINED:
// Undefined frame desc type. Drivers should not use this, it is
// for backwards compatibility.
// @V4L2_MBUS_FRAME_DESC_TYPE_PARALLEL:
// Parallel media bus.
// @V4L2_MBUS_FRAME_DESC_TYPE_CSI2:
// CSI-2 media bus. Frame desc parameters must be set in
// &struct v4l2_mbus_frame_desc_entry->csi2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_mbus_frame_desc_type {
    V4L2_MBUS_FRAME_DESC_TYPE_UNDEFINED = 0,
    V4L2_MBUS_FRAME_DESC_TYPE_PARALLEL,
    V4L2_MBUS_FRAME_DESC_TYPE_CSI2,
}

//
// struct v4l2_mbus_frame_desc - media bus data frame description
// @type: type of the bus (enum v4l2_mbus_frame_desc_type)
// @entry: frame descriptors array
// @num_entries: number of entries in @entry array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_mbus_frame_desc {
    pub type: v4l2_mbus_frame_desc_type,
    pub entry: [v4l2_mbus_frame_desc_entry; V4L2_FRAME_DESC_ENTRY_MAX],
    pub num_entries: c_ushort,
}

//
// enum v4l2_subdev_pre_streamon_flags - Flags for pre_streamon subdev core op
//
// @V4L2_SUBDEV_PRE_STREAMON_FL_MANUAL_LP: Set the transmitter to either LP-11
// or LP-111 mode before call to s_stream().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_subdev_pre_streamon_flags {
    V4L2_SUBDEV_PRE_STREAMON_FL_MANUAL_LP = BIT(0),
}

//
// struct v4l2_subdev_video_ops - Callbacks used when v4l device was opened
// in video mode.
//
// @s_routing: see s_routing in audio_ops, except this version is for video
// devices.
//
// @s_crystal_freq: sets the frequency of the crystal used to generate the
// clocks in Hz. An extra flags field allows device specific configuration
// regarding clock frequency dividers, etc. If not used, then set flags
// to 0. If the frequency is not supported, then -EINVAL is returned.
//
// @g_std: callback for VIDIOC_G_STD() ioctl handler code.
//
// @s_std: callback for VIDIOC_S_STD() ioctl handler code.
//
// @s_std_output: set v4l2_std_id for video OUTPUT devices. This is ignored by
// video input devices.
//
// @g_std_output: get current standard for video OUTPUT devices. This is ignored
// by video input devices.
//
// @querystd: callback for VIDIOC_QUERYSTD() ioctl handler code.
//
// @g_tvnorms: get &v4l2_std_id with all standards supported by the video
// CAPTURE device. This is ignored by video output devices.
//
// @g_tvnorms_output: get v4l2_std_id with all standards supported by the video
// OUTPUT device. This is ignored by video capture devices.
//
// @g_input_status: get input status. Same as the status field in the
// &struct v4l2_input
//
// @s_stream: start (enabled == 1) or stop (enabled == 0) streaming on the
// sub-device. Failure on stop will remove any resources acquired in
// streaming start, while the error code is still returned by the driver.
// The caller shall track the subdev state, and shall not start or stop an
// already started or stopped subdev. Also see call_s_stream wrapper in
// v4l2-subdev.c.
//
// This callback is DEPRECATED. New drivers should instead implement
// &v4l2_subdev_pad_ops.enable_streams and
// &v4l2_subdev_pad_ops.disable_streams operations, and use
// v4l2_subdev_s_stream_helper for the &v4l2_subdev_video_ops.s_stream
// operation to support legacy users.
//
// Drivers should also not call the .s_stream() subdev operation directly,
// but use the v4l2_subdev_enable_streams() and
// v4l2_subdev_disable_streams() helpers.
//
// @s_rx_buffer: set a host allocated memory buffer for the subdev. The subdev
// can adjust @size to a lower value and must not write more data to the
// buffer starting at @data than the original value of @size.
//
// @pre_streamon: May be called before streaming is actually started, to help
// initialising the bus. Current usage is to set a CSI-2 transmitter to
// LP-11 or LP-111 mode before streaming. See &enum
// v4l2_subdev_pre_streamon_flags.
//
// pre_streamon shall return error if it cannot perform the operation as
// indicated by the flags argument. In particular, -EACCES indicates lack
// of support for the operation. The caller shall call post_streamoff for
// each successful call of pre_streamon.
//
// @post_streamoff: Called after streaming is stopped, but if and only if
// pre_streamon was called earlier.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_video_ops {
    pub config): *mut *mut *mut int (s_routing)(struct v4l2_subdev sd, u32 input, u32 output, u32,
    pub flags): *mut *mut *mut int (s_crystal_freq)(struct v4l2_subdev sd, u32 freq, u32,
    pub norm): *mut *mut *mut int (g_std)(struct v4l2_subdev sd, v4l2_std_id,
    pub norm): *mut *mut *mut int (s_std)(struct v4l2_subdev sd, v4l2_std_id,
    pub std): *mut *mut *mut int (s_std_output)(struct v4l2_subdev sd, v4l2_std_id,
    pub std): *mut *mut *mut int (g_std_output)(struct v4l2_subdev sd, v4l2_std_id,
    pub std): *mut *mut *mut int (querystd)(struct v4l2_subdev sd, v4l2_std_id,
    pub std): *mut *mut *mut int (g_tvnorms)(struct v4l2_subdev sd, v4l2_std_id,
    pub std): *mut *mut *mut int (g_tvnorms_output)(struct v4l2_subdev sd, v4l2_std_id,
    pub status): *mut *mut *mut int (g_input_status)(struct v4l2_subdev sd, u32,
    pub enable): *mut *mut *mut int (s_stream)(struct v4l2_subdev sd, int,
    pub size): *mut c_uint,
    pub flags): *mut *mut *mut int (pre_streamon)(struct v4l2_subdev sd, u32,
    pub sd): *mut *mut int (post_streamoff)(struct v4l2_subdev,
}

//
// struct v4l2_subdev_vbi_ops - Callbacks used when v4l device was opened
// in video mode via the vbi device node.
//
// @decode_vbi_line: video decoders that support sliced VBI need to implement
// this ioctl. Field p of the &struct v4l2_decode_vbi_line is set to the
// start of the VBI data that was generated by the decoder. The driver
// then parses the sliced VBI data and sets the other fields in the
// struct accordingly. The pointer p is updated to point to the start of
// the payload which can be copied verbatim into the data field of the
// &struct v4l2_sliced_vbi_data. If no valid VBI data was found, then the
// type field is set to 0 on return.
//
// @s_vbi_data: used to generate VBI signals on a video signal.
// &struct v4l2_sliced_vbi_data is filled with the data packets that
// should be output. Note that if you set the line field to 0, then that
// VBI signal is disabled. If no valid VBI data was found, then the type
// field is set to 0 on return.
//
// @g_vbi_data: used to obtain the sliced VBI packet from a readback register.
// Not all video decoders support this. If no data is available because
// the readback register contains invalid or erroneous data %-EIO is
// returned. Note that you must fill in the 'id' member and the 'field'
// member (to determine whether CC data from the first or second field
// should be obtained).
//
// @g_sliced_vbi_cap: callback for VIDIOC_G_SLICED_VBI_CAP() ioctl handler
// code.
//
// @s_raw_fmt: setup the video encoder/decoder for raw VBI.
//
// @g_sliced_fmt: retrieve the current sliced VBI settings.
//
// @s_sliced_fmt: setup the sliced VBI settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_vbi_ops {
    pub vbi_line): *mut *mut *mut int (decode_vbi_line)(struct v4l2_subdev sd, struct v4l2_decode_vbi_line,
    pub vbi_data): *const *const *const int (s_vbi_data)(struct v4l2_subdev sd, struct v4l2_sliced_vbi_data,
    pub vbi_data): *mut *mut *mut int (g_vbi_data)(struct v4l2_subdev sd, struct v4l2_sliced_vbi_data,
    pub cap): *mut *mut *mut int (g_sliced_vbi_cap)(struct v4l2_subdev sd, struct v4l2_sliced_vbi_cap,
    pub fmt): *mut *mut *mut int (s_raw_fmt)(struct v4l2_subdev sd, struct v4l2_vbi_format,
    pub fmt): *mut *mut *mut int (g_sliced_fmt)(struct v4l2_subdev sd, struct v4l2_sliced_vbi_format,
    pub fmt): *mut *mut *mut int (s_sliced_fmt)(struct v4l2_subdev sd, struct v4l2_sliced_vbi_format,
}

//
// struct v4l2_subdev_sensor_ops - v4l2-subdev sensor operations
// @g_skip_top_lines: number of lines at the top of the image to be skipped.
// This is needed for some sensors, which always corrupt
// several top lines of the output image, or which send their
// metadata in them.
// @g_skip_frames: number of frames to skip at stream start. This is needed for
// buggy sensors that generate faulty frames when they are
// turned on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_sensor_ops {
    pub lines): *mut *mut *mut int (g_skip_top_lines)(struct v4l2_subdev sd, u32,
    pub frames): *mut *mut *mut int (g_skip_frames)(struct v4l2_subdev sd, u32,
}

//
// enum v4l2_subdev_ir_mode- describes the type of IR supported
//
// @V4L2_SUBDEV_IR_MODE_PULSE_WIDTH: IR uses struct ir_raw_event records
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_subdev_ir_mode {
    V4L2_SUBDEV_IR_MODE_PULSE_WIDTH,
}

//
// struct v4l2_subdev_ir_parameters - Parameters for IR TX or TX
//
// @bytes_per_data_element: bytes per data element of data in read or
// write call.
// @mode: IR mode as defined by &enum v4l2_subdev_ir_mode.
// @enable: device is active if true
// @interrupt_enable: IR interrupts are enabled if true
// @shutdown: if true: set hardware to low/no power, false: normal mode
//
// @modulation: if true, it uses carrier, if false: baseband
// @max_pulse_width:  maximum pulse width in ns, valid only for baseband signal
// @carrier_freq: carrier frequency in Hz, valid only for modulated signal
// @duty_cycle: duty cycle percentage, valid only for modulated signal
// @invert_level: invert signal level
//
// @invert_carrier_sense: Send 0/space as a carrier burst. used only in TX.
//
// @noise_filter_min_width: min time of a valid pulse, in ns. Used only for RX.
// @carrier_range_lower: Lower carrier range, in Hz, valid only for modulated
// signal. Used only for RX.
// @carrier_range_upper: Upper carrier range, in Hz, valid only for modulated
// signal. Used only for RX.
// @resolution: The receive resolution, in ns . Used only for RX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_ir_parameters {
    pub bytes_per_data_element: c_uint,
    pub mode: v4l2_subdev_ir_mode,
    pub enable: bool,
    pub interrupt_enable: bool,
    pub shutdown: bool,
    pub modulation: bool,
    pub max_pulse_width: u32,
    pub carrier_freq: c_uint,
    pub duty_cycle: c_uint,
    pub invert_level: bool,
// Tx only
    pub invert_carrier_sense: bool,
// Rx only
    pub noise_filter_min_width: u32,
    pub carrier_range_lower: c_uint,
    pub carrier_range_upper: c_uint,
    pub resolution: u32,
}

//
// struct v4l2_subdev_ir_ops - operations for IR subdevices
//
// @rx_read: Reads received codes or pulse width data.
// The semantics are similar to a non-blocking read() call.
// @rx_g_parameters: Get the current operating parameters and state of
// the IR receiver.
// @rx_s_parameters: Set the current operating parameters and state of
// the IR receiver.  It is recommended to call
// [rt]x_g_parameters first to fill out the current state, and only change
// the fields that need to be changed.  Upon return, the actual device
// operating parameters and state will be returned.  Note that hardware
// limitations may prevent the actual settings from matching the requested
// settings - e.g. an actual carrier setting of 35,904 Hz when 36,000 Hz
// was requested.  An exception is when the shutdown parameter is true.
// The last used operational parameters will be returned, but the actual
// state of the hardware be different to minimize power consumption and
// processing when shutdown is true.
//
// @tx_write: Writes codes or pulse width data for transmission.
// The semantics are similar to a non-blocking write() call.
// @tx_g_parameters: Get the current operating parameters and state of
// the IR transmitter.
// @tx_s_parameters: Set the current operating parameters and state of
// the IR transmitter.  It is recommended to call
// [rt]x_g_parameters first to fill out the current state, and only change
// the fields that need to be changed.  Upon return, the actual device
// operating parameters and state will be returned.  Note that hardware
// limitations may prevent the actual settings from matching the requested
// settings - e.g. an actual carrier setting of 35,904 Hz when 36,000 Hz
// was requested.  An exception is when the shutdown parameter is true.
// The last used operational parameters will be returned, but the actual
// state of the hardware be different to minimize power consumption and
// processing when shutdown is true.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_ir_ops {
// Receiver
    pub num): *mut isize,
    pub params): *mut v4l2_subdev_ir_parameters,
    pub params): *mut v4l2_subdev_ir_parameters,
// Transmitter
    pub num): *mut isize,
    pub params): *mut v4l2_subdev_ir_parameters,
    pub params): *mut v4l2_subdev_ir_parameters,
}

//
// struct v4l2_subdev_pad_config - Used for storing subdev pad information.
//
// @format: &struct v4l2_mbus_framefmt
// @crop: &struct v4l2_rect to be used for crop
// @compose: &struct v4l2_rect to be used for compose
// @interval: frame interval
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_pad_config {
    pub format: v4l2_mbus_framefmt,
    pub crop: v4l2_rect,
    pub compose: v4l2_rect,
    pub interval: v4l2_fract,
}

//
// struct v4l2_subdev_stream_configs - A collection of stream configs.
//
// @num_configs: number of entries in @config.
// @configs: an array of &struct v4l2_subdev_stream_configs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_stream_configs {
    pub num_configs: u32,
    pub configs: *mut v4l2_subdev_stream_config,
}

//
// struct v4l2_subdev_krouting - subdev routing table
//
// @len_routes: length of routes array, in routes
// @num_routes: number of routes
// @routes: &struct v4l2_subdev_route
//
// This structure contains the routing table for a subdev.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_krouting {
    pub len_routes: c_uint,
    pub num_routes: c_uint,
    pub routes: *mut v4l2_subdev_route,
}

//
// struct v4l2_subdev_state - Used for storing subdev state information.
//
// @_lock: default for 'lock'
// @lock: mutex for the state. May be replaced by the user.
// @sd: the sub-device which the state is related to
// @pads: &struct v4l2_subdev_pad_config array
// @routing: routing table for the subdev
// @stream_configs: stream configurations (only for V4L2_SUBDEV_FL_STREAMS)
//
// This structure only needs to be passed to the pad op if the 'which' field
// of the main argument is set to %V4L2_SUBDEV_FORMAT_TRY. For
// %V4L2_SUBDEV_FORMAT_ACTIVE it is safe to pass %NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_state {
// lock for the struct v4l2_subdev_state fields
    pub _lock: mutex,
    pub lock: *mut mutex,
    pub sd: *mut v4l2_subdev,
    pub pads: *mut v4l2_subdev_pad_config,
    pub routing: v4l2_subdev_krouting,
    pub stream_configs: v4l2_subdev_stream_configs,
}

//
// struct v4l2_subdev_pad_ops - v4l2-subdev pad level operations
//
// @enum_mbus_code: callback for VIDIOC_SUBDEV_ENUM_MBUS_CODE() ioctl handler
// code.
// @enum_frame_size: callback for VIDIOC_SUBDEV_ENUM_FRAME_SIZE() ioctl handler
// code.
//
// @enum_frame_interval: callback for VIDIOC_SUBDEV_ENUM_FRAME_INTERVAL() ioctl
// handler code.
//
// @get_fmt: callback for VIDIOC_SUBDEV_G_FMT() ioctl handler code.
//
// @set_fmt: callback for VIDIOC_SUBDEV_S_FMT() ioctl handler code.
//
// @get_selection: callback for VIDIOC_SUBDEV_G_SELECTION() ioctl handler code.
//
// @set_selection: callback for VIDIOC_SUBDEV_S_SELECTION() ioctl handler code.
//
// @get_frame_interval: callback for VIDIOC_SUBDEV_G_FRAME_INTERVAL()
// ioctl handler code.
//
// @set_frame_interval: callback for VIDIOC_SUBDEV_S_FRAME_INTERVAL()
// ioctl handler code.
//
// @get_edid: callback for VIDIOC_SUBDEV_G_EDID() ioctl handler code.
//
// @set_edid: callback for VIDIOC_SUBDEV_S_EDID() ioctl handler code.
//
// @s_dv_timings: Set custom dv timings in the sub device. This is used
// when sub device is capable of setting detailed timing information
// in the hardware to generate/detect the video signal.
//
// @g_dv_timings: Get custom dv timings in the sub device.
//
// @query_dv_timings: callback for VIDIOC_QUERY_DV_TIMINGS() ioctl handler code.
//
// @dv_timings_cap: callback for VIDIOC_SUBDEV_DV_TIMINGS_CAP() ioctl handler
// code.
//
// @enum_dv_timings: callback for VIDIOC_SUBDEV_ENUM_DV_TIMINGS() ioctl handler
// code.
//
// @link_validate: used by the media controller code to check if the links
// that belongs to a pipeline can be used for stream.
//
// @get_frame_desc: get the current low level media bus frame parameters.
//
// @set_frame_desc: set the low level media bus frame parameters, @fd array
// may be adjusted by the subdev driver to device capabilities.
//
// @get_mbus_config: get the media bus configuration of a remote sub-device.
// The media bus configuration is usually retrieved from the
// firmware interface at sub-device probe time, immediately
// applied to the hardware and eventually adjusted by the
// driver. Remote sub-devices (usually video receivers) shall
// use this operation to query the transmitting end bus
// configuration in order to adjust their own one accordingly.
// Callers should make sure they get the most up-to-date as
// possible configuration from the remote end, likely calling
// this operation as close as possible to stream on time. The
// operation shall fail if the pad index it has been called on
// is not valid or in case of unrecoverable failures. The
// config argument has been memset to 0 just before calling
// the op.
//
// @set_routing: Enable or disable data connection routes described in the
// subdevice routing table. Subdevs that implement this operation
// must set the V4L2_SUBDEV_FL_STREAMS flag.
//
// @enable_streams: Enable the streams defined in streams_mask on the given
// source pad. Subdevs that implement this operation must use the active
// state management provided by the subdev core (enabled through a call to
// v4l2_subdev_init_finalize() at initialization time). Do not call
// directly, use v4l2_subdev_enable_streams() instead.
//
// Drivers that support only a single stream without setting the
// V4L2_SUBDEV_CAP_STREAMS sub-device capability flag can ignore the mask
// argument.
//
// @disable_streams: Disable the streams defined in streams_mask on the given
// source pad. Subdevs that implement this operation must use the active
// state management provided by the subdev core (enabled through a call to
// v4l2_subdev_init_finalize() at initialization time). Do not call
// directly, use v4l2_subdev_disable_streams() instead.
//
// Drivers that support only a single stream without setting the
// V4L2_SUBDEV_CAP_STREAMS sub-device capability flag can ignore the mask
// argument.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_pad_ops {
    pub code): *mut v4l2_subdev_mbus_code_enum,
    pub fse): *mut v4l2_subdev_frame_size_enum,
    pub fie): *mut v4l2_subdev_frame_interval_enum,
    pub format): *mut v4l2_subdev_format,
    pub format): *mut v4l2_subdev_format,
    pub sel): *mut v4l2_subdev_selection,
    pub sel): *mut v4l2_subdev_selection,
    pub interval): *mut v4l2_subdev_frame_interval,
    pub interval): *mut v4l2_subdev_frame_interval,
    pub edid): *mut *mut *mut int (get_edid)(struct v4l2_subdev sd, struct v4l2_edid,
    pub edid): *mut *mut *mut int (set_edid)(struct v4l2_subdev sd, struct v4l2_edid,
    pub timings): *mut v4l2_dv_timings,
    pub timings): *mut v4l2_dv_timings,
    pub timings): *mut v4l2_dv_timings,
    pub cap): *mut v4l2_dv_timings_cap,
    pub timings): *mut v4l2_enum_dv_timings,

    pub sink_fmt): *mut v4l2_subdev_format,

    pub fd): *mut v4l2_mbus_frame_desc,
    pub fd): *mut v4l2_mbus_frame_desc,
    pub config): *mut v4l2_mbus_config,
    pub route): *mut v4l2_subdev_krouting,
    pub streams_mask): u64,
    pub streams_mask): u64,
}

//
// struct v4l2_subdev_ops - Subdev operations
//
// @core: pointer to &struct v4l2_subdev_core_ops. Can be %NULL
// @tuner: pointer to &struct v4l2_subdev_tuner_ops. Can be %NULL
// @audio: pointer to &struct v4l2_subdev_audio_ops. Can be %NULL
// @video: pointer to &struct v4l2_subdev_video_ops. Can be %NULL
// @vbi: pointer to &struct v4l2_subdev_vbi_ops. Can be %NULL
// @ir: pointer to &struct v4l2_subdev_ir_ops. Can be %NULL
// @sensor: pointer to &struct v4l2_subdev_sensor_ops. Can be %NULL
// @pad: pointer to &struct v4l2_subdev_pad_ops. Can be %NULL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_ops {
    pub core: *const v4l2_subdev_core_ops,
    pub tuner: *const v4l2_subdev_tuner_ops,
    pub audio: *const v4l2_subdev_audio_ops,
    pub video: *const v4l2_subdev_video_ops,
    pub vbi: *const v4l2_subdev_vbi_ops,
    pub ir: *const v4l2_subdev_ir_ops,
    pub sensor: *const v4l2_subdev_sensor_ops,
    pub pad: *const v4l2_subdev_pad_ops,
}

//
// struct v4l2_subdev_internal_ops - V4L2 subdev internal ops
//
// @init_state: initialize the subdev state to default values
//
// @registered: called when this subdev is registered. When called the v4l2_dev
// field is set to the correct v4l2_device.
//
// @unregistered: called when this subdev is unregistered. When called the
// v4l2_dev field is still set to the correct v4l2_device.
//
// @open: called when the subdev device node is opened by an application.
//
// @close: called when the subdev device node is closed. Please note that
// it is possible for @close to be called after @unregistered!
//
// @release: called when the last user of the subdev device is gone. This
// happens after the @unregistered callback and when the last open
// filehandle to the v4l-subdevX device node was closed. If no device
// node was created for this sub-device, then the @release callback
// is called right after the @unregistered callback.
// The @release callback is typically used to free the memory containing
// the v4l2_subdev structure. It is almost certainly required for any
// sub-device that sets the V4L2_SUBDEV_FL_HAS_DEVNODE flag.
//
// .. note::
// Never call this from drivers, only the v4l2 framework can call
// these ops.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_internal_ops {
    pub state): *mut v4l2_subdev_state,
    pub sd): *mut *mut int (registered)(struct v4l2_subdev,
    pub sd): *mut *mut void (unregistered)(struct v4l2_subdev,
    pub fh): *mut *mut *mut int (open)(struct v4l2_subdev sd, struct v4l2_subdev_fh,
    pub fh): *mut *mut *mut int (close)(struct v4l2_subdev sd, struct v4l2_subdev_fh,
    pub sd): *mut *mut void (release)(struct v4l2_subdev,
}

// Set this flag if this subdev is a i2c device.

// Set this flag if this subdev is a spi device.

// Set this flag if this subdev needs a device node.

//
// Set this flag if this subdev generates events.
// Note controls can send events, thus drivers exposing controls
// should set this flag.
//

//
// Set this flag if this subdev supports multiplexed streams. This means
// that the driver supports routing and handles the stream parameter in its
// v4l2_subdev_pad_ops handlers. More specifically, this means:
//
// - Centrally managed subdev active state is enabled
// - Legacy pad config is _not_ supported (state->pads is NULL)
// - Routing ioctls are available
// - Multiple streams per pad are supported
//

//
// struct v4l2_subdev_platform_data - regulators config struct
//
// @regulators: Optional regulators used to power on/off the subdevice
// @num_regulators: Number of regululators
// @host_priv: Per-subdevice data, specific for a certain video host device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_platform_data {
    pub regulators: *mut regulator_bulk_data,
    pub num_regulators: c_int,
    pub host_priv: *mut c_void,
}

//
// struct v4l2_subdev - describes a V4L2 sub-device
//
// @entity: pointer to &struct media_entity
// @list: List of sub-devices
// @owner: The owner is the same as the driver's &struct device owner.
// @owner_v4l2_dev: true if the &sd->owner matches the owner of @v4l2_dev->dev
// owner. Initialized by v4l2_device_register_subdev().
// @flags: subdev flags. Can be:
// %V4L2_SUBDEV_FL_IS_I2C - Set this flag if this subdev is a i2c device;
// %V4L2_SUBDEV_FL_IS_SPI - Set this flag if this subdev is a spi device;
// %V4L2_SUBDEV_FL_HAS_DEVNODE - Set this flag if this subdev needs a
// device node;
// %V4L2_SUBDEV_FL_HAS_EVENTS -  Set this flag if this subdev generates
// events.
//
// @v4l2_dev: pointer to struct &v4l2_device
// @ops: pointer to struct &v4l2_subdev_ops
// @internal_ops: pointer to struct &v4l2_subdev_internal_ops.
// Never call these internal ops from within a driver!
// @ctrl_handler: The control handler of this subdev. May be NULL.
// @name: Name of the sub-device. Please notice that the name must be unique.
// @grp_id: can be used to group similar subdevs. Value is driver-specific
// @dev_priv: pointer to private data
// @host_priv: pointer to private data used by the device where the subdev
// is attached.
// @devnode: subdev device node
// @dev: pointer to the physical device, if any
// @fwnode: The fwnode_handle of the subdev, usually the same as
// either dev->of_node->fwnode or dev->fwnode (whichever is non-NULL).
// @async_list: Links this subdev to a global subdev_list or
// @notifier->done_list list.
// @async_subdev_endpoint_list: List entry in async_subdev_endpoint_entry of
// &struct v4l2_async_subdev_endpoint.
// @subdev_notifier: A sub-device notifier implicitly registered for the sub-
// device using v4l2_async_register_subdev_sensor().
// @asc_list: Async connection list, of &struct
// v4l2_async_connection.subdev_entry.
// @pdata: common part of subdevice platform data
// @state_lock: A pointer to a lock used for all the subdev's states, set by the
// driver. This is	optional. If NULL, each state instance will get
// a lock of its own.
// @privacy_led: Optional pointer to a LED classdev for the privacy LED for sensors.
// @active_state: Active state for the subdev (NULL for subdevs tracking the
// state internally). Initialized by calling
// v4l2_subdev_init_finalize().
// @enabled_pads: Bitmask of enabled pads used by v4l2_subdev_enable_streams()
// and v4l2_subdev_disable_streams() helper functions for
// fallback cases.
// @s_stream_enabled: Tracks whether streaming has been enabled with s_stream.
// This is only for call_s_stream() internal use.
//
// Each instance of a subdev driver should create this struct, either
// stand-alone or embedded in a larger struct.
//
// This structure should be initialized by v4l2_subdev_init() or one of
// its variants: v4l2_spi_subdev_init(), v4l2_i2c_subdev_init().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev {

    pub entity: media_entity,

    pub list: list_head,
    pub owner: *mut module,
    pub owner_v4l2_dev: bool,
    pub flags: u32,
    pub v4l2_dev: *mut v4l2_device,
    pub ops: *const v4l2_subdev_ops,
    pub internal_ops: *const v4l2_subdev_internal_ops,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub name: [c_char; 52],
    pub grp_id: u32,
    pub dev_priv: *mut c_void,
    pub host_priv: *mut c_void,
    pub devnode: *mut video_device,
    pub dev: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub async_list: list_head,
    pub async_subdev_endpoint_list: list_head,
    pub subdev_notifier: *mut v4l2_async_notifier,
    pub asc_list: list_head,
    pub pdata: *mut v4l2_subdev_platform_data,
    pub state_lock: *mut mutex,
//
// The fields below are private, and should only be accessed via
// appropriate functions.
//
    pub privacy_led: *mut led_classdev,
//
// TODO: active_state should most likely be changed from a pointer to an
// embedded field. For the time being it's kept as a pointer to more
// easily catch uses of active_state in the cases where the driver
// doesn't support it.
//
    pub active_state: *mut v4l2_subdev_state,
    pub enabled_pads: u64,
    pub s_stream_enabled: bool,
}

//
// media_entity_to_v4l2_subdev - Returns a &struct v4l2_subdev from
// the &struct media_entity embedded in it.
//
// @ent: pointer to &struct media_entity.
//

//
// vdev_to_v4l2_subdev - Returns a &struct v4l2_subdev from
// the &struct video_device embedded on it.
//
// @vdev: pointer to &struct video_device
//

//
// struct v4l2_subdev_fh - Used for storing subdev information per file handle
//
// @vfh: pointer to &struct v4l2_fh
// @state: pointer to &struct v4l2_subdev_state
// @owner: module pointer to the owner of this file handle
// @client_caps: bitmask of ``V4L2_SUBDEV_CLIENT_CAP_*``
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_subdev_fh {
    pub vfh: v4l2_fh,
    pub owner: *mut module,

    pub state: *mut v4l2_subdev_state,
    pub client_caps: u64,

}

//
// to_v4l2_subdev_fh - Returns a &struct v4l2_subdev_fh from
// the &struct v4l2_fh embedded on it.
//
// @fh: pointer to &struct v4l2_fh
//

//
// v4l2_set_subdevdata - Sets V4L2 dev private device data
//
// @sd: pointer to &struct v4l2_subdev
// @p: pointer to the private device data to be stored.
//
// v4l2_get_subdevdata - Gets V4L2 dev private device data
//
// @sd: pointer to &struct v4l2_subdev
//
// Returns the pointer to the private device data to be stored.
//
// v4l2_set_subdev_hostdata - Sets V4L2 dev private host data
//
// @sd: pointer to &struct v4l2_subdev
// @p: pointer to the private data to be stored.
//
// v4l2_get_subdev_hostdata - Gets V4L2 dev private data
//
// @sd: pointer to &struct v4l2_subdev
//
// Returns the pointer to the private host data to be stored.
//

//
// v4l2_subdev_get_fwnode_pad_1_to_1 - Get pad number from a subdev fwnode
// endpoint, assuming 1:1 port:pad
//
// @entity: Pointer to the subdev entity
// @endpoint: Pointer to a parsed fwnode endpoint
//
// This function can be used as the .get_fwnode_pad operation for
// subdevices that map port numbers and pad indexes 1:1. If the endpoint
// is owned by the subdevice, the function returns the endpoint port
// number.
//
// Returns the endpoint port number on success or a negative error code.
//
// v4l2_subdev_link_validate_default - validates a media link
//
// @sd: pointer to &struct v4l2_subdev
// @link: pointer to &struct media_link
// @source_fmt: pointer to &struct v4l2_subdev_format
// @sink_fmt: pointer to &struct v4l2_subdev_format
//
// This function ensures that width, height and the media bus pixel
// code are equal on both source and sink of the link.
//
// v4l2_subdev_link_validate - validates a media link
//
// @link: pointer to &struct media_link
//
// This function calls the subdev's link_validate ops to validate
// if a media link is valid for streaming. It also internally
// calls v4l2_subdev_link_validate_default() to ensure that
// width, height and the media bus pixel code are equal on both
// source and sink of the link.
//
// The function can be used as a drop-in &media_entity_ops.link_validate
// implementation for v4l2_subdev instances. It supports all links between
// subdevs, as well as links between subdevs and video devices, provided that
// the video devices also implement their &media_entity_ops.link_validate
// operation.
//
extern "C" {
    pub fn v4l2_subdev_link_validate(link: *mut media_link) -> c_int;
}
//
// v4l2_subdev_has_pad_interdep - MC has_pad_interdep implementation for subdevs
//
// @entity: pointer to &struct media_entity
// @pad0: pad number for the first pad
// @pad1: pad number for the second pad
//
// This function is an implementation of the
// media_entity_operations.has_pad_interdep operation for subdevs that
// implement the multiplexed streams API (as indicated by the
// V4L2_SUBDEV_FL_STREAMS subdev flag).
//
// It considers two pads interdependent if there is an active route between pad0
// and pad1.
//
// __v4l2_subdev_state_alloc - allocate v4l2_subdev_state
//
// @sd: pointer to &struct v4l2_subdev for which the state is being allocated.
// @lock_name: name of the state lock
// @key: lock_class_key for the lock
//
// Must call __v4l2_subdev_state_free() when state is no longer needed.
//
// Not to be called directly by the drivers.
//
// __v4l2_subdev_state_free - free a v4l2_subdev_state
//
// @state: v4l2_subdev_state to be freed.
//
// Not to be called directly by the drivers.
//
extern "C" {
    pub fn __v4l2_subdev_state_free(state: *mut v4l2_subdev_state);
}
//
// v4l2_subdev_init_finalize() - Finalizes the initialization of the subdevice
// @sd: The subdev
//
// This function finalizes the initialization of the subdev, including
// allocation of the active state for the subdev.
//
// This function must be called by the subdev drivers that use the centralized
// active state, after the subdev struct has been initialized and
// media_entity_pads_init() has been called, but before registering the
// subdev.
//
// The user must call v4l2_subdev_cleanup() when the subdev is being removed.
//

//
// v4l2_subdev_cleanup() - Releases the resources allocated by the subdevice
// @sd: The subdevice
//
// Clean up a V4L2 async sub-device. Must be called for a sub-device as part of
// its release if resources have been associated with it using
// v4l2_async_subdev_endpoint_add() or v4l2_subdev_init_finalize().
//
extern "C" {
    pub fn v4l2_subdev_cleanup(sd: *mut v4l2_subdev);
}
//
// A macro to generate the macro or function name for sub-devices state access
// wrapper macros below.
//

//
// A macro to constify the return value of the state accessors when the state
// parameter is const.
//

//
// v4l2_subdev_state_get_format() - Get pointer to a stream format
// @state: subdevice state
// @pad: pad id
// @...: stream id (optional argument)
//
// This returns a pointer to &struct v4l2_mbus_framefmt for the given pad +
// stream in the subdev state.
//
// For stream-unaware drivers the format for the corresponding pad is returned.
// If the pad does not exist, NULL is returned.
//
// Wrap v4l2_subdev_state_get_format(), allowing the function to be called with
// two or three arguments. The purpose of the __v4l2_subdev_state_gen_call()
// macro is to come up with the name of the function or macro to call, using
// the last two arguments (_stream and _pad). The selected function or macro is
// then called using the arguments specified by the caller. The
// __v4l2_subdev_state_constify_ret() macro constifies the returned pointer
// when the state is const, allowing the state accessors to guarantee
// const-correctness in all cases.
//
// A similar arrangement is used for v4l2_subdev_state_crop(),
// v4l2_subdev_state_compose() and v4l2_subdev_state_get_interval() below.
//

//
// v4l2_subdev_state_get_crop() - Get pointer to a stream crop rectangle
// @state: subdevice state
// @pad: pad id
// @...: stream id (optional argument)
//
// This returns a pointer to crop rectangle for the given pad + stream in the
// subdev state.
//
// For stream-unaware drivers the crop rectangle for the corresponding pad is
// returned. If the pad does not exist, NULL is returned.
//

//
// v4l2_subdev_state_get_compose() - Get pointer to a stream compose rectangle
// @state: subdevice state
// @pad: pad id
// @...: stream id (optional argument)
//
// This returns a pointer to compose rectangle for the given pad + stream in the
// subdev state.
//
// For stream-unaware drivers the compose rectangle for the corresponding pad is
// returned. If the pad does not exist, NULL is returned.
//

//
// v4l2_subdev_state_get_interval() - Get pointer to a stream frame interval
// @state: subdevice state
// @pad: pad id
// @...: stream id (optional argument)
//
// This returns a pointer to the frame interval for the given pad + stream in
// the subdev state.
//
// For stream-unaware drivers the frame interval for the corresponding pad is
// returned. If the pad does not exist, NULL is returned.
//

//
// v4l2_subdev_get_fmt() - Fill format based on state
// @sd: subdevice
// @state: subdevice state
// @format: pointer to &struct v4l2_subdev_format
//
// Fill @format->format field based on the information in the @format struct.
//
// This function can be used by the subdev drivers which support active state to
// implement v4l2_subdev_pad_ops.get_fmt if the subdev driver does not need to
// do anything special in their get_fmt op.
//
// Returns 0 on success, error value otherwise.
//
// v4l2_subdev_get_frame_interval() - Fill frame interval based on state
// @sd: subdevice
// @state: subdevice state
// @fi: pointer to &struct v4l2_subdev_frame_interval
//
// Fill @fi->interval field based on the information in the @fi struct.
//
// This function can be used by the subdev drivers which support active state to
// implement v4l2_subdev_pad_ops.get_frame_interval if the subdev driver does
// not need to do anything special in their get_frame_interval op.
//
// Returns 0 on success, error value otherwise.
//
// v4l2_subdev_set_routing() - Set given routing to subdev state
// @sd: The subdevice
// @state: The subdevice state
// @routing: Routing that will be copied to subdev state
//
// This will release old routing table (if any) from the state, allocate
// enough space for the given routing, and copy the routing.
//
// This can be used from the subdev driver's set_routing op, after validating
// the routing.
//
// for_each_active_route - iterate on all active routes of a routing table
// @routing: The routing table
// @route: The route iterator
//

//
// v4l2_subdev_set_routing_with_fmt() - Set given routing and format to subdev
// state
// @sd: The subdevice
// @state: The subdevice state
// @routing: Routing that will be copied to subdev state
// @fmt: Format used to initialize all the streams
//
// This is the same as v4l2_subdev_set_routing, but additionally initializes
// all the streams using the given format.
//
// v4l2_subdev_routing_find_opposite_end() - Find the opposite stream
// @routing: routing used to find the opposite side
// @pad: pad id
// @stream: stream id
// @other_pad: pointer used to return the opposite pad
// @other_stream: pointer used to return the opposite stream
//
// This function uses the routing table to find the pad + stream which is
// opposite the given pad + stream.
//
// @other_pad and/or @other_stream can be NULL if the caller does not need the
// value.
//
// Returns 0 on success, or -EINVAL if no matching route is found.
//
// v4l2_subdev_state_get_opposite_stream_format() - Get pointer to opposite
// stream format
// @state: subdevice state
// @pad: pad id
// @stream: stream id
//
// This returns a pointer to &struct v4l2_mbus_framefmt for the pad + stream
// that is opposite the given pad + stream in the subdev state.
//
// If the state does not contain the given pad + stream, NULL is returned.
//
// v4l2_subdev_state_xlate_streams() - Translate streams from one pad to another
//
// @state: Subdevice state
// @pad0: The first pad
// @pad1: The second pad
// @streams: Streams bitmask on the first pad
//
// Streams on sink pads of a subdev are routed to source pads as expressed in
// the subdev state routing table. Stream numbers don't necessarily match on
// the sink and source side of a route. This function translates stream numbers
// on @pad0, expressed as a bitmask in @streams, to the corresponding streams
// on @pad1 using the routing table from the @state. It returns the stream mask
// on @pad1, and updates @streams with the streams that have been found in the
// routing table.
//
// @pad0 and @pad1 must be a sink and a source, in any order.
//
// Return: The bitmask of streams of @pad1 that are routed to @streams on @pad0.
//
// enum v4l2_subdev_routing_restriction - Subdevice internal routing restrictions
//
// @V4L2_SUBDEV_ROUTING_NO_1_TO_N:
// an input stream shall not be routed to multiple output streams (stream
// duplication)
// @V4L2_SUBDEV_ROUTING_NO_N_TO_1:
// multiple input streams shall not be routed to the same output stream
// (stream merging)
// @V4L2_SUBDEV_ROUTING_NO_SINK_STREAM_MIX:
// all streams from a sink pad must be routed to a single source pad
// @V4L2_SUBDEV_ROUTING_NO_SOURCE_STREAM_MIX:
// all streams on a source pad must originate from a single sink pad
// @V4L2_SUBDEV_ROUTING_NO_SOURCE_MULTIPLEXING:
// source pads shall not contain multiplexed streams
// @V4L2_SUBDEV_ROUTING_NO_SINK_MULTIPLEXING:
// sink pads shall not contain multiplexed streams
// @V4L2_SUBDEV_ROUTING_ONLY_1_TO_1:
// only non-overlapping 1-to-1 stream routing is allowed (a combination of
// @V4L2_SUBDEV_ROUTING_NO_1_TO_N and @V4L2_SUBDEV_ROUTING_NO_N_TO_1)
// @V4L2_SUBDEV_ROUTING_NO_STREAM_MIX:
// all streams from a sink pad must be routed to a single source pad, and
// that source pad shall not get routes from any other sink pad
// (a combination of @V4L2_SUBDEV_ROUTING_NO_SINK_STREAM_MIX and
// @V4L2_SUBDEV_ROUTING_NO_SOURCE_STREAM_MIX)
// @V4L2_SUBDEV_ROUTING_NO_MULTIPLEXING:
// no multiplexed streams allowed on either source or sink sides.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_subdev_routing_restriction {
    V4L2_SUBDEV_ROUTING_NO_1_TO_N = BIT(0),
    V4L2_SUBDEV_ROUTING_NO_N_TO_1 = BIT(1),
    V4L2_SUBDEV_ROUTING_NO_SINK_STREAM_MIX = BIT(2),
    V4L2_SUBDEV_ROUTING_NO_SOURCE_STREAM_MIX = BIT(3),
    V4L2_SUBDEV_ROUTING_NO_SINK_MULTIPLEXING = BIT(4),
    V4L2_SUBDEV_ROUTING_NO_SOURCE_MULTIPLEXING = BIT(5),
    V4L2_SUBDEV_ROUTING_ONLY_1_TO_1 =
    V4L2_SUBDEV_ROUTING_NO_1_TO_N |
    V4L2_SUBDEV_ROUTING_NO_N_TO_1,
    V4L2_SUBDEV_ROUTING_NO_STREAM_MIX =
    V4L2_SUBDEV_ROUTING_NO_SINK_STREAM_MIX |
    V4L2_SUBDEV_ROUTING_NO_SOURCE_STREAM_MIX,
    V4L2_SUBDEV_ROUTING_NO_MULTIPLEXING =
    V4L2_SUBDEV_ROUTING_NO_SINK_MULTIPLEXING |
    V4L2_SUBDEV_ROUTING_NO_SOURCE_MULTIPLEXING,
}

//
// v4l2_subdev_routing_validate() - Verify that routes comply with driver
// constraints
// @sd: The subdevice
// @routing: Routing to verify
// @disallow: Restrictions on routes
//
// This verifies that the given routing complies with the @disallow constraints.
//
// Returns 0 on success, error value otherwise.
//
// v4l2_subdev_enable_streams() - Enable streams on a pad
// @sd: The subdevice
// @pad: The pad
// @streams_mask: Bitmask of streams to enable
//
// This function enables streams on a source @pad of a subdevice. The pad is
// identified by its index, while the streams are identified by the
// @streams_mask bitmask. This allows enabling multiple streams on a pad at
// once.
//
// Enabling a stream that is already enabled isn't allowed. If @streams_mask
// contains an already enabled stream, this function returns -EALREADY without
// performing any operation.
//
// Per-stream enable is only available for subdevs that implement the
// .enable_streams() and .disable_streams() operations. For other subdevs, this
// function implements a best-effort compatibility by calling the .s_stream()
// operation, limited to subdevs that have a single source pad.
//
// Drivers that are not stream-aware shall set @streams_mask to BIT_ULL(0).
//
// Return:
// * 0: Success
// * -EALREADY: One of the streams in streams_mask is already enabled
// * -EINVAL: The pad index is invalid, or doesn't correspond to a source pad
// * -EOPNOTSUPP: Falling back to the legacy .s_stream() operation is
// impossible because the subdev has multiple source pads
//
// v4l2_subdev_disable_streams() - Disable streams on a pad
// @sd: The subdevice
// @pad: The pad
// @streams_mask: Bitmask of streams to disable
//
// This function disables streams on a source @pad of a subdevice. The pad is
// identified by its index, while the streams are identified by the
// @streams_mask bitmask. This allows disabling multiple streams on a pad at
// once.
//
// Disabling a streams that is not enabled isn't allowed. If @streams_mask
// contains a disabled stream, this function returns -EALREADY without
// performing any operation.
//
// Per-stream disable is only available for subdevs that implement the
// .enable_streams() and .disable_streams() operations. For other subdevs, this
// function implements a best-effort compatibility by calling the .s_stream()
// operation, limited to subdevs that have a single source pad.
//
// Drivers that are not stream-aware shall set @streams_mask to BIT_ULL(0).
//
// Return:
// * 0: Success
// * -EALREADY: One of the streams in streams_mask is not enabled
// * -EINVAL: The pad index is invalid, or doesn't correspond to a source pad
// * -EOPNOTSUPP: Falling back to the legacy .s_stream() operation is
// impossible because the subdev has multiple source pads
//
// v4l2_subdev_s_stream_helper() - Helper to implement the subdev s_stream
// operation using enable_streams and disable_streams
// @sd: The subdevice
// @enable: Enable or disable streaming
//
// Subdevice drivers that implement the streams-aware
// &v4l2_subdev_pad_ops.enable_streams and &v4l2_subdev_pad_ops.disable_streams
// operations can use this helper to implement the legacy
// &v4l2_subdev_video_ops.s_stream operation.
//
// This helper can only be used by subdevs that have a single source pad.
//
// Return: 0 on success, or a negative error code otherwise.
//
extern "C" {
    pub fn v4l2_subdev_s_stream_helper(sd: *mut v4l2_subdev, enable: c_int) -> c_int;
}
//
// __v4l2_subdev_get_frame_desc_passthrough - Helper to implement the
// subdev get_frame_desc operation in simple passthrough cases
// @sd: The subdevice
// @state: The locked subdevice active state
// @pad: The source pad index
// @fd: The mbus frame desc
//
// This helper implements the get_frame_desc operation for subdevices that pass
// streams through without modification.
//
// The helper iterates over the subdevice's sink pads, calls get_frame_desc on
// the remote subdevice connected to each sink pad, and collects the frame desc
// entries for streams that are routed to the given source pad according to the
// subdevice's routing table. Each entry is copied as-is from the upstream
// source, with the exception of the 'stream' field which is remapped to the
// source stream ID from the routing table.
//
// The frame desc type is taken from the first upstream source. If multiple
// sink pads are involved and the upstream sources report different frame desc
// types, -EPIPE is returned.
//
// The caller must hold the subdevice's active state lock. This variant is
// intended for drivers that need to perform additional work around the
// passthrough frame descriptor collection. Drivers that do not need any
// customization should use v4l2_subdev_get_frame_desc_passthrough() instead.
//
// Return: 0 on success, or a negative error code otherwise.
//
// v4l2_subdev_get_frame_desc_passthrough() - Helper to implement the subdev
// get_frame_desc operation in simple passthrough cases
// @sd: The subdevice
// @pad: The source pad index
// @fd: The mbus frame desc
//
// This function locks the subdevice's active state, calls
// __v4l2_subdev_get_frame_desc_passthrough(), and unlocks the state.
//
// This function can be assigned directly as the .get_frame_desc callback in
// &v4l2_subdev_pad_ops for subdevices that pass streams through without
// modification. Drivers that need to perform additional work should use
// __v4l2_subdev_get_frame_desc_passthrough() in their custom
// .get_frame_desc implementation instead.
//
// Return: 0 on success, or a negative error code otherwise.
//

//
// v4l2_subdev_lock_state() - Locks the subdev state
// @state: The subdevice state
//
// Locks the given subdev state.
//
// The state must be unlocked with v4l2_subdev_unlock_state() after use.
//
// v4l2_subdev_unlock_state() - Unlocks the subdev state
// @state: The subdevice state
//
// Unlocks the given subdev state.
//
// v4l2_subdev_lock_states - Lock two sub-device states
// @state1: One subdevice state
// @state2: The other subdevice state
//
// Locks the state of two sub-devices.
//
// The states must be unlocked with v4l2_subdev_unlock_states() after use.
//
// This differs from calling v4l2_subdev_lock_state() on both states so that if
// the states share the same lock, the lock is acquired only once (so no
// deadlock occurs). The caller is responsible for ensuring the locks will
// always be acquired in the same order.
//
// v4l2_subdev_unlock_states() - Unlock two sub-device states
// @state1: One subdevice state
// @state2: The other subdevice state
//
// Unlocks the state of two sub-devices.
//
// This differs from calling v4l2_subdev_unlock_state() on both states so that
// if the states share the same lock, the lock is released only once.
//
// v4l2_subdev_get_unlocked_active_state() - Checks that the active subdev state
// is unlocked and returns it
// @sd: The subdevice
//
// Returns the active state for the subdevice, or NULL if the subdev does not
// support active state. If the state is not NULL, calls
// lockdep_assert_not_held() to issue a warning if the state is locked.
//
// This function is to be used e.g. when getting the active state for the sole
// purpose of passing it forward, without accessing the state fields.
//
// v4l2_subdev_get_locked_active_state() - Checks that the active subdev state
// is locked and returns it
//
// @sd: The subdevice
//
// Returns the active state for the subdevice, or NULL if the subdev does not
// support active state. If the state is not NULL, calls lockdep_assert_held()
// to issue a warning if the state is not locked.
//
// This function is to be used when the caller knows that the active state is
// already locked.
//
// v4l2_subdev_lock_and_get_active_state() - Locks and returns the active subdev
// state for the subdevice
// @sd: The subdevice
//
// Returns the locked active state for the subdevice, or NULL if the subdev
// does not support active state.
//
// The state must be unlocked with v4l2_subdev_unlock_state() after use.
//
// v4l2_subdev_init - initializes the sub-device struct
//
// @sd: pointer to the &struct v4l2_subdev to be initialized
// @ops: pointer to &struct v4l2_subdev_ops.
//
// v4l2_subdev_call - call an operation of a v4l2_subdev.
//
// @sd: pointer to the &struct v4l2_subdev
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of callbacks functions.
// @f: callback function to be called.
// The callback functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Example: err = v4l2_subdev_call(sd, video, s_std, norm);
//

//
// v4l2_subdev_call_state_active - call an operation of a v4l2_subdev which
// takes state as a parameter, passing the
// subdev its active state.
//
// @sd: pointer to the &struct v4l2_subdev
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of callbacks functions.
// @f: callback function to be called.
// The callback functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// This is similar to v4l2_subdev_call(), except that this version can only be
// used for ops that take a subdev state as a parameter. The macro will get the
// active state, lock it before calling the op and unlock it after the call.
//

//
// v4l2_subdev_call_state_try - call an operation of a v4l2_subdev which
// takes state as a parameter, passing the
// subdev a newly allocated try state.
//
// @sd: pointer to the &struct v4l2_subdev
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of callbacks functions.
// @f: callback function to be called.
// The callback functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// This is similar to v4l2_subdev_call_state_active(), except that as this
// version allocates a new state, this is only usable for
// V4L2_SUBDEV_FORMAT_TRY use cases.
//
// Note: only legacy non-MC drivers may need this macro.
//

//
// v4l2_subdev_has_op - Checks if a subdev defines a certain operation.
//
// @sd: pointer to the &struct v4l2_subdev
// @o: The group of callback functions in &struct v4l2_subdev_ops
// which @f is a part of.
// @f: callback function to be checked for its existence.
//

//
// v4l2_subdev_notify_event() - Delivers event notification for subdevice
// @sd: The subdev for which to deliver the event
// @ev: The event to deliver
//
// Will deliver the specified event to all userspace event listeners which are
// subscribed to the v42l subdev event queue as well as to the bridge driver
// using the notify callback. The notification type for the notify callback
// will be %V4L2_DEVICE_NOTIFY_EVENT.
//
// v4l2_subdev_is_streaming() - Returns if the subdevice is streaming
// @sd: The subdevice
//
// v4l2_subdev_is_streaming() tells if the subdevice is currently streaming.
// "Streaming" here means whether .s_stream() or .enable_streams() has been
// successfully called, and the streaming has not yet been disabled.
//
// If the subdevice implements .enable_streams() this function must be called
// while holding the active state lock.
//
extern "C" {
    pub fn v4l2_subdev_is_streaming(sd: *mut v4l2_subdev) -> bool;
}
