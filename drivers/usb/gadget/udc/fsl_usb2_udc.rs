//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/fsl_usb2_udc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2004,2012 Freescale Semiconductor, Inc
// All rights reserved.
//
// Freescale USB device/endpoint management registers
//

// ### define USB registers here
//
pub const USB_MAX_CTRL_PAYLOAD: c_int = 64;
pub const USB_DR_SYS_OFFSET: c_uint = 0x400;
// USB DR device mode registers (Little Endian)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dr_device {
// Capability register
    pub res1: [u8; 256],
    pub /: *mut *mut u16 caplength; / Capability Register Length,
    pub /: *mut *mut u16 hciversion; / Host Controller Interface Version,
    pub /: *mut *mut u32 hcsparams; / Host Controller Structural Parameters,
    pub /: *mut *mut u32 hccparams; / Host Controller Capability Parameters,
    pub res2: [u8; 20],
    pub /: *mut *mut u32 dciversion; / Device Controller Interface Version,
    pub /: *mut *mut u32 dccparams; / Device Controller Capability Parameters,
    pub res3: [u8; 24],
// Operation register
    pub /: *mut *mut u32 usbcmd; / USB Command Register,
    pub /: *mut *mut u32 usbsts; / USB Status Register,
    pub /: *mut *mut u32 usbintr; / USB Interrupt Enable Register,
    pub /: *mut *mut u32 frindex; / Frame Index Register,
    pub res4: [u8; 4],
    pub /: *mut *mut u32 deviceaddr; / Device Address,
    pub /: *mut *mut u32 endpointlistaddr; / Endpoint List Address Register,
    pub res5: [u8; 4],
    pub /: *mut *mut u32 burstsize; / Master Interface Data Burst Size Register,
    pub /: *mut *mut u32 txttfilltuning; / Transmit FIFO Tuning Controls Register,
    pub res6: [u8; 24],
    pub /: *mut *mut u32 configflag; / Configure Flag Register,
    pub /: *mut *mut u32 portsc1; / Port 1 Status and Control Register,
    pub res7: [u8; 28],
    pub /: *mut *mut u32 otgsc; / On-The-Go Status and Control,
    pub /: *mut *mut u32 usbmode; / USB Mode Register,
    pub /: *mut *mut u32 endptsetupstat; / Endpoint Setup Status Register,
    pub /: *mut *mut u32 endpointprime; / Endpoint Initialization Register,
    pub /: *mut *mut u32 endptflush; / Endpoint Flush Register,
    pub /: *mut *mut u32 endptstatus; / Endpoint Status Register,
    pub /: *mut *mut u32 endptcomplete; / Endpoint Complete Register,
    pub /: *mut *mut u32 endptctrl[6]; / Endpoint Control Registers,
}

// USB DR host mode registers (Little Endian)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_dr_host {
// Capability register
    pub res1: [u8; 256],
    pub /: *mut *mut u16 caplength; / Capability Register Length,
    pub /: *mut *mut u16 hciversion; / Host Controller Interface Version,
    pub /: *mut *mut u32 hcsparams; / Host Controller Structural Parameters,
    pub /: *mut *mut u32 hccparams; / Host Controller Capability Parameters,
    pub res2: [u8; 20],
    pub /: *mut *mut u32 dciversion; / Device Controller Interface Version,
    pub /: *mut *mut u32 dccparams; / Device Controller Capability Parameters,
    pub res3: [u8; 24],
