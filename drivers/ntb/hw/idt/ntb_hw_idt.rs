//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ntb/hw/idt/ntb_hw_idt.h
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
// This file is provided under a GPLv2 license.  When using or
// redistributing this file, you may do so under that license.
//
// GPL LICENSE SUMMARY
//
// Copyright (C) 2016-2018 T-Platforms JSC All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms and conditions of the GNU General Public License,
// version 2, as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General
// Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, one can be found http://www.gnu.org/licenses/.
//
// The full GNU General Public License is included in this distribution in
// the file called "COPYING".
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
// IDT PCIe-switch NTB Linux driver
//
// Contact Information:
// Serge Semin <fancer.lancer@gmail.com>, <Sergey.Semin@t-platforms.ru>
//

//
// Macro is used to create the struct pci_device_id that matches
// the supported IDT PCIe-switches
// @devname: Capitalized name of the particular device
// @data: Variable passed to the driver of the particular device
//

//
// IDT PCIe-switches device IDs
//
pub const PCI_DEVICE_ID_IDT_89HPES24NT6AG2: c_uint = 0x8091;
pub const PCI_DEVICE_ID_IDT_89HPES32NT8AG2: c_uint = 0x808F;
pub const PCI_DEVICE_ID_IDT_89HPES32NT8BG2: c_uint = 0x8088;
pub const PCI_DEVICE_ID_IDT_89HPES12NT12G2: c_uint = 0x8092;
pub const PCI_DEVICE_ID_IDT_89HPES16NT16G2: c_uint = 0x8090;
pub const PCI_DEVICE_ID_IDT_89HPES24NT24G2: c_uint = 0x808E;
pub const PCI_DEVICE_ID_IDT_89HPES32NT24AG2: c_uint = 0x808C;
pub const PCI_DEVICE_ID_IDT_89HPES32NT24BG2: c_uint = 0x808A;
//
// NT-function Configuration Space registers
// NOTE 1) The IDT PCIe-switch internal data is little-endian
// so it must be taken into account in the driver
// internals.
// 2) Additionally the registers should be accessed either
// with byte-enables corresponding to their native size or
// the size of one DWORD
//
// So to simplify the driver code, there is only DWORD-sized read/write
// operations utilized.
//
// PCI Express Configuration Space
// PCI Express command/status register	(DWORD)
pub const IDT_NT_PCICMDSTS: c_uint = 0x00004U;
// PCI Express Device Capabilities	(DWORD)
pub const IDT_NT_PCIEDCAP: c_uint = 0x00044U;
// PCI Express Device Control/Status	(WORD+WORD)
pub const IDT_NT_PCIEDCTLSTS: c_uint = 0x00048U;
// PCI Express Link Capabilities	(DWORD)
pub const IDT_NT_PCIELCAP: c_uint = 0x0004CU;
// PCI Express Link Control/Status	(WORD+WORD)
pub const IDT_NT_PCIELCTLSTS: c_uint = 0x00050U;
// PCI Express Device Capabilities 2	(DWORD)
pub const IDT_NT_PCIEDCAP2: c_uint = 0x00064U;
// PCI Express Device Control 2		(WORD+WORD)
pub const IDT_NT_PCIEDCTL2: c_uint = 0x00068U;
// PCI Power Management Control and Status (DWORD)
pub const IDT_NT_PMCSR: c_uint = 0x000C4U;
// ==========================================
// IDT Proprietary NT-port-specific registers
// NT-function main control registers
// NT Endpoint Control			(DWORD)
pub const IDT_NT_NTCTL: c_uint = 0x00400U;
// NT Endpoint Interrupt Status/Mask	(DWORD)
pub const IDT_NT_NTINTSTS: c_uint = 0x00404U;
pub const IDT_NT_NTINTMSK: c_uint = 0x00408U;
// NT Endpoint Signal Data		(DWORD)
pub const IDT_NT_NTSDATA: c_uint = 0x0040CU;
// NT Endpoint Global Signal		(DWORD)
pub const IDT_NT_NTGSIGNAL: c_uint = 0x00410U;
// Internal Error Reporting Mask 0/1	(DWORD)
pub const IDT_NT_NTIERRORMSK0: c_uint = 0x00414U;
pub const IDT_NT_NTIERRORMSK1: c_uint = 0x00418U;
// Doorbel registers
// NT Outbound Doorbell Set		(DWORD)
pub const IDT_NT_OUTDBELLSET: c_uint = 0x00420U;
// NT Inbound Doorbell Status/Mask	(DWORD)
pub const IDT_NT_INDBELLSTS: c_uint = 0x00428U;
pub const IDT_NT_INDBELLMSK: c_uint = 0x0042CU;
// Message registers
// Outbound Message N			(DWORD)
pub const IDT_NT_OUTMSG0: c_uint = 0x00430U;
pub const IDT_NT_OUTMSG1: c_uint = 0x00434U;
pub const IDT_NT_OUTMSG2: c_uint = 0x00438U;
pub const IDT_NT_OUTMSG3: c_uint = 0x0043CU;
// Inbound Message N			(DWORD)
pub const IDT_NT_INMSG0: c_uint = 0x00440U;
pub const IDT_NT_INMSG1: c_uint = 0x00444U;
pub const IDT_NT_INMSG2: c_uint = 0x00448U;
pub const IDT_NT_INMSG3: c_uint = 0x0044CU;
// Inbound Message Source N		(DWORD)
pub const IDT_NT_INMSGSRC0: c_uint = 0x00450U;
pub const IDT_NT_INMSGSRC1: c_uint = 0x00454U;
pub const IDT_NT_INMSGSRC2: c_uint = 0x00458U;
pub const IDT_NT_INMSGSRC3: c_uint = 0x0045CU;
// Message Status			(DWORD)
pub const IDT_NT_MSGSTS: c_uint = 0x00460U;
// Message Status Mask			(DWORD)
pub const IDT_NT_MSGSTSMSK: c_uint = 0x00464U;
// BAR-setup registers
// BAR N Setup/Limit Address/Lower and Upper Translated Base Address (DWORD)
pub const IDT_NT_BARSETUP0: c_uint = 0x00470U;
pub const IDT_NT_BARLIMIT0: c_uint = 0x00474U;
pub const IDT_NT_BARLTBASE0: c_uint = 0x00478U;
pub const IDT_NT_BARUTBASE0: c_uint = 0x0047CU;
pub const IDT_NT_BARSETUP1: c_uint = 0x00480U;
pub const IDT_NT_BARLIMIT1: c_uint = 0x00484U;
pub const IDT_NT_BARLTBASE1: c_uint = 0x00488U;
pub const IDT_NT_BARUTBASE1: c_uint = 0x0048CU;
pub const IDT_NT_BARSETUP2: c_uint = 0x00490U;
pub const IDT_NT_BARLIMIT2: c_uint = 0x00494U;
pub const IDT_NT_BARLTBASE2: c_uint = 0x00498U;
pub const IDT_NT_BARUTBASE2: c_uint = 0x0049CU;
pub const IDT_NT_BARSETUP3: c_uint = 0x004A0U;
pub const IDT_NT_BARLIMIT3: c_uint = 0x004A4U;
pub const IDT_NT_BARLTBASE3: c_uint = 0x004A8U;
pub const IDT_NT_BARUTBASE3: c_uint = 0x004ACU;
pub const IDT_NT_BARSETUP4: c_uint = 0x004B0U;
pub const IDT_NT_BARLIMIT4: c_uint = 0x004B4U;
pub const IDT_NT_BARLTBASE4: c_uint = 0x004B8U;
pub const IDT_NT_BARUTBASE4: c_uint = 0x004BCU;
pub const IDT_NT_BARSETUP5: c_uint = 0x004C0U;
pub const IDT_NT_BARLIMIT5: c_uint = 0x004C4U;
pub const IDT_NT_BARLTBASE5: c_uint = 0x004C8U;
pub const IDT_NT_BARUTBASE5: c_uint = 0x004CCU;
// NT mapping table registers
// NT Mapping Table Address/Status/Data	(DWORD)
pub const IDT_NT_NTMTBLADDR: c_uint = 0x004D0U;
pub const IDT_NT_NTMTBLSTS: c_uint = 0x004D4U;
pub const IDT_NT_NTMTBLDATA: c_uint = 0x004D8U;
// Requester ID (Bus:Device:Function) Capture	(DWORD)
pub const IDT_NT_REQIDCAP: c_uint = 0x004DCU;
// Memory Windows Lookup table registers
// Lookup Table Offset/Lower, Middle and Upper data	(DWORD)
pub const IDT_NT_LUTOFFSET: c_uint = 0x004E0U;
pub const IDT_NT_LUTLDATA: c_uint = 0x004E4U;
pub const IDT_NT_LUTMDATA: c_uint = 0x004E8U;
pub const IDT_NT_LUTUDATA: c_uint = 0x004ECU;
// NT Endpoint Uncorrectable/Correctable Errors Emulation registers (DWORD)
pub const IDT_NT_NTUEEM: c_uint = 0x004F0U;
pub const IDT_NT_NTCEEM: c_uint = 0x004F4U;
// Global Address Space Access/Data registers	(DWARD)
pub const IDT_NT_GASAADDR: c_uint = 0x00FF8U;
pub const IDT_NT_GASADATA: c_uint = 0x00FFCU;
//
// IDT PCIe-switch Global Configuration and Status registers
//
// Port N Configuration register in global space
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP0_PCIECMDSTS: c_uint = 0x01004U;
pub const IDT_SW_NTP0_PCIELCTLSTS: c_uint = 0x01050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP0_NTCTL: c_uint = 0x01400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP0_BARSETUP0: c_uint = 0x01470U;
pub const IDT_SW_NTP0_BARLIMIT0: c_uint = 0x01474U;
pub const IDT_SW_NTP0_BARLTBASE0: c_uint = 0x01478U;
pub const IDT_SW_NTP0_BARUTBASE0: c_uint = 0x0147CU;
pub const IDT_SW_NTP0_BARSETUP1: c_uint = 0x01480U;
pub const IDT_SW_NTP0_BARLIMIT1: c_uint = 0x01484U;
pub const IDT_SW_NTP0_BARLTBASE1: c_uint = 0x01488U;
pub const IDT_SW_NTP0_BARUTBASE1: c_uint = 0x0148CU;
pub const IDT_SW_NTP0_BARSETUP2: c_uint = 0x01490U;
pub const IDT_SW_NTP0_BARLIMIT2: c_uint = 0x01494U;
pub const IDT_SW_NTP0_BARLTBASE2: c_uint = 0x01498U;
pub const IDT_SW_NTP0_BARUTBASE2: c_uint = 0x0149CU;
pub const IDT_SW_NTP0_BARSETUP3: c_uint = 0x014A0U;
pub const IDT_SW_NTP0_BARLIMIT3: c_uint = 0x014A4U;
pub const IDT_SW_NTP0_BARLTBASE3: c_uint = 0x014A8U;
pub const IDT_SW_NTP0_BARUTBASE3: c_uint = 0x014ACU;
pub const IDT_SW_NTP0_BARSETUP4: c_uint = 0x014B0U;
pub const IDT_SW_NTP0_BARLIMIT4: c_uint = 0x014B4U;
pub const IDT_SW_NTP0_BARLTBASE4: c_uint = 0x014B8U;
pub const IDT_SW_NTP0_BARUTBASE4: c_uint = 0x014BCU;
pub const IDT_SW_NTP0_BARSETUP5: c_uint = 0x014C0U;
pub const IDT_SW_NTP0_BARLIMIT5: c_uint = 0x014C4U;
pub const IDT_SW_NTP0_BARLTBASE5: c_uint = 0x014C8U;
pub const IDT_SW_NTP0_BARUTBASE5: c_uint = 0x014CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP2_PCIECMDSTS: c_uint = 0x05004U;
pub const IDT_SW_NTP2_PCIELCTLSTS: c_uint = 0x05050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP2_NTCTL: c_uint = 0x05400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP2_BARSETUP0: c_uint = 0x05470U;
pub const IDT_SW_NTP2_BARLIMIT0: c_uint = 0x05474U;
pub const IDT_SW_NTP2_BARLTBASE0: c_uint = 0x05478U;
pub const IDT_SW_NTP2_BARUTBASE0: c_uint = 0x0547CU;
pub const IDT_SW_NTP2_BARSETUP1: c_uint = 0x05480U;
pub const IDT_SW_NTP2_BARLIMIT1: c_uint = 0x05484U;
pub const IDT_SW_NTP2_BARLTBASE1: c_uint = 0x05488U;
pub const IDT_SW_NTP2_BARUTBASE1: c_uint = 0x0548CU;
pub const IDT_SW_NTP2_BARSETUP2: c_uint = 0x05490U;
pub const IDT_SW_NTP2_BARLIMIT2: c_uint = 0x05494U;
pub const IDT_SW_NTP2_BARLTBASE2: c_uint = 0x05498U;
pub const IDT_SW_NTP2_BARUTBASE2: c_uint = 0x0549CU;
pub const IDT_SW_NTP2_BARSETUP3: c_uint = 0x054A0U;
pub const IDT_SW_NTP2_BARLIMIT3: c_uint = 0x054A4U;
pub const IDT_SW_NTP2_BARLTBASE3: c_uint = 0x054A8U;
pub const IDT_SW_NTP2_BARUTBASE3: c_uint = 0x054ACU;
pub const IDT_SW_NTP2_BARSETUP4: c_uint = 0x054B0U;
pub const IDT_SW_NTP2_BARLIMIT4: c_uint = 0x054B4U;
pub const IDT_SW_NTP2_BARLTBASE4: c_uint = 0x054B8U;
pub const IDT_SW_NTP2_BARUTBASE4: c_uint = 0x054BCU;
pub const IDT_SW_NTP2_BARSETUP5: c_uint = 0x054C0U;
pub const IDT_SW_NTP2_BARLIMIT5: c_uint = 0x054C4U;
pub const IDT_SW_NTP2_BARLTBASE5: c_uint = 0x054C8U;
pub const IDT_SW_NTP2_BARUTBASE5: c_uint = 0x054CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP4_PCIECMDSTS: c_uint = 0x09004U;
pub const IDT_SW_NTP4_PCIELCTLSTS: c_uint = 0x09050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP4_NTCTL: c_uint = 0x09400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP4_BARSETUP0: c_uint = 0x09470U;
pub const IDT_SW_NTP4_BARLIMIT0: c_uint = 0x09474U;
pub const IDT_SW_NTP4_BARLTBASE0: c_uint = 0x09478U;
pub const IDT_SW_NTP4_BARUTBASE0: c_uint = 0x0947CU;
pub const IDT_SW_NTP4_BARSETUP1: c_uint = 0x09480U;
pub const IDT_SW_NTP4_BARLIMIT1: c_uint = 0x09484U;
pub const IDT_SW_NTP4_BARLTBASE1: c_uint = 0x09488U;
pub const IDT_SW_NTP4_BARUTBASE1: c_uint = 0x0948CU;
pub const IDT_SW_NTP4_BARSETUP2: c_uint = 0x09490U;
pub const IDT_SW_NTP4_BARLIMIT2: c_uint = 0x09494U;
pub const IDT_SW_NTP4_BARLTBASE2: c_uint = 0x09498U;
pub const IDT_SW_NTP4_BARUTBASE2: c_uint = 0x0949CU;
pub const IDT_SW_NTP4_BARSETUP3: c_uint = 0x094A0U;
pub const IDT_SW_NTP4_BARLIMIT3: c_uint = 0x094A4U;
pub const IDT_SW_NTP4_BARLTBASE3: c_uint = 0x094A8U;
pub const IDT_SW_NTP4_BARUTBASE3: c_uint = 0x094ACU;
pub const IDT_SW_NTP4_BARSETUP4: c_uint = 0x094B0U;
pub const IDT_SW_NTP4_BARLIMIT4: c_uint = 0x094B4U;
pub const IDT_SW_NTP4_BARLTBASE4: c_uint = 0x094B8U;
pub const IDT_SW_NTP4_BARUTBASE4: c_uint = 0x094BCU;
pub const IDT_SW_NTP4_BARSETUP5: c_uint = 0x094C0U;
pub const IDT_SW_NTP4_BARLIMIT5: c_uint = 0x094C4U;
pub const IDT_SW_NTP4_BARLTBASE5: c_uint = 0x094C8U;
pub const IDT_SW_NTP4_BARUTBASE5: c_uint = 0x094CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP6_PCIECMDSTS: c_uint = 0x0D004U;
pub const IDT_SW_NTP6_PCIELCTLSTS: c_uint = 0x0D050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP6_NTCTL: c_uint = 0x0D400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP6_BARSETUP0: c_uint = 0x0D470U;
pub const IDT_SW_NTP6_BARLIMIT0: c_uint = 0x0D474U;
pub const IDT_SW_NTP6_BARLTBASE0: c_uint = 0x0D478U;
pub const IDT_SW_NTP6_BARUTBASE0: c_uint = 0x0D47CU;
pub const IDT_SW_NTP6_BARSETUP1: c_uint = 0x0D480U;
pub const IDT_SW_NTP6_BARLIMIT1: c_uint = 0x0D484U;
pub const IDT_SW_NTP6_BARLTBASE1: c_uint = 0x0D488U;
pub const IDT_SW_NTP6_BARUTBASE1: c_uint = 0x0D48CU;
pub const IDT_SW_NTP6_BARSETUP2: c_uint = 0x0D490U;
pub const IDT_SW_NTP6_BARLIMIT2: c_uint = 0x0D494U;
pub const IDT_SW_NTP6_BARLTBASE2: c_uint = 0x0D498U;
pub const IDT_SW_NTP6_BARUTBASE2: c_uint = 0x0D49CU;
pub const IDT_SW_NTP6_BARSETUP3: c_uint = 0x0D4A0U;
pub const IDT_SW_NTP6_BARLIMIT3: c_uint = 0x0D4A4U;
pub const IDT_SW_NTP6_BARLTBASE3: c_uint = 0x0D4A8U;
pub const IDT_SW_NTP6_BARUTBASE3: c_uint = 0x0D4ACU;
pub const IDT_SW_NTP6_BARSETUP4: c_uint = 0x0D4B0U;
pub const IDT_SW_NTP6_BARLIMIT4: c_uint = 0x0D4B4U;
pub const IDT_SW_NTP6_BARLTBASE4: c_uint = 0x0D4B8U;
pub const IDT_SW_NTP6_BARUTBASE4: c_uint = 0x0D4BCU;
pub const IDT_SW_NTP6_BARSETUP5: c_uint = 0x0D4C0U;
pub const IDT_SW_NTP6_BARLIMIT5: c_uint = 0x0D4C4U;
pub const IDT_SW_NTP6_BARLTBASE5: c_uint = 0x0D4C8U;
pub const IDT_SW_NTP6_BARUTBASE5: c_uint = 0x0D4CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP8_PCIECMDSTS: c_uint = 0x11004U;
pub const IDT_SW_NTP8_PCIELCTLSTS: c_uint = 0x11050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP8_NTCTL: c_uint = 0x11400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP8_BARSETUP0: c_uint = 0x11470U;
pub const IDT_SW_NTP8_BARLIMIT0: c_uint = 0x11474U;
pub const IDT_SW_NTP8_BARLTBASE0: c_uint = 0x11478U;
pub const IDT_SW_NTP8_BARUTBASE0: c_uint = 0x1147CU;
pub const IDT_SW_NTP8_BARSETUP1: c_uint = 0x11480U;
pub const IDT_SW_NTP8_BARLIMIT1: c_uint = 0x11484U;
pub const IDT_SW_NTP8_BARLTBASE1: c_uint = 0x11488U;
pub const IDT_SW_NTP8_BARUTBASE1: c_uint = 0x1148CU;
pub const IDT_SW_NTP8_BARSETUP2: c_uint = 0x11490U;
pub const IDT_SW_NTP8_BARLIMIT2: c_uint = 0x11494U;
pub const IDT_SW_NTP8_BARLTBASE2: c_uint = 0x11498U;
pub const IDT_SW_NTP8_BARUTBASE2: c_uint = 0x1149CU;
pub const IDT_SW_NTP8_BARSETUP3: c_uint = 0x114A0U;
pub const IDT_SW_NTP8_BARLIMIT3: c_uint = 0x114A4U;
pub const IDT_SW_NTP8_BARLTBASE3: c_uint = 0x114A8U;
pub const IDT_SW_NTP8_BARUTBASE3: c_uint = 0x114ACU;
pub const IDT_SW_NTP8_BARSETUP4: c_uint = 0x114B0U;
pub const IDT_SW_NTP8_BARLIMIT4: c_uint = 0x114B4U;
pub const IDT_SW_NTP8_BARLTBASE4: c_uint = 0x114B8U;
pub const IDT_SW_NTP8_BARUTBASE4: c_uint = 0x114BCU;
pub const IDT_SW_NTP8_BARSETUP5: c_uint = 0x114C0U;
pub const IDT_SW_NTP8_BARLIMIT5: c_uint = 0x114C4U;
pub const IDT_SW_NTP8_BARLTBASE5: c_uint = 0x114C8U;
pub const IDT_SW_NTP8_BARUTBASE5: c_uint = 0x114CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP12_PCIECMDSTS: c_uint = 0x19004U;
pub const IDT_SW_NTP12_PCIELCTLSTS: c_uint = 0x19050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP12_NTCTL: c_uint = 0x19400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP12_BARSETUP0: c_uint = 0x19470U;
pub const IDT_SW_NTP12_BARLIMIT0: c_uint = 0x19474U;
pub const IDT_SW_NTP12_BARLTBASE0: c_uint = 0x19478U;
pub const IDT_SW_NTP12_BARUTBASE0: c_uint = 0x1947CU;
pub const IDT_SW_NTP12_BARSETUP1: c_uint = 0x19480U;
pub const IDT_SW_NTP12_BARLIMIT1: c_uint = 0x19484U;
pub const IDT_SW_NTP12_BARLTBASE1: c_uint = 0x19488U;
pub const IDT_SW_NTP12_BARUTBASE1: c_uint = 0x1948CU;
pub const IDT_SW_NTP12_BARSETUP2: c_uint = 0x19490U;
pub const IDT_SW_NTP12_BARLIMIT2: c_uint = 0x19494U;
pub const IDT_SW_NTP12_BARLTBASE2: c_uint = 0x19498U;
pub const IDT_SW_NTP12_BARUTBASE2: c_uint = 0x1949CU;
pub const IDT_SW_NTP12_BARSETUP3: c_uint = 0x194A0U;
pub const IDT_SW_NTP12_BARLIMIT3: c_uint = 0x194A4U;
pub const IDT_SW_NTP12_BARLTBASE3: c_uint = 0x194A8U;
pub const IDT_SW_NTP12_BARUTBASE3: c_uint = 0x194ACU;
pub const IDT_SW_NTP12_BARSETUP4: c_uint = 0x194B0U;
pub const IDT_SW_NTP12_BARLIMIT4: c_uint = 0x194B4U;
pub const IDT_SW_NTP12_BARLTBASE4: c_uint = 0x194B8U;
pub const IDT_SW_NTP12_BARUTBASE4: c_uint = 0x194BCU;
pub const IDT_SW_NTP12_BARSETUP5: c_uint = 0x194C0U;
pub const IDT_SW_NTP12_BARLIMIT5: c_uint = 0x194C4U;
pub const IDT_SW_NTP12_BARLTBASE5: c_uint = 0x194C8U;
pub const IDT_SW_NTP12_BARUTBASE5: c_uint = 0x194CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP16_PCIECMDSTS: c_uint = 0x21004U;
pub const IDT_SW_NTP16_PCIELCTLSTS: c_uint = 0x21050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP16_NTCTL: c_uint = 0x21400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP16_BARSETUP0: c_uint = 0x21470U;
pub const IDT_SW_NTP16_BARLIMIT0: c_uint = 0x21474U;
pub const IDT_SW_NTP16_BARLTBASE0: c_uint = 0x21478U;
pub const IDT_SW_NTP16_BARUTBASE0: c_uint = 0x2147CU;
pub const IDT_SW_NTP16_BARSETUP1: c_uint = 0x21480U;
pub const IDT_SW_NTP16_BARLIMIT1: c_uint = 0x21484U;
pub const IDT_SW_NTP16_BARLTBASE1: c_uint = 0x21488U;
pub const IDT_SW_NTP16_BARUTBASE1: c_uint = 0x2148CU;
pub const IDT_SW_NTP16_BARSETUP2: c_uint = 0x21490U;
pub const IDT_SW_NTP16_BARLIMIT2: c_uint = 0x21494U;
pub const IDT_SW_NTP16_BARLTBASE2: c_uint = 0x21498U;
pub const IDT_SW_NTP16_BARUTBASE2: c_uint = 0x2149CU;
pub const IDT_SW_NTP16_BARSETUP3: c_uint = 0x214A0U;
pub const IDT_SW_NTP16_BARLIMIT3: c_uint = 0x214A4U;
pub const IDT_SW_NTP16_BARLTBASE3: c_uint = 0x214A8U;
pub const IDT_SW_NTP16_BARUTBASE3: c_uint = 0x214ACU;
pub const IDT_SW_NTP16_BARSETUP4: c_uint = 0x214B0U;
pub const IDT_SW_NTP16_BARLIMIT4: c_uint = 0x214B4U;
pub const IDT_SW_NTP16_BARLTBASE4: c_uint = 0x214B8U;
pub const IDT_SW_NTP16_BARUTBASE4: c_uint = 0x214BCU;
pub const IDT_SW_NTP16_BARSETUP5: c_uint = 0x214C0U;
pub const IDT_SW_NTP16_BARLIMIT5: c_uint = 0x214C4U;
pub const IDT_SW_NTP16_BARLTBASE5: c_uint = 0x214C8U;
pub const IDT_SW_NTP16_BARUTBASE5: c_uint = 0x214CCU;
// PCI Express command/status and link control/status registers (WORD+WORD)
pub const IDT_SW_NTP20_PCIECMDSTS: c_uint = 0x29004U;
pub const IDT_SW_NTP20_PCIELCTLSTS: c_uint = 0x29050U;
// NT-function control register		(DWORD)
pub const IDT_SW_NTP20_NTCTL: c_uint = 0x29400U;
// BAR setup/limit/base address registers (DWORD)
pub const IDT_SW_NTP20_BARSETUP0: c_uint = 0x29470U;
pub const IDT_SW_NTP20_BARLIMIT0: c_uint = 0x29474U;
pub const IDT_SW_NTP20_BARLTBASE0: c_uint = 0x29478U;
pub const IDT_SW_NTP20_BARUTBASE0: c_uint = 0x2947CU;
pub const IDT_SW_NTP20_BARSETUP1: c_uint = 0x29480U;
pub const IDT_SW_NTP20_BARLIMIT1: c_uint = 0x29484U;
pub const IDT_SW_NTP20_BARLTBASE1: c_uint = 0x29488U;
pub const IDT_SW_NTP20_BARUTBASE1: c_uint = 0x2948CU;
pub const IDT_SW_NTP20_BARSETUP2: c_uint = 0x29490U;
pub const IDT_SW_NTP20_BARLIMIT2: c_uint = 0x29494U;
pub const IDT_SW_NTP20_BARLTBASE2: c_uint = 0x29498U;
pub const IDT_SW_NTP20_BARUTBASE2: c_uint = 0x2949CU;
pub const IDT_SW_NTP20_BARSETUP3: c_uint = 0x294A0U;
pub const IDT_SW_NTP20_BARLIMIT3: c_uint = 0x294A4U;
pub const IDT_SW_NTP20_BARLTBASE3: c_uint = 0x294A8U;
pub const IDT_SW_NTP20_BARUTBASE3: c_uint = 0x294ACU;
pub const IDT_SW_NTP20_BARSETUP4: c_uint = 0x294B0U;
pub const IDT_SW_NTP20_BARLIMIT4: c_uint = 0x294B4U;
pub const IDT_SW_NTP20_BARLTBASE4: c_uint = 0x294B8U;
pub const IDT_SW_NTP20_BARUTBASE4: c_uint = 0x294BCU;
pub const IDT_SW_NTP20_BARSETUP5: c_uint = 0x294C0U;
pub const IDT_SW_NTP20_BARLIMIT5: c_uint = 0x294C4U;
pub const IDT_SW_NTP20_BARLTBASE5: c_uint = 0x294C8U;
pub const IDT_SW_NTP20_BARUTBASE5: c_uint = 0x294CCU;
// IDT PCIe-switch control register	(DWORD)
pub const IDT_SW_CTL: c_uint = 0x3E000U;
// Boot Configuration Vector Status	(DWORD)
pub const IDT_SW_BCVSTS: c_uint = 0x3E004U;
// Port Clocking Mode			(DWORD)
pub const IDT_SW_PCLKMODE: c_uint = 0x3E008U;
// Reset Drain Delay			(DWORD)
pub const IDT_SW_RDRAINDELAY: c_uint = 0x3E080U;
// Port Operating Mode Change Drain Delay (DWORD)
pub const IDT_SW_POMCDELAY: c_uint = 0x3E084U;
// Side Effect Delay			(DWORD)
pub const IDT_SW_SEDELAY: c_uint = 0x3E088U;
// Upstream Secondary Bus Reset Delay	(DWORD)
pub const IDT_SW_SSBRDELAY: c_uint = 0x3E08CU;
// Switch partition N Control/Status/Failover registers
pub const IDT_SW_SWPART0CTL: c_uint = 0x3E100U;
pub const IDT_SW_SWPART0STS: c_uint = 0x3E104U;
pub const IDT_SW_SWPART0FCTL: c_uint = 0x3E108U;
pub const IDT_SW_SWPART1CTL: c_uint = 0x3E120U;
pub const IDT_SW_SWPART1STS: c_uint = 0x3E124U;
pub const IDT_SW_SWPART1FCTL: c_uint = 0x3E128U;
pub const IDT_SW_SWPART2CTL: c_uint = 0x3E140U;
pub const IDT_SW_SWPART2STS: c_uint = 0x3E144U;
pub const IDT_SW_SWPART2FCTL: c_uint = 0x3E148U;
pub const IDT_SW_SWPART3CTL: c_uint = 0x3E160U;
pub const IDT_SW_SWPART3STS: c_uint = 0x3E164U;
pub const IDT_SW_SWPART3FCTL: c_uint = 0x3E168U;
pub const IDT_SW_SWPART4CTL: c_uint = 0x3E180U;
pub const IDT_SW_SWPART4STS: c_uint = 0x3E184U;
pub const IDT_SW_SWPART4FCTL: c_uint = 0x3E188U;
pub const IDT_SW_SWPART5CTL: c_uint = 0x3E1A0U;
pub const IDT_SW_SWPART5STS: c_uint = 0x3E1A4U;
pub const IDT_SW_SWPART5FCTL: c_uint = 0x3E1A8U;
pub const IDT_SW_SWPART6CTL: c_uint = 0x3E1C0U;
pub const IDT_SW_SWPART6STS: c_uint = 0x3E1C4U;
pub const IDT_SW_SWPART6FCTL: c_uint = 0x3E1C8U;
pub const IDT_SW_SWPART7CTL: c_uint = 0x3E1E0U;
pub const IDT_SW_SWPART7STS: c_uint = 0x3E1E4U;
pub const IDT_SW_SWPART7FCTL: c_uint = 0x3E1E8U;
// Switch port N control and status registers
pub const IDT_SW_SWPORT0CTL: c_uint = 0x3E200U;
pub const IDT_SW_SWPORT0STS: c_uint = 0x3E204U;
pub const IDT_SW_SWPORT0FCTL: c_uint = 0x3E208U;
pub const IDT_SW_SWPORT2CTL: c_uint = 0x3E240U;
pub const IDT_SW_SWPORT2STS: c_uint = 0x3E244U;
pub const IDT_SW_SWPORT2FCTL: c_uint = 0x3E248U;
pub const IDT_SW_SWPORT4CTL: c_uint = 0x3E280U;
pub const IDT_SW_SWPORT4STS: c_uint = 0x3E284U;
pub const IDT_SW_SWPORT4FCTL: c_uint = 0x3E288U;
pub const IDT_SW_SWPORT6CTL: c_uint = 0x3E2C0U;
pub const IDT_SW_SWPORT6STS: c_uint = 0x3E2C4U;
pub const IDT_SW_SWPORT6FCTL: c_uint = 0x3E2C8U;
pub const IDT_SW_SWPORT8CTL: c_uint = 0x3E300U;
pub const IDT_SW_SWPORT8STS: c_uint = 0x3E304U;
pub const IDT_SW_SWPORT8FCTL: c_uint = 0x3E308U;
pub const IDT_SW_SWPORT12CTL: c_uint = 0x3E380U;
pub const IDT_SW_SWPORT12STS: c_uint = 0x3E384U;
pub const IDT_SW_SWPORT12FCTL: c_uint = 0x3E388U;
pub const IDT_SW_SWPORT16CTL: c_uint = 0x3E400U;
pub const IDT_SW_SWPORT16STS: c_uint = 0x3E404U;
pub const IDT_SW_SWPORT16FCTL: c_uint = 0x3E408U;
pub const IDT_SW_SWPORT20CTL: c_uint = 0x3E480U;
pub const IDT_SW_SWPORT20STS: c_uint = 0x3E484U;
pub const IDT_SW_SWPORT20FCTL: c_uint = 0x3E488U;
// Switch Event registers
// Switch Event Status/Mask/Partition mask (DWORD)
pub const IDT_SW_SESTS: c_uint = 0x3EC00U;
pub const IDT_SW_SEMSK: c_uint = 0x3EC04U;
pub const IDT_SW_SEPMSK: c_uint = 0x3EC08U;
// Switch Event Link Up/Down Status/Mask (DWORD)
pub const IDT_SW_SELINKUPSTS: c_uint = 0x3EC0CU;
pub const IDT_SW_SELINKUPMSK: c_uint = 0x3EC10U;
pub const IDT_SW_SELINKDNSTS: c_uint = 0x3EC14U;
pub const IDT_SW_SELINKDNMSK: c_uint = 0x3EC18U;
// Switch Event Fundamental Reset Status/Mask (DWORD)
pub const IDT_SW_SEFRSTSTS: c_uint = 0x3EC1CU;
pub const IDT_SW_SEFRSTMSK: c_uint = 0x3EC20U;
// Switch Event Hot Reset Status/Mask	(DWORD)
pub const IDT_SW_SEHRSTSTS: c_uint = 0x3EC24U;
pub const IDT_SW_SEHRSTMSK: c_uint = 0x3EC28U;
// Switch Event Failover Mask		(DWORD)
pub const IDT_SW_SEFOVRMSK: c_uint = 0x3EC2CU;
// Switch Event Global Signal Status/Mask (DWORD)
pub const IDT_SW_SEGSIGSTS: c_uint = 0x3EC30U;
pub const IDT_SW_SEGSIGMSK: c_uint = 0x3EC34U;
// NT Global Doorbell Status		(DWORD)
pub const IDT_SW_GDBELLSTS: c_uint = 0x3EC3CU;
// Switch partition N message M control (msgs routing table) (DWORD)
pub const IDT_SW_SWP0MSGCTL0: c_uint = 0x3EE00U;
pub const IDT_SW_SWP1MSGCTL0: c_uint = 0x3EE04U;
pub const IDT_SW_SWP2MSGCTL0: c_uint = 0x3EE08U;
pub const IDT_SW_SWP3MSGCTL0: c_uint = 0x3EE0CU;
pub const IDT_SW_SWP4MSGCTL0: c_uint = 0x3EE10U;
pub const IDT_SW_SWP5MSGCTL0: c_uint = 0x3EE14U;
pub const IDT_SW_SWP6MSGCTL0: c_uint = 0x3EE18U;
pub const IDT_SW_SWP7MSGCTL0: c_uint = 0x3EE1CU;
pub const IDT_SW_SWP0MSGCTL1: c_uint = 0x3EE20U;
pub const IDT_SW_SWP1MSGCTL1: c_uint = 0x3EE24U;
pub const IDT_SW_SWP2MSGCTL1: c_uint = 0x3EE28U;
pub const IDT_SW_SWP3MSGCTL1: c_uint = 0x3EE2CU;
pub const IDT_SW_SWP4MSGCTL1: c_uint = 0x3EE30U;
pub const IDT_SW_SWP5MSGCTL1: c_uint = 0x3EE34U;
pub const IDT_SW_SWP6MSGCTL1: c_uint = 0x3EE38U;
pub const IDT_SW_SWP7MSGCTL1: c_uint = 0x3EE3CU;
pub const IDT_SW_SWP0MSGCTL2: c_uint = 0x3EE40U;
pub const IDT_SW_SWP1MSGCTL2: c_uint = 0x3EE44U;
pub const IDT_SW_SWP2MSGCTL2: c_uint = 0x3EE48U;
pub const IDT_SW_SWP3MSGCTL2: c_uint = 0x3EE4CU;
pub const IDT_SW_SWP4MSGCTL2: c_uint = 0x3EE50U;
pub const IDT_SW_SWP5MSGCTL2: c_uint = 0x3EE54U;
pub const IDT_SW_SWP6MSGCTL2: c_uint = 0x3EE58U;
pub const IDT_SW_SWP7MSGCTL2: c_uint = 0x3EE5CU;
pub const IDT_SW_SWP0MSGCTL3: c_uint = 0x3EE60U;
pub const IDT_SW_SWP1MSGCTL3: c_uint = 0x3EE64U;
pub const IDT_SW_SWP2MSGCTL3: c_uint = 0x3EE68U;
pub const IDT_SW_SWP3MSGCTL3: c_uint = 0x3EE6CU;
pub const IDT_SW_SWP4MSGCTL3: c_uint = 0x3EE70U;
pub const IDT_SW_SWP5MSGCTL3: c_uint = 0x3EE74U;
pub const IDT_SW_SWP6MSGCTL3: c_uint = 0x3EE78U;
pub const IDT_SW_SWP7MSGCTL3: c_uint = 0x3EE7CU;
// SMBus Status and Control registers	(DWORD)
pub const IDT_SW_SMBUSSTS: c_uint = 0x3F188U;
pub const IDT_SW_SMBUSCTL: c_uint = 0x3F18CU;
// Serial EEPROM Interface		(DWORD)
pub const IDT_SW_EEPROMINTF: c_uint = 0x3F190U;
// MBus I/O Expander Address N		(DWORD)
pub const IDT_SW_IOEXPADDR0: c_uint = 0x3F198U;
pub const IDT_SW_IOEXPADDR1: c_uint = 0x3F19CU;
pub const IDT_SW_IOEXPADDR2: c_uint = 0x3F1A0U;
pub const IDT_SW_IOEXPADDR3: c_uint = 0x3F1A4U;
pub const IDT_SW_IOEXPADDR4: c_uint = 0x3F1A8U;
pub const IDT_SW_IOEXPADDR5: c_uint = 0x3F1ACU;
// General Purpose Events Control and Status registers (DWORD)
pub const IDT_SW_GPECTL: c_uint = 0x3F1B0U;
pub const IDT_SW_GPESTS: c_uint = 0x3F1B4U;
// Temperature sensor Control/Status/Alarm/Adjustment/Slope registers
pub const IDT_SW_TMPCTL: c_uint = 0x3F1D4U;
pub const IDT_SW_TMPSTS: c_uint = 0x3F1D8U;
pub const IDT_SW_TMPALARM: c_uint = 0x3F1DCU;
pub const IDT_SW_TMPADJ: c_uint = 0x3F1E0U;
pub const IDT_SW_TSSLOPE: c_uint = 0x3F1E4U;
// SMBus Configuration Block header log	(DWORD)
pub const IDT_SW_SMBUSCBHL: c_uint = 0x3F1E8U;
//
// Common registers related constants
// @IDT_REG_ALIGN:	Registers alignment used in the driver
// @IDT_REG_PCI_MAX:	Maximum PCI configuration space register value
// @IDT_REG_SW_MAX:	Maximum global register value
//
pub const IDT_REG_ALIGN: c_int = 4;
pub const IDT_REG_PCI_MAX: c_uint = 0x00FFFU;
pub const IDT_REG_SW_MAX: c_uint = 0x3FFFFU;
//
// PCICMDSTS register fields related constants
// @IDT_PCICMDSTS_IOAE:	I/O access enable
// @IDT_PCICMDSTS_MAE:	Memory access enable
// @IDT_PCICMDSTS_BME:	Bus master enable
//
pub const IDT_PCICMDSTS_IOAE: c_uint = 0x00000001U;
pub const IDT_PCICMDSTS_MAE: c_uint = 0x00000002U;
pub const IDT_PCICMDSTS_BME: c_uint = 0x00000004U;
//
// PCIEDCAP register fields related constants
// @IDT_PCIEDCAP_MPAYLOAD_MASK:	 Maximum payload size mask
// @IDT_PCIEDCAP_MPAYLOAD_FLD:	 Maximum payload size field offset
// @IDT_PCIEDCAP_MPAYLOAD_S128:	 Max supported payload size of 128 bytes
// @IDT_PCIEDCAP_MPAYLOAD_S256:	 Max supported payload size of 256 bytes
// @IDT_PCIEDCAP_MPAYLOAD_S512:	 Max supported payload size of 512 bytes
// @IDT_PCIEDCAP_MPAYLOAD_S1024: Max supported payload size of 1024 bytes
// @IDT_PCIEDCAP_MPAYLOAD_S2048: Max supported payload size of 2048 bytes
//
pub const IDT_PCIEDCAP_MPAYLOAD_MASK: c_uint = 0x00000007U;
pub const IDT_PCIEDCAP_MPAYLOAD_FLD: c_int = 0;
pub const IDT_PCIEDCAP_MPAYLOAD_S128: c_uint = 0x00000000U;
pub const IDT_PCIEDCAP_MPAYLOAD_S256: c_uint = 0x00000001U;
pub const IDT_PCIEDCAP_MPAYLOAD_S512: c_uint = 0x00000002U;
pub const IDT_PCIEDCAP_MPAYLOAD_S1024: c_uint = 0x00000003U;
pub const IDT_PCIEDCAP_MPAYLOAD_S2048: c_uint = 0x00000004U;
//
// PCIEDCTLSTS registers fields related constants
// @IDT_PCIEDCTL_MPS_MASK:	Maximum payload size mask
// @IDT_PCIEDCTL_MPS_FLD:	MPS field offset
// @IDT_PCIEDCTL_MPS_S128:	Max payload size of 128 bytes
// @IDT_PCIEDCTL_MPS_S256:	Max payload size of 256 bytes
// @IDT_PCIEDCTL_MPS_S512:	Max payload size of 512 bytes
// @IDT_PCIEDCTL_MPS_S1024:	Max payload size of 1024 bytes
// @IDT_PCIEDCTL_MPS_S2048:	Max payload size of 2048 bytes
// @IDT_PCIEDCTL_MPS_S4096:	Max payload size of 4096 bytes
//
pub const IDT_PCIEDCTLSTS_MPS_MASK: c_uint = 0x000000E0U;
pub const IDT_PCIEDCTLSTS_MPS_FLD: c_int = 5;
pub const IDT_PCIEDCTLSTS_MPS_S128: c_uint = 0x00000000U;
pub const IDT_PCIEDCTLSTS_MPS_S256: c_uint = 0x00000020U;
pub const IDT_PCIEDCTLSTS_MPS_S512: c_uint = 0x00000040U;
pub const IDT_PCIEDCTLSTS_MPS_S1024: c_uint = 0x00000060U;
pub const IDT_PCIEDCTLSTS_MPS_S2048: c_uint = 0x00000080U;
pub const IDT_PCIEDCTLSTS_MPS_S4096: c_uint = 0x000000A0U;
//
// PCIELCAP register fields related constants
// @IDT_PCIELCAP_PORTNUM_MASK:	Port number field mask
// @IDT_PCIELCAP_PORTNUM_FLD:	Port number field offset
//
pub const IDT_PCIELCAP_PORTNUM_MASK: c_uint = 0xFF000000U;
pub const IDT_PCIELCAP_PORTNUM_FLD: c_int = 24;
//
// PCIELCTLSTS registers fields related constants
// @IDT_PCIELSTS_CLS_MASK:	Current link speed mask
// @IDT_PCIELSTS_CLS_FLD:	Current link speed field offset
// @IDT_PCIELSTS_NLW_MASK:	Negotiated link width mask
// @IDT_PCIELSTS_NLW_FLD:	Negotiated link width field offset
// @IDT_PCIELSTS_SCLK_COM:	Common slot clock configuration
//
pub const IDT_PCIELCTLSTS_CLS_MASK: c_uint = 0x000F0000U;
pub const IDT_PCIELCTLSTS_CLS_FLD: c_int = 16;
pub const IDT_PCIELCTLSTS_NLW_MASK: c_uint = 0x03F00000U;
pub const IDT_PCIELCTLSTS_NLW_FLD: c_int = 20;
pub const IDT_PCIELCTLSTS_SCLK_COM: c_uint = 0x10000000U;
//
// NTCTL register fields related constants
// @IDT_NTCTL_IDPROTDIS:	ID Protection check disable (disable MTBL)
// @IDT_NTCTL_CPEN:		Completion enable
// @IDT_NTCTL_RNS:		Request no snoop processing (if MTBL disabled)
// @IDT_NTCTL_ATP:		Address type processing (if MTBL disabled)
//
pub const IDT_NTCTL_IDPROTDIS: c_uint = 0x00000001U;
pub const IDT_NTCTL_CPEN: c_uint = 0x00000002U;
pub const IDT_NTCTL_RNS: c_uint = 0x00000004U;
pub const IDT_NTCTL_ATP: c_uint = 0x00000008U;
//
// NTINTSTS register fields related constants
// @IDT_NTINTSTS_MSG:		Message interrupt bit
// @IDT_NTINTSTS_DBELL:		Doorbell interrupt bit
// @IDT_NTINTSTS_SEVENT:	Switch Event interrupt bit
// @IDT_NTINTSTS_TMPSENSOR:	Temperature sensor interrupt bit
//
pub const IDT_NTINTSTS_MSG: c_uint = 0x00000001U;
pub const IDT_NTINTSTS_DBELL: c_uint = 0x00000002U;
pub const IDT_NTINTSTS_SEVENT: c_uint = 0x00000008U;
pub const IDT_NTINTSTS_TMPSENSOR: c_uint = 0x00000080U;
//
// NTINTMSK register fields related constants
// @IDT_NTINTMSK_MSG:		Message interrupt mask bit
// @IDT_NTINTMSK_DBELL:		Doorbell interrupt mask bit
// @IDT_NTINTMSK_SEVENT:	Switch Event interrupt mask bit
// @IDT_NTINTMSK_TMPSENSOR:	Temperature sensor interrupt mask bit
// @IDT_NTINTMSK_ALL:		NTB-related interrupts mask
//
pub const IDT_NTINTMSK_MSG: c_uint = 0x00000001U;
pub const IDT_NTINTMSK_DBELL: c_uint = 0x00000002U;
pub const IDT_NTINTMSK_SEVENT: c_uint = 0x00000008U;
pub const IDT_NTINTMSK_TMPSENSOR: c_uint = 0x00000080U;

