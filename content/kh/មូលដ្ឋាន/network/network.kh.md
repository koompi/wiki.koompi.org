---
title: "ព័ត៌មានពី Network"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
បណ្តាញត្រូវបានរួមបញ្ចូលដោយកុំព្យូទ័រពីរឬច្រើនដែលត្រូវបានភ្ជាប់ទៅនឹងការចែករំលែកធនធាន (ដូចជា ម៉ាស៊ីនព្រីននិងស៊ីឌី) ការផ្លាស់ប្តូរឯកសារ ឬប្រព័ន្ធទំនាក់ទំនងអេឡិចត្រូនិច។  NetworkManager គឺជាការ កំណត់រចនាសម្ព័ន្ធបណ្តាញនៅលើចំណុចប្រទាក់ដែលជាឧបករណ៍ប្រើប្រាស់ software ដែលមានគោល បំណងធ្វើឱ្យការប្រើប្រាស់បណ្តាញកុំព្យូទ័រមានភាពងាយស្រួល។

---
---
## ពិនិត្យមើលបណ្តាញ 
ដើម្បីធានាអំពីការភ្ជាប់បណ្តាញ សូមពិនិត្យលក្ខខណ្ឌដូចខាងក្រោម៖
1. `ចំណុចប្រទាក់បណ្តាញ` របស់អ្នកត្រូវបានចុះបញ្ជីនិងបើកដំណើរការ។ បើមិនដូច្នោះទេ សូមពិនិត្យ​​ មើលកម្មវិធីបញ្ជាឧបករណ៍។
2. ត្រូវប្រាកដថាអ្នកបានភ្ជាប់ជាមួយខ្សែបណ្តាញឬភ្ជាប់ដោយឥតខ្សែ LAN ។
3. ​ចំណុចប្រទាក់បណ្តាញរបស់អ្នកមាន `អាស័យដ្ឋាន IP` ។
4. អ្នកអាច `ping` រកឃើញអាសយដ្ឋាន IPមូលដ្ឋានណាមួយ (ឧ.ផ្លូវចេញចូលលំនាំដើម) 
`The default gateway.`
5. អ្នកក៏អាច `ping` អាស័យដ្ឋាន IP សាធារណៈ (ឧ.Google DNS) ។

## កំណត់សេវាកម្មកម្មវិធីគ្រប់គ្រងបណ្តាញឡើងវិញ
វិធីងាយស្រួលបំផុតគឺ ចាប់ផ្តើមបណ្តាញរបស់អ្នកឡើងវិញដោយប្រើពាក្យបញ្ជានេះ។ វាស្មើនឹងធ្វើតាមក្រាហ្វិច (ចាប់ផ្តើមសេវាកម្មបណ្តាញអ្នកគ្រប់គ្រងឡើងវិញ) ។

```
sudo service network-manager restart
```
{{< notice note >}}
រូបតំណាងបណ្តាញគួរតែបាត់មួយភ្លែតហើយបន្ទាប់មកលេចឡើងម្តងទៀត។
{{< /notice >}}

## កំណត់ប្រព័ន្ធឡើងវិញនៃអ្នកគ្រប់គ្រងបណ្តាញ
ពាក្យបញ្ជា **សេវាកម្ម** គ្រាន់តែជាការផ្តុំវិធីសាស្ត្រ(និងសម្រាប់ស្គ្រីប init.d និងពាក្យបញ្ជាឡើងលើ) ។ ពាក្យបញ្ជា `systemctl` គឺមានលក្ខណៈល្អប្រសើរជាង **សេវាកម្ម** ។
```
sudo systemctl restart NetworkManager.service
```
{{< notice note >}}
រូបតំណាងបណ្តាញ (ម្តងទៀត) គួរតែបាត់មួយភ្លែត។ ដើម្បីពិនិត្យមើលជម្រើស `systemctl` ផ្សេងទៀតអ្នក អាច​យោងទៅទំព័រមេរបស់វា។
{{< /notice >}}
## ការបិទនិងបើកកម្មវិធីគ្រប់គ្រងបណ្តាញ
នេះគឺជាឧបករណ៍មួយទៀតសម្រាប់ដោះស្រាយបណ្តាញ។ វាគឺជាឧបករណ៍ដែលមានអនុភាពគួរសម​ ដែលអ្នកអាចប្រើប្រាស់វាបាន។  **sysadmins** ជាច្រើនចូលចិត្តវា ព្រោះវាងាយស្រួលប្រើប្រាស់។

