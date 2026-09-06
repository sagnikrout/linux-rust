//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx88/cx88.h
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
// v4l2 device driver for cx2388x based TV cards
//
// (c) 2003,04 Gerd Knorr <kraxel@bytesex.org> [SUSE Labs]
//

pub const CX88_MAXBOARDS: c_int = 8;
// Max number of inputs by card
pub const MAX_CX88_INPUT: c_int = 8;
// -----------------------------------------------------------
// defines and enums
// Currently unsupported by the driver: PAL/H, NTSC/Kr, SECAM/LC

pub const FORMAT_FLAGS_PACKED: c_uint = 0x01;
pub const FORMAT_FLAGS_PLANAR: c_uint = 0x02;
pub const VBI_LINE_PAL_COUNT: c_int = 18;
pub const VBI_LINE_NTSC_COUNT: c_int = 12;
pub const VBI_LINE_LENGTH: c_int = 2048;
pub const AUD_RDS_LINES: c_int = 4;
// need "shadow" registers for some write-only ones ...
pub const SHADOW_AUD_VOL_CTL: c_int = 1;
pub const SHADOW_AUD_BAL_CTL: c_int = 2;
pub const SHADOW_MAX: c_int = 3;
// FM Radio deemphasis type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx88_deemph_type {
    FM_NO_DEEMPH = 0,
    FM_DEEMPH_50,
    FM_DEEMPH_75
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx88_board_type {
    CX88_BOARD_NONE = 0,
    CX88_MPEG_DVB,
    CX88_MPEG_BLACKBIRD
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx8802_board_access {
    CX8802_DRVCTL_SHARED    = 1,
    CX8802_DRVCTL_EXCLUSIVE = 2,
}

// -----------------------------------------------------------
// tv norms
// -----------------------------------------------------------
// static data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8800_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub flags: c_int,
    pub cxformat: u32,
}

// -----------------------------------------------------------
// SRAM memory management data (see cx88-core.c)

pub const SRAM_CH22: c_int = 1;
pub const SRAM_CH23: c_int = 2;

pub const SRAM_CH26: c_int = 5;

// more
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_channel {
    pub name: *const c_char,
    pub cmds_start: u32,
    pub ctrl_start: u32,
    pub cdt: u32,
    pub fifo_start: u32,
    pub fifo_size: u32,
    pub ptr1_reg: u32,
    pub ptr2_reg: u32,
    pub cnt1_reg: u32,
    pub cnt2_reg: u32,
}

// -----------------------------------------------------------
// card configuration

