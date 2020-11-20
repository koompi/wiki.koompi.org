---
title: "Social Media Apps"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
As the social media platform has been the most powerful application which not only for communication but also for gathering a lot of information around everywhere in the world. Even they have been developed by many different companies but still focus on one main point that makes the users stay connected.

So in this section, we are going to introduce many social platforms and a little details about them.

---
Here is the list chart on the Media apps plus command to install them.

| Social Media Names  | Installing Commands| Details|
|:-------------|:-------------|:-------------|
| **Dingtalk**|      `pi -S dingtalk`|[Click for more]()|
| **Discord**|      `pi -S discord`|[Click for more]()|
| **Kodi**|      `pi -S kodi`|[Click for more]()|
| **Popcorn Time**|      `pi -S popcorntime`|[Click for more]()|
| **Messenger**|      `pi -S caprine`|[Click for more]()|
| **Microsoft Team**|      `pi -S teams`|[Click for more]()|
| **Teamviewer**|      `pi -S teamviewer`|[Click for more]()|
| **Telegram**|      `pi -S telegram-desktop`|[Click for more]()|
| **Zoom**|      `pi -S zoom`|[Click for more]()|

---
---
<!-- ## Dingtalk
**Dingtalk** is an enterprise communication and collaboration platform developed by Alibaba Group. It
was one of the world's largest professional communication and management mobile apps in China with
over 100 million users.

### How to installa Dingtalk
Dingtalk can be installed by command below:
```
pi -S dingtalk
```

---
## Discord
**Discord** is a proprietary freeware VoIP application and digital distribution platform designed for
video gaming communities, that specializes in text, image, video and audio communication between
users in a chat channel.

### How to install Discord
To install this app you have to run it on terminal and typing this:
```
 pi -S discord
```

---
## Kodi
**Kodi** is one of the best free and open source media server software available in the market. It offers
an intuitive graphical user interface with lots of customization options. 

Kodi is an all in one entertainment software center which supports all the primary OS including Android.

### How to install Kodi
Kodi the most entertaining app which can be installed by a command.
```
 pi -S kodi
```

---
## Popcorn Time
**Popcorn Time** is a multi-platform, free software BitTorrent client that includes an integrated media
player. Popcorn Time provide a free "alternative" to subscription-based video streaming services such as
Netflix.

### How to install the Popcorn Time

As the popcorn is the alternative app and free to use as netflix or hulu. we can install them with 2 main choice:

{{% tabs %}}
  {{% tab "First Choice" %}}
   ```
   pi -S popcorntime-bin
   ```
  {{% /tab %}}

  {{% tab "Second Choice" %}}
  ```
  pi -S popcorntime
  ```
  {{% /tab %}}

{{% tabs %}}

---
## Messenger
**Messenger** is a messaging app and platform developed by Facebook, Inc.The Users can send
messages and exchange photos, videos, stickers, audio, and files, as well as react to other users'
messages and interact with bots. The service also supports voice and video calling. 

The standalone apps support using multiple accounts, conversations with optional end-to-end encryption, and playing games
through this app.

### How to install Messenger
You can install messenger on the desktop by run this command:
```
 pi -S caprine
```

----
## Microsoft Team
Microsoft Team is one of the most comprehensive collaboration tools for seamless work and team management. Microsoft Teams has been designed to address a wide range of collaboration and communication issues faced by companies around the world. When it comes to teamwork, the app serves as a digital hub, allowing you to create a shared workspace. This makes it easier to initiate chats, share files, hold meetings, and perform other functions in real time.

### How to install Microsoft Team
We can install it by command below:
```
pi -S teams
```
---
## Teamviewer
**Team Viewer** is an awesome application for remotely accessing a computer or letting someone
remotely access your computer. It is easy to use and its completely free of charge.

### How to install teamviewer
To install it using this command:
```
pi -S teamviewer
```

### How to reset teamviewer
In order to restart your teamviewer, you can simply run this command:
```
sudo systemctl enable teamviewerd
```
You can start your teamviewer service with command line also:
```
sudo systemctl start teamviewerd
``` -->

---
<!-- ## Skype
**Skype** is a telecommunications application that specializes in providing video chat and voice calls
between computers, tablets, mobile devices, the Xbox One console, and smartwatches via the Internet.
Skype also provides instant messaging services. Users may transmit text, video, audio and images.
You can install app by the following steps below: -->


---
## Telegram
**Telegram** is a cloud-based cross-platform instant messaging service with optional end-to-end
encryption. Account creation requires a phone number.

The official clients are open-source but the code for recent versions is not always immediately
published. The server-side code is proprietary.

### How to install Telegram
You can use one of following methods in order to use Telegram:

{{< notice tip >}}
We are recommending these two commands for installing:
{{< /notice >}}
The main one
```
pi -S telegram-desktop
```
The other one
```
pi -S telegram-desktop-bin
```

---
## Wechat
**WeChat** is more than a messaging and social media app – it is a lifestyle for over one billion users
across the world. It is free and safe downloaded.

### How to install Wechat
You can install it on our OS by 2 ways.
**The first way:**
Open terminal and run this command:
````
pi -S electronic-wechat-bin`
````

{{< notice tip >}}
We recommended you to use **the second way**. It is easier and is not facing alot of issues.
{{< /notice >}}
**The Second way:**
```
curl -S http://repo.koompi.org/packages/electronic-wechat-bin-2.3.1-1-x86_64.pkg.tar.xz -O && sudo pacman -U electronic-wechat-bin-2.3.1-1-x86_64.pkg.tar.xz && sudo pacman -S nss gtk3 libxss
```
{{< notice tip >}}
As the command is too long, We recommend you copy and  paste the command. 
For copying command, select command and `Ctrl+c`.
For pasting command in konsole , `Ctrl+Shift+V`.
{{< /notice >}}

---
## Zoom 
[Zoom](https://zoom.us/) is a web-based video conferencing tool with a local, desktop client and a
mobile app that allows users to meet online, with or without video. Zoom users can choose to record
sessions, collaborate on projects, and share or annotate on one another's screens, all with one easy-touse platform.

### How to install Zoom
You can install zoom by run the command below:
```
pi -S zoom
```

### Solution for Error of Zoom Installation
If you are facing error of installing zooom please run thic command below or [Click
here](https://www.koompi.org/faqs/#zoom-issue-and-can-not-run).
```
pi -S --noconfirm zoom
```


---