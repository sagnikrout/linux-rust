//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc2/core.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// core.h - DesignWare HS OTG Controller common declarations
//
// Copyright (C) 2004-2013 Synopsys, Inc.
//

//
// Suggested defines for tracers:
// - no_printk:    Disable tracing
// - pr_info:      Print this info to the console
// - trace_printk: Print this info to trace buffer (good for verbose logging)
//

// Detailed scheduler tracing, but won't overwhelm console

// Verbose scheduler tracing

// Maximum number of Endpoints/HostChannels
pub const MAX_EPS_CHANNELS: c_int = 16;
// dwc2-hsotg declarations

//
// EP0_MPS_LIMIT
//
// Unfortunately there seems to be a limit of the amount of data that can
// be transferred by IN transactions on EP0. This is either 127 bytes or 3
// packets (which practically means 1 packet and 63 bytes of data) when the
// MPS is set to 64.
//
// This means if we are wanting to move >127 bytes of data, we need to
// split the transactions up, but just doing one packet at a time does
// not work (this may be an implicit DATA0 PID on first packet of the
// transaction) and doing 2 packets is outside the controller's limits.
//
// If we try to lower the MPS size for EP0, then no transfers work properly
// for EP0, and the system will fail basic enumeration. As no cause for this
// has currently been found, we cannot support any large IN transfers for
// EP0.
//
pub const EP0_MPS_LIMIT: c_int = 64;
//
// struct dwc2_hsotg_ep - driver endpoint definition.
// @ep: The gadget layer representation of the endpoint.
// @name: The driver generated name for the endpoint.
// @queue: Queue of requests for this endpoint.
// @parent: Reference back to the parent device structure.
// @req: The current request that the endpoint is processing. This is
// used to indicate an request has been loaded onto the endpoint
// and has yet to be completed (maybe due to data move, or simply
// awaiting an ack from the core all the data has been completed).
// @debugfs: File entry for debugfs file for this endpoint.
// @dir_in: Set to true if this endpoint is of the IN direction, which
// means that it is sending data to the Host.
// @map_dir: Set to the value of dir_in when the DMA buffer is mapped.
// @index: The index for the endpoint registers.
// @mc: Multi Count - number of transactions per microframe
// @interval: Interval for periodic endpoints, in frames or microframes.
// @name: The name array passed to the USB core.
// @halted: Set if the endpoint has been halted.
// @periodic: Set if this is a periodic ep, such as Interrupt
// @isochronous: Set if this is a isochronous ep
// @send_zlp: Set if we need to send a zero-length packet.
// @wedged: Set if ep is wedged.
// @desc_list_dma: The DMA address of descriptor chain currently in use.
// @desc_list: Pointer to descriptor DMA chain head currently in use.
// @desc_count: Count of entries within the DMA descriptor chain of EP.
// @next_desc: index of next free descriptor in the ISOC chain under SW control.
// @compl_desc: index of next descriptor to be completed by xFerComplete
// @total_data: The total number of data bytes done.
// @fifo_size: The size of the FIFO (for periodic IN endpoints)
// @fifo_index: For Dedicated FIFO operation, only FIFO0 can be used for EP0.
// @fifo_load: The amount of data loaded into the FIFO (periodic IN)
// @last_load: The offset of data for the last start of request.
// @size_loaded: The last loaded size for DxEPTSIZE for periodic IN
// @target_frame: Targeted frame num to setup next ISOC transfer
// @frame_overrun: Indicates SOF number overrun in DSTS
//
// This is the driver's state for each registered endpoint, allowing it
// to keep track of transactions that need doing. Each endpoint has a
// lock to protect the state, to try and avoid using an overall lock
// for the host controller as much as possible.
//
// For periodic IN endpoints, we have fifo_size and fifo_load to try
// and keep track of the amount of data in the periodic FIFO for each
// of these as we don't have a status register that tells us how much
// is in each of them. (note, this may actually be useless information
// as in shared-fifo mode periodic in acts like a single-frame packet
// buffer than a fifo)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hsotg_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub parent: *mut dwc2_hsotg,
    pub req: *mut dwc2_hsotg_req,
    pub debugfs: *mut dentry,
    pub total_data: c_ulong,
    pub size_loaded: c_uint,
    pub last_load: c_uint,
    pub fifo_load: c_uint,
    pub fifo_size: c_ushort,
    pub fifo_index: c_ushort,
    pub dir_in: c_uchar,
    pub map_dir: c_uchar,
    pub index: c_uchar,
    pub mc: c_uchar,
    pub interval: u16,
    pub halted:1: c_uint,
    pub periodic:1: c_uint,
    pub isochronous:1: c_uint,
    pub send_zlp:1: c_uint,
    pub wedged:1: c_uint,
    pub target_frame: c_uint,
pub const TARGET_FRAME_INITIAL: c_uint = 0xFFFFFFFF;
    pub frame_overrun: bool,
    pub desc_list_dma: dma_addr_t,
    pub desc_list: *mut dwc2_dma_desc,
    pub desc_count: u8,
    pub next_desc: c_uint,
    pub compl_desc: c_uint,
    pub name: [c_char; 10],
}

//
// struct dwc2_hsotg_req - data transfer request
// @req: The USB gadget request
// @queue: The list of requests for the endpoint this is queued for.
// @saved_req_buf: variable to save req.buf when bounce buffers are used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hsotg_req {
    pub req: usb_request,
    pub queue: list_head,
    pub saved_req_buf: *mut c_void,
}

// Device States
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_lx_state {
    DWC2_L0,	/* On state */
    DWC2_L1,	/* LPM sleep state */
    DWC2_L2,	/* USB suspend state */
    DWC2_L3,	/* Off state */
}

// Gadget ep0 states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_ep0_state {
    DWC2_EP0_SETUP,
    DWC2_EP0_DATA_IN,
    DWC2_EP0_DATA_OUT,
    DWC2_EP0_STATUS_IN,
    DWC2_EP0_STATUS_OUT,
}

