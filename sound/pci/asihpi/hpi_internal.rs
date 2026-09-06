//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpi_internal.h
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

// maximum number of memory regions mapped to an adapter

// Each OS needs its own hpios.h

// physical memory allocation
// Allocate and map an area of locked memory for bus master DMA operations.
//
// < memory handle
// < OS specific data required for memory allocation
// Free mapping and memory represented by LockedMemHandle
//
extern "C" {
    pub fn hpios_locked_mem_free(locked_mem_handle: *mut consistent_dma_area) -> u16;
}
// Get the physical PCI address of memory represented by LockedMemHandle.
//
// locked_mem_handle, u32 *p_physical_addr);
// Get the CPU address of memory represented by LockedMemHandle.
//
// locked_mem_handle, void **ppv_virtual_addr);
// Check that handle is valid
//
extern "C" {
    pub fn hpios_locked_mem_valid(locked_mem_handle: *mut consistent_dma_area) -> u16;
}
// timing/delay
extern "C" {
    pub fn hpios_delay_micro_seconds(num_micro_sec: u32);
}
extern "C" {
    pub fn hpi_handler_func(: *mut hpi_message, : *mut hpi_response) -> typedef void;
}
// If the assert fails, compiler complains
//

// bus types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_BUSES {
    HPI_BUS_ISAPNP = 1,
    HPI_BUS_PCI = 2,
    HPI_BUS_USB = 3,
    HPI_BUS_NET = 4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_SUBSYS_OPTIONS {
// 0, 256 are invalid, 1..255 reserved for global options
    HPI_SUBSYS_OPT_NET_ENABLE = 257,
    HPI_SUBSYS_OPT_NET_BROADCAST = 258,
    HPI_SUBSYS_OPT_NET_UNICAST = 259,
    HPI_SUBSYS_OPT_NET_ADDR = 260,
    HPI_SUBSYS_OPT_NET_MASK = 261,
    HPI_SUBSYS_OPT_NET_ADAPTER_ADDRESS_ADD = 262
}

// Volume flags
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_VOLUME_FLAGS {
// Set if the volume control is muted
    HPI_VOLUME_FLAG_MUTED = (1 << 0),
// Set if the volume control has a mute function
    HPI_VOLUME_FLAG_HAS_MUTE = (1 << 1),
// Set if volume control can do autofading
    HPI_VOLUME_FLAG_HAS_AUTOFADE = (1 << 2)
// Note Flags >= (1<<8) are for DSP internal use only
}

// CONTROL ATTRIBUTES
// (in order of control type ID
// This allows for 255 control types, 256 unique attributes each

// Get the sub-index of the attribute for a control type

// Extract the control from the control attribute

