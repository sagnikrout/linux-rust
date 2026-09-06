//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drx39xyj/drx_driver.h
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
// Redistributions of source code must retain the above copyright notice,
// Redistributions in binary form must reproduce the above copyright notice,
// Neither the name of Trident Microsystems nor Hauppauge Computer Works
//

//
// This structure contains the I2C address, the device ID and a user_data pointer.
// The user_data pointer can be used for application specific purposes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_addr {
    pub /: *mut *mut u16 i2c_addr; / The I2C address of the device.,
    pub /: *mut *mut u16 i2c_dev_id; / The device identifier.,
    pub /: *mut *mut *mut void user_data; / User data pointer,
}

//
// \def IS_I2C_10BIT( addr )
// \brief Determine if I2C address 'addr' is a 10 bits address or not.
// \param addr The I2C address.
// \return int.
// \retval 0 if address is not a 10 bits I2C address.
// \retval 1 if address is a 10 bits I2C address.
//

// ------------------------------------------------------------------------------
//
// \fn drxbsp_i2c_init()
// \brief Initialize I2C communication module.
// \return int Return status.
// \retval 0 Initialization successful.
// \retval -EIO Initialization failed.
//
extern "C" {
    pub fn drxbsp_i2c_init() -> c_int;
}
//
// \fn drxbsp_i2c_term()
// \brief Terminate I2C communication module.
// \return int Return status.
// \retval 0 Termination successful.
// \retval -EIO Termination failed.
//
extern "C" {
    pub fn drxbsp_i2c_term() -> c_int;
}
//
// \fn int drxbsp_i2c_write_read( struct i2c_device_addr *w_dev_addr,
// u16 w_count,
// u8 * wData,
// struct i2c_device_addr *r_dev_addr,
// u16 r_count,
// u8 * r_data)
// \brief Read and/or write count bytes from I2C bus, store them in data[].
// \param w_dev_addr The device i2c address and the device ID to write to
// \param w_count   The number of bytes to write
// \param wData    The array to write the data to
// \param r_dev_addr The device i2c address and the device ID to read from
// \param r_count   The number of bytes to read
// \param r_data    The array to read the data from
// \return int Return status.
// \retval 0 Success.
// \retval -EIO Failure.
// \retval -EINVAL Parameter 'wcount' is not zero but parameter
// 'wdata' contains NULL.
// Idem for 'rcount' and 'rdata'.
// Both w_dev_addr and r_dev_addr are NULL.
//
// This function must implement an atomic write and/or read action on the I2C bus
// No other process may use the I2C bus when this function is executing.
// The critical section of this function runs from and including the I2C
// write, up to and including the I2C read action.
//
// The device ID can be useful if several devices share an I2C address.
// It can be used to control a "switch" on the I2C bus to the correct device.
//
// \fn drxbsp_i2c_error_text()
// \brief Returns a human readable error.
// Counter part of numerical drx_i2c_error_g.
//
// \return char* Pointer to human readable error text.
//
// \var drx_i2c_error_g;
// \brief I2C specific error codes, platform dependent.
//
pub const TUNER_MODE_SUB0: c_uint = 0x0001	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB1: c_uint = 0x0002	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB2: c_uint = 0x0004	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB3: c_uint = 0x0008	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB4: c_uint = 0x0010	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB5: c_uint = 0x0020	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB6: c_uint = 0x0040	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_SUB7: c_uint = 0x0080	/* for sub-mode (e.g. RF-AGC setting) */;
pub const TUNER_MODE_DIGITAL: c_uint = 0x0100	/* for digital channel (e.g. DVB-T)   */;
pub const TUNER_MODE_ANALOG: c_uint = 0x0200	/* for analog channel  (e.g. PAL)     */;
pub const TUNER_MODE_SWITCH: c_uint = 0x0400	/* during channel switch & scanning   */;
pub const TUNER_MODE_LOCK: c_uint = 0x0800	/* after tuner has locked             */;
pub const TUNER_MODE_6MHZ: c_uint = 0x1000	/* for 6MHz bandwidth channels        */;
pub const TUNER_MODE_7MHZ: c_uint = 0x2000	/* for 7MHz bandwidth channels        */;
pub const TUNER_MODE_8MHZ: c_uint = 0x4000	/* for 8MHz bandwidth channels        */;
pub const TUNER_MODE_SUB_MAX: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tuner_lock_status {
    TUNER_LOCKED,
    TUNER_NOT_LOCKED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_common {
    pub /: *mut *mut *mut char name; / Tuner brand & type name,
    pub /: *mut *mut s32 min_freq_rf; / Lowest RF input frequency, in kHz,
    pub /: *mut *mut s32 max_freq_rf; / Highest RF input frequency, in kHz,
    pub /: *mut *mut u8 sub_mode; / Index to sub-mode in use,
    pub /: *mut *mut *mut *mut *mut char sub_mode_descriptions; / Pointer to description of sub-modes,
    pub /: *mut *mut u8 sub_modes; / Number of available sub-modes,
// The following fields will be either 0, NULL or false and do not need
    pub /: *mut *mut *mut void self_check; / gives proof of initialization,
    pub /: *mut *mut bool programmed; / only valid if self_check is OK,
    pub /: *mut *mut s32 r_ffrequency; / only valid if programmed,
    pub /: *mut *mut s32 i_ffrequency; / only valid if programmed,
    pub /: *mut *mut *mut void my_user_data; / pointer to associated demod instance,
    pub /: *mut *mut u16 my_capabilities; / value for storing application flags,
}

extern "C" {
    pub fn int(tuner: *mut *mut tuner_open_func_t) (struct tuner_instance) -> typedef;
}
extern "C" {
    pub fn int(tuner: *mut *mut tuner_close_func_t) (struct tuner_instance) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_ops {
    pub open_func: tuner_open_func_t,
    pub close_func: tuner_close_func_t,
    pub set_frequency_func: tuner_set_frequency_func_t,
    pub get_frequency_func: tuner_get_frequency_func_t,
    pub lock_status_func: tuner_lock_status_func_t,
    pub i2c_write_read_func: tune_ri2c_write_read_func_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tuner_instance {
    pub my_i2c_dev_addr: i2c_device_addr,
    pub my_common_attr: *mut tuner_common,
    pub my_ext_attr: *mut c_void,
    pub my_funct: *mut tuner_ops,
}

//
// This section configures the DRX Data Access Protocols (DAPs).
//
// \def DRXDAP_SINGLE_MASTER
// \brief Enable I2C single or I2C multimaster mode on host.
//
// Set to 1 to enable single master mode
// Set to 0 to enable multi master mode
//
// The actual DAP implementation may be restricted to only one of the modes.
// A compiler warning or error will be generated if the DAP implementation
// overrides or cannot handle the mode defined below.
//

pub const DRXDAP_SINGLE_MASTER: c_int = 1;

//
// \def DRXDAP_MAX_WCHUNKSIZE
// \brief Defines maximum chunksize of an i2c write action by host.
//
// This indicates the maximum size of data the I2C device driver is able to
// write at a time. This includes I2C device address and register addressing.
//
// This maximum size may be restricted by the actual DAP implementation.
// A compiler warning or error will be generated if the DAP implementation
// overrides or cannot handle the chunksize defined below.
//
// Beware that the DAP uses  DRXDAP_MAX_WCHUNKSIZE to create a temporary data
// buffer. Do not undefine or choose too large, unless your system is able to
// handle a stack buffer of that size.
//

pub const DRXDAP_MAX_WCHUNKSIZE: c_int = 60;

//
// \def DRXDAP_MAX_RCHUNKSIZE
// \brief Defines maximum chunksize of an i2c read action by host.
//
// This indicates the maximum size of data the I2C device driver is able to read
// at a time. Minimum value is 2. Also, the read chunk size must be even.
//
// This maximum size may be restricted by the actual DAP implementation.
// A compiler warning or error will be generated if the DAP implementation
// overrides or cannot handle the chunksize defined below.
//

pub const DRXDAP_MAX_RCHUNKSIZE: c_int = 60;

//
// This section describes drxdriver defines.
//
// \def DRX_UNKNOWN
// \brief Generic UNKNOWN value for DRX enumerated types.
//
// Used to indicate that the parameter value is unknown or not yet initialized.
//

//
// \def DRX_AUTO
// \brief Generic AUTO value for DRX enumerated types.
//
// Used to instruct the driver to automatically determine the value of the
// parameter.
//

//
// This section describes flag definitions for the device capbilities.
//
// \brief LNA capability flag
//
// Device has a Low Noise Amplifier
//

//
// \brief OOB-RX capability flag
//
// Device has OOB-RX
//

//
// \brief ATV capability flag
//
// Device has ATV
//

//
// \brief DVB-T capability flag
//
// Device has DVB-T
//

//
// \brief  ITU-B capability flag
//
// Device has ITU-B
//

//
// \brief  Audio capability flag
//
// Device has Audio
//

//
// \brief  SAW switch capability flag
//
// Device has SAW switch
//

//
// \brief  GPIO1 capability flag
//
// Device has GPIO1
//

//
// \brief  GPIO2 capability flag
//
// Device has GPIO2
//

//
// \brief  IRQN capability flag
//
// Device has IRQN
//

//
// \brief  8VSB capability flag
//
// Device has 8VSB
//

//
// \brief  SMA-TX capability flag
//
// Device has SMATX
//

//
// \brief  SMA-RX capability flag
//
// Device has SMARX
//

//
// \brief  ITU-A/C capability flag
//
// Device has ITU-A/C
//

// -------------------------------------------------------------------------
// Macros to stringify the version number

//
// \brief Macro to create byte array elements from 16 bit integers.
// This macro is used to create byte arrays for block writes.
// Block writes speed up I2C traffic between host and demod.
// The macro takes care of the required byte order in a 16 bits word.
// x->lowbyte(x), highbyte(x)
//

//
// \brief Macro to convert 16 bit register value to a s32
//

// -------------------------------------------------------------------------
//
// \enum enum drx_standard
// \brief Modulation standards.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_standard {
    DRX_STANDARD_DVBT = 0, /*< Terrestrial DVB-T.               */
    DRX_STANDARD_8VSB,     /*< Terrestrial 8VSB.                */
    DRX_STANDARD_NTSC,     /*< Terrestrial\Cable analog NTSC.   */
    DRX_STANDARD_PAL_SECAM_BG,
// < Terrestrial analog PAL/SECAM B/G
    DRX_STANDARD_PAL_SECAM_DK,
// < Terrestrial analog PAL/SECAM D/K
    DRX_STANDARD_PAL_SECAM_I,
// < Terrestrial analog PAL/SECAM I
    DRX_STANDARD_PAL_SECAM_L,
// < Terrestrial analog PAL/SECAM L
    with negative modulation        */
    DRX_STANDARD_PAL_SECAM_LP,
// < Terrestrial analog PAL/SECAM L
    with positive modulation        */
    DRX_STANDARD_ITU_A,    /*< Cable ITU ANNEX A.               */
    DRX_STANDARD_ITU_B,    /*< Cable ITU ANNEX B.               */
    DRX_STANDARD_ITU_C,    /*< Cable ITU ANNEX C.               */
    DRX_STANDARD_ITU_D,    /*< Cable ITU ANNEX D.               */
    DRX_STANDARD_FM,       /*< Terrestrial\Cable FM radio       */
    DRX_STANDARD_DTMB,     /*< Terrestrial DTMB standard (China)*/
    DRX_STANDARD_UNKNOWN = DRX_UNKNOWN,
// < Standard unknown.
    DRX_STANDARD_AUTO = DRX_AUTO
// < Autodetect standard.
}

//
// \enum enum drx_standard
// \brief Modulation sub-standards.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_substandard {
    DRX_SUBSTANDARD_MAIN = 0, /*< Main subvariant of standard   */
    DRX_SUBSTANDARD_ATV_BG_SCANDINAVIA,
    DRX_SUBSTANDARD_ATV_DK_POLAND,
    DRX_SUBSTANDARD_ATV_DK_CHINA,
    DRX_SUBSTANDARD_UNKNOWN = DRX_UNKNOWN,
// < Sub-standard unknown.
    DRX_SUBSTANDARD_AUTO = DRX_AUTO
// < Auto (default) sub-standard
}

//
// \enum enum drx_bandwidth
// \brief Channel bandwidth or channel spacing.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_bandwidth {
    DRX_BANDWIDTH_8MHZ = 0,	 /*< Bandwidth 8 MHz.   */
    DRX_BANDWIDTH_7MHZ,	 /*< Bandwidth 7 MHz.   */
    DRX_BANDWIDTH_6MHZ,	 /*< Bandwidth 6 MHz.   */
    DRX_BANDWIDTH_UNKNOWN = DRX_UNKNOWN,
// < Bandwidth unknown.
    DRX_BANDWIDTH_AUTO = DRX_AUTO
// < Auto Set Bandwidth
}

