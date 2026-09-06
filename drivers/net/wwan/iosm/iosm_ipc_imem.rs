//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wwan/iosm/iosm_ipc_imem.h
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
// Copyright (C) 2020-21 Intel Corporation.
//

// IRQ moderation in usec
pub const IRQ_MOD_OFF: c_int = 0;
pub const IRQ_MOD_NET: c_int = 1000;
pub const IRQ_MOD_TRC: c_int = 4000;
// Either the PSI image is accepted by CP or the suspended flash tool is waken,
// informed that the CP ROM driver is not ready to process the PSI image.
// unit : milliseconds
//
pub const IPC_PSI_TRANSFER_TIMEOUT: c_int = 3000;
// Timeout in 20 msec to wait for the modem to boot up to
// IPC_MEM_DEVICE_IPC_INIT state.
// unit : milliseconds (500 * ipc_util_msleep(20))
//
pub const IPC_MODEM_BOOT_TIMEOUT: c_int = 500;
// Wait timeout for ipc status reflects IPC_MEM_DEVICE_IPC_UNINIT
// unit : milliseconds
//
pub const IPC_MODEM_UNINIT_TIMEOUT_MS: c_int = 30;
// Pending time for processing data.
// unit : milliseconds
//
pub const IPC_PEND_DATA_TIMEOUT: c_int = 500;
// The timeout in milliseconds for application to wait for remote time.
pub const IPC_REMOTE_TS_TIMEOUT_MS: c_int = 10;
// Timeout for TD allocation retry.
// unit : milliseconds
//
pub const IPC_TD_ALLOC_TIMER_PERIOD_MS: c_int = 100;
// Host sleep target is host
pub const IPC_HOST_SLEEP_HOST: c_int = 0;
// Host sleep target is device
pub const IPC_HOST_SLEEP_DEVICE: c_int = 1;
// Sleep message, target host: AP enters sleep / target device: CP is
// allowed to enter sleep and shall use the host sleep protocol
//
pub const IPC_HOST_SLEEP_ENTER_SLEEP: c_int = 0;
// Sleep_message, target host: AP exits  sleep / target device: CP is
// NOT allowed to enter sleep
//
pub const IPC_HOST_SLEEP_EXIT_SLEEP: c_int = 1;

pub const IPC_MEM_MAX_CHANNELS: c_int = 8;
pub const IPC_MEM_MUX_IP_SESSION_ENTRIES: c_int = 8;
pub const IPC_MEM_MUX_IP_CH_IF_ID: c_int = 0;
pub const TD_UPDATE_DEFAULT_TIMEOUT_USEC: c_int = 1900;
pub const FORCE_UPDATE_DEFAULT_TIMEOUT_USEC: c_int = 500;
// Sleep_message, target host: not applicable  / target device: CP is
// allowed to enter sleep and shall NOT use the device sleep protocol
//
pub const IPC_HOST_SLEEP_ENTER_SLEEP_NO_PROTOCOL: c_int = 2;
// in_band_crash_signal IPC_MEM_INBAND_CRASH_SIG
// Modem crash notification configuration. If this value is non-zero then
// FEATURE_SET message will be sent to the Modem as a result the Modem will
// signal Crash via Execution Stage register. If this value is zero then Modem
// will use out-of-band method to notify about it's Crash.
//
pub const IPC_MEM_INBAND_CRASH_SIG: c_int = 1;
// Extra headroom to be allocated for DL SKBs to allow addition of Ethernet
// header
//
pub const IPC_MEM_DL_ETH_OFFSET: c_int = 16;

pub const IOSM_CHIP_INFO_SIZE_MAX: c_int = 100;
pub const FULLY_FUNCTIONAL: c_int = 0;
pub const IOSM_DEVLINK_INIT: c_int = 1;
// List of the supported UL/DL pipes.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_pipes {
    IPC_MEM_PIPE_0 = 0,
    IPC_MEM_PIPE_1,
    IPC_MEM_PIPE_2,
    IPC_MEM_PIPE_3,
    IPC_MEM_PIPE_4,
    IPC_MEM_PIPE_5,
    IPC_MEM_PIPE_6,
    IPC_MEM_PIPE_7,
    IPC_MEM_PIPE_8,
    IPC_MEM_PIPE_9,
    IPC_MEM_PIPE_10,
    IPC_MEM_PIPE_11,
    IPC_MEM_PIPE_12,
    IPC_MEM_PIPE_13,
    IPC_MEM_PIPE_14,
    IPC_MEM_PIPE_15,
    IPC_MEM_PIPE_16,
    IPC_MEM_PIPE_17,
    IPC_MEM_PIPE_18,
    IPC_MEM_PIPE_19,
    IPC_MEM_PIPE_20,
    IPC_MEM_PIPE_21,
    IPC_MEM_PIPE_22,
    IPC_MEM_PIPE_23,
    IPC_MEM_MAX_PIPES
}

