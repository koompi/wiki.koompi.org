---
title: "Cache Info"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---

**Pi** stores its downloaded packages in `/var/cache/pacman/pkg/` and does not remove the old or uninstalled versions automatically. This has some advantages:

1. It allows to `downgrade a package` without the need to retrieve the previous version through other means, such as the `Arch Linux Archive.`
2. A package that has been uninstalled can easily be reinstalled directly from the cache folder, not requiring a new download from the repository.

However, it is necessary to `deliberately clean up the cache` periodically to prevent the folder to grow indefinitely in size.

The paccache script, provided within the `pacman-contrib` package, deletes all cached versions of installed and uninstalled packages, except for the most recent 3, by default:
```
paccache -r
```
**Enable and start** `paccache.timer` to discard unused packages weekly.

{{< notice tip >}}
You can create a `hook` to run this automatically after every pi transaction.
{{< /notice >}}

For more **options** aboout `paccache` use :
```
paccache -h
```
**Pacman** also has some built-in options to clean the cache and the leftover database files from repositories which are no longer listed in the configuration file `/etc/pacman.conf`. 

However, *Pacman* does not offer the possibility to keep a number of past versions and is, therefore, more aggressive than paccache default options.

To remove all the `cached packages` that are not currently installed, and the `unused sync database`, execute:
```
pi -Sc
```

To remove all files from the cache, use the clean switch twice, this is the most aggressive approach and will leave nothing in the cache folder:
```
pi -Scc
```

{{< notice warning >}}
One should avoid deleting from the cache all past versions of installed packages and all uninstalled packages unless one desperately needs to free some disk space. This will prevent downgrading or reinstalling packages without downloading them again.
{{< /notice >}}

{{< notice tip >}}
Automatically clean the package cache.
{{< /notice >}}


If you are too lazy to clean the package cache manually, you can automate this task using Pacman hooks. The Pacman hook will automatically clean the package cache after every Pacman transaction.


To do so, create a file /etc/pacman.d/hooks/clean_package_cache.hook:
```
sudo mkdir /etc/pacman.d/hooks
```
Next:
```
sudo nano /etc/pacman.d/hooks/clean_package_cache.hook
```
Add the following lines:
```
[Trigger]
Operation = Upgrade
Operation = Install
Operation = Remove
Type = Package
Target = *
[Action]
Description = Cleaning pacman cache...
When = PostTransaction
Exec = /usr/bin/paccache -r
```
Save and close the file. From now on, the package cache will be cleaned automatically after every Pacman transactions (like an upgrade, install, remove). You don’t have to run paccache command manually every time.

----
----
