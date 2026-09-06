use core::arch::asm;

pub struct CpuInfo;

impl CpuInfo {
    pub unsafe fn get_vendor() -> [u8; 12] {
        let ebx: u32;
        let edx: u32;
        let ecx: u32;

        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            out(reg) ebx,
            out("edx") edx,
            out("ecx") ecx,
            in("eax") 0,
            options(nomem, preserves_flags)
        );

        let mut vendor = [0u8; 12];
        let bytes_ebx = ebx.to_le_bytes();
        let bytes_edx = edx.to_le_bytes();
        let bytes_ecx = ecx.to_le_bytes();

        vendor[0..4].copy_from_slice(&bytes_ebx);
        vendor[4..8].copy_from_slice(&bytes_edx);
        vendor[8..12].copy_from_slice(&bytes_ecx);

        vendor
    }
}
