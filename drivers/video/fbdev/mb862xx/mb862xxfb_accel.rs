//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/mb862xx/mb862xxfb_accel.h
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
// registers
pub const GDC_GEO_REG_INPUT_FIFO: c_uint = 0x00000400L;
// Special Registers
pub const GDC_REG_CTRL: c_uint = 0x00000400L;
pub const GDC_REG_FIFO_STATUS: c_uint = 0x00000404L;
pub const GDC_REG_FIFO_COUNT: c_uint = 0x00000408L;
pub const GDC_REG_SETUP_STATUS: c_uint = 0x0000040CL;
pub const GDC_REG_DDA_STATUS: c_uint = 0x00000410L;
pub const GDC_REG_ENGINE_STATUS: c_uint = 0x00000414L;
pub const GDC_REG_ERROR_STATUS: c_uint = 0x00000418L;
pub const GDC_REG_MODE_MISC: c_uint = 0x00000420L	/* MDR0 */;
pub const GDC_REG_MODE_LINE: c_uint = 0x00000424L	/* MDR1 */;
pub const GDC_REG_MODE_POLYGON: c_uint = 0x00000428L	/* MDR2 */;
pub const GDC_REG_MODE_TEXTURE: c_uint = 0x0000042CL	/* MDR3 */;
pub const GDC_REG_MODE_BITMAP: c_uint = 0x00000430L	/* MDR4 */;
pub const GDC_REG_MODE_EXTENSION: c_uint = 0x0000043CL	/* MDR7 */;
// Configuration Registers
pub const GDC_REG_DRAW_BASE: c_uint = 0x00000440L;
pub const GDC_REG_X_RESOLUTION: c_uint = 0x00000444L;
pub const GDC_REG_Z_BASE: c_uint = 0x00000448L;
pub const GDC_REG_TEXTURE_BASE: c_uint = 0x0000044CL;
pub const GDC_REG_POLYGON_FLAG_BASE: c_uint = 0x00000450L;
pub const GDC_REG_CLIP_XMIN: c_uint = 0x00000454L;
pub const GDC_REG_CLIP_XMAX: c_uint = 0x00000458L;
pub const GDC_REG_CLIP_YMIN: c_uint = 0x0000045CL;
pub const GDC_REG_CLIP_YMAX: c_uint = 0x00000460L;
pub const GDC_REG_TEXURE_SIZE: c_uint = 0x00000464L;
pub const GDC_REG_TILE_SIZE: c_uint = 0x00000468L;
pub const GDC_REG_TEX_BUF_OFFSET: c_uint = 0x0000046CL;
// for MB86293 or later
pub const GDC_REG_ALPHA_MAP_BASE: c_uint = 0x00000474L	/* ABR */;
// Constant Registers
pub const GDC_REG_FOREGROUND_COLOR: c_uint = 0x00000480L;
pub const GDC_REG_BACKGROUND_COLOR: c_uint = 0x00000484L;
pub const GDC_REG_ALPHA: c_uint = 0x00000488L;
pub const GDC_REG_LINE_PATTERN: c_uint = 0x0000048CL;
pub const GDC_REG_TEX_BORDER_COLOR: c_uint = 0x00000494L;
pub const GDC_REG_LINE_PATTERN_OFFSET: c_uint = 0x000003E0L;
// Coomand Code
pub const GDC_CMD_PIXEL: c_uint = 0x00000000L;
pub const GDC_CMD_PIXEL_Z: c_uint = 0x00000001L;
pub const GDC_CMD_X_VECTOR: c_uint = 0x00000020L;
pub const GDC_CMD_Y_VECTOR: c_uint = 0x00000021L;
pub const GDC_CMD_X_VECTOR_NOEND: c_uint = 0x00000022L;
pub const GDC_CMD_Y_VECTOR_NOEND: c_uint = 0x00000023L;
pub const GDC_CMD_X_VECTOR_BLPO: c_uint = 0x00000024L;
pub const GDC_CMD_Y_VECTOR_BLPO: c_uint = 0x00000025L;
pub const GDC_CMD_X_VECTOR_NOEND_BLPO: c_uint = 0x00000026L;
pub const GDC_CMD_Y_VECTOR_NOEND_BLPO: c_uint = 0x00000027L;
pub const GDC_CMD_AA_X_VECTOR: c_uint = 0x00000028L;
pub const GDC_CMD_AA_Y_VECTOR: c_uint = 0x00000029L;
pub const GDC_CMD_AA_X_VECTOR_NOEND: c_uint = 0x0000002AL;
pub const GDC_CMD_AA_Y_VECTOR_NOEND: c_uint = 0x0000002BL;
pub const GDC_CMD_AA_X_VECTOR_BLPO: c_uint = 0x0000002CL;
pub const GDC_CMD_AA_Y_VECTOR_BLPO: c_uint = 0x0000002DL;
pub const GDC_CMD_AA_X_VECTOR_NOEND_BLPO: c_uint = 0x0000002EL;
pub const GDC_CMD_AA_Y_VECTOR_NOEND_BLPO: c_uint = 0x0000002FL;
pub const GDC_CMD_0_VECTOR: c_uint = 0x00000030L;
pub const GDC_CMD_1_VECTOR: c_uint = 0x00000031L;
pub const GDC_CMD_0_VECTOR_NOEND: c_uint = 0x00000032L;
pub const GDC_CMD_1_VECTOR_NOEND: c_uint = 0x00000033L;
pub const GDC_CMD_0_VECTOR_BLPO: c_uint = 0x00000034L;
pub const GDC_CMD_1_VECTOR_BLPO: c_uint = 0x00000035L;
pub const GDC_CMD_0_VECTOR_NOEND_BLPO: c_uint = 0x00000036L;
pub const GDC_CMD_1_VECTOR_NOEND_BLPO: c_uint = 0x00000037L;
pub const GDC_CMD_AA_0_VECTOR: c_uint = 0x00000038L;
pub const GDC_CMD_AA_1_VECTOR: c_uint = 0x00000039L;
pub const GDC_CMD_AA_0_VECTOR_NOEND: c_uint = 0x0000003AL;
pub const GDC_CMD_AA_1_VECTOR_NOEND: c_uint = 0x0000003BL;
pub const GDC_CMD_AA_0_VECTOR_BLPO: c_uint = 0x0000003CL;
pub const GDC_CMD_AA_1_VECTOR_BLPO: c_uint = 0x0000003DL;
pub const GDC_CMD_AA_0_VECTOR_NOEND_BLPO: c_uint = 0x0000003EL;
pub const GDC_CMD_AA_1_VECTOR_NOEND_BLPO: c_uint = 0x0000003FL;
pub const GDC_CMD_BLT_FILL: c_uint = 0x00000041L;
pub const GDC_CMD_BLT_DRAW: c_uint = 0x00000042L;
pub const GDC_CMD_BITMAP: c_uint = 0x00000043L;
pub const GDC_CMD_BLTCOPY_TOP_LEFT: c_uint = 0x00000044L;
pub const GDC_CMD_BLTCOPY_TOP_RIGHT: c_uint = 0x00000045L;
pub const GDC_CMD_BLTCOPY_BOTTOM_LEFT: c_uint = 0x00000046L;
pub const GDC_CMD_BLTCOPY_BOTTOM_RIGHT: c_uint = 0x00000047L;
pub const GDC_CMD_LOAD_TEXTURE: c_uint = 0x00000048L;
pub const GDC_CMD_LOAD_TILE: c_uint = 0x00000049L;
pub const GDC_CMD_TRAP_RIGHT: c_uint = 0x00000060L;
pub const GDC_CMD_TRAP_LEFT: c_uint = 0x00000061L;
pub const GDC_CMD_TRIANGLE_FAN: c_uint = 0x00000062L;
pub const GDC_CMD_FLAG_TRIANGLE_FAN: c_uint = 0x00000063L;
pub const GDC_CMD_FLUSH_FB: c_uint = 0x000000C1L;
pub const GDC_CMD_FLUSH_Z: c_uint = 0x000000C2L;
pub const GDC_CMD_POLYGON_BEGIN: c_uint = 0x000000E0L;
pub const GDC_CMD_POLYGON_END: c_uint = 0x000000E1L;
pub const GDC_CMD_CLEAR_POLY_FLAG: c_uint = 0x000000E2L;
pub const GDC_CMD_NORMAL: c_uint = 0x000000FFL;
pub const GDC_CMD_VECTOR_BLPO_FLAG: c_uint = 0x00040000L;
pub const GDC_CMD_FAST_VECTOR_BLPO_FLAG: c_uint = 0x00000004L;
// for MB86293 or later
pub const GDC_CMD_MDR1: c_uint = 0x00000000L;
pub const GDC_CMD_MDR1S: c_uint = 0x00000002L;
pub const GDC_CMD_MDR1B: c_uint = 0x00000004L;
pub const GDC_CMD_MDR2: c_uint = 0x00000001L;
pub const GDC_CMD_MDR2S: c_uint = 0x00000003L;
pub const GDC_CMD_MDR2TL: c_uint = 0x00000007L;
pub const GDC_CMD_GMDR1E: c_uint = 0x00000010L;
pub const GDC_CMD_GMDR2E: c_uint = 0x00000020L;
pub const GDC_CMD_OVERLAP_SHADOW_XY: c_uint = 0x00000000L;
pub const GDC_CMD_OVERLAP_SHADOW_XY_COMPOSITION: c_uint = 0x00000001L;
pub const GDC_CMD_OVERLAP_Z_PACKED_ONBS: c_uint = 0x00000007L;
pub const GDC_CMD_OVERLAP_Z_ORIGIN: c_uint = 0x00000000L;
pub const GDC_CMD_OVERLAP_Z_NON_TOPLEFT: c_uint = 0x00000001L;
pub const GDC_CMD_OVERLAP_Z_BORDER: c_uint = 0x00000002L;
pub const GDC_CMD_OVERLAP_Z_SHADOW: c_uint = 0x00000003L;
pub const GDC_CMD_BLTCOPY_ALT_ALPHA: c_uint = 0x00000000L	/* Reserverd */;
pub const GDC_CMD_DC_LOGOUT: c_uint = 0x00000000L	/* Reserverd */;
pub const GDC_CMD_BODY_FORE_COLOR: c_uint = 0x00000000L;
pub const GDC_CMD_BODY_BACK_COLOR: c_uint = 0x00000001L;
pub const GDC_CMD_SHADOW_FORE_COLOR: c_uint = 0x00000002L;
pub const GDC_CMD_SHADOW_BACK_COLOR: c_uint = 0x00000003L;
pub const GDC_CMD_BORDER_FORE_COLOR: c_uint = 0x00000004L;
pub const GDC_CMD_BORDER_BACK_COLOR: c_uint = 0x00000005L;
// Type Code Table
pub const GDC_TYPE_G_NOP: c_uint = 0x00000020L;
pub const GDC_TYPE_G_BEGIN: c_uint = 0x00000021L;
pub const GDC_TYPE_G_BEGINCONT: c_uint = 0x00000022L;
pub const GDC_TYPE_G_END: c_uint = 0x00000023L;
pub const GDC_TYPE_G_VERTEX: c_uint = 0x00000030L;
pub const GDC_TYPE_G_VERTEXLOG: c_uint = 0x00000032L;
pub const GDC_TYPE_G_VERTEXNOPLOG: c_uint = 0x00000033L;
pub const GDC_TYPE_G_INIT: c_uint = 0x00000040L;
pub const GDC_TYPE_G_VIEWPORT: c_uint = 0x00000041L;
pub const GDC_TYPE_G_DEPTHRANGE: c_uint = 0x00000042L;
pub const GDC_TYPE_G_LOADMATRIX: c_uint = 0x00000043L;
pub const GDC_TYPE_G_VIEWVOLUMEXYCLIP: c_uint = 0x00000044L;
pub const GDC_TYPE_G_VIEWVOLUMEZCLIP: c_uint = 0x00000045L;
pub const GDC_TYPE_G_VIEWVOLUMEWCLIP: c_uint = 0x00000046L;
pub const GDC_TYPE_SETLVERTEX2I: c_uint = 0x00000072L;
pub const GDC_TYPE_SETLVERTEX2IP: c_uint = 0x00000073L;
pub const GDC_TYPE_SETMODEREGISTER: c_uint = 0x000000C0L;
pub const GDC_TYPE_SETGMODEREGISTER: c_uint = 0x000000C1L;
pub const GDC_TYPE_OVERLAPXYOFFT: c_uint = 0x000000C8L;
pub const GDC_TYPE_OVERLAPZOFFT: c_uint = 0x000000C9L;
pub const GDC_TYPE_DC_LOGOUTADDR: c_uint = 0x000000CCL;
pub const GDC_TYPE_SETCOLORREGISTER: c_uint = 0x000000CEL;
pub const GDC_TYPE_G_BEGINE: c_uint = 0x000000E1L;
pub const GDC_TYPE_G_BEGINCONTE: c_uint = 0x000000E2L;
pub const GDC_TYPE_G_ENDE: c_uint = 0x000000E3L;
pub const GDC_TYPE_DRAWPIXEL: c_uint = 0x00000000L;
pub const GDC_TYPE_DRAWPIXELZ: c_uint = 0x00000001L;
pub const GDC_TYPE_DRAWLINE: c_uint = 0x00000002L;
pub const GDC_TYPE_DRAWLINE2I: c_uint = 0x00000003L;
pub const GDC_TYPE_DRAWLINE2IP: c_uint = 0x00000004L;
pub const GDC_TYPE_DRAWTRAP: c_uint = 0x00000005L;
pub const GDC_TYPE_DRAWVERTEX2I: c_uint = 0x00000006L;
pub const GDC_TYPE_DRAWVERTEX2IP: c_uint = 0x00000007L;
pub const GDC_TYPE_DRAWRECTP: c_uint = 0x00000009L;
pub const GDC_TYPE_DRAWBITMAPP: c_uint = 0x0000000BL;
pub const GDC_TYPE_BLTCOPYP: c_uint = 0x0000000DL;
pub const GDC_TYPE_BLTCOPYALTERNATEP: c_uint = 0x0000000FL;
pub const GDC_TYPE_LOADTEXTUREP: c_uint = 0x00000011L;
pub const GDC_TYPE_BLTTEXTUREP: c_uint = 0x00000013L;
pub const GDC_TYPE_BLTCOPYALTALPHABLENDP: c_uint = 0x0000001FL;
pub const GDC_TYPE_SETVERTEX2I: c_uint = 0x00000070L;
pub const GDC_TYPE_SETVERTEX2IP: c_uint = 0x00000071L;
pub const GDC_TYPE_DRAW: c_uint = 0x000000F0L;
pub const GDC_TYPE_SETREGISTER: c_uint = 0x000000F1L;
pub const GDC_TYPE_SYNC: c_uint = 0x000000FCL;
pub const GDC_TYPE_INTERRUPT: c_uint = 0x000000FDL;
pub const GDC_TYPE_NOP: c_uint = 0x0;
// Raster operation
pub const GDC_ROP_CLEAR: c_uint = 0x0000;
pub const GDC_ROP_AND: c_uint = 0x0001;
pub const GDC_ROP_AND_REVERSE: c_uint = 0x0002;
pub const GDC_ROP_COPY: c_uint = 0x0003;
pub const GDC_ROP_AND_INVERTED: c_uint = 0x0004;
pub const GDC_ROP_NOP: c_uint = 0x0005;
pub const GDC_ROP_XOR: c_uint = 0x0006;
pub const GDC_ROP_OR: c_uint = 0x0007;
pub const GDC_ROP_NOR: c_uint = 0x0008;
pub const GDC_ROP_EQUIV: c_uint = 0x0009;
pub const GDC_ROP_INVERT: c_uint = 0x000A;
pub const GDC_ROP_OR_REVERSE: c_uint = 0x000B;
pub const GDC_ROP_COPY_INVERTED: c_uint = 0x000C;
pub const GDC_ROP_OR_INVERTED: c_uint = 0x000D;
pub const GDC_ROP_NAND: c_uint = 0x000E;
pub const GDC_ROP_SET: c_uint = 0x000F;
