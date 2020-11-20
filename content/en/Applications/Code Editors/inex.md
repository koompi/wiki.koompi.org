---
title: "Code Editing Apps"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
Code editors are a small piece of software like a text editor. The only difference is code editors have special functionalities than text editors.
 
According to the language used to code, the code editors highlight special keywords, give suggestions to some extent, automatic indentation features, and sometimes it has an integrated terminal as well.

---
This document providesa completed details of the list of available coding apps that could be install in KOOMPI OS.

This chart shows the result of which code editors tcommands:

| Code Editor Names  |  Installing Commands |  Details|
|:-------------|:-------------|:-------------|
| **Android Studio** | `pi -S android-stud`     |[Click for more]()|
|**Atom**  |  `pi -S atom`    |[Click for more]()|
|**RStudio**  |  `pi -S rstudio-desktop-bin`    |[Click for more]()|
|**TeXmaker**  | `pi -S texmaker`     |[Click for more]()|
|**Mousepad**  |   `pi -S mousepad`   |[Click for more]()|
|**Visual Studio Code**  |  `pi -S code`    |[Click for more]()|


 ---
 <!--
# Android Studio
Android Studio is the official integrated development environment for Google's Android operating 
system, built on JetBrains' IntelliJ IDEA software and designed specifically for Android development. It is available for download on Windows, macOS, Linux based and even KOOMPI OS.

## How to install Android Studio
{{< notice note >}}
Before installing any apps, please make sure your system have already updated.
{{< /notice >}}

If you don't know anything related to update the system, you can visit [here](#). You have already updated your system, you can easily install the andriod studio within command.
```
pi -S android-stud
```
## Features
- Navigation Editor
- New Project Wizard
- Single Variant Build Sync
- Dummy Data in Layout Editor

---

# Atom

[**Atom**](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=1&cad=rja&uact=8&ved=2ahUKEwjpsZnQo9boAhUlNKYKHV4lDJsQFjAAegQIFxAC&url=https%3A%2F%2Fatom.io%2F&usg=AOvVaw2f1xmShfpKULMHZBtecYLV) is a free and open-text editor that is user-friendly, customizable, and mobile. It is a desktop application built on HTML, JavaScript, CSS, and Node.js, and operates using the Electron framework.

Atom boasts of simplicity, flexibility and effectiveness that appeal to many developers, expert and novice alike.It is also simple that even those with minimal coding experience can easily learn the system almost instantly.

## How to install Atom
In order to install **atom**, you need to go to *konsole* and input command below:

```
pi -S atom
```
 
{{< notice info >}}
After you type the command in konsole, please follow all instructions that have been given!
{{< /notice >}}
You can also start the atom by command,too.
```
atom
```
## Main Features
- Crossing platform editing
- Built-in package manager
- Smart autocompletion
- File system browser
- Multiple panels
- Find and replace tool
- Packages and them

---
## RStudio
[RStudio](https://rstudio.com/) is an integrated development environment for R, a programming
language for statistical computing and graphics. It is available in two formats: RStudio Desktop is a
regular desktop application while RStudio Server runs on a remote server and allows accessing RStudio
using a web browser.

### How to install RStudio
You can install them by go to terminal, running this command:
````
pi -S rstudio-desktop-bin
````

---
# TeXmaker
In the arena of `Open-Source`, **LaTeX** is considered as a standard markup language. It helps the users for **editing the documents** to the markup level. TeXmaker is one of the best LaTeX editors available out here. It’s the most user-friendly LaTeX IDE for the newbie.

## How to install TeXmaker
LaTex can be downloaded through a command:
```
pi -S texmak
```
{{< notice note >}}
If you want to install it, you can use this command **pi -Rdd texmak**.
{{< /notice >}}

---

# Mousepad

Nothing can beat command-line text editors like – nano, vim ,etc. In terms of being lightweight, but if you want a graphical interface, here it is — The **Mousepad** is a minimal text editor. 

It’s extremely lightweight and blazing fast. It comes with a simple customizable user interface with multiple themes.

## How to install Mousepad

It can be installed through `pi` or` pacman`:
{{% tabs %}}
  {{% tab "By Pi" %}}
   ```
   pi -S mousepad
   ```
  {{% /tab %}}

  {{% tab "By Pacman" %}}
  ```
  pacman -S mousepad
  ```
  {{% /tab %}}

{{% tabs %}}

{{< notice info >}}
Mousepad supports syntax highlighting. So, you can also use it as a basic code editor.
{{< /notice >}}

---

# VsCode( Visual Studio Code )

**Visual Studio Code** is a source-code editor developed by Microsoft which includes supporting for debugging, embeding Git control and GitHub, syntax highlighting, intelligent code completion, snippets,and code refactoring. It is highly customizable, allowing users to change the theme, keyboard shortcuts, preferences, and install extensions that add additional functionality.


## How to instakk VsCode
{{< notice tip >}}
Before you install any apps, make sure your system have been updated.If you have not updated, please run this command **pi -Syu** first.
{{< /notice >}}

```
pi -S code --classic
``` 
-->
---
---