//
// struct dwc2_core_params - Parameters for configuring the core
//
// @otg_caps:           Specifies the OTG capabilities. OTG caps from the platform parameters,
// used to setup the:
// - HNP and SRP capable
// - SRP Only capable
// - No HNP/SRP capable (always available)
// Defaults to best available option
// - OTG revision number the device is compliant with, in binary-coded
// decimal (i.e. 2.0 is 0200H). (see struct usb_otg_caps)
// @host_dma:           Specifies whether to use slave or DMA mode for accessing
// the data FIFOs. The driver will automatically detect the
// value for this parameter if none is specified.
// 0 - Slave (always available)
// 1 - DMA (default, if available)
// @dma_desc_enable:    When DMA mode is enabled, specifies whether to use
// address DMA mode or descriptor DMA mode for accessing
// the data FIFOs. The driver will automatically detect the
// value for this if none is specified.
// 0 - Address DMA
// 1 - Descriptor DMA (default, if available)
// @dma_desc_fs_enable: When DMA mode is enabled, specifies whether to use
// address DMA mode or descriptor DMA mode for accessing
// the data FIFOs in Full Speed mode only. The driver
// will automatically detect the value for this if none is
// specified.
// 0 - Address DMA
// 1 - Descriptor DMA in FS (default, if available)
// @speed:              Specifies the maximum speed of operation in host and
// device mode. The actual speed depends on the speed of
// the attached device and the value of phy_type.
// 0 - High Speed
// (default when phy_type is UTMI+ or ULPI)
// 1 - Full Speed
// (default when phy_type is Full Speed)
// @enable_dynamic_fifo: 0 - Use coreConsultant-specified FIFO size parameters
// 1 - Allow dynamic FIFO sizing (default, if available)
// @en_multiple_tx_fifo: Specifies whether dedicated per-endpoint transmit FIFOs
// are enabled for non-periodic IN endpoints in device
// mode.
// @host_rx_fifo_size:  Number of 4-byte words in the Rx FIFO in host mode when
// dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @host_nperio_tx_fifo_size: Number of 4-byte words in the non-periodic Tx FIFO
// in host mode when dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @host_perio_tx_fifo_size: Number of 4-byte words in the periodic Tx FIFO in
// host mode when dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @max_transfer_size:  The maximum transfer size supported, in bytes
// 2047 to 65,535
// Actual maximum value is autodetected and also
// the default.
// @max_packet_count:   The maximum number of packets in a transfer
// 15 to 511
// Actual maximum value is autodetected and also
// the default.
// @host_channels:      The number of host channel registers to use
// 1 to 16
// Actual maximum value is autodetected and also
// the default.
// @phy_type:           Specifies the type of PHY interface to use. By default,
// the driver will automatically detect the phy_type.
// 0 - Full Speed Phy
// 1 - UTMI+ Phy
// 2 - ULPI Phy
// Defaults to best available option (2, 1, then 0)
// @phy_utmi_width:     Specifies the UTMI+ Data Width (in bits). This parameter
// is applicable for a phy_type of UTMI+ or ULPI. (For a
// ULPI phy_type, this parameter indicates the data width
// between the MAC and the ULPI Wrapper.) Also, this
// parameter is applicable only if the OTG_HSPHY_WIDTH cC
// parameter was set to "8 and 16 bits", meaning that the
// core has been configured to work at either data path
// width.
// 8 or 16 (default 16 if available)
// @eusb2_disc:         Specifies whether eUSB2 PHY disconnect support flow
// applicable or no. Applicable in device mode of HSOTG
// and HS IOT cores v5.00 or higher.
// 0 - eUSB2 PHY disconnect support flow not applicable
// 1 - eUSB2 PHY disconnect support flow applicable
// @phy_ulpi_ddr:       Specifies whether the ULPI operates at double or single
// data rate. This parameter is only applicable if phy_type
// is ULPI.
// 0 - single data rate ULPI interface with 8 bit wide
// data bus (default)
// 1 - double data rate ULPI interface with 4 bit wide
// data bus
// @phy_ulpi_ext_vbus:  For a ULPI phy, specifies whether to use the internal or
// external supply to drive the VBus
// 0 - Internal supply (default)
// 1 - External supply
// @i2c_enable:         Specifies whether to use the I2Cinterface for a full
// speed PHY. This parameter is only applicable if phy_type
// is FS.
// 0 - No (default)
// 1 - Yes
// @ipg_isoc_en:        Indicates the IPG supports is enabled or disabled.
// 0 - Disable (default)
// 1 - Enable
// @acg_enable:		For enabling Active Clock Gating in the controller
// 0 - No
// 1 - Yes
// @ulpi_fs_ls:         Make ULPI phy operate in FS/LS mode only
// 0 - No (default)
// 1 - Yes
// @host_support_fs_ls_low_power: Specifies whether low power mode is supported
// when attached to a Full Speed or Low Speed device in
// host mode.
// 0 - Don't support low power mode (default)
// 1 - Support low power mode
// @host_ls_low_power_phy_clk: Specifies the PHY clock rate in low power mode
// when connected to a Low Speed device in host
// mode. This parameter is applicable only if
// host_support_fs_ls_low_power is enabled.
// 0 - 48 MHz
// (default when phy_type is UTMI+ or ULPI)
// 1 - 6 MHz
// (default when phy_type is Full Speed)
// @oc_disable:		Flag to disable overcurrent condition.
// 0 - Allow overcurrent condition to get detected
// 1 - Disable overcurrent condtion to get detected
// @ts_dline:           Enable Term Select Dline pulsing
// 0 - No (default)
// 1 - Yes
// @reload_ctl:         Allow dynamic reloading of HFIR register during runtime
// 0 - No (default for core < 2.92a)
// 1 - Yes (default for core >= 2.92a)
// @ahbcfg:             This field allows the default value of the GAHBCFG
// register to be overridden
// -1         - GAHBCFG value will be set to 0x06
// (INCR, default)
// all others - GAHBCFG value will be overridden with
// this value
// Not all bits can be controlled like this, the
// bits defined by GAHBCFG_CTRL_MASK are controlled
// by the driver and are ignored in this
// configuration value.
// @uframe_sched:       True to enable the microframe scheduler
// @external_id_pin_ctl: Specifies whether ID pin is handled externally.
// Disable CONIDSTSCHNG controller interrupt in such
// case.
// 0 - No (default)
// 1 - Yes
// @power_down:         Specifies whether the controller support power_down.
// If power_down is enabled, the controller will enter
// power_down in both peripheral and host mode when
// needed.
// 0 - No (default)
// 1 - Partial power down
// 2 - Hibernation
// @no_clock_gating:	Specifies whether to avoid clock gating feature.
// 0 - No (use clock gating)
// 1 - Yes (avoid it)
// @lpm:		Enable LPM support.
// 0 - No
// 1 - Yes
// @lpm_clock_gating:		Enable core PHY clock gating.
// 0 - No
// 1 - Yes
// @besl:		Enable LPM Errata support.
// 0 - No
// 1 - Yes
// @hird_threshold_en:	HIRD or HIRD Threshold enable.
// 0 - No
// 1 - Yes
// @hird_threshold:	Value of BESL or HIRD Threshold.
// @ref_clk_per:        Indicates in terms of pico seconds the period
// of ref_clk.
// 62500 - 16MHz
// 58823 - 17MHz
// 52083 - 19.2MHz
// 50000 - 20MHz
// 41666 - 24MHz
// 33333 - 30MHz (default)
// 25000 - 40MHz
// @sof_cnt_wkup_alert: Indicates in term of number of SOF's after which
// the controller should generate an interrupt if the
// device had been in L1 state until that period.
// This is used by SW to initiate Remote WakeUp in the
// controller so as to sync to the uF number from the host.
// @activate_stm_fs_transceiver: Activate internal transceiver using GGPIO
// register.
// 0 - Deactivate the transceiver (default)
// 1 - Activate the transceiver
// @activate_stm_id_vb_detection: Activate external ID pin and Vbus level
// detection using GGPIO register.
// 0 - Deactivate the external level detection (default)
// 1 - Activate the external level detection
// @activate_ingenic_overcurrent_detection: Activate Ingenic overcurrent
// detection.
// 0 - Deactivate the overcurrent detection
// 1 - Activate the overcurrent detection (default)
// @g_dma:              Enables gadget dma usage (default: autodetect).
// @g_dma_desc:         Enables gadget descriptor DMA (default: autodetect).
// @g_rx_fifo_size:	The periodic rx fifo size for the device, in
// DWORDS from 16-32768 (default: 2048 if
// possible, otherwise autodetect).
// @g_np_tx_fifo_size:	The non-periodic tx fifo size for the device in
// DWORDS from 16-32768 (default: 1024 if
// possible, otherwise autodetect).
// @g_tx_fifo_size:	An array of TX fifo sizes in dedicated fifo
// mode. Each value corresponds to one EP
// starting from EP1 (max 15 values). Sizes are
// in DWORDS with possible values from
// 16-32768 (default: 256, 256, 256, 256, 768,
// 768, 768, 768, 0, 0, 0, 0, 0, 0, 0).
// @change_speed_quirk: Change speed configuration to DWC2_SPEED_PARAM_FULL
// while full&low speed device connect. And change speed
// back to DWC2_SPEED_PARAM_HIGH while device is gone.
// 0 - No (default)
// 1 - Yes
// @service_interval:   Enable service interval based scheduling.
// 0 - No
// 1 - Yes
//
// The following parameters may be specified when starting the module. These
// parameters define how the DWC_otg controller should be configured. A
// value of -1 (or any other out of range value) for any parameter means
// to read the value from hardware (if possible) or use the builtin
// default described above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_core_params {
    pub otg_caps: usb_otg_caps,
    pub phy_type: u8,
