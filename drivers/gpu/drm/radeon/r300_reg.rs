//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/r300_reg.h
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
// Copyright 2005 Nicolai Haehnle et al.
// Copyright 2008 Advanced Micro Devices, Inc.
// Copyright 2009 Jerome Glisse.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: Nicolai Haehnle
// Jerome Glisse
//

pub const R300_MC_INIT_MISC_LAT_TIMER: c_uint = 0x180;

pub const R300_MC_INIT_GFX_LAT_TIMER: c_uint = 0x154;

//
// This file contains registers and constants for the R300. They have been
// found mostly by examining command buffers captured using glxtest, as well
// as by extrapolating some known registers and constants from the R200.
// I am fairly certain that they are correct unless stated otherwise
// in comments.
//
pub const R300_SE_VPORT_XSCALE: c_uint = 0x1D98;
pub const R300_SE_VPORT_XOFFSET: c_uint = 0x1D9C;
pub const R300_SE_VPORT_YSCALE: c_uint = 0x1DA0;
pub const R300_SE_VPORT_YOFFSET: c_uint = 0x1DA4;
pub const R300_SE_VPORT_ZSCALE: c_uint = 0x1DA8;
pub const R300_SE_VPORT_ZOFFSET: c_uint = 0x1DAC;
//
// Vertex Array Processing (VAP) Control
// Stolen from r200 code from Christoph Brill (It's a guess!)
//
pub const R300_VAP_CNTL: c_uint = 0x2080;
// This register is written directly and also starts data section
// in many 3d CP_PACKET3's
//
pub const R300_VAP_VF_CNTL: c_uint = 0x2084;

// State based - direct writes to registers trigger vertex

// I don't think I saw these three used..

// index size - when not set the indices are assumed to be 16 bit

// number of vertices

// BEGIN: Wild guesses
pub const R300_VAP_OUTPUT_VTX_FMT_0: c_uint = 0x2090;

pub const R300_VAP_OUTPUT_VTX_FMT_1: c_uint = 0x2094;
// each of the following is 3 bits wide, specifies number

// END: Wild guesses
pub const R300_SE_VTE_CNTL: c_uint = 0x20b0;

// BEGIN: Vertex data assembly - lots of uncertainties
// gap
pub const R300_VAP_CNTL_STATUS: c_uint = 0x2140;

// gap
// Where do we get our vertex data?
//
// Vertex data either comes either from immediate mode registers or from
// vertex arrays.
// There appears to be no mixed mode (though we can force the pitch of
// vertex arrays to 0, effectively reusing the same element over and over
// again).
//
// Immediate mode is controlled by the INPUT_CNTL registers. I am not sure
// if these registers influence vertex array processing.
//
// Vertex arrays are controlled via the 3D_LOAD_VBPNTR packet3.
//
// In both cases, vertex attributes are then passed through INPUT_ROUTE.
//
// Beginning with INPUT_ROUTE_0_0 is a list of WORDs that route vertex data
// into the vertex processor's input registers.
// The first word routes the first input, the second word the second, etc.
// The corresponding input is routed into the register with the given index.
// The list is ended by a word with INPUT_ROUTE_END set.
//
// Always set COMPONENTS_4 in immediate mode.
//
pub const R300_VAP_INPUT_ROUTE_0_0: c_uint = 0x2150;

pub const R300_VAP_INPUT_ROUTE_0_1: c_uint = 0x2154;
pub const R300_VAP_INPUT_ROUTE_0_2: c_uint = 0x2158;
pub const R300_VAP_INPUT_ROUTE_0_3: c_uint = 0x215C;
pub const R300_VAP_INPUT_ROUTE_0_4: c_uint = 0x2160;
pub const R300_VAP_INPUT_ROUTE_0_5: c_uint = 0x2164;
pub const R300_VAP_INPUT_ROUTE_0_6: c_uint = 0x2168;
pub const R300_VAP_INPUT_ROUTE_0_7: c_uint = 0x216C;
// gap
// Notes:
// - always set up to produce at least two attributes:
// if vertex program uses only position, fglrx will set normal, too
// - INPUT_CNTL_0_COLOR and INPUT_CNTL_COLOR bits are always equal.
//
pub const R300_VAP_INPUT_CNTL_0: c_uint = 0x2180;

pub const R300_VAP_INPUT_CNTL_1: c_uint = 0x2184;

// gap
// Words parallel to INPUT_ROUTE_0; All words that are active in INPUT_ROUTE_0
// are set to a swizzling bit pattern, other words are 0.
//
// In immediate mode, the pattern is always set to xyzw. In vertex array
// mode, the swizzling pattern is e.g. used to set zw components in texture
// coordinates with only tweo components.
//
pub const R300_VAP_INPUT_ROUTE_1_0: c_uint = 0x21E0;

