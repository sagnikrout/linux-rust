//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/cdns3/cdns3-gadget.h
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
// USBSS device controller driver header file
//
// Copyright (C) 2018-2019 Cadence.
// Copyright (C) 2017-2018 NXP
//
// Author: Pawel Laszczak <pawell@cadence.com>
// Pawel Jez <pjez@cadence.com>
// Peter Chen <peter.chen@nxp.com>
//

//
// USBSS-DEV register interface.
// This corresponds to the USBSS Device Controller Interface
//
// struct cdns3_usb_regs - device controller registers.
// @usb_conf:      Global Configuration.
// @usb_sts:       Global Status.
// @usb_cmd:       Global Command.
// @usb_itpn:      ITP/SOF number.
// @usb_lpm:       Global Command.
// @usb_ien:       USB Interrupt Enable.
// @usb_ists:      USB Interrupt Status.
// @ep_sel:        Endpoint Select.
// @ep_traddr:     Endpoint Transfer Ring Address.
// @ep_cfg:        Endpoint Configuration.
// @ep_cmd:        Endpoint Command.
// @ep_sts:        Endpoint Status.
// @ep_sts_sid:    Endpoint Status.
// @ep_sts_en:     Endpoint Status Enable.
// @drbl:          Doorbell.
// @ep_ien:        EP Interrupt Enable.
// @ep_ists:       EP Interrupt Status.
// @usb_pwr:       Global Power Configuration.
// @usb_conf2:     Global Configuration 2.
// @usb_cap1:      Capability 1.
// @usb_cap2:      Capability 2.
// @usb_cap3:      Capability 3.
// @usb_cap4:      Capability 4.
// @usb_cap5:      Capability 5.
// @usb_cap6:      Capability 6.
// @usb_cpkt1:     Custom Packet 1.
// @usb_cpkt2:     Custom Packet 2.
// @usb_cpkt3:     Custom Packet 3.
// @ep_dma_ext_addr: Upper address for DMA operations.
// @buf_addr:      Address for On-chip Buffer operations.
// @buf_data:      Data for On-chip Buffer operations.
// @buf_ctrl:      On-chip Buffer Access Control.
// @dtrans:        DMA Transfer Mode.
// @tdl_from_trb:  Source of TD Configuration.
// @tdl_beh:       TDL Behavior Configuration.
// @ep_tdl:        Endpoint TDL.
// @tdl_beh2:      TDL Behavior 2 Configuration.
// @dma_adv_td:    DMA Advance TD Configuration.
// @reserved1:     Reserved.
// @cfg_regs:      Configuration.
// @reserved2:     Reserved.
// @dma_axi_ctrl:  AXI Control.
// @dma_axi_id:    AXI ID register.
// @dma_axi_cap:   AXI Capability.
// @dma_axi_ctrl0: AXI Control 0.
// @dma_axi_ctrl1: AXI Control 1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_usb_regs {
    pub usb_conf: __le32,
    pub usb_sts: __le32,
    pub usb_cmd: __le32,
    pub usb_itpn: __le32,
    pub usb_lpm: __le32,
    pub usb_ien: __le32,
    pub usb_ists: __le32,
    pub ep_sel: __le32,
    pub ep_traddr: __le32,
    pub ep_cfg: __le32,
    pub ep_cmd: __le32,
    pub ep_sts: __le32,
    pub ep_sts_sid: __le32,
    pub ep_sts_en: __le32,
    pub drbl: __le32,
    pub ep_ien: __le32,
    pub ep_ists: __le32,
    pub usb_pwr: __le32,
    pub usb_conf2: __le32,
    pub usb_cap1: __le32,
    pub usb_cap2: __le32,
    pub usb_cap3: __le32,
    pub usb_cap4: __le32,
    pub usb_cap5: __le32,
    pub usb_cap6: __le32,
    pub usb_cpkt1: __le32,
    pub usb_cpkt2: __le32,
    pub usb_cpkt3: __le32,
    pub ep_dma_ext_addr: __le32,
    pub buf_addr: __le32,
    pub buf_data: __le32,
    pub buf_ctrl: __le32,
    pub dtrans: __le32,
    pub tdl_from_trb: __le32,
    pub tdl_beh: __le32,
    pub ep_tdl: __le32,
    pub tdl_beh2: __le32,
    pub dma_adv_td: __le32,
    pub reserved1: [__le32; 26],
    pub cfg_reg1: __le32,
    pub dbg_link1: __le32,
    pub dbg_link2: __le32,
    pub cfg_regs: [__le32; 74],
    pub reserved2: [__le32; 51],
    pub dma_axi_ctrl: __le32,
    pub dma_axi_id: __le32,
    pub dma_axi_cap: __le32,
    pub dma_axi_ctrl0: __le32,
    pub dma_axi_ctrl1: __le32,
}