pub const CX88_BOARD_UNKNOWN: c_int = 0;
pub const CX88_BOARD_HAUPPAUGE: c_int = 1;
pub const CX88_BOARD_GDI: c_int = 2;
pub const CX88_BOARD_PIXELVIEW: c_int = 3;
pub const CX88_BOARD_ATI_WONDER_PRO: c_int = 4;
pub const CX88_BOARD_WINFAST2000XP_EXPERT: c_int = 5;
pub const CX88_BOARD_AVERTV_STUDIO_303: c_int = 6;
pub const CX88_BOARD_MSI_TVANYWHERE_MASTER: c_int = 7;
pub const CX88_BOARD_WINFAST_DV2000: c_int = 8;
pub const CX88_BOARD_LEADTEK_PVR2000: c_int = 9;
pub const CX88_BOARD_IODATA_GVVCP3PCI: c_int = 10;
pub const CX88_BOARD_PROLINK_PLAYTVPVR: c_int = 11;
pub const CX88_BOARD_ASUS_PVR_416: c_int = 12;
pub const CX88_BOARD_MSI_TVANYWHERE: c_int = 13;
pub const CX88_BOARD_KWORLD_DVB_T: c_int = 14;
pub const CX88_BOARD_DVICO_FUSIONHDTV_DVB_T1: c_int = 15;
pub const CX88_BOARD_KWORLD_LTV883: c_int = 16;
pub const CX88_BOARD_DVICO_FUSIONHDTV_3_GOLD_Q: c_int = 17;
pub const CX88_BOARD_HAUPPAUGE_DVB_T1: c_int = 18;
pub const CX88_BOARD_CONEXANT_DVB_T1: c_int = 19;
pub const CX88_BOARD_PROVIDEO_PV259: c_int = 20;
pub const CX88_BOARD_DVICO_FUSIONHDTV_DVB_T_PLUS: c_int = 21;
pub const CX88_BOARD_PCHDTV_HD3000: c_int = 22;
pub const CX88_BOARD_DNTV_LIVE_DVB_T: c_int = 23;
pub const CX88_BOARD_HAUPPAUGE_ROSLYN: c_int = 24;
pub const CX88_BOARD_DIGITALLOGIC_MEC: c_int = 25;
pub const CX88_BOARD_IODATA_GVBCTV7E: c_int = 26;
pub const CX88_BOARD_PIXELVIEW_PLAYTV_ULTRA_PRO: c_int = 27;
pub const CX88_BOARD_DVICO_FUSIONHDTV_3_GOLD_T: c_int = 28;
pub const CX88_BOARD_ADSTECH_DVB_T_PCI: c_int = 29;
pub const CX88_BOARD_TERRATEC_CINERGY_1400_DVB_T1: c_int = 30;
pub const CX88_BOARD_DVICO_FUSIONHDTV_5_GOLD: c_int = 31;
pub const CX88_BOARD_AVERMEDIA_ULTRATV_MC_550: c_int = 32;
pub const CX88_BOARD_KWORLD_VSTREAM_EXPERT_DVD: c_int = 33;
pub const CX88_BOARD_ATI_HDTVWONDER: c_int = 34;
pub const CX88_BOARD_WINFAST_DTV1000: c_int = 35;
pub const CX88_BOARD_AVERTV_303: c_int = 36;
pub const CX88_BOARD_HAUPPAUGE_NOVASPLUS_S1: c_int = 37;
pub const CX88_BOARD_HAUPPAUGE_NOVASE2_S1: c_int = 38;
pub const CX88_BOARD_KWORLD_DVBS_100: c_int = 39;
pub const CX88_BOARD_HAUPPAUGE_HVR1100: c_int = 40;
pub const CX88_BOARD_HAUPPAUGE_HVR1100LP: c_int = 41;
pub const CX88_BOARD_DNTV_LIVE_DVB_T_PRO: c_int = 42;
pub const CX88_BOARD_KWORLD_DVB_T_CX22702: c_int = 43;
pub const CX88_BOARD_DVICO_FUSIONHDTV_DVB_T_DUAL: c_int = 44;
pub const CX88_BOARD_KWORLD_HARDWARE_MPEG_TV_XPERT: c_int = 45;
pub const CX88_BOARD_DVICO_FUSIONHDTV_DVB_T_HYBRID: c_int = 46;
pub const CX88_BOARD_PCHDTV_HD5500: c_int = 47;
pub const CX88_BOARD_KWORLD_MCE200_DELUXE: c_int = 48;
pub const CX88_BOARD_PIXELVIEW_PLAYTV_P7000: c_int = 49;
pub const CX88_BOARD_NPGTECH_REALTV_TOP10FM: c_int = 50;
pub const CX88_BOARD_WINFAST_DTV2000H: c_int = 51;
pub const CX88_BOARD_GENIATECH_DVBS: c_int = 52;
pub const CX88_BOARD_HAUPPAUGE_HVR3000: c_int = 53;
pub const CX88_BOARD_NORWOOD_MICRO: c_int = 54;
pub const CX88_BOARD_TE_DTV_250_OEM_SWANN: c_int = 55;
pub const CX88_BOARD_HAUPPAUGE_HVR1300: c_int = 56;
pub const CX88_BOARD_ADSTECH_PTV_390: c_int = 57;
pub const CX88_BOARD_PINNACLE_PCTV_HD_800i: c_int = 58;
pub const CX88_BOARD_DVICO_FUSIONHDTV_5_PCI_NANO: c_int = 59;
pub const CX88_BOARD_PINNACLE_HYBRID_PCTV: c_int = 60;
pub const CX88_BOARD_WINFAST_TV2000_XP_GLOBAL: c_int = 61;
pub const CX88_BOARD_POWERCOLOR_REAL_ANGEL: c_int = 62;
pub const CX88_BOARD_GENIATECH_X8000_MT: c_int = 63;
pub const CX88_BOARD_DVICO_FUSIONHDTV_DVB_T_PRO: c_int = 64;
pub const CX88_BOARD_DVICO_FUSIONHDTV_7_GOLD: c_int = 65;
pub const CX88_BOARD_PROLINK_PV_8000GT: c_int = 66;
pub const CX88_BOARD_KWORLD_ATSC_120: c_int = 67;
pub const CX88_BOARD_HAUPPAUGE_HVR4000: c_int = 68;
pub const CX88_BOARD_HAUPPAUGE_HVR4000LITE: c_int = 69;
pub const CX88_BOARD_TEVII_S460: c_int = 70;
pub const CX88_BOARD_OMICOM_SS4_PCI: c_int = 71;
pub const CX88_BOARD_TBS_8920: c_int = 72;
pub const CX88_BOARD_TEVII_S420: c_int = 73;
pub const CX88_BOARD_PROLINK_PV_GLOBAL_XTREME: c_int = 74;
pub const CX88_BOARD_PROF_7300: c_int = 75;
pub const CX88_BOARD_SATTRADE_ST4200: c_int = 76;
pub const CX88_BOARD_TBS_8910: c_int = 77;
pub const CX88_BOARD_PROF_6200: c_int = 78;
pub const CX88_BOARD_TERRATEC_CINERGY_HT_PCI_MKII: c_int = 79;
pub const CX88_BOARD_HAUPPAUGE_IRONLY: c_int = 80;
pub const CX88_BOARD_WINFAST_DTV1800H: c_int = 81;
pub const CX88_BOARD_WINFAST_DTV2000H_J: c_int = 82;
pub const CX88_BOARD_PROF_7301: c_int = 83;
pub const CX88_BOARD_SAMSUNG_SMT_7020: c_int = 84;
pub const CX88_BOARD_TWINHAN_VP1027_DVBS: c_int = 85;
pub const CX88_BOARD_TEVII_S464: c_int = 86;
pub const CX88_BOARD_WINFAST_DTV2000H_PLUS: c_int = 87;
pub const CX88_BOARD_WINFAST_DTV1800H_XC4000: c_int = 88;
pub const CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F36: c_int = 89;
pub const CX88_BOARD_WINFAST_TV2000_XP_GLOBAL_6F43: c_int = 90;
pub const CX88_BOARD_NOTONLYTV_LV3H: c_int = 91;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx88_itype {
    CX88_VMUX_COMPOSITE1 = 1,
    CX88_VMUX_COMPOSITE2,
    CX88_VMUX_COMPOSITE3,
    CX88_VMUX_COMPOSITE4,
    CX88_VMUX_SVIDEO,
    CX88_VMUX_TELEVISION,
    CX88_VMUX_CABLE,
    CX88_VMUX_DVB,
    CX88_VMUX_DEBUG,
    CX88_RADIO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_input {
    pub type: cx88_itype,
    pub gpio3: u32 gpio0, gpio1, gpio2,,
    pub vmux:2: c_uint,
    pub audioroute:4: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx88_audio_chip {
    CX88_AUDIO_WM8775 = 1,
    CX88_AUDIO_TVAUDIO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_board {
    pub name: *const c_char,
    pub tuner_type: c_uint,
    pub radio_type: c_uint,
    pub tuner_addr: c_uchar,
    pub radio_addr: c_uchar,
    pub tda9887_conf: c_int,
    pub input: [cx88_input; MAX_CX88_INPUT],
    pub radio: cx88_input,
    pub mpeg: cx88_board_type,
    pub audio_chip: cx88_audio_chip,
    pub num_frontends: c_int,
// Used for I2S devices
    pub i2sinputcntl: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_subid {
    pub subvendor: u16,
    pub subdevice: u16,
    pub card: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx88_tvaudio {
    WW_NONE = 1,
    WW_BTSC,
    WW_BG,
    WW_DK,
    WW_I,
    WW_L,
    WW_EIAJ,
    WW_I2SPT,
    WW_FM,
    WW_I2SADC,
    WW_M
}

// -----------------------------------------------------------
// device / file handle status
pub const RESOURCE_OVERLAY: c_int = 1;
pub const RESOURCE_VIDEO: c_int = 2;
pub const RESOURCE_VBI: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_riscmem {
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub jmp: *mut __le32,
    pub dma: dma_addr_t,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
// cx88 specific
    pub bpl: c_uint,
    pub risc: cx88_riscmem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_dmaqueue {
    pub active: list_head,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx88_core {
    pub devlist: list_head,
    pub refcount: refcount_t,
// board name
    pub nr: c_int,
    pub name: [c_char; 32],
    pub model: u32,
// pci stuff
    pub pci_bus: c_int,
    pub pci_slot: c_int,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
    pub shadow: [u32; SHADOW_MAX],
    pub pci_irqmask: c_int,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_algo: i2c_algo_bit_data,
    pub i2c_client: i2c_client,
    pub i2c_rc: u32 i2c_state,,
// config info -- analog
    pub v4l2_dev: v4l2_device,
    pub video_hdl: v4l2_ctrl_handler,
    pub chroma_agc: *mut v4l2_ctrl,
    pub audio_hdl: v4l2_ctrl_handler,
    pub sd_wm8775: *mut v4l2_subdev,
    pub i2c_rtc: *mut i2c_client,
    pub boardnr: c_uint,
    pub board: cx88_board,
// Supported V4L _STD_ tuner formats
    pub tuner_formats: c_uint,
// config info -- dvb

    pub voltage): fe_sec_voltage,

    pub open): *mut *mut *mut void (gate_ctrl)(struct cx88_core core, int,
// state info
    pub kthread: *mut task_struct,
    pub tvnorm: v4l2_std_id,
    pub height: unsigned int width,,
    pub field: c_uint,
    pub tvaudio: cx88_tvaudio,
    pub audiomode_manual: u32,
    pub audiomode_current: u32,
    pub input: u32,
    pub last_analog_input: u32,
    pub astat: u32,
    pub use_nicam: u32,
    pub last_change: c_ulong,
// IR remote control state
    pub ir: *mut cx88_IR,
// I2C remote data
    pub init_data: IR_i2c_init_data,
    pub wm8775_data: wm8775_platform_data,
    pub lock: mutex,
// various v4l controls
    pub freq: u32,
//
// cx88-video needs to access cx8802 for hybrid tuner pll access and
// for vb2_is_busy() checks.
//
    pub dvbdev: *mut cx8802_dev,
// cx88-blackbird needs to access cx8800 for vb2_is_busy() checks
    pub v4ldev: *mut cx8800_dev,
    pub active_type_id: cx88_board_type,
    pub active_ref: c_int,
    pub active_fe_id: c_int,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, cx88_core: struct, _arg: v4l2_dev) -> return;
}

// -----------------------------------------------------------
// function 0: video stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8800_suspend_state {
    pub disabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8800_dev {
    pub core: *mut cx88_core,
    pub slock: spinlock_t,
// various device info
    pub resources: c_uint,
    pub video_dev: video_device,
    pub vbi_dev: video_device,
    pub radio_dev: video_device,
// pci i/o
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
    pub fmt: *const cx8800_fmt,
// capture queues
    pub vidq: cx88_dmaqueue,
    pub vb2_vidq: vb2_queue,
    pub vbiq: cx88_dmaqueue,
    pub vb2_vbiq: vb2_queue,
// various v4l controls
// other global state info
    pub state: cx8800_suspend_state,
}

// -----------------------------------------------------------
// function 1: audio/alsa stuff
// =============> moved to cx88-alsa.c <======================
// -----------------------------------------------------------
// function 2: mpeg stuff
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8802_suspend_state {
    pub disabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8802_driver {
    pub core: *mut cx88_core,
// List of drivers attached to device
    pub drvlist: list_head,
// Type of driver and access required
    pub type_id: cx88_board_type,
    pub hw_access: cx8802_board_access,
// MPEG 8802 internal only
    pub state): *mut *mut *mut int (suspend)(struct pci_dev pci_dev, pm_message_t,
    pub pci_dev): *mut *mut int (resume)(struct pci_dev,
// Callers to the following functions must hold core->lock
// MPEG 8802 -> mini driver - Driver probe and configuration
    pub drv): *mut *mut int (probe)(struct cx8802_driver,
    pub drv): *mut *mut int (remove)(struct cx8802_driver,
// MPEG 8802 -> mini driver - Access for hardware control
    pub drv): *mut *mut int (advise_acquire)(struct cx8802_driver,
    pub drv): *mut *mut int (advise_release)(struct cx8802_driver,
// MPEG 8802 <- mini driver - Access for hardware control
    pub drv): *mut *mut int (request_acquire)(struct cx8802_driver,
    pub drv): *mut *mut int (request_release)(struct cx8802_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx8802_dev {
    pub core: *mut cx88_core,
    pub slock: spinlock_t,
// pci i/o
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
// dma queues
    pub mpegq: cx88_dmaqueue,
    pub vb2_mpegq: vb2_queue,
    pub ts_packet_size: u32,
    pub ts_packet_count: u32,
// other global state info
    pub state: cx8802_suspend_state,
// for blackbird only
    pub devlist: list_head,

    pub mpeg_dev: video_device,
    pub mailbox: u32,
// mpeg params
    pub cxhdl: cx2341x_handler,

// for dvb only
    pub frontends: vb2_dvb_frontends,

// For VP3045 secondary I2C bus support
    pub vp3054: *mut vp3054_i2c_state,

// for switching modulation types
    pub ts_gen_cntrl: c_uchar,
// List of attached drivers; must hold core->lock to access
    pub drvlist: list_head,
    pub request_module_wk: work_struct,
}

// -----------------------------------------------------------

// shadow registers

// -----------------------------------------------------------
// cx88-core.c
extern "C" {
    pub fn cx88_core_irq(core: *mut cx88_core, status: u32) -> c_int;
}
extern "C" {
    pub fn cx88_shutdown(core: *mut cx88_core);
}
extern "C" {
    pub fn cx88_reset(core: *mut cx88_core) -> c_int;
}
extern "C" {
    pub fn cx88_set_tvnorm(core: *mut cx88_core, norm: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn cx88_start_audio_dma(core: *mut cx88_core) -> c_int;
}
extern "C" {
    pub fn cx88_stop_audio_dma(core: *mut cx88_core) -> c_int;
}
// -----------------------------------------------------------
// cx88-vbi.c
// Can be used as g_vbi_fmt, try_vbi_fmt and s_vbi_fmt
extern "C" {
    pub fn cx8800_stop_vbi_dma(dev: *mut cx8800_dev);
}
extern "C" {
    pub fn cx8800_restart_vbi_queue(dev: *mut cx8800_dev, q: *mut cx88_dmaqueue) -> c_int;
}
// -----------------------------------------------------------
// cx88-i2c.c
extern "C" {
    pub fn cx88_i2c_init(core: *mut cx88_core, pci: *mut pci_dev) -> c_int;
}
// -----------------------------------------------------------
// cx88-cards.c
extern "C" {
    pub fn cx88_tuner_callback(dev: *mut c_void, component: c_int, command: c_int, arg: c_int) -> c_int;
}
extern "C" {
    pub fn cx88_setup_xc3028(core: *mut cx88_core, ctl: *mut xc2028_ctrl);
}
// -----------------------------------------------------------
// cx88-tvaudio.c
extern "C" {
    pub fn cx88_set_tvaudio(core: *mut cx88_core);
}
extern "C" {
    pub fn cx88_newstation(core: *mut cx88_core);
}
extern "C" {
    pub fn cx88_get_stereo(core: *mut cx88_core, t: *mut v4l2_tuner);
}
extern "C" {
    pub fn cx88_set_stereo(core: *mut cx88_core, mode: u32, manual: c_int);
}
extern "C" {
    pub fn cx88_audio_thread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cx8802_register_driver(drv: *mut cx8802_driver) -> c_int;
}
extern "C" {
    pub fn cx8802_unregister_driver(drv: *mut cx8802_driver) -> c_int;
}
// Caller must hold core->lock
// -----------------------------------------------------------
// cx88-dsp.c
extern "C" {
    pub fn cx88_dsp_detect_stereo_sap(core: *mut cx88_core) -> i32;
}
// -----------------------------------------------------------
// cx88-input.c
extern "C" {
    pub fn cx88_ir_init(core: *mut cx88_core, pci: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn cx88_ir_fini(core: *mut cx88_core) -> c_int;
}
extern "C" {
    pub fn cx88_ir_irq(core: *mut cx88_core);
}
extern "C" {
    pub fn cx88_ir_start(core: *mut cx88_core) -> c_int;
}
extern "C" {
    pub fn cx88_ir_stop(core: *mut cx88_core);
}
extern "C" {
    pub fn cx88_i2c_init_ir(core: *mut cx88_core);
}
// -----------------------------------------------------------
// cx88-mpeg.c
extern "C" {
    pub fn cx8802_buf_queue(dev: *mut cx8802_dev, buf: *mut cx88_buffer);
}
extern "C" {
    pub fn cx8802_cancel_buffers(dev: *mut cx8802_dev);
}
// -----------------------------------------------------------
// cx88-video.c
extern "C" {
    pub fn cx88_enum_input(core: *mut cx88_core, i: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn cx88_set_freq(core: *mut cx88_core, f: *const v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn cx88_video_mux(core: *mut cx88_core, input: c_uint) -> c_int;
}
