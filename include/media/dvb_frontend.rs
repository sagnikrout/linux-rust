//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/dvb_frontend.h
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


//
// dvb_frontend.h
//
// The Digital TV Frontend kABI defines a driver-internal interface for
// registering low-level, hardware specific driver to a hardware independent
// frontend layer.
//
// Copyright (C) 2001 convergence integrated media GmbH
// Copyright (C) 2004 convergence GmbH
//
// Written by Ralph Metzler
// Overhauled by Holger Waechtler
// Kernel I2C stuff by Michael Hunold <hunold@convergence.de>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.
//

//
// Maximum number of Delivery systems per frontend. It
// should be smaller or equal to 32
//
pub const MAX_DELSYS: c_int = 8;
// Helper definitions to be used at frontend drivers

//
// struct dvb_frontend_tune_settings - parameters to adjust frontend tuning
//
// @min_delay_ms:	minimum delay for tuning, in ms
// @step_size:		step size between two consecutive frequencies
// @max_drift:		maximum drift
//
// NOTE: step_size is in Hz, for terrestrial/cable or kHz for satellite
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_tune_settings {
    pub min_delay_ms: c_int,
    pub step_size: c_int,
    pub max_drift: c_int,
}

//
// struct dvb_tuner_info - Frontend name and min/max ranges/bandwidths
//
// @name:		name of the Frontend
// @frequency_min_hz:	minimal frequency supported in Hz
// @frequency_max_hz:	maximum frequency supported in Hz
// @frequency_step_hz:	frequency step in Hz
// @bandwidth_min:	minimal frontend bandwidth supported
// @bandwidth_max:	maximum frontend bandwidth supported
// @bandwidth_step:	frontend bandwidth step
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_tuner_info {
    pub name: [c_char; 128],
    pub frequency_min_hz: u32,
    pub frequency_max_hz: u32,
    pub frequency_step_hz: u32,
    pub bandwidth_min: u32,
    pub bandwidth_max: u32,
    pub bandwidth_step: u32,
}

//
// struct analog_parameters - Parameters to tune into an analog/radio channel
//
// @frequency:	Frequency used by analog TV tuner (either in 62.5 kHz step,
// for TV, or 62.5 Hz for radio)
// @mode:	Tuner mode, as defined on enum v4l2_tuner_type
// @audmode:	Audio mode as defined for the rxsubchans field at videodev2.h,
// e. g. V4L2_TUNER_MODE_
// @std:	TV standard bitmap as defined at videodev2.h, e. g. V4L2_STD_
//
// Hybrid tuners should be supported by both V4L2 and DVB APIs. This
// struct contains the data that are used by the V4L2 side. To avoid
// dependencies from V4L2 headers, all enums here are declared as integers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct analog_parameters {
    pub frequency: c_uint,
    pub mode: c_uint,
    pub audmode: c_uint,
    pub std: u64,
}

//
// enum dvbfe_algo - defines the algorithm used to tune into a channel
//
// @DVBFE_ALGO_HW: Hardware Algorithm -
// Devices that support this algorithm do everything in hardware
// and no software support is needed to handle them.
// Requesting these devices to LOCK is the only thing required,
// device is supposed to do everything in the hardware.
//
// @DVBFE_ALGO_SW: Software Algorithm -
// These are dumb devices, that require software to do everything
//
// @DVBFE_ALGO_CUSTOM: Customizable Agorithm -
// Devices having this algorithm can be customized to have specific
// algorithms in the frontend driver, rather than simply doing a
// software zig-zag. In this case the zigzag maybe hardware assisted
// or it maybe completely done in hardware. In all cases, usage of
// this algorithm, in conjunction with the search and track
// callbacks, utilizes the driver specific algorithm.
//
// @DVBFE_ALGO_RECOVERY: Recovery Algorithm -
// These devices have AUTO recovery capabilities from LOCK failure
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvbfe_algo {
    DVBFE_ALGO_HW			= BIT(0),
    DVBFE_ALGO_SW			= BIT(1),
    DVBFE_ALGO_CUSTOM		= BIT(2),
    DVBFE_ALGO_RECOVERY		= BIT(31),
}