// USB_CONF - bitmasks
// Reset USB device configuration.

// Set Configuration.

// Disconnect USB device in SuperSpeed.

// Disconnect USB device in HS/FS

// Little Endian access - default

//
// Big Endian access. Driver assume that byte order for
// SFRs access always is as Little Endian so this bit
// is not used.
//

// Device software reset.

// Singular DMA transfer mode. Only for VER < DEV_VER_V3

// Multiple DMA transfers mode. Only for VER < DEV_VER_V3

// DMA clock turn-off enable.

// DMA clock turn-off disable.

// Clear Force Full Speed.

// Set Force Full Speed.

// Device enable.

// Device disable.

// L1 LPM state entry enable (used in HS/FS mode).

// L1 LPM state entry disable (used in HS/FS mode).

// USB 2.0 clock gate disable.

// USB 2.0 clock gate enable.

// L0 LPM state entry request (used in HS/FS mode).

// USB 3.0 clock gate disable.

// USB 3.0 clock gate enable.

// Bit 23 is reserved
// U1 state entry enable (used in SS mode).

// U1 state entry disable (used in SS mode).

// U2 state entry enable (used in SS mode).

// U2 state entry disable (used in SS mode).

// U0 state entry request (used in SS mode).

// U1 state entry request (used in SS mode).

// U2 state entry request (used in SS mode).

// SS.Inactive state entry request (used in SS mode)

// USB_STS - bitmasks
//
// Configuration status.
// 1 - device is in the configured state.
// 0 - device is not configured.
//

//
// On-chip memory overflow.
// 0 - On-chip memory status OK.
// 1 - On-chip memory overflow.
//

//
// SuperSpeed connection status.
// 0 - USB in SuperSpeed mode disconnected.
// 1 - USB in SuperSpeed mode connected.
//

//
// DMA transfer configuration status.
// 0 - single request.
// 1 - multiple TRB chain
// Supported only for controller version <  DEV_VER_V3
//

//
// Device speed.
// 0 - Undefined (value after reset).
// 1 - Low speed
// 2 - Full speed
// 3 - High speed
// 4 - Super speed
//

//
// Endianness for SFR access.
// 0 - Little Endian order (default after hardware reset).
// 1 - Big Endian order
//

//
// HS/FS clock turn-off status.
// 0 - hsfs clock is always on.
// 1 - hsfs clock turn-off in L2 (HS/FS mode) is enabled
// (default after hardware reset).
//

//
// PCLK clock turn-off status.
// 0 - pclk clock is always on.
// 1 - pclk clock turn-off in U3 (SS mode) is enabled
// (default after hardware reset).
//

//
// Controller in reset state.
// 0 - Internal reset is active.
// 1 - Internal reset is not active and controller is fully operational.
//

//
// Status of the "TDL calculation basing on TRB" feature.
// 0 - disabled
// 1 - enabled
// Supported only for DEV_VER_V2 controller version.
//

//
// Device enable Status.
// 0 - USB device is disabled (VBUS input is disconnected from internal logic).
// 1 - USB device is enabled (VBUS input is connected to the internal logic).
//

//
// Address status.
// 0 - USB device is default state.
// 1 - USB device is at least in address state.
//

//
// L1 LPM state enable status (used in HS/FS mode).
// 0 - Entering to L1 LPM state disabled.
// 1 - Entering to L1 LPM state enabled.
//

//
// Internal VBUS connection status (used both in HS/FS  and SS mode).
// 0 - internal VBUS is not detected.
// 1 - internal VBUS is detected.
//

//
// HS/FS LPM  state (used in FS/HS mode).
// 0 - L0 State
// 1 - L1 State
// 2 - L2 State
// 3 - L3 State
//

//
// Disable HS status (used in FS/HS mode).
// 0 - the disconnect bit for HS/FS mode is set .
// 1 - the disconnect bit for HS/FS mode is not set.
//

//
// HS/FS mode connection status (used in FS/HS mode).
// 0 - High Speed operations in USB2.0 (FS/HS) mode not disabled.
// 1 - High Speed operations in USB2.0 (FS/HS).
//

