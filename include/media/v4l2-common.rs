//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-common.h
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

// Common printk constructs for v4l-i2c drivers. These macros create a unique

// These three macros assume that the debug level is set with a module

// Add a version of v4l_dbg to be used on drivers using dev_foo() macros

// -------------------------------------------------------------------------
// These printk constructs can be used with v4l2_device and v4l2_subdev

// These three macros assume that the debug level is set with a module

//
// v4l2_ctrl_query_fill- Fill in a struct v4l2_queryctrl
//
// @qctrl: pointer to the &struct v4l2_queryctrl to be filled
// @min: minimum value for the control
// @max: maximum value for the control
// @step: control step
// @def: default value for the control
//
// Fills the &struct v4l2_queryctrl fields for the query control.
//
// .. note::
//
// This function assumes that the @qctrl->id field is filled.
//
// Returns -EINVAL if the control is not known by the V4L2 core, 0 on success.
//
// -------------------------------------------------------------------------
// I2C Helper functions

//
// enum v4l2_i2c_tuner_type - specifies the range of tuner address that
// should be used when seeking for I2C devices.
//
// @ADDRS_RADIO:		Radio tuner addresses.
// Represent the following I2C addresses:
// 0x10 (if compiled with tea5761 support)
// and 0x60.
// @ADDRS_DEMOD:		Demod tuner addresses.
// Represent the following I2C addresses:
// 0x42, 0x43, 0x4a and 0x4b.
// @ADDRS_TV:			TV tuner addresses.
// Represent the following I2C addresses:
// 0x42, 0x43, 0x4a, 0x4b, 0x60, 0x61, 0x62,
// 0x63 and 0x64.
// @ADDRS_TV_WITH_DEMOD:	TV tuner addresses if demod is present, this
// excludes addresses used by the demodulator
// from the list of candidates.
// Represent the following I2C addresses:
// 0x60, 0x61, 0x62, 0x63 and 0x64.
//
// NOTE: All I2C addresses above use the 7-bit notation.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_i2c_tuner_type {
    ADDRS_RADIO,
    ADDRS_DEMOD,
    ADDRS_TV,
    ADDRS_TV_WITH_DEMOD,
}

//
// v4l2_i2c_new_subdev - Load an i2c module and return an initialized
// &struct v4l2_subdev.
//
// @v4l2_dev: pointer to &struct v4l2_device
// @adapter: pointer to struct i2c_adapter
// @client_type:  name of the chip that's on the adapter.
// @addr: I2C address. If zero, it will use @probe_addrs
// @probe_addrs: array with a list of address. The last entry at such
// array should be %I2C_CLIENT_END.
//
// returns a &struct v4l2_subdev pointer.
//
// v4l2_i2c_new_subdev_board - Load an i2c module and return an initialized
// &struct v4l2_subdev.
//
// @v4l2_dev: pointer to &struct v4l2_device
// @adapter: pointer to struct i2c_adapter
// @info: pointer to struct i2c_board_info used to replace the irq,
// platform_data and addr arguments.
// @probe_addrs: array with a list of address. The last entry at such
// array should be %I2C_CLIENT_END.
//
// returns a &struct v4l2_subdev pointer.
//
// v4l2_i2c_subdev_set_name - Set name for an I²C sub-device
//
// @sd: pointer to &struct v4l2_subdev
// @client: pointer to struct i2c_client
// @devname: the name of the device; if NULL, the I²C device drivers's name
// will be used
// @postfix: sub-device specific string to put right after the I²C device name;
// may be NULL
//
// v4l2_i2c_subdev_init - Initializes a &struct v4l2_subdev with data from
// an i2c_client struct.
//
// @sd: pointer to &struct v4l2_subdev
// @client: pointer to struct i2c_client
// @ops: pointer to &struct v4l2_subdev_ops
//
// v4l2_i2c_subdev_addr - returns i2c client address of &struct v4l2_subdev.
//
// @sd: pointer to &struct v4l2_subdev
//
// Returns the address of an I2C sub-device
//
extern "C" {
    pub fn v4l2_i2c_subdev_addr(sd: *mut v4l2_subdev) -> c_ushort;
}
//
// v4l2_i2c_tuner_addrs - Return a list of I2C tuner addresses to probe.
//
// @type: type of the tuner to seek, as defined by
// &enum v4l2_i2c_tuner_type.
//
// NOTE: Use only if the tuner addresses are unknown.
//
// v4l2_i2c_subdev_unregister - Unregister a v4l2_subdev
//
// @sd: pointer to &struct v4l2_subdev
//
extern "C" {
    pub fn v4l2_i2c_subdev_unregister(sd: *mut v4l2_subdev);
}

