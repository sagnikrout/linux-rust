//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/scu_task_context.h
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
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2008 - 2011 Intel Corporation. All rights reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St - Fifth Floor, Boston, MA 02110-1301 USA.
// The full GNU General Public License is included in this distribution
// in the file called LICENSE.GPL.
//
// BSD LICENSE
//
// Copyright(c) 2008 - 2011 Intel Corporation. All rights reserved.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// This file contains the structures and constants for the SCU hardware task
// context.
//
// enum scu_ssp_task_type - This enumberation defines the various SSP task
// types the SCU hardware will accept. The definition for the various task
// types the SCU hardware will accept can be found in the DS specification.
//
// enum scu_sata_task_type - This enumeration defines the various SATA task
// types the SCU hardware will accept. The definition for the various task
// types the SCU hardware will accept can be found in the DS specification.
//
// SCU_CONTEXT_TYPE
//
pub const SCU_TASK_CONTEXT_TYPE: c_int = 0;
pub const SCU_RNC_CONTEXT_TYPE: c_int = 1;
//
// SCU_TASK_CONTEXT_VALIDITY
//
pub const SCU_TASK_CONTEXT_INVALID: c_int = 0;
pub const SCU_TASK_CONTEXT_VALID: c_int = 1;
//
// SCU_COMMAND_CODE
//
pub const SCU_COMMAND_CODE_INITIATOR_NEW_TASK: c_int = 0;
pub const SCU_COMMAND_CODE_ACTIVE_TASK: c_int = 1;
pub const SCU_COMMAND_CODE_PRIMITIVE_SEQ_TASK: c_int = 2;
pub const SCU_COMMAND_CODE_TARGET_RAW_FRAMES: c_int = 3;
//
// SCU_TASK_PRIORITY
//
// This priority is used when there is no priority request for this request.
//
pub const SCU_TASK_PRIORITY_NORMAL: c_int = 0;
//
// This priority indicates that the task should be scheduled to the head of the
// queue.  The task will NOT be executed if the TX is suspended for the remote
// node.
//
pub const SCU_TASK_PRIORITY_HEAD_OF_Q: c_int = 1;
//
// This priority indicates that the task will be executed before all
// SCU_TASK_PRIORITY_NORMAL and SCU_TASK_PRIORITY_HEAD_OF_Q tasks. The task
// WILL be executed if the TX is suspended for the remote node.
//
pub const SCU_TASK_PRIORITY_HIGH: c_int = 2;
//
// This task priority is reserved and should not be used.
//
pub const SCU_TASK_PRIORITY_RESERVED: c_int = 3;
pub const SCU_TASK_INITIATOR_MODE: c_int = 1;
pub const SCU_TASK_TARGET_MODE: c_int = 0;
pub const SCU_TASK_REGULAR: c_int = 0;
pub const SCU_TASK_ABORTED: c_int = 1;
// direction bit defintion
//
// SATA_DIRECTION
//
pub const SCU_SATA_WRITE_DATA_DIRECTION: c_int = 0;
pub const SCU_SATA_READ_DATA_DIRECTION: c_int = 1;
//
// SCU_COMMAND_CONTEXT_MACROS These macros provide the mask and shift
// operations to construct the various SCU commands
//
pub const SCU_CONTEXT_COMMAND_REQUEST_TYPE_SHIFT: c_int = 21;
pub const SCU_CONTEXT_COMMAND_REQUEST_TYPE_MASK: c_uint = 0x00E00000;

pub const SCU_CONTEXT_COMMAND_REQUEST_SUBTYPE_SHIFT: c_int = 18;
pub const SCU_CONTEXT_COMMAND_REQUEST_SUBTYPE_MASK: c_uint = 0x001C0000;
pub const SCU_CONTEXT_COMMAND_PROTOCOL_ENGINE_GROUP_SHIFT: c_int = 16;
pub const SCU_CONTEXT_COMMAND_LOGICAL_PORT_SHIFT: c_int = 12;
pub const SCU_CONTEXT_COMMAND_LOGICAL_PORT_MASK: c_uint = 0x00007000;

//
// MAKE_SCU_CONTEXT_COMMAND_TYPE() -
//
// SCU_COMMAND_TYPES These constants provide the grouping of the different SCU
// command types.
//

//
// SCU_REQUEST_TYPES These constants are the various request types that can be
// posted to the SCU hardware.
//