//
// U1 state enable status (used in SS mode).
// 0 - Entering to  U1 state disabled.
// 1 - Entering to  U1 state enabled.
//

//
// U2 state enable status (used in SS mode).
// 0 - Entering to  U2 state disabled.
// 1 - Entering to  U2 state enabled.
//

//
// SuperSpeed Link LTSSM state. This field reflects USBSS-DEV current
// SuperSpeed link state
//

//
// DMA clock turn-off status.
// 0 - DMA clock is always on (default after hardware reset).
// 1 - DMA clock turn-off in U1, U2 and U3 (SS mode) is enabled.
//

//
// SFR Endian status.
// 0 - Little Endian order (default after hardware reset).
// 1 - Big Endian order.
//

// USB_CMD -  bitmasks
// Set Function Address

//
// Function Address This field is saved to the device only when the field
// SET_ADDR is set '1 ' during write to USB_CMD register.
// Software is responsible for entering the address of the device during
// SET_ADDRESS request service. This field should be set immediately after
// the SETUP packet is decoded, and prior to confirmation of the status phase
//

// Send Function Wake Device Notification TP (used only in SS mode).

// Set Test Mode (used only in HS/FS mode).

// Test mode selector (used only in HS/FS mode)

//
// Send Latency Tolerance Message Device Notification TP (used only
// in SS mode).
//

// Send Custom Transaction Packet (used only in SS mode)

// Device Notification 'Function Wake' - Interface value (only in SS mode.

//
// Device Notification 'Latency Tolerance Message' -373 BELT value [7:0]
// (used only in SS mode).
//

// USB_ITPN - bitmasks
//
// ITP(SS) / SOF (HS/FS) number
// In SS mode this field represent number of last ITP received from host.
// In HS/FS mode this field represent number of last SOF received from host.
//

// USB_LPM - bitmasks
// Host Initiated Resume Duration.

// Remote Wakeup Enable (bRemoteWake).

// USB_IEN - bitmasks
// SS connection interrupt enable

// SS disconnection interrupt enable.

// USB SS warm reset interrupt enable.

// USB SS hot reset interrupt enable

// SS link U3 state enter interrupt enable (suspend).

// SS link U3 state exit interrupt enable (wakeup).

// SS link U2 state enter interrupt enable.

// SS link U2 state exit interrupt enable.

// SS link U1 state enter interrupt enable.

// SS link U1 state exit interrupt enable.

// ITP/SOF packet detected interrupt enable.

// Wakeup interrupt enable.

// Send Custom Packet interrupt enable.

// HS/FS mode connection interrupt enable.

// HS/FS mode disconnection interrupt enable.

// USB reset (HS/FS mode) interrupt enable.

// LPM L2 state enter interrupt enable.

// LPM  L2 state exit interrupt enable.

// LPM L1 state enter interrupt enable.

// LPM  L1 state exit interrupt enable.

// Configuration reset interrupt enable.

// Start of the USB SS warm reset interrupt enable.

// End of the USB SS warm reset interrupt enable.

// USB_ISTS - bitmasks
// SS Connection detected.

// SS Disconnection detected.

// UUSB warm reset detectede.

// USB hot reset detected.

// U3 link state enter detected (suspend).

// U3 link state exit detected (wakeup).

// U2 link state enter detected.

// U2 link state exit detected.

// U1 link state enter detected.

// U1 link state exit detected.

// ITP/SOF packet detected.

// Wakeup detected.

// Send Custom Packet detected.

// HS/FS mode connection detected.

// HS/FS mode disconnection detected.

// USB reset (HS/FS mode) detected.

// LPM L2 state enter detected.

// LPM  L2 state exit detected.

// LPM L1 state enter detected.

// LPM L1 state exit detected.

// USB configuration reset detected.

// Start of the USB warm reset detected.

// End of the USB warm reset detected.

// USB_SEL - bitmasks

// Endpoint number.

// Endpoint direction bit - 0 - OUT, 1 - IN.

// EP_TRADDR - bitmasks
// Transfer Ring address.

// EP_CFG - bitmasks
// Endpoint enable

//
// Endpoint type.
// 1 - isochronous
// 2 - bulk
// 3 - interrupt
//

// Stream support enable (only in SS mode).

// TDL check (only in SS mode for BULK EP).

// SID check (only in SS mode for BULK OUT EP).

// DMA transfer endianness.

// Max burst size (used only in SS mode).

