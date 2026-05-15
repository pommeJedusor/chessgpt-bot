# chessgpt-bot

## how to run

### using docker
```bash
docker run --restart unless-stopped -d pommejedusor/chessgpt-bot:latest
```

### using cargo
after cloning this repo and going in the directory
```bash
cargo run
```

### using python
you can simply run that simple script:
requires requests to be installed
```python
import requests, time

url = "https://chessgpt.ai/api/leaderboard"

while True:
    r = requests.post(url, json={"nickname": "nya >:3 🏳️‍⚧️", "result": "win"})
    while not r.ok:
        time.sleep(5)
        r = requests.post(url, json={"nickname": "nya >:3 🏳️‍⚧️", "result": "win"})
```

## website

the website the bot is for is [chessgpt](https://chessgpt.ai/)
to see the current leaderboard:
- [weekly](https://chessgpt.ai/api/leaderboard?period=week&limit=10)
- [monthly](https://chessgpt.ai/api/leaderboard?period=month&limit=10)
- [all-time](https://chessgpt.ai/api/leaderboard?period=all&limit=10)