//
// SCU_TASK_CONTEXT_PROTOCOL SCU Task context protocol types this is uesd to
// program the SCU Task context protocol field in word 0x00.
//
pub const SCU_TASK_CONTEXT_PROTOCOL_SMP: c_uint = 0x00;
pub const SCU_TASK_CONTEXT_PROTOCOL_SSP: c_uint = 0x01;
pub const SCU_TASK_CONTEXT_PROTOCOL_STP: c_uint = 0x02;
pub const SCU_TASK_CONTEXT_PROTOCOL_NONE: c_uint = 0x07;
//
// struct ssp_task_context - This is the SCU hardware definition for an SSP
// request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_task_context {
// OFFSET 0x18
    pub reserved00:24: u32,
    pub frame_type:8: u32,
// OFFSET 0x1C
    pub reserved01: u32,
// OFFSET 0x20
    pub fill_bytes:2: u32,
    pub reserved02:6: u32,
    pub changing_data_pointer:1: u32,
    pub retransmit:1: u32,
    pub retry_data_frame:1: u32,
    pub tlr_control:2: u32,
    pub reserved03:19: u32,
// OFFSET 0x24
    pub uiRsvd4: u32,
// OFFSET 0x28
    pub target_port_transfer_tag:16: u32,
    pub tag:16: u32,
// OFFSET 0x2C
    pub data_offset: u32,
}

//
// struct stp_task_context - This is the SCU hardware definition for an STP
// request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_task_context {
// OFFSET 0x18
    pub fis_type:8: u32,
    pub pm_port:4: u32,
    pub reserved0:3: u32,
    pub control:1: u32,
    pub command:8: u32,
    pub features:8: u32,
// OFFSET 0x1C
    pub reserved1: u32,
// OFFSET 0x20
    pub reserved2: u32,
// OFFSET 0x24
    pub reserved3: u32,
// OFFSET 0x28
    pub ncq_tag:5: u32,
    pub reserved4:27: u32,
// OFFSET 0x2C
    pub /: *mut *mut u32 data_offset; / TODO: What is this used for?,
}

//
// struct smp_task_context - This is the SCU hardware definition for an SMP
// request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_task_context {
// OFFSET 0x18
    pub response_length:8: u32,
    pub function_result:8: u32,
    pub function:8: u32,
    pub frame_type:8: u32,
// OFFSET 0x1C
    pub smp_response_ufi:12: u32,
    pub reserved1:20: u32,
// OFFSET 0x20
    pub reserved2: u32,
// OFFSET 0x24
    pub reserved3: u32,
// OFFSET 0x28
    pub reserved4: u32,
// OFFSET 0x2C
    pub reserved5: u32,
}

//
// struct primitive_task_context - This is the SCU hardware definition used
// when the driver wants to send a primitive on the link.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct primitive_task_context {
// OFFSET 0x18
//
// This field is the control word and it must be 0.
//
    pub /: *mut *mut u32 control; / /< must be set to 0,
// OFFSET 0x1C
//
// This field specifies the primitive that is to be transmitted.
//
    pub sequence: u32,
// OFFSET 0x20
    pub reserved0: u32,
// OFFSET 0x24
    pub reserved1: u32,
// OFFSET 0x28
    pub reserved2: u32,
// OFFSET 0x2C
    pub reserved3: u32,
}

//
// The union of the protocols that can be selected in the SCU task context
// field.
//
// protocol_context
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union protocol_context {
    pub ssp: ssp_task_context,
    pub stp: stp_task_context,
    pub smp: smp_task_context,
    pub primitive: primitive_task_context,
    pub words: [u32; 6],
}

//
// struct scu_sgl_element - This structure represents a single SCU defined SGL
// element. SCU SGLs contain a 64 bit address with the maximum data transfer
// being 24 bits in size.  The SGL can not cross a 4GB boundary.
//
// struct scu_sgl_element
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_sgl_element {
//
// This field is the upper 32 bits of the 64 bit physical address.
//
    pub address_upper: u32,
//
// This field is the lower 32 bits of the 64 bit physical address.
//
    pub address_lower: u32,
//
// This field is the number of bytes to transfer.
//
    pub length:24: u32,
//
// This field is the address modifier to be used when a virtual function is
// requesting a data transfer.
//
    pub address_modifier:8: u32,
}

