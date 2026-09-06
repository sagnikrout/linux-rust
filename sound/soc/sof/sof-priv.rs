//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/sof-priv.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//

// Flag definitions used in sof_core_debug (sof_debug module parameter)

// 1: override topology
//

// 1: use dynamic pipelines
//

// on primary core
//

// and always download firmware upon D3 exit
//

// in dmesg logs
//

// in dmesg logs
//

// configurations
//

// dump the message payload also
//

// Flag definitions used for controlling the DSP dump behavior

// Output this dump (at the DEBUG level) only when SOF_DBG_PRINT_ALL_DUMPS is set

// global debug state set by SOF_DBG_ flags
extern "C" {
    pub fn sof_debug_check_flag(mask: c_int) -> bool;
}
// max BARs mmaped devices can use
pub const SND_SOF_BARS: c_int = 8;
// time in ms for runtime suspend delay
pub const SND_SOF_SUSPEND_DELAY_MS: c_int = 2000;
// DMA buffer size for trace

pub const SOF_IPC_DSP_REPLY: c_int = 0;
pub const SOF_IPC_HOST_REPLY: c_int = 1;
// So far the primary core on all DSPs has ID 0
pub const SOF_DSP_PRIMARY_CORE: c_int = 0;
// max number of DSP cores
pub const SOF_MAX_DSP_NUM_CORES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_dsp_power_state {
    pub state: u32,
    pub /: *mut *mut u32 substate; / platform-specific,
}

// System suspend target state
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_system_suspend_state {
    SOF_SUSPEND_NONE = 0,
    SOF_SUSPEND_S0IX,
    SOF_SUSPEND_S3,
    SOF_SUSPEND_S4,
    SOF_SUSPEND_S5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_dfsentry_type {
    SOF_DFSENTRY_TYPE_IOMEM = 0,
    SOF_DFSENTRY_TYPE_BUF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_debugfs_access_type {
    SOF_DEBUGFS_ACCESS_ALWAYS = 0,
    SOF_DEBUGFS_ACCESS_D0_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_compr_stream {
    pub copied_total: u64,
    pub sampling_rate: u32,
    pub channels: u16,
    pub sample_container_bytes: u16,
    pub codec_params: snd_codec,
    pub posn_offset: usize,
}

//
// struct snd_sof_platform_stream_params - platform dependent stream parameters
// @phy_addr:		Platform dependent address to be used, if  @use_phy_addr
// is true
// @stream_tag:		Stream tag to use
// @use_phy_addr:	Use the provided @phy_addr for configuration
// @no_ipc_position:	Disable position update IPC from firmware
// @cont_update_posn:	Continuous position update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_platform_stream_params {
    pub phy_addr: u32,
    pub stream_tag: u16,
    pub use_phy_address: bool,
    pub no_ipc_position: bool,
    pub cont_update_posn: bool,
}

//
// struct sof_firmware - Container struct for SOF firmware
// @fw:			Pointer to the firmware
// @payload_offset:	Offset of the data within the loaded firmware image to be
// loaded to the DSP (skipping for example ext_manifest section)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_firmware {
    pub fw: *const firmware,
    pub payload_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sof_dai_access {
    SOF_DAI_DSP_ACCESS,	/* access from DSP only */
    SOF_DAI_HOST_ACCESS,	/* access from host only */

    SOF_DAI_ACCESS_NUM
}

//
// SOF DSP HW abstraction operations.
// Used to abstract DSP HW architecture and any IO busses between host CPU
// and DSP device(s).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dsp_ops {
// probe/remove/shutdown
    pub /: *mut *mut *mut *mut int (probe_early)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (probe)(struct snd_sof_dev sof_dev); / mandatory,
    pub /: *mut *mut *mut *mut void (remove)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut void (remove_late)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (shutdown)(struct snd_sof_dev sof_dev); / optional,
// DSP core boot / reset
    pub /: *mut *mut *mut *mut int (run)(struct snd_sof_dev sof_dev); / mandatory,
    pub /: *mut *mut *mut *mut int (stall)(struct snd_sof_dev sof_dev, unsigned int core_mask); / optional,
    pub /: *mut *mut *mut *mut int (reset)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (core_get)(struct snd_sof_dev sof_dev, int core); / optional,
    pub /: *mut *mut *mut *mut int (core_put)(struct snd_sof_dev sof_dev, int core); / optional,
//
// Register IO: only used by respective drivers themselves,
// TODO: consider removing these operations and calling respective
// implementations directly
//
    pub /: *mut *mut u8 value); / optional,
    pub /: *mut *mut *mut void __iomem addr); / optional,
    pub /: *mut *mut u32 value); / optional,
    pub /: *mut *mut *mut void __iomem addr); / optional,
    pub /: *mut *mut u64 value); / optional,
    pub /: *mut *mut *mut void __iomem addr); / optional,
