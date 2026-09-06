//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/pvrusb2/pvrusb2-hdw-internal.h
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
// Copyright (C) 2005 Mike Isely <isely@pobox.com>
//

// Legal values for PVR2_CID_HSM
pub const PVR2_CVAL_HSM_FAIL: c_int = 0;
pub const PVR2_CVAL_HSM_FULL: c_int = 1;
pub const PVR2_CVAL_HSM_HIGH: c_int = 2;
pub const PVR2_VID_ENDPOINT: c_uint = 0x84;
pub const PVR2_UNK_ENDPOINT: c_uint = 0x86    /* maybe raw yuv ? */;
pub const PVR2_VBI_ENDPOINT: c_uint = 0x88;
pub const PVR2_CTL_BUFFSIZE: c_int = 64;
pub const FREQTABLE_SIZE: c_int = 500;

extern "C" {
    pub fn int(: *mut *mut pvr2_ctlf_is_dirty)(struct pvr2_ctrl) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut pvr2_ctlf_clear_dirty)(struct pvr2_ctrl) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut pvr2_ctlf_check_value)(struct pvr2_ctrl, _arg: c_int) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut pvr2_ctlf_get_value)(struct pvr2_ctrl, : *mut c_int) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut pvr2_ctlf_set_value)(struct pvr2_ctrl, msk: c_int, val: c_int) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut pvr2_ctlf_get_v4lflags)(struct pvr2_ctrl) -> typedef unsigned;
}
// This structure describes a specific control.  A table of these is set up
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_ctl_info {
// Control's name suitable for use as an identifier
    pub name: *const c_char,
// Short description of control
    pub desc: *const c_char,
// Control's implementation
    pub /: *mut *mut pvr2_ctlf_get_value get_value; / Get its value,
    pub /: *mut *mut pvr2_ctlf_get_value get_def_value; / Get its default value,
    pub /: *mut *mut pvr2_ctlf_get_value get_min_value; / Get minimum allowed value,
    pub /: *mut *mut pvr2_ctlf_get_value get_max_value; / Get maximum allowed value,
    pub /: *mut *mut pvr2_ctlf_set_value set_value; / Set its value,
    pub /: *mut *mut pvr2_ctlf_check_value check_value; / Check that value is valid,
    pub /: *mut *mut pvr2_ctlf_val_to_sym val_to_sym; / Custom convert value->symbol,
    pub /: *mut *mut pvr2_ctlf_sym_to_val sym_to_val; / Custom convert symbol->value,
    pub /: *mut *mut pvr2_ctlf_is_dirty is_dirty; / Return true if dirty,
    pub /: *mut *mut pvr2_ctlf_clear_dirty clear_dirty; / Clear dirty state,
    pub /: *mut *mut pvr2_ctlf_get_v4lflags get_v4lflags;/ Retrieve v4l flags,
// Control's type (int, enum, bitmask)
    pub type: pvr2_ctl_type,
// Associated V4L control ID, if any
    pub v4l_id: c_int,
// Associated driver internal ID, if any
    pub internal_id: c_int,
// Don't implicitly initialize this control's value
    pub skip_init: c_int,
// Starting value for this control
    pub default_value: c_int,
// Type-specific control information
    pub /: *mut *mut long min_value; / lower limit,
    pub /: *mut *mut long max_value; / upper limit,
    pub type_int: },
    pub /: *mut *mut unsigned int count; / enum value count,
    pub /: *const *const *const *const char  value_names; / symbol names,
    pub type_enum: },
    pub /: *mut *mut unsigned int valid_bits; / bits in use,
    pub /: *const *const *const *const char bit_names; / symbol name/bit,
    pub type_bitmask: },
    pub def: },
}