//
// enum dvbfe_search - search callback possible return status
//
// @DVBFE_ALGO_SEARCH_SUCCESS:
// The frontend search algorithm completed and returned successfully
//
// @DVBFE_ALGO_SEARCH_ASLEEP:
// The frontend search algorithm is sleeping
//
// @DVBFE_ALGO_SEARCH_FAILED:
// The frontend search for a signal failed
//
// @DVBFE_ALGO_SEARCH_INVALID:
// The frontend search algorithm was probably supplied with invalid
// parameters and the search is an invalid one
//
// @DVBFE_ALGO_SEARCH_ERROR:
// The frontend search algorithm failed due to some error
//
// @DVBFE_ALGO_SEARCH_AGAIN:
// The frontend search algorithm was requested to search again
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dvbfe_search {
    DVBFE_ALGO_SEARCH_SUCCESS	= BIT(0),
    DVBFE_ALGO_SEARCH_ASLEEP	= BIT(1),
    DVBFE_ALGO_SEARCH_FAILED	= BIT(2),
    DVBFE_ALGO_SEARCH_INVALID	= BIT(3),
    DVBFE_ALGO_SEARCH_AGAIN		= BIT(4),
    DVBFE_ALGO_SEARCH_ERROR		= BIT(31),
}

//
// struct dvb_tuner_ops - Tuner information and callbacks
//
// @info:		embedded &struct dvb_tuner_info with tuner properties
// @release:		callback function called when frontend is detached.
// drivers should free any allocated memory.
// @init:		callback function used to initialize the tuner device.
// @sleep:		callback function used to put the tuner to sleep.
// @suspend:		callback function used to inform that the Kernel will
// suspend.
// @resume:		callback function used to inform that the Kernel is
// resuming from suspend.
// @set_params:		callback function used to inform the tuner to tune
// into a digital TV channel. The properties to be used
// are stored at &struct dvb_frontend.dtv_property_cache.
// The tuner demod can change the parameters to reflect
// the changes needed for the channel to be tuned, and
// update statistics. This is the recommended way to set
// the tuner parameters and should be used on newer
// drivers.
// @set_analog_params:	callback function used to tune into an analog TV
// channel on hybrid tuners. It passes @analog_parameters
// to the driver.
// @set_config:		callback function used to send some tuner-specific
// parameters.
// @get_frequency:	get the actual tuned frequency
// @get_bandwidth:	get the bandwidth used by the low pass filters
// @get_if_frequency:	get the Intermediate Frequency, in Hz. For baseband,
// should return 0.
// @get_status:		returns the frontend lock status
// @get_rf_strength:	returns the RF signal strength. Used mostly to support
// analog TV and radio. Digital TV should report, instead,
// via DVBv5 API (&struct dvb_frontend.dtv_property_cache).
// @get_afc:		Used only by analog TV core. Reports the frequency
// drift due to AFC.
// @calc_regs:		callback function used to pass register data settings
// for simple tuners.  Shouldn't be used on newer drivers.
// @set_frequency:	Set a new frequency. Shouldn't be used on newer drivers.
// @set_bandwidth:	Set a new frequency. Shouldn't be used on newer drivers.
//
// NOTE: frequencies used on @get_frequency and @set_frequency are in Hz for
// terrestrial/cable or kHz for satellite.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_tuner_ops {
    pub info: dvb_tuner_info,
    pub fe): *mut *mut void (release)(struct dvb_frontend,
    pub fe): *mut *mut int (init)(struct dvb_frontend,
    pub fe): *mut *mut int (sleep)(struct dvb_frontend,
    pub fe): *mut *mut int (suspend)(struct dvb_frontend,
    pub fe): *mut *mut int (resume)(struct dvb_frontend,
// This is the recommended way to set the tuner
    pub fe): *mut *mut int (set_params)(struct dvb_frontend,
    pub p): *mut *mut *mut int (set_analog_params)(struct dvb_frontend fe, struct analog_parameters,
    pub priv_cfg): *mut *mut *mut int (set_config)(struct dvb_frontend fe, void,
    pub frequency): *mut *mut *mut int (get_frequency)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (get_bandwidth)(struct dvb_frontend fe, u32,
    pub frequency): *mut *mut *mut int (get_if_frequency)(struct dvb_frontend fe, u32,
pub const TUNER_STATUS_LOCKED: c_int = 1;
pub const TUNER_STATUS_STEREO: c_int = 2;
    pub status): *mut *mut *mut int (get_status)(struct dvb_frontend fe, u32,
    pub strength): *mut *mut *mut int (get_rf_strength)(struct dvb_frontend fe, u16,
    pub afc): *mut *mut *mut int (get_afc)(struct dvb_frontend fe, s32,
//
// This is support for demods like the mt352 - fills out the supplied
// buffer with what to write.
//
// Don't use on newer drivers.
//
    pub buf_len): *mut *mut *mut *mut int (calc_regs)(struct dvb_frontend fe, u8 buf, int,
//
// These are provided separately from set_params in order to
// facilitate silicon tuners which require sophisticated tuning loops,
// controlling each parameter separately.
//
// Don't use on newer drivers.
//
    pub frequency): *mut *mut *mut int (set_frequency)(struct dvb_frontend fe, u32,
    pub bandwidth): *mut *mut *mut int (set_bandwidth)(struct dvb_frontend fe, u32,
}