pub const DWC2_PHY_TYPE_PARAM_FS: c_int = 0;
pub const DWC2_PHY_TYPE_PARAM_UTMI: c_int = 1;
pub const DWC2_PHY_TYPE_PARAM_ULPI: c_int = 2;
    pub speed: u8,
pub const DWC2_SPEED_PARAM_HIGH: c_int = 0;
pub const DWC2_SPEED_PARAM_FULL: c_int = 1;
pub const DWC2_SPEED_PARAM_LOW: c_int = 2;
    pub phy_utmi_width: u8,
    pub eusb2_disc: bool,
    pub phy_ulpi_ddr: bool,
    pub phy_ulpi_ext_vbus: bool,
    pub enable_dynamic_fifo: bool,
    pub en_multiple_tx_fifo: bool,
    pub i2c_enable: bool,
    pub acg_enable: bool,
    pub ulpi_fs_ls: bool,
    pub ts_dline: bool,
    pub reload_ctl: bool,
    pub uframe_sched: bool,
    pub external_id_pin_ctl: bool,
    pub power_down: c_int,
pub const DWC2_POWER_DOWN_PARAM_NONE: c_int = 0;
pub const DWC2_POWER_DOWN_PARAM_PARTIAL: c_int = 1;
pub const DWC2_POWER_DOWN_PARAM_HIBERNATION: c_int = 2;
    pub no_clock_gating: bool,
    pub lpm: bool,
    pub lpm_clock_gating: bool,
    pub besl: bool,
    pub hird_threshold_en: bool,
    pub service_interval: bool,
    pub hird_threshold: u8,
    pub activate_stm_fs_transceiver: bool,
    pub activate_stm_id_vb_detection: bool,
    pub activate_ingenic_overcurrent_detection: bool,
    pub ipg_isoc_en: bool,
    pub max_packet_count: u16,
    pub max_transfer_size: u32,
    pub ahbcfg: u32,
// GREFCLK parameters
    pub ref_clk_per: u32,
    pub sof_cnt_wkup_alert: u16,
// Host parameters
    pub host_dma: bool,
    pub dma_desc_enable: bool,
    pub dma_desc_fs_enable: bool,
    pub host_support_fs_ls_low_power: bool,
    pub host_ls_low_power_phy_clk: bool,
    pub oc_disable: bool,
    pub host_channels: u8,
    pub host_rx_fifo_size: u16,
    pub host_nperio_tx_fifo_size: u16,
    pub host_perio_tx_fifo_size: u16,
// Gadget parameters
    pub g_dma: bool,
    pub g_dma_desc: bool,
    pub g_rx_fifo_size: u32,
    pub g_np_tx_fifo_size: u32,
    pub g_tx_fifo_size: [u32; MAX_EPS_CHANNELS],
    pub change_speed_quirk: bool,
}

