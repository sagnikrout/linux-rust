//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arcmsr/arcmsr.h
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


//
// O.S   : Linux
// FILE NAME  : arcmsr.h
// BY    : Nick Cheng
// Description: SCSI RAID Device Driver for
// ARECA RAID Host adapter
//
// Copyright (C) 2002 - 2005, Areca Technology Corporation All rights reserved.
//
// Web site: www.areca.com.tw
// E-mail: support@areca.com.tw
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
// IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
// OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
// INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES(INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION)HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE)ARISING IN ANY WAY OUT OF THE USE OF
// THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//

// The limit of outstanding scsi command that firmware can handle

pub const ARCMSR_MAX_FREECCB_NUM: c_int = 1024;
pub const ARCMSR_MAX_OUTSTANDING_CMD: c_int = 1024;
pub const ARCMSR_DEFAULT_OUTSTANDING_CMD: c_int = 128;
pub const ARCMSR_MIN_OUTSTANDING_CMD: c_int = 32;

pub const ARCMSR_SCSI_INITIATOR_ID: c_int = 255;
pub const ARCMSR_MAX_XFER_SECTORS: c_int = 512;
pub const ARCMSR_MAX_XFER_SECTORS_B: c_int = 4096;
pub const ARCMSR_MAX_XFER_SECTORS_C: c_int = 304;
pub const ARCMSR_MAX_TARGETID: c_int = 17;
pub const ARCMSR_MAX_TARGETLUN: c_int = 8;
pub const ARCMSR_MAX_CMD_PERLUN: c_int = 128;
pub const ARCMSR_DEFAULT_CMD_PERLUN: c_int = 32;
pub const ARCMSR_MIN_CMD_PERLUN: c_int = 1;
pub const ARCMSR_MAX_QBUFFER: c_int = 4096;
pub const ARCMSR_DEFAULT_SG_ENTRIES: c_int = 38;
pub const ARCMSR_MAX_HBB_POSTQUEUE: c_int = 264;
pub const ARCMSR_MAX_ARC1214_POSTQUEUE: c_int = 256;
pub const ARCMSR_MAX_ARC1214_DONEQUEUE: c_int = 257;
pub const ARCMSR_MAX_HBE_DONEQUEUE: c_int = 512;
pub const ARCMSR_MAX_XFER_LEN: c_uint = 0x26000 /* 152K */;
pub const ARCMSR_CDB_SG_PAGE_LENGTH: c_int = 256;
pub const ARCMST_NUM_MSIX_VECTORS: c_int = 4;

pub const PCI_DEVICE_ID_ARECA_1880: c_uint = 0x1880;

pub const PCI_DEVICE_ID_ARECA_1214: c_uint = 0x1214;

pub const PCI_DEVICE_ID_ARECA_1203: c_uint = 0x1203;

pub const PCI_DEVICE_ID_ARECA_1883: c_uint = 0x1883;

pub const PCI_DEVICE_ID_ARECA_1884: c_uint = 0x1884;

pub const PCI_DEVICE_ID_ARECA_1886_0: c_uint = 0x1886;
pub const PCI_DEVICE_ID_ARECA_1886: c_uint = 0x188A;

pub const ARCMSR_DEFAULT_TIMEOUT: c_int = 90;
//
pub const ARC_SUCCESS: c_int = 0;
pub const ARC_FAILURE: c_int = 1;
//
// split 64bits dma addressing
//

//
// MESSAGE CONTROL CODE
//
// IOP Message Transfer Data for user space
//
pub const ARCMSR_API_DATA_BUFLEN: c_int = 1032;
// IOP message transfer
pub const ARCMSR_MESSAGE_FAIL: c_uint = 0x0001;
// DeviceType
pub const ARECA_SATA_RAID: c_uint = 0x90000000;
// FunctionCode
pub const FUNCTION_READ_RQBUFFER: c_uint = 0x0801;
pub const FUNCTION_WRITE_WQBUFFER: c_uint = 0x0802;
pub const FUNCTION_CLEAR_RQBUFFER: c_uint = 0x0803;
pub const FUNCTION_CLEAR_WQBUFFER: c_uint = 0x0804;
pub const FUNCTION_CLEAR_ALLQBUFFER: c_uint = 0x0805;
pub const FUNCTION_RETURN_CODE_3F: c_uint = 0x0806;
pub const FUNCTION_SAY_HELLO: c_uint = 0x0807;
pub const FUNCTION_SAY_GOODBYE: c_uint = 0x0808;
pub const FUNCTION_FLUSH_ADAPTER_CACHE: c_uint = 0x0809;
pub const FUNCTION_GET_FIRMWARE_STATUS: c_uint = 0x080A;
pub const FUNCTION_HARDWARE_RESET: c_uint = 0x080B;
// ARECA IO CONTROL CODE

// ARECA IOCTL ReturnCode
pub const ARCMSR_MESSAGE_RETURNCODE_OK: c_uint = 0x00000001;
pub const ARCMSR_MESSAGE_RETURNCODE_ERROR: c_uint = 0x00000006;
pub const ARCMSR_MESSAGE_RETURNCODE_3F: c_uint = 0x0000003F;
pub const ARCMSR_MESSAGE_RETURNCODE_BUS_HANG_ON: c_uint = 0x00000088;
//
// structure for holding DMA address data
//

