# Rust Note portal
### Fullstack Rust app, written for educational purpose! This web app has *main* page, *sign up/login* pages, page with *all posts*(user can make the posts important) for the current user and page to *add new post*. 

# Getting Started
### Make sure you have installed: 
* [Rust](https://www.rust-lang.org/)
* [Docker](https://www.docker.com/) and [Docker Compose](https://docs.docker.com/compose/). Or you can use [PostgreSQL](https://www.postgresql.org/) 

### Have to run
```
git clone https://github.com/NazarZnet/NotesPortal.git
cd ./NotesPortal
ls
```
Now you can see 3 folders( backend, common and frontend). Go to `backend` and run Database docker file! 
```
cd ./backend
docker compose up
```
Or if you want to use your own PostgreSQL database change configuration in `backend/configuration/config.yaml` file! Also you can change [JWT](https://jwt.io/) configuration there!
```
cd ./backend
cd ./configuration

#use your text editor to change config.yaml file!
#for example VS Code

code config.yaml
```
Now you can run the app. Go back to  the `backend` folder and run the server.

```
cd ../
cargo run
```
If everything ok go to the `frontend` folder and run it. I used [Yew](https://yew.rs/), so make sure you have installed all necessary dependencies!

```
cd ../frontend
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve
```
Congratulation! You run the app! Go to *http://127.0.0.1:8080/* and see the app!

## Key Technologies:
* [Rust](https://www.rust-lang.org/)
* [Actix-Web](https://actix.rs/)
* [Yew](https://yew.rs/)
* [Diesel](https://diesel.rs/)
* [JWT](https://jwt.io/)
* [Docker](https://www.docker.com/)
* [PostgreSQL](https://www.postgresql.org/)

## Rotes:

|      /     	|                             Home page with key navigation!                            	|
|:----------:	|:-------------------------------------------------------------------------------------:	|
|   /signup  	|            Sign Up page. You can move to login by using button in the top!            	|
|   / login  	|  Log in page. Make sure you signed up before or use the button on the top to do that! 	|
|   /posts   	| List of all posts. You can make the post important for you and it will be in the top! 	|
| /posts/add 	|                Create a new post with title and description(optional)!                	|

## API Documentation

| /auth/login    	| POST 	| Log in user. Send username and password in JSON format!                                                        	|
|----------------	|------	|----------------------------------------------------------------------------------------------------------------	|
| /auth/signup   	| POST 	| Sign up user. Send username and password in JSON format!                                                       	|
| /auth/refresh  	| GET  	| Refresh JWT access token                                                                                       	|
| /auth/logout   	| GET  	| Log out user. Delete access and refresh token                                                                  	|
| /posts         	| GET  	| Get all posts for current user. Authorization required!                                                        	|
| /posts         	| POST 	| Create new post, send tittle and description in JSON body. Authorization required!                             	|
| /posts/updated 	| POST 	| Make the post important or not, send post's id and important(true/false) in JSON body. Authorization required! 	|
|                	|      	|                                                                                                                	|