pub const R300_VAP_INPUT_ROUTE_1_1: c_uint = 0x21E4;
pub const R300_VAP_INPUT_ROUTE_1_2: c_uint = 0x21E8;
pub const R300_VAP_INPUT_ROUTE_1_3: c_uint = 0x21EC;
pub const R300_VAP_INPUT_ROUTE_1_4: c_uint = 0x21F0;
pub const R300_VAP_INPUT_ROUTE_1_5: c_uint = 0x21F4;
pub const R300_VAP_INPUT_ROUTE_1_6: c_uint = 0x21F8;
pub const R300_VAP_INPUT_ROUTE_1_7: c_uint = 0x21FC;
// END: Vertex data assembly
// gap
// BEGIN: Upload vertex program and data
//
// The programmable vertex shader unit has a memory bank of unknown size
// that can be written to in 16 byte units by writing the address into
// UPLOAD_ADDRESS, followed by data in UPLOAD_DATA (multiples of 4 DWORDs).
//
// Pointers into the memory bank are always in multiples of 16 bytes.
//
// The memory bank is divided into areas with fixed meaning.
//
// Starting at address UPLOAD_PROGRAM: Vertex program instructions.
// Native limits reported by drivers from ATI suggest size 256 (i.e. 4KB),
// whereas the difference between known addresses suggests size 512.
//
// Starting at address UPLOAD_PARAMETERS: Vertex program parameters.
// Native reported limits and the VPI layout suggest size 256, whereas
// difference between known addresses suggests size 512.
//
// At address UPLOAD_POINTSIZE is a vector (0, 0, ps, 0), where ps is the
// floating point pointsize. The exact purpose of this state is uncertain,
// as there is also the R300_RE_POINTSIZE register.
//
// Multiple vertex programs and parameter sets can be loaded at once,
// which could explain the size discrepancy.
//
pub const R300_VAP_PVS_UPLOAD_ADDRESS: c_uint = 0x2200;

// gap
pub const R300_VAP_PVS_UPLOAD_DATA: c_uint = 0x2208;
// END: Upload vertex program and data
// gap
// I do not know the purpose of this register. However, I do know that
// it is set to 221C_CLEAR for clear operations and to 221C_NORMAL
// for normal rendering.
//
pub const R300_VAP_UNKNOWN_221C: c_uint = 0x221C;

// These seem to be per-pixel and per-vertex X and Y clipping planes. The first
// plane is per-pixel and the second plane is per-vertex.
//
// This was determined by experimentation alone but I believe it is correct.
//
// These registers are called X_QUAD0_1_FL to X_QUAD0_4_FL by glxtest.
//
pub const R300_VAP_CLIP_X_0: c_uint = 0x2220;
pub const R300_VAP_CLIP_X_1: c_uint = 0x2224;
pub const R300_VAP_CLIP_Y_0: c_uint = 0x2228;
pub const R300_VAP_CLIP_Y_1: c_uint = 0x2230;
// gap
// Sometimes, END_OF_PKT and 0x2284=0 are the only commands sent between
// rendering commands and overwriting vertex program parameters.
// Therefore, I suspect writing zero to 0x2284 synchronizes the engine and
// avoids bugs caused by still running shaders reading bad data from memory.
//
pub const R300_VAP_PVS_STATE_FLUSH_REG: c_uint = 0x2284;
// Absolutely no clue what this register is about.
pub const R300_VAP_UNKNOWN_2288: c_uint = 0x2288;

// gap
// Addresses are relative to the vertex program instruction area of the
// memory bank. PROGRAM_END points to the last instruction of the active
// program
//
// The meaning of the two UNKNOWN fields is obviously not known. However,
// experiments so far have shown that both *must* point to an instruction
// inside the vertex program, otherwise the GPU locks up.
//
// fglrx usually sets CNTL_3_UNKNOWN to the end of the program and
// R300_PVS_CNTL_1_POS_END_SHIFT points to instruction where last write to
// position takes place.
//
// Most likely this is used to ignore rest of the program in cases
// where group of verts arent visible. For some reason this "section"
// is sometimes accepted other instruction that have no relationship with
// position calculations.
//
pub const R300_VAP_PVS_CNTL_1: c_uint = 0x22D0;

// Addresses are relative the vertex program parameters area.
pub const R300_VAP_PVS_CNTL_2: c_uint = 0x22D4;

pub const R300_VAP_PVS_CNTL_3: c_uint = 0x22D8;

// The entire range from 0x2300 to 0x2AC inclusive seems to be used for
// immediate vertices
//
pub const R300_VAP_VTX_COLOR_R: c_uint = 0x2464;
pub const R300_VAP_VTX_COLOR_G: c_uint = 0x2468;
pub const R300_VAP_VTX_COLOR_B: c_uint = 0x246C;
pub const R300_VAP_VTX_POS_0_X_1: c_uint = 0x2490 /* used for glVertex2*() */;
pub const R300_VAP_VTX_POS_0_Y_1: c_uint = 0x2494;
pub const R300_VAP_VTX_COLOR_PKD: c_uint = 0x249C /* RGBA */;
pub const R300_VAP_VTX_POS_0_X_2: c_uint = 0x24A0 /* used for glVertex3*() */;
pub const R300_VAP_VTX_POS_0_Y_2: c_uint = 0x24A4;
pub const R300_VAP_VTX_POS_0_Z_2: c_uint = 0x24A8;
// write 0 to indicate end of packet?
pub const R300_VAP_VTX_END_OF_PKT: c_uint = 0x24AC;
// gap
// These are values from r300_reg/r300_reg.h - they are known to be correct
// and are here so we can use one register file instead of several
// - Vladimir
//
pub const R300_GB_VAP_RASTER_VTX_FMT_0: c_uint = 0x4000;

