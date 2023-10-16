from selenium import webdriver
from selenium.webdriver.chrome.options import Options
options = Options()
options.add_argument('--set-headless=false')
driver = webdriver.Chrome(options=options)
driver.get("http://youtube.com/")
driver.maximize_window()
