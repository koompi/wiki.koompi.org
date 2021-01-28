---
title: "Network Info"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
Network is included by two or more computers that are linked to share resources (such as printers and CDs), exchange files, or allow electronic communications. NetworkManager is the configuration of the network on the interfaces, which is the software utility that aims to simplify the use of computer networks.  

---
---
## Check the connection 
To ensure​the network connection, go through the following conditions below:
 1. Your `network interface` is listed and enabled. Otherwise, check the device driver.
 2. Making sure that you are connected with a network cable or connected to the `wireless LAN`.
 3. Your network interface has an `IP Address`.
 4. You can `ping` a local IP Address (e.g. `The default gateway.`)
 5. You can also `ping` a public IP Address(e.g. `The google DNS server`).
## Reset Network Manager Service
This is the easiest way to restart your network using the command line. It’s equivalent to the graphical way of doing it (restarts the Network-Manager service).
```
sudo service network-manager restart
```
{{< notice note >}}
The network icon should disappear for a moment and then reappear.
{{< /notice >}}
## Reset System of Network Manager
The **service** command is just a wrapper for this method (and also for init.d scripts and Upstart commands). The `systemctl` command is much more versatile than **service**. This is what I usually prefer.
```
sudo systemctl restart NetworkManager.service
```
{{< notice note >}}
The network icon (again) should disappear for a moment. To check out other `systemctl` options, you can refer to its man page.
{{< /notice >}}
## Turn OFF and ON Network Manager
This is yet another tool for handling networks. It is a pretty powerful tool that you can find it very practical. Many sysadmins prefer it since it is easy to use.

There are two steps to this method: turning the network off, and then turning it back on.
```
sudo nmcli networking off
sudo nmcli networking on
```
{{< notice info >}}
You can check out the man page of [nmcli](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwihuZzuxIjqAhWIaCsKHU00BTEQFjAAegQIARAB&url=https%3A%2F%2Fdeveloper.gnome.org%2FNetworkManager%2Fstable%2Fnmcli.html&usg=AOvVaw2V2s22-Jb9gHnm4zLaT3KQ) for more options.
{{< /notice >}}

## Ping address
Ping has been used as the command for testing if you have reached to a host or not.
 ```
ping www.example.com
```
Down here is the example:
```
koompi@koompi-os$~ ping www.google.com
PING www.google.com (172.217.194.147) 56(84) bytes of data.
64 bytes from 172.217.194.147: icmp_seq=1 ttl=109 time=32.3 ms
64 bytes from 172.217.194.147: icmp_seq=2 ttl=109 time=33.6 ms
64 bytes from 172.217.194.147: icmp_seq=3 ttl=109 time=33.5 ms
```