pub const R300_GB_VAP_RASTER_VTX_FMT_1: c_uint = 0x4004;
// each of the following is 3 bits wide, specifies number

// UNK30 seems to enables point to quad transformation on textures
// (or something closely related to that).
// This bit is rather fatal at the time being due to lackings at pixel
// shader side
//
pub const R300_GB_ENABLE: c_uint = 0x4008;

// each of the following is 2 bits wide
pub const R300_GB_TEX_REPLICATE: c_int = 0;
pub const R300_GB_TEX_ST: c_int = 1;
pub const R300_GB_TEX_STR: c_int = 2;

// MSPOS - positions for multisample antialiasing (?)
pub const R300_GB_MSPOS0: c_uint = 0x4010;
// shifts - each of the fields is 4 bits

pub const R300_GB_MSPOS1: c_uint = 0x4014;

pub const R300_GB_TILE_CONFIG: c_uint = 0x4018;

pub const R300_GB_FIFO_SIZE: c_uint = 0x4024;
// each of the following is 2 bits wide
pub const R300_GB_FIFO_SIZE_32: c_int = 0;
pub const R300_GB_FIFO_SIZE_64: c_int = 1;
pub const R300_GB_FIFO_SIZE_128: c_int = 2;
pub const R300_GB_FIFO_SIZE_256: c_int = 3;

// the following use the same constants as above, but meaning is

// watermarks, 3 bits wide

pub const R300_GB_SELECT: c_uint = 0x401C;

pub const R300_GB_AA_CONFIG: c_uint = 0x4020;

// gap
// Zero to flush caches.
pub const R300_TX_INVALTAGS: c_uint = 0x4100;
pub const R300_TX_FLUSH: c_uint = 0x0;
// The upper enable bits are guessed, based on fglrx reported limits.
pub const R300_TX_ENABLE: c_uint = 0x4104;

// The pointsize is given in multiples of 6. The pointsize can be
// enormous: Clear() renders a single point that fills the entire
// framebuffer.
//
pub const R300_RE_POINTSIZE: c_uint = 0x421C;

// The line width is given in multiples of 6.
// In default mode lines are classified as vertical lines.
// HO: horizontal
// VE: vertical or horizontal
// HO & VE: no classification
//
pub const R300_RE_LINE_CNT: c_uint = 0x4234;

// Some sort of scale or clamp value for texcoordless textures.
pub const R300_RE_UNK4238: c_uint = 0x4238;
// Something shade related
pub const R300_RE_SHADE: c_uint = 0x4274;
pub const R300_RE_SHADE_MODEL: c_uint = 0x4278;

// Dangerous
pub const R300_RE_POLYGON_MODE: c_uint = 0x4288;

// Fog parameters
pub const R300_RE_FOG_SCALE: c_uint = 0x4294;
pub const R300_RE_FOG_START: c_uint = 0x4298;
// Not sure why there are duplicate of factor and constant values.
// My best guess so far is that there are separate zbiases for test and write.
// Ordering might be wrong.
// Some of the tests indicate that fgl has a fallback implementation of zbias
// via pixel shaders.
//
pub const R300_RE_ZBIAS_CNTL: c_uint = 0x42A0 /* GUESS */;
pub const R300_RE_ZBIAS_T_FACTOR: c_uint = 0x42A4;
pub const R300_RE_ZBIAS_T_CONSTANT: c_uint = 0x42A8;
pub const R300_RE_ZBIAS_W_FACTOR: c_uint = 0x42AC;
pub const R300_RE_ZBIAS_W_CONSTANT: c_uint = 0x42B0;
// This register needs to be set to (1<<1) for RV350 to correctly
// perform depth test (see --vb-triangles in r300_demo)
// Don't know about other chips. - Vladimir
// This is set to 3 when GL_POLYGON_OFFSET_FILL is on.
// My guess is that there are two bits for each zbias primitive
// (FILL, LINE, POINT).
// One to enable depth test and one for depth write.
// Yet this doesn't explain why depth writes work ...
//
pub const R300_RE_OCCLUSION_CNTL: c_uint = 0x42B4;

pub const R300_RE_CULL_CNTL: c_uint = 0x42B8;

// BEGIN: Rasterization / Interpolators - many guesses
// 0_UNKNOWN_18 has always been set except for clear operations.
// TC_CNT is the number of incoming texture coordinate sets (i.e. it depends
// on the vertex program, *not* the fragment program)
//
pub const R300_RS_CNTL_0: c_uint = 0x4300;

