# 3. Messaging Security

In this week, we handle the topics of encryption, trust and digital identity in the context of messaging. 
When we talk about messaging in this exercise, we upgrade the definition for all digital communication and data exchange. 
The similar principles apply also in there.

The first task covers the basics of public-key cryptography and encryption.
In the second task, we explore a bit about digital entities and trust systems in that context; how is the trust of the web page built.
On the third task, we observe what certificates are. 


> The workload is based on the assumption that students will use LLMs!

This exercise covers some very basics of modern cryptography and acts as introduction to the field.
If you would like to know more about the topic, check out the courses **521244S Modern Cryptography (maths)** and **IC00AK18 Cryptographic Systems and Their Weaknesses (technical implementations)**.

## Grading

You can obtain up to five points from this exercise.

You are not required to do tasks in order, but it is recommended this time. They build knowledge on top of each other. 

| Task # | Points | Description |
| ---- | :--: | ---- |
| Task 1 | 3 | The concept of computational complexity (Moodle exam) |
| Task 2 | 1 | Digital signatures (Moodle exam) |
| Task 3 | 1 | Digital identity and trust (Return to GitHub) |
|  |  |  |

Later tasks will require more time investment when compared to the previous tasks to acquire the relative amount of points. 

# Task 1: The concept of computational complexity

Regular passwords (or secrets) alone are not enough to cover all real world requirements of authentication, confidentiality, and integrity.  Especially in the digital messaging context, there are a variety of challenges what we need to consider and solve.

Imagine if you want to send a secret letter to a friend. You put it in a box and lock it with a key. But how do you get the key to your friend without someone else grabbing it first? This is where regular passwords or secrets show their limits. They're like a single key that both locks and unlocks the box. If you share this key, it could fall into the wrong hands. This kind of one-key system is known as a [symmetric-key algorithm](https://en.wikipedia.org/wiki/Symmetric-key_algorithm) because the same key is used on both ends.
We could try to deliver this key on a separate secure line, but that would make the original line impractical, even if we would be somehow able to guarantee the security of another line.
And in that case, why bother with the box?

There are two approaches for solving the previous problem
  * Could it be possible to develop **a secure protocol** where we can share a secret over the same insecure line, with proved security?
  * Or maybe we don't need to share the secret, *at all*.

