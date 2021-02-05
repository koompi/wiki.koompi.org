On Open-source software most applications are installed through **Aur** that contain app package.So, Here are the details for installing and removing packages details.

#### Install applications commands

you can follow the following commands to install a signal package including the dependencies:
```
 pi -S {package name> or pi -i {package name}
```
For the list of packages installation:
```
 pi -S {package name1} {package name2} ...
```
***<p style="color:red;"> All operations require a password. Then if you aren’t satisfied with the build tool and configuration choices, you can eject at any time. This command will remove the single build dependency from your operation.</p>***
<!--
## Installing packages group
Some packages belong to a group of packages that simultaneously call installation. As in here, for instance.
```
pi -S {package group name}
```
To see what inside the package group, you must run.
```
 pi -Sg {package group name}
```
-->
#### Remove applications commands 
A package is always installed with other packages that it *depends on*, **called dependencies**. Quite often those dependencies are already, or partially installed on your system because other **packages also depend on it**.

If you just want to remove the package, the following command will be enough
```    
 pi -R {package name}
```
If you rather want to avoid a **cluttered system** you can remove a package and its dependencies, without removing dependencies that are used by other installed packages, use the following command.

```
 pi -Rs {package name}
```
To remove a package, its dependencies and all the packages that depend on the target package
```
 pi -Rsc {package name}
```
For deleting a package that other packages need without deleting their dependency package

```
 pi -Rdd {package name} {package name1} {package name2} 
```

#### Update applications commands
As I have mentioned above that the system always receives the latest packages, as long as you do the updating command.

The **mandatory step** is to upgrade your operating system before downloading any new applications.
```
pi -Syu
```
**For Database updating:**
```
pi -Syy
```
**For pix command updating:**
```
pix u
```

#### Automatically command
Here Special usage to automate the install procedure (Recommend):

```
 yes | pi -S {package name}
```
Here is other way for using automatically command:	
```
 pi -S --noconfirm {package name}
```

