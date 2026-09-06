use core::arch::asm;

pub unsafe fn get_vendor() -> [u8; 12] {
    let mut vendor = [0u8; 12];
    asm!(
        "push rbx",
        "cpuid",
        "mov [{0}], ebx",
        "mov [{0}+4], edx",
        "mov [{0}+8], ecx",
        "pop rbx",
        in(reg) vendor.as_mut_ptr(),
        in("eax") 0,
        out("edx") _,
        out("ecx") _,
        options(preserves_flags)
    );
    vendor
}
