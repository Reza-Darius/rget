# Download archive files with ease!

Have you ever found yourself doing:

```bash
curl -s -L https://example.com/archive.tar.gz | tar xvz - -C /tmp
```

Or:

```bash
wget -qO- your_link_here | gunzip | tar xvf -
```

Even worse:

```bash
tar -xvz -C /tmp/ -f <(wget -q -O - https://github.com/user/repo/release/download/v/v.tar.gz)
```

Downloading and unpacking a tar ball shouldn't be so troublesome! With `rget` you
do this:

```bash
rget https://example.com/archive.tar.gz
```

And that's it, `cd` into `archive` and you're ready to go!
It downloads the file and picks the appropriate unpack tool depending on the file
extension. Supports tar, tar.gz and zip!

## Install

Requires `cargo`, and is only tested on Linux!

```bash
git clone https://github.com/Reza-Darius/rget 
just install
```

