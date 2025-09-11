# Security Policy

Confidential vulnerability reporting currently works by embedding text encrypted to [OpenPGP Key](https://keys.openpgp.org/vks/v1/by-fingerprint/57CA87DDE11190DEA2446B4755D979432731D4EE) represented via gnupg ascii armor in the body of a standard issue template for security vulnerabilities under [Issues](https://github.com/jurassicLizard/vex2pdf/issues) this is explained under the [Reporting a vulnerability](#reporting-a-vulnerability) section.

## Supported Versions


| Version | Supported          |
|---------|--------------------|
| all     | :white_check_mark: |

## Reporting a Vulnerability

1. Go to [`Issues`](https://gitlab.com/jurassicLizard/vex2pdf/-/issues)
2. Create a new issue and choose the "Incident" type
3. add a non revealing title to the issue or keep the default entry
4. On your local pc write an .md file or any other .txt file and encrypt it to [OpenPGP Key](https://keys.openpgp.org/vks/v1/by-fingerprint/57CA87DDE11190DEA2446B4755D979432731D4EE) after importing the openpgp key
   ```sh
    gpg --keyserver keys.openpgp.org --recv-keys  57CA87DDE11190DEA2446B4755D979432731D4EE
    cat  vulnerability_description.txt | gpg --encrypt --armor --recipient 57CA87DDE11190DEA2446B4755D979432731D4EE
   ```
5. The output should be an ascii encrypted text that is copyable to the issue description section.
6. _Alternatively_ you could directly send an encrypted email to the author
   

