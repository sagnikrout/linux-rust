//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/pm3fb.h
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
// linux/drivers/video/pm3fb.h -- 3DLabs Permedia3 frame buffer device
//
// Copyright (C) 2001 Romain Dolbeau <dolbeau@irisa.fr>
// Copyright (C) 2001 Sven Luther, <luther@dpt-info.u-strasbg.fr>
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// GLINT Permedia3 Control Status registers
//
// Control Status Registers
pub const PM3ResetStatus: c_uint = 0x0000;
pub const PM3IntEnable: c_uint = 0x0008;
pub const PM3IntFlags: c_uint = 0x0010;
pub const PM3InFIFOSpace: c_uint = 0x0018;
pub const PM3OutFIFOWords: c_uint = 0x0020;
pub const PM3DMAAddress: c_uint = 0x0028;
pub const PM3DMACount: c_uint = 0x0030;
pub const PM3ErrorFlags: c_uint = 0x0038;
pub const PM3VClkCtl: c_uint = 0x0040;
pub const PM3TestRegister: c_uint = 0x0048;
pub const PM3Aperture0: c_uint = 0x0050;
pub const PM3Aperture1: c_uint = 0x0058;
pub const PM3DMAControl: c_uint = 0x0060;
pub const PM3FIFODis: c_uint = 0x0068;
pub const PM3ChipConfig: c_uint = 0x0070;
pub const PM3AGPControl: c_uint = 0x0078;
pub const PM3GPOutDMAAddress: c_uint = 0x0080;
pub const PM3PCIFeedbackCount: c_uint = 0x0088;
pub const PM3PCIAbortStatus: c_uint = 0x0090;
pub const PM3PCIAbortAddress: c_uint = 0x0098;
pub const PM3PCIPLLStatus: c_uint = 0x00f0;
pub const PM3HostTextureAddress: c_uint = 0x0100;
pub const PM3TextureDownloadControl: c_uint = 0x0108;
pub const PM3TextureOperation: c_uint = 0x0110;
pub const PM3LogicalTexturePage: c_uint = 0x0118;
pub const PM3TexDMAAddress: c_uint = 0x0120;
pub const PM3TexFIFOSpace: c_uint = 0x0128;
//
// GLINT Permedia3 Region 0 Bypass Controls
//
pub const PM3ByAperture1Mode: c_uint = 0x0300;

pub const PM3ByAperture2Mode: c_uint = 0x0328;
//
// GLINT Permedia3 Memory Control (0x1000)
//
pub const PM3MemCounter: c_uint = 0x1000;
pub const PM3MemBypassWriteMask: c_uint = 0x1008;
pub const PM3MemScratch: c_uint = 0x1010;
pub const PM3LocalMemCaps: c_uint = 0x1018;

pub const PM3LocalMemTimings: c_uint = 0x1020;
pub const PM3LocalMemControl: c_uint = 0x1028;
pub const PM3LocalMemRefresh: c_uint = 0x1030;
pub const PM3LocalMemPowerDown: c_uint = 0x1038;
pub const PM3RemoteMemControl: c_uint = 0x1100;
//
// GLINT Permedia3 Video Control (0x3000)
//
pub const PM3ScreenBase: c_uint = 0x3000;
pub const PM3ScreenStride: c_uint = 0x3008;
pub const PM3HTotal: c_uint = 0x3010;
pub const PM3HgEnd: c_uint = 0x3018;
pub const PM3HbEnd: c_uint = 0x3020;
pub const PM3HsStart: c_uint = 0x3028;
pub const PM3HsEnd: c_uint = 0x3030;
pub const PM3VTotal: c_uint = 0x3038;
pub const PM3VbEnd: c_uint = 0x3040;
pub const PM3VsStart: c_uint = 0x3048;
pub const PM3VsEnd: c_uint = 0x3050;
pub const PM3VideoControl: c_uint = 0x3058;

