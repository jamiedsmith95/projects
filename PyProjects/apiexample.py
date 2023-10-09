import requests
import csv

url = 'http://api.coincap.io/v2/assets'
headers = {'Accept':'application/json', 'Content-Type': 'application/json'}
response = requests.request('Get',url,headers=headers,data=[])
myjson = response.json()
print(myjson)

ourdata = []

for x in myjson['data']:
    listing = [x['symbol'],x['name'],x['priceUsd']]
    ourdata.append(listing)
csvheader = ['Symbol','Name','priceUsd']
with open('crypto.csv','w', encoding='UTF8', newline='') as f:
    writer = csv.writer(f)
    writer.writerows(csvheader)
    writer.writerows(ourdata)
