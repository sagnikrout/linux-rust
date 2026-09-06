//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/udc/m66592-udc.h
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
// M66592 UDC (USB gadget)
//
// Copyright (C) 2006-2007 Renesas Solutions Corp.
//
// Author : Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>
//

pub const M66592_SYSCFG: c_uint = 0x00;
pub const M66592_XTAL: c_uint = 0xC000	/* b15-14: Crystal selection */;
pub const M66592_XTAL48: c_uint = 0x8000		/* 48MHz */;
pub const M66592_XTAL24: c_uint = 0x4000		/* 24MHz */;
pub const M66592_XTAL12: c_uint = 0x0000		/* 12MHz */;
pub const M66592_XCKE: c_uint = 0x2000	/* b13: External clock enable */;
pub const M66592_RCKE: c_uint = 0x1000	/* b12: Register clock enable */;
pub const M66592_PLLC: c_uint = 0x0800	/* b11: PLL control */;
pub const M66592_SCKE: c_uint = 0x0400	/* b10: USB clock enable */;
pub const M66592_ATCKM: c_uint = 0x0100	/* b8: Automatic clock supply */;
pub const M66592_HSE: c_uint = 0x0080	/* b7: Hi-speed enable */;
pub const M66592_DCFM: c_uint = 0x0040	/* b6: Controller function select  */;
pub const M66592_DMRPD: c_uint = 0x0020	/* b5: D- pull down control */;
pub const M66592_DPRPU: c_uint = 0x0010	/* b4: D+ pull up control */;
pub const M66592_FSRPC: c_uint = 0x0004	/* b2: Full-speed receiver enable */;
pub const M66592_PCUT: c_uint = 0x0002	/* b1: Low power sleep enable */;
pub const M66592_USBE: c_uint = 0x0001	/* b0: USB module operation enable */;
pub const M66592_SYSSTS: c_uint = 0x02;
pub const M66592_LNST: c_uint = 0x0003	/* b1-0: D+, D- line status */;
pub const M66592_SE1: c_uint = 0x0003		/* SE1 */;
pub const M66592_KSTS: c_uint = 0x0002		/* K State */;
pub const M66592_JSTS: c_uint = 0x0001		/* J State */;
pub const M66592_SE0: c_uint = 0x0000		/* SE0 */;
pub const M66592_DVSTCTR: c_uint = 0x04;
pub const M66592_WKUP: c_uint = 0x0100	/* b8: Remote wakeup */;
pub const M66592_RWUPE: c_uint = 0x0080	/* b7: Remote wakeup sense */;
pub const M66592_USBRST: c_uint = 0x0040	/* b6: USB reset enable */;
pub const M66592_RESUME: c_uint = 0x0020	/* b5: Resume enable */;
pub const M66592_UACT: c_uint = 0x0010	/* b4: USB bus enable */;
pub const M66592_RHST: c_uint = 0x0003	/* b1-0: Reset handshake status */;
pub const M66592_HSMODE: c_uint = 0x0003		/* Hi-Speed mode */;
pub const M66592_FSMODE: c_uint = 0x0002		/* Full-Speed mode */;
pub const M66592_HSPROC: c_uint = 0x0001		/* HS handshake is processing */;
pub const M66592_TESTMODE: c_uint = 0x06;
pub const M66592_UTST: c_uint = 0x000F	/* b4-0: Test select */;
pub const M66592_H_TST_PACKET: c_uint = 0x000C		/* HOST TEST Packet */;
pub const M66592_H_TST_SE0_NAK: c_uint = 0x000B		/* HOST TEST SE0 NAK */;
pub const M66592_H_TST_K: c_uint = 0x000A		/* HOST TEST K */;
pub const M66592_H_TST_J: c_uint = 0x0009		/* HOST TEST J */;
pub const M66592_H_TST_NORMAL: c_uint = 0x0000		/* HOST Normal Mode */;
pub const M66592_P_TST_PACKET: c_uint = 0x0004		/* PERI TEST Packet */;
pub const M66592_P_TST_SE0_NAK: c_uint = 0x0003		/* PERI TEST SE0 NAK */;
pub const M66592_P_TST_K: c_uint = 0x0002		/* PERI TEST K */;
pub const M66592_P_TST_J: c_uint = 0x0001		/* PERI TEST J */;
pub const M66592_P_TST_NORMAL: c_uint = 0x0000		/* PERI Normal Mode */;
// built-in registers
pub const M66592_CFBCFG: c_uint = 0x0A;
pub const M66592_D0FBCFG: c_uint = 0x0C;
pub const M66592_LITTLE: c_uint = 0x0100	/* b8: Little endian mode */;
// external chip case
pub const M66592_PINCFG: c_uint = 0x0A;
pub const M66592_LDRV: c_uint = 0x8000	/* b15: Drive Current Adjust */;
pub const M66592_BIGEND: c_uint = 0x0100	/* b8: Big endian mode */;
pub const M66592_DMA0CFG: c_uint = 0x0C;
pub const M66592_DMA1CFG: c_uint = 0x0E;
pub const M66592_DREQA: c_uint = 0x4000	/* b14: Dreq active select */;
pub const M66592_BURST: c_uint = 0x2000	/* b13: Burst mode */;
pub const M66592_DACKA: c_uint = 0x0400	/* b10: Dack active select */;
pub const M66592_DFORM: c_uint = 0x0380	/* b9-7: DMA mode select */;
pub const M66592_CPU_ADR_RD_WR: c_uint = 0x0000   /* Address + RD/WR mode (CPU bus) */;
pub const M66592_CPU_DACK_RD_WR: c_uint = 0x0100   /* DACK + RD/WR mode (CPU bus) */;
pub const M66592_CPU_DACK_ONLY: c_uint = 0x0180   /* DACK only mode (CPU bus) */;
pub const M66592_SPLIT_DACK_ONLY: c_uint = 0x0200   /* DACK only mode (SPLIT bus) */;
pub const M66592_SPLIT_DACK_DSTB: c_uint = 0x0300   /* DACK + DSTB0 mode (SPLIT bus) */;
pub const M66592_DENDA: c_uint = 0x0040	/* b6: Dend active select */;
pub const M66592_PKTM: c_uint = 0x0020	/* b5: Packet mode */;
pub const M66592_DENDE: c_uint = 0x0010	/* b4: Dend enable */;
pub const M66592_OBUS: c_uint = 0x0004	/* b2: OUTbus mode */;
// common case
pub const M66592_CFIFO: c_uint = 0x10;
pub const M66592_D0FIFO: c_uint = 0x14;
pub const M66592_D1FIFO: c_uint = 0x18;
pub const M66592_CFIFOSEL: c_uint = 0x1E;
pub const M66592_D0FIFOSEL: c_uint = 0x24;
pub const M66592_D1FIFOSEL: c_uint = 0x2A;
pub const M66592_RCNT: c_uint = 0x8000	/* b15: Read count mode */;
pub const M66592_REW: c_uint = 0x4000	/* b14: Buffer rewind */;
pub const M66592_DCLRM: c_uint = 0x2000	/* b13: DMA buffer clear mode */;
pub const M66592_DREQE: c_uint = 0x1000	/* b12: DREQ output enable */;
pub const M66592_MBW_8: c_uint = 0x0000   /*  8bit */;
pub const M66592_MBW_16: c_uint = 0x0400   /* 16bit */;
pub const M66592_MBW_32: c_uint = 0x0800   /* 32bit */;
pub const M66592_TRENB: c_uint = 0x0200	/* b9: Transaction counter enable */;
pub const M66592_TRCLR: c_uint = 0x0100	/* b8: Transaction counter clear */;
pub const M66592_DEZPM: c_uint = 0x0080	/* b7: Zero-length packet mode */;
pub const M66592_ISEL: c_uint = 0x0020	/* b5: DCP FIFO port direction select */;
pub const M66592_CURPIPE: c_uint = 0x0007	/* b2-0: PIPE select */;
pub const M66592_CFIFOCTR: c_uint = 0x20;
pub const M66592_D0FIFOCTR: c_uint = 0x26;
pub const M66592_D1FIFOCTR: c_uint = 0x2c;
pub const M66592_BVAL: c_uint = 0x8000	/* b15: Buffer valid flag */;
pub const M66592_BCLR: c_uint = 0x4000	/* b14: Buffer clear */;
pub const M66592_FRDY: c_uint = 0x2000	/* b13: FIFO ready */;
pub const M66592_DTLN: c_uint = 0x0FFF	/* b11-0: FIFO received data length */;
pub const M66592_CFIFOSIE: c_uint = 0x22;
pub const M66592_TGL: c_uint = 0x8000	/* b15: Buffer toggle */;
pub const M66592_SCLR: c_uint = 0x4000	/* b14: Buffer clear */;
pub const M66592_SBUSY: c_uint = 0x2000	/* b13: SIE_FIFO busy */;
pub const M66592_D0FIFOTRN: c_uint = 0x28;
pub const M66592_D1FIFOTRN: c_uint = 0x2E;
pub const M66592_TRNCNT: c_uint = 0xFFFF	/* b15-0: Transaction counter */;
pub const M66592_INTENB0: c_uint = 0x30;
pub const M66592_VBSE: c_uint = 0x8000	/* b15: VBUS interrupt */;
pub const M66592_RSME: c_uint = 0x4000	/* b14: Resume interrupt */;
pub const M66592_SOFE: c_uint = 0x2000	/* b13: Frame update interrupt */;
pub const M66592_DVSE: c_uint = 0x1000	/* b12: Device state transition interrupt */;
pub const M66592_CTRE: c_uint = 0x0800	/* b11: Control transfer stage transition irq */;
pub const M66592_BEMPE: c_uint = 0x0400	/* b10: Buffer empty interrupt */;
pub const M66592_NRDYE: c_uint = 0x0200	/* b9: Buffer not ready interrupt */;
pub const M66592_BRDYE: c_uint = 0x0100	/* b8: Buffer ready interrupt */;
pub const M66592_URST: c_uint = 0x0080	/* b7: USB reset detected interrupt */;
pub const M66592_SADR: c_uint = 0x0040	/* b6: Set address executed interrupt */;
pub const M66592_SCFG: c_uint = 0x0020	/* b5: Set configuration executed interrupt */;
pub const M66592_SUSP: c_uint = 0x0010	/* b4: Suspend detected interrupt */;
pub const M66592_WDST: c_uint = 0x0008	/* b3: Control write data stage completed irq */;
pub const M66592_RDST: c_uint = 0x0004	/* b2: Control read data stage completed irq */;
pub const M66592_CMPL: c_uint = 0x0002	/* b1: Control transfer complete interrupt */;
pub const M66592_SERR: c_uint = 0x0001	/* b0: Sequence error interrupt */;
pub const M66592_INTENB1: c_uint = 0x32;
pub const M66592_BCHGE: c_uint = 0x4000	/* b14: USB us chenge interrupt */;
pub const M66592_DTCHE: c_uint = 0x1000	/* b12: Detach sense interrupt */;
pub const M66592_SIGNE: c_uint = 0x0020	/* b5: SETUP IGNORE interrupt */;
pub const M66592_SACKE: c_uint = 0x0010	/* b4: SETUP ACK interrupt */;
pub const M66592_BRDYM: c_uint = 0x0004	/* b2: BRDY clear timing */;
pub const M66592_INTL: c_uint = 0x0002	/* b1: Interrupt sense select */;
pub const M66592_PCSE: c_uint = 0x0001	/* b0: PCUT enable by CS assert */;
pub const M66592_BRDYENB: c_uint = 0x36;
pub const M66592_BRDYSTS: c_uint = 0x46;
pub const M66592_BRDY7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const M66592_BRDY6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const M66592_BRDY5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const M66592_BRDY4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const M66592_BRDY3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const M66592_BRDY2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const M66592_BRDY1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const M66592_BRDY0: c_uint = 0x0001	/* b1: PIPE0 */;
pub const M66592_NRDYENB: c_uint = 0x38;
pub const M66592_NRDYSTS: c_uint = 0x48;
pub const M66592_NRDY7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const M66592_NRDY6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const M66592_NRDY5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const M66592_NRDY4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const M66592_NRDY3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const M66592_NRDY2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const M66592_NRDY1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const M66592_NRDY0: c_uint = 0x0001	/* b1: PIPE0 */;
pub const M66592_BEMPENB: c_uint = 0x3A;
pub const M66592_BEMPSTS: c_uint = 0x4A;
pub const M66592_BEMP7: c_uint = 0x0080	/* b7: PIPE7 */;
pub const M66592_BEMP6: c_uint = 0x0040	/* b6: PIPE6 */;
pub const M66592_BEMP5: c_uint = 0x0020	/* b5: PIPE5 */;
pub const M66592_BEMP4: c_uint = 0x0010	/* b4: PIPE4 */;
pub const M66592_BEMP3: c_uint = 0x0008	/* b3: PIPE3 */;
pub const M66592_BEMP2: c_uint = 0x0004	/* b2: PIPE2 */;
pub const M66592_BEMP1: c_uint = 0x0002	/* b1: PIPE1 */;
pub const M66592_BEMP0: c_uint = 0x0001	/* b0: PIPE0 */;
pub const M66592_SOFCFG: c_uint = 0x3C;
pub const M66592_SOFM: c_uint = 0x000C	/* b3-2: SOF palse mode */;
pub const M66592_SOF_125US: c_uint = 0x0008   /* SOF OUT 125us uFrame Signal */;
pub const M66592_SOF_1MS: c_uint = 0x0004   /* SOF OUT 1ms Frame Signal */;
pub const M66592_SOF_DISABLE: c_uint = 0x0000   /* SOF OUT Disable */;
pub const M66592_INTSTS0: c_uint = 0x40;
pub const M66592_VBINT: c_uint = 0x8000	/* b15: VBUS interrupt */;
pub const M66592_RESM: c_uint = 0x4000	/* b14: Resume interrupt */;
pub const M66592_SOFR: c_uint = 0x2000	/* b13: SOF frame update interrupt */;
pub const M66592_DVST: c_uint = 0x1000	/* b12: Device state transition */;
pub const M66592_CTRT: c_uint = 0x0800	/* b11: Control stage transition */;
pub const M66592_BEMP: c_uint = 0x0400	/* b10: Buffer empty interrupt */;
pub const M66592_NRDY: c_uint = 0x0200	/* b9: Buffer not ready interrupt */;
pub const M66592_BRDY: c_uint = 0x0100	/* b8: Buffer ready interrupt */;
pub const M66592_VBSTS: c_uint = 0x0080	/* b7: VBUS input port */;
pub const M66592_DVSQ: c_uint = 0x0070	/* b6-4: Device state */;
pub const M66592_DS_SPD_CNFG: c_uint = 0x0070	   /* Suspend Configured */;
pub const M66592_DS_SPD_ADDR: c_uint = 0x0060	   /* Suspend Address */;
pub const M66592_DS_SPD_DFLT: c_uint = 0x0050	   /* Suspend Default */;
pub const M66592_DS_SPD_POWR: c_uint = 0x0040	   /* Suspend Powered */;
pub const M66592_DS_SUSP: c_uint = 0x0040	   /* Suspend */;
pub const M66592_DS_CNFG: c_uint = 0x0030	   /* Configured */;
pub const M66592_DS_ADDS: c_uint = 0x0020	   /* Address */;
pub const M66592_DS_DFLT: c_uint = 0x0010	   /* Default */;
pub const M66592_DS_POWR: c_uint = 0x0000	   /* Powered */;
pub const M66592_DVSQS: c_uint = 0x0030	/* b5-4: Device state */;
pub const M66592_VALID: c_uint = 0x0008	/* b3: Setup packet detected flag */;
pub const M66592_CTSQ: c_uint = 0x0007	/* b2-0: Control transfer stage */;
pub const M66592_CS_SQER: c_uint = 0x0006	  /* Sequence error */;
pub const M66592_CS_WRND: c_uint = 0x0005	  /* Control write nodata status */;
pub const M66592_CS_WRSS: c_uint = 0x0004	  /* Control write status stage */;
pub const M66592_CS_WRDS: c_uint = 0x0003	  /* Control write data stage */;
pub const M66592_CS_RDSS: c_uint = 0x0002	  /* Control read status stage */;
pub const M66592_CS_RDDS: c_uint = 0x0001	  /* Control read data stage */;
pub const M66592_CS_IDST: c_uint = 0x0000	  /* Idle or setup stage */;
pub const M66592_INTSTS1: c_uint = 0x42;
pub const M66592_BCHG: c_uint = 0x4000	/* b14: USB bus chenge interrupt */;
pub const M66592_DTCH: c_uint = 0x1000	/* b12: Detach sense interrupt */;
pub const M66592_SIGN: c_uint = 0x0020	/* b5: SETUP IGNORE interrupt */;
pub const M66592_SACK: c_uint = 0x0010	/* b4: SETUP ACK interrupt */;
pub const M66592_FRMNUM: c_uint = 0x4C;
pub const M66592_OVRN: c_uint = 0x8000	/* b15: Overrun error */;
pub const M66592_CRCE: c_uint = 0x4000	/* b14: Received data error */;
pub const M66592_SOFRM: c_uint = 0x0800	/* b11: SOF output mode */;
pub const M66592_FRNM: c_uint = 0x07FF	/* b10-0: Frame number */;
pub const M66592_UFRMNUM: c_uint = 0x4E;
pub const M66592_UFRNM: c_uint = 0x0007	/* b2-0: Micro frame number */;
pub const M66592_RECOVER: c_uint = 0x50;
pub const M66592_STSRECOV: c_uint = 0x0700	/* Status recovery */;
pub const M66592_STSR_HI: c_uint = 0x0400		  /* FULL(0) or HI(1) Speed */;
pub const M66592_STSR_DEFAULT: c_uint = 0x0100		  /* Default state */;
pub const M66592_STSR_ADDRESS: c_uint = 0x0200		  /* Address state */;
pub const M66592_STSR_CONFIG: c_uint = 0x0300		  /* Configured state */;
pub const M66592_USBADDR: c_uint = 0x007F	/* b6-0: USB address */;
pub const M66592_USBREQ: c_uint = 0x54;
pub const M66592_bRequest: c_uint = 0xFF00	/* b15-8: bRequest */;
pub const M66592_GET_STATUS: c_uint = 0x0000;
pub const M66592_CLEAR_FEATURE: c_uint = 0x0100;
pub const M66592_ReqRESERVED: c_uint = 0x0200;
pub const M66592_SET_FEATURE: c_uint = 0x0300;
pub const M66592_ReqRESERVED1: c_uint = 0x0400;
pub const M66592_SET_ADDRESS: c_uint = 0x0500;
pub const M66592_GET_DESCRIPTOR: c_uint = 0x0600;
pub const M66592_SET_DESCRIPTOR: c_uint = 0x0700;
pub const M66592_GET_CONFIGURATION: c_uint = 0x0800;
pub const M66592_SET_CONFIGURATION: c_uint = 0x0900;
pub const M66592_GET_INTERFACE: c_uint = 0x0A00;
pub const M66592_SET_INTERFACE: c_uint = 0x0B00;
pub const M66592_SYNCH_FRAME: c_uint = 0x0C00;
pub const M66592_bmRequestType: c_uint = 0x00FF	/* b7-0: bmRequestType */;
pub const M66592_bmRequestTypeDir: c_uint = 0x0080	/* b7  : Data direction */;
pub const M66592_HOST_TO_DEVICE: c_uint = 0x0000;
pub const M66592_DEVICE_TO_HOST: c_uint = 0x0080;
pub const M66592_bmRequestTypeType: c_uint = 0x0060	/* b6-5: Type */;
pub const M66592_STANDARD: c_uint = 0x0000;
pub const M66592_CLASS: c_uint = 0x0020;
pub const M66592_VENDOR: c_uint = 0x0040;
pub const M66592_bmRequestTypeRecip: c_uint = 0x001F	/* b4-0: Recipient */;
pub const M66592_DEVICE: c_uint = 0x0000;
pub const M66592_INTERFACE: c_uint = 0x0001;
pub const M66592_ENDPOINT: c_uint = 0x0002;
pub const M66592_USBVAL: c_uint = 0x56;
pub const M66592_wValue: c_uint = 0xFFFF	/* b15-0: wValue */;
// Standard Feature Selector
pub const M66592_ENDPOINT_HALT: c_uint = 0x0000;
pub const M66592_DEVICE_REMOTE_WAKEUP: c_uint = 0x0001;
pub const M66592_TEST_MODE: c_uint = 0x0002;
// Descriptor Types
pub const M66592_DT_TYPE: c_uint = 0xFF00;