pub const PM3InterruptLine: c_uint = 0x3060;
pub const PM3DisplayData: c_uint = 0x3068;
pub const PM3VerticalLineCount: c_uint = 0x3070;
pub const PM3FifoControl: c_uint = 0x3078;
pub const PM3ScreenBaseRight: c_uint = 0x3080;
pub const PM3MiscControl: c_uint = 0x3088;
pub const PM3VideoOverlayUpdate: c_uint = 0x3100;

pub const PM3VideoOverlayMode: c_uint = 0x3108;

pub const PM3VideoOverlayFifoControl: c_uint = 0x3110;
pub const PM3VideoOverlayIndex: c_uint = 0x3118;
pub const PM3VideoOverlayBase0: c_uint = 0x3120;
pub const PM3VideoOverlayBase1: c_uint = 0x3128;
pub const PM3VideoOverlayBase2: c_uint = 0x3130;
pub const PM3VideoOverlayStride: c_uint = 0x3138;

pub const PM3VideoOverlayWidth: c_uint = 0x3140;

pub const PM3VideoOverlayHeight: c_uint = 0x3148;

pub const PM3VideoOverlayOrigin: c_uint = 0x3150;

pub const PM3VideoOverlayShrinkXDelta: c_uint = 0x3158;

pub const PM3VideoOverlayZoomXDelta: c_uint = 0x3160;

pub const PM3VideoOverlayYDelta: c_uint = 0x3168;

pub const PM3VideoOverlayFieldOffset: c_uint = 0x3170;
pub const PM3VideoOverlayStatus: c_uint = 0x3178;
//
// GLINT Permedia3 RAMDAC Registers (0x4000)
//
// Direct Registers
pub const PM3RD_PaletteWriteAddress: c_uint = 0x4000;
pub const PM3RD_PaletteData: c_uint = 0x4008;
pub const PM3RD_PixelMask: c_uint = 0x4010;
pub const PM3RD_PaletteReadAddress: c_uint = 0x4018;
pub const PM3RD_IndexLow: c_uint = 0x4020;
pub const PM3RD_IndexHigh: c_uint = 0x4028;
pub const PM3RD_IndexedData: c_uint = 0x4030;
pub const PM3RD_IndexControl: c_uint = 0x4038;

// Indirect Registers
pub const PM3RD_MiscControl: c_uint = 0x000;

pub const PM3RD_SyncControl: c_uint = 0x001;

pub const PM3RD_DACControl: c_uint = 0x002;

pub const PM3RD_PixelSize: c_uint = 0x003;

pub const PM3RD_ColorFormat: c_uint = 0x004;

pub const PM3RD_CursorMode: c_uint = 0x005;

pub const PM3RD_CursorControl: c_uint = 0x006;

pub const PM3RD_CursorXLow: c_uint = 0x007;
pub const PM3RD_CursorXHigh: c_uint = 0x008;
pub const PM3RD_CursorYLow: c_uint = 0x009;
pub const PM3RD_CursorYHigh: c_uint = 0x00a;
pub const PM3RD_CursorHotSpotX: c_uint = 0x00b;
pub const PM3RD_CursorHotSpotY: c_uint = 0x00c;
pub const PM3RD_OverlayKey: c_uint = 0x00d;
pub const PM3RD_Pan: c_uint = 0x00e;

pub const PM3RD_Sense: c_uint = 0x00f;
pub const PM3RD_CheckControl: c_uint = 0x018;

pub const PM3RD_CheckPixelRed: c_uint = 0x019;
pub const PM3RD_CheckPixelGreen: c_uint = 0x01a;
pub const PM3RD_CheckPixelBlue: c_uint = 0x01b;
pub const PM3RD_CheckLUTRed: c_uint = 0x01c;
pub const PM3RD_CheckLUTGreen: c_uint = 0x01d;
pub const PM3RD_CheckLUTBlue: c_uint = 0x01e;
pub const PM3RD_Scratch: c_uint = 0x01f;
pub const PM3RD_VideoOverlayControl: c_uint = 0x020;