//
// struct analog_demod_info - Information struct for analog TV part of the demod
//
// @name:	Name of the analog TV demodulator
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct analog_demod_info {
    pub name: *mut c_char,
}

//
// struct analog_demod_ops  - Demodulation information and callbacks for
// analog TV and radio
//
// @info:		pointer to struct analog_demod_info
// @set_params:		callback function used to inform the demod to set the
// demodulator parameters needed to decode an analog or
// radio channel. The properties are passed via
// &struct analog_params.
// @has_signal:		returns 0xffff if has signal, or 0 if it doesn't.
// @get_afc:		Used only by analog TV core. Reports the frequency
// drift due to AFC.
// @tuner_status:	callback function that returns tuner status bits, e. g.
// %TUNER_STATUS_LOCKED and %TUNER_STATUS_STEREO.
// @standby:		set the tuner to standby mode.
// @release:		callback function called when frontend is detached.
// drivers should free any allocated memory.
// @i2c_gate_ctrl:	controls the I2C gate. Newer drivers should use I2C
// mux support instead.
// @set_config:		callback function used to send some tuner-specific
// parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct analog_demod_ops {
    pub info: analog_demod_info,
    pub params): *mut analog_parameters,
    pub signal): *mut *mut *mut int (has_signal)(struct dvb_frontend fe, u16,
    pub afc): *mut *mut *mut int (get_afc)(struct dvb_frontend fe, s32,
    pub fe): *mut *mut void (tuner_status)(struct dvb_frontend,
    pub fe): *mut *mut void (standby)(struct dvb_frontend,
    pub fe): *mut *mut void (release)(struct dvb_frontend,
    pub enable): *mut *mut *mut int (i2c_gate_ctrl)(struct dvb_frontend fe, int,
// This is to allow setting tuner-specific configuration
    pub priv_cfg): *mut *mut *mut int (set_config)(struct dvb_frontend fe, void,
}

//
// struct dvb_frontend_internal_info - Frontend properties and capabilities
//
// @name:			Name of the frontend
// @frequency_min_hz:		Minimal frequency supported by the frontend.
// @frequency_max_hz:		Minimal frequency supported by the frontend.
// @frequency_stepsize_hz:	All frequencies are multiple of this value.
// @frequency_tolerance_hz:	Frequency tolerance.
// @symbol_rate_min:		Minimal symbol rate, in bauds
// (for Cable/Satellite systems).
// @symbol_rate_max:		Maximal symbol rate, in bauds
// (for Cable/Satellite systems).
// @symbol_rate_tolerance:	Maximal symbol rate tolerance, in ppm
// (for Cable/Satellite systems).
// @caps:			Capabilities supported by the frontend,
// as specified in &enum fe_caps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_internal_info {
    pub name: [c_char; 128],
    pub frequency_min_hz: u32,
    pub frequency_max_hz: u32,
    pub frequency_stepsize_hz: u32,
    pub frequency_tolerance_hz: u32,
    pub symbol_rate_min: u32,
    pub symbol_rate_max: u32,
    pub symbol_rate_tolerance: u32,
    pub caps: fe_caps,
}

