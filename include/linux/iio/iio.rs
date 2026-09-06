//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/iio.h
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
// The industrial I/O core
//
// Copyright (c) 2008 Jonathan Cameron
//

// IIO TODO LIST
//
// Provide means of adjusting timer accuracy.
// Currently assumes nano seconds.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_shared_by {
    IIO_SEPARATE,
    IIO_SHARED_BY_TYPE,
    IIO_SHARED_BY_DIR,
    IIO_SHARED_BY_ALL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_endian {
    IIO_CPU,
    IIO_BE,
    IIO_LE,
}

//
// struct iio_chan_spec_ext_info - Extended channel info attribute
// @name:	Info attribute name
// @shared:	Whether this attribute is shared between all channels.
// @read:	Read callback for this info attribute, may be NULL.
// @write:	Write callback for this info attribute, may be NULL.
// @private:	Data private to the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_chan_spec_ext_info {
    pub name: *const c_char,
    pub shared: iio_shared_by,
    pub buf): *const *const iio_chan_spec , char,
    pub len): usize,
    pub private: uintptr_t,
}

//
// struct iio_enum - Enum channel info attribute
// @items:	An array of strings.
// @num_items:	Length of the item array.
// @set:	Set callback function, may be NULL.
// @get:	Get callback function, may be NULL.
//
// The iio_enum struct can be used to implement enum style channel attributes.
// Enum style attributes are those which have a set of strings which map to
// unsigned integer values. The IIO enum helper code takes care of mapping
// between value and string as well as generating a "_available" file which
// contains a list of all available items. The set callback will be called when
// the attribute is updated. The last parameter is the index to the newly
// activated item. The get callback will be used to query the currently active
// item and is supposed to return the index for it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_enum {
    pub items: *const *const c_char,
    pub num_items: c_uint,
    pub int): *const *const *const *const int (set)(struct iio_dev , struct iio_chan_spec , unsigned,
    pub ): *const *const *const int (get)(struct iio_dev , struct iio_chan_spec,
}

//
// IIO_ENUM() - Initialize enum extended channel attribute
// @_name:	Attribute name
// @_shared:	Whether the attribute is shared between all channels
// @_e:		Pointer to an iio_enum struct
//
// This should usually be used together with IIO_ENUM_AVAILABLE()
//

//
// IIO_ENUM_AVAILABLE() - Initialize enum available extended channel attribute
// @_name:	Attribute name ("_available" will be appended to the name)
// @_shared:	Whether the attribute is shared between all channels
// @_e:		Pointer to an iio_enum struct
//
// Creates a read only attribute which lists all the available enum items in a
// space separated list. This should usually be used together with IIO_ENUM()
//

//
// struct iio_mount_matrix - iio mounting matrix
// @rotation: 3 dimensional space rotation matrix defining sensor alignment with
// main hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_mount_matrix {
    pub rotation: [*const c_char; 9],
}

extern "C" {
    pub fn iio_read_mount_matrix(dev: *mut device, matrix: *mut iio_mount_matrix) -> c_int;
}
//
// IIO_MOUNT_MATRIX() - Initialize mount matrix extended channel attribute
// @_shared:	Whether the attribute is shared between all channels
// @_get:	Pointer to an iio_get_mount_matrix_t accessor
//

//
// struct iio_event_spec - specification for a channel event
// @type:		    Type of the event
// @dir:		    Direction of the event
// @mask_separate:	    Bit mask of enum iio_event_info values. Attributes
// set in this mask will be registered per channel.
// @mask_shared_by_type:    Bit mask of enum iio_event_info values. Attributes
// set in this mask will be shared by channel type.
// @mask_shared_by_dir:	    Bit mask of enum iio_event_info values. Attributes
// set in this mask will be shared by channel type and
// direction.
// @mask_shared_by_all:	    Bit mask of enum iio_event_info values. Attributes
// set in this mask will be shared by all channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_event_spec {
    pub type: iio_event_type,
    pub dir: iio_event_direction,
    pub mask_separate: c_ulong,
    pub mask_shared_by_type: c_ulong,
    pub mask_shared_by_dir: c_ulong,
    pub mask_shared_by_all: c_ulong,
}

//
// define IIO_SCAN_FORMAT_SIGNED_INT - signed integer data format
//
// &iio_scan_type.format value for signed integers (two's complement).
//

