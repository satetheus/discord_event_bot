# discord_event_bot
A tool for sending event announcements on discord.

## Local Testing:
In one terminal run:
```bash
cargo lambda watch
```
In another run:
(don't forget to replace the url parameter)
```bash
cargo lambda invoke --data-ascii "{ \"message\": \"This is working\", \"url\": \"your discord webhook url goes here\" }"
```