//
// NTGSIGNAL register fields related constants
// @IDT_NTGSIGNAL_SET:	Set global signal of the local partition
//
pub const IDT_NTGSIGNAL_SET: c_uint = 0x00000001U;
//
// BARSETUP register fields related constants
// @IDT_BARSETUP_TYPE_MASK:	Mask of the TYPE field
// @IDT_BARSETUP_TYPE_32:	32-bit addressing BAR
// @IDT_BARSETUP_TYPE_64:	64-bit addressing BAR
// @IDT_BARSETUP_PREF:		Value of the BAR prefetchable field
// @IDT_BARSETUP_SIZE_MASK:	Mask of the SIZE field
// @IDT_BARSETUP_SIZE_FLD:	SIZE field offset
// @IDT_BARSETUP_SIZE_CFG:	SIZE field value in case of config space MODE
// @IDT_BARSETUP_MODE_CFG:	Configuration space BAR mode
// @IDT_BARSETUP_ATRAN_MASK:	ATRAN field mask
// @IDT_BARSETUP_ATRAN_FLD:	ATRAN field offset
// @IDT_BARSETUP_ATRAN_DIR:	Direct address translation memory window
// @IDT_BARSETUP_ATRAN_LUT12:	12-entry lookup table
// @IDT_BARSETUP_ATRAN_LUT24:	24-entry lookup table
// @IDT_BARSETUP_TPART_MASK:	TPART field mask
// @IDT_BARSETUP_TPART_FLD:	TPART field offset
// @IDT_BARSETUP_EN:		BAR enable bit
//
pub const IDT_BARSETUP_TYPE_MASK: c_uint = 0x00000006U;
pub const IDT_BARSETUP_TYPE_FLD: c_int = 0;
pub const IDT_BARSETUP_TYPE_32: c_uint = 0x00000000U;
pub const IDT_BARSETUP_TYPE_64: c_uint = 0x00000004U;
pub const IDT_BARSETUP_PREF: c_uint = 0x00000008U;
pub const IDT_BARSETUP_SIZE_MASK: c_uint = 0x000003F0U;
pub const IDT_BARSETUP_SIZE_FLD: c_int = 4;
pub const IDT_BARSETUP_SIZE_CFG: c_uint = 0x000000C0U;
pub const IDT_BARSETUP_MODE_CFG: c_uint = 0x00000400U;
pub const IDT_BARSETUP_ATRAN_MASK: c_uint = 0x00001800U;
pub const IDT_BARSETUP_ATRAN_FLD: c_int = 11;
pub const IDT_BARSETUP_ATRAN_DIR: c_uint = 0x00000000U;
pub const IDT_BARSETUP_ATRAN_LUT12: c_uint = 0x00000800U;
pub const IDT_BARSETUP_ATRAN_LUT24: c_uint = 0x00001000U;
pub const IDT_BARSETUP_TPART_MASK: c_uint = 0x0000E000U;
pub const IDT_BARSETUP_TPART_FLD: c_int = 13;
pub const IDT_BARSETUP_EN: c_uint = 0x80000000U;
//
// NTMTBLDATA register fields related constants
// @IDT_NTMTBLDATA_VALID:	Set the MTBL entry being valid
// @IDT_NTMTBLDATA_REQID_MASK:	Bus:Device:Function field mask
// @IDT_NTMTBLDATA_REQID_FLD:	Bus:Device:Function field offset
// @IDT_NTMTBLDATA_PART_MASK:	Partition field mask
// @IDT_NTMTBLDATA_PART_FLD:	Partition field offset
// @IDT_NTMTBLDATA_ATP_TRANS:	Enable AT field translation on request TLPs
// @IDT_NTMTBLDATA_CNS_INV:	Enable No Snoop attribute inversion of
// Completion TLPs
// @IDT_NTMTBLDATA_RNS_INV:	Enable No Snoop attribute inversion of
// Request TLPs
//
pub const IDT_NTMTBLDATA_VALID: c_uint = 0x00000001U;
pub const IDT_NTMTBLDATA_REQID_MASK: c_uint = 0x0001FFFEU;
pub const IDT_NTMTBLDATA_REQID_FLD: c_int = 1;
pub const IDT_NTMTBLDATA_PART_MASK: c_uint = 0x000E0000U;
pub const IDT_NTMTBLDATA_PART_FLD: c_int = 17;
pub const IDT_NTMTBLDATA_ATP_TRANS: c_uint = 0x20000000U;
pub const IDT_NTMTBLDATA_CNS_INV: c_uint = 0x40000000U;
pub const IDT_NTMTBLDATA_RNS_INV: c_uint = 0x80000000U;
//
// REQIDCAP register fields related constants
// @IDT_REQIDCAP_REQID_MASK:	Request ID field mask
// @IDT_REQIDCAP_REQID_FLD:	Request ID field offset
//
pub const IDT_REQIDCAP_REQID_MASK: c_uint = 0x0000FFFFU;
pub const IDT_REQIDCAP_REQID_FLD: c_int = 0;
//
// LUTOFFSET register fields related constants
// @IDT_LUTOFFSET_INDEX_MASK:	Lookup table index field mask
// @IDT_LUTOFFSET_INDEX_FLD:	Lookup table index field offset
// @IDT_LUTOFFSET_BAR_MASK:	Lookup table BAR select field mask
// @IDT_LUTOFFSET_BAR_FLD:	Lookup table BAR select field offset
//
pub const IDT_LUTOFFSET_INDEX_MASK: c_uint = 0x0000001FU;
pub const IDT_LUTOFFSET_INDEX_FLD: c_int = 0;
pub const IDT_LUTOFFSET_BAR_MASK: c_uint = 0x00000700U;
pub const IDT_LUTOFFSET_BAR_FLD: c_int = 8;
//
// LUTUDATA register fields related constants
// @IDT_LUTUDATA_PART_MASK:	Partition field mask
// @IDT_LUTUDATA_PART_FLD:	Partition field offset
// @IDT_LUTUDATA_VALID:		Lookup table entry valid bit
//
pub const IDT_LUTUDATA_PART_MASK: c_uint = 0x0000000FU;
pub const IDT_LUTUDATA_PART_FLD: c_int = 0;
pub const IDT_LUTUDATA_VALID: c_uint = 0x80000000U;
//
// SWPARTxSTS register fields related constants
// @IDT_SWPARTxSTS_SCI:		Switch partition state change initiated
// @IDT_SWPARTxSTS_SCC:		Switch partition state change completed
// @IDT_SWPARTxSTS_STATE_MASK:	Switch partition state mask
// @IDT_SWPARTxSTS_STATE_FLD:	Switch partition state field offset
// @IDT_SWPARTxSTS_STATE_DIS:	Switch partition disabled
// @IDT_SWPARTxSTS_STATE_ACT:	Switch partition enabled
// @IDT_SWPARTxSTS_STATE_RES:	Switch partition in reset
// @IDT_SWPARTxSTS_US:		Switch partition has upstream port
// @IDT_SWPARTxSTS_USID_MASK:	Switch partition upstream port ID mask
// @IDT_SWPARTxSTS_USID_FLD:	Switch partition upstream port ID field offset
// @IDT_SWPARTxSTS_NT:		Upstream port has NT function
// @IDT_SWPARTxSTS_DMA:		Upstream port has DMA function
//
pub const IDT_SWPARTxSTS_SCI: c_uint = 0x00000001U;
pub const IDT_SWPARTxSTS_SCC: c_uint = 0x00000002U;
pub const IDT_SWPARTxSTS_STATE_MASK: c_uint = 0x00000060U;
pub const IDT_SWPARTxSTS_STATE_FLD: c_int = 5;
pub const IDT_SWPARTxSTS_STATE_DIS: c_uint = 0x00000000U;
pub const IDT_SWPARTxSTS_STATE_ACT: c_uint = 0x00000020U;
pub const IDT_SWPARTxSTS_STATE_RES: c_uint = 0x00000060U;
pub const IDT_SWPARTxSTS_US: c_uint = 0x00000100U;
pub const IDT_SWPARTxSTS_USID_MASK: c_uint = 0x00003E00U;
pub const IDT_SWPARTxSTS_USID_FLD: c_int = 9;
pub const IDT_SWPARTxSTS_NT: c_uint = 0x00004000U;
pub const IDT_SWPARTxSTS_DMA: c_uint = 0x00008000U;
//
// SWPORTxSTS register fields related constants
// @IDT_SWPORTxSTS_OMCI:	Operation mode change initiated
// @IDT_SWPORTxSTS_OMCC:	Operation mode change completed
// @IDT_SWPORTxSTS_LINKUP:	Link up status
// @IDT_SWPORTxSTS_DS:		Port lanes behave as downstream lanes
// @IDT_SWPORTxSTS_MODE_MASK:	Port mode field mask
// @IDT_SWPORTxSTS_MODE_FLD:	Port mode field offset
// @IDT_SWPORTxSTS_MODE_DIS:	Port mode - disabled
// @IDT_SWPORTxSTS_MODE_DS:	Port mode - downstream switch port
// @IDT_SWPORTxSTS_MODE_US:	Port mode - upstream switch port
// @IDT_SWPORTxSTS_MODE_NT:	Port mode - NT function
// @IDT_SWPORTxSTS_MODE_USNT:	Port mode - upstream switch port with NTB
// @IDT_SWPORTxSTS_MODE_UNAT:	Port mode - unattached
// @IDT_SWPORTxSTS_MODE_USDMA:	Port mode - upstream switch port with DMA
// @IDT_SWPORTxSTS_MODE_USNTDMA:Port mode - upstream port with NTB and DMA
// @IDT_SWPORTxSTS_MODE_NTDMA:	Port mode - NT function with DMA
// @IDT_SWPORTxSTS_SWPART_MASK:	Port partition field mask
// @IDT_SWPORTxSTS_SWPART_FLD:	Port partition field offset
// @IDT_SWPORTxSTS_DEVNUM_MASK:	Port device number field mask
// @IDT_SWPORTxSTS_DEVNUM_FLD:	Port device number field offset
//
pub const IDT_SWPORTxSTS_OMCI: c_uint = 0x00000001U;
pub const IDT_SWPORTxSTS_OMCC: c_uint = 0x00000002U;
pub const IDT_SWPORTxSTS_LINKUP: c_uint = 0x00000010U;
pub const IDT_SWPORTxSTS_DS: c_uint = 0x00000020U;
pub const IDT_SWPORTxSTS_MODE_MASK: c_uint = 0x000003C0U;
pub const IDT_SWPORTxSTS_MODE_FLD: c_int = 6;
pub const IDT_SWPORTxSTS_MODE_DIS: c_uint = 0x00000000U;
pub const IDT_SWPORTxSTS_MODE_DS: c_uint = 0x00000040U;
pub const IDT_SWPORTxSTS_MODE_US: c_uint = 0x00000080U;
pub const IDT_SWPORTxSTS_MODE_NT: c_uint = 0x000000C0U;
pub const IDT_SWPORTxSTS_MODE_USNT: c_uint = 0x00000100U;
pub const IDT_SWPORTxSTS_MODE_UNAT: c_uint = 0x00000140U;
pub const IDT_SWPORTxSTS_MODE_USDMA: c_uint = 0x00000180U;
pub const IDT_SWPORTxSTS_MODE_USNTDMA: c_uint = 0x000001C0U;
pub const IDT_SWPORTxSTS_MODE_NTDMA: c_uint = 0x00000200U;
pub const IDT_SWPORTxSTS_SWPART_MASK: c_uint = 0x00001C00U;
pub const IDT_SWPORTxSTS_SWPART_FLD: c_int = 10;
pub const IDT_SWPORTxSTS_DEVNUM_MASK: c_uint = 0x001F0000U;
pub const IDT_SWPORTxSTS_DEVNUM_FLD: c_int = 16;
//
// SEMSK register fields related constants
// @IDT_SEMSK_LINKUP:	Link Up event mask bit
// @IDT_SEMSK_LINKDN:	Link Down event mask bit
// @IDT_SEMSK_GSIGNAL:	Global Signal event mask bit
//
pub const IDT_SEMSK_LINKUP: c_uint = 0x00000001U;
pub const IDT_SEMSK_LINKDN: c_uint = 0x00000002U;
pub const IDT_SEMSK_GSIGNAL: c_uint = 0x00000020U;
//
// SWPxMSGCTL register fields related constants
// @IDT_SWPxMSGCTL_REG_MASK:	Register select field mask
// @IDT_SWPxMSGCTL_REG_FLD:	Register select field offset
// @IDT_SWPxMSGCTL_PART_MASK:	Partition select field mask
// @IDT_SWPxMSGCTL_PART_FLD:	Partition select field offset
//
pub const IDT_SWPxMSGCTL_REG_MASK: c_uint = 0x00000003U;
pub const IDT_SWPxMSGCTL_REG_FLD: c_int = 0;
pub const IDT_SWPxMSGCTL_PART_MASK: c_uint = 0x00000070U;
pub const IDT_SWPxMSGCTL_PART_FLD: c_int = 4;
//
// TMPCTL register fields related constants
// @IDT_TMPCTL_LTH_MASK:	Low temperature threshold field mask
// @IDT_TMPCTL_LTH_FLD:		Low temperature threshold field offset
// @IDT_TMPCTL_MTH_MASK:	Middle temperature threshold field mask
// @IDT_TMPCTL_MTH_FLD:		Middle temperature threshold field offset
// @IDT_TMPCTL_HTH_MASK:	High temperature threshold field mask
// @IDT_TMPCTL_HTH_FLD:		High temperature threshold field offset
// @IDT_TMPCTL_PDOWN:		Temperature sensor power down
//
pub const IDT_TMPCTL_LTH_MASK: c_uint = 0x000000FFU;
pub const IDT_TMPCTL_LTH_FLD: c_int = 0;
pub const IDT_TMPCTL_MTH_MASK: c_uint = 0x0000FF00U;
pub const IDT_TMPCTL_MTH_FLD: c_int = 8;
pub const IDT_TMPCTL_HTH_MASK: c_uint = 0x00FF0000U;
pub const IDT_TMPCTL_HTH_FLD: c_int = 16;
pub const IDT_TMPCTL_PDOWN: c_uint = 0x80000000U;
//
// TMPSTS register fields related constants
// @IDT_TMPSTS_TEMP_MASK:	Current temperature field mask
// @IDT_TMPSTS_TEMP_FLD:	Current temperature field offset
// @IDT_TMPSTS_LTEMP_MASK:	Lowest temperature field mask
// @IDT_TMPSTS_LTEMP_FLD:	Lowest temperature field offset
// @IDT_TMPSTS_HTEMP_MASK:	Highest temperature field mask
// @IDT_TMPSTS_HTEMP_FLD:	Highest temperature field offset
//
pub const IDT_TMPSTS_TEMP_MASK: c_uint = 0x000000FFU;
pub const IDT_TMPSTS_TEMP_FLD: c_int = 0;
pub const IDT_TMPSTS_LTEMP_MASK: c_uint = 0x0000FF00U;
pub const IDT_TMPSTS_LTEMP_FLD: c_int = 8;
pub const IDT_TMPSTS_HTEMP_MASK: c_uint = 0x00FF0000U;
pub const IDT_TMPSTS_HTEMP_FLD: c_int = 16;
//
// TMPALARM register fields related constants
// @IDT_TMPALARM_LTEMP_MASK:	Lowest temperature field mask
// @IDT_TMPALARM_LTEMP_FLD:	Lowest temperature field offset
// @IDT_TMPALARM_HTEMP_MASK:	Highest temperature field mask
// @IDT_TMPALARM_HTEMP_FLD:	Highest temperature field offset
// @IDT_TMPALARM_IRQ_MASK:	Alarm IRQ status mask
//
pub const IDT_TMPALARM_LTEMP_MASK: c_uint = 0x0000FF00U;
pub const IDT_TMPALARM_LTEMP_FLD: c_int = 8;
pub const IDT_TMPALARM_HTEMP_MASK: c_uint = 0x00FF0000U;
pub const IDT_TMPALARM_HTEMP_FLD: c_int = 16;
pub const IDT_TMPALARM_IRQ_MASK: c_uint = 0x3F000000U;
//
// TMPADJ register fields related constants
// @IDT_TMPADJ_OFFSET_MASK:	Temperature value offset field mask
// @IDT_TMPADJ_OFFSET_FLD:	Temperature value offset field offset
//
pub const IDT_TMPADJ_OFFSET_MASK: c_uint = 0x000000FFU;
pub const IDT_TMPADJ_OFFSET_FLD: c_int = 0;
//
// Helper macro to get/set the corresponding field value
// @GET_FIELD:		Retrieve the value of the corresponding field
// @SET_FIELD:		Set the specified field up
// @IS_FLD_SET:		Check whether a field is set with value
//

