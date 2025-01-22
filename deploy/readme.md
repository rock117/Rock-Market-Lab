# build docker
1. docker build -f Rustenv.Dockerfile -t rust-docker:v1 .
2. docker build -t security-api:v1 .
3. docker run -d --name security-api-app1 security-api:v1 sleep infinity

# copy docker file to host dist folder
1. docker cp security-api-app1:/app/security_app {project_dir}/deploy/dist

# upload to server/deploy app
api_deploy_dir = /home/ubuntu/investment-research/deploy/api
upload_dir = /home/ubuntu/investment-research/upload_tmp

# db
mysql -uroot  < investment-research-db.sql 