pub const SCU_SGL_ELEMENT_PAIR_A: c_int = 0;
pub const SCU_SGL_ELEMENT_PAIR_B: c_int = 1;
//
// struct scu_sgl_element_pair - This structure is the SCU hardware definition
// of a pair of SGL elements. The SCU hardware always works on SGL pairs.
// They are refered to in the DS specification as SGL A and SGL B.  Each SGL
// pair is followed by the address of the next pair.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_sgl_element_pair {
// OFFSET 0x60-0x68
//
// This field is the SGL element A of the SGL pair.
//
    pub A: scu_sgl_element,
// OFFSET 0x6C-0x74
//
// This field is the SGL element B of the SGL pair.
//
    pub B: scu_sgl_element,
// OFFSET 0x78-0x7C
//
// This field is the upper 32 bits of the 64 bit address to the next SGL
// element pair.
//
    pub next_pair_upper: u32,
//
// This field is the lower 32 bits of the 64 bit address to the next SGL
// element pair.
//
    pub next_pair_lower: u32,
}

//
// struct transport_snapshot - This structure is the SCU hardware scratch area
// for the task context. This is set to 0 by the driver but can be read by
// issuing a dump TC request to the SCU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transport_snapshot {
// OFFSET 0x48
    pub xfer_rdy_write_data_length: u32,
// OFFSET 0x4C
    pub data_offset: u32,
// OFFSET 0x50
    pub data_transfer_size:24: u32,
    pub reserved_50_0:8: u32,
// OFFSET 0x54
    pub next_initiator_write_data_offset: u32,
// OFFSET 0x58
    pub next_initiator_write_data_xfer_size:24: u32,
    pub reserved_58_0:8: u32,
}

//
// struct scu_task_context - This structure defines the contents of the SCU
// silicon task context. It lays out all of the fields according to the
// expected order and location for the Storage Controller unit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scu_task_context {
// OFFSET 0x00 ------
//
// This field must be encoded to one of the valid SCU task priority values
// - SCU_TASK_PRIORITY_NORMAL
// - SCU_TASK_PRIORITY_HEAD_OF_Q
// - SCU_TASK_PRIORITY_HIGH
//
    pub priority:2: u32,
//
// This field must be set to true if this is an initiator generated request.
// Until target mode is supported all task requests are initiator requests.
//
    pub initiator_request:1: u32,
//
// This field must be set to one of the valid connection rates valid values
// are 0x8, 0x9, and 0xA.
//
    pub connection_rate:4: u32,
//
// This field muse be programed when generating an SMP response since the SMP
// connection remains open until the SMP response is generated.
//
    pub protocol_engine_index:3: u32,
//
// This field must contain the logical port for the task request.
//
    pub logical_port_index:3: u32,
//
// This field must be set to one of the SCU_TASK_CONTEXT_PROTOCOL values
// - SCU_TASK_CONTEXT_PROTOCOL_SMP
// - SCU_TASK_CONTEXT_PROTOCOL_SSP
// - SCU_TASK_CONTEXT_PROTOCOL_STP
// - SCU_TASK_CONTEXT_PROTOCOL_NONE
//
    pub protocol_type:3: u32,
//
// This filed must be set to the TCi allocated for this task
//
    pub task_index:12: u32,
//
// This field is reserved and must be set to 0x00
//
    pub reserved_00_0:1: u32,
//
// For a normal task request this must be set to 0.  If this is an abort of
// this task request it must be set to 1.
//
    pub abort:1: u32,
//
// This field must be set to true for the SCU hardware to process the task.
//
    pub valid:1: u32,
//
// This field must be set to SCU_TASK_CONTEXT_TYPE
//
    pub context_type:1: u32,
// OFFSET 0x04
//
// This field contains the RNi that is the target of this request.
//
    pub remote_node_index:12: u32,
//
// This field is programmed if this is a mirrored request, which we are not
// using, in which case it is the RNi for the mirrored target.
//
    pub mirrored_node_index:12: u32,
//
// This field is programmed with the direction of the SATA reqeust
// - SCU_SATA_WRITE_DATA_DIRECTION
// - SCU_SATA_READ_DATA_DIRECTION
//
    pub sata_direction:1: u32,
//
// This field is programmsed with one of the following SCU_COMMAND_CODE
// - SCU_COMMAND_CODE_INITIATOR_NEW_TASK
// - SCU_COMMAND_CODE_ACTIVE_TASK
// - SCU_COMMAND_CODE_PRIMITIVE_SEQ_TASK
// - SCU_COMMAND_CODE_TARGET_RAW_FRAMES
//
    pub command_code:2: u32,