វិធីនេះមានពីរផ្នែកដែលត្រូវការបិទបណ្តាញហើយនឹងបើកវាវិញ។
```
sudo nmcli networking off
sudo nmcli networking on
```
{{< notice info >}}
អ្នកអាចពិនិត្យមើលទំព័ររបស់ [`nmcli`](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwihuZzuxIjqAhWIaCsKHU00BTEQFjAAegQIARAB&url=https%3A%2F%2Fdeveloper.gnome.org%2FNetworkManager%2Fstable%2Fnmcli.html&usg=AOvVaw2V2s22-Jb9gHnm4zLaT3KQ) សម្រាប់ជម្រើសជាច្រើនទៀត។
{{< /notice >}}

## Ping អាស័យដ្ឋាន
Ping ត្រូវបានប្រើជាពាក្យបញ្ជាសម្រាប់ការសាកល្បងប្រសិនបើអ្នកអាចភ្ជាប់ទៅកាន់គោលដៅរឺអត់។
 ```
ping www.example.com
```
ខាងក្រោមនេះគឺជាឧទាហរណ៍៖
```
koompi@koompi-os$~ ping www.google.com
PING www.google.com (172.217.194.147) 56(84) bytes of data.
64 bytes from 172.217.194.147: icmp_seq=1 ttl=109 time=32.3 ms
64 bytes from 172.217.194.147: icmp_seq=2 ttl=109 time=33.6 ms
64 bytes from 172.217.194.147: icmp_seq=3 ttl=109 time=33.5 ms
```