//
// define IIO_SCAN_FORMAT_UNSIGNED_INT - unsigned integer data format
//
// &iio_scan_type.format value for unsigned integers.
//

//
// define IIO_SCAN_FORMAT_FLOAT - floating-point data format
//
// &iio_scan_type.format value for IEEE 754 floating-point numbers.
//

//
// struct iio_scan_type - specification for channel data format in buffer
// @sign:		Deprecated, use @format instead.
// @format:		Data format, can have any of the IIO_SCAN_FORMAT_
// values.
// @realbits:		Number of valid bits of data
// @storagebits:	Realbits + padding
// @shift:		Shift right by this before masking out realbits.
// @repeat:		Number of times real/storage bits repeats. When the
// repeat element is more than 1, then the type element in
// sysfs will show a repeat value. Otherwise, the number
// of repetitions is omitted.
// @endianness:		little or big endian
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_scan_type {
    pub sign: c_char,
    pub format: c_char,
}

//
// struct iio_chan_spec - specification of a single channel
// @type:		What type of measurement is the channel making.
// @channel:		What number do we wish to assign the channel.
// @channel2:		If there is a second number for a differential
// channel then this is it. If modified is set then the
// value here specifies the modifier.
// @address:		Driver specific identifier.
// @scan_index:		Monotonic index to give ordering in scans when read
// from a buffer.
// @scan_type:		struct describing the scan type - mutually exclusive
// with ext_scan_type.
// @ext_scan_type:	Used in rare cases where there is more than one scan
// format for a channel. When this is used, the flag
// has_ext_scan_type must be set and the driver must
// implement get_current_scan_type in struct iio_info.
// @num_ext_scan_type:	Number of elements in ext_scan_type.
// @info_mask_separate: What information is to be exported that is specific to
// this channel.
// @info_mask_separate_available: What availability information is to be
// exported that is specific to this channel.
// @info_mask_shared_by_type: What information is to be exported that is shared
// by all channels of the same type.
// @info_mask_shared_by_type_available: What availability information is to be
// exported that is shared by all channels of the same
// type.
// @info_mask_shared_by_dir: What information is to be exported that is shared
// by all channels of the same direction.
// @info_mask_shared_by_dir_available: What availability information is to be
// exported that is shared by all channels of the same
// direction.
// @info_mask_shared_by_all: What information is to be exported that is shared
// by all channels.
// @info_mask_shared_by_all_available: What availability information is to be
// exported that is shared by all channels.
// @event_spec:		Array of events which should be registered for this
// channel.
// @num_event_specs:	Size of the event_spec array.
// @ext_info:		Array of extended info attributes for this channel.
// The array is NULL terminated, the last element should
// have its name field set to NULL.
// @extend_name:	Allows labeling of channel attributes with an
// informative name. Note this has no effect codes etc,
// unlike modifiers.
// This field is deprecated in favour of providing
// iio_info->read_label() to override the label, which
// unlike @extend_name does not affect sysfs filenames.
// @datasheet_name:	A name used in in-kernel mapping of channels. It should
// correspond to the first name that the channel is referred
// to by in the datasheet (e.g. IND), or the nearest
// possible compound name (e.g. IND-INC).
// @modified:		Does a modifier apply to this channel. What these are
// depends on the channel type.  Modifier is set in
// channel2. Examples are IIO_MOD_X for axial sensors about
// the 'x' axis.
// @indexed:		Specify the channel has a numerical index. If not,
// the channel index number will be suppressed for sysfs
// attributes but not for event codes.
// @output:		Channel is output.
// @differential:	Channel is differential.
// @has_ext_scan_type:	True if ext_scan_type is used instead of scan_type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_chan_spec {
    pub type: iio_chan_type,
    pub channel: c_int,
    pub channel2: c_int,
    pub address: c_ulong,
    pub scan_index: c_int,
    pub scan_type: iio_scan_type,
    pub ext_scan_type: *const iio_scan_type,
    pub num_ext_scan_type: c_uint,
}

//
// iio_channel_has_info() - Checks whether a channel supports a info attribute
// @chan: The channel to be queried
// @type: Type of the info attribute to be checked
//
// Returns true if the channels supports reporting values for the given info
// attribute type, false otherwise.
//
// iio_channel_has_available() - Checks if a channel has an available attribute
// @chan: The channel to be queried
// @type: Type of the available attribute to be checked
//
// Returns true if the channel supports reporting available values for the
// given attribute type, false otherwise.
//

