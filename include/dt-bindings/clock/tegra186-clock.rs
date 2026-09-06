//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/tegra186-clock.h
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
// @file
//
// @defgroup clock_ids Clock Identifiers
// @{
// @defgroup extern_input external input clocks
// @{
// @def TEGRA186_CLK_OSC
// @def TEGRA186_CLK_CLK_32K
// @def TEGRA186_CLK_DTV_INPUT
// @def TEGRA186_CLK_SOR0_PAD_CLKOUT
// @def TEGRA186_CLK_SOR1_PAD_CLKOUT
// @def TEGRA186_CLK_I2S1_SYNC_INPUT
// @def TEGRA186_CLK_I2S2_SYNC_INPUT
// @def TEGRA186_CLK_I2S3_SYNC_INPUT
// @def TEGRA186_CLK_I2S4_SYNC_INPUT
// @def TEGRA186_CLK_I2S5_SYNC_INPUT
// @def TEGRA186_CLK_I2S6_SYNC_INPUT
// @def TEGRA186_CLK_SPDIFIN_SYNC_INPUT
// @}
//
// @defgroup extern_output external output clocks
// @{
// @def TEGRA186_CLK_EXTPERIPH1
// @def TEGRA186_CLK_EXTPERIPH2
// @def TEGRA186_CLK_EXTPERIPH3
// @def TEGRA186_CLK_EXTPERIPH4
// @}
//
// @defgroup display_clks display related clocks
// @{
// @def TEGRA186_CLK_CEC
// @def TEGRA186_CLK_DSIC
// @def TEGRA186_CLK_DSIC_LP
// @def TEGRA186_CLK_DSID
// @def TEGRA186_CLK_DSID_LP
// @def TEGRA186_CLK_DPAUX1
// @def TEGRA186_CLK_DPAUX
// @def TEGRA186_CLK_HDA2HDMICODEC
// @def TEGRA186_CLK_NVDISPLAY_DISP
// @def TEGRA186_CLK_NVDISPLAY_DSC
// @def TEGRA186_CLK_NVDISPLAY_P0
// @def TEGRA186_CLK_NVDISPLAY_P1
// @def TEGRA186_CLK_NVDISPLAY_P2
// @def TEGRA186_CLK_NVDISPLAYHUB
// @def TEGRA186_CLK_SOR_SAFE
// @def TEGRA186_CLK_SOR0
// @def TEGRA186_CLK_SOR0_OUT
// @def TEGRA186_CLK_SOR1
// @def TEGRA186_CLK_SOR1_OUT
// @def TEGRA186_CLK_DSI
// @def TEGRA186_CLK_MIPI_CAL
// @def TEGRA186_CLK_DSIA_LP
// @def TEGRA186_CLK_DSIB
// @def TEGRA186_CLK_DSIB_LP
// @}
//
// @defgroup camera_clks camera related clocks
// @{
// @def TEGRA186_CLK_NVCSI
// @def TEGRA186_CLK_NVCSILP
// @def TEGRA186_CLK_VI
// @}
//
// @defgroup audio_clks audio related clocks
// @{
// @def TEGRA186_CLK_ACLK
// @def TEGRA186_CLK_ADSP
// @def TEGRA186_CLK_ADSPNEON
// @def TEGRA186_CLK_AHUB
// @def TEGRA186_CLK_APE
// @def TEGRA186_CLK_APB2APE
// @def TEGRA186_CLK_AUD_MCLK
// @def TEGRA186_CLK_DMIC1
// @def TEGRA186_CLK_DMIC2
// @def TEGRA186_CLK_DMIC3
// @def TEGRA186_CLK_DMIC4
// @def TEGRA186_CLK_DSPK1
// @def TEGRA186_CLK_DSPK2
// @def TEGRA186_CLK_HDA
// @def TEGRA186_CLK_HDA2CODEC_2X
// @def TEGRA186_CLK_I2S1
// @def TEGRA186_CLK_I2S2
// @def TEGRA186_CLK_I2S3
// @def TEGRA186_CLK_I2S4
// @def TEGRA186_CLK_I2S5
// @def TEGRA186_CLK_I2S6
// @def TEGRA186_CLK_MAUD
// @def TEGRA186_CLK_PLL_A_OUT0
// @def TEGRA186_CLK_SPDIF_DOUBLER
// @def TEGRA186_CLK_SPDIF_IN
// @def TEGRA186_CLK_SPDIF_OUT
// @def TEGRA186_CLK_SYNC_DMIC1
// @def TEGRA186_CLK_SYNC_DMIC2
// @def TEGRA186_CLK_SYNC_DMIC3
// @def TEGRA186_CLK_SYNC_DMIC4
// @def TEGRA186_CLK_SYNC_DMIC5
// @def TEGRA186_CLK_SYNC_DSPK1
// @def TEGRA186_CLK_SYNC_DSPK2
// @def TEGRA186_CLK_SYNC_I2S1
// @def TEGRA186_CLK_SYNC_I2S2
// @def TEGRA186_CLK_SYNC_I2S3
// @def TEGRA186_CLK_SYNC_I2S4
// @def TEGRA186_CLK_SYNC_I2S5
// @def TEGRA186_CLK_SYNC_I2S6
// @def TEGRA186_CLK_SYNC_SPDIF
// @}
//
// @defgroup uart_clks UART clocks
// @{
// @def TEGRA186_CLK_AON_UART_FST_MIPI_CAL
// @def TEGRA186_CLK_UARTA
// @def TEGRA186_CLK_UARTB
// @def TEGRA186_CLK_UARTC
// @def TEGRA186_CLK_UARTD
// @def TEGRA186_CLK_UARTE
// @def TEGRA186_CLK_UARTF
// @def TEGRA186_CLK_UARTG
// @def TEGRA186_CLK_UART_FST_MIPI_CAL
// @}
//
// @defgroup i2c_clks I2C clocks
// @{
// @def TEGRA186_CLK_AON_I2C_SLOW
// @def TEGRA186_CLK_I2C1
// @def TEGRA186_CLK_I2C2
// @def TEGRA186_CLK_I2C3
// @def TEGRA186_CLK_I2C4
// @def TEGRA186_CLK_I2C5
// @def TEGRA186_CLK_I2C6
// @def TEGRA186_CLK_I2C8
// @def TEGRA186_CLK_I2C9
// @def TEGRA186_CLK_I2C1
// @def TEGRA186_CLK_I2C12
// @def TEGRA186_CLK_I2C13
// @def TEGRA186_CLK_I2C14
// @def TEGRA186_CLK_I2C_SLOW
// @def TEGRA186_CLK_VI_I2C
// @}
//
// @defgroup spi_clks SPI clocks
// @{
// @def TEGRA186_CLK_SPI1
// @def TEGRA186_CLK_SPI2
// @def TEGRA186_CLK_SPI3
// @def TEGRA186_CLK_SPI4
// @}
//
// @defgroup storage storage related clocks
// @{
// @def TEGRA186_CLK_SATA
// @def TEGRA186_CLK_SATA_OOB
// @def TEGRA186_CLK_SATA_IOBIST
// @def TEGRA186_CLK_SDMMC_LEGACY_TM
// @def TEGRA186_CLK_SDMMC1
// @def TEGRA186_CLK_SDMMC2
// @def TEGRA186_CLK_SDMMC3
// @def TEGRA186_CLK_SDMMC4
// @def TEGRA186_CLK_QSPI
// @def TEGRA186_CLK_QSPI_OUT
// @def TEGRA186_CLK_UFSDEV_REF
// @def TEGRA186_CLK_UFSHC
// @}
//
// @defgroup pwm_clks PWM clocks
// @{
// @def TEGRA186_CLK_PWM1
// @def TEGRA186_CLK_PWM2
// @def TEGRA186_CLK_PWM3
// @def TEGRA186_CLK_PWM4
// @def TEGRA186_CLK_PWM5
// @def TEGRA186_CLK_PWM6
// @def TEGRA186_CLK_PWM7
// @def TEGRA186_CLK_PWM8
// @}
//
// @defgroup plls PLLs and related clocks
// @{
// @def TEGRA186_CLK_PLLREFE_OUT_GATED
// @def TEGRA186_CLK_PLLREFE_OUT1
// @def TEGRA186_CLK_PLLD_OUT1
// @def TEGRA186_CLK_PLLP_OUT0
// @def TEGRA186_CLK_PLLP_OUT5
// @def TEGRA186_CLK_PLLA
// @def TEGRA186_CLK_PLLE_PWRSEQ
// @def TEGRA186_CLK_PLLA_OUT1
// @def TEGRA186_CLK_PLLREFE_REF
// @def TEGRA186_CLK_UPHY_PLL0_PWRSEQ
// @def TEGRA186_CLK_UPHY_PLL1_PWRSEQ
// @def TEGRA186_CLK_PLLREFE_PLLE_PASSTHROUGH
// @def TEGRA186_CLK_PLLREFE_PEX
// @def TEGRA186_CLK_PLLREFE_IDDQ
// @def TEGRA186_CLK_PLLC_OUT_AON
// @def TEGRA186_CLK_PLLC_OUT_ISP
// @def TEGRA186_CLK_PLLC_OUT_VE
// @def TEGRA186_CLK_PLLC4_OUT
// @def TEGRA186_CLK_PLLREFE_OUT
// @def TEGRA186_CLK_PLLREFE_PLL_REF
// @def TEGRA186_CLK_PLLE
// @def TEGRA186_CLK_PLLC
// @def TEGRA186_CLK_PLLP
// @def TEGRA186_CLK_PLLD
// @def TEGRA186_CLK_PLLD2
// @def TEGRA186_CLK_PLLREFE_VCO
// @def TEGRA186_CLK_PLLC2
// @def TEGRA186_CLK_PLLC3
// @def TEGRA186_CLK_PLLDP
// @def TEGRA186_CLK_PLLC4_VCO
// @def TEGRA186_CLK_PLLA1
// @def TEGRA186_CLK_PLLNVCSI
// @def TEGRA186_CLK_PLLDISPHUB
// @def TEGRA186_CLK_PLLD3
// @def TEGRA186_CLK_PLLBPMPCAM
// @def TEGRA186_CLK_PLLAON
// @def TEGRA186_CLK_PLLU
// @def TEGRA186_CLK_PLLC4_VCO_DIV2
// @def TEGRA186_CLK_PLL_REF
// @def TEGRA186_CLK_PLLREFE_OUT1_DIV5
// @def TEGRA186_CLK_UTMIP_PLL_PWRSEQ
// @def TEGRA186_CLK_PLL_U_48M
// @def TEGRA186_CLK_PLL_U_480M
// @def TEGRA186_CLK_PLLC4_OUT0
// @def TEGRA186_CLK_PLLC4_OUT1
// @def TEGRA186_CLK_PLLC4_OUT2
// @def TEGRA186_CLK_PLLC4_OUT_MUX
// @def TEGRA186_CLK_DFLLDISP_DIV
// @def TEGRA186_CLK_PLLDISPHUB_DIV
// @def TEGRA186_CLK_PLLP_DIV8
// @}
//
// @defgroup nafll_clks NAFLL clock sources
// @{
// @def TEGRA186_CLK_NAFLL_AXI_CBB
// @def TEGRA186_CLK_NAFLL_BCPU
// @def TEGRA186_CLK_NAFLL_BPMP
// @def TEGRA186_CLK_NAFLL_DISP
// @def TEGRA186_CLK_NAFLL_GPU
// @def TEGRA186_CLK_NAFLL_ISP
// @def TEGRA186_CLK_NAFLL_MCPU
// @def TEGRA186_CLK_NAFLL_NVDEC
// @def TEGRA186_CLK_NAFLL_NVENC
// @def TEGRA186_CLK_NAFLL_NVJPG
// @def TEGRA186_CLK_NAFLL_SCE
// @def TEGRA186_CLK_NAFLL_SE
// @def TEGRA186_CLK_NAFLL_TSEC
// @def TEGRA186_CLK_NAFLL_TSECB
// @def TEGRA186_CLK_NAFLL_VI
// @def TEGRA186_CLK_NAFLL_VIC
// @}
//
// @defgroup mphy MPHY related clocks
// @{
// @def TEGRA186_CLK_MPHY_L0_RX_SYMB
// @def TEGRA186_CLK_MPHY_L0_RX_LS_BIT
// @def TEGRA186_CLK_MPHY_L0_TX_SYMB
// @def TEGRA186_CLK_MPHY_L0_TX_LS_3XBIT
// @def TEGRA186_CLK_MPHY_L0_RX_ANA
// @def TEGRA186_CLK_MPHY_L1_RX_ANA
// @def TEGRA186_CLK_MPHY_IOBIST
// @def TEGRA186_CLK_MPHY_TX_1MHZ_REF
// @def TEGRA186_CLK_MPHY_CORE_PLL_FIXED
// @}
//
// @defgroup eavb EAVB related clocks
// @{
// @def TEGRA186_CLK_EQOS_AXI
// @def TEGRA186_CLK_EQOS_PTP_REF
// @def TEGRA186_CLK_EQOS_RX
// @def TEGRA186_CLK_EQOS_RX_INPUT
// @def TEGRA186_CLK_EQOS_TX
// @}
//
// @defgroup usb USB related clocks
// @{
// @def TEGRA186_CLK_PEX_USB_PAD0_MGMT
// @def TEGRA186_CLK_PEX_USB_PAD1_MGMT
// @def TEGRA186_CLK_HSIC_TRK
// @def TEGRA186_CLK_USB2_TRK
// @def TEGRA186_CLK_USB2_HSIC_TRK
// @def TEGRA186_CLK_XUSB_CORE_SS
// @def TEGRA186_CLK_XUSB_CORE_DEV
// @def TEGRA186_CLK_XUSB_FALCON
// @def TEGRA186_CLK_XUSB_FS
// @def TEGRA186_CLK_XUSB
// @def TEGRA186_CLK_XUSB_DEV
// @def TEGRA186_CLK_XUSB_HOST
// @def TEGRA186_CLK_XUSB_SS
// @}
//
// @defgroup bigblock compute block related clocks
// @{
// @def TEGRA186_CLK_GPCCLK
// @def TEGRA186_CLK_GPC2CLK
// @def TEGRA186_CLK_GPU
// @def TEGRA186_CLK_HOST1X
// @def TEGRA186_CLK_ISP
// @def TEGRA186_CLK_NVDEC
// @def TEGRA186_CLK_NVENC
// @def TEGRA186_CLK_NVJPG
// @def TEGRA186_CLK_SE
// @def TEGRA186_CLK_TSEC
// @def TEGRA186_CLK_TSECB
// @def TEGRA186_CLK_VIC
// @}
//
// @defgroup can CAN bus related clocks
// @{
// @def TEGRA186_CLK_CAN1
// @def TEGRA186_CLK_CAN1_HOST
// @def TEGRA186_CLK_CAN2
// @def TEGRA186_CLK_CAN2_HOST
// @}
//
// @defgroup system basic system clocks
// @{
// @def TEGRA186_CLK_ACTMON
// @def TEGRA186_CLK_AON_APB
// @def TEGRA186_CLK_AON_CPU_NIC
// @def TEGRA186_CLK_AON_NIC
// @def TEGRA186_CLK_AXI_CBB
// @def TEGRA186_CLK_BPMP_APB
// @def TEGRA186_CLK_BPMP_CPU_NIC
// @def TEGRA186_CLK_BPMP_NIC_RATE
// @def TEGRA186_CLK_CLK_M
// @def TEGRA186_CLK_EMC
// @def TEGRA186_CLK_MSS_ENCRYPT
// @def TEGRA186_CLK_SCE_APB
// @def TEGRA186_CLK_SCE_CPU_NIC
// @def TEGRA186_CLK_SCE_NIC
// @def TEGRA186_CLK_TSC
// @}
//
// @defgroup pcie_clks PCIe related clocks
// @{
// @def TEGRA186_CLK_AFI
// @def TEGRA186_CLK_PCIE
// @def TEGRA186_CLK_PCIE2_IOBIST
// @def TEGRA186_CLK_PCIERX0
// @def TEGRA186_CLK_PCIERX1
// @def TEGRA186_CLK_PCIERX2
// @def TEGRA186_CLK_PCIERX3
// @def TEGRA186_CLK_PCIERX4
// @}
//
// @brief output of gate CLK_ENB_FUSE
pub const TEGRA186_CLK_FUSE: c_int = 0;
//
// @brief It's not what you think
// @details output of gate CLK_ENB_GPU. This output connects to the GPU
// pwrclk. @warning: This is almost certainly not the clock you think
// it is. If you're looking for the clock of the graphics engine, see
// TEGRA186_GPCCLK
//
pub const TEGRA186_CLK_GPU: c_int = 1;
// @brief output of gate CLK_ENB_PCIE
pub const TEGRA186_CLK_PCIE: c_int = 3;
// @brief output of the divider IPFS_CLK_DIVISOR
pub const TEGRA186_CLK_AFI: c_int = 4;
// @brief output of gate CLK_ENB_PCIE2_IOBIST
pub const TEGRA186_CLK_PCIE2_IOBIST: c_int = 5;
// @brief output of gate CLK_ENB_PCIERX0
pub const TEGRA186_CLK_PCIERX0: c_int = 6;
// @brief output of gate CLK_ENB_PCIERX1
pub const TEGRA186_CLK_PCIERX1: c_int = 7;
// @brief output of gate CLK_ENB_PCIERX2
pub const TEGRA186_CLK_PCIERX2: c_int = 8;
// @brief output of gate CLK_ENB_PCIERX3
pub const TEGRA186_CLK_PCIERX3: c_int = 9;
// @brief output of gate CLK_ENB_PCIERX4
pub const TEGRA186_CLK_PCIERX4: c_int = 10;
// @brief output branch of PLL_C for ISP, controlled by gate CLK_ENB_PLLC_OUT_ISP
pub const TEGRA186_CLK_PLLC_OUT_ISP: c_int = 11;
// @brief output branch of PLL_C for VI, controlled by gate CLK_ENB_PLLC_OUT_VE
pub const TEGRA186_CLK_PLLC_OUT_VE: c_int = 12;
// @brief output branch of PLL_C for AON domain, controlled by gate CLK_ENB_PLLC_OUT_AON
pub const TEGRA186_CLK_PLLC_OUT_AON: c_int = 13;
// @brief output of gate CLK_ENB_SOR_SAFE
pub const TEGRA186_CLK_SOR_SAFE: c_int = 39;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S2
pub const TEGRA186_CLK_I2S2: c_int = 42;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S3
pub const TEGRA186_CLK_I2S3: c_int = 43;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPDF_IN
pub const TEGRA186_CLK_SPDIF_IN: c_int = 44;
// @brief output of gate CLK_ENB_SPDIF_DOUBLER
pub const TEGRA186_CLK_SPDIF_DOUBLER: c_int = 45;
// @clkdesc{spi_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_SPI3}
pub const TEGRA186_CLK_SPI3: c_int = 46;
// @clkdesc{i2c_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_I2C1}
pub const TEGRA186_CLK_I2C1: c_int = 47;
// @clkdesc{i2c_clks, out, mux, CLK_RST_CONTROLLER_CLK_SOURCE_I2C5}
pub const TEGRA186_CLK_I2C5: c_int = 48;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI1
pub const TEGRA186_CLK_SPI1: c_int = 49;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_ISP
pub const TEGRA186_CLK_ISP: c_int = 50;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI
pub const TEGRA186_CLK_VI: c_int = 51;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC1
pub const TEGRA186_CLK_SDMMC1: c_int = 52;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC2
pub const TEGRA186_CLK_SDMMC2: c_int = 53;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC4
pub const TEGRA186_CLK_SDMMC4: c_int = 54;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTA
pub const TEGRA186_CLK_UARTA: c_int = 55;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTB
pub const TEGRA186_CLK_UARTB: c_int = 56;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HOST1X
pub const TEGRA186_CLK_HOST1X: c_int = 57;
//
// @brief controls the EMC clock frequency.
// @details Doing a clk_set_rate on this clock will select the
// appropriate clock source, program the source rate and execute a
// specific sequence to switch to the new clock source for both memory
// controllers. This can be used to control the balance between memory
// throughput and memory controller power.
//
pub const TEGRA186_CLK_EMC: c_int = 58;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH4
pub const TEGRA186_CLK_EXTPERIPH4: c_int = 73;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI4
pub const TEGRA186_CLK_SPI4: c_int = 74;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C3
pub const TEGRA186_CLK_I2C3: c_int = 75;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC3
pub const TEGRA186_CLK_SDMMC3: c_int = 76;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTD
pub const TEGRA186_CLK_UARTD: c_int = 77;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S1
pub const TEGRA186_CLK_I2S1: c_int = 79;
// output of gate CLK_ENB_DTV
pub const TEGRA186_CLK_DTV: c_int = 80;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSEC
pub const TEGRA186_CLK_TSEC: c_int = 81;
// @brief output of gate CLK_ENB_DP2
pub const TEGRA186_CLK_DP2: c_int = 82;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S4
pub const TEGRA186_CLK_I2S4: c_int = 84;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2S5
pub const TEGRA186_CLK_I2S5: c_int = 85;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C4
pub const TEGRA186_CLK_I2C4: c_int = 86;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AHUB
pub const TEGRA186_CLK_AHUB: c_int = 87;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA2CODEC_2X
pub const TEGRA186_CLK_HDA2CODEC_2X: c_int = 88;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH1
pub const TEGRA186_CLK_EXTPERIPH1: c_int = 89;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH2
pub const TEGRA186_CLK_EXTPERIPH2: c_int = 90;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_EXTPERIPH3
pub const TEGRA186_CLK_EXTPERIPH3: c_int = 91;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C_SLOW
pub const TEGRA186_CLK_I2C_SLOW: c_int = 92;
// @brief output of the SOR1_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1
pub const TEGRA186_CLK_SOR1: c_int = 93;
// @brief output of gate CLK_ENB_CEC
pub const TEGRA186_CLK_CEC: c_int = 94;
// @brief output of gate CLK_ENB_DPAUX1
pub const TEGRA186_CLK_DPAUX1: c_int = 95;
// @brief output of gate CLK_ENB_DPAUX
pub const TEGRA186_CLK_DPAUX: c_int = 96;
// @brief output of the SOR0_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0
pub const TEGRA186_CLK_SOR0: c_int = 97;
// @brief output of gate CLK_ENB_HDA2HDMICODEC
pub const TEGRA186_CLK_HDA2HDMICODEC: c_int = 98;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SATA
pub const TEGRA186_CLK_SATA: c_int = 99;
// @brief output of gate CLK_ENB_SATA_OOB
pub const TEGRA186_CLK_SATA_OOB: c_int = 100;
// @brief output of gate CLK_ENB_SATA_IOBIST
pub const TEGRA186_CLK_SATA_IOBIST: c_int = 101;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_HDA
pub const TEGRA186_CLK_HDA: c_int = 102;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SE
pub const TEGRA186_CLK_SE: c_int = 103;
// @brief output of gate CLK_ENB_APB2APE
pub const TEGRA186_CLK_APB2APE: c_int = 104;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_APE
pub const TEGRA186_CLK_APE: c_int = 105;
// @brief output of gate CLK_ENB_IQC1
pub const TEGRA186_CLK_IQC1: c_int = 106;
// @brief output of gate CLK_ENB_IQC2
pub const TEGRA186_CLK_IQC2: c_int = 107;
// divide by 2 version of TEGRA186_CLK_PLLREFE_VCO
pub const TEGRA186_CLK_PLLREFE_OUT: c_int = 108;
// @brief output of gate CLK_ENB_PLLREFE_PLL_REF
pub const TEGRA186_CLK_PLLREFE_PLL_REF: c_int = 109;
// @brief output of gate CLK_ENB_PLLC4_OUT
pub const TEGRA186_CLK_PLLC4_OUT: c_int = 110;
// @brief output of mux xusb_core_clk_switch on page 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB: c_int = 111;
// controls xusb_dev_ce signal on page 66 and 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_DEV: c_int = 112;
// controls xusb_host_ce signal on page 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_HOST: c_int = 113;
// controls xusb_ss_ce signal on page 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_SS: c_int = 114;
// @brief output of gate CLK_ENB_DSI
pub const TEGRA186_CLK_DSI: c_int = 115;
// @brief output of gate CLK_ENB_MIPI_CAL
pub const TEGRA186_CLK_MIPI_CAL: c_int = 116;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIA_LP
pub const TEGRA186_CLK_DSIA_LP: c_int = 117;
// @brief output of gate CLK_ENB_DSIB
pub const TEGRA186_CLK_DSIB: c_int = 118;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIB_LP
pub const TEGRA186_CLK_DSIB_LP: c_int = 119;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC1
pub const TEGRA186_CLK_DMIC1: c_int = 122;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC2
pub const TEGRA186_CLK_DMIC2: c_int = 123;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AUD_MCLK
pub const TEGRA186_CLK_AUD_MCLK: c_int = 124;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6
pub const TEGRA186_CLK_I2C6: c_int = 125;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UART_FST_MIPI_CAL
pub const TEGRA186_CLK_UART_FST_MIPI_CAL: c_int = 126;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VIC
pub const TEGRA186_CLK_VIC: c_int = 127;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SDMMC_LEGACY_TM
pub const TEGRA186_CLK_SDMMC_LEGACY_TM: c_int = 128;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDEC
pub const TEGRA186_CLK_NVDEC: c_int = 129;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVJPG
pub const TEGRA186_CLK_NVJPG: c_int = 130;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVENC
pub const TEGRA186_CLK_NVENC: c_int = 131;
// @brief output of the QSPI_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI
pub const TEGRA186_CLK_QSPI: c_int = 132;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_VI_I2C
pub const TEGRA186_CLK_VI_I2C: c_int = 133;
// @brief output of gate CLK_ENB_HSIC_TRK
pub const TEGRA186_CLK_HSIC_TRK: c_int = 134;
// @brief output of gate CLK_ENB_USB2_TRK
pub const TEGRA186_CLK_USB2_TRK: c_int = 135;
// output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_MAUD
pub const TEGRA186_CLK_MAUD: c_int = 136;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSECB
pub const TEGRA186_CLK_TSECB: c_int = 137;
// @brief output of gate CLK_ENB_ADSP
pub const TEGRA186_CLK_ADSP: c_int = 138;
// @brief output of gate CLK_ENB_ADSPNEON
pub const TEGRA186_CLK_ADSPNEON: c_int = 139;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_RX_LS_SYMB
pub const TEGRA186_CLK_MPHY_L0_RX_SYMB: c_int = 140;
// @brief output of gate CLK_ENB_MPHY_L0_RX_LS_BIT
pub const TEGRA186_CLK_MPHY_L0_RX_LS_BIT: c_int = 141;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_L0_TX_LS_SYMB
pub const TEGRA186_CLK_MPHY_L0_TX_SYMB: c_int = 142;
// @brief output of gate CLK_ENB_MPHY_L0_TX_LS_3XBIT
pub const TEGRA186_CLK_MPHY_L0_TX_LS_3XBIT: c_int = 143;
// @brief output of gate CLK_ENB_MPHY_L0_RX_ANA
pub const TEGRA186_CLK_MPHY_L0_RX_ANA: c_int = 144;
// @brief output of gate CLK_ENB_MPHY_L1_RX_ANA
pub const TEGRA186_CLK_MPHY_L1_RX_ANA: c_int = 145;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_IOBIST
pub const TEGRA186_CLK_MPHY_IOBIST: c_int = 146;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_TX_1MHZ_REF
pub const TEGRA186_CLK_MPHY_TX_1MHZ_REF: c_int = 147;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_MPHY_CORE_PLL_FIXED
pub const TEGRA186_CLK_MPHY_CORE_PLL_FIXED: c_int = 148;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AXI_CBB
pub const TEGRA186_CLK_AXI_CBB: c_int = 149;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC3
pub const TEGRA186_CLK_DMIC3: c_int = 150;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC4
pub const TEGRA186_CLK_DMIC4: c_int = 151;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK1
pub const TEGRA186_CLK_DSPK1: c_int = 152;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSPK2
pub const TEGRA186_CLK_DSPK2: c_int = 153;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C6
pub const TEGRA186_CLK_I2S6: c_int = 154;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P0
pub const TEGRA186_CLK_NVDISPLAY_P0: c_int = 155;
// @brief output of the NVDISPLAY_DISP_CLK_SRC mux in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP
pub const TEGRA186_CLK_NVDISPLAY_DISP: c_int = 156;
// @brief output of gate CLK_ENB_NVDISPLAY_DSC
pub const TEGRA186_CLK_NVDISPLAY_DSC: c_int = 157;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAYHUB
pub const TEGRA186_CLK_NVDISPLAYHUB: c_int = 158;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P1
pub const TEGRA186_CLK_NVDISPLAY_P1: c_int = 159;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_P2
pub const TEGRA186_CLK_NVDISPLAY_P2: c_int = 160;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TACH
pub const TEGRA186_CLK_TACH: c_int = 166;
// @brief output of gate CLK_ENB_EQOS
pub const TEGRA186_CLK_EQOS_AXI: c_int = 167;
// @brief output of gate CLK_ENB_EQOS_RX
pub const TEGRA186_CLK_EQOS_RX: c_int = 168;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSHC_CG_SYS
pub const TEGRA186_CLK_UFSHC: c_int = 178;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UFSDEV_REF
pub const TEGRA186_CLK_UFSDEV_REF: c_int = 179;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSI
pub const TEGRA186_CLK_NVCSI: c_int = 180;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_NVCSILP
pub const TEGRA186_CLK_NVCSILP: c_int = 181;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C7
pub const TEGRA186_CLK_I2C7: c_int = 182;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C9
pub const TEGRA186_CLK_I2C9: c_int = 183;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C12
pub const TEGRA186_CLK_I2C12: c_int = 184;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C13
pub const TEGRA186_CLK_I2C13: c_int = 185;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C14
pub const TEGRA186_CLK_I2C14: c_int = 186;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM1
pub const TEGRA186_CLK_PWM1: c_int = 187;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM2
pub const TEGRA186_CLK_PWM2: c_int = 188;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM3
pub const TEGRA186_CLK_PWM3: c_int = 189;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM5
pub const TEGRA186_CLK_PWM5: c_int = 190;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM6
pub const TEGRA186_CLK_PWM6: c_int = 191;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM7
pub const TEGRA186_CLK_PWM7: c_int = 192;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM8
pub const TEGRA186_CLK_PWM8: c_int = 193;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTE
pub const TEGRA186_CLK_UARTE: c_int = 194;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTF
pub const TEGRA186_CLK_UARTF: c_int = 195;
// @deprecated
pub const TEGRA186_CLK_DBGAPB: c_int = 196;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_CPU_NIC
pub const TEGRA186_CLK_BPMP_CPU_NIC: c_int = 197;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_BPMP_APB
pub const TEGRA186_CLK_BPMP_APB: c_int = 199;
// @brief output of mux controlled by TEGRA186_CLK_SOC_ACTMON
pub const TEGRA186_CLK_ACTMON: c_int = 201;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_CPU_NIC
pub const TEGRA186_CLK_AON_CPU_NIC: c_int = 208;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN1
pub const TEGRA186_CLK_CAN1: c_int = 210;
// @brief output of gate CLK_ENB_CAN1_HOST
pub const TEGRA186_CLK_CAN1_HOST: c_int = 211;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_CAN2
pub const TEGRA186_CLK_CAN2: c_int = 212;
// @brief output of gate CLK_ENB_CAN2_HOST
pub const TEGRA186_CLK_CAN2_HOST: c_int = 213;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_APB
pub const TEGRA186_CLK_AON_APB: c_int = 214;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTC
pub const TEGRA186_CLK_UARTC: c_int = 215;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_UARTG
pub const TEGRA186_CLK_UARTG: c_int = 216;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_UART_FST_MIPI_CAL
pub const TEGRA186_CLK_AON_UART_FST_MIPI_CAL: c_int = 217;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C2
pub const TEGRA186_CLK_I2C2: c_int = 218;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C8
pub const TEGRA186_CLK_I2C8: c_int = 219;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_I2C10
pub const TEGRA186_CLK_I2C10: c_int = 220;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_I2C_SLOW
pub const TEGRA186_CLK_AON_I2C_SLOW: c_int = 221;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPI2
pub const TEGRA186_CLK_SPI2: c_int = 222;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DMIC5
pub const TEGRA186_CLK_DMIC5: c_int = 223;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_AON_TOUCH
pub const TEGRA186_CLK_AON_TOUCH: c_int = 224;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_PWM4
pub const TEGRA186_CLK_PWM4: c_int = 225;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_TSC. This clock object is read only and is used for all timers in the system.
pub const TEGRA186_CLK_TSC: c_int = 226;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_MSS_ENCRYPT
pub const TEGRA186_CLK_MSS_ENCRYPT: c_int = 227;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_CPU_NIC
pub const TEGRA186_CLK_SCE_CPU_NIC: c_int = 228;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SCE_APB
pub const TEGRA186_CLK_SCE_APB: c_int = 230;
// @brief output of gate CLK_ENB_DSIC
pub const TEGRA186_CLK_DSIC: c_int = 231;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSIC_LP
pub const TEGRA186_CLK_DSIC_LP: c_int = 232;
// @brief output of gate CLK_ENB_DSID
pub const TEGRA186_CLK_DSID: c_int = 233;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_DSID_LP
pub const TEGRA186_CLK_DSID_LP: c_int = 234;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_SATA_USB_RX_BYP
pub const TEGRA186_CLK_PEX_SATA_USB_RX_BYP: c_int = 236;
// @brief output of mux controlled by CLK_RST_CONTROLLER_CLK_SOURCE_SPDIF_OUT
pub const TEGRA186_CLK_SPDIF_OUT: c_int = 238;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_PTP_REF_CLK_0
pub const TEGRA186_CLK_EQOS_PTP_REF: c_int = 239;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_EQOS_TX_CLK
pub const TEGRA186_CLK_EQOS_TX: c_int = 240;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_USB2_HSIC_TRK
pub const TEGRA186_CLK_USB2_HSIC_TRK: c_int = 241;
// @brief output of mux xusb_ss_clk_switch on page 66 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_CORE_SS: c_int = 242;
// @brief output of mux xusb_core_dev_clk_switch on page 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_CORE_DEV: c_int = 243;
// @brief output of mux xusb_core_falcon_clk_switch on page 67 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_FALCON: c_int = 244;
// @brief output of mux xusb_fs_clk_switch on page 66 of T186_Clocks_IAS.doc
pub const TEGRA186_CLK_XUSB_FS: c_int = 245;
// @brief output of the divider CLK_RST_CONTROLLER_PLLA_OUT
pub const TEGRA186_CLK_PLL_A_OUT0: c_int = 246;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S1
pub const TEGRA186_CLK_SYNC_I2S1: c_int = 247;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S2
pub const TEGRA186_CLK_SYNC_I2S2: c_int = 248;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S3
pub const TEGRA186_CLK_SYNC_I2S3: c_int = 249;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S4
pub const TEGRA186_CLK_SYNC_I2S4: c_int = 250;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S5
pub const TEGRA186_CLK_SYNC_I2S5: c_int = 251;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_I2S6
pub const TEGRA186_CLK_SYNC_I2S6: c_int = 252;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK1
pub const TEGRA186_CLK_SYNC_DSPK1: c_int = 253;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DSPK2
pub const TEGRA186_CLK_SYNC_DSPK2: c_int = 254;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC1
pub const TEGRA186_CLK_SYNC_DMIC1: c_int = 255;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC2
pub const TEGRA186_CLK_SYNC_DMIC2: c_int = 256;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC3
pub const TEGRA186_CLK_SYNC_DMIC3: c_int = 257;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_DMIC4
pub const TEGRA186_CLK_SYNC_DMIC4: c_int = 259;
// @brief output of mux controlled by CLK_RST_CONTROLLER_AUDIO_SYNC_CLK_SPDIF
pub const TEGRA186_CLK_SYNC_SPDIF: c_int = 260;
// @brief output of gate CLK_ENB_PLLREFE_OUT
pub const TEGRA186_CLK_PLLREFE_OUT_GATED: c_int = 261;
// @brief output of the divider PLLREFE_DIVP in CLK_RST_CONTROLLER_PLLREFE_BASE. PLLREFE has 2 outputs:
// * VCO/pdiv defined by this clock object
// * VCO/2 defined by TEGRA186_CLK_PLLREFE_OUT
//
pub const TEGRA186_CLK_PLLREFE_OUT1: c_int = 262;
pub const TEGRA186_CLK_PLLD_OUT1: c_int = 267;
// @brief output of the divider PLLP_DIVP in CLK_RST_CONTROLLER_PLLP_BASE
pub const TEGRA186_CLK_PLLP_OUT0: c_int = 269;
// @brief output of the divider CLK_RST_CONTROLLER_PLLP_OUTC
pub const TEGRA186_CLK_PLLP_OUT5: c_int = 270;
// PLL controlled by CLK_RST_CONTROLLER_PLLA_BASE for use by audio clocks
pub const TEGRA186_CLK_PLLA: c_int = 271;
// @brief output of mux controlled by CLK_RST_CONTROLLER_ACLK_BURST_POLICY divided by the divider controlled by ACLK_CLK_DIVISOR in CLK_RST_CONTROLLER_SUPER_ACLK_DIVIDER
pub const TEGRA186_CLK_ACLK: c_int = 273;
// fixed 48MHz clock divided down from TEGRA186_CLK_PLL_U
pub const TEGRA186_CLK_PLL_U_48M: c_int = 274;
// fixed 480MHz clock divided down from TEGRA186_CLK_PLL_U
pub const TEGRA186_CLK_PLL_U_480M: c_int = 275;
// @brief output of the divider PLLC4_DIVP in CLK_RST_CONTROLLER_PLLC4_BASE. Output frequency is TEGRA186_CLK_PLLC4_VCO/PLLC4_DIVP
pub const TEGRA186_CLK_PLLC4_OUT0: c_int = 276;
// fixed /3 divider. Output frequency of this clock is TEGRA186_CLK_PLLC4_VCO/3
pub const TEGRA186_CLK_PLLC4_OUT1: c_int = 277;
// fixed /5 divider. Output frequency of this clock is TEGRA186_CLK_PLLC4_VCO/5
pub const TEGRA186_CLK_PLLC4_OUT2: c_int = 278;
// @brief output of mux controlled by PLLC4_CLK_SEL in CLK_RST_CONTROLLER_PLLC4_MISC1
pub const TEGRA186_CLK_PLLC4_OUT_MUX: c_int = 279;
// @brief output of divider NVDISPLAY_DISP_CLK_DIVISOR in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP when DFLLDISP_DIV is selected in NVDISPLAY_DISP_CLK_SRC
pub const TEGRA186_CLK_DFLLDISP_DIV: c_int = 284;
// @brief output of divider NVDISPLAY_DISP_CLK_DIVISOR in CLK_RST_CONTROLLER_CLK_SOURCE_NVDISPLAY_DISP when PLLDISPHUB_DIV is selected in NVDISPLAY_DISP_CLK_SRC
pub const TEGRA186_CLK_PLLDISPHUB_DIV: c_int = 285;
// fixed /8 divider which is used as the input for TEGRA186_CLK_SOR_SAFE
pub const TEGRA186_CLK_PLLP_DIV8: c_int = 286;
// @brief output of divider CLK_RST_CONTROLLER_BPMP_NIC_RATE
pub const TEGRA186_CLK_BPMP_NIC: c_int = 287;
// @brief output of the divider CLK_RST_CONTROLLER_PLLA1_OUT1
pub const TEGRA186_CLK_PLL_A_OUT1: c_int = 288;
// @deprecated
pub const TEGRA186_CLK_GPC2CLK: c_int = 289;
// A fake clock which must be enabled during KFUSE read operations to ensure adequate VDD_CORE voltage.
pub const TEGRA186_CLK_KFUSE: c_int = 293;
//
// @brief controls the PLLE hardware sequencer.
// @details This clock only has enable and disable methods. When the
// PLLE hw sequencer is enabled, PLLE, will be enabled or disabled by
// hw based on the control signals from the PCIe, SATA and XUSB
// clocks. When the PLLE hw sequencer is disabled, the state of PLLE
// is controlled by sw using clk_enable/clk_disable on
// TEGRA186_CLK_PLLE.
//
pub const TEGRA186_CLK_PLLE_PWRSEQ: c_int = 294;
// fixed 60MHz clock divided down from, TEGRA186_CLK_PLL_U
pub const TEGRA186_CLK_PLLREFE_REF: c_int = 295;
// @brief output of mux controlled by SOR0_CLK_SEL0 and SOR0_CLK_SEL1 in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0
pub const TEGRA186_CLK_SOR0_OUT: c_int = 296;
// @brief output of mux controlled by SOR1_CLK_SEL0 and SOR1_CLK_SEL1 in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1
pub const TEGRA186_CLK_SOR1_OUT: c_int = 297;
// @brief fixed /5 divider.  Output frequency of this clock is TEGRA186_CLK_PLLREFE_OUT1/5. Used as input for TEGRA186_CLK_EQOS_AXI
pub const TEGRA186_CLK_PLLREFE_OUT1_DIV5: c_int = 298;
// @brief controls the UTMIP_PLL (aka PLLU) hardware sqeuencer
pub const TEGRA186_CLK_UTMIP_PLL_PWRSEQ: c_int = 301;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL0_MGMT
pub const TEGRA186_CLK_PEX_USB_PAD0_MGMT: c_int = 302;
// @brief output of the divider CLK_RST_CONTROLLER_CLK_SOURCE_PEX_USB_PAD_PLL1_MGMT
pub const TEGRA186_CLK_PEX_USB_PAD1_MGMT: c_int = 303;
// @brief controls the UPHY_PLL0 hardware sqeuencer
pub const TEGRA186_CLK_UPHY_PLL0_PWRSEQ: c_int = 304;
// @brief controls the UPHY_PLL1 hardware sqeuencer
pub const TEGRA186_CLK_UPHY_PLL1_PWRSEQ: c_int = 305;
// @brief control for PLLREFE_IDDQ in CLK_RST_CONTROLLER_PLLREFE_MISC so the bypass output even be used when the PLL is disabled
pub const TEGRA186_CLK_PLLREFE_PLLE_PASSTHROUGH: c_int = 306;
// @brief output of the mux controlled by PLLREFE_SEL_CLKIN_PEX in CLK_RST_CONTROLLER_PLLREFE_MISC
pub const TEGRA186_CLK_PLLREFE_PEX: c_int = 307;
// @brief control for PLLREFE_IDDQ in CLK_RST_CONTROLLER_PLLREFE_MISC to turn on the PLL when enabled
pub const TEGRA186_CLK_PLLREFE_IDDQ: c_int = 308;
// @brief output of the divider QSPI_CLK_DIV2_SEL in CLK_RST_CONTROLLER_CLK_SOURCE_QSPI
pub const TEGRA186_CLK_QSPI_OUT: c_int = 309;
//
// @brief GPC2CLK-div-2
// @details fixed /2 divider. Output frequency is
// TEGRA186_CLK_GPC2CLK/2. The frequency of this clock is the
// frequency at which the GPU graphics engine runs.
pub const TEGRA186_CLK_GPCCLK: c_int = 310;
// @brief output of divider CLK_RST_CONTROLLER_AON_NIC_RATE
pub const TEGRA186_CLK_AON_NIC: c_int = 450;
// @brief output of divider CLK_RST_CONTROLLER_SCE_NIC_RATE
pub const TEGRA186_CLK_SCE_NIC: c_int = 451;
// Fixed 100MHz PLL for PCIe, SATA and superspeed USB
pub const TEGRA186_CLK_PLLE: c_int = 512;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC_BASE
pub const TEGRA186_CLK_PLLC: c_int = 513;
// Fixed 408MHz PLL for use by peripheral clocks
pub const TEGRA186_CLK_PLLP: c_int = 516;
// @deprecated