pub const EP_CFG_MAXBURST_MAX: c_int = 15;
// ISO max burst.

pub const EP_CFG_MULT_MAX: c_int = 2;
// ISO max burst.

// Max number of buffered packets.

pub const EP_CFG_BUFFERING_MAX: c_int = 15;
// EP_CMD - bitmasks
// Endpoint reset.

// Endpoint STALL set.

// Endpoint STALL clear.

// Send ERDY TP.

// Request complete.

// Transfer descriptor ready.

// Data flush.

//
// Transfer Descriptor Length write  (used only for Bulk Stream capable
// endpoints in SS mode).
// Bit Removed from DEV_VER_V3 controller version.
//

//
// Transfer Descriptor Length (used only in SS mode for bulk endpoints).
// Bits Removed from DEV_VER_V3 controller version.
//

// ERDY Stream ID value (used in SS mode).

// EP_STS - bitmasks
// Setup transfer complete.

// Endpoint STALL status.

// Interrupt On Complete.

// Interrupt on Short Packet.

// Transfer descriptor missing.

// Stream Rejected (used only in SS mode)

// EXIT from MOVE DATA State (used only for stream transfers in SS mode).

// TRB error.

// Not ready (used only in SS mode).

// DMA busy bit.

// Endpoint Buffer Empty

// Current Cycle Status

// Prime (used only in SS mode.

// Stream error (used only in SS mode).

// OUT size mismatch.

// ISO transmission error.

// Host Packet Pending (only for SS mode).

// Stream Protocol State Machine State (only for Bulk stream endpoints).

// Interrupt On Transfer complete.

// OUT queue endpoint number.

// OUT queue valid flag.

// SETUP WAIT.

// EP_STS_SID - bitmasks
// Stream ID (used only in SS mode).

// EP_STS_EN - bitmasks
// SETUP interrupt enable.

// OUT transfer missing descriptor enable.

// Stream Rejected enable.

// Move Data Exit enable.

// TRB enable.

// NRDY enable.

// Prime enable.

// Stream error enable.

// OUT size mismatch enable.

// ISO transmission error enable.

// Interrupt on Transmission complete enable.

// Setup Wait interrupt enable.

// DRBL- bitmasks

// EP_IEN - bitmasks

// EP_ISTS - bitmasks

// USB_PWR- bitmasks
// Power Shut Off capability enable

// Power Shut Off capability disable

//
// Enables turning-off Reference Clock.
// This bit is optional and implemented only when support for OTG is
// implemented (indicated by OTG_READY bit set to '1').
//

//
// Status bit indicating that operation required by STB_CLK_SWITCH_EN write
// is completed
//

// This bit informs if Fast Registers Access is enabled.

// Fast Registers Access Enable.

// USB_CONF2- bitmasks
//
// Writing 1 disables TDL calculation basing on TRB feature in controller
// for DMULT mode.
// Bit supported only for DEV_VER_V2 version.
//

//
// Writing 1 enables TDL calculation basing on TRB feature in controller
// for DMULT mode.
// Bit supported only for DEV_VER_V2 version.
//

// USB_CAP1- bitmasks
//
// SFR Interface type
// These field reflects type of SFR interface implemented:
// 0x0 - OCP
// 0x1 - AHB,
// 0x2 - PLB
// 0x3 - AXI
// 0x4-0xF - reserved
//

//
// SFR Interface width
// These field reflects width of SFR interface implemented:
// 0x0 - 8 bit interface,
// 0x1 - 16 bit interface,
// 0x2 - 32 bit interface
// 0x3 - 64 bit interface
// 0x4-0xF - reserved
//

//
// DMA Interface type
// These field reflects type of DMA interface implemented:
// 0x0 - OCP
// 0x1 - AHB,
// 0x2 - PLB
// 0x3 - AXI
// 0x4-0xF - reserved
//

//
// DMA Interface width
// These field reflects width of DMA interface implemented:
// 0x0 - reserved,
// 0x1 - reserved,
// 0x2 - 32 bit interface
// 0x3 - 64 bit interface
// 0x4-0xF - reserved
//

//
// USB3 PHY Interface type
// These field reflects type of USB3 PHY interface implemented:
// 0x0 - USB PIPE,
// 0x1 - RMMI,
// 0x2-0xF - reserved
//

