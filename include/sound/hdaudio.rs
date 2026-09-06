//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/hdaudio.h
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


// SPDX-License-Identifier: GPL-2.0
//
// HD-audio core stuff
//

// codec node id
pub type hda_nid_t = u16;
//
// exported bus type
//
// generic arrays
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_array {
    pub used: c_uint,
    pub alloced: c_uint,
    pub elem_size: c_uint,
    pub alloc_align: c_uint,
    pub list: *mut c_void,
}

//
// HD-audio codec base device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_device {
    pub dev: device,
    pub type: c_int,
    pub bus: *mut hdac_bus,
    pub /: *mut *mut unsigned int addr; / codec address,
    pub /: *mut *mut list_head list; / list point for bus codec_list,
    pub /: *mut *mut hda_nid_t afg; / AFG node id,
    pub /: *mut *mut hda_nid_t mfg; / MFG node id,
// ids
    pub vendor_id: c_uint,
    pub subsystem_id: c_uint,
    pub revision_id: c_uint,
    pub afg_function_id: c_uint,
    pub mfg_function_id: c_uint,
    pub afg_unsol:1: c_uint,
    pub mfg_unsol:1: c_uint,
    pub /: *mut *mut unsigned int power_caps; / FG power caps,
    pub /: *const *const *const char vendor_name; / codec vendor name,
    pub /: *const *const *const char chip_name; / codec chip name,
// verb exec op override
    pub res): *mut unsigned int flags, unsigned int,
// widgets
    pub num_nodes: c_uint,
    pub end_nid: hda_nid_t start_nid,,
// misc flags
    pub /: *mut *mut atomic_t in_pm; / suspend/resume being performed,
// sysfs
    pub widget_lock: mutex,
    pub widgets: *mut hdac_widget_tree,
// regmap
    pub regmap: *mut regmap,
    pub regmap_lock: mutex,
    pub vendor_verbs: snd_array,
    pub /: *mut *mut bool lazy_cache:1; / don't wake up for writes,
    pub /: *mut *mut bool caps_overwriting:1; / caps overwrite being in process,
    pub /: *mut *mut bool cache_coef:1; / cache COEF read/write too,
    pub /: *mut *mut unsigned int registered:1; / codec was registered,
}

// device/driver type used for matching
// direction