// Enable event generation for a control.
//
// Unique identifiers for every control attribute
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_CONTROL_ATTRIBUTES {
    HPI_GENERIC_ENABLE = HPI_CTL_ATTR(GENERIC, 1),
    HPI_GENERIC_EVENT_ENABLE = HPI_CTL_ATTR(GENERIC, 2),

    HPI_VOLUME_GAIN = HPI_CTL_ATTR(VOLUME, 1),
    HPI_VOLUME_AUTOFADE = HPI_CTL_ATTR(VOLUME, 2),
    HPI_VOLUME_MUTE = HPI_CTL_ATTR(VOLUME, 3),
    HPI_VOLUME_GAIN_AND_FLAGS = HPI_CTL_ATTR(VOLUME, 4),
    HPI_VOLUME_NUM_CHANNELS = HPI_CTL_ATTR(VOLUME, 6),
    HPI_VOLUME_RANGE = HPI_CTL_ATTR(VOLUME, 10),

    HPI_METER_RMS = HPI_CTL_ATTR(METER, 1),
    HPI_METER_PEAK = HPI_CTL_ATTR(METER, 2),
    HPI_METER_RMS_BALLISTICS = HPI_CTL_ATTR(METER, 3),
    HPI_METER_PEAK_BALLISTICS = HPI_CTL_ATTR(METER, 4),
    HPI_METER_NUM_CHANNELS = HPI_CTL_ATTR(METER, 5),

    HPI_MULTIPLEXER_SOURCE = HPI_CTL_ATTR(MULTIPLEXER, 1),
    HPI_MULTIPLEXER_QUERYSOURCE = HPI_CTL_ATTR(MULTIPLEXER, 2),

    HPI_AESEBUTX_FORMAT = HPI_CTL_ATTR(AESEBUTX, 1),
    HPI_AESEBUTX_SAMPLERATE = HPI_CTL_ATTR(AESEBUTX, 3),
    HPI_AESEBUTX_CHANNELSTATUS = HPI_CTL_ATTR(AESEBUTX, 4),
    HPI_AESEBUTX_USERDATA = HPI_CTL_ATTR(AESEBUTX, 5),

    HPI_AESEBURX_FORMAT = HPI_CTL_ATTR(AESEBURX, 1),
    HPI_AESEBURX_ERRORSTATUS = HPI_CTL_ATTR(AESEBURX, 2),
    HPI_AESEBURX_SAMPLERATE = HPI_CTL_ATTR(AESEBURX, 3),
    HPI_AESEBURX_CHANNELSTATUS = HPI_CTL_ATTR(AESEBURX, 4),
    HPI_AESEBURX_USERDATA = HPI_CTL_ATTR(AESEBURX, 5),

    HPI_LEVEL_GAIN = HPI_CTL_ATTR(LEVEL, 1),
    HPI_LEVEL_RANGE = HPI_CTL_ATTR(LEVEL, 10),

    HPI_TUNER_BAND = HPI_CTL_ATTR(TUNER, 1),
    HPI_TUNER_FREQ = HPI_CTL_ATTR(TUNER, 2),
    HPI_TUNER_LEVEL_AVG = HPI_CTL_ATTR(TUNER, 3),
    HPI_TUNER_LEVEL_RAW = HPI_CTL_ATTR(TUNER, 4),
    HPI_TUNER_SNR = HPI_CTL_ATTR(TUNER, 5),
    HPI_TUNER_GAIN = HPI_CTL_ATTR(TUNER, 6),
    HPI_TUNER_STATUS = HPI_CTL_ATTR(TUNER, 7),
    HPI_TUNER_MODE = HPI_CTL_ATTR(TUNER, 8),
    HPI_TUNER_RDS = HPI_CTL_ATTR(TUNER, 9),
    HPI_TUNER_DEEMPHASIS = HPI_CTL_ATTR(TUNER, 10),
    HPI_TUNER_PROGRAM = HPI_CTL_ATTR(TUNER, 11),
    HPI_TUNER_HDRADIO_SIGNAL_QUALITY = HPI_CTL_ATTR(TUNER, 12),
    HPI_TUNER_HDRADIO_SDK_VERSION = HPI_CTL_ATTR(TUNER, 13),
    HPI_TUNER_HDRADIO_DSP_VERSION = HPI_CTL_ATTR(TUNER, 14),
    HPI_TUNER_HDRADIO_BLEND = HPI_CTL_ATTR(TUNER, 15),

    HPI_VOX_THRESHOLD = HPI_CTL_ATTR(VOX, 1),

    HPI_CHANNEL_MODE_MODE = HPI_CTL_ATTR(CHANNEL_MODE, 1),

    HPI_BITSTREAM_DATA_POLARITY = HPI_CTL_ATTR(BITSTREAM, 1),
    HPI_BITSTREAM_CLOCK_EDGE = HPI_CTL_ATTR(BITSTREAM, 2),
    HPI_BITSTREAM_CLOCK_SOURCE = HPI_CTL_ATTR(BITSTREAM, 3),
    HPI_BITSTREAM_ACTIVITY = HPI_CTL_ATTR(BITSTREAM, 4),

    HPI_SAMPLECLOCK_SOURCE = HPI_CTL_ATTR(SAMPLECLOCK, 1),
    HPI_SAMPLECLOCK_SAMPLERATE = HPI_CTL_ATTR(SAMPLECLOCK, 2),
    HPI_SAMPLECLOCK_SOURCE_INDEX = HPI_CTL_ATTR(SAMPLECLOCK, 3),
    HPI_SAMPLECLOCK_LOCAL_SAMPLERATE = HPI_CTL_ATTR(SAMPLECLOCK, 4),
    HPI_SAMPLECLOCK_AUTO = HPI_CTL_ATTR(SAMPLECLOCK, 5),
    HPI_SAMPLECLOCK_LOCAL_LOCK = HPI_CTL_ATTR(SAMPLECLOCK, 6),

    HPI_MICROPHONE_PHANTOM_POWER = HPI_CTL_ATTR(MICROPHONE, 1),

    HPI_EQUALIZER_NUM_FILTERS = HPI_CTL_ATTR(EQUALIZER, 1),
    HPI_EQUALIZER_FILTER = HPI_CTL_ATTR(EQUALIZER, 2),
    HPI_EQUALIZER_COEFFICIENTS = HPI_CTL_ATTR(EQUALIZER, 3),

    HPI_COMPANDER_PARAMS = HPI_CTL_ATTR(COMPANDER, 1),
    HPI_COMPANDER_MAKEUPGAIN = HPI_CTL_ATTR(COMPANDER, 2),
    HPI_COMPANDER_THRESHOLD = HPI_CTL_ATTR(COMPANDER, 3),
    HPI_COMPANDER_RATIO = HPI_CTL_ATTR(COMPANDER, 4),
    HPI_COMPANDER_ATTACK = HPI_CTL_ATTR(COMPANDER, 5),
    HPI_COMPANDER_DECAY = HPI_CTL_ATTR(COMPANDER, 6),

    HPI_COBRANET_SET = HPI_CTL_ATTR(COBRANET, 1),
    HPI_COBRANET_GET = HPI_CTL_ATTR(COBRANET, 2),
    HPI_COBRANET_GET_STATUS = HPI_CTL_ATTR(COBRANET, 5),
    HPI_COBRANET_SEND_PACKET = HPI_CTL_ATTR(COBRANET, 6),
    HPI_COBRANET_GET_PACKET = HPI_CTL_ATTR(COBRANET, 7),

    HPI_TONEDETECTOR_THRESHOLD = HPI_CTL_ATTR(TONEDETECTOR, 1),
    HPI_TONEDETECTOR_STATE = HPI_CTL_ATTR(TONEDETECTOR, 2),
    HPI_TONEDETECTOR_FREQUENCY = HPI_CTL_ATTR(TONEDETECTOR, 3),

    HPI_SILENCEDETECTOR_THRESHOLD = HPI_CTL_ATTR(SILENCEDETECTOR, 1),
    HPI_SILENCEDETECTOR_STATE = HPI_CTL_ATTR(SILENCEDETECTOR, 2),
    HPI_SILENCEDETECTOR_DELAY = HPI_CTL_ATTR(SILENCEDETECTOR, 3),

    HPI_PAD_CHANNEL_NAME = HPI_CTL_ATTR(PAD, 1),
    HPI_PAD_ARTIST = HPI_CTL_ATTR(PAD, 2),
    HPI_PAD_TITLE = HPI_CTL_ATTR(PAD, 3),
    HPI_PAD_COMMENT = HPI_CTL_ATTR(PAD, 4),
    HPI_PAD_PROGRAM_TYPE = HPI_CTL_ATTR(PAD, 5),
    HPI_PAD_PROGRAM_ID = HPI_CTL_ATTR(PAD, 6),
    HPI_PAD_TA_SUPPORT = HPI_CTL_ATTR(PAD, 7),
    HPI_PAD_TA_ACTIVE = HPI_CTL_ATTR(PAD, 8),

    HPI_UNIVERSAL_ENTITY = HPI_CTL_ATTR(UNIVERSAL, 1)
}

pub const HPI_POLARITY_POSITIVE: c_int = 0;
pub const HPI_POLARITY_NEGATIVE: c_int = 1;
// ------------------------------------------------------------
pub const HPI_COBRANET_HMI_cobra_bridge: c_uint = 0x20000;

pub const HPI_COBRANET_HMI_cobra_if_table1: c_uint = 0x110000;

pub const HPI_COBRANET_HMI_cobra_protocolIP: c_uint = 0x72000;

pub const HPI_COBRANET_HMI_cobra_sys: c_uint = 0x100000;

// ------------------------------------------------------------
pub const HPI_COBRANET_HMI_STATUS_RXPACKET: c_int = 2;
pub const HPI_COBRANET_HMI_STATUS_TXPACKET: c_int = 3;
// ------------------------------------------------------------

// These defines are used to fill in protocol information for an Ethernet packet
// ID supplied by Cirrus for ASI packets.
pub const HPI_ETHERNET_PACKET_ID: c_uint = 0x85;
// Simple packet - no special routing required
pub const HPI_ETHERNET_PACKET_V1: c_uint = 0x01;
// This packet must make its way to the host across the HPI interface
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HMI: c_uint = 0x20;
// This packet must make its way to the host across the HPI interface
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HMI_V1: c_uint = 0x21;
// This packet must make its way to the host across the HPI interface
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HPI: c_uint = 0x40;
// This packet must make its way to the host across the HPI interface
pub const HPI_ETHERNET_PACKET_HOSTED_VIA_HPI_V1: c_uint = 0x41;