// Operation register
    pub /: *mut *mut u32 usbcmd; / USB Command Register,
    pub /: *mut *mut u32 usbsts; / USB Status Register,
    pub /: *mut *mut u32 usbintr; / USB Interrupt Enable Register,
    pub /: *mut *mut u32 frindex; / Frame Index Register,
    pub res4: [u8; 4],
    pub /: *mut *mut u32 periodiclistbase; / Periodic Frame List Base Address Register,
    pub /: *mut *mut u32 asynclistaddr; / Current Asynchronous List Address Register,
    pub res5: [u8; 4],
    pub /: *mut *mut u32 burstsize; / Master Interface Data Burst Size Register,
    pub /: *mut *mut u32 txttfilltuning; / Transmit FIFO Tuning Controls Register,
    pub res6: [u8; 24],
    pub /: *mut *mut u32 configflag; / Configure Flag Register,
    pub /: *mut *mut u32 portsc1; / Port 1 Status and Control Register,
    pub res7: [u8; 28],
    pub /: *mut *mut u32 otgsc; / On-The-Go Status and Control,
    pub /: *mut *mut u32 usbmode; / USB Mode Register,
    pub /: *mut *mut u32 endptsetupstat; / Endpoint Setup Status Register,
    pub /: *mut *mut u32 endpointprime; / Endpoint Initialization Register,
    pub /: *mut *mut u32 endptflush; / Endpoint Flush Register,
    pub /: *mut *mut u32 endptstatus; / Endpoint Status Register,
    pub /: *mut *mut u32 endptcomplete; / Endpoint Complete Register,
    pub /: *mut *mut u32 endptctrl[6]; / Endpoint Control Registers,
}

// non-EHCI USB system interface registers (Big Endian)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_sys_interface {
    pub snoop1: u32,
    pub snoop2: u32,
    pub /: *mut *mut u32 age_cnt_thresh; / Age Count Threshold Register,
    pub /: *mut *mut u32 pri_ctrl; / Priority Control Register,
    pub /: *mut *mut u32 si_ctrl; / System Interface Control Register,
    pub res: [u8; 236],
    pub /: *mut *mut u32 control; / General Purpose Control Register,
}