// memcpy IO
    pub /: *mut *mut *mut void dest, size_t size); / mandatory,
    pub /: *mut *mut *mut void src, size_t size); / mandatory,
// Mailbox IO
    pub /: *mut *mut size_t size); / optional,
    pub /: *mut *mut size_t size); / optional,
// doorbell
    pub /: *mut *mut *mut *mut irqreturn_t (irq_handler)(int irq, void context); / optional,
    pub /: *mut *mut *mut *mut irqreturn_t (irq_thread)(int irq, void context); / optional,
// ipc
    pub /: *mut *mut *mut snd_sof_ipc_msg msg); / mandatory,
// FW loading
    pub /: *mut *mut *mut *mut int (load_firmware)(struct snd_sof_dev sof_dev); / mandatory,
    pub /: *mut *mut *mut snd_sof_mod_hdr hdr); / optional,
// connect pcm substream to a host stream
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
// disconnect pcm substream to a host stream
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
// host stream hw params
    pub /: *mut *mut *mut snd_sof_platform_stream_params platform_params); / optional,
// host stream hw_free
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
// host stream trigger
    pub /: *mut *mut int cmd); / optional,
// host stream pointer
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
// pcm ack
    pub /: *mut *mut *mut *mut *mut int (pcm_ack)(struct snd_sof_dev sdev, struct snd_pcm_substream substream); / optional,
//
// optional callback to retrieve the number of frames left/arrived from/to
// the DSP on the DAI side (link/codec/DMIC/etc).
//
// The callback is used when the firmware does not provide this information
// via the shared SRAM window and it can be retrieved by host.
//
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
//
// Optional callback to retrieve the number of bytes left/arrived from/to
// the DSP on the host side (bytes between host ALSA buffer and DSP).
//
// The callback is needed for ALSA delay reporting.
//
    pub /: *mut *mut *mut snd_pcm_substream substream); / optional,
// host read DSP stream data
    pub /: *mut *mut *mut void p, size_t sz); / mandatory,
// host side configuration of the stream's data offset in stream mailbox area
    pub /: *mut *mut size_t posn_offset); / optional,
// pre/post firmware run
    pub /: *mut *mut *mut *mut int (pre_fw_run)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (post_fw_run)(struct snd_sof_dev sof_dev); / optional,
// parse platform specific extended manifest, optional
    pub hdr): *const sof_ext_man_elem_header,
// DSP PM
    pub /: *mut *mut u32 target_state); / optional,
    pub /: *mut *mut *mut *mut int (resume)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (runtime_suspend)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (runtime_resume)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (runtime_idle)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut *mut *mut int (set_hw_params_upon_resume)(struct snd_sof_dev sdev); / optional,
    pub /: *const *const *const sof_dsp_power_state target_state); / optional,
// DSP clocking
    pub /: *mut *mut *mut *mut int (set_clk)(struct snd_sof_dev sof_dev, u32 freq); / optional,
// debug
    pub /: *const *const *const snd_sof_debugfs_map debug_map; / optional,
    pub /: *mut *mut int debug_map_count; / optional,
    pub /: *mut *mut u32 flags); / optional,
    pub /: *mut *mut *mut *mut void (ipc_dump)(struct snd_sof_dev sof_dev); / optional,
    pub /: *mut *mut sof_debugfs_access_type access_type); / optional,
// host DMA trace (IPC3)
    pub /: *mut *mut *mut sof_ipc_dma_trace_params_ext dtrace_params); / optional,
    pub /: *mut *mut *mut *mut int (trace_release)(struct snd_sof_dev sdev); / optional,
    pub /: *mut *mut int cmd); / optional,