//
// \enum enum drx_mirror
// \brief Indicate if channel spectrum is mirrored or not.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_mirror {
    DRX_MIRROR_NO = 0,   /*< Spectrum is not mirrored.           */
    DRX_MIRROR_YES,	     /*< Spectrum is mirrored.               */
    DRX_MIRROR_UNKNOWN = DRX_UNKNOWN,
// < Unknown if spectrum is mirrored.
    DRX_MIRROR_AUTO = DRX_AUTO
// < Autodetect if spectrum is mirrored.
}

//
// \enum enum drx_modulation
// \brief Constellation type of the channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_modulation {
    DRX_CONSTELLATION_BPSK = 0,  /*< Modulation is BPSK.       */
    DRX_CONSTELLATION_QPSK,	     /*< Constellation is QPSK.    */
    DRX_CONSTELLATION_PSK8,	     /*< Constellation is PSK8.    */
    DRX_CONSTELLATION_QAM16,     /*< Constellation is QAM16.   */
    DRX_CONSTELLATION_QAM32,     /*< Constellation is QAM32.   */
    DRX_CONSTELLATION_QAM64,     /*< Constellation is QAM64.   */
    DRX_CONSTELLATION_QAM128,    /*< Constellation is QAM128.  */
    DRX_CONSTELLATION_QAM256,    /*< Constellation is QAM256.  */
    DRX_CONSTELLATION_QAM512,    /*< Constellation is QAM512.  */
    DRX_CONSTELLATION_QAM1024,   /*< Constellation is QAM1024. */
    DRX_CONSTELLATION_QPSK_NR,   /*< Constellation is QPSK_NR  */
    DRX_CONSTELLATION_UNKNOWN = DRX_UNKNOWN,
// < Constellation unknown.
    DRX_CONSTELLATION_AUTO = DRX_AUTO
// < Autodetect constellation.
}

//
// \enum enum drx_hierarchy
// \brief Hierarchy of the channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_hierarchy {
    DRX_HIERARCHY_NONE = 0,	/*< None hierarchical channel.     */
    DRX_HIERARCHY_ALPHA1,	/*< Hierarchical channel, alpha=1. */
    DRX_HIERARCHY_ALPHA2,	/*< Hierarchical channel, alpha=2. */
    DRX_HIERARCHY_ALPHA4,	/*< Hierarchical channel, alpha=4. */
    DRX_HIERARCHY_UNKNOWN = DRX_UNKNOWN,
