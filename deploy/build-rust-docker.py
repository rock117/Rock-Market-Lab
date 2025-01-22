import os
project_dir = 'C:/rock/coding/code/my/rust/programmer-investment-research'
os.chdir(f'{project_dir}/api')
os.system('docker build -f Rustenv.Dockerfile -t rust-docker:v1 .')
print('done...')
