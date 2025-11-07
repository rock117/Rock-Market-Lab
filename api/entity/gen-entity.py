import os
os.chdir('C:/rock/coding/code/my/rust/Rock-Market-Lab/api/entity')
os.system('sea-orm-cli generate entity  -u mysql://root:123456@localhost:3306/investment_research  -o src --expanded-format  --lib --with-serde both')
print('gen entity complete')



# pub use sea_orm;

# #[serde(rename = "nca_within_1y")]
# pub nca_within_y1: Option<Decimal>,
# [serde(rename = "non_cur_liab_due_1y")]
# pub non_cur_liab_due_y1: Option<Decimal>,