//
// struct dwc2_hw_params - Autodetected parameters.
//
// These parameters are the various parameters read from hardware
// registers during initialization. They typically contain the best
// supported or maximum value that can be configured in the
// corresponding dwc2_core_params value.
//
// The values that are not in dwc2_core_params are documented below.
//
// @op_mode:             Mode of Operation
// 0 - HNP- and SRP-Capable OTG (Host & Device)
// 1 - SRP-Capable OTG (Host & Device)
// 2 - Non-HNP and Non-SRP Capable OTG (Host & Device)
// 3 - SRP-Capable Device
// 4 - Non-OTG Device
// 5 - SRP-Capable Host
// 6 - Non-OTG Host
// @arch:                Architecture
// 0 - Slave only
// 1 - External DMA
// 2 - Internal DMA
// @ipg_isoc_en:        This feature indicates that the controller supports
// the worst-case scenario of Rx followed by Rx
// Interpacket Gap (IPG) (32 bitTimes) as per the utmi
// specification for any token following ISOC OUT token.
// 0 - Don't support
// 1 - Support
// @power_optimized:    Are power optimizations enabled?
// @num_dev_ep:         Number of device endpoints available
// @num_dev_in_eps:     Number of device IN endpoints available
// @num_dev_perio_in_ep: Number of device periodic IN endpoints
// available
// @dev_token_q_depth:  Device Mode IN Token Sequence Learning Queue
// Depth
// 0 to 30
// @host_perio_tx_q_depth:
// Host Mode Periodic Request Queue Depth
// 2, 4 or 8
// @nperio_tx_q_depth:
// Non-Periodic Request Queue Depth
// 2, 4 or 8
// @hs_phy_type:         High-speed PHY interface type
// 0 - High-speed interface not supported
// 1 - UTMI+
// 2 - ULPI
// 3 - UTMI+ and ULPI
// @fs_phy_type:         Full-speed PHY interface type
// 0 - Full speed interface not supported
// 1 - Dedicated full speed interface
// 2 - FS pins shared with UTMI+ pins
// 3 - FS pins shared with ULPI pins
// @total_fifo_size:    Total internal RAM for FIFOs (bytes)
// @hibernation:	Is hibernation enabled?
// @utmi_phy_data_width: UTMI+ PHY data width
// 0 - 8 bits
// 1 - 16 bits
// 2 - 8 or 16 bits
// @snpsid:             Value from SNPSID register
// @dev_ep_dirs:        Direction of device endpoints (GHWCFG1)
// @g_tx_fifo_size:	Power-on values of TxFIFO sizes
// @dma_desc_enable:    When DMA mode is enabled, specifies whether to use
// address DMA mode or descriptor DMA mode for accessing
// the data FIFOs. The driver will automatically detect the
// value for this if none is specified.
// 0 - Address DMA
// 1 - Descriptor DMA (default, if available)
// @enable_dynamic_fifo: 0 - Use coreConsultant-specified FIFO size parameters
// 1 - Allow dynamic FIFO sizing (default, if available)
// @en_multiple_tx_fifo: Specifies whether dedicated per-endpoint transmit FIFOs
// are enabled for non-periodic IN endpoints in device
// mode.
// @host_nperio_tx_fifo_size: Number of 4-byte words in the non-periodic Tx FIFO
// in host mode when dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @host_perio_tx_fifo_size: Number of 4-byte words in the periodic Tx FIFO in
// host mode when dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @max_transfer_size:  The maximum transfer size supported, in bytes
// 2047 to 65,535
// Actual maximum value is autodetected and also
// the default.
// @max_packet_count:   The maximum number of packets in a transfer
// 15 to 511
// Actual maximum value is autodetected and also
// the default.
// @host_channels:      The number of host channel registers to use
// 1 to 16
// Actual maximum value is autodetected and also
// the default.
// @dev_nperio_tx_fifo_size: Number of 4-byte words in the non-periodic Tx FIFO
// in device mode when dynamic FIFO sizing is enabled
// 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @i2c_enable:         Specifies whether to use the I2Cinterface for a full
// speed PHY. This parameter is only applicable if phy_type
// is FS.
// 0 - No (default)
// 1 - Yes
// @acg_enable:		For enabling Active Clock Gating in the controller
// 0 - Disable
// 1 - Enable
// @lpm_mode:		For enabling Link Power Management in the controller
// 0 - Disable
// 1 - Enable
// @rx_fifo_size:	Number of 4-byte words in the  Rx FIFO when dynamic
// FIFO sizing is enabled 16 to 32768
// Actual maximum value is autodetected and also
// the default.
// @service_interval_mode: For enabling service interval based scheduling in the
// controller.
// 0 - Disable
// 1 - Enable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hw_params {
    pub op_mode:3: unsigned,
    pub arch:2: unsigned,
    pub dma_desc_enable:1: unsigned,
    pub enable_dynamic_fifo:1: unsigned,
    pub en_multiple_tx_fifo:1: unsigned,
    pub rx_fifo_size:16: unsigned,
    pub host_nperio_tx_fifo_size:16: unsigned,
    pub dev_nperio_tx_fifo_size:16: unsigned,
    pub host_perio_tx_fifo_size:16: unsigned,
    pub nperio_tx_q_depth:3: unsigned,
    pub host_perio_tx_q_depth:3: unsigned,
    pub dev_token_q_depth:5: unsigned,
    pub max_transfer_size:26: unsigned,
    pub max_packet_count:11: unsigned,
    pub host_channels:5: unsigned,
    pub hs_phy_type:2: unsigned,
    pub fs_phy_type:2: unsigned,
    pub i2c_enable:1: unsigned,
    pub acg_enable:1: unsigned,
    pub num_dev_ep:4: unsigned,
    pub 4: unsigned num_dev_in_eps :,
    pub num_dev_perio_in_ep:4: unsigned,
    pub total_fifo_size:16: unsigned,
    pub power_optimized:1: unsigned,
    pub hibernation:1: unsigned,
    pub utmi_phy_data_width:2: unsigned,
    pub lpm_mode:1: unsigned,
    pub ipg_isoc_en:1: unsigned,
    pub service_interval_mode:1: unsigned,
    pub snpsid: u32,
    pub dev_ep_dirs: u32,
    pub g_tx_fifo_size: [u32; MAX_EPS_CHANNELS],
}

// Size of control and EP0 buffers
pub const DWC2_CTRL_BUFF_SIZE: c_int = 8;
//
// struct dwc2_gregs_backup - Holds global registers state before
// entering partial power down
// @gintsts:		Backup of GINTSTS register
// @gotgctl:		Backup of GOTGCTL register
// @gintmsk:		Backup of GINTMSK register
// @gahbcfg:		Backup of GAHBCFG register
// @gusbcfg:		Backup of GUSBCFG register
// @grxfsiz:		Backup of GRXFSIZ register
// @gnptxfsiz:		Backup of GNPTXFSIZ register
// @gi2cctl:		Backup of GI2CCTL register
// @glpmcfg:		Backup of GLPMCFG register
// @gdfifocfg:		Backup of GDFIFOCFG register
// @pcgcctl:		Backup of PCGCCTL register
// @pcgcctl1:		Backup of PCGCCTL1 register
// @dtxfsiz:		Backup of DTXFSIZ registers for each endpoint
// @gpwrdn:		Backup of GPWRDN register
// @valid:		True if registers values backuped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_gregs_backup {
    pub gintsts: u32,
    pub gotgctl: u32,
    pub gintmsk: u32,
    pub gahbcfg: u32,
    pub gusbcfg: u32,
    pub grxfsiz: u32,
    pub gnptxfsiz: u32,
    pub gi2cctl: u32,
    pub glpmcfg: u32,
    pub pcgcctl: u32,
    pub pcgcctl1: u32,
    pub gdfifocfg: u32,
    pub gpwrdn: u32,
    pub valid: bool,
}

//
// struct dwc2_dregs_backup - Holds device registers state before
// entering partial power down
// @dcfg:		Backup of DCFG register
// @dctl:		Backup of DCTL register
// @daintmsk:		Backup of DAINTMSK register
// @diepmsk:		Backup of DIEPMSK register
// @doepmsk:		Backup of DOEPMSK register
// @diepctl:		Backup of DIEPCTL register
// @dieptsiz:		Backup of DIEPTSIZ register
// @diepdma:		Backup of DIEPDMA register
// @doepctl:		Backup of DOEPCTL register
// @doeptsiz:		Backup of DOEPTSIZ register
// @doepdma:		Backup of DOEPDMA register
// @dtxfsiz:		Backup of DTXFSIZ registers for each endpoint
// @valid:      True if registers values backuped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_dregs_backup {
    pub dcfg: u32,
    pub dctl: u32,
    pub daintmsk: u32,
    pub diepmsk: u32,
    pub doepmsk: u32,
    pub diepctl: [u32; MAX_EPS_CHANNELS],
    pub dieptsiz: [u32; MAX_EPS_CHANNELS],
    pub diepdma: [u32; MAX_EPS_CHANNELS],
    pub doepctl: [u32; MAX_EPS_CHANNELS],
    pub doeptsiz: [u32; MAX_EPS_CHANNELS],
    pub doepdma: [u32; MAX_EPS_CHANNELS],
    pub dtxfsiz: [u32; MAX_EPS_CHANNELS],
    pub valid: bool,
}