//
// struct dvb_frontend_ops - Demodulation information and callbacks for
// ditialt TV
//
// @info:		embedded &struct dvb_tuner_info with tuner properties
// @delsys:		Delivery systems supported by the frontend
// @detach:		callback function called when frontend is detached.
// drivers should clean up, but not yet free the &struct
// dvb_frontend allocation.
// @release:		callback function called when frontend is ready to be
// freed.
// drivers should free any allocated memory.
// @release_sec:	callback function requesting that the Satellite Equipment
// Control (SEC) driver to release and free any memory
// allocated by the driver.
// @init:		callback function used to initialize the tuner device.
// @sleep:		callback function used to put the tuner to sleep.
// @suspend:		callback function used to inform that the Kernel will
// suspend.
// @resume:		callback function used to inform that the Kernel is
// resuming from suspend.
// @write:		callback function used by some demod legacy drivers to
// allow other drivers to write data into their registers.
// Should not be used on new drivers.
// @tune:		callback function used by demod drivers that use
// @DVBFE_ALGO_HW to tune into a frequency.
// @get_frontend_algo:	returns the desired hardware algorithm.
// @set_frontend:	callback function used to inform the demod to set the
// parameters for demodulating a digital TV channel.
// The properties to be used are stored at &struct
// dvb_frontend.dtv_property_cache. The demod can change
// the parameters to reflect the changes needed for the
// channel to be decoded, and update statistics.
// @get_tune_settings:	callback function
// @get_frontend:	callback function used to inform the parameters
// actuall in use. The properties to be used are stored at
// &struct dvb_frontend.dtv_property_cache and update
// statistics. Please notice that it should not return
// an error code if the statistics are not available
// because the demog is not locked.
// @read_status:	returns the locking status of the frontend.
// @read_ber:		legacy callback function to return the bit error rate.
// Newer drivers should provide such info via DVBv5 API,
// e. g. @set_frontend;/@get_frontend, implementing this
// callback only if DVBv3 API compatibility is wanted.
// @read_signal_strength: legacy callback function to return the signal
// strength. Newer drivers should provide such info via
// DVBv5 API, e. g. @set_frontend/@get_frontend,
// implementing this callback only if DVBv3 API
// compatibility is wanted.
// @read_snr:		legacy callback function to return the Signal/Noise
// rate. Newer drivers should provide such info via
// DVBv5 API, e. g. @set_frontend/@get_frontend,
// implementing this callback only if DVBv3 API
// compatibility is wanted.
// @read_ucblocks:	legacy callback function to return the Uncorrected Error
// Blocks. Newer drivers should provide such info via
// DVBv5 API, e. g. @set_frontend/@get_frontend,
// implementing this callback only if DVBv3 API
// compatibility is wanted.
// @diseqc_reset_overload: callback function to implement the
// FE_DISEQC_RESET_OVERLOAD() ioctl (only Satellite)
// @diseqc_send_master_cmd: callback function to implement the
// FE_DISEQC_SEND_MASTER_CMD() ioctl (only Satellite).
// @diseqc_recv_slave_reply: callback function to implement the
// FE_DISEQC_RECV_SLAVE_REPLY() ioctl (only Satellite)
// @diseqc_send_burst:	callback function to implement the
// FE_DISEQC_SEND_BURST() ioctl (only Satellite).
// @set_tone:		callback function to implement the
// FE_SET_TONE() ioctl (only Satellite).
// @set_voltage:	callback function to implement the
// FE_SET_VOLTAGE() ioctl (only Satellite).
// @enable_high_lnb_voltage: callback function to implement the
// FE_ENABLE_HIGH_LNB_VOLTAGE() ioctl (only Satellite).
// @dishnetwork_send_legacy_command: callback function to implement the
// FE_DISHNETWORK_SEND_LEGACY_CMD() ioctl (only Satellite).
// Drivers should not use this, except when the DVB
// core emulation fails to provide proper support (e.g.
// if @set_voltage takes more than 8ms to work), and
// when backward compatibility with this legacy API is
// required.
// @i2c_gate_ctrl:	controls the I2C gate. Newer drivers should use I2C
// mux support instead.
// @ts_bus_ctrl:	callback function used to take control of the TS bus.
// @set_lna:		callback function to power on/off/auto the LNA.
// @search:		callback function used on some custom algo search algos.
// @tuner_ops:		pointer to &struct dvb_tuner_ops
// @analog_ops:		pointer to &struct analog_demod_ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend_ops {
    pub info: dvb_frontend_internal_info,
    pub delsys: [u8; MAX_DELSYS],
    pub fe): *mut *mut void (detach)(struct dvb_frontend,
    pub fe): *mut *mut *mut void (release)(struct dvb_frontend,
    pub fe): *mut *mut *mut void (release_sec)(struct dvb_frontend,
    pub fe): *mut *mut *mut int (init)(struct dvb_frontend,
    pub fe): *mut *mut *mut int (sleep)(struct dvb_frontend,
    pub fe): *mut *mut int (suspend)(struct dvb_frontend,
    pub fe): *mut *mut int (resume)(struct dvb_frontend,
    pub len): *const *const *const int (write)(struct dvb_frontend fe, u8 buf[], int,
// if this is set, it overrides the default swzigzag
    pub status): *mut fe_status,