extern "C" {
    pub fn iio_get_time_ns(indio_dev: *const iio_dev) -> i64;
}
//
// Device operating modes
// @INDIO_DIRECT_MODE: There is an access to either:
// a) The last single value available for devices that do not provide
// on-demand reads.
// b) A new value after performing an on-demand read otherwise.
// On most devices, this is a single-shot read. On some devices with data
// streams without an 'on-demand' function, this might also be the 'last value'
// feature. Above all, this mode internally means that we are not in any of the
// other modes, and sysfs reads should work.
// Device drivers should inform the core if they support this mode.
// @INDIO_BUFFER_TRIGGERED: Common mode when dealing with kfifo buffers.
// It indicates that an explicit trigger is required. This requests the core to
// attach a poll function when enabling the buffer, which is indicated by the
// _TRIGGERED suffix.
// The core will ensure this mode is set when registering a triggered buffer
// with iio_triggered_buffer_setup().
// @INDIO_BUFFER_SOFTWARE: Another kfifo buffer mode, but not event triggered.
// No poll function can be attached because there is no triggered infrastructure
// we can use to cause capture. There is a kfifo that the driver will fill, but
// not "only one scan at a time". Typically, hardware will have a buffer that
// can hold multiple scans. Software may read one or more scans at a single time
// and push the available data to a Kfifo. This means the core will not attach
// any poll function when enabling the buffer.
// The core will ensure this mode is set when registering a simple kfifo buffer
// with devm_iio_kfifo_buffer_setup().
// @INDIO_BUFFER_HARDWARE: For specific hardware, if unsure do not use this mode.
// Same as above but this time the buffer is not a kfifo where we have direct
// access to the data. Instead, the consumer driver must access the data through
// non software visible channels (or DMA when there is no demux possible in
// software)
// The core will ensure this mode is set when registering a dmaengine buffer
// with devm_iio_dmaengine_buffer_setup().
// @INDIO_EVENT_TRIGGERED: Very unusual mode.
// Triggers usually refer to an external event which will start data capture.
// Here it is kind of the opposite as, a particular state of the data might
// produce an event which can be considered as an event. We don't necessarily
// have access to the data itself, but to the event produced. For example, this
// can be a threshold detector. The internal path of this mode is very close to
// the INDIO_BUFFER_TRIGGERED mode.
// The core will ensure this mode is set when registering a triggered event.
// @INDIO_HARDWARE_TRIGGERED: Very unusual mode.
// Here, triggers can result in data capture and can be routed to multiple
// hardware components, which make them close to regular triggers in the way
// they must be managed by the core, but without the entire interrupts/poll
// functions burden. Interrupts are irrelevant as the data flow is hardware
// mediated and distributed.
//
pub const INDIO_DIRECT_MODE: c_uint = 0x01;
pub const INDIO_BUFFER_TRIGGERED: c_uint = 0x02;
pub const INDIO_BUFFER_SOFTWARE: c_uint = 0x04;
pub const INDIO_BUFFER_HARDWARE: c_uint = 0x08;
pub const INDIO_EVENT_TRIGGERED: c_uint = 0x10;
pub const INDIO_HARDWARE_TRIGGERED: c_uint = 0x20;

pub const INDIO_MAX_RAW_ELEMENTS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_val_int_plus_micro {
    pub integer: c_int,
    pub micro: c_int,
}