//
// USB3 PHY Interface width
// These field reflects width of USB3 PHY interface implemented:
// 0x0 - 8 bit PIPE interface,
// 0x1 - 16 bit PIPE interface,
// 0x2 - 32 bit PIPE interface,
// 0x3 - 64 bit PIPE interface
// 0x4-0xF - reserved
// Note: When SSIC interface is implemented this field shows the width of
// internal PIPE interface. The RMMI interface is always 20bit wide.
//

//
// USB2 PHY Interface enable
// These field informs if USB2 PHY interface is implemented:
// 0x0 - interface NOT implemented,
// 0x1 - interface implemented
//

//
// USB2 PHY Interface type
// These field reflects type of USB2 PHY interface implemented:
// 0x0 - UTMI,
// 0x1 - ULPI
//

//
// USB2 PHY Interface width
// These field reflects width of USB2 PHY interface implemented:
// 0x0 - 8 bit interface,
// 0x1 - 16 bit interface,
// Note: The ULPI interface is always 8bit wide.
//

//
// OTG Ready
// 0x0 - pure device mode
// 0x1 - some features and ports for CDNS USB OTG controller are implemented.
//

//
// When set, indicates that controller supports automatic internal TDL
// calculation basing on the size provided in TRB (TRB[22:17]) for DMULT mode
// Supported only for DEV_VER_V2 controller version.
//

// USB_CAP2- bitmasks
//
// The actual size of the connected On-chip RAM memory in kB:
// - 0 means 256 kB (max supported mem size)
// - value other than 0 reflects the mem size in kB
//

//
// Max supported mem size
// These field reflects width of on-chip RAM address bus width,
// which determines max supported mem size:
// 0x0-0x7 - reserved,
// 0x8 - support for 4kB mem,
// 0x9 - support for 8kB mem,
// 0xA - support for 16kB mem,
// 0xB - support for 32kB mem,
// 0xC - support for 64kB mem,
// 0xD - support for 128kB mem,
// 0xE - support for 256kB mem,
// 0xF - reserved
//

// USB_CAP3- bitmasks

// USB_CAP4- bitmasks

// USB_CAP5- bitmasks

// USB_CAP6- bitmasks
// The USBSS-DEV Controller  Internal build number.

// The USBSS-DEV Controller version number.

pub const DEV_VER_NXP_V1: c_uint = 0x00024502;
pub const DEV_VER_TI_V1: c_uint = 0x00024509;
pub const DEV_VER_V2: c_uint = 0x0002450C;
pub const DEV_VER_V3: c_uint = 0x0002450d;
// DBG_LINK1- bitmasks
//
// LFPS_MIN_DET_U1_EXIT value This parameter configures the minimum
// time required for decoding the received LFPS as an LFPS.U1_Exit.
//

//
// LFPS_MIN_GEN_U1_EXIT value This parameter configures the minimum time for
// phytxelecidle deassertion when LFPS.U1_Exit
//

//
// RXDET_BREAK_DIS value This parameter configures terminating the Far-end
// Receiver termination detection sequence:
// 0: it is possible that USBSS_DEV will terminate Farend receiver
// termination detection sequence
// 1: USBSS_DEV will not terminate Far-end receiver termination
// detection sequence
//

// LFPS_GEN_PING value This parameter configures the LFPS.Ping generation

//
// Set the LFPS_MIN_DET_U1_EXIT value Writing '1' to this bit writes the
// LFPS_MIN_DET_U1_EXIT field value to the device. This bit is automatically
// cleared. Writing '0' has no effect
//

//
// Set the LFPS_MIN_GEN_U1_EXIT value. Writing '1' to this bit writes the
// LFPS_MIN_GEN_U1_EXIT field value to the device. This bit is automatically
// cleared. Writing '0' has no effect
//

//
// Set the RXDET_BREAK_DIS value Writing '1' to this bit writes
// the RXDET_BREAK_DIS field value to the device. This bit is automatically
// cleared. Writing '0' has no effect
//

//
// Set the LFPS_GEN_PING_SET value Writing '1' to this bit writes
// the LFPS_GEN_PING field value to the device. This bit is automatically
// cleared. Writing '0' has no effect."
//

// DMA_AXI_CTRL- bitmasks
// The mawprot pin configuration.

// The marprot pin configuration.

pub const DMA_AXI_CTRL_NON_SECURE: c_uint = 0x02;

// -------------------------------------------------------------------------
//
// USBSS-DEV DMA interface.
//
pub const TRBS_PER_SEGMENT: c_int = 600;
pub const ISO_MAX_INTERVAL: c_int = 10;