//
// struct dwc2_hregs_backup - Holds host registers state before
// entering partial power down
// @hcfg:		Backup of HCFG register
// @hflbaddr:		Backup of HFLBADDR register
// @haintmsk:		Backup of HAINTMSK register
// @hcchar:		Backup of HCCHAR register
// @hcsplt:		Backup of HCSPLT register
// @hcintmsk:		Backup of HCINTMSK register
// @hctsiz:		Backup of HCTSIZ register
// @hdma:		Backup of HCDMA register
// @hcdmab:		Backup of HCDMAB register
// @hprt0:		Backup of HPTR0 register
// @hfir:		Backup of HFIR register
// @hptxfsiz:		Backup of HPTXFSIZ register
// @valid:      True if registers values backuped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hregs_backup {
    pub hcfg: u32,
    pub hflbaddr: u32,
    pub haintmsk: u32,
    pub hcchar: [u32; MAX_EPS_CHANNELS],
    pub hcsplt: [u32; MAX_EPS_CHANNELS],
    pub hcintmsk: [u32; MAX_EPS_CHANNELS],
    pub hctsiz: [u32; MAX_EPS_CHANNELS],
    pub hcidma: [u32; MAX_EPS_CHANNELS],
    pub hcidmab: [u32; MAX_EPS_CHANNELS],
    pub hprt0: u32,
    pub hfir: u32,
    pub hptxfsiz: u32,
    pub valid: bool,
}

//
// Constants related to high speed periodic scheduling
//
// We have a periodic schedule that is DWC2_HS_SCHEDULE_UFRAMES long.  From a
// reservation point of view it's assumed that the schedule goes right back to
// the beginning after the end of the schedule.
//
// What does that mean for scheduling things with a long interval?  It means
// we'll reserve time for them in every possible microframe that they could
// ever be scheduled in.  ...but we'll still only actually schedule them as
// often as they were requested.
//
// We keep our schedule in a "bitmap" structure.  This simplifies having
// to keep track of and merge intervals: we just let the bitmap code do most
// of the heavy lifting.  In a way scheduling is much like memory allocation.
//
// We schedule 100us per uframe or 80% of 125us (the maximum amount you're
// supposed to schedule for periodic transfers).  That's according to spec.
//
// Note that though we only schedule 80% of each microframe, the bitmap that we
// keep the schedule in is tightly packed (AKA it doesn't have 100us worth of
// space for each uFrame).
//
// Requirements:
// - DWC2_HS_SCHEDULE_UFRAMES must even divide 0x4000 (HFNUM_MAX_FRNUM + 1)
// - DWC2_HS_SCHEDULE_UFRAMES must be 8 times DWC2_LS_SCHEDULE_FRAMES (probably
// could be any multiple of 8 times DWC2_LS_SCHEDULE_FRAMES, but there might
// be bugs).  The 8 comes from the USB spec: number of microframes per frame.
//
pub const DWC2_US_PER_UFRAME: c_int = 125;
pub const DWC2_HS_PERIODIC_US_PER_UFRAME: c_int = 100;
pub const DWC2_HS_SCHEDULE_UFRAMES: c_int = 8;

//
// Constants related to low speed scheduling
//
// For high speed we schedule every 1us.  For low speed that's a bit overkill,
// so we make up a unit called a "slice" that's worth 25us.  There are 40
// slices in a full frame and we can schedule 36 of those (90%) for periodic
// transfers.
//
// Our low speed schedule can be as short as 1 frame or could be longer.  When
// we only schedule 1 frame it means that we'll need to reserve a time every
// frame even for things that only transfer very rarely, so something that runs
// every 2048 frames will get time reserved in every frame.  Our low speed
// schedule can be longer and we'll be able to handle more overlap, but that
// will come at increased memory cost and increased time to schedule.
//
// Note: one other advantage of a short low speed schedule is that if we mess
// up and miss scheduling we can jump in and use any of the slots that we
// happened to reserve.
//
// With 25 us per slice and 1 frame in the schedule, we only need 4 bytes for
// the schedule.  There will be one schedule per TT.
//
// Requirements:
// - DWC2_US_PER_SLICE must evenly divide DWC2_LS_PERIODIC_US_PER_FRAME.
//
pub const DWC2_US_PER_SLICE: c_int = 25;

pub const DWC2_LS_SCHEDULE_FRAMES: c_int = 1;

