# Ashring File Format

An Ashring file is a container for encrypted data which corresponds to a
plaintext file entry. It has a header, containing parsing information for the
rest of the file and cryptographic IVs, and two encrypted payloads: the first
containing metadata about the file, and the second containing the actual data
of the plaintext file.

The encryption used in version 0 is AES-256 in CBC mode. This uses a 16 byte IV
and a 32 byte key. The filename, metadata and data are encrypted separately and
therefore use different IVs. The data payload is optional hence the header may
contain just 2 IVs instead of 3.

All values use big endianness.

## Magic number

An Ashring file always starts with 8 bytes that are used to indicate what kind
of file it is. These bytes are the magic number, `61 73 68 72 69 6e 67`, and a
version number as a u8. For the current version, these 8 bytes are:

    00000000  61 73 68 72 69 6e 67 00                           |ashring.|

## Header

The first 8 bytes contain the size in bytes of the encrypted metadata payload
(as it may be variable) and should be parsed as a u64, e.g.:

    0000000f  00 00 00 00 00 00 00 50                           |.......P|

The next 16 bytes contain the IV for the encrypted metadata payload, e.g.:

    00000010  ee 66 e8 57 d1 db ec 8f  8b 21 b4 9d 04 5e f1 26  |.f.W.....!...^.&|

And again after that, the IV for the encrypted filename, e.g.:

    00000020  72 85 53 bf 69 a6 19 77  b8 e0 9a 13 af 83 97 ab  |r.S.i..w........|

If the next 16 bytes are all NULs, this indicates that there is no data (the IV
for the encrypted data payload will *never* be all NULs). Otherwise, they
contain the IV for the encrypted data payload, e.g.:

    00000030  0e e0 e0 13 1f 00 ea 95  d0 f2 2e d6 96 d0 5b c4  |..............[.|

Finally, the next 4 bytes contain a bit field for flags. For now the only flag
available is `EXTENDED_FILENAME = 00 00 00 01`.

An example magic number + header is shown below as with hexdump:

    00000000  61 73 68 72 69 6e 67 00  00 00 00 00 00 00 00 50  |ashring........P|
    00000010  ee 66 e8 57 d1 db ec 8f  8b 21 b4 9d 04 5e f1 26  |.f.W.....!...^.&|
    00000020  72 85 53 bf 69 a6 19 77  b8 e0 9a 13 af 83 97 ab  |r.S.i..w........|
    00000030  0e e0 e0 13 1f 00 ea 95  d0 f2 2e d6 96 d0 5b c4  |..............[.|
    00000040  00 00 00 00                                       |....|
    00000044

This is always 68 bytes long.

## Metadata

The metadata payload is described here in its plaintext version. That plaintext
is then encrypted according to the method above. The length of the ciphertext
is then computed and that is the value given in the header. The plaintext
metadata is formatted as [msgpack](http://msgpack.org/).


