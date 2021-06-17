# build dependencies
# (cd verifier && make)
# (cd nnf && pip3 install -r requirements.txt)

# download rustc compiler
wget -O stage2.tar.gz https://www.dropbox.com/s/5g26h9qo2rswm7h/stage2.tar.gz?dl=1
tar xvf stage2.tar.gz
rm stage2.tar.gz