// -------------------------------------------------------------------------
// SPI Helper functions

//
// v4l2_spi_new_subdev - Load an spi module and return an initialized
// &struct v4l2_subdev.
//
// @v4l2_dev: pointer to &struct v4l2_device.
// @ctlr: pointer to struct spi_controller.
// @info: pointer to struct spi_board_info.
//
// returns a &struct v4l2_subdev pointer.
//
// v4l2_spi_subdev_init - Initialize a v4l2_subdev with data from an
// spi_device struct.
//
// @sd: pointer to &struct v4l2_subdev
// @spi: pointer to struct spi_device.
// @ops: pointer to &struct v4l2_subdev_ops
//
// v4l2_spi_subdev_unregister - Unregister a v4l2_subdev
//
// @sd: pointer to &struct v4l2_subdev
//
extern "C" {
    pub fn v4l2_spi_subdev_unregister(sd: *mut v4l2_subdev);
}

// -------------------------------------------------------------------------
//
// FIXME: these remaining ioctls/structs should be removed as well, but they
// are still used in tuner-simple.c (TUNER_SET_CONFIG) and cx18/ivtv (RESET).
// To remove these ioctls some more cleanup is needed in those modules.
//
// It doesn't make much sense on documenting them, as what we really want is
// to get rid of them.
//
// s_config
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_priv_tun_config {
    pub tuner: c_int,
    pub priv: *mut c_void,
}

// -------------------------------------------------------------------------
// Miscellaneous helper functions
//
// v4l_bound_align_image - adjust video dimensions according to
// a given constraints.
//
// @width:	pointer to width that will be adjusted if needed.
// @wmin:	minimum width.
// @wmax:	maximum width.
// @walign:	least significant bit on width.
// @height:	pointer to height that will be adjusted if needed.
// @hmin:	minimum height.
// @hmax:	maximum height.
// @halign:	least significant bit on height.
// @salign:	least significant bit for the image size (e. g.
// :math:`width * height`).
//
// Clip an image to have @width between @wmin and @wmax, and @height between
// @hmin and @hmax, inclusive.
//
// Additionally, the @width will be a multiple of :math:`2^{walign}`,
// the @height will be a multiple of :math:`2^{halign}`, and the overall
// size :math:`width * height` will be a multiple of :math:`2^{salign}`.
//
// .. note::
//
// #. The clipping rectangle may be shrunk or enlarged to fit the alignment
// constraints.
// #. @wmax must not be smaller than @wmin.
// #. @hmax must not be smaller than @hmin.
// #. The alignments must not be so high there are no possible image
// sizes within the allowed bounds.
// #. @wmin and @hmin must be at least 1 (don't use 0).
// #. For @walign, @halign and @salign, if you don't care about a certain
// alignment, specify ``0``, as :math:`2^0 = 1` and one byte alignment
// is equivalent to no alignment.
// #. If you only want to adjust downward, specify a maximum that's the
// same as the initial value.
//
// v4l2_find_nearest_size_conditional - Find the nearest size among a discrete
// set of resolutions contained in an array of a driver specific struct,
// with conditionally exlusion of certain modes
//
// @array: a driver specific array of image sizes
// @array_size: the length of the driver specific array of image sizes
// @width_field: the name of the width field in the driver specific struct
// @height_field: the name of the height field in the driver specific struct
// @width: desired width
// @height: desired height
// @func: ignores mode if returns false
// @context: context for the function
//
// Finds the closest resolution to minimize the width and height differences
// between what requested and the supported resolutions. The size of the width
// and height fields in the driver specific must equal to that of u32, i.e. four
// bytes. @func is called for each mode considered, a mode is ignored if @func
// returns false for it.
//
// Returns the best match or NULL if the length of the array is zero.
//