// ep0 transfer state
pub const WAIT_FOR_SETUP: c_int = 0;
pub const DATA_STATE_XMIT: c_int = 1;
pub const DATA_STATE_NEED_ZLP: c_int = 2;
pub const WAIT_FOR_OUT_STATUS: c_int = 3;
pub const DATA_STATE_RECV: c_int = 4;
// Device Controller Capability Parameter register
pub const DCCPARAMS_DC: c_uint = 0x00000080;
pub const DCCPARAMS_DEN_MASK: c_uint = 0x0000001f;
// Frame Index Register Bit Masks
pub const USB_FRINDEX_MASKS: c_uint = 0x3fff;
// USB CMD  Register Bit Masks
pub const USB_CMD_RUN_STOP: c_uint = 0x00000001;
pub const USB_CMD_CTRL_RESET: c_uint = 0x00000002;
pub const USB_CMD_PERIODIC_SCHEDULE_EN: c_uint = 0x00000010;
pub const USB_CMD_ASYNC_SCHEDULE_EN: c_uint = 0x00000020;
pub const USB_CMD_INT_AA_DOORBELL: c_uint = 0x00000040;
pub const USB_CMD_ASP: c_uint = 0x00000300;
pub const USB_CMD_ASYNC_SCH_PARK_EN: c_uint = 0x00000800;
pub const USB_CMD_SUTW: c_uint = 0x00002000;
pub const USB_CMD_ATDTW: c_uint = 0x00004000;
pub const USB_CMD_ITC: c_uint = 0x00FF0000;
// bit 15,3,2 are frame list size
pub const USB_CMD_FRAME_SIZE_1024: c_uint = 0x00000000;
pub const USB_CMD_FRAME_SIZE_512: c_uint = 0x00000004;
pub const USB_CMD_FRAME_SIZE_256: c_uint = 0x00000008;
pub const USB_CMD_FRAME_SIZE_128: c_uint = 0x0000000C;
pub const USB_CMD_FRAME_SIZE_64: c_uint = 0x00008000;
pub const USB_CMD_FRAME_SIZE_32: c_uint = 0x00008004;
pub const USB_CMD_FRAME_SIZE_16: c_uint = 0x00008008;
pub const USB_CMD_FRAME_SIZE_8: c_uint = 0x0000800C;
// bit 9-8 are async schedule park mode count
pub const USB_CMD_ASP_00: c_uint = 0x00000000;
pub const USB_CMD_ASP_01: c_uint = 0x00000100;
pub const USB_CMD_ASP_10: c_uint = 0x00000200;
pub const USB_CMD_ASP_11: c_uint = 0x00000300;
pub const USB_CMD_ASP_BIT_POS: c_int = 8;
// bit 23-16 are interrupt threshold control
pub const USB_CMD_ITC_NO_THRESHOLD: c_uint = 0x00000000;
pub const USB_CMD_ITC_1_MICRO_FRM: c_uint = 0x00010000;
pub const USB_CMD_ITC_2_MICRO_FRM: c_uint = 0x00020000;
pub const USB_CMD_ITC_4_MICRO_FRM: c_uint = 0x00040000;
pub const USB_CMD_ITC_8_MICRO_FRM: c_uint = 0x00080000;
pub const USB_CMD_ITC_16_MICRO_FRM: c_uint = 0x00100000;
pub const USB_CMD_ITC_32_MICRO_FRM: c_uint = 0x00200000;
pub const USB_CMD_ITC_64_MICRO_FRM: c_uint = 0x00400000;
pub const USB_CMD_ITC_BIT_POS: c_int = 16;
// USB STS Register Bit Masks
pub const USB_STS_INT: c_uint = 0x00000001;
pub const USB_STS_ERR: c_uint = 0x00000002;
pub const USB_STS_PORT_CHANGE: c_uint = 0x00000004;
pub const USB_STS_FRM_LST_ROLL: c_uint = 0x00000008;
pub const USB_STS_SYS_ERR: c_uint = 0x00000010;
pub const USB_STS_IAA: c_uint = 0x00000020;
pub const USB_STS_RESET: c_uint = 0x00000040;
pub const USB_STS_SOF: c_uint = 0x00000080;
pub const USB_STS_SUSPEND: c_uint = 0x00000100;
pub const USB_STS_HC_HALTED: c_uint = 0x00001000;
pub const USB_STS_RCL: c_uint = 0x00002000;
pub const USB_STS_PERIODIC_SCHEDULE: c_uint = 0x00004000;
pub const USB_STS_ASYNC_SCHEDULE: c_uint = 0x00008000;
// USB INTR Register Bit Masks
pub const USB_INTR_INT_EN: c_uint = 0x00000001;
pub const USB_INTR_ERR_INT_EN: c_uint = 0x00000002;
pub const USB_INTR_PTC_DETECT_EN: c_uint = 0x00000004;
pub const USB_INTR_FRM_LST_ROLL_EN: c_uint = 0x00000008;
pub const USB_INTR_SYS_ERR_EN: c_uint = 0x00000010;
pub const USB_INTR_ASYN_ADV_EN: c_uint = 0x00000020;
pub const USB_INTR_RESET_EN: c_uint = 0x00000040;
pub const USB_INTR_SOF_EN: c_uint = 0x00000080;
pub const USB_INTR_DEVICE_SUSPEND: c_uint = 0x00000100;
// Device Address bit masks
pub const USB_DEVICE_ADDRESS_MASK: c_uint = 0xFE000000;
pub const USB_DEVICE_ADDRESS_BIT_POS: c_int = 25;
// endpoint list address bit masks
pub const USB_EP_LIST_ADDRESS_MASK: c_uint = 0xfffff800;
// PORTSCX  Register Bit Masks
pub const PORTSCX_CURRENT_CONNECT_STATUS: c_uint = 0x00000001;
pub const PORTSCX_CONNECT_STATUS_CHANGE: c_uint = 0x00000002;
pub const PORTSCX_PORT_ENABLE: c_uint = 0x00000004;
pub const PORTSCX_PORT_EN_DIS_CHANGE: c_uint = 0x00000008;
pub const PORTSCX_OVER_CURRENT_ACT: c_uint = 0x00000010;
pub const PORTSCX_OVER_CURRENT_CHG: c_uint = 0x00000020;
pub const PORTSCX_PORT_FORCE_RESUME: c_uint = 0x00000040;
pub const PORTSCX_PORT_SUSPEND: c_uint = 0x00000080;
pub const PORTSCX_PORT_RESET: c_uint = 0x00000100;
pub const PORTSCX_LINE_STATUS_BITS: c_uint = 0x00000C00;
pub const PORTSCX_PORT_POWER: c_uint = 0x00001000;
pub const PORTSCX_PORT_INDICTOR_CTRL: c_uint = 0x0000C000;
pub const PORTSCX_PORT_TEST_CTRL: c_uint = 0x000F0000;
pub const PORTSCX_WAKE_ON_CONNECT_EN: c_uint = 0x00100000;
pub const PORTSCX_WAKE_ON_CONNECT_DIS: c_uint = 0x00200000;
pub const PORTSCX_WAKE_ON_OVER_CURRENT: c_uint = 0x00400000;
pub const PORTSCX_PHY_LOW_POWER_SPD: c_uint = 0x00800000;
pub const PORTSCX_PORT_FORCE_FULL_SPEED: c_uint = 0x01000000;
pub const PORTSCX_PORT_SPEED_MASK: c_uint = 0x0C000000;
pub const PORTSCX_PORT_WIDTH: c_uint = 0x10000000;
pub const PORTSCX_PHY_TYPE_SEL: c_uint = 0xC0000000;
// bit 11-10 are line status
pub const PORTSCX_LINE_STATUS_SE0: c_uint = 0x00000000;
pub const PORTSCX_LINE_STATUS_JSTATE: c_uint = 0x00000400;
pub const PORTSCX_LINE_STATUS_KSTATE: c_uint = 0x00000800;
pub const PORTSCX_LINE_STATUS_UNDEF: c_uint = 0x00000C00;
pub const PORTSCX_LINE_STATUS_BIT_POS: c_int = 10;
// bit 15-14 are port indicator control
pub const PORTSCX_PIC_OFF: c_uint = 0x00000000;
pub const PORTSCX_PIC_AMBER: c_uint = 0x00004000;
pub const PORTSCX_PIC_GREEN: c_uint = 0x00008000;
pub const PORTSCX_PIC_UNDEF: c_uint = 0x0000C000;
pub const PORTSCX_PIC_BIT_POS: c_int = 14;
// bit 19-16 are port test control
pub const PORTSCX_PTC_DISABLE: c_uint = 0x00000000;
pub const PORTSCX_PTC_JSTATE: c_uint = 0x00010000;
pub const PORTSCX_PTC_KSTATE: c_uint = 0x00020000;
pub const PORTSCX_PTC_SEQNAK: c_uint = 0x00030000;
pub const PORTSCX_PTC_PACKET: c_uint = 0x00040000;
pub const PORTSCX_PTC_FORCE_EN: c_uint = 0x00050000;
pub const PORTSCX_PTC_BIT_POS: c_int = 16;
// bit 27-26 are port speed
pub const PORTSCX_PORT_SPEED_FULL: c_uint = 0x00000000;
pub const PORTSCX_PORT_SPEED_LOW: c_uint = 0x04000000;
pub const PORTSCX_PORT_SPEED_HIGH: c_uint = 0x08000000;
pub const PORTSCX_PORT_SPEED_UNDEF: c_uint = 0x0C000000;
pub const PORTSCX_SPEED_BIT_POS: c_int = 26;
// bit 28 is parallel transceiver width for UTMI interface
pub const PORTSCX_PTW: c_uint = 0x10000000;
pub const PORTSCX_PTW_8BIT: c_uint = 0x00000000;
pub const PORTSCX_PTW_16BIT: c_uint = 0x10000000;
// bit 31-30 are port transceiver select
pub const PORTSCX_PTS_UTMI: c_uint = 0x00000000;
pub const PORTSCX_PTS_ULPI: c_uint = 0x80000000;
pub const PORTSCX_PTS_FSLS: c_uint = 0xC0000000;
pub const PORTSCX_PTS_BIT_POS: c_int = 30;
// otgsc Register Bit Masks
pub const OTGSC_CTRL_VUSB_DISCHARGE: c_uint = 0x00000001;
pub const OTGSC_CTRL_VUSB_CHARGE: c_uint = 0x00000002;
pub const OTGSC_CTRL_OTG_TERM: c_uint = 0x00000008;
pub const OTGSC_CTRL_DATA_PULSING: c_uint = 0x00000010;
pub const OTGSC_STS_USB_ID: c_uint = 0x00000100;
pub const OTGSC_STS_A_VBUS_VALID: c_uint = 0x00000200;
pub const OTGSC_STS_A_SESSION_VALID: c_uint = 0x00000400;
pub const OTGSC_STS_B_SESSION_VALID: c_uint = 0x00000800;
pub const OTGSC_STS_B_SESSION_END: c_uint = 0x00001000;
pub const OTGSC_STS_1MS_TOGGLE: c_uint = 0x00002000;
pub const OTGSC_STS_DATA_PULSING: c_uint = 0x00004000;
pub const OTGSC_INTSTS_USB_ID: c_uint = 0x00010000;
pub const OTGSC_INTSTS_A_VBUS_VALID: c_uint = 0x00020000;
pub const OTGSC_INTSTS_A_SESSION_VALID: c_uint = 0x00040000;
pub const OTGSC_INTSTS_B_SESSION_VALID: c_uint = 0x00080000;
pub const OTGSC_INTSTS_B_SESSION_END: c_uint = 0x00100000;
pub const OTGSC_INTSTS_1MS: c_uint = 0x00200000;
pub const OTGSC_INTSTS_DATA_PULSING: c_uint = 0x00400000;
pub const OTGSC_INTR_USB_ID: c_uint = 0x01000000;
pub const OTGSC_INTR_A_VBUS_VALID: c_uint = 0x02000000;
pub const OTGSC_INTR_A_SESSION_VALID: c_uint = 0x04000000;
pub const OTGSC_INTR_B_SESSION_VALID: c_uint = 0x08000000;
pub const OTGSC_INTR_B_SESSION_END: c_uint = 0x10000000;
pub const OTGSC_INTR_1MS_TIMER: c_uint = 0x20000000;
pub const OTGSC_INTR_DATA_PULSING: c_uint = 0x40000000;
// USB MODE Register Bit Masks
pub const USB_MODE_CTRL_MODE_IDLE: c_uint = 0x00000000;
pub const USB_MODE_CTRL_MODE_DEVICE: c_uint = 0x00000002;
pub const USB_MODE_CTRL_MODE_HOST: c_uint = 0x00000003;
pub const USB_MODE_CTRL_MODE_MASK: c_uint = 0x00000003;
pub const USB_MODE_CTRL_MODE_RSV: c_uint = 0x00000001;
pub const USB_MODE_ES: c_uint = 0x00000004 /* Endian Select */;
pub const USB_MODE_SETUP_LOCK_OFF: c_uint = 0x00000008;
pub const USB_MODE_STREAM_DISABLE: c_uint = 0x00000010;
// Endpoint Flush Register
pub const EPFLUSH_TX_OFFSET: c_uint = 0x00010000;
pub const EPFLUSH_RX_OFFSET: c_uint = 0x00000000;
// Endpoint Setup Status bit masks
pub const EP_SETUP_STATUS_MASK: c_uint = 0x0000003F;
pub const EP_SETUP_STATUS_EP0: c_uint = 0x00000001;
// ENDPOINTCTRLx  Register Bit Masks
pub const EPCTRL_TX_ENABLE: c_uint = 0x00800000;
pub const EPCTRL_TX_DATA_TOGGLE_RST: c_uint = 0x00400000	/* Not EP0 */;
pub const EPCTRL_TX_DATA_TOGGLE_INH: c_uint = 0x00200000	/* Not EP0 */;
pub const EPCTRL_TX_TYPE: c_uint = 0x000C0000;
pub const EPCTRL_TX_DATA_SOURCE: c_uint = 0x00020000	/* Not EP0 */;
pub const EPCTRL_TX_EP_STALL: c_uint = 0x00010000;
pub const EPCTRL_RX_ENABLE: c_uint = 0x00000080;
pub const EPCTRL_RX_DATA_TOGGLE_RST: c_uint = 0x00000040	/* Not EP0 */;
pub const EPCTRL_RX_DATA_TOGGLE_INH: c_uint = 0x00000020	/* Not EP0 */;
pub const EPCTRL_RX_TYPE: c_uint = 0x0000000C;
pub const EPCTRL_RX_DATA_SINK: c_uint = 0x00000002	/* Not EP0 */;
pub const EPCTRL_RX_EP_STALL: c_uint = 0x00000001;
// bit 19-18 and 3-2 are endpoint type
pub const EPCTRL_EP_TYPE_CONTROL: c_int = 0;
pub const EPCTRL_EP_TYPE_ISO: c_int = 1;
pub const EPCTRL_EP_TYPE_BULK: c_int = 2;
pub const EPCTRL_EP_TYPE_INTERRUPT: c_int = 3;
pub const EPCTRL_TX_EP_TYPE_SHIFT: c_int = 18;
pub const EPCTRL_RX_EP_TYPE_SHIFT: c_int = 2;
// SNOOPn Register Bit Masks
pub const SNOOP_ADDRESS_MASK: c_uint = 0xFFFFF000;
pub const SNOOP_SIZE_ZERO: c_uint = 0x00	/* snooping disable */;
pub const SNOOP_SIZE_4KB: c_uint = 0x0B	/* 4KB snoop size */;
pub const SNOOP_SIZE_8KB: c_uint = 0x0C;
pub const SNOOP_SIZE_16KB: c_uint = 0x0D;
pub const SNOOP_SIZE_32KB: c_uint = 0x0E;
pub const SNOOP_SIZE_64KB: c_uint = 0x0F;
pub const SNOOP_SIZE_128KB: c_uint = 0x10;
pub const SNOOP_SIZE_256KB: c_uint = 0x11;
pub const SNOOP_SIZE_512KB: c_uint = 0x12;
pub const SNOOP_SIZE_1MB: c_uint = 0x13;
pub const SNOOP_SIZE_2MB: c_uint = 0x14;
pub const SNOOP_SIZE_4MB: c_uint = 0x15;
pub const SNOOP_SIZE_8MB: c_uint = 0x16;
pub const SNOOP_SIZE_16MB: c_uint = 0x17;
pub const SNOOP_SIZE_32MB: c_uint = 0x18;
pub const SNOOP_SIZE_64MB: c_uint = 0x19;
pub const SNOOP_SIZE_128MB: c_uint = 0x1A;
pub const SNOOP_SIZE_256MB: c_uint = 0x1B;
pub const SNOOP_SIZE_512MB: c_uint = 0x1C;
pub const SNOOP_SIZE_1GB: c_uint = 0x1D;
pub const SNOOP_SIZE_2GB: c_uint = 0x1E	/* 2GB snoop size */;
// pri_ctrl Register Bit Masks
pub const PRI_CTRL_PRI_LVL1: c_uint = 0x0000000C;
pub const PRI_CTRL_PRI_LVL0: c_uint = 0x00000003;
// si_ctrl Register Bit Masks
pub const SI_CTRL_ERR_DISABLE: c_uint = 0x00000010;
pub const SI_CTRL_IDRC_DISABLE: c_uint = 0x00000008;
pub const SI_CTRL_RD_SAFE_EN: c_uint = 0x00000004;
pub const SI_CTRL_RD_PREFETCH_DISABLE: c_uint = 0x00000002;
pub const SI_CTRL_RD_PREFEFETCH_VAL: c_uint = 0x00000001;
// control Register Bit Masks
pub const USB_CTRL_IOENB: c_uint = 0x00000004;
pub const USB_CTRL_ULPI_INT0EN: c_uint = 0x00000001;
pub const USB_CTRL_UTMI_PHY_EN: c_uint = 0x00000200;
pub const USB_CTRL_USB_EN: c_uint = 0x00000004;
pub const USB_CTRL_ULPI_PHY_CLK_SEL: c_uint = 0x00000400;
// Endpoint Queue Head data struct
// Rem: all the variables of qh are LittleEndian Mode
// and NEXT_POINTER_MASK should operate on a LittleEndian, Phy Addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep_queue_head {
    pub len: *mut *mut u32 max_pkt_length; / Mult(31-30) , Zlt(29) , Max Pkt,
    pub /: *mut *mut u32 curr_dtd_ptr; / Current dTD Pointer(31-5),
    pub /: *mut *mut u32 next_dtd_ptr; / Next dTD Pointer(31-5), T(0),
    pub (15),: *mut *mut u32 size_ioc_int_sts; / Total bytes (30-16), IOC,
    pub /: *mut *mut u32 buff_ptr0; / Buffer pointer Page 0 (31-12),
    pub /: *mut *mut u32 buff_ptr1; / Buffer pointer Page 1 (31-12),
    pub /: *mut *mut u32 buff_ptr2; / Buffer pointer Page 2 (31-12),
    pub /: *mut *mut u32 buff_ptr3; / Buffer pointer Page 3 (31-12),
    pub /: *mut *mut u32 buff_ptr4; / Buffer pointer Page 4 (31-12),
    pub res1: u32,
    pub /: *mut *mut u8 setup_buffer[8]; / Setup data 8 bytes,
    pub res2: [u32; 4],
}