// number of color interpolators used

// Guess: RS_CNTL_1 holds the index of the highest used RS_ROUTE_n
pub const R300_RS_CNTL_1: c_uint = 0x4304;
// gap
// Only used for texture coordinates.
// Use the source field to route texture coordinate input from the
// vertex program to the desired interpolator. Note that the source
// field is relative to the outputs the vertex program *actually
// writes. If a vertex program only writes texcoord[1], this will
// be source index 0.
// Set INTERP_USED on all interpolators that produce data used by
// the fragment program. INTERP_USED looks like a swizzling mask,
// but I haven't seen it used that way.
//
// Note: The _UNKNOWN constants are always set in their respective
// register. I don't know if this is necessary.
//
pub const R300_RS_INTERP_0: c_uint = 0x4310;
pub const R300_RS_INTERP_1: c_uint = 0x4314;

pub const R300_RS_INTERP_2: c_uint = 0x4318;

pub const R300_RS_INTERP_3: c_uint = 0x431C;

pub const R300_RS_INTERP_4: c_uint = 0x4320;
pub const R300_RS_INTERP_5: c_uint = 0x4324;
pub const R300_RS_INTERP_6: c_uint = 0x4328;
pub const R300_RS_INTERP_7: c_uint = 0x432C;

// These DWORDs control how vertex data is routed into fragment program
// registers, after interpolators.
//
pub const R300_RS_ROUTE_0: c_uint = 0x4330;
pub const R300_RS_ROUTE_1: c_uint = 0x4334;
pub const R300_RS_ROUTE_2: c_uint = 0x4338;
pub const R300_RS_ROUTE_3: c_uint = 0x433C /* GUESS */;
pub const R300_RS_ROUTE_4: c_uint = 0x4340 /* GUESS */;
pub const R300_RS_ROUTE_5: c_uint = 0x4344 /* GUESS */;
pub const R300_RS_ROUTE_6: c_uint = 0x4348 /* GUESS */;
pub const R300_RS_ROUTE_7: c_uint = 0x434C /* GUESS */;

// Special handling for color: When the fragment program uses color,
// the ROUTE_0_COLOR bit is set and ROUTE_0_COLOR_DEST contains the
// color register index.
//
// Apperently you may set the R300_RS_ROUTE_0_COLOR bit, but not provide any
// R300_RS_ROUTE_0_COLOR_DEST value; this setup is used for clearing the state.
// See r300_ioctl.c:r300EmitClearState. I'm not sure if this setup is strictly
// correct or not. - Oliver.
//

// As above, but for secondary color

// END: Rasterization / Interpolators - many guesses
// Hierarchical Z Enable
pub const R300_SC_HYPERZ: c_uint = 0x43a4;

pub const R300_SC_EDGERULE: c_uint = 0x43a8;
// BEGIN: Scissors and cliprects
// There are four clipping rectangles. Their corner coordinates are inclusive.
// Every pixel is assigned a number from 0 and 15 by setting bits 0-3 depending
// on whether the pixel is inside cliprects 0-3, respectively. For example,
// if a pixel is inside cliprects 0 and 1, but outside 2 and 3, it is assigned
// the number 3 (binary 0011).
// Iff the bit corresponding to the pixel's number in RE_CLIPRECT_CNTL is set,
// the pixel is rasterized.
//
// In addition to this, there is a scissors rectangle. Only pixels inside the
// scissors rectangle are drawn. (coordinates are inclusive)
//
// For some reason, the top-left corner of the framebuffer is at (1440, 1440)
// for the purpose of clipping and scissors.
//
pub const R300_RE_CLIPRECT_TL_0: c_uint = 0x43B0;
pub const R300_RE_CLIPRECT_BR_0: c_uint = 0x43B4;
pub const R300_RE_CLIPRECT_TL_1: c_uint = 0x43B8;
pub const R300_RE_CLIPRECT_BR_1: c_uint = 0x43BC;
pub const R300_RE_CLIPRECT_TL_2: c_uint = 0x43C0;
pub const R300_RE_CLIPRECT_BR_2: c_uint = 0x43C4;
pub const R300_RE_CLIPRECT_TL_3: c_uint = 0x43C8;
pub const R300_RE_CLIPRECT_BR_3: c_uint = 0x43CC;

pub const R300_RE_CLIPRECT_CNTL: c_uint = 0x43D0;

// gap
pub const R300_RE_SCISSORS_TL: c_uint = 0x43E0;
pub const R300_RE_SCISSORS_BR: c_uint = 0x43E4;

// END: Scissors and cliprects
// BEGIN: Texture specification
//
// The texture specification dwords are grouped by meaning and not by texture
// unit. This means that e.g. the offset for texture image unit N is found in
// register TX_OFFSET_0 + (4*N)
//
pub const R300_TX_FILTER_0: c_uint = 0x4400;