pub const PM3RD_VideoOverlayXStartLow: c_uint = 0x021;
pub const PM3RD_VideoOverlayXStartHigh: c_uint = 0x022;
pub const PM3RD_VideoOverlayYStartLow: c_uint = 0x023;
pub const PM3RD_VideoOverlayYStartHigh: c_uint = 0x024;
pub const PM3RD_VideoOverlayXEndLow: c_uint = 0x025;
pub const PM3RD_VideoOverlayXEndHigh: c_uint = 0x026;
pub const PM3RD_VideoOverlayYEndLow: c_uint = 0x027;
pub const PM3RD_VideoOverlayYEndHigh: c_uint = 0x028;
pub const PM3RD_VideoOverlayKeyR: c_uint = 0x029;
pub const PM3RD_VideoOverlayKeyG: c_uint = 0x02a;
pub const PM3RD_VideoOverlayKeyB: c_uint = 0x02b;
pub const PM3RD_VideoOverlayBlend: c_uint = 0x02c;

pub const PM3RD_DClkSetup1: c_uint = 0x1f0;
pub const PM3RD_DClkSetup2: c_uint = 0x1f1;
pub const PM3RD_KClkSetup1: c_uint = 0x1f2;
pub const PM3RD_KClkSetup2: c_uint = 0x1f3;
pub const PM3RD_DClkControl: c_uint = 0x200;

pub const PM3RD_DClk0PreScale: c_uint = 0x201;
pub const PM3RD_DClk0FeedbackScale: c_uint = 0x202;
pub const PM3RD_DClk0PostScale: c_uint = 0x203;
pub const PM3_REF_CLOCK: c_int = 14318;
pub const PM3RD_DClk1PreScale: c_uint = 0x204;
pub const PM3RD_DClk1FeedbackScale: c_uint = 0x205;
pub const PM3RD_DClk1PostScale: c_uint = 0x206;
pub const PM3RD_DClk2PreScale: c_uint = 0x207;
pub const PM3RD_DClk2FeedbackScale: c_uint = 0x208;
pub const PM3RD_DClk2PostScale: c_uint = 0x209;
pub const PM3RD_DClk3PreScale: c_uint = 0x20a;
pub const PM3RD_DClk3FeedbackScale: c_uint = 0x20b;
pub const PM3RD_DClk3PostScale: c_uint = 0x20c;
pub const PM3RD_KClkControl: c_uint = 0x20d;

pub const PM3RD_KClkPreScale: c_uint = 0x20e;
pub const PM3RD_KClkFeedbackScale: c_uint = 0x20f;
pub const PM3RD_KClkPostScale: c_uint = 0x210;
pub const PM3RD_MClkControl: c_uint = 0x211;

pub const PM3RD_MClkPreScale: c_uint = 0x212;
pub const PM3RD_MClkFeedbackScale: c_uint = 0x213;
pub const PM3RD_MClkPostScale: c_uint = 0x214;
pub const PM3RD_SClkControl: c_uint = 0x215;

pub const PM3RD_SClkPreScale: c_uint = 0x216;
pub const PM3RD_SClkFeedbackScale: c_uint = 0x217;
pub const PM3RD_SClkPostScale: c_uint = 0x218;

