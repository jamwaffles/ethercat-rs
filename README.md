# EtherCAT test

# Archived: please see [ethercrab-rs/ethercrab](https://github.com/ethercrab-rs/ethercrab) instead

## TODO

- Load ESI (EtherCAT Slave Information) XML file into runtime with https://docs.rs/ethercat-esi/0.2.0/ethercat_esi/.

## Set up EtherCAT master

Uses EtherCAT master from <https://etherlab.org/en/ethercat/>

Forum stuff [here](https://forum.linuxcnc.org/24-hal-components/22346-ethercat-hal-driver?limit=6&start=636#119224), [here](https://www.forum.linuxcnc.org/27-driver-boards/35591-beckhoff-ethercat-64-with-bit-linuxcnc-how-to-install?start=0).

Some decent EtherCAT setup docs at <https://etherlab.org/download/ethercat/ethercat-1.5.2.pdf> which is where most of the below instructions are from

```bash
git clone https://gitlab.com/etherlab.org/ethercat.git
# Use the 1.5 stable branch (commit 779437f)
git checkout stable-1.5
cd ethercat
./bootstrap
# Note: I previously added --disable-eoe here to squelch some annoying Cinnamon popups, but I ended
# up going into network settings and disabling autoconnection instead. Adding --disable-eoe gives an
# `Inappropriate ioctl for device` for some reason.
./configure --disable-8139too --enable-generic
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

The ethernet device should look like this:

```
4: enx00e1000003fb: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 00:e1:00:00:03:fb brd ff:ff:ff:ff:ff:ff
```

Run the Rust code:

```bash
ETHERCAT_PATH=/home/james/Repositories/ethercat-shit/ethercat-src cargo run
```

Note that if `ethercat-src` was compiled with `--disable-eoe`, some bindings won't work as the symbols they're looking for no longer exist. I might PR a fix for this in the future as an optional feature, but it doesn't cause an issue with my stuff today.

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

## The AKD drive

EtherCAT stuff specific to the drive is here <https://www.kollmorgen.com/sites/default/files/public_downloads/903-200005-00%20AKD%20EtherCAT%20Communications%20Manual%20EN%20REV%20U.pdf>

Note that the `ethercat-esi` dependency is aliased to a local path, the repo of which points to <https://github.com/jamwaffles/ethercat-esi/tree/fixes-for-akd> which has some un-PRed fixes in it at time of writing.

`ethercat xml` gives this:

```xml
<?xml version="1.0" ?>
<EtherCATInfo>
  <!-- Slave 0 -->
  <Vendor>
    <Id>106</Id>
  </Vendor>
  <Descriptions>
    <Devices>
      <Device>
        <Type ProductCode="#x00414b44" RevisionNo="#x00000002">AKD</Type>
        <Name><![CDATA[AKD EtherCAT Drive (CoE)]]></Name>
        <Sm Enable="1" StartAddress="#x1800" ControlByte="#x26" DefaultSize="1024" />
        <Sm Enable="1" StartAddress="#x1c00" ControlByte="#x22" DefaultSize="1024" />
        <Sm Enable="1" StartAddress="#x1100" ControlByte="#x24" DefaultSize="0" />
        <Sm Enable="1" StartAddress="#x1140" ControlByte="#x20" DefaultSize="0" />
        <RxPdo Sm="2" Fixed="1" Mandatory="1">
          <Index>#x1600</Index>
          <Name></Name>
          <Entry>
            <Index>#x6040</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
        </RxPdo>
        <RxPdo Sm="2" Fixed="1" Mandatory="1">
          <Index>#x1601</Index>
          <Name></Name>
          <Entry>
            <Index>#x6040</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x6060</Index>
            <SubIndex>0</SubIndex>
            <BitLen>8</BitLen>
            <Name></Name>
            <DataType>UINT8</DataType>
          </Entry>
        </RxPdo>
        <RxPdo Sm="2" Fixed="1" Mandatory="1">
          <Index>#x1602</Index>
          <Name></Name>
          <Entry>
            <Index>#x6040</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x607a</Index>
            <SubIndex>0</SubIndex>
            <BitLen>32</BitLen>
            <Name></Name>
            <DataType>UINT32</DataType>
          </Entry>
        </RxPdo>
        <RxPdo Sm="2" Fixed="1" Mandatory="1">
          <Index>#x1603</Index>
          <Name></Name>
          <Entry>
            <Index>#x6040</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x60ff</Index>
            <SubIndex>0</SubIndex>
            <BitLen>32</BitLen>
            <Name></Name>
            <DataType>UINT32</DataType>
          </Entry>
        </RxPdo>
        <TxPdo Sm="3" Fixed="1" Mandatory="1">
          <Index>#x1a00</Index>
          <Name></Name>
          <Entry>
            <Index>#x6041</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
        </TxPdo>
        <TxPdo Sm="3" Fixed="1" Mandatory="1">
          <Index>#x1a01</Index>
          <Name></Name>
          <Entry>
            <Index>#x6041</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x6061</Index>
            <SubIndex>0</SubIndex>
            <BitLen>8</BitLen>
            <Name></Name>
            <DataType>UINT8</DataType>
          </Entry>
        </TxPdo>
        <TxPdo Sm="3" Fixed="1" Mandatory="1">
          <Index>#x1a02</Index>
          <Name></Name>
          <Entry>
            <Index>#x6041</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x6064</Index>
            <SubIndex>0</SubIndex>
            <BitLen>32</BitLen>
            <Name></Name>
            <DataType>UINT32</DataType>
          </Entry>
        </TxPdo>
        <TxPdo Sm="3" Fixed="1" Mandatory="1">
          <Index>#x1a03</Index>
          <Name></Name>
          <Entry>
            <Index>#x6041</Index>
            <SubIndex>0</SubIndex>
            <BitLen>16</BitLen>
            <Name></Name>
            <DataType>UINT16</DataType>
          </Entry>
          <Entry>
            <Index>#x606c</Index>
            <SubIndex>0</SubIndex>
            <BitLen>32</BitLen>
            <Name></Name>
            <DataType>UINT32</DataType>
          </Entry>
        </TxPdo>
      </Device>
    </Devices>
  </Descriptions>
</EtherCATInfo>
```

## Kollmorgen Workbench

Must connect over Service/HMI interface - connecting over EtherCAT bus never works lmao. Connect the HMI port to the local network as a normal device.

Kollmorgen Workbench works in VirtualBox, but the network interface must be set to "bridged" mode. I turned promiscuous mode to "all" but maybe that's not necessary, idk.

## SOEM/SOES

Simple Open EtherCAT Master/Slave

<https://crates.io/crates/soem>

<https://openethercatsociety.github.io/doc/soes/index.html>

<https://openethercatsociety.github.io/doc/soem/index.html>

C tutorial: <https://openethercatsociety.github.io/doc/soem/tutorial_8txt.html>

---

Also this one which seems newer <https://crates.io/crates/ethercat-soem> but incomplete.

## SDO/PDO difference

neotech
PDO is hardware registers mapped, SDO is software registers, that can float
SDO's are usually set on boot, as part of configuration
PDO is where shit actually happens
PDO's can be interupt read, SDO's cannot

jamwaffles
Right I see
So I'd set e.g. encoder counts per rev in an SDO, but set velocity/read position from PDO

neotech
yeap
<https://www.can-cia.org/fileadmin/resources/documents/proceedings/2005_rostan.pdf>
your servo most likely implements this

jamwaffles
It does indeed

neotech
in some shape or another at least
so reading up on the OpenCAN over ethercat standard is prob. a good starting point
i think it was called CIA 4 or something like that