// @brief PLL controlled by CLK_RST_CONTROLLER_PLLD_BASE for use by DSI
pub const TEGRA186_CLK_PLLD: c_int = 518;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLD2_BASE for use by HDMI or DP
pub const TEGRA186_CLK_PLLD2: c_int = 519;
//
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLREFE_BASE.
// @details Note that this clock only controls the VCO output, before
// the post-divider. See TEGRA186_CLK_PLLREFE_OUT1 for more
// information.
//
pub const TEGRA186_CLK_PLLREFE_VCO: c_int = 520;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC2_BASE
pub const TEGRA186_CLK_PLLC2: c_int = 521;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC3_BASE
pub const TEGRA186_CLK_PLLC3: c_int = 522;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLDP_BASE for use as the DP link clock
pub const TEGRA186_CLK_PLLDP: c_int = 523;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLC4_BASE
pub const TEGRA186_CLK_PLLC4_VCO: c_int = 524;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLA1_BASE for use by audio clocks
pub const TEGRA186_CLK_PLLA1: c_int = 525;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLNVCSI_BASE
pub const TEGRA186_CLK_PLLNVCSI: c_int = 526;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLDISPHUB_BASE
pub const TEGRA186_CLK_PLLDISPHUB: c_int = 527;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLD3_BASE for use by HDMI or DP
pub const TEGRA186_CLK_PLLD3: c_int = 528;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLBPMPCAM_BASE
pub const TEGRA186_CLK_PLLBPMPCAM: c_int = 531;
// @brief PLL controlled by CLK_RST_CONTROLLER_PLLAON_BASE for use by IP blocks in the AON domain
pub const TEGRA186_CLK_PLLAON: c_int = 532;
// Fixed frequency 960MHz PLL for USB and EAVB
pub const TEGRA186_CLK_PLLU: c_int = 533;
// fixed /2 divider. Output frequency is TEGRA186_CLK_PLLC4_VCO/2
pub const TEGRA186_CLK_PLLC4_VCO_DIV2: c_int = 535;
// @brief NAFLL clock source for AXI_CBB
pub const TEGRA186_CLK_NAFLL_AXI_CBB: c_int = 564;
// @brief NAFLL clock source for BPMP
pub const TEGRA186_CLK_NAFLL_BPMP: c_int = 565;
// @brief NAFLL clock source for ISP
pub const TEGRA186_CLK_NAFLL_ISP: c_int = 566;
// @brief NAFLL clock source for NVDEC
pub const TEGRA186_CLK_NAFLL_NVDEC: c_int = 567;
// @brief NAFLL clock source for NVENC
pub const TEGRA186_CLK_NAFLL_NVENC: c_int = 568;
// @brief NAFLL clock source for NVJPG
pub const TEGRA186_CLK_NAFLL_NVJPG: c_int = 569;
// @brief NAFLL clock source for SCE
pub const TEGRA186_CLK_NAFLL_SCE: c_int = 570;
// @brief NAFLL clock source for SE
pub const TEGRA186_CLK_NAFLL_SE: c_int = 571;
// @brief NAFLL clock source for TSEC
pub const TEGRA186_CLK_NAFLL_TSEC: c_int = 572;
// @brief NAFLL clock source for TSECB
pub const TEGRA186_CLK_NAFLL_TSECB: c_int = 573;
// @brief NAFLL clock source for VI
pub const TEGRA186_CLK_NAFLL_VI: c_int = 574;
// @brief NAFLL clock source for VIC
pub const TEGRA186_CLK_NAFLL_VIC: c_int = 575;
// @brief NAFLL clock source for DISP
pub const TEGRA186_CLK_NAFLL_DISP: c_int = 576;
// @brief NAFLL clock source for GPU
pub const TEGRA186_CLK_NAFLL_GPU: c_int = 577;
// @brief NAFLL clock source for M-CPU cluster
pub const TEGRA186_CLK_NAFLL_MCPU: c_int = 578;
// @brief NAFLL clock source for B-CPU cluster
pub const TEGRA186_CLK_NAFLL_BCPU: c_int = 579;
// @brief input from Tegra's CLK_32K_IN pad
pub const TEGRA186_CLK_CLK_32K: c_int = 608;
// @brief output of divider CLK_RST_CONTROLLER_CLK_M_DIVIDE
pub const TEGRA186_CLK_CLK_M: c_int = 609;
// @brief output of divider PLL_REF_DIV in CLK_RST_CONTROLLER_OSC_CTRL
pub const TEGRA186_CLK_PLL_REF: c_int = 610;
// @brief input from Tegra's XTAL_IN
pub const TEGRA186_CLK_OSC: c_int = 612;
// @brief clock recovered from EAVB input
pub const TEGRA186_CLK_EQOS_RX_INPUT: c_int = 613;
// @brief clock recovered from DTV input
pub const TEGRA186_CLK_DTV_INPUT: c_int = 614;
// @brief SOR0 brick output which feeds into SOR0_CLK_SEL mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR0
pub const TEGRA186_CLK_SOR0_PAD_CLKOUT: c_int = 615;
// @brief SOR1 brick output which feeds into SOR1_CLK_SEL mux in CLK_RST_CONTROLLER_CLK_SOURCE_SOR1
pub const TEGRA186_CLK_SOR1_PAD_CLKOUT: c_int = 616;
// @brief clock recovered from I2S1 input
pub const TEGRA186_CLK_I2S1_SYNC_INPUT: c_int = 617;
// @brief clock recovered from I2S2 input
pub const TEGRA186_CLK_I2S2_SYNC_INPUT: c_int = 618;
// @brief clock recovered from I2S3 input
pub const TEGRA186_CLK_I2S3_SYNC_INPUT: c_int = 619;
// @brief clock recovered from I2S4 input
pub const TEGRA186_CLK_I2S4_SYNC_INPUT: c_int = 620;
// @brief clock recovered from I2S5 input
pub const TEGRA186_CLK_I2S5_SYNC_INPUT: c_int = 621;
// @brief clock recovered from I2S6 input
pub const TEGRA186_CLK_I2S6_SYNC_INPUT: c_int = 622;
// @brief clock recovered from SPDIFIN input
pub const TEGRA186_CLK_SPDIFIN_SYNC_INPUT: c_int = 623;
//
// @brief subject to change
// @details maximum clock identifier value plus one.
//
pub const TEGRA186_CLK_CLK_MAX: c_int = 624;
// @}