// Endpoint Queue Head Bit Masks
pub const EP_QUEUE_HEAD_MULT_POS: c_int = 30;
pub const EP_QUEUE_HEAD_ZLT_SEL: c_uint = 0x20000000;
pub const EP_QUEUE_HEAD_MAX_PKT_LEN_POS: c_int = 16;

pub const EP_QUEUE_HEAD_IOS: c_uint = 0x00008000;
pub const EP_QUEUE_HEAD_NEXT_TERMINATE: c_uint = 0x00000001;
pub const EP_QUEUE_HEAD_IOC: c_uint = 0x00008000;
pub const EP_QUEUE_HEAD_MULTO: c_uint = 0x00000C00;
pub const EP_QUEUE_HEAD_STATUS_HALT: c_uint = 0x00000040;
pub const EP_QUEUE_HEAD_STATUS_ACTIVE: c_uint = 0x00000080;
pub const EP_QUEUE_CURRENT_OFFSET_MASK: c_uint = 0x00000FFF;
pub const EP_QUEUE_HEAD_NEXT_POINTER_MASK: c_uint = 0xFFFFFFE0;
pub const EP_QUEUE_FRINDEX_MASK: c_uint = 0x000007FF;
pub const EP_MAX_LENGTH_TRANSFER: c_uint = 0x4000;
// Endpoint Transfer Descriptor data struct
// Rem: all the variables of td are LittleEndian Mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ep_td_struct {
    pub set: *mut *mut u32 next_td_ptr; / Next TD pointer(31-5), T(0),
    pub (15),: *mut *mut u32 size_ioc_sts; / Total bytes (30-16), IOC,
    pub /: *mut *mut u32 buff_ptr0; / Buffer pointer Page 0,
    pub /: *mut *mut u32 buff_ptr1; / Buffer pointer Page 1,
    pub /: *mut *mut u32 buff_ptr2; / Buffer pointer Page 2,
    pub /: *mut *mut u32 buff_ptr3; / Buffer pointer Page 3,
    pub /: *mut *mut u32 buff_ptr4; / Buffer pointer Page 4,
    pub res: u32,