// < Hierarchy unknown.
    DRX_HIERARCHY_AUTO = DRX_AUTO
// < Autodetect hierarchy.
}

//
// \enum enum drx_priority
// \brief Channel priority in case of hierarchical transmission.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_priority {
    DRX_PRIORITY_LOW = 0,  /*< Low priority channel.  */
    DRX_PRIORITY_HIGH,     /*< High priority channel. */
    DRX_PRIORITY_UNKNOWN = DRX_UNKNOWN
// < Priority unknown.
}

//
// \enum enum drx_coderate
// \brief Channel priority in case of hierarchical transmission.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_coderate {
    DRX_CODERATE_1DIV2 = 0,	/*< Code rate 1/2nd.      */
    DRX_CODERATE_2DIV3,	/*< Code rate 2/3nd.      */
    DRX_CODERATE_3DIV4,	/*< Code rate 3/4nd.      */
    DRX_CODERATE_5DIV6,	/*< Code rate 5/6nd.      */
    DRX_CODERATE_7DIV8,	/*< Code rate 7/8nd.      */
    DRX_CODERATE_UNKNOWN = DRX_UNKNOWN,
// < Code rate unknown.
    DRX_CODERATE_AUTO = DRX_AUTO
// < Autodetect code rate.
}

//
// \enum enum drx_guard
// \brief Guard interval of a channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_guard {
    DRX_GUARD_1DIV32 = 0, /*< Guard interval 1/32nd.     */
    DRX_GUARD_1DIV16,     /*< Guard interval 1/16th.     */
    DRX_GUARD_1DIV8,      /*< Guard interval 1/8th.      */
    DRX_GUARD_1DIV4,      /*< Guard interval 1/4th.      */
    DRX_GUARD_UNKNOWN = DRX_UNKNOWN,
// < Guard interval unknown.
    DRX_GUARD_AUTO = DRX_AUTO
// < Autodetect guard interval.
}

//
// \enum enum drx_fft_mode
// \brief FFT mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_fft_mode {
    DRX_FFTMODE_2K = 0,    /*< 2K FFT mode.         */
    DRX_FFTMODE_4K,	       /*< 4K FFT mode.         */
    DRX_FFTMODE_8K,	       /*< 8K FFT mode.         */
    DRX_FFTMODE_UNKNOWN = DRX_UNKNOWN,
// < FFT mode unknown.
    DRX_FFTMODE_AUTO = DRX_AUTO
// < Autodetect FFT mode.
}

//
// \enum enum drx_classification
// \brief Channel classification.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_classification {
    DRX_CLASSIFICATION_GAUSS = 0, /*< Gaussion noise.            */
    DRX_CLASSIFICATION_HVY_GAUSS, /*< Heavy Gaussion noise.      */
    DRX_CLASSIFICATION_COCHANNEL, /*< Co-channel.                */
    DRX_CLASSIFICATION_STATIC,    /*< Static echo.               */
    DRX_CLASSIFICATION_MOVING,    /*< Moving echo.               */
    DRX_CLASSIFICATION_ZERODB,    /*< Zero dB echo.              */
    DRX_CLASSIFICATION_UNKNOWN = DRX_UNKNOWN,
// < Unknown classification
    DRX_CLASSIFICATION_AUTO = DRX_AUTO
// < Autodetect classification.
}

//
// /enum enum drx_interleave_mode
// /brief Interleave modes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_interleave_mode {
    DRX_INTERLEAVEMODE_I128_J1 = 0,
    DRX_INTERLEAVEMODE_I128_J1_V2,
    DRX_INTERLEAVEMODE_I128_J2,
    DRX_INTERLEAVEMODE_I64_J2,
    DRX_INTERLEAVEMODE_I128_J3,
    DRX_INTERLEAVEMODE_I32_J4,
    DRX_INTERLEAVEMODE_I128_J4,
    DRX_INTERLEAVEMODE_I16_J8,
    DRX_INTERLEAVEMODE_I128_J5,
    DRX_INTERLEAVEMODE_I8_J16,
    DRX_INTERLEAVEMODE_I128_J6,
    DRX_INTERLEAVEMODE_RESERVED_11,
    DRX_INTERLEAVEMODE_I128_J7,
    DRX_INTERLEAVEMODE_RESERVED_13,
    DRX_INTERLEAVEMODE_I128_J8,
    DRX_INTERLEAVEMODE_RESERVED_15,
    DRX_INTERLEAVEMODE_I12_J17,
    DRX_INTERLEAVEMODE_I5_J4,
    DRX_INTERLEAVEMODE_B52_M240,
    DRX_INTERLEAVEMODE_B52_M720,
    DRX_INTERLEAVEMODE_B52_M48,
    DRX_INTERLEAVEMODE_B52_M0,
    DRX_INTERLEAVEMODE_UNKNOWN = DRX_UNKNOWN,
// < Unknown interleave mode
    DRX_INTERLEAVEMODE_AUTO = DRX_AUTO
// < Autodetect interleave mode
}

//
// \enum enum drx_carrier_mode
// \brief Channel Carrier Mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_carrier_mode {
    DRX_CARRIER_MULTI = 0,		/*< Multi carrier mode       */
    DRX_CARRIER_SINGLE,		/*< Single carrier mode      */
    DRX_CARRIER_UNKNOWN = DRX_UNKNOWN,
// < Carrier mode unknown.
    DRX_CARRIER_AUTO = DRX_AUTO	/*< Autodetect carrier mode  */
}

//
// \enum enum drx_frame_mode
// \brief Channel Frame Mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_frame_mode {
    DRX_FRAMEMODE_420 = 0,	 /*< 420 with variable PN  */
    DRX_FRAMEMODE_595,	 /*< 595                   */
    DRX_FRAMEMODE_945,	 /*< 945 with variable PN  */
    DRX_FRAMEMODE_420_FIXED_PN,
// < 420 with fixed PN
    DRX_FRAMEMODE_945_FIXED_PN,
// < 945 with fixed PN
    DRX_FRAMEMODE_UNKNOWN = DRX_UNKNOWN,
// < Frame mode unknown.
    DRX_FRAMEMODE_AUTO = DRX_AUTO
// < Autodetect frame mode
}

//
// \enum enum drx_tps_frame
// \brief Frame number in current super-frame.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_tps_frame {
    DRX_TPS_FRAME1 = 0,	  /*< TPS frame 1.       */
    DRX_TPS_FRAME2,		  /*< TPS frame 2.       */
    DRX_TPS_FRAME3,		  /*< TPS frame 3.       */
    DRX_TPS_FRAME4,		  /*< TPS frame 4.       */
    DRX_TPS_FRAME_UNKNOWN = DRX_UNKNOWN
// < TPS frame unknown.
}

//
// \enum enum drx_ldpc
// \brief TPS LDPC .
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_ldpc {
    DRX_LDPC_0_4 = 0,	  /*< LDPC 0.4           */
    DRX_LDPC_0_6,		  /*< LDPC 0.6           */
    DRX_LDPC_0_8,		  /*< LDPC 0.8           */
    DRX_LDPC_UNKNOWN = DRX_UNKNOWN,
// < LDPC unknown.
    DRX_LDPC_AUTO = DRX_AUTO  /*< Autodetect LDPC    */
}

//
// \enum enum drx_pilot_mode
// \brief Pilot modes in DTMB.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_pilot_mode {
    DRX_PILOT_ON = 0,	  /*< Pilot On             */
    DRX_PILOT_OFF,		  /*< Pilot Off            */
    DRX_PILOT_UNKNOWN = DRX_UNKNOWN,
// < Pilot unknown.
    DRX_PILOT_AUTO = DRX_AUTO /*< Autodetect Pilot     */
}