// Default network timeout in milli-seconds.
pub const HPI_ETHERNET_TIMEOUT_MS: c_int = 500;
// Locked memory buffer alloc/free phases
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_BUFFER_CMDS {
// use one message to allocate or free physical memory
    HPI_BUFFER_CMD_EXTERNAL = 0,
// alloc physical memory
    HPI_BUFFER_CMD_INTERNAL_ALLOC = 1,
// send physical memory address to adapter
    HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER = 2,
// notify adapter to stop using physical buffer
    HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER = 3,
// free physical buffer
    HPI_BUFFER_CMD_INTERNAL_FREE = 4
}

//
// HPI LOW LEVEL MESSAGES
//
// Pnp ids
// "ASI"  - actual is "ASX" - need to change
pub const HPI_ID_ISAPNP_AUDIOSCIENCE: c_uint = 0x0669;
// PCI vendor ID that AudioScience uses
pub const HPI_PCI_VENDOR_ID_AUDIOSCIENCE: c_uint = 0x175C;
// PCI vendor ID that the DSP56301 has
pub const HPI_PCI_VENDOR_ID_MOTOROLA: c_uint = 0x1057;
// PCI vendor ID that TI uses
pub const HPI_PCI_VENDOR_ID_TI: c_uint = 0x104C;
pub const HPI_PCI_DEV_ID_PCI2040: c_uint = 0xAC60;
// TI's C6205 PCI interface has this ID
pub const HPI_PCI_DEV_ID_DSP6205: c_uint = 0xA106;
pub const HPI_USB_VENDOR_ID_AUDIOSCIENCE: c_uint = 0x1257;
pub const HPI_USB_W2K_TAG: c_uint = 0x57495341	/* "ASIW"       */;
pub const HPI_USB_LINUX_TAG: c_uint = 0x4C495341	/* "ASIL"       */;
// Invalid Adapter index
//
pub const HPI_ADAPTER_INDEX_INVALID: c_uint = 0xFFFF;
// First 2 hex digits define the adapter family
pub const HPI_ADAPTER_FAMILY_MASK: c_uint = 0xff00;
pub const HPI_MODULE_FAMILY_MASK: c_uint = 0xfff0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_MESSAGE_TYPES {
    HPI_TYPE_REQUEST = 1,
    HPI_TYPE_RESPONSE = 2,
    HPI_TYPE_DATA = 3,
    HPI_TYPE_SSX2BYPASS_MESSAGE = 4,
    HPI_TYPE_COMMAND = 5,
    HPI_TYPE_NOTIFICATION = 6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_OBJECT_TYPES {
    HPI_OBJ_SUBSYSTEM = 1,
    HPI_OBJ_ADAPTER = 2,
    HPI_OBJ_OSTREAM = 3,
    HPI_OBJ_ISTREAM = 4,
    HPI_OBJ_MIXER = 5,
    HPI_OBJ_NODE = 6,
    HPI_OBJ_CONTROL = 7,
    HPI_OBJ_NVMEMORY = 8,
    HPI_OBJ_GPIO = 9,
    HPI_OBJ_WATCHDOG = 10,
    HPI_OBJ_CLOCK = 11,
    HPI_OBJ_PROFILE = 12,
// HPI_ OBJ_ CONTROLEX  = 13,
    HPI_OBJ_ASYNCEVENT = 14
pub const HPI_OBJ_MAXINDEX: c_int = 14;
}

pub const HPI_OBJ_FUNCTION_SPACING: c_uint = 0x100;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HPI_FUNCTION_IDS {
    HPI_SUBSYS_OPEN = HPI_FUNC_ID(SUBSYSTEM, 1),
    HPI_SUBSYS_GET_VERSION = HPI_FUNC_ID(SUBSYSTEM, 2),
    HPI_SUBSYS_GET_INFO = HPI_FUNC_ID(SUBSYSTEM, 3),
    HPI_SUBSYS_CREATE_ADAPTER = HPI_FUNC_ID(SUBSYSTEM, 5),
    HPI_SUBSYS_CLOSE = HPI_FUNC_ID(SUBSYSTEM, 6),
    HPI_SUBSYS_DRIVER_LOAD = HPI_FUNC_ID(SUBSYSTEM, 8),
    HPI_SUBSYS_DRIVER_UNLOAD = HPI_FUNC_ID(SUBSYSTEM, 9),
    HPI_SUBSYS_GET_NUM_ADAPTERS = HPI_FUNC_ID(SUBSYSTEM, 12),
    HPI_SUBSYS_GET_ADAPTER = HPI_FUNC_ID(SUBSYSTEM, 13),
    HPI_SUBSYS_SET_NETWORK_INTERFACE = HPI_FUNC_ID(SUBSYSTEM, 14),
    HPI_SUBSYS_OPTION_INFO = HPI_FUNC_ID(SUBSYSTEM, 15),
    HPI_SUBSYS_OPTION_GET = HPI_FUNC_ID(SUBSYSTEM, 16),
    HPI_SUBSYS_OPTION_SET = HPI_FUNC_ID(SUBSYSTEM, 17),
pub const HPI_SUBSYS_FUNCTION_COUNT: c_int = 17;

    HPI_ADAPTER_OPEN = HPI_FUNC_ID(ADAPTER, 1),
    HPI_ADAPTER_CLOSE = HPI_FUNC_ID(ADAPTER, 2),
    HPI_ADAPTER_GET_INFO = HPI_FUNC_ID(ADAPTER, 3),
    HPI_ADAPTER_GET_ASSERT = HPI_FUNC_ID(ADAPTER, 4),
    HPI_ADAPTER_TEST_ASSERT = HPI_FUNC_ID(ADAPTER, 5),
    HPI_ADAPTER_SET_MODE = HPI_FUNC_ID(ADAPTER, 6),
    HPI_ADAPTER_GET_MODE = HPI_FUNC_ID(ADAPTER, 7),
    HPI_ADAPTER_ENABLE_CAPABILITY = HPI_FUNC_ID(ADAPTER, 8),
    HPI_ADAPTER_SELFTEST = HPI_FUNC_ID(ADAPTER, 9),
    HPI_ADAPTER_FIND_OBJECT = HPI_FUNC_ID(ADAPTER, 10),
    HPI_ADAPTER_QUERY_FLASH = HPI_FUNC_ID(ADAPTER, 11),
    HPI_ADAPTER_START_FLASH = HPI_FUNC_ID(ADAPTER, 12),
    HPI_ADAPTER_PROGRAM_FLASH = HPI_FUNC_ID(ADAPTER, 13),
    HPI_ADAPTER_SET_PROPERTY = HPI_FUNC_ID(ADAPTER, 14),
    HPI_ADAPTER_GET_PROPERTY = HPI_FUNC_ID(ADAPTER, 15),
    HPI_ADAPTER_ENUM_PROPERTY = HPI_FUNC_ID(ADAPTER, 16),
    HPI_ADAPTER_MODULE_INFO = HPI_FUNC_ID(ADAPTER, 17),
    HPI_ADAPTER_DEBUG_READ = HPI_FUNC_ID(ADAPTER, 18),
    HPI_ADAPTER_IRQ_QUERY_AND_CLEAR = HPI_FUNC_ID(ADAPTER, 19),
    HPI_ADAPTER_IRQ_CALLBACK = HPI_FUNC_ID(ADAPTER, 20),
    HPI_ADAPTER_DELETE = HPI_FUNC_ID(ADAPTER, 21),
    HPI_ADAPTER_READ_FLASH = HPI_FUNC_ID(ADAPTER, 22),
    HPI_ADAPTER_END_FLASH = HPI_FUNC_ID(ADAPTER, 23),
    HPI_ADAPTER_FILESTORE_DELETE_ALL = HPI_FUNC_ID(ADAPTER, 24),
pub const HPI_ADAPTER_FUNCTION_COUNT: c_int = 24;

    HPI_OSTREAM_OPEN = HPI_FUNC_ID(OSTREAM, 1),
    HPI_OSTREAM_CLOSE = HPI_FUNC_ID(OSTREAM, 2),
    HPI_OSTREAM_WRITE = HPI_FUNC_ID(OSTREAM, 3),
    HPI_OSTREAM_START = HPI_FUNC_ID(OSTREAM, 4),
    HPI_OSTREAM_STOP = HPI_FUNC_ID(OSTREAM, 5),
    HPI_OSTREAM_RESET = HPI_FUNC_ID(OSTREAM, 6),
    HPI_OSTREAM_GET_INFO = HPI_FUNC_ID(OSTREAM, 7),
    HPI_OSTREAM_QUERY_FORMAT = HPI_FUNC_ID(OSTREAM, 8),
    HPI_OSTREAM_DATA = HPI_FUNC_ID(OSTREAM, 9),
    HPI_OSTREAM_SET_VELOCITY = HPI_FUNC_ID(OSTREAM, 10),
    HPI_OSTREAM_SET_PUNCHINOUT = HPI_FUNC_ID(OSTREAM, 11),
    HPI_OSTREAM_SINEGEN = HPI_FUNC_ID(OSTREAM, 12),
    HPI_OSTREAM_ANC_RESET = HPI_FUNC_ID(OSTREAM, 13),
    HPI_OSTREAM_ANC_GET_INFO = HPI_FUNC_ID(OSTREAM, 14),
    HPI_OSTREAM_ANC_READ = HPI_FUNC_ID(OSTREAM, 15),
    HPI_OSTREAM_SET_TIMESCALE = HPI_FUNC_ID(OSTREAM, 16),
    HPI_OSTREAM_SET_FORMAT = HPI_FUNC_ID(OSTREAM, 17),
    HPI_OSTREAM_HOSTBUFFER_ALLOC = HPI_FUNC_ID(OSTREAM, 18),
    HPI_OSTREAM_HOSTBUFFER_FREE = HPI_FUNC_ID(OSTREAM, 19),
    HPI_OSTREAM_GROUP_ADD = HPI_FUNC_ID(OSTREAM, 20),
    HPI_OSTREAM_GROUP_GETMAP = HPI_FUNC_ID(OSTREAM, 21),
    HPI_OSTREAM_GROUP_RESET = HPI_FUNC_ID(OSTREAM, 22),
    HPI_OSTREAM_HOSTBUFFER_GET_INFO = HPI_FUNC_ID(OSTREAM, 23),
    HPI_OSTREAM_WAIT_START = HPI_FUNC_ID(OSTREAM, 24),
    HPI_OSTREAM_WAIT = HPI_FUNC_ID(OSTREAM, 25),
pub const HPI_OSTREAM_FUNCTION_COUNT: c_int = 25;

    HPI_ISTREAM_OPEN = HPI_FUNC_ID(ISTREAM, 1),
    HPI_ISTREAM_CLOSE = HPI_FUNC_ID(ISTREAM, 2),
    HPI_ISTREAM_SET_FORMAT = HPI_FUNC_ID(ISTREAM, 3),
    HPI_ISTREAM_READ = HPI_FUNC_ID(ISTREAM, 4),
    HPI_ISTREAM_START = HPI_FUNC_ID(ISTREAM, 5),
    HPI_ISTREAM_STOP = HPI_FUNC_ID(ISTREAM, 6),
    HPI_ISTREAM_RESET = HPI_FUNC_ID(ISTREAM, 7),
    HPI_ISTREAM_GET_INFO = HPI_FUNC_ID(ISTREAM, 8),
    HPI_ISTREAM_QUERY_FORMAT = HPI_FUNC_ID(ISTREAM, 9),
    HPI_ISTREAM_ANC_RESET = HPI_FUNC_ID(ISTREAM, 10),
    HPI_ISTREAM_ANC_GET_INFO = HPI_FUNC_ID(ISTREAM, 11),
    HPI_ISTREAM_ANC_WRITE = HPI_FUNC_ID(ISTREAM, 12),
    HPI_ISTREAM_HOSTBUFFER_ALLOC = HPI_FUNC_ID(ISTREAM, 13),
    HPI_ISTREAM_HOSTBUFFER_FREE = HPI_FUNC_ID(ISTREAM, 14),
    HPI_ISTREAM_GROUP_ADD = HPI_FUNC_ID(ISTREAM, 15),
    HPI_ISTREAM_GROUP_GETMAP = HPI_FUNC_ID(ISTREAM, 16),
    HPI_ISTREAM_GROUP_RESET = HPI_FUNC_ID(ISTREAM, 17),
    HPI_ISTREAM_HOSTBUFFER_GET_INFO = HPI_FUNC_ID(ISTREAM, 18),
    HPI_ISTREAM_WAIT_START = HPI_FUNC_ID(ISTREAM, 19),
    HPI_ISTREAM_WAIT = HPI_FUNC_ID(ISTREAM, 20),
pub const HPI_ISTREAM_FUNCTION_COUNT: c_int = 20;

// NOTE:
    GET_NODE_INFO, SET_CONNECTION, GET_CONNECTIONS are not currently used */
    HPI_MIXER_OPEN = HPI_FUNC_ID(MIXER, 1),
    HPI_MIXER_CLOSE = HPI_FUNC_ID(MIXER, 2),
    HPI_MIXER_GET_INFO = HPI_FUNC_ID(MIXER, 3),
    HPI_MIXER_GET_NODE_INFO = HPI_FUNC_ID(MIXER, 4),
    HPI_MIXER_GET_CONTROL = HPI_FUNC_ID(MIXER, 5),
    HPI_MIXER_SET_CONNECTION = HPI_FUNC_ID(MIXER, 6),
    HPI_MIXER_GET_CONNECTIONS = HPI_FUNC_ID(MIXER, 7),
    HPI_MIXER_GET_CONTROL_BY_INDEX = HPI_FUNC_ID(MIXER, 8),
    HPI_MIXER_GET_CONTROL_ARRAY_BY_INDEX = HPI_FUNC_ID(MIXER, 9),
    HPI_MIXER_GET_CONTROL_MULTIPLE_VALUES = HPI_FUNC_ID(MIXER, 10),
    HPI_MIXER_STORE = HPI_FUNC_ID(MIXER, 11),
    HPI_MIXER_GET_CACHE_INFO = HPI_FUNC_ID(MIXER, 12),
    HPI_MIXER_GET_BLOCK_HANDLE = HPI_FUNC_ID(MIXER, 13),
    HPI_MIXER_GET_PARAMETER_HANDLE = HPI_FUNC_ID(MIXER, 14),
pub const HPI_MIXER_FUNCTION_COUNT: c_int = 14;

    HPI_CONTROL_GET_INFO = HPI_FUNC_ID(CONTROL, 1),
    HPI_CONTROL_GET_STATE = HPI_FUNC_ID(CONTROL, 2),
    HPI_CONTROL_SET_STATE = HPI_FUNC_ID(CONTROL, 3),
pub const HPI_CONTROL_FUNCTION_COUNT: c_int = 3;

    HPI_NVMEMORY_OPEN = HPI_FUNC_ID(NVMEMORY, 1),
    HPI_NVMEMORY_READ_BYTE = HPI_FUNC_ID(NVMEMORY, 2),
    HPI_NVMEMORY_WRITE_BYTE = HPI_FUNC_ID(NVMEMORY, 3),
pub const HPI_NVMEMORY_FUNCTION_COUNT: c_int = 3;

    HPI_GPIO_OPEN = HPI_FUNC_ID(GPIO, 1),
    HPI_GPIO_READ_BIT = HPI_FUNC_ID(GPIO, 2),
    HPI_GPIO_WRITE_BIT = HPI_FUNC_ID(GPIO, 3),
    HPI_GPIO_READ_ALL = HPI_FUNC_ID(GPIO, 4),
    HPI_GPIO_WRITE_STATUS = HPI_FUNC_ID(GPIO, 5),
pub const HPI_GPIO_FUNCTION_COUNT: c_int = 5;

    HPI_ASYNCEVENT_OPEN = HPI_FUNC_ID(ASYNCEVENT, 1),
    HPI_ASYNCEVENT_CLOSE = HPI_FUNC_ID(ASYNCEVENT, 2),
    HPI_ASYNCEVENT_WAIT = HPI_FUNC_ID(ASYNCEVENT, 3),
    HPI_ASYNCEVENT_GETCOUNT = HPI_FUNC_ID(ASYNCEVENT, 4),
    HPI_ASYNCEVENT_GET = HPI_FUNC_ID(ASYNCEVENT, 5),
    HPI_ASYNCEVENT_SENDEVENTS = HPI_FUNC_ID(ASYNCEVENT, 6),
pub const HPI_ASYNCEVENT_FUNCTION_COUNT: c_int = 6;

    HPI_WATCHDOG_OPEN = HPI_FUNC_ID(WATCHDOG, 1),
    HPI_WATCHDOG_SET_TIME = HPI_FUNC_ID(WATCHDOG, 2),
    HPI_WATCHDOG_PING = HPI_FUNC_ID(WATCHDOG, 3),

    HPI_CLOCK_OPEN = HPI_FUNC_ID(CLOCK, 1),
    HPI_CLOCK_SET_TIME = HPI_FUNC_ID(CLOCK, 2),
    HPI_CLOCK_GET_TIME = HPI_FUNC_ID(CLOCK, 3),

    HPI_PROFILE_OPEN_ALL = HPI_FUNC_ID(PROFILE, 1),
    HPI_PROFILE_START_ALL = HPI_FUNC_ID(PROFILE, 2),
    HPI_PROFILE_STOP_ALL = HPI_FUNC_ID(PROFILE, 3),
    HPI_PROFILE_GET = HPI_FUNC_ID(PROFILE, 4),
    HPI_PROFILE_GET_IDLECOUNT = HPI_FUNC_ID(PROFILE, 5),
    HPI_PROFILE_GET_NAME = HPI_FUNC_ID(PROFILE, 6),
    HPI_PROFILE_GET_UTILIZATION = HPI_FUNC_ID(PROFILE, 7)
pub const HPI_PROFILE_FUNCTION_COUNT: c_int = 7;
}

// //////////////////////////////////////////////////////////////////////
// STRUCTURES

// PCI bus resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_pci {
    pub ap_mem_base: [*mut u32 __iomem; HPI_MAX_ADAPTER_MEM_SPACES],
    pub pci_dev: *mut pci_dev,
}

