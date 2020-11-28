---
title: "Root Info"
date: 2019-09-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
**root** is the user name or account that by default has access to all commands and files on a Linux or other Unix-like operating system with full privileges. Root access is often necessary for performing commands in the system, especially commands that affect system files because root is so powerful, it's recommended to only request root access when necessary, as opposed to logging in as the root user. This can help prevent accidental damage to important system files.

---
---

### Root Access through Terminal(Konsole)
If you attemp to log you in as **"super user."** You can actually use this command to log in as any user on the machine.
```
su -
``` 
{{< notice note >}}
It will be required your password for root. Please make sure to enter your password correctly because while you were typing in it was hidden.
{{< /notice >}}

When you are logged in as **root**, the command prompt should end these symbols:
- `$` --- means you are not in root.
- `#` --- means you are in root
Here is the sample:
**Not In Root**
```
koompi@koompi-os$~ 
```
**In Root**
```
root@koompi-os ~ #  
```
### Logout Of Root
You can **Logout of Root** in two ways:
**+ First Way, Type:**
```
exit
```
**+ Second Way,Press:**
```
logout
```
### Set or reset Password Root
If you've forgotten the `root password` and your user password, you'll need to boot into **recovery mode** in order to change them. If you know your user password and need to change the root password, just type sudo passwd root, enter your user password, then create a new root password. 
```
sudo password root
```
### Lock And Unlock Root
If you want to lock the root account, enter the following command to remove the password and lock root: 
```
sudo passwd -dl root
```
ប្រសិនបើអ្នកចង់ដោះសោរគណនី root សូមបញ្ចូលពាក្យបញ្ជាខាងក្រោមដើម្បីកំណត់លេខសំងាត់ថ្មីហើយដោះសោរវា៖
```
sudo passwd root
```
{{< notice note >}}
អ្នកនឹងត្រូវបានសួរឱ្យបង្កើតពាក្យសម្ងាត់ថ្មីហើយបញ្ចូលវាពីរដង។ នៅពេលដែលពាក្យសម្ងាត់ត្រូវបានកំណត់គណនី root នឹងដំណើរការ។
{{< /notice >}}

----