// get frontend tuning algorithm from the module
    pub fe): *mut *mut dvbfe_algo (get_frontend_algo)(struct dvb_frontend,
// these two are only used for the swzigzag code
    pub fe): *mut *mut int (set_frontend)(struct dvb_frontend,
    pub settings): *mut *mut *mut *mut int (get_tune_settings)(struct dvb_frontend fe, struct dvb_frontend_tune_settings,
    pub props): *mut dtv_frontend_properties,
    pub status): *mut *mut *mut int (read_status)(struct dvb_frontend fe, enum fe_status,
    pub ber): *mut *mut *mut *mut int (read_ber)(struct dvb_frontend fe, u32,
    pub strength): *mut *mut *mut *mut int (read_signal_strength)(struct dvb_frontend fe, u16,
    pub snr): *mut *mut *mut *mut int (read_snr)(struct dvb_frontend fe, u16,
    pub ucblocks): *mut *mut *mut *mut int (read_ucblocks)(struct dvb_frontend fe, u32,
    pub fe): *mut *mut *mut int (diseqc_reset_overload)(struct dvb_frontend,
    pub cmd): *mut *mut *mut *mut int (diseqc_send_master_cmd)(struct dvb_frontend fe, struct dvb_diseqc_master_cmd,
    pub reply): *mut *mut *mut *mut int (diseqc_recv_slave_reply)(struct dvb_frontend fe, struct dvb_diseqc_slave_reply,
    pub minicmd): fe_sec_mini_cmd,
    pub tone): *mut *mut *mut int (set_tone)(struct dvb_frontend fe, enum fe_sec_tone_mode,
    pub voltage): fe_sec_voltage,
    pub arg): *mut *mut *mut int (enable_high_lnb_voltage)(struct dvb_frontend fe, long,
    pub cmd): *mut *mut *mut int (dishnetwork_send_legacy_command)(struct dvb_frontend fe, unsigned long,
    pub enable): *mut *mut *mut int (i2c_gate_ctrl)(struct dvb_frontend fe, int,
    pub acquire): *mut *mut *mut int (ts_bus_ctrl)(struct dvb_frontend fe, int,
    pub ): *mut *mut int (set_lna)(struct dvb_frontend,
//
// These callbacks are for devices that implement their own
// tuning algorithms, rather than a simple swzigzag
//
    pub fe): *mut *mut dvbfe_search (search)(struct dvb_frontend,
    pub tuner_ops: dvb_tuner_ops,
    pub analog_ops: analog_demod_ops,
}

pub const MAX_EVENT: c_int = 8;
// Used only internally at dvb_frontend.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_fe_events {
    pub events: [dvb_frontend_event; MAX_EVENT],
    pub eventw: c_int,
    pub eventr: c_int,
    pub overflow: c_int,
    pub wait_queue: wait_queue_head_t,
    pub mtx: mutex,
}

