# 5.  Data Input Security

Software faults, errors, or bugs are familiar terms to anyone who has been involved in software development.
These terms typically refer to any condition that prevents a system from performing its required functions. 
Such issues can arise from design faults, implementation errors, or a combination of both, ultimately leading to the same result. [^1]

Sometimes these faults can have more serious consequences than software just “not working”.
There is a specific goal of what software is supposed to do.
Whoever designed and implemented the software, set these goals, maybe based on the strict requirement list.
When designing the software, even the cyber security triad CIA might have come up. 
What if the design or implementation fault compromises some, if not all, of the triad? Or the requirements were just misunderstood?

The **Common Weakness Enumeration** (CWE) is a well-known category system [^2] for these kinds of faults; It especially focuses on faults or *weaknesses* which has an impact on the security of the software or overall systems, and provides a framework for understanding and mitigating potential security risks.

> A “weakness” is a condition in a software, firmware, hardware, or service component that, under certain circumstances, could contribute to the introduction of vulnerabilities. [^5]


We can take a look at CWE's top 25 most dangerous software weaknesses from 2022 [^3] and 2023 [^4], and see that *most of the most dangerous weaknesses are related to data processing of the software, especially to data that can be thought as **input.***


We can further see this by looking into the *Top 10 Known Exploited Vulnerabilities Catalog from 2023, which was published by the United States' [Cybersecurity and Infrastructure Security Agency (CISA)](https://www.dhs.gov/cisa/cybersecurity-division).
It maps the usage of known exploits to weakness categories and ranks the misuse of the specific weaknesses after the vulnerability in the software has been found. [^5] [^6]
We have more about software-specific vulnerabilities on the final week's exercises.

![KEV_top10.png](KEV_top10.png)

Memory errors are typically related to input processing, and they are also handled more in the next week.
Their dangers and exploitation of them are handled more in-depth on the course *Cyber Security III: Hardware and Software Security.*

From the list, 4, 5, 6, 8 and 9th are also related in some way to input. 
Typically, the software operates through interfaces. 
Software functions by executing predefined sequences of operations, which are directed through user interfaces or automated inputs. 
The software is designed to follow a specific workflow, branching into different execution paths based on its programming logic.

Inputs act as the initial triggers that activate and guide the software's processes. As the software receives new data through its interfaces, these inputs can alter the course of its operational logic, leading to varied outcomes.
What if some of these inputs are something unexpected?

The type of data a software system can accept and correctly process is determined by its _input validation and sanitisation_ mechanisms. These mechanisms attempt to ensure that only appropriate and safe data is processed, protecting the system from unexpected behaviour or security vulnerabilities.

However, these can fail, as seen above.

## Input validation

Input validation checks if the input meets a specific criterion before accepting it — kind of like a bouncer at a club.

Let's say, we want to validate that program takes **only valid emails as input**. 
It might sound easy, but for example, the 99.99% correct RFC 5322 standard compliant email regex looks like the following [^7].

```none
(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])
```

Input validation can fail, and if your security depends on the validator which might not be 100% correct, there can be issues.
If the unintended input that bypasses the validator can do something unexpected with the underlying software, you might have introduced a vulnerability.

## Input sanitisation

Input sanitisation, on the other hand, cleans up the input, removing any potentially malicious bits that could cause chaos.

Imagine you have a web application with a form that asks users for their names, which will be displayed on the website.
Without sanitisation, an attacker could enter a script into the name field, like `<script>alert('Hacked!');</script>`.
If unsanitised, this script could execute in the browser of anyone viewing the name, leading to a Cross-Site Scripting (XSS) attack [^8].

To prevent this, input sanitisation would involve stripping out or encoding potentially dangerous characters or strings before they're processed or displayed. For example, you could sanitise the input by converting characters like `<` and `>` into their HTML-encoded equivalents `&lt;` and `&gt;`, rendering the script harmless and preventing it from executing as part of the webpage.

Input sanitisation can also fail, for example, if you do not process every dangerous character, or if there is a way to undo or bypass this process.

The exercises focus purely on input processing this week, mostly by using manual means for testing it.

## Grading

You can obtain up to five points from this exercise.

You are not required to do tasks in order, but it is recommended.

All the tasks should be returned to GitHub this week.

| Task # | Points | Description |
| ---- | :--: | ---- |
| Task 1 | 2 | Basics of command injections |
| Task 2 | 2 | Interceptions and SQL injections |
| Task 3 | 1 | TBA |
|  |  |  |

## Task 1: Basics of command injections

> Return this task to GitHub

Take a look at the sample Python application [ping_service.py](ping_service.py).

It has a [command injection](https://owasp.org/www-community/attacks/Command_Injection) weakness.


Assuming that you have `ping` command and Python on your system, you can run it as

```sh
python ping_service.py 1.1.1.1
```

As a completion of this task, answer the following questions.

### Q1: Find an example command injection that prints the content of `/etc/passwd` file, by just providing input for the sample program.
### Q2: It has potentially two issues which have led injection to be possible. What are they? 

### Q3: How can you fix them?

### Q4: How would you implement either input validation or input sanitisation for this context? What could be better? 

> [!Note]
> *Implement either one of them, or both, and provide the code.* Command-injection should not be possible any more.

### Q5: How can you be sure that injection is not possible anymore?

### Extra: Do the same (and more) with a Damn Vulnerable Web Application (DVWA).

Check the [DVWA repository](https://github.com/digininja/DVWA/tree/master) and clone it.
Run `docker-compose up` on that directory.

Log in as `admin:password`in address `http://localhost:4280`, create the default database and log in again. You will find `ping` command injection from there, but this time with PHP implementation.

On the left, close to the bottom, there is **DVWA Security** section; on there you can change the security level and see how the source code implementation changes for that part.
You must also set this to other than impossible before you can actually make an injection. 
You can adjust the injection difficulty with the same setting (how good is the input validation/sanitisation!).

# Task 2: Interceptions and SQL injections

TBA

# Task 3:  TBA


 [^1]: [Software bug](https://en.wikipedia.org/wiki/Software_bug)
 [^2]: [Common Weakness Enumeration (CWE)](https://cwe.mitre.org)
 [^3]: [2022 CWE Top 25 Most Dangerous Software Weaknesses](https://cwe.mitre.org/top25/archive/2022/2022_cwe_top25.html)
 [^4]: [2023 CWE Top 25 Most Dangerous Software Weaknesses](https://cwe.mitre.org/top25/archive/2023/2023_top25_list.html)
[^5]: [2023 CWE Top 10 KEV Weaknesses List Insights](https://cwe.mitre.org/top25/archive/2023/2023_kev_insights.html#)
[^6]: [Known Exploited Vulnerabilities Catalog](https://www.cisa.gov/known-exploited-vulnerabilities-catalog)
[^7]: [Email Address Regular Expression That 99.99% Works.  Disagree?](https://emailregex.com)
[^8]: [Cross Site Scripting](https://owasp.org/www-community/attacks/xss/)

