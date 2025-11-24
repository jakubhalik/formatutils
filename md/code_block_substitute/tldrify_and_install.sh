read -sp "Enter sudo pass: " sudo_pass
rustc md_code_block_substitute.rs && echo $sudo_pass | sudo -S mv md_code_block_substitute /usr/bin/md_code_block_substitute && chmod +x /usr/bin/md_code_block_substitute

cp md_code_block_substitute.md ~/.cache/tldr/pages/common/md_code_block_substitute.md

