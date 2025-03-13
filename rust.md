If you don't have sudo access, you can install tmux in your home directory using the following method:


---

Step 1: Download & Extract tmux

Run the following commands in your home directory:

mkdir -p ~/local ~/tmux_tmp
cd ~/tmux_tmp

# Download latest tmux source
curl -LO https://github.com/tmux/tmux/releases/download/3.4/tmux-3.4.tar.gz

# Extract the file
tar -xzf tmux-3.4.tar.gz
cd tmux-3.4


---

Step 2: Install Dependencies Locally

Since tmux depends on libevent and ncurses, you need to install them too.

Install libevent

curl -LO https://github.com/libevent/libevent/releases/download/release-2.1.12-stable/libevent-2.1.12-stable.tar.gz
tar -xzf libevent-2.1.12-stable.tar.gz
cd libevent-2.1.12-stable
./configure --prefix=$HOME/local
make -j$(nproc) && make install
cd ..

Install ncurses

curl -LO https://ftp.gnu.org/pub/gnu/ncurses/ncurses-6.3.tar.gz
tar -xzf ncurses-6.3.tar.gz
cd ncurses-6.3
./configure --prefix=$HOME/local
make -j$(nproc) && make install
cd ..


---

Step 3: Compile & Install tmux

cd tmux-3.4
./configure --prefix=$HOME/local CFLAGS="-I$HOME/local/include" LDFLAGS="-L$HOME/local/lib"
make -j$(nproc) && make install


---

Step 4: Add tmux to PATH

echo 'export PATH=$HOME/local/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=$HOME/local/lib:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc


---

Step 5: Verify Installation

tmux -V

It should output:

tmux 3.4


---

Now you can use tmux without sudo!

Let me know if you need any modifications.

