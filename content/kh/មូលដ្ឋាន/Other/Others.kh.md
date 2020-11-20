---
title: "ព័ត៌មានពី ផ្នែកផ្សេងៗ"
date: 2017-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: true
# search related keywords
keywords: ["induct", "instate"]
---
## Coming soon!!
<!-- # Removing Unused Packages (orphans)
For recursively removing orphans and their configuration files:
```shell
     pi -Rns $ (pi -Qtdq)
```
If no orphans were found pacman outputs `error: no targets specified`. This is expected as no arguments were passed to `pi -Rns`.


# Backup The Pi Database
The following command can be used to back up the local pi database:
```
     tar -cjf pacman_database.tar.bz2 /var/lib/pacman/local
```
> **Noted**: If the **pi** database files are corrupted, and there is no backup file available, there exists some hope of rebuilding the pi database.
{.is-info}

# Reinstalling all packages
To reinstall all native packages, use:
```shell
 pi -Qqn | pi -S -
```








# Querying Package Database
```shell
    $ pi -Q      #queries the local package database
    $ pi -s      #sync database
    $ pi -F      #files database
```
For more options about querying:
```shell
    $ pi -Q --help
```
Options:
```shell
  -b, --dbpath <path>               # set an alternate database location
  -c, --changelog                   # view the changelog of a package
  -d, --deps                        # list packages installed as dependencies [filter]
  -e, --explicit                    # list packages explicitly installed [filter]
  -g, --groups                      # view all members of a package group
  -i, --info                        # view package information (-ii for backup files)
  -k, --check                       # check that package files exist (-kk for file properties)
  -l, --list                        # list the files owned by the queried package
  -m, --foreign                     # list installed packages not found in sync db(s) [filter]
  -n, --native                      # list installed packages only found in sync db(s) [filter]
  -o, --owns <file>                 # query the package that owns <file>
  -p, --file <package>              # query a package file instead of the database
  -q, --quiet                       # show less information for query and search
  -r, --root <path>                 # set an alternate installation root
  -s, --search <regex>              # search locally-installed packages for matching strings
  -t, --unrequired                  # list packages not (optionally) required by any package (-tt to ignore optdepends) [filter]
  -u, --upgrades                    # list outdated packages [filter]
  -v, --verbose                     # be verbose
      --arch <arch>                 # set an alternate architecture
      --cachedir <dir>              # set an alternate package cache location
      --color <when>                # colorize the output
      --config <path>               # set an alternate configuration file
      --confirm                     # always ask for confirmation
      --debug                       # display debug messages
      --disable-download-timeout    # use relaxed timeouts for download
      --gpgdir <path>               # set an alternate home directory for GnuPG
      --hookdir <dir>               # set an alternate hook location
      --logfile <path>              # set an alternate log file
      --noconfirm                   # do not ask for any confirmation
      --sysroot                     # operate on a mounted guest system (root-only)
```
>  **Warning**: Sometimes, -s's builtin ERE (Extended Regular Expressions) can cause a lot of unwanted results, so it has to be limited to match the package name only; not the description nor any other field.
{.is-warning}
# Searching Packages
**Pi** can also use for the search for packages in the database `(package_name and descriptions)`:
```shell
 pi -Ss String
```
**Example :**
```
pi -Ss '^vim-'
```
To Search for installed package:
```
pi -Qs String1 String2 ...
```
To display extensive information about the given packages:
```
 pi -F String1 String2
```
> **Tips**: Passing two `-i` flags will also display the list of backup files and their modification states.
{.is-success}

```
    $ pi -Qii package_name
```
To verify the presence of the files installed by a package:
```
    $ pi -Qk package_name
```

> **Tips**: Passing the `k` flag twice will perform a more thorough check.
{.is-success}



# Other Operations

**Operations:**

```shell
    $ pi {-h --help}
    $ pi {-V --version}
    $ pi {-D --database}    # <options> <package(s)>
    $ pi {-F --files}       # [options] [package(s)]
    $ pi {-Q --query}       # [options] [package(s)]
    $ pi {-R --remove}      # [options] <package(s)>
    $ pi {-S --sync}        # [options] [package(s)]
    $ pi {-T --deptest}     # [options] [package(s)]
    $ pi {-U --upgrade}     # [options] <file(s)>
```

New operations:

```shell
    $ pi {-Y --pi}          # [options] [package(s)]
    $ pi {-P --show}        # [options]
    $ pi {-G --getpkgbuild} # [package(s)]
```

