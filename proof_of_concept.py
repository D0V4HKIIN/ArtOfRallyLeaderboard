import requests


area = "Finland"
stage = 1
direction = "Forward"
weather = "Dry"
group = "60s"
filter = 7
platform = 2
user = 76561198230518420

print(f"https://www.funselektorfun.com/artofrally/leaderboard/{area}_Stage_{
      stage}_{direction}_{weather}_{group}/{filter}/{platform}/{user}")

r = requests.get(f"https://www.funselektorfun.com/artofrally/leaderboard/{area}_Stage_{
                 stage}_{direction}_{weather}_{group}/{filter}/{platform}/{user}")
# /{user}/{friends}
print(r.text)