extern "C" {
    pub fn snd_hdac_device_exit(dev: *mut hdac_device);
}
extern "C" {
    pub fn snd_hdac_device_register(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_device_unregister(codec: *mut hdac_device);
}
extern "C" {
    pub fn snd_hdac_device_set_chip_name(codec: *mut hdac_device, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn snd_hdac_codec_modalias(hdac: *const hdac_device, buf: *mut c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn snd_hdac_refresh_widgets(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_stream_format(channels: c_uint, bits: c_uint, rate: c_uint) -> c_uint;
}
//
// snd_hdac_read_parm - read a codec parameter
// @codec: the codec object
// @nid: NID to read a parameter
// @parm: parameter to read
//
// Returns -1 for error.  If you need to distinguish the error more
// strictly, use _snd_hdac_read_parm() directly.
//

extern "C" {
    pub fn snd_hdac_power_up(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_power_down(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_power_up_pm(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_power_down_pm(codec: *mut hdac_device) -> c_int;
}
extern "C" {
    pub fn snd_hdac_keep_power_up(codec: *mut hdac_device) -> c_int;
}
// call this at entering into suspend/resume callbacks in codec driver
// call this at leaving from suspend/resume callbacks in codec driver
extern "C" {
    pub fn atomic_read(_arg: &codec->in_pm) -> return;
}

//
// HD-audio codec base driver
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_driver {
    pub driver: device_driver,
    pub type: c_int,
    pub id_table: *const hda_device_id,
    pub drv): *const *const *const int (match)(struct hdac_device dev, struct hdac_driver,
    pub event): *mut *mut *mut void (unsol_event)(struct hdac_device dev, unsigned int,
// fields used by ext bus APIs
    pub dev): *mut *mut int (probe)(struct hdac_device,
    pub dev): *mut *mut int (remove)(struct hdac_device,
    pub dev): *mut *mut void (shutdown)(struct hdac_device,
}

//
// Bus verb operators
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_bus_ops {
// send a single command
    pub cmd): *mut *mut *mut int (command)(struct hdac_bus bus, unsigned int,
// get a response from the last command
    pub res): *mut c_uint,
// notify of codec link power-up/down
    pub enable): *mut *mut *mut void (link_power)(struct hdac_device hdev, bool,
}

//
// ops used for ASoC HDA codec drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_ext_bus_ops {
    pub hdev): *mut *mut int (hdev_attach)(struct hdac_device,
    pub hdev): *mut *mut int (hdev_detach)(struct hdac_device,
}

pub const HDA_UNSOL_QUEUE_SIZE: c_int = 64;

//
// CORB/RIRB
//
// Each CORB entry is 4byte, RIRB is 8byte
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_rb {
    pub /: *mut *mut *mut __le32 buf; / virtual address of CORB/RIRB buffer,
    pub /: *mut *mut dma_addr_t addr; / physical address of CORB/RIRB buffer,
    pub /: *mut *mut unsigned short rp, wp; / RIRB read/write pointers,
    pub /: *mut *mut int cmds[HDA_MAX_CODECS]; / number of pending requests,
    pub /: *mut *mut u32 res[HDA_MAX_CODECS]; / last read value,
}

//
// HD-audio bus base driver
//
// @ppcap: pp capabilities pointer
// @spbcap: SPIB capabilities pointer
// @mlcap: MultiLink capabilities pointer
// @gtscap: gts capabilities pointer
// @drsmcap: dma resume capabilities pointer
// @num_streams: streams supported
// @idx: HDA link index
// @hlink_list: link list of HDA links
// @lock: lock for link and display power mgmt
// @cmd_dma_state: state of cmd DMAs: CORB and RIRB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub ops: *const hdac_bus_ops,
    pub ext_ops: *const hdac_ext_bus_ops,
// h/w resources
    pub addr: c_ulong,
    pub remap_addr: *mut void __iomem,
    pub irq: c_int,
    pub ppcap: *mut void __iomem,
    pub spbcap: *mut void __iomem,
    pub mlcap: *mut void __iomem,
    pub gtscap: *mut void __iomem,
    pub drsmcap: *mut void __iomem,
// codec linked list
    pub codec_list: list_head,
    pub num_codecs: c_uint,
// link caddr -> codec
    pub 1]: *mut *mut hdac_device caddr_tbl[HDA_MAX_CODEC_ADDRESS +,
// unsolicited event queue
    pub /: *mut *mut *mut u32 unsol_queue[HDA_UNSOL_QUEUE_SIZE  2]; / ring buffer,
    pub unsol_wp: unsigned int unsol_rp,,
    pub unsol_work: work_struct,
// bit flags of detected codecs
    pub codec_mask: c_ulong,
// bit flags of powered codecs
    pub codec_powered: c_ulong,
// CORB/RIRB
    pub corb: hdac_rb,
    pub rirb: hdac_rb,
    pub /: *mut *mut unsigned int last_cmd[HDA_MAX_CODECS]; / last sent command,
    pub rirb_wq: wait_queue_head_t,
// CORB/RIRB and position buffers
    pub rb: snd_dma_buffer,
    pub posbuf: snd_dma_buffer,
    pub /: *mut *mut int dma_type; / SNDRV_DMA_TYPE_XXX for CORB/RIRB,
// hdac_stream linked list
    pub stream_list: list_head,
// operation state
    pub /: *mut *mut bool chip_init:1; / h/w initialized,
// behavior flags
    pub /: *mut *mut bool aligned_mmio:1; / aligned MMIO access,
    pub /: *mut *mut bool sync_write:1; / sync after verb write,
    pub /: *mut *mut bool use_posbuf:1; / use position buffer,
    pub /: *mut *mut bool snoop:1; / enable snooping,
    pub /: *mut *mut bool align_bdle_4k:1; / BDLE align 4K boundary,
    pub /: *mut *mut bool reverse_assign:1; / assign devices in reverse order,
    pub /: *mut *mut bool corbrp_self_clear:1; / CORBRP clears itself after reset,
    pub polling_mode:1: bool,
    pub needs_damn_long_delay:1: bool,
    pub /: *mut *mut bool not_use_interrupts:1; / prohibiting the RIRB IRQ,
    pub /: *mut *mut bool access_sdnctl_in_dword:1; / accessing the sdnctl register by dword,
    pub /: *mut *mut bool use_pio_for_commands:1; / Use PIO instead of CORB for commands,
    pub poll_count: c_int,
    pub /: *mut *mut int bdl_pos_adj; / BDL position adjustment,
// delay time in us for dma stop
    pub dma_stop_delay: c_uint,
// locks
    pub reg_lock: spinlock_t,
    pub cmd_mutex: mutex,
    pub lock: mutex,
// DRM component interface
    pub audio_component: *mut drm_audio_component,
    pub display_power_status: c_long,
    pub display_power_active: c_ulong,
// parameters required for enhanced capabilities
    pub num_streams: c_int,
    pub idx: c_int,
// link management
    pub hlink_list: list_head,
    pub cmd_dma_state: bool,
// factor used to derive STRIPE control value
    pub sdo_limit: c_uint,
// address offset between host and hadc
    pub addr_offset: dma_addr_t,
}

extern "C" {
    pub fn snd_hdac_bus_exit(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_codec_link_up(codec: *mut hdac_device);
}
extern "C" {
    pub fn snd_hdac_codec_link_down(codec: *mut hdac_device);
}
extern "C" {
    pub fn snd_hdac_bus_send_cmd(bus: *mut hdac_bus, val: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_hdac_bus_parse_capabilities(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn snd_hdac_bus_init_chip(bus: *mut hdac_bus, full_reset: bool) -> bool;
}
extern "C" {
    pub fn snd_hdac_bus_stop_chip(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_init_cmd_io(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_stop_cmd_io(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_enter_link_reset(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_exit_link_reset(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_reset_link(bus: *mut hdac_bus, full_reset: bool) -> c_int;
}
extern "C" {
    pub fn snd_hdac_bus_link_power(hdev: *mut hdac_device, enable: bool);
}
extern "C" {
    pub fn snd_hdac_bus_update_rirb(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_bus_alloc_stream_pages(bus: *mut hdac_bus) -> c_int;
}
extern "C" {
    pub fn snd_hdac_bus_free_stream_pages(bus: *mut hdac_bus);
}

extern "C" {
    pub fn snd_hdac_aligned_read(addr: *mut void __iomem, mask: c_uint) -> c_uint;
}

//
// macros for easy use
//

// read/write a register, pass without AZX_REG_ prefix

// update a register, pass without AZX_REG_ prefix

// update register macro

//
// HD-audio stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdac_stream {
    pub bus: *mut hdac_bus,
    pub /: *mut *mut snd_dma_buffer bdl; / BDL buffer,
    pub /: *mut *mut *mut __le32 posbuf; / position buffer pointer,
    pub /: *mut *mut *mut int direction; / playback / capture (SNDRV_PCM_STREAM_),
    pub /: *mut *mut unsigned int bufsize; / size of the play buffer in bytes,
    pub /: *mut *mut unsigned int period_bytes; / size of the period in bytes,
    pub /: *mut *mut unsigned int frags; / number for period in the play buffer,
    pub /: *mut *mut unsigned int fifo_size; / FIFO size,
    pub /: *mut *mut *mut void __iomem sd_addr; / stream descriptor pointer,
    pub /: *mut *mut *mut void __iomem spib_addr; / software position in buffers stream pointer,
    pub /: *mut *mut *mut void __iomem fifo_addr; / software position Max fifos stream pointer,
    pub /: *mut *mut *mut void __iomem dpibr_addr; / DMA position in buffer resume pointer,
    pub /: *mut *mut u32 dpib; / DMA position in buffer,
    pub /: *mut *mut u32 lpib; / Linear position in buffer,
    pub /: *mut *mut u32 sd_int_sta_mask; / stream int status mask,
// pcm support
    pub substream,: *mut *mut *mut snd_pcm_substream substream; / assigned,
// set in PCM open
//
    pub cstream: *mut snd_compr_stream,
    pub the: *mut *mut unsigned int format_val; / format value to be set in,
// controller and the codec
//
    pub /: *mut *mut unsigned char stream_tag; / assigned stream,
    pub /: *mut *mut unsigned char index; / stream index,
    pub /: *mut *mut int assigned_key; / last device# key assigned to,
    pub opened:1: bool,
    pub running:1: bool,
    pub prepared:1: bool,
    pub no_period_wakeup:1: bool,
    pub locked:1: bool,
    pub /: *mut *mut bool stripe:1; / apply stripe control,
    pub curr_pos: u64,
// timestamp
    pub /: *mut *mut unsigned long start_wallclk; / start + minimum wallclk,
    pub /: *mut *mut unsigned long period_wallclk; / wallclk for period,
    pub tc: timecounter,
    pub cc: cyclecounter,
    pub delay_negative_threshold: c_int,
    pub list: list_head,

// DSP access mutex
    pub dsp_mutex: mutex,

}

extern "C" {
    pub fn snd_hdac_stream_release_locked(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stream_release(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stream_setup(azx_dev: *mut hdac_stream, code_loading: bool) -> c_int;
}
extern "C" {
    pub fn snd_hdac_stream_cleanup(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stream_setup_periods(azx_dev: *mut hdac_stream) -> c_int;
}
extern "C" {
    pub fn snd_hdac_stream_start(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stream_stop(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stop_streams(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_stop_streams_and_chip(bus: *mut hdac_bus);
}
extern "C" {
    pub fn snd_hdac_stream_reset(azx_dev: *mut hdac_stream);
}
extern "C" {
    pub fn snd_hdac_stream_wait_drsm(azx_dev: *mut hdac_stream) -> c_int;
}
extern "C" {
    pub fn snd_hdac_stream_set_lpib(azx_dev: *mut hdac_stream, value: u32) -> c_int;
}
//
// macros for easy use
//
// read/write a register, pass without AZX_REG_ prefix

// update a register, pass without AZX_REG_ prefix

// DSP lock helpers

// DSP loader helpers
extern "C" {
    pub fn snd_hdac_dsp_trigger(azx_dev: *mut hdac_stream, start: bool);
}

pub const snd_hdac_stream_is_locked(dev): c_int = 0;

//
// Easy macros for widget capabilities
//

// get the widget type from widget capability bits
// get the number of supported channels
//
// generic array helpers
//
extern "C" {
    pub fn snd_array_free(array: *mut snd_array);
}
// a helper macro to iterate for each snd_array element

//
// Device matching
//