//
// v4l2_find_nearest_size - Find the nearest size among a discrete set of
// resolutions contained in an array of a driver specific struct
//
// @array: a driver specific array of image sizes
// @array_size: the length of the driver specific array of image sizes
// @width_field: the name of the width field in the driver specific struct
// @height_field: the name of the height field in the driver specific struct
// @width: desired width
// @height: desired height
//
// Finds the closest resolution to minimize the width and height differences
// between what requested and the supported resolutions. The size of the width
// and height fields in the driver specific must equal to that of u32, i.e. four
// bytes.
//
// Returns the best match or NULL if the length of the array is zero.
//

//
// v4l2_g_parm_cap - helper routine for vidioc_g_parm to fill this in by
// calling the get_frame_interval op of the given subdev. It only works
// for V4L2_BUF_TYPE_VIDEO_CAPTURE(_MPLANE), hence the _cap in the
// function name.
//
// @vdev: the struct video_device pointer. Used to determine the device caps.
// @sd: the sub-device pointer.
// @a: the VIDIOC_G_PARM argument.
//
// v4l2_s_parm_cap - helper routine for vidioc_s_parm to fill this in by
// calling the set_frame_interval op of the given subdev. It only works
// for V4L2_BUF_TYPE_VIDEO_CAPTURE(_MPLANE), hence the _cap in the
// function name.
//
// @vdev: the struct video_device pointer. Used to determine the device caps.
// @sd: the sub-device pointer.
// @a: the VIDIOC_S_PARM argument.
//
// Compare two v4l2_fract structs

// -------------------------------------------------------------------------
// Pixel format and FourCC helpers
//
// enum v4l2_pixel_encoding - specifies the pixel encoding value
//
// @V4L2_PIXEL_ENC_UNKNOWN:	Pixel encoding is unknown/un-initialized
// @V4L2_PIXEL_ENC_YUV:		Pixel encoding is YUV
// @V4L2_PIXEL_ENC_RGB:		Pixel encoding is RGB
// @V4L2_PIXEL_ENC_BAYER:	Pixel encoding is Bayer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_pixel_encoding {
    V4L2_PIXEL_ENC_UNKNOWN = 0,
    V4L2_PIXEL_ENC_YUV = 1,
    V4L2_PIXEL_ENC_RGB = 2,
    V4L2_PIXEL_ENC_BAYER = 3,
}

//
// struct v4l2_format_info - information about a V4L2 format
// @format: 4CC format identifier (V4L2_PIX_FMT_*)
// @pixel_enc: Pixel encoding (see enum v4l2_pixel_encoding above)
// @mem_planes: Number of memory planes, which includes the alpha plane (1 to 4).
// @comp_planes: Number of component planes, which includes the alpha plane (1 to 4).
// @bpp: Array of per-plane bytes per pixel
// @bpp_div: Array of per-plane bytes per pixel divisors to support fractional pixel sizes.
// @hdiv: Horizontal chroma subsampling factor
// @vdiv: Vertical chroma subsampling factor
// @block_w: Per-plane macroblock pixel width (optional)
// @block_h: Per-plane macroblock pixel height (optional)
// @has_alpha: Does the format embeds an alpha component?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_format_info {
    pub format: u32,
    pub pixel_enc: u8,
    pub mem_planes: u8,
    pub comp_planes: u8,
    pub bpp: [u8; 4],
    pub bpp_div: [u8; 4],
    pub hdiv: u8,
    pub vdiv: u8,
    pub block_w: [u8; 4],
    pub block_h: [u8; 4],
    pub has_alpha: bool,
}

// @stride_alignment is a power of 2 value in bytes
//
// v4l2_get_link_freq - Get link rate from transmitter
//
// @pad: The transmitter's media pad
// @mul: The multiplier between pixel rate and link frequency. Bits per pixel on
// D-PHY, samples per clock on parallel. 0 otherwise.
// @div: The divisor between pixel rate and link frequency. Number of data lanes
// times two on D-PHY, 1 on parallel. 0 otherwise.
//
// This function obtains and returns the link frequency from the transmitter
// sub-device's pad. The link frequency is retrieved using the get_mbus_config
// sub-device pad operation. If this fails, the function falls back to obtaining
// the frequency either directly from the V4L2_CID_LINK_FREQ control if
// implemented by the transmitter, or by calculating it from the pixel rate
// obtained from the V4L2_CID_PIXEL_RATE control.
//
// Return:
// * >0: Link frequency
// * %-ENOENT: Link frequency or pixel rate control not found
// * %-EINVAL: Invalid link frequency value
//

