import os
import time
project_dir = 'C:/rock/coding/code/my/rust/programmer-investment-research'
os.chdir(f'{project_dir}/api')
os.system('docker build -t security-api:v1 .')
# os.system('docker run --name security-api-container security-api:v1 ')
# print('sleep 10 seconds...')
# time.sleep(10)
# os.system(f'docker cp security-api-container:/app/security_app {project_dir}/deploy/dist')
print('done...')

# docker cp 44fb2d74bf95:/app/security_app C:/rock/coding/code/my/rust/programmer-investment-research/deploy/dist
# docker cp security-api-container:/app/security_app C:/rock/coding/code/my/rust/programmer-investment-research/deploy/dist