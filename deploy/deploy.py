import paramiko
import zipfile
import os
import shutil
import time

project_dir = 'C:/rock/coding/code/my/rust/programmer-investment-research'
blog_dir = 'C:/rock/coding/code/my/rust/programmer-investment-research/blog'
api_deploy_dir = '/home/ubuntu/investment-research/deploy/api'
upload_dir = '/home/ubuntu/investment-research/upload_tmp'

class Ssh:
    def __init__(self, server, username, password):
        self.ssh = paramiko.SSHClient()
        self.ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        self.ssh.connect(server, 22, username=username, password=password)
        self.scp = self.ssh.open_sftp()

    def download(self, remote_file, localpath):
        self.scp.get(remote_file, localpath)

    def upload(self, local_file, remotepath):
        self.scp.put(local_file, remotepath)

 

    def exec(self, cmd_str):
        (stdin, stdout, stderr) = self.ssh.exec_command(cmd_str)
        output = stdout.read().decode("utf-8")
        err = stderr.read().decode("utf-8")
        print(output)
        print(err)

    def exec_batch(self, cmd_arr):
        cmd_str = '; '.join(cmd_arr)
        (stdin, stdout, stderr) = self.ssh.exec_command(cmd_str)
        output = stdout.read().decode("utf-8")
        err = stderr.read().decode("utf-8")
        print(output)
        print(err)

def deploy_blog(server, username, password):
    os.chdir(blog_dir)
    os.system('hugo')
    print('build hugo complete...')
    zip_directory(f'{blog_dir}/public', f'{blog_dir}/public.zip')
    ssh = Ssh(server, username=username, password=password)
    print('upload zip...')
    ssh.exec('cd /home/ubuntu/investment-research/upload_tmp; sudo rm blog.zip; sudo rm -rf blog;')
    ssh.upload('public.zip', '/home/ubuntu/investment-research/upload_tmp/blog.zip')
    ssh.exec('cd /home/ubuntu/investment-research/upload_tmp; sudo unzip blog.zip -d blog')
    ssh.exec('cd /home/ubuntu/investment-research; sudo cp -rf upload_tmp/blog deploy/blog; echo "deploy success: $?"')

def deploy_nginx(server, username, password):
    ssh = Ssh(server, username=username, password=password)
    os.chdir('C:/rock/coding/code/my/rust/programmer-investment-research/deploy')
    ssh.upload('nginx.conf', '/home/ubuntu/investment-research/upload_tmp/nginx.conf')
    ssh.upload('play-investment.conf', '/home/ubuntu/investment-research/upload_tmp/play-investment.conf')
    ssh.exec('cd /home/ubuntu/investment-research/upload_tmp; sudo cp -rf nginx.conf /etc/nginx; sudo cp -rf play-investment.conf /etc/nginx/conf.d;')
    ssh.exec('sudo pkill nginx; sudo nginx')

def zip_directory(folder_path, zip_filename):
    with zipfile.ZipFile(zip_filename, 'w', zipfile.ZIP_DEFLATED) as zipf:
        for root, dirs, files in os.walk(folder_path):
            for file in files:
                file_path = os.path.join(root, file)
                arcname = os.path.relpath(file_path, folder_path)
                zipf.write(file_path, arcname)

def build_docker():
    os.chdir(f'{project_dir}/api')
    os.system('docker build -t security-api:v1 .')
    os.system('docker run -d --name security-api-app1 security-api:v1 sleep infinity')
    print('run docker success..')
    time.sleep(10)
    os.system(f'docker cp security-api-app1:/app/security_app {project_dir}/deploy/dist')
    print('copy docker file to host success')

def deploy_api(server, username, password):
    shutil.copyfile(f'{project_dir}/api/app_config/cfg/config-prod.toml', f'{project_dir}/deploy/dist/config.toml')
    shutil.copyfile(f'{project_dir}/api/app_config/cfg/log4rs-prod.yml', f'{project_dir}/deploy/dist/log4rs.yml')
    print('copy to dist complete')

    ssh = Ssh(server, username=username, password=password)
    os.chdir(f'{project_dir}/deploy/dist')
    ssh.upload('security_app', f'{upload_dir}/security_app')
    ssh.upload('config.toml',  f'{upload_dir}/config.toml')
    ssh.upload('log4rs.yml',  f'{upload_dir}/log4rs.yml')
    
    os.chdir(f'{project_dir}/python-api')
    ssh.upload('server.py',  f'{upload_dir}/server.py')

    os.chdir(f'{project_dir}/deploy')
    print("begin upload...")
    ssh.upload('investment-research-db.sql',  f'{upload_dir}/investment-research-db.sql')
    print("upload complete...")

    ssh.exec_batch([f'cd {upload_dir}',
                    f'cp security_app {api_deploy_dir}', 
                    f'cp config.toml {api_deploy_dir}', 
                    f'cp log4rs.yml {api_deploy_dir}', 
                    f'cp server.py {api_deploy_dir}/py_service'
                    ])

# (server, username, password) = ('xx', 'xx', 'xx')
(server, username, password) = ('123.207.73.59', 'ubuntu', '@Ke2023River1878')
# build_docker()
# deploy_api(server, username, password)
deploy_blog(server, username, password)
# deploy_nginx(server, username, password)
print('deploy complete...')