kernel_image="../linux/arch/x86/boot/bzImage"
rootfs_img=$PWD"/rootfs_img"

qemu-system-x86_64 \
-netdev "user,id=eth0" \
-device "e1000,netdev=eth0" \
-object "filter-dump,id=eth0,netdev=eth0,file=dump.dat" \
-kernel $kernel_image \
-append "root=/dev/ram rdinit=sbin/init ip=10.0.2.15::10.0.2.1:255.255.255.0 console=ttyS0 no_timer_check" \
-nographic \
-initrd $rootfs_img
