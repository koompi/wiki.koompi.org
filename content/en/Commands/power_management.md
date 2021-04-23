
## Power management
Power Management is a feature that turns off the power or switches system's components to a low-power state when inactive, which is located at the corner of the screen.

You can also run those services through Konsole also. So, This is the guide for those commands.

#### Poweroff computer

If you want to shut down your system you can follow the command below:
```
poweroff    
```
#### Reboot computer
Reboot is referring to restart your system. After you run it, it immediately resets your system.
```
reboot  
```
#### Hibernate computer
**systemd** provides commands to *suspend* to RAM or *hibernate* using the kernel's native suspend/resume functionality. There are also mechanisms to add hooks to customize *pre-* and *post-suspend* actions.

To cut power to unneeded systems while RAM still has enough power to run, type the command below
```
systemctl hibernate
```
#### Suspend computer
Powers down the computer while retaining its state, which saves all contents access memory to HARD DISK. You can run the following line:
```
systemctl suspend
```
----
----