Currently, known solutions for the both previous approaches are based on the **[concept of computational complexity.](https://en.wikipedia.org/wiki/Computational_complexity)**

 We can make use of **complex problems** which are computationally feasible to perform in [one direction](https://en.wikipedia.org/wiki/One-way_function) (e.g., multiplying large primes) but extremely difficult to reverse (e.g., factoring the product of these primes) without having the secret.
This is the area of modern cryptography; we use [computational hardness assumptions](https://en.wikipedia.org/wiki/Computational_hardness_assumption) as base for secrecy. 
We assume again that systems are secure if *any adversaries are computationally limited*. The security follows the idea of information entropy from the previous week, but well, *it is much more complex*.

Let's consider the second scenario from a complex problem perspective; what if we don't need a secret?
If we continue with the box example, it's like giving your friend an unbreakable box with a special lock. Your friend sends you this box, which anyone can lock, but only your friend (the one who sent it) has the unique key to open it. This means you (who received the box) can send your secret message safely, without ever needing to worry about the key being stolen!

This is called as [public-key (or asymmetric) cryptography](https://en.wikipedia.org/wiki/Public-key_cryptography). In a short, we have **public key** to encrypt the contents and **private key** to decrypt them.  The secrecy is based on computational hardness; it is easy to encrypt with public key, but extremely difficult to decrypt with it, and hence, we require private key to access the contents. 

We only cover one basic example in this course how it works internally, but it is a very important concept on a higher level.

## Task 1A) Key exchange algorithms: Diffie-Hellman

[Diffie-Hellman key exchange](https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange) was one of the first widely adapted public-key protocols. 
It is based on [the discrete logarithm problem.](https://en.wikipedia.org/wiki/Discrete_logarithm)
And it is still one of the most critical protocols out there; **every [TLS connection](https://en.wikipedia.org/wiki/Transport_Layer_Security) &#128274; from your browser and other places uses it!**
Also, [Signal](https://signal.org/docs/specifications/x3dh/#introduction) and WhatsApp (based on Signal's protocol), for example, use the **advanced version** of the same protocol.

> The protocol *makes possible to exchange shared secret key under the insecure line* with some proved security assumptions, if we select parameters correctly. 

Let's take a look at the original **Finite Field Diffie-Hellman** protocol, which is easier to understand.  Wikipedia is *very accurate and correct* in cryptography, and you should read that about Diffie-Hellman. Here we have a shortened version.
We don't have to understand all the maths related to the problem for understanding the protocol itself. 

$$
\begin{align*}
 \text{Let the following happen} \\
 \text{with Alice and Bob:} \\
& g \text{ is a publicly shared base and primitive root} \\
& p \text{ is a publicly shared prime number} \\
& a \text{ is Alice's private key} \\
& b \text{ is Bob's private key} \\
\end{align*}
$$

Alice and Bob each choose their private keys, $a$ and $b$, which are kept secret. They compute their public keys as follows:

$$
\begin{flalign*}
\text{Alice's public key:} \quad & A = g^a \mod p \\
\text{Bob's public key:} \quad & B = g^b \mod p
\\\\
 \text{Where:} \\
& A \text{ is Alice's public key.} \\
& B \text{ is Bob's public key.}
\end{flalign*}
$$

After exchanging their public keys, Alice and Bob can compute the shared secret independently:

$$
\begin{align*}
& \text{Shared secret as computed by Alice:} \quad & S_A = B^a \mod p \\
& \text{Shared secret as computed by Bob:} \quad & S_B = A^b \mod p
\end{align*}
$$

The shared secret value depends on the initially chosen secrets, and shared secrets will be equal because of the properties of the modular exponentiation.

$$
{A}^{b}\bmod {p} = {g}^{ab}\bmod {p} = {g}^{ba}\bmod {p} = {B}^{a}\bmod {p} \Rightarrow S_A = S_B
$$


<details><summary>As a result, we get the following sequence diagram. (click me!)</summary>

```mermaid
sequenceDiagram
    participant Alice
    participant Bob
    Note over Alice,Bob: Initial Setup
    Alice->>Bob: Agree on public numbers g and p
    Bob->>Alice: Agree on public numbers g and p
    Note over Alice,Bob: Private Keys
    Alice->>Alice: Choose private key a
    Bob->>Bob: Choose private key b
    Note over Alice,Bob: Public Keys Calculation
    Alice->>Alice: Calculate A = g^a mod p
    Bob->>Bob: Calculate B = g^b mod p
    Alice->>Bob: Share public key A
    Bob->>Alice: Share public key B
    Note over Alice,Bob: Shared Secret Calculation
    Alice->>Alice: Compute shared secret S_A = B^a mod p
    Bob->>Bob: Compute shared secret S_B = A^b mod p
    Note over Alice,Bob: Final Shared Secret
    Alice-->>Bob: S_A = S_B (Shared Secret Established)
    Bob-->>Alice: S_A = S_B (Shared Secret Established)
```

</details>

### Task assignment: Brute forcing the private key — **how secret is this secret?**

Let's observe in practice how hard it actually can be to “crack” the private key, even on small numbers. You can do this by simply brute forcing the private key. Iterate all possible numbers based on the group $p$ and see if with some number you are able to get the matching public key. You can assume that private key is smaller than $p -1$.


This means that you need to do a small programming exercise by applying previous equations and guess the private key.
The assignment uses 30-bit length for $p$. 


> [!Tip]
> You will notice, that linear brute forcing process is very slow, and you want to speed up the process, by using [Baby-step giant-step algorithm!](https://en.wikipedia.org/wiki/Baby-step_giant-step) 
> *One might find some existing code pieces to just reuse them or consult a friendly LLM…*
> If you want to go the deep end, [here are](https://math.mit.edu/classes/18.783/2022/LectureNotes9.pdf) some excellent notes (not required).


Since we work on very small numbers (30 bits), it is rather easy to find multiple different private keys which produce the same public key. This highlights the importance of usage of large numbers in Diffie-Hellman.

* The properties of cyclic groups and the behaviour of the modulo operation can lead to situations where multiple different exponents (private keys) result in the same value when raised with a given base $g$ and taken modulo $p$.
* The range of possible keys is significantly limited due to low bit length, and this increases the chances for collisions. 
* The security is based on the probability assumptions of above. Again, the concept of information entropy.
* However, **the shared secrets are always the same because of the properties of modular exponentiation!**

> [!Important]
> On the Moodle exam, you will be provided Diffie-Hellman parameters, excluding the Bob's and Alice's hidden secrets. Brute force the private key, either by using Bob's or Alice's public key, **then calculate the shared secret by using the other public key**. The bit length is intentionally selected so that you might need to consider using “a baby-step giant-step algorithm”.


Correct parameter selection is essential in Diffie-Hellman to get the maximum security. We don't cover that in depth this course, but here is an overview.

$$
\begin{align*}
& \text{Let } p \text{ be a large prime and } g \text{ a primitive root modulo } p. \\
& \text{The private keys } a \text{ (for Alice) and } b \text{ (for Bob) are chosen under the following conditions:}
\end{align*}
$$

$$
\begin{align*}
1.\ & a, b \in \mathbb{Z} \\
2.\ & 1 < a, b < p-1 \\
3.\ & a, b \text{ are chosen randomly and kept secret} \\
\\
\text{Where:} \\
\mathbb{Z} & \text{ denotes the set of all integers.} \\
a, b & \text{ are the private keys of Alice and Bob, respectively.}
\end{align*}
$$

***Minimum*** recommended length for Diffie-Hellman's group prime $p$ is usually 2048 bits.  Sometimes there can be benefits if the prime $p$ is also a safe prime: $p=2q+1$ where $q$ is also prime.  

$\textcolor{red}{\textit{If you manage to solve the discrete logarithm problem in polynomial-time, \textbf{you will break the world.}}}$

If you have ever wondered of the excitement around quantum computers, that is what they potentially [can do.](https://en.wikipedia.org/wiki/Shor's_algorithm)

## Task 1B) Basics of public-key encryption

Diffie-Hellman key-exchange algorithm can be expanded to [ElGamal encryption](https://en.wikipedia.org/wiki/ElGamal_encryption).  
Let's take a look at a simplified explanation of that.

In the context of Diffie-Hellman, each participant often uses public keys only once per exchange to maintain [perfect forward secrecy](https://en.wikipedia.org/wiki/Forward_secrecy).
However, in ElGamal encryption, a key aspect is the introduction of a unique, temporary key for each encryption process. This key is known as the _ephemeral key_, denoted as $k$. 
Typically, only the message sender changes this key, and the receiver's public key stays constant.

The secret $k$ must be changed for each message to _maintain the entropy of the message; it should be truly randomly generated._

Consider a scenario where Bob sends an encrypted message to Alice. The encryption process in `ElGamal` is as follows:


$$
\begin{align*}
S = A^k \mod p \\
C_1 = g^k \mod p \\
C_2 = M \times S \mod p \\
\\
\text{Where:} \\
& S \text{ is shared secret.} \\
& k \text{ is ephemeral key (Bob's hidden secret).} \\
& A \text{ is Alice's public key.} \\
& C_1 \text{ is sender's ephemeral (aka Bob's) public key.} \\
& M \text{ is the message as integer.} \\
& C_2 \text{ is the ciphertext.}
\end{align*}
$$

$C_1$ and $C_2$ will be delivered to Alice, and Alice decrypts the 
content as follows.

$$
\begin{align*}
\text{To decrypt:} \\
 S &= C_1^a \mod p \\
 M &= C_2 \times S^{-1} \mod p \\\\
&\text{Where }a \text{ is Alice's hidden secret and } S^{-1} \text{ is the modular multiplicative inverse of } S \text{ modulo } p.
\end{align*}
$$

The decryption process effectively cancels out the shared secret, leaving the original message $M$.

The above is simplification; usually $C_1$ is just thought as a component necessary for the decryption of the ciphertext, but when comparing to the Diffie-Hellman, it can be thought as public key. 

Overall, we can think that `ElGamal` encryption combines the aspects of both asymmetric and symmetric encryption. The shared key is obtained with asymmetric means, but internally the encryption and decryption of the data is symmetric by using the same shared secret.

*The message size for public-key encryption algorithms must smaller than $`p - 1`$ or even smaller. As a result, public-key algorithms are commonly used to encrypt the secret of the symmetric key algorithms!* They are also slow, because they do complex mathematical operations with large numbers.

> [!Important]
> On the Moodle exam, you will get all Diffie-Hellman parameters and ElGamal encrypted message. Parameter $p$ is 1024 bits, so **you will notice some big numbers**.  Decrypt the message by applying the above. You don't have to break anything here, just apply the equations to decrypt the message. We just want to understand the basic concept.

> [!Tip]
>Covert the resulting integer to ASCII; reverse the process (in Python) `m_int = int.from_bytes(sentence.encode('utf-8'), "big")`, which was used to convert the original message into integer. Note that you must round up bits to next full byte to not lose any data, when considering how many bytes this integer takes to present. Original message is a phrase and ends to dot `.`.

The exam parameters are only for educational use, and $p$ is not safe prime. The security depends on the correct generation of the parameters. The original Diffie-Hellman is also vulnerable to [man-in-the-middle attack.](https://en.wikipedia.org/wiki/Man-in-the-middle_attack) 


<details><summary>On high level, with modern public-key algorithms, the encryption sequence can be following. (click me!)</summary>

```mermaid
sequenceDiagram
    participant Alice
    participant Bob
    Note over Alice,Bob: Key Generation
    Alice->>Alice: Generate Public_A and Private_A
    Bob->>Bob: Generate Public_B and Private_B
    Note over Alice,Bob: Public Key Sharing
    Alice->>Bob: Share Public_A
    Bob->>Alice: Share Public_B
    Note over Alice,Bob: Encryption
    Alice->>Bob: Encrypt message with Public_B
    Bob->>Alice: Encrypt message with Public_A
    Note over Alice,Bob: Decryption
    Bob->>Bob: Decrypt with Private_B
    Alice->>Alice: Decrypt with Private_A
```
</details>

## Task 2: Digital signatures and wannabe Alice's

> Return as Moodle exam

Public-key cryptography is also the foundation of [digital signatures](https://en.wikipedia.org/wiki/Digital_signature). 
Digital signing is used to ensure the _integrity and authenticity of a message_.

We sign data using **private keys**, and validate the signature using **public keys**.

If you ever have wondered how digital signatures in PDFs and other places work, they are also derived from the public-key cryptography.
If you own an ID card, [it has certificate and private key inside](https://dvv.fi/en/citizen-certificate-electronic-signature).

Both integrity and authenticity are achieved by first creating a hash (a fixed-size string of bytes) of the original message, and then signing this hash with the private key. 
Anyone can verify the signature using the correct public key.

When proving the integrity, the message content is hashed, and this hash value is compared to the value obtained from the signature after verifying the signature. The hash ensures that the content is unmodified. If the signature is correctly verified with the public key, it also validates the origin of the message.

### Task assignment

Alice has attempted to send a message for Bob.

The message is in the “messages” folder. However, there are many *wannabe Alice's* who are not the real and try to confuse and scam Bob. 
Bob cannot be sure which message is the correct one.

> Clone this repository to get the messages.

However, *the real Alice* was smart and she digitally signed her message. Wannabe Alice's have signed their messages too, but they didn't have the correct private key…

Alice has shared her public key with a small twist; it is a QR code. What is the correct message? Can you help Bob?

> You will get the QR code in Moodle exam. Return the account where message is referring into.

All the messages have been signed by using [GnuPG](https://www.gnupg.org/).
GnuPG is based on the [OpenPGP standard,](https://www.openpgp.org/) which was originally created for end-to-end encryption of emails.

On this task, you will need to import Alice's public key to `gpg`.
The course VM has[`gpg`](https://man.archlinux.org/man/gpg.1) pre-installed. 

To read a single message, you can use command `gpg --decrypt <message>.`


Since there are quite a lot of messages, you need to somehow automate this process and see which message can be validated with the provided public key.
You can use either just `bash` script or [`python-gnupg`](https://gnupg.readthedocs.io/en/latest/) library.

To read the QR codes, you need to install `zbar` or just use your phone to get the public key.
QR code contains a public key which uses PGP's ASCII-armoured format.

`sudo pacman -Sy zbar`

Then, from the command-line, you can use `zbarimg` to read QR codes.

Extra: if you want to [learn more about QR codes](https://qr.blinry.org/).

You can ignore the following when you find the correct message:

```sh
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
```

## Task 3: Digital identity and trust

> Return this task to GitHub

Take a look what [digital certificates](https://en.wikipedia.org/wiki/Public_key_certificate) are.
Public-key cryptography is also the foundation of the certificates.

Certificate can be thought as digital information, which is issued for specific purpose.
The issuer uses digital signatures to sign information in specific format, ensuring that trust and integrity of the information is based on the issuer. The most common such a format is [X.509.](https://en.wikipedia.org/wiki/X.509)

Digital certificates are issued by trusted entities known as Certificate Authorities (CAs).
We have ultimate trust for these authorities, and these authorities are used as root of trust to create digital identities for other entities.

For example, when you use a website, and if this website uses TLS connection, it has certificate, which is used as base for secure connection.

When the browser initiates the connection, certificate authenticity is validated by the browser. The domain name of the website is tied to the certificate, and in that way the browser attempts to verity that data is indeed coming from the claimed entity.
Once authenticity is validated and an encrypted connection is formed, then the browser accesses the content of the website.

The flow behind the scenes can be simplified as follows:

1. **Certificate Presentation**: When you connect to a secure website, it presents its TLS certificate to your browser.
    
2. **Certificate Validation**: Your browser checks the certificate's validity. This involves verifying that it hasn't expired and that it's signed by a trusted CA.
    
3. **Domain Verification**: The browser ensures that the certificate's domain name matches the website's domain. This step is crucial to prevent man-in-the-middle attacks.
    
4. **Certificate Revocation Check**: The browser may check if the certificate has been revoked by the CA, indicating it should no longer be trusted.
    
5. **Encryption Algorithms Confirmation**: The browser also verifies the encryption methods (ciphers) used in the certificate to ensure they meet current security standards.

> In this task, we attempt to do the same manually with some command-line tools.


[`curl`](https://curl.se/) is the de-facto way to interact with websites from command line.

With command `curl -v <website>` you can also see the process of certificate validation and secure connection initiation.

```sh
» curl  -v https://oulu.fi

*   Trying 130.231.240.1:443...
* Connected to oulu.fi (130.231.240.1) port 443
* ALPN: curl offers h2,http/1.1
* (304) (OUT), TLS handshake, Client hello (1):
*  CAfile: /etc/ssl/cert.pem
*  CApath: none
* (304) (IN), TLS handshake, Server hello (2):
* TLSv1.2 (IN), TLS handshake, Certificate (11):
* TLSv1.2 (IN), TLS handshake, Server key exchange (12):
* TLSv1.2 (IN), TLS handshake, Server finished (14):
* TLSv1.2 (OUT), TLS handshake, Client key exchange (16):
* TLSv1.2 (OUT), TLS change cipher, Change cipher spec (1):
* TLSv1.2 (OUT), TLS handshake, Finished (20):
* TLSv1.2 (IN), TLS change cipher, Change cipher spec (1):
* TLSv1.2 (IN), TLS handshake, Finished (20):
* SSL connection using TLSv1.2 / ECDHE-RSA-AES128-GCM-SHA256
* ALPN: server did not agree on a protocol. Uses default.
* Server certificate:
*  subject: C=FI; ST=Pohjois-Pohjanmaa; O=University of Oulu; CN=www.oulu.fi
*  start date: Sep 13 00:00:00 2023 GMT
*  expire date: Sep 12 23:59:59 2024 GMT
*  subjectAltName: host "oulu.fi" matched cert's "oulu.fi"
*  issuer: C=NL; O=GEANT Vereniging; CN=GEANT OV RSA CA 4
*  SSL certificate verify ok.
* using HTTP/1.x
> GET / HTTP/1.1
```

We can see that `curl` does the validation for us. How it happens? Internally, `curl` depends on some TLS library, which is usually [`OpenSSL`](https://www.openssl.org/). 

You can, for example, use `openssl` to check all the certificates the server provides for you:

```sh
» openssl s_client -connect oulu.fi:443 -showcerts
CONNECTED(00000003)
depth=2 C = US, ST = New Jersey, L = Jersey City, O = The USERTRUST Network, CN = USERTrust RSA Certification Authority
verify return:1
depth=1 C = NL, O = GEANT Vereniging, CN = GEANT OV RSA CA 4
verify return:1
depth=0 C = FI, ST = Pohjois-Pohjanmaa, O = University of Oulu, CN = www.oulu.fi
verify return:1
---
Certificate chain
 0 s:C = FI, ST = Pohjois-Pohjanmaa, O = University of Oulu, CN = www.oulu.fi
   i:C = NL, O = GEANT Vereniging, CN = GEANT OV RSA CA 4
   a:PKEY: rsaEncryption, 2048 (bit); sigalg: RSA-SHA384
   v:NotBefore: Sep 13 00:00:00 2023 GMT; NotAfter: Sep 12 23:59:59 2024 GMT
-----BEGIN CERTIFICATE-----
...
```

 > **Experiment with `curl` and `openssl` on a chosen website. Include the commands and try to answer for the following questions:**
  - **Who has issued the certificate?**
    - This helps identify the Certificate Authority (CA) that authenticated the website's identity.
  - **For what exact domain it has been certificated?**
    - Certificates are issued for specific domain names, ensuring you're communicating with the intended website.
  - **When does it expire?**
    - TLS certificates have a validity period. Knowing the expiration date is important for maintaining the website's security.
  - **What is the encryption algorithm and key size?**
    - This information provides insight into the strength and type of encryption used for securing communications.
  - **Manual validation with `openssl verify` or `openssl s_client` with local file:**
    - Attempt to manually validate the website's certificate using the local root CA files. If you download the target server's certificate and possibly other parts, can you verify it?
  - **Troubleshooting failed validation:**
    - If manual validation fails, explain how this process typically happens on Linux and what components might be necessary. This can include understanding the role of root and intermediate certificates, and how to construct a complete certificate chain for verification.