// misc
    pub /: *mut *mut u32 type); / optional,
    pub /: *mut *mut *mut *mut int (get_mailbox_offset)(struct snd_sof_dev sdev);/ mandatory for common loader code,
    pub /: *mut *mut u32 id);/ mandatory for common loader code,
// machine driver ops
    pub /: *mut *mut *mut void pdata); / optional,
    pub /: *mut *mut *mut void pdata); / optional,
    pub /: *mut *mut *mut *mut *mut snd_soc_acpi_mach  (machine_select)(snd_sof_dev sdev); / optional,
    pub /: *mut *mut *mut snd_sof_dev sdev); / optional,
// IPC client ops
    pub /: *mut *mut *mut *mut int (register_ipc_clients)(struct snd_sof_dev sdev); / optional,
    pub /: *mut *mut *mut *mut void (unregister_ipc_clients)(struct snd_sof_dev sdev); / optional,
// DAI ops
    pub drv: *mut snd_soc_dai_driver,
    pub num_drv: c_int,
    pub /: *mut *mut *mut *mut bool (is_chain_dma_supported)(struct snd_sof_dev sdev, u32 dai_type); / optional,
// ALSA HW info flags, will be stored in snd_pcm_runtime.hw.info
    pub hw_info: u32,
    pub dsp_arch_ops: *const dsp_arch_ops,
}

// DSP architecture specific callbacks for oops and stack dumps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_arch_ops {
    pub oops): *const *const *const *const void (dsp_oops)(struct snd_sof_dev sdev, char level, void,
    pub stack_words): *mut *mut u32 stack, u32,
}

// FS entry for debug files that can expose DSP memories, registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dfsentry {
    pub size: usize,
    pub /: *mut *mut size_t buf_data_size; / length of buffered data for file read operation,
    pub type: sof_dfsentry_type,
//
// access_type specifies if the
// memory -> DSP resource (memory, register etc) is always accessible
// or if it is accessible only when the DSP is in D0.
//
    pub access_type: sof_debugfs_access_type,

    pub /: *mut *mut *mut char cache_buf; / buffer to cache the contents of debugfs memory,

    pub sdev: *mut snd_sof_dev,
    pub /: *mut *mut list_head list; / list in sdev dfsentry list,
    pub io_mem: *mut void __iomem,
    pub buf: *mut c_void,
}

// Debug mapping for any DSP memory or registers that can used for debug
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_debugfs_map {
    pub name: *const c_char,
    pub bar: u32,
    pub offset: u32,
    pub size: u32,
//
// access_type specifies if the memory is always accessible
// or if it is accessible only when the DSP is in D0.
//
    pub access_type: sof_debugfs_access_type,
}

// mailbox descriptor, used for host <-> DSP IPC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_mailbox {
    pub size: usize,
    pub offset: u32,
}

// IPC message descriptor for host <-> DSP IO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_ipc_msg {
// message data
    pub msg_data: *mut c_void,
    pub reply_data: *mut c_void,
    pub msg_size: usize,
    pub reply_size: usize,
    pub reply_error: c_int,
    pub ipc_complete: bool,
    pub waitq: wait_queue_head_t,
// notification, firmware initiated messages
    pub rx_data: *mut c_void,
}

//
// struct sof_ipc_fw_tracing_ops - IPC-specific firmware tracing ops
// @init:	Function pointer for initialization of the tracing
// @free:	Optional function pointer for freeing of the tracing
// @fw_crashed:	Optional function pointer to notify the tracing of a firmware crash
// @suspend:	Function pointer for system/runtime suspend
// @resume:	Function pointer for system/runtime resume
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_fw_tracing_ops {
    pub sdev): *mut *mut int (init)(struct snd_sof_dev,
    pub sdev): *mut *mut void (free)(struct snd_sof_dev,
    pub sdev): *mut *mut void (fw_crashed)(struct snd_sof_dev,
    pub pm_state): *mut *mut *mut void (suspend)(struct snd_sof_dev sdev, pm_message_t,
    pub sdev): *mut *mut int (resume)(struct snd_sof_dev,
}