// NOTE: NEAREST doesn't seem to exist.
// Im not seting MAG_FILTER_MASK and (3 << 11) on for all
// anisotropy modes because that would void selected mag filter
//

pub const R300_TX_FILTER1_0: c_uint = 0x4440;

pub const R300_TX_SIZE_0: c_uint = 0x4480;

pub const R300_TX_FORMAT_0: c_uint = 0x44C0;
// The interpretation of the format word by Wladimir van der Laan
// The X, Y, Z and W refer to the layout of the components.

// 0x16 - some 16 bit green format.. ??

// gap
// Floating point formats
// Note - hardware supports both 16 and 32 bit floating point

// alpha modes, convenience mostly
// if you have alpha, pick constant appropriate to the

// Swizzling
// constants

// 2.0*Z, everything above 1.0 is set to 0.0

// 2.0*W, everything above 1.0 is set to 0.0

// Convenience macro to take care of layout and swizzling

// These can be ORed with result of R300_EASY_TX_FORMAT()

pub const R300_TX_PITCH_0: c_uint = 0x4500 /* obvious missing in gap */;
pub const R300_TX_OFFSET_0: c_uint = 0x4540;
// BEGIN: Guess from R200

// END: Guess from R200
// 32 bit chroma key
pub const R300_TX_CHROMA_KEY_0: c_uint = 0x4580;
// ff00ff00 == { 0, 1.0, 0, 1.0 }
pub const R300_TX_BORDER_COLOR_0: c_uint = 0x45C0;
// END: Texture specification
// BEGIN: Fragment program instruction set
// Fragment programs are written directly into register space.
// There are separate instruction streams for texture instructions and ALU
// instructions.
// In order to synchronize these streams, the program is divided into up
// to 4 nodes. Each node begins with a number of TEX operations, followed
// by a number of ALU operations.
// The first node can have zero TEX ops, all subsequent nodes must have at
// least
// one TEX ops.
// All nodes must have at least one ALU op.
//
// The index of the last node is stored in PFS_CNTL_0: A value of 0 means
// 1 node, a value of 3 means 4 nodes.
// The total amount of instructions is defined in PFS_CNTL_2. The offsets are
// offsets into the respective instruction streams, while *_END points to the
// last instruction relative to this offset.
//
pub const R300_PFS_CNTL_0: c_uint = 0x4600;

pub const R300_PFS_CNTL_1: c_uint = 0x4604;
// There is an unshifted value here which has so far always been equal to the
// index of the highest used temporary register.
//
pub const R300_PFS_CNTL_2: c_uint = 0x4608;

// gap
// Nodes are stored backwards. The last active node is always stored in
// PFS_NODE_3.
// Example: In a 2-node program, NODE_0 and NODE_1 are set to 0. The
// first node is stored in NODE_2, the second node is stored in NODE_3.
//
// Offsets are relative to the master offset from PFS_CNTL_2.
//
pub const R300_PFS_NODE_0: c_uint = 0x4610;
pub const R300_PFS_NODE_1: c_uint = 0x4614;
pub const R300_PFS_NODE_2: c_uint = 0x4618;
pub const R300_PFS_NODE_3: c_uint = 0x461C;

// TEX
// As far as I can tell, texture instructions cannot write into output
// registers directly. A subsequent ALU instruction is always necessary,
// even if it's just MAD o0, r0, 1, 0
//
pub const R300_PFS_TEXI_0: c_uint = 0x4620;

// GUESS

// GUESS based on layout and native limits

// Unsure if these are opcodes, or some kind of bitfield, but this is how
// they were set when I checked
//

