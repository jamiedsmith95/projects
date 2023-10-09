import pandas as pd
import requests
from bs4 import BeautifulSoup

url = 'https://en.wikipedia.org/wiki/List_of_largest_companies_in_the_United_States_by_revenue'
page = requests.get(url)

soup = BeautifulSoup(page.text, 'html.parser')
table = soup.find_all('table')[1]

titles = table.find_all('th')

table_titles = [title.text.strip() for title in titles]
print(table_titles)

df = pd.DataFrame(columns=table_titles)
column_data = table.find_all('tr')
for row in column_data[1:]:
    row_data = row.find_all('td')
    table_indiv_row = [data.text.strip() for data in row_data]
    length = len(df)
    df.loc[length] = table_indiv_row
print(df)
# <table class="wikitable sortable jquery-tablesorter">