pub const IS_SG64_ADDR: c_uint = 0x01000000 /* bit24 */;
//
// Q Buffer of IOP Message Transfer
//
// FIRMWARE INFO for Intel IOP R 80331 processor (Type A)
//
// signature of set and get firmware config
pub const ARCMSR_SIGNATURE_GET_CONFIG: c_uint = 0x87974060;
pub const ARCMSR_SIGNATURE_SET_CONFIG: c_uint = 0x87974063;
// message code of inbound message register
pub const ARCMSR_INBOUND_MESG0_NOP: c_uint = 0x00000000;
pub const ARCMSR_INBOUND_MESG0_GET_CONFIG: c_uint = 0x00000001;
pub const ARCMSR_INBOUND_MESG0_SET_CONFIG: c_uint = 0x00000002;
pub const ARCMSR_INBOUND_MESG0_ABORT_CMD: c_uint = 0x00000003;
pub const ARCMSR_INBOUND_MESG0_STOP_BGRB: c_uint = 0x00000004;
pub const ARCMSR_INBOUND_MESG0_FLUSH_CACHE: c_uint = 0x00000005;
pub const ARCMSR_INBOUND_MESG0_START_BGRB: c_uint = 0x00000006;
pub const ARCMSR_INBOUND_MESG0_CHK331PENDING: c_uint = 0x00000007;
pub const ARCMSR_INBOUND_MESG0_SYNC_TIMER: c_uint = 0x00000008;
// doorbell interrupt generator
pub const ARCMSR_INBOUND_DRIVER_DATA_WRITE_OK: c_uint = 0x00000001;
pub const ARCMSR_INBOUND_DRIVER_DATA_READ_OK: c_uint = 0x00000002;
pub const ARCMSR_OUTBOUND_IOP331_DATA_WRITE_OK: c_uint = 0x00000001;
pub const ARCMSR_OUTBOUND_IOP331_DATA_READ_OK: c_uint = 0x00000002;
// ccb areca cdb flag
pub const ARCMSR_CCBPOST_FLAG_SGL_BSIZE: c_uint = 0x80000000;
pub const ARCMSR_CCBPOST_FLAG_IAM_BIOS: c_uint = 0x40000000;
pub const ARCMSR_CCBREPLY_FLAG_IAM_BIOS: c_uint = 0x40000000;
pub const ARCMSR_CCBREPLY_FLAG_ERROR_MODE0: c_uint = 0x10000000;
pub const ARCMSR_CCBREPLY_FLAG_ERROR_MODE1: c_uint = 0x00000001;
// outbound firmware ok
pub const ARCMSR_OUTBOUND_MESG1_FIRMWARE_OK: c_uint = 0x80000000;
// ARC-1680 Bus Reset
pub const ARCMSR_ARC1680_BUS_RESET: c_uint = 0x00000003;
// ARC-1880 Bus Reset
pub const ARCMSR_ARC1880_RESET_ADAPTER: c_uint = 0x00000024;
pub const ARCMSR_ARC1880_DiagWrite_ENABLE: c_uint = 0x00000080;
//
// SPEC. for Areca Type B adapter
//
// ARECA HBB COMMAND for its FIRMWARE
// window of "instruction flags" from driver to iop
pub const ARCMSR_DRV2IOP_DOORBELL: c_uint = 0x00020400;
pub const ARCMSR_DRV2IOP_DOORBELL_MASK: c_uint = 0x00020404;
// window of "instruction flags" from iop to driver
pub const ARCMSR_IOP2DRV_DOORBELL: c_uint = 0x00020408;
pub const ARCMSR_IOP2DRV_DOORBELL_MASK: c_uint = 0x0002040C;
// window of "instruction flags" from iop to driver
pub const ARCMSR_IOP2DRV_DOORBELL_1203: c_uint = 0x00021870;
pub const ARCMSR_IOP2DRV_DOORBELL_MASK_1203: c_uint = 0x00021874;
// window of "instruction flags" from driver to iop
pub const ARCMSR_DRV2IOP_DOORBELL_1203: c_uint = 0x00021878;
pub const ARCMSR_DRV2IOP_DOORBELL_MASK_1203: c_uint = 0x0002187C;
// ARECA FLAG LANGUAGE
// ioctl transfer
pub const ARCMSR_IOP2DRV_DATA_WRITE_OK: c_uint = 0x00000001;
// ioctl transfer
pub const ARCMSR_IOP2DRV_DATA_READ_OK: c_uint = 0x00000002;
pub const ARCMSR_IOP2DRV_CDB_DONE: c_uint = 0x00000004;
pub const ARCMSR_IOP2DRV_MESSAGE_CMD_DONE: c_uint = 0x00000008;
pub const ARCMSR_DOORBELL_HANDLE_INT: c_uint = 0x0000000F;
pub const ARCMSR_DOORBELL_INT_CLEAR_PATTERN: c_uint = 0xFF00FFF0;
pub const ARCMSR_MESSAGE_INT_CLEAR_PATTERN: c_uint = 0xFF00FFF7;
// (ARCMSR_INBOUND_MESG0_GET_CONFIG<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_GET_CONFIG: c_uint = 0x00010008;
// (ARCMSR_INBOUND_MESG0_SET_CONFIG<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_SET_CONFIG: c_uint = 0x00020008;
// (ARCMSR_INBOUND_MESG0_ABORT_CMD<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_ABORT_CMD: c_uint = 0x00030008;
// (ARCMSR_INBOUND_MESG0_STOP_BGRB<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_STOP_BGRB: c_uint = 0x00040008;
// (ARCMSR_INBOUND_MESG0_FLUSH_CACHE<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_FLUSH_CACHE: c_uint = 0x00050008;
// (ARCMSR_INBOUND_MESG0_START_BGRB<<16)|ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED)
pub const ARCMSR_MESSAGE_START_BGRB: c_uint = 0x00060008;
pub const ARCMSR_MESSAGE_SYNC_TIMER: c_uint = 0x00080008;
pub const ARCMSR_MESSAGE_START_DRIVER_MODE: c_uint = 0x000E0008;
pub const ARCMSR_MESSAGE_SET_POST_WINDOW: c_uint = 0x000F0008;
pub const ARCMSR_MESSAGE_ACTIVE_EOI_MODE: c_uint = 0x00100008;
// ARCMSR_OUTBOUND_MESG1_FIRMWARE_OK
pub const ARCMSR_MESSAGE_FIRMWARE_OK: c_uint = 0x80000000;
// ioctl transfer
pub const ARCMSR_DRV2IOP_DATA_WRITE_OK: c_uint = 0x00000001;
// ioctl transfer
pub const ARCMSR_DRV2IOP_DATA_READ_OK: c_uint = 0x00000002;
pub const ARCMSR_DRV2IOP_CDB_POSTED: c_uint = 0x00000004;
pub const ARCMSR_DRV2IOP_MESSAGE_CMD_POSTED: c_uint = 0x00000008;
pub const ARCMSR_DRV2IOP_END_OF_INTERRUPT: c_uint = 0x00000010;
// data tunnel buffer between user space program and its firmware
// user space data to iop 128bytes
pub const ARCMSR_MESSAGE_WBUFFER: c_uint = 0x0000fe00;
// iop data to user space 128bytes
pub const ARCMSR_MESSAGE_RBUFFER: c_uint = 0x0000ff00;
// iop message_rwbuffer for message command
pub const ARCMSR_MESSAGE_RWBUFFER: c_uint = 0x0000fa00;