//
// struct iio_info - constant information about device
// @event_attrs:	event control attributes
// @attrs:		general purpose device attributes
// @read_raw:		function to request a value from the device.
// mask specifies which value. Note 0 means a reading of
// the channel in question.  Return value will specify the
// type of value returned by the device. val and val2 will
// contain the elements making up the returned value.
// @read_raw_multi:	function to return values from the device.
// mask specifies which value. Note 0 means a reading of
// the channel in question.  Return value will specify the
// type of value returned by the device. vals pointer
// contain the elements making up the returned value.
// max_len specifies maximum number of elements
// vals pointer can contain. val_len is used to return
// length of valid elements in vals.
// @read_avail:		function to return the available values from the device.
// mask specifies which value. Note 0 means the available
// values for the channel in question.  Return value
// specifies if a IIO_AVAIL_LIST or a IIO_AVAIL_RANGE is
// returned in vals. The type of the vals are returned in
// type and the number of vals is returned in length. For
// ranges, there are always three vals returned; min, step
// and max. For lists, all possible values are enumerated.
// @write_raw:		function to write a value to the device.
// Parameters are the same as for read_raw.
// @read_label:		function to request label name for a specified label,
// for better channel identification.
// @write_raw_get_fmt:	callback function to query the expected
// format/precision. If not set by the driver, write_raw
// returns IIO_VAL_INT_PLUS_MICRO.
// @read_event_config:	find out if the event is enabled.
// @write_event_config:	set if the event is enabled.
// @read_event_value:	read a configuration value associated with the event.
// @write_event_value:	write a configuration value for the event.
// @read_event_label:	function to request label name for a specified label,
// for better event identification.
// @validate_trigger:	function to validate the trigger when the
// current trigger gets changed.
// @get_current_scan_type: must be implemented by drivers that use ext_scan_type
// in the channel spec to return the index of the currently
// active ext_scan type for a channel.
// @update_scan_mode:	function to configure device and scan buffer when
// channels have changed
// @debugfs_reg_access:	function to read or write register value of device
// @fwnode_xlate:	fwnode based function pointer to obtain channel specifier index.
// @hwfifo_set_watermark: function pointer to set the current hardware
// fifo watermark level; see hwfifo_* entries in
// Documentation/ABI/testing/sysfs-bus-iio for details on
// how the hardware fifo operates
// @hwfifo_flush_to_buffer: function pointer to flush the samples stored
// in the hardware fifo to the device buffer. The driver
// should not flush more than count samples. The function
// must return the number of samples flushed, 0 if no
// samples were flushed or a negative integer if no samples
// were flushed and there was an error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_info {
    pub event_attrs: *const attribute_group,
    pub attrs: *const attribute_group,
    pub mask): c_long,
    pub mask): c_long,
    pub mask): c_long,
    pub mask): c_long,
    pub label): *mut c_char,
    pub mask): c_long,
    pub dir): iio_event_direction,
    pub state): bool,
    pub val2): *mut *mut iio_event_info info, int val, int,
    pub val2): iio_event_info info, int val, int,
    pub label): *mut c_char,
    pub trig): *mut iio_trigger,
    pub chan): *const iio_chan_spec,
    pub scan_mask): *const c_ulong,
    pub readval): *mut c_uint,
    pub iiospec): *const fwnode_reference_args,
    pub val): *mut *mut *mut int (hwfifo_set_watermark)(struct iio_dev indio_dev, unsigned int,
    pub count): c_uint,
}

//
// struct iio_buffer_setup_ops - buffer setup related callbacks
// @preenable:		[DRIVER] function to run prior to marking buffer enabled
// @postenable:		[DRIVER] function to run after marking buffer enabled
// @predisable:		[DRIVER] function to run prior to marking buffer
// disabled
// @postdisable:	[DRIVER] function to run after marking buffer disabled
// @validate_scan_mask: [DRIVER] function callback to check whether a given
// scan mask is valid for the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_buffer_setup_ops {
    pub ): *mut *mut int (preenable)(struct iio_dev,
    pub ): *mut *mut int (postenable)(struct iio_dev,
    pub ): *mut *mut int (predisable)(struct iio_dev,
    pub ): *mut *mut int (postdisable)(struct iio_dev,
    pub scan_mask): *const c_ulong,
}

