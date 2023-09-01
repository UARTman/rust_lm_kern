# Create a directory which will be our ISO root.
mkdir -p target/iso_root
 
# Copy the relevant files over.
cp -v target/x86_64-unknown-none/debug/rust_lm_kern limine.cfg limine/limine-bios.sys \
      limine/limine-bios-cd.bin limine/limine-uefi-cd.bin target/iso_root/
 
# Create the EFI boot tree and copy Limine's EFI executables over.
mkdir -p target/iso_root/EFI/BOOT
cp -v limine/BOOTX64.EFI target/iso_root/EFI/BOOT/
cp -v limine/BOOTIA32.EFI target/iso_root/EFI/BOOT/
 
# Create the bootable ISO.
xorriso -as mkisofs -b limine-bios-cd.bin \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        --efi-boot limine-uefi-cd.bin \
        -efi-boot-part --efi-boot-image --protective-msdos-label \
        target/iso_root -o target/image.iso
 
# Install Limine stage 1 and 2 for legacy BIOS boot.
./limine/limine bios-install target/image.iso