{{< notice note >}}
If you receive an error like `pingL icmp open socket: Operation not permitted` when executing `ping`, try to re-install [iputils](#) package.
{{< /notice >}}

## IP Address
An **Internet Protocol Address (IP Address)** is a numerical label assigned to each device connected to a `computer network` that uses the `internet protocol` for communication.

An IP address serves two main functions:
1. The host or Network interface `identification`
2. The location `addressing`.

Internet Protocol `version 4` **(IPv4)** defines an IP address as a `32-bit` number.


To discover which IP addresses your computer has, you use the `ip` command with the object `address`.The default action is `show`, which lists the IP addresses. You can also omit `show` and abbreviate `address` as "addr" or even "a".

The following commands are all equivalent:
{{% tabs %}}
  {{% tab "First Choice" %}}
   ```
   ip address show
   ```
  {{% /tab %}}

  {{% tab "Second Choice" %}}
  ```
  ip addr show
  ```
  {{% /tab %}}
  {{% tab "Third Choice" %}}
  ```
  ip addr 
  ```
  {{% /tab %}}
   {{% tab "Forth Choice" %}}
  ```
  ip a
  ```
  {{% /tab %}}

{{% tabs %}}

The output is down below:
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qle
    link/ether 34:41:5d:83:dd:fd brd ff:ff:ff:ff:ff:ff
    inet 192.168.10.107/24 brd 192.168.10.255 scope global dynamic noprefixroute wlan0
       valid_lft 5938sec preferred_lft 5938sec
    inet6 fe80::ba0c:6781:2174:12f4/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
```
We can see two IP addresses, along with a lot of other information. The IP addresses are associated with the network interface controller(NICs). The `ip` command tries to be helpful and provides a bunch of information about the interface, too.

The first IP address is the (internal) loopback address used to communicate within the computer. The second is the actual (external) IP address the computer has on the local area network (LAN).

Let's break down all the information we received:

- **lo**: The network interface name as a string.

- **<LOOPBACK,UP,LOWER_UP>**: This is a loopback interface. It is `UP`, meaning it's operation. The [Physical networking layer](https://en.wikipedia.org/wiki/OSI_model)**(layer one)** is also up.
- **mtu 65536**: The maximum transfer unit. This is the size of the largest chunk of data this interface can transmit.

- **qdisc noqueue**: A `qdisc` is a queuing mechanism. It schedules the transmission of packets. There are different queuing techniques called disciplines. The `noqueue` discipline means “send instantly, don’t queue.” This is the default `qdisc` discipline for virtual devices, such as the loopback address.

- **state UNKNOWN**: This can be `DOWN` (the network interface is not operational), `UNKNOWN` (the network interface is operational but nothing is connected), or `UP` (the network is operational and there is a connection).

- **group default**: Interfaces can be grouped logically. The default is to place them all in a group called “default.”

- **qlen 1000**: The maximum length of the transmission queue.

- **link/loopback**: The [media access control](https://en.wikipedia.org/wiki/MAC_address) (MAC) address of the interface.

- **inet 127.0.0.1/8**: The IP version 4 address. The part of the address after the forward-slash (/) is [Classless Inter-Domain Routing notation](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing#CIDR_notation) (CIDR) representing the subnet mask. It indicates how many leading contiguous bits are set to one in the subnet mask. The value of eight means eight bits. Eight bits set to one represents 255 in binary, so the subnet mask is 255.0.0.0.

- **scope host**: The IP address scope. This IP address is only valid inside the computer (the “host”).

- **lo**: The interface with which this IP address is associated.

- **valid_lft**: Valid lifetime. For an IP version 4 IP address allocated by [Dynamic Host Configuration Protocol](https://en.wikipedia.org/wiki/Dynamic_Host_Configuration_Protocol) (DHCP), this is the length of time the IP address is considered valid and able to make and accept connection requests.

- **preferred_lft**: Preferred lifetime. For an IP version 4 IP address allocated by `DHCP`, this is the amount of time the IP address can be used with no restrictions. This should never be larger than the `valid_lft` value. 

- **<BROADCAST,MULTICAST,UP,LOWER_UP>**: This interface supports `broad-` and `multicasting`, and the interface is UP (operational and connected). The hardware layer of the network (layer one) is also UP.

## Display Only IPv4 and IPv6 Addresses

If you want to limit the output to the IP version 4 addresses, you can use the `-4` option, as follows:

```
ip -4 addr
```
Output:
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    inet 192.168.10.107/24 brd 192.168.10.255 scope global dynamic noprefixroute wlan0
       valid_lft 4487sec preferred_lft 4487sec
```
if you want to limit the output to the IP version 6 addresses, you can use the `-6` option, as follows:
```
ip -6 addr
```
Output:
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 state UNKNOWN qlen 1000
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 state UP qlen 1000
    inet6 fe80::ba0c:6781:2174:12f4/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
```

## Display Information for Single Interface
If you want to see the IP address information for a single interface, you can use the `show` and `dev` options, and name the interface, as shown below.

```
ip addr show dev <Network_Interface_Name>
```
For example:
```
ip addr show dev lo
```
Output:
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
```
{{< notice note >}}
You can also use the -4 or -6 flag to further refine the output so you only see that in which you’re interested.
{{< /notice >}}

**Sample:** If you want to see the IP version 4 information related to addresses on interface `lo`, type the following command.
```
ip -4 addr show dev lo
```
Output:
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
```


## Adding an IP Address
You can use the `add` and `dev` options to add an IP address to an interface. You just have to tell the `ip` command which IP address to add, and to which interface to add it.

We’re going to add the IP address 192.168.4.44 to the `lo` interface. We also have to provide the CIDR notation for the subnet mask.

We type the following:
```
sudo ip addr add 192.168.4.44/24 dev lo
```
The new IP Address is present on the network interface.

{{< notice note >}}
We can jump on another computer and use the following command to see if we can `ping the new IP address`.
{{< /notice >}}
```
koompi@koompi-os$~ ping 192.168.4.44
PING 192.168.4.44 (192.168.4.44) 56(84) bytes of data.
64 bytes from 192.168.4.44: icmp_seq=1 ttl=64 time=0.039 ms
64 bytes from 192.168.4.44: icmp_seq=2 ttl=64 time=0.092 ms
64 bytes from 192.168.4.44: icmp_seq=3 ttl=64 time=0.109 ms
```
The IP address responds and sends back acknowledgments to the pings. Our new IP address is up and running after one simple `ip` command.

## Deleting IP Address
To delete an IP address, the command is almost the same as the one to add one, except you replace `add` with `del`, as shown below:
```
sudo ip addr del 192.168.4.44/24 dev lo
```
## Using IP With Network Interfaces

You use the `link` object to inspect and work with network interfaces. Type the following command to see the interfaces installed on your computer:
```
ip link show
```
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP mode DORMANT group default qlen 1000
    link/ether 34:41:5d:83:dd:fd brd ff:ff:ff:ff:ff:ff
```
To see a single network interface, just add its name to the command, as shown below:
```
ip link show <Network_Interface_Name>
```
Sample:
```
ip link show lo
```
### Starting and Stopping Links
You can use the `set` options with either `up` or `down` to **stop** or **start** network interface option. You can also have to use sudo (Super User), as shown below.
```
sudo ip link set <Network_Interface_Name> <up or down>
```
Sample:
```
sudo ip link set lo down
```
{{< notice note >}}
If you use the `up` option, your network interface will be restarted, and the state is shown as `UP`.
{{< /notice >}}

## Usinng IP With Routes
With the route object, you can inspect and manipulate routes. Routes define to where network traffic to different IP addresses is forwarded, and through which network interface.

If the destination computer or device shares a network with the sending computer, the sending computer can forward the packet directly to it.

However, if the destination device is not directly connected, the sending computer forwards the packet to the default router. The router then decides where to send the packet.

To see the routes defined on your computer, type the following command:
```
ip route
```
Output:
```
default via 192.168.10.1 dev wlan0 proto dhcp metric 600 
192.168.10.0/24 dev wlan0 proto kernel scope link src 192.168.10.107 metric 600 
```

Let’s take a look at the info we received:

- **default**: The default rule. This route is used if none of the other rules match what’s being sent.
- **via 192.168.10.1**: Routes the packets via the device at 192.168.10.1. This is the IP address of the default router on this network.
- **dev wlan0**: Use this network interface to send the packets to the router.
- **proto dhcp**: The routing protocol identifier. DHCP means the routes will be determined dynamically.
- **metric 100**: An indication of the preference of the route compared to others. Routes with lower metrics are preferentially used over those with higher metrics. You can use this to give preference to a wired network interface over a Wi-Fi one.

### Display Information for Single Route
If you want to focus on the details of a particular route, you can add the `list` option and IP address range of the route to the command as follows:
```
ip route list <ip_address>
```
### Adding Route
We just added a new network interface card to this computer. We type the following and it's showing up as `wlan2`:

We’ll add a new route to the computer to use this new interface. First, we type the following to associate an IP address with the interface:
```
sudo ip addr add 192.168.121.1/24 dev wlan2
```
A default route using the existing IP address is added to the new interface. We use the `delete` option, as shown below, to delete the route and provide its details:
```
sudo ip route delete default via 192.168.4.1 dev wlan2
```
We’ll now use the add option to add our new route. The new interface will handle network traffic in the 192.168.121.0/24 IP address range. We’ll give it a metric of 100; because it will be the only route handling this traffic, the metric is pretty much academic.

We type the following:
```
sudo ip route add 192.168.121.0/24 dev wlan2 metric 100
```
Now , typing `ip route` command to see what it gives. As our new route is now in place;however, we still have the 192.168.4.0/24 route that points to interface enp0s8—we type the following to remove it:
```
sudo ip route delete 192.168.4.0/24 dev wlan2
```

We should now have a new route that points all traffic destined for IP range 192.168.121.0/24 through interface `wlan2`. It should also be the only route that uses our new interface.

## Taken Route, Not Taken Root

The great thing about these commands is they’re not permanent. If you want to clear them, just reboot your system. This means you can experiment with them until they work the way you want. And it’s a very good thing if you make a terrible mess of your system—a simple reboot will restore order.

On the other hand, if you want the changes to be permanent, you have to do some more work. Exactly what varies depending on the distribution family, but they all involve changing config files.

This way, though, you can test-drive commands before you make anything permanent.

----