pub const M66592_DT_DEVICE: c_uint = 0x01;
pub const M66592_DT_CONFIGURATION: c_uint = 0x02;
pub const M66592_DT_STRING: c_uint = 0x03;
pub const M66592_DT_INTERFACE: c_uint = 0x04;
pub const M66592_DT_ENDPOINT: c_uint = 0x05;
pub const M66592_DT_DEVICE_QUALIFIER: c_uint = 0x06;
pub const M66592_DT_OTHER_SPEED_CONFIGURATION: c_uint = 0x07;
pub const M66592_DT_INTERFACE_POWER: c_uint = 0x08;
pub const M66592_DT_INDEX: c_uint = 0x00FF;
pub const M66592_CONF_NUM: c_uint = 0x00FF;
pub const M66592_ALT_SET: c_uint = 0x00FF;
pub const M66592_USBINDEX: c_uint = 0x58;
pub const M66592_wIndex: c_uint = 0xFFFF	/* b15-0: wIndex */;
pub const M66592_TEST_SELECT: c_uint = 0xFF00	/* b15-b8: Test Mode */;
pub const M66592_TEST_J: c_uint = 0x0100	  /* Test_J */;
pub const M66592_TEST_K: c_uint = 0x0200	  /* Test_K */;
pub const M66592_TEST_SE0_NAK: c_uint = 0x0300	  /* Test_SE0_NAK */;
pub const M66592_TEST_PACKET: c_uint = 0x0400	  /* Test_Packet */;
pub const M66592_TEST_FORCE_ENABLE: c_uint = 0x0500	  /* Test_Force_Enable */;
pub const M66592_TEST_STSelectors: c_uint = 0x0600	  /* Standard test selectors */;
pub const M66592_TEST_Reserved: c_uint = 0x4000	  /* Reserved */;
pub const M66592_TEST_VSTModes: c_uint = 0xC000	  /* Vendor-specific tests */;
pub const M66592_EP_DIR: c_uint = 0x0080	/* b7: Endpoint Direction */;
pub const M66592_EP_DIR_IN: c_uint = 0x0080;
pub const M66592_EP_DIR_OUT: c_uint = 0x0000;
pub const M66592_USBLENG: c_uint = 0x5A;
pub const M66592_wLength: c_uint = 0xFFFF	/* b15-0: wLength */;
pub const M66592_DCPCFG: c_uint = 0x5C;
pub const M66592_CNTMD: c_uint = 0x0100	/* b8: Continuous transfer mode */;
pub const M66592_DIR: c_uint = 0x0010	/* b4: Control transfer DIR select */;
pub const M66592_DCPMAXP: c_uint = 0x5E;
pub const M66592_DEVSEL: c_uint = 0xC000	/* b15-14: Device address select */;
pub const M66592_DEVICE_0: c_uint = 0x0000		  /* Device address 0 */;
pub const M66592_DEVICE_1: c_uint = 0x4000		  /* Device address 1 */;
pub const M66592_DEVICE_2: c_uint = 0x8000		  /* Device address 2 */;
pub const M66592_DEVICE_3: c_uint = 0xC000		  /* Device address 3 */;
pub const M66592_MAXP: c_uint = 0x007F	/* b6-0: Maxpacket size of ep0 */;
pub const M66592_DCPCTR: c_uint = 0x60;
pub const M66592_BSTS: c_uint = 0x8000	/* b15: Buffer status */;
pub const M66592_SUREQ: c_uint = 0x4000	/* b14: Send USB request  */;
pub const M66592_SQCLR: c_uint = 0x0100	/* b8: Sequence toggle bit clear */;
pub const M66592_SQSET: c_uint = 0x0080	/* b7: Sequence toggle bit set */;
pub const M66592_SQMON: c_uint = 0x0040	/* b6: Sequence toggle bit monitor */;
pub const M66592_CCPL: c_uint = 0x0004	/* b2: control transfer complete */;
pub const M66592_PID: c_uint = 0x0003	/* b1-0: Response PID */;
pub const M66592_PID_STALL: c_uint = 0x0002		  /* STALL */;
pub const M66592_PID_BUF: c_uint = 0x0001		  /* BUF */;
pub const M66592_PID_NAK: c_uint = 0x0000		  /* NAK */;
pub const M66592_PIPESEL: c_uint = 0x64;
pub const M66592_PIPENM: c_uint = 0x0007	/* b2-0: Pipe select */;
pub const M66592_PIPE0: c_uint = 0x0000		  /* PIPE 0 */;
pub const M66592_PIPE1: c_uint = 0x0001		  /* PIPE 1 */;
pub const M66592_PIPE2: c_uint = 0x0002		  /* PIPE 2 */;
pub const M66592_PIPE3: c_uint = 0x0003		  /* PIPE 3 */;
pub const M66592_PIPE4: c_uint = 0x0004		  /* PIPE 4 */;
pub const M66592_PIPE5: c_uint = 0x0005		  /* PIPE 5 */;
pub const M66592_PIPE6: c_uint = 0x0006		  /* PIPE 6 */;
pub const M66592_PIPE7: c_uint = 0x0007		  /* PIPE 7 */;
pub const M66592_PIPECFG: c_uint = 0x66;
pub const M66592_TYP: c_uint = 0xC000	/* b15-14: Transfer type */;
pub const M66592_ISO: c_uint = 0xC000		  /* Isochronous */;
pub const M66592_INT: c_uint = 0x8000		  /* Interrupt */;
pub const M66592_BULK: c_uint = 0x4000		  /* Bulk */;
pub const M66592_BFRE: c_uint = 0x0400	/* b10: Buffer ready interrupt mode */;
pub const M66592_DBLB: c_uint = 0x0200	/* b9: Double buffer mode select */;
pub const M66592_CNTMD: c_uint = 0x0100	/* b8: Continuous transfer mode */;
pub const M66592_SHTNAK: c_uint = 0x0080	/* b7: Transfer end NAK */;
pub const M66592_DIR: c_uint = 0x0010	/* b4: Transfer direction select */;
pub const M66592_DIR_H_OUT: c_uint = 0x0010		  /* HOST OUT */;
pub const M66592_DIR_P_IN: c_uint = 0x0010		  /* PERI IN */;
pub const M66592_DIR_H_IN: c_uint = 0x0000		  /* HOST IN */;
pub const M66592_DIR_P_OUT: c_uint = 0x0000		  /* PERI OUT */;
pub const M66592_EPNUM: c_uint = 0x000F	/* b3-0: Eendpoint number select */;
pub const M66592_EP1: c_uint = 0x0001;
pub const M66592_EP2: c_uint = 0x0002;
pub const M66592_EP3: c_uint = 0x0003;
pub const M66592_EP4: c_uint = 0x0004;
pub const M66592_EP5: c_uint = 0x0005;
pub const M66592_EP6: c_uint = 0x0006;
pub const M66592_EP7: c_uint = 0x0007;
pub const M66592_EP8: c_uint = 0x0008;
pub const M66592_EP9: c_uint = 0x0009;
pub const M66592_EP10: c_uint = 0x000A;
pub const M66592_EP11: c_uint = 0x000B;
pub const M66592_EP12: c_uint = 0x000C;
pub const M66592_EP13: c_uint = 0x000D;
pub const M66592_EP14: c_uint = 0x000E;
pub const M66592_EP15: c_uint = 0x000F;
pub const M66592_PIPEBUF: c_uint = 0x68;
pub const M66592_BUFSIZE: c_uint = 0x7C00	/* b14-10: Pipe buffer size */;