//
// Useful registers masks:
// @IDT_DBELL_MASK:	Doorbell bits mask
// @IDT_OUTMSG_MASK:	Out messages status bits mask
// @IDT_INMSG_MASK:	In messages status bits mask
// @IDT_MSG_MASK:	Any message status bits mask
//

//
// Number of IDT NTB resources:
// @IDT_MSG_CNT:	Number of Message registers
// @IDT_BAR_CNT:	Number of BARs of each port
// @IDT_MTBL_ENTRY_CNT:	Number mapping table entries
//
pub const IDT_MSG_CNT: c_int = 4;
pub const IDT_BAR_CNT: c_int = 6;
pub const IDT_MTBL_ENTRY_CNT: c_int = 64;
//
// General IDT PCIe-switch constant
// @IDT_MAX_NR_PORTS:	Maximum number of ports per IDT PCIe-switch
// @IDT_MAX_NR_PARTS:	Maximum number of partitions per IDT PCIe-switch
// @IDT_MAX_NR_PEERS:	Maximum number of NT-peers per IDT PCIe-switch
// @IDT_MAX_NR_MWS:	Maximum number of Memory Widows
// @IDT_PCIE_REGSIZE:	Size of the registers in bytes
// @IDT_TRANS_ALIGN:	Alignment of translated base address
// @IDT_DIR_SIZE_ALIGN:	Alignment of size setting for direct translated MWs.
// Even though the lower 10 bits are reserved, they are
// treated by IDT as one's so basically there is no any
// alignment of size limit for DIR address translation.
//
pub const IDT_MAX_NR_PORTS: c_int = 24;
pub const IDT_MAX_NR_PARTS: c_int = 8;
pub const IDT_MAX_NR_PEERS: c_int = 8;
pub const IDT_MAX_NR_MWS: c_int = 29;
pub const IDT_PCIE_REGSIZE: c_int = 4;
pub const IDT_TRANS_ALIGN: c_int = 4;
pub const IDT_DIR_SIZE_ALIGN: c_int = 1;
//
// IDT PCIe-switch temperature sensor value limits
// @IDT_TEMP_MIN_MDEG:	Minimal integer value of temperature
// @IDT_TEMP_MAX_MDEG:	Maximal integer value of temperature
// @IDT_TEMP_MIN_OFFSET:Minimal integer value of temperature offset
// @IDT_TEMP_MAX_OFFSET:Maximal integer value of temperature offset
//
pub const IDT_TEMP_MIN_MDEG: c_int = 0;
pub const IDT_TEMP_MAX_MDEG: c_int = 127500;