//
// enum drxu_code_action - indicate if firmware has to be uploaded or verified.
// @UCODE_UPLOAD:	Upload the microcode image to device
// @UCODE_VERIFY:	Compare microcode image with code on device
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxu_code_action {
    UCODE_UPLOAD,
    UCODE_VERIFY
}

//
// \enum enum drx_lock_status * \brief Used to reflect current lock status of demodulator.
//
// The generic lock states have device dependent semantics.
// < Device will never lock on this signal
// < Device has no lock at all
// < Generic lock state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_lock_status {
    DRX_NEVER_LOCK = 0,
    DRX_NOT_LOCKED,
    DRX_LOCK_STATE_1,
    DRX_LOCK_STATE_2,
    DRX_LOCK_STATE_3,
    DRX_LOCK_STATE_4,
    DRX_LOCK_STATE_5,
    DRX_LOCK_STATE_6,
    DRX_LOCK_STATE_7,
    DRX_LOCK_STATE_8,
    DRX_LOCK_STATE_9,
    DRX_LOCKED
}

//
// \enum enum drx_uio* \brief Used to address a User IO (UIO).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_uio {
    DRX_UIO1,
    DRX_UIO2,
    DRX_UIO3,
    DRX_UIO4,
    DRX_UIO5,
    DRX_UIO6,
    DRX_UIO7,
    DRX_UIO8,
    DRX_UIO9,
    DRX_UIO10,
    DRX_UIO11,
    DRX_UIO12,
    DRX_UIO13,
    DRX_UIO14,
    DRX_UIO15,
    DRX_UIO16,
    DRX_UIO17,
    DRX_UIO18,
    DRX_UIO19,
    DRX_UIO20,
    DRX_UIO21,
    DRX_UIO22,
    DRX_UIO23,
    DRX_UIO24,
    DRX_UIO25,
    DRX_UIO26,
    DRX_UIO27,
    DRX_UIO28,
    DRX_UIO29,
    DRX_UIO30,
    DRX_UIO31,
    DRX_UIO32,
    DRX_UIO_MAX = DRX_UIO32
}

//
// \enum enum drxuio_mode * \brief Used to configure the modus oprandi of a UIO.
//
// DRX_UIO_MODE_FIRMWARE is an old uio mode.
// It is replaced by the modes DRX_UIO_MODE_FIRMWARE0 .. DRX_UIO_MODE_FIRMWARE9.
// To be backward compatible DRX_UIO_MODE_FIRMWARE is equivalent to
// DRX_UIO_MODE_FIRMWARE0.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxuio_mode {
    DRX_UIO_MODE_DISABLE = 0x01,
// < not used, pin is configured as input
    DRX_UIO_MODE_READWRITE = 0x02,
// < used for read/write by application
    DRX_UIO_MODE_FIRMWARE = 0x04,
// < controlled by firmware, function 0
    DRX_UIO_MODE_FIRMWARE0 = DRX_UIO_MODE_FIRMWARE,
// < same as above
    DRX_UIO_MODE_FIRMWARE1 = 0x08,
// < controlled by firmware, function 1
    DRX_UIO_MODE_FIRMWARE2 = 0x10,
// < controlled by firmware, function 2
    DRX_UIO_MODE_FIRMWARE3 = 0x20,
// < controlled by firmware, function 3
    DRX_UIO_MODE_FIRMWARE4 = 0x40,
// < controlled by firmware, function 4
    DRX_UIO_MODE_FIRMWARE5 = 0x80
// < controlled by firmware, function 5
}

//
// \enum enum drxoob_downstream_standard * \brief Used to select OOB standard.
//
// Based on ANSI 55-1 and 55-2
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxoob_downstream_standard {
    DRX_OOB_MODE_A = 0,
// < ANSI 55-1
    DRX_OOB_MODE_B_GRADE_A,
// < ANSI 55-2 A
    DRX_OOB_MODE_B_GRADE_B
// < ANSI 55-2 B
}

// -------------------------------------------------------------------------
// ============================================================================
// == CTRL CFG related data structures ========================================
// ============================================================================

pub const DRX_CFG_BASE: c_int = 0;

// ============================================================================
// == CTRL related data structures ============================================
// ============================================================================
//
// struct drxu_code_info	Parameters for microcode upload and verfiy.
//
// @mc_file:	microcode file name
//
// Used by DRX_CTRL_LOAD_UCODE and DRX_CTRL_VERIFY_UCODE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxu_code_info {
    pub mc_file: *mut c_char,
}

//
// \struct drx_mc_version_rec_t
// \brief Microcode version record
// Version numbers are stored in BCD format, as usual:
// o major number = bits 31-20 (first three nibbles of MSW)
// o minor number = bits 19-16 (fourth nibble of MSW)
// o patch number = bits 15-0  (remaining nibbles in LSW)
//
// The device type indicates for which the device is meant. It is based on the
// JTAG ID, using everything except the bond ID and the metal fix.
//
// Special values:
// - mc_dev_type == 0         => any device allowed
// - mc_base_version == 0.0.0 => full microcode (mc_version is the version)
// - mc_base_version != 0.0.0 => patch microcode, the base microcode version
// (mc_version is the version)
//
pub const AUX_VER_RECORD: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_mc_version_rec {
    pub /: *mut *mut u16 aux_type; / type of aux data - 0x8000 for version record,
    pub /: *mut *mut u32 mc_dev_type; / device type, based on JTAG ID,
    pub /: *mut *mut u32 mc_version; / version of microcode,
    pub /: *mut *mut u32 mc_base_version; / in case of patch: the original microcode version,
}

// ========================================
//
// \struct drx_filter_info_t
// \brief Parameters for loading filter coefficients
//
// Used by DRX_CTRL_LOAD_FILTER
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_filter_info {
    pub data_re: *mut u8,
// < pointer to coefficients for RE
    pub data_im: *mut u8,
// < pointer to coefficients for IM
    pub size_re: u16,
// < size of coefficients for RE
    pub size_im: u16,
// < size of coefficients for IM
}

// ========================================
//
// \struct struct drx_channel * \brief The set of parameters describing a single channel.
//
// Used by DRX_CTRL_SET_CHANNEL and DRX_CTRL_GET_CHANNEL.
// Only certain fields need to be used for a specific standard.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_channel {
    pub frequency: i32,
// < frequency in kHz
    pub bandwidth: drx_bandwidth,
// < bandwidth
    pub /: *mut *mut drx_mirror mirror; /< mirrored or not on RF,
    pub constellation: drx_modulation,
// < constellation
    pub hierarchy: drx_hierarchy,
// < hierarchy
    pub /: *mut *mut drx_priority priority; /< priority,
    pub /: *mut *mut drx_coderate coderate; /< coderate,
    pub /: *mut *mut drx_guard guard; /< guard interval,
    pub /: *mut *mut drx_fft_mode fftmode; /< fftmode,
    pub classification: drx_classification,
// < classification
    pub symbolrate: u32,
// < symbolrate in symbols/sec
    pub interleavemode: drx_interleave_mode,
// < interleaveMode QAM
    pub /: *mut *mut drx_ldpc ldpc; /< ldpc,
    pub /: *mut *mut drx_carrier_mode carrier; /< carrier,
    pub framemode: drx_frame_mode,
// < frame mode
    pub /: *mut *mut drx_pilot_mode pilot; /< pilot mode,
}

// ========================================
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_cfg_sqi_speed {
    DRX_SQI_SPEED_FAST = 0,
    DRX_SQI_SPEED_MEDIUM,
    DRX_SQI_SPEED_SLOW,
    DRX_SQI_SPEED_UNKNOWN = DRX_UNKNOWN
}

