import requests


area = "Finland"
stage = 1
direction = "Forward"
weather = "Dry"
group = "60s"
filter = 0
platform = 2

r = requests.get(f"https://www.funselektorfun.com/artofrally/leaderboard/{area}_Stage_{
                 stage}_{direction}_{weather}_{group}/{filter}/{platform}")
# /{user}/{friends}
print(r.text)