//
// struct dtv_frontend_properties - contains a list of properties that are
// specific to a digital TV standard.
//
// @frequency:		frequency in Hz for terrestrial/cable or in kHz for
// Satellite
// @modulation:		Frontend modulation type
// @voltage:		SEC voltage (only Satellite)
// @sectone:		SEC tone mode (only Satellite)
// @inversion:		Spectral inversion
// @fec_inner:		Forward error correction inner Code Rate
// @transmission_mode:	Transmission Mode
// @bandwidth_hz:	Bandwidth, in Hz. A zero value means that userspace
// wants to autodetect.
// @guard_interval:	Guard Interval
// @hierarchy:		Hierarchy
// @symbol_rate:	Symbol Rate
// @code_rate_HP:	high priority stream code rate
// @code_rate_LP:	low priority stream code rate
// @pilot:		Enable/disable/autodetect pilot tones
// @rolloff:		Rolloff factor (alpha)
// @delivery_system:	FE delivery system (e. g. digital TV standard)
// @interleaving:	interleaving
// @isdbt_partial_reception: ISDB-T partial reception (only ISDB standard)
// @isdbt_sb_mode:	ISDB-T Sound Broadcast (SB) mode (only ISDB standard)
// @isdbt_sb_subchannel:	ISDB-T SB subchannel (only ISDB standard)
// @isdbt_sb_segment_idx:	ISDB-T SB segment index (only ISDB standard)
// @isdbt_sb_segment_count:	ISDB-T SB segment count (only ISDB standard)
// @isdbt_layer_enabled:	ISDB Layer enabled (only ISDB standard)
// @layer:		ISDB per-layer data (only ISDB standard)
// @layer.segment_count: Segment Count;
// @layer.fec:		per layer code rate;
// @layer.modulation:	per layer modulation;
// @layer.interleaving:	 per layer interleaving.
// @stream_id:		If different than zero, enable substream filtering, if
// hardware supports (DVB-S2 and DVB-T2).
// @scrambling_sequence_index:	Carries the index of the DVB-S2 physical layer
// scrambling sequence.
// @atscmh_fic_ver:	Version number of the FIC (Fast Information Channel)
// signaling data (only ATSC-M/H)
// @atscmh_parade_id:	Parade identification number (only ATSC-M/H)
// @atscmh_nog:		Number of MH groups per MH subframe for a designated
// parade (only ATSC-M/H)
// @atscmh_tnog:	Total number of MH groups including all MH groups
// belonging to all MH parades in one MH subframe
// (only ATSC-M/H)
// @atscmh_sgn:		Start group number (only ATSC-M/H)
// @atscmh_prc:		Parade repetition cycle (only ATSC-M/H)
// @atscmh_rs_frame_mode:	Reed Solomon (RS) frame mode (only ATSC-M/H)
// @atscmh_rs_frame_ensemble:	RS frame ensemble (only ATSC-M/H)
// @atscmh_rs_code_mode_pri:	RS code mode pri (only ATSC-M/H)
// @atscmh_rs_code_mode_sec:	RS code mode sec (only ATSC-M/H)
// @atscmh_sccc_block_mode:	Series Concatenated Convolutional Code (SCCC)
// Block Mode (only ATSC-M/H)
// @atscmh_sccc_code_mode_a:	SCCC code mode A (only ATSC-M/H)
// @atscmh_sccc_code_mode_b:	SCCC code mode B (only ATSC-M/H)
// @atscmh_sccc_code_mode_c:	SCCC code mode C (only ATSC-M/H)
// @atscmh_sccc_code_mode_d:	SCCC code mode D (only ATSC-M/H)
// @lna:		Power ON/OFF/AUTO the Linear Now-noise Amplifier (LNA)
// @strength:		DVBv5 API statistics: Signal Strength
// @cnr:		DVBv5 API statistics: Signal to Noise ratio of the
// (main) carrier
// @pre_bit_error:	DVBv5 API statistics: pre-Viterbi bit error count
// @pre_bit_count:	DVBv5 API statistics: pre-Viterbi bit count
// @post_bit_error:	DVBv5 API statistics: post-Viterbi bit error count
// @post_bit_count:	DVBv5 API statistics: post-Viterbi bit count
// @block_error:	DVBv5 API statistics: block error count
// @block_count:	DVBv5 API statistics: block count
//
// NOTE: derivated statistics like Uncorrected Error blocks (UCE) are
// calculated on userspace.
//
// Only a subset of the properties are needed for a given delivery system.
// For more info, consult the media_api.html with the documentation of the
// Userspace API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dtv_frontend_properties {
    pub frequency: u32,
    pub modulation: fe_modulation,
    pub voltage: fe_sec_voltage,
    pub sectone: fe_sec_tone_mode,
    pub inversion: fe_spectral_inversion,
    pub fec_inner: fe_code_rate,
    pub transmission_mode: fe_transmit_mode,
    pub /: *mut *mut u32 bandwidth_hz; / 0 = AUTO,
    pub guard_interval: fe_guard_interval,
    pub hierarchy: fe_hierarchy,
    pub symbol_rate: u32,
    pub code_rate_HP: fe_code_rate,
    pub code_rate_LP: fe_code_rate,
    pub pilot: fe_pilot,
    pub rolloff: fe_rolloff,
    pub delivery_system: fe_delivery_system,
    pub interleaving: fe_interleaving,