// ========================================
//
// \struct struct drx_complex * A complex number.
//
// Used by DRX_CTRL_CONSTEL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_complex {
    pub im: i16,
// < Imaginary part.
    pub re: i16,
// < Real part.
}

// ========================================
//
// \struct struct drx_frequency_plan * Array element of a frequency plan.
//
// Used by DRX_CTRL_SCAN_INIT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_frequency_plan {
    pub first: i32,
// < First centre frequency in this band
    pub last: i32,
// < Last centre frequency in this band
    pub step: i32,
// < Stepping frequency in this band
    pub bandwidth: drx_bandwidth,
// < Bandwidth within this frequency band
    pub ch_number: u16,
// < First channel number in this band, or first
    pub ch_names: *mut c_char,
// < Optional list of channel names in this
}

// ========================================
//
// \struct struct drx_scan_param * Parameters for channel scan.
//
// Used by DRX_CTRL_SCAN_INIT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_scan_param {
    pub frequency_plan: *mut drx_frequency_plan,
// < Frequency plan (array)
    pub /: *mut *mut u16 frequency_plan_size; /< Number of bands,
    pub /: *mut *mut u32 num_tries; /< Max channels tried,
    pub take: *mut *mut s32 skip; /< Minimum frequency step to,
    pub /: *mut *mut *mut void ext_params; /< Standard specific params,
}

// ========================================
//
// \brief Scan commands.
// Used by scanning algorithms.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_scan_command {
    DRX_SCAN_COMMAND_INIT = 0,/*< Initialize scanning */
    DRX_SCAN_COMMAND_NEXT,	  /*< Next scan           */
    DRX_SCAN_COMMAND_STOP	  /*< Stop scanning       */
}

// ========================================
//
// \brief Inner scan function prototype.
//
// ========================================
//
// \struct struct drxtps_info * TPS information, DVB-T specific.
//
// Used by DRX_CTRL_TPS_INFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxtps_info {
    pub /: *mut *mut drx_fft_mode fftmode; /< Fft mode,
    pub /: *mut *mut drx_guard guard; /< Guard interval,
    pub constellation: drx_modulation,
// < Constellation
    pub hierarchy: drx_hierarchy,
// < Hierarchy
    pub high_coderate: drx_coderate,
// < High code rate
    pub low_coderate: drx_coderate,
// < Low cod rate
    pub /: *mut *mut drx_tps_frame frame; /< Tps frame,
    pub /: *mut *mut u8 length; /< Length,
    pub /: *mut *mut u16 cell_id; /< Cell id,
}

// ========================================
//
// \brief Power mode of device.
//
// Used by DRX_CTRL_SET_POWER_MODE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_power_mode {
    DRX_POWER_UP = 0,
// < Generic         , Power Up Mode
    DRX_POWER_MODE_1,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_2,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_3,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_4,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_5,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_6,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_7,
// < Device specific , Power Up Mode
    DRX_POWER_MODE_8,
// < Device specific , Power Up Mode

    DRX_POWER_MODE_9,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_10,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_11,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_12,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_13,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_14,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_15,
// < Device specific , Power Down Mode
    DRX_POWER_MODE_16,
// < Device specific , Power Down Mode
    DRX_POWER_DOWN = 255
// < Generic         , Power Down Mode
}

// ========================================
//
// \enum enum drx_module * \brief Software module identification.
//
// Used by DRX_CTRL_VERSION.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_module {
    DRX_MODULE_DEVICE,
    DRX_MODULE_MICROCODE,
    DRX_MODULE_DRIVERCORE,
    DRX_MODULE_DEVICEDRIVER,
    DRX_MODULE_DAP,
    DRX_MODULE_BSP_I2C,
    DRX_MODULE_BSP_TUNER,
    DRX_MODULE_BSP_HOST,
    DRX_MODULE_UNKNOWN
}

//
// \enum struct drx_version * \brief Version information of one software module.
//
// Used by DRX_CTRL_VERSION.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_version {
    pub module_type: drx_module,
// < Type identifier of the module
    pub module_name: *mut c_char,
// < Name or description of module
    pub /: *mut *mut u16 v_major; /< Major version number,
    pub /: *mut *mut u16 v_minor; /< Minor version number,
    pub /: *mut *mut u16 v_patch; /< Patch version number,
    pub /: *mut *mut *mut char v_string; /< Version as text string,
}

//
// \enum struct drx_version_list * \brief List element of NULL terminated, linked list for version information.
//
// Used by DRX_CTRL_VERSION.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_version_list {
    pub /: *mut *mut *mut drx_version version;/< Version information,
    pub next: *mut drx_version_list,
// < Next list element
}

// ========================================
//
// \brief Parameters needed to confiugure a UIO.
//
// Used by DRX_CTRL_UIO_CFG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxuio_cfg {
    pub uio: drx_uio,
// < UIO identifier
    pub mode: drxuio_mode,
// < UIO operational mode
}

// ========================================
//
// \brief Parameters needed to read from or write to a UIO.
//
// Used by DRX_CTRL_UIO_READ and DRX_CTRL_UIO_WRITE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxuio_data {
    pub uio: drx_uio,
// < UIO identifier
    pub value: bool,
// < UIO value (true=1, false=0)
}

// ========================================
//
// \brief Parameters needed to configure OOB.
//
// Used by DRX_CTRL_SET_OOB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxoob {
    pub /: *mut *mut s32 frequency; /< Frequency in kHz,
    pub standard: drxoob_downstream_standard,
// < OOB standard
    pub spectrum: *mut *mut bool spectrum_inverted; /< If true, then,
}

// ========================================
//
// \brief Metrics from OOB.
//
// Used by DRX_CTRL_GET_OOB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxoob_status {
    pub /: *mut *mut s32 frequency; /< Frequency in Khz,
    pub /: *mut *mut drx_lock_status lock; /< Lock status,
    pub /: *mut *mut u32 mer; /< MER,
    pub /: *mut *mut s32 symbol_rate_offset; /< Symbolrate offset in ppm,
}

// ========================================
//
// \brief Device dependent configuration data.
//
// Used by DRX_CTRL_SET_CFG and DRX_CTRL_GET_CFG.
// A sort of nested drx_ctrl() functionality for device specific controls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg {
    pub cfg_type: u32,
// < Function identifier
    pub cfg_data: *mut c_void,
// < Function data
}

// ========================================
//
// /struct DRXMpegStartWidth_t
// MStart width [nr MCLK cycles] for serial MPEG output.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxmpeg_str_width {
    DRX_MPEG_STR_WIDTH_1,
    DRX_MPEG_STR_WIDTH_8
}

// CTRL CFG MPEG output
//
// \struct struct drx_cfg_mpeg_output * \brief Configuration parameters for MPEG output control.
//
// Used by DRX_CFG_MPEG_OUTPUT, in combination with DRX_CTRL_SET_CFG and
// DRX_CTRL_GET_CFG.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_mpeg_output {
    pub /: *mut *mut bool enable_mpeg_output;/< If true, enable MPEG output,
    pub /: *mut *mut bool insert_rs_byte; /< If true, insert RS byte,
    pub otherwise: *mut *mut bool enable_parallel; /< If true, parallel out,
    pub /: *mut *mut bool invert_data; /< If true, invert DATA signals,
    pub /: *mut *mut bool invert_err; /< If true, invert ERR signal,
    pub /: *mut *mut bool invert_str; /< If true, invert STR signals,
    pub /: *mut *mut bool invert_val; /< If true, invert VAL signals,
    pub /: *mut *mut bool invert_clk; /< If true, invert CLK signals,
    pub clockrate: *mut *mut bool static_clk; /< If true, static MPEG,
    pub case: *mut *mut u32 bitrate; /< Maximum bitrate in b/s in,
    pub width_str: drxmpeg_str_width,