// Enum defining channel states.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_channel_state {
    IMEM_CHANNEL_FREE,
    IMEM_CHANNEL_RESERVED,
    IMEM_CHANNEL_ACTIVE,
    IMEM_CHANNEL_CLOSING,
}

//
// enum ipc_ctype - Enum defining supported channel type needed for control
// /IP traffic.
// @IPC_CTYPE_WWAN:		Used for IP traffic
// @IPC_CTYPE_CTRL:		Used for Control Communication
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_ctype {
    IPC_CTYPE_WWAN,
    IPC_CTYPE_CTRL,
}

// Pipe direction.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_mem_pipe_dir {
    IPC_MEM_DIR_UL,
    IPC_MEM_DIR_DL,
}

// HP update identifier. To be used as data for ipc_cp_irq_hpda_update()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_hp_identifier {
    IPC_HP_MR = 0,
    IPC_HP_PM_TRIGGER,
    IPC_HP_WAKEUP_SPEC_TMR,
    IPC_HP_TD_UPD_TMR_START,
    IPC_HP_TD_UPD_TMR,
    IPC_HP_FAST_TD_UPD_TMR,
    IPC_HP_UL_WRITE_TD,
    IPC_HP_DL_PROCESS,
    IPC_HP_NET_CHANNEL_INIT,
    IPC_HP_CDEV_OPEN,
}

//
// struct ipc_pipe - Structure for Pipe.
// @tdr_start:			Ipc private protocol Transfer Descriptor Ring
// @channel:			Id of the sio device, set by imem_sio_open,
// needed to pass DL char to the user terminal
// @skbr_start:			Circular buffer for skbuf and the buffer
// reference in a tdr_start entry.
// @phy_tdr_start:		Transfer descriptor start address
// @old_head:			last head pointer reported to CP.
// @old_tail:			AP read position before CP moves the read
// position to write/head. If CP has consumed the
// buffers, AP has to freed the skbuf starting at
// tdr_start[old_tail].
// @nr_of_entries:		Number of elements of skb_start and tdr_start.
// @max_nr_of_queued_entries:	Maximum number of queued entries in TDR
// @accumulation_backoff:	Accumulation in usec for accumulation
// backoff (0 = no acc backoff)
// @irq_moderation:		timer in usec for irq_moderation
// (0=no irq moderation)
// @pipe_nr:			Pipe identification number
// @irq:			Interrupt vector
// @dir:			Direction of data stream in pipe
// @buf_size:			Buffer size (in bytes) for preallocated
// buffers (for DL pipes)
// @nr_of_queued_entries:	Aueued number of entries
// @is_open:			Check for open pipe status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_pipe {
    pub tdr_start: *mut ipc_protocol_td,
    pub channel: *mut ipc_mem_channel,
    pub skbr_start: *mut sk_buff,
    pub phy_tdr_start: dma_addr_t,
    pub old_head: u32,
    pub old_tail: u32,
    pub nr_of_entries: u32,
    pub max_nr_of_queued_entries: u32,
    pub accumulation_backoff: u32,
    pub irq_moderation: u32,
    pub pipe_nr: u32,
    pub irq: u32,
    pub dir: ipc_mem_pipe_dir,
    pub buf_size: u32,
    pub nr_of_queued_entries: u16,
    pub is_open:1: u8,
}

