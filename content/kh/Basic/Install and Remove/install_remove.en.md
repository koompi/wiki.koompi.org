---
title: "Install/Remove"
date: 2019-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
On Open-source software most applications are installed through `Aur` that contain app package.So, Here are the details for installing and removing packages details.

----
----
### Installing Packages

Follow the following command to install a signal package including the dependencies:
```
 pi -S <Package name> or pi -i <Package name>
```
For the list of packages installation:
```
 pi -S <Package name1 Package name2 ...>
```
---

### Installing Package Group

Some packages belong to a **group of packages** that simultaneously call installation. As in here, for instance
```
 pi -S <Package group name>
```
To see what inside the package group, run
```
 pi -Sg <Package group name>
```

---

### Removing Packages
A package is always installed with other packages that it *depends on*, `called dependencies`. Quite often those dependencies are already, or partially installed on your system because other **packages also depend on it**.

If you just want to remove the package, the command following will be enough
```    
 pi -R <Package name>
```
If you rather want to avoid a `cluttered system` you can remove a package and its dependencies, without removing dependencies that are used by other installed packages, use the following command.

```
 pi -Rs <Package name>
```
To remove a package, its dependencies and all the packages that depend on the target package
```
 pi -Rsc <Package name>
```
For deleting a package that other packages need without deleting their dependency package

```
 pi -Rdd <Package name>
```

{{< notice note >}}
All operations have required a password. Then if you aren’t satisfied with the build tool and configuration choices, you can eject at any time. This command will remove the single build dependency from your operation.
{{< /notice >}}

{{< notice tip >}}
Here Special usage to automate the install procedure (Recommend):
{{< /notice >}}

```
 yes | pi -S <Package name> 
```
	
{{< notice tip >}}
Install packages with no confirm.
{{< /notice >}}

```
 pi -S --noconfirm <Package name>
```

----