// < MPEG start width
}

// ========================================
//
// \struct struct drxi2c_data * \brief Data for I2C via 2nd or 3rd or etc I2C port.
//
// Used by DRX_CTRL_I2C_READWRITE.
// If port_nr is equal to primairy port_nr BSPI2C will be used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drxi2c_data {
    pub /: *mut *mut u16 port_nr; /< I2C port number,
    pub w_dev_addr: *mut i2c_device_addr,
// < Write device address
    pub /: *mut *mut u16 w_count; /< Size of write data in bytes,
    pub /: *mut *mut *mut u8 wData; /< Pointer to write data,
    pub r_dev_addr: *mut i2c_device_addr,
// < Read device address
    pub /: *mut *mut u16 r_count; /< Size of data to read in bytes,
    pub /: *mut *mut *mut u8 r_data; /< Pointer to read buffer,
}

// ========================================
//
// \enum enum drx_aud_standard * \brief Audio standard identifier.
//
// Used by DRX_CTRL_SET_AUD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_standard {
    DRX_AUD_STANDARD_BTSC,	   /*< set BTSC standard (USA)       */
    DRX_AUD_STANDARD_A2,	   /*< set A2-Korea FM Stereo        */
    DRX_AUD_STANDARD_EIAJ,	   /*< set to Japanese FM Stereo     */
    DRX_AUD_STANDARD_FM_STEREO,/*< set to FM-Stereo Radio        */
    DRX_AUD_STANDARD_M_MONO,   /*< for 4.5 MHz mono detected     */
    DRX_AUD_STANDARD_D_K_MONO, /*< for 6.5 MHz mono detected     */
    DRX_AUD_STANDARD_BG_FM,	   /*< set BG_FM standard            */
    DRX_AUD_STANDARD_D_K1,	   /*< set D_K1 standard             */
    DRX_AUD_STANDARD_D_K2,	   /*< set D_K2 standard             */
    DRX_AUD_STANDARD_D_K3,	   /*< set D_K3 standard             */
    DRX_AUD_STANDARD_BG_NICAM_FM,
// < set BG_NICAM_FM standard
    DRX_AUD_STANDARD_L_NICAM_AM,
// < set L_NICAM_AM standard
    DRX_AUD_STANDARD_I_NICAM_FM,
// < set I_NICAM_FM standard
    DRX_AUD_STANDARD_D_K_NICAM_FM,
// < set D_K_NICAM_FM standard
    DRX_AUD_STANDARD_NOT_READY,/*< used to detect audio standard */
    DRX_AUD_STANDARD_AUTO = DRX_AUTO,
// < Automatic Standard Detection
    DRX_AUD_STANDARD_UNKNOWN = DRX_UNKNOWN
// < used as auto and for readback
}

// CTRL_AUD_GET_STATUS    - struct drx_aud_status
//
// \enum enum drx_aud_nicam_status * \brief Status of NICAM carrier.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_nicam_status {
    DRX_AUD_NICAM_DETECTED = 0,
// < NICAM carrier detected
    DRX_AUD_NICAM_NOT_DETECTED,
// < NICAM carrier not detected
    DRX_AUD_NICAM_BAD	  /*< NICAM carrier bad quality      */
}

//
// \struct struct drx_aud_status * \brief Audio status characteristics.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_aud_status {
    pub /: *mut *mut bool stereo; /< stereo detection,
    pub /: *mut *mut bool carrier_a; /< carrier A detected,
    pub /: *mut *mut bool carrier_b; /< carrier B detected,
    pub /: *mut *mut bool sap; /< sap / bilingual detection,
    pub /: *mut *mut bool rds; /< RDS data array present,
    pub nicam_status: drx_aud_nicam_status,
// < status of NICAM carrier
    pub /: *mut *mut s8 fm_ident; /< FM Identification value,
}

// CTRL_AUD_READ_RDS       - DRXRDSdata_t
//
// \struct DRXRDSdata_t
// \brief Raw RDS data array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_aud_rds {
    pub /: *mut *mut bool valid; /< RDS data validation,
    pub /: *mut *mut u16 data[18]; /< data from one RDS data array,
}

// DRX_CFG_AUD_VOLUME      - struct drx_cfg_aud_volume - set/get
//
// \enum DRXAudAVCDecayTime_t
// \brief Automatic volume control configuration.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_avc_mode {
    DRX_AUD_AVC_OFF,	  /*< Automatic volume control off   */
    DRX_AUD_AVC_DECAYTIME_8S, /*< level volume in  8 seconds     */
    DRX_AUD_AVC_DECAYTIME_4S, /*< level volume in  4 seconds     */
    DRX_AUD_AVC_DECAYTIME_2S, /*< level volume in  2 seconds     */
    DRX_AUD_AVC_DECAYTIME_20MS/*< level volume in 20 millisec    */
}

//
// /enum DRXAudMaxAVCGain_t
// /brief Automatic volume control max gain in audio baseband.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_avc_max_gain {
    DRX_AUD_AVC_MAX_GAIN_0DB, /*< maximum AVC gain  0 dB         */
    DRX_AUD_AVC_MAX_GAIN_6DB, /*< maximum AVC gain  6 dB         */
    DRX_AUD_AVC_MAX_GAIN_12DB /*< maximum AVC gain 12 dB         */
}

//
// /enum DRXAudMaxAVCAtten_t
// /brief Automatic volume control max attenuation in audio baseband.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_avc_max_atten {
    DRX_AUD_AVC_MAX_ATTEN_12DB,
// < maximum AVC attenuation 12 dB
    DRX_AUD_AVC_MAX_ATTEN_18DB,
// < maximum AVC attenuation 18 dB
    DRX_AUD_AVC_MAX_ATTEN_24DB/*< maximum AVC attenuation 24 dB  */
}

//
// \struct struct drx_cfg_aud_volume * \brief Audio volume configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_aud_volume {
    pub /: *mut *mut bool mute; /< mute overrides volume setting,
    pub /: *mut *mut s16 volume; /< volume, range -114 to 12 dB,
    pub /: *mut *mut drx_aud_avc_mode avc_mode; /< AVC auto volume control mode,
    pub /: *mut *mut u16 avc_ref_level; /< AVC reference level,
    pub avc_max_gain: drx_aud_avc_max_gain,
// < AVC max gain selection
    pub avc_max_atten: drx_aud_avc_max_atten,
// < AVC max attenuation selection
    pub /: *mut *mut s16 strength_left; /< quasi-peak, left speaker,
    pub /: *mut *mut s16 strength_right; /< quasi-peak, right speaker,
}

// DRX_CFG_I2S_OUTPUT      - struct drx_cfg_i2s_output - set/get
//
// \enum enum drxi2s_mode * \brief I2S output mode.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxi2s_mode {
    DRX_I2S_MODE_MASTER,	  /*< I2S is in master mode          */
    DRX_I2S_MODE_SLAVE	  /*< I2S is in slave mode           */
}

//
// \enum enum drxi2s_word_length * \brief Width of I2S data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxi2s_word_length {
    DRX_I2S_WORDLENGTH_32 = 0,/*< I2S data is 32 bit wide        */
    DRX_I2S_WORDLENGTH_16 = 1 /*< I2S data is 16 bit wide        */
}

//
// \enum enum drxi2s_format * \brief Data wordstrobe alignment for I2S.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxi2s_format {
    DRX_I2S_FORMAT_WS_WITH_DATA,
// < I2S data and wordstrobe are aligned
    DRX_I2S_FORMAT_WS_ADVANCED
// < I2S data one cycle after wordstrobe
}

