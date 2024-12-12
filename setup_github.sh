#!/bin/bash

# Initialize git repository
git init

# Create .gitignore file
cat > .gitignore << EOL
/target
/pkg
Cargo.lock
**/*.rs.bk
.DS_Store
EOL

# Add all files
git add .

# Initial commit
git commit -m "Initial commit: Guru Connect - Amiga BBS Terminal"

# Instructions for GitHub setup will be printed
echo "
Now follow these steps:

1. Go to https://github.com/new
2. Create a new repository named 'guru-connect'
3. Do NOT initialize with README, .gitignore, or license

Then run these commands (replace YOUR_USERNAME with your GitHub username):

git remote add origin git@github.com:stlalpha/guru-connect.git
git branch -M main
git push -u origin main
" 
