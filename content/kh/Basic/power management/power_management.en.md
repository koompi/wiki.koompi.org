---
title: "Power Management"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
Power Management is a feature that turns off the power or switches system's components to a low-power state when inactive, which is located at the corner of the screen.

You can also run those services through Konsole also. So, This is the guide for those commands.

---
---

## Poweroff and Reboot
If you want to shut down your system you can follow the command below:
```
poweroff    
```
Reboot is referring to restart your system. After you run it, it immediately resets your system.
```
reboot  
```
## Hibernate and Suspend
[systemd](https://www.koompi.org/basic/system-manager/system_manager/) provides commands to `suspend` to RAM or `hibernate` using the kernel's native suspend/resume functionality. There are also mechanisms to add hooks to customize `pre-` and `post-suspend` actions.

To cut power to unneeded systems while RAM still has enough power to run, type the command below
```
systemctl suspend
```

Powers down the computer while retaining its state, which saves all contents access memory to HARD DISK. You can run the following line:
```
systemctl hibernate
```

----
----