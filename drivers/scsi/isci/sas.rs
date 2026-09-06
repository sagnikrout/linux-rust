//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/isci/sas.h
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

//
// SATA FIS Types These constants depict the various SATA FIS types devined in
// the serial ATA specification.
// XXX: This needs to go into <scsi/sas.h>
//
pub const FIS_REGH2D: c_uint = 0x27;
pub const FIS_REGD2H: c_uint = 0x34;
pub const FIS_SETDEVBITS: c_uint = 0xA1;
pub const FIS_DMA_ACTIVATE: c_uint = 0x39;
pub const FIS_DMA_SETUP: c_uint = 0x41;
pub const FIS_BIST_ACTIVATE: c_uint = 0x58;
pub const FIS_PIO_SETUP: c_uint = 0x5F;
pub const FIS_DATA: c_uint = 0x46;
//
pub const SSP_RESP_IU_MAX_SIZE: c_int = 280;
//
// contents of the SSP COMMAND INFORMATION UNIT.
// For specific information on each of these individual fields please
// reference the SAS specification SSP transport layer section.
// XXX: This needs to go into <scsi/sas.h>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_cmd_iu {
    pub LUN: [u8; 8],
    pub add_cdb_len:6: u8,
    pub _r_a:2: u8,
    pub _r_b: u8,
    pub en_fburst:1: u8,
    pub task_prio:4: u8,
    pub task_attr:3: u8,
    pub _r_c: u8,
    pub cdb: [u8; 16],
    pub __packed: },
//
// contents of the SSP TASK INFORMATION UNIT.
// For specific information on each of these individual fields please
// reference the SAS specification SSP transport layer section.
// XXX: This needs to go into <scsi/sas.h>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_task_iu {
    pub LUN: [u8; 8],
    pub _r_a: u8,
    pub task_func: u8,
    pub _r_b: [u8; 4],
    pub task_tag: u16,
    pub _r_c: [u8; 12],
    pub __packed: },
//
// struct smp_req_phy_id - This structure defines the contents of
// an SMP Request that is comprised of the struct smp_request_header and a
// phy identifier.
// Examples: SMP_REQUEST_DISCOVER, SMP_REQUEST_REPORT_PHY_SATA.
//
// For specific information on each of these individual fields please reference
// the SAS specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_req_phy_id {
    pub /: *mut *mut u8 _r_a[4]; / bytes 4-7,
    pub /: *mut *mut u8 ign_zone_grp:1; / byte 8,
    pub _r_b:7: u8,
    pub /: *mut *mut u8 phy_id; / byte 9,
    pub /: *mut *mut u8 _r_c; / byte 10,
    pub /: *mut *mut u8 _r_d; / byte 11,
    pub __packed: },
//
// struct smp_req_config_route_info - This structure defines the
// contents of an SMP Configure Route Information request.
//
// For specific information on each of these individual fields please reference
// the SAS specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_req_conf_rtinfo {
    pub /: *mut *mut u16 exp_change_cnt; / bytes 4-5,
    pub /: *mut *mut u8 exp_rt_idx_hi; / byte 6,
    pub /: *mut *mut u8 exp_rt_idx; / byte 7,
    pub /: *mut *mut u8 _r_a; / byte 8,
    pub /: *mut *mut u8 phy_id; / byte 9,
    pub /: *mut *mut u16 _r_b; / bytes 10-11,
    pub /: *mut *mut u8 _r_c:7; / byte 12,
    pub dis_rt_entry:1: u8,
    pub /: *mut *mut u8 _r_d[3]; / bytes 13-15,
    pub /: *mut *mut u8 rt_sas_addr[8]; / bytes 16-23,
    pub /: *mut *mut u8 _r_e[16]; / bytes 24-39,
    pub __packed: },
//
// struct smp_req_phycntl - This structure defines the contents of an
// SMP Phy Controller request.
//
// For specific information on each of these individual fields please reference
// the SAS specification.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_req_phycntl {
    pub /: *mut *mut u16 exp_change_cnt; / byte 4-5,
    pub /: *mut *mut u8 _r_a[3]; / bytes 6-8,
    pub /: *mut *mut u8 phy_id; / byte 9,
    pub /: *mut *mut u8 phy_op; / byte 10,
    pub /: *mut *mut u8 upd_pathway:1; / byte 11,
    pub _r_b:7: u8,
    pub /: *mut *mut u8 _r_c[12]; / byte 12-23,
    pub /: *mut *mut u8 att_dev_name[8]; / byte 24-31,
    pub /: *mut *mut u8 _r_d:4; / byte 32,
    pub min_linkrate:4: u8,
    pub /: *mut *mut u8 _r_e:4; / byte 33,
    pub max_linkrate:4: u8,
    pub /: *mut *mut u8 _r_f[2]; / byte 34-35,
    pub /: *mut *mut u8 pathway:4; / byte 36,
    pub _r_g:4: u8,
    pub /: *mut *mut u8 _r_h[3]; / bytes 37-39,
    pub __packed: },
//
// struct smp_req - This structure simply unionizes the existing request
// structures into a common request type.
//
// XXX: This data structure may need to go to scsi/sas.h
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smp_req {
    pub /: *mut *mut u8 type; / byte 0,
    pub /: *mut *mut u8 func; / byte 1,
    pub /: *mut *mut u8 alloc_resp_len; / byte 2,
    pub /: *mut *mut u8 req_len; / byte 3,
    pub req_data: [u8; ],
    pub __packed: },
//
// struct sci_sas_address - This structure depicts how a SAS address is
// represented by SCI.
// XXX convert this to u8 [SAS_ADDR_SIZE] like the rest of libsas
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sci_sas_address {
    pub high: u32,
    pub low: u32,
}