{{< notice note >}}
ប្រសិនបើអ្នកទទួលបានបញ្ហាបែបនេះ `pingL icmp open socket: បOperation not permitted` នៅពេលប្រតិបត្តិ `ping` ព្យាយាមតំឡើងកញ្ចប់ [iputils] (#)​។
{{< /notice >}}

## IP Address
**Internet Protocol Address (IP Address)** គឺជាស្លាកលេខដែលត្រូវបានចាត់ចូលឧបករណ៍នីមួយៗដែលភ្ជាប់ ទៅនឹង `បណ្តាញកុំព្យូទ័រ` ដែលប្រើ `internet protocol` សម្រាប់ទំនាក់ទំនង។

អាសយដ្ឋាន IP បម្រើមុខងារសំខាន់ពីរ៖
1. ម៉ាស៊ីនឬបណ្តាញចំណុចប្រទាក់ `អត្តសញ្ញាណ`
2. ទីតាំង `អាស័យដ្ឋាន` ។

Internet Protocol `version 4` **(IPv4)**  កំណត់អាស័យដ្ឋាន IP ជា`៣២ប៊ីត`។

ដើម្បីដឹងថាអាស័យដ្ឋាន IP មួយណាដែលកុំព្យូទ័ររបស់អ្នកមាន អ្នកប្រើពាក្យបញ្ជា `ip` ជាមួយវត្ថុបំណង ` អាស័យដ្ឋាន`។​ សកម្មភាពលំនាំដើមគឺ` បង្ហាញ `ដែលរាយអាស័យដ្ឋាន IP ។ អ្នកក៏អាចលុប `បង្ហាញ`និង
អក្សរ​​សង្ខេប`អាស័យដ្ឋាន`ជា «addr»ឬ«a»។

ពាក្យបញ្ជាខាងក្រោមបង្ហាញតម្លៃស្មើគ្នាទាំងអស់៖
{{% tabs %}}
  {{% tab "ជម្រើសទីមួយ" %}}
   ```
   ip address show
   ```
  {{% /tab %}}

  {{% tab "ជម្រើសទីពីរ" %}}
  ```
  ip addr show
  ```
  {{% /tab %}}
  {{% tab "ជម្រើសទីបី" %}}
  ```
  ip addr 
  ```
  {{% /tab %}}
   {{% tab "ជម្រើសទីបួន" %}}
  ```
  ip a
  ```
  {{% /tab %}}

{{% tabs %}}

ការបង្ហាញមានខាងក្រោម៖
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
យើងអាចមើលឃើញអាស័យដ្ឋាន IP ពីររួមជាមួយព័ត៌មានជាច្រើនទៀត។ អាសយដ្ឋាន IP ត្រូវបានភ្ជាប់ជាមួយ​ឧបករណ៍ ត្រួតពិនិត្យចំណុចប្រទាក់បណ្តាញ (NICs) ។ អាសយដ្ឋាន IP ដំបូងគឺជាអាស័យដ្ឋានរង្វិលជុំ (ខាងក្នុង) ដែលត្រូវបានប្រើដើម្បីទំនាក់ទំនងនៅក្នុងកុំព្យូទ័រ។ ឯទីពីរគឺអាសយដ្ឋាន IP ពិតប្រាកដ (ខាងក្រៅ) ដែលកុំព្យូទ័រ​ មានលើបណ្តាញមូលដ្ឋាន (LAN) ។


សូមបំបែករាល់ព័ត៌មានដែលយើងបានទទួល៖

- **lo**: ឈ្មោះចំណុចប្រទាក់បណ្តាញជាអក្សរ។

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

## បង្ហាញតែអាស័យដ្ឋាន IPv4 និង IPv6
ប្រសិនបើអ្នកចង់កំណត់លទ្ធផលទៅអាស័យដ្ឋាន IP ជំនាន់ទី ៤ អ្នកអាចប្រើជម្រើស `-4` ដូចខាងក្រោម៖

```
ip -4 addr
```
លទ្ធផល៖
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    inet 192.168.10.107/24 brd 192.168.10.255 scope global dynamic noprefixroute wlan0
       valid_lft 4487sec preferred_lft 4487sec
```
ប្រសិនបើអ្នកចង់កំណត់លទ្ធផលទៅអាស័យដ្ឋាន IP ជំនាន់ ៦ អ្នកអាចប្រើជម្រើស `-6` ដូចខាងក្រោមៈ
```
ip -6 addr
```
លទ្ធផល៖
```
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 state UNKNOWN qlen 1000
    inet6 ::1/128 scope host 
       valid_lft forever preferred_lft forever
2: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 state UP qlen 1000
    inet6 fe80::ba0c:6781:2174:12f4/64 scope link noprefixroute 
       valid_lft forever preferred_lft forever
```

## បង្ហាញព័ត៌មានសម្រាប់ Interfaceមួយ
ប្រសិនបើអ្នកចង់ឃើញព័ត៌មានអាស័យដ្ឋាន IP ជា Interface អ្នកអាចប្រើជម្រើស `show` និង `dev` ហើយដាក់ឈ្មោះថាចំណុចប្រទាក់ដូចបង្ហាញខាងក្រោម។
```
ip addr show dev <Network_Interface_Name>
```
ឧទាហរណ៍៖
```
ip addr show dev lo
```
លទ្ធផល៖
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
ដើម្បីមើលឃើញចំណុចប្រទាក់បណ្តាញតែមួយ​ អ្នកគ្រាន់តែបន្ថែមឈ្មោះរបស់វាទៅពាក្យបញ្ជាដូចដែលបានបង្ហាញខាងក្រោម៖
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