// ISDB-T specifics
    pub isdbt_partial_reception: u8,
    pub isdbt_sb_mode: u8,
    pub isdbt_sb_subchannel: u8,
    pub isdbt_sb_segment_idx: u32,
    pub isdbt_sb_segment_count: u32,
    pub isdbt_layer_enabled: u8,
    pub segment_count: u8,
    pub fec: fe_code_rate,
    pub modulation: fe_modulation,
    pub interleaving: u8,
    pub layer: [}; 3],
// Multistream specifics
    pub stream_id: u32,
// Physical Layer Scrambling specifics
    pub scrambling_sequence_index: u32,
// ATSC-MH specifics
    pub atscmh_fic_ver: u8,
    pub atscmh_parade_id: u8,
    pub atscmh_nog: u8,
    pub atscmh_tnog: u8,
    pub atscmh_sgn: u8,
    pub atscmh_prc: u8,
    pub atscmh_rs_frame_mode: u8,
    pub atscmh_rs_frame_ensemble: u8,
    pub atscmh_rs_code_mode_pri: u8,
    pub atscmh_rs_code_mode_sec: u8,
    pub atscmh_sccc_block_mode: u8,
    pub atscmh_sccc_code_mode_a: u8,
    pub atscmh_sccc_code_mode_b: u8,
    pub atscmh_sccc_code_mode_c: u8,
    pub atscmh_sccc_code_mode_d: u8,
    pub lna: u32,
// statistics data
    pub strength: dtv_fe_stats,
    pub cnr: dtv_fe_stats,
    pub pre_bit_error: dtv_fe_stats,
    pub pre_bit_count: dtv_fe_stats,
    pub post_bit_error: dtv_fe_stats,
    pub post_bit_count: dtv_fe_stats,
    pub block_error: dtv_fe_stats,
    pub block_count: dtv_fe_stats,
}

pub const DVB_FE_NO_EXIT: c_int = 0;
pub const DVB_FE_NORMAL_EXIT: c_int = 1;
pub const DVB_FE_DEVICE_REMOVED: c_int = 2;
pub const DVB_FE_DEVICE_RESUME: c_int = 3;
//
// struct dvb_frontend - Frontend structure to be used on drivers.
//
// @refcount:		refcount to keep track of &struct dvb_frontend
// references
// @ops:		embedded &struct dvb_frontend_ops
// @dvb:		pointer to &struct dvb_adapter
// @demodulator_priv:	demod private data
// @tuner_priv:		tuner private data
// @frontend_priv:	frontend private data
// @sec_priv:		SEC private data
// @analog_demod_priv:	Analog demod private data
// @dtv_property_cache:	embedded &struct dtv_frontend_properties
// @callback:		callback function used on some drivers to call
// either the tuner or the demodulator.
// @id:			Frontend ID
// @exit:		Used to inform the DVB core that the frontend
// thread should exit (usually, means that the hardware
// got disconnected.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_frontend {
    pub refcount: kref,
    pub ops: dvb_frontend_ops,
    pub dvb: *mut dvb_adapter,
    pub demodulator_priv: *mut c_void,
    pub tuner_priv: *mut c_void,
    pub frontend_priv: *mut c_void,
    pub sec_priv: *mut c_void,
    pub analog_demod_priv: *mut c_void,
    pub dtv_property_cache: dtv_frontend_properties,
pub const DVB_FRONTEND_COMPONENT_TUNER: c_int = 0;
pub const DVB_FRONTEND_COMPONENT_DEMOD: c_int = 1;
    pub arg): *mut *mut *mut int (callback)(void adapter_priv, int component, int cmd, int,
    pub id: c_int,
    pub exit: c_uint,
}