pub const IDT_TEMP_MAX_OFFSET: c_int = 63500;
//
// Temperature sensor values enumeration
// @IDT_TEMP_CUR:	Current temperature
// @IDT_TEMP_LOW:	Lowest historical temperature
// @IDT_TEMP_HIGH:	Highest historical temperature
// @IDT_TEMP_OFFSET:	Current temperature offset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idt_temp_val {
    IDT_TEMP_CUR,
    IDT_TEMP_LOW,
    IDT_TEMP_HIGH,
    IDT_TEMP_OFFSET
}

//
// IDT Memory Windows type. Depending on the device settings, IDT supports
// Direct Address Translation MW registers and Lookup Table registers
// @IDT_MW_DIR:		Direct address translation
// @IDT_MW_LUT12:	12-entry lookup table entry
// @IDT_MW_LUT24:	24-entry lookup table entry
//
// NOTE These values are exactly the same as one of the BARSETUP ATRAN field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idt_mw_type {
    IDT_MW_DIR = 0x0,
    IDT_MW_LUT12 = 0x1,
    IDT_MW_LUT24 = 0x2
}

//
// IDT PCIe-switch model private data
// @name:	Device name
// @port_cnt:	Total number of NT endpoint ports
// @ports:	Port ids
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_89hpes_cfg {
    pub name: *mut c_char,
    pub port_cnt: c_uchar,
    pub ports: [c_uchar; ],
}

