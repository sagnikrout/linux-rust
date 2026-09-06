//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/esas2r/esas2r_log.h
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
// linux/drivers/scsi/esas2r/esas2r_log.h
// For use with ATTO ExpressSAS R6xx SAS/SATA RAID controllers
//
// Copyright (c) 2001-2013 ATTO Technology, Inc.
// (mailto:linuxdrivers@attotech.com)
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
//
// DISCLAIMER OF LIABILITY
// NEITHER RECIPIENT NOR ANY CONTRIBUTORS SHALL HAVE ANY LIABILITY FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING WITHOUT LIMITATION LOST PROFITS), HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
// TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OR DISTRIBUTION OF THE PROGRAM OR THE EXERCISE OF ANY RIGHTS GRANTED
// HEREUNDER, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGES
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301,
// USA.
//

// Macro flag: #define __esas2r_log_h__

//
// the following macros are provided specifically for debugging and tracing
// messages.  esas2r_debug() is provided for generic non-hardware layer
// debugging and tracing events.  esas2r_hdebug is provided specifically for
// hardware layer debugging and tracing events.
//

//
// the following macros are provided in order to trace the driver and catch
// some more serious bugs.  be warned, enabling these macros may *severely
// impact performance.
//

// Macro flag: #define esas2r_bugon()
// Macro flag: #define esas2r_trace_enter()
// Macro flag: #define esas2r_trace_exit()

