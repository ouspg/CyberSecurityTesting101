# Building

Building the container image is relatively easy.
By modifying `fake_usernames.csv` - more users can be added.

```bash
docker build --build-arg USERNAMES_FILE=fake_usernames.csv -t ghcr.io/ouspg/cybersecuritytesting101:week2task .
```