pub const M66592_BUFNMB: c_uint = 0x00FF	/* b7-0: Pipe buffer number */;
pub const M66592_PIPEMAXP: c_uint = 0x6A;
pub const M66592_MXPS: c_uint = 0x07FF	/* b10-0: Maxpacket size */;
pub const M66592_PIPEPERI: c_uint = 0x6C;
pub const M66592_IFIS: c_uint = 0x1000	/* b12: ISO in-buffer flush mode */;
pub const M66592_IITV: c_uint = 0x0007	/* b2-0: ISO interval */;
pub const M66592_PIPE1CTR: c_uint = 0x70;
pub const M66592_PIPE2CTR: c_uint = 0x72;
pub const M66592_PIPE3CTR: c_uint = 0x74;
pub const M66592_PIPE4CTR: c_uint = 0x76;
pub const M66592_PIPE5CTR: c_uint = 0x78;
pub const M66592_PIPE6CTR: c_uint = 0x7A;
pub const M66592_PIPE7CTR: c_uint = 0x7C;
pub const M66592_BSTS: c_uint = 0x8000	/* b15: Buffer status */;
pub const M66592_INBUFM: c_uint = 0x4000	/* b14: IN buffer monitor (PIPE 1-5) */;
pub const M66592_ACLRM: c_uint = 0x0200	/* b9: Out buffer auto clear mode */;
pub const M66592_SQCLR: c_uint = 0x0100	/* b8: Sequence toggle bit clear */;
pub const M66592_SQSET: c_uint = 0x0080	/* b7: Sequence toggle bit set */;
pub const M66592_SQMON: c_uint = 0x0040	/* b6: Sequence toggle bit monitor */;
pub const M66592_PID: c_uint = 0x0003	/* b1-0: Response PID */;
pub const M66592_INVALID_REG: c_uint = 0x7E;