//
// struct ipc_mem_channel - Structure for Channel.
// @channel_id:		Instance of the channel list and is return to the user
// at the end of the open operation.
// @ctype:		Control or netif channel.
// @index:		unique index per ctype
// @ul_pipe:		pipe objects
// @dl_pipe:		pipe objects
// @if_id:		Interface ID
// @net_err_count:	Number of downlink errors returned by ipc_wwan_receive
// interface at the entry point of the IP stack.
// @state:		Free, reserved or busy (in use).
// @ul_sem:		Needed for the blocking write or uplink transfer.
// @ul_list:		Uplink accumulator which is filled by the uplink
// char app or IP stack. The socket buffer pointer are
// added to the descriptor list in the kthread context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_mem_channel {
    pub channel_id: c_int,
    pub ctype: ipc_ctype,
    pub index: c_int,
    pub ul_pipe: ipc_pipe,
    pub dl_pipe: ipc_pipe,
    pub if_id: c_int,
    pub net_err_count: u32,
    pub state: ipc_channel_state,
    pub ul_sem: completion,
    pub ul_list: sk_buff_head,
}

//
// enum ipc_phase - Different AP and CP phases.
// The enums defined after "IPC_P_ROM" and before
// "IPC_P_RUN" indicates the operating state where CP can
// respond to any requests. So while introducing new phase
// this shall be taken into consideration.
// @IPC_P_OFF:		On host PC, the PCIe device link settings are known
// about the combined power on. PC is running, the driver
// is loaded and CP is in power off mode. The PCIe bus
// driver call the device power mode D3hot. In this phase
// the driver the polls the device, until the device is in
// the power on state and signals the power mode D0.
// @IPC_P_OFF_REQ:	The intermediate phase between cleanup activity starts
// and ends.
// @IPC_P_CRASH:	The phase indicating CP crash
// @IPC_P_CD_READY:	The phase indicating CP core dump is ready
// @IPC_P_ROM:		After power on, CP starts in ROM mode and the IPC ROM
// driver is waiting 150 ms for the AP active notification
// saved in the PCI link status register.
// @IPC_P_PSI:		Primary signed image download phase
// @IPC_P_EBL:		Extended bootloader pahse
// @IPC_P_RUN:		The phase after flashing to RAM is the RUNTIME phase.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipc_phase {
    IPC_P_OFF,
    IPC_P_OFF_REQ,
    IPC_P_CRASH,
    IPC_P_CD_READY,
    IPC_P_ROM,
    IPC_P_PSI,
    IPC_P_EBL,
    IPC_P_RUN,
}

//
// struct iosm_imem - Current state of the IPC shared memory.
// @mmio:			mmio instance to access CP MMIO area
// doorbell scratchpad.
// @ipc_protocol:		IPC Protocol instance
// @ipc_task:			Task for entry into ipc task queue
// @wwan:			WWAN device pointer
// @mux:			IP Data multiplexing state.
// @sio:			IPC SIO data structure pointer
// @ipc_port:			IPC PORT data structure pointer
// @pcie:			IPC PCIe
// @trace:			IPC trace data structure pointer
// @dev:			Pointer to device structure
// @ipc_requested_state:	Expected IPC state on CP.
// @channels:			Channel list with UL/DL pipe pairs.
// @ipc_devlink:		IPC Devlink data structure pointer
// @ipc_status:			local ipc_status
// @nr_of_channels:		number of configured channels
// @startup_timer:		startup timer for NAND support.
// @hrtimer_period:		Hr timer period
// @tdupdate_timer:		Delay the TD update doorbell.
// @fast_update_timer:		forced head pointer update delay timer.
// @td_alloc_timer:		Timer for DL pipe TD allocation retry
// @adb_timer:			Timer for finishing the ADB.
// @rom_exit_code:		Mapped boot rom exit code.
// @enter_runtime:		1 means the transition to runtime phase was
// executed.
// @ul_pend_sem:		Semaphore to wait/complete of UL TDs
// before closing pipe.
// @app_notify_ul_pend:		Signal app if UL TD is pending
// @dl_pend_sem:		Semaphore to wait/complete of DL TDs
// before closing pipe.
// @app_notify_dl_pend:		Signal app if DL TD is pending
// @phase:			Operating phase like runtime.
// @pci_device_id:		Device ID
// @cp_version:			CP version
// @device_sleep:		Device sleep state
// @run_state_worker:		Pointer to worker component for device
// setup operations to be called when modem
// reaches RUN state
// @ev_irq_pending:		0 means inform the IPC tasklet to
// process the irq actions.
// @flag:			Flag to monitor the state of driver
// @td_update_timer_suspended:	if true then td update timer suspend
// @ev_mux_net_transmit_pending:0 means inform the IPC tasklet to pass
// @reset_det_n:		Reset detect flag
// @pcie_wake_n:		Pcie wake flag
// @debugfs_wwan_dir:		WWAN Debug FS directory entry
// @debugfs_dir:		Debug FS directory for driver-specific entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iosm_imem {
    pub mmio: *mut iosm_mmio,
    pub ipc_protocol: *mut iosm_protocol,
    pub ipc_task: *mut ipc_task,
    pub wwan: *mut iosm_wwan,
    pub mux: *mut iosm_mux,
    pub ipc_port: [*mut iosm_cdev; IPC_MEM_MAX_CHANNELS],
    pub pcie: *mut iosm_pcie,

    pub trace: *mut iosm_trace,

    pub dev: *mut device,
    pub ipc_requested_state: ipc_mem_device_ipc_state,
    pub channels: [ipc_mem_channel; IPC_MEM_MAX_CHANNELS],
    pub ipc_devlink: *mut iosm_devlink,
    pub ipc_status: u32,
    pub nr_of_channels: u32,
    pub startup_timer: hrtimer,
    pub hrtimer_period: ktime_t,
    pub tdupdate_timer: hrtimer,
    pub fast_update_timer: hrtimer,
    pub td_alloc_timer: hrtimer,
    pub adb_timer: hrtimer,
    pub rom_exit_code: rom_exit_code,
    pub enter_runtime: u32,
    pub ul_pend_sem: completion,
    pub app_notify_ul_pend: u32,
    pub dl_pend_sem: completion,
    pub app_notify_dl_pend: u32,
    pub phase: ipc_phase,
    pub pci_device_id: u16,
    pub cp_version: c_int,
    pub device_sleep: c_int,
    pub run_state_worker: work_struct,
    pub ev_irq_pending: [u8; IPC_IRQ_VECTORS],
    pub flag: c_ulong,

    pub debugfs_wwan_dir: *mut dentry,
    pub debugfs_dir: *mut dentry,

}

