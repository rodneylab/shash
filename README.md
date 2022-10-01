<p align="center">
  <a aria-label="Open Rodney Lab site" href="https://rodneylab.com" rel="nofollow noopener noreferrer">
    <img alt="Rodney Lab logo" src="https://rodneylab.com/assets/icon.png" width="60" />
  </a>
</p>
<h1 align="center">
  shash
</h1>

Basic CLI tool to generate SHA256 hashes for files.

Example usage:

```shell
shash --output SHA256 *.mp3
```

Generates an SHA256 file:

```
SHA256 (audio-one.mp3) = e33465356c112bd076860ec2a2bf5ec07880866d7cd87b277a8d203c8b99d489
SHA256 (audio-two.mp3) = b947d657b3a59270b105ad6fc156457a5efb8a5dd3bb1de9a5328e9b83e43c09
SHA256 (audio-three.mp3) = 2361d5efddcecdfe93e9cbb16d7e78302ef1984acbec2b4b32967856d2574a61
```

Later you can use this file to verify the files (using the Linux `shasum` tool), for example to check they were not corrupted on uploading to a remote system:

```shell
shasum -c SHASUM
```
