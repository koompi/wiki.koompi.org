---
title: "ព័ត៌មានពី System Manager"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---

**Systemctl** is init system and `system manager` that is widely becoming the new standard for opensource machines. While there are considerable opinions about whether **systemd** is an improvement over the traditional `SysV` init systems it is replacing, the majority of distributions plan to adopt it or have already done so.

Due to its heavy adoption, familiarizing yourself with `systemd` is well worth the trouble, as it will make administering servers considerably easier. Learning about and **utilizing the tools and daemons** that comprise `systemd` will help you better appreciate the power, flexibility, and capabilities it provides, or at least help you to do your job with minimal hassle.

In this guide, we will be discussing the `systemctl` command, which is the central management tool for controlling the init system. We will cover how to manage services, check statuses, change `system states`, and work with the `configuration files`.

{{< notice note >}}
Please note that although `systemd` has become the default init system for many **Linux distributions**, it isn’t implemented universally across all distros. As you go through this tutorial, if your terminal outputs the `error bash: systemctl` is not installed then it is likely that **your machine has a different init system installed**.
{{< /notice >}}

----
----

## System Service Management
**The fundamental purpose** of an init system is to initialize the components that must be started after the **Linux kernel** is booted `(traditionally known as “userland” components)`. The init system is also used to manage services and daemons for the server at any point while the system is running. With that in mind, we will start with some simple **service management operations.**

In `systemd`, the target of most actions are **“units”**, which are resources that `systemd` knows how to manage. Units are categorized by the type of resource they represent and they are defined with files known as unit files. The type of each unit can be inferred from the suffix on the end of the file.

For **service management** tasks, the target unit will be service units, which have unit files with a suffix of `.service`. However, for most service management commands, you can actually leave off the `.service suffix`, as **systemd** is smart enough to know that you probably want to operate on a service when using service management commands.

## Basic systemctl usage 
The main command used to introspect and control systemd is `systemctl`, which has mentioned above. Some of its uses are examining the `system state` and managing the system and services.

## Analyzing the system state
The commands so far have been useful for managing single services, but they are not very helpful for exploring the current state of the system. There are a number of `systemctl` commands that provide this information.

Show system status using: 
```
systemctl status
```
## List Running Units
To see a list of all of the active units that **systemd** knows about, we can use the `list-units` command:
```
systemctl list-units
```
Since the `list-units` command shows only active units by default, all of the entries above will show `“loaded”` in the LOAD column and `“active”` in the **ACTIVE** column. This display is actually the default behavior of **systemctl** when called without additional commands, so you will see the same thing if you call **systemctl** with no arguments:
```
systemctl
```
We can tell **systemctl** to output different information by adding additional flags. For instance, to see all of the units that **systemd** has loaded (or attempted to load), regardless of whether they are currently active, you can use the `--all` flag, like this:
```
systemctl list-units --all
```
This will show any unit that **systemd** loaded or attempted to load, regardless of its current state on the system. Some units become inactive after running, and some units that **systemd** attempted to load may have not been found on disk.

## List Failed Units 
```
systemctl --failed
```
If **your output** has beedn provided like below, It means that your system does not have any `UNIT Failed`.
```
UNIT LOAD ACTIVE SUB DESCRIPTION
0 loaded units listed.
```
## Using Units
**Start** a unit immediately:
```
sudo systemctl start <unit_name>
```
Sample for starting `nginx.service`
```
sudo systemctl start nginx.service
```
{{< notice note >}}
After you run the command, there will be nothing showing on your desktop. You can check if it really running or not with flag `status`.
{{< /notice >}}

{{< notice info >}}
All the flag are being used the same as sample have been given above.
{{< /notice >}}

**Stop** a unit immediately:
```
sudo systemctl stop <unit_name>
```
**Restart** a unit:
```
sudo systemctl restart <unit_name>
```
Ask a unit to **reload** its configuration:
```
sudo systemctl reload <unit_name>
```
**Show the status** of a unit, including whether it is running or not:
```
systemctl status <unit_name>
```
Check whether a unit is already **enabled** or not:
```
systemctl is-enabled <unit_name>
```
**Enable a unit** to be started on bootup:
```
sudo systemctl enable <unit_name>
```
**Enable** a unit to be started on bootup and Start immediately:
```
sudo systemctl enable --now <unit_name>
```
**Disable a unit** to not start during bootup:
```
sudo systemctl disable <unit_name>
```
**Mask** a unit to make it impossible to start it (both manually and as a dependency, which makes masking dangerous):
```
sudo systemctl mask <unit_name>
```
**Unmask** a unit:
```
sudo  systemctl unmask <unit_name>
```
Show the **manual page** associated with a unit (this has to be supported by the unit file):
```
systemctl help <unit_name>
```
**Reload systemd** manager `configuration`, scanning for `new` or `changed units`:
{{< notice note >}}
This does not ask the changed units to reload their own configurations. See reload example above.
{{< /notice >}}
```
systemctl daemon-reload
```

## Removing Units
To remove any additions you have made, either delete the unit’s `.d` configuration directory or the modified service file from `/etc/systemd/system`. For instance, to remove a snippet, we could type:
```
sudo rm -r /etc/systemd/system/<unit_name>
```
To remove a full modified unit file, we would type:
```
sudo rm /etc/systemd/system/<unit_name>
```
After deleting the file or directory, you should reload the `systemd` process so that it no longer attempts to reference these files and reverts back to using the system copies. You can do this by typing:
```
sudo systemctl daemon-reload
```

## Getting and Setting the Default Target

The systemd process has a default target that it uses when booting the system. Satisfying the cascade of dependencies from that single target will bring the system into the desired state. To find the default target for your system, type:
```
systemctl get-default
```

```
multi-user.target
```
If you wish to set a different default target, you can use the set-default. For instance, if you have a graphical desktop installed and you wish for the system to boot into that by default, you can change your default target accordingly:
```
sudo systemctl set-default graphical.target
```

---
---