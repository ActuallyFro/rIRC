Rust-based IRC waRgaming Client (rIRC)
======================================
An Attempt to Build an IRC Client that works with the Rust-based IRC server (RIS).

* It OUGHT to comply with the [RFC for IRC Client](https://www.rfc-editor.org/rfc/rfc2812.txt) -- but it's NOT enforced.

* The IP/port is provided as a single string at the start of program execution (e.g., `127.0.0.1:6667`).

Simplified Commands list
------------------------
* `/nick <nickname>`: Set your nickname.
* `/join <channel>`: Join a channel.
* `/quit`: Disconnect from the server.

* ONLY Sending a message to the joined channel is supported. The "ready to send" status shows the '>' character. Press enter to send the message.

Open Source License Note
------------------------
1. The project is licensed under the [Creative Commons Zero v1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/) license.
2. It aligns to the DoD CIO's [MFR on Software DEvelopement and Open Source Software](https://dodcio.defense.gov/portals/0/documents/library/softwaredev-opensource.pdf) dated 24 Jan 22.
3. This license extends the policy intrepretation of CISA's Open Source Approach. [Details Here](https://github.com/cisagov/development-guide/blob/develop/open-source-policy/policy.md)
4. Specifically, the [CISA-implemented version of CC0 is used.](https://github.com/cisagov/development-guide/blob/develop/LICENSE)