//
// struct dwc2_hsotg - Holds the state of the driver, including the non-periodic
// and periodic schedules
//
// These are common for both host and peripheral modes:
//
// @dev:                The struct device pointer
// @regs:		Pointer to controller regs
// @hw_params:          Parameters that were autodetected from the
// hardware registers
// @params:	Parameters that define how the core should be configured
// @op_state:           The operational State, during transitions (a_host=>
// a_peripheral and b_device=>b_host) this may not match
// the core, but allows the software to determine
// transitions
// @dr_mode:            Requested mode of operation, one of following:
// - USB_DR_MODE_PERIPHERAL
// - USB_DR_MODE_HOST
// - USB_DR_MODE_OTG
// @role_sw:		usb_role_switch handle
// @role_sw_default_mode: default operation mode of controller while usb role
// is USB_ROLE_NONE
// @hcd_enabled:	Host mode sub-driver initialization indicator.
// @gadget_enabled:	Peripheral mode sub-driver initialization indicator.
// @ll_hw_enabled:	Status of low-level hardware resources.
// @hibernated:		True if core is hibernated
// @in_ppd:		True if core is partial power down mode.
// @bus_suspended:	True if bus is suspended
// @reset_phy_on_wake:	Quirk saying that we should assert PHY reset on a
// remote wakeup.
// @phy_off_for_suspend: Status of whether we turned the PHY off at suspend.
// @need_phy_for_wake:	Quirk saying that we should keep the PHY on at
// suspend if we need USB to wake us up.
// @frame_number:       Frame number read from the core. For both device
// and host modes. The value ranges are from 0
// to HFNUM_MAX_FRNUM.
// @phy:                The otg phy transceiver structure for phy control.
// @uphy:               The otg phy transceiver structure for old USB phy
// control.
// @plat:               The platform specific configuration data. This can be
// removed once all SoCs support usb transceiver.
// @supplies:           Definition of USB power supplies
// @vbus_supply:        Regulator supplying vbus.
// @usb33d:		Optional 3.3v regulator used on some stm32 devices to
// supply ID and VBUS detection hardware.
// @lock:		Spinlock that protects all the driver data structures
// @priv:		Stores a pointer to the struct usb_hcd
// @queuing_high_bandwidth: True if multiple packets of a high-bandwidth
// transfer are in process of being queued
// @srp_success:        Stores status of SRP request in the case of a FS PHY
// with an I2C interface
// @wq_otg:             Workqueue object used for handling of some interrupts
// @wf_otg:             Work object for handling Connector ID Status Change
// interrupt
// @wkp_timer:          Timer object for handling Wakeup Detected interrupt
// @lx_state:           Lx state of connected device
// @gr_backup: Backup of global registers during suspend
// @dr_backup: Backup of device registers during suspend
// @hr_backup: Backup of host registers during suspend
// @needs_byte_swap:		Specifies whether the opposite endianness.
//
// These are for host mode:
//
// @flags:              Flags for handling root port state changes
// @flags.d32:          Contain all root port flags
// @flags.b:            Separate root port flags from each other
// @flags.b.port_connect_status_change: True if root port connect status
// changed
// @flags.b.port_connect_status: True if device connected to root port
// @flags.b.port_reset_change: True if root port reset status changed
// @flags.b.port_enable_change: True if root port enable status changed
// @flags.b.port_suspend_change: True if root port suspend status changed
// @flags.b.port_over_current_change: True if root port over current state
// changed.
// @flags.b.port_l1_change: True if root port l1 status changed
// @flags.b.reserved:   Reserved bits of root port register
// @non_periodic_sched_inactive: Inactive QHs in the non-periodic schedule.
// Transfers associated with these QHs are not currently
// assigned to a host channel.
// @non_periodic_sched_active: Active QHs in the non-periodic schedule.
// Transfers associated with these QHs are currently
// assigned to a host channel.
// @non_periodic_qh_ptr: Pointer to next QH to process in the active
// non-periodic schedule
// @non_periodic_sched_waiting: Waiting QHs in the non-periodic schedule.
// Transfers associated with these QHs are not currently
// assigned to a host channel.
// @periodic_sched_inactive: Inactive QHs in the periodic schedule. This is a
// list of QHs for periodic transfers that are _not_
// scheduled for the next frame. Each QH in the list has an
// interval counter that determines when it needs to be
// scheduled for execution. This scheduling mechanism
// allows only a simple calculation for periodic bandwidth
// used (i.e. must assume that all periodic transfers may
// need to execute in the same frame). However, it greatly
// simplifies scheduling and should be sufficient for the
// vast majority of OTG hosts, which need to connect to a
// small number of peripherals at one time. Items move from
// this list to periodic_sched_ready when the QH interval
// counter is 0 at SOF.
// @periodic_sched_ready:  List of periodic QHs that are ready for execution in
// the next frame, but have not yet been assigned to host
// channels. Items move from this list to
// periodic_sched_assigned as host channels become
// available during the current frame.
// @periodic_sched_assigned: List of periodic QHs to be executed in the next
// frame that are assigned to host channels. Items move
// from this list to periodic_sched_queued as the
// transactions for the QH are queued to the DWC_otg
// controller.
// @periodic_sched_queued: List of periodic QHs that have been queued for
// execution. Items move from this list to either
// periodic_sched_inactive or periodic_sched_ready when the
// channel associated with the transfer is released. If the
// interval for the QH is 1, the item moves to
// periodic_sched_ready because it must be rescheduled for
// the next frame. Otherwise, the item moves to
// periodic_sched_inactive.
// @split_order:        List keeping track of channels doing splits, in order.
// @periodic_usecs:     Total bandwidth claimed so far for periodic transfers.
// This value is in microseconds per (micro)frame. The
// assumption is that all periodic transfers may occur in
// the same (micro)frame.
// @hs_periodic_bitmap: Bitmap used by the microframe scheduler any time the
// host is in high speed mode; low speed schedules are
// stored elsewhere since we need one per TT.
// @periodic_qh_count:  Count of periodic QHs, if using several eps. Used for
// SOF enable/disable.
// @free_hc_list:       Free host channels in the controller. This is a list of
// struct dwc2_host_chan items.
// @periodic_channels:  Number of host channels assigned to periodic transfers.
// Currently assuming that there is a dedicated host
// channel for each periodic transaction and at least one
// host channel is available for non-periodic transactions.
// @non_periodic_channels: Number of host channels assigned to non-periodic
// transfers
// @available_host_channels: Number of host channels available for the
// microframe scheduler to use
// @hc_ptr_array:       Array of pointers to the host channel descriptors.
// Allows accessing a host channel descriptor given the
// host channel number. This is useful in interrupt
// handlers.
// @status_buf:         Buffer used for data received during the status phase of
// a control transfer.
// @status_buf_dma:     DMA address for status_buf
// @start_work:         Delayed work for handling host A-cable connection
// @reset_work:         Delayed work for handling a port reset
// @phy_reset_work:     Work structure for doing a PHY reset
// @otg_port:           OTG port number
// @frame_list:         Frame list
// @frame_list_dma:     Frame list DMA address
// @frame_list_sz:      Frame list size
// @desc_gen_cache:     Kmem cache for generic descriptors
// @desc_hsisoc_cache:  Kmem cache for hs isochronous descriptors
// @unaligned_cache:    Kmem cache for DMA mode to handle non-aligned buf
//
// These are for peripheral mode:
//
// @driver:             USB gadget driver
// @dedicated_fifos:    Set if the hardware has dedicated IN-EP fifos.
// @num_of_eps:         Number of available EPs (excluding EP0)
// @debug_root:         Root directrory for debugfs.
// @ep0_reply:          Request used for ep0 reply.
// @ep0_buff:           Buffer for EP0 reply data, if needed.
// @ctrl_buff:          Buffer for EP0 control requests.
// @ctrl_req:           Request for EP0 control packets.
// @ep0_state:          EP0 control transfers state
// @delayed_status:		true when gadget driver asks for delayed status
// @test_mode:          USB test mode requested by the host
// @remote_wakeup_allowed: True if device is allowed to wake-up host by
// remote-wakeup signalling
// @setup_desc_dma:	EP0 setup stage desc chain DMA address
// @setup_desc:		EP0 setup stage desc chain pointer
// @ctrl_in_desc_dma:	EP0 IN data phase desc chain DMA address
// @ctrl_in_desc:	EP0 IN data phase desc chain pointer
// @ctrl_out_desc_dma:	EP0 OUT data phase desc chain DMA address
// @ctrl_out_desc:	EP0 OUT data phase desc chain pointer
// @irq:		Interrupt request line number
// @clk:		Pointer to otg clock
// @utmi_clk:		Pointer to utmi_clk clock
// @reset:		Pointer to dwc2 reset controller
// @reset_ecc:          Pointer to dwc2 optional reset controller in Stratix10.
// @regset:		A pointer to a struct debugfs_regset32, which contains
// a pointer to an array of register definitions, the
// array size and the base address where the register bank
// is to be found.
// @last_frame_num:	Number of last frame. Range from 0 to  32768
// @frame_num_array:    Used only  if CONFIG_USB_DWC2_TRACK_MISSED_SOFS is
// defined, for missed SOFs tracking. Array holds that
// frame numbers, which not equal to last_frame_num +1
// @last_frame_num_array:   Used only  if CONFIG_USB_DWC2_TRACK_MISSED_SOFS is
// defined, for missed SOFs tracking.
// If current_frame_number != last_frame_num+1
// then last_frame_num added to this array
// @frame_num_idx:	Actual size of frame_num_array and last_frame_num_array
// @dumped_frame_num_array:	1 - if missed SOFs frame numbers dumbed
// 0 - if missed SOFs frame numbers not dumbed
// @fifo_mem:			Total internal RAM for FIFOs (bytes)
// @fifo_map:		Each bit intend for concrete fifo. If that bit is set,
// then that fifo is used
// @gadget:		Represents a usb gadget device
// @connected:		Used in slave mode. True if device connected with host
// @eps_in:		The IN endpoints being supplied to the gadget framework
// @eps_out:		The OUT endpoints being supplied to the gadget framework
// @new_connection:	Used in host mode. True if there are new connected
// device
// @enabled:		Indicates the enabling state of controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwc2_hsotg {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
// Params detected from hardware
    pub hw_params: dwc2_hw_params,