// ALU
// The ALU instructions register blocks are enumerated according to the order
// in which fglrx. I assume there is space for 64 instructions, since
// each block has space for a maximum of 64 DWORDs, and this matches reported
// native limits.
//
// The basic functional block seems to be one MAD for each color and alpha,
// and an adder that adds all components after the MUL.
// - ADD, MUL, MAD etc.: use MAD with appropriate neutral operands
// - DP4: Use OUTC_DP4, OUTA_DP4
// - DP3: Use OUTC_DP3, OUTA_DP4, appropriate alpha operands
// - DPH: Use OUTC_DP4, OUTA_DP4, appropriate alpha operands
// - CMPH: If ARG2 > 0.5, return ARG0, else return ARG1
// - CMP: If ARG2 < 0, return ARG1, else return ARG0
// - FLR: use FRC+MAD
// - XPD: use MAD+MAD
// - SGE, SLT: use MAD+CMP
// - RSQ: use ABS modifier for argument
// - Use OUTC_REPL_ALPHA to write results of an alpha-only operation
// (e.g. RCP) into color register
// - apparently, there's no quick DST operation
// - fglrx set FPI2_UNKNOWN_31 on a "MAD fragment.color, tmp0, tmp1, tmp2"
// - fglrx set FPI2_UNKNOWN_31 on a "MAX r2, r1, c0"
// - fglrx once set FPI0_UNKNOWN_31 on a "FRC r1, r1"
//
// Operand selection
// First stage selects three sources from the available registers and
// constant parameters. This is defined in INSTR1 (color) and INSTR3 (alpha).
// fglrx sorts the three source fields: Registers before constants,
// lower indices before higher indices; I do not know whether this is
// necessary.
//
// fglrx fills unused sources with "read constant 0"
// According to specs, you cannot select more than two different constants.
//
// Second stage selects the operands from the sources. This is defined in
// INSTR0 (color) and INSTR2 (alpha). You can also select the special constants
// zero and one.
// Swizzling and negation happens in this stage, as well.
//
// Important: Color and alpha seem to be mostly separate, i.e. their sources
// selection appears to be fully independent (the register storage is probably
// physically split into a color and an alpha section).
// However (because of the apparent physical split), there is some interaction
// WRT swizzling. If, for example, you want to load an R component into an
// Alpha operand, this R component is taken from a *color* source, not from
// an alpha source. The corresponding register doesn't even have to appear in
// the alpha sources list. (I hope this all makes sense to you)
//
// Destination selection
// The destination register index is in FPI1 (color) and FPI3 (alpha)
// together with enable bits.
// There are separate enable bits for writing into temporary registers
// (DSTC_REG_* /DSTA_REG) and program output registers (DSTC_OUTPUT_
// /DSTA_OUTPUT). You can write to both at once, or not write at all (the
// same index must be used for both).
//
// Note: There is a special form for LRP
// - Argument order is the same as in ARB_fragment_program.
// - Operation is MAD
// - ARG1 is set to ARGC_SRC1C_LRP/ARGC_SRC1A_LRP
// - Set FPI0/FPI2_SPECIAL_LRP
// Arbitrary LRP (including support for swizzling) requires vanilla MAD+MAD
//
pub const R300_PFS_INSTR1_0: c_uint = 0x46C0;

pub const R300_PFS_INSTR3_0: c_uint = 0x47C0;

pub const R300_PFS_INSTR0_0: c_uint = 0x48C0;

// GUESS

pub const R300_PFS_INSTR2_0: c_uint = 0x49C0;

// GUESS

// GUESS

// GUESS

// GUESS

// END: Fragment program instruction set
// Fog state and color
pub const R300_RE_FOG_STATE: c_uint = 0x4BC0;

pub const R300_FOG_COLOR_R: c_uint = 0x4BC8;
pub const R300_FOG_COLOR_G: c_uint = 0x4BCC;
pub const R300_FOG_COLOR_B: c_uint = 0x4BD0;
pub const R300_PP_ALPHA_TEST: c_uint = 0x4BD4;

// gap
// Fragment program parameters in 7.16 floating point
pub const R300_PFS_PARAM_0_X: c_uint = 0x4C00;
pub const R300_PFS_PARAM_0_Y: c_uint = 0x4C04;
pub const R300_PFS_PARAM_0_Z: c_uint = 0x4C08;
pub const R300_PFS_PARAM_0_W: c_uint = 0x4C0C;
// GUESS: PARAM_31 is last, based on native limits reported by fglrx
pub const R300_PFS_PARAM_31_X: c_uint = 0x4DF0;
pub const R300_PFS_PARAM_31_Y: c_uint = 0x4DF4;
pub const R300_PFS_PARAM_31_Z: c_uint = 0x4DF8;
pub const R300_PFS_PARAM_31_W: c_uint = 0x4DFC;
// Notes:
// - AFAIK fglrx always sets BLEND_UNKNOWN when blending is used in
// the application
// - AFAIK fglrx always sets BLEND_NO_SEPARATE when CBLEND and ABLEND
// are set to the same
// function (both registers are always set up completely in any case)
// - Most blend flags are simply copied from R200 and not tested yet
//
pub const R300_RB3D_CBLEND: c_uint = 0x4E04;
pub const R300_RB3D_ABLEND: c_uint = 0x4E08;
// the following only appear in CBLEND

// the following are shared between CBLEND and ABLEND

pub const R300_RB3D_BLEND_COLOR: c_uint = 0x4E10;
pub const R300_RB3D_COLORMASK: c_uint = 0x4E0C;

// gap
pub const R300_RB3D_COLOROFFSET0: c_uint = 0x4E28;

pub const R300_RB3D_COLOROFFSET1: c_uint = 0x4E2C /* GUESS */;
pub const R300_RB3D_COLOROFFSET2: c_uint = 0x4E30 /* GUESS */;
pub const R300_RB3D_COLOROFFSET3: c_uint = 0x4E34 /* GUESS */;
// gap
// Bit 16: Larger tiles
// Bit 17: 4x2 tiles
// Bit 18: Extremely weird tile like, but some pixels duplicated?
//
pub const R300_RB3D_COLORPITCH0: c_uint = 0x4E38;