pub const TRBS_PER_STREAM_SEGMENT: c_int = 2;

//
// Only for ISOC endpoints - maximum number of TRBs is calculated as
// pow(2, bInterval-1) * number of usb requests. It is limitation made by
// driver to save memory. Controller must prepare TRB for each ITP even
// if bInterval > 1. It's the reason why driver needs so many TRBs for
// isochronous endpoints.
//

//
// struct cdns3_trb - represent Transfer Descriptor block.
// @buffer:	pointer to buffer data
// @length:	length of data
// @control:	control flags.
//
// This structure describes transfer block serviced by DMA module.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_trb {
    pub buffer: __le32,
    pub length: __le32,
    pub control: __le32,
}

// TRB bit mask

// TRB type IDs
// bulk, interrupt, isoc , and control data stage
pub const TRB_NORMAL: c_int = 1;
// TRB for linking ring segments
pub const TRB_LINK: c_int = 6;
// Cycle bit - indicates TRB ownership by driver or hw

//
// When set to '1', the device will toggle its interpretation of the Cycle bit
//

//
// The controller will set it if OUTSMM (OUT size mismatch) is detected,
// this bit is for normal TRB
//

//
// Short Packet (SP). OUT EPs at DMULT=1 only. Indicates if the TRB was
// processed while USB short packet was received. No more buffers defined by
// the TD will be used. DMA will automatically advance to next TD.
// - Shall be set to 0 by Software when putting TRB on the Transfer Ring
// - Shall be set to 1 by Controller when Short Packet condition for this TRB
// is detected independent if ISP is set or not.
//

// Interrupt on short packet

// Setting this bit enables FIFO DMA operation mode

// Set PCIe no snoop attribute

// Interrupt on completion

// stream ID bitmasks.

// Size of TD expressed in USB packets for HS/FS mode.

// transfer_len bitmasks.

// Size of TD expressed in USB packets for SS mode.

// transfer_len bitmasks - bits 31:24

// Data buffer pointer bitmasks

// -------------------------------------------------------------------------
// Driver numeric constants
// Such declaration should be added to ch9.h
pub const USB_DEVICE_MAX_ADDRESS: c_int = 127;
// Endpoint init values
pub const CDNS3_EP_MAX_PACKET_LIMIT: c_int = 1024;
pub const CDNS3_EP_MAX_STREAMS: c_int = 15;
pub const CDNS3_EP0_MAX_PACKET_LIMIT: c_int = 512;
// All endpoints including EP0
pub const CDNS3_ENDPOINTS_MAX_COUNT: c_int = 32;
pub const CDNS3_EP_ZLP_BUF_SIZE: c_int = 1024;
pub const CDNS3_MAX_NUM_DESCMISS_BUF: c_int = 32;

pub const CDNS3_WA2_NUM_BUFFERS: c_int = 128;
// -------------------------------------------------------------------------
// Used structs
//
// struct cdns3_endpoint - extended device side representation of USB endpoint.
// @endpoint: usb endpoint
// @pending_req_list: list of requests queuing on transfer ring.
// @deferred_req_list: list of requests waiting for queuing on transfer ring.
// @wa2_descmiss_req_list: list of requests internally allocated by driver.
// @trb_pool: transfer ring - array of transaction buffers
// @trb_pool_dma: dma address of transfer ring
// @cdns3_dev: device associated with this endpoint
// @name: a human readable name e.g. ep1out
// @flags: specify the current state of endpoint
// @descmis_req: internal transfer object used for getting data from on-chip
// buffer. It can happen only if function driver doesn't send usb_request
// object on time.
// @dir: endpoint direction
// @num: endpoint number (1 - 15)
// @type: set to bmAttributes & USB_ENDPOINT_XFERTYPE_MASK
// @interval: interval between packets used for ISOC endpoint.
// @free_trbs: number of free TRBs in transfer ring
// @num_trbs: number of all TRBs in transfer ring
// @alloc_ring_size: size of the allocated TRB ring
// @pcs: producer cycle state
// @ccs: consumer cycle state
// @enqueue: enqueue index in transfer ring
// @dequeue: dequeue index in transfer ring
// @trb_burst_size: number of burst used in trb.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_endpoint {
    pub endpoint: usb_ep,
    pub pending_req_list: list_head,
    pub deferred_req_list: list_head,
    pub wa2_descmiss_req_list: list_head,
    pub wa2_counter: c_int,
    pub trb_pool: *mut cdns3_trb,
    pub trb_pool_dma: dma_addr_t,
    pub cdns3_dev: *mut cdns3_device,
    pub name: [c_char; 20],
    pub flags: u32,
    pub descmis_req: *mut cdns3_request,
    pub dir: u8,
    pub num: u8,
    pub type: u8,
    pub mult: u8,
    pub bMaxBurst: u8,
    pub wMaxPacketSize: u16,
    pub interval: c_int,
    pub free_trbs: c_int,
    pub num_trbs: c_int,
    pub alloc_ring_size: c_int,
    pub pcs: u8,
    pub ccs: u8,
    pub enqueue: c_int,
    pub dequeue: c_int,
    pub trb_burst_size: u8,
    pub wa1_set:1: c_uint,
    pub wa1_trb: *mut cdns3_trb,
    pub wa1_trb_index: c_uint,
    pub wa1_cycle_bit:1: c_uint,