// Adapter specification resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_specification {
    pub type: u32,
    pub modules: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_resource {
    pub pci: *const hpi_pci,
    pub net_if: *const c_char,
    pub adapter_spec: hpi_adapter_specification,
    pub sw_if: *const c_void,
    pub r: },
    pub /: *mut *mut u16 bus_type; / HPI_BUS_PNPISA, _PCI, _USB etc,
    pub padding: u16,
}

// Format info used inside struct hpi_message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_format {
    pub /: *mut *mut *mut u32 sample_rate; /< 11025, 32000, 44100 etc.,
    pub /: *mut *mut *mut u32 bit_rate; /< for MPEG,
    pub /: *mut *mut *mut u32 attributes; /< stereo/joint_stereo/mono,
    pub /: *mut *mut *mut u16 channels; /< 1,2..., (or ancillary mode or idle bit,
    pub /: *mut *mut *mut u16 format; /< HPI_FORMAT_PCM16, _MPEG etc. see \ref HPI_FORMATS.,
}

// Buffer+format structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_data {
    pub format: hpi_msg_format,
    pub pb_data: *mut u8,

    pub padding: u32,

    pub data_size: u32,
}

// struct hpi_datastructure used up to 3.04 driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_data_legacy32 {
    pub format: hpi_format,
    pub pb_data: u32,
    pub data_size: u32,
}

