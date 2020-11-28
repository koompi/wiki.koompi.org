---
title: "Basic Commands"
date: 2018-12-28T11:02:05+06:00
icon: "ti-files"
description: "All available command lines supported in the open-source world "
type : "docs"
---
This section gives you the commands to keep your system up and running. Keep in mind that these commands can both install packages from any open-source repository and packages from the AUR.

{{< notice note >}}
If you do not know anything about the Konsole [Click here](https://www.koompi.org/details/#konsole-details).
{{< /notice >}}

As KOOMPI OS is operating base on Arch Linux and Plasma. This means your system always receives the latest packages.

----

## Pi or Pacman
The `pi` that is **Pacman's shortcut** is one of our system's primary most efficient commands. It is a powerful tool at the center of the system, that allows you to maintain, expand, and update the system.

Since you installed the operating system, `pi command` has always been installed, but We can also install `pacman or pi packages` on the system with `pix`. Here is the command:
```
 pix i pi
```

----
## Pix (Pi Extra)
As you can see that in KOOMPI OS, pi is our primary system's primary command. However, Our team has worked on another influence command that can help users a lot more than pi is Pix. 

Pix is the pacman extra which we consider it as the extra command that help improving in installing the applications and packages that pi can not do.

To install it you can run command belows:
```
curl -s https://repo.koompi.org/script/pix.sh -o pix && chmod +x pix && sudo mv pix /usr/bin
```

{{< notice tip >}}
We are recommending you to copy and paste, which makes the command is in correction always.
{{< /notice >}}

---

## Update Operating System
As I have mentioned above that the system always receives the latest packages, as long as you do the updating command.

The **mandatory step** is to upgrade your operating system before downloading any new applications.
```
pi -Syu
```
For Database Updates:
```
pi -Syy
```

----

## Refreshing mirrors
Sometimes it can happen that your system can’t find the updates or packages. In that case, there’s a problem with the mirrors you’re trying to connect to. You can simply refresh the mirrors with this command.
```
 pi -Syyu
```

----
----