//
// v4l2_get_active_data_lanes - Get number of active data lanes from driver
//
// @pad: The transmitter's media pad.
// @max_data_lanes: The maximum number of active data lanes supported by
// the MIPI CSI link in hardware.
//
// This function is intended for obtaining the number of data lanes that are
// actively being used by the driver for a MIPI CSI-2 device on a given media pad.
// This information is derived from a mbus_config fetched from a device driver
// using the get_mbus_config v4l2_subdev pad op.
//
// Return:
// * >0: Number of active data lanes
// * %-EINVAL: Number of active data lanes is invalid, as it exceeds the maximum
// supported data lanes.
//

extern "C" {
    pub fn v4l2_fraction_to_interval(numerator: u32, denominator: u32) -> u32;
}
//
// v4l2_link_freq_to_bitmap - Figure out platform-supported link frequencies
// @dev: The struct device
// @fw_link_freqs: Array of link frequencies from firmware
// @num_of_fw_link_freqs: Number of entries in @fw_link_freqs
// @driver_link_freqs: Array of link frequencies supported by the driver
// @num_of_driver_link_freqs: Number of entries in @driver_link_freqs
// @bitmap: Bitmap of driver-supported link frequencies found in @fw_link_freqs
//
// This function checks which driver-supported link frequencies are enabled in
// system firmware and sets the corresponding bits in @bitmap (after first
// zeroing it).
//
// Return:
// * %0: Success
// * %-ENOENT: No match found between driver-supported link frequencies and
// those available in firmware.
// * %-ENODATA: No link frequencies were specified in firmware.
//
// devm_v4l2_sensor_clk_get - lookup and obtain a reference to a clock producer
// for a camera sensor
//
// @dev: device for v4l2 sensor clock "consumer"
// @id: clock consumer ID
//
// This function behaves the same way as devm_clk_get() except where there
// is no clock producer like in ACPI-based platforms.
//
// For ACPI-based platforms, the function will read the "clock-frequency"
// ACPI _DSD property and register a fixed-clock with the frequency indicated
// in the property.
//
// This function also handles the special ACPI-based system case where:
//
// * The clock-frequency _DSD property is present.
// * A reference to the clock producer is present, where the clock is provided
// by a camera sensor PMIC driver (e.g. int3472/tps68470.c)
//
// In this case try to set the clock-frequency value to the provided clock.
//
// As the name indicates, this function may only be used on camera sensor
// devices. This is because generally only camera sensors do need a clock to
// query the frequency from, due to the requirement to configure the PLL for a
// given CSI-2 interface frequency where the sensor's external clock frequency
// is a factor. Additionally, the clock frequency tends to be available on ACPI
// firmware based systems for camera sensors specifically (if e.g. DisCo for
// Imaging compliant).
//
// Returns a pointer to a struct clk on success or an error pointer on failure.
//
extern "C" {
    pub fn __devm_v4l2_sensor_clk_get(_arg: dev, _arg: id, _arg: false, _arg: false, _arg: 0) -> return;
}
//
// devm_v4l2_sensor_clk_get_legacy - lookup and obtain a reference to a clock
// producer for a camera sensor.
//
// @dev: device for v4l2 sensor clock "consumer"
// @id: clock consumer ID
// @fixed_rate: interpret the @clk_rate as a fixed rate or default rate
// @clk_rate: the clock rate
//
// This function behaves the same way as devm_v4l2_sensor_clk_get() except that
// it extends the behaviour on ACPI platforms to all platforms.
//
// The function also provides the ability to set the clock rate to a fixed
// frequency by setting @fixed_rate to true and specifying the fixed frequency
// in @clk_rate, or to use a default clock rate when the "clock-frequency"
// property is absent by setting @fixed_rate to false and specifying the default
// frequency in @clk_rate. Setting @fixed_rate to true and @clk_rate to 0 is an
// error.
//
// This function is meant to support legacy behaviour in existing drivers only.
// It must not be used in any new driver.
//
// Returns a pointer to a struct clk on success or an error pointer on failure.
//
extern "C" {
    pub fn __devm_v4l2_sensor_clk_get(_arg: dev, _arg: id, _arg: true, _arg: fixed_rate, _arg: clk_rate) -> return;
}
//
// When the timestamp comes from 32-bit user space, there may be
// uninitialized data in tv_usec, so cast it to u32.
// Otherwise allow invalid input for backwards compatibility.
//