//
// SPEC. for Areca HBC adapter
//
pub const ARCMSR_HBC_ISR_THROTTLING_LEVEL: c_int = 12;
pub const ARCMSR_HBC_ISR_MAX_DONE_QUEUE: c_int = 20;
// Host Interrupt Mask
pub const ARCMSR_HBCMU_UTILITY_A_ISR_MASK: c_uint = 0x00000001 /* When clear, the Utility_A interrupt routes to the host.*/;
pub const ARCMSR_HBCMU_OUTBOUND_DOORBELL_ISR_MASK: c_uint = 0x00000004 /* When clear, the General Outbound Doorbell interrupt routes to the host.*/;
pub const ARCMSR_HBCMU_OUTBOUND_POSTQUEUE_ISR_MASK: c_uint = 0x00000008 /* When clear, the Outbound Post List FIFO Not Empty interrupt routes to the host.*/;
pub const ARCMSR_HBCMU_ALL_INTMASKENABLE: c_uint = 0x0000000D /* disable all ISR */;
// Host Interrupt Status
pub const ARCMSR_HBCMU_UTILITY_A_ISR: c_uint = 0x00000001;
//
// Set when the Utility_A Interrupt bit is set in the Outbound Doorbell Register.
// It clears by writing a 1 to the Utility_A bit in the Outbound Doorbell Clear Register or through automatic clearing (if enabled).
//
pub const ARCMSR_HBCMU_OUTBOUND_DOORBELL_ISR: c_uint = 0x00000004;
//
// Set if Outbound Doorbell register bits 30:1 have a non-zero
// value. This bit clears only when Outbound Doorbell bits
// 30:1 are ALL clear. Only a write to the Outbound Doorbell
// Clear register clears bits in the Outbound Doorbell register.
//
pub const ARCMSR_HBCMU_OUTBOUND_POSTQUEUE_ISR: c_uint = 0x00000008;
//
// Set whenever the Outbound Post List Producer/Consumer
// Register (FIFO) is not empty. It clears when the Outbound
// Post List FIFO is empty.
//
pub const ARCMSR_HBCMU_SAS_ALL_INT: c_uint = 0x00000010;
//
// This bit indicates a SAS interrupt from a source external to
// the PCIe core. This bit is not maskable.
//
// DoorBell
pub const ARCMSR_HBCMU_DRV2IOP_DATA_WRITE_OK: c_uint = 0x00000002;
pub const ARCMSR_HBCMU_DRV2IOP_DATA_READ_OK: c_uint = 0x00000004;
// inbound message 0 ready
pub const ARCMSR_HBCMU_DRV2IOP_MESSAGE_CMD_DONE: c_uint = 0x00000008;
// more than 12 request completed in a time
pub const ARCMSR_HBCMU_DRV2IOP_POSTQUEUE_THROTTLING: c_uint = 0x00000010;
pub const ARCMSR_HBCMU_IOP2DRV_DATA_WRITE_OK: c_uint = 0x00000002;
// outbound DATA WRITE isr door bell clear
pub const ARCMSR_HBCMU_IOP2DRV_DATA_WRITE_DOORBELL_CLEAR: c_uint = 0x00000002;
pub const ARCMSR_HBCMU_IOP2DRV_DATA_READ_OK: c_uint = 0x00000004;
// outbound DATA READ isr door bell clear
pub const ARCMSR_HBCMU_IOP2DRV_DATA_READ_DOORBELL_CLEAR: c_uint = 0x00000004;
// outbound message 0 ready
pub const ARCMSR_HBCMU_IOP2DRV_MESSAGE_CMD_DONE: c_uint = 0x00000008;
// outbound message cmd isr door bell clear
pub const ARCMSR_HBCMU_IOP2DRV_MESSAGE_CMD_DONE_DOORBELL_CLEAR: c_uint = 0x00000008;
// ARCMSR_HBAMU_MESSAGE_FIRMWARE_OK
pub const ARCMSR_HBCMU_MESSAGE_FIRMWARE_OK: c_uint = 0x80000000;
//
// SPEC. for Areca Type D adapter
//
pub const ARCMSR_ARC1214_CHIP_ID: c_uint = 0x00004;
pub const ARCMSR_ARC1214_CPU_MEMORY_CONFIGURATION: c_uint = 0x00008;
pub const ARCMSR_ARC1214_I2_HOST_INTERRUPT_MASK: c_uint = 0x00034;
pub const ARCMSR_ARC1214_SAMPLE_RESET: c_uint = 0x00100;
pub const ARCMSR_ARC1214_RESET_REQUEST: c_uint = 0x00108;
pub const ARCMSR_ARC1214_MAIN_INTERRUPT_STATUS: c_uint = 0x00200;
pub const ARCMSR_ARC1214_PCIE_F0_INTERRUPT_ENABLE: c_uint = 0x0020C;
pub const ARCMSR_ARC1214_INBOUND_MESSAGE0: c_uint = 0x00400;
pub const ARCMSR_ARC1214_INBOUND_MESSAGE1: c_uint = 0x00404;
pub const ARCMSR_ARC1214_OUTBOUND_MESSAGE0: c_uint = 0x00420;
pub const ARCMSR_ARC1214_OUTBOUND_MESSAGE1: c_uint = 0x00424;
pub const ARCMSR_ARC1214_INBOUND_DOORBELL: c_uint = 0x00460;
pub const ARCMSR_ARC1214_OUTBOUND_DOORBELL: c_uint = 0x00480;
pub const ARCMSR_ARC1214_OUTBOUND_DOORBELL_ENABLE: c_uint = 0x00484;
pub const ARCMSR_ARC1214_INBOUND_LIST_BASE_LOW: c_uint = 0x01000;
pub const ARCMSR_ARC1214_INBOUND_LIST_BASE_HIGH: c_uint = 0x01004;
pub const ARCMSR_ARC1214_INBOUND_LIST_WRITE_POINTER: c_uint = 0x01018;
pub const ARCMSR_ARC1214_OUTBOUND_LIST_BASE_LOW: c_uint = 0x01060;
pub const ARCMSR_ARC1214_OUTBOUND_LIST_BASE_HIGH: c_uint = 0x01064;
pub const ARCMSR_ARC1214_OUTBOUND_LIST_COPY_POINTER: c_uint = 0x0106C;
pub const ARCMSR_ARC1214_OUTBOUND_LIST_READ_POINTER: c_uint = 0x01070;
pub const ARCMSR_ARC1214_OUTBOUND_INTERRUPT_CAUSE: c_uint = 0x01088;
pub const ARCMSR_ARC1214_OUTBOUND_INTERRUPT_ENABLE: c_uint = 0x0108C;
pub const ARCMSR_ARC1214_MESSAGE_WBUFFER: c_uint = 0x02000;
pub const ARCMSR_ARC1214_MESSAGE_RBUFFER: c_uint = 0x02100;
pub const ARCMSR_ARC1214_MESSAGE_RWBUFFER: c_uint = 0x02200;
// Host Interrupt Mask
pub const ARCMSR_ARC1214_ALL_INT_ENABLE: c_uint = 0x00001010;
pub const ARCMSR_ARC1214_ALL_INT_DISABLE: c_uint = 0x00000000;
// Host Interrupt Status
pub const ARCMSR_ARC1214_OUTBOUND_DOORBELL_ISR: c_uint = 0x00001000;
pub const ARCMSR_ARC1214_OUTBOUND_POSTQUEUE_ISR: c_uint = 0x00000010;
// DoorBell
pub const ARCMSR_ARC1214_DRV2IOP_DATA_IN_READY: c_uint = 0x00000001;
pub const ARCMSR_ARC1214_DRV2IOP_DATA_OUT_READ: c_uint = 0x00000002;
// inbound message 0 ready
pub const ARCMSR_ARC1214_IOP2DRV_DATA_WRITE_OK: c_uint = 0x00000001;
// outbound DATA WRITE isr door bell clear
pub const ARCMSR_ARC1214_IOP2DRV_DATA_READ_OK: c_uint = 0x00000002;
// outbound message 0 ready
pub const ARCMSR_ARC1214_IOP2DRV_MESSAGE_CMD_DONE: c_uint = 0x02000000;
// outbound message cmd isr door bell clear
// ARCMSR_HBAMU_MESSAGE_FIRMWARE_OK
pub const ARCMSR_ARC1214_MESSAGE_FIRMWARE_OK: c_uint = 0x80000000;
pub const ARCMSR_ARC1214_OUTBOUND_LIST_INTERRUPT_CLEAR: c_uint = 0x00000001;
//
// SPEC. for Areca Type E adapter
//
pub const ARCMSR_SIGNATURE_1884: c_uint = 0x188417D3;
pub const ARCMSR_HBEMU_DRV2IOP_DATA_WRITE_OK: c_uint = 0x00000002;
pub const ARCMSR_HBEMU_DRV2IOP_DATA_READ_OK: c_uint = 0x00000004;
pub const ARCMSR_HBEMU_DRV2IOP_MESSAGE_CMD_DONE: c_uint = 0x00000008;
pub const ARCMSR_HBEMU_IOP2DRV_DATA_WRITE_OK: c_uint = 0x00000002;
pub const ARCMSR_HBEMU_IOP2DRV_DATA_READ_OK: c_uint = 0x00000004;
pub const ARCMSR_HBEMU_IOP2DRV_MESSAGE_CMD_DONE: c_uint = 0x00000008;
pub const ARCMSR_HBEMU_MESSAGE_FIRMWARE_OK: c_uint = 0x80000000;
pub const ARCMSR_HBEMU_OUTBOUND_DOORBELL_ISR: c_uint = 0x00000001;
pub const ARCMSR_HBEMU_OUTBOUND_POSTQUEUE_ISR: c_uint = 0x00000008;
pub const ARCMSR_HBEMU_ALL_INTMASKENABLE: c_uint = 0x00000009;
// ARC-1884 doorbell sync
pub const ARCMSR_HBEMU_DOORBELL_SYNC: c_uint = 0x100;
pub const ARCMSR_ARC188X_RESET_ADAPTER: c_uint = 0x00000004;
pub const ARCMSR_ARC1884_DiagWrite_ENABLE: c_uint = 0x00000080;
//
// SPEC. for Areca Type F adapter
//
pub const ARCMSR_SIGNATURE_1886: c_uint = 0x188617D3;
// Doorbell and interrupt definition are same as Type E adapter
// ARC-1886 doorbell sync
pub const ARCMSR_HBFMU_DOORBELL_SYNC: c_uint = 0x100;
// set host rw buffer physical address at inbound message 0, 1 (low,high)
pub const ARCMSR_HBFMU_DOORBELL_SYNC1: c_uint = 0x300;
pub const ARCMSR_HBFMU_MESSAGE_FIRMWARE_OK: c_uint = 0x80000000;
pub const ARCMSR_HBFMU_MESSAGE_NO_VOLUME_CHANGE: c_uint = 0x20000000;
//
// ARECA SCSI COMMAND DESCRIPTOR BLOCK size 0x1F8 (504)
//
pub const ARCMSR_CDB_FLAG_SGL_BSIZE: c_uint = 0x01;
pub const ARCMSR_CDB_FLAG_BIOS: c_uint = 0x02;
pub const ARCMSR_CDB_FLAG_WRITE: c_uint = 0x04;
pub const ARCMSR_CDB_FLAG_SIMPLEQ: c_uint = 0x00;
pub const ARCMSR_CDB_FLAG_HEADQ: c_uint = 0x08;
pub const ARCMSR_CDB_FLAG_ORDEREDQ: c_uint = 0x10;
pub const ARCMSR_DEV_CHECK_CONDITION: c_uint = 0x02;
pub const ARCMSR_DEV_SELECT_TIMEOUT: c_uint = 0xF0;
pub const ARCMSR_DEV_ABORTED: c_uint = 0xF1;
pub const ARCMSR_DEV_INIT_FAIL: c_uint = 0xF2;
//
// Messaging Unit (MU) of the Intel R 80331 I/O processor(Type A) and Type B processor
//
// LSI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MessageUnit_C {
    pub 0003*/: *mut *mut uint32_t message_unit_status; /0000,
    pub 0007*/: *mut *mut uint32_t slave_error_attribute; /0004,
    pub 000B*/: *mut *mut uint32_t slave_error_address; /0008,
    pub 000F*/: *mut *mut uint32_t posted_outbound_doorbell; /000C,
    pub 0013*/: *mut *mut uint32_t master_error_attribute; /0010,
    pub 0017*/: *mut *mut uint32_t master_error_address_low; /0014,
    pub 001B*/: *mut *mut uint32_t master_error_address_high; /0018,
    pub 001F*/: *mut *mut uint32_t hcb_size; /001C,
    pub 0023*/: *mut *mut uint32_t inbound_doorbell; /0020,
    pub 0027*/: *mut *mut uint32_t diagnostic_rw_data; /0024,
    pub 002B*/: *mut *mut uint32_t diagnostic_rw_address_low; /0028,
    pub 002F*/: *mut *mut uint32_t diagnostic_rw_address_high; /002C,
    pub 0033*/: *mut *mut uint32_t host_int_status; /0030,
    pub 0037*/: *mut *mut uint32_t host_int_mask; /0034,
    pub 003B*/: *mut *mut uint32_t dcr_data; /0038,
    pub 003F*/: *mut *mut uint32_t dcr_address; /003C,
    pub 0043*/: *mut *mut uint32_t inbound_queueport; /0040,
    pub 0047*/: *mut *mut uint32_t outbound_queueport; /0044,
    pub 004B*/: *mut *mut uint32_t hcb_pci_address_low; /0048,
    pub 004F*/: *mut *mut uint32_t hcb_pci_address_high; /004C,
    pub 0053*/: *mut *mut uint32_t iop_int_status; /0050,
    pub 0057*/: *mut *mut uint32_t iop_int_mask; /0054,
    pub 005B*/: *mut *mut uint32_t iop_inbound_queue_port; /0058,
    pub 005F*/: *mut *mut uint32_t iop_outbound_queue_port; /005C,
    pub 0063*/: *mut *mut uint32_t inbound_free_list_index; /0060,
    pub 0067*/: *mut *mut uint32_t inbound_post_list_index; /0064,
    pub 006B*/: *mut *mut uint32_t outbound_free_list_index; /0068,
    pub 006F*/: *mut *mut uint32_t outbound_post_list_index; /006C,
    pub 0073*/: *mut *mut uint32_t inbound_doorbell_clear; /0070,
    pub 0077*/: *mut *mut uint32_t i2o_message_unit_control; /0074,
    pub 007B*/: *mut *mut uint32_t last_used_message_source_address_low; /0078,
    pub 007F*/: *mut *mut uint32_t last_used_message_source_address_high; /007C,
    pub 008F*/: *mut *mut uint32_t pull_mode_data_byte_count[4]; /0080,
    pub 0093*/: *mut *mut uint32_t message_dest_address_index; /0090,
    pub 0097*/: *mut *mut uint32_t done_queue_not_empty_int_counter_timer; /0094,
    pub 009B*/: *mut *mut uint32_t utility_A_int_counter_timer; /0098,
    pub 009F*/: *mut *mut uint32_t outbound_doorbell; /009C,
    pub 00A3*/: *mut *mut uint32_t outbound_doorbell_clear; /00A0,
    pub 00A7*/: *mut *mut uint32_t message_source_address_index; /00A4,
    pub 00AB*/: *mut *mut uint32_t message_done_queue_index; /00A8,
    pub 00AF*/: *mut *mut uint32_t reserved0; /00AC,
    pub 00B3*/: *mut *mut uint32_t inbound_msgaddr0; /00B0,
    pub 00B7*/: *mut *mut uint32_t inbound_msgaddr1; /00B4,
    pub 00BB*/: *mut *mut uint32_t outbound_msgaddr0; /00B8,
    pub 00BF*/: *mut *mut uint32_t outbound_msgaddr1; /00BC,
    pub 00C3*/: *mut *mut uint32_t inbound_queueport_low; /00C0,
    pub 00C7*/: *mut *mut uint32_t inbound_queueport_high; /00C4,
    pub 00CB*/: *mut *mut uint32_t outbound_queueport_low; /00C8,
    pub 00CF*/: *mut *mut uint32_t outbound_queueport_high; /00CC,
    pub 00D3*/: *mut *mut uint32_t iop_inbound_queue_port_low; /00D0,
    pub 00D7*/: *mut *mut uint32_t iop_inbound_queue_port_high; /00D4,
    pub 00DB*/: *mut *mut uint32_t iop_outbound_queue_port_low; /00D8,
    pub 00DF*/: *mut *mut uint32_t iop_outbound_queue_port_high; /00DC,
    pub 00E3*/: *mut *mut uint32_t message_dest_queue_port_low; /00E0,
    pub 00E7*/: *mut *mut uint32_t message_dest_queue_port_high; /00E4,
    pub 00EB*/: *mut *mut uint32_t last_used_message_dest_address_low; /00E8,
    pub 00EF*/: *mut *mut uint32_t last_used_message_dest_address_high; /00EC,
    pub 00F3*/: *mut *mut uint32_t message_done_queue_base_address_low; /00F0,
    pub 00F7*/: *mut *mut uint32_t message_done_queue_base_address_high; /00F4,
    pub 00FB*/: *mut *mut uint32_t host_diagnostic; /00F8,
    pub 00FF*/: *mut *mut uint32_t write_sequence; /00FC,
    pub 0187*/: *mut *mut uint32_t reserved1[34]; /0100,
    pub 1FFF*/: *mut *mut uint32_t reserved2[1950]; /0188,
    pub 207F*/: *mut *mut uint32_t message_wbuffer[32]; /2000,
    pub 20FF*/: *mut *mut uint32_t reserved3[32]; /2080,
    pub 217F*/: *mut *mut uint32_t message_rbuffer[32]; /2100,
    pub 21FF*/: *mut *mut uint32_t reserved4[32]; /2180,
    pub 23FF*/: *mut *mut uint32_t msgcode_rwbuffer[256]; /2200,
}