//
// struct sof_ipc_pm_ops - IPC-specific PM ops
// @ctx_save:		Optional function pointer for context save
// @ctx_restore:	Optional function pointer for context restore
// @set_core_state:	Optional function pointer for turning on/off a DSP core
// @set_pm_gate:	Optional function pointer for pm gate settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_pm_ops {
    pub sdev): *mut *mut int (ctx_save)(struct snd_sof_dev,
    pub sdev): *mut *mut int (ctx_restore)(struct snd_sof_dev,
    pub on): *mut *mut *mut int (set_core_state)(struct snd_sof_dev sdev, int core_idx, bool,
    pub flags): *mut *mut *mut int (set_pm_gate)(struct snd_sof_dev sdev, u32,
}

//
// struct sof_ipc_fw_loader_ops - IPC/FW-specific loader ops
// @validate:		Function pointer for validating the firmware image
// @parse_ext_manifest:	Function pointer for parsing the manifest of the firmware
// @load_fw_to_dsp:	Optional function pointer for loading the firmware to the
// DSP.
// The function implements generic, hardware independent way
// of loading the initial firmware and its modules (if any).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_fw_loader_ops {
    pub sdev): *mut *mut int (validate)(struct snd_sof_dev,
    pub sdev): *mut *mut size_t (parse_ext_manifest)(struct snd_sof_dev,
    pub sdev): *mut *mut int (load_fw_to_dsp)(struct snd_sof_dev,
}

//
// struct sof_ipc_ops - IPC-specific ops
// @tplg:	Pointer to IPC-specific topology ops
// @pm:		Pointer to PM ops
// @pcm:	Pointer to PCM ops
// @fw_loader:	Pointer to Firmware Loader ops
// @fw_tracing:	Optional pointer to Firmware tracing ops
//
// @init:	Optional pointer for IPC related initialization
// @exit:	Optional pointer for IPC related cleanup
// @post_fw_boot: Optional pointer to execute IPC related tasks after firmware
// boot.
//
// @tx_msg:	Function pointer for sending a 'short' IPC message
// @set_get_data: Function pointer for set/get data ('large' IPC message). This
// function may split up the 'large' message and use the @tx_msg
// path to transfer individual chunks, or use other means to transfer
// the message.
// @get_reply:	Function pointer for fetching the reply to
// sdev->ipc->msg.reply_data
// @rx_msg:	Function pointer for handling a received message
//
// Note: both @tx_msg and @set_get_data considered as TX functions and they are
// serialized for the duration of the instructed transfer. A large message sent
// via @set_get_data is a single transfer even if at the hardware level it is
// handled with multiple chunks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_ops {
    pub tplg: *const sof_ipc_tplg_ops,
    pub pm: *const sof_ipc_pm_ops,
    pub pcm: *const sof_ipc_pcm_ops,
    pub fw_loader: *const sof_ipc_fw_loader_ops,
    pub fw_tracing: *const sof_ipc_fw_tracing_ops,
    pub sdev): *mut *mut int (init)(struct snd_sof_dev,
    pub sdev): *mut *mut void (exit)(struct snd_sof_dev,
    pub sdev): *mut *mut int (post_fw_boot)(struct snd_sof_dev,
    pub no_pm): *mut *mut void reply_data, size_t reply_bytes, bool,
    pub set): bool,
    pub sdev): *mut *mut int (get_reply)(struct snd_sof_dev,
    pub sdev): *mut *mut void (rx_msg)(struct snd_sof_dev,
}

// SOF generic IPC data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_ipc {
    pub sdev: *mut snd_sof_dev,
// protects messages and the disable flag
    pub tx_mutex: mutex,
// disables further sending of ipc's
    pub disable_ipc_tx: bool,
// Maximum allowed size of a single IPC message/reply
    pub max_payload_size: usize,
    pub msg: snd_sof_ipc_msg,
// IPC ops based on version
    pub ops: *const sof_ipc_ops,
}

// Helper to retrieve the IPC ops

//
// SOF Device Level.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sof_dev {
    pub dev: *mut device,
    pub /: *mut *mut spinlock_t ipc_lock; / lock for IPC users,
    pub /: *mut *mut spinlock_t hw_lock; / lock for HW IO access,