//
// Memory window configuration structure
// @type:	Type of the memory window (direct address translation or lookup
// table)
//
// @bar:	PCIe BAR the memory window referenced to
// @idx:	Index of the memory window within the BAR
//
// @addr_align:	Alignment of translated address
// @size_align:	Alignment of memory window size
// @size_max:	Maximum size of memory window
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_mw_cfg {
    pub type: idt_mw_type,
    pub bar: c_uchar,
    pub idx: c_uchar,
    pub addr_align: u64,
    pub size_align: u64,
    pub size_max: u64,
}

//
// Description structure of peer IDT NT-functions:
// @port:		NT-function port
// @part:		NT-function partition
//
// @mw_cnt:		Number of memory windows supported by NT-function
// @mws:		Array of memory windows descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_peer {
    pub port: c_uchar,
    pub part: c_uchar,
    pub mw_cnt: c_uchar,
    pub mws: *mut idt_mw_cfg,
}

//
// Description structure of local IDT NT-function:
// @ntb:		Linux NTB-device description structure
// @swcfg:		Pointer to the structure of local IDT PCIe-switch
// specific cofnfigurations
//
// @port:		Local NT-function port
// @part:		Local NT-function partition
//
// @peer_cnt:		Number of peers with activated NTB-function
// @peers:		Array of peers descripting structures
// @port_idx_map:	Map of port number -> peer index
// @part_idx_map:	Map of partition number -> peer index
//
// @mtbl_lock:		Mapping table access lock
//
// @mw_cnt:		Number of memory windows supported by NT-function
// @mws:		Array of memory windows descriptors
// @lut_lock:		Lookup table access lock
//
// @msg_locks:		Message registers mapping table lockers
//
// @cfgspc:		Virtual address of the memory mapped configuration
// space of the NT-function
// @db_mask_lock:	Doorbell mask register lock
// @msg_mask_lock:	Message mask register lock
// @gasa_lock:		GASA registers access lock
//
// @hwmon_mtx:		Temperature sensor interface update mutex
//
// @dbgfs_info:		DebugFS info node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_dev {
    pub ntb: ntb_dev,
    pub swcfg: *mut idt_89hpes_cfg,
    pub port: c_uchar,
    pub part: c_uchar,
    pub peer_cnt: c_uchar,
    pub peers: [idt_ntb_peer; IDT_MAX_NR_PEERS],
    pub port_idx_map: [c_char; IDT_MAX_NR_PORTS],
    pub part_idx_map: [c_char; IDT_MAX_NR_PARTS],
    pub mtbl_lock: spinlock_t,
    pub mw_cnt: c_uchar,
    pub mws: *mut idt_mw_cfg,
    pub lut_lock: spinlock_t,
    pub msg_locks: [spinlock_t; IDT_MSG_CNT],
    pub cfgspc: *mut void __iomem,
    pub db_mask_lock: spinlock_t,
    pub msg_mask_lock: spinlock_t,
    pub gasa_lock: spinlock_t,
    pub hwmon_mtx: mutex,
    pub dbgfs_info: *mut dentry,
}