// Compatibility version of struct hpi_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_data_compat32 {
    pub format: hpi_msg_format,
    pub pb_data: u32,
    pub padding: u32,
    pub data_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_buffer {
// placeholder for backward compatibility (see dwBufferSize)
    pub reserved: hpi_msg_format,
    pub HPI_BUFFER_CMD_xxx*/: *mut *mut *mut u32 command; /<,
    pub /: *mut *mut *mut u32 pci_address; /< PCI physical address of buffer for DSP DMA,
    pub HPI_DATA*/: *mut *mut *mut u32 buffer_size; /< must line up with data_size of,
}

//
// This is used for background buffer bus mastering stream buffers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_hostbuffer_status {
    pub samples_processed: u32,
    pub auxiliary_data_available: u32,
    pub stream_state: u32,
// DSP index in to the host bus master buffer.
    pub dsp_index: u32,
// Host index in to the host bus master buffer.
    pub host_index: u32,
    pub size_in_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_streamid {
    pub object_type: u16,
// < Type of object, HPI_OBJ_OSTREAM or HPI_OBJ_ISTREAM.
    pub /: *mut *mut *mut u16 stream_index; /< outstream or instream index.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_punchinout {
    pub punch_in_sample: u32,
    pub punch_out_sample: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_msg {
    pub resource: hpi_resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_res {
    pub version: u32,
    pub /: *mut *mut u32 data; / extended version,
    pub num_adapters: u16,
    pub adapter_index: u16,
    pub adapter_type: u16,
    pub pad16: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_adapterx_msg {
    pub dsp_address: u32,
    pub count_bytes: u32,
    pub debug_read: },
    pub adapter_mode: u32,
    pub query_or_set: u16,
    pub mode: },
    pub index: u16,
    pub module_info: },
    pub index: u16,
    pub what: u16,
    pub property_index: u16,
    pub property_enum: },
    pub property: u16,
    pub parameter1: u16,
    pub parameter2: u16,
    pub property_set: },
    pub pad32: u32,
    pub key1: u16,
    pub key2: u16,
    pub restart: },
    pub pad32: u32,
    pub value: u16,
    pub test_assert: },
    pub message: u32,
    pub irq: },
    pub pad: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_res {
    pub serial_number: u32,
    pub adapter_type: u16,
    pub adapter_index: u16,
    pub num_instreams: u16,
    pub num_outstreams: u16,
    pub num_mixers: u16,
    pub version: u16,
    pub sz_adapter_assert: [u8; HPI_STRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_adapterx_res {
    pub info: hpi_adapter_res,
    pub p1: u32,
    pub count: u16,
    pub dsp_index: u16,
    pub p2: u32,
    pub dsp_msg_addr: u32,
    pub sz_message: [c_char; HPI_STRING_LEN],
    pub assert: },
    pub adapter_mode: u32,
    pub mode: },
    pub parameter1: u16,
    pub parameter2: u16,
    pub property_get: },
    pub yes: u32,
    pub irq_query: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_stream_msg {
    pub data: hpi_msg_data,
    pub data32: hpi_data_legacy32,
    pub velocity: u16,
    pub pio: hpi_punchinout,
    pub time_scale: u32,
    pub buffer: hpi_buffer,
    pub stream: hpi_streamid,
    pub threshold_bytes: u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_stream_res {
// size of hardware buffer
    pub buffer_size: u32,
// OutStream - data to play,
    pub data_available: u32,
// OutStream - samples played,
    pub samples_transferred: u32,
// Adapter - OutStream - data to play,
    pub auxiliary_data_available: u32,
    pub /: *mut *mut u16 state; / HPI_STATE_PLAYING, _STATE_STOPPED,
    pub padding: u16,
    pub stream_info: },
    pub buffer_size: u32,
    pub data_available: u32,
    pub samples_transfered: u32,
    pub state: u16,
    pub outstream_index: u16,
    pub instream_index: u16,
    pub padding: u16,
    pub auxiliary_data_available: u32,
    pub legacy_stream_info: },