//
// GLINT Permedia3 Video Streaming Registers (0x5000)
//
pub const PM3VSConfiguration: c_uint = 0x5800;
//
// GLINT Permedia3 Core Registers (0x8000+)
//
pub const PM3AALineWidth: c_uint = 0x94c0;
pub const PM3AAPointsize: c_uint = 0x94a0;
pub const PM3AlphaBlendAlphaMode: c_uint = 0xafa8;
pub const PM3AlphaBlendAlphaModeAnd: c_uint = 0xad30;
pub const PM3AlphaBlendAlphaModeOr: c_uint = 0xad38;
pub const PM3AlphaBlendColorMode: c_uint = 0xafa0;
pub const PM3AlphaBlendColorModeAnd: c_uint = 0xacb0;
pub const PM3AlphaBlendColorModeOr: c_uint = 0xacb8;
pub const PM3AlphaDestColor: c_uint = 0xaf88;
pub const PM3AlphaSourceColor: c_uint = 0xaf80;
pub const PM3AlphaTestMode: c_uint = 0x8800;
pub const PM3AlphaTestModeAnd: c_uint = 0xabf0;
pub const PM3AlphaTestModeOr: c_uint = 0xabf8;
pub const PM3AntialiasMode: c_uint = 0x8808;
pub const PM3AntialiasModeAnd: c_uint = 0xac00;
pub const PM3AntialiasModeOr: c_uint = 0xac08;
// ...
pub const PM3BackgroundColor: c_uint = 0xb0c8;
// ...
pub const PM3ColorDDAMode: c_uint = 0x87e0;
pub const PM3ColorDDAModeAnd: c_uint = 0xabe0;
pub const PM3ColorDDAModeOr: c_uint = 0xabe8;
pub const PM3CommandInterrupt: c_uint = 0xa990;
pub const PM3ConstantColorDDA: c_uint = 0xafb0;

pub const PM3ContextData: c_uint = 0x8dd0;
pub const PM3ContextDump: c_uint = 0x8dc0;
pub const PM3ContextRestore: c_uint = 0x8dc8;
pub const PM3Continue: c_uint = 0x8058;
pub const PM3ContinueNewDom: c_uint = 0x8048;
pub const PM3ContinueNewLine: c_uint = 0x8040;
pub const PM3ContinueNewSub: c_uint = 0x8050;
pub const PM3Count: c_uint = 0x8030;
// ...
pub const PM3DeltaControl: c_uint = 0x9350;
pub const PM3DeltaControlAnd: c_uint = 0xab20;
pub const PM3DeltaControlOr: c_uint = 0xab28;
pub const PM3DeltaMode: c_uint = 0x9300;
pub const PM3DeltaModeAnd: c_uint = 0xaad0;
pub const PM3DeltaModeOr: c_uint = 0xaad8;
// ...
pub const PM3DitherMode: c_uint = 0x8818;
pub const PM3DitherModeAnd: c_uint = 0xacd0;
pub const PM3DitherModeOr: c_uint = 0xacd8;
// ...
pub const PM3dXDom: c_uint = 0x8008;
pub const PM3dXSub: c_uint = 0x8018;
pub const PM3dY: c_uint = 0x8028;
// ...
pub const PM3FBBlockColor: c_uint = 0x8ac8;
pub const PM3FBBlockColor0: c_uint = 0xb060;
pub const PM3FBBlockColor1: c_uint = 0xb068;
pub const PM3FBBlockColor2: c_uint = 0xb070;
pub const PM3FBBlockColor3: c_uint = 0xb078;
pub const PM3FBBlockColorBack: c_uint = 0xb0a0;
pub const PM3FBBlockColorBack0: c_uint = 0xb080;
pub const PM3FBBlockColorBack1: c_uint = 0xb088;
pub const PM3FBBlockColorBack2: c_uint = 0xb090;
pub const PM3FBBlockColorBack3: c_uint = 0xb098;
pub const PM3FBColor: c_uint = 0x8a98;
pub const PM3FBDestReadBufferAddr0: c_uint = 0xae80;
pub const PM3FBDestReadBufferAddr1: c_uint = 0xae88;
pub const PM3FBDestReadBufferAddr2: c_uint = 0xae90;
pub const PM3FBDestReadBufferAddr3: c_uint = 0xae98;
pub const PM3FBDestReadBufferOffset0: c_uint = 0xaea0;
pub const PM3FBDestReadBufferOffset1: c_uint = 0xaea8;
pub const PM3FBDestReadBufferOffset2: c_uint = 0xaeb0;
pub const PM3FBDestReadBufferOffset3: c_uint = 0xaeb8;