//
// struct iio_dev - industrial I/O device
// @modes:		[DRIVER] bitmask listing all the operating modes
// supported by the IIO device. This list should be
// initialized before registering the IIO device. It can
// also be filed up by the IIO core, as a result of
// enabling particular features in the driver
// (see iio_triggered_event_setup()).
// @dev:		[DRIVER] device structure, should be assigned a parent
// and owner
// @buffer:		[DRIVER] any buffer present
// @scan_bytes:		[INTERN] num bytes captured to be fed to buffer demux
// @scan_timestamp_offset: [INTERN] cache of the offset (in bytes) for the
// timestamp in the scan buffer
// @available_scan_masks: [DRIVER] optional array of allowed bitmasks. Sort the
// array in order of preference, the most preferred
// masks first.
// @masklength:		[INTERN] the length of the mask established from
// channels
// @active_scan_mask:	[INTERN] union of all scan masks requested by buffers
// @scan_timestamp:	[INTERN] set if any buffers have requested timestamp
// @trig:		[INTERN] current device trigger (buffer modes)
// @pollfunc:		[DRIVER] function run on trigger being received
// @pollfunc_event:	[DRIVER] function run on events trigger being received
// @channels:		[DRIVER] channel specification structure table
// @num_channels:	[DRIVER] number of channels specified in @channels.
// @name:		[DRIVER] name of the device.
// @label:              [DRIVER] unique name to identify which device this is
// @info:		[DRIVER] callbacks and constant info from driver
// @setup_ops:		[DRIVER] callbacks to call before and after buffer
// enable/disable
// @priv:		[DRIVER] reference to driver's private information
// **MUST** be accessed **ONLY** via iio_priv() helper
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dev {
    pub modes: c_int,
    pub dev: device,
    pub buffer: *mut iio_buffer,
    pub scan_bytes: c_int,
    pub scan_timestamp_offset: unsigned int __private,
    pub available_scan_masks: *const c_ulong,
    pub masklength: unsigned int __private,
    pub active_scan_mask: *const c_ulong,
    pub scan_timestamp: bool __private,
    pub trig: *mut iio_trigger,
    pub pollfunc: *mut iio_poll_func,
    pub pollfunc_event: *mut iio_poll_func,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
    pub name: *const c_char,
    pub label: *const c_char,
    pub info: *const iio_info,
    pub setup_ops: *const iio_buffer_setup_ops,
    pub priv: *mut *mut void __private,
}