//
// This field is set to true if the remote node should be suspended.
// This bit is only valid for SSP & SMP target devices.
//
    pub suspend_node:1: u32,
//
// This field is programmed with one of the following command type codes
//
// For SAS requests use the scu_ssp_task_type
// - SCU_TASK_TYPE_IOREAD
// - SCU_TASK_TYPE_IOWRITE
// - SCU_TASK_TYPE_SMP_REQUEST
// - SCU_TASK_TYPE_RESPONSE
// - SCU_TASK_TYPE_RAW_FRAME
// - SCU_TASK_TYPE_PRIMITIVE
//
// For SATA requests use the scu_sata_task_type
// - SCU_TASK_TYPE_DMA_IN
// - SCU_TASK_TYPE_FPDMAQ_READ
// - SCU_TASK_TYPE_PACKET_DMA_IN
// - SCU_TASK_TYPE_SATA_RAW_FRAME
// - SCU_TASK_TYPE_DMA_OUT
// - SCU_TASK_TYPE_FPDMAQ_WRITE
// - SCU_TASK_TYPE_PACKET_DMA_OUT
//
    pub task_type:4: u32,
// OFFSET 0x08
//
// This field is reserved and the must be set to 0x00
//
    pub /: *mut *mut u32 link_layer_control:8; / presently all reserved,
//
// This field is set to true when TLR is to be enabled
//
    pub ssp_tlr_enable:1: u32,
//
// This is field specifies if the SCU DMAs a response frame to host
// memory for good response frames when operating in target mode.
//
    pub dma_ssp_target_good_response:1: u32,
//
// This field indicates if the SCU should DMA the response frame to
// host memory.
//
    pub do_not_dma_ssp_good_response:1: u32,
//
// This field is set to true when strict ordering is to be enabled
//
    pub strict_ordering:1: u32,
//
// This field indicates the type of endianess to be utilized for the
// frame.  command, task, and response frames utilized control_frame
// set to 1.
//
    pub control_frame:1: u32,
//
// This field is reserved and the driver should set to 0x00
//
    pub tl_control_reserved:3: u32,
//
// This field is set to true when the SCU hardware task timeout control is to
// be enabled
//
    pub timeout_enable:1: u32,
//
// This field is reserved and the driver should set it to 0x00
//
    pub pts_control_reserved:7: u32,
//
// This field should be set to true when block guard is to be enabled
//
    pub block_guard_enable:1: u32,
//
// This field is reserved and the driver should set to 0x00
//
    pub sdma_control_reserved:7: u32,
// OFFSET 0x0C
//
// This field is the address modifier for this io request it should be
// programmed with the virtual function that is making the request.
//
    pub address_modifier:16: u32,
//
// @todo What we support mirrored SMP response frame?
//
    pub /: *mut *mut u32 mirrored_protocol_engine:3; / mirrored protocol Engine Index,
//
// If this is a mirrored request the logical port index for the mirrored RNi
// must be programmed.
//
    pub /: *mut *mut u32 mirrored_logical_port:4; / mirrored local port index,
//
// This field is reserved and the driver must set it to 0x00
//
    pub reserved_0C_0:8: u32,
//
// This field must be set to true if the mirrored request processing is to be
// enabled.
//
    pub /: *mut *mut u32 mirror_request_enable:1; / Mirrored request Enable,
// OFFSET 0x10
//
// This field is the command iu length in dwords
//
    pub ssp_command_iu_length:8: u32,
//
// This is the target TLR enable bit it must be set to 0 when creatning the
// task context.
//
    pub xfer_ready_tlr_enable:1: u32,
//
// This field is reserved and the driver must set it to 0x00
//
    pub reserved_10_0:7: u32,
//
// This is the maximum burst size that the SCU hardware will send in one
// connection its value is (N x 512) and N must be a multiple of 2.  If the
// value is 0x00 then maximum burst size is disabled.
//
    pub ssp_max_burst_size:16: u32,
// OFFSET 0x14
//
// This filed is set to the number of bytes to be transfered in the request.
//
    pub /: *mut *mut u32 transfer_length_bytes:24; / In terms of bytes,
//
// This field is reserved and the driver should set it to 0x00
//
    pub reserved_14_0:8: u32,
// OFFSET 0x18-0x2C
//
// This union provides for the protocol specif part of the SCU Task Context.
//
    pub type: protocol_context,
// OFFSET 0x30-0x34
//
// This field is the upper 32 bits of the 64 bit physical address of the
// command iu buffer
//
    pub command_iu_upper: u32,