// bitmap of grouped OutStreams
    pub outstream_group_map: u32,
// bitmap of grouped InStreams
    pub instream_group_map: u32,
    pub group_info: },
// pointer to the buffer
    pub p_buffer: *mut u8,
// pointer to the hostbuffer status
    pub p_status: *mut hpi_hostbuffer_status,
    pub hostbuffer_info: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_mixer_msg {
    pub control_index: u16,
    pub /: *mut *mut u16 control_type; / = HPI_CONTROL_METER _VOLUME etc,
    pub /: *mut *mut u16 padding1; / Maintain alignment of subsequent fields,
    pub /: *mut *mut u16 node_type1; / = HPI_SOURCENODE_LINEIN etc,
    pub /: *mut *mut u16 node_index1; / = 0..N,
    pub node_type2: u16,
    pub node_index2: u16,
    pub /: *mut *mut u16 padding2; / round to 4 bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_mixer_res {
    pub /: *mut *mut u16 src_node_type; / = HPI_SOURCENODE_LINEIN etc,
    pub /: *mut *mut u16 src_node_index; / = 0..N,
    pub dst_node_type: u16,
    pub dst_node_index: u16,
// Also controlType for MixerGetControlByIndex
    pub control_index: u16,
// may indicate which DSP the control is located on
    pub dsp_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_mixerx_msg {
    pub starting_index: u16,
    pub flags: u16,
    pub /: *mut *mut u32 length_in_bytes; / length in bytes of p_data,
    pub /: *mut *mut u32 p_data; / pointer to a data array,
    pub gcabi: },
    pub command: u16,
    pub index: u16,
    pub /: *mut *mut } store; / for HPI_MIXER_STORE message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_mixerx_res {
    pub /: *mut *mut u32 bytes_returned; / size of items returned,
    pub /: *mut *mut u32 p_data; / pointer to data array,
    pub /: *mut *mut u16 more_to_do; / indicates if there is more to do,
    pub gcabi: },
    pub /: *mut *mut u32 total_controls; / count of controls in the mixer,
    pub /: *mut *mut u32 cache_controls; / count of controls in the cac,
    pub /: *mut *mut u32 cache_bytes; / size of cache,
    pub cache_info: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_msg {
    pub /: *mut *mut u16 attribute; / control attribute or property,
    pub saved_index: u16,
    pub /: *mut *mut u32 param1; / generic parameter 1,
    pub /: *mut *mut u32 param2; / generic parameter 2,
    pub an_log_value: [c_short; HPI_MAX_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_union_msg {
    pub /: *mut *mut u16 attribute; / control attribute or property,
    pub /: *mut *mut u16 saved_index; / only used in ctrl save/restore,
    pub /: *mut *mut u32 param1; / generic parameter 1,
    pub /: *mut *mut u32 param2; / generic parameter 2,
    pub an_log_value: [c_short; HPI_MAX_CHANNELS],
    pub old: },
    pub frequency: u32,
    pub gain: u32,
    pub band: u32,
    pub deemphasis: u32,
    pub program: u32,
    pub mode: u32,
    pub value: u32,
    pub mode: },
    pub blend: u32,
    pub tuner: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_res {
// Could make union. dwParam, anLogValue never used in same response
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [c_short; HPI_MAX_CHANNELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_control_union_res {
    pub param1: u32,
    pub param2: u32,
    pub an_log_value: [c_short; HPI_MAX_CHANNELS],
    pub old: },
    pub band: u32,
    pub frequency: u32,
    pub gain: u32,
    pub deemphasis: u32,
    pub data: [u32; 2],
    pub bLER: u32,
    pub rds: },
    pub s_level: c_short,
    pub value: u16,
    pub mask: u16,
    pub status: },
    pub tuner: },
    pub sz_data: [c_char; 8],
    pub remaining_chars: u32,
    pub chars8: },
    pub c_data12: [c_char; 12],
    pub status: u32,
    pub readable_size: u32,
    pub writeable_size: u32,
    pub status: },
    pub cobranet: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_nvmemory_msg {
    pub address: u16,
    pub data: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_nvmemory_res {
    pub size_in_bytes: u16,
    pub data: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_gpio_msg {
    pub bit_index: u16,
    pub bit_data: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_gpio_res {
    pub number_input_bits: u16,
    pub number_output_bits: u16,
    pub bit_data: [u16; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_async_msg {
    pub events: u32,
    pub maximum_events: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_async_res {
    pub count: u16,
    pub count: },
    pub events: u32,
    pub number_returned: u16,
    pub padding: u16,
    pub get: },
    pub event: hpi_async_event,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_watchdog_msg {
    pub time_ms: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_watchdog_res {
    pub time_ms: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_clock_msg {
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub milli_seconds: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_clock_res {
    pub size_in_bytes: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub milli_seconds: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_profile_msg {
    pub bin_index: u16,
    pub padding: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_open {
    pub max_profiles: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_time {
    pub total_tick_count: u32,
    pub call_count: u32,
    pub max_tick_count: u32,
    pub ticks_per_millisecond: u32,
    pub profile_interval: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res_name {
    pub sz_name: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_profile_res {
    pub o: hpi_profile_res_open,
    pub t: hpi_profile_res_time,
    pub n: hpi_profile_res_name,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_message_header {
    pub /: *mut *mut u16 size; / total size in bytes,
    pub /: *mut *mut u8 type; / HPI_TYPE_MESSAGE,
    pub /: *mut *mut u8 version; / message version,
    pub /: *mut *mut *mut u16 object; / HPI_OBJ_,
    pub /: *mut *mut u16 function; / HPI_SUBSYS_xxx, HPI_ADAPTER_xxx,
    pub /: *mut *mut u16 adapter_index; / the adapter index,
    pub /: *mut *mut u16 obj_index; /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_message {
// following fields must match HPI_MESSAGE_HEADER
    pub /: *mut *mut u16 size; / total size in bytes,
    pub /: *mut *mut u8 type; / HPI_TYPE_MESSAGE,
    pub /: *mut *mut u8 version; / message version,
    pub /: *mut *mut *mut u16 object; / HPI_OBJ_,
    pub /: *mut *mut u16 function; / HPI_SUBSYS_xxx, HPI_ADAPTER_xxx,
    pub /: *mut *mut u16 adapter_index; / the adapter index,
    pub /: *mut *mut u16 obj_index; /,
    pub s: hpi_subsys_msg,
    pub ax: hpi_adapterx_msg,
    pub d: hpi_stream_msg,
    pub m: hpi_mixer_msg,
    pub /: *mut *mut hpi_mixerx_msg mx; / extended mixer;,
    pub /: *mut *mut hpi_control_msg c; / mixer control;,
// identical to struct hpi_control_msg,
    pub cu: hpi_control_union_msg,
    pub n: hpi_nvmemory_msg,
    pub /: *mut *mut hpi_gpio_msg l; / digital i/o,
    pub w: hpi_watchdog_msg,
    pub /: *mut *mut hpi_clock_msg t; / dsp time,
    pub p: hpi_profile_msg,
    pub as: hpi_async_msg,
    pub fixed_size: [c_char; 32],
    pub u: },
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_response_header {
    pub size: u16,
    pub /: *mut *mut u8 type; / HPI_TYPE_RESPONSE,
    pub /: *mut *mut u8 version; / response version,
    pub /: *mut *mut *mut u16 object; / HPI_OBJ_,
    pub /: *mut *mut u16 function; / HPI_SUBSYS_xxx, HPI_ADAPTER_xxx,
    pub /: *mut *mut u16 error; / HPI_ERROR_xxx,
    pub /: *mut *mut u16 specific_error; / adapter specific error,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_response {
// following fields must match HPI_RESPONSE_HEADER
    pub size: u16,
    pub /: *mut *mut u8 type; / HPI_TYPE_RESPONSE,
    pub /: *mut *mut u8 version; / response version,
    pub /: *mut *mut *mut u16 object; / HPI_OBJ_,
    pub /: *mut *mut u16 function; / HPI_SUBSYS_xxx, HPI_ADAPTER_xxx,
    pub /: *mut *mut u16 error; / HPI_ERROR_xxx,
    pub /: *mut *mut u16 specific_error; / adapter specific error,
    pub s: hpi_subsys_res,
    pub ax: hpi_adapterx_res,
    pub d: hpi_stream_res,
    pub m: hpi_mixer_res,
    pub /: *mut *mut hpi_mixerx_res mx; / extended mixer;,
    pub /: *mut *mut hpi_control_res c; / mixer control;,
// identical to hpi_control_res, but field naming is improved
    pub cu: hpi_control_union_res,
    pub n: hpi_nvmemory_res,
    pub /: *mut *mut hpi_gpio_res l; / digital i/o,
    pub w: hpi_watchdog_res,
    pub /: *mut *mut hpi_clock_res t; / dsp time,
    pub p: hpi_profile_res,
    pub as: hpi_async_res,
    pub bytes: [u8; 52],
    pub u: },
}

// version 1 message/response

// New style message/response, but still V0 compatible
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_adapter_get_info {
    pub h: hpi_message_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_res_adapter_get_info {
    pub /: *mut *mut hpi_response_header h; /v0,
    pub p: hpi_adapter_res,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_res_adapter_debug_read {
    pub h: hpi_response_header,
    pub bytes: [u8; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmi {
    pub attribute: u16,
    pub padding: u16,
    pub hmi_address: u32,
    pub byte_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmiwrite {
    pub h: hpi_message_header,
    pub p: hpi_msg_cobranet_hmi,
    pub bytes: [u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_cobranet_hmiread {
    pub h: hpi_message_header,
    pub p: hpi_msg_cobranet_hmi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_res_cobranet_hmiread {
    pub h: hpi_response_header,
    pub byte_count: u32,
    pub bytes: [u8; 256],
}

// V1 headers in Addition to v0 headers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_message_header_v1 {
    pub h0: hpi_message_header,
// struct {
    pub /: *mut } h1;,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_response_header_v1 {
    pub h0: hpi_response_header,
    pub /: *mut *mut u16 adapter_index; / the adapter index,
    pub /: *mut *mut u16 obj_index; / object index,
    pub h1: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_msg_payload_v0 {
    pub h: hpi_message_header,
    pub s: hpi_subsys_msg,
    pub ax: hpi_adapterx_msg,
    pub d: hpi_stream_msg,
    pub m: hpi_mixer_msg,
    pub mx: hpi_mixerx_msg,
    pub c: hpi_control_msg,
    pub cu: hpi_control_union_msg,
    pub n: hpi_nvmemory_msg,
    pub l: hpi_gpio_msg,
    pub w: hpi_watchdog_msg,
    pub t: hpi_clock_msg,
    pub p: hpi_profile_msg,
    pub as: hpi_async_msg,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_res_payload_v0 {
    pub h: hpi_response_header,
    pub s: hpi_subsys_res,
    pub ax: hpi_adapterx_res,
    pub d: hpi_stream_res,
    pub m: hpi_mixer_res,
    pub mx: hpi_mixerx_res,
    pub c: hpi_control_res,
    pub cu: hpi_control_union_res,
    pub n: hpi_nvmemory_res,
    pub l: hpi_gpio_res,
    pub w: hpi_watchdog_res,
    pub t: hpi_clock_res,
    pub p: hpi_profile_res,
    pub as: hpi_async_res,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_message_buffer_v1 {
    pub /: *mut *mut hpi_message m0; / version 0,
    pub h: hpi_message_header_v1,
    pub buf: [u8; HPI_MAX_PAYLOAD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_response_buffer_v1 {
    pub /: *mut *mut hpi_response r0; / version 0,
    pub h: hpi_response_header_v1,
    pub buf: [u8; HPI_MAX_PAYLOAD_SIZE],
}

//
// declarations for compact control calls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_defn {
    pub type: u8,
    pub channels: u8,
    pub src_node_type: u8,
    pub src_node_index: u8,
    pub dest_node_type: u8,
    pub dest_node_index: u8,
}

//
// declarations for control caching (internal to HPI<->DSP interaction)
// indicates a cached u16 value is invalid.
pub const HPI_CACHE_INVALID_UINT16: c_uint = 0xFFFF;
// indicates a cached short value is invalid.

// A compact representation of (part of) a controls state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_info {
// one of HPI_CONTROL_*
    pub control_type: u8,
// The total size of cached information in 32-bit words.
    pub size_in32bit_words: u8,
// The original index of the control on the DSP
    pub control_index: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_vol {
    pub i: hpi_control_cache_info,
    pub an_log: [c_short; 2],
    pub flags: c_ushort,
    pub padding: [c_char; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_meter {
    pub i: hpi_control_cache_info,
    pub an_log_peak: [c_short; 2],
    pub an_logRMS: [c_short; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_channelmode {
    pub i: hpi_control_cache_info,
    pub mode: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_mux {
    pub i: hpi_control_cache_info,
    pub source_node_type: u16,
    pub source_node_index: u16,
    pub temp_padding: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_level {
    pub i: hpi_control_cache_info,
    pub an_log: [c_short; 2],
    pub temp_padding: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_tuner {
    pub i: hpi_control_cache_info,
    pub freq_ink_hz: u32,
    pub band: u16,
    pub s_level_avg: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_aes3rx {
    pub i: hpi_control_cache_info,
    pub error_status: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_aes3tx {
    pub i: hpi_control_cache_info,
    pub format: u32,
    pub temp_padding: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_tonedetector {
    pub i: hpi_control_cache_info,
    pub state: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_silencedetector {
    pub i: hpi_control_cache_info,
    pub state: u32,
    pub temp_padding: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_sampleclock {
    pub i: hpi_control_cache_info,
    pub source: u16,
    pub source_index: u16,
    pub sample_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_microphone {
    pub i: hpi_control_cache_info,
    pub phantom_state: u16,
    pub temp_padding: [c_char; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_single {
    pub i: hpi_control_cache_info,
    pub vol: hpi_control_cache_vol,
    pub meter: hpi_control_cache_meter,
    pub mode: hpi_control_cache_channelmode,
    pub mux: hpi_control_cache_mux,
    pub level: hpi_control_cache_level,
    pub tuner: hpi_control_cache_tuner,
    pub aes3rx: hpi_control_cache_aes3rx,
    pub aes3tx: hpi_control_cache_aes3tx,
    pub tone: hpi_control_cache_tonedetector,
    pub silence: hpi_control_cache_silencedetector,
    pub clk: hpi_control_cache_sampleclock,
    pub microphone: hpi_control_cache_microphone,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache_pad {
    pub i: hpi_control_cache_info,
    pub field_valid_flags: u32,
    pub c_channel: [u8; 40],
    pub c_artist: [u8; 100],
    pub c_title: [u8; 100],
    pub c_comment: [u8; 200],
    pub pTY: u32,
    pub pI: u32,
    pub traffic_supported: u32,
    pub traffic_anouncement: u32,
}

// 2^N sized FIFO buffer (internal to HPI<->DSP interaction)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_fifo_buffer {
    pub size: u32,
    pub dsp_index: u32,
    pub host_index: u32,
}

// skip host side function declarations for DSP
extern "C" {
    pub fn hpi_handle_object(handle: u32) -> c_char;
}
//
// main HPI entry point
extern "C" {
    pub fn hpi_send_recv(phm: *mut hpi_message, phr: *mut hpi_response);
}
// used in PnP OS/driver
//
extern "C" {
    pub fn hpi_stream_response_to_legacy(pSR: *mut hpi_stream_res);
}
//
// declarations for individual HPI entry points
