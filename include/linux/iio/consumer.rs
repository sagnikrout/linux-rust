//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/consumer.h
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
// Industrial I/O in kernel consumer interface
//
// Copyright (c) 2011 Jonathan Cameron
//

//
// struct iio_channel - everything needed for a consumer to use a channel
// @indio_dev:		Device on which the channel exists.
// @channel:		Full description of the channel.
// @data:		Data about the channel used by consumer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_channel {
    pub indio_dev: *mut iio_dev,
    pub channel: *const iio_chan_spec,
    pub data: *mut c_void,
}

//
// iio_channel_get() - get description of all that is needed to access channel.
// @dev:		Pointer to consumer device. Device name must match
// the name of the device as provided in the iio_map
// with which the desired provider to consumer mapping
// was registered.
// @consumer_channel:	Unique name to identify the channel on the consumer
// side. This typically describes the channels use within
// the consumer. E.g. 'battery_voltage'
//
// iio_channel_release() - release channels obtained via iio_channel_get
// @chan:		The channel to be released.
//
extern "C" {
    pub fn iio_channel_release(chan: *mut iio_channel);
}
//
// devm_iio_channel_get() - Resource managed version of iio_channel_get().
// @dev:		Pointer to consumer device. Device name must match
// the name of the device as provided in the iio_map
// with which the desired provider to consumer mapping
// was registered.
// @consumer_channel:	Unique name to identify the channel on the consumer
// side. This typically describes the channels use within
// the consumer. E.g. 'battery_voltage'
//
// Returns a pointer to negative errno if it is not able to get the iio channel
// otherwise returns valid pointer for iio channel.
//
// The allocated iio channel is automatically released when the device is
// unbound.
//
// iio_channel_get_all() - get all channels associated with a client
// @dev:		Pointer to consumer device.
//
// Returns an array of iio_channel structures terminated with one with
// null iio_dev pointer.
// This function is used by fairly generic consumers to get all the
// channels registered as having this consumer.
//
// iio_channel_release_all() - reverse iio_channel_get_all
// @chan:		Array of channels to be released.
//
extern "C" {
    pub fn iio_channel_release_all(chan: *mut iio_channel);
}
//
// devm_iio_channel_get_all() - Resource managed version of
// iio_channel_get_all().
// @dev: Pointer to consumer device.
//
// Returns a pointer to negative errno if it is not able to get the iio channel
// otherwise returns an array of iio_channel structures terminated with one with
// null iio_dev pointer.
//
// This function is used by fairly generic consumers to get all the
// channels registered as having this consumer.
//
// The allocated iio channels are automatically released when the device is
// unbounded.
//
// fwnode_iio_channel_get_by_name() - get description of all that is needed to access channel.
// @fwnode:		Pointer to consumer Firmware node
// @consumer_channel:	Unique name to identify the channel on the consumer
// side. This typically describes the channels use within
// the consumer. E.g. 'battery_voltage'
//
// devm_fwnode_iio_channel_get_by_name() - Resource managed version of
// fwnode_iio_channel_get_by_name().
// @dev:		Pointer to consumer device.
// @fwnode:		Pointer to consumer Firmware node
// @consumer_channel:	Unique name to identify the channel on the consumer
// side. This typically describes the channels use within
// the consumer. E.g. 'battery_voltage'
//
// Returns a pointer to negative errno if it is not able to get the iio channel
// otherwise returns valid pointer for iio channel.
//
// The allocated iio channel is automatically released when the device is
// unbound.
//
// iio_channel_get_all_cb() - register callback for triggered capture
// @dev:		Pointer to client device.
// @cb:			Callback function. Must be safe to call from any context
// (e.g. must not sleep).
// @private:		Private data passed to callback.
//
// NB right now we have no ability to mux data from multiple devices.
// So if the channels requested come from different devices this will
// fail.
//
// iio_channel_cb_set_buffer_watermark() - set the buffer watermark.
// @cb_buffer:		The callback buffer from whom we want the channel
// information.
// @watermark: buffer watermark in bytes.
//
// This function allows to configure the buffer watermark.
//
// iio_channel_release_all_cb() - release and unregister the callback.
// @cb_buffer:		The callback buffer that was allocated.
//
extern "C" {
    pub fn iio_channel_release_all_cb(cb_buffer: *mut iio_cb_buffer);
}
//
// iio_channel_start_all_cb() - start the flow of data through callback.
// @cb_buff:		The callback buffer we are starting.
//
extern "C" {
    pub fn iio_channel_start_all_cb(cb_buff: *mut iio_cb_buffer) -> c_int;
}
//
// iio_channel_stop_all_cb() - stop the flow of data through the callback.
// @cb_buff:		The callback buffer we are stopping.
//
extern "C" {
    pub fn iio_channel_stop_all_cb(cb_buff: *mut iio_cb_buffer);
}
//
// iio_channel_cb_get_channels() - get access to the underlying channels.
// @cb_buffer:		The callback buffer from whom we want the channel
// information.
//
// This function allows one to obtain information about the channels.
// Whilst this may allow direct reading if all buffers are disabled, the
// primary aim is to allow drivers that are consuming a channel to query
// things like scaling of the channel.
//
// iio_channel_cb_get_channels(const struct iio_cb_buffer *cb_buffer);
//
// iio_channel_cb_get_iio_dev() - get access to the underlying device.
// @cb_buffer:		The callback buffer from whom we want the device
// information.
//
// This function allows one to obtain information about the device.
// The primary aim is to allow drivers that are consuming a device to query
// things like current trigger.
//
// iio_channel_cb_get_iio_dev(const struct iio_cb_buffer *cb_buffer);
//
// iio_read_channel_raw() - read from a given channel
// @chan:		The channel being queried.
// @val:		Value read back.
//
// Note, if standard units are required, raw reads from iio channels
// need the offset (default 0) and scale (default 1) to be applied
// as (raw + offset) * scale.
//
// iio_read_channel_average_raw() - read from a given channel
// @chan:		The channel being queried.
// @val:		Value read back.
//
// Note, if standard units are required, raw reads from iio channels
// need the offset (default 0) and scale (default 1) to be applied
// as (raw + offset) * scale.
//
// In opposit to the normal iio_read_channel_raw this function
// returns the average of multiple reads.
//
extern "C" {
    pub fn iio_read_channel_average_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
}
//
// iio_read_channel_processed() - read processed value from a given channel
// @chan:		The channel being queried.
// @val:		Value read back.
//
// Returns an error code or 0.
//
// This function will read a processed value from a channel. A processed value
// means that this value will have the correct unit and not some device internal
// representation. If the device does not support reporting a processed value
// the function will query the raw value and the channels scale and offset and
// do the appropriate transformation.
//
extern "C" {
    pub fn iio_read_channel_processed(chan: *mut iio_channel, val: *mut c_int) -> c_int;
}
//
// iio_read_channel_processed_scale() - read and scale a processed value
// @chan:		The channel being queried.
// @val:		Value read back.
// @scale:		Scale factor to apply during the conversion
//
// Returns an error code or 0.
//
// This function will read a processed value from a channel. This will work
// like @iio_read_channel_processed() but also scale with an additional
// scale factor while attempting to minimize any precision loss.
//
// iio_write_channel_attribute() - Write values to the device attribute.
// @chan:	The channel being queried.
// @val:	Value being written.
// @val2:	Value being written.val2 use depends on attribute type.
// @attribute:	info attribute to be read.
//
// Returns an error code or 0.
//
// iio_read_channel_attribute() - Read values from the device attribute.
// @chan:	The channel being queried.
// @val:	Value being written.
// @val2:	Value being written.Val2 use depends on attribute type.
// @attribute:	info attribute to be written.
//
// Returns an error code if failed. Else returns a description of what is in val
// and val2, such as IIO_VAL_INT_PLUS_MICRO telling us we have a value of val
// + val2/1e6
//
// iio_write_channel_raw() - write to a given channel
// @chan:		The channel being queried.
// @val:		Value being written.
//
// Note that for raw writes to iio channels, if the value provided is
// in standard units, the affect of the scale and offset must be removed
// as (value / scale) - offset.
//
extern "C" {
    pub fn iio_write_channel_raw(chan: *mut iio_channel, val: c_int) -> c_int;
}
//
// iio_read_max_channel_raw() - read maximum available raw value from a given
// channel, i.e. the maximum possible value.
// @chan:		The channel being queried.
// @val:		Value read back.
//
// Note, if standard units are required, raw reads from iio channels
// need the offset (default 0) and scale (default 1) to be applied
// as (raw + offset) * scale.
//
extern "C" {
    pub fn iio_read_max_channel_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
}
//
// iio_read_min_channel_raw() - read minimum available raw value from a given
// channel, i.e. the minimum possible value.
// @chan:		The channel being queried.
// @val:		Value read back.
//
// Note, if standard units are required, raw reads from iio channels
// need the offset (default 0) and scale (default 1) to be applied
// as (raw + offset) * scale.
//
extern "C" {
    pub fn iio_read_min_channel_raw(chan: *mut iio_channel, val: *mut c_int) -> c_int;
}
//
// iio_read_avail_channel_raw() - read available raw values from a given channel
// @chan:		The channel being queried.
// @vals:		Available values read back.
// @length:		Number of entries in vals.
//
// Returns an error code, IIO_AVAIL_RANGE or IIO_AVAIL_LIST.
//
// For ranges, three vals are always returned; min, step and max.
// For lists, all the possible values are enumerated.
//
// Note, if standard units are required, raw available values from iio
// channels need the offset (default 0) and scale (default 1) to be applied
// as (raw + offset) * scale.
//
// iio_read_avail_channel_attribute() - read available channel attribute values
// @chan:		The channel being queried.
// @vals:		Available values read back.
// @type:		Type of values read back.
// @length:		Number of entries in vals.
// @attribute:		info attribute to be read back.
//
// Returns an error code, IIO_AVAIL_RANGE or IIO_AVAIL_LIST.
//
// iio_get_channel_type() - get the type of a channel
// @channel:		The channel being queried.
// @type:		The type of the channel.
//
// returns the enum iio_chan_type of the channel
//
// iio_read_channel_offset() - read the offset value for a channel
// @chan:		The channel being queried.
// @val:		First part of value read back.
// @val2:		Second part of value read back.
//
// Note returns a description of what is in val and val2, such
// as IIO_VAL_INT_PLUS_MICRO telling us we have a value of val
// + val2/1e6
//
// iio_read_channel_scale() - read the scale value for a channel
// @chan:		The channel being queried.
// @val:		First part of value read back.
// @val2:		Second part of value read back.
//
// Note returns a description of what is in val and val2, such
// as IIO_VAL_INT_PLUS_MICRO telling us we have a value of val
// + val2/1e6
//
// iio_multiply_value() - Multiply an IIO value
// @result:	Destination pointer for the multiplication result
// @multiplier:	Multiplier.
// @type:	One of the IIO_VAL_* constants. This decides how the @val and
// @val2 parameters are interpreted.
// @val:	Value being multiplied.
// @val2:	Value being multiplied. @val2 use depends on type.
//
// Multiply an IIO value with a s64 multiplier storing the result as
// IIO_VAL_INT. This is typically used for scaling.
//
// Returns:
// IIO_VAL_INT on success or a negative error-number on failure.
//
// iio_convert_raw_to_processed() - Converts a raw value to a processed value
// @chan:		The channel being queried
// @raw:		The raw IIO to convert
// @processed:		The result of the conversion
// @scale:		Scale factor to apply during the conversion
//
// Returns an error code or 0.
//
// This function converts a raw value to processed value for a specific channel.
// A raw value is the device internal representation of a sample and the value
// returned by iio_read_channel_raw, so the unit of that value is device
// depended. A processed value on the other hand is value has a normed unit
// according with the IIO specification.
//
// The scale factor allows to increase the precession of the returned value. For
// a scale factor of 1 the function will return the result in the normal IIO
// unit for the channel type. E.g. millivolt for voltage channels, if you want
// nanovolts instead pass 1000000 as the scale factor.
//
// iio_get_channel_ext_info_count() - get number of ext_info attributes
// connected to the channel.
// @chan:		The channel being queried
//
// Returns the number of ext_info attributes
//
extern "C" {
    pub fn iio_get_channel_ext_info_count(chan: *mut iio_channel) -> c_uint;
}
//
// iio_read_channel_ext_info() - read ext_info attribute from a given channel
// @chan:		The channel being queried.
// @attr:		The ext_info attribute to read.
// @buf:		Where to store the attribute value. Assumed to hold
// at least PAGE_SIZE bytes and to be aligned at PAGE_SIZE.
//
// Returns the number of bytes written to buf (perhaps w/o zero termination;
// it need not even be a string), or an error code.
//
// iio_write_channel_ext_info() - write ext_info attribute from a given channel
// @chan:		The channel being queried.
// @attr:		The ext_info attribute to read.
// @buf:		The new attribute value. Strings needs to be zero-
// terminated, but the terminator should not be included
// in the below len.
// @len:		The size of the new attribute value.
//
// Returns the number of accepted bytes, which should be the same as len.
// An error code can also be returned.
//
// iio_read_channel_label() - read label for a given channel
// @chan:		The channel being queried.
// @buf:		Where to store the attribute value. Assumed to hold
// at least PAGE_SIZE bytes and to be aligned at PAGE_SIZE.
//
// Returns the number of bytes written to buf, or an error code.
//
extern "C" {
    pub fn iio_read_channel_label(chan: *mut iio_channel, buf: *mut c_char) -> isize;
}
