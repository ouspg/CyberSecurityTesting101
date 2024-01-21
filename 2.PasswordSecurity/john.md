# John the Ripper instructions

John the Ripper is a popular and powerful password cracker, check the project [front page](https://www.openwall.com/john/) and
[documentation](https://www.openwall.com/john/doc/) for more complete information.

![](img/john.png)

## Installation

John the Ripper should be straightforward to install.

In Arch Linux in `x86_64`:

```sh
sudo pacman –S john
```

> [!IMPORTANT]
> Currently, you **cannot** run `john` in virtual machine, if your host is M1/M2/M3 MacBook. There isn't proper OpenCL support yet. There isn't ARM64 package either in Arch Linux.
>  Instead, install [homebrew](https://brew.sh/) and then `john` directly to your host system as `brew install john`, for example.

For other platforms, consult the official project page.

## Benchmark

```sh
john --test
```

Get the brute forcing speeds of all available algorithms.

```sh
john --test --format=[algorithm]

```

For jtesting only the cracking speed of a specific algorithm.

To list all available format:

```sh
john --list=formats
```

## Basic usage

```sh
john filename
```

Where “filename” is the name of the file containing the hashes you wish to crack. 
You may need to specify paths to John’s executable and your hash file. With no cracking mode specified, John will first try
single mode, then a wordlist with rules and finally incremental mode.

Viewing cracked passwords:

```sh
john --show filename
```

Or see the pot file “john.pot” which is created in the same file as the executable.

## Modes

### Single mode

```sh
john --single filename
```

John will try to crack the hash with a large word-mangling ruleset and words extracted from login names, directory names and “GECOS” / “full name” fields.

## Incremental mode

```sh
john --incremental filename
```

John tries all combinations, starting with the shortest.
Unless the expected password is trivial, incremental mode
should typically be combined with rules, because otherwise the number of possible answers is massive.
```sh
john --incremental:Alpha --format=Raw-MD5 filename
```
The above example incrementally cracks MD5 hashes only using lower- and upper-case letters.

## External mode

Use your own external program to generate guesses for John. You can read more here.

## Masks

Specify positions for characters, or choice of characters, in the password guess. 
For example, looking for
a password that is five characters long and ends in a number, we could try something like:
```sh
john --mask=’?a?a?a?a?d’ filename
```
Where “a” stands for any characters, and “d” for digit.

## Wordlists

```sh
john --wordlist=password.lst filename
```

Try every word in a list.

## Rules

```sh
john --rules:RuleName --wordlist=password.lst filename
```

Go through a given word list, but with a set of rules defined in `john.conf`. 
The rules can do a variety of things, such as capitalising different letters, replacing them with numbers etc.

## Salts

To crack salted passwords, you need to provide the format of the salts and passwords are stored in your file. 
You can choose from many predefined formats or create one. 

View the predefined dynamic formats with:

```sh
john --list=subformats
```

For example, if the hashes and salts are stored in the file “salted.txt” in lines of {password hash}${salt}
and they are MD5-hashed, the command to start cracking them with John would be:

```sh
john --format=dynamic_1 salted.txt
```

or

```sh
john --form=dynamic=’md5($p.$s)’ salted.txt
```

## Example

With the sample `input.txt`, with three MD5 hashes:
```txt
d0fb963ff976f9c37fc81fe03c21ea7b
ff2364a0be3d20e46cc69efb36afe9a5
22fc7874c4fa844b82a016fb2ecb9c7e
```

We can crack hashes, for example

```sh
» john --incremental --format=Raw-MD5 input.txt
Using default input encoding: UTF-8
Loaded 3 password hashes with no different salts (Raw-MD5 [MD5 128/128 ASIMD 4x2])
Press 'q' or Ctrl-C to abort, almost any other key for status
2001             (?)
space            (?)
2g 0:00:14:14  0.002340g/s 28427Kp/s 28427Kc/s 28429KC/s dmu5ut2..dmu5EXC
Use the "--show --format=Raw-MD5" options to display all of the cracked passwords reliably
Session aborted

» john --incremental:Lower --format=Raw-MD5 input.txt
Using default input encoding: UTF-8
Loaded 3 password hashes with no different salts (Raw-MD5 [MD5 128/128 ASIMD 4x2])
Remaining 1 password hash
Press 'q' or Ctrl-C to abort, almost any other key for status
odyssey          (?)
1g 0:00:00:52 DONE (2024-01-21 13:18) 0.01887g/s 29133Kp/s 29133Kc/s 29133KC/s odegwoo..odyssay
Use the "--show --format=Raw-MD5" options to display all of the cracked passwords reliably
Session completed
```

The above example tries to brute force at first by trying to guess all possible combinations. It works for the two first hashes rather fast, but the third one is longer word and take exponentially more time.

Instead, masking for lowercase is applied on the second try to get the third one, and the process is much faster.