pub const PM3FBDestReadBufferWidth0: c_uint = 0xaec0;
pub const PM3FBDestReadBufferWidth1: c_uint = 0xaec8;
pub const PM3FBDestReadBufferWidth2: c_uint = 0xaed0;
pub const PM3FBDestReadBufferWidth3: c_uint = 0xaed8;

pub const PM3FBDestReadEnables: c_uint = 0xaee8;
pub const PM3FBDestReadEnablesAnd: c_uint = 0xad20;
pub const PM3FBDestReadEnablesOr: c_uint = 0xad28;

pub const PM3FBDestReadMode: c_uint = 0xaee0;
pub const PM3FBDestReadModeAnd: c_uint = 0xac90;
pub const PM3FBDestReadModeOr: c_uint = 0xac98;

pub const PM3FBHardwareWriteMask: c_uint = 0x8ac0;
pub const PM3FBSoftwareWriteMask: c_uint = 0x8820;
pub const PM3FBData: c_uint = 0x8aa0;
pub const PM3FBSourceData: c_uint = 0x8aa8;
pub const PM3FBSourceReadBufferAddr: c_uint = 0xaf08;
pub const PM3FBSourceReadBufferOffset: c_uint = 0xaf10;

pub const PM3FBSourceReadBufferWidth: c_uint = 0xaf18;

pub const PM3FBSourceReadMode: c_uint = 0xaf00;
pub const PM3FBSourceReadModeAnd: c_uint = 0xaca0;
pub const PM3FBSourceReadModeOr: c_uint = 0xaca8;

pub const PM3FBWriteBufferAddr0: c_uint = 0xb000;
pub const PM3FBWriteBufferAddr1: c_uint = 0xb008;
pub const PM3FBWriteBufferAddr2: c_uint = 0xb010;
pub const PM3FBWriteBufferAddr3: c_uint = 0xb018;
pub const PM3FBWriteBufferOffset0: c_uint = 0xb020;
pub const PM3FBWriteBufferOffset1: c_uint = 0xb028;
pub const PM3FBWriteBufferOffset2: c_uint = 0xb030;
pub const PM3FBWriteBufferOffset3: c_uint = 0xb038;

pub const PM3FBWriteBufferWidth0: c_uint = 0xb040;
pub const PM3FBWriteBufferWidth1: c_uint = 0xb048;
pub const PM3FBWriteBufferWidth2: c_uint = 0xb050;
pub const PM3FBWriteBufferWidth3: c_uint = 0xb058;

pub const PM3FBWriteMode: c_uint = 0x8ab8;
pub const PM3FBWriteModeAnd: c_uint = 0xacf0;
pub const PM3FBWriteModeOr: c_uint = 0xacf8;

pub const PM3ForegroundColor: c_uint = 0xb0c0;
// ...
pub const PM3GIDMode: c_uint = 0xb538;
pub const PM3GIDModeAnd: c_uint = 0xb5b0;
pub const PM3GIDModeOr: c_uint = 0xb5b8;
// ...
pub const PM3LBDestReadBufferAddr: c_uint = 0xb510;
pub const PM3LBDestReadBufferOffset: c_uint = 0xb518;
pub const PM3LBDestReadEnables: c_uint = 0xb508;
pub const PM3LBDestReadEnablesAnd: c_uint = 0xb590;
pub const PM3LBDestReadEnablesOr: c_uint = 0xb598;
pub const PM3LBDestReadMode: c_uint = 0xb500;
pub const PM3LBDestReadModeAnd: c_uint = 0xb580;
pub const PM3LBDestReadModeOr: c_uint = 0xb588;