// 32 bytes
    pub /: *mut *mut dma_addr_t td_dma; / dma address for this td,
// virtual address of next td specified in next_td_ptr
    pub next_td_virt: *mut ep_td_struct,
}

// Endpoint Transfer Descriptor bit Masks
pub const DTD_NEXT_TERMINATE: c_uint = 0x00000001;
pub const DTD_IOC: c_uint = 0x00008000;
pub const DTD_STATUS_ACTIVE: c_uint = 0x00000080;
pub const DTD_STATUS_HALTED: c_uint = 0x00000040;
pub const DTD_STATUS_DATA_BUFF_ERR: c_uint = 0x00000020;
pub const DTD_STATUS_TRANSACTION_ERR: c_uint = 0x00000008;
pub const DTD_RESERVED_FIELDS: c_uint = 0x80007300;
pub const DTD_ADDR_MASK: c_uint = 0xFFFFFFE0;
pub const DTD_PACKET_SIZE: c_uint = 0x7FFF0000;
pub const DTD_LENGTH_BIT_POS: c_int = 16;

// Alignment requirements; must be a power of two
pub const DTD_ALIGNMENT: c_uint = 0x20;
pub const QH_ALIGNMENT: c_int = 2048;
// Controller dma boundary
pub const UDC_DMA_BOUNDARY: c_uint = 0x1000;
// -------------------------------------------------------------------------
// ### driver private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_req {
    pub req: usb_request,
    pub queue: list_head,