New options:
```shell 
    --repo                  # Assume targets are from the repositories
    -a --aur                # Assume targets are from the AUR
```

Permanent configuration options:

```shell
    --save                 # Causes the following options to be saved back to the config file when used
    --aururl      <url>    # Set an alternative AUR URL  
    --builddir    <dir>    # Directory used to download and run PKGBUILDS
    --editor      <file>   # Editor to use when editing PKGBUILDs
    --editorflags <flags>  # Pass arguments to editor
    --makepkg     <file>   # makepkg command to use  
    --mflags      <flags>  # Pass arguments to makepkg
    --pacman      <file>   # pacman command to use
    --tar         <file>   # bsdtar command to use
    --git         <file>   # Git command to use  
    --gitflags    <flags>  # Pass arguments to git
    --gpg         <file>   # gpg command to use  
    --gpgflags    <flags>  # Pass arguments to gpg
    --config      <file>   # pacman.conf file to use
    --makepkgconf <file>   # makepkg.conf file to use
    --nomakepkgconf        # Use the default makepkg.conf

    --requestsplitn <n>    # Max amount of packages to query per AUR request
    --completioninterval   # <n> Time in days to to refresh completion cache
    --sortby    <field>    # Sort AUR results by a specific field during search
    --answerclean   <a>    # Set a predetermined answer for the clean build menu
    --answerdiff    <a>    # Set a predetermined answer for the diff menu
    --answeredit    <a>    # Set a predetermined answer for the edit pkgbuild menu
    --answerupgrade <a>    # Set a predetermined answer for the upgrade menu
    --noanswerclean        # Unset the answer for the clean build menu
    --noanswerdiff         # Unset the answer for the edit diff menu
    --noansweredit         # Unset the answer for the edit pkgbuild menu
    --noanswerupgrade      # Unset the answer for the upgrade menu
    --cleanmenu            # Give the option to clean build PKGBUILDS
    --diffmenu             # Give the option to show diffs for build files
    --editmenu             # Give the option to edit/view PKGBUILDS
    --upgrademenu          # Show a detailed list of updates with the option to skip any
    --nocleanmenu          # Don't clean build PKGBUILDS
    --nodiffmenu           # Don't show diffs for build files
    --noeditmenu           # Don't edit/view PKGBUILDS
    --noupgrademenu        # Don't show the upgrade menu
    --askremovemake        # Ask to remove makedepends after install
    --removemake           # Remove makedepends after install
    --noremovemake         # Don't remove makedepends after install

    --cleanafter           # Remove package sources after successful install
    --nocleanafter         # Do not remove package sources after successful build
    --bottomup             # Shows AUR's packages first and then repository's
    --topdown              # Shows repository's packages first and then AUR's

    --devel                # Check development packages during sysupgrade
    --nodevel              # Do not check development packages
    --gitclone             # Use git clone for PKGBUILD retrieval
    --nogitclone           # Never use git clone for PKGBUILD retrieval
    --rebuild              # Always build target packages
    --rebuildall           # Always build all AUR packages
    --norebuild            # Skip package build if in cache and up to date
    --rebuildtree          # Always build all AUR packages even if installed
    --redownload           # Always download pkgbuilds of targets
    --noredownload         # Skip pkgbuild download if in cache and up to date
    --redownloadall        # Always download pkgbuilds of all AUR packages
    --provides             # Look for matching providers when searching for packages
    --noprovides           # Just look for packages by pkgname
    --pgpfetch             # Automatically resolve conflicts using pacman's ask flag
    --nouseask             # Confirm conflicts manually during the install
    --combinedupgrade      # Refresh then perform the repo and AUR upgrade together
    --nocombinedupgrade    # Perform the repo upgrade and AUR upgrade separately

    --sudoloop             # Loop sudo calls in the background to avoid timeout
    --nosudoloop           # Do not loop sudo calls in the background

    --timeupdate           # Check packages' AUR page for changes during sysupgrade
    --notimeupdate         # Do not check packages' AUR page for changes
```

Show specific options:

```shell
    -c --complete         # Used for completions
    -d --defaultconfig    # Print default pi configuration
    -g --currentconfig    # Print current pi configuration
    -s --stats            # Display system package statistics
    -w --news             # Print arch news
```

Pi specific options:

```
    -c --clean            # Remove unneeded dependencies
       --gendb            # Generates development package DB used for updating
```

get pkgbuild specific options:

```
    -f --force            # Force download for existing tar packages
```
*Contributed by @LyhourChhen*
 -->