//
// When true the DSP is not used.
// It is set under the following condition:
// User sets the SOF_DBG_DSPLESS_MODE flag in sof_debug module parameter
// and
// the platform advertises that it can support such mode
// pdata->desc->dspless_mode_supported is true.
//
    pub dspless_mode_selected: bool,
// Main, Base firmware image
    pub basefw: sof_firmware,
//
// ASoC components. plat_drv fields are set dynamically so
// can't use const
//
    pub plat_drv: snd_soc_component_driver,
// current DSP power state
    pub dsp_power_state: sof_dsp_power_state,
// mutex to protect the dsp_power_state access
    pub power_state_access: mutex,
// Intended power target of system suspend
    pub system_suspend_target: sof_system_suspend_state,
// DSP firmware boot
    pub boot_wait: wait_queue_head_t,
    pub fw_state: sof_fw_state,
    pub first_boot: bool,
// mutex to protect DSP firmware boot (except initial, probe time boot
    pub dsp_fw_boot_mutex: mutex,
// work queue in case the probe is implemented in two steps
    pub probe_work: work_struct,
    pub probe_completed: bool,
// DSP HW differentiation
    pub pdata: *mut snd_sof_pdata,
// IPC
    pub ipc: *mut snd_sof_ipc,
    pub /: *mut *mut snd_sof_mailbox fw_info_box; / FW shared memory,
    pub /: *mut *mut snd_sof_mailbox dsp_box; / DSP initiated IPC,
    pub /: *mut *mut snd_sof_mailbox host_box; / Host initiated IPC,
    pub /: *mut *mut snd_sof_mailbox stream_box; / Stream position update,
    pub /: *mut *mut snd_sof_mailbox debug_box; / Debug info updates,
    pub msg: *mut snd_sof_ipc_msg,
    pub ipc_irq: c_int,
    pub /: *mut *mut u32 next_comp_id; / monotonic - reset during S3,
// memory bases for mmaped DSPs - set by dsp_init()
    pub /: *mut *mut *mut void __iomem bar[SND_SOF_BARS]; / DSP base address,
    pub mmio_bar: c_int,
    pub mailbox_bar: c_int,
    pub dsp_oops_offset: usize,
// debug
    pub debugfs_root: *mut dentry,
    pub dfsentry_list: list_head,
    pub dbg_dump_printed: bool,
    pub ipc_dump_printed: bool,
    pub /: *mut *mut bool d3_prevented; / runtime pm use count incremented to prevent context lost,
// firmware loader
    pub fw_ready: sof_ipc_fw_ready,
    pub fw_version: sof_ipc_fw_version,
    pub cc_version: *mut sof_ipc_cc_version,
// topology
    pub tplg_ops: *mut snd_soc_tplg_ops,
    pub pcm_list: list_head,
    pub kcontrol_list: list_head,
    pub widget_list: list_head,
    pub pipeline_list: list_head,
    pub dai_list: list_head,
    pub dai_link_list: list_head,
    pub route_list: list_head,
    pub component: *mut snd_soc_component,
    pub /: *mut *mut u32 enabled_cores_mask; / keep track of enabled cores,
    pub led_present: bool,
// FW configuration
    pub info_window: *mut sof_ipc_window,
// IPC timeouts in ms
    pub ipc_timeout: c_int,
    pub boot_timeout: c_int,
// firmwre tracing
    pub /: *mut *mut bool fw_trace_is_supported; / set with Kconfig or module parameter,
    pub /: *mut *mut *mut void fw_trace_data; / private data used by firmware tracing implementation,
    pub msi_enabled: bool,
// DSP core context
    pub num_cores: u32,
//
// ref count per core that will be modified during system suspend/resume and during pcm
// hw_params/hw_free. This doesn't need to be protected with a mutex because pcm
// hw_params/hw_free are already protected by the PCM mutex in the ALSA framework in
// sound/core/ when streams are active and during system suspend/resume, streams are
// already suspended.
//
    pub dsp_core_ref_count: [c_int; SOF_MAX_DSP_NUM_CORES],
//
// Used to keep track of registered IPC client devices so that they can
// be removed when the parent SOF module is removed.
//
    pub ipc_client_list: list_head,
// mutex to protect client list
    pub ipc_client_mutex: mutex,
