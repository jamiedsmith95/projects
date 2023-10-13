import pandas as pd


df = pd.read_csv('owid-covid-data.csv')
print(df.head())
deathsf = df[['iso_code','continent','location','date','population','total_cases','new_cases','new_cases_smoothed','total_deaths','new_deaths','new_deaths_smoothed','total_cases_per_million','new_cases_per_million','new_cases_smoothed_per_million','total_deaths_per_million','new_deaths_per_million','new_deaths_smoothed_per_million','reproduction_rate','icu_patients','icu_patients_per_million','hosp_patients','hosp_patients_per_million','weekly_icu_admissions','weekly_icu_admissions_per_million','weekly_hosp_admissions','weekly_hosp_admissions_per_million']]
deathsf.to_csv('coviddeaths.csv',index = False)
print(deathsf.columns)
vaxf = df.drop(deathsf.columns,axis=1,inplace=False)

vaxf[['iso_code','continent','location','date']] = df[['iso_code','continent','location','date']]
# print(list(vaxf.columns))
vaxf.insert(0,'date',vaxf.pop('date'))
vaxf.insert(0,'location',vaxf.pop('location'))
vaxf.insert(0,'continent',vaxf.pop('continent'))
vaxf.insert(0,'iso_code',vaxf.pop('iso_code'))
print(vaxf.head())
vaxf.to_csv('vaxinfo.csv',index=False)
