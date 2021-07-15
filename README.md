# EtherCAT test

## TODO

- Load ESI (EtherCAT Slave Information) XML file into runtime with https://docs.rs/ethercat-esi/0.2.0/ethercat_esi/.

## Set up EtherCAT master

Uses EtherCAT master from <https://etherlab.org/en/ethercat/>

Forum stuff [here](https://forum.linuxcnc.org/24-hal-components/22346-ethercat-hal-driver?limit=6&start=636#119224), [here](https://www.forum.linuxcnc.org/27-driver-boards/35591-beckhoff-ethercat-64-with-bit-linuxcnc-how-to-install?start=0).

Some decent EtherCAT setup docs at <https://etherlab.org/download/ethercat/ethercat-1.5.2.pdf> which is where most of the below instructions are from

```bash
git clone https://gitlab.com/etherlab.org/ethercat.git
cd ethercat
./bootstrap
# Note I'm disabling EoE (ethernet over ethercat) to squelch the annoying "could not configure
# interface" message - the eoe device shows up as a new network device which fails to configure.
# I guess you could disable it from some ethernet config whatever but I don't need it anyway.
./configure --disable-8139too --enable-generic --disable-eoe
make -j4
make modules -j4
sudo make install
sudo make modules_install
# MAC address of network interface to use
# You can also use `MASTER0_DEVICE="ff:ff:ff:ff:ff:ff"` if you just want the first device that can be found.
echo 'MASTER0_DEVICE="ab:cd:ef:ab:cd:ef"' | sudo tee -a /etc/ethercat.conf
# If this line is missing, the network device will never get hooked up to the ethercat stuff
echo 'DEVICE_MODULES="generic"' | sudo tee -a /etc/ethercat.conf
# Updates kernel module list
sudo depmod
# Allow non-root access to /dev/EtherCAT0
echo 'KERNEL=="EtherCAT[0-9]*", MODE="0664" GROUP="plugdev"' | sudo tee -a /etc/udev/rules.d/99-ethercat.rules
# Inserts ec_generic kernel module and makes /dev/EtherCAT0 available
# This must also be run if /etc/ethercat.conf is changed
sudo ethercatctl restart
```

Run the Rust code:

```bash
ETHERCAT_PATH=/home/james/Repositories/ethercat-shit/ethercat cargo run
```

## USB ethernet adapter

Name from `ip link list` is `enx00e1000003fb`

MAC is `00:e1:00:00:03:fb`

## Notes

- TODO: Make module load on startup
- It IS possible to use EtherCAT over your normal ethernet connection and get internet too
- It IS possible to communicate through an ethernet switch - hooray for not needing multiple NICs.
- I got ethercat working over a USB GBE adaptor too
- My `/etc/ethercat.conf`:

  ```
  # MASTER0_DEVICE="ff:ff:ff:ff:ff:ff"
  # Mobo ethernet
  MASTER0_DEVICE="70:85:c2:8d:d3:c4"
  # USB ethernet
  # MASTER0_DEVICE="00:e1:00:00:03:fb"
  DEVICE_MODULES="generic"
  ```

- I did not use <https://github.com/sittner/ec-debianize> - maybe that's an easier way to go, but it is using the outdated Mercurial repo at time of writing. Note that this talks about `/etc/default/ethercat` a lot, which AFAICS is the same as `/etc/ethercat.conf` but automated.
- Gotta disable the network interface autoconfig or you'll get annoying config warnings
- If EoE support is enabled, you get a network interface that looks like this:

  ```
  28: eoe0s0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UNKNOWN mode DEFAULT group default qlen 1000
      link/ether 00:11:22:33:44:1c brd ff:ff:ff:ff:ff:ff
  ```