//
// ipc_imem_init - Initialize the shared memory region
// @pcie:	Pointer to core driver data-struct
// @device_id:	PCI device ID
// @mmio:	Pointer to the mmio area
// @dev:	Pointer to device structure
//
// Returns:  Initialized imem pointer on success else NULL
//
// ipc_imem_pm_s2idle_sleep - Set PM variables to sleep/active for
// s2idle sleep/active
// @ipc_imem:	Pointer to imem data-struct
// @sleep:	Set PM Variable to sleep/active
//
extern "C" {
    pub fn ipc_imem_pm_s2idle_sleep(ipc_imem: *mut iosm_imem, sleep: bool);
}
//
// ipc_imem_pm_suspend - The HAL shall ask the shared memory layer
// whether D3 is allowed.
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_pm_suspend(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_pm_resume - The HAL shall inform the shared memory layer
// that the device is active.
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_pm_resume(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_cleanup -	Inform CP and free the shared memory resources.
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_cleanup(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_irq_process - Shift the IRQ actions to the IPC thread.
// @ipc_imem:	Pointer to imem data-struct
// @irq:	Irq number
//
extern "C" {
    pub fn ipc_imem_irq_process(ipc_imem: *mut iosm_imem, irq: c_int);
}
//
// imem_get_device_sleep_state - Get the device sleep state value.
// @ipc_imem:	Pointer to imem instance
//
// Returns: device sleep state
//
extern "C" {
    pub fn imem_get_device_sleep_state(ipc_imem: *mut iosm_imem) -> c_int;
}
//
// ipc_imem_td_update_timer_suspend - Updates the TD Update Timer suspend flag.
// @ipc_imem:	Pointer to imem data-struct
// @suspend:	Flag to update. If TRUE then HP update doorbell is triggered to
// device without any wait. If FALSE then HP update doorbell is
// delayed until timeout.
//
extern "C" {
    pub fn ipc_imem_td_update_timer_suspend(ipc_imem: *mut iosm_imem, suspend: bool);
}
//
// ipc_imem_channel_close - Release the channel resources.
// @ipc_imem:		Pointer to imem data-struct
// @channel_id:		Channel ID to be cleaned up.
//
extern "C" {
    pub fn ipc_imem_channel_close(ipc_imem: *mut iosm_imem, channel_id: c_int);
}
//
// ipc_imem_channel_alloc - Reserves a channel
// @ipc_imem:	Pointer to imem data-struct
// @index:	ID to lookup from the preallocated list.
// @ctype:	Channel type.
//
// Returns: Index on success and failure value on error
//
// ipc_imem_channel_open - Establish the pipes.
// @ipc_imem:		Pointer to imem data-struct
// @channel_id:		Channel ID returned during alloc.
// @db_id:		Doorbell ID for trigger identifier.
//
// Returns: Pointer of ipc_mem_channel on success and NULL on failure.
//
// ipc_imem_td_update_timer_start - Starts the TD Update Timer if not running.
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_td_update_timer_start(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_ul_write_td - Pass the channel UL list to protocol layer for TD
// preparation and sending them to the device.
// @ipc_imem:	Pointer to imem data-struct
//
// Returns: TRUE of HP Doorbell trigger is pending. FALSE otherwise.
//
extern "C" {
    pub fn ipc_imem_ul_write_td(ipc_imem: *mut iosm_imem) -> bool;
}
//
// ipc_imem_ul_send - Dequeue SKB from channel list and start with
// the uplink transfer.If HP Doorbell is pending to be
// triggered then starts the TD Update Timer.
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_ul_send(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_channel_update - Set or modify pipe config of an existing channel
// @ipc_imem:		Pointer to imem data-struct
// @id:			Channel config index
// @chnl_cfg:		Channel config struct
// @irq_moderation:	Timer in usec for irq_moderation
//
// ipc_imem_channel_free -Free an IPC channel.
// @channel:	Channel to be freed
//
extern "C" {
    pub fn ipc_imem_channel_free(channel: *mut ipc_mem_channel);
}
//
// ipc_imem_hrtimer_stop - Stop the hrtimer
// @hr_timer:	Pointer to hrtimer instance
//
extern "C" {
    pub fn ipc_imem_hrtimer_stop(hr_timer: *mut hrtimer);
}
//
// ipc_imem_pipe_cleanup - Reset volatile pipe content for all channels
// @ipc_imem:	Pointer to imem data-struct
// @pipe:	Pipe to cleaned up
//
extern "C" {
    pub fn ipc_imem_pipe_cleanup(ipc_imem: *mut iosm_imem, pipe: *mut ipc_pipe);
}
//
// ipc_imem_pipe_close - Send msg to device to close pipe
// @ipc_imem:	Pointer to imem data-struct
// @pipe:	Pipe to be closed
//
extern "C" {
    pub fn ipc_imem_pipe_close(ipc_imem: *mut iosm_imem, pipe: *mut ipc_pipe);
}
//
// ipc_imem_phase_update - Get the CP execution state
// and map it to the AP phase.
// @ipc_imem:	Pointer to imem data-struct
//
// Returns: Current ap updated phase
//
extern "C" {
    pub fn ipc_imem_phase_update(ipc_imem: *mut iosm_imem) -> ipc_phase;
}
//
// ipc_imem_phase_get_string - Return the current operation
// phase as string.
// @phase:	AP phase
//
// Returns: AP phase string
//
// ipc_imem_msg_send_feature_set - Send feature set message to modem
// @ipc_imem:		Pointer to imem data-struct
// @reset_enable:	0 = out-of-band, 1 = in-band-crash notification
// @atomic_ctx:		if disabled call in tasklet context
//
// ipc_imem_ipc_init_check - Send the init event to CP, wait a certain time and
// set CP to runtime with the context information
// @ipc_imem:	Pointer to imem data-struct
//
extern "C" {
    pub fn ipc_imem_ipc_init_check(ipc_imem: *mut iosm_imem);
}
//
// ipc_imem_channel_init - Initialize the channel list with UL/DL pipe pairs.
// @ipc_imem:		Pointer to imem data-struct
// @ctype:		Channel type
// @chnl_cfg:		Channel configuration struct
// @irq_moderation:	Timer in usec for irq_moderation
//
// ipc_imem_devlink_trigger_chip_info - Inform devlink that the chip
// information are available if the
// flashing to RAM interworking shall be
// executed.
// @ipc_imem:	Pointer to imem structure
//
// Returns: 0 on success, -1 on failure
//
extern "C" {
    pub fn ipc_imem_devlink_trigger_chip_info(ipc_imem: *mut iosm_imem) -> c_int;
}
extern "C" {
    pub fn ipc_imem_adb_timer_start(ipc_imem: *mut iosm_imem);
}