pub const PM3LBReadFormat: c_uint = 0x8888;

pub const PM3LBSourceReadBufferAddr: c_uint = 0xb528;
pub const PM3LBSourceReadBufferOffset: c_uint = 0xb530;
pub const PM3LBSourceReadMode: c_uint = 0xb520;
pub const PM3LBSourceReadModeAnd: c_uint = 0xb5a0;
pub const PM3LBSourceReadModeOr: c_uint = 0xb5a8;

pub const PM3LBStencil: c_uint = 0x88a8;
pub const PM3LBWriteBufferAddr: c_uint = 0xb540;
pub const PM3LBWriteBufferOffset: c_uint = 0xb548;
pub const PM3LBWriteFormat: c_uint = 0x88c8;

pub const PM3LBWriteMode: c_uint = 0x88c0;
pub const PM3LBWriteModeAnd: c_uint = 0xac80;
pub const PM3LBWriteModeOr: c_uint = 0xac88;

// ...
pub const PM3LineStippleMode: c_uint = 0x81a8;
pub const PM3LineStippleModeAnd: c_uint = 0xabc0;
pub const PM3LineStippleModeOr: c_uint = 0xabc8;
pub const PM3LoadLineStippleCounters: c_uint = 0x81b0;
// ...
pub const PM3LogicalOpMode: c_uint = 0x8828;
pub const PM3LogicalOpModeAnd: c_uint = 0xace0;
pub const PM3LogicalOpModeOr: c_uint = 0xace8;

// ...
pub const PM3LUT: c_uint = 0x8e80;
pub const PM3LUTAddress: c_uint = 0x84d0;
pub const PM3LUTData: c_uint = 0x84c8;
pub const PM3LUTIndex: c_uint = 0x84c0;
pub const PM3LUTMode: c_uint = 0xb378;
pub const PM3LUTModeAnd: c_uint = 0xad70;
pub const PM3LUTModeOr: c_uint = 0xad78;
pub const PM3LUTTransfer: c_uint = 0x84d8;
// ...
pub const PM3PixelSize: c_uint = 0x80c0;

// ...
pub const PM3Render: c_uint = 0x8038;