//
// Messaging Unit (MU) of Type D processor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InBound_SRB {
    pub /: *mut *mut uint32_t addressLow; / pointer to SRB block,
    pub addressHigh: u32,
    pub /: *mut *mut uint32_t length; / in DWORDs,
    pub reserved0: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OutBound_SRB {
    pub /: *mut *mut uint32_t addressLow; / pointer to SRB block,
    pub addressHigh: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MessageUnit_D {
    pub post_qbuffer: [InBound_SRB; ARCMSR_MAX_ARC1214_POSTQUEUE],
    pub postq_index: u16,
    pub doneq_index: volatile u16,
    pub /: *mut *mut *mut u32 __iomem chip_id; / 0x00004,
    pub /: *mut *mut *mut u32 __iomem cpu_mem_config; / 0x00008,
    pub /: *mut *mut *mut u32 __iomem i2o_host_interrupt_mask; / 0x00034,
    pub /: *mut *mut *mut u32 __iomem sample_at_reset; / 0x00100,
    pub /: *mut *mut *mut u32 __iomem reset_request; / 0x00108,
    pub /: *mut *mut *mut u32 __iomem host_int_status; / 0x00200,
    pub /: *mut *mut *mut u32 __iomem pcief0_int_enable; / 0x0020C,
    pub /: *mut *mut *mut u32 __iomem inbound_msgaddr0; / 0x00400,
    pub /: *mut *mut *mut u32 __iomem inbound_msgaddr1; / 0x00404,
    pub /: *mut *mut *mut u32 __iomem outbound_msgaddr0; / 0x00420,
    pub /: *mut *mut *mut u32 __iomem outbound_msgaddr1; / 0x00424,
    pub /: *mut *mut *mut u32 __iomem inbound_doorbell; / 0x00460,
    pub /: *mut *mut *mut u32 __iomem outbound_doorbell; / 0x00480,
    pub /: *mut *mut *mut u32 __iomem outbound_doorbell_enable; / 0x00484,
    pub /: *mut *mut *mut u32 __iomem inboundlist_base_low; / 0x01000,
    pub /: *mut *mut *mut u32 __iomem inboundlist_base_high; / 0x01004,
    pub /: *mut *mut *mut u32 __iomem inboundlist_write_pointer; / 0x01018,
    pub /: *mut *mut *mut u32 __iomem outboundlist_base_low; / 0x01060,
    pub /: *mut *mut *mut u32 __iomem outboundlist_base_high; / 0x01064,
    pub /: *mut *mut *mut u32 __iomem outboundlist_copy_pointer; / 0x0106C,
    pub /: *mut *mut *mut u32 __iomem outboundlist_read_pointer; / 0x01070 0x01072,
    pub /: *mut *mut *mut u32 __iomem outboundlist_interrupt_cause; / 0x1088,
    pub /: *mut *mut *mut u32 __iomem outboundlist_interrupt_enable; / 0x108C,
    pub /: *mut *mut *mut u32 __iomem message_wbuffer; / 0x2000,
    pub /: *mut *mut *mut u32 __iomem message_rbuffer; / 0x2100,
    pub /: *mut *mut *mut u32 __iomem msgcode_rwbuffer; / 0x2200,
}

//
// Messaging Unit (MU) of Type E processor(LSI)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MessageUnit_E {
    pub 0003*/: *mut *mut uint32_t iobound_doorbell; /0000,
    pub 0007*/: *mut *mut uint32_t write_sequence_3xxx; /0004,
    pub 000B*/: *mut *mut uint32_t host_diagnostic_3xxx; /0008,
    pub 000F*/: *mut *mut uint32_t posted_outbound_doorbell; /000C,
    pub 0013*/: *mut *mut uint32_t master_error_attribute; /0010,
    pub 0017*/: *mut *mut uint32_t master_error_address_low; /0014,
    pub 001B*/: *mut *mut uint32_t master_error_address_high; /0018,
    pub 001F*/: *mut *mut uint32_t hcb_size; /001C,
    pub 0023*/: *mut *mut uint32_t inbound_doorbell; /0020,
    pub 0027*/: *mut *mut uint32_t diagnostic_rw_data; /0024,
    pub 002B*/: *mut *mut uint32_t diagnostic_rw_address_low; /0028,
    pub 002F*/: *mut *mut uint32_t diagnostic_rw_address_high; /002C,
    pub 0033*/: *mut *mut uint32_t host_int_status; /0030,
    pub 0037*/: *mut *mut uint32_t host_int_mask; /0034,
    pub 003B*/: *mut *mut uint32_t dcr_data; /0038,
    pub 003F*/: *mut *mut uint32_t dcr_address; /003C,
    pub 0043*/: *mut *mut uint32_t inbound_queueport; /0040,
    pub 0047*/: *mut *mut uint32_t outbound_queueport; /0044,
    pub 004B*/: *mut *mut uint32_t hcb_pci_address_low; /0048,
    pub 004F*/: *mut *mut uint32_t hcb_pci_address_high; /004C,
    pub 0053*/: *mut *mut uint32_t iop_int_status; /0050,
    pub 0057*/: *mut *mut uint32_t iop_int_mask; /0054,
    pub 005B*/: *mut *mut uint32_t iop_inbound_queue_port; /0058,
    pub 005F*/: *mut *mut uint32_t iop_outbound_queue_port; /005C,
    pub 0063*/: *mut *mut uint32_t inbound_free_list_index; /0060,
    pub 0067*/: *mut *mut uint32_t inbound_post_list_index; /0064,
    pub 006B*/: *mut *mut uint32_t reply_post_producer_index; /0068,
    pub 006F*/: *mut *mut uint32_t reply_post_consumer_index; /006C,
    pub 0073*/: *mut *mut uint32_t inbound_doorbell_clear; /0070,
    pub 0077*/: *mut *mut uint32_t i2o_message_unit_control; /0074,
    pub 007B*/: *mut *mut uint32_t last_used_message_source_address_low; /0078,
    pub 007F*/: *mut *mut uint32_t last_used_message_source_address_high; /007C,
    pub 008F*/: *mut *mut uint32_t pull_mode_data_byte_count[4]; /0080,
    pub 0093*/: *mut *mut uint32_t message_dest_address_index; /0090,
    pub 0097*/: *mut *mut uint32_t done_queue_not_empty_int_counter_timer; /0094,
    pub 009B*/: *mut *mut uint32_t utility_A_int_counter_timer; /0098,
    pub 009F*/: *mut *mut uint32_t outbound_doorbell; /009C,
    pub 00A3*/: *mut *mut uint32_t outbound_doorbell_clear; /00A0,
    pub 00A7*/: *mut *mut uint32_t message_source_address_index; /00A4,
    pub 00AB*/: *mut *mut uint32_t message_done_queue_index; /00A8,
    pub 00AF*/: *mut *mut uint32_t reserved0; /00AC,
    pub 00B3*/: *mut *mut uint32_t inbound_msgaddr0; /00B0,
    pub 00B7*/: *mut *mut uint32_t inbound_msgaddr1; /00B4,
    pub 00BB*/: *mut *mut uint32_t outbound_msgaddr0; /00B8,
    pub 00BF*/: *mut *mut uint32_t outbound_msgaddr1; /00BC,
    pub 00C3*/: *mut *mut uint32_t inbound_queueport_low; /00C0,
    pub 00C7*/: *mut *mut uint32_t inbound_queueport_high; /00C4,
    pub 00CB*/: *mut *mut uint32_t outbound_queueport_low; /00C8,
    pub 00CF*/: *mut *mut uint32_t outbound_queueport_high; /00CC,
    pub 00D3*/: *mut *mut uint32_t iop_inbound_queue_port_low; /00D0,
    pub 00D7*/: *mut *mut uint32_t iop_inbound_queue_port_high; /00D4,
    pub 00DB*/: *mut *mut uint32_t iop_outbound_queue_port_low; /00D8,
    pub 00DF*/: *mut *mut uint32_t iop_outbound_queue_port_high; /00DC,
    pub 00E3*/: *mut *mut uint32_t message_dest_queue_port_low; /00E0,
    pub 00E7*/: *mut *mut uint32_t message_dest_queue_port_high; /00E4,
    pub 00EB*/: *mut *mut uint32_t last_used_message_dest_address_low; /00E8,
    pub 00EF*/: *mut *mut uint32_t last_used_message_dest_address_high; /00EC,
    pub 00F3*/: *mut *mut uint32_t message_done_queue_base_address_low; /00F0,
    pub 00F7*/: *mut *mut uint32_t message_done_queue_base_address_high; /00F4,
    pub 00FB*/: *mut *mut uint32_t host_diagnostic; /00F8,
    pub 00FF*/: *mut *mut uint32_t write_sequence; /00FC,
    pub 0187*/: *mut *mut uint32_t reserved1[34]; /0100,
    pub 1FFF*/: *mut *mut uint32_t reserved2[1950]; /0188,
    pub 207F*/: *mut *mut uint32_t message_wbuffer[32]; /2000,
    pub 20FF*/: *mut *mut uint32_t reserved3[32]; /2080,
    pub 217F*/: *mut *mut uint32_t message_rbuffer[32]; /2100,
    pub 21FF*/: *mut *mut uint32_t reserved4[32]; /2180,
    pub 23FF*/: *mut *mut uint32_t msgcode_rwbuffer[256]; /2200,
}

