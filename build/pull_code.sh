mkdir ~/bin
curl https://gitee.com/oschina/repo/raw/fork_flow/repo-py3 -o ~/bin/repo
chmod a+x ~/bin/repo
pip3 install -i https://repo.huaweicloud.com/repository/pypi/simple requests
echo PATH=~/bin:$PATH >> ~/.bashrc
source ~/.bashrc

mkdir -p ~/oh-5.1
cd ~/oh-5.1
git config --global user.email "clangllvm@126.com"
git config --global user.name "fripSide"
git config --global color.ui false

~/bin/repo init -u https://gitee.com/openharmony/manifest.git -b refs/tags/OpenHarmony-v5.1.0-Release --no-repo-verify
~/bin/repo sync -c
~/bin/repo forall -c 'git lfs pull'

