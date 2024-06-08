fn main() {
    use std::{env, fs, path::PathBuf};

    let base_address = 0x8020_0000usize;
    let ld = &PathBuf::from(env::var("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(
        ld,
        format!(
            "\
OUTPUT_ARCH(riscv)
ENTRY(__entry)
SECTIONS {{
    . = {base_address};
    skernel = .;
    stext = .;
    .text : {{
        *(.text.entry)
        *(.text .text.*)
    }}

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {{
        *(.rodata .rodata.*)
    }}

    . = ALIGN(4K);
    erodata = .;
    s_data = .;
    .data : {{
        *(.data .data.*)
    }}
    e_data = .;

    . = ALIGN(4K);
    e_data = .;
    .bss : {{
        *(.bss.stack)
        s_bss = .;
        *(.sbss .bss .bss.*)
        e_bss = .;
    }}

    . = ALIGN(4K);
    e_bss = .;
    ekernel = .;

    /DISCARD/ : {{
        *(.eh_frame)
    }}
}}"
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}