//
// dvb_register_frontend() - Registers a DVB frontend at the adapter
//
// @dvb: pointer to &struct dvb_adapter
// @fe: pointer to &struct dvb_frontend
//
// Allocate and initialize the private data needed by the frontend core to
// manage the frontend and calls dvb_register_device() to register a new
// frontend. It also cleans the property cache that stores the frontend
// parameters and selects the first available delivery system.
//
// dvb_unregister_frontend() - Unregisters a DVB frontend
//
// @fe: pointer to &struct dvb_frontend
//
// Stops the frontend kthread, calls dvb_unregister_device() and frees the
// private frontend data allocated by dvb_register_frontend().
//
// NOTE: This function doesn't frees the memory allocated by the demod,
// by the SEC driver and by the tuner. In order to free it, an explicit call to
// dvb_frontend_detach() is needed, after calling this function.
//
extern "C" {
    pub fn dvb_unregister_frontend(fe: *mut dvb_frontend) -> c_int;
}
//
// dvb_frontend_detach() - Detaches and frees frontend specific data
//
// @fe: pointer to &struct dvb_frontend
//
// This function should be called after dvb_unregister_frontend(). It
// calls the SEC, tuner and demod release functions:
// &dvb_frontend_ops.release_sec, &dvb_frontend_ops.tuner_ops.release,
// &dvb_frontend_ops.analog_ops.release and &dvb_frontend_ops.release.
//
// If the driver is compiled with %CONFIG_MEDIA_ATTACH, it also decreases
// the module reference count, needed to allow userspace to remove the
// previously used DVB frontend modules.
//
extern "C" {
    pub fn dvb_frontend_detach(fe: *mut dvb_frontend);
}
//
// dvb_frontend_suspend() - Suspends a Digital TV frontend
//
// @fe: pointer to &struct dvb_frontend
//
// This function prepares a Digital TV frontend to suspend.
//
// In order to prepare the tuner to suspend, if
// &dvb_frontend_ops.tuner_ops.suspend\(\) is available, it calls it. Otherwise,
// it will call &dvb_frontend_ops.tuner_ops.sleep\(\), if available.
//
// It will also call &dvb_frontend_ops.suspend\(\) to put the demod to suspend,
// if available. Otherwise it will call &dvb_frontend_ops.sleep\(\).
//
// The drivers should also call dvb_frontend_suspend\(\) as part of their
// handler for the &device_driver.suspend\(\).
//
extern "C" {
    pub fn dvb_frontend_suspend(fe: *mut dvb_frontend) -> c_int;
}
//
// dvb_frontend_resume() - Resumes a Digital TV frontend
//
// @fe: pointer to &struct dvb_frontend
//
// This function resumes the usual operation of the tuner after resume.
//
// In order to resume the frontend, it calls the demod
// &dvb_frontend_ops.resume\(\) if available. Otherwise it calls demod
// &dvb_frontend_ops.init\(\).
//
// If &dvb_frontend_ops.tuner_ops.resume\(\) is available, It, it calls it.
// Otherwise,t will call &dvb_frontend_ops.tuner_ops.init\(\), if available.
//
// Once tuner and demods are resumed, it will enforce that the SEC voltage and
// tone are restored to their previous values and wake up the frontend's
// kthread in order to retune the frontend.
//
// The drivers should also call dvb_frontend_resume() as part of their
// handler for the &device_driver.resume\(\).
//
extern "C" {
    pub fn dvb_frontend_resume(fe: *mut dvb_frontend) -> c_int;
}
//
// dvb_frontend_reinitialise() - forces a reinitialisation at the frontend
//
// @fe: pointer to &struct dvb_frontend
//
// Calls &dvb_frontend_ops.init\(\) and &dvb_frontend_ops.tuner_ops.init\(\),
// and resets SEC tone and voltage (for Satellite systems).
//
// NOTE: Currently, this function is used only by one driver (budget-av).
// It seems to be due to address some special issue with that specific
// frontend.
//
extern "C" {
    pub fn dvb_frontend_reinitialise(fe: *mut dvb_frontend);
}
//
// dvb_frontend_sleep_until() - Sleep for the amount of time given by
// add_usec parameter
//
// @waketime: pointer to &struct ktime_t
// @add_usec: time to sleep, in microseconds
//
// This function is used to measure the time required for the
// FE_DISHNETWORK_SEND_LEGACY_CMD() ioctl to work. It needs to be as precise
// as possible, as it affects the detection of the dish tone command at the
// satellite subsystem.
//
// Its used internally by the DVB frontend core, in order to emulate
// FE_DISHNETWORK_SEND_LEGACY_CMD() using the &dvb_frontend_ops.set_voltage\(\)
// callback.
//
// NOTE: it should not be used at the drivers, as the emulation for the
// legacy callback is provided by the Kernel. The only situation where this
// should be at the drivers is when there are some bugs at the hardware that
// would prevent the core emulation to work. On such cases, the driver would
// be writing a &dvb_frontend_ops.dishnetwork_send_legacy_command\(\) and
// calling this function directly.
//
extern "C" {
    pub fn dvb_frontend_sleep_until(waketime: *mut ktime_t, add_usec: u32);
}