//
// This field is the lower 32 bits of the 64 bit physical address of the
// command iu buffer
//
    pub command_iu_lower: u32,
// OFFSET 0x38-0x3C
//
// This field is the upper 32 bits of the 64 bit physical address of the
// response iu buffer
//
    pub response_iu_upper: u32,
//
// This field is the lower 32 bits of the 64 bit physical address of the
// response iu buffer
//
    pub response_iu_lower: u32,
// OFFSET 0x40
//
// This field is set to the task phase of the SCU hardware. The driver must
// set this to 0x01
//
    pub task_phase:8: u32,
//
// This field is set to the transport layer task status.  The driver must set
// this to 0x00
//
    pub task_status:8: u32,
//
// This field is used during initiator write TLR
//
    pub previous_extended_tag:4: u32,
//
// This field is set the maximum number of retries for a STP non-data FIS
//
    pub stp_retry_count:2: u32,
//
// This field is reserved and the driver must set it to 0x00
//
    pub reserved_40_1:2: u32,
//
// This field is used by the SCU TL to determine when to take a snapshot when
// transmitting read data frames.
// - 0x00 The entire IO
// - 0x01 32k
// - 0x02 64k
// - 0x04 128k
// - 0x08 256k
//
    pub ssp_tlr_threshold:4: u32,
//
// This field is reserved and the driver must set it to 0x00
//
    pub reserved_40_2:4: u32,
// OFFSET 0x44
    pub /: *mut *mut u32 write_data_length; / read only set to 0,
// OFFSET 0x48-0x58
    pub /: *mut *mut transport_snapshot snapshot; / read only set to 0,
// OFFSET 0x5C
    pub blk_prot_en:1: u32,
    pub blk_sz:2: u32,
    pub blk_prot_func:2: u32,
    pub reserved_5C_0:9: u32,
    pub /: *mut *mut u32 active_sgl_element:2; / read only set to 0,
    pub /: *mut *mut u32 sgl_exhausted:1; / read only set to 0,
    pub /: *mut *mut u32 payload_data_transfer_error:4; / read only set to 0,
    pub /: *mut *mut u32 frame_buffer_offset:11; / read only set to 0,
// OFFSET 0x60-0x7C
//
// This field is the first SGL element pair found in the TC data structure.
//
    pub sgl_pair_ab: scu_sgl_element_pair,
// OFFSET 0x80-0x9C
//
// This field is the second SGL element pair found in the TC data structure.
//
    pub sgl_pair_cd: scu_sgl_element_pair,
// OFFSET 0xA0-BC
    pub sgl_snapshot_ac: scu_sgl_element_pair,
// OFFSET 0xC0
    pub /: *mut *mut u32 active_sgl_element_pair; / read only set to 0,
// OFFSET 0xC4-0xCC
    pub reserved_C4_CC: [u32; 3],
// OFFSET 0xD0
    pub interm_crc_val:16: u32,
    pub init_crc_seed:16: u32,
// OFFSET 0xD4
    pub app_tag_verify:16: u32,
    pub app_tag_gen:16: u32,
// OFFSET 0xD8
    pub ref_tag_seed_verify: u32,
// OFFSET 0xDC
    pub UD_bytes_immed_val:13: u32,
    pub reserved_DC_0:3: u32,
    pub DIF_bytes_immed_val:4: u32,
    pub reserved_DC_1:12: u32,
// OFFSET 0xE0
    pub bgc_blk_sz:13: u32,
    pub reserved_E0_0:3: u32,
    pub app_tag_gen_mask:16: u32,
// OFFSET 0xE4
    pub bgctl: u16,
    pub crc_verify:1: u16,
    pub app_tag_chk:1: u16,
    pub ref_tag_chk:1: u16,
    pub op:2: u16,
    pub legacy:1: u16,
    pub invert_crc_seed:1: u16,
    pub ref_tag_gen:1: u16,
    pub fixed_ref_tag:1: u16,
    pub invert_crc:1: u16,
    pub app_ref_f_detect:1: u16,
    pub uninit_dif_check_err:1: u16,
    pub uninit_dif_bypass:1: u16,
    pub app_f_detect:1: u16,
    pub reserved_0:2: u16,
    pub bgctl_f: },
}

// OFFSET 0xE8
// OFFSET 0xEC
// OFFSET 0xF0
// OFFSET 0xF4
// OFFSET 0xF8
// OFFSET 0xFC