// Same as pvr2_ctl_info, but includes storage for the control description
pub const PVR2_CTLD_INFO_DESC_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_ctld_info {
    pub info: pvr2_ctl_info,
    pub desc: [c_char; PVR2_CTLD_INFO_DESC_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_ctrl {
    pub info: *const pvr2_ctl_info,
    pub hdw: *mut pvr2_hdw,
}

// Disposition of firmware1 loading situation
pub const FW1_STATE_UNKNOWN: c_int = 0;
pub const FW1_STATE_MISSING: c_int = 1;
pub const FW1_STATE_FAILED: c_int = 2;
pub const FW1_STATE_RELOAD: c_int = 3;
pub const FW1_STATE_OK: c_int = 4;
// What state the device is in if it is a hybrid
pub const PVR2_PATHWAY_UNKNOWN: c_int = 0;
pub const PVR2_PATHWAY_ANALOG: c_int = 1;
pub const PVR2_PATHWAY_DIGITAL: c_int = 2;
extern "C" {
    pub fn int(: *mut *mut pvr2_i2c_func)(struct pvr2_hdw, _arg: u8, : *mut u8, _arg: u16, : *mut u8, _arg: u16) -> typedef;
}
pub const PVR2_I2C_FUNC_CNT: c_int = 128;
// This structure contains all state data directly needed to
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr2_hdw {
// Underlying USB device handle
    pub usb_dev: *mut usb_device,
    pub usb_intf: *mut usb_interface,
// Our handle into the v4l2 sub-device architecture
    pub v4l2_dev: v4l2_device,
// Device description, anything that must adjust behavior based on
    pub hdw_desc: *const pvr2_device_desc,
// Kernel worker thread handling
    pub /: *mut *mut work_workpoll; / Update driver state,
// Video spigot
    pub vid_stream: *mut pvr2_stream,
// Mutex for all hardware state control
    pub big_lock_mutex: mutex,
    pub /: *mut *mut int big_lock_held; / For debugging,
// This is a simple string which identifies the instance of this
    pub name: [c_char; 32],
// This is a simple string which identifies the physical device
    pub identifier: [c_char; 32],
// I2C stuff
    pub i2c_adap: i2c_adapter,
    pub i2c_algo: i2c_algorithm,
    pub i2c_func: [pvr2_i2c_func; PVR2_I2C_FUNC_CNT],
    pub i2c_cx25840_hack_state: c_int,
    pub i2c_linked: c_int,
// IR related
    pub /: *mut *mut unsigned int ir_scheme_active; / IR scheme as seen from the outside,
    pub /: *mut *mut IR_i2c_init_data ir_init_data; / params passed to IR modules,
// Frequency table
    pub freqTable: [c_uint; FREQTABLE_SIZE],
    pub freqProgSlot: c_uint,
// Stuff for handling low level control interaction with device
    pub ctl_lock_mutex: mutex,
    pub /: *mut *mut int ctl_lock_held; / For debugging,
    pub ctl_write_urb: *mut urb,
    pub ctl_read_urb: *mut urb,
    pub ctl_write_buffer: *mut c_uchar,
    pub ctl_read_buffer: *mut c_uchar,
    pub ctl_write_pend_flag: c_int,
    pub ctl_read_pend_flag: c_int,
    pub ctl_timeout_flag: c_int,
    pub ctl_done: completion,
    pub cmd_buffer: [c_uchar; PVR2_CTL_BUFFSIZE],
    pub info: int cmd_debug_state; // Low level command debugging,
    pub //: unsigned char cmd_debug_code;,
    pub //: unsigned int cmd_debug_write_len;,
    pub //: unsigned int cmd_debug_read_len;,
// Bits of state that describe what is going on with various parts
    pub /: *mut *mut int state_pathway_ok; / Pathway config is ok,
    pub /: *mut *mut int state_encoder_ok; / Encoder is operational,
    pub /: *mut *mut int state_encoder_run; / Encoder is running,
    pub /: *mut *mut int state_encoder_config; / Encoder is configured,
    pub /: *mut *mut int state_encoder_waitok; / Encoder pre-wait done,
    pub /: *mut *mut int state_encoder_runok; / Encoder has run for >= .25 sec,
    pub /: *mut *mut int state_decoder_run; / Decoder is running,
    pub /: *mut *mut int state_decoder_ready; / Decoder is stabilized & streamable,
    pub /: *mut *mut int state_usbstream_run; / FX2 is streaming,
    pub /: *mut *mut int state_decoder_quiescent; / Decoder idle for minimal interval,
    pub /: *mut *mut int state_pipeline_config; / Pipeline is configured,
    pub /: *mut *mut int state_pipeline_req; / Somebody wants to stream,
    pub /: *mut *mut int state_pipeline_pause; / Pipeline must be paused,
    pub /: *mut *mut int state_pipeline_idle; / Pipeline not running,
// This is the master state of the driver.  It is the combined
    pub master_state: c_uint,
// True if device led is currently on
    pub led_on: c_int,
// True if states must be re-evaluated
    pub state_stale: c_int,
    pub ): *mut *mut void (state_func)(void,
    pub state_data: *mut c_void,
// Timer for measuring required decoder settling time before we're
    pub quiescent_timer: timer_list,
// Timer for measuring decoder stabilization time, which is the
    pub decoder_stabilization_timer: timer_list,
// Timer for measuring encoder pre-wait time
    pub encoder_wait_timer: timer_list,
// Timer for measuring encoder minimum run time
    pub encoder_run_timer: timer_list,
// Place to block while waiting for state changes
    pub state_wait_data: wait_queue_head_t,
    pub /: *mut *mut int force_dirty; / consider all controls dirty if true,
    pub /: *mut *mut int flag_ok; / device in known good state,
    pub /: *mut *mut int flag_modulefail; / true if at least one module failed to load,
    pub /: *mut *mut int flag_disconnected; / flag_ok == 0 due to disconnect,
    pub /: *mut *mut int flag_init_ok; / true if structure is fully initialized,
    pub /: *mut *mut int fw1_state; / current situation with fw1,
    pub /: *mut *mut int pathway_state; / one of PVR2_PATHWAY_xxx,
    pub /: *mut *mut int flag_decoder_missed;/ We've noticed missing decoder,
    pub /: *mut *mut int flag_tripped; / Indicates overall failure to start,
    pub decoder_client_id: c_uint,
// CPU firmware info (used to help find / save firmware data)
    pub fw_buffer: *mut c_char,
    pub fw_size: c_uint,
    pub /: *mut *mut int fw_cpu_flag; / True if we are dealing with the CPU,
// Tuner / frequency control stuff
    pub tuner_type: c_uint,
    pub tuner_updated: c_int,
    pub /: *mut *mut unsigned int freqValTelevision; / Current freq for tv mode,
    pub /: *mut *mut unsigned int freqValRadio; / Current freq for radio mode,
    pub /: *mut *mut unsigned int freqSlotTelevision; / Current slot for tv mode,
    pub /: *mut *mut unsigned int freqSlotRadio; / Current slot for radio mode,
    pub /: *mut *mut unsigned int freqSelector; / 0=radio 1=television,
    pub freqDirty: c_int,
// Current tuner info - this information is polled from the I2C bus
    pub tuner_signal_info: v4l2_tuner,
    pub tuner_signal_stale: c_int,
// Cropping capability info
    pub cropcap_info: v4l2_cropcap,
    pub cropcap_stale: c_int,
// Video standard handling
    pub selections: v4l2_std_id std_mask_eeprom; // Hardware supported,
    pub from: v4l2_std_id std_mask_avail; // Which standards we may select,
    pub standard(s): v4l2_std_id std_mask_cur; // Currently selected,
    pub value: int std_enum_cur; // selected standard enumeration,
    pub changed: int std_dirty; // True if std_mask_cur has,
    pub std_info_enum: pvr2_ctl_info,
    pub std_info_avail: pvr2_ctl_info,
    pub std_info_cur: pvr2_ctl_info,
    pub std_info_detect: pvr2_ctl_info,
// Generated string names, one per actual V4L2 standard
    pub std_mask_ptrs: [*const c_char; 32],
    pub std_mask_names: [c_char; 32][16],
    pub /: *mut *mut int unit_number; / ID for driver instance,
    pub /: *mut *mut unsigned long serial_number; / ID for hardware itself,
    pub /: *mut *mut char bus_info[32]; / Bus location info,
// Minor numbers used by v4l logic (yes, this is a hack, as there
    pub v4l_minor_number_video: c_int,
    pub v4l_minor_number_vbi: c_int,
    pub v4l_minor_number_radio: c_int,
// Bit mask of PVR2_CVAL_INPUT choices which are valid for the hardware
    pub input_avail_mask: c_uint,
// Bit mask of PVR2_CVAL_INPUT choices which are currently allowed
    pub input_allowed_mask: c_uint,
// Location of eeprom or a negative number if none
    pub eeprom_addr: c_int,
    pub active_stream_type: pvr2_config,
    pub desired_stream_type: pvr2_config,
// Control state needed for cx2341x module
    pub enc_cur_state: cx2341x_mpeg_params,
    pub enc_ctl_state: cx2341x_mpeg_params,
// True if an encoder attribute has changed
    pub enc_stale: c_int,
// True if an unsafe encoder attribute has changed
    pub enc_unsafe_stale: c_int,
// True if enc_cur_state is valid
    pub enc_cur_valid: c_int,
// Control state

    pub mpeg_ctrl_info: *mut pvr2_ctld_info,
    pub controls: *mut pvr2_ctrl,
    pub control_cnt: c_uint,
}

// This function gets the current frequency
extern "C" {
    pub fn pvr2_hdw_get_cur_freq(: *mut pvr2_hdw) -> c_ulong;
}
extern "C" {
    pub fn pvr2_hdw_status_poll(: *mut pvr2_hdw);
}
