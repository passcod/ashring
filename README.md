# Ashring

___NOT FOR CRITICAL USE___ This started as a toy project and grew a bit. I do
not know as much about cryptography as I should. There are probably glaring
flaws in the design and/or implementation that I have completely overlooked.
At this stage, and until it has been checked by security folks, Ashring may: be
succeptible to obvious attacks, not keep your data safe, eat your data,
self-destruct, trash your laundry, steal your lunch money, and otherwise be
unreliable for serious use. You have been warned.

Ashring was previously designed as a direct competitor to EncFS/TrueCrypt.
See the README of this repo around [a1ea9d96] for more details.

[a1ea9d96]: https://github.com/passcod/ashring/blob/a1ea9d96fa4ee7c25760a28aef763d3ea0b5bbd7/README.md

## Intro

Ashring is a high-level cryptographic system. It can be used to create complex
cryptography-backed solutions and has one simple guarantee: that all security
mechanisms it implements are _enforced by cryptography_.

For example, take a common filesystem with permissions or ACLs: certain files
can be protected from an attacker by making them unreadable or unwritable by,
say, anyone except the root user. However, if an attacker gains hardware access
or some other means to read arbitrary bytes directly off the medium, these
files will be easily accessible and modifiable.

On the other hand, with a filesystem that enforces these permissions using
cryptography, even if an attacker has the hardware, they cannot access the
data without the required key. A more advanced system could allow one key
to read the data, but require a second to modify it. Yet another system,
one that could be used for e.g. logging, could allow appending to files, but
not reading previous data, nor modifying it.

Ashring makes creating these systems a relatively simple matter of "wiring
together" components. The careful choice of appropriate algorithms and their
combination is abstracted away, and sane defaults are set. The capabilities
and limitations of each component are documented. Ashring does not remove the
burden of proper research and careful approach to security and cryptography
that is required of the prosepctive developer, but it does do away with a lot
of the boilerplate and many low-level decisions.

## State

Ashring is an idea-in-progress. Any and all code and lit currently associated
with the project should be subject to extreme prejudice and remain so until
this notice is updated.