//
// Used for tracking the IPC client's RX registration for DSP initiated
// message handling.
//
    pub ipc_rx_handler_list: list_head,
//
// Used for tracking the IPC client's registration for DSP state change
// notification
//
    pub fw_state_handler_list: list_head,
// to protect the ipc_rx_handler_list  and  dsp_state_handler_list list
    pub client_event_handler_mutex: mutex,
// quirks to override topology values
    pub mclk_id_override: bool,
    pub /: *mut *mut u16 mclk_id_quirk; / same size as in IPC3 definitions,
    pub /: *mut *mut *mut void private; / core does not touch this,
}

//
// Device Level.
//
extern "C" {
    pub fn snd_sof_device_probe(dev: *mut device, plat_data: *mut snd_sof_pdata) -> c_int;
}
extern "C" {
    pub fn snd_sof_device_remove(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_device_shutdown(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_device_probe_completed(dev: *mut device) -> bool;
}
extern "C" {
    pub fn snd_sof_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_runtime_idle(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_dsp_power_down_notify(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_prepare(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn snd_sof_complete(dev: *mut device);
}
extern "C" {
    pub fn snd_sof_boot_dsp_firmware(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_new_platform_drv(sdev: *mut snd_sof_dev);
}
//
// Compress support
//
// Firmware (firmware, libraries, topologies) file location
//
// Firmware loading.
//
extern "C" {
    pub fn snd_sof_load_firmware_raw(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_load_firmware_memcpy(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_run_firmware(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_fw_unload(sdev: *mut snd_sof_dev);
}
//
// IPC low level APIs.
//
extern "C" {
    pub fn snd_sof_ipc_free(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn snd_sof_ipc_get_reply(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn snd_sof_ipc_reply(sdev: *mut snd_sof_dev, msg_id: u32);
}
extern "C" {
    pub fn sof_ipc_tx_message(_arg: ipc, _arg: msg_data, _arg: msg_bytes, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn sof_ipc_tx_message_no_pm(_arg: ipc, _arg: msg_data, _arg: msg_bytes, _arg: NULL, _arg: 0) -> return;
}
//
// Trace/debug
//
extern "C" {
    pub fn snd_sof_dbg_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn snd_sof_free_debug(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn snd_sof_handle_fw_exception(sdev: *mut snd_sof_dev, msg: *const c_char);
}
extern "C" {
    pub fn snd_sof_dbg_memory_info_init(sdev: *mut snd_sof_dev) -> c_int;
}
// Firmware tracing
extern "C" {
    pub fn sof_fw_trace_init(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_fw_trace_free(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn sof_fw_trace_fw_crashed(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn sof_fw_trace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t);
}
extern "C" {
    pub fn sof_fw_trace_resume(sdev: *mut snd_sof_dev) -> c_int;
}
//
// DSP Architectures.
//
// Firmware state tracking
//
extern "C" {
    pub fn sof_set_fw_state(sdev: *mut snd_sof_dev, new_state: sof_fw_state);
}
//
// Utilities
//
extern "C" {
    pub fn sof_io_write(sdev: *mut snd_sof_dev, addr: *mut void __iomem, value: u32);
}
extern "C" {
    pub fn sof_io_write64(sdev: *mut snd_sof_dev, addr: *mut void __iomem, value: u64);
}
extern "C" {
    pub fn sof_io_read(sdev: *mut snd_sof_dev, addr: *mut void __iomem) -> u32;
}
extern "C" {
    pub fn sof_io_read64(sdev: *mut snd_sof_dev, addr: *mut void __iomem) -> u64;
}
// SOF client support

extern "C" {
    pub fn sof_client_dev_unregister(sdev: *mut snd_sof_dev, name: *const c_char, id: u32);
}
extern "C" {
    pub fn sof_register_clients(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn sof_unregister_clients(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn sof_client_ipc_rx_dispatcher(sdev: *mut snd_sof_dev, msg_buf: *mut c_void);
}
extern "C" {
    pub fn sof_client_fw_state_dispatcher(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn sof_suspend_clients(sdev: *mut snd_sof_dev, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn sof_resume_clients(sdev: *mut snd_sof_dev) -> c_int;
}

// Main ops for IPC implementations
