## Development
Application development goes through a process of planning, creating, testing, and deploying an information system, also known as the software development life cycle. Applications are also often developed to automate some type of internal business process or processes, build a product to address common business challenges or to drive innovation.

### Android Studio
[Android Studio](https://developer.android.com/) is the official integrated development environment for Google's Android operating 
system, built on JetBrains' IntelliJ IDEA software and designed specifically for Android development. It is available for download on Windows, macOS, Linux based and even KOOMPI OS.

**Install**
```
pi -S android-studio
```
**Features**
- Navigation Editor
- New Project Wizard
- Single Variant Build Sync
- Dummy Data in Layout Editor

**Pros**
- Supports Gradle instead of Maven.
- Debug mode is excellent.
- Android Studio is not the unique product for Android development. IntelliJ IDEA is good for it too.
- It is open source software.
- No need to pay, free of charge product.
- It supports big community of other android developers.
- Provide Java to Kotlin code auto translation.

**Cons**
- Cold start. We need some time to open Android Studio.
- It requires a lot of memory for reliable work and powerful processor, for example in case of work with layout editor...
- First build could take a minutes, second build of the same application will be fast.
<!--
### Apache
-->
### Atom
[Atom](#) is a free and open-text editor that is user-friendly, customizable, and mobile. It is a desktop application built on HTML, JavaScript, CSS, and Node.js, and operates using the Electron framework.

Atom boasts of simplicity, flexibility and effectiveness that appeal to many developers, expert and novice alike.It is also simple that even those with minimal coding experience can easily learn the system almost instantly.

**Install**
```
pi -S atom
```

**Features**
- Crossing platform editing
- Built-in package manager
- Smart autocompletion
- File system browser
- Multiple panels
- Find and replace tool
- Packages and them

**Pros**
- Atom has an amazing plugin library that is easy to use and integrates seamlessly.
- Specifically, Atom's multi-cursor plug-in functionality provides a powerful method for manipulating text in bulk.
- Atom provides good syntax highlighting and other interactive support for a number of programming languages through its available plug-ins.

**Cons**
- Some of Atom's default UX could be improved. Depending on your previous workspace, it can open with two panes and a welcome tab in each, requiring you to close lots of cruft on startup.
- Atom's documentation and plugin marketplace could be a little more discoverable.

### Codeblocks
[CodeBlocks](http://www.codeblocks.org/) is an open-source, cross-platform (Windows, Linux, MacOS), and free C/C++ IDE . It supports many compilers, such as GNU GCC (MinGW and Cygwin) and MS Visual C++. It supports interactive debugging (via GNU GDB or MS CDB).

**Install**
```
pi -S codeblocks
```

**Features**
- Syntax highlighting, customizable and extensible.
- Code folding for C, C++, Fortran, XML and many more files.
- Tabbed interface.
- Code completion.
- Class Browser.
- Smart indent.
- One-key swap between . h and . c/. cpp files.
- Open files list for quick switching between files (optional)


### Cordova
[Cordova](https://cordova.apache.org/) is an open-source mobile development framework. It allows you to use standard web technologies such as HTML5, CSS3, and JavaScript for cross-platform development, avoiding each mobile platforms' native development language.

**Install**
```
pi -S cordova
```

**Features**
- Command Line Interface, a tool can be used for starting projects, building processes for different platforms, installing plugins and lot of other useful things that make the development process easier. 
- Cordova Core Components, a set of core components that every mobile application needs.
- It offers API that will be used for implementing native mobile functions to our JavaScript app.

**Pros**
- Cordova offers one platform for building hybrid mobile apps so we can develop one app that will be used on different mobile platforms – IOS, Android, Windows Phone, Amazon-fireos, blackberry, Firefox OS, Ubuntu and tizien.
- It is faster to develop hybrid app then native app so Cordova can save on the development time.
- Since we are using JavaScript when working with Cordova, we don't need to learn platform specific programming languages.
- There are many community add-ons that can be used with Cordova, these have several libraries and frameworks, which are optimized for working with it.

**Cons**
- Hybrid apps are slower than native ones so it is not optimal to use Cordova for large apps that require lots of data and functionality.
- Cross browser compatibility can create lots of issues. Most of the time we are building apps for different platforms so the testing and optimizing can be time consuming since we need to cover large number of devices and operating systems.
- Some plugins have compatibility issues with different devices and platforms. There are also some native APIs that are not yet supported by Cordova.

### Flutter
[Flutter](https://flutter.dev) is a free mobile app development software that is best suited for hybrid apps. It is one of the newest members in the mobile app development space and is written in C, C++, and Skia Graphics Engine. 

It is Google’s UI toolkit that enables you to create applications for mobile, web, and desktop from a single codebase. What’s more, you don’t have to restart the application when testing your project. It offers the Hot Reload functionality, which makes the whole process of development stress-free and optimized.


**Install**
```
pix -i flutter 
```

**Features**

- Incorporate all critical platform differences such as navigation, scrolling, icons, and fonts.
- Offers fully-customizable widgets to render fast development of native apps.
- Creates plugins using channels to be easily used by every developer.

**Pros**
- Expressive and flexible UI.
- Builds native interfaces in minutes.

**Cons**
- Not-so-rich library collections.

### Github
[Github](https://github.com/) is a code hosting platform for version control and collaboration. It lets you and others work together on projects from anywhere. 
**Install**
```
pi -S git
```

**Features**
- Git is compatible with all the Operating Systems that are being used these days. 
- Git repositories can also access the repositories of other Version Control Systems like SVN, CVK, etc.
- Git allows users from all over the world to perform operations on a project remotely.
- Git allows its users to work on a line that runs parallel to the main project files.
- Git stores all the data from the central repository on to the local repository while cloning is done. 
- Git is a free and open-source distributed version control system designed to handle everything from small to very large projects with speed and efficiency.
- Git keeps a record of all the commits done by each of the collaborators on the local copy of the developer.

**Pros**
- Git's branch management model is wonderful and simple, especially when compared to some other VCS tools I've used.
- Git has a very small footprint and works seamlessly on all major platforms.
- Easy to pull/push code.

**Cons**
- It still has some confusing merge issues.
- Git Repository configuration is done at the host level which makes it difficult to enforce certain requirements, like with Git hooks for example.

### Konsole
[Konsole](https://konsole.kde.org/)is the default terminal emulator of the KDE Desktop environment. It runs a command shell, an application that executes the commands you type in.

**Install**
```
pi -S konsole
```

**Features**
- X Terminal emulator.
- Makes using the command line easier.
- Use different profiles for different tasks.
- Set scrollback limit and save contents to file.
- Right-click on links to open in user's default web browser.
- Open current path in user's file manager.

### PhoneGap
[PhoneGap]() is a popular open source mobile app development software that allows you to create hybrid applications. With PhoneGap, the developer doesn’t necessarily need to know mobile programming languages. Instead, they can get started with languages like CSS, HTML, and JavaScript for app creation.

In fact, it allows you to create apps that will work for multiple platforms with a single codebase to reach the maximum number of audience. 

**Install**
```
pi -S phonegap
```

**Features**

- Easily integrates with various libraries for developing the app. 
- Allows you to view and manage any changes in the app using PhoneGap.
- Operates on multiple operating systems, including iPhone, Android, and Windows. 

**Pros**

- A single code base for multiple platforms
- Rapid testing and deployment

**Cons**
- Lack of UI widgets

### QT
[Qt’s](https://www.qt.io) cross-platform framework allows you to design, develop, and deploy mobile applications cost-effectively for different types of portable, handheld iOS, Android, and Windows devices. The free and open-source platform provides a comprehensive and conducive development environment for delivering excellent user experiences. The time-honored and stable solution was initially used for developing software for Windows and Mac.

<!--![Image of Yaktocat](https://octodex.github.com/images/yaktocat.png)
![Image of Yaktocat](https://github.com/koompi/wiki.koompi.org/blob/Sub-Wiki-koompi-org/Images/Fluter1000.png)-->

**Install**
```
pi -S qt
```

**Features**
- Support for opaque private keys
- Supports dual-mode networks and IPv6
- Buggy SSL server workarounds

**Pros**

- A large community of experienced developers and years of available documentation
- Compiler and parser optimization

**Cons**
- Increased process complexity owing to the meta-object compiler

### RStudio
[RStudio](https://rstudio.com/) is an integrated development environment for R, a programming
language for statistical computing and graphics. It is available in two formats: RStudio Desktop is a
regular desktop application while RStudio Server runs on a remote server and allows accessing RStudio
using a web browser.

**Install**
```
pi -S rstudio-desktop-bin
```

**Features**
- Syntax highlighting, code completion, and smart indentation.
- Execute R code directly from the source editor.
- Quickly jump to function definitions.

**Pros**
- It has software project management
- It has software package development
- It has report publishing

**Cons**

- Multiple versions of R can be confusing to maneuver
- Quick view of library locations relevant to the R version in use would be a good resource and reduce confusion
- Better online publication options for quick release, small apps by students

### Visual Studio Code
[Visual Studio Code](https://code.visualstudio.com/) is a source-code editor developed by Microsoft which includes supporting for debugging, embeding Git control and GitHub, syntax highlighting, intelligent code completion, snippets,and code refactoring. It is highly customizable, allowing users to change the theme, keyboard shortcuts, preferences, and install extensions that add additional functionality.

**Install**
```
pi -S visual-studio-code-bin
```

**Features**
- Combines UI of a modern editor with code assistance and navigation
- Integrated debugging experience

**Pros**
- Powerful multilanguage IDE
- Front-end develop out of the box
- Support TypeScript IntelliSense
- Git integration
- Intellisense
- Faster than Atom
- Better ui, easy plugins, and nice git integration

**Cons**
- none