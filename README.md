## Resources

[CHIP Documentation](http://chip.jfpossibilities.com/docs/chip.html)

## Installation

1. given a fresh CHIP
2. [connect with serial](http://chip.jfpossibilities.com/docs/chip.html#control-chip-using-a-serial-terminal)
3. [setup wifi](http://chip.jfpossibilities.com/docs/chip.html#connecting-c-h-i-p-to-wi-fi-with-nmcli)
4. ssh-copy-id, and configure ssh client to allow for legacy algorithms (see below)
5. apt update (see below)
6. make install_deps
7. make
8. configure mpd (see below)

### configure local client to allow for legacy algorithms

Add the following to your local .ssh/config

```
Host chip.lan
  PubkeyAcceptedAlgorithms +ssh-rsa
  HostkeyAlgorithms +ssh-rsa
```

### apt update

jessie is no longer supported, but as of this writing, using this snapshot worked

1. update /etc/apt/sources.list with the following

    ```
    deb [trusted=yes] http://snapshot.debian.org/archive/debian-archive/20190328T105444Z/debian/ jessie main contrib non-free
    ```

    Also, I set up some settings to ignore some errors, but I'm not sure if they worked or were needed.

2. run `apt update`


   ignore the gpg error

### configure mpd

1. copy music

    ```
    rsync -avz /path/to/music root@chip.lan:/var/lib/mpd/music                                                                                                                                 ✓  2m 24s  3.3.4   hni ⎈  2.48   00:06:23 
    ```

    Note that Music.app music can be found in ~/Music/Music/Media.localized/Music

2. allow remote connections (if you want to control the player from app or phone)

   update /etc/mpd.config
   update bind_to_address to "any"



