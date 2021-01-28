---
title: "Password Info"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
This section will provide the information related password information.

---
---
### Set User Password
Type the following command to change your own password
```
passwd
```
**Sample Output:**
```
[koompi@koompi-pc ~]$ passwd
Changing password for koompi.
Current password: 
New password: 
Retype new password: 
passwd: password updated successfully
```
The **user** is first prompted for his/her old password if one is present. This password is then encrypted and compared against the stored password. The user has only one chance to enter the correct **password**. 
 
**The superuser** is permitted to bypass this step so that forgotten passwords may be changed. **A new password** is tested for complexity. As a general guideline, passwords should consist of 10 `to` 20 `characters` including one or more** from each of the following sets:
 

1. `Lower` case alphabetic
1. `Upper` case alphabetic
1. Digits `0` to `9`
1. `Punctuation marks`/`special characters`

### Changing Password For Other User Account

You need to [login as the root user](https://koompi.org/KOOMPI%20OS#root-info). To go into the **root**, type the following command to change the password for User_Name:

```
sudo passwd <USER_NAME>
```

**Sample Output:**

```
[koompi@koompi-pc ~]$ sudo passwd koompi
New password: 
Retype new password: 
passwd: password updated successfully
```
Where, **koompi** – is username or account's name.

### Changing Group Password

When the `-g` option is used, the password for the named group is changed. In this example, change the password for the group:

```
    passwd -g Group_Name
```

The current group password is not prompted for. The `-r` option is used with the `-g` option to remove the current password from the named group. This allows group access to all members. The `-R` option is used with the `-g` option to restrict the named group for all users.

### Changing User Passwords

As a KOOMPI OS or Linux system administrator (sysadmin), you can change the password for any users on your server. To change a password on behalf of a user:

1. First sign on or “`su`” or “`sudo`” to the “`root`” account on **KOOMPI OS**, run: `sudo -i`
1. Then type, `passwd Administrator_name` to change a password for Admin user
1. The system will prompt you to enter a password twice

---