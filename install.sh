bin_name=whosts

cp target/debug/$bin_name ./
sudo chown root ./$bin_name
sudo chmod +s ./$bin_name
