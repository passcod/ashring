# Sherman File Format

A Sherman file is a container for encrypted data which corresponds to a
plaintext file entry. It has two headers: one in plaintext, containing parsing
information for the rest of the file (length of second header and data, whether
the filename is extended or not…) and 2 or 3 IVs. The other header is
encrypted, and contains metadata about the file; its schema is more flexible
than the plain header. Finally, the encrypted data is appended.

The encryption used in version 0 is AES-256 in CBC mode. This uses a 16 byte IV
and a 32 byte key. The filename, header and data are encrypted separately and
therefore use different IVs. The data is optional hence the plain header may
contain just 2 IVs instead of the normal 3.

All values use big endianness.

## Magic number

A Sherman file always starts with 8 bytes that are used to indicate what kind
of file it is. These bytes are the magic number, `53 68 65 72 6d 61 6e`, and a
version number as a u8. For the current version, these 8 bytes are:

    00000000  53 68 65 72 6d 61 6e 00                           |Sherman.|

## Plain header

The first 8 bytes contain the size in bytes of the encrypted header (as it can
be variable) and should be parsed as a u64, e.g.:

    0000000f  00 00 00 00 00 00 00 50                           |.......P|

The next 16 bytes contain the IV for the encrypted header, e.g.:

    00000010  ee 66 e8 57 d1 db ec 8f  8b 21 b4 9d 04 5e f1 26  |.f.W.....!...^.&|

And again after that, the IV for the encrypted filename, e.g.:

    00000020  72 85 53 bf 69 a6 19 77  b8 e0 9a 13 af 83 97 ab  |r.S.i..w........|

If the next 16 bytes are all NULs, this indicates that there is no data (the IV
for the encrypted data will *never* be all NULs). Otherwise, they contain the
IV for the encrypted data, e.g.:

    00000030  0e e0 e0 13 1f 00 ea 95  d0 f2 2e d6 96 d0 5b c4  |..............[.|

Finally, the next 4 bytes contain a bit field for flags. For now the only flag
available is `EXTENDED_FILENAME = 00 00 00 01`.

An example magic number + plain header is shown below as with hexdump:

    00000000  53 68 65 72 6d 61 6e 00  00 00 00 00 00 00 00 50  |Sherman........P|
    00000010  ee 66 e8 57 d1 db ec 8f  8b 21 b4 9d 04 5e f1 26  |.f.W.....!...^.&|
    00000020  72 85 53 bf 69 a6 19 77  b8 e0 9a 13 af 83 97 ab  |r.S.i..w........|
    00000030  0e e0 e0 13 1f 00 ea 95  d0 f2 2e d6 96 d0 5b c4  |..............[.|
    00000040  00 00 00 00                                       |....|
    00000044

This is always 68 bytes long.

## Encrypted header

The header is described here in its plaintext version. That plaintext is then
encrypted according to the method above. The length of the ciphertext is then
computed and that is the value given in the plain header. The plaintext header
is formatted as [msgpack](http://msgpack.org/).

