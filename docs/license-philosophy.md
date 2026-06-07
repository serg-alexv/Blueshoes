# License Philosophy: Vitality vs. Immortality

Choosing a license for Blueshoes depends on whether you prioritize **adoption (vitality)** or **preservation of openness (immortality)**.

## The MIT License (Current Choice)
- **Pros (Vitality):** Maximum frictionless adoption. Commercial router vendors (like GL.iNet themselves, Asus, Ubiquiti) could integrate Blueshoes into their firmware with zero legal friction. It encourages wide distribution.
- **Cons (Mortality):** Someone can take the Blueshoes code, make a highly advanced proprietary version of it, and close the source code. The community does not benefit from their improvements. 

## The GPLv3 / AGPLv3 License (The "Immortality" Option)
- **Pros (Immortality):** Ensures that anyone who modifies Blueshoes and distributes it *must* release their modifications under the same open license. This prevents corporate capture and guarantees the codebase remains a public good for humanity forever.
- **Cons (Friction):** Commercial router vendors are notoriously terrified of GPLv3 (due to "tivoization" clauses). It might reduce the chances of a major vendor shipping it by default in their stock firmware.

## Recommendation for timelabs-npo
If the goal is to protect the project from being enclosed by bad actors while ensuring it remains a tool for internet freedom, **GPLv3** is the standard "immortality" license for router software (OpenWrt itself uses GPLv2). If you want maximum chaotic spread regardless of who profits, stick with **MIT**.
