---
title: "Applications"
date: 2018-12-27T11:02:05+06:00
icon: "ti-package"
description: "Providing the guide of applications installation; documents, images, and videos."
type : "docs"
---
 The following steps that connected with applicantions are stored here to help you. If you don’t know konsole at all, we strongly suggest you learn more about it by following this great documentation for beginners in the Basic Command section.

{{% notice note %}}
 Before, You install any applications, you have to make sure your system is already up to date. If you haven't done it, you must do it.
{{% /notice %}}

In order to update your system, you have to run this command below in the konsole:
```
pi -Syu
```

In order to install the applications in KOOMPI OS, You must know the keyword's names of each apps. And here is the form of the command for installing the applications:
```
pi -S Package_Name
```
If you wish to remove any applications from your PC, you will need to use this command:
```
 pi -Rn Package_Name
```
For automatic command to accept all the requiring requests have been given:
```
yes | Command to install or remove
```
Down here are the samples for you:

{{% tabs %}}
  {{% tab "Install Sample" %}}
   ```
   yes | pi -S brave
   ```
  {{% /tab %}}

  {{% tab "Remove Sample" %}}
  ```
  yes | pi -Rn brave
  ```
  {{% /tab %}}

{{% tabs %}}

{{% notice info %}}
This auto command can do on both install and remove command.
{{% /notice %}}

{{% notice note %}}
This auto command is working on only some apps.
{{% /notice %}}

---