pub const M66592_MAX_SAMPLING: c_int = 10;
pub const M66592_MAX_NUM_PIPE: c_int = 8;
pub const M66592_MAX_NUM_BULK: c_int = 3;
pub const M66592_MAX_NUM_ISOC: c_int = 2;
pub const M66592_MAX_NUM_INT: c_int = 2;
pub const M66592_BASE_PIPENUM_BULK: c_int = 3;
pub const M66592_BASE_PIPENUM_ISOC: c_int = 1;
pub const M66592_BASE_PIPENUM_INT: c_int = 6;
pub const M66592_BASE_BUFNUM: c_int = 6;
pub const M66592_MAX_BUFNUM: c_uint = 0x4F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct m66592_pipe_info {
    pub pipe: u16,
    pub epnum: u16,
    pub maxpacket: u16,
    pub type: u16,
    pub interval: u16,
    pub dir_in: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m66592_request {
    pub req: usb_request,
    pub queue: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m66592_ep {
    pub ep: usb_ep,
    pub m66592: *mut m66592,
    pub queue: list_head,
    pub busy:1: unsigned,
    pub /: *mut *mut unsigned internal_ccpl:1; / use only control,
// this member can able to after m66592_enable
    pub use_dma:1: unsigned,
    pub pipenum: u16,
    pub type: u16,
// register address
    pub fifoaddr: c_ulong,
    pub fifosel: c_ulong,
    pub fifoctr: c_ulong,
    pub fifotrn: c_ulong,
    pub pipectr: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct m66592 {
    pub lock: spinlock_t,
    pub reg: *mut void __iomem,
    pub clk: *mut clk,
    pub pdata: *mut m66592_platdata,
    pub irq_trigger: c_ulong,
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub ep: [m66592_ep; M66592_MAX_NUM_PIPE],
    pub pipenum2ep: [*mut m66592_ep; M66592_MAX_NUM_PIPE],
    pub epaddr2ep: [*mut m66592_ep; 16],
    pub /: *mut *mut *mut usb_request ep0_req; / for internal request,
    pub /: *mut *mut __le16 ep0_data; / for internal request,
    pub old_vbus: u16,
    pub timer: timer_list,
    pub scount: c_int,
    pub old_dvsq: c_int,
// pipe config
    pub bulk: c_int,
    pub interrupt: c_int,
    pub isochronous: c_int,
    pub num_dma: c_int,
}

// -------------------------------------------------------------------------
extern "C" {
    pub fn ioread16(offset: m66592->reg +) -> return;
}

