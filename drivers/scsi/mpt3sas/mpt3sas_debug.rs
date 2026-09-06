//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpt3sas_debug.h
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
// Logging Support for MPT (Message Passing Technology) based controllers
//
// This code is based on drivers/scsi/mpt3sas/mpt3sas_debug.c
// Copyright (C) 2012-2014  LSI Corporation
// Copyright (C) 2013-2014 Avago Technologies
// (mailto: MPT-FusionLinux.pdl@avagotech.com)
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// NO WARRANTY
// THE PROGRAM IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, EITHER EXPRESS OR IMPLIED INCLUDING, WITHOUT
// LIMITATION, ANY WARRANTIES OR CONDITIONS OF TITLE, NON-INFRINGEMENT,
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE. Each Recipient is
// solely responsible for determining the appropriateness of using and
// distributing the Program and assumes all risks associated with its
// exercise of rights under this Agreement, including but not limited to
// the risks and costs of program errors, damage to or loss of data,
// programs or equipment, and unavailability or interruption of operations.
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301,
// USA.
//

// Macro flag: #define MPT3SAS_DEBUG_H_INCLUDED
pub const MPT_DEBUG: c_uint = 0x00000001;
pub const MPT_DEBUG_MSG_FRAME: c_uint = 0x00000002;
pub const MPT_DEBUG_SG: c_uint = 0x00000004;
pub const MPT_DEBUG_EVENTS: c_uint = 0x00000008;
pub const MPT_DEBUG_EVENT_WORK_TASK: c_uint = 0x00000010;
pub const MPT_DEBUG_INIT: c_uint = 0x00000020;
pub const MPT_DEBUG_EXIT: c_uint = 0x00000040;
pub const MPT_DEBUG_FAIL: c_uint = 0x00000080;
pub const MPT_DEBUG_TM: c_uint = 0x00000100;
pub const MPT_DEBUG_REPLY: c_uint = 0x00000200;
pub const MPT_DEBUG_HANDSHAKE: c_uint = 0x00000400;
pub const MPT_DEBUG_CONFIG: c_uint = 0x00000800;
pub const MPT_DEBUG_DL: c_uint = 0x00001000;
pub const MPT_DEBUG_RESET: c_uint = 0x00002000;
pub const MPT_DEBUG_SCSI: c_uint = 0x00004000;
pub const MPT_DEBUG_IOCTL: c_uint = 0x00008000;
pub const MPT_DEBUG_SAS: c_uint = 0x00020000;
pub const MPT_DEBUG_TRANSPORT: c_uint = 0x00040000;
pub const MPT_DEBUG_TASK_SET_FULL: c_uint = 0x00080000;
pub const MPT_DEBUG_TRIGGER_DIAG: c_uint = 0x00200000;

//
// debug macros
//

// inline functions for dumping debug data
//
// _debug_dump_mf - print message frame contents
// @mpi_request: pointer to message frame
// @sz: number of dwords
//
// _debug_dump_reply - print message frame contents
// @mpi_request: pointer to message frame
// @sz: number of dwords
//
// _debug_dump_config - print config page contents
// @mpi_request: pointer to message frame
// @sz: number of dwords
//