// Params to actually use
    pub params: dwc2_core_params,
    pub op_state: usb_otg_state,
    pub dr_mode: usb_dr_mode,
    pub role_sw: *mut usb_role_switch,
    pub role_sw_default_mode: usb_dr_mode,
    pub hcd_enabled:1: c_uint,
    pub gadget_enabled:1: c_uint,
    pub ll_hw_enabled:1: c_uint,
    pub hibernated:1: c_uint,
    pub in_ppd:1: c_uint,
    pub bus_suspended: bool,
    pub reset_phy_on_wake:1: c_uint,
    pub need_phy_for_wake:1: c_uint,
    pub phy_off_for_suspend:1: c_uint,
    pub frame_number: u16,
    pub phy: *mut phy,
    pub uphy: *mut usb_phy,
    pub plat: *mut dwc2_hsotg_plat,
    pub supplies: [regulator_bulk_data; DWC2_NUM_SUPPLIES],
    pub vbus_supply: *mut regulator,
    pub usb33d: *mut regulator,
    pub lock: spinlock_t,
    pub priv: *mut c_void,
    pub irq: c_int,
    pub clk: *mut clk,
    pub utmi_clk: *mut clk,
    pub reset: *mut reset_control,
    pub reset_ecc: *mut reset_control,
    pub queuing_high_bandwidth:1: c_uint,
    pub srp_success:1: c_uint,
    pub wq_otg: *mut workqueue_struct,
    pub wf_otg: work_struct,
    pub wkp_timer: timer_list,
    pub lx_state: dwc2_lx_state,
    pub gr_backup: dwc2_gregs_backup,
    pub dr_backup: dwc2_dregs_backup,
    pub hr_backup: dwc2_hregs_backup,
    pub debug_root: *mut dentry,
    pub regset: *mut debugfs_regset32,
    pub needs_byte_swap: bool,
// DWC OTG HW Release versions
pub const DWC2_CORE_REV_4_30a: c_uint = 0x4f54430a;
pub const DWC2_CORE_REV_2_71a: c_uint = 0x4f54271a;
pub const DWC2_CORE_REV_2_72a: c_uint = 0x4f54272a;
pub const DWC2_CORE_REV_2_80a: c_uint = 0x4f54280a;
pub const DWC2_CORE_REV_2_90a: c_uint = 0x4f54290a;
pub const DWC2_CORE_REV_2_91a: c_uint = 0x4f54291a;
pub const DWC2_CORE_REV_2_92a: c_uint = 0x4f54292a;
pub const DWC2_CORE_REV_2_94a: c_uint = 0x4f54294a;
pub const DWC2_CORE_REV_3_00a: c_uint = 0x4f54300a;
pub const DWC2_CORE_REV_3_10a: c_uint = 0x4f54310a;
pub const DWC2_CORE_REV_4_00a: c_uint = 0x4f54400a;
pub const DWC2_CORE_REV_4_20a: c_uint = 0x4f54420a;
pub const DWC2_CORE_REV_5_00a: c_uint = 0x4f54500a;
pub const DWC2_FS_IOT_REV_1_00a: c_uint = 0x5531100a;
pub const DWC2_HS_IOT_REV_1_00a: c_uint = 0x5532100a;
pub const DWC2_HS_IOT_REV_5_00a: c_uint = 0x5532500a;
pub const DWC2_CORE_REV_MASK: c_uint = 0x0000ffff;
// DWC OTG HW Core ID
pub const DWC2_OTG_ID: c_uint = 0x4f540000;
pub const DWC2_FS_IOT_ID: c_uint = 0x55310000;
pub const DWC2_HS_IOT_ID: c_uint = 0x55320000;

#[repr(C)]
#[derive(Copy, Clone)]
pub union dwc2_hcd_internal_flags {
    pub d32: u32,
    pub port_connect_status_change:1: unsigned,
    pub port_connect_status:1: unsigned,
    pub port_reset_change:1: unsigned,
    pub port_enable_change:1: unsigned,
    pub port_suspend_change:1: unsigned,
    pub port_over_current_change:1: unsigned,
    pub port_l1_change:1: unsigned,
    pub reserved:25: unsigned,
    pub b: },
    pub flags: },
    pub non_periodic_sched_inactive: list_head,
    pub non_periodic_sched_waiting: list_head,
    pub non_periodic_sched_active: list_head,
    pub non_periodic_qh_ptr: *mut list_head,
    pub periodic_sched_inactive: list_head,
    pub periodic_sched_ready: list_head,
    pub periodic_sched_assigned: list_head,
    pub periodic_sched_queued: list_head,
    pub split_order: list_head,
    pub periodic_usecs: u16,
    pub DWC2_HS_SCHEDULE_US): DECLARE_BITMAP(hs_periodic_bitmap,,
    pub periodic_qh_count: u16,
    pub new_connection: bool,
    pub last_frame_num: u16,

pub const FRAME_NUM_ARRAY_SIZE: c_int = 1000;
    pub frame_num_array: *mut u16,
    pub last_frame_num_array: *mut u16,
    pub frame_num_idx: c_int,
    pub dumped_frame_num_array: c_int,

    pub free_hc_list: list_head,
    pub periodic_channels: c_int,
    pub non_periodic_channels: c_int,
    pub available_host_channels: c_int,
    pub hc_ptr_array: [*mut dwc2_host_chan; MAX_EPS_CHANNELS],
    pub status_buf: *mut u8,
    pub status_buf_dma: dma_addr_t,
pub const DWC2_HCD_STATUS_BUF_SIZE: c_int = 64;
    pub start_work: delayed_work,
    pub reset_work: delayed_work,
    pub phy_reset_work: work_struct,
    pub otg_port: u8,
    pub frame_list: *mut u32,
    pub frame_list_dma: dma_addr_t,
    pub frame_list_sz: u32,
    pub desc_gen_cache: *mut kmem_cache,
    pub desc_hsisoc_cache: *mut kmem_cache,
    pub unaligned_cache: *mut kmem_cache,
pub const DWC2_KMEM_UNALIGNED_BUF_SIZE: c_int = 1024;

// Gadget structures
    pub driver: *mut usb_gadget_driver,
    pub fifo_mem: c_int,
    pub dedicated_fifos:1: c_uint,
    pub num_of_eps: c_uchar,
    pub fifo_map: u32,
    pub ep0_reply: *mut usb_request,
    pub ctrl_req: *mut usb_request,
    pub ep0_buff: *mut c_void,
    pub ctrl_buff: *mut c_void,
    pub ep0_state: dwc2_ep0_state,
    pub 1: unsigned delayed_status :,
    pub test_mode: u8,
    pub setup_desc_dma: [dma_addr_t; 2],
    pub setup_desc: [*mut dwc2_dma_desc; 2],
    pub ctrl_in_desc_dma: dma_addr_t,
    pub ctrl_in_desc: *mut dwc2_dma_desc,
    pub ctrl_out_desc_dma: dma_addr_t,
    pub ctrl_out_desc: *mut dwc2_dma_desc,
    pub gadget: usb_gadget,
    pub enabled:1: c_uint,
    pub connected:1: c_uint,
    pub remote_wakeup_allowed:1: c_uint,
    pub eps_in: [*mut dwc2_hsotg_ep; MAX_EPS_CHANNELS],
    pub eps_out: [*mut dwc2_hsotg_ep; MAX_EPS_CHANNELS],
}

