
# 6. Fuzzing

In the previous week, we handled input sanitation and validation, especially the context-specific injections.

We didn't handle the more dangerous and complex memory-related problems, which are typically also the result of specific, unhandled input.
Historically, the most dangerous type of these has been [buffer overflows](https://en.wikipedia.org/wiki/Buffer_overflow).

Sometimes, the program does not handle the data properly on the lower level, and it causes data to be written to invalid parts of the memory, or even accessing it incorrectly.
This has been an issue, especially in [C and C++ languages](https://www.cisa.gov/news-events/news/urgent-need-memory-safety-software-products).

The exploiting and internal explanation of these bugs are handled in Cyber III: Software and Hardware Security course.

On this week, we briefly introduce some testing methodologies to detect these.

These methodologies **can also detect other input-related issues, like injections**.

For additional material, check the [fuzzing book](https://www.fuzzingbook.org/).
## Grading

You can obtain up to five points from this exercise.

You are not required to do tasks in order, but it is recommended.


| Task # | Points | Description |
| ---- | :--: | ---- |
| Task 1 | 2 | Getting started with fuzzing (Moodle Exam)  |
| Task 2 | 1 | Fuzzing with AFL++ (GitHub) |
| Task 3 | 2 | Web fuzzing with `ffuf` (GitHub) |

# Task 1: Getting started with fuzzing

> This task is Moodle exam.

Read the introduction material about fuzzing [here.](https://github.com/ouspg/fuzz-testing-beginners-guide)

In this exercise, will be using [radamsa](https://gitlab.com/akihe/radamsa), which is a test case generator for general-purpose robustness testing.

You first need to install `radamsa` and its dependencies, in case they are not installed already.

In Arch Linux, run:

```sh
pacman -Sy radamsa
```

You can find instructions on manual installation in Radamsa's GitLab page.

You can also use Radamsa as a library, e.g. with a [Python wrapper](https://github.com/tsundokul/pyradamsa). There is native support for C and Schema, and also a native [port in Rust](https://github.com/microsoft/rusty-radamsa).


### Task 1A) Basic use of Radamsa (0.5p)

Generating inputs with Radamsa can be done with, for example:

```sh
echo -n "Fuzztron 2000" | radamsa
```

where the initial input is “Fuzztron 2000”.
Play around a bit — you can see how the output changes each time.

Radamsa uses the provided string above as the base input value, which is then further [mutated](https://www.fuzzingbook.org/html/MutationFuzzer.html). 

Radamsa and most other fuzzers are pseudorandom; meaning that they provide outputs deterministically.
This is important for reproducing the possible test cases and relevant errors.
Radamsa takes a seed from `/dev/urandom` by default, but you can also set the seed manually with `-s` or `--seed` parameter.
The seed works as a starting point for the fuzzer's random value generator, and based on that, further values are generated. 

Typically, you don't need to set the seed value, but instead when fuzzing, you might want to capture the metadata with `-M` flag, and then reproduce the possible output data, if you want to reproduce some testing out.
For reviewing purposes, we also use seed here.


> In the exam, you get initial input string and seed.  Return `SHA256` sum of the output as result. Output is always the same because of the given seed.

> [!Tip]
> Make sure to not add anything extra for the fuzzer input (like newline characters.) You can pipe output directly `sha256sum`, for example, to get the hash. Data might contain non-visible characters!

### Task 1B) A bit more samples with Radamsa (0.5p)

With `radamsa`, you can generate multiple samples at once, where directly into files is recommended. 

Find out how can you do that.

> In the exam, you are given an initial input string and seed. Return the `SHA256` sum of the asked `nth` file contents. 

### Task 1C) Testing actual program (1.0p)

You will find a vulnerable program [`sample.c`](sample.c), which takes input both as file argument and from `stdin`.
Check the source to identify the vulnerabilities.

You can compile it as

```sh
gcc -o sample sample.c
```

And then, you can play around like

```sh
echo -n "Voilà" | radamsa | ./sample
```

There are two ways to actively fuzz test the program:

1. Loop in bash or in some other programming language to repeatedly call `radamsa` and pipe the output for the sample program. Check `radamsa` documentation, how to create a loop and check error exit codes.
2. Or generate multiple files at once, and try to run the program for each of them, using programming again.

To complete and get this task automatically graded, you need to use the *file-based method.*

> Based on the given the input and the seed, what is the *first* file to crash the program? Make sure to linearly iterate the numbers from 1 to upwards. You should return integer as an answer.

## Task 2: Fuzzing with AFL++

> Return this task to GitHub

Radamsa has great mutation capabilities, but it is not [coverage-guided](https://www.fuzzingbook.org/html/Coverage.html) — which means that it is sometimes harder to automatically discover deeper paths in the software.

The software might use many protocols in many layers, and if you also want to test the most internal layers, your input data needs to be valid for the outer layers before it gets to the inner layers.

Coverage guiding is achieved by instrumenting the compiled binary — during fuzzing the fuzzer detects these instruments and knows which paths of the program have been processed, and based on that, it can adapt the input generation to discover new code paths.
This is also known as feedback-based or feedback-driven fuzzing.

One well-known fuzzer like this is [afl++](https://aflplus.plus/), based on the no-longer-maintained [afl](https://github.com/google/AFL).
Check the [docs](https://aflplus.plus/docs/) for more, especially part [fuzzing in depth](https://aflplus.plus/docs/fuzzing_in_depth/).

To install `afl++` from the official repository in Arch Linux, run:

```sh
pacman -Sy afl++
```

To detect problems, that might not typically lead to fatal crashes of the program, we can use sanitizers, which can find memory corruption vulnerabilities like use-after-free, NULL pointer dereference and buffer overruns.
For this task, we will test [AddressSanitizer](https://github.com/google/sanitizers/wiki/AddressSanitizer). It also can tell what is the problem in the code.

To enable Address Sanitizer with `afl++` instrumentation, compile the same `sample.c` binary as

```sh
AFL_USE_ASAN=1 afl-cc -o sample sample.c
```

And then start fuzzing! Find the information on how to start the fuzzer.
It should find thousands of crashes pretty fast.
The provided sample program is not complex, so the instrumentation is not very useful here, but we just get an introduction for it.

After you have fuzz tested a while, take a look at your *output* directory, which stores the metadata and the files that caused a crash.
Reproduce the crash by running one of them as the parameter for the sample program e.g. 

```sh
./sample outputs/crashes/id:000000,sig:11,src:000006,op:havoc,rep:64
```

You should also see AddressSanitizer output.

> **As completion of this task, do the following and answer to the questions:**

- Take a screenshot of AFL++ report screen of your own execution with crashes
- Give all the required commands for completing this task.
- Copy-paste the AddressSanitizer output. Does it identify the line of code in the program, which causes the problem? What other information does it tell?
- How many crashes did the fuzzer find? How many of them were unique?
- How many cycles did the program do? What does “cycles” mean?
- When you should stop the fuzzer? Explain.
- What fuzzing strategy is being used at the time of the screenshot?


## Task 3: Web fuzzing with [`ffuf`](https://github.com/ffuf/ffuf)

> Return this task to GitHub

This task is a bit more open-ended.
Fuzzing can be done to every interface, and `ffuf` is especially focused on the HTTP protocol and web interfaces.

You can use fuzzing to enumerate possible subdomains or routes on the website, brute force passwords, check API response codes and so on.
Read `ffuf`'s [wiki](https://github.com/ffuf/ffuf/wiki) for more.

You can choose one task to use `ffuf` to fuzz test OWASP's Juice Shop.  Check the previous week to get started with Juice Shop.


Available possibilities:

1. [Log in with Amy’s original user credentials](https://pwning.owasp-juice.shop/companion-guide/latest/part2/sensitive-data-exposure.html#_log_in_with_amys_original_user_credentials)
2. [Retrieve the language file that never made it into production](https://pwning.owasp-juice.shop/companion-guide/latest/part2/broken-anti-automation.html#_retrieve_the_language_file_that_never_made_it_into_production)
3. [Reset Morty’s password via the Forgot Password mechanism](https://pwning.owasp-juice.shop/companion-guide/latest/part2/broken-anti-automation.html#_reset_mortys_password_via_the_forgot_password_mechanism)

> Select **one** from above, and explain the process what you needed to do to brute force some required information. Include all the commands and possible other code. Challenge solutions are available for the above, but you still need to do some work to adapt `ffuf` for this task.
