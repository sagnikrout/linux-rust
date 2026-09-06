//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/bridge/synopsys/dw-hdmi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2011 Freescale Semiconductor, Inc.
//
// Identification Registers
pub const HDMI_DESIGN_ID: c_uint = 0x0000;
pub const HDMI_REVISION_ID: c_uint = 0x0001;
pub const HDMI_PRODUCT_ID0: c_uint = 0x0002;
pub const HDMI_PRODUCT_ID1: c_uint = 0x0003;
pub const HDMI_CONFIG0_ID: c_uint = 0x0004;
pub const HDMI_CONFIG1_ID: c_uint = 0x0005;
pub const HDMI_CONFIG2_ID: c_uint = 0x0006;
pub const HDMI_CONFIG3_ID: c_uint = 0x0007;
// Interrupt Registers
pub const HDMI_IH_FC_STAT0: c_uint = 0x0100;
pub const HDMI_IH_FC_STAT1: c_uint = 0x0101;
pub const HDMI_IH_FC_STAT2: c_uint = 0x0102;
pub const HDMI_IH_AS_STAT0: c_uint = 0x0103;
pub const HDMI_IH_PHY_STAT0: c_uint = 0x0104;
pub const HDMI_IH_I2CM_STAT0: c_uint = 0x0105;
pub const HDMI_IH_CEC_STAT0: c_uint = 0x0106;
pub const HDMI_IH_VP_STAT0: c_uint = 0x0107;
pub const HDMI_IH_I2CMPHY_STAT0: c_uint = 0x0108;
pub const HDMI_IH_AHBDMAAUD_STAT0: c_uint = 0x0109;
pub const HDMI_IH_MUTE_FC_STAT0: c_uint = 0x0180;
pub const HDMI_IH_MUTE_FC_STAT1: c_uint = 0x0181;
pub const HDMI_IH_MUTE_FC_STAT2: c_uint = 0x0182;
pub const HDMI_IH_MUTE_AS_STAT0: c_uint = 0x0183;
pub const HDMI_IH_MUTE_PHY_STAT0: c_uint = 0x0184;
pub const HDMI_IH_MUTE_I2CM_STAT0: c_uint = 0x0185;
pub const HDMI_IH_MUTE_CEC_STAT0: c_uint = 0x0186;
pub const HDMI_IH_MUTE_VP_STAT0: c_uint = 0x0187;
pub const HDMI_IH_MUTE_I2CMPHY_STAT0: c_uint = 0x0188;
pub const HDMI_IH_MUTE_AHBDMAAUD_STAT0: c_uint = 0x0189;
pub const HDMI_IH_MUTE: c_uint = 0x01FF;
// Video Sample Registers
pub const HDMI_TX_INVID0: c_uint = 0x0200;
pub const HDMI_TX_INSTUFFING: c_uint = 0x0201;
pub const HDMI_TX_GYDATA0: c_uint = 0x0202;
pub const HDMI_TX_GYDATA1: c_uint = 0x0203;
pub const HDMI_TX_RCRDATA0: c_uint = 0x0204;
pub const HDMI_TX_RCRDATA1: c_uint = 0x0205;
pub const HDMI_TX_BCBDATA0: c_uint = 0x0206;
pub const HDMI_TX_BCBDATA1: c_uint = 0x0207;
// Video Packetizer Registers
pub const HDMI_VP_STATUS: c_uint = 0x0800;
pub const HDMI_VP_PR_CD: c_uint = 0x0801;
pub const HDMI_VP_STUFF: c_uint = 0x0802;
pub const HDMI_VP_REMAP: c_uint = 0x0803;
pub const HDMI_VP_CONF: c_uint = 0x0804;
pub const HDMI_VP_STAT: c_uint = 0x0805;
pub const HDMI_VP_INT: c_uint = 0x0806;
pub const HDMI_VP_MASK: c_uint = 0x0807;
pub const HDMI_VP_POL: c_uint = 0x0808;
// Frame Composer Registers
pub const HDMI_FC_INVIDCONF: c_uint = 0x1000;
pub const HDMI_FC_INHACTV0: c_uint = 0x1001;
pub const HDMI_FC_INHACTV1: c_uint = 0x1002;
pub const HDMI_FC_INHBLANK0: c_uint = 0x1003;
pub const HDMI_FC_INHBLANK1: c_uint = 0x1004;
pub const HDMI_FC_INVACTV0: c_uint = 0x1005;
pub const HDMI_FC_INVACTV1: c_uint = 0x1006;
pub const HDMI_FC_INVBLANK: c_uint = 0x1007;
pub const HDMI_FC_HSYNCINDELAY0: c_uint = 0x1008;
pub const HDMI_FC_HSYNCINDELAY1: c_uint = 0x1009;
pub const HDMI_FC_HSYNCINWIDTH0: c_uint = 0x100A;
pub const HDMI_FC_HSYNCINWIDTH1: c_uint = 0x100B;
pub const HDMI_FC_VSYNCINDELAY: c_uint = 0x100C;
pub const HDMI_FC_VSYNCINWIDTH: c_uint = 0x100D;
pub const HDMI_FC_INFREQ0: c_uint = 0x100E;
pub const HDMI_FC_INFREQ1: c_uint = 0x100F;
pub const HDMI_FC_INFREQ2: c_uint = 0x1010;
pub const HDMI_FC_CTRLDUR: c_uint = 0x1011;
pub const HDMI_FC_EXCTRLDUR: c_uint = 0x1012;
pub const HDMI_FC_EXCTRLSPAC: c_uint = 0x1013;
pub const HDMI_FC_CH0PREAM: c_uint = 0x1014;
pub const HDMI_FC_CH1PREAM: c_uint = 0x1015;
pub const HDMI_FC_CH2PREAM: c_uint = 0x1016;
pub const HDMI_FC_AVICONF3: c_uint = 0x1017;
pub const HDMI_FC_GCP: c_uint = 0x1018;
pub const HDMI_FC_AVICONF0: c_uint = 0x1019;
pub const HDMI_FC_AVICONF1: c_uint = 0x101A;
pub const HDMI_FC_AVICONF2: c_uint = 0x101B;
pub const HDMI_FC_AVIVID: c_uint = 0x101C;
pub const HDMI_FC_AVIETB0: c_uint = 0x101D;
pub const HDMI_FC_AVIETB1: c_uint = 0x101E;
pub const HDMI_FC_AVISBB0: c_uint = 0x101F;
pub const HDMI_FC_AVISBB1: c_uint = 0x1020;
pub const HDMI_FC_AVIELB0: c_uint = 0x1021;
pub const HDMI_FC_AVIELB1: c_uint = 0x1022;
pub const HDMI_FC_AVISRB0: c_uint = 0x1023;
pub const HDMI_FC_AVISRB1: c_uint = 0x1024;
pub const HDMI_FC_AUDICONF0: c_uint = 0x1025;
pub const HDMI_FC_AUDICONF1: c_uint = 0x1026;
pub const HDMI_FC_AUDICONF2: c_uint = 0x1027;
pub const HDMI_FC_AUDICONF3: c_uint = 0x1028;
pub const HDMI_FC_VSDIEEEID0: c_uint = 0x1029;
pub const HDMI_FC_VSDSIZE: c_uint = 0x102A;
pub const HDMI_FC_VSDIEEEID1: c_uint = 0x1030;
pub const HDMI_FC_VSDIEEEID2: c_uint = 0x1031;
pub const HDMI_FC_VSDPAYLOAD0: c_uint = 0x1032;
pub const HDMI_FC_VSDPAYLOAD1: c_uint = 0x1033;
pub const HDMI_FC_VSDPAYLOAD2: c_uint = 0x1034;
pub const HDMI_FC_VSDPAYLOAD3: c_uint = 0x1035;
pub const HDMI_FC_VSDPAYLOAD4: c_uint = 0x1036;
pub const HDMI_FC_VSDPAYLOAD5: c_uint = 0x1037;
pub const HDMI_FC_VSDPAYLOAD6: c_uint = 0x1038;
pub const HDMI_FC_VSDPAYLOAD7: c_uint = 0x1039;
pub const HDMI_FC_VSDPAYLOAD8: c_uint = 0x103A;
pub const HDMI_FC_VSDPAYLOAD9: c_uint = 0x103B;
pub const HDMI_FC_VSDPAYLOAD10: c_uint = 0x103C;
pub const HDMI_FC_VSDPAYLOAD11: c_uint = 0x103D;
pub const HDMI_FC_VSDPAYLOAD12: c_uint = 0x103E;
pub const HDMI_FC_VSDPAYLOAD13: c_uint = 0x103F;
pub const HDMI_FC_VSDPAYLOAD14: c_uint = 0x1040;
pub const HDMI_FC_VSDPAYLOAD15: c_uint = 0x1041;
pub const HDMI_FC_VSDPAYLOAD16: c_uint = 0x1042;
pub const HDMI_FC_VSDPAYLOAD17: c_uint = 0x1043;
pub const HDMI_FC_VSDPAYLOAD18: c_uint = 0x1044;
pub const HDMI_FC_VSDPAYLOAD19: c_uint = 0x1045;
pub const HDMI_FC_VSDPAYLOAD20: c_uint = 0x1046;
pub const HDMI_FC_VSDPAYLOAD21: c_uint = 0x1047;
pub const HDMI_FC_VSDPAYLOAD22: c_uint = 0x1048;
pub const HDMI_FC_VSDPAYLOAD23: c_uint = 0x1049;
pub const HDMI_FC_SPDVENDORNAME0: c_uint = 0x104A;
pub const HDMI_FC_SPDVENDORNAME1: c_uint = 0x104B;
pub const HDMI_FC_SPDVENDORNAME2: c_uint = 0x104C;
pub const HDMI_FC_SPDVENDORNAME3: c_uint = 0x104D;
pub const HDMI_FC_SPDVENDORNAME4: c_uint = 0x104E;
pub const HDMI_FC_SPDVENDORNAME5: c_uint = 0x104F;
pub const HDMI_FC_SPDVENDORNAME6: c_uint = 0x1050;
pub const HDMI_FC_SPDVENDORNAME7: c_uint = 0x1051;
pub const HDMI_FC_SDPPRODUCTNAME0: c_uint = 0x1052;
pub const HDMI_FC_SDPPRODUCTNAME1: c_uint = 0x1053;
pub const HDMI_FC_SDPPRODUCTNAME2: c_uint = 0x1054;
pub const HDMI_FC_SDPPRODUCTNAME3: c_uint = 0x1055;
pub const HDMI_FC_SDPPRODUCTNAME4: c_uint = 0x1056;
pub const HDMI_FC_SDPPRODUCTNAME5: c_uint = 0x1057;
pub const HDMI_FC_SDPPRODUCTNAME6: c_uint = 0x1058;
pub const HDMI_FC_SDPPRODUCTNAME7: c_uint = 0x1059;
pub const HDMI_FC_SDPPRODUCTNAME8: c_uint = 0x105A;
pub const HDMI_FC_SDPPRODUCTNAME9: c_uint = 0x105B;
pub const HDMI_FC_SDPPRODUCTNAME10: c_uint = 0x105C;
pub const HDMI_FC_SDPPRODUCTNAME11: c_uint = 0x105D;
pub const HDMI_FC_SDPPRODUCTNAME12: c_uint = 0x105E;
pub const HDMI_FC_SDPPRODUCTNAME13: c_uint = 0x105F;
pub const HDMI_FC_SDPPRODUCTNAME14: c_uint = 0x1060;
pub const HDMI_FC_SPDPRODUCTNAME15: c_uint = 0x1061;
pub const HDMI_FC_SPDDEVICEINF: c_uint = 0x1062;
pub const HDMI_FC_AUDSCONF: c_uint = 0x1063;
pub const HDMI_FC_AUDSSTAT: c_uint = 0x1064;
pub const HDMI_FC_AUDSV: c_uint = 0x1065;
pub const HDMI_FC_AUDSU: c_uint = 0x1066;
pub const HDMI_FC_AUDSCHNLS0: c_uint = 0x1067;
pub const HDMI_FC_AUDSCHNLS1: c_uint = 0x1068;
pub const HDMI_FC_AUDSCHNLS2: c_uint = 0x1069;
pub const HDMI_FC_AUDSCHNLS3: c_uint = 0x106A;
pub const HDMI_FC_AUDSCHNLS4: c_uint = 0x106B;
pub const HDMI_FC_AUDSCHNLS5: c_uint = 0x106C;
pub const HDMI_FC_AUDSCHNLS6: c_uint = 0x106D;
pub const HDMI_FC_AUDSCHNLS7: c_uint = 0x106E;
pub const HDMI_FC_AUDSCHNLS8: c_uint = 0x106F;
pub const HDMI_FC_DATACH0FILL: c_uint = 0x1070;
pub const HDMI_FC_DATACH1FILL: c_uint = 0x1071;
pub const HDMI_FC_DATACH2FILL: c_uint = 0x1072;
pub const HDMI_FC_CTRLQHIGH: c_uint = 0x1073;
pub const HDMI_FC_CTRLQLOW: c_uint = 0x1074;
pub const HDMI_FC_ACP0: c_uint = 0x1075;
pub const HDMI_FC_ACP28: c_uint = 0x1076;
pub const HDMI_FC_ACP27: c_uint = 0x1077;
pub const HDMI_FC_ACP26: c_uint = 0x1078;
pub const HDMI_FC_ACP25: c_uint = 0x1079;
pub const HDMI_FC_ACP24: c_uint = 0x107A;
pub const HDMI_FC_ACP23: c_uint = 0x107B;
pub const HDMI_FC_ACP22: c_uint = 0x107C;
pub const HDMI_FC_ACP21: c_uint = 0x107D;
pub const HDMI_FC_ACP20: c_uint = 0x107E;
pub const HDMI_FC_ACP19: c_uint = 0x107F;
pub const HDMI_FC_ACP18: c_uint = 0x1080;
pub const HDMI_FC_ACP17: c_uint = 0x1081;
pub const HDMI_FC_ACP16: c_uint = 0x1082;
pub const HDMI_FC_ACP15: c_uint = 0x1083;
pub const HDMI_FC_ACP14: c_uint = 0x1084;
pub const HDMI_FC_ACP13: c_uint = 0x1085;
pub const HDMI_FC_ACP12: c_uint = 0x1086;
pub const HDMI_FC_ACP11: c_uint = 0x1087;
pub const HDMI_FC_ACP10: c_uint = 0x1088;
pub const HDMI_FC_ACP9: c_uint = 0x1089;
pub const HDMI_FC_ACP8: c_uint = 0x108A;
pub const HDMI_FC_ACP7: c_uint = 0x108B;
pub const HDMI_FC_ACP6: c_uint = 0x108C;
pub const HDMI_FC_ACP5: c_uint = 0x108D;
pub const HDMI_FC_ACP4: c_uint = 0x108E;
pub const HDMI_FC_ACP3: c_uint = 0x108F;
pub const HDMI_FC_ACP2: c_uint = 0x1090;
pub const HDMI_FC_ACP1: c_uint = 0x1091;
pub const HDMI_FC_ISCR1_0: c_uint = 0x1092;
pub const HDMI_FC_ISCR1_16: c_uint = 0x1093;
pub const HDMI_FC_ISCR1_15: c_uint = 0x1094;
pub const HDMI_FC_ISCR1_14: c_uint = 0x1095;
pub const HDMI_FC_ISCR1_13: c_uint = 0x1096;
pub const HDMI_FC_ISCR1_12: c_uint = 0x1097;
pub const HDMI_FC_ISCR1_11: c_uint = 0x1098;
pub const HDMI_FC_ISCR1_10: c_uint = 0x1099;
pub const HDMI_FC_ISCR1_9: c_uint = 0x109A;
pub const HDMI_FC_ISCR1_8: c_uint = 0x109B;
pub const HDMI_FC_ISCR1_7: c_uint = 0x109C;
pub const HDMI_FC_ISCR1_6: c_uint = 0x109D;
pub const HDMI_FC_ISCR1_5: c_uint = 0x109E;
pub const HDMI_FC_ISCR1_4: c_uint = 0x109F;
pub const HDMI_FC_ISCR1_3: c_uint = 0x10A0;
pub const HDMI_FC_ISCR1_2: c_uint = 0x10A1;
pub const HDMI_FC_ISCR1_1: c_uint = 0x10A2;
pub const HDMI_FC_ISCR2_15: c_uint = 0x10A3;
pub const HDMI_FC_ISCR2_14: c_uint = 0x10A4;
pub const HDMI_FC_ISCR2_13: c_uint = 0x10A5;
pub const HDMI_FC_ISCR2_12: c_uint = 0x10A6;
pub const HDMI_FC_ISCR2_11: c_uint = 0x10A7;
pub const HDMI_FC_ISCR2_10: c_uint = 0x10A8;
pub const HDMI_FC_ISCR2_9: c_uint = 0x10A9;
pub const HDMI_FC_ISCR2_8: c_uint = 0x10AA;
pub const HDMI_FC_ISCR2_7: c_uint = 0x10AB;
pub const HDMI_FC_ISCR2_6: c_uint = 0x10AC;
pub const HDMI_FC_ISCR2_5: c_uint = 0x10AD;
pub const HDMI_FC_ISCR2_4: c_uint = 0x10AE;
pub const HDMI_FC_ISCR2_3: c_uint = 0x10AF;
pub const HDMI_FC_ISCR2_2: c_uint = 0x10B0;
pub const HDMI_FC_ISCR2_1: c_uint = 0x10B1;
pub const HDMI_FC_ISCR2_0: c_uint = 0x10B2;
pub const HDMI_FC_DATAUTO0: c_uint = 0x10B3;
pub const HDMI_FC_DATAUTO1: c_uint = 0x10B4;
pub const HDMI_FC_DATAUTO2: c_uint = 0x10B5;
pub const HDMI_FC_DATMAN: c_uint = 0x10B6;
pub const HDMI_FC_DATAUTO3: c_uint = 0x10B7;
pub const HDMI_FC_RDRB0: c_uint = 0x10B8;
pub const HDMI_FC_RDRB1: c_uint = 0x10B9;
pub const HDMI_FC_RDRB2: c_uint = 0x10BA;
pub const HDMI_FC_RDRB3: c_uint = 0x10BB;
pub const HDMI_FC_RDRB4: c_uint = 0x10BC;
pub const HDMI_FC_RDRB5: c_uint = 0x10BD;
pub const HDMI_FC_RDRB6: c_uint = 0x10BE;
pub const HDMI_FC_RDRB7: c_uint = 0x10BF;
pub const HDMI_FC_STAT0: c_uint = 0x10D0;
pub const HDMI_FC_INT0: c_uint = 0x10D1;
pub const HDMI_FC_MASK0: c_uint = 0x10D2;
pub const HDMI_FC_POL0: c_uint = 0x10D3;
pub const HDMI_FC_STAT1: c_uint = 0x10D4;
pub const HDMI_FC_INT1: c_uint = 0x10D5;
pub const HDMI_FC_MASK1: c_uint = 0x10D6;
pub const HDMI_FC_POL1: c_uint = 0x10D7;
pub const HDMI_FC_STAT2: c_uint = 0x10D8;
pub const HDMI_FC_INT2: c_uint = 0x10D9;
pub const HDMI_FC_MASK2: c_uint = 0x10DA;
pub const HDMI_FC_POL2: c_uint = 0x10DB;
pub const HDMI_FC_PRCONF: c_uint = 0x10E0;
pub const HDMI_FC_SCRAMBLER_CTRL: c_uint = 0x10E1;
pub const HDMI_FC_PACKET_TX_EN: c_uint = 0x10E3;
pub const HDMI_FC_GMD_STAT: c_uint = 0x1100;
pub const HDMI_FC_GMD_EN: c_uint = 0x1101;
pub const HDMI_FC_GMD_UP: c_uint = 0x1102;
pub const HDMI_FC_GMD_CONF: c_uint = 0x1103;
pub const HDMI_FC_GMD_HB: c_uint = 0x1104;
pub const HDMI_FC_GMD_PB0: c_uint = 0x1105;
pub const HDMI_FC_GMD_PB1: c_uint = 0x1106;
pub const HDMI_FC_GMD_PB2: c_uint = 0x1107;
pub const HDMI_FC_GMD_PB3: c_uint = 0x1108;
pub const HDMI_FC_GMD_PB4: c_uint = 0x1109;
pub const HDMI_FC_GMD_PB5: c_uint = 0x110A;
pub const HDMI_FC_GMD_PB6: c_uint = 0x110B;
pub const HDMI_FC_GMD_PB7: c_uint = 0x110C;
pub const HDMI_FC_GMD_PB8: c_uint = 0x110D;
pub const HDMI_FC_GMD_PB9: c_uint = 0x110E;
pub const HDMI_FC_GMD_PB10: c_uint = 0x110F;
pub const HDMI_FC_GMD_PB11: c_uint = 0x1110;
pub const HDMI_FC_GMD_PB12: c_uint = 0x1111;
pub const HDMI_FC_GMD_PB13: c_uint = 0x1112;
pub const HDMI_FC_GMD_PB14: c_uint = 0x1113;
pub const HDMI_FC_GMD_PB15: c_uint = 0x1114;
pub const HDMI_FC_GMD_PB16: c_uint = 0x1115;
pub const HDMI_FC_GMD_PB17: c_uint = 0x1116;
pub const HDMI_FC_GMD_PB18: c_uint = 0x1117;
pub const HDMI_FC_GMD_PB19: c_uint = 0x1118;
pub const HDMI_FC_GMD_PB20: c_uint = 0x1119;
pub const HDMI_FC_GMD_PB21: c_uint = 0x111A;
pub const HDMI_FC_GMD_PB22: c_uint = 0x111B;
pub const HDMI_FC_GMD_PB23: c_uint = 0x111C;
pub const HDMI_FC_GMD_PB24: c_uint = 0x111D;
pub const HDMI_FC_GMD_PB25: c_uint = 0x111E;
pub const HDMI_FC_GMD_PB26: c_uint = 0x111F;
pub const HDMI_FC_GMD_PB27: c_uint = 0x1120;
pub const HDMI_FC_DRM_UP: c_uint = 0x1167;
pub const HDMI_FC_DRM_HB0: c_uint = 0x1168;
pub const HDMI_FC_DRM_HB1: c_uint = 0x1169;
pub const HDMI_FC_DRM_PB0: c_uint = 0x116A;
pub const HDMI_FC_DRM_PB1: c_uint = 0x116B;
pub const HDMI_FC_DRM_PB2: c_uint = 0x116C;
pub const HDMI_FC_DRM_PB3: c_uint = 0x116D;
pub const HDMI_FC_DRM_PB4: c_uint = 0x116E;
pub const HDMI_FC_DRM_PB5: c_uint = 0x116F;
pub const HDMI_FC_DRM_PB6: c_uint = 0x1170;
pub const HDMI_FC_DRM_PB7: c_uint = 0x1171;
pub const HDMI_FC_DRM_PB8: c_uint = 0x1172;
pub const HDMI_FC_DRM_PB9: c_uint = 0x1173;
pub const HDMI_FC_DRM_PB10: c_uint = 0x1174;
pub const HDMI_FC_DRM_PB11: c_uint = 0x1175;
pub const HDMI_FC_DRM_PB12: c_uint = 0x1176;
pub const HDMI_FC_DRM_PB13: c_uint = 0x1177;
pub const HDMI_FC_DRM_PB14: c_uint = 0x1178;
pub const HDMI_FC_DRM_PB15: c_uint = 0x1179;
pub const HDMI_FC_DRM_PB16: c_uint = 0x117A;
pub const HDMI_FC_DRM_PB17: c_uint = 0x117B;
pub const HDMI_FC_DRM_PB18: c_uint = 0x117C;
pub const HDMI_FC_DRM_PB19: c_uint = 0x117D;
pub const HDMI_FC_DRM_PB20: c_uint = 0x117E;
pub const HDMI_FC_DRM_PB21: c_uint = 0x117F;
pub const HDMI_FC_DRM_PB22: c_uint = 0x1180;
pub const HDMI_FC_DRM_PB23: c_uint = 0x1181;
pub const HDMI_FC_DRM_PB24: c_uint = 0x1182;
pub const HDMI_FC_DRM_PB25: c_uint = 0x1183;
pub const HDMI_FC_DRM_PB26: c_uint = 0x1184;
pub const HDMI_FC_DBGFORCE: c_uint = 0x1200;
pub const HDMI_FC_DBGAUD0CH0: c_uint = 0x1201;
pub const HDMI_FC_DBGAUD1CH0: c_uint = 0x1202;
pub const HDMI_FC_DBGAUD2CH0: c_uint = 0x1203;
pub const HDMI_FC_DBGAUD0CH1: c_uint = 0x1204;
pub const HDMI_FC_DBGAUD1CH1: c_uint = 0x1205;
pub const HDMI_FC_DBGAUD2CH1: c_uint = 0x1206;
pub const HDMI_FC_DBGAUD0CH2: c_uint = 0x1207;
pub const HDMI_FC_DBGAUD1CH2: c_uint = 0x1208;
pub const HDMI_FC_DBGAUD2CH2: c_uint = 0x1209;
pub const HDMI_FC_DBGAUD0CH3: c_uint = 0x120A;
pub const HDMI_FC_DBGAUD1CH3: c_uint = 0x120B;
pub const HDMI_FC_DBGAUD2CH3: c_uint = 0x120C;
pub const HDMI_FC_DBGAUD0CH4: c_uint = 0x120D;
pub const HDMI_FC_DBGAUD1CH4: c_uint = 0x120E;
pub const HDMI_FC_DBGAUD2CH4: c_uint = 0x120F;
pub const HDMI_FC_DBGAUD0CH5: c_uint = 0x1210;
pub const HDMI_FC_DBGAUD1CH5: c_uint = 0x1211;
pub const HDMI_FC_DBGAUD2CH5: c_uint = 0x1212;
pub const HDMI_FC_DBGAUD0CH6: c_uint = 0x1213;
pub const HDMI_FC_DBGAUD1CH6: c_uint = 0x1214;
pub const HDMI_FC_DBGAUD2CH6: c_uint = 0x1215;
pub const HDMI_FC_DBGAUD0CH7: c_uint = 0x1216;
pub const HDMI_FC_DBGAUD1CH7: c_uint = 0x1217;
pub const HDMI_FC_DBGAUD2CH7: c_uint = 0x1218;
pub const HDMI_FC_DBGTMDS0: c_uint = 0x1219;
pub const HDMI_FC_DBGTMDS1: c_uint = 0x121A;
pub const HDMI_FC_DBGTMDS2: c_uint = 0x121B;
// HDMI Source PHY Registers
pub const HDMI_PHY_CONF0: c_uint = 0x3000;
pub const HDMI_PHY_TST0: c_uint = 0x3001;
pub const HDMI_PHY_TST1: c_uint = 0x3002;
pub const HDMI_PHY_TST2: c_uint = 0x3003;
pub const HDMI_PHY_STAT0: c_uint = 0x3004;
pub const HDMI_PHY_INT0: c_uint = 0x3005;
pub const HDMI_PHY_MASK0: c_uint = 0x3006;
pub const HDMI_PHY_POL0: c_uint = 0x3007;
// HDMI Master PHY Registers
pub const HDMI_PHY_I2CM_SLAVE_ADDR: c_uint = 0x3020;
pub const HDMI_PHY_I2CM_ADDRESS_ADDR: c_uint = 0x3021;
pub const HDMI_PHY_I2CM_DATAO_1_ADDR: c_uint = 0x3022;
pub const HDMI_PHY_I2CM_DATAO_0_ADDR: c_uint = 0x3023;
pub const HDMI_PHY_I2CM_DATAI_1_ADDR: c_uint = 0x3024;
pub const HDMI_PHY_I2CM_DATAI_0_ADDR: c_uint = 0x3025;
pub const HDMI_PHY_I2CM_OPERATION_ADDR: c_uint = 0x3026;
pub const HDMI_PHY_I2CM_INT_ADDR: c_uint = 0x3027;
pub const HDMI_PHY_I2CM_CTLINT_ADDR: c_uint = 0x3028;
pub const HDMI_PHY_I2CM_DIV_ADDR: c_uint = 0x3029;
pub const HDMI_PHY_I2CM_SOFTRSTZ_ADDR: c_uint = 0x302a;
pub const HDMI_PHY_I2CM_SS_SCL_HCNT_1_ADDR: c_uint = 0x302b;
pub const HDMI_PHY_I2CM_SS_SCL_HCNT_0_ADDR: c_uint = 0x302c;
pub const HDMI_PHY_I2CM_SS_SCL_LCNT_1_ADDR: c_uint = 0x302d;
pub const HDMI_PHY_I2CM_SS_SCL_LCNT_0_ADDR: c_uint = 0x302e;
pub const HDMI_PHY_I2CM_FS_SCL_HCNT_1_ADDR: c_uint = 0x302f;
pub const HDMI_PHY_I2CM_FS_SCL_HCNT_0_ADDR: c_uint = 0x3030;
pub const HDMI_PHY_I2CM_FS_SCL_LCNT_1_ADDR: c_uint = 0x3031;
pub const HDMI_PHY_I2CM_FS_SCL_LCNT_0_ADDR: c_uint = 0x3032;
// Audio Sampler Registers
pub const HDMI_AUD_CONF0: c_uint = 0x3100;
pub const HDMI_AUD_CONF1: c_uint = 0x3101;
pub const HDMI_AUD_INT: c_uint = 0x3102;
pub const HDMI_AUD_CONF2: c_uint = 0x3103;
pub const HDMI_AUD_N1: c_uint = 0x3200;
pub const HDMI_AUD_N2: c_uint = 0x3201;
pub const HDMI_AUD_N3: c_uint = 0x3202;
pub const HDMI_AUD_CTS1: c_uint = 0x3203;
pub const HDMI_AUD_CTS2: c_uint = 0x3204;
pub const HDMI_AUD_CTS3: c_uint = 0x3205;
pub const HDMI_AUD_INPUTCLKFS: c_uint = 0x3206;
pub const HDMI_AUD_SPDIFINT: c_uint = 0x3302;
pub const HDMI_AUD_CONF0_HBR: c_uint = 0x3400;
pub const HDMI_AUD_HBR_STATUS: c_uint = 0x3401;
pub const HDMI_AUD_HBR_INT: c_uint = 0x3402;
pub const HDMI_AUD_HBR_POL: c_uint = 0x3403;
pub const HDMI_AUD_HBR_MASK: c_uint = 0x3404;
//
// Generic Parallel Audio Interface Registers
// Not used as GPAUD interface is not enabled in hw
//
pub const HDMI_GP_CONF0: c_uint = 0x3500;
pub const HDMI_GP_CONF1: c_uint = 0x3501;
pub const HDMI_GP_CONF2: c_uint = 0x3502;
pub const HDMI_GP_STAT: c_uint = 0x3503;
pub const HDMI_GP_INT: c_uint = 0x3504;
pub const HDMI_GP_MASK: c_uint = 0x3505;
pub const HDMI_GP_POL: c_uint = 0x3506;
// Audio DMA Registers
pub const HDMI_AHB_DMA_CONF0: c_uint = 0x3600;
pub const HDMI_AHB_DMA_START: c_uint = 0x3601;
pub const HDMI_AHB_DMA_STOP: c_uint = 0x3602;
pub const HDMI_AHB_DMA_THRSLD: c_uint = 0x3603;
pub const HDMI_AHB_DMA_STRADDR0: c_uint = 0x3604;
pub const HDMI_AHB_DMA_STRADDR1: c_uint = 0x3605;
pub const HDMI_AHB_DMA_STRADDR2: c_uint = 0x3606;
pub const HDMI_AHB_DMA_STRADDR3: c_uint = 0x3607;
pub const HDMI_AHB_DMA_STPADDR0: c_uint = 0x3608;
pub const HDMI_AHB_DMA_STPADDR1: c_uint = 0x3609;
pub const HDMI_AHB_DMA_STPADDR2: c_uint = 0x360a;
pub const HDMI_AHB_DMA_STPADDR3: c_uint = 0x360b;
pub const HDMI_AHB_DMA_BSTADDR0: c_uint = 0x360c;
pub const HDMI_AHB_DMA_BSTADDR1: c_uint = 0x360d;
pub const HDMI_AHB_DMA_BSTADDR2: c_uint = 0x360e;
pub const HDMI_AHB_DMA_BSTADDR3: c_uint = 0x360f;
pub const HDMI_AHB_DMA_MBLENGTH0: c_uint = 0x3610;
pub const HDMI_AHB_DMA_MBLENGTH1: c_uint = 0x3611;
pub const HDMI_AHB_DMA_STAT: c_uint = 0x3612;
pub const HDMI_AHB_DMA_INT: c_uint = 0x3613;
pub const HDMI_AHB_DMA_MASK: c_uint = 0x3614;
pub const HDMI_AHB_DMA_POL: c_uint = 0x3615;
pub const HDMI_AHB_DMA_CONF1: c_uint = 0x3616;
pub const HDMI_AHB_DMA_BUFFSTAT: c_uint = 0x3617;
pub const HDMI_AHB_DMA_BUFFINT: c_uint = 0x3618;
pub const HDMI_AHB_DMA_BUFFMASK: c_uint = 0x3619;
pub const HDMI_AHB_DMA_BUFFPOL: c_uint = 0x361a;
// Main Controller Registers
pub const HDMI_MC_SFRDIV: c_uint = 0x4000;
pub const HDMI_MC_CLKDIS: c_uint = 0x4001;
pub const HDMI_MC_SWRSTZ: c_uint = 0x4002;
pub const HDMI_MC_OPCTRL: c_uint = 0x4003;
pub const HDMI_MC_FLOWCTRL: c_uint = 0x4004;
pub const HDMI_MC_PHYRSTZ: c_uint = 0x4005;
pub const HDMI_MC_LOCKONCLOCK: c_uint = 0x4006;
pub const HDMI_MC_HEACPHY_RST: c_uint = 0x4007;
// Color Space  Converter Registers
pub const HDMI_CSC_CFG: c_uint = 0x4100;
pub const HDMI_CSC_SCALE: c_uint = 0x4101;
pub const HDMI_CSC_COEF_A1_MSB: c_uint = 0x4102;
pub const HDMI_CSC_COEF_A1_LSB: c_uint = 0x4103;
pub const HDMI_CSC_COEF_A2_MSB: c_uint = 0x4104;
pub const HDMI_CSC_COEF_A2_LSB: c_uint = 0x4105;
pub const HDMI_CSC_COEF_A3_MSB: c_uint = 0x4106;
pub const HDMI_CSC_COEF_A3_LSB: c_uint = 0x4107;
pub const HDMI_CSC_COEF_A4_MSB: c_uint = 0x4108;
pub const HDMI_CSC_COEF_A4_LSB: c_uint = 0x4109;
pub const HDMI_CSC_COEF_B1_MSB: c_uint = 0x410A;
pub const HDMI_CSC_COEF_B1_LSB: c_uint = 0x410B;
pub const HDMI_CSC_COEF_B2_MSB: c_uint = 0x410C;
pub const HDMI_CSC_COEF_B2_LSB: c_uint = 0x410D;
pub const HDMI_CSC_COEF_B3_MSB: c_uint = 0x410E;
pub const HDMI_CSC_COEF_B3_LSB: c_uint = 0x410F;
pub const HDMI_CSC_COEF_B4_MSB: c_uint = 0x4110;
pub const HDMI_CSC_COEF_B4_LSB: c_uint = 0x4111;
pub const HDMI_CSC_COEF_C1_MSB: c_uint = 0x4112;
pub const HDMI_CSC_COEF_C1_LSB: c_uint = 0x4113;
pub const HDMI_CSC_COEF_C2_MSB: c_uint = 0x4114;
pub const HDMI_CSC_COEF_C2_LSB: c_uint = 0x4115;
pub const HDMI_CSC_COEF_C3_MSB: c_uint = 0x4116;
pub const HDMI_CSC_COEF_C3_LSB: c_uint = 0x4117;
pub const HDMI_CSC_COEF_C4_MSB: c_uint = 0x4118;
pub const HDMI_CSC_COEF_C4_LSB: c_uint = 0x4119;
// HDCP Encryption Engine Registers
pub const HDMI_A_HDCPCFG0: c_uint = 0x5000;
pub const HDMI_A_HDCPCFG1: c_uint = 0x5001;
pub const HDMI_A_HDCPOBS0: c_uint = 0x5002;
pub const HDMI_A_HDCPOBS1: c_uint = 0x5003;
pub const HDMI_A_HDCPOBS2: c_uint = 0x5004;
pub const HDMI_A_HDCPOBS3: c_uint = 0x5005;
pub const HDMI_A_APIINTCLR: c_uint = 0x5006;
pub const HDMI_A_APIINTSTAT: c_uint = 0x5007;
pub const HDMI_A_APIINTMSK: c_uint = 0x5008;
pub const HDMI_A_VIDPOLCFG: c_uint = 0x5009;
pub const HDMI_A_OESSWCFG: c_uint = 0x500A;
pub const HDMI_A_TIMER1SETUP0: c_uint = 0x500B;
pub const HDMI_A_TIMER1SETUP1: c_uint = 0x500C;
pub const HDMI_A_TIMER2SETUP0: c_uint = 0x500D;
pub const HDMI_A_TIMER2SETUP1: c_uint = 0x500E;
pub const HDMI_A_100MSCFG: c_uint = 0x500F;
pub const HDMI_A_2SCFG0: c_uint = 0x5010;
pub const HDMI_A_2SCFG1: c_uint = 0x5011;
pub const HDMI_A_5SCFG0: c_uint = 0x5012;
pub const HDMI_A_5SCFG1: c_uint = 0x5013;
pub const HDMI_A_SRMVERLSB: c_uint = 0x5014;
pub const HDMI_A_SRMVERMSB: c_uint = 0x5015;
pub const HDMI_A_SRMCTRL: c_uint = 0x5016;
pub const HDMI_A_SFRSETUP: c_uint = 0x5017;
pub const HDMI_A_I2CHSETUP: c_uint = 0x5018;
pub const HDMI_A_INTSETUP: c_uint = 0x5019;
pub const HDMI_A_PRESETUP: c_uint = 0x501A;
pub const HDMI_A_SRM_BASE: c_uint = 0x5020;
// I2C Master Registers (E-DDC)
pub const HDMI_I2CM_SLAVE: c_uint = 0x7E00;
pub const HDMI_I2CM_ADDRESS: c_uint = 0x7E01;
pub const HDMI_I2CM_DATAO: c_uint = 0x7E02;
pub const HDMI_I2CM_DATAI: c_uint = 0x7E03;
pub const HDMI_I2CM_OPERATION: c_uint = 0x7E04;
pub const HDMI_I2CM_INT: c_uint = 0x7E05;
pub const HDMI_I2CM_CTLINT: c_uint = 0x7E06;
pub const HDMI_I2CM_DIV: c_uint = 0x7E07;
pub const HDMI_I2CM_SEGADDR: c_uint = 0x7E08;
pub const HDMI_I2CM_SOFTRSTZ: c_uint = 0x7E09;
pub const HDMI_I2CM_SEGPTR: c_uint = 0x7E0A;
pub const HDMI_I2CM_SS_SCL_HCNT_1_ADDR: c_uint = 0x7E0B;
pub const HDMI_I2CM_SS_SCL_HCNT_0_ADDR: c_uint = 0x7E0C;
pub const HDMI_I2CM_SS_SCL_LCNT_1_ADDR: c_uint = 0x7E0D;
pub const HDMI_I2CM_SS_SCL_LCNT_0_ADDR: c_uint = 0x7E0E;
pub const HDMI_I2CM_FS_SCL_HCNT_1_ADDR: c_uint = 0x7E0F;
pub const HDMI_I2CM_FS_SCL_HCNT_0_ADDR: c_uint = 0x7E10;
pub const HDMI_I2CM_FS_SCL_LCNT_1_ADDR: c_uint = 0x7E11;
pub const HDMI_I2CM_FS_SCL_LCNT_0_ADDR: c_uint = 0x7E12;
// PRODUCT_ID0 field values
// PRODUCT_ID1 field values
// CONFIG0_ID field values
// CONFIG1_ID field values
// CONFIG3_ID field values
// IH_FC_INT2 field values
// IH_FC_STAT2 field values
// IH_PHY_STAT0 field values
// IH_I2CM_STAT0 and IH_MUTE_I2CM_STAT0 field values
// IH_MUTE_I2CMPHY_STAT0 field values
// IH_AHBDMAAUD_STAT0 field values
// IH_MUTE_FC_STAT2 field values
// IH_MUTE_AHBDMAAUD_STAT0 field values
// IH_MUTE field values
// TX_INVID0 field values
// TX_INSTUFFING field values
// VP_PR_CD field values
// VP_STUFF field values
// VP_CONF field values
// VP_REMAP field values
// FC_INVIDCONF field values
// FC_AUDICONF0 field values
// FC_AUDICONF1 field values
// FC_AUDICONF3 field values
// FC_AUDSCHNLS0 field values
// FC_AUDSCHNLS3-6 field values
// HDMI_FC_AUDSCHNLS7 field values
// HDMI_FC_AUDSCHNLS8 field values
// FC_AUDSCONF field values
// FC_STAT2 field values
// FC_INT2 field values
// FC_MASK2 field values
// FC_PRCONF field values
// FC_PACKET_TX_EN field values
// FC_AVICONF0-FC_AVICONF3 field values
// FC_DBGFORCE field values
// FC_DATAUTO0 field values
// FC_DATAUTO3 field values
// PHY_CONF0 field values
// PHY_TST0 field values
// PHY_STAT0 field values
// PHY_I2CM_SLAVE_ADDR field values
// PHY_I2CM_OPERATION_ADDR field values
// HDMI_PHY_I2CM_INT_ADDR
// HDMI_PHY_I2CM_CTLINT_ADDR
// AUD_CONF0 field values
// AUD_CONF1 field values
// AUD_CTS3 field values
// note that the CTS3 MANUAL bit has been removed
// HDMI_AUD_INPUTCLKFS field values
// AHB_DMA_CONF0 field values
// HDMI_AHB_DMA_START field values
// HDMI_AHB_DMA_STOP field values
// AHB_DMA_STAT, AHB_DMA_INT, AHB_DMA_MASK, AHB_DMA_POL field values
// AHB_DMA_BUFFSTAT, AHB_DMA_BUFFINT,AHB_DMA_BUFFMASK,AHB_DMA_BUFFPOL values
// MC_CLKDIS field values
// MC_SWRSTZ field values
// MC_FLOWCTRL field values
// MC_PHYRSTZ field values
// MC_HEACPHY_RST field values
// CSC_CFG field values
// CSC_SCALE field values
// A_HDCPCFG0 field values
// A_HDCPCFG1 field values
// A_VIDPOLCFG field values
// I2CM_OPERATION field values
// I2CM_INT field values
// I2CM_CTLINT field values
//
// HDMI 3D TX PHY registers
//
pub const HDMI_3D_TX_PHY_PWRCTRL: c_uint = 0x00;
pub const HDMI_3D_TX_PHY_SERDIVCTRL: c_uint = 0x01;
pub const HDMI_3D_TX_PHY_SERCKCTRL: c_uint = 0x02;
pub const HDMI_3D_TX_PHY_SERCKKILLCTRL: c_uint = 0x03;
pub const HDMI_3D_TX_PHY_TXRESCTRL: c_uint = 0x04;
pub const HDMI_3D_TX_PHY_CKCALCTRL: c_uint = 0x05;
pub const HDMI_3D_TX_PHY_CPCE_CTRL: c_uint = 0x06;
pub const HDMI_3D_TX_PHY_TXCLKMEASCTRL: c_uint = 0x07;
pub const HDMI_3D_TX_PHY_TXMEASCTRL: c_uint = 0x08;
pub const HDMI_3D_TX_PHY_CKSYMTXCTRL: c_uint = 0x09;
pub const HDMI_3D_TX_PHY_CMPSEQCTRL: c_uint = 0x0a;
pub const HDMI_3D_TX_PHY_CMPPWRCTRL: c_uint = 0x0b;
pub const HDMI_3D_TX_PHY_CMPMODECTRL: c_uint = 0x0c;
pub const HDMI_3D_TX_PHY_MEASCTRL: c_uint = 0x0d;
pub const HDMI_3D_TX_PHY_VLEVCTRL: c_uint = 0x0e;
pub const HDMI_3D_TX_PHY_D2ACTRL: c_uint = 0x0f;
pub const HDMI_3D_TX_PHY_CURRCTRL: c_uint = 0x10;
pub const HDMI_3D_TX_PHY_DRVANACTRL: c_uint = 0x11;
pub const HDMI_3D_TX_PHY_PLLMEASCTRL: c_uint = 0x12;
pub const HDMI_3D_TX_PHY_PLLPHBYCTRL: c_uint = 0x13;
pub const HDMI_3D_TX_PHY_GRP_CTRL: c_uint = 0x14;
pub const HDMI_3D_TX_PHY_GMPCTRL: c_uint = 0x15;
pub const HDMI_3D_TX_PHY_MPLLMEASCTRL: c_uint = 0x16;
pub const HDMI_3D_TX_PHY_MSM_CTRL: c_uint = 0x17;
pub const HDMI_3D_TX_PHY_SCRPB_STATUS: c_uint = 0x18;
pub const HDMI_3D_TX_PHY_TXTERM: c_uint = 0x19;
pub const HDMI_3D_TX_PHY_PTRPT_ENBL: c_uint = 0x1a;
pub const HDMI_3D_TX_PHY_PATTERNGEN: c_uint = 0x1b;
pub const HDMI_3D_TX_PHY_SDCAP_MODE: c_uint = 0x1c;
pub const HDMI_3D_TX_PHY_SCOPEMODE: c_uint = 0x1d;
pub const HDMI_3D_TX_PHY_DIGTXMODE: c_uint = 0x1e;
pub const HDMI_3D_TX_PHY_STR_STATUS: c_uint = 0x1f;
pub const HDMI_3D_TX_PHY_SCOPECNT0: c_uint = 0x20;
pub const HDMI_3D_TX_PHY_SCOPECNT1: c_uint = 0x21;
pub const HDMI_3D_TX_PHY_SCOPECNT2: c_uint = 0x22;
pub const HDMI_3D_TX_PHY_SCOPECNTCLK: c_uint = 0x23;
pub const HDMI_3D_TX_PHY_SCOPESAMPLE: c_uint = 0x24;
pub const HDMI_3D_TX_PHY_SCOPECNTMSB01: c_uint = 0x25;
pub const HDMI_3D_TX_PHY_SCOPECNTMSB2CK: c_uint = 0x26;
// HDMI_3D_TX_PHY_CKCALCTRL values

// HDMI_3D_TX_PHY_MSM_CTRL values

// HDMI_3D_TX_PHY_PTRPT_ENBL values