// Normal architectures just use readl/write
extern "C" {
    pub fn swab32(_arg: val) -> return;
}

// buf++ = x;
// Reasons for halting a host channel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dwc2_halt_status {
    DWC2_HC_XFER_NO_HALT_STATUS,
    DWC2_HC_XFER_COMPLETE,
    DWC2_HC_XFER_URB_COMPLETE,
    DWC2_HC_XFER_ACK,
    DWC2_HC_XFER_NAK,
    DWC2_HC_XFER_NYET,
    DWC2_HC_XFER_STALL,
    DWC2_HC_XFER_XACT_ERR,
    DWC2_HC_XFER_FRAME_OVERRUN,
    DWC2_HC_XFER_BABBLE_ERR,
    DWC2_HC_XFER_DATA_TOGGLE_ERR,
    DWC2_HC_XFER_AHB_ERR,
    DWC2_HC_XFER_PERIODIC_INCOMPLETE,
    DWC2_HC_XFER_URB_DEQUEUE,
}

// Core version information
//
// The following functions support initialization of the core driver component
// and the DWC_otg controller
//
extern "C" {
    pub fn dwc2_core_reset(hsotg: *mut dwc2_hsotg, skip_wait: bool) -> c_int;
}
extern "C" {
    pub fn dwc2_enter_partial_power_down(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_enter_hibernation(hsotg: *mut dwc2_hsotg, is_host: c_int) -> c_int;
}
extern "C" {
    pub fn dwc2_init_fs_ls_pclk_sel(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_phy_init(hsotg: *mut dwc2_hsotg, select_phy: bool) -> c_int;
}
extern "C" {
    pub fn dwc2_force_mode(hsotg: *mut dwc2_hsotg, host: bool);
}
extern "C" {
    pub fn dwc2_force_dr_mode(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_is_controller_alive(hsotg: *mut dwc2_hsotg) -> bool;
}
extern "C" {
    pub fn dwc2_check_core_version(hsotg: *mut dwc2_hsotg) -> c_int;
}
//
// Common core Functions.
// The following functions support managing the DWC_otg controller in either
// device or host mode.
//
extern "C" {
    pub fn dwc2_read_packet(hsotg: *mut dwc2_hsotg, dest: *mut u8, bytes: u16);
}
extern "C" {
    pub fn dwc2_flush_tx_fifo(hsotg: *mut dwc2_hsotg, num: c_int);
}
extern "C" {
    pub fn dwc2_flush_rx_fifo(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_enable_global_interrupts(hcd: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_disable_global_interrupts(hcd: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_backup_global_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_restore_global_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_enable_acg(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_wakeup_from_lpm_l1(hsotg: *mut dwc2_hsotg, remotewakeup: bool);
}
// This function should be called on every hardware interrupt.
extern "C" {
    pub fn dwc2_handle_common_intr(irq: c_int, dev: *mut c_void) -> irqreturn_t;
}
// The device ID match table
extern "C" {
    pub fn dwc2_lowlevel_hw_enable(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_lowlevel_hw_disable(hsotg: *mut dwc2_hsotg) -> c_int;
}
// Common polling functions
// Parameters
extern "C" {
    pub fn dwc2_get_hwparams(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_init_params(hsotg: *mut dwc2_hsotg) -> c_int;
}
//
// The following functions check the controller's OTG operation mode
// capability (GHWCFG2.OTG_MODE).
//
// These functions can be used before the internal hsotg->hw_params
// are read in and cached so they always read directly from the
// GHWCFG2 register.
//
extern "C" {
    pub fn dwc2_op_mode(hsotg: *mut dwc2_hsotg) -> c_uint;
}
extern "C" {
    pub fn dwc2_hw_is_otg(hsotg: *mut dwc2_hsotg) -> bool;
}
extern "C" {
    pub fn dwc2_hw_is_host(hsotg: *mut dwc2_hsotg) -> bool;
}
extern "C" {
    pub fn dwc2_hw_is_device(hsotg: *mut dwc2_hsotg) -> bool;
}
//
// Returns the mode of operation, host or device
//
extern "C" {
    pub fn dwc2_drd_init(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_drd_suspend(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_drd_resume(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_drd_exit(hsotg: *mut dwc2_hsotg);
}
//
// Dump core registers and SPRAM
//
extern "C" {
    pub fn dwc2_dump_dev_registers(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_dump_host_registers(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_dump_global_registers(hsotg: *mut dwc2_hsotg);
}
// Gadget defines

extern "C" {
    pub fn dwc2_hsotg_remove(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hsotg_suspend(dwc2: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hsotg_resume(dwc2: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_gadget_init(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hsotg_core_disconnect(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_hsotg_core_connect(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_hsotg_disconnect(dwc2: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_hsotg_set_test_mode(hsotg: *mut dwc2_hsotg, testmode: c_int) -> c_int;
}

extern "C" {
    pub fn dwc2_backup_device_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_restore_device_registers(hsotg: *mut dwc2_hsotg, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn dwc2_gadget_enter_hibernation(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_gadget_enter_partial_power_down(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_gadget_enter_clock_gating(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_hsotg_tx_fifo_count(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hsotg_tx_fifo_total_depth(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hsotg_tx_fifo_average_depth(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_gadget_init_lpm(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_gadget_program_ref_clk(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_gadget_backup_critical_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}

extern "C" {
    pub fn dwc2_hcd_get_frame_number(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_hcd_get_future_frame_number(hsotg: *mut dwc2_hsotg, us: c_int) -> c_int;
}
extern "C" {
    pub fn dwc2_hcd_connect(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_hcd_disconnect(hsotg: *mut dwc2_hsotg, force: bool);
}
extern "C" {
    pub fn dwc2_hcd_start(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_core_init(hsotg: *mut dwc2_hsotg, initial_setup: bool) -> c_int;
}
extern "C" {
    pub fn dwc2_port_suspend(hsotg: *mut dwc2_hsotg, windex: u16) -> c_int;
}
extern "C" {
    pub fn dwc2_port_resume(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_backup_host_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_restore_host_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_host_enter_hibernation(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_host_enter_partial_power_down(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_host_enter_clock_gating(hsotg: *mut dwc2_hsotg);
}
extern "C" {
    pub fn dwc2_host_exit_clock_gating(hsotg: *mut dwc2_hsotg, rem_wakeup: c_int);
}
extern "C" {
    pub fn dwc2_host_can_poweroff_phy(dwc2: *mut dwc2_hsotg) -> bool;
}
extern "C" {
    pub fn dwc2_host_backup_critical_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}
extern "C" {
    pub fn dwc2_host_restore_critical_registers(hsotg: *mut dwc2_hsotg) -> c_int;
}

