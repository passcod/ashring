# Sherman

___NOT FOR CRITICAL USE___ This started as a toy project and grew a bit. I do
not know as much about cryptography as I should. There are probably glaring
flaws in the design and/or implementation that I have completely overlooked.
At this stage, and until it has been checked by security folks, Sherman may: be
succeptible to obvious attacks, not keep your data safe, eat your data,
self-destruct, trash your laundry, steal your lunch money, and otherwise be
unreliable for serious use. You have been warned.

OpenSSL is used for asymmetric cryptography until rust-crypto gets there.

## Intro

Sherman works as a FUSE filesystem. To create a Sherman _box_, or to access
one, simply use the `sherman`binary with a path and a mountpoint. It will ask
you for the mode (one of "simple", "shared", "cooperative", or "shared
cooperative"), for additional settings if necessary, and for one or more
passphrases depending on the mode. The mountpoint will then be available for
you to use as a folder. To unmount, use `fusermount -u <mountpoint>`.

## Modes

Sherman has four modes, which vary how and how many passphrases are required to
open a box:

### Simple

This is the classical mode of operation for an encrypted thing, and the one
Sherman provides by default. It's really simple, you've done it a thousand
times: to open the box, you need one passphrase. That's it.

But like every secret, you shouldn't give it to someone else. So what if you
want to give access to someone else, without relinquishing your secret
passphrase?

### Shared

In shared mode, you provide as many passphrases as you want, and *any* of them
can open the box. Now everyone you chose to give a passphrase to is able to
open the box, and nobody knows the passphrase of anyone else.

### Cooperative

In cooperative mode, you again provide as many passphrases as you want, but
this time *all* of them *together* are required to open the box. In other
words, if Alice, Bob and Carol each have a passphrase to a cooperative box,
they all need to get together for anyone to open it. In a more individual
situation, you could have a passphrase on a Yubikey, another on your laptop's
disk, and a third one in your mind, and all of these would be required to open
your box.

### Shared cooperative

This is a mix of the shared and cooperative modes: you provide as many
passphrases are you want, but this time you also specify how many of them are
required to open the box. If Alice, Bob and Carol each have a passphrase, and
you specify that two passphrases are required, Alice and Bob, or Bob and Carol,
or Alice and Carol, can open the box without the third member needing to be
there. In a wider situation, say you are in a team of developers. There is a
`3 in N` shared cooperative Sherman box which contains the SSH keys required
to access a server, thus you can only push new code or make a change to
production when two other people support your decision.

## Versions and roadmap

Versions 0 to 4 are planned for, versions 5 onward require more research and
may change or vanish without notice.

### Zero (current)

In version 0, only the *simple* mode is implemented. The encryption used is
AES-256, but it is not authenticated. This version is to set up the necessary
architecture for FUSE-based encrypted shares, the base format of the encrypted
files, and what data is to be encrypted or not.

### One

In version 1, all modes will be implemented and HMAC authentication will be
added to the format.

### Two

In version 2, simple support for different access levels will be added.
Notably, this means that one will be able to have two passphrases or sets of
passphrases (according to the modes described above): one for read-only access,
and one for read-write access. This involves significant changes to the format
and the way encryption is done as assymetric encryption is required.

### Three

In version 3, simple support for multiple boxes within the one Sherman mount
will be added. This means it will be possible to have multiple passphrases (or
sets of passphrases, etc) which each access a *different* set of encrypted
files and folders. Each box will still support the full range of features
(different modes for each box, read-only/read-write accesses, etc).

### Four

In version 4, support for multiple boxes will be expanded to include an
optional *master box*, which will allow control over the creation and
management of boxes. Only by providing passphrase(s) for the master box
would it be possible to create (or delete) new boxes. Access to existing
boxes would not need these passphrases.

### Five

_May change without notice_

Investigate an extended version of access level control (as introduced in
version 2), notably to separate the rights for write, create, delete.

### Six

_May change without notice_

Investigate support for sub-boxes, which would allow parts (subfolders) of
boxes to have different access levels within the same mount.

### Seven

_May change without notice_

Investigate partial origins, which would allow an origin folder (that which
contains the encrypted versions of the files in a Sherman box) to be
partially downloaded from a server or peer and still be functional. Also
investigate hooks that would allow Sherman to request and push files from and
to the remote host as needed by the user.