extern "C" {
    pub fn iio_device_id(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_device_get_current_mode(indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn iio_buffer_enabled(indio_dev: *mut iio_dev) -> bool;
}
// iio_find_channel_from_si(struct iio_dev *indio_dev, int si);
//
// iio_device_register() - register a device with the IIO subsystem
// @indio_dev:		Device structure filled by the device driver
//

extern "C" {
    pub fn __iio_device_register(indio_dev: *mut iio_dev, this_mod: *mut module) -> c_int;
}
extern "C" {
    pub fn iio_device_unregister(indio_dev: *mut iio_dev);
}
//
// devm_iio_device_register - Resource-managed iio_device_register()
// @dev:	Device to allocate iio_dev for
// @indio_dev:	Device structure filled by the device driver
//
// Managed iio_device_register.  The IIO device registered with this
// function is automatically unregistered on driver detach. This function
// calls iio_device_register() internally. Refer to that function for more
// information.
//
// RETURNS:
// 0 on success, negative error number on failure.
//

extern "C" {
    pub fn iio_push_event(indio_dev: *mut iio_dev, ev_code: u64, timestamp: i64) -> c_int;
}
extern "C" {
    pub fn __iio_dev_mode_lock(__acquires(indio_dev: *mut *mut iio_dev indio_dev));
}
extern "C" {
    pub fn __iio_dev_mode_unlock(__releases(indio_dev: *mut *mut iio_dev indio_dev));
}
//
// Helper functions that allow claim and release of direct mode
// in a fashion that doesn't generate many false positives from sparse.
// Note this must remain static inline in the header so that sparse
// can see the __acquires() and __releases() annotations.
//
// iio_device_claim_direct() - Keep device in direct mode
// @indio_dev:	the iio_dev associated with the device
//
// If the device is in direct mode it is guaranteed to stay
// that way until iio_device_release_direct() is called.
//
// Use with iio_device_release_direct().
//
// Returns: true on success, false on failure.
//
// iio_device_release_direct() - Releases claim on direct mode
// @indio_dev:	the iio_dev associated with the device
//
// Release the claim. Device is no longer guaranteed to stay
// in direct mode.
//
// Use with iio_device_claim_direct().
//

//
// iio_device_try_claim_buffer_mode() - Keep device in buffer mode
// @indio_dev:	the iio_dev associated with the device
//
// If the device is in buffer mode it is guaranteed to stay
// that way until iio_device_release_buffer_mode() is called.
//
// Use with iio_device_release_buffer_mode().
//
// Returns: true on success, false on failure.
//
// iio_device_release_buffer_mode() - releases claim on buffer mode
// @indio_dev:	the iio_dev associated with the device
//
// Release the claim. Device is no longer guaranteed to stay
// in buffer mode.
//
// Use with iio_device_try_claim_buffer_mode().
//

//
// These classes are not meant to be used directly by drivers (hence the
// __priv__ prefix). Instead, documented wrapper macros are provided below to
// enforce the use of ACQUIRE() or guard() semantics and avoid the problematic
// scoped guard variants.
//
// IIO_DEV_ACQUIRE_DIRECT_MODE() - Tries to acquire the direct mode lock with
// automatic release
// @dev: IIO device instance
// @claim: Variable identifier to store acquire result
//
// Tries to acquire the direct mode lock with cleanup ACQUIRE() semantics and
// automatically releases it at the end of the scope. It most be always paired
// with IIO_DEV_ACQUIRE_ERR(), for example (notice the scope braces)::
//
// switch() {
// case IIO_CHAN_INFO_RAW: {
// IIO_DEV_ACQUIRE_DIRECT_MODE(indio_dev, claim);
// if (IIO_DEV_ACQUIRE_FAILED(claim))
// return -EBUSY;
//
// ...
// }
// case IIO_CHAN_INFO_SCALE:
// ...
// }
//
// Context: Can sleep
//

//
// IIO_DEV_ACQUIRE_FAILED() - ACQUIRE_ERR() wrapper
// @claim: The claim variable passed to IIO_DEV_ACQUIRE_*_MODE()
//
// Return: true if failed to acquire the mode, otherwise false.
//

//
// IIO_DEV_GUARD_CURRENT_MODE() - Acquires the mode lock with automatic release
// @dev: IIO device instance
//
// Acquires the mode lock with cleanup guard() semantics. It is usually paired
// with iio_buffer_enabled().
//
// This should *not* be used to protect internal driver state and it's use in
// general is *strongly* discouraged. Use any of the IIO_DEV_ACQUIRE_*_MODE()
// variants.
//
// Context: Can sleep
//

//
// iio_device_put() - reference counted deallocation of struct device
// @indio_dev: IIO device structure containing the device
//
extern "C" {
    pub fn iio_device_get_clock(indio_dev: *const iio_dev) -> clockid_t;
}
extern "C" {
    pub fn iio_device_set_clock(indio_dev: *mut iio_dev, clock_id: clockid_t) -> c_int;
}
//
// dev_to_iio_dev() - Get IIO device struct from a device struct
// @dev: 		The device embedded in the IIO device
//
// Note: The device must be a IIO device, otherwise the result is undefined.
//
extern "C" {
    pub fn container_of(_arg: dev, iio_dev: struct, _arg: dev) -> return;
}
//
// iio_device_get() - increment reference count for the device
// @indio_dev: 		IIO device structure
//
// Returns: The passed IIO device
//
// iio_device_set_parent() - assign parent device to the IIO device object
// @indio_dev: 		IIO device structure
// @parent:		reference to parent device object
//
// This utility must be called between IIO device allocation
// (via devm_iio_device_alloc()) & IIO device registration
// (via iio_device_register() and devm_iio_device_register())).
// By default, the device allocation will also assign a parent device to
// the IIO device object. In cases where devm_iio_device_alloc() is used,
// sometimes the parent device must be different than the device used to
// manage the allocation.
// In that case, this helper should be used to change the parent, hence the
// requirement to call this between allocation & registration.
//
// iio_device_set_drvdata() - Set device driver data
// @indio_dev: IIO device structure
// @data: Driver specific data
//
// Allows to attach an arbitrary pointer to an IIO device, which can later be
// retrieved by iio_device_get_drvdata().
//
// iio_device_get_drvdata() - Get device driver data
// @indio_dev: IIO device structure
//
// Returns the data previously set with iio_device_set_drvdata()
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &indio_dev->dev) -> return;
}
//
// Used to ensure the iio_priv() structure is aligned to allow that structure
// to in turn include IIO_DMA_MINALIGN'd elements such as buffers which
// must not share  cachelines with the rest of the structure, thus making
// them safe for use with non-coherent DMA.
//
// A number of drivers also use this on buffers that include a 64-bit timestamp
// that is used with iio_push_to_buffers_with_ts(). Therefore, in the case where
// DMA alignment is not sufficient for proper timestamp alignment, we align to
// 8 bytes instead.
//

