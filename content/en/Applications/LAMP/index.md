---
title: "Web Stack (LAMP)"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
A LAMP (Linux, Apache, MySQL, PHP) stack is a common web stack used to prepare servers for hosting web content. This guide shows you how to install a LAMP stack an KOOMPI OS.

{{% notice info %}}
  This guide is written for a non-root user. Commands that require elevated privileges are prefixed with sudo. If you’re not familiar with the sudo command, you can check our Users and Groups guide.
{{% /notice %}}

---

In order to install **lamp server**, you need to run the command in terminal:
```
pi -Syu
```
{{% notice note %}}
  Don't forget to update your system before installing this.
{{% /notice %}}


## Apache Install & Configure

In order to install apache, you have to run command below:
```
pi -S apache
```

And then follow the next steps below:

1. First you need to edit `/etc/httpd/conf/httpd.conf` file,
1. sudo nano `/etc/httpd/conf/httpd.conf`
1. Then, searchring and commenting out the following line. If it is not commenting, yet 
1. And after that **save** close files:

Here is the following line, you need to comment:
```
[...]
  #LoadModule unique_id_module modules/mod_unique_id.so
[...]
```
### Enable Apache Service 
To start your service at boot in http, you can run this command:
```
systemctl enable httpd
```
### Restart Apache Service
you can restart your serviceon http by:
```
systemctl restart httpd
```
### Verifying Apache
You can verify whether Apache is running or not with command:
```
systemctl status httpd
```
The output of server that we can't run yet
```
● httpd.service - Apache Web Server
     Loaded: loaded (/usr/lib/systemd/system/httpd.service; disabled; vendor preset: disabled)
     Active: inactive (dead)
```     
Output of sample server that is ready to use:
```
● httpd.service - Apache Web Server
   Loaded: loaded (/usr/lib/systemd/system/httpd.service; disabled; vendor preset: disabled)
   Active: active (running) since Tue 2016-02-16 13:00:18 IST; 7s ago
   Main PID: 1067 (httpd)
   Tasks: 82 (limit: 512)
   CGroup: /system.slice/httpd.service
   ├─1067 /usr/bin/httpd -k start -DFOREGROUND
   ├─1070 /usr/bin/httpd -k start -DFOREGROUND
   ├─1071 /usr/bin/httpd -k start -DFOREGROUND
   └─1072 /usr/bin/httpd -k start -DFOREGROUND

    Feb 16 13:00:18 server systemd[1]: Started Apache Web Server.
    Feb 16 13:00:18 server httpd[1067]: AH00558: httpd: Could not reliably dete...ge
    Hint: Some lines were ellipsized, use -l to show in full.
```
### Test Apache

First, Let create a sample page in the Apache root directory.
For Example: /srv/http.
```
nano /srv/http/index.html
```
Add the following line:
```
<html>
 <title>Welcome</title>
  <body>
   Welcome to OSTechNix test page

</html>
```
Now, open your web browser and navigate to http://localhost or http://IP-address. You will be pleased with Apache server Test page.
### Extent Details

To set Apache to start at boot:
```
sudo systemctl enable httpd.service
```

To make a backup:
```
cp /etc/httpd/conf/extra/httpd-mpm.conf ~/httpd-mpm.conf.backup
```
{{% notice note %}}
Before changing any configuration files, it is advised that you make a backup of the file.
{{% /notice %}}

---
## MariaDB Or MySql

Run the following command to install MariaDB:
```
pi -S mysql
```
After you run the command above, you will be asked whether to install MariaDB or Percona server, just hit enter and then type “Y” and press enter again. The default selection i.e MariaDB will be installed on your KOOMPI OS and the output will be like this,
```
[koompi@koompi-pc ~]$ sudo pacman -S mysql
:: There are 2 providers available for mysql:
:: Repository extra
   1) mariadb
:: Repository community
   2) percona-server
```
You need to initialize the **MariaDB data directory** prior to starting the service. To do so, run:
```
sudo mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
```
Then issue the following command to enable and start **MariaDB service**.
```
systemctl enable mysqld
systemctl start mysqld
```
You can verify whether MariaDb is running or not using command:
```
systemctl status mysqld
```
### Setup MySQL/MariaDB root user password

As you may know, It is recommended to setup a password for database root user.

Run the following command to setup MariaDB root user password:
```
mysql_secure_installation
```
> MariaDB has been installed and ready to use.
PHP

To install PHP in Arch Linux, run:
```
pi -S php php-apache
```
> Then PHP is installed, we need to configure **Apache PHP module**.

To do so, edit `/etc/httpd/conf/httpd.conf` file,
````
sudo nano /etc/httpd/conf/httpd.conf
````
Find the following line and comment it out:
```
[...]
  #LoadModule mpm_event_module modules/mod_mpm_event.so
[...]
```
Then, add the following lines at the bottom, Save and close the file.:
```
[...]
LoadModule mpm_prefork_module modules/mod_mpm_prefork.so
LoadModule php7_module modules/libphp7.so
AddHandler php7-script php
Include conf/extra/php7_module.conf
```
## Testing PHP

Now create a test.php file in the Apache root directory.
```
sudo nano /srv/http/test.php
```
Add the following lines:

```
<?php
 phpinfo();
?>
```
## Restart httpd service.
```
systemctl restart httpd
```
> Open up your web browser and navigate to [http://ip-address/test.php](#)

---