// Stream related
    pub use_streams:1: c_uint,
    pub prime_flag:1: c_uint,
    pub ep_sts_pending: u32,
    pub last_stream_id: u16,
    pub pending_tdl: u16,
    pub stream_sg_idx: c_uint,
}

//
// struct cdns3_aligned_buf - represent aligned buffer used for DMA transfer
// @buf: aligned to 8 bytes data buffer. Buffer address used in
// TRB shall be aligned to 8.
// @dma: dma address
// @size: size of buffer
// @in_use: inform if this buffer is associated with usb_request
// @list: used to adding instance of this object to list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_aligned_buf {
    pub buf: *mut c_void,
    pub dma: dma_addr_t,
    pub size: u32,
    pub dir: dma_data_direction,
    pub in_use:1: unsigned,
    pub list: list_head,
}

//
// struct cdns3_request - extended device side representation of usb_request
// object .
// @request: generic usb_request object describing single I/O request.
// @priv_ep: extended representation of usb_ep object
// @trb: the first TRB association with this request
// @start_trb: number of the first TRB in transfer ring
// @end_trb: number of the last TRB in transfer ring
// @aligned_buf: object holds information about aligned buffer associated whit
// this endpoint
// @flags: flag specifying special usage of request
// @list: used by internally allocated request to add to wa2_descmiss_req_list.
// @finished_trb: number of trb has already finished per request
// @num_of_trb: how many trbs in this request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_request {
    pub request: usb_request,
    pub priv_ep: *mut cdns3_endpoint,
    pub trb: *mut cdns3_trb,
    pub start_trb: c_int,
    pub end_trb: c_int,
    pub aligned_buf: *mut cdns3_aligned_buf,

    pub flags: u32,
    pub list: list_head,
    pub finished_trb: c_int,
    pub num_of_trb: c_int,
}