//
// \enum enum drxi2s_polarity * \brief Polarity of I2S data.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drxi2s_polarity {
    DRX_I2S_POLARITY_RIGHT,/*< wordstrobe - right high, left low */
    DRX_I2S_POLARITY_LEFT  /*< wordstrobe - right low, left high */
}

//
// \struct struct drx_cfg_i2s_output * \brief I2S output configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_i2s_output {
    pub /: *mut *mut bool output_enable; /< I2S output enable,
    pub /: *mut *mut u32 frequency; /< range from 8000-48000 Hz,
    pub /: *mut *mut drxi2s_mode mode; /< I2S mode, master or slave,
    pub word_length: drxi2s_word_length,
// < I2S wordlength, 16 or 32 bits
    pub /: *mut *mut drxi2s_polarity polarity;/< I2S wordstrobe polarity,
    pub /: *mut *mut drxi2s_format format; /< I2S wordstrobe delay to data,
}

// ------------------------------expert interface-----------------------------
//
// /enum enum drx_aud_fm_deemphasis * setting for FM-Deemphasis in audio demodulator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_fm_deemphasis {
    DRX_AUD_FM_DEEMPH_50US,
    DRX_AUD_FM_DEEMPH_75US,
    DRX_AUD_FM_DEEMPH_OFF
}

//
// /enum DRXAudDeviation_t
// setting for deviation mode in audio demodulator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_cfg_aud_deviation {
    DRX_AUD_DEVIATION_NORMAL,
    DRX_AUD_DEVIATION_HIGH
}

//
// /enum enum drx_no_carrier_option * setting for carrier, mute/noise.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_no_carrier_option {
    DRX_NO_CARRIER_MUTE,
    DRX_NO_CARRIER_NOISE
}

//
// \enum DRXAudAutoSound_t
// \brief Automatic Sound
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_cfg_aud_auto_sound {
    DRX_AUD_AUTO_SOUND_OFF = 0,
    DRX_AUD_AUTO_SOUND_SELECT_ON_CHANGE_ON,
    DRX_AUD_AUTO_SOUND_SELECT_ON_CHANGE_OFF
}

//
// \enum DRXAudASSThres_t
// \brief Automatic Sound Select Thresholds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_aud_ass_thres {
    pub /: *mut *mut u16 a2; / A2 Threshold for ASS configuration,
    pub /: *mut *mut u16 btsc; / BTSC Threshold for ASS configuration,
    pub /: *mut *mut u16 nicam; / Nicam Threshold for ASS configuration,
}

//
// \struct struct drx_aud_carrier * \brief Carrier detection related parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_aud_carrier {
    pub /: *mut *mut u16 thres; / carrier detetcion threshold for primary carrier (A),
    pub /: *mut *mut drx_no_carrier_option opt; / Mute or noise at no carrier detection (A),
    pub /: *mut *mut s32 shift; / DC level of incoming signal (A),
    pub /: *mut *mut s32 dco; / frequency adjustment (A),
}

//
// \struct struct drx_cfg_aud_carriers * \brief combining carrier A & B to one struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_cfg_aud_carriers {
    pub a: drx_aud_carrier,
    pub b: drx_aud_carrier,
}

//
// /enum enum drx_aud_i2s_src * Selection of audio source
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_i2s_src {
    DRX_AUD_SRC_MONO,
    DRX_AUD_SRC_STEREO_OR_AB,
    DRX_AUD_SRC_STEREO_OR_A,
    DRX_AUD_SRC_STEREO_OR_B};

//
// \enum enum drx_aud_i2s_matrix * \brief Used for selecting I2S output.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_i2s_matrix {
    DRX_AUD_I2S_MATRIX_A_MONO,
// < A sound only, stereo or mono
    DRX_AUD_I2S_MATRIX_B_MONO,
// < B sound only, stereo or mono
    DRX_AUD_I2S_MATRIX_STEREO,
// < A+B sound, transparent
    DRX_AUD_I2S_MATRIX_MONO	/*< A+B mixed to mono sum, (L+R)/2   */};

//
// /enum enum drx_aud_fm_matrix * setting for FM-Matrix in audio demodulator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_fm_matrix {
    DRX_AUD_FM_MATRIX_NO_MATRIX,
    DRX_AUD_FM_MATRIX_GERMAN,
    DRX_AUD_FM_MATRIX_KOREAN,
    DRX_AUD_FM_MATRIX_SOUND_A,
    DRX_AUD_FM_MATRIX_SOUND_B};

//
// \struct DRXAudMatrices_t
// \brief Mixer settings
//
    struct drx_cfg_aud_mixer {
    enum drx_aud_i2s_src source_i2s;
    enum drx_aud_i2s_matrix matrix_i2s;
    enum drx_aud_fm_matrix matrix_fm;
}

//
// \enum DRXI2SVidSync_t
// \brief Audio/video synchronization, interacts with I2S mode.
// AUTO_1 and AUTO_2 are for automatic video standard detection with preference
// for NTSC or Monochrome, because the frequencies are too close (59.94 & 60 Hz)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_cfg_aud_av_sync {
    DRX_AUD_AVSYNC_OFF,/*< audio/video synchronization is off   */
    DRX_AUD_AVSYNC_NTSC,
// < it is an NTSC system
    DRX_AUD_AVSYNC_MONOCHROME,
// < it is a MONOCHROME system
    DRX_AUD_AVSYNC_PAL_SECAM
// < it is a PAL/SECAM system             */};

//
// \struct struct drx_cfg_aud_prescale * \brief Prescalers
//
    struct drx_cfg_aud_prescale {
    u16 fm_deviation;
    s16 nicam_gain;
}

//
// \struct struct drx_aud_beep * \brief Beep
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_aud_beep {
    pub /: *mut *mut s16 volume; / dB,
    pub /: *mut *mut u16 frequency; / Hz,
    pub mute: bool,
}

//
// \enum enum drx_aud_btsc_detect * \brief BTSC detetcion mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_aud_btsc_detect {
    DRX_BTSC_STEREO,
    DRX_BTSC_MONO_AND_SAP};

//
// \struct struct drx_aud_data * \brief Audio data structure
//
    struct drx_aud_data {
// audio storage
    bool audio_is_active;
    enum drx_aud_standard audio_standard;
    struct drx_cfg_i2s_output i2sdata;
    struct drx_cfg_aud_volume volume;
    enum drx_cfg_aud_auto_sound auto_sound;
    struct drx_cfg_aud_ass_thres ass_thresholds;
    struct drx_cfg_aud_carriers carriers;
    struct drx_cfg_aud_mixer mixer;
    enum drx_cfg_aud_deviation deviation;
    enum drx_cfg_aud_av_sync av_sync;
    struct drx_cfg_aud_prescale prescale;
    enum drx_aud_fm_deemphasis deemph;
    enum drx_aud_btsc_detect btsc_detect;
// rds
    u16 rds_data_counter;
    bool rds_data_present;
}

//
// \enum enum drx_qam_lock_range * \brief QAM lock range mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drx_qam_lock_range {
    DRX_QAM_LOCKRANGE_NORMAL,
    DRX_QAM_LOCKRANGE_EXTENDED};

// ============================================================================
// == Data access structures ==================================================
// ============================================================================

// Address on device
    typedef u32 dr_xaddr_t, *pdr_xaddr_t;

// Protocol specific flags
    typedef u32 dr_xflags_t, *pdr_xflags_t;

// Write block of data to device
    typedef int(*drx_write_block_func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u16 datasize,	/* size of data in bytes        */
    u8 *data,	/* data to send                 */
    u32 flags);

// Read block of data from device
    typedef int(*drx_read_block_func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u16 datasize,	/* size of data in bytes        */
    u8 *data,	/* receive buffer               */
    u32 flags);

