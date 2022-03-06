# Google-meet-bot

## How to use
1. Create a .env file in the root directory
2. Add the email id and password in `root/.env` file.
```bash
 ❯ cat .env
email = email@gmail.com
 password = password
```
4. Run the project:
- Run chrome drivers  
  (Download the chrome driver from [https://chromedriver.chromium.org/](https://chromedriver.chromium.org/) )
   ```bash
   chromedriver --port=4444`  
   ```
- Run the project 
  ```bash
  cargo run
  ```
