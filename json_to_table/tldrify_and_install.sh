tldrify_and_install () {
	randomzus=$RANDOM$RANDOM$RANDOM 
	echo '
    tldrify() {
      cp $1.md ~/.cache/tldr/pages/common/$1.md
      cp $1.md ~/.cache/tlrc/pages.en/common/$1.md
    }
    tldrify $1
    read -sp "Enter sudo pass: " sudo_pass 
    cargo build --release
    echo "$sudo_pass" | sudo-rs -S install target/release/$1 /usr/bin/
  ' > /tmp/$randomzus
	chmod +x /tmp/$randomzus
	/tmp/$randomzus $1
	rm /tmp/$randomzus
}
tldrify_and_install $1