// ep_queue() func will add
    pub ep: *mut fsl_ep,
    pub mapped:1: unsigned,
    pub List: *mut *mut *mut *mut ep_td_head, tail; / For dTD,
    pub dtd_count: c_uint,
}

pub const REQ_UNCOMPLETE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_ep {
    pub ep: usb_ep,
    pub queue: list_head,
    pub udc: *mut fsl_udc,
    pub qh: *mut ep_queue_head,
    pub gadget: *mut usb_gadget,
    pub name: [c_char; 14],
    pub stopped:1: unsigned,
}

pub const EP_DIR_IN: c_int = 1;
pub const EP_DIR_OUT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_udc {
    pub dev: *mut device,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub pdata: *mut fsl_usb2_platform_data,
    pub /: *mut *mut *mut completion done; / to make sure release() is done,
    pub eps: *mut fsl_ep,
    pub max_ep: c_uint,
    pub irq: c_uint,
    pub local_setup_buff: usb_ctrlrequest,
    pub lock: spinlock_t,
    pub transceiver: *mut usb_phy,
    pub softconnect:1: unsigned,
    pub vbus_active:1: unsigned,
    pub stopped:1: unsigned,
    pub remote_wakeup:1: unsigned,
    pub already_stopped:1: unsigned,
    pub big_endian_desc:1: unsigned,
    pub /: *mut *mut *mut ep_queue_head ep_qh; / Endpoints Queue-Head,
    pub /: *mut *mut *mut fsl_req status_req; / ep0 status request,
    pub /: *mut *mut *mut dma_pool td_pool; / dma pool for DTD,
    pub phy_mode: fsl_usb2_phy_modes,
    pub adjustment*/: *mut *mut size_t ep_qh_size; / size after alignment,
    pub /: *mut *mut dma_addr_t ep_qh_dma; / dma address of QH,
    pub /: *mut *mut u32 max_pipes; / Device max pipes,
    pub /: *mut *mut u32 bus_reset; / Device is bus resetting,
    pub /: *mut *mut u32 resume_state; / USB state to resume,
    pub /: *mut *mut u32 usb_state; / USB current state,
    pub /: *mut *mut u32 ep0_state; / Endpoint zero state,
    pub be: *mut *mut u32 ep0_dir; / Endpoint zero direction: can,
    pub /: *mut *mut u8 device_address; / Device USB address,
}

// -------------------------------------------------------------------------
// ### Add board specific defines here
//
// ### pipe direction macro from device view
//

//
// ### internal used help routines.
//

// 2 + ((windex & USB_DIR_IN) ? 1 : 0))

// we only have one ep0 structure but two queue heads