pub const PM3RasterizerMode: c_uint = 0x80a0;
pub const PM3RasterizerModeAnd: c_uint = 0xaba0;
pub const PM3RasterizerModeOr: c_uint = 0xaba8;
pub const PM3RectangleHeight: c_uint = 0x94e0;
pub const PM3RepeatLine: c_uint = 0x9328;
pub const PM3ResetPickResult: c_uint = 0x8c20;
pub const PM3RLEMask: c_uint = 0x8c48;
pub const PM3RouterMode: c_uint = 0x8840;
pub const PM3RStart: c_uint = 0x8780;
pub const PM3S1Start: c_uint = 0x8400;
pub const PM3aveLineStippleCounters: c_uint = 0x81c0;
pub const PM3ScissorMaxXY: c_uint = 0x8190;
pub const PM3ScissorMinXY: c_uint = 0x8188;
pub const PM3ScissorMode: c_uint = 0x8180;
pub const PM3ScissorModeAnd: c_uint = 0xabb0;
pub const PM3ScissorModeOr: c_uint = 0xabb8;
pub const PM3ScreenSize: c_uint = 0x8198;
pub const PM3Security: c_uint = 0x8908;
pub const PM3SetLogicalTexturePage: c_uint = 0xb360;
pub const PM3SizeOfFramebuffer: c_uint = 0xb0a8;
pub const PM3SStart: c_uint = 0x8388;
pub const PM3StartXDom: c_uint = 0x8000;
pub const PM3StartXSub: c_uint = 0x8010;
pub const PM3StartY: c_uint = 0x8020;
// ...
pub const PM3SpanColorMask: c_uint = 0x8168;
// ...
pub const PM3TextureApplicationMode: c_uint = 0x8680;
pub const PM3TextureApplicationModeAnd: c_uint = 0xac50;
pub const PM3TextureApplicationModeOr: c_uint = 0xac58;
pub const PM3TextureBaseAddr: c_uint = 0x8500;
pub const PM3TextureCacheControl: c_uint = 0x8490;
pub const PM3TextureChromaLower0: c_uint = 0x84f0;
pub const PM3TextureChromaLower1: c_uint = 0x8608;
pub const PM3TextureChromaUpper0: c_uint = 0x84e8;
pub const PM3TextureChromaUpper1: c_uint = 0x8600;
pub const PM3TextureCompositeAlphaMode0: c_uint = 0xb310;
pub const PM3TextureCompositeAlphaMode0And: c_uint = 0xb390;
pub const PM3TextureCompositeAlphaMode0Or: c_uint = 0xb398;
pub const PM3TextureCompositeAlphaMode1: c_uint = 0xb320;
pub const PM3TextureCompositeAlphaMode1And: c_uint = 0xb3b0;
pub const PM3TextureCompositeAlphaMode1Or: c_uint = 0xb3b8;
pub const PM3TextureCompositeColorMode0: c_uint = 0xb308;
pub const PM3TextureCompositeColorMode0And: c_uint = 0xb380;
pub const PM3TextureCompositeColorMode0Or: c_uint = 0xb388;
pub const PM3TextureCompositeColorMode1: c_uint = 0xb318;
pub const PM3TextureCompositeColorMode1And: c_uint = 0xb3a0;
pub const PM3TextureCompositeColorMode1Or: c_uint = 0xb3a8;
pub const PM3TextureCompositeFactor0: c_uint = 0xb328;
pub const PM3TextureCompositeFactor1: c_uint = 0xb330;
pub const PM3TextureCompositeMode: c_uint = 0xb300;
pub const PM3TextureCoordMode: c_uint = 0x8380;
pub const PM3TextureCoordModeAnd: c_uint = 0xac20;
pub const PM3TextureCoordModeOr: c_uint = 0xac28;
pub const PM3TextureData: c_uint = 0x88e8;
//
pub const PM3TextureDownloadControl: c_uint = 0x0108;
//
pub const PM3TextureDownloadOffset: c_uint = 0x88f0;
pub const PM3TextureEnvColor: c_uint = 0x8688;
pub const PM3TextureFilterMode: c_uint = 0x84e0;
pub const PM3TextureFilterModeAnd: c_uint = 0xad50;
pub const PM3TextureFilterModeOr: c_uint = 0xad58;
pub const PM3TextureIndexMode0: c_uint = 0xb338;
pub const PM3TextureIndexMode0And: c_uint = 0xb3c0;
pub const PM3TextureIndexMode0Or: c_uint = 0xb3c8;
pub const PM3TextureIndexMode1: c_uint = 0xb340;
pub const PM3TextureIndexMode1And: c_uint = 0xb3d0;
pub const PM3TextureIndexMode1Or: c_uint = 0xb3d8;
// ...
pub const PM3TextureMapSize: c_uint = 0xb428;
pub const PM3TextureMapWidth0: c_uint = 0x8580;
pub const PM3TextureMapWidth1: c_uint = 0x8588;

pub const PM3TextureReadMode0: c_uint = 0xb400;
pub const PM3TextureReadMode0And: c_uint = 0xac30;
pub const PM3TextureReadMode0Or: c_uint = 0xac38;
pub const PM3TextureReadMode1: c_uint = 0xb408;
pub const PM3TextureReadMode1And: c_uint = 0xad40;
pub const PM3TextureReadMode1Or: c_uint = 0xad48;
// ...
pub const PM3WaitForCompletion: c_uint = 0x80b8;
pub const PM3Window: c_uint = 0x8980;

