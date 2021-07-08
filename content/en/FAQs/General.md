## General Questions

General Question section, the place where all users can find all the answers related to simple questions, or commonly asked by other users. Some of the Faqs here are from the submission of the users to our email os@koompi.com All these questions have been noted down by our team. 

This section will be updated every time there are new questions which are relevant to this section.

---

### What is KOOMPI OS?
KOOMPI OS is the free open-source software, it is free to use as much as everyone like it. KOOMPI OS is fully customized and self-customizable which make it more special. It is based on arclinux which we consider the most suitable operating system for users.

---

### Which application used for designing on KOOMPI OS?

### Which application used for word processing on KOOMPI OS?

### Which application used for installing application on KOOMPI OS?

### How to switch languages?
There are two ways for changing your languages usage.

First, you can change it by clicking on the icon on the top-right corner of your screen that sometimes it's appeared as abbreviation text or flag of the language. 
Second, you can change by using **Button KOOMPI** with **Button Space**.  

---

### How to remove notes(bugs)?
As recently, There is a known bug happened related to the desktop widgets or panels keep reappearing after **removed** or **turn off** your pc. You can fix this problems by closing after you open your pc once again. 

What you need to do next is to install a tool by using [pix sub primary](#) command on the [konsole](#). 
Here is the command to  install the tool.
```
pix -i refresher       
```

**How to use the tool?**

This tool can only run the konsole and here are the commands.

**To refresh panel**

```
refresher panel
```

**To fix the panel problem**

```
refresher widget 
```

**To fix both panels and widget**

```
refresher all
```

**Note:** Please, follow all the instuction have been give. If you don't know what is kind of theme you are using, check **KOOMPI THEME** in your pc. If you want to learn more about pix, [Click here](#).

---

### How can I check the specs of the PC?
You can check your specification of your Pc by opening the **System Settings** and then scroll down to the bottom of the setting. After that you will see the sector named **System Information**, Click on it you will see all the details there.

---

### Can KOOMPI OS run Microsoft Offices?

The answers is **NO**. On KOOMPI OS, we recommend all users to use free open-source applications such as **Libre Office**. However you can save your file as *Microsoft file Types*.

Check here for [Office applications](/en/Applications/Education.md/###_Zoom)!

Check here for [Videos Tutorial](https://sala.koompi.com/organization/5fe4a9bd4500f10685c2b7d0/#home)!

---

### How can I change the screen resolution?
You can change the screen resolution by going into System Setting and then Hardware and then move on to Display and Monitor section. After that Click Display Configuration.

The default resolution for most PCs is 1920x1080 so does KOOMPI. So, set it to that resolution.

---

### I have sound problems, what should I do?

There are two solutions we are recommending for you.

If you are surely having sound problems, Please, Following these instructions:

- Open your terminal and run this command for killing your audio.
```
pulseaudio -k
```

- After that run another command below to make your audio work again.

```
pulseaudio --start
```

If the first solution isn’t working you can try with pavucontroll which is the app that supporting the audio configuration.

- To install it run this command below:

```
pi -S pavucontrol
```

- After you have installed it, you can search and open it for configuring the input and output audio.

**Note: If you try every one of the solutions have been provided and still have problems, please bring it to [our office](http://bit.do/koompi-boran) for fixing.**

---

### How can I change my password on the  PC?

You need to open **System Settings**.
Then you need to find **Users** section.
After that you need to choose the user you are using right now by double click on it.
And finally, you will see the **Change Password** button, double click it, and follow the instruction that have been given.

---

### My zoom is not running, what should I do?

Normally **Zoom** package needs **QT5** packages, it could be caused by it. You can run command below in Konsole.
```
pi -S --noconfirm xz qt5-webengine zoom
```
After you had following the instruction that been have told, and it is still not working. You can drop a message in [our community group](https://t.me/koompicom). 

----

### How can I set my desktop back to normal size?

![Image](/public/Images/Screenzoom.png)

If you are facing  problem like the image has been shown above, you can solve the probelm by:
Press  **KOOMPI Key** on your Keyboard and press **- key** the same time to zoom out. 

----

### What is the default password of KOOMPI OS?
Normally, Our team members always input the **123** as the default password.

**Note:** ***Please check all your langauge keyboard before entering the password. Sometimes it is caused by it so that why the password is incorrect.***

Besides, if you have already make sure with everything, please kindly try **kmp** or **koompi123**.

---

### How to set automatically connect to used Wifi?
You have to follow the following steps provided below:
- **Step 1:** You need to search for **System Settings**.
- **Step 2:** Then you find a section named **connection**.
- **Step 3:** You have to click on it, and it will appear a lot of sections.
- **Step 4:** You have to look for **General Configuration**.
- **Step 5:** And you  have to click check on checkbox named **All users may connect to this network**.

---

### How can I enable touchpad, when it is disable?

You can open your touchpad through your keyboard. First, you need to go into the setting Enter and Click System Settings. And then you will have to find Input Devices, Select Touchpad feature, and then *Enable it* and you will be done.

**Note:** ***Use tab key for changing you are selected. If you are finding it hard to do with the keyboard, we recommend using the external mouse instead.***

---

### How to set my screen not to automatically turning off?

At the top-right corner of your desktop environment on the [Taskbar](/en/KOOMPI_OS/Desktop_environment.md), you can see the **icon looks like monitor**. Click on it and then you will see sectoin **Presentation mode** and **checkbox** for **enable/disable** the automatically funtion. 

![Image](/public/Images/Noneautoscreen1000.png)

---

### How can I set my wanted application to the desktop?

Plese click on this [link for details about it](/en/KOOMPI_OS/Application_management.md).

----

### I cannot connect Wifi, What should I  do? (KOOMPI E11)
For only  KOOMPI E11, there is some problems with ***firmware*** and here is the  soultion for fixing it. You need to run **commands** below on **konsole**
```
pi -R linux-lts && sudo grub-mkconfig -o /boot/grub/grub.cfg && reboot
```
After rebooting, you should be able to connect the wifi. Then you have to open konsole
```
pix i e11-firmware
```

----

### How can I change the password, If I forgot the password? 

Dealing with forget password is not an easy thing !!!

KOOMPI respect *user privacy 100%*, so KOOMPI knows nothing about user password or any data. It is the user’s responsibility to take care of their data and password. However, there is still a way to **set a new password,** but it requires advanced administration skills.

Please bring your laptop to the [KOOMPI office](http://bit.do/koompi-boran).

---

### Can KOOMPI OS run Adobe Lightroom and Adobe Photo Shop?


In the meantime, We can’t run Lightroom in KOOMPI Operating yet, but many other applications have similar features and high quality as it.

For example, GIMP] which is supporting many works such as graphics editor, Image retouching and editing, free-form drawing, converting between different image formats, and more specialized tasks.

For Adobe Lightroom, Our team is trying their best to bring it as much as possible.

Check here for [Graphic Applications](/en/Applications/Graphic.md)!

---


### What should I  do, after I opened my PC and got Black screen?

**Here is sample of text shows on the black screen!**

````
Loading Linux linux...
error: premature end of file /boot/vmkinuz-linux
Loading initial ramdisk ...
error: you need to load the kernel first,

Press any keys to continue...._
````
This happens when your kernel is corrupted. It is caused by 
- Unfinish system update.
- Improper system shutdown.

The solution for this matter,x   please bring it to [KOOMPI office](http://bit.do/koompi-boran) for fixing!

---

