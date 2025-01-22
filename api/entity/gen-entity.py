import os
os.chdir('C:/rock/coding/code/my/rust/programmer-investment-research/api/entity')
os.system('sea-orm-cli generate entity  -u postgres://postgres:123456@localhost:5432/investment_research  -o src --expanded-format  --lib --with-serde both')
print('gen entity complete')