//
// Descriptor of the IDT PCIe-switch BAR resources
// @setup:	BAR setup register
// @limit:	BAR limit register
// @ltbase:	Lower translated base address
// @utbase:	Upper translated base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_bar {
    pub setup: c_uint,
    pub limit: c_uint,
    pub ltbase: c_uint,
    pub utbase: c_uint,
}

//
// Descriptor of the IDT PCIe-switch message resources
// @in:		Inbound message register
// @out:	Outbound message register
// @src:	Source of inbound message register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_msg {
    pub in: c_uint,
    pub out: c_uint,
    pub src: c_uint,
}

//
// Descriptor of the IDT PCIe-switch NT-function specific parameters in the
// PCI Configuration Space
// @bars:	BARs related registers
// @msgs:	Messaging related registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_regs {
    pub bars: [idt_ntb_bar; IDT_BAR_CNT],
    pub msgs: [idt_ntb_msg; IDT_MSG_CNT],
}

//
// Descriptor of the IDT PCIe-switch port specific parameters in the
// Global Configuration Space
// @pcicmdsts:	 PCI command/status register
// @pcielctlsts: PCIe link control/status
//
// @ctl:	Port control register
// @sts:	Port status register
//
// @bars:	BARs related registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_port {
    pub pcicmdsts: c_uint,
    pub pcielctlsts: c_uint,
    pub ntctl: c_uint,
    pub ctl: c_uint,
    pub sts: c_uint,
    pub bars: [idt_ntb_bar; IDT_BAR_CNT],
}

//
// Descriptor of the IDT PCIe-switch partition specific parameters.
// @ctl:	Partition control register in the Global Address Space
// @sts:	Partition status register in the Global Address Space
// @msgctl:	Messages control registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idt_ntb_part {
    pub ctl: c_uint,
    pub sts: c_uint,
    pub msgctl: [c_uint; IDT_MSG_CNT],
}