pub const PM3WindowAnd: c_uint = 0xab80;
pub const PM3WindowOr: c_uint = 0xab88;
pub const PM3WindowOrigin: c_uint = 0x81c8;
pub const PM3XBias: c_uint = 0x9480;
pub const PM3YBias: c_uint = 0x9488;
pub const PM3YLimits: c_uint = 0x80a8;
pub const PM3UVMode: c_uint = 0x8f00;
pub const PM3ZFogBias: c_uint = 0x86b8;
pub const PM3ZStart: c_uint = 0xadd8;
pub const PM3ZStartL: c_uint = 0x89b8;
pub const PM3ZStartU: c_uint = 0x89b0;
//
// GLINT Permedia3 2D setup Unit
//
pub const PM3Config2D: c_uint = 0xb618;

pub const PM3DownloadGlyphwidth: c_uint = 0xb658;

pub const PM3DownloadTarget: c_uint = 0xb650;

pub const PM3GlyphData: c_uint = 0xb660;
pub const PM3GlyphPosition: c_uint = 0xb608;

pub const PM3Packed4Pixels: c_uint = 0xb668;
pub const PM3Packed8Pixels: c_uint = 0xb630;
pub const PM3Packed16Pixels: c_uint = 0xb638;
pub const PM3RectanglePosition: c_uint = 0xb600;

pub const PM3Render2D: c_uint = 0xb640;

pub const PM3Render2DGlyph: c_uint = 0xb648;

pub const PM3RenderPatchOffset: c_uint = 0xb610;

pub const PM3RLCount: c_uint = 0xb678;

pub const PM3RLData: c_uint = 0xb670;
//
// GLINT Permedia3 Alias Register
//
pub const PM3FillBackgroundColor: c_uint = 0x8330;
pub const PM3FillConfig2D0: c_uint = 0x8338;
pub const PM3FillConfig2D1: c_uint = 0x8360;

pub const PM3FillFBDestReadBufferAddr: c_uint = 0x8310;
pub const PM3FillFBSourceReadBufferAddr: c_uint = 0x8308;
pub const PM3FillFBSourceReadBufferOffset: c_uint = 0x8340;

pub const PM3FillFBWriteBufferAddr: c_uint = 0x8300;
pub const PM3FillForegroundColor0: c_uint = 0x8328;
pub const PM3FillForegroundColor1: c_uint = 0x8358;
pub const PM3FillGlyphPosition: c_uint = 0x8368;

pub const PM3FillRectanglePosition: c_uint = 0x8348;

// a few more useful registers & regs value...
pub const PM3Sync: c_uint = 0x8c40;
pub const PM3Sync_Tag: c_uint = 0x188;
pub const PM3FilterMode: c_uint = 0x8c00;
pub const PM3FilterModeSync: c_uint = 0x400;
pub const PM3OutputFifo: c_uint = 0x2000;
pub const PM3StatisticMode: c_uint = 0x8c08;
pub const PM3AreaStippleMode: c_uint = 0x81a0;

pub const PM3DepthMode: c_uint = 0x89a0;
pub const PM3StencilMode: c_uint = 0x8988;
pub const PM3StencilData: c_uint = 0x8990;
pub const PM3TextureReadMode: c_uint = 0x8670;
pub const PM3FogMode: c_uint = 0x8690;
pub const PM3ChromaTestMode: c_uint = 0x8f18;
pub const PM3YUVMode: c_uint = 0x8f00;
pub const PM3BitMaskPattern: c_uint = 0x8068;
// *****************************
// ***** pm3fb IOCTL const *****
// *****************************
pub const PM3FBIO_RESETCHIP: c_uint = 0x504D33FF /* 'PM3\377' */;
// *****************************************
// ***** pm3fb useful define and macro *****
// *****************************************
// fifo size in chip
pub const PM3_FIFO_SIZE: c_int = 120;
pub const PM3_REGS_SIZE: c_uint = 0x10000;
pub const PM3_MAX_PIXCLOCK: c_int = 300000;