// Stages used during enumeration process.
pub const CDNS3_SETUP_STAGE: c_uint = 0x0;
pub const CDNS3_DATA_STAGE: c_uint = 0x1;
pub const CDNS3_STATUS_STAGE: c_uint = 0x2;
//
// struct cdns3_device - represent USB device.
// @dev: pointer to device structure associated whit this controller
// @sysdev: pointer to the DMA capable device
// @gadget: device side representation of the peripheral controller
// @gadget_driver: pointer to the gadget driver
// @dev_ver: device controller version.
// @lock: for synchronizing
// @regs: base address for device side registers
// @setup_buf: used while processing usb control requests
// @setup_dma: dma address for setup_buf
// @zlp_buf - zlp buffer
// @ep0_stage: ep0 stage during enumeration process.
// @ep0_data_dir: direction for control transfer
// @eps: array of pointers to all endpoints with exclusion ep0
// @aligned_buf_list: list of aligned buffers internally allocated by driver
// @aligned_buf_wq: workqueue freeing  no longer used aligned buf.
// @selected_ep: actually selected endpoint. It's used only to improve
// performance.
// @isoch_delay: value from Set Isoch Delay request. Only valid on SS/SSP.
// @u1_allowed: allow device transition to u1 state
// @u2_allowed: allow device transition to u2 state
// @is_selfpowered: device is self powered
// @setup_pending: setup packet is processing by gadget driver
// @hw_configured_flag: hardware endpoint configuration was set.
// @wake_up_flag: allow device to remote up the host
// @status_completion_no_call: indicate that driver is waiting for status s
// stage completion. It's used in deferred SET_CONFIGURATION request.
// @onchip_buffers: number of available on-chip buffers.
// @onchip_used_size: actual size of on-chip memory assigned to endpoints.
// @pending_status_wq: workqueue handling status stage for deferred requests.
// @pending_status_request: request for which status stage was deferred
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_device {
    pub dev: *mut device,
    pub sysdev: *mut device,
    pub gadget: usb_gadget,
    pub gadget_driver: *mut usb_gadget_driver,
pub const CDNS_REVISION_V0: c_uint = 0x00024501;
pub const CDNS_REVISION_V1: c_uint = 0x00024509;
    pub dev_ver: u32,
// generic spin-lock for drivers
    pub lock: spinlock_t,
    pub regs: *mut cdns3_usb_regs __iomem,
    pub eps_dma_pool: *mut dma_pool,
    pub setup_buf: *mut usb_ctrlrequest,
    pub setup_dma: dma_addr_t,
    pub zlp_buf: *mut c_void,
    pub ep0_stage: u8,
    pub ep0_data_dir: c_int,
    pub eps: [*mut cdns3_endpoint; CDNS3_ENDPOINTS_MAX_COUNT],
    pub aligned_buf_list: list_head,
    pub aligned_buf_wq: work_struct,
    pub selected_ep: u32,
    pub isoch_delay: u16,
    pub wait_for_setup:1: unsigned,
    pub u1_allowed:1: unsigned,
    pub u2_allowed:1: unsigned,
    pub is_selfpowered:1: unsigned,
    pub setup_pending:1: unsigned,
    pub hw_configured_flag:1: unsigned,
    pub wake_up_flag:1: unsigned,
    pub status_completion_no_call:1: unsigned,
    pub using_streams:1: unsigned,
    pub out_mem_is_allocated: c_int,
    pub pending_status_wq: work_struct,
    pub pending_status_request: *mut usb_request,
// in KB
    pub onchip_buffers: u16,
    pub onchip_used_size: u16,
    pub ep_buf_size: u16,
    pub ep_iso_burst: u16,
}

extern "C" {
    pub fn cdns3_set_register_bit(ptr: *mut void __iomem, mask: u32);
}
extern "C" {
    pub fn cdns3_get_speed(priv_dev: *mut cdns3_device) -> usb_device_speed;
}
extern "C" {
    pub fn cdns3_pending_setup_status_handler(work: *mut work_struct);
}
extern "C" {
    pub fn cdns3_hw_reset_eps_config(priv_dev: *mut cdns3_device);
}
extern "C" {
    pub fn cdns3_set_hw_configuration(priv_dev: *mut cdns3_device);
}
extern "C" {
    pub fn cdns3_select_ep(priv_dev: *mut cdns3_device, ep: u32);
}
extern "C" {
    pub fn cdns3_allow_enable_l1(priv_dev: *mut cdns3_device, enable: c_int);
}
extern "C" {
    pub fn cdns3_rearm_transfer(priv_ep: *mut cdns3_endpoint, rearm: u8);
}
extern "C" {
    pub fn cdns3_allocate_trb_pool(priv_ep: *mut cdns3_endpoint) -> c_int;
}
extern "C" {
    pub fn cdns3_ep_addr_to_index(ep_addr: u8) -> u8;
}
extern "C" {
    pub fn cdns3_gadget_ep_set_wedge(ep: *mut usb_ep) -> c_int;
}
extern "C" {
    pub fn cdns3_gadget_ep_set_halt(ep: *mut usb_ep, value: c_int) -> c_int;
}
extern "C" {
    pub fn __cdns3_gadget_ep_set_halt(priv_ep: *mut cdns3_endpoint);
}
extern "C" {
    pub fn __cdns3_gadget_ep_clear_halt(priv_ep: *mut cdns3_endpoint) -> c_int;
}
extern "C" {
    pub fn cdns3_gadget_ep_dequeue(ep: *mut usb_ep, request: *mut usb_request) -> c_int;
}
extern "C" {
    pub fn cdns3_ep0_config(priv_dev: *mut cdns3_device);
}
extern "C" {
    pub fn cdns3_ep_config(priv_ep: *mut cdns3_endpoint, enable: bool) -> c_int;
}
extern "C" {
    pub fn cdns3_check_ep0_interrupt_proceed(priv_dev: *mut cdns3_device, dir: c_int);
}
extern "C" {
    pub fn __cdns3_gadget_wakeup(priv_dev: *mut cdns3_device) -> c_int;
}