pub const R300_RB3D_COLORPITCH1: c_uint = 0x4E3C /* GUESS */;
pub const R300_RB3D_COLORPITCH2: c_uint = 0x4E40 /* GUESS */;
pub const R300_RB3D_COLORPITCH3: c_uint = 0x4E44 /* GUESS */;
pub const R300_RB3D_AARESOLVE_OFFSET: c_uint = 0x4E80;
pub const R300_RB3D_AARESOLVE_PITCH: c_uint = 0x4E84;
pub const R300_RB3D_AARESOLVE_CTL: c_uint = 0x4E88;
// gap
// Guess by Vladimir.
// Set to 0A before 3D operations, set to 02 afterwards.
//
// #define R300_RB3D_DSTCACHE_CTLSTAT          0x4E4C

// gap
// There seems to be no "write only" setting, so use Z-test = ALWAYS
// for this.
// Bit (1<<8) is the "test" bit. so plain write is 6  - vd
//
pub const R300_ZB_CNTL: c_uint = 0x4F00;

pub const R300_ZB_ZSTENCILCNTL: c_uint = 0x4f04;
// functions

// operations

// front and back refer to operations done for front

pub const R300_ZB_STENCILREFMASK: c_uint = 0x4f08;

// gap
pub const R300_ZB_FORMAT: c_uint = 0x4f10;

// reserved up to (15 << 0)

pub const R300_ZB_ZTOP: c_uint = 0x4F14;

// gap
pub const R300_ZB_ZCACHE_CTLSTAT: c_uint = 0x4f18;

pub const R300_ZB_BW_CNTL: c_uint = 0x4f1c;

// gap
// Z Buffer Address Offset.
// Bits 31 to 5 are used for aligned Z buffer address offset for macro tiles.
//
pub const R300_ZB_DEPTHOFFSET: c_uint = 0x4f20;
// Z Buffer Pitch and Endian Control
pub const R300_ZB_DEPTHPITCH: c_uint = 0x4f24;

// Z Buffer Clear Value
pub const R300_ZB_DEPTHCLEARVALUE: c_uint = 0x4f28;
pub const R300_ZB_ZMASK_OFFSET: c_uint = 0x4f30;
pub const R300_ZB_ZMASK_PITCH: c_uint = 0x4f34;
pub const R300_ZB_ZMASK_WRINDEX: c_uint = 0x4f38;
pub const R300_ZB_ZMASK_DWORD: c_uint = 0x4f3c;
pub const R300_ZB_ZMASK_RDINDEX: c_uint = 0x4f40;
// Hierarchical Z Memory Offset
pub const R300_ZB_HIZ_OFFSET: c_uint = 0x4f44;
// Hierarchical Z Write Index
pub const R300_ZB_HIZ_WRINDEX: c_uint = 0x4f48;
// Hierarchical Z Data
pub const R300_ZB_HIZ_DWORD: c_uint = 0x4f4c;
// Hierarchical Z Read Index
pub const R300_ZB_HIZ_RDINDEX: c_uint = 0x4f50;
// Hierarchical Z Pitch
pub const R300_ZB_HIZ_PITCH: c_uint = 0x4f54;
// Z Buffer Z Pass Counter Data
pub const R300_ZB_ZPASS_DATA: c_uint = 0x4f58;
// Z Buffer Z Pass Counter Address
pub const R300_ZB_ZPASS_ADDR: c_uint = 0x4f5c;
// Depth buffer X and Y coordinate offset
pub const R300_ZB_DEPTHXY_OFFSET: c_uint = 0x4f60;

// Sets the fifo sizes
pub const R500_ZB_FIFO_SIZE: c_uint = 0x4fd0;

// Stencil Reference Value and Mask for backfacing quads
// R300_ZB_STENCILREFMASK handles front face
pub const R500_ZB_STENCILREFMASK_BF: c_uint = 0x4fd4;

// BEGIN: Vertex program instruction set
// Every instruction is four dwords long:
// DWORD 0: output and opcode
// DWORD 1: first argument
// DWORD 2: second argument
// DWORD 3: third argument
//
// Notes:
// - ABS r, a is implemented as MAX r, a, -a
// - MOV is implemented as ADD to zero
// - XPD is implemented as MUL + MAD
// - FLR is implemented as FRC + ADD
// - apparently, fglrx tries to schedule instructions so that there is at
// least one instruction between the write to a temporary and the first
// read from said temporary; however, violations of this scheduling are
// allowed
// - register indices seem to be unrelated with OpenGL aliasing to
// conventional state
// - only one attribute and one parameter can be loaded at a time; however,
// the same attribute/parameter can be used for more than one argument
// - the second software argument for POW is the third hardware argument
// (no idea why)
// - MAD with only temporaries as input seems to use VPI_OUT_SELECT_MAD_2
//
// There is some magic surrounding LIT:
// The single argument is replicated across all three inputs, but swizzled:
// First argument: xyzy
// Second argument: xyzx
// Third argument: xyzw
// Whenever the result is used later in the fragment program, fglrx forces
// x and w to be 1.0 in the input selection; I don't know whether this is
// strictly necessary
//

// Used in GL_POINT_DISTANCE_ATTENUATION_ARB, vector(scalar, vector)

// Used in fog computations, scalar(scalar)