//
// Messaging Unit (MU) of Type F processor(LSI)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MessageUnit_F {
    pub 0003*/: *mut *mut uint32_t iobound_doorbell; /0000,
    pub 0007*/: *mut *mut uint32_t write_sequence_3xxx; /0004,
    pub 000B*/: *mut *mut uint32_t host_diagnostic_3xxx; /0008,
    pub 000F*/: *mut *mut uint32_t posted_outbound_doorbell; /000C,
    pub 0013*/: *mut *mut uint32_t master_error_attribute; /0010,
    pub 0017*/: *mut *mut uint32_t master_error_address_low; /0014,
    pub 001B*/: *mut *mut uint32_t master_error_address_high; /0018,
    pub 001F*/: *mut *mut uint32_t hcb_size; /001C,
    pub 0023*/: *mut *mut uint32_t inbound_doorbell; /0020,
    pub 0027*/: *mut *mut uint32_t diagnostic_rw_data; /0024,
    pub 002B*/: *mut *mut uint32_t diagnostic_rw_address_low; /0028,
    pub 002F*/: *mut *mut uint32_t diagnostic_rw_address_high; /002C,
    pub 0033*/: *mut *mut uint32_t host_int_status; /0030,
    pub 0037*/: *mut *mut uint32_t host_int_mask; /0034,
    pub 003B*/: *mut *mut uint32_t dcr_data; /0038,
    pub 003F*/: *mut *mut uint32_t dcr_address; /003C,
    pub 0043*/: *mut *mut uint32_t inbound_queueport; /0040,
    pub 0047*/: *mut *mut uint32_t outbound_queueport; /0044,
    pub 004B*/: *mut *mut uint32_t hcb_pci_address_low; /0048,
    pub 004F*/: *mut *mut uint32_t hcb_pci_address_high; /004C,
    pub 0053*/: *mut *mut uint32_t iop_int_status; /0050,
    pub 0057*/: *mut *mut uint32_t iop_int_mask; /0054,
    pub 005B*/: *mut *mut uint32_t iop_inbound_queue_port; /0058,
    pub 005F*/: *mut *mut uint32_t iop_outbound_queue_port; /005C,
    pub 0063*/: *mut *mut uint32_t inbound_free_list_index; /0060,
    pub 0067*/: *mut *mut uint32_t inbound_post_list_index; /0064,
    pub 006B*/: *mut *mut uint32_t reply_post_producer_index; /0068,
    pub 006F*/: *mut *mut uint32_t reply_post_consumer_index; /006C,
    pub 0073*/: *mut *mut uint32_t inbound_doorbell_clear; /0070,
    pub 0077*/: *mut *mut uint32_t i2o_message_unit_control; /0074,
    pub 007B*/: *mut *mut uint32_t last_used_message_source_address_low; /0078,
    pub 007F*/: *mut *mut uint32_t last_used_message_source_address_high; /007C,
    pub 008F*/: *mut *mut uint32_t pull_mode_data_byte_count[4]; /0080,
    pub 0093*/: *mut *mut uint32_t message_dest_address_index; /0090,
    pub 0097*/: *mut *mut uint32_t done_queue_not_empty_int_counter_timer; /0094,
    pub 009B*/: *mut *mut uint32_t utility_A_int_counter_timer; /0098,
    pub 009F*/: *mut *mut uint32_t outbound_doorbell; /009C,
    pub 00A3*/: *mut *mut uint32_t outbound_doorbell_clear; /00A0,
    pub 00A7*/: *mut *mut uint32_t message_source_address_index; /00A4,
    pub 00AB*/: *mut *mut uint32_t message_done_queue_index; /00A8,
    pub 00AF*/: *mut *mut uint32_t reserved0; /00AC,
    pub 00B3*/: *mut *mut uint32_t inbound_msgaddr0; /00B0,
    pub 00B7*/: *mut *mut uint32_t inbound_msgaddr1; /00B4,
    pub 00BB*/: *mut *mut uint32_t outbound_msgaddr0; /00B8,
    pub 00BF*/: *mut *mut uint32_t outbound_msgaddr1; /00BC,
    pub 00C3*/: *mut *mut uint32_t inbound_queueport_low; /00C0,
    pub 00C7*/: *mut *mut uint32_t inbound_queueport_high; /00C4,
    pub 00CB*/: *mut *mut uint32_t outbound_queueport_low; /00C8,
    pub 00CF*/: *mut *mut uint32_t outbound_queueport_high; /00CC,
    pub 00D3*/: *mut *mut uint32_t iop_inbound_queue_port_low; /00D0,
    pub 00D7*/: *mut *mut uint32_t iop_inbound_queue_port_high; /00D4,
    pub 00DB*/: *mut *mut uint32_t iop_outbound_queue_port_low; /00D8,
    pub 00DF*/: *mut *mut uint32_t iop_outbound_queue_port_high; /00DC,
    pub 00E3*/: *mut *mut uint32_t message_dest_queue_port_low; /00E0,
    pub 00E7*/: *mut *mut uint32_t message_dest_queue_port_high; /00E4,
    pub 00EB*/: *mut *mut uint32_t last_used_message_dest_address_low; /00E8,
    pub 00EF*/: *mut *mut uint32_t last_used_message_dest_address_high; /00EC,
    pub 00F3*/: *mut *mut uint32_t message_done_queue_base_address_low; /00F0,
    pub 00F7*/: *mut *mut uint32_t message_done_queue_base_address_high; /00F4,
    pub 00FB*/: *mut *mut uint32_t host_diagnostic; /00F8,
    pub 00FF*/: *mut *mut uint32_t write_sequence; /00FC,
    pub 01B7*/: *mut *mut uint32_t reserved1[46]; /0100,
    pub 01BB*/: *mut *mut uint32_t reply_post_producer_index1; /01B8,
    pub 01BF*/: *mut *mut uint32_t reply_post_consumer_index1; /01BC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostRamBuf {
    pub "HRBS": uint32_t hrbSignature; // must be,
    pub MB: uint32_t hrbSize; // total sg size, be multiples of,
    pub 0: uint32_t hrbRes[2]; // reserved, must be set to,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Xor_sg {
    pub xorPhys: dma_addr_t,
    pub xorBufLen: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct XorHandle {
    pub xorPhys: dma_addr_t,
    pub xorBufLen: u64,
    pub xorVirt: *mut c_void,
}

//
// Adapter Control Block
//
pub const ACB_ADAPTER_TYPE_A: c_uint = 0x00000000	/* hba I IOP */;
pub const ACB_ADAPTER_TYPE_B: c_uint = 0x00000001	/* hbb M IOP */;
pub const ACB_ADAPTER_TYPE_C: c_uint = 0x00000002	/* hbc L IOP */;
pub const ACB_ADAPTER_TYPE_D: c_uint = 0x00000003	/* hbd M IOP */;
pub const ACB_ADAPTER_TYPE_E: c_uint = 0x00000004	/* hba L IOP */;
pub const ACB_ADAPTER_TYPE_F: c_uint = 0x00000005	/* hba L IOP */;
// Offset is used in making arc cdb physical to virtual calculations
// message unit ATU inbound base address0
// 0x000 - COMPORT_IN  (Host sent to ROC)
// 0x100 - COMPORT_OUT (ROC sent to Host)
pub const ACB_F_SCSISTOPADAPTER: c_uint = 0x0001;
pub const ACB_F_MSG_STOP_BGRB: c_uint = 0x0002;
// stop RAID background rebuild
pub const ACB_F_MSG_START_BGRB: c_uint = 0x0004;
// stop RAID background rebuild
pub const ACB_F_IOPDATA_OVERFLOW: c_uint = 0x0008;
// iop message data rqbuffer overflow
pub const ACB_F_MESSAGE_WQBUFFER_CLEARED: c_uint = 0x0010;
// message clear wqbuffer
pub const ACB_F_MESSAGE_RQBUFFER_CLEARED: c_uint = 0x0020;
// message clear rqbuffer
pub const ACB_F_MESSAGE_WQBUFFER_READED: c_uint = 0x0040;
pub const ACB_F_BUS_RESET: c_uint = 0x0080;
pub const ACB_F_IOP_INITED: c_uint = 0x0100;
// iop init
pub const ACB_F_ABORT: c_uint = 0x0200;
pub const ACB_F_FIRMWARE_TRAP: c_uint = 0x0400;
pub const ACB_F_ADAPTER_REMOVED: c_uint = 0x0800;
pub const ACB_F_MSG_GET_CONFIG: c_uint = 0x1000;
// used for memory free
// head of free ccb list
// The present outstanding command number that in the IOP that
// dma_coherent used for memory free
// dma_coherent_handle used for memory free
// data collection buffer for read from 80331
// first of read buffer
// last of read buffer
// data collection buffer for write to 80331
// first of write buffer
// last of write buffer
// id0 ..... id15, lun0...lun7
pub const ARECA_RAID_GONE: c_uint = 0x55;
pub const ARECA_RAID_GOOD: c_uint = 0xaa;
pub const FW_NORMAL: c_uint = 0x0000;
pub const FW_BOG: c_uint = 0x0001;
pub const FW_DEADLOCK: c_uint = 0x0010;
//
// Command Control Block
// this CCB length must be 32 bytes boundary
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandControlBlock {
// x32:sizeof struct_CCB=(64+60)byte, x64:sizeof struct_CCB=(64+60)byte
    pub 16byte*/: *mut *mut list_head list; /x32: 8byte, x64:,
    pub /: *mut *mut *mut scsi_cmnd pcmd; /8 bytes pointer of linux scsi command,
    pub 8byte*/: *mut *mut *mut AdapterControlBlock acb; /x32: 4byte, x64:,
    pub 8byte*/: *mut *mut unsigned long cdb_phyaddr; /x32: 4byte, x64:,
    pub /*x32:4byte,x64:4byte*/: *mut uint32_t arc_cdb_size;,
    pub 2byte*/: *mut *mut uint16_t ccb_flags; /x32: 2byte, x64:,
pub const CCB_FLAG_READ: c_uint = 0x0000;
pub const CCB_FLAG_WRITE: c_uint = 0x0001;
pub const CCB_FLAG_ERROR: c_uint = 0x0002;
pub const CCB_FLAG_FLUSHCACHE: c_uint = 0x0004;
pub const CCB_FLAG_MASTER_ABORTED: c_uint = 0x0008;
    pub /*x32:2byte,x32:2byte*/: *mut uint16_t startdone;,
pub const ARCMSR_CCB_DONE: c_uint = 0x0000;
pub const ARCMSR_CCB_START: c_uint = 0x55AA;
pub const ARCMSR_CCB_ABORTED: c_uint = 0xAA55;
pub const ARCMSR_CCB_ILLEGAL: c_uint = 0xFFFF;
    pub smid: u32,

// ======================512+64 bytes========================
    pub byte*/: *mut *mut uint32_t reserved[3]; /12,

// ======================512+32 bytes========================
    pub byte*/: *mut *mut uint32_t reserved[8]; /32,

// =======================================================
    pub arcmsr_cdb: ARCMSR_CDB,
}

//
// ARECA SCSI sense data
//
pub const SCSI_SENSE_CURRENT_ERRORS: c_uint = 0x70;
pub const SCSI_SENSE_DEFERRED_ERRORS: c_uint = 0x71;
//
// Outbound Interrupt Status Register - OISR
//
pub const ARCMSR_MU_OUTBOUND_INTERRUPT_STATUS_REG: c_uint = 0x30;
pub const ARCMSR_MU_OUTBOUND_PCI_INT: c_uint = 0x10;
pub const ARCMSR_MU_OUTBOUND_POSTQUEUE_INT: c_uint = 0x08;
pub const ARCMSR_MU_OUTBOUND_DOORBELL_INT: c_uint = 0x04;
pub const ARCMSR_MU_OUTBOUND_MESSAGE1_INT: c_uint = 0x02;
pub const ARCMSR_MU_OUTBOUND_MESSAGE0_INT: c_uint = 0x01;

//
// Outbound Interrupt Mask Register - OIMR
//
pub const ARCMSR_MU_OUTBOUND_INTERRUPT_MASK_REG: c_uint = 0x34;
pub const ARCMSR_MU_OUTBOUND_PCI_INTMASKENABLE: c_uint = 0x10;
pub const ARCMSR_MU_OUTBOUND_POSTQUEUE_INTMASKENABLE: c_uint = 0x08;
pub const ARCMSR_MU_OUTBOUND_DOORBELL_INTMASKENABLE: c_uint = 0x04;
pub const ARCMSR_MU_OUTBOUND_MESSAGE1_INTMASKENABLE: c_uint = 0x02;
pub const ARCMSR_MU_OUTBOUND_MESSAGE0_INTMASKENABLE: c_uint = 0x01;
pub const ARCMSR_MU_OUTBOUND_ALL_INTMASKENABLE: c_uint = 0x1F;
extern "C" {
    pub fn arcmsr_write_ioctldata2iop(: *mut AdapterControlBlock);
}
extern "C" {
    pub fn arcmsr_clear_iop2drv_rqueue_buffer(: *mut AdapterControlBlock);
}
extern "C" {
    pub fn arcmsr_alloc_sysfs_attr(: *mut AdapterControlBlock) -> c_int;
}
extern "C" {
    pub fn arcmsr_free_sysfs_attr(acb: *mut AdapterControlBlock);
}
