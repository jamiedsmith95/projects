from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from webdriver_manager.chrome import ChromeDriverManager
import chromedriver_autoinstaller
import os
import pathlib
from time import sleep
chromedriver_autoinstaller.install()
options = Options()
dir_path = pathlib.Path().absolute()
path = str(chromedriver_autoinstaller.install())
# path = 'C:\\Users\\jamie\\jobstuff\\chromedriver-win64'
print(path)
service = Service(executable_path=path)
# options.add_argument('--remote-allow-origins=*')

options.add_experimental_option("useAutomationExtension", False)
options.add_experimental_option("excludeSwitches",["enable-automation"])
options.add_argument('user-data-dir=C:\\Program\ Files\ (x86)\\Google\\Chrome\\Application')
options.add_argument('set_headless="false')
driver = webdriver.Chrome(service= service,options = options)

sleep(1)
# Open Scrapingbee's website
driver.get("http://google.com")
driver.maximize_window()
driver.quit()