// Used in GL_POINT_DISTANCE_ATTENUATION_ARB, scalar(scalar)

// all temps, vector(scalar, vector, vector)

pub const R300_VPI_OUT_REG_INDEX_SHIFT: c_int = 13;
// GUESS based on fglrx native limits

pub const R300_VPI_IN_REG_INDEX_SHIFT: c_int = 5;
// GUESS based on fglrx native limits

// The R300 can select components from the input register arbitrarily.
// Use the following constants, shifted by the component shift you
// want to select
//
pub const R300_VPI_IN_SELECT_X: c_int = 0;
pub const R300_VPI_IN_SELECT_Y: c_int = 1;
pub const R300_VPI_IN_SELECT_Z: c_int = 2;
pub const R300_VPI_IN_SELECT_W: c_int = 3;
pub const R300_VPI_IN_SELECT_ZERO: c_int = 4;
pub const R300_VPI_IN_SELECT_ONE: c_int = 5;
pub const R300_VPI_IN_SELECT_MASK: c_int = 7;
pub const R300_VPI_IN_X_SHIFT: c_int = 13;
pub const R300_VPI_IN_Y_SHIFT: c_int = 16;
pub const R300_VPI_IN_Z_SHIFT: c_int = 19;
pub const R300_VPI_IN_W_SHIFT: c_int = 22;

// END: Vertex program instruction set
// BEGIN: Packet 3 commands
// A primitive emission dword.

// GUESS (based on r200)

pub const R300_PRIM_TYPE_MASK: c_uint = 0xF;

// GUESS (based on r200)

pub const R300_PRIM_NUM_VERTICES_SHIFT: c_int = 16;
pub const R300_PRIM_NUM_VERTICES_MASK: c_uint = 0xffff;
// Draw a primitive from vertex data in arrays loaded via 3D_LOAD_VBPNTR.
// Two parameter dwords:
// 0. The first parameter appears to be always 0
// 1. The second parameter is a standard primitive emission dword.
//
pub const R300_PACKET3_3D_DRAW_VBUF: c_uint = 0x00002800;
// Specify the full set of vertex arrays as (address, stride).
// The first parameter is the number of vertex arrays specified.
// The rest of the command is a variable length list of blocks, where
// each block is three dwords long and specifies two arrays.
// The first dword of a block is split into two words, the lower significant
// word refers to the first array, the more significant word to the second
// array in the block.
// The low byte of each word contains the size of an array entry in dwords,
// the high byte contains the stride of the array.
// The second dword of a block contains the pointer to the first array,
// the third dword of a block contains the pointer to the second array.
// Note that if the total number of arrays is odd, the third dword of
// the last block is omitted.
//
pub const R300_PACKET3_3D_LOAD_VBPNTR: c_uint = 0x00002F00;
pub const R300_PACKET3_INDX_BUFFER: c_uint = 0x00003300;

pub const R300_PACKET3_3D_DRAW_VBUF_2: c_uint = 0x00003400;
pub const R300_PACKET3_3D_DRAW_INDX_2: c_uint = 0x00003600;
// END: Packet 3 commands
// Color formats for 2d packets
//
pub const R300_CP_COLOR_FORMAT_CI8: c_int = 2;
pub const R300_CP_COLOR_FORMAT_ARGB1555: c_int = 3;
pub const R300_CP_COLOR_FORMAT_RGB565: c_int = 4;
pub const R300_CP_COLOR_FORMAT_ARGB8888: c_int = 6;
pub const R300_CP_COLOR_FORMAT_RGB332: c_int = 7;
pub const R300_CP_COLOR_FORMAT_RGB8: c_int = 9;
pub const R300_CP_COLOR_FORMAT_ARGB4444: c_int = 15;
//
// CP type-3 packets
//
pub const R300_CP_CMD_BITBLT_MULTI: c_uint = 0xC0009B00;
pub const R500_VAP_INDEX_OFFSET: c_uint = 0x208c;
pub const R500_GA_US_VECTOR_INDEX: c_uint = 0x4250;
pub const R500_GA_US_VECTOR_DATA: c_uint = 0x4254;
pub const R500_RS_IP_0: c_uint = 0x4074;
pub const R500_RS_INST_0: c_uint = 0x4320;
pub const R500_US_CONFIG: c_uint = 0x4600;
pub const R500_US_FC_CTRL: c_uint = 0x4624;
pub const R500_US_CODE_ADDR: c_uint = 0x4630;
pub const R500_RB3D_COLOR_CLEAR_VALUE_AR: c_uint = 0x46c0;
pub const R500_RB3D_CONSTANT_COLOR_AR: c_uint = 0x4ef8;
pub const R300_SU_REG_DEST: c_uint = 0x42c8;
pub const RV530_FG_ZBREG_DEST: c_uint = 0x4be8;
pub const R300_ZB_ZPASS_DATA: c_uint = 0x4f58;
pub const R300_ZB_ZPASS_ADDR: c_uint = 0x4f5c;
