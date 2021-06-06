# Change log + tasks
This is file where I describe all the [changes])(#сhages) I've done so far and [tasks](#to-do) I'm going to make

## TO DO:

#### Errors:
- [ ] utf8 error handling
- [ ] "no `graph_targets` to no results" - bug
- [ ] reqults handling correctly

#### New features:

##### Big inverntions:
- [ ] ssl ca verification 
- [ ] ssl certificate acceptance
- [x] make config for users credentials
- [x] add router custum naming in login config file
- [ ] make config file and gradana dashboard using prometheus data got by this application
- [ ] upgrade `README.md` file
- [ ] Verbose and non-verbore loging update

##### Slight updates:
- [ ] command query execution from `commands.json` file
- [x] Write config tutorial in README.md
- [ ] update commands template and example files

#### Not that important:
- [ ] connect to routerboards using domain name
- [ ] make config change on site?
- [ ] make error type, not just a string
- [ ] Optimise responce parsing using custum types for each query
- [ ] async requests or parallel

#### Older tasks:
- [x] make `query_teller` being able to operate with list of connections
- [x] rename functions to make them more meaningful
- [x] custum configuration of web server from main function
- [x] queries live update or button to update
- [x] make some comments + function documentation
- [x] remove length mark or use it correctly in router's reply
- [x] update commands config file
- [x] commands config implementation in lib
- [x] do all mikrotik query types
- [x] cover mikrotik responce message commands
- [x] prometheus integration
- [x] encrypt user credentials
- [x] uft8 converter error ( some characters appears in the end of the responce or just disapear ... )
- [x] turn `tell_get` into functuins that borows mutable vector to push responces inside it
- [x] move enerything into a library
- [x] ssl encryption

## Chages:

### Last changes:
- added config files explanation
- changed files location slightly
- moved routerboards users into separate file to seek identities from

### Older Changes: 
##### v0.1.2
- reorganised todo list
- slightly reorganised files organization
- made Readme.md more prety
- made `query_teller` being able to operate with list of connections
- queries live update or button to update ( on the web page ) 
- custum configuration of web server from main function

##### v0.1.1
- moved to tiny_http
- first version of http post page
- cleaned everything up a little bit

##### v0.1.0
- json file to seek commands in automaticly
- slight commands config file
- added some comments
- first test of asyncing