//
// IIO_DECLARE_BUFFER_WITH_TS() - Declare a buffer with timestamp
// @type: element type of the buffer
// @name: identifier name of the buffer
// @count: number of elements in the buffer
//
// Declares a buffer that is safe to use with iio_push_to_buffers_with_ts(). In
// addition to allocating enough space for @count elements of @type, it also
// allocates space for a s64 timestamp at the end of the buffer and ensures
// proper alignment of the timestamp.
//

//
// IIO_DECLARE_DMA_BUFFER_WITH_TS() - Declare a DMA-aligned buffer with timestamp
// @type: element type of the buffer
// @name: identifier name of the buffer
// @count: number of elements in the buffer
//
// Same as IIO_DECLARE_BUFFER_WITH_TS(), but is uses __aligned(IIO_DMA_MINALIGN)
// to ensure that the buffer doesn't share cachelines with anything that comes
// before it in a struct. This should not be used for stack-allocated buffers
// as stack memory cannot generally be used for DMA.
//

//
// IIO_DECLARE_QUATERNION() - Declare a quaternion element
// @type: element type of the individual vectors
// @name: identifier name
//
// Quaternions are a vector composed of 4 elements (W, X, Y, Z). Use this macro
// to declare a quaternion element in a struct to ensure proper alignment in
// an IIO buffer.
//

// The information at the returned address is guaranteed to be cacheline aligned
extern "C" {
    pub fn ACCESS_PRIVATE(_arg: indio_dev, _arg: priv) -> return;
}
extern "C" {
    pub fn iio_device_free(indio_dev: *mut iio_dev);
}

//
// iio_get_debugfs_dentry() - helper function to get the debugfs_dentry
// @indio_dev:		IIO device structure for device
//

//
// iio_device_suspend_triggering() - suspend trigger attached to an iio_dev
// @indio_dev: iio_dev associated with the device that will have triggers suspended
//
// Return 0 if successful, negative otherwise
//
extern "C" {
    pub fn iio_device_suspend_triggering(indio_dev: *mut iio_dev) -> c_int;
}
//
// iio_device_resume_triggering() - resume trigger attached to an iio_dev
// that was previously suspended with iio_device_suspend_triggering()
// @indio_dev: iio_dev associated with the device that will have triggers resumed
//
// Return 0 if successful, negative otherwise
//
extern "C" {
    pub fn iio_device_resume_triggering(indio_dev: *mut iio_dev) -> c_int;
}

extern "C" {
    pub fn iio_get_acpi_device_name_and_data(_arg: dev, _arg: NULL) -> return;
}
//
// iio_get_current_scan_type - Get the current scan type for a channel
// @indio_dev:	the IIO device to get the scan type for
// @chan:	the channel to get the scan type for
//
// Most devices only have one scan type per channel and can just access it
// directly without calling this function. Core IIO code and drivers that
// implement ext_scan_type in the channel spec should use this function to
// get the current scan type for a channel.
//
// Returns: the current scan type for the channel or error.
//
// iio_get_current_scan_type(const struct iio_dev *indio_dev,
extern "C" {
    pub fn ERR_PTR(_arg: ret) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
//
// iio_get_masklength - Get length of the channels mask
// @indio_dev: the IIO device to get the masklength for
//
extern "C" {
    pub fn ACCESS_PRIVATE(_arg: indio_dev, _arg: masklength) -> return;
}
extern "C" {
    pub fn iio_active_scan_mask_index(indio_dev: *mut iio_dev) -> c_int;
}
//
// iio_for_each_active_channel - Iterated over active channels
// @indio_dev: the IIO device
// @chan: Holds the index of the enabled channel
//

extern "C" {
    pub fn iio_format_value(buf: *mut c_char, type: c_uint, size: c_int, vals: *mut c_int) -> isize;
}
//
// IIO_DEGREE_TO_RAD() - Convert degree to rad
// @deg: A value in degree
//
// Returns the given value converted from degree to rad
//

//
// IIO_RAD_TO_DEGREE() - Convert rad to degree
// @rad: A value in rad
//
// Returns the given value converted from rad to degree
//

//
// IIO_G_TO_M_S_2() - Convert g to meter / second**2
// @g: A value in g
//
// Returns the given value converted from g to meter / second**2
//

//
// IIO_M_S_2_TO_G() - Convert meter / second**2 to g
// @ms2: A value in meter / second**2
//
// Returns the given value converted from meter / second**2 to g
//

