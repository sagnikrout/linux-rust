//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ngene/ngene.h
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
// ngene.h: nGene PCIe bridge driver
//
// Copyright (C) 2005-2007 Micronas
//

pub const NGENE_VID: c_uint = 0x18c3;
pub const NGENE_PID: c_uint = 0x0720;

pub const VIDEO_CAP_AVC: c_int = 128;
pub const VIDEO_CAP_H264: c_int = 128;
pub const VIDEO_CAP_VC1: c_int = 256;
pub const VIDEO_CAP_WMV9: c_int = 256;
pub const VIDEO_CAP_MPEG4: c_int = 512;

pub const DEMOD_TYPE_STV090X: c_int = 0;
pub const DEMOD_TYPE_DRXK: c_int = 1;
pub const DEMOD_TYPE_STV0367: c_int = 2;
pub const DEMOD_TYPE_XO2: c_int = 32;

pub const NGENE_XO2_TYPE_NONE: c_int = 0;
pub const NGENE_XO2_TYPE_DUOFLEX: c_int = 1;
pub const NGENE_XO2_TYPE_CI: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum STREAM {
    STREAM_VIDEOIN1 = 0,        /* ITU656 or TS Input */
    STREAM_VIDEOIN2,
    STREAM_AUDIOIN1,            /* I2S or SPI Input */
    STREAM_AUDIOIN2,
    STREAM_AUDIOOUT,
    MAX_STREAM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SMODE_BITS {
    SMODE_AUDIO_SPDIF = 0x20,
    SMODE_AVSYNC = 0x10,
    SMODE_TRANSPORT_STREAM = 0x08,
    SMODE_AUDIO_CAPTURE = 0x04,
    SMODE_VBI_CAPTURE = 0x02,
    SMODE_VIDEO_CAPTURE = 0x01
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum STREAM_FLAG_BITS {
    SFLAG_CHROMA_FORMAT_2COMP  = 0x01, /* Chroma Format : 2's complement */
    SFLAG_CHROMA_FORMAT_OFFSET = 0x00, /* Chroma Format : Binary offset */
    SFLAG_ORDER_LUMA_CHROMA    = 0x02, /* Byte order: Y,Cb,Y,Cr */
    SFLAG_ORDER_CHROMA_LUMA    = 0x00, /* Byte order: Cb,Y,Cr,Y */
    SFLAG_COLORBAR             = 0x04, /* Select colorbar */
}

pub const PROGRAM_ROM: c_uint = 0x0000;
pub const PROGRAM_SRAM: c_uint = 0x1000;
pub const PERIPHERALS0: c_uint = 0x8000;
pub const PERIPHERALS1: c_uint = 0x9000;
pub const SHARED_BUFFER: c_uint = 0xC000;

pub const TIMESTAMPS: c_uint = 0xA000;
pub const SCRATCHPAD: c_uint = 0xA080;
pub const FORCE_INT: c_uint = 0xA088;
pub const FORCE_NMI: c_uint = 0xA090;
pub const INT_STATUS: c_uint = 0xA0A0;
pub const DEV_VER: c_uint = 0x9004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SG_ADDR {
    pub start: u64,
    pub curr: u64,
    pub curr_ptr: u16,
    pub elements: u16,
    pub pad: [u32; 3],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SHARED_MEMORY {
// C000
    pub HostToNgene: [u32; 64],
// C100
    pub NgeneToHost: [u32; 64],
// C200
    pub NgeneCommand: u64,
    pub NgeneStatus: u64,
    pub NgeneEvent: u64,
// C210
    pub 0xc218]: u8 pad1[0xc260 -,
// C260
    pub IntCounts: u32,
    pub IntEnable: u32,
// C268
    pub 0xc268]: u8 pad2[0xd000 -,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BUFFER_STREAM_RESULTS {
    pub /: *mut *mut u32 Clock; / Stream time in 100ns units,
    pub field.: *mut *mut u16 RemainingLines; / Remaining lines in this,
    pub /: *mut *mut u8 FieldCount; / Video field number,
    pub overflow,: *mut *mut u8 Flags; / Bit 7 = Done, Bit 6 = seen, Bit 5 =,
    pub /: *mut *mut u16 BlockCount; / Audio block count (unused),
    pub Reserved: [u8; 2],
    pub DTOUpdate: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HW_SCATTER_GATHER_ELEMENT {
    pub Address: u64,
    pub Length: u32,
    pub Reserved: u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BUFFER_HEADER {
    pub Next: u64,
    pub SR: BUFFER_STREAM_RESULTS,
    pub Number_of_entries_1: u32,
    pub Reserved5: u32,
    pub Address_of_first_entry_1: u64,
    pub Number_of_entries_2: u32,
    pub Reserved7: u32,
    pub Address_of_first_entry_2: u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EVENT_BUFFER {
    pub TimeStamp: u32,
    pub GPIOStatus: u8,
    pub UARTStatus: u8,
    pub RXCharacter: u8,
    pub EventStatus: u8,
    pub Reserved: [u32; 2],
// C attribute field omitted
// Firmware commands.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OPCODES {
    CMD_NOP = 0,
    CMD_FWLOAD_PREPARE  = 0x01,
    CMD_FWLOAD_FINISH   = 0x02,
    CMD_I2C_READ        = 0x03,
    CMD_I2C_WRITE       = 0x04,

    CMD_I2C_WRITE_NOSTOP = 0x05,
    CMD_I2C_CONTINUE_WRITE = 0x06,
    CMD_I2C_CONTINUE_WRITE_NOSTOP = 0x07,

    CMD_DEBUG_OUTPUT    = 0x09,

    CMD_CONTROL         = 0x10,
    CMD_CONFIGURE_BUFFER = 0x11,
    CMD_CONFIGURE_FREE_BUFFER = 0x12,

    CMD_SPI_READ        = 0x13,
    CMD_SPI_WRITE       = 0x14,

    CMD_MEM_READ        = 0x20,
    CMD_MEM_WRITE	    = 0x21,
    CMD_SFR_READ	    = 0x22,
    CMD_SFR_WRITE	    = 0x23,
    CMD_IRAM_READ	    = 0x24,
    CMD_IRAM_WRITE	    = 0x25,
    CMD_SET_GPIO_PIN    = 0x26,
    CMD_SET_GPIO_INT    = 0x27,
    CMD_CONFIGURE_UART  = 0x28,
    CMD_WRITE_UART      = 0x29,
    MAX_CMD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RESPONSES {
    OK = 0,
    ERROR = 1
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_HEADER {
    pub Opcode: u8,
    pub Length: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_I2C_WRITE {
    pub hdr: FW_HEADER,
    pub Device: u8,
    pub Data: [u8; 250],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_I2C_CONTINUE_WRITE {
    pub hdr: FW_HEADER,
    pub Data: [u8; 250],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_I2C_READ {
    pub hdr: FW_HEADER,
    pub Device: u8,
    pub /: *mut *mut u8 Data[252]; / followed by two bytes of read data count,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SPI_WRITE {
    pub hdr: FW_HEADER,
    pub ModeSelect: u8,
    pub Data: [u8; 250],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SPI_READ {
    pub hdr: FW_HEADER,
    pub ModeSelect: u8,
    pub /: *mut *mut u8 Data[252]; / followed by two bytes of read data count,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_FWLOAD_PREPARE {
    pub hdr: FW_HEADER,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_FWLOAD_FINISH {
    pub hdr: FW_HEADER,
    pub /: *mut *mut u16 Address; / address of final block,
    pub Length: u16,
// C attribute field omitted
//
// Meaning of FW_STREAM_CONTROL::Mode bits:
// Bit 7: Loopback PEXin to PEXout using TVOut channel
// Bit 6: AVLOOP
// Bit 5: Audio select; 0=I2S, 1=SPDIF
// Bit 4: AVSYNC
// Bit 3: Enable transport stream
// Bit 2: Enable audio capture
// Bit 1: Enable ITU-Video VBI capture
// Bit 0: Enable ITU-Video capture
//
// Meaning of FW_STREAM_CONTROL::Control bits (see UVI1_CTL)
// Bit 7: continuous capture
// Bit 6: capture one field
// Bit 5: capture one frame
// Bit 4: unused
// Bit 3: starting field; 0=odd, 1=even
// Bit 2: sample size; 0=8-bit, 1=10-bit
// Bit 1: data format; 0=UYVY, 1=YUY2
// Bit 0: resets buffer pointers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FSC_MODE_BITS {
    SMODE_LOOPBACK          = 0x80,
    SMODE_AVLOOP            = 0x40,
    _SMODE_AUDIO_SPDIF      = 0x20,
    _SMODE_AVSYNC           = 0x10,
    _SMODE_TRANSPORT_STREAM = 0x08,
    _SMODE_AUDIO_CAPTURE    = 0x04,
    _SMODE_VBI_CAPTURE      = 0x02,
    _SMODE_VIDEO_CAPTURE    = 0x01
}

// Meaning of FW_STREAM_CONTROL::Stream bits:
// Bit 3: Audio sample count:  0 = relative, 1 = absolute
// Bit 2: color bar select; 1=color bars, 0=CV3 decoder
// Bits 1-0: stream select, UVI1, UVI2, TVOUT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_STREAM_CONTROL {
    pub hdr: FW_HEADER,
    pub /: *mut *mut u8 Stream; / Stream number (UVI1, UVI2, TVOUT),
    pub /: *mut *mut u8 Control; / Value written to UVI1_CTL,
    pub /: *mut *mut u8 Mode; / Controls clock source,
    pub write: *mut *mut u8 SetupDataLen; / Length of setup data, MSB=1,
    pub buffer: *mut *mut u16 CaptureBlockCount; / Blocks (a 256 Bytes) to capture per,
    pub /: *mut *mut u64 Buffer_Address; / Address of first buffer header,
    pub BytesPerVideoLine: u16,
    pub MaxLinesPerField: u16,
    pub MinLinesPerField: u16,
    pub Reserved_1: u16,
    pub BytesPerVBILine: u16,
    pub MaxVBILinesPerField: u16,
    pub MinVBILinesPerField: u16,
    pub /: *mut *mut u16 SetupDataAddr; / ngene relative address of setup data,
    pub /: *mut *mut u8 SetupData[32]; / setup data,
    pub __attribute__((__packed__)): },
pub const AUDIO_BLOCK_SIZE: c_int = 256;
pub const TS_BLOCK_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_MEM_READ {
    pub hdr: FW_HEADER,
    pub address: u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_MEM_WRITE {
    pub hdr: FW_HEADER,
    pub address: u16,
    pub data: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SFR_IRAM_READ {
    pub hdr: FW_HEADER,
    pub address: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SFR_IRAM_WRITE {
    pub hdr: FW_HEADER,
    pub address: u8,
    pub data: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SET_GPIO_PIN {
    pub hdr: FW_HEADER,
    pub select: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SET_GPIO_INT {
    pub hdr: FW_HEADER,
    pub select: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_SET_DEBUGMODE {
    pub hdr: FW_HEADER,
    pub debug_flags: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_CONFIGURE_BUFFERS {
    pub hdr: FW_HEADER,
    pub config: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _BUFFER_CONFIGS {
// 4k UVI1, 4k UVI2, 2k AUD1, 2k AUD2  (standard usage)
    BUFFER_CONFIG_4422 = 0,
// 3k UVI1, 3k UVI2, 3k AUD1, 3k AUD2  (4x TS input usage)
    BUFFER_CONFIG_3333 = 1,
// 8k UVI1, 0k UVI2, 2k AUD1, 2k I2SOut  (HDTV decoder usage)
    BUFFER_CONFIG_8022 = 2,
    BUFFER_CONFIG_FW17 = 255, /* Use new FW 17 command */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_CONFIGURE_FREE_BUFFERS {
    pub hdr: FW_HEADER,
    pub UVI1_BufferLength: u8,
    pub UVI2_BufferLength: u8,
    pub TVO_BufferLength: u8,
    pub AUD1_BufferLength: u8,
    pub AUD2_BufferLength: u8,
    pub TVA_BufferLength: u8,
    pub config: } __packed,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_CONFIGURE_UART {
    pub hdr: FW_HEADER,
    pub UartControl: u8,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _UART_CONFIG {
    _UART_BAUDRATE_19200 = 0,
    _UART_BAUDRATE_9600  = 1,
    _UART_BAUDRATE_4800  = 2,
    _UART_BAUDRATE_2400  = 3,
    _UART_RX_ENABLE      = 0x40,
    _UART_TX_ENABLE      = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FW_WRITE_UART {
    pub hdr: FW_HEADER,
    pub Data: [u8; 252],
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ngene_command {
    pub in_len: u32,
    pub out_len: u32,
    pub raw: [u32; 64],
    pub raw8: [u8; 256],
    pub hdr: FW_HEADER,
    pub I2CWrite: FW_I2C_WRITE,
    pub I2CContinueWrite: FW_I2C_CONTINUE_WRITE,
    pub I2CRead: FW_I2C_READ,
    pub StreamControl: FW_STREAM_CONTROL,
    pub FWLoadPrepare: FW_FWLOAD_PREPARE,
    pub FWLoadFinish: FW_FWLOAD_FINISH,
    pub MemoryRead: FW_MEM_READ,
    pub MemoryWrite: FW_MEM_WRITE,
    pub SfrIramRead: FW_SFR_IRAM_READ,
    pub SfrIramWrite: FW_SFR_IRAM_WRITE,
    pub SPIWrite: FW_SPI_WRITE,
    pub SPIRead: FW_SPI_READ,
    pub SetGpioPin: FW_SET_GPIO_PIN,
    pub SetGpioInt: FW_SET_GPIO_INT,
    pub SetDebugMode: FW_SET_DEBUGMODE,
    pub ConfigureBuffers: FW_CONFIGURE_BUFFERS,
    pub ConfigureFreeBuffers: FW_CONFIGURE_FREE_BUFFERS,
    pub ConfigureUart: FW_CONFIGURE_UART,
    pub WriteUart: FW_WRITE_UART,
    pub cmd: },
// C attribute field omitted
pub const NGENE_INTERFACE_VERSION: c_uint = 0x103;

pub const RING_SIZE_VIDEO: c_int = 4;
pub const RING_SIZE_AUDIO: c_int = 8;
pub const RING_SIZE_TS: c_int = 8;
pub const NUM_SCATTER_GATHER_ENTRIES: c_int = 8;

pub const EVENT_QUEUE_SIZE: c_int = 16;
// Gathers the current state of a single channel.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SBufferHeader {
    pub /: *mut *mut BUFFER_HEADER ngeneBuffer; / Physical descriptor,
    pub Next: *mut SBufferHeader,
    pub Buffer1: *mut c_void,
    pub scList1: *mut HW_SCATTER_GATHER_ELEMENT,
    pub Buffer2: *mut c_void,
    pub scList2: *mut HW_SCATTER_GATHER_ELEMENT,
}

// Sizeof SBufferHeader aligned to next 64 Bit boundary (hw restriction)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HWSTATE {
    HWSTATE_STOP,
    HWSTATE_STARTUP,
    HWSTATE_RUN,
    HWSTATE_PAUSE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KSSTATE {
    KSSTATE_STOP,
    KSSTATE_ACQUIRE,
    KSSTATE_PAUSE,
    KSSTATE_RUN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SRingBufferDescriptor {
    pub buffer: *mut *mut *mut SBufferHeader Head; / Points to first buffer in ring,
    pub /: *mut *mut u64 PAHead; / Physical address of first buffer,
    pub buffers: *mut *mut u32 MemSize; / Memory size of allocated ring,
    pub /: *mut *mut u32 NumBuffers; / Number of buffers in the ring,
    pub /: *mut *mut u32 Buffer1Length; / Allocated length of Buffer 1,
    pub /: *mut *mut u32 Buffer2Length; / Allocated length of Buffer 2,
    pub this: *mut *mut *mut void SCListMem; / Memory to hold scatter gather lists for,
    pub /: *mut *mut u64 PASCListMem; / Physical address ..,
    pub /: *mut *mut u32 SCListMemSize; / Size of this memory,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum STREAMMODEFLAGS {
    StreamMode_NONE   = 0, /* Stream not used */
    StreamMode_ANALOG = 1, /* Analog: Stream 0,1 = Video, 2,3 = Audio */
    StreamMode_TSIN   = 2, /* Transport stream input (all) */
    StreamMode_HDTV   = 4, /* HDTV: Maximum 1920x1080p30,1920x1080i60
    (only stream 0) */
    StreamMode_TSOUT  = 8, /* Transport stream output (only stream 3) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BufferExchangeFlags {
    BEF_EVEN_FIELD   = 0x00000001,
    BEF_CONTINUATION = 0x00000002,
    BEF_MORE_DATA    = 0x00000004,
    BEF_OVERFLOW     = 0x00000008,
    DF_SWAP32        = 0x00010000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MICI_STREAMINFO {
    pub pExchange: *mut IBufferExchange,
    pub /: *mut *mut *mut IBufferExchange pExchangeVBI; / Secondary (VBI, ancillary),
    pub Stream: u8,
    pub Flags: u8,
    pub Mode: u8,
    pub Reserved: u8,
    pub nLinesVideo: u16,
    pub nBytesPerLineVideo: u16,
    pub nLinesVBI: u16,
    pub nBytesPerLineVBI: u16,
    pub /: *mut *mut u32 CaptureLength; / Used for audio and transport stream,
}

//
// STRUCTS
//
// sound hardware definition
pub const MIXER_ADDR_TVTUNER: c_int = 0;
pub const MIXER_ADDR_LAST: c_int = 0;
// struct sound chip
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mychip {
    pub chan: *mut ngene_channel,
    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub substream: *mut snd_pcm_substream,
    pub pcm: *mut snd_pcm,
    pub port: c_ulong,
    pub irq: c_int,
    pub mixer_lock: spinlock_t,
    pub lock: spinlock_t,
    pub 1][2]: int mixer_volume[MIXER_ADDR_LAST +,
    pub 1][2]: int capture_source[MIXER_ADDR_LAST +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ngene_channel {
    pub device: device,
    pub i2c_adapter: i2c_adapter,
    pub i2c_client: [*mut i2c_client; 1],
    pub i2c_client_fe: c_int,
    pub dev: *mut ngene,
    pub number: c_int,
    pub type: c_int,
    pub mode: c_int,
    pub has_adapter: bool,
    pub has_demux: bool,
    pub demod_type: c_int,
    pub int): *mut *mut *mut int (gate_ctrl)(struct dvb_frontend ,,
    pub fe: *mut dvb_frontend,
    pub fe2: *mut dvb_frontend,
    pub dmxdev: dmxdev,
    pub demux: dvb_demux,
    pub dvbnet: dvb_net,
    pub hw_frontend: dmx_frontend,
    pub mem_frontend: dmx_frontend,
    pub users: c_int,
    pub v4l_dev: *mut video_device,
    pub ci_dev: *mut dvb_device,
    pub demux_bh_work: work_struct,
    pub nextBuffer: *mut SBufferHeader,
    pub State: KSSTATE,
    pub HWState: HWSTATE,
    pub Stream: u8,
    pub Flags: u8,
    pub Mode: u8,
    pub pBufferExchange: *mut IBufferExchange,
    pub pBufferExchange2: *mut IBufferExchange,
    pub state_lock: spinlock_t,
    pub nLines: u16,
    pub nBytesPerLine: u16,
    pub nVBILines: u16,
    pub nBytesPerVBILine: u16,
    pub itumode: u16,
    pub Capture1Length: u32,
    pub Capture2Length: u32,
    pub RingBuffer: SRingBufferDescriptor,
    pub TSRingBuffer: SRingBufferDescriptor,
    pub TSIdleBuffer: SRingBufferDescriptor,
    pub DataFormatFlags: u32,
    pub AudioDTOUpdated: c_int,
    pub AudioDTOValue: u32,
    pub fe_sec_tone_mode): *mut *mut *mut int (set_tone)(struct dvb_frontend , enum,
    pub lnbh: u8,
// stuff from analog driver
    pub minor: c_int,
    pub mychip: *mut mychip,
    pub soundcard: *mut snd_card,
    pub evenbuffer: *mut u8,
    pub dma_on: u8,
    pub soundstreamon: c_int,
    pub audiomute: c_int,
    pub soundbuffisallocated: c_int,
    pub sndbuffflag: c_int,
    pub tun_rdy: c_int,
    pub dec_rdy: c_int,
    pub tun_dec_rdy: c_int,
    pub lastbufferflag: c_int,
    pub tvnorms: *mut ngene_tvnorm,
    pub tvnorm_num: c_int,
    pub tvnorm: c_int,
    pub running: c_int,
    pub tsin_offset: c_int,
    pub tsin_buffer: [u8; 188],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ngene_ci {
    pub device: device,
    pub i2c_adapter: i2c_adapter,
    pub dev: *mut ngene,
    pub en: *mut dvb_ca_en50221,
}

extern "C" {
    pub fn void(: *mut rx_cb_t)(struct ngene, _arg: u32, _arg: u8) -> typedef;
}
extern "C" {
    pub fn void(: *mut tx_cb_t)(struct ngene, _arg: u32) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ngene {
    pub nr: c_int,
    pub pci_dev: *mut pci_dev,
    pub iomem: *mut unsigned char __iomem,
// struct i2c_adapter  i2c_adapter;
    pub device_version: u32,
    pub fw_interface_version: u32,
    pub icounts: u32,
    pub msi_enabled: bool,
    pub cmd_timeout_workaround: bool,
    pub CmdDoneByte: *mut u8,
    pub BootFirmware: c_int,
    pub OverflowBuffer: *mut c_void,
    pub PAOverflowBuffer: dma_addr_t,
    pub FWInterfaceBuffer: *mut c_void,
    pub PAFWInterfaceBuffer: dma_addr_t,
    pub ngenetohost: *mut u8,
    pub hosttongene: *mut u8,
    pub EventQueue: [EVENT_BUFFER; EVENT_QUEUE_SIZE],
    pub EventQueueOverflowCount: c_int,
    pub EventQueueOverflowFlag: c_int,
    pub event_bh_work: work_struct,
    pub EventBuffer: *mut EVENT_BUFFER,
    pub EventQueueWriteIndex: c_int,
    pub EventQueueReadIndex: c_int,
    pub cmd_wq: wait_queue_head_t,
    pub cmd_done: c_int,
    pub cmd_mutex: mutex,
    pub stream_mutex: mutex,
    pub pll_mutex: semaphore,
    pub i2c_switch_mutex: mutex,
    pub i2c_current_channel: c_int,
    pub i2c_current_bus: c_int,
    pub cmd_lock: spinlock_t,
    pub adapter: [dvb_adapter; MAX_STREAM],
    pub /: *mut *mut *mut dvb_adapter first_adapter; / "one_adapter" modprobe opt,
    pub channel: [ngene_channel; MAX_STREAM],
    pub card_info: *mut ngene_info,
    pub TxEventNotify: *mut tx_cb_t,
    pub RxEventNotify: *mut rx_cb_t,
    pub tx_busy: c_int,
    pub tx_wq: wait_queue_head_t,
    pub rx_wq: wait_queue_head_t,
pub const UART_RBUF_LEN: c_int = 4096;
    pub uart_rbuf: [u8; UART_RBUF_LEN],
    pub uart_wp: int uart_rp,,
pub const TS_FILLER: c_uint = 0x6f;
    pub tsout_buf: *mut u8,

    pub tsout_rbuf: dvb_ringbuffer,
    pub tsin_buf: *mut u8,

    pub tsin_rbuf: dvb_ringbuffer,
    pub ain_buf: *mut u8,

    pub ain_rbuf: dvb_ringbuffer,
    pub vin_buf: *mut u8,

    pub vin_rbuf: dvb_ringbuffer,
    pub exp_val: c_ulong,
    pub prev_cmd: c_int,
    pub ci: ngene_ci,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ngene_info {
    pub type: c_int,
pub const NGENE_APP: c_int = 0;
pub const NGENE_TERRATEC: c_int = 1;
pub const NGENE_SIDEWINDER: c_int = 2;
pub const NGENE_RACER: c_int = 3;
pub const NGENE_VIPER: c_int = 4;
pub const NGENE_PYTHON: c_int = 5;
pub const NGENE_VBOX_V1: c_int = 6;
pub const NGENE_VBOX_V2: c_int = 7;
    pub fw_version: c_int,
    pub msi_supported: bool,
    pub name: *mut c_char,
    pub io_type: [c_int; MAX_STREAM],
pub const NGENE_IO_NONE: c_int = 0;
pub const NGENE_IO_TV: c_int = 1;
pub const NGENE_IO_HDTV: c_int = 2;
pub const NGENE_IO_TSIN: c_int = 4;
pub const NGENE_IO_TSOUT: c_int = 8;
pub const NGENE_IO_AIN: c_int = 16;
    pub fe_config: [*mut c_void; 4],
    pub tuner_config: [*mut c_void; 4],
    pub ): *mut *mut int (demod_attach[4])(struct ngene_channel,
    pub ): *mut *mut int (tuner_attach[4])(struct ngene_channel,
    pub avf: [u8; 4],
    pub msp: [u8; 4],
    pub demoda: [u8; 4],
    pub lnb: [u8; 4],
    pub i2c_access: c_int,
    pub ntsc: u8,
    pub tsf: [u8; 4],
    pub i2s: [u8; 4],
    pub int): *mut *mut *mut int (gate_ctrl)(struct dvb_frontend ,,
    pub int): *mut *mut *mut int (switch_ctrl)(struct ngene_channel , int,,
}

// Provided by ngene-core.c
extern "C" {
    pub fn ngene_probe(pci_dev: *mut pci_dev, id: *const pci_device_id) -> c_int;
}
extern "C" {
    pub fn ngene_remove(pdev: *mut pci_dev);
}
extern "C" {
    pub fn ngene_shutdown(pdev: *mut pci_dev);
}
extern "C" {
    pub fn ngene_command(dev: *mut ngene, com: *mut ngene_command) -> c_int;
}
extern "C" {
    pub fn ngene_command_gpio_set(dev: *mut ngene, select: u8, level: u8) -> c_int;
}
extern "C" {
    pub fn set_transfer(chan: *mut ngene_channel, state: c_int);
}
extern "C" {
    pub fn FillTSBuffer(Buffer: *mut c_void, Length: c_int, Flags: u32);
}
// Provided by ngene-cards.c
extern "C" {
    pub fn ngene_port_has_cxd2099(i2c: *mut i2c_adapter, type: *mut u8) -> c_int;
}
// Provided by ngene-i2c.c
extern "C" {
    pub fn ngene_i2c_init(dev: *mut ngene, dev_nr: c_int) -> c_int;
}
// Provided by ngene-dvb.c
extern "C" {
    pub fn ngene_start_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int;
}
extern "C" {
    pub fn ngene_stop_feed(dvbdmxfeed: *mut dvb_demux_feed) -> c_int;
}

// LocalWords:  Endif
//
