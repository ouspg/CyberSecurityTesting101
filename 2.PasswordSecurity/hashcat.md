# `hashcat` Instructions

![](img/hashcat.png)


This document will work as introduction to `hashcat`.
You can find more information from `hashcat`'s own [documentation.](https://hashcat.net/wiki/doku.php?id=hashcat)

To install `hashcat` for the provided Arch Linux virtual machine, run the command

```sh
sudo pacman -Sy hashcat opencl-clover-mesa intel-compute-runtime pocl
```

This will install both `hashcat` and support for [OpenCL](https://en.wikipedia.org/wiki/OpenCL) for various processors.

> [!IMPORTANT]
> Currently, you **cannot** run `hashcat` in virtual machine, if your host is M1/M2/M3 MacBook. There isn't proper OpenCL support yet.
>  Instead, install [homebrew](https://brew.sh/) and then `hashcat` directly to your host system as `brew install hashcat`, for example.


Then you can get started by listing different flag types for `hashcat` (list will be long)

```sh
hashcat --help
hashcat (v6.2.6) starting in help mode

Usage: hashcat [options]... hash|hashfile|hccapxfile [dictionary|mask|directory]...

- [ Options ] -
...
```

## Cracking with `hashcat`

The most basic `hashcat` command for “cracking” `md5` hashes is

```sh
hashcat “insert_hash_here” -m 0 -a 3
```

This will try to reverse the hash from `md5` type using brute force attack.
You can also use, for example, a text file instead of the hash, and it will crack all the hashes inside the file with given options.

Option `-m` specifies the hash type the hash is.
We will be mainly using `–m 0`  and `-m 10`

The first one is for “pure”  `md5` hashes, and `-m 10` is for salted `md5` hashes.
There are many other options for other cryptographic functions.

Option `-a` specifies the attack type `hashcat` runs trying to figure out the hash.
We will be using `–a 3` as an example, which means brute force attack, where `hashcat` just tries to hash different inputs to figure out the hash. 
Other attack options include using common
password wordlist.

Example result for command `hashcat -m0 -a3 ef775988943825d2871e1cfa75473ec0`:
```sh
ef775988943825d2871e1cfa75473ec0:99999999

Session..........: hashcat
Status...........: Cracked
Hash.Mode........: 0 (MD5)
Hash.Target......: ef775988943825d2871e1cfa75473ec0
Time.Started.....: Sun Jan 21 11:40:51 2024 (0 secs)
Time.Estimated...: Sun Jan 21 11:40:51 2024 (0 secs)
Kernel.Feature...: Pure Kernel
Guess.Mask.......: ?1?2?2?2?2?2?2?3 [8]
Guess.Charset....: -1 ?l?d?u, -2 ?l?d, -3 ?l?d*!$@_, -4 Undefined
Guess.Queue......: 8/15 (53.33%)
Speed.#1.........:  3929.9 MH/s (9.23ms) @ Accel:256 Loops:256 Thr:32 Vec:1
Recovered........: 1/1 (100.00%) Digests (total), 1/1 (100.00%) Digests (new)
Progress.........: 717225984/5533380698112 (0.01%)
Rejected.........: 0/717225984 (0.00%)
Restore.Point....: 0/68864256 (0.00%)
Restore.Sub.#1...: Salt:0 Amplifier:4352-4608 Iteration:0-256
Candidate.Engine.: Device Generator
Candidates.#1....: r90erane -> imogocha
Hardware.Mon.SMC.: Fan0: 0%, Fan1: 0%
Hardware.Mon.#1..: Util:100%
```

The output tells us that hash `ef775988943825d2871e1cfa75473ec0` belongs to value `9999999`.

## Cracking the characters

You can also specify to `hashcat` what character types to try when using brute force attack.
If we want to specify to `hashcat` that the correct password for the `md5` hash is 5 numbers, we can do:

```sh
hashcat “insert_hash_here” -m 0 –a 3 ?d?d?d?d?d
```

This specifies `hashcat` to guess using only numbers (d symbol). 
For other characters, see wiki entry about [mask attack.](https://hashcat.net/wiki/doku.php?id=mask_attack)

For example, if you want to crack `md5` with 9 letter password with starting capital letter
```sh
hashcat “insert_hash_here” -m 0 –a 3 ?u?l?l?l?l?l?l?l?l
```

## Cracking the salt

Hashing with known salt can be done by using
```sh
hashcat “insert_hash_here:insert_salt_here” -m 10 –a 3
```

## In the potfile

Sometimes you will get the following:
```sh
INFO: All hashes found as potfile and/or empty entries! Use --show to display them.
```

This means that the hash has been cracked before and can be shown by adding
“--show” at the end.

## Not enough memory.

Running `hashcat` can take a lot of memory out of the GPU.
This will vary based on the hashing algorithm, attack type and the size of the wordlists, for example. 
It might help to close down applications in the background.
* Device #1: Not enough allocatable device memory for this attack
* 
Possible solutions:
`-D 1` flag to run `hashcat` on CPU and use RAM instead.
`-increment-min` 8 flag to run hash at min 8 characters, reducing the memory usage.