// Write 8-bits value to device
    typedef int(*drx_write_reg8func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u8 data,	/* data to send                 */
    u32 flags);

// Read 8-bits value to device
    typedef int(*drx_read_reg8func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u8 *data,	/* receive buffer               */
    u32 flags);

// Read modify write 8-bits value to device
    typedef int(*drx_read_modify_write_reg8func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device       */
    u32 waddr,	/* write address of register   */
    u32 raddr,	/* read  address of register   */
    u8 wdata,	/* data to write               */
    u8 *rdata);	/* data to read                */

// Write 16-bits value to device
    typedef int(*drx_write_reg16func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u16 data,	/* data to send                 */
    u32 flags);

// Read 16-bits value to device
    typedef int(*drx_read_reg16func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u16 *data,	/* receive buffer               */
    u32 flags);

// Read modify write 16-bits value to device
    typedef int(*drx_read_modify_write_reg16func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device       */
    u32 waddr,	/* write address of register   */
    u32 raddr,	/* read  address of register   */
    u16 wdata,	/* data to write               */
    u16 *rdata);	/* data to read                */

// Write 32-bits value to device
    typedef int(*drx_write_reg32func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u32 data,	/* data to send                 */
    u32 flags);

// Read 32-bits value to device
    typedef int(*drx_read_reg32func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device        */
    u32 addr,	/* address of register/memory   */
    u32 *data,	/* receive buffer               */
    u32 flags);

// Read modify write 32-bits value to device
    typedef int(*drx_read_modify_write_reg32func_t) (struct i2c_device_addr *dev_addr,	/* address of I2C device       */
    u32 waddr,	/* write address of register   */
    u32 raddr,	/* read  address of register   */
    u32 wdata,	/* data to write               */
    u32 *rdata);	/* data to read                */

//
// \struct struct drx_access_func * \brief Interface to an access protocol.
//
    struct drx_access_func {
    drx_write_block_func_t write_block_func;
    drx_read_block_func_t read_block_func;
    drx_write_reg8func_t write_reg8func;
    drx_read_reg8func_t read_reg8func;
    drx_read_modify_write_reg8func_t read_modify_write_reg8func;
    drx_write_reg16func_t write_reg16func;
    drx_read_reg16func_t read_reg16func;
    drx_read_modify_write_reg16func_t read_modify_write_reg16func;
    drx_write_reg32func_t write_reg32func;
    drx_read_reg32func_t read_reg32func;
    drx_read_modify_write_reg32func_t read_modify_write_reg32func;
}

// Register address and data for register dump function
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_reg_dump {
    pub address: u32,
    pub data: u32,
}

// ============================================================================
// == Demod instance data structures ==========================================
// ============================================================================
//
// \struct struct drx_common_attr * \brief Set of common attributes, shared by all DRX devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_common_attr {
// Microcode (firmware) attributes
    pub /: *mut *mut *mut char microcode_file; /< microcode filename,
    pub verify_microcode: bool,
// < Use microcode verify or not.
    pub mcversion: drx_mc_version_rec,
// < Version record of microcode from file
// Clocks and tuner attributes
    pub intermediate_freq: i32,
// < IF,if tuner instance not used. (kHz)
    pub sys_clock_freq: i32,
// < Systemclock frequency.  (kHz)
    pub osc_clock_freq: i32,
// < Oscillator clock frequency.  (kHz)
    pub osc_clock_deviation: i16,
// < Oscillator clock deviation.  (ppm)
    pub mirror_freq_spect: bool,
// < Mirror IF frequency spectrum or not.
// Initial MPEG output attributes
    pub mpeg_cfg: drx_cfg_mpeg_output,
// < MPEG configuration
    pub /: *mut *mut bool is_opened; /< if true instance is already opened.,
// Channel scan
    pub scan_param: *mut drx_scan_param,
// < scan parameters
    pub scan_freq_plan_index: u16,
// < next index in freq plan
    pub scan_next_frequency: i32,
// < next freq to scan
    pub /: *mut *mut bool scan_ready; /< scan ready flag,
    pub /: *mut *mut u32 scan_max_channels;/< number of channels in freqplan,
    pub scan_channels_scanned: u32,
// < number of channels scanned
// Channel scan - inner loop: demod related
    pub scan_function: drx_scan_func_t,
// < function to check channel
// Channel scan - inner loop: SYSObj related
    pub /: *mut *mut *mut void scan_context; /< Context Pointer of SYSObj,
// Channel scan - parameters for default DTV scan function in core driver
    pub scan_demod_lock_timeout: u16,
// < millisecs to wait for lock
    pub scan_desired_lock: drx_lock_status,
// < lock requirement for channel found
// scan_active can be used by SetChannel to decide how to program the tuner,
    pub /: *mut *mut bool scan_active; /< true when scan routines are active,
// Power management
    pub current_power_mode: drx_power_mode,
// < current power management mode
// Tuner
    pub /: *mut *mut u8 tuner_port_nr; /< nr of I2C port to which tuner is,
    pub tuner_min_freq_rf: i32,
// < minimum RF input frequency, in kHz
    pub tuner_max_freq_rf: i32,
// < maximum RF input frequency, in kHz
    pub /: *mut *mut bool tuner_rf_agc_pol; /< if true invert RF AGC polarity,
    pub /: *mut *mut bool tuner_if_agc_pol; /< if true invert IF AGC polarity,
    pub /: *mut *mut bool tuner_slow_mode; /< if true invert IF AGC polarity,
    pub current_channel: drx_channel,
// < current channel parameters
    pub current_standard: drx_standard,
// < current standard selection
    pub prev_standard: drx_standard,
// < previous standard selection
    pub di_cache_standard: drx_standard,
// < standard in DI cache if available
    pub /: *mut *mut bool use_bootloader; /< use bootloader in open,
    pub /: *mut *mut u32 capabilities; /< capabilities flags,
    pub /}: *mut *mut u32 product_id; /< product ID inc. metal fix number,
//
// Generic functions for DRX devices.
//
    pub drx_demod_instance: struct,
//
// \struct struct drx_demod_instance * \brief Top structure of demodulator instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drx_demod_instance {
// < data access protocol functions
    pub my_i2c_dev_addr: *mut i2c_device_addr,
// < i2c address and device identifier
    pub my_common_attr: *mut drx_common_attr,
// < common DRX attributes
    pub /: *mut *mut *mut void my_ext_attr; /< device specific attributes,
// generic demodulator data
    pub i2c: *mut i2c_adapter,
}

// -------------------------------------------------------------------------
// standard

// channel

// TPS

// lock status

// version information , modules

// -------------------------------------------------------------------------
//
// \brief Create a compilable reference to the microcode attribute
// \param d pointer to demod instance
//
// Used as main reference to an attribute field.
// Used by both macro implementation and function implementation.
// These macros are defined to avoid duplication of code in macro and function
// definitions that handle access of demod common or extended attributes.
//

// Macro flag: #define DRX_ATTR_CURRENTPOWERMODE(d)((d)->my_common_attr->current_power_mode)

//
// Macros with device-specific handling are converted to CFG functions

// Configuration functions for usage by Access (XS) Macros

// Access Macros with device-specific handling

//
// \brief Macro to check if std is an ATV standard
// \retval true std is an ATV standard
// \retval false std is an ATV standard
//

//
// \brief Macro to check if std is an QAM standard
// \retval true std is an QAM standards
// \retval false std is an QAM standards
//

//
// \brief Macro to check if std is VSB standard
// \retval true std is VSB standard
// \retval false std is not VSB standard
//

//
// \brief Macro to check if std is DVBT standard
// \retval true std is DVBT standard
// \retval false std is not DVBT standard
//

// -------------------------------------------------------------------------
